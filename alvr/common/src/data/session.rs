use super::settings::*;
use crate::{
    logging::{LogId, SessionUpdateType},
    *,
};
use serde::*;
use serde_json as json;
use settings_schema::SchemaNode;
use std::{
    fs,
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
};

pub const SESSION_FNAME: &str = "session.json";

// SessionSettings is similar to Settings but it contains every branch, even unused ones. This is
// the settings representation that the UI uses.
type SessionSettings = SettingsDefault;

pub fn load_session(path: &Path) -> StrResult<SessionDesc> {
    trace_err!(json::from_str(&trace_err!(fs::read_to_string(path))?))
}

pub fn save_session(session_desc: &SessionDesc, path: &Path) -> StrResult {
    trace_err!(fs::write(
        path,
        trace_err!(json::to_string_pretty(session_desc))?
    ))
}

// This structure is used to store the minimum configuration data that ALVR driver needs to
// initialize OpenVR before having the chance to communicate with a client. When a client is
// connected, a new OpenvrConfig instance is generated, then the connection is accepted only if that
// instance is equivalent to the one stored in the session, otherwise SteamVR is restarted.
// Other components (like the encoder, audio recorder) don't need this treatment and are initialized
// dynamically.
// todo: properties that can be set after the OpenVR initialization should be removed and set with
// UpdateForStream.
#[derive(Serialize, Deserialize, PartialEq, Default)]
pub struct OpenvrConfig {
    pub headset_serial_number: String,
    pub headset_tracking_system_name: String,
    pub headset_model_number: String,
    pub headset_driver_version: String,
    pub headset_manufacturer_name: String,
    pub headset_render_model_name: String,
    pub headset_registered_device_type: String,
    pub eye_resolution_width: u32,
    pub eye_resolution_height: u32,
    pub target_eye_resolution_width: u32,
    pub target_eye_resolution_height: u32,
    pub eye_fov: [Fov; 2],
    pub enable_game_audio: bool,
    pub game_audio_device: String,
    pub enable_microphone: bool,
    pub microphone_device: String,
    pub seconds_from_vsync_to_photons: f32,
    pub ipd: f32,
    pub client_buffer_size: u64,
    pub frame_queue_size: u64,
    pub force_60hz: bool,
    pub force_3dof: bool,
    pub aggressive_keyframe_resend: bool,
    pub adapter_index: u32,
    pub codec: u32,
    pub refresh_rate: u32,
    pub encode_bitrate_mbs: u64,
    pub throttling_bitrate_bits: u64,
    pub listen_host: String,
    pub listen_port: u16,
    pub client_address: String,
    pub controllers_tracking_system_name: String,
    pub controllers_manufacturer_name: String,
    pub controllers_model_number: String,
    pub render_model_name_left_controller: String,
    pub render_model_name_right_controller: String,
    pub controllers_serial_number: String,
    pub controllers_type: String,
    pub controllers_registered_device_type: String,
    pub controllers_input_profile_path: String,
    pub controllers_mode_idx: i32,
    pub controllers_enabled: bool,
    pub position_offset: [f32; 3],
    pub tracking_frame_offset: i32,
    pub controller_pose_offset: f32,
    pub position_offset_left: [f32; 3],
    pub rotation_offset_left: [f32; 3],
    pub haptics_intensity: f32,
    pub enable_foveated_rendering: bool,
    pub foveation_strength: f32,
    pub foveation_shape: f32,
    pub foveation_vertical_offset: f32,
    pub enable_color_correction: bool,
    pub brightness: f32,
    pub contrast: f32,
    pub saturation: f32,
    pub gamma: f32,
    pub sharpening: f32,
}

