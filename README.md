# Iced - Testing utilities

This crate contains testing utilities for the GUI library [Iced](https://github.com/hecrj/iced). This crate is a WIP and is looking for more direction from other Iced users and developers; constructive criticism is welcomed.


## Overview:

This crate tests applications by executing traces. A trace is a serialized record of messages and assertions that are passed to an application. In code, a trace can be described as follows:


```rust

//there isnt a formal Trace type yet
pub type Trace<A: iced::Application> = impl IntoIterator<Item=TraceEvent<A>>;

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
    ///function to be inspected
    CheckScreenshot(Box<dyn Fn(Screenshot) -> bool>),
    ///Take a screenshot, save it to the provided [`PathBuf`]
    TakeScreenshot(PathBuf),
}
```


A trace can also be a serialized view of solely Application messages





## Usage:

Include `iced_test` as a dev-dependency in your `Cargo.toml`:
```toml
[dependencies]
iced = {git = "https://github.com/1024bees/iced", branch="headless" }


[dev-dependencies]
iced-test = {git="https://github.com/1024bees/iced_test"}
```


Write your Iced application logic. e.g. 

```rust
use iced::{Application,button};

struct Counter {
    // The counter value
    value: i32,
    ...
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Application for Counter {
    // simple counter app
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
        Command::none()
    }
....
}
```

Now in a test module, you can use iced_test to run the application via `cargo test`!
```rust
#[cfg(test)]
mod test {
    use super::*;
    use iced_test::{
        runners::{execute_iced_trace, execute_message_trace},
        TraceEvent,
    };
    use std::path::PathBuf;
    //test that passes two messages to the application, inspects state afterwards
    #[test]
    fn simple_increment_test() {
        let message_trace = vec![Message::IncrementPressed, Message::IncrementPressed];
        let app_state: Counter = execute_message_trace((), message_trace);
        assert_eq!(app_state.value, 2)
    }

    //simple test that passes two messages to the application, and then takes a screenshot of the application, saving it to 
    //golden/screenshot.png
    #[test]
    fn increment_ss_test() {
        let message_trace = vec![
            TraceEvent::Message(Message::IncrementPressed),
            TraceEvent::Message(Message::IncrementPressed),
            TraceEvent::TakeScreenshot(PathBuf::from(format!(
                "{}/golden/screenshot.png",
                env!("CARGO_MANIFEST_DIR")
            ))),
        ];
        let app_state: Counter = execute_iced_trace((), message_trace);
        assert_eq!(app_state.value, 2)
    }
}
```



