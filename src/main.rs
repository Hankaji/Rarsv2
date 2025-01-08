use std::fs;
use std::process::exit;

use bar::Bar;
use clap::Parser;
use cli::{Cli, Commands};
use file_guard::FileGuard;
use iced_layershell::reexport::{Anchor, Layer};
use iced_layershell::settings::{LayerShellSettings, Settings, StartMode};
use iced_layershell::Application;
use nix::sys::signal::Signal;
use nix::unistd::Pid;
use notification::Service;

mod bar;
mod cli;
mod config;
mod themes;
mod widgets;

fn main() -> Result<(), iced_layershell::Error> {
    let args: Vec<String> = std::env::args().collect();
    let _ = Cli::parse();

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

    pub(crate) static INSTANCE: LazyLock<Option<FileGuard<Box<File>>>> = LazyLock::new(|| {
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
        let cli = Cli::parse();
        match flock {
            Ok(mut flock) => match cli.command {
                None => {
                    write!(flock, "{pid}").expect("Could not write to pid file");
                    Some(flock)
                }
                Some(cmd) => match cmd {
                    Commands::Quit => {
                        panic!("No available instance to terminate")
                    }
                    _ => None,
                },
            },
            Err(_) => {
                let running_pid = fs::read_to_string(PID_DIR).expect("Could not read pid file");
                let Ok(running_pid): Result<u32, _> = running_pid.parse() else {
                    panic!("PID {running_pid} can't be parsed to u8")
                };

                match cli.command {
                    None => {
                        panic!("Another instance with proccess id {running_pid} is already running")
                    }
                    Some(cmd) => match cmd {
                        Commands::Quit => {
                            match nix::sys::signal::kill(
                                Pid::from_raw(running_pid as i32),
                                Signal::SIGTERM,
                            ) {
                                Ok(_) => {
                                    println!("Proccess {} terminated successfully", running_pid)
                                }
                                Err(e) => eprintln!("Failed to terminate proccess {}", e),
                            }
                            exit(0)
                        }
                        _ => None,
                    },
                }
            }
        }
    });

    let _init_only = &*INSTANCE;
}
