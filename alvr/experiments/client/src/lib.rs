mod connection;
mod scene;
mod storage;
mod streaming_compositor;
mod video_decoder;
mod xr;

use crate::xr::{XrContext, XrEvent, XrPresentationGuard, XrSession};
use alvr_common::{
    glam::{Quat, UVec2, Vec3},
    log,
    prelude::*,
    Fov,
};
use alvr_graphics::{wgpu::Texture, GraphicsContext};
use alvr_session::{CodecType, TrackingSpace};
use alvr_sockets::VideoFrameHeaderPacket;
use connection::VideoStreamingComponents;
use parking_lot::{Mutex, RwLock};
use scene::Scene;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};
use streaming_compositor::StreamingCompositor;
use tokio::{
    runtime::{self, Runtime},
    sync::Notify,
};
use video_decoder::VideoDecoder;

const MAX_RENDERING_LOOP_FAILS: usize = 5;

// Timeout stream after this portion of frame interval. must be less than 1, so Phase Sync can
// compensate for it.
const FRAME_TIMEOUT_MULTIPLIER: f32 = 0.9;

#[derive(Clone)]
pub struct ViewConfig {
    orientation: Quat,
    position: Vec3,
    fov: Fov,
}

#[cfg_attr(target_os = "android", ndk_glue::main)]
pub fn main() {
    env_logger::init();
    log::error!("enter main");

    show_err(run());

    #[cfg(target_os = "android")]
    ndk_glue::native_activity().finish();
}

fn run() -> StrResult {
    let xr_context = Arc::new(XrContext::new());

    let graphics_context = Arc::new(xr::create_graphics_context(&xr_context)?);

    let mut scene = Scene::new(Arc::clone(&graphics_context))?;

    let xr_session = XrSession::new(
        Arc::clone(&xr_context),
        Arc::clone(&graphics_context),
        UVec2::new(1, 1),
        &[],
        vec![],
        TrackingSpace::Local,
        openxr::EnvironmentBlendMode::OPAQUE,
    )?;
    let xr_session = Arc::new(RwLock::new(Some(xr_session)));

    let video_streaming_components = Arc::new(RwLock::new(None));

    let standby_status = Arc::new(AtomicBool::new(true));
    let idr_request_notifier = Arc::new(Notify::new());

    let runtime = trace_err!(Runtime::new())?;
    runtime.spawn(connection::connection_lifecycle_loop(
        xr_context,
        graphics_context,
        Arc::clone(&xr_session),
        Arc::clone(&video_streaming_components),
        Arc::clone(&standby_status),
        Arc::clone(&idr_request_notifier),
    ));

    let mut fails_count = 0;
    loop {
        let res = show_err(rendering_loop(
            &mut scene,
            Arc::clone(&xr_session),
            Arc::clone(&video_streaming_components),
            Arc::clone(&standby_status),
            Arc::clone(&idr_request_notifier),
        ));

        if res.is_some() {
            break Ok(());
        } else {
            thread::sleep(Duration::from_millis(500));

            fails_count += 1;

            if fails_count == MAX_RENDERING_LOOP_FAILS {
                log::error!("Rendering loop failed {} times. Terminating.", fails_count);
                break Ok(());
            }
        }
    }
}

