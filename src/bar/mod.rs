use std::default;

use iced::alignment::{Horizontal, Vertical};
use iced::widget::{container, row, Container};
use iced::{Color, Element, Length, Padding, Subscription, Task as Command, Theme};
use iced_layershell::to_layer_message;
use iced_layershell::Application;
use modules::clock::{self, Clock};
use modules::hyprland::{self, Hyprland};

mod modules;

#[derive(Default)]
pub struct Bar {
    start: Hyprland,
    center: Clock,
    // test: [BarModules],
}

// TODO: Implement later for directional bar
// #[derive(Debug, Clone, Copy)]
// enum WindowDirection {
//     Top,
//     Left,
//     Right,
//     Bottom,
// }

#[to_layer_message]
#[derive(Debug, Clone)]
pub enum Message {
    Clock(clock::Message),
    Hyprland(hyprland::Message),
}

impl Application for Bar {
    type Message = Message;
    type Flags = ();
    type Theme = Theme;
    type Executor = iced::executor::Default;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                start: Hyprland::new(),
                ..Default::default()
            },
            Command::none(),
        )
    }

    fn namespace(&self) -> String {
        String::from("Rarsv2")
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        // event::listen().map(Message::IcedEvent)
        // self.center.subscription().map(Message::Clock)
        self.start.subscription().map(Message::Hyprland)
        // Subscription::batch(subscriptions)
    }

    fn update(&mut self, msg: Message) -> Command<Message> {
        match msg {
            Message::Clock(msg) => {
                self.center.update(msg);
                Command::none()
            }
            Message::Hyprland(msg) => {
                self.start.update(msg);
                Command::none()
            }
            _ => unimplemented!(),
        }
    }

    fn view(&self) -> Element<Message> {
        let hyprland_module = self.start.view().map(Message::Hyprland);
        let clock_module = self.center.view().map(Message::Clock);

        // ------------------------- Container -------------------------
        let left: Container<Message> = container(row![hyprland_module])
            .width(Length::Fill)
            .height(Length::Shrink)
            .align_left(Length::Fill)
            .align_y(Vertical::Center);

        let center: Container<Message> = container(row![clock_module])
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
            .height(Length::Fill)
            .padding(Padding::from([0, 10]))
            .align_y(Vertical::Center)
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

#[derive(Clone, Debug)]
enum BarModules {
    Clock,
    Hyprland,
}
