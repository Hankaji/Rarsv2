use std::sync::{Arc, OnceLock};

use device::{DeviceProxy, DeviceProxyBlocking};
use upower::{UPowerProxy, UPowerProxyBlocking};
use zbus::{blocking::Connection as BConnection, Connection};

pub mod device;
pub mod upower;

// My stupid brain will never be able to understand the concept of asynchronous and runtime in rust

static SERVICE_INSTANCE: OnceLock<Arc<Service>> = OnceLock::new();
static SERVICE_BLOCKING_INSTANCE: OnceLock<Arc<ServiceBlocking>> = OnceLock::new();

// Service is mainly a struct wrapped around the zbus client with a single connection lel
// TODO: Make a Proc-macro where it implement bind(FnOnce) for each property in Proxy
#[derive(Debug)]
pub struct Service {
    _connection: Connection,
    pub upower: UPowerProxy<'static>,
    pub devices: Vec<DeviceProxy<'static>>,
}

impl Service {
    pub async fn init() -> zbus::Result<Arc<Service>> {
        let service = Self::establish_conn().await?;
        let instance = SERVICE_INSTANCE.get_or_init(|| Arc::new(service));

        Ok(instance.clone())
    }

    pub(self) async fn establish_conn() -> zbus::Result<Service> {
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

        // println!(
        //     "Inside lib percentage: {}",
        //     devices_proxy[0].percentage().await?
        // );

        Ok(Service {
            _connection: conn,
            upower: up_proxy,
            devices: devices_proxy,
        })
    }
}

#[derive(Debug)]
pub struct ServiceBlocking {
    _connection: BConnection,
    pub upower: UPowerProxyBlocking<'static>,
    pub devices: Vec<DeviceProxyBlocking<'static>>,
}

impl ServiceBlocking {
    pub fn init() -> zbus::Result<Arc<ServiceBlocking>> {
        let instance = SERVICE_BLOCKING_INSTANCE
            .get_or_init(|| Arc::new(ServiceBlocking::establish_conn().unwrap()));

        Ok(instance.clone())
    }

    pub(self) fn establish_conn() -> zbus::Result<ServiceBlocking> {
        let conn = BConnection::system()?;

        let up_proxy = UPowerProxyBlocking::new(&conn)?;

        // Find all battery devices from user
        // WARNING: I don't have 2+ batteries nor 0 battery so cant really know how to test these
        // scenarios
        let devices_proxy: Vec<DeviceProxyBlocking> = {
            let mut devices_path = up_proxy.enumerate_devices()?;
            devices_path.retain(|oop| {
                oop.to_string()
                    .contains("/org/freedesktop/UPower/devices/battery_BAT")
            });

            devices_path
                .into_iter()
                .map(|bat_dev_path| {
                    DeviceProxyBlocking::builder(&conn)
                        .path(bat_dev_path.clone())
                        // TODO: Instead of panic, maybe log the error
                        .unwrap_or_else(|_| panic!("Could not find device at {bat_dev_path}"))
                        .build()
                        .unwrap()
                })
                .collect()
        };

        // println!(
        //     "Inside lib percentage: {}",
        //     devices_proxy[0].percentage().await?
        // );

        Ok(ServiceBlocking {
            _connection: conn,
            upower: up_proxy,
            devices: devices_proxy,
        })
    }
}
