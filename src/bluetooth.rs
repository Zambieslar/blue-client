use std::sync::mpsc::SyncSender;

use bluer::{Adapter, Address};
use futures::{pin_mut, StreamExt};

pub struct DeviceProperties {
    pub name: Option<String>,
    pub remote_address: Address,
    pub paired: bool,
    pub connected: bool,
    pub trusted: bool,
    pub blocked: bool,
    pub tx_power: Option<i16>,
    pub battey_percentage: Option<u8>,
}

impl DeviceProperties {
    async fn get_all(addr: Address, station: &Adapter) -> Self {
        let device = station.device(addr).unwrap();
        Self {
            name: device.name().await.unwrap(),
            remote_address: device.remote_address().await.unwrap(),
            paired: device.is_paired().await.unwrap(),
            connected: device.is_connected().await.unwrap(),
            trusted: device.is_trusted().await.unwrap(),
            blocked: device.is_blocked().await.unwrap(),
            tx_power: device.tx_power().await.unwrap(),
            battey_percentage: device.battery_percentage().await.unwrap(),
        }
    }
}

pub async fn blue_init(tx: std::sync::Arc<SyncSender<DeviceProperties>>) {
    let session = bluer::Session::new().await.unwrap();
    let station = session.default_adapter().await.unwrap();
    let discover = station.discover_devices().await.unwrap();
    pin_mut!(discover);
    while let Some(evt) = discover.next().await {
        match evt {
            bluer::AdapterEvent::DeviceAdded(device) => {
                let properties = DeviceProperties::get_all(device, &station).await;
                tx.send(properties).unwrap();
            }
            _ => {}
        }
    }
}

pub async fn bt_connect() {}
