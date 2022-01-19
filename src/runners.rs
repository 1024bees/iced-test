//! Interfaces for running applicationss end-to-end with no user interaction


use iced::Application;

/// Run an [`Application`] headlessly, discarding all commands. Returns the [`Application`] after
/// all messages have been exhausted 
pub fn run<A>(flags: A::Flags, messages: impl Iterator<Item = A::Message>) -> A
where
    A: iced::Application,
{
    let (mut application, _) = A::new(flags);

    for message in messages {
        let _ = application.update(message);
    }

    application
}



