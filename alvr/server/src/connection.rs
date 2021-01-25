use crate::{ClientListAction, CLIENTS_UPDATED_NOTIFIER, SESSION_MANAGER};
use alvr_common::{audio::AudioSession, data::*, logging::*, sockets::*, *};
use futures::future::BoxFuture;
use nalgebra::Translation3;
use serde_json as json;
use settings_schema::Switch;
use std::{
    collections::HashMap,
    future,
    process::Command,
    sync::{mpsc as smpsc, Arc},
    time::Duration,
};
use tokio::{
    sync::{mpsc as tmpsc, Mutex},
    task, time,
};

const RETRY_CONNECT_INTERVAL: Duration = Duration::from_millis(500);
const NETWORK_KEEPALIVE_INTERVAL: Duration = Duration::from_secs(5);

fn align32(value: f32) -> u32 {
    ((value / 32.).floor() * 32.) as u32
}

async fn client_discovery() -> StrResult {
    let res = search_client_loop(|client_ip, handshake_packet| async move {
        crate::update_client_list(
            handshake_packet.hostname.clone(),
            ClientListAction::AddIfMissing {
                device_name: handshake_packet.device_name,
                ip: client_ip,
                certificate_pem: Some(handshake_packet.certificate_pem),
            },
        )
        .await;

        if let Some(connection_desc) = SESSION_MANAGER
            .lock()
            .get()
            .client_connections
            .get(&handshake_packet.hostname)
        {
            connection_desc.trusted
        } else {
            false
        }
    })
    .await;

    Err(res.err().unwrap_or_else(|| "".into()))
}

struct ConnectionInfo {
    control_sender: ControlSocketSender<ServerControlPacket>,
    control_receiver: ControlSocketReceiver<ClientControlPacket>,
    game_audio_config: Option<AudioConfig>,
}

