use hidapi::{BusType, HidApi};

use crate::report::TryMerge;

mod report;

const KEYCHRON_VENDOR_ID: u16 = 0x3434;
const KEYCHRON_PRODUCT_ID: u16 = 0xd028;
const KEYCHRON_USAGE: u16 = 0x1;
const KEYCHRON_USAGE_PAGE: u16 = 0xffc1;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    HidApi::disable_device_discovery();
    let mut hid_api = HidApi::new()?;
    // TODO: use Keychron name to search instead of hardcoded ids
    hid_api.add_devices(KEYCHRON_VENDOR_ID, KEYCHRON_PRODUCT_ID)?;

    let mut dlit = hid_api.device_list();
    let dev = dlit
        .find(|d| {
            d.bus_type() as u8 == BusType::Usb as u8
                && d.usage() == KEYCHRON_USAGE
                && d.usage_page() == KEYCHRON_USAGE_PAGE
        })
        .ok_or("No matching devices found.")?;

    println!("{:#?}", dev);
    let hid_dev_read = dev.open_device(&hid_api)?;
    let hid_dev_write = dev.open_device(&hid_api)?;

    let handle = tokio::spawn(async move {
        let mut buf = [0u8; 64];
        let mut r = report::Report::default();
        loop {
            let buf_size;
            match hid_dev_read.read(&mut buf) {
                Ok(s) => buf_size = s,
                Err(e) => {
                    eprintln!("{}", e);
                    break;
                }
            }
            if buf_size == 0 {
                continue;
            }
            // strip reportId from buffer
            match r.merge(&buf[1..buf_size]) {
                Ok(r) => println!("{:#?}", r),
                Err(e) => eprintln!("{}", e),
            }
        }
    });

    {
        let mut req_info = [0u8; 21];
        req_info[0] = 0xb5;
        req_info[1] = 2;
        req_info[2] = 1;
        hid_dev_write.write(&req_info)?;
    }
    {
        let mut req_full = [0u8; 64];
        req_full[0] = 0xb3;
        req_full[1] = 6;
        hid_dev_write.write(&req_full)?;
    }
    handle.await?;
    Ok(())
}
