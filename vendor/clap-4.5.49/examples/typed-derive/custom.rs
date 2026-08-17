use clap::Args;

use crate::implicit::BumpLevel;

#[derive(Args, Debug)]
pub(crate) struct CustomParser {
    /// Hand-implement `TypedValueParser`
    #[arg(long)]
    target_version: Option<TargetVersion>,
}

/// Enum or custom value
#[derive(Clone, Debug)]
pub(crate) enum TargetVersion {
    Relative(BumpLevel),
    Absolute(semver::Version),
}

impl std::fmt::Display for TargetVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            TargetVersion::Relative(bump_level) => {
                write!(f, "{bump_level}")
            }
            TargetVersion::Absolute(version) => {
                write!(f, "{version}")
            }
        }
    }
}

impl std::str::FromStr for TargetVersion {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(bump_level) = BumpLevel::from_str(s) {
            Ok(TargetVersion::Relative(bump_level))
        } else {
            Ok(TargetVersion::Absolute(
                semver::Version::parse(s).map_err(|e| e.to_string())?,
            ))
        }
    }
}

/// Default to `TargetVersionParser` for `TargetVersion`, instead of `FromStr`
impl clap::builder::ValueParserFactory for TargetVersion {
    type Parser = TargetVersionParser;

    fn value_parser() -> Self::Parser {
        TargetVersionParser
    }
}

#[derive(Copy, Clone)]
pub(crate) struct TargetVersionParser;

impl clap::builder::TypedValueParser for TargetVersionParser {
    type Value = TargetVersion;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let inner_parser = <TargetVersion as std::str::FromStr>::from_str;
        inner_parser.parse_ref(cmd, arg, value)
    }

    fn possible_values(
        &self,
    ) -> Option<Box<dyn Iterator<Item = clap::builder::PossibleValue> + '_>> {
        let inner_parser = clap::builder::EnumValueParser::<BumpLevel>::new();
        #[allow(clippy::needless_collect)] // Erasing a lifetime
        inner_parser.possible_values().map(|ps| {
            let ps = ps.collect::<Vec<_>>();
            let ps: Box<dyn Iterator<Item = clap::builder::PossibleValue> + '_> =
                Box::new(ps.into_iter());
            ps
        })
    }
}
