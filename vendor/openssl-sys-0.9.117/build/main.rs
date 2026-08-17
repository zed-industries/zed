#[cfg(feature = "bindgen")]
extern crate bindgen;
extern crate cc;
#[cfg(feature = "vendored")]
extern crate openssl_src;
extern crate pkg_config;
extern crate vcpkg;

use std::collections::HashSet;
use std::env;
use std::ffi::OsString;
use std::path::PathBuf;
mod cfgs;

mod find_normal;
#[cfg(feature = "vendored")]
mod find_vendored;
mod run_bindgen;

#[derive(PartialEq)]
enum Version {
    Openssl4xx,
    Openssl3xx,
    Openssl11x,
    Libressl,
    Boringssl,
    AwsLc,
}

fn env_inner(name: &str) -> Option<OsString> {
    let var = env::var_os(name);
    println!("cargo:rerun-if-env-changed={name}");

    match var {
        Some(ref v) => println!("{} = {}", name, v.to_string_lossy()),
        None => println!("{name} unset"),
    }

    var
}

fn env(name: &str) -> Option<OsString> {
    let prefix = env::var("TARGET").unwrap().to_uppercase().replace('-', "_");
    let prefixed = format!("{prefix}_{name}");
    env_inner(&prefixed).or_else(|| env_inner(name))
}

fn find_openssl(target: &str) -> (Vec<PathBuf>, PathBuf) {
    #[cfg(feature = "vendored")]
    {
        // vendor if the feature is present, unless
        // OPENSSL_NO_VENDOR exists and isn't `0`
        if env("OPENSSL_NO_VENDOR").map_or(true, |s| s == "0") {
            return find_vendored::get_openssl(target);
        }
    }
    find_normal::get_openssl(target)
}

fn check_ssl_kind() {
    if cfg!(feature = "unstable_boringssl") {
        println!("cargo:rustc-cfg=boringssl");
        println!("cargo:boringssl=true");

        if let Ok(vars) = env::var("DEP_BSSL_CONF") {
            for var in vars.split(',') {
                println!("cargo:rustc-cfg=osslconf=\"{var}\"");
            }
            println!("cargo:conf={vars}");
        }

        // BoringSSL does not have any build logic, exit early
        std::process::exit(0);
    }

    let is_aws_lc = cfg!(feature = "aws-lc");
    let is_aws_lc_fips = cfg!(feature = "aws-lc-fips");

    if is_aws_lc || is_aws_lc_fips {
        // The aws-lc-sys crate uses a link name that embeds
        // the version number of crate. Examples (crate-name => links name):
        //   * aws-lc-sys => aws_lc_0_26_0
        // This is done to avoid issues if the cargo dependency graph for an application
        // were to resolve to multiple versions for the same crate.
        //
        // Due to this we need to determine what version of the AWS-LC has been selected (fips or non-fips)
        // and then need to parse out the pieces we are interested in ignoring the version component of the name.
        let aws_lc_env_var_prefix: &'static str = if is_aws_lc_fips {
            "DEP_AWS_LC_FIPS_"
        } else {
            "DEP_AWS_LC_"
        };

        println!("cargo:rustc-cfg=awslc");
        println!("cargo:awslc=true");
        if is_aws_lc_fips {
            println!("cargo:rustc-cfg=awslc_fips");
            println!("cargo:awslc_fips=true");
        }

        let mut version = None;
        for (name, _) in std::env::vars() {
            if let Some(name) = name.strip_prefix(aws_lc_env_var_prefix) {
                if let Some(name) = name.strip_suffix("_INCLUDE") {
                    version = Some(name.to_owned());
                    break;
                }
            }
        }
        let version = version.expect("aws-lc version detected");

        // Read the OpenSSL configuration statements and emit rust-cfg for each.
        if let Ok(vars) = std::env::var(format!("{aws_lc_env_var_prefix}{version}_CONF")) {
            for var in vars.split(',') {
                println!("cargo:rustc-cfg=osslconf=\"{var}\"");
            }
            println!("cargo:conf={vars}");
        }

        // Emit the include header directory from the aws-lc(-fips)-sys crate so that it can be used if needed
        // by crates consuming openssl-sys.
        if let Ok(val) = std::env::var(format!("{aws_lc_env_var_prefix}{version}_INCLUDE")) {
            println!("cargo:include={val}");
        }

        // AWS-LC does not have any build logic, exit early
        std::process::exit(0);
    }
}