#[derive(Serialize, Debug)]
pub struct ServerHandshakePacket {
    pub packet_type: u32,
    pub codec: u32,
    pub realtime_decoder: bool,
    pub video_width: u32,
    pub video_height: u32,
    pub buffer_size_bytes: u32,
    pub frame_queue_size: u32,
    pub refresh_rate: u8,
    pub stream_mic: bool,
    pub foveation_mode: u8,
    pub foveation_strength: f32,
    pub foveation_shape: f32,
    pub foveation_vertical_offset: f32,
    pub tracking_space: u32,
    pub web_gui_url: [u8; 32], // serde do not support arrays larger than 32. Slices can be of any
                               // size, but are not c compatible
}

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClientHandshakePacket {
    pub packet_type: u32,
    pub alvr_name: [u8; 4],
    pub version: [u8; 32],
    pub device_name: [u8; 32],
    pub client_refresh_rate: u16,
    pub render_width: u32,
    pub render_height: u32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ClientConnectionState {
    AvailableUntrusted,
    AvailableTrusted,
    UnavailableTrusted,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientConnectionDesc {
    pub state: ClientConnectionState,
    pub last_update_ms_since_epoch: u64,
    pub address: String,
    pub handshake_packet: ClientHandshakePacket,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionDesc {
    pub setup_wizard: bool,
    pub openvr_config: OpenvrConfig,
    pub last_clients: Vec<ClientConnectionDesc>,
    pub session_settings: SessionSettings,
}

impl Default for SessionDesc {
    fn default() -> Self {
        Self {
            setup_wizard: true,
            openvr_config: OpenvrConfig {
                headset_serial_number: "1WMGH000XX0000".into(),
                headset_tracking_system_name: "oculus".into(),
                headset_model_number: "Oculus Rift S".into(),
                headset_driver_version: "1.42.0".into(),
                headset_manufacturer_name: "Oculus".into(),
                headset_render_model_name: "generic_hmd".into(),
                headset_registered_device_type: "oculus/1WMGH000XX0000".into(),
                eye_resolution_width: 960,
                eye_resolution_height: 1080,
                target_eye_resolution_width: 960,
                target_eye_resolution_height: 1080,
                eye_fov: [
                    Fov {
                        left: 45_f32,
                        right: 45_f32,
                        top: 45_f32,
                        bottom: 45_f32,
                    },
                    Fov {
                        left: 45_f32,
                        right: 45_f32,
                        top: 45_f32,
                        bottom: 45_f32,
                    },
                ],
                seconds_from_vsync_to_photons: 0.005,
                ipd: 0.063,
                adapter_index: 0,
                refresh_rate: 60,
                controllers_enabled: false,
                enable_foveated_rendering: false,
                enable_color_correction: false,
                ..<_>::default()
            },
            last_clients: vec![],
            session_settings: session_settings_default(),
        }
    }
}

impl SessionDesc {
    // If json_value is not a valid representation of SessionDesc (because of version upgrade), use
    // some fuzzy logic to extrapolate as much information as possible.
    // Since SessionDesc cannot have a schema (because SessionSettings would need to also have a
    // schema, but it is generated out of our control), I only do basic name checking on fields and
    // deserialization will fail if the type of values does not match. Because of this,
    // `session_settings` must be handled separately to do a better job of retrieving data using the
    // settings schema.
    pub fn merge_from_json(&mut self, json_value: &json::Value) -> StrResult {
        const SESSION_SETTINGS_STR: &str = "sessionSettings";

        if let Ok(session_desc) = json::from_value(json_value.clone()) {
            *self = session_desc;
            return Ok(());
        }

        let old_session_json = trace_err!(json::to_value(SessionDesc::default()))?;
        let old_session_fields = trace_none!(old_session_json.as_object())?;

        let session_settings_json =
            json_value
                .get(SESSION_SETTINGS_STR)
                .map(|new_session_settings_json| {
                    extrapolate_session_settings(
                        &old_session_json[SESSION_SETTINGS_STR],
                        new_session_settings_json,
                        &settings_schema(session_settings_default()),
                    )
                });

        let new_fields = old_session_fields
            .iter()
            .map(|(name, json_field_value)| {
                let new_json_field_value = if name == SESSION_SETTINGS_STR {
                    json::to_value(session_settings_default()).unwrap()
                } else {
                    json_value.get(name).unwrap_or(json_field_value).clone()
                };
                (name.clone(), new_json_field_value)
            })
            .collect();
        // Failure to extrapolate other session_desc fields is not notified.
        let mut session_desc_mut =
            json::from_value::<SessionDesc>(json::Value::Object(new_fields)).unwrap_or_default();

        match json::from_value::<SessionSettings>(trace_none!(session_settings_json)?) {
            Ok(session_settings) => {
                session_desc_mut.session_settings = session_settings;
                *self = session_desc_mut;
                Ok(())
            }
            Err(e) => {
                *self = session_desc_mut;
                trace_str!(
                    id: LogId::SessionSettingsExtrapolationFailed,
                    "Error while deserializing extrapolated session settings: {}",
                    e
                )
            }
        }
    }

    // This function requires that settings enums with data have tag = "type" and content = "content", and
    // enums without data do not have tag and content set.
    pub fn to_settings(&self) -> Settings {
        let session_settings_json = json::to_value(&self.session_settings).unwrap();
        let schema = settings_schema(session_settings_default());
        json::from_value(json_session_settings_to_settings(
            &session_settings_json,
            &schema,
        ))
        .unwrap()
    }
}

// Current data extrapolation strategy: match both field name and value type exactly.
// Integer bounds are not validated, if they do not match the schema, deserialization will fail and
// all data is lost.
// Future strategies: check if value respects schema constraints, fuzzy field name matching, accept
// integer to float and float to integer, tree traversal.
fn extrapolate_session_settings(
    old_session_settings: &json::Value,
    new_session_settings: &json::Value,
    schema: &SchemaNode,
) -> json::Value {
    match schema {
        SchemaNode::Section { entries } => json::Value::Object(
            entries
                .iter()
                .filter_map(|(field_name, maybe_data)| {
                    maybe_data.as_ref().map(|data_schema| {
                        let value_json =
                            if let Some(new_value_json) = new_session_settings.get(field_name) {
                                extrapolate_session_settings(
                                    &old_session_settings[field_name],
                                    new_value_json,
                                    &data_schema.content,
                                )
                            } else {
                                old_session_settings[field_name].clone()
                            };
                        (field_name.clone(), value_json)
                    })
                })
                .collect(),
        ),

        SchemaNode::Choice { variants, .. } => {
            let variant_json = new_session_settings
                .get("variant")
                .cloned()
                .filter(|new_variant_json| {
                    new_variant_json
                        .as_str()
                        .map(|variant_str| {
                            variants
                                .iter()
                                .any(|(variant_name, _)| variant_str == variant_name)
                        })
                        .is_some()
                })
                .unwrap_or_else(|| old_session_settings["variant"].clone());

            let mut fields: json::Map<_, _> = variants
                .iter()
                .filter_map(|(variant_name, maybe_data)| {
                    maybe_data.as_ref().map(|data_schema| {
                        let value_json =
                            if let Some(new_value_json) = new_session_settings.get(variant_name) {
                                extrapolate_session_settings(
                                    &old_session_settings[variant_name],
                                    new_value_json,
                                    &data_schema.content,
                                )
                            } else {
                                old_session_settings[variant_name].clone()
                            };
                        (variant_name.clone(), value_json)
                    })
                })
                .collect();
            fields.insert("variant".into(), variant_json);

            json::Value::Object(fields)
        }

        SchemaNode::Optional { content, .. } => {
            let set_json = new_session_settings
                .get("set")
                .cloned()
                .filter(|new_set_json| new_set_json.is_boolean())
                .unwrap_or_else(|| old_session_settings["set"].clone());

            let content_json = new_session_settings
                .get("content")
                .map(|new_content_json| {
                    extrapolate_session_settings(
                        &old_session_settings["content"],
                        new_content_json,
                        content,
                    )
                })
                .unwrap_or_else(|| old_session_settings["content"].clone());

            json::json!({
                "set": set_json,
                "content": content_json
            })
        }

        SchemaNode::Switch { content, .. } => {
            let enabled_json = new_session_settings
                .get("enabled")
                .cloned()
                .filter(|new_enabled_json| new_enabled_json.is_boolean())
                .unwrap_or_else(|| old_session_settings["enabled"].clone());

            let content_json = new_session_settings
                .get("content")
                .map(|new_content_json| {
                    extrapolate_session_settings(
                        &old_session_settings["content"],
                        new_content_json,
                        content,
                    )
                })
                .unwrap_or_else(|| old_session_settings["content"].clone());

            json::json!({
                "enabled": enabled_json,
                "content": content_json
            })
        }

        SchemaNode::Boolean { .. } => {
            if new_session_settings.is_boolean() {
                new_session_settings.clone()
            } else {
                old_session_settings.clone()
            }
        }

        SchemaNode::Integer { .. } => {
            if new_session_settings.is_i64() {
                new_session_settings.clone()
            } else {
                old_session_settings.clone()
            }
        }

        SchemaNode::Float { .. } => {
            if new_session_settings.is_f64() {
                new_session_settings.clone()
            } else {
                old_session_settings.clone()
            }
        }

        SchemaNode::Text { .. } => {
            if new_session_settings.is_string() {
                new_session_settings.clone()
            } else {
                old_session_settings.clone()
            }
        }

        SchemaNode::Array(array_schema) => {
            let array_vec = (0..array_schema.len())
                .map(|idx| {
                    new_session_settings
                        .get(idx)
                        .cloned()
                        .unwrap_or_else(|| old_session_settings[idx].clone())
                })
                .collect();
            json::Value::Array(array_vec)
        }

        SchemaNode::Vector {
            default_element, ..
        } => {
            let element_json = new_session_settings
                .get("element")
                .map(|new_element_json| {
                    extrapolate_session_settings(
                        &old_session_settings["content"],
                        new_element_json,
                        default_element,
                    )
                })
                .unwrap_or_else(|| old_session_settings["content"].clone());

            // todo: default field cannot be properly validated until I implement plain settings
            // validation (not to be confused with session/session_settings validation). Any
            // problem inside this new_session_settings default will result in the loss all data in the new
            // session_settings.
            let default = new_session_settings
                .get("default")
                .cloned()
                .unwrap_or_else(|| old_session_settings["default"].clone());

            json::json!({
                "element": element_json,
                "default": default
            })
        }

        SchemaNode::Dictionary { default_value, .. } => {
            let key_json = new_session_settings
                .get("key")
                .cloned()
                .filter(|new_key| new_key.is_string())
                .unwrap_or_else(|| old_session_settings["key"].clone());

            let value_json = new_session_settings
                .get("value")
                .map(|new_value_json| {
                    extrapolate_session_settings(
                        &old_session_settings["value"],
                        new_value_json,
                        default_value,
                    )
                })
                .unwrap_or_else(|| old_session_settings["content"].clone());

            // todo: validate default using settings validation
            let default = new_session_settings
                .get("default")
                .cloned()
                .unwrap_or_else(|| old_session_settings["default"].clone());

            json::json!({
                "key": key_json,
                "value": value_json,
                "default": default
            })
        }
    }
}

fn json_session_settings_to_settings(
    session_settings: &json::Value,
    schema: &SchemaNode,
) -> json::Value {
    match schema {
        SchemaNode::Section { entries } => json::Value::Object(
            entries
                .iter()
                .filter_map(|(field_name, maybe_data)| {
                    maybe_data.as_ref().map(|data_schema| {
                        (
                            field_name.clone(),
                            json_session_settings_to_settings(
                                &session_settings[field_name],
                                &data_schema.content,
                            ),
                        )
                    })
                })
                .collect(),
        ),

        SchemaNode::Choice { variants, .. } => {
            let variant = session_settings["variant"].clone();
            let only_tag = variants
                .iter()
                .all(|(_, maybe_data)| matches!(maybe_data, None));
            if only_tag {
                variant
            } else {
                let variant = variant.as_str().unwrap();
                let maybe_content = variants
                    .iter()
                    .find(|(variant_name, _)| variant_name == variant)
                    .map(|(_, maybe_data)| maybe_data.as_ref())
                    .unwrap()
                    .map(|data_schema| {
                        json_session_settings_to_settings(
                            &session_settings[variant],
                            &data_schema.content,
                        )
                    });
                json::json!({
                    "type": variant,
                    "content": maybe_content
                })
            }
        }

        SchemaNode::Optional { content, .. } => {
            if session_settings["set"].as_bool().unwrap() {
                json_session_settings_to_settings(&session_settings["content"], content)
            } else {
                json::Value::Null
            }
        }

        SchemaNode::Switch { content, .. } => {
            let state;
            let maybe_content;
            if session_settings["enabled"].as_bool().unwrap() {
                state = "enabled";
                maybe_content = Some(json_session_settings_to_settings(
                    &session_settings["content"],
                    content,
                ))
            } else {
                state = "disabled";
                maybe_content = None;
            }

            json::json!({
                "type": state,
                "content": maybe_content
            })
        }

        SchemaNode::Boolean { .. }
        | SchemaNode::Integer { .. }
        | SchemaNode::Float { .. }
        | SchemaNode::Text { .. } => session_settings.clone(),

        SchemaNode::Array(array_schema) => json::Value::Array(
            array_schema
                .iter()
                .enumerate()
                .map(|(idx, element_schema)| {
                    json_session_settings_to_settings(&session_settings[idx], element_schema)
                })
                .collect(),
        ),

        SchemaNode::Vector { .. } | SchemaNode::Dictionary { .. } => {
            session_settings["default"].clone()
        }
    }
}

// SessionDesc wrapper that saves settings.json and session.json on destruction.
pub struct SessionLock<'a> {
    session_desc: &'a mut SessionDesc,
    dir: &'a Path,
    update_author_id: Option<String>,
    update_type: SessionUpdateType,
}

impl Deref for SessionLock<'_> {
    type Target = SessionDesc;
    fn deref(&self) -> &SessionDesc {
        self.session_desc
    }
}

impl DerefMut for SessionLock<'_> {
    fn deref_mut(&mut self) -> &mut SessionDesc {
        self.session_desc
    }
}

