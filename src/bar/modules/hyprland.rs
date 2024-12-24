use std::sync::Arc;

use fnv::FnvBuildHasher;
use hyprland::{
    data::{Workspace, Workspaces},
    shared::{HyprData, HyprDataActive, HyprDataVec},
};
use iced::{
    alignment::Vertical,
    color,
    futures::SinkExt,
    stream::channel,
    widget::{container, row},
    Border, Color, Element, Subscription, Theme,
};
use indexmap::IndexMap;

use crate::config::CONFIG;

type FnvIndexMap<K, V> = IndexMap<K, V, FnvBuildHasher>;

#[derive(Default)]
pub struct Hyprland {
    workspaces: FnvIndexMap<String, WorkspaceState>,
}

#[derive(Clone, Debug)]
pub enum Message {
    WorkspaceChanged,
}

// fn hyprland_listen() -> impl iced::futures::Stream<Item = Message> {}

impl Hyprland {
    pub fn new(workspaces: &Vec<String>) -> Self {
        let mut hm = FnvIndexMap::with_capacity_and_hasher(10, FnvBuildHasher::default());
        for wp_name in workspaces {
            hm.insert(wp_name.clone(), WorkspaceState::Inactive);
        }

        let mut s = Self { workspaces: hm };
        s.check_workspaces();

        s
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::run(|| {
            channel(1, |output| async move {
                use hyprland::event_listener::AsyncEventListener;

                let arc_output = Arc::new(iced::futures::lock::Mutex::new(output));

                let mut hyprland_ev_listener = AsyncEventListener::new();

                let output = arc_output.clone();
                hyprland_ev_listener.add_workspace_changed_handler({
                    move |_data| {
                        let output = output.clone();
                        Box::pin(async move {
                            let mut output = output.lock().await;

                            let _ = output.send(Message::WorkspaceChanged).await;
                        })
                    }
                });

                let output = arc_output.clone();
                hyprland_ev_listener.add_window_moved_handler({
                    move |_| {
                        let output = output.clone();
                        Box::pin(async move {
                            let mut output = output.lock().await;

                            let _ = output.send(Message::WorkspaceChanged).await;
                        })
                    }
                });

                if let Err(e) = hyprland_ev_listener.start_listener_async().await {
                    eprintln!("{e}")
                }
            })
        })
    }

    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::WorkspaceChanged => self.check_workspaces(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        let wp_icons: Vec<Element<Message>> = self
            .workspaces
            .values()
            .map(|wp_state| {
                let size = CONFIG.bar.icon_size;

                let width: f32;
                let height: f32;
                let color: Color;
                match wp_state {
                    WorkspaceState::Active => {
                        width = size * 4.8;
                        height = size * 2.8;
                        color = color!(0xbb9af7);
                    }
                    WorkspaceState::Occupied => {
                        width = size * 1.5;
                        height = size * 1.5;
                        color = color!(0xa9b1d6);
                    }
                    _ => {
                        width = size;
                        height = size;
                        color = color!(0x565f89);
                    }
                };

                // AnimationBuilder::new((width, height), |(w, h)| {
                //     container("")
                //         .style(move |theme: &Theme| container::Style {
                //             border: Border {
                //                 width: 0.0,
                //                 radius: 50.0.into(),
                //                 ..Default::default()
                //             },
                //             background: Some(theme.extended_palette().secondary.weak.color.into()),
                //             ..Default::default()
                //         })
                //         .center_x(w)
                //         .center_y(h)
                //         .into()
                // })
                // .animates_layout(true)
                // .into()

                container("")
                    .style(move |_theme: &Theme| container::Style {
                        border: Border {
                            width: 0.0,
                            radius: 50.0.into(),
                            ..Default::default()
                        },
                        background: Some(color.into()),
                        ..Default::default()
                    })
                    .center_x(width)
                    .center_y(height)
                    .into()
            })
            .collect();

        row(wp_icons)
            .align_y(Vertical::Center)
            .spacing(CONFIG.bar.gap)
            .into()
    }

    // ------------------------- Hyprland methods -------------------------
    /// Check the current state of workspaces and renew the workspaces vector inside this struct
    /// Implemetation seem pretty inefficient, however since the size is small enough (hopefully
    /// ~10) the performance can be pretty much neglectable
    ///
    /// Will re-implement this algo if performance is a problem
    fn check_workspaces(&mut self) {
        let mut occupied_workspaces = Workspaces::get().expect("Cant get workspaces").to_vec();
        let active_workspace = Workspace::get_active().expect("Cant get active workspace");

        for v in self.workspaces.values_mut() {
            *v = WorkspaceState::Inactive
        }

        occupied_workspaces.retain(|wp| wp.windows > 0);
        for o_wp in occupied_workspaces {
            self.workspaces
                .entry(o_wp.name)
                .and_modify(|state| *state = WorkspaceState::Occupied);
        }

        self.workspaces
            .entry(active_workspace.name)
            .and_modify(|old_state| *old_state = WorkspaceState::Active);
    }
}

#[derive(Clone, Debug)]
pub enum WorkspaceState {
    Inactive,
    Active,
    Occupied,
}
