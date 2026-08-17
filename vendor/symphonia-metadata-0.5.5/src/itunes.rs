// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! iTunes metadata support.

use symphonia_core::meta::StandardTagKey;

use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    static ref ITUNES_TAG_MAP: HashMap<&'static str, StandardTagKey> = {
        let mut m = HashMap::new();
        m.insert("com.apple.iTunes:ARTISTS", StandardTagKey::Artist);
        m.insert("com.apple.iTunes:ASIN", StandardTagKey::IdentAsin);
        m.insert("com.apple.iTunes:BARCODE", StandardTagKey::IdentBarcode);
        m.insert("com.apple.iTunes:CATALOGNUMBER", StandardTagKey::IdentCatalogNumber);
        m.insert("com.apple.iTunes:CONDUCTOR", StandardTagKey::Conductor);
        m.insert("com.apple.iTunes:DISCSUBTITLE", StandardTagKey::DiscSubtitle);
        m.insert("com.apple.iTunes:DJMIXER", StandardTagKey::MixDj);
        m.insert("com.apple.iTunes:ENGINEER", StandardTagKey::Engineer);
        m.insert("com.apple.iTunes:ISRC", StandardTagKey::IdentIsrc);
        m.insert("com.apple.iTunes:LABEL", StandardTagKey::Label);
        m.insert("com.apple.iTunes:LANGUAGE", StandardTagKey::Language);
        m.insert("com.apple.iTunes:LICENSE", StandardTagKey::License);
        m.insert("com.apple.iTunes:LYRICIST", StandardTagKey::Lyricist);
        m.insert("com.apple.iTunes:MEDIA", StandardTagKey::MediaFormat);
        m.insert("com.apple.iTunes:MIXER", StandardTagKey::MixEngineer);
        m.insert("com.apple.iTunes:MOOD", StandardTagKey::Mood);
        m.insert(
            "com.apple.iTunes:MusicBrainz Album Artist Id",
            StandardTagKey::MusicBrainzAlbumArtistId,
        );
        m.insert("com.apple.iTunes:MusicBrainz Album Id", StandardTagKey::MusicBrainzAlbumId);
        m.insert(
            "com.apple.iTunes:MusicBrainz Album Release Country",
            StandardTagKey::ReleaseCountry,
        );
        m.insert(
            "com.apple.iTunes:MusicBrainz Album Status",
            StandardTagKey::MusicBrainzReleaseStatus,
        );
        m.insert("com.apple.iTunes:MusicBrainz Album Type", StandardTagKey::MusicBrainzReleaseType);
        m.insert("com.apple.iTunes:MusicBrainz Artist Id", StandardTagKey::MusicBrainzArtistId);
        m.insert(
            "com.apple.iTunes:MusicBrainz Release Group Id",
            StandardTagKey::MusicBrainzReleaseGroupId,
        );
        m.insert(
            "com.apple.iTunes:MusicBrainz Release Track Id",
            StandardTagKey::MusicBrainzReleaseTrackId,
        );
        m.insert("com.apple.iTunes:MusicBrainz Track Id", StandardTagKey::MusicBrainzTrackId);
        m.insert("com.apple.iTunes:MusicBrainz Work Id", StandardTagKey::MusicBrainzWorkId);
        m.insert("com.apple.iTunes:originaldate", StandardTagKey::OriginalDate);
        m.insert("com.apple.iTunes:PRODUCER", StandardTagKey::Producer);
        m.insert("com.apple.iTunes:REMIXER", StandardTagKey::Remixer);
        m.insert("com.apple.iTunes:SCRIPT", StandardTagKey::Script);
        m.insert("com.apple.iTunes:SUBTITLE", StandardTagKey::TrackSubtitle);
        m
    };
}

/// Try to map the iTunes `tag` name to a `StandardTagKey`.
pub fn std_key_from_tag(key: &str) -> Option<StandardTagKey> {
    ITUNES_TAG_MAP.get(key).copied()
}
