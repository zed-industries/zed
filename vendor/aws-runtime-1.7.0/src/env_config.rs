/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::env_config::property::PropertiesKey;
use crate::env_config::section::EnvConfigSections;
use aws_types::origin::Origin;
use aws_types::os_shim_internal::Env;
use aws_types::service_config::ServiceConfigKey;
use std::borrow::Cow;
use std::error::Error;
use std::fmt;

pub mod error;
pub mod file;
mod normalize;
pub mod parse;
pub mod property;
pub mod section;
pub mod source;

/// Given a key, access to the environment, and a validator, return a config value if one was set.
pub fn get_service_env_config<'a, T, E>(
    key: ServiceConfigKey<'a>,
    env: &'a Env,
    shared_config_sections: Option<&'a EnvConfigSections>,
    validator: impl Fn(&str) -> Result<T, E>,
) -> Result<Option<T>, EnvConfigError<E>>
where
    E: Error + Send + Sync + 'static,
{
    EnvConfigValue::default()
        .env(key.env())
        .profile(key.profile())
        .service_id(key.service_id())
        .validate(env, shared_config_sections, validator)
}

#[derive(Debug)]
enum Location<'a> {
    Environment,
    Profile { name: Cow<'a, str> },
}

impl fmt::Display for Location<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Location::Environment => write!(f, "environment variable"),
            Location::Profile { name } => write!(f, "profile (`{name}`)"),
        }
    }
}

#[derive(Debug)]
enum Scope<'a> {
    Global,
    Service { service_id: Cow<'a, str> },
}

impl fmt::Display for Scope<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Scope::Global => write!(f, "global"),
            Scope::Service { service_id } => write!(f, "service-specific (`{service_id}`)"),
        }
    }
}

/// The source that env config was derived from.
///
/// Includes:
///
/// - Whether some config came from a config file or an env var.
/// - The key used to identify the config value.
///
/// Only used when displaying config-extraction errors.
#[derive(Debug)]
pub struct EnvConfigSource<'a> {
    key: Cow<'a, str>,
    location: Location<'a>,
    scope: Scope<'a>,
}

#[allow(clippy::from_over_into)]
impl Into<Origin> for &EnvConfigSource<'_> {
    fn into(self) -> Origin {
        match (&self.scope, &self.location) {
            (Scope::Global, Location::Environment) => Origin::shared_environment_variable(),
            (Scope::Global, Location::Profile { .. }) => Origin::shared_profile_file(),
            (Scope::Service { .. }, Location::Environment) => {
                Origin::service_environment_variable()
            }
            (Scope::Service { .. }, Location::Profile { .. }) => Origin::service_profile_file(),
        }
    }
}

impl<'a> EnvConfigSource<'a> {
    pub(crate) fn global_from_env(key: Cow<'a, str>) -> Self {
        Self {
            key,
            location: Location::Environment,
            scope: Scope::Global,
        }
    }

    pub(crate) fn global_from_profile(key: Cow<'a, str>, profile_name: Cow<'a, str>) -> Self {
        Self {
            key,
            location: Location::Profile { name: profile_name },
            scope: Scope::Global,
        }
    }

    pub(crate) fn service_from_env(key: Cow<'a, str>, service_id: Cow<'a, str>) -> Self {
        Self {
            key,
            location: Location::Environment,
            scope: Scope::Service { service_id },
        }
    }

    pub(crate) fn service_from_profile(
        key: Cow<'a, str>,
        profile_name: Cow<'a, str>,
        service_id: Cow<'a, str>,
    ) -> Self {
        Self {
            key,
            location: Location::Profile { name: profile_name },
            scope: Scope::Service { service_id },
        }
    }
}

impl fmt::Display for EnvConfigSource<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} key: `{}`", self.scope, self.location, self.key)
    }
}

/// An error occurred when resolving config from a user's environment.
#[derive(Debug)]
pub struct EnvConfigError<E = Box<dyn Error>> {
    property_source: String,
    err: E,
}

impl<E> EnvConfigError<E> {
    /// Return a reference to the inner error wrapped by this error.
    pub fn err(&self) -> &E {
        &self.err
    }
}

impl<E: fmt::Display> fmt::Display for EnvConfigError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}. source: {}", self.err, self.property_source)
    }
}

impl<E: Error> Error for EnvConfigError<E> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.err.source()
    }
}

/// Environment config values are config values sourced from a user's environment variables or profile file.
///
/// `EnvConfigValue` will first look in the environment, then the AWS profile. They track the
/// provenance of properties so that unified validation errors can be created.
#[derive(Default, Debug)]
pub struct EnvConfigValue<'a> {
    environment_variable: Option<Cow<'a, str>>,
    profile_key: Option<Cow<'a, str>>,
    service_id: Option<Cow<'a, str>>,
}

