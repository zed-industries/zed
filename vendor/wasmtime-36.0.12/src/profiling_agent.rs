use crate::prelude::*;

cfg_if::cfg_if! {
    if #[cfg(all(feature = "profiling", target_os = "linux"))] {
        mod jitdump;
        pub use jitdump::new as new_jitdump;
    } else {
        pub fn new_jitdump() -> Result<Box<dyn ProfilingAgent>> {
            if cfg!(feature = "profiling") {
                bail!("jitdump is not supported on this platform");
            } else {
                bail!("jitdump support disabled at compile time");
            }
        }
    }
}

cfg_if::cfg_if! {
    if #[cfg(all(unix, feature = "std"))] {
        mod perfmap;
        pub use perfmap::new as new_perfmap;
    } else {
        pub fn new_perfmap() -> Result<Box<dyn ProfilingAgent>> {
            bail!("perfmap support not supported on this platform");
        }
    }
}

cfg_if::cfg_if! {
    // Note that the `#[cfg]` here should be kept in sync with the
    // corresponding dependency directive on `ittapi` in `Cargo.toml`.
    if #[cfg(all(
        feature = "profiling",
        target_arch = "x86_64",
        any(
            target_os = "windows",
            target_os = "macos",
            target_os = "linux",
        ),
    ))] {
        mod vtune;
        pub use vtune::new as new_vtune;
    } else {
        pub fn new_vtune() -> Result<Box<dyn ProfilingAgent>> {
            if cfg!(feature = "profiling") {
                bail!("VTune is not supported on this platform.");
            } else {
                bail!("VTune support disabled at compile time.");
            }
        }
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "profile-pulley")] {
        mod pulley;
        pub use pulley::new as new_pulley;
    } else {
        pub fn new_pulley() -> Result<Box<dyn ProfilingAgent>> {
            bail!("pulley profiling support disabled at compile time.");
        }
    }
}

/// Common interface for profiling tools.
pub trait ProfilingAgent: Send + Sync + 'static {
    fn register_function(&self, name: &str, code: &[u8]);

    #[cfg(all(feature = "runtime", feature = "pulley"))]
    fn register_interpreter(&self, interp: &crate::vm::Interpreter) {
        let _ = interp;
    }

    fn register_module(&self, code: &[u8], custom_name: &dyn Fn(usize) -> Option<String>) {
        use object::{File, Object as _, ObjectSection, ObjectSymbol, SectionKind, SymbolKind};

        let image = match File::parse(code) {
            Ok(image) => image,
            Err(_) => return,
        };

        let text = match image
            .sections()
            .find(|s| s.kind() == SectionKind::Text || s.name() == Ok(".text"))
        {
            Some(section) => match section.data() {
                Ok(data) => data,
                Err(_) => return,
            },
            None => return,
        };

        for sym in image.symbols() {
            if !sym.is_definition() {
                continue;
            }
            if sym.kind() != SymbolKind::Text {
                continue;
            }
            let address = sym.address();
            let size = sym.size();
            if size == 0 {
                continue;
            }
            if let Ok(name) = sym.name() {
                let owned;
                let name = match custom_name(address as usize) {
                    Some(name) => {
                        owned = name;
                        &owned
                    }
                    None => name,
                };
                let code = &text[address as usize..][..size as usize];
                self.register_function(name, code)
            }
        }
    }
}

pub fn new_null() -> Box<dyn ProfilingAgent> {
    Box::new(NullProfilerAgent)
}

#[derive(Debug, Default, Clone, Copy)]
struct NullProfilerAgent;

impl ProfilingAgent for NullProfilerAgent {
    fn register_function(&self, _name: &str, _code: &[u8]) {}
    fn register_module(&self, _code: &[u8], _custom_name: &dyn Fn(usize) -> Option<String>) {}
}
