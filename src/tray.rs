use std::{error, fmt, process::exit, str};

use notify_rust::Notification;
#[cfg(target_os = "linux")]
use notify_rust::Timeout;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use trayicon::{Icon, MenuBuilder, TrayIcon, TrayIconBuilder, TrayIconStatus};

#[cfg(target_os = "linux")]
use crate::udev;

const KEYCHRON_URL: &str = "https://launcher.keychron.com";
const ICON_NORMAL_BYTES: &[u8] = include_bytes!("../assets/Keychron_icon.ico");
const ICON_BAT_FULL_BYTES: &[u8] = include_bytes!("../assets/Keychron_icon_bat_full.ico");
const ICON_BAT_GOOD_BYTES: &[u8] = include_bytes!("../assets/Keychron_icon_bat_good.ico");
const ICON_BAT_HALF_BYTES: &[u8] = include_bytes!("../assets/Keychron_icon_bat_half.ico");
const ICON_BAT_LOW_BYTES: &[u8] = include_bytes!("../assets/Keychron_icon_bat_low.ico");

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
enum TrayEvent {
    #[default]
    None,
    #[cfg(target_os = "linux")]
    UdevRules,
    Configure,
    Close,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
enum PollLevel {
    #[default]
    Level0,
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub struct ParsePollLevelError;

impl fmt::Display for ParsePollLevelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Invalid poll level given")
    }
}

impl error::Error for ParsePollLevelError {
    fn description(&self) -> &str {
        "invalid poll level"
    }
}

impl fmt::Display for PollLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PollLevel::Level0 => f.write_str("⚪"),
            PollLevel::Level1 => f.write_str("🔵"),
            PollLevel::Level2 => f.write_str("🔴"),
            PollLevel::Level3 => f.write_str("⚪🔵"),
            PollLevel::Level4 => f.write_str("⚪🔴"),
            PollLevel::Level5 => f.write_str("⚪🔵🔴"),
        }
    }
}

