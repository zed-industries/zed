// Parse system-deps metadata from Cargo.toml

use std::{fmt, fs, io::Read, path::Path};

use toml::de::{DeArray, DeTable, DeValue};
use toml::Spanned;

#[derive(Debug, PartialEq)]
pub(crate) struct MetaData {
    pub(crate) deps: Vec<Dependency>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Dependency {
    pub(crate) key: String,
    pub(crate) version: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) fallback_names: Option<Vec<String>>,
    pub(crate) feature: Option<String>,
    pub(crate) optional: bool,
    pub(crate) cfg: Option<cfg_expr::Expression>,
    pub(crate) version_overrides: Vec<VersionOverride>,
}

impl Dependency {
    fn new(name: &str) -> Self {
        Self {
            key: name.to_string(),
            ..Default::default()
        }
    }

    pub(crate) fn lib_name(&self) -> &str {
        self.name.as_ref().unwrap_or(&self.key)
    }
}

impl Default for Dependency {
    fn default() -> Self {
        Self {
            key: "".to_string(),
            version: None,
            name: None,
            fallback_names: None,
            feature: None,
            optional: false,
            cfg: None,
            version_overrides: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq)]
enum VersionOverrideBuilderError {
    MissingVersionField,
}

impl fmt::Display for VersionOverrideBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::MissingVersionField => write!(f, "missing version field"),
        }
    }
}

impl std::error::Error for VersionOverrideBuilderError {}

#[derive(Debug, PartialEq)]
enum MetadataError {
    MissingKey(String),
    NotATable(String),
    NestedCfg(String),
    NotStringOrTable(String),
    NotString(String),
    CfgExpr(cfg_expr::ParseError),
    Toml(toml::de::Error),
    UnexpectedVersionSetting(String, String, String),
    UnexpectedKey(String, String, String),
    VersionOverrideBuilder(VersionOverrideBuilderError),
}

impl fmt::Display for MetadataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingKey(k) => write!(f, "missing key `{k}`"),
            Self::NotATable(k) => write!(f, "`{k}` is not a table"),
            Self::NestedCfg(k) => write!(f, "`{k}`: cfg() cannot be nested"),
            Self::NotString(k) => write!(f, "`{k}`: not a string"),
            Self::NotStringOrTable(k) => write!(f, "`{k}`: not a string or a table"),
            Self::CfgExpr(e) => write!(f, "{e}"),
            Self::Toml(e) => write!(f, "error parsing TOML: {e}"),
            Self::UnexpectedVersionSetting(n, k, t) => {
                write!(f, "{n}: unexpected version settings key: {k} type: {t}")
            }
            Self::UnexpectedKey(n, k, t) => write!(f, "{n}: unexpected key {k} type {t}"),
            Self::VersionOverrideBuilder(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for MetadataError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::CfgExpr(e) => Some(e),
            Self::Toml(e) => Some(e),
            Self::VersionOverrideBuilder(e) => Some(e),
            _ => None,
        }
    }
}

impl From<cfg_expr::ParseError> for MetadataError {
    fn from(err: cfg_expr::ParseError) -> Self {
        Self::CfgExpr(err)
    }
}

impl From<toml::de::Error> for MetadataError {
    fn from(err: toml::de::Error) -> Self {
        Self::Toml(err)
    }
}