fn main() {
    println!("cargo:rustc-check-cfg=cfg(osslconf, values(\"OPENSSL_NO_OCB\", \"OPENSSL_NO_SM4\", \"OPENSSL_NO_SEED\", \"OPENSSL_NO_CHACHA\", \"OPENSSL_NO_CAST\", \"OPENSSL_NO_IDEA\", \"OPENSSL_NO_CAMELLIA\", \"OPENSSL_NO_RC4\", \"OPENSSL_NO_BF\", \"OPENSSL_NO_PSK\", \"OPENSSL_NO_DEPRECATED_3_0\", \"OPENSSL_NO_SCRYPT\", \"OPENSSL_NO_SM3\", \"OPENSSL_NO_RMD160\", \"OPENSSL_NO_EC2M\", \"OPENSSL_NO_OCSP\", \"OPENSSL_NO_CMS\", \"OPENSSL_NO_COMP\", \"OPENSSL_NO_SOCK\", \"OPENSSL_NO_STDIO\", \"OPENSSL_NO_EC\", \"OPENSSL_NO_SSL3_METHOD\", \"OPENSSL_NO_KRB5\", \"OPENSSL_NO_TLSEXT\", \"OPENSSL_NO_SRP\", \"OPENSSL_NO_SRTP\", \"OPENSSL_NO_RFC3779\", \"OPENSSL_NO_SHA\", \"OPENSSL_NO_NEXTPROTONEG\", \"OPENSSL_NO_ENGINE\", \"OPENSSL_NO_BUF_FREELISTS\", \"OPENSSL_NO_RC2\"))");

    println!("cargo:rustc-check-cfg=cfg(openssl)");
    println!("cargo:rustc-check-cfg=cfg(libressl)");
    println!("cargo:rustc-check-cfg=cfg(boringssl)");
    println!("cargo:rustc-check-cfg=cfg(awslc)");
    println!("cargo:rustc-check-cfg=cfg(awslc_pregenerated)");

    println!("cargo:rustc-check-cfg=cfg(libressl250)");
    println!("cargo:rustc-check-cfg=cfg(libressl251)");
    println!("cargo:rustc-check-cfg=cfg(libressl252)");
    println!("cargo:rustc-check-cfg=cfg(libressl261)");
    println!("cargo:rustc-check-cfg=cfg(libressl270)");
    println!("cargo:rustc-check-cfg=cfg(libressl271)");
    println!("cargo:rustc-check-cfg=cfg(libressl273)");
    println!("cargo:rustc-check-cfg=cfg(libressl280)");
    println!("cargo:rustc-check-cfg=cfg(libressl281)");
    println!("cargo:rustc-check-cfg=cfg(libressl291)");
    println!("cargo:rustc-check-cfg=cfg(libressl310)");
    println!("cargo:rustc-check-cfg=cfg(libressl321)");
    println!("cargo:rustc-check-cfg=cfg(libressl332)");
    println!("cargo:rustc-check-cfg=cfg(libressl340)");
    println!("cargo:rustc-check-cfg=cfg(libressl350)");
    println!("cargo:rustc-check-cfg=cfg(libressl360)");
    println!("cargo:rustc-check-cfg=cfg(libressl361)");
    println!("cargo:rustc-check-cfg=cfg(libressl370)");
    println!("cargo:rustc-check-cfg=cfg(libressl380)");
    println!("cargo:rustc-check-cfg=cfg(libressl381)");
    println!("cargo:rustc-check-cfg=cfg(libressl382)");
    println!("cargo:rustc-check-cfg=cfg(libressl390)");
    println!("cargo:rustc-check-cfg=cfg(libressl400)");
    println!("cargo:rustc-check-cfg=cfg(libressl410)");
    println!("cargo:rustc-check-cfg=cfg(libressl420)");
    println!("cargo:rustc-check-cfg=cfg(libressl430)");

    println!("cargo:rustc-check-cfg=cfg(ossl101)");
    println!("cargo:rustc-check-cfg=cfg(ossl102)");
    println!("cargo:rustc-check-cfg=cfg(ossl102f)");
    println!("cargo:rustc-check-cfg=cfg(ossl102h)");
    println!("cargo:rustc-check-cfg=cfg(ossl110)");
    println!("cargo:rustc-check-cfg=cfg(ossl110f)");
    println!("cargo:rustc-check-cfg=cfg(ossl110g)");
    println!("cargo:rustc-check-cfg=cfg(ossl110h)");
    println!("cargo:rustc-check-cfg=cfg(ossl111)");
    println!("cargo:rustc-check-cfg=cfg(ossl111b)");
    println!("cargo:rustc-check-cfg=cfg(ossl111c)");
    println!("cargo:rustc-check-cfg=cfg(ossl111d)");
    println!("cargo:rustc-check-cfg=cfg(ossl300)");
    println!("cargo:rustc-check-cfg=cfg(ossl310)");
    println!("cargo:rustc-check-cfg=cfg(ossl320)");
    println!("cargo:rustc-check-cfg=cfg(ossl330)");
    println!("cargo:rustc-check-cfg=cfg(ossl340)");
    println!("cargo:rustc-check-cfg=cfg(ossl400)");

    check_ssl_kind();

    let target = env::var("TARGET").unwrap();

    let (lib_dirs, include_dir) = find_openssl(&target);
    // rerun-if-changed causes openssl-sys to rebuild if the openssl include
    // dir has changed since the last build. However, this causes a rebuild
    // every time when vendoring so we disable it.
    let potential_path = include_dir.join("openssl");
    if potential_path.exists() && !cfg!(feature = "vendored") {
        if let Some(printable_include) = potential_path.to_str() {
            println!("cargo:rerun-if-changed={printable_include}");
        }
    }

    if !lib_dirs.iter().all(|p| p.exists()) {
        panic!("OpenSSL library directory does not exist: {lib_dirs:?}");
    }
    if !include_dir.exists() {
        panic!(
            "OpenSSL include directory does not exist: {}",
            include_dir.to_string_lossy()
        );
    }

    for lib_dir in lib_dirs.iter() {
        println!(
            "cargo:rustc-link-search=native={}",
            lib_dir.to_string_lossy()
        );
    }
    println!("cargo:include={}", include_dir.to_string_lossy());

    let version = postprocess(&[include_dir]);

    let libs_env = env("OPENSSL_LIBS");
    let libs = match libs_env.as_ref().and_then(|s| s.to_str()) {
        Some(v) => {
            if v.is_empty() {
                vec![]
            } else {
                v.split(':').collect()
            }
        }
        None => match version {
            Version::Openssl4xx | Version::Openssl3xx | Version::Openssl11x
                if target.contains("windows-msvc") =>
            {
                vec!["libssl", "libcrypto"]
            }
            _ => vec!["ssl", "crypto"],
        },
    };

    let kind = determine_mode(&lib_dirs, &libs);
    for lib in libs.into_iter() {
        println!("cargo:rustc-link-lib={kind}={lib}");
    }

    // libssl in BoringSSL requires the C++ runtime, and static libraries do
    // not carry dependency information. On unix-like platforms, the C++
    // runtime and standard library are typically picked up by default via the
    // C++ compiler, which has a platform-specific default. (See implementations
    // of `GetDefaultCXXStdlibType` in Clang.) Builds may also choose to
    // override this and specify their own with `-nostdinc++` and `-nostdlib++`
    // flags. Some compilers also provide options like `-stdlib=libc++`.
    //
    // Typically, such information is carried all the way up the build graph,
    // but Cargo is not an integrated cross-language build system, so it cannot
    // safely handle any of these situations. As a result, we need to make
    // guesses. Getting this wrong may result in symbol conflicts and memory
    // errors, but this unsafety is inherent to driving builds with
    // externally-built libraries using Cargo.
    //
    // For now, we guess that the build was made with the defaults. This too is
    // difficult because Rust does not expose this information from Clang, but
    // try to match the behavior for common platforms. For a more robust option,
    // this likely needs to be deferred to the caller with an environment
    // variable.
    if (version == Version::Boringssl || version == Version::AwsLc)
        && kind == "static"
        && env::var("CARGO_CFG_UNIX").is_ok()
    {
        let cpp_lib = match env::var("CARGO_CFG_TARGET_OS").unwrap().as_ref() {
            "macos" => "c++",
            _ => "stdc++",
        };
        println!("cargo:rustc-link-lib={cpp_lib}");
    }

    // https://github.com/openssl/openssl/pull/15086
    if (version == Version::Openssl3xx || version == Version::Openssl4xx)
        && kind == "static"
        && (env::var("CARGO_CFG_TARGET_OS").unwrap() == "linux"
            || env::var("CARGO_CFG_TARGET_OS").unwrap() == "android")
        && env::var("CARGO_CFG_TARGET_POINTER_WIDTH").unwrap() == "32"
    {
        println!("cargo:rustc-link-lib=atomic");
    }

    if kind == "static" && target.contains("windows") {
        println!("cargo:rustc-link-lib=dylib=gdi32");
        println!("cargo:rustc-link-lib=dylib=user32");
        println!("cargo:rustc-link-lib=dylib=crypt32");
        println!("cargo:rustc-link-lib=dylib=ws2_32");
        println!("cargo:rustc-link-lib=dylib=advapi32");
    }
}