fn rendering_loop(
    scene: &mut Scene,
    xr_session: Arc<RwLock<Option<XrSession>>>,
    video_streaming_components: Arc<RwLock<Option<VideoStreamingComponents>>>,
    standby_status: Arc<AtomicBool>,
    idr_request_notifier: Arc<Notify>,
) -> StrResult {
    // this is used to keep the last stream frame in place when the stream is stuck
    let old_stream_view_configs = vec![];

    loop {
        let xr_session_rlock = xr_session.read();
        let xr_session = xr_session_rlock.as_ref().unwrap();
        let mut presentation_guard = match xr_session.begin_frame()? {
            XrEvent::ShouldRender(guard) => {
                if standby_status.load(Ordering::Relaxed) {
                    idr_request_notifier.notify_one();
                    standby_status.store(false, Ordering::Relaxed);
                }

                guard
            }
            XrEvent::Idle => {
                standby_status.store(true, Ordering::Relaxed);
                continue;
            }
            XrEvent::Shutdown => return Ok(()),
        };

        let maybe_stream_view_configs =
            video_streaming_pipeline(&video_streaming_components, &mut presentation_guard);
        presentation_guard.scene_view_configs =
            if let Some(stream_view_configs) = maybe_stream_view_configs.clone() {
                stream_view_configs
            } else {
                old_stream_view_configs.clone()
            };

        let scene_input = xr_session.get_scene_input()?;

        scene.update(
            scene_input.left_pose_input,
            scene_input.right_pose_input,
            scene_input.buttons,
            maybe_stream_view_configs.is_some(),
            scene_input.is_focused,
        );
        presentation_guard.scene_view_configs = scene_input.view_configs;

        for (index, acquired_swapchain) in presentation_guard
            .acquired_scene_swapchains
            .iter_mut()
            .enumerate()
        {
            scene.render(
                &presentation_guard.scene_view_configs[index],
                Arc::clone(&acquired_swapchain.texture_view),
                acquired_swapchain.size,
            )
        }
    }
}

// Returns true if stream is updated for the current frame
fn video_streaming_pipeline(
    streaming_components: &Arc<RwLock<Option<VideoStreamingComponents>>>,
    presentation_guard: &mut XrPresentationGuard,
) -> Option<Vec<ViewConfig>> {
    if let Some(streaming_components) = streaming_components.read().as_ref() {
        let decoder_target = streaming_components.compositor.input_texture();

        let timeout = Duration::from_micros(
            (presentation_guard.predicted_frame_interval.as_micros() as f32
                * FRAME_TIMEOUT_MULTIPLIER) as _,
        );
        let frame_metadata = get_video_frame_data(streaming_components, decoder_target, timeout)?;

        let compositor_target = presentation_guard
            .acquired_stream_swapchains
            .iter()
            .map(|swapchain| Arc::clone(&swapchain.texture_view))
            .collect::<Vec<_>>();

        streaming_components.compositor.render(&compositor_target);

        // presentation_guard.display_timestamp = frame_metadata.timestamp;

        // Some(frame_metadata.view_configs)
        None
    } else {
        None
    }
}

// Dequeue decoded frames and metadata and makes sure they are on the same latest timestamp
fn get_video_frame_data(
    streaming_components: &VideoStreamingComponents,
    decoder_target: &Texture,
    timeout: Duration,
) -> Option<VideoFrameHeaderPacket> {
    let mut frame_metadata = streaming_components
        .frame_metadata_receiver
        .recv_timeout(timeout)
        .ok()?;

    let mut decoder_timestamps = vec![];
    for decoder in &streaming_components.video_decoders {
        decoder_timestamps.push(
            decoder
                .get_output_frame(decoder_target, 0, timeout)
                .ok()
                .flatten()?,
        );
    }

    // let greatest_timestamp = decoder_timestamps
    //     .iter()
    //     .cloned()
    //     .fold(frame_metadata.timestamp, Duration::max);

    // while frame_metadata.timestamp < greatest_timestamp {
    //     frame_metadata = streaming_components
    //         .frame_metadata_receiver
    //         .recv_timeout(timeout)
    //         .ok()?;
    // }

    // for (mut timestamp, decoder) in decoder_timestamps
    //     .into_iter()
    //     .zip(streaming_components.video_decoders.iter())
    // {
    //     while timestamp < greatest_timestamp {
    //         timestamp = decoder
    //             .get_output_frame(decoder_target, 0, timeout)
    //             .ok()
    //             .flatten()?;
    //     }
    // }

    Some(frame_metadata)
}
