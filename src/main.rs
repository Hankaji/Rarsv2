use bar::Bar;
use file_guard::FileGuard;
use iced_layershell::reexport::{Anchor, Layer};
use iced_layershell::settings::{LayerShellSettings, Settings, StartMode};
use iced_layershell::Application;
use notification::Service;

mod bar;
mod config;
mod themes;
mod widgets;

fn main() -> Result<(), iced_layershell::Error> {
    let args: Vec<String> = std::env::args().collect();

    check_instance();

    // Initialize
    let cfg = &config::CONFIG;
    Service::new(); // TODO: Remove this when NotificationService is used in a notification widget

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
            layer: Layer::Top,
            ..Default::default()
        },
        ..Default::default()
    })
}

fn check_instance() {
    use std::fs::File;
    use std::fs::OpenOptions;
    use std::io::prelude::*;
    use std::sync::LazyLock;

    static INSTANCE: LazyLock<FileGuard<Box<File>>> = LazyLock::new(|| {
        const PID_DIR: &str = "/tmp/rarsv2.pid";
        let pid = std::process::id();

        let file = Box::new(
            OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(false)
                .open(PID_DIR)
                .expect("Could not open the PID file"),
        );

        // Attempt to obtain the lock file 'rarsv2.pid'
        // Fail if another instance is already running
        let flock = file_guard::try_lock(file, file_guard::Lock::Exclusive, 0, 1);
        match flock {
            Ok(mut flock) => {
                write!(flock, "{pid}").unwrap();
                flock
            }
            Err(_) => panic!("Another instance with proccess id {pid} is already running"),
        }
    });

    let _init_only = &*INSTANCE;
}
