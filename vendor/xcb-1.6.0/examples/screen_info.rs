fn main() -> xcb::Result<()> {
    let (conn, screen_num) = xcb::Connection::connect(None)?;

    let setup = conn.get_setup();

    let screen = setup.roots().nth(screen_num as usize).unwrap();

    println!("");
    println!("Informations of screen {}:", screen_num);
    println!("  width..........: {}", screen.width_in_pixels());
    println!("  height.........: {}", screen.height_in_pixels());
    println!("  white pixel....: {:x}", screen.white_pixel());
    println!("  black pixel....: {:x}", screen.black_pixel());

    Ok(())
}
