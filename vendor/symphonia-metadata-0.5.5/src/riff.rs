// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! A RIFF INFO metadata reader.

use lazy_static::lazy_static;
use std::collections::HashMap;
use symphonia_core::meta::{StandardTagKey, Tag, Value};

lazy_static! {
    static ref RIFF_INFO_MAP: HashMap<&'static str, StandardTagKey> = {
        let mut m = HashMap::new();
        m.insert("ages", StandardTagKey::Rating);
        m.insert("cmnt", StandardTagKey::Comment);
        // Is this the same as a cmnt?
        m.insert("comm", StandardTagKey::Comment);
        m.insert("dtim", StandardTagKey::OriginalDate);
        m.insert("genr", StandardTagKey::Genre);
        m.insert("iart", StandardTagKey::Artist);
        // Is this also  the same as cmnt?
        m.insert("icmt", StandardTagKey::Comment);
        m.insert("icop", StandardTagKey::Copyright);
        m.insert("icrd", StandardTagKey::Date);
        m.insert("idit", StandardTagKey::OriginalDate);
        m.insert("ienc", StandardTagKey::EncodedBy);
        m.insert("ieng", StandardTagKey::Engineer);
        m.insert("ifrm", StandardTagKey::TrackTotal);
        m.insert("ignr", StandardTagKey::Genre);
        m.insert("ilng", StandardTagKey::Language);
        m.insert("imus", StandardTagKey::Composer);
        m.insert("inam", StandardTagKey::TrackTitle);
        m.insert("iprd", StandardTagKey::Album);
        m.insert("ipro", StandardTagKey::Producer);
        m.insert("iprt", StandardTagKey::TrackNumber);
        m.insert("irtd", StandardTagKey::Rating);
        m.insert("isft", StandardTagKey::Encoder);
        m.insert("isgn", StandardTagKey::Genre);
        m.insert("isrf", StandardTagKey::MediaFormat);
        m.insert("itch", StandardTagKey::EncodedBy);
        m.insert("iwri", StandardTagKey::Writer);
        m.insert("lang", StandardTagKey::Language);
        m.insert("prt1", StandardTagKey::TrackNumber);
        m.insert("prt2", StandardTagKey::TrackTotal);
        // Same as inam?
        m.insert("titl", StandardTagKey::TrackTitle);
        m.insert("torg", StandardTagKey::Label);
        m.insert("trck", StandardTagKey::TrackNumber);
        m.insert("tver", StandardTagKey::Version);
        m.insert("year", StandardTagKey::Date);
        m
    };
}

/// Parse the RIFF INFO block into a `Tag` using the block's identifier tag and a slice
/// containing the block's contents.
pub fn parse(tag: [u8; 4], buf: &[u8]) -> Tag {
    // TODO: Key should be checked that it only contains ASCII characters.
    let key = String::from_utf8_lossy(&tag);
    let value = String::from_utf8_lossy(buf);

    // Attempt to assign a standardized tag key.
    let std_tag = RIFF_INFO_MAP.get(key.to_lowercase().as_str()).copied();

    Tag::new(std_tag, &key, Value::from(value))
}
