//! Rust Windows resource helper
//!
//! This crate implements a simple generator for Windows resource (.rc) files
//! for use with either Microsoft `rc.exe` resource compiler or with GNU `windres.exe`
//!
//! The [`WindowsResource::compile()`] method is intended to be used from a build script and
//! needs environment variables from cargo to be set. It not only compiles the resource
//! but directs cargo to link the resource compiler's output.
//!
//! # Example
//!
//! ```rust
//! # extern crate winresource;
//! # use std::io;
//! # fn test_main() -> io::Result<()> {
//! if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
//!     let mut res = winresource::WindowsResource::new();
//!     res.set_icon("test.ico")
//! #      .set_output_directory(".")
//!        .set("InternalName", "TEST.EXE")
//!        // manually set version 1.0.0.0
//!        .set_version_info(winresource::VersionInfo::PRODUCTVERSION, 0x0001000000000000);
//!     res.compile()?;
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # Defaults
//!
//! We try to guess some sensible default values from Cargo's build time environment variables
//! This is described in [`WindowsResource::new()`]. Furthermore we have to know where to find the
//! resource compiler for the MSVC Toolkit. This can be done by looking up a registry key but
//! for MinGW this has to be done manually.
//!
//! The following paths are the hardcoded defaults:
//! MSVC the last registry key at
//! `HKLM\SOFTWARE\Microsoft\Windows Kits\Installed Roots`, for MinGW we try our luck by simply
//! using the `%PATH%` environment variable.
//!
//! Note that the toolkit bitness as to match the one from the current Rust compiler. If you are
//! using Rust GNU 64-bit you have to use MinGW64. For MSVC this is simpler as (recent) Windows
//! SDK always installs both versions on a 64-bit system.
//!
//! [`WindowsResource::compile()`]: struct.WindowsResource.html#method.compile
//! [`WindowsResource::new()`]: struct.WindowsResource.html#method.new

use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process;

#[cfg(feature = "toml")]
extern crate toml;

/// Version info field names
#[derive(PartialEq, Eq, Hash, Debug)]
pub enum VersionInfo {
    /// The version value consists of four 16 bit words, e.g.,
    /// `MAJOR << 48 | MINOR << 32 | PATCH << 16 | RELEASE`
    FILEVERSION,
    /// The version value consists of four 16 bit words, e.g.,
    /// `MAJOR << 48 | MINOR << 32 | PATCH << 16 | RELEASE`
    PRODUCTVERSION,
    /// Should be Windows NT Win32, with value `0x40004`
    FILEOS,
    /// The value (for a rust compiler output) should be
    /// 1 for a EXE and 2 for a DLL
    FILETYPE,
    /// Only for Windows drivers
    FILESUBTYPE,
    /// Bit mask for FILEFLAGS
    FILEFLAGSMASK,
    /// Only the bits set in FILEFLAGSMASK are read
    FILEFLAGS,
}

#[derive(Debug)]
struct Icon {
    path: String,
    name_id: String,
}

#[derive(Debug)]
pub struct WindowsResource {
    toolkit_path: PathBuf,
    properties: HashMap<String, String>,
    version_info: HashMap<VersionInfo, u64>,
    rc_file: Option<String>,
    icons: Vec<Icon>,
    language: u16,
    manifest: Option<String>,
    manifest_file: Option<String>,
    output_directory: String,
    windres_path: String,
    ar_path: String,
    add_toolkit_include: bool,
    append_rc_content: String,
}

