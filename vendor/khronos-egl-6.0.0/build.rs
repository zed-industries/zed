#[cfg(all(feature = "static", not(feature = "no-pkg-config")))]
extern crate pkg_config;

fn main() {
	#[cfg(all(feature = "static", not(feature = "no-pkg-config")))]
	{
		pkg_config::Config::new()
			.atleast_version("1")
			.probe("egl")
			.unwrap();
	}
}
