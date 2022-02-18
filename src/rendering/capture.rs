//! Utilities for rendering headless runners
//
use super::headless::VirtualCompositor;
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
    let ss = render_wgpu(application, window_size);
    ss
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

//#[cfg(feature = "glow")]
///// Glutin implementation for rendering a single frame to a [`Screenshot`]; this compiles fine but
///// hasn't been tested yet
//fn render_glutin<A>(application: &mut A, window_size: (u32, u32)) -> Screenshot
//where
//    A: Application<Renderer = iced_graphics::Renderer<crate::renderer::Backend>> + 'static,
//{
//    use glutin::event_loop::EventLoop;
//
//    use glutin::ContextBuilder;
//    use iced_graphics::window::GLCompositor;
//    use iced_winit::application;
//
//    let renderer_settings = crate::renderer::Settings {
//        headless: true,
//        ..crate::renderer::Settings::from_env()
//    };
//    let window = Window {
//        size: window_size,
//        ..Window::default()
//    };
//
//    let mut debug = Debug::new();
//    debug.startup_started();
//
//    let mut event_loop: EventLoop<A::Message> = EventLoop::with_user_event();
//    let mut proxy = event_loop.create_proxy();
//
//    let subscription = application.subscription();
//
//    let context = {
//        let builder = window.into_builder(
//            &application.title(),
//            Mode::Hidden,
//            event_loop.primary_monitor(),
//            None,
//        );
//
//        let context = ContextBuilder::new()
//            .with_vsync(true)
//            .with_multisampling(crate::renderer::window::Compositor::sample_count(
//                &renderer_settings,
//            ) as u16)
//            .build_windowed(builder, &event_loop)
//            .unwrap();
//
//        #[allow(unsafe_code)]
//        unsafe {
//            context.make_current().expect("Make OpenGL context current")
//        }
//    };
//
//    #[allow(unsafe_code)]
//    let (mut compositor, renderer) = unsafe {
//        crate::renderer::window::Compositor::new(renderer_settings, |address| {
//            context.get_proc_address(address)
//        })
//        .unwrap()
//    };
//
//    let mut state = State::new(application, context.window());
//    let mut user_interface = ManuallyDrop::new(application::build_user_interface(
//        application,
//        Cache::default(),
//        &mut renderer,
//        state.logical_size(),
//        &mut debug,
//    ));
//    let _ = user_interface.draw(&mut renderer, iced::Point::default());
//    context.window().request_redraw();
//
//    compositor.present(
//        &mut renderer,
//        state.viewport(),
//        state.background_color(),
//        &debug.overlay(),
//    );
//
//    let ss = compositor.read().unwrap();
//    drop(ManuallyDrop::into_inner(user_interface));
//    ss
//}
