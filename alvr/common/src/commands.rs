use crate::*;
use serde_json as json;
use std::{
    collections::HashSet,
    env, fs,
    path::{Path, PathBuf},
    process::*,
    thread,
    time::Duration,
};
use sysinfo::*;

#[cfg(windows)]
use std::os::windows::process::CommandExt;

const ALVR_DIR_STORAGE_FNAME: &str = "alvr_dir.txt";
const DRIVER_PATHS_BACKUP_FNAME: &str = "alvr_drivers_paths_backup.txt";

#[cfg(windows)]
pub const CREATE_NO_WINDOW: u32 = 0x0800_0000;

#[cfg(target_os = "linux")]
pub fn exec_fname(name: &str) -> String {
    name.to_owned()
}
#[cfg(windows)]
pub fn exec_fname(name: &str) -> String {
    format!("{}.exe", name)
}

fn openvr_source_file_path() -> StrResult<PathBuf> {
    Ok(trace_none!(dirs::cache_dir())?.join("openvr/openvrpaths.vrpath"))
}

fn load_openvr_paths_json() -> StrResult<json::Value> {
    let file_content = trace_err!(fs::read_to_string(openvr_source_file_path()?))?;

    trace_err!(json::from_str(&file_content))
}

fn save_openvr_paths_json(openvr_paths: &json::Value) -> StrResult {
    let file_content = trace_err!(json::to_string_pretty(openvr_paths))?;

    trace_err!(fs::write(openvr_source_file_path()?, file_content))
}

