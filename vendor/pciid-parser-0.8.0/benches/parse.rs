use pciid_parser::Database;
use std::io::Cursor;

const DB_DATA: &[u8] = include_bytes!("../tests/pci.ids");

fn main() {
    divan::main();
}

#[divan::bench]
fn parse_embedded() -> Database {
    let cursor = Cursor::new(DB_DATA);
    Database::parse_db(cursor).unwrap()
}

#[divan::bench]
fn find_polaris() -> Option<String> {
    let cursor = Cursor::new(DB_DATA);
    pciid_parser::find_device_name_with_reader(cursor, 0x1002, 0x67df).unwrap()
}

#[divan::bench]
fn find_end() -> Option<String> {
    let cursor = Cursor::new(DB_DATA);
    pciid_parser::find_device_name_with_reader(cursor, 0x1fc9, 0x3010).unwrap()
}

#[divan::bench]
fn find_end_in_file() -> Option<String> {
    pciid_parser::find_device_name(0x1fc9, 0x3010).unwrap()
}
