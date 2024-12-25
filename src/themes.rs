#![allow(unused)]

use iced::{
    color,
    widget::{
        container::{self, StyleFn},
        text,
    },
    Color,
};
use iced_layershell::{Appearance, DefaultStyle};
use palette::Palette;

pub mod palette {
    use iced::{color, Color};
    use serde::{Deserialize, Deserializer};

    use crate::config::CONFIG;

    #[derive(Deserialize, Debug, Clone, PartialEq)]
    pub struct Palette {
        pub primary: ColorPair,
        // secondary: ColorPair,
        // muted: ColorPair,
        // active: ColorPair,
    }

    impl Default for Palette {
        /// Default of palette will take color defined in the config file
        fn default() -> Self {
            CONFIG.theme.colors.clone()
        }
    }

    #[derive(Deserialize, Debug, Clone, PartialEq)]
    pub struct ColorPair {
        #[serde(deserialize_with = "parse_color")]
        pub bg: Color,
        #[serde(deserialize_with = "parse_color")]
        pub fg: Color,
    }

    fn parse_color<'de, D>(deserializer: D) -> Result<Color, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut color_code: String = Deserialize::deserialize(deserializer)?;
        if let Some(color_code) = color_code.strip_prefix("#") {
            u32::from_str_radix(color_code, 16)
                .map(|hex_code| color!(hex_code))
                .map_err(serde::de::Error::custom)
        } else {
            Err(serde::de::Error::custom("Invalid color format"))
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Theme {
    palette: Palette,
}

impl Theme {
    pub fn palette(&self) -> &Palette {
        &self.palette
    }
}

impl DefaultStyle for Theme {
    fn default_style(&self) -> Appearance {
        Appearance {
            background_color: self.palette.primary.bg,
            text_color: self.palette.primary.fg,
        }
    }
}

#[derive(Default)]
pub enum ContainerVariant<'a> {
    #[default]
    Default,
    Transparent,
    Custom(StyleFn<'a, Theme>),
}

impl<'a> From<StyleFn<'a, Theme>> for ContainerVariant<'a> {
    fn from(value: StyleFn<'a, Theme>) -> Self {
        Self::Custom(value)
    }
}

impl container::Catalog for Theme {
    type Class<'a> = ContainerVariant<'a>;

    fn default<'a>() -> Self::Class<'a> {
        ContainerVariant::default()
    }

    fn style(&self, variant: &Self::Class<'_>) -> container::Style {
        match variant {
            ContainerVariant::Default => container::Style::default(),
            ContainerVariant::Transparent => container::Style {
                background: Some(Color::TRANSPARENT.into()),
                ..Default::default()
            },
            ContainerVariant::Custom(style_fn) => style_fn(self),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum TextVariant {
    #[default]
    Default,
}

impl text::Catalog for Theme {
    type Class<'a> = TextVariant;

    fn default<'a>() -> Self::Class<'a> {
        TextVariant::default()
    }

    fn style(&self, item: &Self::Class<'_>) -> text::Style {
        match item {
            TextVariant::Default => text::Style::default(),
        }
    }
}
