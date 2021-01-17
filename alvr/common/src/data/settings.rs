use serde::{Deserialize, Serialize};
use settings_schema::*;

#[derive(SettingsSchema, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type", content = "content")]
pub enum FrameSize {
    #[schema(min = 0.25, max = 2., step = 0.01)]
    Scale(f32),

    Absolute {
        #[schema(min = 32, step = 32)]
        width: u32,
        #[schema(min = 32, step = 32)]
        height: u32,
    },
}

#[derive(SettingsSchema, Serialize, Deserialize, PartialEq, Default, Clone)]
pub struct Fov {
    #[schema(min = 0., max = 90., step = 0.1, gui = "UpDown")]
    pub left: f32,

    #[schema(min = 0., max = 90., step = 0.1, gui = "UpDown")]
    pub right: f32,

    #[schema(min = 0., max = 90., step = 0.1, gui = "UpDown")]
    pub top: f32,

    #[schema(min = 0., max = 90., step = 0.1, gui = "UpDown")]
    pub bottom: f32,
}

#[derive(SettingsSchema, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FoveatedRenderingDesc {
    #[schema(min = 0.5, max = 10., step = 0.1)]
    pub strength: f32,

    #[schema(advanced, min = 0.5, max = 2., step = 0.1)]
    pub shape: f32,

    #[schema(min = -0.05, max = 0.05, step = 0.001)]
    pub vertical_offset: f32,
}

#[derive(SettingsSchema, Serialize, Deserialize)]
pub struct ColorCorrectionDesc {
    #[schema(min = -1., max = 1., step = 0.01)]
    pub brightness: f32,

    #[schema(min = -1., max = 1., step = 0.01)]
    pub contrast: f32,

    #[schema(min = -1., max = 1., step = 0.01)]
    pub saturation: f32,

    #[schema(min = 0., max = 5., step = 0.01)]
    pub gamma: f32,

    #[schema(min = -1., max = 5., step = 0.01)]
    pub sharpening: f32,
}

// Note: This enum cannot be converted to camelCase due to a inconsistency between generation and
// validation: "hevc" vs "hEVC".
// This is caused by serde and settings-schema using different libraries for casing conversion
// todo: don't use casing conversion also for all other structs and enums
#[derive(SettingsSchema, Serialize, Deserialize, Debug, Copy, Clone)]
#[serde(tag = "type", content = "content")]
#[repr(u8)]
pub enum CodecType {
    H264,
    HEVC,
}

#[derive(SettingsSchema, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "content")]
#[repr(u8)]
pub enum TrackingSpace {
    Local,
    Stage,
}

#[derive(SettingsSchema, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoDesc {
    #[schema(advanced)]
    pub adapter_index: u32,

    // Dropdown with 25%, 50%, 75%, 100%, 125%, 150% etc or custom
    // Should set renderResolution (always in scale mode).
    // When the user sets a resolution not obtainable with the preset scales, set the dropdown to
    // custom.
    // Warping compensation is already applied by the web server and driver
    #[schema(placeholder = "resolution_dropdown")]
    //
    #[schema(advanced)]
    pub render_resolution: FrameSize,

    #[schema(advanced)]
    pub recommended_target_resolution: FrameSize,

    #[schema(placeholder = "display_refresh_rate")]
    //
    #[schema(advanced, min = 60.0, max = 90.0)]
    pub preferred_fps: f32,

    pub codec: CodecType,

    #[schema(advanced)]
    pub client_request_realtime_decoder: bool,

    #[schema(min = 1, max = 500)]
    pub encode_bitrate_mbs: u64,

    #[schema(advanced)]
    pub seconds_from_vsync_to_photons: f32,

    pub foveated_rendering: Switch<FoveatedRenderingDesc>,
    pub color_correction: Switch<ColorCorrectionDesc>,
}

#[derive(SettingsSchema, Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub enum SampleFormat {
    Signed16Bit,
    Unsigned16Bit,
    Signed32Bit,
}

impl SampleFormat {
    pub fn to_cpal(self) -> cpal::SampleFormat {
        match self {
            Self::Signed16Bit => cpal::SampleFormat::I16,
            Self::Unsigned16Bit => cpal::SampleFormat::U16,
            Self::Signed32Bit => cpal::SampleFormat::F32,
        }
    }

    pub fn from_cpal(format: cpal::SampleFormat) -> Self {
        match format {
            cpal::SampleFormat::I16 => Self::Signed16Bit,
            cpal::SampleFormat::U16 => Self::Unsigned16Bit,
            cpal::SampleFormat::F32 => Self::Signed32Bit,
        }
    }
}

#[derive(SettingsSchema, Serialize, Deserialize, PartialEq, Debug)]
pub struct AudioConfig {
    pub preferred_channels_count: u16,
    pub preferred_sample_rate: u32,
    pub preferred_buffer_size: Option<u32>,
    pub preferred_sample_format: SampleFormat,
    pub max_buffer_count_extra: u64,
}

