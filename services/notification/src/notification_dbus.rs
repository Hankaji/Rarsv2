#![allow(clippy::too_many_arguments)]
use zbus::{interface, zvariant::Value};

use crate::notification::Notification;
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU32, Ordering},
        Mutex,
    },
};

#[derive(Default)]
pub struct NotificationDaemon {
    count: AtomicU32,
    notifications: Mutex<HashMap<u32, Notification>>,
}

#[interface(name = "org.freedesktop.Notifications")]
impl NotificationDaemon {
    /// org.freedesktop.Notifications.Notify D-Bus method
    fn notify(
        &self,
        app_name: String,
        replaces_id: u32,
        app_icon: String,
        summary: String,
        body: String,
        actions: Vec<String>,
        _hints: HashMap<String, Value<'_>>,
        expire_timeout: i32,
    ) -> zbus::fdo::Result<u32> {
        let notification_id = if replaces_id == 0 {
            self.count.fetch_add(1, Ordering::SeqCst)
        } else {
            replaces_id
        };

        let notification = Notification {
            app_name,
            replaces_id,
            app_icon,
            summary,
            body,
            actions,
            expire_timeout,
            notification_id,
        };

        let mut notifs = self.notifications.lock().unwrap();
        println!("{notification:?}");
        notifs.insert(notification_id, notification);

        Ok(notification_id)
    }

    fn close_notification(&self, id: u32) {
        let mut notifications = self.notifications.lock().unwrap();
        notifications.remove(&id);
    }

    fn get_server_information(&mut self) -> zbus::fdo::Result<(String, String, String, String)> {
        let name = String::from("Notification Daemon Test");
        let vendor = String::from(env!("CARGO_PKG_NAME"));
        let version = String::from(env!("CARGO_PKG_VERSION"));
        let specification_version = String::from("1.2");

        Ok((name, vendor, version, specification_version))
    }
}
