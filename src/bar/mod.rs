use iced::alignment::{Horizontal, Vertical};
use iced::widget::{container, row, Container};
use iced::{Color, Element, Length, Padding, Task as Command, Theme};
use iced_layershell::to_layer_message;
use iced_layershell::Application;
use modules::date_time::{self, DateTime};

mod modules;

pub struct Bar {
    center: DateTime,
}

// TODO: Implement later for directional bar
// #[derive(Debug, Clone, Copy)]
// enum WindowDirection {
//     Top,
//     Left,
//     Right,
//     Bottom,
// }

// Because new iced delete the custom command, so now we make a macro crate to generate
// the Command
#[to_layer_message]
#[derive(Debug, Clone)]
pub enum Message {
    DateTime(date_time::Message),
}

impl Application for Bar {
    type Message = Message;
    type Flags = ();
    type Theme = Theme;
    type Executor = iced::executor::Default;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                center: DateTime::new(),
            },
            Command::none(),
        )
    }

    fn namespace(&self) -> String {
        String::from("Rarsv2")
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        // event::listen().map(Message::IcedEvent)
        self.center.subscription().map(Message::DateTime)
    }

    fn update(&mut self, msg: Message) -> Command<Message> {
        match msg {
            Message::DateTime(msg) => {
                self.center.update(msg);
                Command::none()
            }
            _ => unimplemented!(),
        }
    }

    fn view(&self) -> Element<Message> {
        let date_time_module = self.center.view().map(Message::DateTime);

        // ------------------------- Container -------------------------
        let left: Container<Message> = container("Left")
            .width(Length::Fill)
            .height(Length::Shrink)
            .align_left(Length::Fill)
            .align_y(Vertical::Center);

        let center: Container<Message> = container(row![date_time_module])
            .width(Length::Fill)
            .height(Length::Shrink)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center);

        let right: Container<Message> = container("Right")
            .width(Length::Fill)
            .height(Length::Shrink)
            .align_right(Length::Fill)
            .align_y(Vertical::Center);

        row![left, center, right]
            .width(Length::Fill)
            .padding(Padding::from([0, 10]))
            .into()
    }

    fn style(&self, theme: &Self::Theme) -> iced_layershell::Appearance {
        use iced_layershell::Appearance;
        Appearance {
            background_color: Color::TRANSPARENT,
            text_color: theme.palette().text,
        }
    }
}
