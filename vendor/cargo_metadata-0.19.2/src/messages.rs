use super::{Diagnostic, PackageId, Target};
use camino::Utf8PathBuf;
#[cfg(feature = "builder")]
use derive_builder::Builder;
use serde::{de, ser, Deserialize, Serialize};
use std::fmt::{self, Write};
use std::io::{self, BufRead, Read};

/// Profile settings used to determine which compiler flags to use for a
/// target.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[non_exhaustive]
#[cfg_attr(feature = "builder", builder(pattern = "owned", setter(into)))]
pub struct ArtifactProfile {
    /// Optimization level. Possible values are 0-3, s or z.
    pub opt_level: String,
    /// The kind of debug information.
    #[serde(default)]
    pub debuginfo: ArtifactDebuginfo,
    /// State of the `cfg(debug_assertions)` directive, enabling macros like
    /// `debug_assert!`
    pub debug_assertions: bool,
    /// State of the overflow checks.
    pub overflow_checks: bool,
    /// Whether this profile is a test
    pub test: bool,
}

/// The kind of debug information included in the artifact.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum ArtifactDebuginfo {
    /// No debug information.
    #[default]
    None,
    /// Line directives only.
    LineDirectivesOnly,
    /// Line tables only.
    LineTablesOnly,
    /// Debug information without type or variable-level information.
    Limited,
    /// Full debug information.
    Full,
    /// An unknown integer level.
    ///
    /// This may be produced by a version of rustc in the future that has
    /// additional levels represented by an integer that are not known by this
    /// version of `cargo_metadata`.
    UnknownInt(i64),
    /// An unknown string level.
    ///
    /// This may be produced by a version of rustc in the future that has
    /// additional levels represented by a string that are not known by this
    /// version of `cargo_metadata`.
    UnknownString(String),
}

impl ser::Serialize for ArtifactDebuginfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match self {
            Self::None => 0.serialize(serializer),
            Self::LineDirectivesOnly => "line-directives-only".serialize(serializer),
            Self::LineTablesOnly => "line-tables-only".serialize(serializer),
            Self::Limited => 1.serialize(serializer),
            Self::Full => 2.serialize(serializer),
            Self::UnknownInt(n) => n.serialize(serializer),
            Self::UnknownString(s) => s.serialize(serializer),
        }
    }
}

impl<'de> de::Deserialize<'de> for ArtifactDebuginfo {
    fn deserialize<D>(d: D) -> Result<ArtifactDebuginfo, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct Visitor;

        impl de::Visitor<'_> for Visitor {
            type Value = ArtifactDebuginfo;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("an integer or string")
            }

            fn visit_i64<E>(self, value: i64) -> Result<ArtifactDebuginfo, E>
            where
                E: de::Error,
            {
                let debuginfo = match value {
                    0 => ArtifactDebuginfo::None,
                    1 => ArtifactDebuginfo::Limited,
                    2 => ArtifactDebuginfo::Full,
                    n => ArtifactDebuginfo::UnknownInt(n),
                };
                Ok(debuginfo)
            }

            fn visit_u64<E>(self, value: u64) -> Result<ArtifactDebuginfo, E>
            where
                E: de::Error,
            {
                self.visit_i64(value as i64)
            }

            fn visit_str<E>(self, value: &str) -> Result<ArtifactDebuginfo, E>
            where
                E: de::Error,
            {
                let debuginfo = match value {
                    "none" => ArtifactDebuginfo::None,
                    "limited" => ArtifactDebuginfo::Limited,
                    "full" => ArtifactDebuginfo::Full,
                    "line-directives-only" => ArtifactDebuginfo::LineDirectivesOnly,
                    "line-tables-only" => ArtifactDebuginfo::LineTablesOnly,
                    s => ArtifactDebuginfo::UnknownString(s.to_string()),
                };
                Ok(debuginfo)
            }

            fn visit_unit<E>(self) -> Result<ArtifactDebuginfo, E>
            where
                E: de::Error,
            {
                Ok(ArtifactDebuginfo::None)
            }
        }

        d.deserialize_any(Visitor)
    }
}

impl fmt::Display for ArtifactDebuginfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArtifactDebuginfo::None => f.write_char('0'),
            ArtifactDebuginfo::Limited => f.write_char('1'),
            ArtifactDebuginfo::Full => f.write_char('2'),
            ArtifactDebuginfo::LineDirectivesOnly => f.write_str("line-directives-only"),
            ArtifactDebuginfo::LineTablesOnly => f.write_str("line-tables-only"),
            ArtifactDebuginfo::UnknownInt(n) => write!(f, "{}", n),
            ArtifactDebuginfo::UnknownString(s) => f.write_str(s),
        }
    }
}

