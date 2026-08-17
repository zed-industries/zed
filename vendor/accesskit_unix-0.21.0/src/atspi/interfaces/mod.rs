// Copyright 2022 The AccessKit Authors. All rights reserved.
// Licensed under the Apache License, Version 2.0 (found in
// the LICENSE-APACHE file) or the MIT license (found in
// the LICENSE-MIT file), at your option.

mod accessible;
mod action;
mod application;
mod component;
mod hyperlink;
mod selection;
mod text;
mod value;

use crate::atspi::{ObjectId, OwnedObjectAddress};
use zbus::{fdo, names::UniqueName};

fn map_root_error(error: accesskit_atspi_common::Error) -> fdo::Error {
    crate::util::map_error(ObjectId::Root, error)
}

fn optional_object_address(
    bus_name: &UniqueName,
    object_id: Option<ObjectId>,
) -> (OwnedObjectAddress,) {
    match object_id {
        Some(id) => (id.to_address(bus_name),),
        None => (OwnedObjectAddress::null(),),
    }
}

pub(crate) use accessible::*;
pub(crate) use action::*;
pub(crate) use application::*;
pub(crate) use component::*;
pub(crate) use hyperlink::*;
pub(crate) use selection::*;
pub(crate) use text::*;
pub(crate) use value::*;