#[derive(SettingsSchema, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputAudioDesc {
    // deviceDropdown should poll the available audio devices and set "device"
    #[schema(placeholder = "device_dropdown")]
    //
    #[schema(advanced)]
    pub device: String,

    pub mute_when_streaming: bool,
}

#[derive(SettingsSchema, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InputAudioDesc {
    // deviceDropdown should poll the available audio devices and set "device"
    #[schema(placeholder = "device_dropdown")]
    //
    #[schema(advanced)]
    pub device: String,
}

#[derive(SettingsSchema, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioSection {
    pub game_audio: Switch<OutputAudioDesc>,
    pub microphone: Switch<InputAudioDesc>,
}

#[derive(SettingsSchema, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllersDesc {
    // Dropdown:
    // Oculus Rift S
    // Oculus Rift S (no handtracking pinch)
    // Valve Index
    // Valve Index (no handtracking pinch)
    // modeIdx and the following strings must be set accordingly
    #[schema(placeholder = "controller_mode")]
    //
    #[schema(advanced)]
    pub mode_idx: i32,

    #[schema(advanced)]
    pub tracking_system_name: String,

    #[schema(advanced)]
    pub manufacturer_name: String,

    #[schema(advanced)]
    pub model_number: String,

    #[schema(advanced)]
    pub render_model_name_left: String,

    #[schema(advanced)]
    pub render_model_name_right: String,

    #[schema(advanced)]
    pub serial_number: String,

    #[schema(advanced)]
    pub ctrl_type: String,

    #[schema(advanced)]
    pub registered_device_type: String,

    #[schema(advanced)]
    pub input_profile_path: String,

    #[schema(placeholder = "tracking_speed")]
    //
    #[schema(advanced)]
    pub pose_time_offset: f32,

    #[schema(advanced)]
    pub clientside_prediction: bool,

    #[schema(advanced)]
    pub position_offset_left: [f32; 3],

    #[schema(advanced)]
    pub rotation_offset_left: [f32; 3],

    #[schema(min = 0., max = 5., step = 0.1)]
    pub haptics_intensity: f32,
}

#[derive(SettingsSchema, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeadsetDesc {
    #[schema(advanced)]
    pub mode_idx: u64,

    #[schema(advanced)]
    pub universe_id: u64,

    // Oculus Rift S or HTC Vive. Should all the following strings accordingly
    #[schema(placeholder = "headset_emulation_mode")]
    //
    #[schema(advanced)]
    pub serial_number: String,

    #[schema(advanced)]
    pub tracking_system_name: String,

    #[schema(advanced)]
    pub model_number: String,

    #[schema(advanced)]
    pub driver_version: String,

    #[schema(advanced)]
    pub manufacturer_name: String,

    #[schema(advanced)]
    pub render_model_name: String,

    #[schema(advanced)]
    pub registered_device_type: String,

    pub tracking_frame_offset: i32,

    #[schema(advanced)]
    pub position_offset: [f32; 3],

    pub force_3dof: bool,

    pub controllers: Switch<ControllersDesc>,

    pub tracking_space: TrackingSpace,

    #[schema(advanced)]
    pub extra_latency_mode: bool,
}

#[derive(SettingsSchema, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionDesc {
    #[schema(advanced)]
    pub auto_trust_clients: bool,

    #[schema(advanced, min = 1024, max = 65535)]
    pub web_server_port: u16,

    #[schema(advanced)]
    pub listen_port: u16,

    // If disableThrottling=true, set throttlingBitrateBits to 0,
    // Given audioBitrate=2000'000:
    // If false, set throttlingBitrateBits=encodeBitrateMbs * 1000'000 * 3 / 2 + audioBitrate
    #[schema(placeholder = "disable_throttling")]
    //
    #[schema(advanced)]
    pub throttling_bitrate_bits: u64,

    // clientRecvBufferSize=max(encodeBitrateMbs * 2 + bufferOffset, 0)
    #[schema(placeholder = "buffer_offset")]
    //
    #[schema(advanced)]
    pub client_recv_buffer_size: u64,

    pub aggressive_keyframe_resend: bool,

    #[schema(advanced)]
    pub enable_fec: bool,
}

#[derive(SettingsSchema, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type", content = "content")]
pub enum Theme {
    SystemDefault,
    Classic,
    Darkly,
}

#[derive(SettingsSchema, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type", content = "content")]
pub enum UpdateChannel {
    NoUpdates,
    Stable,
    Beta,
    Nightly,
}

#[derive(SettingsSchema, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type", content = "content")]
pub enum LogLevel {
    Error,
    Warning,
    Info,
    Debug,
}

#[derive(SettingsSchema, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtraDesc {
    pub theme: Theme,
    pub client_dark_mode: bool,
    pub revert_confirm_dialog: bool,
    pub restart_confirm_dialog: bool,
    pub prompt_before_update: bool,
    pub update_channel: UpdateChannel,
    pub log_to_disk: bool,

    #[schema(advanced)]
    pub notification_level: LogLevel,
    #[schema(advanced)]
    pub exclude_notifications_without_id: bool,
}

