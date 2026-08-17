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

use super::{ServiceBase, ServiceResult, LIVEKIT_PACKAGE};
use crate::{access_token::VideoGrants, get_env_keys, services::twirp_client::TwirpClient};

#[derive(Default, Clone, Debug)]
pub struct CreateIngressOptions {
    pub name: String,
    pub room_name: String,
    pub participant_metadata: String,
    pub participant_identity: String,
    pub participant_name: String,
    pub audio: proto::IngressAudioOptions,
    pub video: proto::IngressVideoOptions,
    pub bypass_transcoding: bool,
    pub enable_transcoding: Option<bool>,
    pub url: String,
}

#[derive(Default, Clone, Debug)]
pub struct UpdateIngressOptions {
    pub name: String,
    pub room_name: String,
    pub participant_metadata: String,
    pub participant_identity: String,
    pub participant_name: String,
    pub audio: proto::IngressAudioOptions,
    pub video: proto::IngressVideoOptions,
    pub bypass_transcoding: Option<bool>,
    pub enable_transcoding: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IngressListFilter {
    All,
    Room(String),
    IngressId(String),
}

const SVC: &str = "Ingress";

#[derive(Debug)]
pub struct IngressClient {
    base: ServiceBase,
    client: TwirpClient,
}

impl IngressClient {
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

    pub async fn create_ingress(
        &self,
        input_type: proto::IngressInput,
        options: CreateIngressOptions,
    ) -> ServiceResult<proto::IngressInfo> {
        self.client
            .request(
                SVC,
                "CreateIngress",
                proto::CreateIngressRequest {
                    input_type: input_type as i32,
                    name: options.name,
                    room_name: options.room_name,
                    participant_metadata: options.participant_metadata,
                    participant_identity: options.participant_identity,
                    participant_name: options.participant_name,
                    audio: Some(options.audio),
                    video: Some(options.video),
                    bypass_transcoding: options.bypass_transcoding,
                    enable_transcoding: options.enable_transcoding,
                    url: options.url,
                    enabled: Default::default(), // TODO: support this attribute
                },
                self.base
                    .auth_header(VideoGrants { ingress_admin: true, ..Default::default() }, None)?,
            )
            .await
            .map_err(Into::into)
    }

    pub async fn update_ingress(
        &self,
        ingress_id: &str,
        options: UpdateIngressOptions,
    ) -> ServiceResult<proto::IngressInfo> {
        self.client
            .request(
                SVC,
                "UpdateIngress",
                proto::UpdateIngressRequest {
                    ingress_id: ingress_id.to_owned(),
                    name: options.name,
                    room_name: options.room_name,
                    participant_metadata: options.participant_metadata,
                    participant_identity: options.participant_identity,
                    participant_name: options.participant_name,
                    audio: Some(options.audio),
                    video: Some(options.video),
                    bypass_transcoding: options.bypass_transcoding,
                    enable_transcoding: options.enable_transcoding,
                    enabled: Default::default(), // TODO: support this attribute
                },
                self.base
                    .auth_header(VideoGrants { ingress_admin: true, ..Default::default() }, None)?,
            )
            .await
            .map_err(Into::into)
    }

    pub async fn list_ingress(
        &self,
        filter: IngressListFilter,
    ) -> ServiceResult<Vec<proto::IngressInfo>> {
        let resp: proto::ListIngressResponse = self
            .client
            .request(
                SVC,
                "ListIngress",
                proto::ListIngressRequest {
                    ingress_id: match filter.clone() {
                        IngressListFilter::IngressId(id) => id,
                        _ => Default::default(),
                    },
                    room_name: match filter {
                        IngressListFilter::Room(room) => room,
                        _ => Default::default(),
                    },
                    page_token: Default::default(),
                },
                self.base
                    .auth_header(VideoGrants { ingress_admin: true, ..Default::default() }, None)?,
            )
            .await?;

        Ok(resp.items)
    }

    pub async fn delete_ingress(&self, ingress_id: &str) -> ServiceResult<proto::IngressInfo> {
        self.client
            .request(
                SVC,
                "DeleteIngress",
                proto::DeleteIngressRequest { ingress_id: ingress_id.to_owned() },
                self.base
                    .auth_header(VideoGrants { ingress_admin: true, ..Default::default() }, None)?,
            )
            .await
            .map_err(Into::into)
    }
}