#[allow(clippy::new_without_default)]
impl WindowsResource {
    /// Create a new resource with version info struct
    ///
    ///
    /// We initialize the resource file with values provided by cargo
    ///
    /// | Field                | Cargo / Values               |
    /// |----------------------|------------------------------|
    /// | `"FileVersion"`      | `package.version`            |
    /// | `"ProductVersion"`   | `package.version`            |
    /// | `"ProductName"`      | `package.name`               |
    /// | `"FileDescription"`  | `package.description`        |
    ///
    /// Furthermore if a section `package.metadata.winresource` exists
    /// in `Cargo.toml` it will be parsed. Values in this section take precedence
    /// over the values provided natively by cargo. Only the string table
    /// of the version struct can be set this way.
    /// Additionally, the language field is set to neutral (i.e. `0`)
    /// and no icon is set. These settings have to be done programmatically.
    ///
    /// `Cargo.toml` files have to be written in UTF-8, so we support all valid UTF-8 strings
    /// provided.
    ///
    /// ```,toml
    /// #Cargo.toml
    /// [package.metadata.winresource]
    /// OriginalFilename = "testing.exe"
    /// FileDescription = "⛄❤☕"
    /// LegalCopyright = "Copyright © 2016"
    /// ```
    ///
    /// The version info struct is set to some values
    /// sensible for creating an executable file.
    ///
    /// | Property             | Cargo / Values               |
    /// |----------------------|------------------------------|
    /// | `FILEVERSION`        | `package.version`            |
    /// | `PRODUCTVERSION`     | `package.version`            |
    /// | `FILEOS`             | `VOS_NT_WINDOWS32 (0x40004)` |
    /// | `FILETYPE`           | `VFT_APP (0x1)`              |
    /// | `FILESUBTYPE`        | `VFT2_UNKNOWN (0x0)`         |
    /// | `FILEFLAGSMASK`      | `VS_FFI_FILEFLAGSMASK (0x3F)`|
    /// | `FILEFLAGS`          | `0x0`                        |
    ///
    pub fn new() -> Self {
        let mut props: HashMap<String, String> = HashMap::new();
        let mut ver: HashMap<VersionInfo, u64> = HashMap::new();

        props.insert(
            "FileVersion".to_string(),
            env::var("CARGO_PKG_VERSION").unwrap(),
        );
        props.insert(
            "ProductVersion".to_string(),
            env::var("CARGO_PKG_VERSION").unwrap(),
        );
        props.insert(
            "ProductName".to_string(),
            env::var("CARGO_PKG_NAME").unwrap(),
        );

        // Note: It is not a mistake that we use the package name as `FileDescription`.
        // Windows uses the `FileDescription` as the application name in tools and dialogs.
        props.insert(
            "FileDescription".to_string(),
            env::var("CARGO_PKG_NAME").unwrap(),
        );

        #[cfg(feature = "toml")]
        parse_cargo_toml(&mut props).unwrap();

        let mut version = 0_u64;
        version |= env::var("CARGO_PKG_VERSION_MAJOR")
            .unwrap()
            .parse()
            .unwrap_or(0)
            << 48;
        version |= env::var("CARGO_PKG_VERSION_MINOR")
            .unwrap()
            .parse()
            .unwrap_or(0)
            << 32;
        version |= env::var("CARGO_PKG_VERSION_PATCH")
            .unwrap()
            .parse()
            .unwrap_or(0)
            << 16;
        // version |= env::var("CARGO_PKG_VERSION_PRE").unwrap().parse().unwrap_or(0);
        ver.insert(VersionInfo::FILEVERSION, version);
        ver.insert(VersionInfo::PRODUCTVERSION, version);
        ver.insert(VersionInfo::FILEOS, 0x00040004);
        ver.insert(VersionInfo::FILETYPE, 1);
        ver.insert(VersionInfo::FILESUBTYPE, 0);
        ver.insert(VersionInfo::FILEFLAGSMASK, 0x3F);
        ver.insert(VersionInfo::FILEFLAGS, 0);

        let sdk = if cfg!(all(windows, target_env = "msvc")) {
            match get_sdk() {
                Ok(mut v) => v.pop().unwrap(),
                Err(_) => PathBuf::new(),
            }
        } else if cfg!(windows) {
            PathBuf::from("\\")
        } else {
            PathBuf::from("/")
        };

        let prefix = if let Ok(cross) = env::var("CROSS_COMPILE") {
            cross
        } else if env::var_os("HOST").unwrap() != env::var_os("TARGET").unwrap()
            && cfg!(not(all(windows, target_env = "msvc")))
        // use mingw32 under linux
        {
            match env::var("TARGET").unwrap().as_str() {
                "x86_64-pc-windows-msvc" | // use mingw32 under linux
                "x86_64-pc-windows-gnu" => "x86_64-w64-mingw32-",
                "i686-pc-windows-msvc" | // use mingw32 under linux
                "i686-pc-windows-gnu" => "i686-w64-mingw32-",
                "i586-pc-windows-msvc" | // use mingw32 under linux
                "i586-pc-windows-gnu" => "i586-w64-mingw32-",
                // MinGW supports ARM64 only with an LLVM-based toolchain
                // (x86 users might also be using LLVM, but we can't tell that from the Rust target...)
                "aarch64-pc-windows-gnu" => "llvm-",
                // *-gnullvm targets by definition use LLVM-based toolchains
                "x86_64-pc-windows-gnullvm"
                | "i686-pc-windows-gnullvm"
                | "aarch64-pc-windows-gnullvm" => "llvm-",
                // fail safe
                target => {
                    println!(
                        "cargo:warning=unknown Windows target {target} used for cross-compilation; \
                              invoking unprefixed windres"
                    );
                    ""
                }
            }
            .into()
        } else {
            "".into()
        };
        let windres_path = if let Ok(windres) = env::var("WINDRES") {
            windres
        } else {
            format!("{}windres", prefix)
        };
        let ar_path = if let Ok(ar) = env::var("AR") {
            ar
        } else {
            format!("{}ar", prefix)
        };

        WindowsResource {
            toolkit_path: sdk,
            properties: props,
            version_info: ver,
            rc_file: None,
            icons: Vec::new(),
            language: 0,
            manifest: None,
            manifest_file: None,
            output_directory: env::var("OUT_DIR").unwrap_or_else(|_| ".".to_string()),
            windres_path,
            ar_path,
            add_toolkit_include: false,
            append_rc_content: String::new(),
        }
    }

