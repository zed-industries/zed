// Copyright 2025 LiveKit, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fmt::{self, Debug, Formatter};

use self::key_provider::KeyProvider;

pub mod key_provider;
pub mod manager;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum EncryptionType {
    #[default]
    None,
    Gcm,
    Custom,
}

#[derive(Clone)]
pub struct E2eeOptions {
    pub encryption_type: EncryptionType,
    pub key_provider: KeyProvider,
}

impl Debug for E2eeOptions {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("E2eeOptions").field("encryption_type", &self.encryption_type).finish()
    }
}
