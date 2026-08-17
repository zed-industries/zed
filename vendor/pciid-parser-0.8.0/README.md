# PCI ID Parser

[![Crates.io](https://img.shields.io/crates/v/pciid-parser)](https://crates.io/crates/pciid-parser)
[![Docs.rs](https://docs.rs/pciid-parser/badge.svg)](https://docs.rs/pciid-parser/)

This is a library that lets you use a PCI ID database, such as one shipped with Linux distros or from <https://pci-ids.ucw.cz/>.
It can either read the locally installed file or fetch one from the website.

## Usage

Read the local DB:
```rust
use pciid_parser::Database;

let db = Database::read().unwrap();

// Get vendor
let vendor = db.vendors.get(&0x1002).unwrap();
assert_eq!(vendor.name, "Advanced Micro Devices, Inc. [AMD/ATI]");
// Get device
let device = vendor.devices.get(&0x67df).unwrap();
assert_eq!(
  device.name,
  "Ellesmere [Radeon RX 470/480/570/570X/580/580X/590]"
);  

// Get full device and subdevice info:
let info = db.get_device_info(0x1002, 0x67df, 0x1DA2, 0xE387);

// Get class
let class = db.classes.get(&0x05).unwrap();
assert_eq!(class.name, "Memory controller");
```
You can also fetch the online DB:

```rust, ignore
use pciid_parser::Database;

let db = Database::get_online().unwrap();
```

## Breaking changes

- 0.8.0: version vendor and device ids are now stored as integers instead of strings
