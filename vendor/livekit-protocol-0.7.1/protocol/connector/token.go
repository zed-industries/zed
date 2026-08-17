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

package connector

import (
	"time"

	"github.com/livekit/protocol/auth"
	"github.com/livekit/protocol/livekit"
)

type Kind int

const (
	WHATSAPP Kind = iota
	TWILIO
)

type ConnectorTokenParams struct {
	APIKey                string
	APISecret             string
	RoomName              string
	ParticipantIdentity   string
	ParticipantName       string
	ParticipantMetadata   string
	ParticipantAttributes map[string]string
	Agents                []*livekit.RoomAgentDispatch
	Hidden                bool
	Kind                  Kind
}

func BuildConnectorToken(params ConnectorTokenParams) (string, error) {
	t := true
	at := auth.NewAccessToken(params.APIKey, params.APISecret).
		SetVideoGrant(&auth.VideoGrant{
			RoomJoin:             true,
			Room:                 params.RoomName,
			CanSubscribe:         &t,
			CanPublish:           &t,
			CanPublishData:       &t,
			CanUpdateOwnMetadata: &t,
			Hidden:               params.Hidden,
		}).
		SetAgents(params.Agents...).
		SetIdentity(params.ParticipantIdentity).
		SetName(params.ParticipantName).
		SetMetadata(params.ParticipantMetadata).
		SetAttributes(params.ParticipantAttributes).
		SetKind(livekit.ParticipantInfo_CONNECTOR).
		SetKindDetail(livekitKindDetailFromConnectorKind(params.Kind)).
		SetValidFor(24 * time.Hour).
		SetAllowSensitiveCredentials(true)

	return at.ToJWT()
}

func livekitKindDetailFromConnectorKind(kind Kind) livekit.ParticipantInfo_KindDetail {
	switch kind {
	case WHATSAPP:
		return livekit.ParticipantInfo_CONNECTOR_WHATSAPP
	case TWILIO:
		return livekit.ParticipantInfo_CONNECTOR_TWILIO
	default:
		return livekit.ParticipantInfo_CONNECTOR_WHATSAPP
	}
}
