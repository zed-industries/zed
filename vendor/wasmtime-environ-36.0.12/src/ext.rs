use target_lexicon::{Architecture, Triple};

/// Extension methods for `target_lexicon::Triple`.
pub trait TripleExt {
    /// Helper for returning whether this target is for pulley, wasmtime's
    /// interpreter.
    fn is_pulley(&self) -> bool;

    /// Returns the target triple for pulley to run on this host.
    fn pulley_host() -> Self;
}

impl TripleExt for Triple {
    fn is_pulley(&self) -> bool {
        match self.architecture {
            Architecture::Pulley32 | Architecture::Pulley32be => true,
            Architecture::Pulley64 | Architecture::Pulley64be => true,
            _ => false,
        }
    }

    fn pulley_host() -> Self {
        if cfg!(target_endian = "little") {
            if cfg!(target_pointer_width = "32") {
                return "pulley32".parse().unwrap();
            } else if cfg!(target_pointer_width = "64") {
                return "pulley64".parse().unwrap();
            }
        }
        if cfg!(target_endian = "big") {
            if cfg!(target_pointer_width = "32") {
                return "pulley32be".parse().unwrap();
            } else if cfg!(target_pointer_width = "64") {
                return "pulley64be".parse().unwrap();
            }
        }

        unreachable!()
    }
}
