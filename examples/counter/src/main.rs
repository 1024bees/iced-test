use iced::{button, Alignment, Application, Button, Column, Command, Element, Settings, Text};

pub fn main() -> iced::Result {
    Counter::run(Settings::default())
}

#[derive(Default)]
struct Counter {
    value: i32,
    increment_button: button::State,
    decrement_button: button::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Application for Counter {
    type Message = Message;
    type Flags = ();
    type Executor = iced::executor::Default;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

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

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .align_items(Alignment::Center)
            .push(
                Button::new(&mut self.increment_button, Text::new("Increment"))
                    .on_press(Message::IncrementPressed),
            )
            .push(Text::new(self.value.to_string()).size(50))
            .push(
                Button::new(&mut self.decrement_button, Text::new("Decrement"))
                    .on_press(Message::DecrementPressed),
            )
            .into()
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use iced_test::{
        runners::{execute_iced_trace, execute_message_trace},
        Screenshot, TraceEvent,
    };

    use std::path::PathBuf;
    #[test]
    fn simple_increment_test() {
        let message_trace = vec![Message::IncrementPressed, Message::IncrementPressed];
        let app_state: Counter = execute_message_trace((), message_trace);
        assert_eq!(app_state.value, 2)
    }

    #[test]
    fn increment_ss_test() {
        let message_trace = vec![
            TraceEvent::Message(Message::IncrementPressed),
            TraceEvent::Message(Message::IncrementPressed),
            TraceEvent::TakeScreenshot(PathBuf::from(format!(
                "{}/golden/increment_ss.png",
                env!("CARGO_MANIFEST_DIR")
            ))),
            TraceEvent::CheckScreenshot(Box::new(|ss: Screenshot| {
                let golden_ss = Screenshot::from_png(PathBuf::from(format!(
                    "{}/golden/increment_ss.png",
                    env!("CARGO_MANIFEST_DIR")
                )))
                .unwrap();
                println!("Golden is {:#?}",golden_ss);
                println!("taken is {:#?}",ss);

                ss == golden_ss
            })),
        ];
        let app_state: Counter = execute_iced_trace((), message_trace);
        assert_eq!(app_state.value, 2)
    }
}
