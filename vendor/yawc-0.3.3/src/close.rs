//
// This code is sourced primarily from the tungstenite-rs library, which can be found at:
// https://github.com/snapview/tungstenite-rs/blob/42b8797e8b7f39efb7d9322dc8af3e9089db4f7d/src/protocol/frame/coding.rs#L117
//
// Original contributions by:
// Copyright (c) 2017 Alexey Galakhov
// Copyright (c) 2016 Jason Housley
// Licensed under both MIT and Apache 2.0 licenses
//
// Modifications made by:
// Copyright 2023 Divy Srivastava <dj.srivastava23@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// You may obtain a copy of the License at:
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is provided "AS IS", WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND,
// either express or implied. See the License for specific language governing permissions and limitations.
//

use self::CloseCode::*;

/// Status codes representing the reasons why an endpoint is closing the WebSocket connection.
/// Each code provides specific information that helps in understanding and handling the close event.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum CloseCode {
    /// Normal closure, indicating that the purpose for which the connection was established has been fulfilled.
    /// This is a graceful closure where everything went as expected.
    Normal,
    /// Indicates that the endpoint is "going away", such as when a server is shutting down or a user navigates away from a page.
    /// This is not an error but a planned disconnection.
    Away,
    /// Protocol error occurred, leading the endpoint to terminate the connection.
    /// This signifies a violation of the WebSocket protocol.
    Protocol,
    /// The endpoint is terminating the connection because it received a type of data it cannot accept.
    /// For example, a server that only supports text data might use this code if it gets a binary message.
    Unsupported,
    /// No status code was included in a closing frame.
    /// This allows handling cases where a close code isn’t present by using a single method such as `on_close`.
    Status,
    /// Abnormal closure, meaning the connection was closed without any data frame being sent or received.
    /// This isn’t used for errors, which should be handled by the `on_error` method instead.
    Abnormal,
    /// Connection is being terminated because the received message data was inconsistent with the expected type.
    /// For instance, receiving non-UTF-8 data in a text message triggers this.
    Invalid,
    /// Policy violation occurred, causing the endpoint to close the connection.
    /// This is a generic code for policy-related issues, when no more specific code applies.
    Policy,
    /// The message sent is too large for the endpoint to process, leading to termination.
    Size,
    /// Termination happens because the client expected certain extensions that the server didn’t provide.
    /// This doesn't apply to servers as they can refuse the handshake outright if unsupported.
    Extension,
    /// An unexpected server error condition prompted the server to terminate the connection.
    /// This indicates a problem on the server side that isn't directly related to the WebSocket protocol.
    Error,
    /// The server is restarting, suggesting that the client may try reconnecting.
    /// Clients are advised to use a randomized delay between 5-30 seconds for reconnection attempts.
    Restart,
    /// Server overload caused it to close the connection; clients should try different IPs or reconnect after a user action.
    /// This suggests a temporary unavailability rather than a permanent issue.
    Again,
    #[doc(hidden)]
    /// Indicates a TLS handshake failure. This is primarily used inside the library and not exposed to users.
    Tls,
    #[doc(hidden)]
    /// Reserved status codes for future use. These are not meant for public use and are for internal consistency.
    Reserved(u16),
    #[doc(hidden)]
    /// IANA-assigned codes, designated for experimental or future use by the Internet Assigned Numbers Authority.
    Iana(u16),
    #[doc(hidden)]
    /// Reserved for library-defined codes, allowing developers to extend functionality with custom codes.
    Library(u16),
    #[doc(hidden)]
    /// Represents bad or unknown codes outside of the range of acceptable WebSocket close codes.
    Bad(u16),
}

impl CloseCode {
    /// Check if this CloseCode is allowed.
    pub fn is_allowed(self) -> bool {
        !matches!(self, Bad(_) | Reserved(_) | Status | Abnormal | Tls)
    }
}

impl From<u16> for CloseCode {
    fn from(code: u16) -> CloseCode {
        match code {
            1000 => Normal,
            1001 => Away,
            1002 => Protocol,
            1003 => Unsupported,
            1005 => Status,
            1006 => Abnormal,
            1007 => Invalid,
            1008 => Policy,
            1009 => Size,
            1010 => Extension,
            1011 => Error,
            1012 => Restart,
            1013 => Again,
            1015 => Tls,
            1..=999 => Bad(code),
            1016..=2999 => Reserved(code),
            3000..=3999 => Iana(code),
            4000..=4999 => Library(code),
            _ => Bad(code),
        }
    }
}

impl From<CloseCode> for u16 {
    fn from(code: CloseCode) -> u16 {
        match code {
            Normal => 1000,
            Away => 1001,
            Protocol => 1002,
            Unsupported => 1003,
            Status => 1005,
            Abnormal => 1006,
            Invalid => 1007,
            Policy => 1008,
            Size => 1009,
            Extension => 1010,
            Error => 1011,
            Restart => 1012,
            Again => 1013,
            Tls => 1015,
            Reserved(code) => code,
            Iana(code) => code,
            Library(code) => code,
            Bad(code) => code,
        }
    }
}
