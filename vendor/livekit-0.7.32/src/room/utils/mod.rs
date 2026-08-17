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

use std::collections::HashMap;

use livekit_protocol as proto;

use crate::participant::ParticipantKindDetail;

pub mod take_cell;
pub(crate) mod ttl_map;
pub(crate) mod tx_queue;
pub mod utf8_chunk;

pub(crate) fn convert_kind_details(kind_details: &[i32]) -> Vec<ParticipantKindDetail> {
    kind_details
        .iter()
        .filter_map(|k| proto::participant_info::KindDetail::try_from(*k).ok())
        .map(Into::into)
        .collect()
}

pub fn calculate_changed_attributes(
    old_attributes: HashMap<String, String>,
    new_attributes: HashMap<String, String>,
) -> HashMap<String, String> {
    let old_keys = old_attributes.keys();
    let new_keys = new_attributes.keys();
    let all_keys: Vec<_> = old_keys.chain(new_keys).collect();

    let mut changed: HashMap<String, String> = HashMap::new();
    for key in all_keys {
        let old_value = old_attributes.get(key);
        let new_value = new_attributes.get(key);

        if old_value != new_value {
            match new_value {
                Some(new_value) => {
                    changed.insert(key.clone(), new_value.clone());
                }
                None => {
                    changed.insert(key.clone(), String::new());
                }
            }
        }
    }
    changed
}