async fn client_handshake() -> StrResult<ConnectionInfo> {
    let auto_trust_clients = SESSION_MANAGER
        .lock()
        .get()
        .to_settings()
        .connection
        .auto_trust_clients;
    let clients_info = SESSION_MANAGER
        .lock()
        .get()
        .client_connections
        .iter()
        .filter(|(_, client)| client.trusted || auto_trust_clients)
        .fold(HashMap::new(), |mut clients_info, (hostname, client)| {
            let id = PublicIdentity {
                hostname: hostname.clone(),
                certificate_pem: client.certificate_pem.clone(),
            };
            clients_info.extend(client.manual_ips.iter().map(|&ip| (ip, id.clone())));
            clients_info.insert(client.last_local_ip, id);
            clients_info
        });

    let maybe_pending_connection =
        sockets::begin_connecting_to_client(&clients_info.keys().cloned().collect::<Vec<_>>())
            .await;
    let PendingClientConnection {
        pending_socket,
        client_ip,
        server_ip,
        headset_info,
    } = maybe_pending_connection?;

    let settings = SESSION_MANAGER.lock().get().to_settings();

    let (eye_width, eye_height) = match settings.video.render_resolution {
        FrameSize::Scale(scale) => (
            headset_info.recommended_eye_width as f32 * scale,
            headset_info.recommended_eye_height as f32 * scale,
        ),
        FrameSize::Absolute { width, height } => (width as f32 / 2_f32, height as f32),
    };
    let video_eye_width = align32(eye_width);
    let video_eye_height = align32(eye_height);

    let (eye_width, eye_height) = match settings.video.recommended_target_resolution {
        FrameSize::Scale(scale) => (
            headset_info.recommended_eye_width as f32 * scale,
            headset_info.recommended_eye_height as f32 * scale,
        ),
        FrameSize::Absolute { width, height } => (width as f32 / 2_f32, height as f32),
    };
    let target_eye_width = align32(eye_width);
    let target_eye_height = align32(eye_height);

    let fps = {
        let mut best_match = 0_f32;
        let mut min_diff = f32::MAX;
        for rr in &headset_info.available_refresh_rates {
            let diff = (*rr - settings.video.preferred_fps).abs();
            if diff < min_diff {
                best_match = *rr;
                min_diff = diff;
            }
        }
        best_match
    };

    let controller_pose_offset = match settings.headset.controllers {
        Switch::Enabled(content) => {
            if content.clientside_prediction {
                0.
            } else {
                content.pose_time_offset
            }
        }
        Switch::Disabled => 0.,
    };

    if !headset_info
        .available_refresh_rates
        .contains(&settings.video.preferred_fps)
    {
        warn!("Chosen refresh rate not supported. Using {}Hz", fps);
    }

    let dashboard_url = format!(
        "http://{}:{}/",
        server_ip, settings.connection.web_server_port
    );

    let mut game_audio_config = None;
    if let Ok(reserved_data) = json::from_str::<json::Value>(&headset_info.reserved) {
        if let Switch::Enabled(audio_desc) = settings.audio.game_audio {
            if let Some(configs_json) = reserved_data.get("game_audio_configs") {
                if let Ok(sink_configs) =
                    json::from_value::<Vec<AudioConfigRange>>(configs_json.clone())
                {
                    // CPAL sometimes crashes if supported_audio_output_configs() (non async) is not
                    // called within a separate thread. This might be a bug of Tokio (non async
                    // functions do not have await points and should not be sent between threads).

                    // let source_configs = audio::supported_audio_output_configs(None)?;
                    let source_configs = trace_err!(
                        task::spawn_blocking({
                            let index = audio_desc.device_index;
                            move || audio::supported_audio_output_configs(index)
                        })
                        .await
                    )??;

                    game_audio_config = Some(audio::select_audio_config(
                        source_configs,
                        sink_configs,
                        audio_desc.preferred_config,
                    )?);
                }
            }
        }
    }

    #[derive(serde::Serialize)]
    struct ReservedData {
        game_audio_config: Option<AudioConfig>,
    }

    let client_config = ClientConfigPacket {
        session_desc: trace_err!(serde_json::to_string(SESSION_MANAGER.lock().get()))?,
        eye_resolution_width: video_eye_width,
        eye_resolution_height: video_eye_height,
        fps,
        dashboard_url: dashboard_url,
        reserved: trace_err!(json::to_string(&ReservedData {
            game_audio_config: game_audio_config.clone()
        }))?,
    };

    let (mut control_sender, control_receiver) =
        sockets::finish_connecting_to_client(pending_socket, client_config).await?;

    let session_settings = SESSION_MANAGER.lock().get().session_settings.clone();

    let new_openvr_config = OpenvrConfig {
        universe_id: settings.headset.universe_id,
        headset_serial_number: settings.headset.serial_number,
        headset_tracking_system_name: settings.headset.tracking_system_name,
        headset_model_number: settings.headset.model_number,
        headset_driver_version: settings.headset.driver_version,
        headset_manufacturer_name: settings.headset.manufacturer_name,
        headset_render_model_name: settings.headset.render_model_name,
        headset_registered_device_type: settings.headset.registered_device_type,
        eye_resolution_width: video_eye_width,
        eye_resolution_height: video_eye_height,
        target_eye_resolution_width: target_eye_width,
        target_eye_resolution_height: target_eye_height,
        enable_microphone: session_settings.audio.microphone.enabled,
        microphone_device: session_settings.audio.microphone.content.device.clone(),
        seconds_from_vsync_to_photons: settings.video.seconds_from_vsync_to_photons,
        client_buffer_size: settings.connection.client_recv_buffer_size,
        force_3dof: settings.headset.force_3dof,
        aggressive_keyframe_resend: settings.connection.aggressive_keyframe_resend,
        adapter_index: settings.video.adapter_index,
        codec: matches!(settings.video.codec, CodecType::HEVC) as _,
        refresh_rate: fps as _,
        encode_bitrate_mbs: settings.video.encode_bitrate_mbs,
        throttling_bitrate_bits: settings.connection.throttling_bitrate_bits,
        listen_port: settings.connection.listen_port,
        client_address: client_ip.to_string(),
        controllers_tracking_system_name: session_settings
            .headset
            .controllers
            .content
            .tracking_system_name
            .clone(),
        controllers_manufacturer_name: session_settings
            .headset
            .controllers
            .content
            .manufacturer_name
            .clone(),
        controllers_model_number: session_settings
            .headset
            .controllers
            .content
            .model_number
            .clone(),
        render_model_name_left_controller: session_settings
            .headset
            .controllers
            .content
            .render_model_name_left
            .clone(),
        render_model_name_right_controller: session_settings
            .headset
            .controllers
            .content
            .render_model_name_right
            .clone(),
        controllers_serial_number: session_settings
            .headset
            .controllers
            .content
            .serial_number
            .clone(),
        controllers_type: session_settings
            .headset
            .controllers
            .content
            .ctrl_type
            .clone(),
        controllers_registered_device_type: session_settings
            .headset
            .controllers
            .content
            .registered_device_type
            .clone(),
        controllers_input_profile_path: session_settings
            .headset
            .controllers
            .content
            .input_profile_path
            .clone(),
        controllers_mode_idx: session_settings.headset.controllers.content.mode_idx,
        controllers_enabled: session_settings.headset.controllers.enabled,
        position_offset: settings.headset.position_offset,
        tracking_frame_offset: settings.headset.tracking_frame_offset,
        controller_pose_offset,
        position_offset_left: session_settings
            .headset
            .controllers
            .content
            .position_offset_left,
        rotation_offset_left: session_settings
            .headset
            .controllers
            .content
            .rotation_offset_left,
        haptics_intensity: session_settings
            .headset
            .controllers
            .content
            .haptics_intensity,
        enable_foveated_rendering: session_settings.video.foveated_rendering.enabled,
        foveation_strength: session_settings.video.foveated_rendering.content.strength,
        foveation_shape: session_settings.video.foveated_rendering.content.shape,
        foveation_vertical_offset: session_settings
            .video
            .foveated_rendering
            .content
            .vertical_offset,
        enable_color_correction: session_settings.video.color_correction.enabled,
        brightness: session_settings.video.color_correction.content.brightness,
        contrast: session_settings.video.color_correction.content.contrast,
        saturation: session_settings.video.color_correction.content.saturation,
        gamma: session_settings.video.color_correction.content.gamma,
        sharpening: session_settings.video.color_correction.content.sharpening,
    };

    if SESSION_MANAGER.lock().get().openvr_config != new_openvr_config {
        SESSION_MANAGER
            .lock()
            .get_mut(None, SessionUpdateType::Other)
            .openvr_config = new_openvr_config;

        control_sender
            .send(&ServerControlPacket::Restarting)
            .await
            .ok();

        crate::notify_restart_driver();

        // waiting for execution canceling
        future::pending::<()>().await;
    }

    Ok(ConnectionInfo {
        control_sender,
        control_receiver,
        game_audio_config,
    })
}

