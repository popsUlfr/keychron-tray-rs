use crate::{
    keychron_device::KeychronDevice,
    report::{Report, TryMerge},
};
use hidapi::{BusType, DeviceInfo, HidApi, HidError};
use std::error::Error;
use tokio::{sync::watch, task};

pub const KEYCHRON_VENDOR_ID: u16 = 0x3434;
pub const KEYCHRON_PRODUCT_ID: u16 = 0xd028;
pub const KEYCHRON_USAGE: u16 = 0x1;
pub const KEYCHRON_USAGE_PAGE: u16 = 0xffc1;

pub struct KeychronHid {
    hid_api: HidApi,
}

impl KeychronHid {
    pub fn new() -> Result<Self, HidError> {
        HidApi::disable_device_discovery();
        let hid_api = HidApi::new()?;
        Ok(KeychronHid { hid_api })
    }

    // Lists the compatible Keychron devices
    pub fn list_compatible_devices(&mut self) -> Result<Vec<&DeviceInfo>, HidError> {
        self.hid_api.reset_devices()?;
        self.hid_api.add_devices(KEYCHRON_VENDOR_ID, 0)?;
        Ok(self
            .hid_api
            .device_list()
            .filter(|d| {
                d.bus_type() as u8 == BusType::Usb as u8
                    && d.usage() == KEYCHRON_USAGE
                    && d.usage_page() == KEYCHRON_USAGE_PAGE
                    && TryInto::<KeychronDevice>::try_into(d.product_id()).is_ok()
            })
            .collect())
    }

    pub fn listen(
        &self,
        dev: &DeviceInfo,
    ) -> Result<
        (
            watch::Receiver<Report>,
            task::JoinHandle<Result<(), Box<dyn Error + Send + Sync>>>,
        ),
        HidError,
    > {
        let hid_dev_read = dev.open_device(&self.hid_api)?;
        let (tx, rx) = watch::channel(Report::default());
        let handle = task::spawn_blocking(move || {
            let mut buf = [0u8; 64];
            let mut r = Report::default();
            loop {
                let buf_size = hid_dev_read.read(&mut buf)?;
                if buf_size == 0 {
                    continue;
                }
                // strip reportId from buffer
                r.merge(&buf[1..buf_size])?;
                tx.send(r)?;
            }
        });
        Ok((rx, handle))
    }

    // Poke the device to have it report its status
    pub fn poke_device(&self, dev: &DeviceInfo) -> Result<(), HidError> {
        let hid_dev_write = dev.open_device(&self.hid_api)?;
        {
            let mut req_info = [0u8; 21];
            req_info[0] = 181;
            req_info[1] = 2;
            req_info[2] = 1;
            hid_dev_write.write(&req_info)?;
        }
        {
            let mut req_full = [0u8; 64];
            req_full[0] = 179;
            req_full[1] = 6;
            hid_dev_write.write(&req_full)?;
        }
        Ok(())
    }
}