impl From<VersionOverrideBuilderError> for MetadataError {
    fn from(err: VersionOverrideBuilderError) -> Self {
        Self::VersionOverrideBuilder(err)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct VersionOverride {
    pub(crate) key: String,
    pub(crate) version: String,
    pub(crate) name: Option<String>,
    pub(crate) fallback_names: Option<Vec<String>>,
    pub(crate) optional: Option<bool>,
}

struct VersionOverrideBuilder {
    version_id: String,
    version: Option<String>,
    full_name: Option<String>,
    fallback_names: Option<Vec<String>>,
    optional: Option<bool>,
}

impl VersionOverrideBuilder {
    fn new(version_id: &str) -> Self {
        Self {
            version_id: version_id.to_string(),
            version: None,
            full_name: None,
            fallback_names: None,
            optional: None,
        }
    }

    fn build(self) -> Result<VersionOverride, VersionOverrideBuilderError> {
        let version = self
            .version
            .ok_or(VersionOverrideBuilderError::MissingVersionField)?;

        Ok(VersionOverride {
            key: self.version_id,
            version,
            name: self.full_name,
            fallback_names: self.fallback_names,
            optional: self.optional,
        })
    }
}

impl MetaData {
    pub(crate) fn from_file(path: &Path) -> Result<Self, crate::Error> {
        let mut manifest = fs::File::open(path).map_err(|e| {
            crate::Error::FailToRead(format!("error opening {}", path.display()), e)
        })?;

        let mut manifest_str = String::new();
        manifest.read_to_string(&mut manifest_str).map_err(|e| {
            crate::Error::FailToRead(format!("error reading {}", path.display()), e)
        })?;

        Self::from_str(manifest_str)
            .map_err(|e| crate::Error::InvalidMetadata(format!("{}: {}", path.display(), e)))
    }

    fn from_str(manifest_str: String) -> Result<Self, MetadataError> {
        let toml = DeTable::parse(&manifest_str)?;
        let key = "package.metadata.system-deps";
        let meta = toml
            .get_ref()
            .get("package")
            .and_then(|v| v.get_ref().get("metadata"))
            .and_then(|v| v.get_ref().get("system-deps"))
            .ok_or_else(|| MetadataError::MissingKey(key.to_owned()))?;

        let deps = Self::parse_deps_table(meta, key, true)?;

        Ok(MetaData { deps })
    }

    fn parse_deps_table(
        table: &Spanned<DeValue<'_>>,
        key: &str,
        allow_cfg: bool,
    ) -> Result<Vec<Dependency>, MetadataError> {
        let table = table
            .get_ref()
            .as_table()
            .ok_or_else(|| MetadataError::NotATable(key.to_owned()))?;

        let mut deps = Vec::new();

        for (name_spanned, value) in table {
            let name = name_spanned.as_ref();
            if name.starts_with("cfg(") {
                if allow_cfg {
                    let cfg_exp = cfg_expr::Expression::parse(name)?;

                    for mut dep in Self::parse_deps_table(value, &format!("{key}.{name}"), false)? {
                        dep.cfg = Some(cfg_exp.clone());
                        deps.push(dep);
                    }
                } else {
                    return Err(MetadataError::NestedCfg(format!("{key}.{name}")));
                }
            } else {
                let dep = Self::parse_dep(key, name, value)?;
                deps.push(dep);
            }
        }

        Ok(deps)
    }

    fn parse_dep(
        key: &str,
        name: &str,
        value: &Spanned<DeValue<'_>>,
    ) -> Result<Dependency, MetadataError> {
        let mut dep = Dependency::new(name);

        match value.as_ref() {
            // somelib = "1.0"
            DeValue::String(ref s) => {
                if !validate_version(s) {
                    return Err(MetadataError::UnexpectedVersionSetting(
                        key.into(),
                        name.into(),
                        value.as_ref().type_str().to_owned(),
                    ));
                }

                dep.version = Some(s.clone().into_owned());
            }
            DeValue::Table(ref t) => {
                Self::parse_dep_table(key, name, &mut dep, t)?;
            }
            _ => {
                return Err(MetadataError::NotStringOrTable(format!("{key}.{name}")));
            }
        }

        Ok(dep)
    }

    fn parse_dep_table(
        p_key: &str,
        name: &str,
        dep: &mut Dependency,
        t: &DeTable<'_>,
    ) -> Result<(), MetadataError> {
        for (key_spanned, value) in t {
            let key = key_spanned.as_ref().as_ref();
            match (key, value.as_ref()) {
                ("feature", DeValue::String(s)) => {
                    dep.feature = Some(s.clone().into_owned());
                }
                ("version", DeValue::String(s)) => {
                    if !validate_version(s) {
                        return Err(MetadataError::UnexpectedVersionSetting(
                            format!("{p_key}.{name}"),
                            key.into(),
                            value.as_ref().type_str().to_owned(),
                        ));
                    }

                    dep.version = Some(s.clone().into_owned());
                }
                ("name", DeValue::String(s)) => {
                    dep.name = Some(s.clone().into_owned());
                }
                ("fallback-names", DeValue::Array(values)) => {
                    let key = format!("{p_key}.{name}");
                    dep.fallback_names = Some(Self::parse_name_list(&key, values)?);
                }
                ("optional", &DeValue::Boolean(optional)) => {
                    dep.optional = optional;
                }
                (version_feature, DeValue::Table(version_settings))
                    if version_feature.starts_with('v') =>
                {
                    let mut builder = VersionOverrideBuilder::new(version_feature);

                    for (k_spanned, v) in version_settings {
                        let k = k_spanned.as_ref().as_ref();
                        match (k, v.as_ref()) {
                            ("version", DeValue::String(feat_vers)) => {
                                if !validate_version(feat_vers) {
                                    return Err(MetadataError::UnexpectedVersionSetting(
                                        format!("{p_key}.{name}"),
                                        k.into(),
                                        v.as_ref().type_str().to_owned(),
                                    ));
                                }

                                builder.version = Some(feat_vers.clone().into_owned());
                            }
                            ("name", DeValue::String(feat_name)) => {
                                builder.full_name = Some(feat_name.clone().into_owned());
                            }
                            ("fallback-names", DeValue::Array(values)) => {
                                let key = format!("{p_key}.{name}.{version_feature}");
                                builder.fallback_names = Some(Self::parse_name_list(&key, values)?);
                            }
                            ("optional", &DeValue::Boolean(optional)) => {
                                builder.optional = Some(optional);
                            }
                            _ => {
                                return Err(MetadataError::UnexpectedVersionSetting(
                                    format!("{p_key}.{name}"),
                                    k.to_owned(),
                                    v.as_ref().type_str().to_owned(),
                                ));
                            }
                        }
                    }

                    dep.version_overrides.push(builder.build()?);
                }
                _ => {
                    return Err(MetadataError::UnexpectedKey(
                        format!("{p_key}.{name}"),
                        key.to_owned(),
                        value.as_ref().type_str().to_owned(),
                    ));
                }
            }
        }
        Ok(())
    }

    fn parse_name_list(key: &str, values: &DeArray<'_>) -> Result<Vec<String>, MetadataError> {
        values
            .iter()
            .enumerate()
            .map(|(i, value)| {
                value
                    .as_ref()
                    .as_str()
                    .map(|x| x.to_owned())
                    .ok_or_else(|| MetadataError::NotString(format!("{key}[{i}]")))
            })
            .collect()
    }
}

fn validate_version(version: &str) -> bool {
    if let Some((min, max)) = version.split_once(',') {
        if !min.trim_start().starts_with(">=") || !max.trim_start().starts_with('<') {
            return false;
        }

        true
    } else {
        true
    }
}

#[derive(Debug, Clone)]
pub(crate) enum VersionRange<'a> {
    Range(std::ops::Range<&'a str>),
    RangeFrom(std::ops::RangeFrom<&'a str>),
}

impl<'a> std::ops::RangeBounds<&'a str> for VersionRange<'a> {
    fn start_bound(&self) -> std::ops::Bound<&&'a str> {
        match self {
            VersionRange::Range(r) => r.start_bound(),
            VersionRange::RangeFrom(r) => r.start_bound(),
        }
    }

    fn end_bound(&self) -> std::ops::Bound<&&'a str> {
        match self {
            VersionRange::Range(r) => r.end_bound(),
            VersionRange::RangeFrom(r) => r.end_bound(),
        }
    }
}

