/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::env_config::file::EnvConfigFileKind;
use crate::env_config::parse::{RawProfileSet, WHITESPACE};
use crate::env_config::property::{PropertiesKey, Property};
use crate::env_config::section::{EnvConfigSections, Profile, Section, SsoSession};
use std::borrow::Cow;
use std::collections::HashMap;

const DEFAULT: &str = "default";
const PROFILE_PREFIX: &str = "profile";
const SSO_SESSION_PREFIX: &str = "sso-session";

/// Any section like `[<prefix> <suffix>]` or `[<suffix-only>]`
#[derive(Eq, PartialEq, Hash, Debug)]
struct SectionPair<'a> {
    prefix: Option<Cow<'a, str>>,
    suffix: Cow<'a, str>,
}

impl SectionPair<'_> {
    fn is_unprefixed_default(&self) -> bool {
        self.prefix.is_none() && self.suffix == DEFAULT
    }

    fn is_prefixed_default(&self) -> bool {
        self.prefix.as_deref() == Some(PROFILE_PREFIX) && self.suffix == DEFAULT
    }

    fn parse(input: &str) -> SectionPair<'_> {
        let input = input.trim_matches(WHITESPACE);
        match input.split_once(WHITESPACE) {
            // Something like `[profile name]`
            Some((prefix, suffix)) => SectionPair {
                prefix: Some(prefix.trim().into()),
                suffix: suffix.trim().into(),
            },
            // Either `[profile-name]` or `[default]`
            None => SectionPair {
                prefix: None,
                suffix: input.trim().into(),
            },
        }
    }

    /// Validate a SectionKey for a given file key
    ///
    /// 1. `name` must ALWAYS be a valid identifier
    /// 2. For Config files, the profile must either be `default` or it must have a profile prefix
    /// 3. For credentials files, the profile name MUST NOT have a profile prefix
    /// 4. Only config files can have sections other than `profile` sections
    fn valid_for(self, kind: EnvConfigFileKind) -> Result<Self, String> {
        match kind {
            EnvConfigFileKind::Config => match (&self.prefix, &self.suffix) {
                (Some(prefix), suffix) => {
                    if validate_identifier(suffix).is_ok() {
                        Ok(self)
                    } else {
                        Err(format!("section [{prefix} {suffix}] ignored; `{suffix}` is not a valid identifier"))
                    }
                }
                (None, suffix) => {
                    if self.is_unprefixed_default() {
                        Ok(self)
                    } else {
                        Err(format!("profile [{suffix}] ignored; sections in the AWS config file (other than [default]) must have a prefix i.e. [profile my-profile]"))
                    }
                }
            },
            EnvConfigFileKind::Credentials => match (&self.prefix, &self.suffix) {
                (Some(prefix), suffix) => {
                    if prefix == PROFILE_PREFIX {
                        Err(format!("profile `{suffix}` ignored because credential profiles must NOT begin with `profile`"))
                    } else {
                        Err(format!("section [{prefix} {suffix}] ignored; config must be in the AWS config file rather than the credentials file"))
                    }
                }
                (None, suffix) => {
                    if validate_identifier(suffix).is_ok() {
                        Ok(self)
                    } else {
                        Err(format!(
                            "profile [{suffix}] ignored because `{suffix}` is not a valid identifier",
                        ))
                    }
                }
            },
        }
    }
}