    /// Set string properties of the version info struct.
    ///
    /// Possible field names are:
    ///
    ///  - `"FileVersion"`
    ///  - `"FileDescription"`
    ///  - `"ProductVersion"`
    ///  - `"ProductName"`
    ///  - `"OriginalFilename"`
    ///  - `"LegalCopyright"`
    ///  - `"LegalTrademark"`
    ///  - `"CompanyName"`
    ///  - `"Comments"`
    ///  - `"InternalName"`
    ///
    /// Additionally there exists
    /// `"PrivateBuild"`, `"SpecialBuild"`
    /// which should only be set, when the `FILEFLAGS` property is set to
    /// `VS_FF_PRIVATEBUILD(0x08)` or `VS_FF_SPECIALBUILD(0x20)`
    ///
    /// It is possible to use arbitrary field names but Windows Explorer and other
    /// tools might not show them.
    pub fn set(&mut self, name: &str, value: &str) -> &mut Self {
        self.properties.insert(name.to_string(), value.to_string());
        self
    }

    /// Set the correct path for the toolkit.
    ///
    /// For the GNU toolkit this has to be the path where MinGW
    /// put `windres.exe` and `ar.exe`. This could be something like:
    /// `"C:\Program Files\mingw-w64\x86_64-5.3.0-win32-seh-rt_v4-rev0\mingw64\bin"`
    ///
    /// For MSVC the Windows SDK has to be installed. It comes with the resource compiler
    /// `rc.exe`. This should be set to the root directory of the Windows SDK, e.g.,
    /// `"C:\Program Files (x86)\Windows Kits\10"`
    /// or, if multiple 10 versions are installed,
    /// set it directly to the correct bin directory
    /// `"C:\Program Files (x86)\Windows Kits\10\bin\10.0.14393.0\x64"`
    ///
    /// If it is left unset, it will look up a path in the registry,
    /// i.e. `HKLM\SOFTWARE\Microsoft\Windows Kits\Installed Roots`
    pub fn set_toolkit_path(&mut self, path: &str) -> &mut Self {
        self.toolkit_path = PathBuf::from(path);
        self
    }

