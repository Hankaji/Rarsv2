use std::{borrow::Borrow, cell::RefCell, sync::Arc, thread::sleep, time::Duration};

use fnv::FnvBuildHasher;
use hyprland::{
    ctl::output,
    data::{Workspace, Workspaces},
    event_listener::EventListener,
    shared::{HyprData, HyprDataActive, HyprDataVec},
};
use iced::{
    advanced::text::Editor,
    alignment::Vertical,
    color,
    futures::SinkExt,
    stream::channel,
    widget::{button, row, text, Button},
    Element, Subscription,
};
use indexmap::IndexMap;

type FnvIndexMap<K, V> = IndexMap<K, V, FnvBuildHasher>;

#[derive(Default)]
pub struct Hyprland {
    workspaces: FnvIndexMap<String, WorkspaceState>,
}

#[derive(Clone, Debug)]
pub enum Message {
    WorkspaceChanged,
}

// fn hyprland_listen() -> impl iced::futures::Stream<Item = Message> {
//     use hyprland::{async_closure, data::Workspaces, event_listener::AsyncEventListener};
//     let test = Arc::new(1);
//     let t = test.clone();
//
//     channel(1, |mut output| async move {
//         let arc_output = Arc::new(Mutex::new(output));
//
//         let mut hyprland_ev_listener = AsyncEventListener::new();
//         let t = t.clone();
//         hyprland_ev_listener.add_workspace_changed_handler({
//             use std::future::IntoFuture;
//             move |_data| {
//
//                 Box::pin(async move {
//                     {
//                         println!("TEST");
//                     }
//                 })
//             }
//         });
//
//         hyprland_ev_listener.add_window_moved_handler(async_closure! {
//             |data| println!("Window moved: {data:?}")
//         });
//
//         if let Err(e) = hyprland_ev_listener.start_listener_async().await {
//             eprintln!("{e}")
//         }
//
//         // let _ = output
//         //     .send(Message::WorkspaceChange(
//         //         "1".to_string(),
//         //         WorkspaceState::Occupied,
//         //     ))
//         //     .await;
//     })
// }

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
        let wps: Vec<&str> = self
            .workspaces
            .values()
            .map(|wp_state| match wp_state {
                WorkspaceState::Inactive => "i",
                WorkspaceState::Active => "a",
                WorkspaceState::Occupied => "o",
            })
            .collect();

        let text = text(wps.join(" ")).color(color!(0xffffff));

        // row![text].align_y(Vertical::Center).into()
        text.into()
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
