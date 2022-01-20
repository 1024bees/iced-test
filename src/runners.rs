//! Interfaces for running applicationss end-to-end with no user interaction

use crate::trace_events::TraceEvent;
//use iced::{Application};



/// Run an [`Application`] headlessly, discarding all commands with a series of [`TraceEvents`]. Returns the [`Application`] after
/// all messages have been exhausted
pub fn run_trace_events<A>(flags: A::Flags, trace_events: impl Iterator<Item = TraceEvent<A>>) -> A
where
    A: iced::Application,
{
    let (mut application, _) = A::new(flags);

    for event in trace_events {
        match event {
            TraceEvent::Message(message) => {
                let _ = application.update(message);
            }
            TraceEvent::Duration(duration) => {
                std::thread::sleep(duration);
            }
            TraceEvent::StateCheck(state_check) => {
                assert!(state_check(&application));
            }
            TraceEvent::SetState(set_state) => {
                set_state(&mut application);
            }
        }
    }

    application
}

/// Run an [`Application`] headlessly, discarding all commands with a series of [`TraceEvents`]. Returns the [`Application`] after
/// all messages have been exhausted
pub fn run_messages<A>(flags: A::Flags, messages: impl Iterator<Item = A::Message>) -> A
where
    A: iced::Application,
{
    let messages = messages.map(|msg| TraceEvent::Message(msg));
    run_trace_events(flags, messages)
}
