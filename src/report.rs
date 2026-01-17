use crate::keychron_device::KeychronDevice;
use num_enum::TryFromPrimitiveError;

pub const REPORT_TYPE_UNKNOWN_1: u8 = 1;
pub const REPORT_TYPE_DESCRIPTION: u8 = 2;
pub const REPORT_TYPE_UNKNOWN_4: u8 = 4;
pub const REPORT_TYPE_FULL: u8 = 6;
pub const REPORT_TYPE_UNKNOWN_7: u8 = 7;
pub const REPORT_TYPE_UNKNOWN_65: u8 = 65;
pub const REPORT_TYPE_UNKNOWN_68: u8 = 68;
pub const REPORT_TYPE_LIGHT: u8 = 225;
pub const REPORT_TYPE_BASE: u8 = 226;
pub const REPORT_TYPE_PROFILE: u8 = 229;

pub trait TryMerge<T> {
    type Error;
    fn merge(&mut self, value: T) -> Result<&mut Self, Self::Error>;
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Copy)]
pub struct ReportProfile {
    pub current: u8,
    pub level_num: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Copy)]
pub struct ReportDPI {
    pub level: [u8; 3],
    pub levels_val: [u16; 5],
    pub level_num: u8,
    pub level_val_max: u16,
    pub level_val_step: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Copy)]
pub struct ReportPollingRate {
    pub level: [u8; 3],
    pub levels_val: [u8; 6],
    pub level_num: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Copy)]
pub struct ReportSysFeatures {
    pub lod: bool,
    pub wave: bool,
    pub line: bool,
    pub motion: bool,
    pub scroll_dir: bool,
    pub fps20k: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Copy)]
pub struct ReportDebounce {
    pub value: u8,
    pub values: [u8; 10],
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Copy)]
pub struct ReportScroll {
    pub speed: u8,
    pub inertia: u8,
    pub spl: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Copy)]
pub struct ReportSleep {
    pub time: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Copy)]
pub struct ReportPower {
    pub value: u8,
    pub state: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Copy)]
pub struct ReportRouseOrigin {
    pub key: bool,
    pub key_support: bool,
    pub scroll: bool,
    pub scroll_support: bool,
    pub mmove: bool,
    pub move_support: bool,
    pub side_scroll: bool,
    pub side_scroll_support: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Copy)]
pub struct ReportSupport {
    pub scroll_support: bool,
    pub debounce_support: bool,
    pub max_and_step_support: bool,
    pub polling_gears_support: bool,
    pub profile_support: bool,
    pub rouse_origin_support: bool,
    pub fps20k_support: bool,
    pub sleep_support: bool,
    pub loop_dpress_support: bool,
    pub pair_key_support: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Copy)]
pub struct ReportLight {
    pub mode: u8,
    pub speed: u8,
    pub brightness: u8,
    pub rgb: [u8; 3],
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Copy)]
pub struct Report {
    pub vid: u16,
    pub pid: u16,
    pub version: u16,
    pub fr_version: [u8; 3],
    pub work_mode: u8,
    pub connect: u8,
    pub profile: ReportProfile,
    pub dpi: ReportDPI,
    pub polling_rate: ReportPollingRate,
    pub sys_features: ReportSysFeatures,
    pub debounce: ReportDebounce,
    pub scroll: ReportScroll,
    pub sleep: ReportSleep,
    pub power: ReportPower,
    pub rouse_origin: ReportRouseOrigin,
    pub support: ReportSupport,
    pub light: ReportLight,
}

impl Report {
    pub fn fr_version_string(&self) -> String {
        format!(
            "{}.{}.{}",
            self.fr_version[0], self.fr_version[1], self.fr_version[2]
        )
    }

    pub fn vendor_product_id(&self) -> u32 {
        65536u32 * self.vid as u32 + self.pid as u32
    }

    pub fn keychron_device(&self) -> Result<KeychronDevice, TryFromPrimitiveError<KeychronDevice>> {
        self.pid.try_into()
    }
}

