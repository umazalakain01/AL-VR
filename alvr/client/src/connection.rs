use alvr_common::{data::*, logging::*, sockets::ControlSocket, *};
use jni::{
    objects::{GlobalRef, JClass},
    JavaVM,
};
use serde_json as json;
use settings_schema::Switch;
use std::{
    ffi::CString,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::{
    sync::broadcast,
    time::{self, Instant},
};

const SERVER_RESTART_MESSAGE: &str = "The server is restarting\nPlease wait...";
const SERVER_DISCONNECTED_MESSAGE: &str = "The server has disconnected.";

// close stream on Drop (manual disconnection or execution canceling)
struct StreamCloseGuard {
    is_connected: Arc<AtomicBool>,
    java_vm: Arc<JavaVM>,
    activity_ref: Arc<GlobalRef>,
}

impl Drop for StreamCloseGuard {
    fn drop(&mut self) {
        // if let Ok(env) = self.java_vm.attach_current_thread() {
        //     unsafe { crate::disconnectSocket(env.get_native_interface() as _) };
        // }
        self.is_connected.store(false, Ordering::Relaxed)
    }
}

async fn setLoadingMessage(java_vm: &JavaVM, activity_ref: &GlobalRef, message: &str) -> StrResult {
    // Note: env = java_vm.attach_current_thread() cannot be saved into a variable because it is
    // not Send (compile error). This makes sense since tokio could move the execution of this
    // task to another thread at any time, and env is valid only within a specific thread. For
    // the same reason, other jni objects cannot be made into variables and the arguments must
    // be created inline within the call_method() call
    trace_err!(trace_err!(java_vm.attach_current_thread())?.call_method(
        activity_ref,
        "setLoadingMessage",
        "(Ljava/lang/String;)V",
        &[trace_err!(trace_err!(java_vm.attach_current_thread())?.new_string(message))?.into()],
    ))?;

    Ok(())
}

async fn try_connect(
    headset_info: &HeadsetInfoPacket,
    device_name: String,
    private_identity: &PrivateIdentity,
    java_vm: Arc<JavaVM>,
    activity_ref: Arc<GlobalRef>,
    nal_class_ref: Arc<GlobalRef>,
) -> StrResult {
    let (mut control_socket, config_packet) = trace_err!(
        ControlSocket::connect_to_server(
            &headset_info,
            device_name,
            private_identity.hostname.clone(),
            private_identity.certificate_pem.clone(),
        )
        .await
    )?;

    let baseline_settings = {
        let mut session_desc = SessionDesc::default();
        session_desc.merge_from_json(&trace_err!(json::from_str(&config_packet.session_desc))?)?;
        session_desc.to_settings()
    };

    let is_connected = Arc::new(AtomicBool::new(true));
    let _stream_guard = StreamCloseGuard {
        is_connected: is_connected.clone(),
        java_vm: java_vm.clone(),
        activity_ref: activity_ref.clone(),
    };

    trace_err!(trace_err!(java_vm.attach_current_thread())?.call_method(
        &*activity_ref,
        "onServerConnected",
        "(IIIZIZIFFFILjava/lang/String;)V",
        &[
            (config_packet.eye_resolution_width as i32 * 2).into(),
            (config_packet.eye_resolution_height as i32).into(),
            (matches!(baseline_settings.video.codec, CodecType::HEVC) as i32).into(),
            baseline_settings
                .video
                .client_request_realtime_decoder
                .into(),
            (config_packet.fps as i32).into(),
            matches!(baseline_settings.audio.microphone, Switch::Enabled(_)).into(),
            (matches!(
                baseline_settings.video.foveated_rendering,
                Switch::Enabled(_)
            ) as i32)
                .into(),
            (if let Switch::Enabled(foveation_vars) = &baseline_settings.video.foveated_rendering {
                foveation_vars.strength
            } else {
                0_f32
            })
            .into(),
            (if let Switch::Enabled(foveation_vars) = &baseline_settings.video.foveated_rendering {
                foveation_vars.shape
            } else {
                1_f32
            })
            .into(),
            (if let Switch::Enabled(foveation_vars) = &baseline_settings.video.foveated_rendering {
                foveation_vars.vertical_offset
            } else {
                0_f32
            })
            .into(),
            (matches!(
                baseline_settings.headset.tracking_space,
                TrackingSpace::Stage
            ) as i32)
                .into(),
            trace_err!(
                trace_err!(java_vm.attach_current_thread())?.new_string(config_packet.web_gui_url)
            )?
            .into()
        ],
    ))?;

    info!("Connected to server");

    // setup stream loops

    // The main stream loop must be run in a normal thread, because it needs to access the JNI env
    // many times per second. If using a future I'm forced to attach and detach the env continuously.
    // When the parent function gets canceled, this loop will run to finish.
    let ip_cstring = CString::new(control_socket.peer_ip().to_string()).unwrap();
    let stream_socket_loop = tokio::task::spawn_blocking({
        let java_vm = java_vm.clone();
        let activity_ref = activity_ref.clone();
        let nal_class_ref = nal_class_ref.clone();
        move || -> StrResult {
            let env = trace_err!(java_vm.attach_current_thread())?;
            // let env_ptr = env.get_native_interface() as _;
            let activity_obj = activity_ref.as_obj();
            let nal_class: JClass = nal_class_ref.as_obj().into();

            unsafe {
                crate::initializeSocket(env.get_native_interface() as _, *activity_obj as _, **nal_class as _);
                crate::connectSocket(
                    ip_cstring.as_ptr(),
                    matches!(baseline_settings.video.codec, CodecType::HEVC) as _,
                    baseline_settings.connection.client_recv_buffer_size as _,
                );

                while is_connected.load(Ordering::Relaxed) {
                    crate::runSocketLoopIter();
                }

                crate::disconnectSocket(env.get_native_interface() as _);
                crate::closeSocket(env.get_native_interface() as _);
            }

            Ok(())
        }
    });

    let tracking_interval = Duration::from_secs_f32(1_f32 / (config_packet.fps * 3_f32));
    let tracking_loop = async move {
        let mut deadline = Instant::now();
        loop {
            unsafe { crate::onTrackingNative() };
            deadline += tracking_interval;
            time::delay_until(deadline).await;
        }
    };

    let control_loop = async move {
        loop {
            match control_socket.recv().await {
                Ok(ServerControlPacket::Restarting) => {
                    info!("Server restarting");
                    setLoadingMessage(&*java_vm, &*activity_ref, SERVER_RESTART_MESSAGE).await?;
                    break Ok(());
                }
                Ok(ServerControlPacket::Reserved(_))
                | Ok(ServerControlPacket::ReservedBuffer(_)) => (),
                Err(e) => {
                    info!("Server disconnected. Cause: {}", e);
                    setLoadingMessage(&*java_vm, &*activity_ref, SERVER_DISCONNECTED_MESSAGE)
                        .await?;
                    break Ok(());
                }
            }
        }
    };

    error!("starting loops");

    tokio::select! {
        res = stream_socket_loop => trace_err!(res)?,
        res = tracking_loop => res,
        res = control_loop => res,
    }
}

pub async fn connection_lifecycle_loop(
    headset_info: HeadsetInfoPacket,
    device_name: &str,
    private_identity: PrivateIdentity,
    on_stream_stop_notifier: broadcast::Sender<()>,
    java_vm: Arc<JavaVM>,
    activity_ref: Arc<GlobalRef>,
    nal_class_ref: Arc<GlobalRef>,
) {
    let mut on_stream_stop_receiver = on_stream_stop_notifier.subscribe();

    // this loop has no exit, but the execution can be halted by the caller with tokio::select!{}
    loop {
        let try_connect_future = show_err_async(try_connect(
            &headset_info,
            device_name.to_owned(),
            &private_identity,
            java_vm.clone(),
            activity_ref.clone(),
            nal_class_ref.clone(),
        ));

        tokio::select! {
            _ = try_connect_future => (),
            _ = on_stream_stop_receiver.recv() => (),
        }
    }
}