/// Normalize a raw profile into a `MergedProfile`
///
/// This function follows the following rules, codified in the tests & the reference Java implementation
/// - When the profile is a config file, strip `profile` and trim whitespace (`profile foo` => `foo`)
/// - Profile names are validated (see `validate_profile_name`)
/// - A profile named `profile default` takes priority over a profile named `default`.
/// - Profiles with identical names are merged
pub(super) fn merge_in(
    base: &mut EnvConfigSections,
    raw_profile_set: RawProfileSet<'_>,
    kind: EnvConfigFileKind,
) {
    // parse / validate sections
    let validated_sections = raw_profile_set
        .into_iter()
        .map(|(section_key, properties)| {
            (SectionPair::parse(section_key).valid_for(kind), properties)
        });

    // remove invalid profiles & emit a warning
    // valid_sections contains only valid profiles, but it may contain `[profile default]` and `[default]`
    // which must be filtered later
    let valid_sections = validated_sections
        .filter_map(|(section_key, properties)| match section_key {
            Ok(section_key) => Some((section_key, properties)),
            Err(err_str) => {
                tracing::warn!("{err_str}");
                None
            }
        })
        .collect::<Vec<_>>();
    // if a `[profile default]` exists then we should ignore `[default]`
    let ignore_unprefixed_default = valid_sections
        .iter()
        .any(|(section_key, _)| section_key.is_prefixed_default());

    for (section_key, raw_profile) in valid_sections {
        // When normalizing profiles, profiles should be merged. However, `[profile default]` and
        // `[default]` are considered two separate profiles. Furthermore, `[profile default]` fully
        // replaces any contents of `[default]`!
        if ignore_unprefixed_default && section_key.is_unprefixed_default() {
            tracing::warn!("profile `[default]` ignored because `[profile default]` was found which takes priority");
            continue;
        }
        let section: &mut dyn Section = match (
            section_key.prefix.as_deref(),
            section_key.suffix.as_ref(),
        ) {
            (Some(PROFILE_PREFIX), DEFAULT) | (None, DEFAULT) => base
                .profiles
                .entry(DEFAULT.to_string())
                .or_insert_with(|| Profile::new("default", Default::default())),
            (Some(PROFILE_PREFIX), name) | (None, name) => base
                .profiles
                .entry(name.to_string())
                .or_insert_with(|| Profile::new(name.to_string(), Default::default())),
            (Some(SSO_SESSION_PREFIX), name) => base
                .sso_sessions
                .entry(name.to_string())
                .or_insert_with(|| SsoSession::new(name.to_string(), Default::default())),
            (Some(prefix), suffix) => {
                for (sub_properties_group_name, raw_sub_properties) in &raw_profile {
                    match validate_identifier(sub_properties_group_name.as_ref())
                        .map(ToOwned::to_owned)
                    {
                        Ok(sub_properties_group_name) => parse_sub_properties(raw_sub_properties)
                            .for_each(|(sub_property_name, sub_property_value)| {
                                if let Ok(key) = PropertiesKey::builder()
                                    .section_key(prefix)
                                    .section_name(suffix)
                                    .property_name(&sub_properties_group_name)
                                    .sub_property_name(sub_property_name)
                                    .build()
                                {
                                    base.other_sections.insert(key, sub_property_value);
                                }
                            }),
                        Err(_) => {
                            tracing::warn!("`[{prefix} {suffix}].{sub_properties_group_name}` \
                            ignored because `{sub_properties_group_name}` was not a valid identifier");
                        }
                    }
                }

                continue;
            }
        };
        merge_into_base(section, raw_profile)
    }
}

fn merge_into_base(target: &mut dyn Section, profile: HashMap<Cow<'_, str>, Cow<'_, str>>) {
    for (k, v) in profile {
        match validate_identifier(k.as_ref()) {
            Ok(k) => {
                target.insert(k.to_owned(), Property::new(k.to_owned(), v.into()));
            }
            Err(_) => {
                tracing::warn!(profile = %target.name(), key = ?k, "key ignored because `{k}` was not a valid identifier");
            }
        }
    }
}

/// Validate that a string is a valid identifier
///
/// Identifiers must match `[A-Za-z0-9_\-/.%@:\+]+`
fn validate_identifier(input: &str) -> Result<&str, ()> {
    input
        .chars()
        .all(|ch| {
            ch.is_ascii_alphanumeric() || ['_', '-', '/', '.', '%', '@', ':', '+'].contains(&ch)
        })
        .then_some(input)
        .ok_or(())
}

fn parse_sub_properties(sub_properties_str: &str) -> impl Iterator<Item = (String, String)> + '_ {
    sub_properties_str
        .split('\n')
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim_matches(WHITESPACE).to_owned();
                let value = value.trim_matches(WHITESPACE).to_owned();
                Some((key, value))
            } else {
                tracing::warn!("`{line}` ignored because it is not a valid sub-property");
                None
            }
        })
}

