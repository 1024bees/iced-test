//! Utilities for rendering headless runners
//
use super::screenshot::Screenshot;
use crate::runtime::{application::Application, settings::Window, Size};
use iced_graphics::window::Compositor;
use iced_native::user_interface::Cache;
use winit::platform::unix::EventLoopExtUnix;

use std::mem::{drop, ManuallyDrop};

use iced_winit::application::{build_user_interface, State};
use iced_winit::{Debug, Mode};
//
///// Render one frame of an application; this may only be called from testing utils
pub fn render_once<A>(application: &mut A, window_size: (u32, u32)) -> Screenshot
where
    A: Application<Renderer = iced_graphics::Renderer<crate::renderer::Backend>> + 'static,
{
    //#[cfg(feature = "glow")]
    //let ss = render_glutin(application, window_size);

    //#[cfg(not(feature = "glow"))]
    
    render_wgpu(application, window_size)
}
//

/// Winit implementation for rendering a single frame to a [`Screenshot`]
fn render_wgpu<A>(application: &mut A, window_size: (u32, u32)) -> Screenshot
where
    A: Application<Renderer = iced_graphics::Renderer<crate::renderer::Backend>> + 'static,
{
    use super::headless_compositors::wgpu::Compositor as HeadlessCompositor;
    use winit::event_loop::EventLoop;
    #[cfg(not(target_os = "ios"))]
    let event_loop: EventLoop<A::Message> = EventLoop::new_any_thread();
    #[cfg(target_os = "ios")]
    let event_loop: EventLoop<A::Message> = EventLoop::with_user_event();

    let renderer_settings = crate::renderer::Settings {
        ..crate::renderer::Settings::from_env()
    };

    let window = Window {
        size: window_size,
        ..Window::default()
    }
    .into_builder(&application.title(), Mode::Hidden, None, None)
    .build(&event_loop)
    .expect("FAILURE IS NOT AN OPTION");
    let (mut compositor, mut renderer) =
        HeadlessCompositor::new(renderer_settings, Some(&window)).expect("FAILURE NOT ALLOWED!");
    let state = State::new(application, &window);
    let mut debug = Debug::new();
    let mut surface = compositor.create_surface(&window);
    compositor.configure_surface(&mut surface, window_size.0, window_size.1);

    let mut user_interface = ManuallyDrop::new(build_user_interface(
        application,
        Cache::default(),
        &mut renderer,
        Size {
            width: window_size.0 as f32,
            height: window_size.1 as f32,
        },
        &mut debug,
    ));

    let _ = user_interface.draw(&mut renderer, iced::Point::default());
    window.request_redraw();

    compositor
        .present(
            &mut renderer,
            &mut surface,
            state.viewport(),
            state.background_color(),
            &debug.overlay(),
        )
        .expect("NO FAILURE");

    let ss = compositor.read().unwrap();
    drop(ManuallyDrop::into_inner(user_interface));
    ss
}
