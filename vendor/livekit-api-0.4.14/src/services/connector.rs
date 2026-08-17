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

use livekit_protocol as proto;
use std::collections::HashMap;

use super::{ServiceBase, ServiceResult, LIVEKIT_PACKAGE};
use crate::{access_token::VideoGrants, get_env_keys, services::twirp_client::TwirpClient};

const SVC: &str = "Connector";

/// Options for dialing a WhatsApp call
#[derive(Default, Clone, Debug)]
pub struct DialWhatsAppCallOptions {
    /// Optional - An arbitrary string useful for tracking and logging purposes
    pub biz_opaque_callback_data: Option<String>,
    /// Optional - What LiveKit room should this participant be connected to
    pub room_name: Option<String>,
    /// Optional - Agents to dispatch the call to
    pub agents: Option<Vec<proto::RoomAgentDispatch>>,
    /// Optional - Identity of the participant in LiveKit room
    pub participant_identity: Option<String>,
    /// Optional - Name of the participant in LiveKit room
    pub participant_name: Option<String>,
    /// Optional - User-defined metadata attached to the participant in the room
    pub participant_metadata: Option<String>,
    /// Optional - User-defined attributes attached to the participant in the room
    pub participant_attributes: Option<HashMap<String, String>>,
    /// Optional - Country where the call terminates as ISO 3166-1 alpha-2
    pub destination_country: Option<String>,
}

/// Options for accepting a WhatsApp call
#[derive(Default, Clone, Debug)]
pub struct AcceptWhatsAppCallOptions {
    /// Optional - An arbitrary string useful for tracking and logging purposes
    pub biz_opaque_callback_data: Option<String>,
    /// Optional - What LiveKit room should this participant be connected to
    pub room_name: Option<String>,
    /// Optional - Agents to dispatch the call to
    pub agents: Option<Vec<proto::RoomAgentDispatch>>,
    /// Optional - Identity of the participant in LiveKit room
    pub participant_identity: Option<String>,
    /// Optional - Name of the participant in LiveKit room
    pub participant_name: Option<String>,
    /// Optional - User-defined metadata attached to the participant in the room
    pub participant_metadata: Option<String>,
    /// Optional - User-defined attributes attached to the participant in the room
    pub participant_attributes: Option<HashMap<String, String>>,
    /// Optional - Country where the call terminates as ISO 3166-1 alpha-2
    pub destination_country: Option<String>,
}

/// Options for connecting a Twilio call
#[derive(Default, Clone, Debug)]
pub struct ConnectTwilioCallOptions {
    /// Optional - Agents to dispatch the call to
    pub agents: Option<Vec<proto::RoomAgentDispatch>>,
    /// Optional - Identity of the participant in LiveKit room
    pub participant_identity: Option<String>,
    /// Optional - Name of the participant in LiveKit room
    pub participant_name: Option<String>,
    /// Optional - User-defined metadata attached to the participant in the room
    pub participant_metadata: Option<String>,
    /// Optional - User-defined attributes attached to the participant in the room
    pub participant_attributes: Option<HashMap<String, String>>,
    /// Optional - Country where the call terminates as ISO 3166-1 alpha-2
    pub destination_country: Option<String>,
}

#[derive(Debug)]
pub struct ConnectorClient {
    base: ServiceBase,
    client: TwirpClient,
}

impl ConnectorClient {
    pub fn with_api_key(host: &str, api_key: &str, api_secret: &str) -> Self {
        Self {
            base: ServiceBase::with_api_key(api_key, api_secret),
            client: TwirpClient::new(host, LIVEKIT_PACKAGE, None),
        }
    }

    pub fn new(host: &str) -> ServiceResult<Self> {
        let (api_key, api_secret) = get_env_keys()?;
        Ok(Self::with_api_key(host, &api_key, &api_secret))
    }

    /// Dials a WhatsApp call
    ///
    /// # Arguments
    /// * `phone_number_id` - The identifier of the number for business initiating the call
    /// * `to_phone_number` - The number of the user that should receive the call
    /// * `api_key` - The API key of the business initiating the call
    /// * `cloud_api_version` - WhatsApp Cloud API version (e.g., "23.0", "24.0")
    /// * `options` - Additional options for the call
    ///
    /// # Returns
    /// Information about the dialed call including the WhatsApp call ID and room name
    pub async fn dial_whatsapp_call(
        &self,
        phone_number_id: impl Into<String>,
        to_phone_number: impl Into<String>,
        api_key: impl Into<String>,
        cloud_api_version: impl Into<String>,
        options: DialWhatsAppCallOptions,
    ) -> ServiceResult<proto::DialWhatsAppCallResponse> {
        self.client
            .request(
                SVC,
                "DialWhatsAppCall",
                proto::DialWhatsAppCallRequest {
                    whatsapp_phone_number_id: phone_number_id.into(),
                    whatsapp_to_phone_number: to_phone_number.into(),
                    whatsapp_api_key: api_key.into(),
                    whatsapp_cloud_api_version: cloud_api_version.into(),
                    whatsapp_biz_opaque_callback_data: options
                        .biz_opaque_callback_data
                        .unwrap_or_default(),
                    room_name: options.room_name.unwrap_or_default(),
                    agents: options.agents.unwrap_or_default(),
                    participant_identity: options.participant_identity.unwrap_or_default(),
                    participant_name: options.participant_name.unwrap_or_default(),
                    participant_metadata: options.participant_metadata.unwrap_or_default(),
                    participant_attributes: options.participant_attributes.unwrap_or_default(),
                    destination_country: options.destination_country.unwrap_or_default(),
                },
                self.base
                    .auth_header(VideoGrants { room_create: true, ..Default::default() }, None)?,
            )
            .await
            .map_err(Into::into)
    }

