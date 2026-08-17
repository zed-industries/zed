#![warn(clippy::pedantic)]
#![doc = include_str!("../README.md")]
mod error;
mod parser;
pub mod schema;

use crate::parser::Parser;
use error::Error;
use parser::Event;
use schema::{Class, Device, DeviceInfo, SubClass, SubDeviceId, Vendor};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
    path::{Path, PathBuf},
};

const DB_PATHS: &[&str] = &[
    "/usr/share/hwdata/pci.ids",
    "/usr/share/misc/pci.ids",
    "@hwdata@/share/hwdata/pci.ids",
];
#[cfg(feature = "online")]
const URL: &str = "https://pci-ids.ucw.cz/v2.2/pci.ids";

#[derive(Debug)]
pub enum VendorDataError {
    MissingIdsFile,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Database {
    pub vendors: HashMap<u16, Vendor>,
    pub classes: HashMap<u8, Class>,
}

impl Database {
    /// Attempt to read the database from a list of known file paths
    ///
    /// # Errors
    /// Returns an error when either no file could be found or the parsing fails.
    pub fn read() -> Result<Self, Error> {
        let file = Self::open_file()?;
        Self::parse_db(file)
    }

    /// Read the database from a given path
    ///
    /// # Errors
    /// Returns an error when the file can't be read or when parsing fails
    pub fn read_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let file = File::open(path)?;
        Self::parse_db(file)
    }

    /// Fetch a database from an online source
    ///
    /// # Errors
    /// Returns an error when the database either can't be fetched or parsed
    #[cfg(feature = "online")]
    pub fn get_online() -> Result<Self, Error> {
        let response = ureq::get(URL).call()?;

        Self::parse_db(response.into_body().into_reader())
    }

