// Copyright 2022 The AccessKit Authors. All rights reserved.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file) or the MIT license (found in
// the LICENSE-MIT file), at your option.

use atspi::ObjectRef;
use serde::{Deserialize, Serialize};
use zbus::{
    names::UniqueName,
    zvariant::{ObjectPath, OwnedObjectPath, OwnedValue, Type, Value},
};

// https://gnome.pages.gitlab.gnome.org/at-spi2-core/libatspi/const.DBUS_PATH_NULL.html
const NULL_PATH: &str = "/org/a11y/atspi/null";

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, OwnedValue, Type, Value)]
pub(crate) struct OwnedObjectAddress {
    bus_name: String,
    path: OwnedObjectPath,
}

impl OwnedObjectAddress {
    pub(crate) fn new(bus_name: &UniqueName, path: OwnedObjectPath) -> Self {
        Self {
            bus_name: bus_name.to_string(),
            path,
        }
    }

    pub(crate) fn null() -> Self {
        Self {
            bus_name: String::new(),
            path: ObjectPath::from_str_unchecked(NULL_PATH).into(),
        }
    }
}

impl<'a> From<ObjectRef<'a>> for OwnedObjectAddress {
    fn from(object: ObjectRef<'a>) -> Self {
        Self {
            bus_name: object.name().map(|s| s.to_string()).unwrap_or_default(),
            path: object.path().to_owned().into(),
        }
    }
}