impl<'a> EnvConfigValue<'a> {
    /// Create a new `EnvConfigValue`
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the environment variable to read
    pub fn env(mut self, key: &'a str) -> Self {
        self.environment_variable = Some(Cow::Borrowed(key));
        self
    }

    /// Set the profile key to read
    pub fn profile(mut self, key: &'a str) -> Self {
        self.profile_key = Some(Cow::Borrowed(key));
        self
    }

    /// Set the service id to check for service config
    pub fn service_id(mut self, service_id: &'a str) -> Self {
        self.service_id = Some(Cow::Borrowed(service_id));
        self
    }

    /// Load the value from the env or profile files, validating with `validator`
    pub fn validate<T, E: Error + Send + Sync + 'static>(
        self,
        env: &Env,
        profiles: Option<&EnvConfigSections>,
        validator: impl Fn(&str) -> Result<T, E>,
    ) -> Result<Option<T>, EnvConfigError<E>> {
        let value = self.load(env, profiles);
        value
            .map(|(v, ctx)| {
                validator(v.as_ref()).map_err(|err| EnvConfigError {
                    property_source: format!("{ctx}"),
                    err,
                })
            })
            .transpose()
    }

    /// Load the value from the env or profile files, validating with `validator`
    ///
    /// This version of the function will also return the origin of the config.
    pub fn validate_and_return_origin<T, E: Error + Send + Sync + 'static>(
        self,
        env: &Env,
        profiles: Option<&EnvConfigSections>,
        validator: impl Fn(&str) -> Result<T, E>,
    ) -> Result<(Option<T>, Origin), EnvConfigError<E>> {
        let value = self.load(env, profiles);
        match value {
            Some((v, ctx)) => {
                let origin: Origin = (&ctx).into();
                validator(v.as_ref())
                    .map_err(|err| EnvConfigError {
                        property_source: format!("{ctx}"),
                        err,
                    })
                    .map(|value| (Some(value), origin))
            }
            None => Ok((None, Origin::unknown())),
        }
    }

    /// Load the value from the environment
    pub fn load(
        &self,
        env: &'a Env,
        profiles: Option<&'a EnvConfigSections>,
    ) -> Option<(Cow<'a, str>, EnvConfigSource<'a>)> {
        let env_value = self.environment_variable.as_ref().and_then(|env_var| {
            // Check for a service-specific env var first
            let service_config =
                get_service_config_from_env(env, self.service_id.clone(), env_var.clone());
            // Then check for a global env var
            let global_config = env.get(env_var).ok().map(|value| {
                (
                    Cow::Owned(value),
                    EnvConfigSource::global_from_env(env_var.clone()),
                )
            });

            if let Some(v) = service_config {
                tracing::trace!("(service env) {env_var} = {v:?}");
                Some(v)
            } else if let Some(v) = global_config {
                tracing::trace!("(global env) {env_var} = {v:?}");
                Some(v)
            } else {
                tracing::trace!("(env) no value set for {env_var}");
                None
            }
        });

        let profile_value = match (profiles, self.profile_key.as_ref()) {
            (Some(profiles), Some(profile_key)) => {
                // Check for a service-specific profile key first
                let service_config = get_service_config_from_profile(
                    profiles,
                    self.service_id.clone(),
                    profile_key.clone(),
                );
                let global_config = profiles.get(profile_key.as_ref()).map(|value| {
                    (
                        Cow::Borrowed(value),
                        EnvConfigSource::global_from_profile(
                            profile_key.clone(),
                            Cow::Owned(profiles.selected_profile().to_owned()),
                        ),
                    )
                });

                if let Some(v) = service_config {
                    tracing::trace!("(service profile) {profile_key} = {v:?}");
                    Some(v)
                } else if let Some(v) = global_config {
                    tracing::trace!("(global profile) {profile_key} = {v:?}");
                    Some(v)
                } else {
                    tracing::trace!("(service profile) no value set for {profile_key}");
                    None
                }
            }
            _ => None,
        };

        env_value.or(profile_value)
    }
}

fn get_service_config_from_env<'a>(
    env: &'a Env,
    service_id: Option<Cow<'a, str>>,
    env_var: Cow<'a, str>,
) -> Option<(Cow<'a, str>, EnvConfigSource<'a>)> {
    let service_id = service_id?;
    let env_case_service_id = format_service_id_for_env(service_id.clone());
    let service_specific_env_key = format!("{env_var}_{env_case_service_id}");
    let env_var = env.get(&service_specific_env_key).ok()?;
    let env_var: Cow<'_, str> = Cow::Owned(env_var);
    let source = EnvConfigSource::service_from_env(env_var.clone(), service_id);

    Some((env_var, source))
}

