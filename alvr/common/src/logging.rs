use serde::{Deserialize, Serialize};
use std::{fmt::Display, future::Future};

pub type StrResult<T = ()> = Result<T, String>;

pub const SESSION_LOG_FNAME: &str = "session_log.txt";
pub const CRASH_LOG_FNAME: &str = "crash_log.txt";

pub fn set_panic_hook() {
    std::panic::set_hook(Box::new(|panic_info| {
        let message = panic_info
            .payload()
            .downcast_ref::<&str>()
            .unwrap_or(&"Unavailable");
        let err_str = format!(
            "Message: {:?}\nBacktrace:\n{:?}",
            message,
            backtrace::Backtrace::new()
        );

        log::error!("{}", err_str);

        #[cfg(not(target_os = "android"))]
        std::thread::spawn(move || {
            msgbox::create("ALVR panicked", &err_str, msgbox::IconType::Error).ok();
        });
    }))
}

pub fn show_w<W: Display>(w: W) {
    log::warn!("{}", w);

    #[cfg(not(target_os = "android"))]
    std::thread::spawn({
        let warn_string = w.to_string();
        move || {
            msgbox::create(
                "ALVR encountered a non-fatal error",
                &warn_string,
                msgbox::IconType::Info,
            )
            .ok();
        }
    });
}

pub fn show_warn<T, E: Display>(res: Result<T, E>) -> Option<T> {
    res.map_err(show_w).ok()
}

fn show_e_block<E: Display>(e: E, blocking: bool) {
    log::error!("{}", e);

    #[cfg(not(target_os = "android"))]
    {
        let show_msgbox = {
            let err_string = e.to_string();
            move || {
                msgbox::create(
                    "ALVR encountered an error",
                    &err_string,
                    msgbox::IconType::Error,
                )
                .ok();
            }
        };

        if blocking {
            show_msgbox();
        } else {
            std::thread::spawn(show_msgbox);
        }
    }
}

pub fn show_e<E: Display>(e: E) {
    show_e_block(e, false);
}

pub fn show_e_blocking<E: Display>(e: E) {
    show_e_block(e, true);
}

pub fn show_err<T, E: Display>(res: Result<T, E>) -> Option<T> {
    res.map_err(|e| show_e_block(e, false)).ok()
}

pub fn show_err_blocking<T, E: Display>(res: Result<T, E>) -> Option<T> {
    res.map_err(|e| show_e_block(e, true)).ok()
}

pub async fn show_err_async<T, E: Display>(
    future_res: impl Future<Output = Result<T, E>>,
) -> Option<T> {
    show_err(future_res.await)
}

#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum SessionUpdateType {
    Settings,
    ClientList,
    Other, // other top level flags, like "setup_wizard"
}

// Log id is serialized as #{ "id": "..." [, "data": ...] }#
// Pound signs are used to identify start and finish of json
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "id", content = "data")]
pub enum LogId {
    #[serde(rename_all = "camelCase")]
    SessionUpdated {
        web_client_id: Option<String>,
        update_type: SessionUpdateType,
    },
    SessionSettingsExtrapolationFailed,
    ClientFoundOk,
    ClientFoundInvalid,
    ClientFoundWrongIp,
    ClientFoundWrongVersion(String),
    ClientConnected,
    ClientDisconnected,
    UpdateDownloadProgress(f32),
    UpdateDownloadError,
}

pub fn log_id(id: LogId) {
    log::info!("#{}#", serde_json::to_string(&id).unwrap());
}

#[macro_export]
macro_rules! fmt_e {
    ($($args:tt)+) => {
        Err(format!($($args)+))
    };
}

#[macro_export]
macro_rules! trace_str {
    () => {
        format!("At {}:{}", file!(), line!())
    };
}

#[macro_export]
macro_rules! trace_err {
    ($res:expr) => {
        $res.map_err(|e| format!("{}: {}", trace_str!(), e))
    };
}

// trace_err variant for errors that do not implement fmt::Display
#[macro_export]
macro_rules! trace_err_dbg {
    ($res:expr) => {
        $res.map_err(|e| format!("{}: {:?}", trace_str!(), e))
    };
}

#[macro_export]
macro_rules! trace_none {
    ($res:expr) => {
        $res.ok_or_else(|| trace_str!())
    };
}
