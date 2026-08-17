// Copyright 2023 LiveKit, Inc.
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

package auth

import (
	"encoding/base64"
	"encoding/json"
	"strings"
	"testing"
	"time"

	"github.com/go-jose/go-jose/v3/jwt"
	"github.com/stretchr/testify/require"

	"github.com/livekit/protocol/livekit"
	"github.com/livekit/protocol/utils"
	"github.com/livekit/protocol/utils/guid"
)

func TestAccessToken(t *testing.T) {
	t.Parallel()

	t.Run("keys must be set", func(t *testing.T) {
		token := NewAccessToken("", "")
		_, err := token.ToJWT()
		require.Equal(t, ErrKeysMissing, err)
	})

	t.Run("generates a decode-able key", func(t *testing.T) {
		apiKey, secret := apiKeypair()
		videoGrant := &VideoGrant{RoomJoin: true, Room: "myroom"}
		sipGrant := &SIPGrant{Admin: true}
		agentGrant := &AgentGrant{Admin: true}
		inferenceGrant := &InferenceGrant{Perform: true}
		at := NewAccessToken(apiKey, secret).
			SetVideoGrant(videoGrant).
			SetSIPGrant(sipGrant).
			SetInferenceGrant(inferenceGrant).
			SetAgentGrant(agentGrant).
			SetValidFor(time.Minute * 5).
			SetKind(livekit.ParticipantInfo_AGENT).
			SetIdentity("user")
		value, err := at.ToJWT()
		//fmt.Println(raw)
		require.NoError(t, err)

		require.Len(t, strings.Split(value, "."), 3)

		// ensure it's a valid JWT
		token, err := jwt.ParseSigned(value)
		require.NoError(t, err)

		decodedGrant := ClaimGrants{}
		err = token.UnsafeClaimsWithoutVerification(&decodedGrant)
		require.NoError(t, err)

		require.EqualValues(t, livekit.ParticipantInfo_AGENT, decodedGrant.GetParticipantKind())
		require.EqualValues(t, videoGrant, decodedGrant.Video)
		require.EqualValues(t, sipGrant, decodedGrant.SIP)
		require.EqualValues(t, agentGrant, decodedGrant.Agent)
		require.EqualValues(t, inferenceGrant, decodedGrant.Inference)
	})

	t.Run("missing kind should be interpreted as standard", func(t *testing.T) {
		apiKey, secret := apiKeypair()
		value, err := NewAccessToken(apiKey, secret).
			SetVideoGrant(&VideoGrant{RoomJoin: true, Room: "myroom"}).
			ToJWT()
		require.NoError(t, err)
		token, err := jwt.ParseSigned(value)
		require.NoError(t, err)

		decodedGrant := ClaimGrants{}
		err = token.UnsafeClaimsWithoutVerification(&decodedGrant)
		require.NoError(t, err)

		// default validity
		require.EqualValues(t, livekit.ParticipantInfo_STANDARD, decodedGrant.GetParticipantKind())
	})

	t.Run("default validity should be more than a minute", func(t *testing.T) {
		apiKey, secret := apiKeypair()
		videoGrant := &VideoGrant{RoomJoin: true, Room: "myroom"}
		at := NewAccessToken(apiKey, secret).
			SetVideoGrant(videoGrant)
		value, err := at.ToJWT()
		require.NoError(t, err)
		token, err := jwt.ParseSigned(value)
		require.NoError(t, err)

		claim := jwt.Claims{}
		decodedGrant := ClaimGrants{}
		err = token.UnsafeClaimsWithoutVerification(&claim, &decodedGrant)
		require.NoError(t, err)
		require.EqualValues(t, videoGrant, decodedGrant.Video)

		// default validity
		require.True(t, claim.Expiry.Time().Sub(claim.IssuedAt.Time()) > time.Minute)
	})

	t.Run("room configuration serialization and deserialization", func(t *testing.T) {
		apiKey, secret := apiKeypair()
		roomConfig := &livekit.RoomConfiguration{
			Agents: []*livekit.RoomAgentDispatch{{
				AgentName: "agent1",
				Metadata:  "metadata1",
			}},
			SyncStreams: true,
			Egress: &livekit.RoomEgress{
				Room: &livekit.RoomCompositeEgressRequest{
					FileOutputs: []*livekit.EncodedFileOutput{{
						DisableManifest: true,
					}},
				},
			},
		}
		videoGrant := &VideoGrant{RoomJoin: true, Room: "test-room"}
		at := NewAccessToken(apiKey, secret).
			SetVideoGrant(videoGrant).
			SetRoomConfig(roomConfig)

		value, err := at.ToJWT()
		require.NoError(t, err)

		// Parse and verify the token
		token, err := jwt.ParseSigned(value)
		require.NoError(t, err)

		decodedGrant := ClaimGrants{}
		err = token.UnsafeClaimsWithoutVerification(&decodedGrant)
		require.NoError(t, err)

		// Check if the room configuration was correctly serialized and deserialized
		roomDecoded := (*livekit.RoomConfiguration)(decodedGrant.RoomConfig)
		require.NotNil(t, roomDecoded)
		agents := roomDecoded.Agents
		require.NotNil(t, agents)
		require.Len(t, agents, 1)
		require.Equal(t, "agent1", agents[0].AgentName)
		require.Equal(t, "metadata1", agents[0].Metadata)
		egress := roomDecoded.Egress
		require.NotNil(t, egress)
		require.Equal(t, true, egress.Room.FileOutputs[0].DisableManifest)

		// Ensure that we are encoding room configuration with camelCase
		parts := strings.Split(value, ".")
		require.Equal(t, 3, len(parts), "JWT should have three parts")
		payload, err := base64.RawURLEncoding.DecodeString(parts[1])
		require.NoError(t, err)

		// Parse the JSON
		var jsonPayload map[string]interface{}
		err = json.Unmarshal(payload, &jsonPayload)
		require.NoError(t, err)

		// Navigate to the agents array
		room, ok := jsonPayload["roomConfig"].(map[string]interface{})
		require.True(t, ok, "room should be a map")
		agentsJSON, ok := room["agents"].([]interface{})
		require.True(t, ok, "agents should be an array")
		require.Len(t, agents, 1, "there should be one agent")

		// Check if agentName is in camelCase
		agent, ok := agentsJSON[0].(map[string]interface{})
		require.True(t, ok, "agent should be a map")
		_, hasAgentName := agent["agentName"]
		require.True(t, hasAgentName, "agentName should be present in camelCase")
		_, hasAgentNameSnakeCase := agent["agent_name"]
		require.False(t, hasAgentNameSnakeCase, "agent_name should not be present in snake_case")

		t.Run("room configuration blocks sensitive credentials by default", func(t *testing.T) {
			apiKey, secret := apiKeypair()
			roomConfig := &livekit.RoomConfiguration{
				Egress: &livekit.RoomEgress{
					Room: &livekit.RoomCompositeEgressRequest{
						FileOutputs: []*livekit.EncodedFileOutput{{
							Output: &livekit.EncodedFileOutput_S3{S3: &livekit.S3Upload{Secret: "super-secret"}},
						}},
					},
				},
			}
			_, err := NewAccessToken(apiKey, secret).
				SetVideoGrant(&VideoGrant{RoomJoin: true, Room: "test-room"}).
				SetRoomConfig(roomConfig).
				ToJWT()
			require.ErrorIs(t, err, ErrSensitiveCredentials)
		})

		t.Run("room configuration allows sensitive credentials when enabled", func(t *testing.T) {
			apiKey, secret := apiKeypair()
			roomConfig := &livekit.RoomConfiguration{
				Egress: &livekit.RoomEgress{
					Room: &livekit.RoomCompositeEgressRequest{
						FileOutputs: []*livekit.EncodedFileOutput{{
							Output: &livekit.EncodedFileOutput_S3{S3: &livekit.S3Upload{Secret: "super-secret"}},
						}},
					},
				},
			}
			value, err := NewAccessToken(apiKey, secret).
				SetVideoGrant(&VideoGrant{RoomJoin: true, Room: "test-room"}).
				SetRoomConfig(roomConfig).
				SetAllowSensitiveCredentials(true).
				ToJWT()
			require.NoError(t, err)

			v, err := ParseAPIToken(value)
			require.NoError(t, err)
			_, claims, err := v.Verify(secret)
			require.NoError(t, err)

			rc := (*livekit.RoomConfiguration)(claims.RoomConfig)
			require.NotNil(t, rc)
			require.NotNil(t, rc.Egress)
			require.NotNil(t, rc.Egress.Room)
			require.NotEmpty(t, rc.Egress.Room.FileOutputs)
			s3Out, ok := rc.Egress.Room.FileOutputs[0].Output.(*livekit.EncodedFileOutput_S3)
			require.True(t, ok)
			require.Equal(t, "super-secret", s3Out.S3.Secret)
		})
	})
}

func apiKeypair() (string, string) {
	return guid.New(utils.APIKeyPrefix), utils.RandomSecret()
}
