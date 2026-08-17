use derive_setters::Setters;

use crate::toolchain::Version;
use crate::{Run, Step};

#[derive(Clone, Setters)]
#[setters(strip_option, into)]
pub struct Cargo {
    /// The command to be executed for eg: fmt, clippy, build, test, etc.
    pub command: String,

    /// The unique identifier of the Step.
    pub id: Option<String>,

    /// Name of the Step
    pub name: Option<String>,

    /// Toolchain to be used for example `+nightly`.
    pub toolchain: Option<Version>,

    /// Arguments to be passed to the cargo command.
    #[setters(skip)]
    pub args: Vec<String>,
}

impl Cargo {
    /// Creates a new `Cargo` instance with the specified command.
    pub fn new<T: ToString>(cmd: T) -> Cargo {
        Cargo {
            command: cmd.to_string(),
            id: Default::default(),
            name: Default::default(),
            toolchain: Default::default(),
            args: Default::default(),
        }
    }

    /// Sets the toolchain to nightly.
    pub fn nightly(mut self) -> Self {
        self.toolchain = Some(Version::Nightly);
        self
    }

    /// Sets the arguments for the cargo command. If arguments are already set,
    /// it will be overwritten.
    pub fn args<T: ToString>(mut self, args: T) -> Self {
        self.args = vec![args.to_string()];
        self
    }

    /// Adds additional arguments to the cargo command.
    pub fn add_args<T: ToString>(mut self, args: T) -> Self {
        self.args.extend(
            args.to_string()
                .split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<_>>(),
        );
        self
    }
}

impl From<Cargo> for Step<Run> {
    fn from(value: Cargo) -> Self {
        let mut command = vec!["cargo".to_string()];

        if let Some(toolchain) = value.toolchain {
            command.push(format!("+{toolchain}"));
        }

        command.push(value.command.clone());

        // Extend the command with non-empty arguments
        command.extend(
            value
                .args
                .into_iter()
                .map(|arg| arg.trim().to_string())
                .filter(|arg| !arg.is_empty()),
        );

        let mut step = Step::new(format!("Cargo {}", value.command)).run(command.join(" "));

        if let Some(id) = value.id {
            step = step.id(id);
        }

        if let Some(name) = value.name {
            step = step.name(name);
        }

        step
    }
}
