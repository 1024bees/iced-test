//! Traces are serialized views of [`Application`] executions.
//!
//!
//! [`Application`]: iced::Application

use std::time::Duration;
// currently of the opinion this type is that it is a needless abstraction
//#[derive(Debug)]
///// Message Trace object; this represents a serialized
//pub enum Trace<Message> {
//    ///A list of messages that is to be immediately sent to [`iced::Application`]'s update
//    MessageTrace(Vec<Message>),
//    ///A list of messages that are sent to [`iced::Application`] after waiting [`Duration`]
//    TimedMessageTrace(Vec<(Duration, Message)>),
//}

/// A message that is sent to the application after [`Duration`] has passed
pub type TimedMessage<Message> = (Duration, Message);
