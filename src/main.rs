use iced::widget::{button, column, row, text, text_input};
use iced::{Alignment, Element, Sandbox, Settings};

#[derive(Default)]
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
        Self::default()
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
            .padding(10);

        let buttons = row![
            button("=").on_press(Message::Evaluate),
            button("Clear").on_press(Message::Clear)
        ]
        .spacing(10);

        let content = column![
            input,
            buttons,
            text(&self.result).size(30),
        ]
        .padding(20)
        .spacing(10)
        .align_items(Alignment::Center);

        content.into()
    }
}

fn eval(expr: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let result = expr.replace(" ", "")
        .split(|c| c == '+' || c == '-' || c == '*' || c == '/')
        .map(|n| n.parse::<f64>())
        .collect::<Result<Vec<f64>, _>>()?;

    let mut sum = result[0];
    let mut op = '+';
    for (i, &num) in result.iter().enumerate().skip(1) {
        match expr.chars().nth(expr.find(|c| c == '+' || c == '-' || c == '*' || c == '/').unwrap() + i - 1).unwrap() {
            '+' => op = '+',
            '-' => op = '-',
            '*' => op = '*',
            '/' => op = '/',
            _ => {}
        }
        match op {
            '+' => sum += num,
            '-' => sum -= num,
            '*' => sum *= num,
            '/' => sum /= num,
            _ => {}
        }
    }
    Ok(sum)
}

fn main() -> iced::Result {
    Calculator::run(Settings::default())
}