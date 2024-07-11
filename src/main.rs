mod widgets;

use iced::gradient::Linear;
use iced::{executor, Gradient};
use widgets::custom_header::{self, custom_header, Message as HeaderMessage};
use iced::{
    Application, Command, 
    Color, Element, Length, 
    Theme, Settings,
    Background
};
use iced::widget::{text, column, container};
use iced::window;

pub fn main() -> iced::Result {
    CustomWindow::run(Settings {
        window: window::Settings {
            decorations: false,
            position: window::Position::Centered,
            ..Default::default()
        },
        ..Default::default()
    })
}

struct CustomWindow {}

#[derive(Debug, Clone)]
enum Message {
    Header(HeaderMessage),
    Resize,
}


impl Application for CustomWindow {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (Self { }, Command::none())
    }

    fn title(&self) -> String {
        "Header".into()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Header(header_msg) => {
                custom_header::handle_header_message(header_msg).map(Message::Header)
            }
            Message::Resize => {
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let gradient = Gradient::Linear(
            Linear::new(0.0)
            .add_stop(0.0, Color::from_rgb(0.9, 0.9, 0.9))
            .add_stop(100.0, Color::from_rgb(0.0, 0.0, 0.0))
        );

        // Create the custom header
        let custom_header = custom_header()
        .background_color(Background::Gradient(gradient))
        .border_bottom(1.0, Color::from_rgb(0.8, 0.8, 0.8));

        let body = column![
            Element::from(custom_header).map(Message::Header), // Convert to Element and map its messages
            text("Custom Header")
        ]
        .width(Length::Fill)
        .height(Length::Fill);

        container(body)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(
                container::Appearance {
                    // border: Border {
                    //     color: Color::from_rgb(0.8, 0.8, 0.8),
                    //     ..Default::default()
                    // },
                    background: Some(Background::Color(Color::from_rgb(0.9, 0.9, 0.9))),
                    ..Default::default()
                }
            )
            .into()
    }
}