const SERVICES: &str = "services";

fn get_service_config_from_profile<'a>(
    profile: &EnvConfigSections,
    service_id: Option<Cow<'a, str>>,
    profile_key: Cow<'a, str>,
) -> Option<(Cow<'a, str>, EnvConfigSource<'a>)> {
    let service_id = service_id?.clone();
    let profile_case_service_id = format_service_id_for_profile(service_id.clone());
    let services_section_name = profile.get(SERVICES)?;
    let properties_key = PropertiesKey::builder()
        .section_key(SERVICES)
        .section_name(services_section_name)
        .property_name(profile_case_service_id)
        .sub_property_name(profile_key.clone())
        .build()
        .ok()?;
    let value = profile.other_sections().get(&properties_key)?;
    let profile_name = Cow::Owned(profile.selected_profile().to_owned());
    let source = EnvConfigSource::service_from_profile(profile_key, profile_name, service_id);

    Some((Cow::Owned(value.to_owned()), source))
}

fn format_service_id_for_env(service_id: impl AsRef<str>) -> String {
    service_id.as_ref().to_uppercase().replace(' ', "_")
}

fn format_service_id_for_profile(service_id: impl AsRef<str>) -> String {
    service_id.as_ref().to_lowercase().replace(' ', "-")
}

#[cfg(test)]
mod test {
    use crate::env_config::property::{Properties, PropertiesKey};
    use crate::env_config::section::EnvConfigSections;
    use aws_types::os_shim_internal::Env;
    use std::borrow::Cow;
    use std::collections::HashMap;
    use std::num::ParseIntError;

    use super::EnvConfigValue;

    fn validate_some_key(s: &str) -> Result<i32, ParseIntError> {
        s.parse()
    }

    fn new_prop_key(
        section_key: impl Into<String>,
        section_name: impl Into<String>,
        property_name: impl Into<String>,
        sub_property_name: Option<impl Into<String>>,
    ) -> PropertiesKey {
        let mut builder = PropertiesKey::builder()
            .section_key(section_key)
            .section_name(section_name)
            .property_name(property_name);

        if let Some(sub_property_name) = sub_property_name {
            builder = builder.sub_property_name(sub_property_name);
        }

        builder.build().unwrap()
    }

    #[tokio::test]
    async fn test_service_config_multiple_services() {
        let env = Env::from_slice(&[
            ("AWS_CONFIG_FILE", "config"),
            ("AWS_SOME_KEY", "1"),
            ("AWS_SOME_KEY_SERVICE", "2"),
            ("AWS_SOME_KEY_ANOTHER_SERVICE", "3"),
        ]);
        let profiles = EnvConfigSections::new(
            HashMap::from([(
                "default".to_owned(),
                HashMap::from([
                    ("some_key".to_owned(), "4".to_owned()),
                    ("services".to_owned(), "dev".to_owned()),
                ]),
            )]),
            Cow::Borrowed("default"),
            HashMap::new(),
            Properties::new_from_slice(&[
                (
                    new_prop_key("services", "dev", "service", Some("some_key")),
                    "5".to_string(),
                ),
                (
                    new_prop_key("services", "dev", "another_service", Some("some_key")),
                    "6".to_string(),
                ),
            ]),
        );
        let profiles = Some(&profiles);
        let global_from_env = EnvConfigValue::new()
            .env("AWS_SOME_KEY")
            .profile("some_key")
            .validate(&env, profiles, validate_some_key)
            .expect("config resolution succeeds");
        assert_eq!(Some(1), global_from_env);

        let service_from_env = EnvConfigValue::new()
            .env("AWS_SOME_KEY")
            .profile("some_key")
            .service_id("service")
            .validate(&env, profiles, validate_some_key)
            .expect("config resolution succeeds");
        assert_eq!(Some(2), service_from_env);

        let other_service_from_env = EnvConfigValue::new()
            .env("AWS_SOME_KEY")
            .profile("some_key")
            .service_id("another_service")
            .validate(&env, profiles, validate_some_key)
            .expect("config resolution succeeds");
        assert_eq!(Some(3), other_service_from_env);

        let global_from_profile = EnvConfigValue::new()
            .profile("some_key")
            .validate(&env, profiles, validate_some_key)
            .expect("config resolution succeeds");
        assert_eq!(Some(4), global_from_profile);

        let service_from_profile = EnvConfigValue::new()
            .profile("some_key")
            .service_id("service")
            .validate(&env, profiles, validate_some_key)
            .expect("config resolution succeeds");
        assert_eq!(Some(5), service_from_profile);

        let service_from_profile = EnvConfigValue::new()
            .profile("some_key")
            .service_id("another_service")
            .validate(&env, profiles, validate_some_key)
            .expect("config resolution succeeds");
        assert_eq!(Some(6), service_from_profile);
    }

