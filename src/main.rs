use bar::Bar;
use iced_layershell::reexport::Anchor;
use iced_layershell::settings::{LayerShellSettings, Settings, StartMode};
use iced_layershell::Application;

mod bar;

fn main() -> Result<(), iced_layershell::Error> {
    let args: Vec<String> = std::env::args().collect();

    let mut binded_output_name = None;
    if args.len() >= 2 {
        binded_output_name = Some(args[1].to_string())
    }

    let start_mode = match binded_output_name {
        Some(output) => StartMode::TargetScreen(output),
        None => StartMode::Active,
    };

    // let settings: Settings<()> = Settings {
    //     layer_settings: LayerShellSettings {
    //         size: Some((0, 400)),
    //         exclusive_zone: 400,
    //         anchor: Anchor::Top | Anchor::Left | Anchor::Right,
    //         start_mode,
    //         ..Default::default()
    //     },
    //     ..Default::default()
    // };

    Bar::run(Settings {
        layer_settings: LayerShellSettings {
            size: Some((0, 20)),
            exclusive_zone: 20,
            anchor: Anchor::Top | Anchor::Left | Anchor::Right,
            start_mode,
            ..Default::default()
        },
        ..Default::default()
    })
}
