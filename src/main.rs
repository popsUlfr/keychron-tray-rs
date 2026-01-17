use tokio::time;

use crate::{keychron_hid::KeychronHid, tray::Tray};
use std::{error::Error, time::Duration};

mod keychron_device;
mod keychron_hid;
mod report;
mod tray;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut tray_app = Tray::new()?;
    let mut keychron_hid = KeychronHid::new()?;
    let dev = loop {
        let devs = keychron_hid.list_compatible_devices()?;
        if devs.is_empty() {
            time::sleep(Duration::from_secs(5)).await;
        }
        break devs[0].clone();
    };

    let (mut report_rx, listen_handle) = keychron_hid.listen(&dev)?;
    let report_handle: tokio::task::JoinHandle<Result<(), Box<dyn Error + Send + Sync>>> =
        tokio::spawn(async move {
            loop {
                report_rx.changed().await?;
                let r = report_rx.borrow_and_update();
                tray_app.set_battery_level(r.power.value);
            }
        });
    keychron_hid.poke_device(&dev)?;
    let (lres, rres) = tokio::join!(listen_handle, report_handle);
    let _ = lres?;
    let _ = rres?;
    Ok(())
}
