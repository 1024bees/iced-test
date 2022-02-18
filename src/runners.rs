//! Interfaces for running applicationss end-to-end with no user interaction

use crate::rendering::capture::render_once;
use crate::trace_events::TraceEvent;
//use iced::{Application};
use iced_native::Program;



/// Run an [`Application`] headlessly, discarding all commands with a series of [`TraceEvents`]. Returns the [`Application`] after
/// all messages have been exhausted
pub fn execute_iced_trace<A>(
    flags: <A as iced::Application>::Flags,
    trace_events: impl IntoIterator<Item = TraceEvent<A>>,
) -> A
where
    A: iced::Application + 'static,
    //<A as Program>::Message: 'static,
{
    let (raw_application, _) = A::new(flags);
    let mut application = AppHarness(raw_application);

    //TODO: find a better window_size; have a way to configure this via the trace
    let window_size = (600, 600);
    for event in trace_events {
        match event {
            TraceEvent::Message(message) => {
                let _ = application.update(message);
            }
            TraceEvent::Duration(duration) => {
                std::thread::sleep(duration);
            }
            TraceEvent::StateCheck(state_check) => {
                assert!(state_check(&application.0));
            }
            TraceEvent::SetState(set_state) => {
                set_state(&mut application.0);
            }
            TraceEvent::CheckScreenshot(screenshot_check) => {
                let screenshot = render_once(&mut application, window_size);
                assert!(screenshot_check(screenshot));
            }
            TraceEvent::TakeScreenshot(path) => {
                let screenshot = render_once(&mut application, window_size);
                screenshot.save_image_to_png(path);
            }
        }
    }

    application.0
}

/// Run an [`Application`] headlessly, discarding all commands with a series of [`TraceEvents`]. Returns the [`Application`] after
/// all messages have been exhausted
pub fn execute_message_trace<A>(
    flags: A::Flags,
    messages: impl IntoIterator<Item = A::Message>,
) -> A
where
    //A: iced::Application + crate::runtime::Application + 'static,
    A: iced::Application + 'static,
{
    let messages = messages.into_iter().map(|msg| TraceEvent::Message(msg));
    execute_iced_trace(flags, messages)
}


///Functionally, this is identical to the Instance type that exists in [`iced::application`] but is
///not publically exposed. we use this as a "proxy" type to impl `crate::runtime::Application`.
///
///Maybe its better to expose the Instance type in iced to avoid repeated code
#[repr(transparent)]
struct AppHarness<A: iced::Application>(pub(crate) A);

impl<A> crate::runtime::Program for AppHarness<A>
where
    A: iced::Application,
{
    type Renderer = crate::renderer::Renderer;
    type Message = A::Message;

    fn update(&mut self, message: Self::Message) -> crate::runtime::Command<Self::Message> {
        self.0.update(message)
    }

    fn view(&mut self) -> crate::runtime::Element<'_, Self::Message, Self::Renderer> {
        self.0.view()
    }
}

impl<A> crate::runtime::Application for AppHarness<A>
where
    A: iced::Application,
{
    type Flags = A::Flags;

    fn new(flags: Self::Flags) -> (Self, crate::runtime::Command<A::Message>) {
        let (app, command) = A::new(flags);

        (Self(app), command)
    }

    fn title(&self) -> String {
        self.0.title()
    }

    fn mode(&self) -> crate::runtime::Mode {
        crate::runtime::Mode::Hidden
    }

    fn subscription(&self) -> crate::runtime::Subscription<Self::Message> {
        self.0.subscription()
    }

    fn background_color(&self) -> crate::runtime::Color {
        self.0.background_color()
    }

    fn scale_factor(&self) -> f64 {
        self.0.scale_factor()
    }

    fn should_exit(&self) -> bool {
        self.0.should_exit()
    }
}