impl TryMerge<&[u8]> for Report {
    type Error = &'static str;
    fn merge(&mut self, value: &[u8]) -> Result<&mut Self, Self::Error> {
        match value[0] {
            REPORT_TYPE_DESCRIPTION => {
                if value.len() < 10 {
                    return Err("Not enough data.");
                }
                self.vid = ((value[4] as u16) << 8) | (value[3] as u16);
                self.pid = ((value[6] as u16) << 8) | (value[5] as u16);
                self.version = ((value[2] as u16) << 8) | (value[1] as u16);
                self.fr_version = [value[8], (value[7] >> 4) & 15, value[7] & 15];
                self.work_mode = value[9] & 7;
            }
            REPORT_TYPE_FULL => {
                if value.len() < 54 {
                    return Err("Not enough data.");
                }
                self.profile.current = value[1];
                self.profile.level_num = value[50];
                self.dpi.level = [value[2] & 15, value[3] & 15, value[4] & 15];
                self.dpi.levels_val = [
                    (value[6] as u16) << 8 | (value[5] as u16),
                    (value[8] as u16) << 8 | (value[7] as u16),
                    (value[10] as u16) << 8 | (value[9] as u16),
                    (value[12] as u16) << 8 | (value[11] as u16),
                    (value[14] as u16) << 8 | (value[13] as u16),
                ];
                self.dpi.level_num = value[16];
                self.dpi.level_val_max = ((value[41] as u16) << 8) | (value[40] as u16);
                self.dpi.level_val_step = if value[42] != 0 { value[42] } else { 50 };
                self.polling_rate.level =
                    [value[2] >> 4 & 15, value[3] >> 4 & 15, value[4] >> 4 & 15];
                self.polling_rate.levels_val = value[43..49].try_into().unwrap_or_default();
                self.polling_rate.level_num = if value[49] != 0 { value[49] } else { 6 };
                self.sys_features.lod = (value[15] & 3) != 0;
                self.sys_features.wave = ((value[15] >> 2) & 1) != 0;
                self.sys_features.line = ((value[15] >> 3) & 1) != 0;
                self.sys_features.motion = ((value[15] >> 4) & 1) != 0;
                self.sys_features.scroll_dir = ((value[15] >> 6) & 1) != 0;
                self.sys_features.fps20k = (value[52] & 1) != 0;
                self.debounce.value = value[17];
                self.debounce.values = value[30..40].try_into().unwrap_or_default();
                self.scroll.speed = value[27];
                self.scroll.inertia = value[28];
                self.scroll.spl = value[29];
                self.sleep.time = value[18] as u16;
                self.power.value = value[19] & 127;
                self.power.state = ((value[19] >> 7) & 1) != 0;
                self.rouse_origin.key = ((value[51] >> 4) & 1) != 0;
                self.rouse_origin.key = ((value[51] >> 4) & 1) != 0;
                self.rouse_origin.key_support = (value[51] & 1) != 0;
                self.rouse_origin.scroll = ((value[51] >> 5) & 1) != 0;
                self.rouse_origin.scroll_support = (value[51] & 2) != 0;
                self.rouse_origin.mmove = ((value[51] >> 6) & 1) != 0;
                self.rouse_origin.move_support = (value[51] & 4) != 0;
                self.rouse_origin.side_scroll = ((value[51] >> 7) & 1) != 0;
                self.rouse_origin.side_scroll_support = (value[51] & 8) != 0;
                self.support.scroll_support = (value[26] & 1) != 0;
                self.support.debounce_support = (value[26] & 2) != 0;
                self.support.max_and_step_support = (value[26] & 8) != 0;
                self.support.polling_gears_support = (value[26] & 16) != 0;
                self.support.profile_support = (value[26] & 32) != 0;
                self.support.rouse_origin_support = (value[26] & 64) != 0;
                self.support.fps20k_support = (value[26] & 128) != 0;
                //self.support.sleep_support = false; // why ?
                self.support.loop_dpress_support = true; // why ?
                self.support.pair_key_support = ((value[53] >> 1) & 1) != 0;
            }
            REPORT_TYPE_LIGHT => {
                if value.len() < 8 {
                    return Err("Not enough data.");
                }
                self.light.mode = value[1];
                self.light.speed = value[3];
                self.light.brightness = value[4];
                self.light.rgb = [value[5], value[6], value[7]];
            }
            REPORT_TYPE_BASE => {
                if value.len() < 8 {
                    return Err("Not enough data.");
                }
                self.work_mode = value[1];
                self.connect = value[2];
                self.power.state = value[3] != 0;
                self.power.value = value[4];
                self.dpi.level = [value[5], value[5], value[5]];
                self.dpi.level_num = value[7];
                self.polling_rate.level = [value[6], value[6], value[6]];
            }
            REPORT_TYPE_PROFILE => {
                if value.len() < 2 {
                    return Err("Not enough data.");
                }
                self.profile.current = value[1];
            }
            REPORT_TYPE_UNKNOWN_1 | REPORT_TYPE_UNKNOWN_65 => {
                if value.len() < 10 {
                    return Err("Not enough data.");
                }
                if (value[2] == 140 || value[2] == 142) && value[3] == 1 {
                    self.power.state = value[5] != 0;
                    self.power.value = value[6];
                    self.dpi.level = [value[8], value[8], value[8]];
                    let level = if value[9] > 0 { value[9] - 1 } else { value[9] };
                    self.polling_rate.level = [level, level, level];
                }
            }
            REPORT_TYPE_UNKNOWN_4 | REPORT_TYPE_UNKNOWN_68 => {
                if value.len() < 55 {
                    return Err("Not enough data.");
                }
                if value[3] == 1 {
                    self.dpi.level = [value[7], value[7], value[7]];
                    self.dpi.levels_val = [
                        (value[9] as u16) << 8 | (value[8] as u16),
                        (value[13] as u16) << 8 | (value[12] as u16),
                        (value[17] as u16) << 8 | (value[16] as u16),
                        (value[21] as u16) << 8 | (value[20] as u16),
                        (value[25] as u16) << 8 | (value[24] as u16),
                    ];
                    self.dpi.level_num = value[6].count_ones() as u8;
                    self.sys_features.lod = value[43] != 0;
                    self.sys_features.wave = ((value[4] >> 4) & 1) != 0;
                    self.sys_features.line = (value[4] & 1) != 0;
                    self.sys_features.motion = ((value[4] >> 5) & 1) != 0;
                    self.sys_features.scroll_dir = ((value[4] >> 7) & 1) != 0;
                    self.debounce.value = value[54];
                    self.sleep.time = ((value[53] as u16) << 8) | (value[52] as u16);
                    let level = if value[5] > 1 { value[5] - 1 } else { 0 };
                    self.polling_rate.level = [level, level, level];
                    //self.polling_rate.levels_val = [0, 1, 2, 3, 4, 5]; // why ?
                    //self.polling_rate.level_num = report_rate_max;
                }
            }
            REPORT_TYPE_UNKNOWN_7 => {
                if value.len() < 18 {
                    return Err("Not enough data.");
                }
                self.profile.current = value[1];
                self.profile.level_num = 1;
                self.dpi.level = [value[2] & 15, value[3] & 16, value[4] & 15];
                self.dpi.levels_val = [
                    (value[6] as u16) << 8 | (value[5] as u16),
                    (value[8] as u16) << 8 | (value[7] as u16),
                    (value[10] as u16) << 8 | (value[9] as u16),
                    (value[12] as u16) << 8 | (value[11] as u16),
                    (value[14] as u16) << 8 | (value[13] as u16),
                ];
                self.dpi.level_num = value[16];
                self.polling_rate.level =
                    [value[2] >> 4 & 15, value[3] >> 4 & 15, value[4] >> 4 & 15];
                //self.polling_rate.levels_val = [0, 1, 2, 0, 0, 0]; // Why is this shorter ?
                self.polling_rate.level_num = 0;
                self.sys_features.lod = (value[15] & 3) != 0;
                self.sys_features.wave = (value[15] & 4) != 0;
                self.sys_features.line = (value[15] & 8) != 0;
                self.sys_features.motion = (value[15] & 16) != 0;
                self.sys_features.scroll_dir = (value[15] & 64) != 0;
                //self.sys_features.fps20k = false;
                self.debounce.value = value[17];
            }
            _ => println!("Type unknown: {}", value[0]),
        }
        Ok(self)
    }
}

impl TryFrom<&[u8]> for Report {
    type Error = &'static str;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Report::default().merge(value).map(|r| r.to_owned())
    }
}
