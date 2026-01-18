use tokio::time;

use crate::{keychron_hid::KeychronHid, tray::Tray, udev::udev_rule_install};
use std::{error::Error, time::Duration};

mod keychron_device;
mod keychron_hid;
mod report;
mod tray;
mod udev;

const DEVICE_CHECK_PERIOD: Duration = Duration::from_secs(5);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut tray_app = Tray::new()?;
    let (keychron_hid, mut report_rx, listen_handle, dev) = loop {
        let mut keychron_hid = KeychronHid::new()?;
        let dev = loop {
            let devs = keychron_hid.list_compatible_devices()?;
            if devs.is_empty() {
                time::sleep(DEVICE_CHECK_PERIOD).await;
                continue;
            }
            break devs[0].clone();
        };
        match keychron_hid.listen(&dev) {
            Ok((r, l)) => break (keychron_hid, r, l, dev),
            Err(e) => {
                if e.to_string()
                    .to_lowercase()
                    .find("permission denied")
                    .is_some()
                {
                    tray_app.needs_udev_rules(true);
                    if !tray_app.notify_udev_rules().await.is_ok_and(|r| r) {
                        time::sleep(DEVICE_CHECK_PERIOD).await;
                    }
                } else {
                    return Err(e.into());
                }
            }
        }
    };
    tray_app.needs_udev_rules(false);
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
                    connected: !r.power.state,
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
