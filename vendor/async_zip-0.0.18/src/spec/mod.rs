// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

pub(crate) mod attribute;
pub(crate) mod compression;
pub(crate) mod consts;
pub(crate) mod extra_field;
pub(crate) mod header;
pub(crate) mod parse;
pub(crate) mod version;

pub use compression::Compression;
