use anyhow::Result;
use cranelift_codegen::isa::IsaBuilder as Builder;
use cranelift_codegen::settings::{self, Configurable, Flags, SetError};
use target_lexicon::Triple;
use wasmtime_environ::{Setting, SettingKind};

/// A helper to build an Isa for a compiler implementation.
/// Compiler builders can wrap this to provide better flexibility when setting flags.
///
/// Most methods are mirrored from the `wasmtime_environ::CompilerBuilder` trait, so look there for more
/// information.
pub struct IsaBuilder<T> {
    /// The shared flags that all targets share.
    shared_flags: settings::Builder,
    /// The internal ISA builder for the current target.
    inner: Builder<T>,
    /// A callback to lookup a new ISA builder for a target.
    pub lookup: fn(Triple) -> Result<Builder<T>>,
}

impl<T> IsaBuilder<T> {
    /// Create a new ISA builder with the given lookup function.
    pub fn new(triple: Option<Triple>, lookup: fn(Triple) -> Result<Builder<T>>) -> Result<Self> {
        let mut flags = settings::builder();

        // We don't use probestack as a stack limit mechanism
        flags
            .set("enable_probestack", "false")
            .expect("should be valid flag");

        let triple_specified = triple.is_some();
        let triple = triple.unwrap_or_else(Triple::host);
        let mut isa_flags = lookup(triple)?;
        if !triple_specified {
            cranelift_native::infer_native_flags(&mut isa_flags).unwrap();
        }

        Ok(Self {
            shared_flags: flags,
            inner: isa_flags,
            lookup,
        })
    }

    pub fn triple(&self) -> &target_lexicon::Triple {
        self.inner.triple()
    }

    pub fn target(&mut self, target: target_lexicon::Triple) -> Result<()> {
        self.inner = (self.lookup)(target)?;
        Ok(())
    }

    pub fn settings(&self) -> Vec<Setting> {
        self.inner
            .iter()
            .map(|s| Setting {
                description: s.description,
                name: s.name,
                values: s.values,
                kind: match s.kind {
                    settings::SettingKind::Preset => SettingKind::Preset,
                    settings::SettingKind::Enum => SettingKind::Enum,
                    settings::SettingKind::Num => SettingKind::Num,
                    settings::SettingKind::Bool => SettingKind::Bool,
                },
            })
            .collect()
    }

    pub fn set(&mut self, name: &str, value: &str) -> Result<()> {
        if let Err(err) = self.shared_flags.set(name, value) {
            match err {
                SetError::BadName(_) => {
                    self.inner.set(name, value)?;
                }
                _ => return Err(err.into()),
            }
        }
        Ok(())
    }

    pub fn enable(&mut self, name: &str) -> Result<()> {
        if let Err(err) = self.shared_flags.enable(name) {
            match err {
                SetError::BadName(_) => {
                    // Try the target-specific flags.
                    self.inner.enable(name)?;
                }
                _ => return Err(err.into()),
            }
        }
        Ok(())
    }

    pub fn build(&self) -> T {
        self.inner
            .finish(settings::Flags::new(self.shared_flags.clone()))
    }

    pub fn shared_flags(&self) -> Flags {
        settings::Flags::new(self.shared_flags.clone())
    }
}
