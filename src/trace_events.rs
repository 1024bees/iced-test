//! Traces are serialized views of [`Application`] executions.
//!
//!
//! [`Application`]: iced::Application

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
}
