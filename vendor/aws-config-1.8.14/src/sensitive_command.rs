/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use std::fmt;

#[derive(Clone)]
pub(crate) struct CommandWithSensitiveArgs<T>(T);

impl<T> CommandWithSensitiveArgs<T>
where
    T: AsRef<str>,
{
    pub(crate) fn new(value: T) -> Self {
        Self(value)
    }

    #[allow(dead_code)]
    pub(crate) fn to_owned_string(&self) -> CommandWithSensitiveArgs<String> {
        CommandWithSensitiveArgs(self.0.as_ref().to_string())
    }

    #[allow(dead_code)]
    pub(crate) fn unredacted(&self) -> &str {
        self.0.as_ref()
    }
}

impl<T> fmt::Display for CommandWithSensitiveArgs<T>
where
    T: AsRef<str>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Security: The arguments for command must be redacted since they can be sensitive
        let command = self.0.as_ref();
        match command.find(char::is_whitespace) {
            Some(index) => write!(f, "{} ** arguments redacted **", &command[0..index]),
            None => write!(f, "{command}"),
        }
    }
}

impl<T> fmt::Debug for CommandWithSensitiveArgs<T>
where
    T: AsRef<str>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", format!("{}", self))
    }
}