// close stream on Drop (manual disconnection or execution canceling)
struct StreamCloseGuard;

impl Drop for StreamCloseGuard {
    fn drop(&mut self) {
        #[cfg(windows)]
        unsafe {
            crate::DeinitializeStreaming()
        };

        let on_disconnect_script = SESSION_MANAGER
            .lock()
            .get()
            .to_settings()
            .connection
            .on_disconnect_script;
        if !on_disconnect_script.is_empty() {
            info!(
                "Running on disconnect script (disconnect): {}",
                on_disconnect_script
            );
            if let Err(e) = Command::new(&on_disconnect_script)
                .env("ACTION", "disconnect")
                .spawn()
            {
                warn!("Failed to run disconnect script: {}", e);
            }
        }
    }
}

async fn connection_pipeline() -> StrResult {
    let connection_info = tokio::select! {
        maybe_info = client_handshake() => {
            match maybe_info {
                Ok(info) => info,
                Err(e) => {
                    // treat handshake problems not as an hard error
                    warn!("Handshake: {}", e);
                    return Ok(());
                }
            }
        }
        Err(e) = client_discovery() => return fmt_e!("Client discovery failed: {}", e),
        _ = CLIENTS_UPDATED_NOTIFIER.notified() => return Ok(()),
        else => unreachable!(),
    };

    log_id(LogId::ClientConnected);

    {
        let on_connect_script = SESSION_MANAGER
            .lock()
            .get()
            .to_settings()
            .connection
            .on_connect_script;

        if !on_connect_script.is_empty() {
            info!("Running on connect script (connect): {}", on_connect_script);
            if let Err(e) = Command::new(&on_connect_script)
                .env("ACTION", "connect")
                .spawn()
            {
                warn!("Failed to run connect script: {}", e);
            }
        }
    }

    let ConnectionInfo {
        control_sender,
        mut control_receiver,
        game_audio_config,
    } = connection_info;

    let control_sender = Arc::new(Mutex::new(control_sender));

    #[cfg(windows)]
    unsafe {
        crate::InitializeStreaming()
    };

    let _stream_guard = StreamCloseGuard;

    let game_audio_desc = SESSION_MANAGER.lock().get().to_settings().audio.game_audio;
    let (_destroy_game_audio_stream_notifier, destroy_game_audio_stream_receiver) =
        smpsc::channel::<()>();

    let game_audio_loop: BoxFuture<_> =
        if let (Switch::Enabled(desc), Some(config)) = (game_audio_desc, game_audio_config) {
            let (sender, mut receiver) = tmpsc::unbounded_channel();
            let control_sender = control_sender.clone();

            // AudioSession is !Send, so keep it in a separate thread.
            Box::pin(futures::future::join(
                task::spawn_blocking(move || {
                    let _audio_stream_guard = Some(AudioSession::start_recording(
                        desc.device_index,
                        config,
                        true,
                        sender,
                    )?);

                    // notified when _destroy_game_audio_stream_notifier goes out of scope
                    destroy_game_audio_stream_receiver.recv().ok();
                    StrResult::Ok(())
                }),
                async move {
                    while let Some(data) = receiver.recv().await {
                        control_sender
                            .lock()
                            .await
                            .send(&ServerControlPacket::ReservedBuffer(data))
                            .await?;
                    }

                    StrResult::Ok(())
                },
            ))
        } else {
            Box::pin(future::pending())
        };

    let keepalive_sender_loop = {
        let control_sender = control_sender.clone();
        async move {
            loop {
                control_sender
                    .lock()
                    .await
                    .send(&ServerControlPacket::Reserved(
                        "{ \"keepalive\": true }".into(),
                    ))
                    .await
                    .ok();
                time::sleep(NETWORK_KEEPALIVE_INTERVAL).await;
            }
        }
    };

    let control_loop = async move {
        loop {
            match control_receiver.recv().await {
                Ok(ClientControlPacket::PlayspaceSync(packet)) => {
                    let transform = packet.rotation * Translation3::from(packet.position.coords);
                    // transposition is done to switch from column major to row major
                    let matrix_transp = transform.to_matrix().transpose();

                    let perimeter_points = if let Some(perimeter_points) = packet.perimeter_points {
                        perimeter_points.iter().map(|p| [p[0], p[2]]).collect()
                    } else {
                        vec![]
                    };

                    #[cfg(windows)]
                    unsafe {
                        crate::SetChaperone(
                            matrix_transp.as_ptr(),
                            packet.area_width,
                            packet.area_height,
                            perimeter_points.as_ptr() as _,
                            perimeter_points.len() as _,
                        )
                    };
                }
                Ok(ClientControlPacket::RequestIDR) => unsafe {
                    #[cfg(windows)]
                    crate::RequestIDR()
                },
                Ok(ClientControlPacket::Reserved(_))
                | Ok(ClientControlPacket::ReservedBuffer(_)) => (),
                Err(e) => {
                    log_id(LogId::ClientDisconnected);
                    info!("Client disconnected. Cause: {}", e);
                    break;
                }
            }
        }

        Ok(())
    };

    tokio::select! {
        _ = crate::RESTART_NOTIFIER.notified() => {
            control_sender
                .lock()
                .await
                .send(&ServerControlPacket::Restarting)
                .await
                .ok();

            Ok(())
        }
        _ = game_audio_loop => Ok(()),
        _ = keepalive_sender_loop => Ok(()),
        res = control_loop => res,
    }
}

pub async fn connection_lifecycle_loop() {
    loop {
        tokio::join!(
            async {
                show_err(connection_pipeline().await);
            },
            time::sleep(RETRY_CONNECT_INTERVAL),
        );
    }
}
