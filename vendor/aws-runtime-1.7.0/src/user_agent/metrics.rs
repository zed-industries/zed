/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::sdk_feature::AwsSdkFeature;
use aws_credential_types::credential_feature::AwsCredentialFeature;
use aws_smithy_runtime::client::sdk_feature::SmithySdkFeature;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::sync::LazyLock;

const MAX_COMMA_SEPARATED_METRICS_VALUES_LENGTH: usize = 1024;
#[allow(dead_code)]
const MAX_METRICS_ID_NUMBER: usize = 350;

macro_rules! iterable_enum {
    ($docs:tt, $enum_name:ident, $( $variant:ident ),*) => {
        #[derive(Clone, Debug, Eq, Hash, PartialEq)]
        #[non_exhaustive]
        #[doc = $docs]
        #[allow(missing_docs)] // for variants, not for the Enum itself
        pub enum $enum_name {
            $( $variant ),*
        }

        #[allow(dead_code)]
        impl $enum_name {
            pub(crate) fn iter() -> impl Iterator<Item = &'static $enum_name> {
                const VARIANTS: &[$enum_name] = &[
                    $( $enum_name::$variant ),*
                ];
                VARIANTS.iter()
            }
        }
    };
}

struct Base64Iterator {
    current: Vec<usize>,
    base64_chars: Vec<char>,
}

impl Base64Iterator {
    #[allow(dead_code)]
    fn new() -> Self {
        Base64Iterator {
            current: vec![0], // Start with the first character
            base64_chars: (b'A'..=b'Z') // 'A'-'Z'
                .chain(b'a'..=b'z') // 'a'-'z'
                .chain(b'0'..=b'9') // '0'-'9'
                .chain([b'+', b'-']) // '+' and '-'
                .map(|c| c as char)
                .collect(),
        }
    }

    fn increment(&mut self) {
        let mut i = 0;
        while i < self.current.len() {
            self.current[i] += 1;
            if self.current[i] < self.base64_chars.len() {
                // The value at current position hasn't reached 64
                return;
            }
            self.current[i] = 0;
            i += 1;
        }
        self.current.push(0); // Add new digit if all positions overflowed
    }
}

impl Iterator for Base64Iterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_empty() {
            return None; // No more items
        }

        // Convert the current indices to characters
        let result: String = self
            .current
            .iter()
            .rev()
            .map(|&idx| self.base64_chars[idx])
            .collect();

        // Increment to the next value
        self.increment();
        Some(result)
    }
}

pub(super) static FEATURE_ID_TO_METRIC_VALUE: LazyLock<HashMap<BusinessMetric, Cow<'static, str>>> =
    LazyLock::new(|| {
        let mut m = HashMap::new();
        for (metric, value) in BusinessMetric::iter()
            .cloned()
            .zip(Base64Iterator::new())
            .take(MAX_METRICS_ID_NUMBER)
        {
            m.insert(metric, Cow::Owned(value));
        }
        m
    });

