use iced::executor;
use iced::widget::{column, container};
use iced::{Application, Command, Element, Length, Settings, Theme};

pub fn launch() -> iced::Result {
    Grisked::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}

struct Grisked {}

#[derive(Debug, Clone, Copy)]
enum Message {}

impl Application for Grisked {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = Theme;

    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Grisked {}, Command::none())
    }

    fn title(&self) -> String {
        "Grisked".to_string()
    }

    fn update(&mut self, _message: Message) -> Command<Message> {
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let content = column!["Hello, world!"];

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .into()
    }
}