pub(crate) fn parse_version(version: &str) -> VersionRange<'_> {
    if let Some((min, max)) = version.split_once(',') {
        // Format checked when parsing
        let min = min.trim_start().strip_prefix(">=").unwrap().trim();
        let max = max.trim_start().strip_prefix('<').unwrap().trim();
        VersionRange::Range(min..max)
    } else if let Some(min) = version.trim_start().strip_prefix(">=") {
        VersionRange::RangeFrom(min..)
    } else {
        VersionRange::RangeFrom(version..)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_matches::assert_matches;
    use cfg_expr::Expression;
    use std::{env, path::PathBuf};

    fn parse_file(dir: &str) -> Result<MetaData, crate::Error> {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let mut p: PathBuf = manifest_dir.into();
        p.push("src");
        p.push("tests");
        p.push(dir);
        p.push("Cargo.toml");
        assert!(p.exists());

        MetaData::from_file(&p)
    }

    #[test]
    fn parse_good() {
        let m = parse_file("toml-good").unwrap();

        assert_eq!(
            m,
            MetaData {
                deps: vec![
                    Dependency {
                        key: "testdata".into(),
                        version: Some("4".into()),
                        ..Default::default()
                    },
                    Dependency {
                        key: "testlib".into(),
                        version: Some("1".into()),
                        feature: Some("test-feature".into()),
                        ..Default::default()
                    },
                    Dependency {
                        key: "testmore".into(),
                        version: Some("2".into()),
                        feature: Some("another-test-feature".into()),
                        ..Default::default()
                    }
                ]
            }
        )
    }

    #[test]
    fn parse_feature_not_string() {
        assert_matches!(
            parse_file("toml-feature-not-string"),
            Err(crate::Error::InvalidMetadata(_))
        );
    }

    #[test]
    fn parse_override_name() {
        let m = parse_file("toml-override-name").unwrap();

        assert_eq!(
            m,
            MetaData {
                deps: vec![Dependency {
                    key: "test_lib".into(),
                    version: Some("1.0".into()),
                    name: Some("testlib".into()),
                    version_overrides: vec![VersionOverride {
                        key: "v1_2".into(),
                        version: "1.2".into(),
                        name: None,
                        fallback_names: None,
                        optional: None,
                    }],
                    ..Default::default()
                },]
            }
        )
    }

    #[test]
    fn parse_feature_versions() {
        let m = parse_file("toml-feature-versions").unwrap();

        assert_eq!(
            m,
            MetaData {
                deps: vec![Dependency {
                    key: "testdata".into(),
                    version: Some("4".into()),
                    version_overrides: vec![
                        VersionOverride {
                            key: "v5".into(),
                            version: "5".into(),
                            name: None,
                            fallback_names: None,
                            optional: None,
                        },
                        VersionOverride {
                            key: "v6".into(),
                            version: "6".into(),
                            name: None,
                            fallback_names: None,
                            optional: None,
                        },
                    ],
                    ..Default::default()
                },]
            }
        )
    }

    #[test]
    fn parse_fallback_names() {
        let m = parse_file("toml-fallback-names").unwrap();

        assert_eq!(
            m,
            MetaData {
                deps: vec![Dependency {
                    key: "test_lib".into(),
                    version: Some("1.0".into()),
                    name: Some("nosuchlib".into()),
                    fallback_names: Some(vec![
                        "also-no-such-lib".into(),
                        "testlib".into(),
                        "should-not-get-here".into(),
                    ]),
                    version_overrides: vec![],
                    ..Default::default()
                }]
            }
        )
    }

    #[test]
    fn parse_version_fallback_names() {
        let m = parse_file("toml-version-fallback-names").unwrap();

        assert_eq!(
            m,
            MetaData {
                deps: vec![Dependency {
                    key: "test_lib".into(),
                    version: Some("0.1".into()),
                    name: Some("nosuchlib".into()),
                    fallback_names: Some(vec![
                        "also-no-such-lib".into(),
                        "testlib".into(),
                        "should-not-get-here".into(),
                    ]),
                    version_overrides: vec![
                        VersionOverride {
                            key: "v1".into(),
                            version: "1.0".into(),
                            name: None,
                            fallback_names: None,
                            optional: None,
                        },
                        VersionOverride {
                            key: "v2".into(),
                            version: "2.0".into(),
                            name: None,
                            fallback_names: Some(vec!["testlib-2.0".into()]),
                            optional: None,
                        },
                        VersionOverride {
                            key: "v99".into(),
                            version: "99.0".into(),
                            name: None,
                            fallback_names: Some(vec![]),
                            optional: None,
                        },
                    ],
                    ..Default::default()
                }]
            }
        )
    }

    #[test]
    fn parse_optional() {
        let m = parse_file("toml-optional").unwrap();

        assert_eq!(
            m,
            MetaData {
                deps: vec![
                    Dependency {
                        key: "testbadger".into(),
                        version: Some("1".into()),
                        optional: true,
                        ..Default::default()
                    },
                    Dependency {
                        key: "testlib".into(),
                        version: Some("1.0".into()),
                        optional: true,
                        version_overrides: vec![VersionOverride {
                            key: "v5".into(),
                            version: "5.0".into(),
                            name: Some("testlib-5.0".into()),
                            fallback_names: None,
                            optional: Some(false),
                        },],
                        ..Default::default()
                    },
                    Dependency {
                        key: "testmore".into(),
                        version: Some("2".into()),
                        version_overrides: vec![VersionOverride {
                            key: "v3".into(),
                            version: "3.0".into(),
                            name: None,
                            fallback_names: None,
                            optional: Some(true),
                        },],
                        ..Default::default()
                    },
                ]
            }
        )
    }

    #[test]
    fn parse_os_specific() {
        let m = parse_file("toml-os-specific").unwrap();

        assert_eq!(
            m,
            MetaData {
                deps: vec![
                    Dependency {
                        key: "testlib".into(),
                        version: Some("1".into()),
                        cfg: Some(Expression::parse("not(target_os = \"macos\")").unwrap()),
                        ..Default::default()
                    },
                    Dependency {
                        key: "testdata".into(),
                        version: Some("1".into()),
                        cfg: Some(Expression::parse("target_os = \"linux\"").unwrap()),
                        ..Default::default()
                    },
                    Dependency {
                        key: "testanotherlib".into(),
                        version: Some("1".into()),
                        cfg: Some(Expression::parse("unix").unwrap()),
                        optional: true,
                        ..Default::default()
                    },
                ]
            }
        )
    }
}