    #[tokio::test]
    async fn test_service_config_precedence() {
        let env = Env::from_slice(&[
            ("AWS_CONFIG_FILE", "config"),
            ("AWS_SOME_KEY", "1"),
            ("AWS_SOME_KEY_S3", "2"),
        ]);

        let profiles = EnvConfigSections::new(
            HashMap::from([(
                "default".to_owned(),
                HashMap::from([
                    ("some_key".to_owned(), "3".to_owned()),
                    ("services".to_owned(), "dev".to_owned()),
                ]),
            )]),
            Cow::Borrowed("default"),
            HashMap::new(),
            Properties::new_from_slice(&[(
                new_prop_key("services", "dev", "s3", Some("some_key")),
                "4".to_string(),
            )]),
        );
        let profiles = Some(&profiles);
        let global_from_env = EnvConfigValue::new()
            .env("AWS_SOME_KEY")
            .profile("some_key")
            .validate(&env, profiles, validate_some_key)
            .expect("config resolution succeeds");
        assert_eq!(Some(1), global_from_env);

        let service_from_env = EnvConfigValue::new()
            .env("AWS_SOME_KEY")
            .profile("some_key")
            .service_id("s3")
            .validate(&env, profiles, validate_some_key)
            .expect("config resolution succeeds");
        assert_eq!(Some(2), service_from_env);

        let global_from_profile = EnvConfigValue::new()
            .profile("some_key")
            .validate(&env, profiles, validate_some_key)
            .expect("config resolution succeeds");
        assert_eq!(Some(3), global_from_profile);

        let service_from_profile = EnvConfigValue::new()
            .profile("some_key")
            .service_id("s3")
            .validate(&env, profiles, validate_some_key)
            .expect("config resolution succeeds");
        assert_eq!(Some(4), service_from_profile);
    }

    #[tokio::test]
    async fn test_multiple_services() {
        let env = Env::from_slice(&[
            ("AWS_CONFIG_FILE", "config"),
            ("AWS_SOME_KEY", "1"),
            ("AWS_SOME_KEY_S3", "2"),
            ("AWS_SOME_KEY_EC2", "3"),
        ]);

        let profiles = EnvConfigSections::new(
            HashMap::from([(
                "default".to_owned(),
                HashMap::from([
                    ("some_key".to_owned(), "4".to_owned()),
                    ("services".to_owned(), "dev".to_owned()),
                ]),
            )]),
            Cow::Borrowed("default"),
            HashMap::new(),
            Properties::new_from_slice(&[
                (
                    new_prop_key("services", "dev-wrong", "s3", Some("some_key")),
                    "998".into(),
                ),
                (
                    new_prop_key("services", "dev-wrong", "ec2", Some("some_key")),
                    "999".into(),
                ),
                (
                    new_prop_key("services", "dev", "s3", Some("some_key")),
                    "5".into(),
                ),
                (
                    new_prop_key("services", "dev", "ec2", Some("some_key")),
                    "6".into(),
                ),
            ]),
        );
        let profiles = Some(&profiles);
        let global_from_env = EnvConfigValue::new()
            .env("AWS_SOME_KEY")
            .profile("some_key")
            .validate(&env, profiles, validate_some_key)
            .expect("config resolution succeeds");
        assert_eq!(Some(1), global_from_env);

        let service_from_env = EnvConfigValue::new()
            .env("AWS_SOME_KEY")
            .profile("some_key")
            .service_id("s3")
            .validate(&env, profiles, validate_some_key)
            .expect("config resolution succeeds");
        assert_eq!(Some(2), service_from_env);

        let service_from_env = EnvConfigValue::new()
            .env("AWS_SOME_KEY")
            .profile("some_key")
            .service_id("ec2")
            .validate(&env, profiles, validate_some_key)
            .expect("config resolution succeeds");
        assert_eq!(Some(3), service_from_env);

        let global_from_profile = EnvConfigValue::new()
            .profile("some_key")
            .validate(&env, profiles, validate_some_key)
            .expect("config resolution succeeds");
        assert_eq!(Some(4), global_from_profile);

        let service_from_profile = EnvConfigValue::new()
            .profile("some_key")
            .service_id("s3")
            .validate(&env, profiles, validate_some_key)
            .expect("config resolution succeeds");
        assert_eq!(Some(5), service_from_profile);

        let service_from_profile = EnvConfigValue::new()
            .profile("some_key")
            .service_id("ec2")
            .validate(&env, profiles, validate_some_key)
            .expect("config resolution succeeds");
        assert_eq!(Some(6), service_from_profile);
    }
}
