use iced::Padding;
use iced_layershell::reexport::Anchor;
use serde::{Deserialize, Deserializer};

#[derive(Deserialize, Debug, PartialEq)]
pub struct BarConf {
    #[serde(default, deserialize_with = "parse_padding")]
    pub padding: Padding,
    #[serde(default = "default_anchor", deserialize_with = "parse_anchor")]
    pub anchor: Anchor,
    pub workspaces: Vec<String>,
    #[serde(default = "default_icon_size")]
    pub icon_size: f32,
    #[serde(default = "default_gap")]
    pub gap: f32,
}

fn parse_padding<'de, D>(deserializer: D) -> Result<Padding, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum PaddingParser {
        X1(f32),
        X2([f32; 2]),
        X4(f32, f32, f32, f32),
    }

    Ok(match PaddingParser::deserialize(deserializer)? {
        PaddingParser::X1(all) => Padding::new(all),
        PaddingParser::X2(vh) => Padding::from(vh),
        PaddingParser::X4(top, right, bottom, left) => Padding {
            top,
            right,
            bottom,
            left,
        },
    })
}

fn default_anchor() -> Anchor {
    Anchor::Top
}

fn parse_anchor<'de, D>(deserializer: D) -> Result<Anchor, D::Error>
where
    D: Deserializer<'de>,
{
    let mut anchor: String = Deserialize::deserialize(deserializer)?;

    // Capitalize the String
    anchor = anchor.to_lowercase();
    if let Some(r) = anchor.get_mut(0..1) {
        r.make_ascii_uppercase();
    }

    Ok(Anchor::from_name(&anchor).expect("Invalid anchor point {anchor}"))
}

fn default_icon_size() -> f32 {
    6.0
}

fn default_gap() -> f32 {
    10.0
}
