use crate::themes::Theme;

pub type Renderer = iced::Renderer;
pub type Element<'a, Message, Theme = crate::themes::Theme> =
    iced::Element<'a, Message, Theme, Renderer>;
pub type Container<'a, Message> = iced::widget::Container<'a, Message, Theme, Renderer>;
pub type Text<'a> = iced::widget::Text<'a, Theme, Renderer>;
