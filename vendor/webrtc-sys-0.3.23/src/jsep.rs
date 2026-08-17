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

use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use crate::impl_thread_safety;

#[cxx::bridge(namespace = "livekit_ffi")]
pub mod ffi {
    #[derive(Debug)]
    #[repr(i32)]
    pub enum SdpType {
        Offer,
        PrAnswer,
        Answer,
        Rollback,
    }

    #[derive(Debug)]
    pub struct SdpParseError {
        pub line: String,
        pub description: String,
    }

    extern "C++" {
        include!("livekit/rtc_error.h");

        type RtcError = crate::rtc_error::ffi::RtcError;
    }

    unsafe extern "C++" {
        include!("livekit/jsep.h");

        type IceCandidate;
        type SessionDescription;

        fn sdp_mid(self: &IceCandidate) -> String;
        fn sdp_mline_index(self: &IceCandidate) -> i32;
        fn candidate(self: &IceCandidate) -> String;
        fn stringify(self: &IceCandidate) -> String;

        fn sdp_type(self: &SessionDescription) -> SdpType;
        fn stringify(self: &SessionDescription) -> String;
        fn clone(self: &SessionDescription) -> UniquePtr<SessionDescription>;

        fn create_ice_candidate(
            sdp_mid: String,
            sdp_mline_index: i32,
            sdp: String,
        ) -> Result<SharedPtr<IceCandidate>>;

        fn create_session_description(
            sdp_type: SdpType,
            sdp: String,
        ) -> Result<UniquePtr<SessionDescription>>;

        fn _shared_ice_candidate() -> SharedPtr<IceCandidate>; // Ignore
        fn _unique_session_description() -> UniquePtr<SessionDescription>; // Ignore
    }
}

impl Error for ffi::SdpParseError {}

impl Display for ffi::SdpParseError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "SdpParseError occurred {}: {}", self.line, self.description)
    }
}

impl_thread_safety!(ffi::SessionDescription, Send + Sync);
impl_thread_safety!(ffi::IceCandidate, Send + Sync);

impl ffi::SdpParseError {
    /// # Safety
    /// The value must be correctly encoded
    pub unsafe fn from(value: &str) -> Self {
        // Parse the hex encoded error from c++
        let line_length = u32::from_str_radix(&value[0..8], 16).unwrap() as usize + 8;
        let line = String::from(&value[8..line_length]);
        let description = String::from(&value[line_length..]);

        Self { line, description }
    }
}

#[cfg(test)]
mod tests {
    use log::info;

    use crate::jsep::ffi;

    #[test]
    fn throw_error() {
        let sdp_string = "v=0
o=- 6549709950142776241 2 IN IP4 127.0.0.1
s=-
t=0 0
======================== ERROR HERE
a=group:BUNDLE 0
a=extmap-allow-mixed
a=msid-semantic: WMS
m=application 9 UDP/DTLS/SCTP webrtc-datachannel
c=IN IP4 0.0.0.0
a=ice-ufrag:Tw7h
a=ice-pwd:6XOVUD6HpcB4c1M8EB8jXJE9
a=ice-options:trickle
a=fingerprint:sha-256 4F:EC:23:59:5D:A5:E6:3E:3E:5D:8A:09:B6:FA:04:AA:19:99:49:67:BD:65:93:06:BB:EE:AC:D5:21:0F:57:D6
a=setup:actpass
a=mid:0
a=sctp-port:5000
a=max-message-size:262144
";

        let sdp = ffi::create_session_description(ffi::SdpType::Offer, sdp_string.to_string());
        let err = unsafe { ffi::SdpParseError::from(sdp.err().unwrap().what()) };
        info!("parse err: {:?}", err)
    }
}