/// A compiler-generated file.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[non_exhaustive]
#[cfg_attr(feature = "builder", builder(pattern = "owned", setter(into)))]
pub struct Artifact {
    /// The package this artifact belongs to
    pub package_id: PackageId,
    /// Path to the `Cargo.toml` file
    #[serde(default)]
    pub manifest_path: Utf8PathBuf,
    /// The target this artifact was compiled for
    pub target: Target,
    /// The profile this artifact was compiled with
    pub profile: ArtifactProfile,
    /// The enabled features for this artifact
    pub features: Vec<String>,
    /// The full paths to the generated artifacts
    /// (e.g. binary file and separate debug info)
    pub filenames: Vec<Utf8PathBuf>,
    /// Path to the executable file
    pub executable: Option<Utf8PathBuf>,
    /// If true, then the files were already generated
    pub fresh: bool,
}

/// Message left by the compiler
// TODO: Better name. This one comes from machine_message.rs
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[non_exhaustive]
#[cfg_attr(feature = "builder", builder(pattern = "owned", setter(into)))]
pub struct CompilerMessage {
    /// The package this message belongs to
    pub package_id: PackageId,
    /// The target this message is aimed at
    pub target: Target,
    /// The message the compiler sent.
    pub message: Diagnostic,
}

/// Output of a build script execution.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[non_exhaustive]
#[cfg_attr(feature = "builder", builder(pattern = "owned", setter(into)))]
pub struct BuildScript {
    /// The package this build script execution belongs to
    pub package_id: PackageId,
    /// The libs to link
    pub linked_libs: Vec<Utf8PathBuf>,
    /// The paths to search when resolving libs
    pub linked_paths: Vec<Utf8PathBuf>,
    /// Various `--cfg` flags to pass to the compiler
    pub cfgs: Vec<String>,
    /// The environment variables to add to the compilation
    pub env: Vec<(String, String)>,
    /// The `OUT_DIR` environment variable where this script places its output
    ///
    /// Added in Rust 1.41.
    #[serde(default)]
    pub out_dir: Utf8PathBuf,
}

/// Final result of a build.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "builder", derive(Builder))]
#[non_exhaustive]
#[cfg_attr(feature = "builder", builder(pattern = "owned", setter(into)))]
pub struct BuildFinished {
    /// Whether or not the build finished successfully.
    pub success: bool,
}

/// A cargo message
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[serde(tag = "reason", rename_all = "kebab-case")]
pub enum Message {
    /// The compiler generated an artifact
    CompilerArtifact(Artifact),
    /// The compiler wants to display a message
    CompilerMessage(CompilerMessage),
    /// A build script successfully executed.
    BuildScriptExecuted(BuildScript),
    /// The build has finished.
    ///
    /// This is emitted at the end of the build as the last message.
    /// Added in Rust 1.44.
    BuildFinished(BuildFinished),
    /// A line of text which isn't a cargo or compiler message.
    /// Line separator is not included
    #[serde(skip)]
    TextLine(String),
}

impl Message {
    /// Creates an iterator of Message from a Read outputting a stream of JSON
    /// messages. For usage information, look at the top-level documentation.
    pub fn parse_stream<R: Read>(input: R) -> MessageIter<R> {
        MessageIter { input }
    }
}

impl fmt::Display for CompilerMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// An iterator of Messages.
pub struct MessageIter<R> {
    input: R,
}

impl<R: BufRead> Iterator for MessageIter<R> {
    type Item = io::Result<Message>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut line = String::new();
        self.input
            .read_line(&mut line)
            .map(|n| {
                if n == 0 {
                    None
                } else {
                    if line.ends_with('\n') {
                        line.truncate(line.len() - 1);
                    }
                    let mut deserializer = serde_json::Deserializer::from_str(&line);
                    deserializer.disable_recursion_limit();
                    Some(Message::deserialize(&mut deserializer).unwrap_or(Message::TextLine(line)))
                }
            })
            .transpose()
    }
}

/// An iterator of Message.
type MessageIterator<R> =
    serde_json::StreamDeserializer<'static, serde_json::de::IoRead<R>, Message>;

/// Creates an iterator of Message from a Read outputting a stream of JSON
/// messages. For usage information, look at the top-level documentation.
#[deprecated(note = "Use Message::parse_stream instead")]
pub fn parse_messages<R: Read>(input: R) -> MessageIterator<R> {
    serde_json::Deserializer::from_reader(input).into_iter::<Message>()
}