#[cfg(test)]
mod tests {
    use crate::env_config::file::EnvConfigFileKind;
    use crate::env_config::normalize::{merge_in, validate_identifier, SectionPair};
    use crate::env_config::parse::RawProfileSet;
    use crate::env_config::section::{EnvConfigSections, Section};
    use std::borrow::Cow;
    use std::collections::HashMap;
    use tracing_test::traced_test;

    #[test]
    fn section_key_parsing() {
        assert_eq!(
            SectionPair {
                prefix: None,
                suffix: "default".into()
            },
            SectionPair::parse("default"),
        );
        assert_eq!(
            SectionPair {
                prefix: None,
                suffix: "default".into()
            },
            SectionPair::parse("   default "),
        );
        assert_eq!(
            SectionPair {
                prefix: Some("profile".into()),
                suffix: "default".into()
            },
            SectionPair::parse("profile default"),
        );
        assert_eq!(
            SectionPair {
                prefix: Some("profile".into()),
                suffix: "default".into()
            },
            SectionPair::parse(" profile   default "),
        );

        assert_eq!(
            SectionPair {
                suffix: "name".into(),
                prefix: Some("profile".into())
            },
            SectionPair::parse("profile name"),
        );
        assert_eq!(
            SectionPair {
                suffix: "name".into(),
                prefix: None
            },
            SectionPair::parse("name"),
        );
        assert_eq!(
            SectionPair {
                suffix: "name".into(),
                prefix: Some("profile".into())
            },
            SectionPair::parse("profile\tname"),
        );
        assert_eq!(
            SectionPair {
                suffix: "name".into(),
                prefix: Some("profile".into())
            },
            SectionPair::parse("profile     name  "),
        );
        assert_eq!(
            SectionPair {
                suffix: "profilename".into(),
                prefix: None
            },
            SectionPair::parse("profilename"),
        );
        assert_eq!(
            SectionPair {
                suffix: "whitespace".into(),
                prefix: None
            },
            SectionPair::parse("   whitespace   "),
        );

        assert_eq!(
            SectionPair {
                prefix: Some("sso-session".into()),
                suffix: "foo".into()
            },
            SectionPair::parse("sso-session foo"),
        );
        assert_eq!(
            SectionPair {
                prefix: Some("sso-session".into()),
                suffix: "foo".into()
            },
            SectionPair::parse("sso-session\tfoo "),
        );
        assert_eq!(
            SectionPair {
                suffix: "sso-sessionfoo".into(),
                prefix: None
            },
            SectionPair::parse("sso-sessionfoo"),
        );
        assert_eq!(
            SectionPair {
                suffix: "sso-session".into(),
                prefix: None
            },
            SectionPair::parse("sso-session "),
        );
    }

    #[test]
    fn test_validate_identifier() {
        assert_eq!(
            Ok("some-thing:long/the_one%only.foo@bar+"),
            validate_identifier("some-thing:long/the_one%only.foo@bar+")
        );
        assert_eq!(Err(()), validate_identifier("foo!bar"));
    }

    #[test]
    #[traced_test]
    fn ignored_key_generates_warning() {
        let mut profile: RawProfileSet<'_> = HashMap::new();
        profile.insert("default", {
            let mut out = HashMap::new();
            out.insert(Cow::Borrowed("invalid key"), "value".into());
            out
        });
        let mut base = EnvConfigSections::default();
        merge_in(&mut base, profile, EnvConfigFileKind::Config);
        assert!(base
            .get_profile("default")
            .expect("contains default profile")
            .is_empty());
        assert!(logs_contain(
            "key ignored because `invalid key` was not a valid identifier"
        ));
    }

    #[test]
    #[traced_test]
    fn invalid_profile_generates_warning() {
        let mut profile: RawProfileSet<'_> = HashMap::new();
        profile.insert("foo", HashMap::new());
        merge_in(
            &mut EnvConfigSections::default(),
            profile,
            EnvConfigFileKind::Config,
        );
        assert!(logs_contain("profile [foo] ignored"));
    }
}
