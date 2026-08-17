extern crate pkg_config;

fn main() {
    if let Err(e) = pkg_config::probe_library("alsa") {
        match e {
            pkg_config::Error::Failure { .. } => panic! (
                "Pkg-config failed - usually this is because alsa development headers are not installed.\n\n\
                For Fedora users:\n# dnf install alsa-lib-devel\n\n\
                For Debian/Ubuntu users:\n# apt-get install libasound2-dev\n\n\
                pkg_config details:\n{}",
                e
            ),
            _ => panic!("{}", e)
        }
    }
}
