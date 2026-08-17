use anyhow::{Result, bail};
use std::collections::HashSet;
use std::fmt;
use wit_parser::{Function, FunctionKind, Resolve, WorldKey};

/// Structure used to parse the command line argument `--async` consistently
/// across guest generators.
#[cfg_attr(feature = "clap", derive(clap::Parser))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default, Debug)]
pub struct AsyncFilterSet {
    /// Determines which functions to lift or lower `async`, if any.
    ///
    /// This option can be passed multiple times and additionally accepts
    /// comma-separated values for each option passed. Each individual argument
    /// passed here can be one of:
    ///
    /// - `all` - all imports and exports will be async
    /// - `-all` - force all imports and exports to be sync
    /// - `foo:bar/baz#method` - force this method to be async
    /// - `import:foo:bar/baz#method` - force this method to be async, but only
    ///   as an import
    /// - `-export:foo:bar/baz#method` - force this export to be sync
    ///
    /// If a method is not listed in this option then the WIT's default bindings
    /// mode will be used. If the WIT function is defined as `async` then async
    /// bindings will be generated, otherwise sync bindings will be generated.
    ///
    /// Options are processed in the order they are passed here, so if a method
    /// matches two directives passed the least-specific one should be last.
    #[cfg_attr(
        feature = "clap",
        arg(
            long = "async",
            value_parser = parse_async,
            value_delimiter =',',
            value_name = "FILTER",
        ),
    )]
    #[cfg_attr(feature = "serde", serde(rename = "async"))]
    async_: Vec<Async>,

    #[cfg_attr(feature = "clap", arg(skip))]
    #[cfg_attr(feature = "serde", serde(skip))]
    used_options: HashSet<usize>,
}

#[cfg(feature = "clap")]
fn parse_async(s: &str) -> Result<Async, String> {
    Ok(Async::parse(s))
}

impl AsyncFilterSet {
    /// Returns a set where all functions should be async or not depending on
    /// `async_` provided.
    pub fn all(async_: bool) -> AsyncFilterSet {
        AsyncFilterSet {
            async_: vec![Async {
                enabled: async_,
                filter: AsyncFilter::All,
            }],
            used_options: HashSet::new(),
        }
    }

    /// Returns whether the `func` provided is to be bound `async` or not.
    pub fn is_async(
        &mut self,
        resolve: &Resolve,
        interface: Option<&WorldKey>,
        func: &Function,
        is_import: bool,
    ) -> bool {
        let name_to_test = match interface {
            Some(key) => format!("{}#{}", resolve.name_world_key(key), func.name),
            None => func.name.clone(),
        };
        for (i, opt) in self.async_.iter().enumerate() {
            let name = match &opt.filter {
                AsyncFilter::All => {
                    self.used_options.insert(i);
                    return opt.enabled;
                }
                AsyncFilter::Function(s) => s,
                AsyncFilter::Import(s) => {
                    if !is_import {
                        continue;
                    }
                    s
                }
                AsyncFilter::Export(s) => {
                    if is_import {
                        continue;
                    }
                    s
                }
            };
            if *name == name_to_test {
                self.used_options.insert(i);
                return opt.enabled;
            }
        }

        match &func.kind {
            FunctionKind::Freestanding
            | FunctionKind::Method(_)
            | FunctionKind::Static(_)
            | FunctionKind::Constructor(_) => false,
            FunctionKind::AsyncFreestanding
            | FunctionKind::AsyncMethod(_)
            | FunctionKind::AsyncStatic(_) => true,
        }
    }

    /// Intended to be used in the header comment of generated code to help
    /// indicate what options were specified.
    pub fn debug_opts(&self) -> impl Iterator<Item = String> + '_ {
        self.async_.iter().map(|opt| opt.to_string())
    }

    /// Tests whether all `--async` options were used throughout bindings
    /// generation, returning an error if any were unused.
    pub fn ensure_all_used(&self) -> Result<()> {
        for (i, opt) in self.async_.iter().enumerate() {
            if self.used_options.contains(&i) {
                continue;
            }
            if !matches!(opt.filter, AsyncFilter::All) {
                bail!("unused async option: {opt}");
            }
        }
        Ok(())
    }

    /// Returns whether any option explicitly requests that async is enabled.
    pub fn any_enabled(&self) -> bool {
        self.async_.iter().any(|o| o.enabled)
    }

    /// Pushes a new option into this set.
    pub fn push(&mut self, directive: &str) {
        self.async_.push(Async::parse(directive));
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
struct Async {
    enabled: bool,
    filter: AsyncFilter,
}

impl Async {
    fn parse(s: &str) -> Async {
        let (s, enabled) = match s.strip_prefix('-') {
            Some(s) => (s, false),
            None => (s, true),
        };
        let filter = match s {
            "all" => AsyncFilter::All,
            other => match other.strip_prefix("import:") {
                Some(s) => AsyncFilter::Import(s.to_string()),
                None => match other.strip_prefix("export:") {
                    Some(s) => AsyncFilter::Export(s.to_string()),
                    None => AsyncFilter::Function(s.to_string()),
                },
            },
        };
        Async { enabled, filter }
    }
}

impl fmt::Display for Async {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.enabled {
            write!(f, "-")?;
        }
        self.filter.fmt(f)
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
enum AsyncFilter {
    All,
    Function(String),
    Import(String),
    Export(String),
}

impl fmt::Display for AsyncFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AsyncFilter::All => write!(f, "all"),
            AsyncFilter::Function(s) => write!(f, "{s}"),
            AsyncFilter::Import(s) => write!(f, "import:{s}"),
            AsyncFilter::Export(s) => write!(f, "export:{s}"),
        }
    }
}
