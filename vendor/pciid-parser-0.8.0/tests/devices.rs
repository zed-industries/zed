use pciid_parser::Database;
use pretty_assertions::assert_eq;

#[test]
fn parse_polaris_local() {
    let db = Database::read().unwrap();
    parse_polaris(db);
}

#[cfg(feature = "online")]
#[test]
fn parse_polaris_online() {
    let db = Database::get_online().unwrap();
    parse_polaris(db);
}

fn parse_polaris(db: Database) {
    let data = db.get_device_info(0x1002, 0x67df, 0x1DA2, 0xE387);

    assert_eq!(
        data.vendor_name,
        Some("Advanced Micro Devices, Inc. [AMD/ATI]"),
    );
    assert_eq!(
        data.device_name,
        Some("Ellesmere [Radeon RX 470/480/570/570X/580/580X/590]"),
    );
    assert_eq!(data.subvendor_name, Some("Sapphire Technology Limited"));
    // Depending on the pci.ids version shipped this may be different
    let card_model = data.subdevice_name.unwrap();
    assert!(card_model == "Radeon RX 570 Pulse 4GB" || card_model == "Radeon RX 580 Pulse 4GB");
}

#[test]
fn parse_vega() {
    let db = Database::read().unwrap();
    let data = db.get_device_info(0x1002, 0x687F, 0x1043, 0x0555);

    assert_eq!(
        data.vendor_name,
        Some("Advanced Micro Devices, Inc. [AMD/ATI]")
    );
    assert_eq!(
        data.device_name,
        Some("Vega 10 XL/XT [Radeon RX Vega 56/64]")
    );
    assert_eq!(data.subvendor_name, Some("ASUSTeK Computer Inc."));
    assert_eq!(data.subdevice_name, None);
}

#[test]
fn class_not_in_vendors() {
    let db = Database::read().unwrap();

    assert_eq!(db.vendors.get(&0xc), None);
}

#[test]
fn find_amd() {
    let name = pciid_parser::find_vendor_name(0x1002).unwrap().unwrap();
    assert_eq!("Advanced Micro Devices, Inc. [AMD/ATI]", name);
}

#[test]
fn find_polaris() {
    let name = pciid_parser::find_device_name(0x1002, 0x67df)
        .unwrap()
        .unwrap();
    assert_eq!("Ellesmere [Radeon RX 470/480/570/570X/580/580X/590]", name);
}

#[test]
fn find_following_device() {
    let name = pciid_parser::find_device_name(0x1002, 0x1304)
        .unwrap()
        .unwrap();
    assert_eq!("Kaveri", name);
}

#[test]
fn find_between_vendors() {
    let name = pciid_parser::find_device_name(0x1001, 0x1306).unwrap();
    assert_eq!(None, name);
}

#[test]
fn find_between_vendors_2() {
    let name = pciid_parser::find_device_name(0x0001, 0x8139).unwrap();
    assert_eq!(None, name);
}

#[test]
fn find_subdevice() {
    let name = pciid_parser::find_subdevice_name(0x1002, 0x67df, 0x1da2, 0xe387)
        .unwrap()
        .unwrap();
    assert_eq!("Radeon RX 580 Pulse 4GB", name);
}
