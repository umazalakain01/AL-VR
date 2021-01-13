use crate::*;
use gfx_hal::Instance;

#[cfg(windows)]
pub fn get_gpu_names() -> Vec<String> {
    let instance = gfx_backend_dx11::Instance::create("ALVR", 0).unwrap();
    let adapters = instance.enumerate_adapters();

    adapters
        .into_iter()
        .map(|a| a.info.name)
        .collect::<Vec<_>>()
}

pub fn get_screen_size() -> StrResult<(u32, u32)> {
    #[cfg(not(windows))]
    use winit::platform::unix::EventLoopExtUnix;
    #[cfg(windows)]
    use winit::platform::windows::EventLoopExtWindows;
    use winit::{window::*, *};

    let event_loop = event_loop::EventLoop::<Window>::new_any_thread();
    let size = trace_none!(trace_err!(WindowBuilder::new()
        .with_visible(false)
        .build(&event_loop))?
    .primary_monitor())?
    .size();
    Ok((size.width, size.height))
}
