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
                tray_app.update_device(tray::Device {
                    name: match r.keychron_device() {
                        Ok(kd) => kd.to_string(),
                        Err(_) => "".to_string(),
                    },
                    version: r.fr_version_string(),
                    battery: r.power.value,
                    dpi: *r
                        .dpi
                        .levels_val
                        .get(r.dpi.level[0] as usize)
                        .unwrap_or(&0u16),
                    polling_rate_level: r.polling_rate.level[0],
                });
            }
        });
    keychron_hid.poke_device(&dev)?;
    let (lres, rres) = tokio::join!(listen_handle, report_handle);
    let _ = lres?;
    let _ = rres?;
    Ok(())
}
