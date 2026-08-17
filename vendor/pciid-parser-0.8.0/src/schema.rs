#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, hash::Hash};

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DeviceInfo<'a> {
    pub vendor_name: Option<&'a str>,
    pub device_name: Option<&'a str>,
    pub subvendor_name: Option<&'a str>,
    pub subdevice_name: Option<&'a str>,
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vendor {
    pub name: String,
    pub devices: HashMap<u16, Device>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Device {
    pub name: String,
    pub subdevices: HashMap<SubDeviceId, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SubDeviceId {
    pub subvendor: u16,
    pub subdevice: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Class {
    pub name: String,
    pub subclasses: HashMap<u8, SubClass>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SubClass {
    pub name: String,
    pub prog_ifs: HashMap<u8, String>,
}
