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

use std::fmt::Debug;

use crate::{imp::ice_candidate as imp_ic, session_description::SdpParseError};

pub struct IceCandidate {
    pub(crate) handle: imp_ic::IceCandidate,
}

impl IceCandidate {
    pub fn parse(
        sdp_mid: &str,
        sdp_mline_index: i32,
        sdp: &str,
    ) -> Result<IceCandidate, SdpParseError> {
        imp_ic::IceCandidate::parse(sdp_mid, sdp_mline_index, sdp)
    }

    pub fn sdp_mid(&self) -> String {
        self.handle.sdp_mid()
    }

    pub fn sdp_mline_index(&self) -> i32 {
        self.handle.sdp_mline_index()
    }

    pub fn candidate(&self) -> String {
        self.handle.candidate()
    }
}

impl ToString for IceCandidate {
    fn to_string(&self) -> String {
        self.handle.to_string()
    }
}

impl Debug for IceCandidate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IceCandidate").field("candidate", &self.to_string()).finish()
    }
}
