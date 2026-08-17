/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Types representing specific pieces of data contained within credentials or within token

/// Type representing a unique identifier representing an AWS account.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountId {
    inner: String,
}

impl AccountId {
    /// Return the string equivalent of this account id.
    pub fn as_str(&self) -> &str {
        &self.inner
    }
}

impl<T> From<T> for AccountId
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self {
            inner: value.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn account_id_creation() {
        let expected = "012345678901";
        assert_eq!(expected, AccountId::from(expected).as_str());
        assert_eq!(expected, AccountId::from(String::from(expected)).as_str());
    }
}