impl str::FromStr for PollLevel {
    type Err = ParsePollLevelError;

    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        match s.replace(" ", "").to_uppercase().as_str() {
            "⚪" => Ok(PollLevel::Level0),
            "🔵" => Ok(PollLevel::Level1),
            "🔴" => Ok(PollLevel::Level2),
            "⚪🔵" => Ok(PollLevel::Level3),
            "⚪🔴" => Ok(PollLevel::Level4),
            "⚪🔵🔴" => Ok(PollLevel::Level5),
            _ => Err(ParsePollLevelError),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Device {
    pub name: String,
    pub version: String,
    pub battery: u8,
    pub charging: bool,
    pub dpi: u16,
    pub polling_rate_level: u8,
}

pub struct Tray {
    tray_icon: TrayIcon<TrayEvent>,
    icon: Icon,
    bat_icons: [Icon; 4],
    dev: Option<Device>,
    #[cfg(target_os = "linux")]
    install_udev_rules: bool,
    changes: usize,
}

unsafe impl Send for Tray {}
unsafe impl Sync for Tray {}

impl Tray {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let icon_normal = Icon::from_buffer(ICON_NORMAL_BYTES, None, None).unwrap();
        let icon_bat_full = Icon::from_buffer(ICON_BAT_FULL_BYTES, None, None).unwrap();
        let icon_bat_good = Icon::from_buffer(ICON_BAT_GOOD_BYTES, None, None).unwrap();
        let icon_bat_half = Icon::from_buffer(ICON_BAT_HALF_BYTES, None, None).unwrap();
        let icon_bat_low = Icon::from_buffer(ICON_BAT_LOW_BYTES, None, None).unwrap();
        let mut tray_icon = TrayIconBuilder::new()
            .icon(icon_normal.clone())
            .title("Keychron")
            .tooltip("Keychron")
            .sender(move |evt: &TrayEvent| match evt {
                TrayEvent::Configure => {
                    webbrowser::open(KEYCHRON_URL).ok();
                }
                #[cfg(target_os = "linux")]
                TrayEvent::UdevRules => {
                    tokio::spawn(async move {
                        let _ = udev::udev_rule_install().await;
                    });
                }
                TrayEvent::Close => {
                    exit(0);
                }
                _ => (),
            })
            .menu(
                MenuBuilder::new()
                    .item("Configure", TrayEvent::Configure)
                    .separator()
                    .item("✖ Close", TrayEvent::Close),
            )
            .build()?;
        tray_icon.set_status(trayicon::TrayIconStatus::Active).ok();
        Ok(Tray {
            tray_icon,
            icon: icon_normal,
            bat_icons: [icon_bat_low, icon_bat_half, icon_bat_good, icon_bat_full],
            dev: None,
            #[cfg(target_os = "linux")]
            install_udev_rules: false,
            changes: 0,
        })
    }

    fn gen_menu(&self) -> MenuBuilder<TrayEvent> {
        let mut mb = MenuBuilder::new();
        #[cfg(target_os = "linux")]
        if self.install_udev_rules {
            mb = mb.item("Install udev rules", TrayEvent::UdevRules);
        }
        if let Some(dev) = &self.dev {
            mb = mb
                .item(format!("🖱️{}", dev.name).as_str(), TrayEvent::None)
                .separator()
                .item(
                    format!(
                        "┣{}{}%",
                        if dev.charging {
                            "⚡"
                        } else {
                            if dev.battery <= 25 { "🪫" } else { "🔋" }
                        },
                        dev.battery
                    )
                    .as_str(),
                    TrayEvent::None,
                )
                .item(format!("┣📏{} dpi", dev.dpi).as_str(), TrayEvent::None)
                .item(
                    format!(
                        "┣⏱{}:{}",
                        dev.polling_rate_level,
                        TryInto::<PollLevel>::try_into(dev.polling_rate_level).unwrap_or_default()
                    )
                    .as_str(),
                    TrayEvent::None,
                )
                .item(format!("┗🛈{}", dev.version).as_str(), TrayEvent::None)
                .separator();
        }
        mb.item("Configure", TrayEvent::Configure)
            .separator()
            .item("✖ Close", TrayEvent::Close)
    }

    pub fn clear(&mut self) {
        self.dev = None;
        self.changes = 0;
        self.tray_icon.set_tooltip("Keychron").ok();
        self.tray_icon.set_status(TrayIconStatus::Passive).ok();
        self.tray_icon.set_icon(&self.icon).ok();
        self.tray_icon.set_menu(&self.gen_menu()).ok();
    }

    pub fn update_device(&mut self, dev: Device) {
        if let Some(old_dev) = &self.dev {
            if self.changes > 0 {
                if old_dev.dpi != dev.dpi {
                    Notification::new()
                        .appname(dev.name.as_str())
                        .summary(format!("{} dpi", dev.dpi).as_str())
                        .icon("input-mouse")
                        .urgency(notify_rust::Urgency::Low)
                        .show()
                        .ok();
                }
                if old_dev.polling_rate_level != dev.polling_rate_level {
                    Notification::new()
                        .appname(dev.name.as_str())
                        .summary(
                            format!(
                                "Polling rate {} {}",
                                dev.polling_rate_level,
                                TryInto::<PollLevel>::try_into(dev.polling_rate_level)
                                    .unwrap_or_default()
                            )
                            .as_str(),
                        )
                        .icon("input-mouse")
                        .urgency(notify_rust::Urgency::Low)
                        .show()
                        .ok();
                }
            }
            self.changes += 1;
        }
        self.dev = Some(dev);
        if let Some(dev) = &mut self.dev {
            if dev.battery != 255 {
                dev.battery = dev.battery.clamp(0, 100);
            }
            let mut tis = TrayIconStatus::Active;
            self.tray_icon
                .set_tooltip(
                    format!(
                        "{}{}%",
                        if dev.charging {
                            "⚡"
                        } else {
                            if dev.battery <= 25 { "🪫" } else { "🔋" }
                        },
                        dev.battery
                    )
                    .as_str(),
                )
                .ok();
            if dev.battery <= 25 {
                tis = TrayIconStatus::NeedsAttention;
                self.tray_icon.set_icon(&self.bat_icons[0]).ok();
            } else if dev.battery <= 50 {
                self.tray_icon.set_icon(&self.bat_icons[1]).ok();
            } else if dev.battery <= 75 {
                self.tray_icon.set_icon(&self.bat_icons[2]).ok();
            } else {
                self.tray_icon.set_icon(&self.bat_icons[3]).ok();
            }
            self.tray_icon.set_status(tis).ok();

            self.tray_icon.set_menu(&self.gen_menu()).ok();
        }
    }

    #[cfg(target_os = "linux")]
    pub fn needs_udev_rules(&mut self, b: bool) {
        self.install_udev_rules = b;
        self.tray_icon.set_menu(&self.gen_menu()).ok();
    }

    #[cfg(target_os = "linux")]
    pub async fn notify_udev_rules(&self) -> Result<bool, Box<dyn error::Error + Send + Sync>> {
        let mut do_it = false;
        if let Ok(n) = Notification::new()
            .appname("Keychron")
            .action("udev", "Install udev rules")
            .hint(notify_rust::Hint::Resident(true))
            .summary("udev rules needed to access devices")
            .icon("input-mouse")
            .urgency(notify_rust::Urgency::Low)
            .timeout(Timeout::Never)
            .show_async()
            .await
        {
            n.wait_for_action(|action| match action {
                "udev" => {
                    do_it = true;
                }
                "__closed" => (),
                _ => (),
            });
        };
        if do_it {
            udev::udev_rule_install().await.map(|_| true)
        } else {
            Ok(false)
        }
    }
}
