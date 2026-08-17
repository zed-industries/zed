extern crate xcb;

use std::{fs::File, io::BufWriter};
use xcb::x;

fn main() {
    let (conn, screen_num) = xcb::Connection::connect(None).unwrap();
    let setup = conn.get_setup();
    let screen = setup.roots().nth(screen_num as usize).unwrap();

    let width = screen.width_in_pixels();
    let height = screen.height_in_pixels();

    let cookie = conn.send_request(&x::GetImage {
        format: x::ImageFormat::ZPixmap,
        drawable: x::Drawable::Window(screen.root()),
        x: 0,
        y: 0,
        width,
        height,
        plane_mask: u32::MAX,
    });

    let file = File::create("screenshot.png").unwrap();
    let writer = BufWriter::new(file);
    let mut encoder = png::Encoder::new(writer, width as _, height as _);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder
        .write_header()
        .expect("Failed to write image header");

    let reply = conn.wait_for_reply(cookie).unwrap();

    let src = reply.data();
    let mut data = vec![0; width as usize * height as usize * 3];
    for (src, dest) in src.chunks(4).zip(data.chunks_mut(3)) {
        // Captured image stores orders pixels as BGR, so we need to
        // reorder them.
        dest[0] = src[2];
        dest[1] = src[1];
        dest[2] = src[0];
    }
    writer
        .write_image_data(&data)
        .expect("Failed to write image data");
}