    /// Set the user interface language of the file
    ///
    /// # Example
    ///
    /// ```no_run
    /// extern crate winapi;
    /// extern crate winresource;
    /// # use std::io;
    /// fn main() {
    ///   if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
    ///     let mut res = winresource::WindowsResource::new();
    /// #   res.set_output_directory(".");
    ///     res.set_language(winapi::um::winnt::MAKELANGID(
    ///         winapi::um::winnt::LANG_ENGLISH,
    ///         winapi::um::winnt::SUBLANG_ENGLISH_US
    ///     ));
    ///     res.compile().unwrap();
    ///   }
    /// }
    /// ```
    /// For possible values look at the `winapi::um::winnt` constants, specifically those
    /// starting with `LANG_` and `SUBLANG_`.
    ///
    /// [`MAKELANGID`]: https://docs.rs/winapi/0.3/x86_64-pc-windows-msvc/winapi/um/winnt/fn.MAKELANGID.html
    /// [`winapi::um::winnt`]: https://docs.rs/winapi/0.3/x86_64-pc-windows-msvc/winapi/um/winnt/index.html#constants
    ///
    /// # Table
    /// Sometimes it is just simpler to specify the numeric constant directly
    /// (That is what most `.rc` files do).
    /// For possible values take a look at the MSDN page for resource files;
    /// we only listed some values here.
    ///
    /// | Language            | Value    |
    /// |---------------------|----------|
    /// | Neutral             | `0x0000` |
    /// | English             | `0x0009` |
    /// | English (US)        | `0x0409` |
    /// | English (GB)        | `0x0809` |
    /// | German              | `0x0407` |
    /// | German (AT)         | `0x0c07` |
    /// | French              | `0x000c` |
    /// | French (FR)         | `0x040c` |
    /// | Catalan             | `0x0003` |
    /// | Basque              | `0x042d` |
    /// | Breton              | `0x007e` |
    /// | Scottish Gaelic     | `0x0091` |
    /// | Romansch            | `0x0017` |
    pub fn set_language(&mut self, language: u16) -> &mut Self {
        self.language = language;
        self
    }

    /// Add an icon with name ID `1`.
    ///
    /// This icon needs to be in `ico` format. The filename can be absolute
    /// or relative to the projects root.
    ///
    /// Equivalent to `set_icon_with_id(path, "1")`.
    pub fn set_icon(&mut self, path: &str) -> &mut Self {
        const DEFAULT_APPLICATION_ICON_ID: &str = "1";

        self.set_icon_with_id(path, DEFAULT_APPLICATION_ICON_ID)
    }

    /// Add an icon with the specified name ID.
    ///
    /// This icon need to be in `ico` format. The path can be absolute or
    /// relative to the projects root.
    ///
    /// ## Name ID and Icon Loading
    ///
    /// The name ID can be (the string representation of) a 16-bit unsigned
    /// integer, or some other string.
    ///
    /// You should not add multiple icons with the same name ID. It will result
    /// in a build failure.
    ///
    /// When the name ID is an integer, the icon can be loaded at runtime with
    ///
    /// ```ignore
    /// LoadIconW(h_instance, MAKEINTRESOURCEW(name_id_as_integer))
    /// ```
    ///
    /// Otherwise, it can be loaded with
    ///
    /// ```ignore
    /// LoadIconW(h_instance, name_id_as_wide_c_str_as_ptr)
    /// ```
    ///
    /// Where `h_instance` is the module handle of the current executable
    /// ([`GetModuleHandleW`](https://docs.rs/winapi/0.3.8/winapi/um/libloaderapi/fn.GetModuleHandleW.html)`(null())`),
    /// [`LoadIconW`](https://docs.rs/winapi/0.3.8/winapi/um/winuser/fn.LoadIconW.html)
    /// and
    /// [`MAKEINTRESOURCEW`](https://docs.rs/winapi/0.3.8/winapi/um/winuser/fn.MAKEINTRESOURCEW.html)
    /// are defined in winapi.
    ///
    /// ## Multiple Icons, Which One is Application Icon?
    ///
    /// When you have multiple icons, it's a bit complicated which one will be
    /// chosen as the application icon:
    /// <https://docs.microsoft.com/en-us/previous-versions/ms997538(v=msdn.10)?redirectedfrom=MSDN#choosing-an-icon>.
    ///
    /// To keep things simple, we recommend you use only 16-bit unsigned integer
    /// name IDs, and add the application icon first with the lowest id:
    ///
    /// ```nocheck
    /// res.set_icon("icon.ico") // This is application icon.
    ///    .set_icon_with_id("icon2.icon", "2")
    ///    .set_icon_with_id("icon3.icon", "3")
    ///    // ...
    /// ```
    pub fn set_icon_with_id(&mut self, path: &str, name_id: &str) -> &mut Self {
        self.icons.push(Icon {
            path: path.into(),
            name_id: name_id.into(),
        });
        self
    }

    /// Set a version info struct property
    /// Currently we only support numeric values; you have to look them up.
    pub fn set_version_info(&mut self, field: VersionInfo, value: u64) -> &mut Self {
        self.version_info.insert(field, value);
        self
    }

