use iced::widget::{button, column, row, text, text_input};
use iced::{alignment, Element, Sandbox, Settings, Theme, Length};

struct Calculator {
    value: String,
    result: String,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    Evaluate,
    Clear,
}

impl Sandbox for Calculator {
    type Message = Message;

    fn new() -> Self {
        Self {
            value: String::new(),
            result: String::new(),
        }
    }

    fn title(&self) -> String {
        String::from("Iced Calculator")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(value) => self.value = value,
            Message::Evaluate => {
                self.result = match eval(&self.value) {
                    Ok(result) => result.to_string(),
                    Err(_) => "Error".to_string(),
                };
            }
            Message::Clear => {
                self.value.clear();
                self.result.clear();
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let input = text_input("Enter expression", &self.value)
            .on_input(Message::InputChanged)
            .padding(10)
            .width(Length::Fill);

        let buttons = row![
            button("=").on_press(Message::Evaluate),
            button("Clear").on_press(Message::Clear)
        ]
        .spacing(10);

        let content = column![
            input,
            buttons,
            text(&self.result).size(30).horizontal_alignment(alignment::Horizontal::Center),
        ]
        .padding(20)
        .spacing(10)
        .align_items(iced::alignment::Alignment::Center);

        content.into()
    }
}

fn eval(expr: &str) -> Result<f64, Box<dyn std::error::Error>> {
    Ok(42.0) 
}

fn main() -> iced::Result {
    Calculator::run(Settings::default())
}
