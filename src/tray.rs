use std::process::exit;

use trayicon::{Icon, MenuBuilder, TrayIcon, TrayIconBuilder, TrayIconStatus};

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
    Configure,
    Close,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Device {
    pub name: String,
    pub version: String,
    pub battery: u8,
    pub dpi: u16,
    pub polling_rate_level: u8,
}

pub struct Tray {
    tray_icon: TrayIcon<TrayEvent>,
    icon: Icon,
    bat_icons: [Icon; 4],
    dev: Option<Device>,
}

impl Tray {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let icon_normal = Icon::from_buffer(ICON_NORMAL_BYTES, None, None).unwrap();
        let icon_bat_full = Icon::from_buffer(ICON_BAT_FULL_BYTES, None, None).unwrap();
        let icon_bat_good = Icon::from_buffer(ICON_BAT_GOOD_BYTES, None, None).unwrap();
        let icon_bat_half = Icon::from_buffer(ICON_BAT_HALF_BYTES, None, None).unwrap();
        let icon_bat_low = Icon::from_buffer(ICON_BAT_LOW_BYTES, None, None).unwrap();
        let mut tray_icon = TrayIconBuilder::new()
            .icon(icon_normal.clone())
            .tooltip("Keychron")
            .sender(move |evt: &TrayEvent| match evt {
                TrayEvent::Configure => {
                    webbrowser::open(KEYCHRON_URL).ok();
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
        tray_icon.set_status(trayicon::TrayIconStatus::Passive).ok();
        Ok(Tray {
            tray_icon: tray_icon,
            icon: icon_normal,
            bat_icons: [icon_bat_low, icon_bat_half, icon_bat_good, icon_bat_full],
            dev: None,
        })
    }

    fn gen_menu(&self) -> MenuBuilder<TrayEvent> {
        let mut mb = MenuBuilder::new();
        if let Some(dev) = &self.dev {
            mb = mb
                .item(format!("⌄🖱️{}⌄", dev.name).as_str(), TrayEvent::None)
                .separator()
                .item(
                    format!(
                        "{} {}%",
                        if dev.battery <= 25 { "🪫" } else { "🔋" },
                        dev.battery
                    )
                    .as_str(),
                    TrayEvent::None,
                )
                .item(format!("📏Dpi: {}", dev.dpi).as_str(), TrayEvent::None)
                .item(
                    format!("⏱Poll: {}", dev.polling_rate_level).as_str(),
                    TrayEvent::None,
                )
                .item(format!("🛈 {}", dev.version).as_str(), TrayEvent::None)
                .separator();
        }
        mb.item("Configure", TrayEvent::Configure)
            .separator()
            .item("✖ Close", TrayEvent::Close)
    }

    pub fn update_device(&mut self, dev: Device) {
        self.dev = Some(dev);
        if let Some(dev) = &mut self.dev {
            dev.battery = dev.battery.clamp(0, 100);
            let mut tis = TrayIconStatus::Passive;
            self.tray_icon
                .set_tooltip(
                    format!(
                        "{}{}%",
                        if dev.battery <= 25 { "🪫" } else { "🔋" },
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
}