#[derive(SettingsSchema, Serialize, Deserialize)]
pub struct Settings {
    pub video: VideoDesc,
    pub audio: AudioSection,
    pub headset: HeadsetDesc,
    pub connection: ConnectionDesc,
    pub extra: ExtraDesc,
}

pub fn session_settings_default() -> SettingsDefault {
    SettingsDefault {
        video: VideoDescDefault {
            adapter_index: 0,
            preferred_fps: 72_f32,
            render_resolution: FrameSizeDefault {
                variant: FrameSizeDefaultVariant::Scale,
                Scale: 0.75,
                Absolute: FrameSizeAbsoluteDefault {
                    width: 2880,
                    height: 1600,
                },
            },
            recommended_target_resolution: FrameSizeDefault {
                variant: FrameSizeDefaultVariant::Scale,
                Scale: 0.75,
                Absolute: FrameSizeAbsoluteDefault {
                    width: 2880,
                    height: 1600,
                },
            },
            seconds_from_vsync_to_photons: 0.005,
            foveated_rendering: SwitchDefault {
                enabled: true,
                content: FoveatedRenderingDescDefault {
                    strength: 2.,
                    shape: 1.5,
                    vertical_offset: 0.,
                },
            },
            color_correction: SwitchDefault {
                enabled: false,
                content: ColorCorrectionDescDefault {
                    brightness: 0.,
                    contrast: 0.,
                    saturation: 0.,
                    gamma: 1.,
                    sharpening: 0.,
                },
            },
            codec: CodecTypeDefault {
                variant: CodecTypeDefaultVariant::H264,
            },
            client_request_realtime_decoder: true,
            encode_bitrate_mbs: 30,
        },
        audio: AudioSectionDefault {
            game_audio: SwitchDefault {
                enabled: true,
                content: OutputAudioDescDefault {
                    device: "".into(),
                    mute_when_streaming: true,
                },
            },
            microphone: SwitchDefault {
                enabled: false,
                content: InputAudioDescDefault { device: "".into() },
            },
        },
        headset: HeadsetDescDefault {
            mode_idx: 2,
            universe_id: 2,
            serial_number: "1WMGH000XX0000".into(),
            tracking_system_name: "oculus".into(),
            model_number: "Miramar".into(),
            driver_version: "1.55.0".into(),
            manufacturer_name: "Oculus".into(),
            render_model_name: "generic_hmd".into(),
            registered_device_type: "oculus/1WMGH000XX0000".into(),
            tracking_frame_offset: 0,
            position_offset: [0., 0., 0.],
            force_3dof: false,
            controllers: SwitchDefault {
                enabled: true,
                content: ControllersDescDefault {
                    mode_idx: 7,
                    tracking_system_name: "oculus".into(),
                    manufacturer_name: "Oculus".into(),
                    model_number: "Miramar".into(),
                    render_model_name_left: "oculus_quest2_controller_left".into(),
                    render_model_name_right: "oculus_quest2_controller_right".into(),
                    serial_number: "1WMGH000XX0000_Controller".into(),
                    ctrl_type: "oculus_touch".into(),
                    registered_device_type: "oculus/1WMGH000XX0000_Controller".into(),
                    input_profile_path: "{oculus}/input/touch_profile.json".into(),
                    pose_time_offset: 0.01,
                    clientside_prediction: false,
                    position_offset_left: [-0.007, 0.005, -0.053],
                    rotation_offset_left: [36., 0., 0.],
                    haptics_intensity: 1.,
                },
            },
            tracking_space: TrackingSpaceDefault {
                variant: TrackingSpaceDefaultVariant::Local,
            },
            extra_latency_mode: true,
        },
        connection: ConnectionDescDefault {
            auto_trust_clients: cfg!(debug_assertions),
            web_server_port: 8082,
            listen_port: 9944,
            throttling_bitrate_bits: 30_000_000 * 3 / 2 + 2_000_000,
            client_recv_buffer_size: 60_000,
            aggressive_keyframe_resend: false,
            enable_fec: true,
        },
        extra: ExtraDescDefault {
            theme: ThemeDefault {
                variant: ThemeDefaultVariant::SystemDefault,
            },
            client_dark_mode: false,
            revert_confirm_dialog: true,
            restart_confirm_dialog: true,
            prompt_before_update: !cfg!(feature = "nightly"),
            update_channel: UpdateChannelDefault {
                variant: if cfg!(feature = "nightly") {
                    UpdateChannelDefaultVariant::Nightly
                } else {
                    UpdateChannelDefaultVariant::Stable
                },
            },
            log_to_disk: cfg!(debug_assertions),
            notification_level: LogLevelDefault {
                variant: if cfg!(debug_assertions) {
                    LogLevelDefaultVariant::Info
                } else {
                    LogLevelDefaultVariant::Warning
                },
            },
            exclude_notifications_without_id: false,
        },
    }
}
