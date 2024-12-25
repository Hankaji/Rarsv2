use iced::alignment::{Horizontal, Vertical};
use iced::widget::row;
use iced::{Length, Padding, Task as Command};
use iced_layershell::to_layer_message;
use iced_layershell::Application;
use modules::clock::{self, Clock};
use modules::hyprland::{self, Hyprland};

use crate::config;
use crate::themes::Theme;
use crate::widgets::{Container, Element};

pub mod modules;

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
        let cfg = &config::CONFIG;

        (
            Self {
                start: Hyprland::new(&cfg.bar.workspaces),
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
        let start: Container<Message> = Container::new(row![hyprland_module])
            .width(Length::Fill)
            .height(Length::Shrink)
            .align_left(Length::Fill)
            .align_y(Vertical::Center);

        let center: Container<Message> = Container::new(row![clock_module])
            .width(Length::Fill)
            .height(Length::Shrink)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center);

        let end: Container<Message> = Container::new("Right")
            .width(Length::Fill)
            .height(Length::Shrink)
            .align_right(Length::Fill)
            .align_y(Vertical::Center);

        row![start, center, end]
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(Padding::from([0, 10]))
            .align_y(Vertical::Center)
            .into()
    }
}

#[derive(Clone, Debug)]
enum BarModules {
    Clock,
    Hyprland,
}
