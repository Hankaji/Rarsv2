use std::sync::{Arc, LazyLock};

use device::DeviceProxy;
use tokio::runtime::Runtime;
use upower::UPowerProxy;
use zbus::Connection;

pub mod device;
pub mod upower;

static SERVICE_INSTANCE: LazyLock<Arc<Service>> = LazyLock::new(|| {
    Arc::new(
        Runtime::new()
            .unwrap()
            .block_on(async { Service::establish_conn().await.unwrap() }),
    )
});

// Service is mainly a struct wrapped around the zbus client with a single connection lel
pub struct Service<'s> {
    _connection: Connection,
    pub upower: UPowerProxy<'s>,
    pub devices: Vec<DeviceProxy<'s>>,
}

impl<'s> Service<'s> {
    pub fn new() -> Arc<Self> {
        (*SERVICE_INSTANCE).clone()
    }

    pub(self) async fn establish_conn() -> zbus::Result<Service<'s>> {
        let conn = Connection::system().await?;

        let up_proxy = UPowerProxy::new(&conn).await?;

        // Find all battery devices from user
        // WARNING: I don't have 2+ batteries nor 0 battery so cant really know how to test these
        // scenarios
        let devices_proxy: Vec<DeviceProxy> = {
            let mut devices_path = up_proxy.enumerate_devices().await?;
            devices_path.retain(|oop| {
                oop.to_string()
                    .contains("/org/freedesktop/UPower/devices/battery_BAT")
            });

            let t = devices_path.into_iter().map(|bat_dev_path| {
                DeviceProxy::builder(&conn)
                    .path(bat_dev_path.clone())
                    // TODO: Instead of panic, maybe log the error
                    .unwrap_or_else(|_| panic!("Could not find device at {bat_dev_path}"))
                    .build()
            });

            futures::future::join_all(t)
                .await
                .into_iter()
                .filter_map(|res| res.ok())
                .collect()
        };

        Ok(Service {
            _connection: conn,
            upower: up_proxy,
            devices: devices_proxy,
        })
    }
}
