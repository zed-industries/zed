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
    fmt::{Debug, Display},
    str::FromStr,
};

use thiserror::Error;

use crate::imp::session_description as sd_imp;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SdpType {
    Offer,
    PrAnswer,
    Answer,
    Rollback,
}

impl FromStr for SdpType {
    type Err = &'static str;

    fn from_str(sdp_type: &str) -> Result<Self, Self::Err> {
        match sdp_type {
            "offer" => Ok(Self::Offer),
            "pranswer" => Ok(Self::PrAnswer),
            "answer" => Ok(Self::Answer),
            "rollback" => Ok(Self::Rollback),
            _ => Err("invalid SdpType"),
        }
    }
}

impl Display for SdpType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            SdpType::Offer => "offer",
            SdpType::PrAnswer => "pranswer",
            SdpType::Answer => "answer",
            SdpType::Rollback => "rollback",
        };
        write!(f, "{}", s)
    }
}

#[derive(Clone)]
pub struct SessionDescription {
    pub(crate) handle: sd_imp::SessionDescription,
}

#[derive(Clone, Error, Debug)]
#[error("Failed to parse sdp: {line} - {description}")]
pub struct SdpParseError {
    pub line: String,
    pub description: String,
}

impl SessionDescription {
    pub fn parse(sdp: &str, sdp_type: SdpType) -> Result<Self, SdpParseError> {
        sd_imp::SessionDescription::parse(sdp, sdp_type)
    }

    pub fn sdp_type(&self) -> SdpType {
        self.handle.sdp_type()
    }
}

impl ToString for SessionDescription {
    fn to_string(&self) -> String {
        self.handle.to_string()
    }
}

impl Debug for SessionDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SessionDescription").field("sdp_type", &self.sdp_type()).finish()
    }
}