fn postprocess(include_dirs: &[PathBuf]) -> Version {
    let version = validate_headers(include_dirs);

    // Never run bindgen for BoringSSL or AWS-LC, if it was needed we already ran it.
    if !(version == Version::Boringssl || version == Version::AwsLc) {
        #[cfg(feature = "bindgen")]
        run_bindgen::run(&include_dirs);
    }

    version
}

/// Validates the header files found in `include_dir` and then returns the
/// version string of OpenSSL.
#[allow(clippy::unusual_byte_groupings)]
fn validate_headers(include_dirs: &[PathBuf]) -> Version {
    // This `*-sys` crate only works with OpenSSL 1.1.0, 1.1.1, 3.x, and 4.x.
    // To correctly expose the right API from this crate, take a look at
    // `opensslv.h` to see what version OpenSSL claims to be.
    println!("cargo:rerun-if-changed=build/expando.c");
    let mut gcc = cc::Build::new();
    gcc.includes(include_dirs);
    let expanded = match gcc.file("build/expando.c").try_expand() {
        Ok(expanded) => expanded,
        Err(e) => {
            panic!(
                "
Header expansion error:
{e:?}

Failed to find OpenSSL development headers.

You can try fixing this setting the `OPENSSL_DIR` environment variable
pointing to your OpenSSL installation or installing OpenSSL headers package
specific to your distribution:

    # On Ubuntu
    sudo apt-get install pkg-config libssl-dev
    # On Arch Linux
    sudo pacman -S pkgconf openssl
    # On Fedora
    sudo dnf install pkgconf perl-FindBin perl-IPC-Cmd openssl-devel
    # On Alpine Linux
    apk add pkgconf openssl-dev

See rust-openssl documentation for more information:

    https://docs.rs/openssl
"
            );
        }
    };
    let expanded = String::from_utf8(expanded).unwrap();

    let mut enabled = vec![];
    let mut openssl_version = None;
    let mut libressl_version = None;
    let mut is_boringssl = false;
    let mut is_awslc = false;
    let mut bindgen_symbol_prefix: Option<String> = None;
    for line in expanded.lines() {
        let line = line.trim();

        let openssl_prefix = "RUST_VERSION_OPENSSL_";
        let new_openssl_prefix = "RUST_VERSION_NEW_OPENSSL_";
        let libressl_prefix = "RUST_VERSION_LIBRESSL_";
        let boringssl_prefix = "RUST_OPENSSL_IS_BORINGSSL";
        let awslc_prefix = "RUST_OPENSSL_IS_AWSLC";
        let conf_prefix = "RUST_CONF_";
        let symbol_prefix = "RUST_BINDGEN_SYMBOL_PREFIX_";
        if let Some(version) = line.strip_prefix(openssl_prefix) {
            openssl_version = Some(parse_version(version));
        } else if let Some(version) = line.strip_prefix(new_openssl_prefix) {
            openssl_version = Some(parse_new_version(version));
        } else if let Some(version) = line.strip_prefix(libressl_prefix) {
            libressl_version = Some(parse_version(version));
        } else if let Some(conf) = line.strip_prefix(conf_prefix) {
            enabled.push(conf);
        } else if line.starts_with(boringssl_prefix) {
            is_boringssl = true;
        } else if line.starts_with(awslc_prefix) {
            is_awslc = true;
        } else if line.starts_with(symbol_prefix) {
            let sym_prefix = String::from(line.strip_prefix(symbol_prefix).unwrap());
            bindgen_symbol_prefix = Some(sym_prefix);
        }
    }

    for enabled in &enabled {
        println!("cargo:rustc-cfg=osslconf=\"{enabled}\"");
    }
    println!("cargo:conf={}", enabled.join(","));

    if is_boringssl {
        println!("cargo:rustc-cfg=boringssl");
        println!("cargo:boringssl=true");
        run_bindgen::run_boringssl(include_dirs);
        return Version::Boringssl;
    }

    if is_awslc {
        println!("cargo:rustc-cfg=awslc");
        println!("cargo:awslc=true");
        run_bindgen::run_awslc(include_dirs, bindgen_symbol_prefix);
        return Version::AwsLc;
    }

    // We set this for any non-BoringSSL lib.
    println!("cargo:rustc-cfg=openssl");

    for cfg in cfgs::get(openssl_version, libressl_version) {
        println!("cargo:rustc-cfg={cfg}");
    }

    if let Some(libressl_version) = libressl_version {
        println!("cargo:libressl_version_number={libressl_version:x}");

        let major = (libressl_version >> 28) as u8;
        let minor = (libressl_version >> 20) as u8;
        let fix = (libressl_version >> 12) as u8;
        let (major, minor, fix) = match (major, minor, fix) {
            (3, 5, _) => ('3', '5', 'x'),
            (3, 6, 0) => ('3', '6', '0'),
            (3, 6, _) => ('3', '6', 'x'),
            (3, 7, 0) => ('3', '7', '0'),
            (3, 7, 1) => ('3', '7', '1'),
            (3, 7, _) => ('3', '7', 'x'),
            (3, 8, 0) => ('3', '8', '0'),
            (3, 8, 1) => ('3', '8', '1'),
            (3, 8, _) => ('3', '8', 'x'),
            (3, 9, 0) => ('3', '9', '0'),
            (3, 9, _) => ('3', '9', 'x'),
            (4, 0, 0) => ('4', '0', '0'),
            (4, 0, _) => ('4', '0', 'x'),
            (4, 1, 0) => ('4', '1', '0'),
            (4, 1, _) => ('4', '1', 'x'),
            (4, 2, _) => ('4', '2', 'x'),
            (4, 3, _) => ('4', '3', 'x'),
            _ => version_error(),
        };

        println!("cargo:libressl=true");
        println!("cargo:libressl_version={major}{minor}{fix}");
        println!("cargo:version=101");
        Version::Libressl
    } else {
        let openssl_version = openssl_version.unwrap();
        println!("cargo:version_number={openssl_version:x}");

        if openssl_version >= 0x5_00_00_00_0 {
            version_error()
        } else if openssl_version >= 0x4_00_00_00_0 {
            Version::Openssl4xx
        } else if openssl_version >= 0x3_00_00_00_0 {
            Version::Openssl3xx
        } else if openssl_version >= 0x1_01_01_00_0 {
            println!("cargo:version=111");
            Version::Openssl11x
        } else if openssl_version >= 0x1_01_00_06_0 {
            println!("cargo:version=110");
            println!("cargo:patch=f");
            Version::Openssl11x
        } else if openssl_version >= 0x1_01_00_00_0 {
            println!("cargo:version=110");
            Version::Openssl11x
        } else {
            version_error()
        }
    }
}