    /// Disconnects a WhatsApp call
    ///
    /// # Arguments
    /// * `call_id` - Call ID sent by Meta
    /// * `api_key` - The API key of the business disconnecting the call
    ///
    /// # Returns
    /// Empty response on success
    pub async fn disconnect_whatsapp_call(
        &self,
        call_id: impl Into<String>,
        api_key: impl Into<String>,
    ) -> ServiceResult<proto::DisconnectWhatsAppCallResponse> {
        self.client
            .request(
                SVC,
                "DisconnectWhatsAppCall",
                proto::DisconnectWhatsAppCallRequest {
                    whatsapp_call_id: call_id.into(),
                    whatsapp_api_key: api_key.into(),
                    ..Default::default()
                },
                self.base
                    .auth_header(VideoGrants { room_create: true, ..Default::default() }, None)?,
            )
            .await
            .map_err(Into::into)
    }

    /// Connects a WhatsApp call (handles the SDP exchange)
    ///
    /// # Arguments
    /// * `call_id` - Call ID sent by Meta
    /// * `sdp` - The SDP from Meta (answer SDP for business-initiated call)
    ///
    /// # Returns
    /// Empty response on success
    pub async fn connect_whatsapp_call(
        &self,
        call_id: impl Into<String>,
        sdp: proto::SessionDescription,
    ) -> ServiceResult<proto::ConnectWhatsAppCallResponse> {
        self.client
            .request(
                SVC,
                "ConnectWhatsAppCall",
                proto::ConnectWhatsAppCallRequest {
                    whatsapp_call_id: call_id.into(),
                    sdp: Some(sdp),
                },
                self.base
                    .auth_header(VideoGrants { room_create: true, ..Default::default() }, None)?,
            )
            .await
            .map_err(Into::into)
    }

    /// Accepts an incoming WhatsApp call
    ///
    /// # Arguments
    /// * `phone_number_id` - The identifier of the number for business initiating the call
    /// * `api_key` - The API key of the business connecting the call
    /// * `cloud_api_version` - WhatsApp Cloud API version (e.g., "23.0", "24.0")
    /// * `call_id` - Call ID sent by Meta
    /// * `sdp` - The SDP from Meta (for user-initiated call)
    /// * `options` - Additional options for the call
    ///
    /// # Returns
    /// Information about the accepted call including the room name
    pub async fn accept_whatsapp_call(
        &self,
        phone_number_id: impl Into<String>,
        api_key: impl Into<String>,
        cloud_api_version: impl Into<String>,
        call_id: impl Into<String>,
        sdp: proto::SessionDescription,
        options: AcceptWhatsAppCallOptions,
    ) -> ServiceResult<proto::AcceptWhatsAppCallResponse> {
        self.client
            .request(
                SVC,
                "AcceptWhatsAppCall",
                proto::AcceptWhatsAppCallRequest {
                    whatsapp_phone_number_id: phone_number_id.into(),
                    whatsapp_api_key: api_key.into(),
                    whatsapp_cloud_api_version: cloud_api_version.into(),
                    whatsapp_call_id: call_id.into(),
                    whatsapp_biz_opaque_callback_data: options
                        .biz_opaque_callback_data
                        .unwrap_or_default(),
                    sdp: Some(sdp),
                    room_name: options.room_name.unwrap_or_default(),
                    agents: options.agents.unwrap_or_default(),
                    participant_identity: options.participant_identity.unwrap_or_default(),
                    participant_name: options.participant_name.unwrap_or_default(),
                    participant_metadata: options.participant_metadata.unwrap_or_default(),
                    participant_attributes: options.participant_attributes.unwrap_or_default(),
                    destination_country: options.destination_country.unwrap_or_default(),
                },
                self.base
                    .auth_header(VideoGrants { room_create: true, ..Default::default() }, None)?,
            )
            .await
            .map_err(Into::into)
    }

    /// Connects a Twilio call
    ///
    /// # Arguments
    /// * `direction` - The direction of the call (inbound or outbound)
    /// * `room_name` - What LiveKit room should this call be connected to
    /// * `options` - Additional options for the call
    ///
    /// # Returns
    /// The WebSocket URL which Twilio media stream should connect to
    pub async fn connect_twilio_call(
        &self,
        direction: proto::connect_twilio_call_request::TwilioCallDirection,
        room_name: impl Into<String>,
        options: ConnectTwilioCallOptions,
    ) -> ServiceResult<proto::ConnectTwilioCallResponse> {
        self.client
            .request(
                SVC,
                "ConnectTwilioCall",
                proto::ConnectTwilioCallRequest {
                    twilio_call_direction: direction as i32,
                    room_name: room_name.into(),
                    agents: options.agents.unwrap_or_default(),
                    participant_identity: options.participant_identity.unwrap_or_default(),
                    participant_name: options.participant_name.unwrap_or_default(),
                    participant_metadata: options.participant_metadata.unwrap_or_default(),
                    participant_attributes: options.participant_attributes.unwrap_or_default(),
                    destination_country: options.destination_country.unwrap_or_default(),
                },
                self.base
                    .auth_header(VideoGrants { room_create: true, ..Default::default() }, None)?,
            )
            .await
            .map_err(Into::into)
    }
}
