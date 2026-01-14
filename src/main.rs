use hidapi::{BusType, HidApi};

const KEYCHRON_VENDOR_ID: u16 = 0x3434;
const KEYCHRON_PRODUCT_ID: u16 = 0xd028;
const KEYCHRON_USAGE: u16 = 0x1;
const KEYCHRON_USAGE_PAGE: u16 = 0xffc1;

const REPORT_TYPE_INFO: u8 = 0x2;
const REPORT_TYPE_FULL: u8 = 0x6;
const REPORT_TYPE_LIGHT: u8 = 0xe1;
const REPORT_TYPE_BASE: u8 = 0xe2;
const REPORT_TYPE_PROFILE: u8 = 0xe5;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct ReportInfo {
    vid: u16,
    pid: u16,
    version: u16,
    fr_version: String,
    work_mode: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct ReportProfile {
    current: u8,
    level_num: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct ReportProfileLight {
    current: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct ReportDPI {
    level: [u8; 3],
    levels_val: [u16; 5],
    level_num: u8,
    level_val_max: u16,
    level_val_step: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct ReportDPIBase {
    level: u8,
    level_num: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct ReportDPIMisc {
    level: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct ReportPollingRate {
    level: [u8; 3],
    levels_val: [u8; 6],
    level_num: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct ReportPollingRateBase {
    level: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct ReportSysFeatures {
    lod: u8,
    wave: bool,
    line: bool,
    motion: bool,
    scroll_dir: bool,
    fps20k: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct ReportDebounce {
    value: u8,
    values: [u8; 10],
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct ReportScroll {
    speed: u8,
    inertia: u8,
    spl: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct ReportSleep {
    time: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct ReportPower {
    value: u8,
    state: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct ReportRouseOrigin {
    key: bool,
    key_support: bool,
    scroll: bool,
    scroll_support: bool,
    mmove: bool,
    move_support: bool,
    side_scroll: bool,
    side_scroll_support: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct ReportSupport {
    scroll_support: bool,
    debounce_support: bool,
    max_and_step_support: bool,
    polling_gears_support: bool,
    profile_support: bool,
    rouse_origin_support: bool,
    fps20k_support: bool,
    sleep_support: bool,
    loop_dpress_support: bool,
    pair_key_support: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct ReportFull {
    profile: ReportProfile,
    dpi: ReportDPI,
    polling_rate: ReportPollingRate,
    sys_features: ReportSysFeatures,
    debounce: ReportDebounce,
    scroll: ReportScroll,
    sleep: ReportSleep,
    power: ReportPower,
    rouse_origin: ReportRouseOrigin,
    support: ReportSupport,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct ReportLight {
    mode: u8,
    speed: u8,
    brightness: u8,
    rgb: [u8; 3],
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct ReportBase {
    work_mode: u8,
    connect: u8,
    power: ReportPower,
    dpi: ReportDPIBase,
    polling_rate: ReportPollingRateBase,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct ReportMisc {
    power: ReportPower,
    dpi: ReportDPIMisc,
    polling_rate: ReportPollingRateBase,
}

fn handle_input_report(buf: &[u8]) {
    if buf.len() < 2 {
        return;
    }
    let data = &buf[1..];
    match data[0] {
        REPORT_TYPE_INFO => {
            if data.len() < 10 {
                return;
            }
            let ri = ReportInfo {
                vid: ((data[4] as u16) << 8) | data[3] as u16,
                pid: ((data[6] as u16) << 8) | data[5] as u16,
                version: ((data[2] as u16) << 8) | data[1] as u16,
                fr_version: format!("{}.{}.{}", data[8], (data[7] >> 4) & 0xf, 0xf & data[7]),
                work_mode: data[9] & 7,
            };
            println!("{:#?}", ri);
        }
        REPORT_TYPE_FULL => {
            if data.len() < 54 {
                return;
            }
            let rf = ReportFull {
                profile: ReportProfile {
                    current: data[1],
                    level_num: data[50],
                },
                dpi: ReportDPI {
                    level: [0xf & data[2], 0xf & data[3], 0xf & data[4]],
                    levels_val: [
                        ((data[6] as u16) << 8) | data[5] as u16,
                        ((data[8] as u16) << 8) | data[7] as u16,
                        ((data[10] as u16) << 8) | data[9] as u16,
                        ((data[12] as u16) << 8) | data[11] as u16,
                        ((data[14] as u16) << 8) | data[13] as u16,
                    ],
                    level_num: data[16],
                    level_val_max: ((data[41] as u16) << 8) | data[40] as u16,
                    level_val_step: if data[42] == 0 { 50 } else { data[42] },
                },
                polling_rate: ReportPollingRate {
                    level: [
                        (data[2] >> 4) & 15,
                        (data[3] >> 4) & 15,
                        (data[4] >> 4) & 15,
                    ],
                    levels_val: data[43..49].try_into().unwrap_or_default(),
                    level_num: if data[49] == 0 { 6 } else { data[49] },
                },
                sys_features: ReportSysFeatures {
                    lod: 3 & data[15],
                    wave: ((data[15] >> 2) & 1) != 0,
                    line: ((data[15] >> 3) & 1) != 0,
                    motion: ((data[15] >> 4) & 1) != 0,
                    scroll_dir: ((data[15] >> 6) & 1) != 0,
                    fps20k: (1 & data[52]) != 0,
                },
                debounce: ReportDebounce {
                    value: data[17],
                    values: data[30..40].try_into().unwrap_or_default(),
                },
                scroll: ReportScroll {
                    speed: data[27],
                    inertia: data[28],
                    spl: data[29],
                },
                sleep: ReportSleep { time: data[18] },
                power: ReportPower {
                    value: 0x7f & data[19],
                    state: ((data[19] >> 7) & 1) != 0,
                },
                rouse_origin: ReportRouseOrigin {
                    key: ((data[51] >> 4) & 1) != 0,
                    key_support: (1 & data[51]) != 0,
                    scroll: ((data[51] >> 5) & 1) != 0,
                    scroll_support: (2 & data[51]) != 0,
                    mmove: ((data[51] >> 6) & 1) != 0,
                    move_support: (4 & data[51]) != 0,
                    side_scroll: ((data[51] >> 7) & 1) != 0,
                    side_scroll_support: (8 & data[51]) != 0,
                },
                support: ReportSupport {
                    scroll_support: (1 & data[26]) != 0,
                    debounce_support: (2 & data[26]) != 0,
                    max_and_step_support: (8 & data[26]) != 0,
                    polling_gears_support: (16 & data[26]) != 0,
                    profile_support: (32 & data[26]) != 0,
                    rouse_origin_support: (64 & data[26]) != 0,
                    fps20k_support: (128 & data[26]) != 0,
                    sleep_support: false,
                    loop_dpress_support: true,
                    pair_key_support: ((data[53] >> 1) & 1) != 0,
                },
            };
            println!("{:#?}", rf);
        }
        REPORT_TYPE_LIGHT => {
            if data.len() < 8 {
                return;
            }
            let rl = ReportLight {
                mode: data[1],
                speed: data[3],
                brightness: data[4],
                rgb: [data[5], data[6], data[7]],
            };
            println!("{:#?}", rl);
        }
        REPORT_TYPE_BASE => {
            if data.len() < 8 {
                return;
            }
            let rb = ReportBase {
                work_mode: data[1],
                connect: data[2],
                power: ReportPower {
                    value: data[4],
                    state: data[3] != 0,
                },
                dpi: ReportDPIBase {
                    level: data[5],
                    level_num: data[7],
                },
                polling_rate: ReportPollingRateBase { level: data[6] },
            };
            println!("{:#?}", rb);
        }
        REPORT_TYPE_PROFILE => {
            if data.len() < 2 {
                return;
            }
            let rp = ReportProfileLight { current: data[1] };
            println!("{:#?}", rp);
        }
        1 | 65 => {
            if (data[2] == 140 || data[2] == 142) && data[3] == 1 {
                let rm = ReportMisc {
                    power: ReportPower {
                        value: data[6],
                        state: data[5] != 0,
                    },
                    dpi: ReportDPIMisc { level: data[8] },
                    polling_rate: ReportPollingRateBase {
                        level: if data[9] > 0 { data[9] - 1 } else { data[9] },
                    },
                };
                println!("{:#?}", rm);
            }
        }
        _ => (),
    }
}

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
            handle_input_report(&buf[..buf_size]);
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