    /// Set the embedded manifest file
    ///
    /// # Example
    ///
    /// The following manifest will brand the exe as requesting administrator privileges.
    /// Thus, everytime it is executed, a Windows UAC dialog will appear.
    ///
    /// ```rust
    /// if std::env::var("CARGO_CFG_TARGET_OS").unwrap_or("".to_string()) == "windows" {
    /// let mut res = winresource::WindowsResource::new();
    /// res.set_manifest(r#"
    /// <assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
    /// <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
    ///     <security>
    ///         <requestedPrivileges>
    ///             <requestedExecutionLevel level="requireAdministrator" uiAccess="false" />
    ///         </requestedPrivileges>
    ///     </security>
    /// </trustInfo>
    /// </assembly>
    /// "#);
    /// }
    /// ```
    pub fn set_manifest(&mut self, manifest: &str) -> &mut Self {
        self.manifest_file = None;
        self.manifest = Some(manifest.to_string());
        self
    }

    /// Some as [`set_manifest()`] but a filename can be provided and
    /// file is included by the resource compiler itself.
    /// This method works the same way as [`set_icon()`]
    ///
    /// [`set_manifest()`]: #method.set_manifest
    /// [`set_icon()`]: #method.set_icon
    pub fn set_manifest_file(&mut self, file: &str) -> &mut Self {
        self.manifest_file = Some(file.to_string());
        self.manifest = None;
        self
    }

    /// Set the path to the windres executable.
    pub fn set_windres_path(&mut self, path: &str) -> &mut Self {
        self.windres_path = path.to_string();
        self
    }

    /// Set the path to the ar executable.
    pub fn set_ar_path(&mut self, path: &str) -> &mut Self {
        self.ar_path = path.to_string();
        self
    }

    /// Set the path to the ar executable.
    pub fn add_toolkit_include(&mut self, add: bool) -> &mut Self {
        self.add_toolkit_include = add;
        self
    }

    /// Write a resource file with the set values
    pub fn write_resource_file<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let mut f = fs::File::create(path)?;

        // use UTF8 as an encoding
        // this makes it easier since in rust all string are UTF8
        writeln!(f, "#pragma code_page(65001)")?;
        writeln!(f, "1 VERSIONINFO")?;
        for (k, v) in self.version_info.iter() {
            match *k {
                VersionInfo::FILEVERSION | VersionInfo::PRODUCTVERSION => writeln!(
                    f,
                    "{:?} {}, {}, {}, {}",
                    k,
                    (*v >> 48) as u16,
                    (*v >> 32) as u16,
                    (*v >> 16) as u16,
                    *v as u16
                )?,
                _ => writeln!(f, "{:?} {:#x}", k, v)?,
            };
        }
        writeln!(f, "{{\nBLOCK \"StringFileInfo\"")?;
        writeln!(f, "{{\nBLOCK \"{:04x}04b0\"\n{{", self.language)?;
        for (k, v) in self.properties.iter() {
            if !v.is_empty() {
                writeln!(
                    f,
                    "VALUE \"{}\", \"{}\"",
                    escape_string(k),
                    escape_string(v)
                )?;
            }
        }
        writeln!(f, "}}\n}}")?;

