/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_smithy_types::config_bag::{Storable, StoreAppend};

/// IDs for the credential related features that may be used in the AWS SDK
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AwsCredentialFeature {
    /// An operation where credential resolution resolved an account ID
    ResolvedAccountId,
    /// An operation called using credentials resolved from code, cli parameters, session object, or client instance
    CredentialsCode,
    /// An operation called using credentials resolved from environment variables
    CredentialsEnvVars,
    /// An operation called using credentials resolved from environment variables for assuming a role with STS using a web identity token
    CredentialsEnvVarsStsWebIdToken,
    /// An operation called using credentials resolved from STS using assume role
    CredentialsStsAssumeRole,
    /// An operation called using credentials resolved from STS using assume role with SAML
    CredentialsStsAssumeRoleSaml,
    /// An operation called using credentials resolved from STS using assume role with web identity
    CredentialsStsAssumeRoleWebId,
    /// An operation called using credentials resolved from STS using a federation token
    CredentialsStsFederationToken,
    /// An operation called using credentials resolved from STS using a session token
    CredentialsStsSessionToken,
    /// An operation called using credentials resolved from a config file(s) profile with static credentials
    CredentialsProfile,
    /// An operation called using credentials resolved from a source profile in a config file(s) profile
    CredentialsProfileSourceProfile,
    /// An operation called using credentials resolved from a named provider in a config file(s) profile
    CredentialsProfileNamedProvider,
    /// An operation called using credentials resolved from configuration for assuming a role with STS using web identity token in a config file(s) profile
    CredentialsProfileStsWebIdToken,
    /// An operation called using credentials resolved from an SSO session in a config file(s) profile
    CredentialsProfileSso,
    /// An operation called using credentials resolved from an SSO session
    CredentialsSso,
    /// An operation called using credentials resolved from a process in a config file(s) profile
    CredentialsProfileProcess,
    /// An operation called using credentials resolved from a process
    CredentialsProcess,
    /// An operation called using credentials resolved from an HTTP endpoint
    CredentialsHttp,
    /// An operation called using credentials resolved from the instance metadata service (IMDS)
    CredentialsImds,
    /// An operation called using a Bearer token resolved from service-specific environment variables
    BearerServiceEnvVars,
    /// An operation called using S3 Express bucket credentials
    S3ExpressBucket,
    /// An operation called using credentials resolved from a LoginCredentialsProvider configured via profile
    CredentialsProfileLogin,
    /// An operation called using credentials resolved from a LoginCredentialsProvider configured explicitly via code
    CredentialsLogin,
}

impl Storable for AwsCredentialFeature {
    type Storer = StoreAppend<Self>;
}
