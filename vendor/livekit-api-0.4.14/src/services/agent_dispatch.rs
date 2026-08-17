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

use super::{twirp_client::TwirpClient, ServiceBase, ServiceResult, LIVEKIT_PACKAGE};
use crate::{
    access_token::{AccessTokenError, VideoGrants},
    get_env_keys,
};
use http::header::HeaderMap;
use livekit_protocol as proto;

const SVC: &str = "AgentDispatchService";

#[derive(Debug)]
pub struct AgentDispatchClient {
    base: ServiceBase,
    client: TwirpClient,
}

impl AgentDispatchClient {
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

    /// Creates an explicit dispatch for an agent to join a room.
    ///
    /// To use explicit dispatch, your agent must be registered with an `agent_name`.
    ///
    /// # Arguments
    /// * `req` - Request containing dispatch creation parameters
    ///
    /// # Returns
    /// The created agent dispatch object
    ///
    pub async fn create_dispatch(
        &self,
        req: proto::CreateAgentDispatchRequest,
    ) -> ServiceResult<proto::AgentDispatch> {
        const METHOD: &str = "CreateDispatch";
        let headers = self.auth_headers(req.room.to_string())?;
        Ok(self.client.request(SVC, METHOD, req, headers).await?)
    }

    /// Deletes an explicit dispatch for an agent in a room.
    ///
    /// # Arguments
    /// * `dispatch_id` - ID of the dispatch to delete
    /// * `room_name` - Name of the room containing the dispatch
    ///
    /// # Returns
    /// The deleted agent dispatch object
    ///
    pub async fn delete_dispatch(
        &self,
        dispatch_id: impl Into<String>,
        room_name: impl Into<String>,
    ) -> ServiceResult<proto::AgentDispatch> {
        const METHOD: &str = "DeleteDispatch";
        let req = proto::DeleteAgentDispatchRequest {
            dispatch_id: dispatch_id.into(),
            room: room_name.into(),
        };
        let headers = self.auth_headers(req.room.to_string())?;
        Ok(self.client.request(SVC, METHOD, req, headers).await?)
    }

    /// Lists all agent dispatches in a room.
    ///
    /// # Arguments
    /// * `room_name` - Name of the room to list dispatches from
    ///
    /// # Returns
    /// List of dispatch objects in the room
    ///
    pub async fn list_dispatch(
        &self,
        room_name: impl Into<String>,
    ) -> ServiceResult<Vec<proto::AgentDispatch>> {
        const METHOD: &str = "ListDispatch";
        let req = proto::ListAgentDispatchRequest { room: room_name.into(), ..Default::default() };
        let headers = self.auth_headers(req.room.to_string())?;
        let res: proto::ListAgentDispatchResponse =
            self.client.request(SVC, METHOD, req, headers).await?;
        Ok(res.agent_dispatches)
    }

    /// Gets an agent dispatch by ID.
    ///
    /// # Arguments
    /// * `dispatch_id` - ID of the dispatch to retrieve
    /// * `room_name` - Name of the room containing the dispatch
    ///
    /// # Returns
    /// Requested dispatch object if found, `None` otherwise
    ///
    pub async fn get_dispatch(
        &self,
        dispatch_id: impl Into<String>,
        room_name: impl Into<String>,
    ) -> ServiceResult<Option<proto::AgentDispatch>> {
        const METHOD: &str = "ListDispatch";
        let req = proto::ListAgentDispatchRequest {
            room: room_name.into(),
            dispatch_id: dispatch_id.into(),
        };
        let headers = self.auth_headers(req.room.to_string())?;
        let mut res: proto::ListAgentDispatchResponse =
            self.client.request(SVC, METHOD, req, headers).await?;
        Ok(res.agent_dispatches.pop())
    }
}

impl AgentDispatchClient {
    /// Generates the auth header common to all dispatch request types.
    fn auth_headers(&self, room: String) -> Result<HeaderMap, AccessTokenError> {
        self.base.auth_header(VideoGrants { room, room_admin: true, ..Default::default() }, None)
    }
}