        writeln!(f, "BLOCK \"VarFileInfo\" {{")?;
        writeln!(f, "VALUE \"Translation\", {:#x}, 0x04b0", self.language)?;
        writeln!(f, "}}\n}}")?;
        for icon in &self.icons {
            writeln!(
                f,
                "{} ICON \"{}\"",
                escape_string(&icon.name_id),
                escape_string(&icon.path)
            )?;
        }
        if let Some(e) = self.version_info.get(&VersionInfo::FILETYPE) {
            if let Some(manf) = self.manifest.as_ref() {
                writeln!(f, "{} 24", e)?;
                writeln!(f, "{{")?;
                for line in manf.lines() {
                    writeln!(f, "\" {} \"", escape_string(line.trim()))?;
                }
                writeln!(f, "}}")?;
            } else if let Some(manf) = self.manifest_file.as_ref() {
                writeln!(f, "{} 24 \"{}\"", e, escape_string(manf))?;
            }
        }
        writeln!(f, "{}", self.append_rc_content)?;
        Ok(())
    }

    /// Set a path to an already existing resource file.
    ///
    /// We will neither modify this file nor parse its contents. This function
    /// simply replaces the internally generated resource file that is passed to
    /// the compiler. You can use this function to write a resource file yourself.
    pub fn set_resource_file(&mut self, path: &str) -> &mut Self {
        self.rc_file = Some(path.to_string());
        self
    }

    /// Append an additional snippet to the generated rc file.
    ///
    /// # Example
    ///
    /// Define a menu resource:
    ///
    /// ```rust
    /// extern crate winresource;
    /// # if std::env::var("CARGO_CFG_TARGET_OS").unwrap_or("".to_string()) == "windows" {
    ///     let mut res = winresource::WindowsResource::new();
    ///     res.append_rc_content(r##"sample MENU
    /// {
    ///     MENUITEM "&Soup", 100
    ///     MENUITEM "S&alad", 101
    ///     POPUP "&Entree"
    ///     {
    ///          MENUITEM "&Fish", 200
    ///          MENUITEM "&Chicken", 201, CHECKED
    ///          POPUP "&Beef"
    ///          {
    ///               MENUITEM "&Steak", 301
    ///               MENUITEM "&Prime Rib", 302
    ///          }
    ///     }
    ///     MENUITEM "&Dessert", 103
    /// }"##);
    /// #    res.compile()?;
    /// # }
    /// # Ok::<_, std::io::Error>(())
    /// ```
    pub fn append_rc_content(&mut self, content: &str) -> &mut Self {
        if !(self.append_rc_content.ends_with('\n') || self.append_rc_content.is_empty()) {
            self.append_rc_content.push('\n');
        }
        self.append_rc_content.push_str(content);
        self
    }

    /// Override the output directory.
    ///
    /// As a default, we use `%OUT_DIR%` set by cargo, but it may be necessary to override the
    /// the setting.
    pub fn set_output_directory(&mut self, path: &str) -> &mut Self {
        self.output_directory = path.to_string();
        self
    }

    fn compile_with_toolkit_gnu(&self, input: &str, output_dir: &str) -> io::Result<()> {
        let bfd_target = match env::var("CARGO_CFG_TARGET_ARCH").unwrap().as_str() {
            "x86_64" => &["--target", "pe-x86-64"][..],
            "x86" => &["--target", "pe-i386"],
            // The case below would be correct for AArch64, but LLVM's windres does not handle
            // the conversion from this BFD target to its LLVM target, treating it as a native
            // LLVM target instead, which causes an error. Obviously, passing a LLVM target
            // is not portable to the binutils windres implementation. So, to prevent breaking
            // native AArch64 compilation with the LLVM toolchain, let's fall back to the default
            // target, which would provide the highest success rate
            //"aarch64" => &["--target", "pe-aarch64-little"],
            _ => &[], // A quite strange arch - use the default windres target and hope for the best
        };
        let output = PathBuf::from(output_dir).join("resource.o");
        let input = PathBuf::from(input);
        let status = process::Command::new(&self.windres_path)
            .current_dir(&self.toolkit_path)
            .arg(format!("-I{}", env::var("CARGO_MANIFEST_DIR").unwrap()))
            .args(bfd_target)
            .arg(format!("{}", input.display()))
            .arg(format!("{}", output.display()))
            .status()?;
        if !status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Could not compile resource file",
            ));
        }

        let libname = PathBuf::from(output_dir).join("libresource.a");
        let status = process::Command::new(&self.ar_path)
            .current_dir(&self.toolkit_path)
            .arg("rsc")
            .arg(format!("{}", libname.display()))
            .arg(format!("{}", output.display()))
            .status()?;
        if !status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Could not create static library for resource file",
            ));
        }

        println!("cargo:rustc-link-search=native={}", output_dir);

        if version_check::is_min_version("1.61.0").unwrap_or(true) {
            println!("cargo:rustc-link-lib=static:+whole-archive=resource");
        } else {
            println!("cargo:rustc-link-lib=static=resource");
        }

        Ok(())
    }

    /// Run the resource compiler
    ///
    /// This function generates a resource file from the settings or
    /// uses an existing resource file and passes it to the resource compiler
    /// of your toolkit.
    ///
    /// Further more we will print the correct statements for
    /// `cargo:rustc-link-lib=` and `cargo:rustc-link-search` on the console,
    /// so that the cargo build script can link the compiled resource file.
    pub fn compile(&self) -> io::Result<()> {
        let output = PathBuf::from(&self.output_directory);
        let rc = output.join("resource.rc");
        if self.rc_file.is_none() {
            self.write_resource_file(&rc)?;
        }
        let rc = if let Some(s) = self.rc_file.as_ref() {
            s.clone()
        } else {
            rc.to_str().unwrap().to_string()
        };

        let target_env = std::env::var("CARGO_CFG_TARGET_ENV").unwrap();
        match target_env.as_str() {
            "gnu" => self.compile_with_toolkit_gnu(rc.as_str(), &self.output_directory),
            "msvc" => self.compile_with_toolkit_msvc(rc.as_str(), &self.output_directory),
            _ => Err(io::Error::new(
                io::ErrorKind::Other,
                "Can only compile resource file when target_env is \"gnu\" or \"msvc\"",
            )),
        }
    }

    fn compile_with_toolkit_msvc(&self, input: &str, output_dir: &str) -> io::Result<()> {
        let rc_exe = if let Some(rc_path) = std::env::var_os("RC_PATH") {
            PathBuf::from(rc_path)
        } else if cfg!(unix) {
            PathBuf::from("llvm-rc")
        } else {
            let rc_exe = PathBuf::from(&self.toolkit_path).join("rc.exe");
            if !rc_exe.exists() {
                if cfg!(target_arch = "x86_64") {
                    PathBuf::from(&self.toolkit_path).join(r"bin\x64\rc.exe")
                } else {
                    PathBuf::from(&self.toolkit_path).join(r"bin\x86\rc.exe")
                }
            } else {
                rc_exe
            }
        };

        println!("Selected RC path: '{}'", rc_exe.display());
        let output = PathBuf::from(output_dir).join("resource.lib");
        let input = PathBuf::from(input);
        let mut command = process::Command::new(&rc_exe);
        let command = command.arg(format!("/I{}", env::var("CARGO_MANIFEST_DIR").unwrap()));

        if self.add_toolkit_include {
            let root = win_sdk_include_root(&rc_exe);
            println!("Adding toolkit include: {}", root.display());
            command.arg(format!("/I{}", root.join("um").display()));
            command.arg(format!("/I{}", root.join("shared").display()));
        }

        command.arg(format!("/fo{}", output.display()));

        if cfg!(unix) {
            // Fix for https://github.com/llvm/llvm-project/issues/63426
            command.args(["/C", "65001"]);

            // Ensure paths starting with "/Users" on macOS are not interpreted as a /U option
            command.arg("--");
        }

        let status = command.arg(format!("{}", input.display())).output()?;

        println!(
            "RC Output:\n{}\n------",
            String::from_utf8_lossy(&status.stdout)
        );
        println!(
            "RC Error:\n{}\n------",
            String::from_utf8_lossy(&status.stderr)
        );
        if !status.status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Could not compile resource file",
            ));
        }

        println!("cargo:rustc-link-search=native={}", output_dir);
        println!("cargo:rustc-link-lib=dylib=resource");
        Ok(())
    }
}

