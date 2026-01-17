use std::error;
use std::fmt;
use std::str;

use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum KeychronDeviceCategory {
    Receiver = 24,
    Mouse = 25,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub struct ParseKeychronDeviceCategoryError;

impl fmt::Display for ParseKeychronDeviceCategoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Invalid Keychron device category given")
    }
}

impl error::Error for ParseKeychronDeviceCategoryError {
    fn description(&self) -> &str {
        "invalid Keychron device category"
    }
}

impl fmt::Display for KeychronDeviceCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            KeychronDeviceCategory::Mouse => f.write_str("Mouse"),
            KeychronDeviceCategory::Receiver => f.write_str("Receiver"),
        }
    }
}

impl str::FromStr for KeychronDeviceCategory {
    type Err = ParseKeychronDeviceCategoryError;

    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        match s.replace(" ", "").to_uppercase().as_str() {
            "MOUSE" => Ok(KeychronDeviceCategory::Mouse),
            "RECEIVER" => Ok(KeychronDeviceCategory::Receiver),
            _ => Err(ParseKeychronDeviceCategoryError),
        }
    }
}

impl KeychronDeviceCategory {
    pub fn description(&self) -> &'static str {
        match *self {
            KeychronDeviceCategory::Mouse => "",
            KeychronDeviceCategory::Receiver => "Keychron Link",
        }
    }

    pub fn ttype(&self) -> u8 {
        match *self {
            KeychronDeviceCategory::Mouse => 1,
            KeychronDeviceCategory::Receiver => 2,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub enum KeychronDevice {
    UltraLink8K = 0xd028,
    M3Mini4K = 0xd037,       // productID: 0x0620
    M3_4K = 0xd03c,          // productID: 0x07a0
    M4_4K = 0xd040,          // productID: 0x0621
    M3Mini4KAluAll = 0xd041, // productID: 0x0622
    M3M24K = 0xd045,         // productID: 0x0623
    M6_4K = 0xd046,          // productID: 0x0624
    M6_8K = 0xd049,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub struct ParseKeychronDeviceError;

impl fmt::Display for ParseKeychronDeviceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Invalid Keychron device given")
    }
}

impl error::Error for ParseKeychronDeviceError {
    fn description(&self) -> &str {
        "invalid Keychron device"
    }
}

impl fmt::Display for KeychronDevice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            KeychronDevice::UltraLink8K => f.write_str("Ultra-Link 8K"),
            KeychronDevice::M3Mini4K => f.write_str("M3 Mini 4K"),
            KeychronDevice::M3_4K => f.write_str("M3 4K"),
            KeychronDevice::M4_4K => f.write_str("M4 4K"),
            KeychronDevice::M3Mini4KAluAll => f.write_str("M3 Mini 4K 铝合金"),
            KeychronDevice::M3M24K => f.write_str("M3 M2 4K"),
            KeychronDevice::M6_4K => f.write_str("M6 4K"),
            KeychronDevice::M6_8K => f.write_str("M6 8K"),
        }
    }
}

impl str::FromStr for KeychronDevice {
    type Err = ParseKeychronDeviceError;

    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        match s.replace(" ", "").to_uppercase().as_str() {
            "ULTRA-LINK8K" => Ok(KeychronDevice::UltraLink8K),
            "M3MINI4K" => Ok(KeychronDevice::M3Mini4K),
            "M34K" => Ok(KeychronDevice::M3_4K),
            "M44K" => Ok(KeychronDevice::M4_4K),
            "M3MINI4K铝合金" => Ok(KeychronDevice::M3Mini4KAluAll),
            "M3M24K" => Ok(KeychronDevice::M3M24K),
            "M64K" => Ok(KeychronDevice::M6_4K),
            "M68K" => Ok(KeychronDevice::M6_8K),
            _ => Err(ParseKeychronDeviceError),
        }
    }
}

impl KeychronDevice {
    pub fn device_type(&self) -> KeychronDeviceCategory {
        match *self {
            KeychronDevice::UltraLink8K => KeychronDeviceCategory::Receiver,
            KeychronDevice::M3Mini4K
            | KeychronDevice::M3_4K
            | KeychronDevice::M4_4K
            | KeychronDevice::M3Mini4KAluAll
            | KeychronDevice::M3M24K
            | KeychronDevice::M6_4K
            | KeychronDevice::M6_8K => KeychronDeviceCategory::Mouse,
        }
    }
}
