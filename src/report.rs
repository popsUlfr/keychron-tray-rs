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
    current: u8,
    level_num: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Copy)]
pub struct ReportDPI {
    level: [u8; 3],
    levels_val: [u16; 5],
    level_num: u8,
    level_val_max: u16,
    level_val_step: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Copy)]
pub struct ReportPollingRate {
    level: [u8; 3],
    levels_val: [u8; 6],
    level_num: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Copy)]
pub struct ReportSysFeatures {
    lod: bool,
    wave: bool,
    line: bool,
    motion: bool,
    scroll_dir: bool,
    fps20k: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Copy)]
pub struct ReportDebounce {
    value: u8,
    values: [u8; 10],
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Copy)]
pub struct ReportScroll {
    speed: u8,
    inertia: u8,
    spl: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Copy)]
pub struct ReportSleep {
    time: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Copy)]
pub struct ReportPower {
    value: u8,
    state: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Copy)]
pub struct ReportRouseOrigin {
    key: bool,
    key_support: bool,
    scroll: bool,
    scroll_support: bool,
    mmove: bool,
    move_support: bool,
    side_scroll: bool,
    side_scroll_support: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Copy)]
pub struct ReportSupport {
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

#[derive(Debug, Clone, PartialEq, Eq, Default, Copy)]
pub struct ReportLight {
    mode: u8,
    speed: u8,
    brightness: u8,
    rgb: [u8; 3],
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Report {
    vid: u16,
    pid: u16,
    version: u16,
    fr_version: String,
    work_mode: u8,
    connect: u8,
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
    light: ReportLight,
}

impl TryMerge<&[u8]> for Report {
    type Error = &'static str;
    fn merge(&mut self, value: &[u8]) -> Result<&mut Self, Self::Error> {
        match value[0] {
            REPORT_TYPE_DESCRIPTION => {
                println!("Type description");
                if value.len() < 10 {
                    return Err("Not enough data.");
                }
                self.vid = ((value[4] as u16) << 8) | (value[3] as u16);
                self.pid = ((value[6] as u16) << 8) | (value[5] as u16);
                self.version = ((value[2] as u16) << 8) | (value[1] as u16);
                self.fr_version =
                    format!("{}.{}.{}", value[8], (value[7] >> 4) & 15, value[7] & 15);
                self.work_mode = value[9] & 7;
            }
            REPORT_TYPE_FULL => {
                println!("Type full");
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
                println!("Type light");
                if value.len() < 8 {
                    return Err("Not enough data.");
                }
                self.light.mode = value[1];
                self.light.speed = value[3];
                self.light.brightness = value[4];
                self.light.rgb = [value[5], value[6], value[7]];
            }
            REPORT_TYPE_BASE => {
                println!("Type base");
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
                println!("Type profile");
                if value.len() < 2 {
                    return Err("Not enough data.");
                }
                self.profile.current = value[1];
            }
            REPORT_TYPE_UNKNOWN_1 | REPORT_TYPE_UNKNOWN_65 => {
                println!("Type 1 or 65: {}", value[0]);
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
                println!("Type 4 or 68: {}", value[0]);
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
                println!("Type 7");
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