    /// Parse a database from the given reader
    ///
    /// # Errors
    /// Returns an error whenever there's a parsing error
    #[allow(clippy::too_many_lines)] // todo
    pub fn parse_db<R: Read>(reader: R) -> Result<Self, Error> {
        let reader = BufReader::new(reader);
        let mut parser = Parser::new(reader);

        let mut current_vendor: Option<(u16, Vendor)> = None;
        let mut current_device: Option<(u16, Device)> = None;

        let mut current_class: Option<(u8, Class)> = None;
        let mut current_subclass: Option<(u8, SubClass)> = None;

        let mut vendors: HashMap<u16, Vendor> = HashMap::with_capacity(2500);
        let mut classes: HashMap<u8, Class> = HashMap::with_capacity(200);

        while let Some(event) = parser.next_event()? {
            match event {
                Event::Vendor { id, name } => {
                    // The vendor section is complete so it needs to be pushed to the main list
                    if let Some((device_id, device)) = current_device.take() {
                        let (_, vendor) = current_vendor
                            .as_mut()
                            .ok_or_else(Error::no_current_vendor)?;
                        vendor.devices.insert(device_id, device);
                    }
                    if let Some((vendor_id, vendor)) = current_vendor.take() {
                        vendors.insert(vendor_id, vendor);
                    }

                    let vendor = Vendor {
                        name: name.to_owned(),
                        devices: HashMap::new(),
                    };
                    current_vendor = Some((
                        u16::from_str_radix(id, 16).map_err(|_| Error::invalid_int(id))?,
                        vendor,
                    ));
                }
                Event::Device { id, name } => {
                    // Device section is over, write to vendor
                    if let Some((device_id, device)) = current_device.take() {
                        let (_, current_vendor) = current_vendor
                            .as_mut()
                            .ok_or_else(Error::no_current_vendor)?;

                        current_vendor.devices.insert(device_id, device);
                    }

                    let device = Device {
                        name: name.to_owned(),
                        subdevices: HashMap::new(),
                    };

                    current_device = Some((
                        u16::from_str_radix(id, 16).map_err(|_| Error::invalid_int(id))?,
                        device,
                    ));
                }
                Event::Subdevice {
                    subvendor,
                    subdevice,
                    subsystem_name,
                } => {
                    let (_, current_device) = current_device
                        .as_mut()
                        .ok_or_else(Error::no_current_device)?;

                    let subdevice_id = SubDeviceId {
                        subvendor: u16::from_str_radix(subvendor, 16)
                            .map_err(|_| Error::invalid_int(subvendor))?,
                        subdevice: u16::from_str_radix(subdevice, 16)
                            .map_err(|_| Error::invalid_int(subdevice))?,
                    };
                    current_device
                        .subdevices
                        .insert(subdevice_id, subsystem_name.to_owned());
                }
                Event::Class { id, name } => {
                    if let Some((subclass_id, subclass)) = current_subclass.take() {
                        let (_, class) =
                            current_class.as_mut().ok_or_else(Error::no_current_class)?;

                        class.subclasses.insert(subclass_id, subclass);
                    }
                    if let Some((class_id, class)) = current_class.take() {
                        classes.insert(class_id, class);
                    }

                    let class = Class {
                        name: name.to_owned(),
                        subclasses: HashMap::new(),
                    };
                    current_class = Some((
                        u8::from_str_radix(id, 16).map_err(|_| Error::invalid_int(id))?,
                        class,
                    ));
                }
                Event::SubClass { id, name } => {
                    if let Some((subclass_id, subclass)) = current_subclass {
                        let (_, class) =
                            current_class.as_mut().ok_or_else(Error::no_current_class)?;

                        class.subclasses.insert(subclass_id, subclass);
                    }

                    let subclass = SubClass {
                        name: name.to_owned(),
                        prog_ifs: HashMap::new(),
                    };
                    current_subclass = Some((
                        u8::from_str_radix(id, 16).map_err(|_| Error::invalid_int(id))?,
                        subclass,
                    ));
                }
                Event::ProgIf { id, name } => {
                    let (_, subclass) = current_subclass
                        .as_mut()
                        .ok_or_else(Error::no_current_subclass)?;

                    subclass.prog_ifs.insert(
                        u8::from_str_radix(id, 16).map_err(|_| Error::invalid_int(id))?,
                        name.to_owned(),
                    );
                }
            }
        }
        // Finish writing the last vendor and class
        if let Some((device_id, device)) = current_device.take() {
            let (_, vendor) = current_vendor
                .as_mut()
                .ok_or_else(Error::no_current_vendor)?;
            vendor.devices.insert(device_id, device);
        }
        if let Some((vendor_id, vendor)) = current_vendor.take() {
            vendors.insert(vendor_id, vendor);
        }

        if let Some((subclass_id, subclass)) = current_subclass.take() {
            let (_, class) = current_class.as_mut().ok_or_else(Error::no_current_class)?;

            class.subclasses.insert(subclass_id, subclass);
        }
        if let Some((class_id, class)) = current_class.take() {
            classes.insert(class_id, class);
        }

        vendors.shrink_to_fit();
        classes.shrink_to_fit();

        Ok(Self { vendors, classes })
    }

    fn open_file() -> Result<File, Error> {
        if let Some(path) = DB_PATHS
            .iter()
            .find(|path| Path::exists(&PathBuf::from(path)))
        {
            Ok(File::open(path)?)
        } else {
            Err(Error::FileNotFound)
        }
    }

    #[must_use]
    pub fn get_device_info(
        &self,
        vendor_id: u16,
        model_id: u16,
        subsys_vendor_id: u16,
        subsys_model_id: u16,
    ) -> DeviceInfo<'_> {
        let mut vendor_name = None;
        let mut device_name = None;
        let mut subvendor_name = None;
        let mut subdevice_name = None;

        if let Some(vendor) = self.vendors.get(&vendor_id) {
            vendor_name = Some(vendor.name.as_str());

            if let Some(device) = vendor.devices.get(&model_id) {
                device_name = Some(device.name.as_str());

                if let Some(subvendor) = self.vendors.get(&subsys_vendor_id) {
                    subvendor_name = Some(subvendor.name.as_str());
                }

                let subdevice_id = SubDeviceId {
                    subvendor: subsys_vendor_id,
                    subdevice: subsys_model_id,
                };

                subdevice_name = device.subdevices.get(&subdevice_id).map(String::as_str);
            }
        }

        DeviceInfo {
            vendor_name,
            device_name,
            subvendor_name,
            subdevice_name,
        }
    }
}

/// Try to find the name of a vendor by its id.
/// This will search the database from one of the known file paths for the name.
///
/// # Errors
/// Returns an error when the file can't be read or when parsing fails
pub fn find_vendor_name(vendor_id: u16) -> Result<Option<String>, Error> {
    let reader = Database::open_file()?;
    find_vendor_name_with_reader(reader, vendor_id)
}