fn from_openvr_paths(paths: &json::Value) -> StrResult<Vec<PathBuf>> {
    let paths_vec = match paths.as_array() {
        Some(vec) => vec,
        None => return Ok(vec![]),
    };

    Ok(paths_vec
        .iter()
        .filter_map(json::Value::as_str)
        .map(|s| PathBuf::from(s.replace(r"\\", r"\")))
        .collect())
}

fn to_openvr_paths(paths: &[PathBuf]) -> json::Value {
    let paths_vec = paths
        .iter()
        .map(|p| p.to_string_lossy().into())
        .map(json::Value::String) // backslashes gets duplicated here
        .collect::<Vec<_>>();

    json::Value::Array(paths_vec)
}

fn get_single_openvr_path(path_type: &str) -> StrResult<PathBuf> {
    let openvr_paths_json = load_openvr_paths_json()?;
    let paths_json = trace_none!(openvr_paths_json.get(path_type))?;
    trace_none!(from_openvr_paths(paths_json)?.get(0).cloned())
}

fn steamvr_root_dir() -> StrResult<PathBuf> {
    get_single_openvr_path("runtime")
}

fn steam_config_dir() -> StrResult<PathBuf> {
    get_single_openvr_path("config")
}

pub fn get_registered_drivers() -> StrResult<Vec<PathBuf>> {
    from_openvr_paths(trace_none!(
        load_openvr_paths_json()?.get_mut("external_drivers")
    )?)
}

pub fn driver_registration(driver_paths: &[PathBuf], register: bool) -> StrResult {
    let mut openvr_paths_json = load_openvr_paths_json()?;
    let paths_json_ref = trace_none!(openvr_paths_json.get_mut("external_drivers"))?;

    let mut paths: HashSet<_> = from_openvr_paths(paths_json_ref)?.into_iter().collect();

    if register {
        paths.extend(driver_paths.iter().cloned());
    } else {
        for path in driver_paths {
            paths.remove(path);
        }
    }

    // write into openvr_paths_json, the other fields are preserved
    *paths_json_ref = to_openvr_paths(paths.into_iter().collect::<Vec<_>>().as_slice());

    save_openvr_paths_json(&openvr_paths_json)
}

#[cfg(windows)]
fn kill_process(pid: usize) {
    Command::new("taskkill.exe")
        .args(&["/PID", &pid.to_string(), "/F"])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .ok();
}

pub fn is_steamvr_running() -> bool {
    let mut system = System::new_with_specifics(RefreshKind::new().with_processes());
    system.refresh_processes();

    !system
        .get_process_by_name(&exec_fname("vrserver"))
        .is_empty()
}

pub fn maybe_launch_steamvr() {
    let mut system = System::new_with_specifics(RefreshKind::new().with_processes());
    system.refresh_processes();

    if system
        .get_process_by_name(&exec_fname("vrserver"))
        .is_empty()
    {
        Command::new("cmd")
            .args(&["/C", "start", "steam://rungameid/250820"])
            .creation_flags(CREATE_NO_WINDOW)
            .spawn()
            .ok();
    }
}

// this will not kill the child process "ALVR launcher"
pub fn kill_steamvr() {
    let mut system = System::new_with_specifics(RefreshKind::new().with_processes());
    system.refresh_processes();

    // first kill vrmonitor, then kill vrserver if it is hung.

    for process in system.get_process_by_name(&exec_fname("vrmonitor")) {
        #[cfg(not(windows))]
        process.kill(Signal::Term);
        #[cfg(windows)]
        kill_process(process.pid());
    }

    thread::sleep(Duration::from_secs(1));

    for process in system.get_process_by_name(&exec_fname("vrserver")) {
        #[cfg(not(windows))]
        process.kill(Signal::Term);
        #[cfg(windows)]
        kill_process(process.pid());
    }
}

fn netsh_add_rule_command_string(rule_name: &str, program_path: &Path) -> String {
    format!(
        "netsh advfirewall firewall add rule name=\"{}\" dir=in program=\"{}\" action=allow",
        rule_name,
        program_path.to_string_lossy()
    )
}

fn netsh_delete_rule_command_string(rule_name: &str) -> String {
    format!(
        "netsh advfirewall firewall delete rule name=\"{}\"",
        rule_name,
    )
}

// Errors:
// 1: firewall rule is already set
// other: command failed
pub fn firewall_rules(add: bool) -> Result<(), i32> {
    let script_path = env::temp_dir().join("alvr_firewall_rules.bat");

    let firewall_rules_script_content = if add {
        format!(
            "{}\n{}",
            netsh_add_rule_command_string(
                "SteamVR ALVR vrserver",
                &steamvr_root_dir()
                    .map_err(|_| -1)?
                    .join("bin")
                    .join("win64")
                    .join("vrserver.exe")
            ),
            netsh_add_rule_command_string(
                "SteamVR ALVR vrserver",
                &steamvr_root_dir()
                    .map_err(|_| -1)?
                    .join("bin")
                    .join("win32")
                    .join("vrserver.exe")
            ),
        )
    } else {
        format!(
            "{}\n{}",
            netsh_delete_rule_command_string("ALVR Launcher"),
            netsh_delete_rule_command_string("SteamVR ALVR vrserver")
        )
    };
    fs::write(&script_path, firewall_rules_script_content).map_err(|_| -1)?;

    // run with admin priviles
    let exit_status = runas::Command::new(script_path)
        .show(cfg!(target_os = "linux"))
        .gui(true) // UAC, if available
        .status()
        .map_err(|_| -1)?;

    if exit_status.success() {
        Ok(())
    } else {
        Err(exit_status.code().unwrap())
    }
}

fn get_alvr_dir_from_storage() -> StrResult<PathBuf> {
    let alvr_dir_store_path = env::temp_dir().join(ALVR_DIR_STORAGE_FNAME);
    if let Ok(path) = fs::read_to_string(alvr_dir_store_path) {
        Ok(PathBuf::from(path))
    } else {
        Err("ALVR driver path not stored".into())
    }
}

pub fn get_alvr_dir_from_registered_drivers() -> StrResult<PathBuf> {
    for dir in get_registered_drivers()? {
        if dir.join(exec_fname("ALVR launcher")).exists() && dir.join("web_gui").exists() {
            return Ok(dir);
        }
    }
    Err("ALVR driver path not registered".into())
}

pub fn get_alvr_dir() -> StrResult<PathBuf> {
    get_alvr_dir_from_storage()
        .or_else(|_| get_alvr_dir_from_registered_drivers())
        .map_err(|_| "ALVR driver path not stored and not registered".into())
}

pub fn store_alvr_dir(alvr_dir: &Path) -> StrResult {
    let alvr_dir_store_path = env::temp_dir().join(ALVR_DIR_STORAGE_FNAME);

    trace_err!(fs::write(
        alvr_dir_store_path,
        alvr_dir.to_string_lossy().as_bytes()
    ))
}

pub fn maybe_delete_alvr_dir_storage() {
    fs::remove_file(env::temp_dir().join(ALVR_DIR_STORAGE_FNAME)).ok();
}

pub fn maybe_open_launcher(alvr_dir: &Path) {
    let mut command = Command::new(alvr_dir.join("ALVR Launcher"));
    command.creation_flags(CREATE_NO_WINDOW).spawn().ok();
}

pub fn check_steamvr_installation() -> bool {
    openvr_source_file_path()
        .map(|p| p.exists())
        .unwrap_or(false)
}

#[cfg(windows)]
pub fn check_msvcp_installation() -> StrResult<bool> {
    let output = trace_err!(Command::new("where")
        .arg("msvcp140_2.dll")
        .creation_flags(CREATE_NO_WINDOW)
        .output())?;
    let output = String::from_utf8_lossy(&output.stdout);

    Ok(output.contains("msvcp140_2.dll"))
}

pub fn restart_steamvr_with_timeout(alvr_dir: &Path) -> StrResult {
    trace_err!(Command::new(alvr_dir.join(exec_fname("ALVR launcher")))
        .arg("--restart-steamvr")
        .status())?;

    Ok(())
}

fn driver_paths_backup_present() -> bool {
    env::temp_dir().join(DRIVER_PATHS_BACKUP_FNAME).exists()
}

pub fn maybe_save_driver_paths_backup(paths_backup: &[PathBuf]) -> StrResult {
    if !driver_paths_backup_present() {
        trace_err!(fs::write(
            env::temp_dir().join(DRIVER_PATHS_BACKUP_FNAME),
            trace_err!(json::to_string_pretty(paths_backup))?,
        ))?;
    }

    Ok(())
}

pub fn apply_driver_paths_backup(alvr_dir: PathBuf) -> StrResult {
    if driver_paths_backup_present() {
        let backup_path = env::temp_dir().join(DRIVER_PATHS_BACKUP_FNAME);
        let driver_paths = trace_err!(json::from_str::<Vec<_>>(&trace_err!(fs::read_to_string(
            &backup_path
        ))?))?;
        trace_err!(fs::remove_file(backup_path))?;

        driver_registration(&[alvr_dir], false)?;

        driver_registration(&driver_paths, true).ok();
    }

    Ok(())
}

pub fn unblock_alvr_addon() -> StrResult {
    let config_path = steam_config_dir()?.join("steamvr.vrsettings");

    let mut fields_ref: json::Map<String, json::Value> = trace_err!(json::from_str(&trace_err!(
        fs::read_to_string(&config_path)
    )?))?;

    fields_ref.remove("driver_alvr_server");

    trace_err!(fs::write(
        config_path,
        trace_err!(json::to_string_pretty(&fields_ref))?
    ))?;

    Ok(())
}
