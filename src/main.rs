use iced::widget::{button, column, container, text, text_input, Row};
use iced::{Alignment, Element, Length, Sandbox, Settings};

fn main() -> iced::Result {
    Calculator::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    InputReceived(String),
    ButtonPressed(char),
}

struct Calculator {
    input: String,
    result: String,
    last_operation: Option<Operation>,
    last_number: Option<f64>,
    clear_on_next: bool,
}

#[derive(Clone, Copy)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Sandbox for Calculator {
    type Message = Message;

    fn new() -> Self {
        Self {
            input: String::new(),
            result: String::new(),
            last_operation: None,
            last_number: None,
            clear_on_next: false,
        }
    }

    fn title(&self) -> String {
        String::from("Calculator")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::InputReceived(value) => {
                self.input = value;
            }
            Message::ButtonPressed(key) => match key {
                '0'..='9' => self.handle_number(key),
                '+' => self.handle_operation(Operation::Add),
                '-' => self.handle_operation(Operation::Subtract),
                '*' => self.handle_operation(Operation::Multiply),
                '/' => self.handle_operation(Operation::Divide),
                '=' => self.handle_equals(),
                'C' => self.handle_clear(),
                _ => {}
            },
        }
    }

    fn view(&self) -> Element<Message> {
        let display = text_input("0", &self.input)
            .padding(10)
            .size(30)
            .on_input(Message::InputReceived);

        let buttons = [
            ['7', '8', '9', '/'],
            ['4', '5', '6', '*'],
            ['1', '2', '3', '-'],
            ['0', 'C', '=', '+'],
        ];

        let button_rows: Vec<Element<_>> = buttons
            .iter()
            .map(|button_row| {
                Row::with_children(
                    button_row
                        .iter()
                        .map(|&key| {
                            button(text(key).size(20))
                                .padding(20)
                                .on_press(Message::ButtonPressed(key))
                                .into()
                        })
                        .collect(),
                )
                .spacing(5)
                .padding(5)
                .align_items(Alignment::Center)
                .into()
            })
            .collect();

        let content = column![display]
            .push(column(button_rows))
            .spacing(20)
            .padding(20)
            .align_items(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

impl Calculator {
    fn handle_number(&mut self, num: char) {
        if self.clear_on_next {
            self.input.clear();
            self.clear_on_next = false;
        }
        self.input.push(num);
    }

    fn handle_operation(&mut self, op: Operation) {
        if let Ok(number) = self.input.parse::<f64>() {
            if let Some(last_number) = self.last_number {
                if let Some(last_op) = self.last_operation {
                    let result = match last_op {
                        Operation::Add => last_number + number,
                        Operation::Subtract => last_number - number,
                        Operation::Multiply => last_number * number,
                        Operation::Divide => last_number / number,
                    };
                    self.input = format!("{}", result);
                    self.result = self.input.clone();
                }
            }
            self.last_number = Some(self.input.parse().unwrap());
            self.last_operation = Some(op);
            self.clear_on_next = true;
        }
    }

    fn handle_equals(&mut self) {
        if let (Some(last_number), Some(last_op)) = (self.last_number, self.last_operation) {
            if let Ok(number) = self.input.parse::<f64>() {
                let result = match last_op {
                    Operation::Add => last_number + number,
                    Operation::Subtract => last_number - number,
                    Operation::Multiply => last_number * number,
                    Operation::Divide => last_number / number,
                };
                self.input = format!("{}", result);
                self.result = self.input.clone();
                self.last_number = None;
                self.last_operation = None;
                self.clear_on_next = true;
            }
        }
    }

    fn handle_clear(&mut self) {
        self.input.clear();
        self.result.clear();
        self.last_number = None;
        self.last_operation = None;
        self.clear_on_next = false;
    }
}