/// Find a Windows SDK
fn get_sdk() -> io::Result<Vec<PathBuf>> {
    // use the reg command, so we don't need a winapi dependency
    let output = process::Command::new("reg")
        .arg("query")
        .arg(r"HKLM\SOFTWARE\Microsoft\Windows Kits\Installed Roots")
        .arg("/reg:32")
        .output()?;

    if !output.status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "Querying the registry failed with error message:\n{}",
                String::from_utf8(output.stderr)
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
            ),
        ));
    }

    let lines = String::from_utf8(output.stdout)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    let mut kits: Vec<PathBuf> = Vec::new();
    let mut lines: Vec<&str> = lines.lines().collect();
    lines.reverse();
    for line in lines {
        if line.trim().starts_with("KitsRoot") {
            let kit: String = line
                .chars()
                .skip(line.find("REG_SZ").unwrap() + 6)
                .skip_while(|c| c.is_whitespace())
                .collect();

            let p = PathBuf::from(&kit);
            let rc = if cfg!(target_arch = "x86_64") {
                p.join(r"bin\x64\rc.exe")
            } else {
                p.join(r"bin\x86\rc.exe")
            };

            if rc.exists() {
                println!("{:?}", rc);
                kits.push(rc.parent().unwrap().to_owned());
            }

            if let Ok(bin) = p.join("bin").read_dir() {
                for e in bin.filter_map(|e| e.ok()) {
                    let p = if cfg!(target_arch = "x86_64") {
                        e.path().join(r"x64\rc.exe")
                    } else {
                        e.path().join(r"x86\rc.exe")
                    };
                    if p.exists() {
                        println!("{:?}", p);
                        kits.push(p.parent().unwrap().to_owned());
                    }
                }
            }
        }
    }
    if kits.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Can not find Windows SDK",
        ));
    }

    Ok(kits)
}

