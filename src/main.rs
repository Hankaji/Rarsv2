use bar::Bar;
use iced_layershell::reexport::Anchor;
use iced_layershell::settings::{LayerShellSettings, Settings, StartMode};
use iced_layershell::Application;

mod bar;
mod config;

fn main() -> Result<(), iced_layershell::Error> {
    let args: Vec<String> = std::env::args().collect();

    // Init config
    let cfg = &config::CONFIG;

    // Start layershell
    let mut binded_output_name = None;
    if args.len() >= 2 {
        binded_output_name = Some(args[1].to_string())
    }

    let start_mode = match binded_output_name {
        Some(output) => StartMode::TargetScreen(output),
        None => StartMode::Active,
    };

    let anchor = match cfg.bar.anchor {
        Anchor::Top => Anchor::Top | Anchor::Left | Anchor::Right,
        Anchor::Bottom => Anchor::Bottom | Anchor::Left | Anchor::Right,
        Anchor::Left => Anchor::Left | Anchor::Top | Anchor::Bottom,
        Anchor::Right => Anchor::Right | Anchor::Top | Anchor::Bottom,
        _ => unreachable!(),
    };

    Bar::run(Settings {
        layer_settings: LayerShellSettings {
            size: Some((0, 20)),
            exclusive_zone: 20,
            anchor,
            start_mode,
            ..Default::default()
        },
        ..Default::default()
    })
}
