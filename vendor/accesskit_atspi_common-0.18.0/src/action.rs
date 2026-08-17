// Copyright 2022 The AccessKit Authors. All rights reserved.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file) or the MIT license (found in
// the LICENSE-MIT file), at your option.

use serde::{Deserialize, Serialize};
use zvariant::Type;

#[derive(Deserialize, Serialize, Type)]
pub struct Action {
    pub localized_name: String,
    pub description: String,
    pub key_binding: String,
}
