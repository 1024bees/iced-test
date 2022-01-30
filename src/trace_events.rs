//! Traces are serialized views of [`Application`] executions.
//!
//!
//! [`Application`]: iced::Application

pub use iced::Screenshot;
use std::path::PathBuf;
/// Events that can compose serialized execution
pub enum TraceEvent<A: iced::Application> {
    /// Send a message directly to the application's uppdate fn
    Message(A::Message),
    /// Delay for [`Duration`]
    Duration(std::time::Duration),
    ///Check [`Application`] state -- allow the end user to embed assertions in the trace
    StateCheck(Box<dyn Fn(&A) -> bool>),
    ///Configure [`Application`] state directly
    SetState(Box<dyn Fn(&mut A)>),
    ///Render a [`Screenshot`] of the current app, passes the [`Screenshot`] into a user provided
    ///function
    CheckScreenshot(Box<dyn Fn(Screenshot) -> bool>),
    ///Take a screenshot, save it to the provided [`PathBuf`]
    TakeScreenshot(PathBuf),
}
