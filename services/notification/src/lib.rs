use std::{error::Error, future::pending};

use notification_dbus::NotificationDaemon;
use notificationd::Daemon;
use zbus::connection;

mod notification;
mod notification_dbus;
mod notificationd;

pub async fn register_bus() -> Result<(), Box<dyn Error>> {
    let notification_daemon = NotificationDaemon::default();
    let _conn = connection::Builder::session()?
        .name("org.freedesktop.Notifications")?
        .serve_at("/org/freedesktop/Notifications", notification_daemon)?
        .build()
        .await?;

    pending::<()>().await;

    Ok(())
}

pub struct Service;

impl Service {
    pub fn new() -> Self {
        Daemon::run();

        Service
    }
}

impl Default for Service {
    fn default() -> Self {
        Daemon::run();

        Service
    }
}
