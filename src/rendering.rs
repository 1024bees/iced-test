//! Utilities for rendering headless runners

use iced::Screenshot;
use iced_graphics::window;
use iced_winit::{
    application::{Application, State},
    settings::Window,
    Debug, Mode,
};

/// Render one frame of an application; this may only be called from testing utils
pub fn render_once<A, C>(application: &A, compositor_settings: C::Settings) -> Screenshot
where
    A: Application + 'static,
    C: window::Compositor<Renderer = A::Renderer> + 'static,
{
    use winit::event_loop::EventLoop;

    let event_loop: EventLoop<A::Message> = EventLoop::with_user_event();

    let window = Window::default()
        .into_builder(&application.title(), Mode::Hidden, None, None)
        .build(&event_loop)
        .expect("FAILURE IS NOT AN OPTION");
    let (mut compositor, mut renderer) =
        C::new(compositor_settings, Some(&window)).expect("FAILURE NOT ALLOWED!");
    let state = State::new(application, &window);
    let debug = Debug::new();

    compositor
        .present(
            &mut renderer,
            state.viewport(),
            state.background_color(),
            &debug.overlay(),
        )
        .expect("NO FAILURE");

    compositor.read().unwrap()
}
