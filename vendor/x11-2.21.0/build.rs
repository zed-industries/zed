// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

extern crate pkg_config;

use std::env;

fn main() {
    if cfg!(feature = "dox") {
        return;
    }

    let deps = [
        ("gl", "1", "glx"),
        ("x11", "1.4.99.1", "xlib"),
        ("x11-xcb", "1.6", "xlib_xcb"),
        ("xcursor", "1.1", "xcursor"),
        ("xext", "1.3", "dpms"),
        ("xfixes", "3.1", "xfixes"),
        ("xft", "2.1", "xft"),
        ("xi", "1.7", "xinput"),
        ("xinerama", "1.1", "xinerama"),
        ("xmu", "1.1", "xmu"),
        ("xrandr", "1.5", "xrandr"),
        ("xrender", "0.9.6", "xrender"),
        ("xpresent", "1", "xpresent"),
        ("xscrnsaver", "1.2", "xss"),
        ("xt", "1.1", "xt"),
        ("xtst", "1.2", "xtst"),
        ("xxf86vm", "1.1", "xf86vmode"),
    ];

    for &(dep, version, feature) in deps.iter() {
        let var = format!("CARGO_FEATURE_{}", feature.to_uppercase().replace('-', "_"));
        if env::var_os(var).is_none() {
            continue;
        }
        pkg_config::Config::new()
            .atleast_version(version)
            .probe(dep)
            .unwrap();
    }
}
