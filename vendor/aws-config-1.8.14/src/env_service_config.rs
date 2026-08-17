/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_runtime::env_config::section::EnvConfigSections;
use aws_runtime::env_config::EnvConfigValue;
use aws_types::os_shim_internal::Env;
use aws_types::service_config::{LoadServiceConfig, ServiceConfigKey};

#[derive(Debug)]
pub(crate) struct EnvServiceConfig {
    pub(crate) env: Env,
    pub(crate) env_config_sections: EnvConfigSections,
}

impl LoadServiceConfig for EnvServiceConfig {
    fn load_config(&self, key: ServiceConfigKey<'_>) -> Option<String> {
        let (value, _source) = EnvConfigValue::new()
            .env(key.env())
            .profile(key.profile())
            .service_id(key.service_id())
            .load(&self.env, Some(&self.env_config_sections))?;

        Some(value.to_string())
    }
}