fn version_error() -> ! {
    panic!(
        "

This crate is only compatible with OpenSSL (version 1.1.0, 1.1.1, 3.x, or 4.x), or LibreSSL 3.5.0
through 4.3.x, but a different version of OpenSSL was found. The build is now aborting
due to this version mismatch.

"
    );
}

// parses a string that looks like "0x100020cfL"
fn parse_version(version: &str) -> u64 {
    // cut off the 0x prefix
    assert!(version.starts_with("0x"));
    let version = &version[2..];

    // and the type specifier suffix
    let version = version.trim_end_matches(|c: char| !c.is_ascii_hexdigit());

    u64::from_str_radix(version, 16).unwrap()
}

// parses a string that looks like 3_0_0
fn parse_new_version(version: &str) -> u64 {
    println!("version: {version}");
    let mut it = version.split('_');
    let major = it.next().unwrap().parse::<u64>().unwrap();
    let minor = it.next().unwrap().parse::<u64>().unwrap();
    let patch = it.next().unwrap().parse::<u64>().unwrap();

    (major << 28) | (minor << 20) | (patch << 4)
}

/// Given a libdir for OpenSSL (where artifacts are located) as well as the name
/// of the libraries we're linking to, figure out whether we should link them
/// statically or dynamically.
fn determine_mode(libdirs: &[PathBuf], libs: &[&str]) -> &'static str {
    // First see if a mode was explicitly requested
    let kind = env("OPENSSL_STATIC");
    match kind.as_ref().and_then(|s| s.to_str()) {
        Some("0") => return "dylib",
        Some(_) => return "static",
        None => {}
    }

    // Next, see what files we actually have to link against, and see what our
    // possibilities even are.
    let mut files = HashSet::new();
    for dir in libdirs {
        for path in dir
            .read_dir()
            .unwrap()
            .map(|e| e.unwrap())
            .map(|e| e.file_name())
            .filter_map(|e| e.into_string().ok())
        {
            files.insert(path);
        }
    }
    let can_static = libs
        .iter()
        .all(|l| files.contains(&format!("lib{l}.a")) || files.contains(&format!("{l}.lib")));
    let can_dylib = libs.iter().all(|l| {
        files.contains(&format!("lib{l}.so"))
            || files.contains(&format!("{l}.dll"))
            || files.contains(&format!("lib{l}.dylib"))
    });
    match (can_static, can_dylib) {
        (true, false) => return "static",
        (false, true) => return "dylib",
        (false, false) => {
            panic!(
                "OpenSSL libdir at `{libdirs:?}` does not contain the required files \
                 to either statically or dynamically link OpenSSL"
            );
        }
        (true, true) => {}
    }

    // Ok, we've got not explicit preference and can *either* link statically or
    // link dynamically. In the interest of "security upgrades" and/or "best
    // practices with security libs", let's link dynamically.
    "dylib"
}
