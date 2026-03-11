#![allow(
    unused_imports,
    non_camel_case_types,
    non_snake_case,
    dead_code,
    unused_mut,
    unused_variables
)]

pub mod client {
    use wayland_client;
    use wayland_client::protocol::*;

    pub mod __interfaces {
        use wayland_client::protocol::__interfaces::*;
        wayland_scanner::generate_interfaces!("src/linux/wayland/appmenu.xml");
    }
    use self::__interfaces::*;

    wayland_scanner::generate_client_code!("src/linux/wayland/appmenu.xml");
}
