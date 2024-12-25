use serde::Deserialize;

use crate::themes::palette::Palette;

#[derive(Deserialize, Debug, PartialEq)]
pub struct ThemeSettings {
    #[serde(default = "default_colors")]
    pub colors: Palette,
}

/// Default theme color is [`Tokyo-Night`](https://github.com/tokyo-night/tokyo-night-vscode-theme) inspired
fn default_colors() -> Palette {
    unimplemented!()
}
