use std::{error::Error, future::pending, sync::OnceLock};

use tokio::runtime::Runtime;
use zbus::connection;

use crate::notification_dbus::NotificationDaemon;

// lazy_static! {
//     pub(crate) static ref DAEMON: Daemon = Daemon::run();
// }

static DAEMON: OnceLock<Daemon> = OnceLock::new();

pub struct Daemon {
    _runtime: Runtime,
}

impl Daemon {
    async fn start() -> Result<(), Box<dyn Error>> {
        let notification_daemon = NotificationDaemon::default();
        let _conn = connection::Builder::session()?
            .name("org.freedesktop.Notifications")?
            .serve_at("/org/freedesktop/Notifications", notification_daemon)?
            .build()
            .await?;

        pending::<()>().await;

        Ok(())
    }

    pub fn run() {
        let rt = Runtime::new().unwrap();
        rt.spawn(async { Self::start().await.unwrap() });

        DAEMON.get_or_init(|| Daemon { _runtime: rt });
    }
}