iterable_enum!(
    "Enumerates human readable identifiers for the features tracked by metrics",
    BusinessMetric,
    ResourceModel,
    Waiter,
    Paginator,
    RetryModeLegacy,
    RetryModeStandard,
    RetryModeAdaptive,
    S3Transfer,
    S3CryptoV1n,
    S3CryptoV2,
    S3ExpressBucket,
    S3AccessGrants,
    GzipRequestCompression,
    ProtocolRpcV2Cbor,
    EndpointOverride,
    AccountIdEndpoint,
    AccountIdModePreferred,
    AccountIdModeDisabled,
    AccountIdModeRequired,
    Sigv4aSigning,
    ResolvedAccountId,
    FlexibleChecksumsReqCrc32,
    FlexibleChecksumsReqCrc32c,
    FlexibleChecksumsReqCrc64,
    FlexibleChecksumsReqSha1,
    FlexibleChecksumsReqSha256,
    FlexibleChecksumsReqWhenSupported,
    FlexibleChecksumsReqWhenRequired,
    FlexibleChecksumsResWhenSupported,
    FlexibleChecksumsResWhenRequired,
    DdbMapper,
    CredentialsCode,
    CredentialsJvmSystemProperties,
    CredentialsEnvVars,
    CredentialsEnvVarsStsWebIdToken,
    CredentialsStsAssumeRole,
    CredentialsStsAssumeRoleSaml,
    CredentialsStsAssumeRoleWebId,
    CredentialsStsFederationToken,
    CredentialsStsSessionToken,
    CredentialsProfile,
    CredentialsProfileSourceProfile,
    CredentialsProfileNamedProvider,
    CredentialsProfileStsWebIdToken,
    CredentialsProfileSso,
    CredentialsSso,
    CredentialsProfileSsoLegacy,
    CredentialsSsoLegacy,
    CredentialsProfileProcess,
    CredentialsProcess,
    CredentialsBoto2ConfigFile,
    CredentialsAwsSdkStore,
    CredentialsHttp,
    CredentialsImds,
    SsoLoginDevice,
    SsoLoginAuth,
    BearerServiceEnvVars,
    ObservabilityTracing,
    ObservabilityMetrics,
    ObservabilityOtelTracing,
    ObservabilityOtelMetrics,
    CredentialsCognito,
    S3TransferUploadDirectory,
    S3TransferDownloadDirectory,
    CliV1ToV2MigrationDebugMode,
    LoginSameDevice,
    LoginCrossDevice,
    CredentialsProfileLogin,
    CredentialsLogin
);

pub(crate) trait ProvideBusinessMetric {
    fn provide_business_metric(&self) -> Option<BusinessMetric>;
}

impl ProvideBusinessMetric for SmithySdkFeature {
    fn provide_business_metric(&self) -> Option<BusinessMetric> {
        use SmithySdkFeature::*;
        match self {
            Waiter => Some(BusinessMetric::Waiter),
            Paginator => Some(BusinessMetric::Paginator),
            GzipRequestCompression => Some(BusinessMetric::GzipRequestCompression),
            ProtocolRpcV2Cbor => Some(BusinessMetric::ProtocolRpcV2Cbor),
            RetryModeStandard => Some(BusinessMetric::RetryModeStandard),
            RetryModeAdaptive => Some(BusinessMetric::RetryModeAdaptive),
            FlexibleChecksumsReqCrc32 => Some(BusinessMetric::FlexibleChecksumsReqCrc32),
            FlexibleChecksumsReqCrc32c => Some(BusinessMetric::FlexibleChecksumsReqCrc32c),
            FlexibleChecksumsReqCrc64 => Some(BusinessMetric::FlexibleChecksumsReqCrc64),
            FlexibleChecksumsReqSha1 => Some(BusinessMetric::FlexibleChecksumsReqSha1),
            FlexibleChecksumsReqSha256 => Some(BusinessMetric::FlexibleChecksumsReqSha256),
            FlexibleChecksumsReqWhenSupported => {
                Some(BusinessMetric::FlexibleChecksumsReqWhenSupported)
            }
            FlexibleChecksumsReqWhenRequired => {
                Some(BusinessMetric::FlexibleChecksumsReqWhenRequired)
            }
            FlexibleChecksumsResWhenSupported => {
                Some(BusinessMetric::FlexibleChecksumsResWhenSupported)
            }
            FlexibleChecksumsResWhenRequired => {
                Some(BusinessMetric::FlexibleChecksumsResWhenRequired)
            }
            ObservabilityOtelMetrics => Some(BusinessMetric::ObservabilityOtelMetrics),
            otherwise => {
                // This may occur if a customer upgrades only the `aws-smithy-runtime-api` crate
                // while continuing to use an outdated version of an SDK crate or the `aws-runtime`
                // crate.
                tracing::warn!(
                    "Attempted to provide `BusinessMetric` for `{otherwise:?}`, which is not recognized in the current version of the `aws-runtime` crate. \
                    Consider upgrading to the latest version to ensure that all tracked features are properly reported in your metrics."
                );
                None
            }
        }
    }
}