impl Drop for SessionLock<'_> {
    fn drop(&mut self) {
        save_session(self.session_desc, &self.dir.join(SESSION_FNAME)).ok();
        info!(id: LogId::SessionUpdated {
            web_client_id: self.update_author_id.to_owned(),
            update_type: self.update_type
        });
    }
}

pub struct SessionManager {
    session_desc: SessionDesc,
    dir: PathBuf,
}

impl SessionManager {
    pub fn new(dir: &Path) -> Self {
        let session_path = dir.join(SESSION_FNAME);
        let session_desc = match fs::read_to_string(&session_path) {
            Ok(session_string) => {
                let json_value = json::from_str::<json::Value>(&session_string).unwrap();
                match json::from_value(json_value.clone()) {
                    Ok(session_desc) => session_desc,
                    Err(_) => {
                        fs::write(dir.join("session_old.json"), &session_string).ok();
                        let mut session_desc = SessionDesc::default();
                        match session_desc.merge_from_json(&json_value) {
                            Ok(_) => info!(
                                "{} {}",
                                "Session extrapolated successfully.",
                                "Old session.json is stored as session_old.json"
                            ),
                            Err(e) => error!(
                                "{} {} {}",
                                "Error while extrapolating session.",
                                "Old session.json is stored as session_old.json.",
                                e
                            ),
                        }
                        // not essential, but useful to avoid duplicated errors
                        save_session(&session_desc, &session_path).ok();

                        session_desc
                    }
                }
            }
            Err(_) => SessionDesc::default(),
        };

        Self {
            session_desc,
            dir: dir.to_owned(),
        }
    }

    pub fn get(&self) -> &SessionDesc {
        &self.session_desc
    }

    pub fn get_mut(
        &mut self,
        update_author_id: Option<String>,
        update_type: SessionUpdateType,
    ) -> SessionLock {
        SessionLock {
            session_desc: &mut self.session_desc,
            dir: &self.dir,
            update_author_id,
            update_type,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_to_settings() {
        let _settings = SessionDesc::default().to_settings();
    }

    // todo: add more tests
    #[test]
    fn test_session_extrapolation_trivial() {
        SessionDesc::default()
            .merge_from_json(&json::to_value(SessionDesc::default()).unwrap())
            .unwrap();
    }
}