/// Try to find the name of a vendor by its id.
/// This will search the database from the given reader for the name.
///
/// # Errors
/// Returns an error when parsing fails
pub fn find_vendor_name_with_reader<R: Read>(
    reader: R,
    vendor_id: u16,
) -> Result<Option<String>, Error> {
    let vendor_id = format!("{vendor_id:x?}");

    let mut parser = Parser::new(BufReader::new(reader));

    while let Some(event) = parser.next_event()? {
        if let Event::Vendor { id, name } = event {
            if id == vendor_id {
                return Ok(Some(name.to_owned()));
            }
        }
    }

    Ok(None)
}

/// Try to find the name of a device by its vendor and device id.
/// This will search the database from one of the known file paths for the name.
///
/// # Errors
/// Returns an error when the file can't be read or when parsing fails
pub fn find_device_name(vendor_id: u16, device_id: u16) -> Result<Option<String>, Error> {
    let reader = Database::open_file()?;
    find_device_name_with_reader(reader, vendor_id, device_id)
}

/// Try to find the name of a device by its vendor and device id.
/// This will search the database from the given reader for the name.
///
/// # Errors
/// Returns an error when parsing fails
pub fn find_device_name_with_reader<R: Read>(
    reader: R,
    vendor_id: u16,
    device_id: u16,
) -> Result<Option<String>, Error> {
    let vendor_id = format!("{vendor_id:x?}");
    let device_id = format!("{device_id:x?}");

    let mut parser = Parser::new(BufReader::new(reader));

    while let Some(event) = parser.next_event()? {
        if let Event::Vendor { id, .. } = event {
            if id == vendor_id {
                while let Some(event) = parser.next_event()? {
                    match event {
                        Event::Device { id, name } => {
                            if id == device_id {
                                return Ok(Some(name.to_owned()));
                            }
                        }
                        Event::Vendor { .. } => break,
                        _ => (),
                    }
                }

                break;
            }
        }
    }

    Ok(None)
}

/// Try to find the name of a subdevice by its ids.
/// This will search the database from the given reader for the name.
///
/// # Errors
/// Returns an error when parsing fails
pub fn find_subdevice_name(
    parent_vendor_id: u16,
    parent_device_id: u16,
    subvendor_id: u16,
    subdevice_id: u16,
) -> Result<Option<String>, Error> {
    let reader = Database::open_file()?;
    find_subdevice_name_with_reader(
        reader,
        parent_vendor_id,
        parent_device_id,
        subvendor_id,
        subdevice_id,
    )
}

/// Try to find the name of a subdevice by its ids.
/// This will search the database from the given reader for the name.
///
/// # Errors
/// Returns an error when parsing fails
pub fn find_subdevice_name_with_reader<R: Read>(
    reader: R,
    parent_vendor_id: u16,
    parent_device_id: u16,
    subvendor_id: u16,
    subdevice_id: u16,
) -> Result<Option<String>, Error> {
    let parent_vendor_id = format!("{parent_vendor_id:x?}");
    let parent_device_id = format!("{parent_device_id:x?}");
    let subvendor_id = format!("{subvendor_id:x?}");
    let subdevice_id = format!("{subdevice_id:x?}");

    let mut parser = Parser::new(BufReader::new(reader));

    while let Some(event) = parser.next_event()? {
        if let Event::Vendor { id, .. } = event {
            if id == parent_vendor_id {
                while let Some(event) = parser.next_event()? {
                    match event {
                        Event::Device { id, .. } => {
                            if id == parent_device_id {
                                while let Some(event) = parser.next_event()? {
                                    match event {
                                        Event::Subdevice {
                                            subvendor,
                                            subdevice,
                                            subsystem_name,
                                        } => {
                                            if subvendor == subvendor_id
                                                && subdevice == subdevice_id
                                            {
                                                return Ok(Some(subsystem_name.to_owned()));
                                            }
                                        }
                                        _ => break,
                                    }
                                }

                                break;
                            }
                        }
                        Event::Vendor { .. } => break,
                        _ => (),
                    }
                }

                break;
            }
        }
    }

    Ok(None)
}
