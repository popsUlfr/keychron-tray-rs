use tokio::{sync::Mutex, time};

use crate::{keychron_hid::KeychronHid, tray::Tray};
use std::{error::Error, sync::Arc, time::Duration};

mod keychron_device;
mod keychron_hid;
mod report;
mod tray;
mod udev;

const DEVICE_CHECK_PERIOD: Duration = Duration::from_secs(5);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tray_app = Arc::new(Mutex::new(Tray::new()?));
    loop {
        {
            let mut tray_app_lock = tray_app.lock().await;
            tray_app_lock.clear();
        }
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
                        let mut tray_app_lock = tray_app.lock().await;
                        tray_app_lock.needs_udev_rules(true);
                        if !tray_app_lock.notify_udev_rules().await.is_ok_and(|r| r) {
                            time::sleep(DEVICE_CHECK_PERIOD).await;
                        }
                    } else {
                        return Err(e.into());
                    }
                }
            }
        };
        {
            let mut tray_app_lock = tray_app.lock().await;
            tray_app_lock.needs_udev_rules(false);
        }
        let tray_app2 = tray_app.clone();
        let report_handle: tokio::task::JoinHandle<Result<(), Box<dyn Error + Send + Sync>>> =
            tokio::spawn(async move {
                loop {
                    report_rx.changed().await?;
                    let dev = {
                        let r = report_rx.borrow_and_update();
                        if r.power.value == 255 {
                            return Err("reset".into());
                        }
                        tray::Device {
                            name: match r.keychron_device() {
                                Ok(kd) => kd.to_string(),
                                Err(_) => "".to_string(),
                            },
                            version: r.fr_version_string(),
                            charging: r.power.state,
                            battery: r.power.value,
                            dpi: *r
                                .dpi
                                .levels_val
                                .get(r.dpi.level[0] as usize)
                                .unwrap_or(&0u16),
                            polling_rate_level: r.polling_rate.level[0],
                        }
                    };
                    let mut tray_app_lock = tray_app2.lock().await;
                    tray_app_lock.update_device(dev);
                }
            });

        let poke_handle = tokio::task::spawn_blocking(move || keychron_hid.poke_device(&dev));
        let res = tokio::try_join!(listen_handle, report_handle, poke_handle);
        match res {
            Ok((l, r, p)) => {
                if let Some(e) = l.err() {
                    if e.to_string()
                        .to_lowercase()
                        .find("input/output error")
                        .is_some()
                    {
                        continue;
                    } else {
                        return Err(e.to_string().into());
                    }
                }
                if let Some(e) = r.err() {
                    if e.to_string().to_lowercase().find("reset").is_some() {
                        continue;
                    } else {
                        return Err(e.to_string().into());
                    }
                }
                if let Some(e) = p.err() {
                    return Err(e.into());
                }
            }
            Err(e) => {
                return Err(e.into());
            }
        }
        break;
    }
    Ok(())
}