impl ProvideBusinessMetric for AwsSdkFeature {
    fn provide_business_metric(&self) -> Option<BusinessMetric> {
        use AwsSdkFeature::*;
        match self {
            AccountIdModePreferred => Some(BusinessMetric::AccountIdModePreferred),
            AccountIdModeDisabled => Some(BusinessMetric::AccountIdModeDisabled),
            AccountIdModeRequired => Some(BusinessMetric::AccountIdModeRequired),
            S3Transfer => Some(BusinessMetric::S3Transfer),
            SsoLoginDevice => Some(BusinessMetric::SsoLoginDevice),
            SsoLoginAuth => Some(BusinessMetric::SsoLoginAuth),
            EndpointOverride => Some(BusinessMetric::EndpointOverride),
        }
    }
}

impl ProvideBusinessMetric for AwsCredentialFeature {
    fn provide_business_metric(&self) -> Option<BusinessMetric> {
        use AwsCredentialFeature::*;
        match self {
            ResolvedAccountId => Some(BusinessMetric::ResolvedAccountId),
            CredentialsCode => Some(BusinessMetric::CredentialsCode),
            CredentialsEnvVars => Some(BusinessMetric::CredentialsEnvVars),
            CredentialsEnvVarsStsWebIdToken => {
                Some(BusinessMetric::CredentialsEnvVarsStsWebIdToken)
            }
            CredentialsStsAssumeRole => Some(BusinessMetric::CredentialsStsAssumeRole),
            CredentialsStsAssumeRoleSaml => Some(BusinessMetric::CredentialsStsAssumeRoleSaml),
            CredentialsStsAssumeRoleWebId => Some(BusinessMetric::CredentialsStsAssumeRoleWebId),
            CredentialsStsFederationToken => Some(BusinessMetric::CredentialsStsFederationToken),
            CredentialsStsSessionToken => Some(BusinessMetric::CredentialsStsSessionToken),
            CredentialsProfile => Some(BusinessMetric::CredentialsProfile),
            CredentialsProfileSourceProfile => {
                Some(BusinessMetric::CredentialsProfileSourceProfile)
            }
            CredentialsProfileNamedProvider => {
                Some(BusinessMetric::CredentialsProfileNamedProvider)
            }
            CredentialsProfileStsWebIdToken => {
                Some(BusinessMetric::CredentialsProfileStsWebIdToken)
            }
            CredentialsProfileSso => Some(BusinessMetric::CredentialsProfileSso),
            CredentialsSso => Some(BusinessMetric::CredentialsSso),
            CredentialsProfileProcess => Some(BusinessMetric::CredentialsProfileProcess),
            CredentialsProcess => Some(BusinessMetric::CredentialsProcess),
            CredentialsHttp => Some(BusinessMetric::CredentialsHttp),
            CredentialsImds => Some(BusinessMetric::CredentialsImds),
            BearerServiceEnvVars => Some(BusinessMetric::BearerServiceEnvVars),
            S3ExpressBucket => Some(BusinessMetric::S3ExpressBucket),
            CredentialsProfileLogin => Some(BusinessMetric::CredentialsProfileLogin),
            CredentialsLogin => Some(BusinessMetric::CredentialsLogin),
            otherwise => {
                // This may occur if a customer upgrades only the `aws-smithy-runtime-api` crate
                // while continuing to use an outdated version of an SDK crate or the `aws-credential-types`
                // crate.
                tracing::warn!(
                    "Attempted to provide `BusinessMetric` for `{otherwise:?}`, which is not recognized in the current version of the `aws-runtime` crate. \
                    Consider upgrading to the latest version to ensure that all tracked features are properly reported in your metrics."
                );
                None
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub(super) struct BusinessMetrics(Vec<BusinessMetric>);

impl BusinessMetrics {
    pub(super) fn push(&mut self, metric: BusinessMetric) {
        self.0.push(metric);
    }

    pub(super) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

fn drop_unfinished_metrics_to_fit(csv: &str, max_len: usize) -> Cow<'_, str> {
    if csv.len() <= max_len {
        Cow::Borrowed(csv)
    } else {
        let truncated = &csv[..max_len];
        if let Some(pos) = truncated.rfind(',') {
            Cow::Owned(truncated[..pos].to_owned())
        } else {
            Cow::Owned(truncated.to_owned())
        }
    }
}

impl fmt::Display for BusinessMetrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // business-metrics = "m/" metric_id *(comma metric_id)
        let metrics_values = self
            .0
            .iter()
            .map(|feature_id| {
                FEATURE_ID_TO_METRIC_VALUE
                    .get(feature_id)
                    .expect("{feature_id:?} should be found in `FEATURE_ID_TO_METRIC_VALUE`")
                    .clone()
            })
            .collect::<Vec<_>>()
            .join(",");

        let metrics_values = drop_unfinished_metrics_to_fit(
            &metrics_values,
            MAX_COMMA_SEPARATED_METRICS_VALUES_LENGTH,
        );

        write!(f, "m/{metrics_values}")
    }
}
#[cfg(test)]
mod tests {
    use crate::user_agent::metrics::{
        drop_unfinished_metrics_to_fit, Base64Iterator, FEATURE_ID_TO_METRIC_VALUE,
        MAX_METRICS_ID_NUMBER,
    };
    use crate::user_agent::BusinessMetric;
    use convert_case::{Boundary, Case, Casing};
    use std::collections::HashMap;
    use std::fmt::{Display, Formatter};

    impl Display for BusinessMetric {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.write_str(
                &format!("{:?}", self)
                    .as_str()
                    .from_case(Case::Pascal)
                    .with_boundaries(&[Boundary::DigitUpper, Boundary::LowerUpper])
                    .to_case(Case::ScreamingSnake),
            )
        }
    }

    #[test]
    fn feature_id_to_metric_value() {
        const EXPECTED: &str = include_str!("test_data/feature_id_to_metric_value.json");

        let expected: HashMap<&str, &str> = serde_json::from_str(EXPECTED).unwrap();
        assert_eq!(expected.len(), FEATURE_ID_TO_METRIC_VALUE.len());

        for (feature_id, metric_value) in &*FEATURE_ID_TO_METRIC_VALUE {
            let expected = expected.get(format!("{feature_id}").as_str());
            assert_eq!(
                expected.unwrap_or_else(|| panic!("Expected {feature_id} to have value `{metric_value}` but it was `{expected:?}` instead.")),
                metric_value,
            );
        }
    }

    #[test]
    fn test_base64_iter() {
        // 350 is the max number of metric IDs we support for now
        let ids: Vec<String> = Base64Iterator::new().take(MAX_METRICS_ID_NUMBER).collect();
        assert_eq!("A", ids[0]);
        assert_eq!("Z", ids[25]);
        assert_eq!("a", ids[26]);
        assert_eq!("z", ids[51]);
        assert_eq!("0", ids[52]);
        assert_eq!("9", ids[61]);
        assert_eq!("+", ids[62]);
        assert_eq!("-", ids[63]);
        assert_eq!("AA", ids[64]);
        assert_eq!("AB", ids[65]);
        assert_eq!("A-", ids[127]);
        assert_eq!("BA", ids[128]);
        assert_eq!("Ed", ids[349]);
    }

    #[test]
    fn test_drop_unfinished_metrics_to_fit() {
        let csv = "A,10BC,E";
        assert_eq!("A", drop_unfinished_metrics_to_fit(csv, 5));

        let csv = "A10B,CE";
        assert_eq!("A10B", drop_unfinished_metrics_to_fit(csv, 5));

        let csv = "A10BC,E";
        assert_eq!("A10BC", drop_unfinished_metrics_to_fit(csv, 5));

        let csv = "A10BCE";
        assert_eq!("A10BC", drop_unfinished_metrics_to_fit(csv, 5));

        let csv = "A";
        assert_eq!("A", drop_unfinished_metrics_to_fit(csv, 5));

        let csv = "A,B";
        assert_eq!("A,B", drop_unfinished_metrics_to_fit(csv, 5));
    }
}
