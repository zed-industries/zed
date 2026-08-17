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

use cxx::SharedPtr;
use webrtc_sys::jsep as sys_jsep;

use crate::{ice_candidate as ic, session_description::SdpParseError};

#[derive(Clone)]
pub struct IceCandidate {
    pub(crate) sys_handle: SharedPtr<sys_jsep::ffi::IceCandidate>,
}

impl IceCandidate {
    pub fn parse(
        sdp_mid: &str,
        sdp_mline_index: i32,
        sdp: &str,
    ) -> Result<ic::IceCandidate, SdpParseError> {
        let res = sys_jsep::ffi::create_ice_candidate(
            sdp_mid.to_string(),
            sdp_mline_index,
            sdp.to_string(),
        );

        match res {
            Ok(sys_handle) => Ok(ic::IceCandidate { handle: IceCandidate { sys_handle } }),
            Err(e) => Err(unsafe { sys_jsep::ffi::SdpParseError::from(e.what()).into() }),
        }
    }

    pub fn sdp_mid(&self) -> String {
        self.sys_handle.sdp_mid()
    }

    pub fn sdp_mline_index(&self) -> i32 {
        self.sys_handle.sdp_mline_index()
    }

    pub fn candidate(&self) -> String {
        self.sys_handle.candidate()
    }
}

impl ToString for IceCandidate {
    fn to_string(&self) -> String {
        self.sys_handle.stringify()
    }
}
