//! Emits default minidump with no streams to specified path

use {
    minidump_writer::{
        dir_section::DirSection,
        mem_writer::{Buffer, MemoryWriter},
        minidump_format::{MDRawHeader, MD_HEADER_SIGNATURE, MD_HEADER_VERSION},
    },
    std::fs::File,
};

// usage: `cargo run --example synthetic /tmp/micro-minidump.dmp`
fn main() {
    let output_path = std::env::args()
        .nth(1)
        .expect("missing argument: output file path");

    let num_writers = 0u32;
    let buffer_capacity = 32;

    let mut destination = File::create(output_path).expect("failed to create file");
    let mut buffer = Buffer::with_capacity(buffer_capacity);
    let mut header_section = MemoryWriter::<MDRawHeader>::alloc(&mut buffer).unwrap();
    let mut dir_section = DirSection::new(&mut buffer, num_writers, &mut destination).unwrap();

    let header = MDRawHeader {
        signature: MD_HEADER_SIGNATURE,
        version: MD_HEADER_VERSION,
        stream_count: num_writers,
        stream_directory_rva: dir_section.position(),
        checksum: 0,
        time_date_stamp: 0u32,
        flags: 0,
    };
    header_section.set_value(&mut buffer, header).unwrap();
    dir_section.write_to_file(&mut buffer, None).unwrap();
}