#[cfg(feature = "toml")]
fn parse_cargo_toml(props: &mut HashMap<String, String>) -> io::Result<()> {
    let cargo = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("Cargo.toml");
    let mut f = fs::File::open(cargo)?;
    let mut cargo_toml = String::new();
    f.read_to_string(&mut cargo_toml)?;
    if let Ok(ml) = cargo_toml.parse::<toml::Value>() {
        if let Some(pkg) = ml.get("package") {
            if let Some(pkg) = pkg.get("metadata") {
                if let Some(pkg) = pkg.get("winresource") {
                    if let Some(pkg) = pkg.as_table() {
                        for (k, v) in pkg {
                            // println!("{} {}", k ,v);
                            if let Some(v) = v.as_str() {
                                props.insert(k.clone(), v.to_string());
                            } else {
                                println!("package.metadata.winresource.{} is not a string", k);
                            }
                        }
                    } else {
                        println!("package.metadata.winresource is not a table");
                    }
                } else {
                    println!("package.metadata.winresource does not exist");
                }
            } else {
                println!("package.metadata does not exist");
            }
        } else {
            println!("package does not exist");
        }
    } else {
        println!("TOML parsing error")
    }
    Ok(())
}

fn escape_string(string: &str) -> String {
    let mut escaped = String::new();
    for chr in string.chars() {
        // In quoted RC strings, double-quotes are escaped by using two
        // consecutive double-quotes.  Other characters are escaped in the
        // usual C way using backslashes.
        match chr {
            '"' => escaped.push_str("\"\""),
            '\'' => escaped.push_str("\\'"),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\t' => escaped.push_str("\\t"),
            '\r' => escaped.push_str("\\r"),
            _ => escaped.push(chr),
        };
    }
    escaped
}

#[allow(dead_code)]
fn win_sdk_include_root(path: &Path) -> PathBuf {
    let mut tools_path = PathBuf::new();
    let mut iter = path.iter();
    while let Some(p) = iter.next() {
        if p == "bin" {
            let version = iter.next().unwrap();
            tools_path.push("Include");
            if version.to_string_lossy().starts_with("10.") {
                tools_path.push(version);
            }
            break;
        } else {
            tools_path.push(p);
        }
    }

    tools_path
}

#[cfg(test)]
mod tests {
    use super::escape_string;
    use super::win_sdk_include_root;

    #[test]
    fn string_escaping() {
        assert_eq!(&escape_string(""), "");
        assert_eq!(&escape_string("foo"), "foo");
        assert_eq!(&escape_string(r#""Hello""#), r#"""Hello"""#);
        assert_eq!(
            &escape_string(r"C:\Program Files\Foobar"),
            r"C:\\Program Files\\Foobar"
        );
    }

    #[test]
    #[cfg_attr(not(target_os = "windows"), ignore)]
    fn toolkit_include_win10() {
        use std::path::Path;

        let res = win_sdk_include_root(Path::new(
            r"C:\Program Files (x86)\Windows Kits\10\bin\10.0.17763.0\x64\rc.exe",
        ));

        assert_eq!(
            res.as_os_str(),
            r"C:\Program Files (x86)\Windows Kits\10\Include\10.0.17763.0"
        );
    }

    #[test]
    #[cfg_attr(not(target_os = "windows"), ignore)]
    fn toolkit_include_win8() {
        use std::path::Path;

        let res = win_sdk_include_root(Path::new(
            r"C:\Program Files (x86)\Windows Kits\8.1\bin\x86\rc.exe",
        ));
        assert_eq!(
            res.as_os_str(),
            r"C:\Program Files (x86)\Windows Kits\8.1\Include"
        );
    }
}
