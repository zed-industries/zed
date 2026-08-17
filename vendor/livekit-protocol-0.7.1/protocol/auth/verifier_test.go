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

package auth_test

import (
	"testing"
	"time"

	"github.com/go-jose/go-jose/v3/json"
	"github.com/stretchr/testify/require"

	"github.com/livekit/protocol/auth"
)

func TestVerifier(t *testing.T) {
	apiKey := "APID3B67uxk4Nj2GKiRPibAZ9"
	secret := "YHC-CUhbQhGeVCaYgn1BNA++"
	accessToken := "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJleHAiOjE2MDg5MzAzMDgsImlzcyI6IkFQSUQzQjY3dXhrNE5qMkdLaVJQaWJBWjkiLCJuYmYiOjE2MDg5MjY3MDgsInJvb21fam9pbiI6dHJ1ZSwicm9vbV9zaWQiOiJteWlkIiwic3ViIjoiQVBJRDNCNjd1eGs0TmoyR0tpUlBpYkFaOSJ9.cmHEBq0MLyRqphmVLM2cLXg5ao5Sro7am8yXhcYKcwE"
	t.Run("cannot decode with incorrect key", func(t *testing.T) {
		v, err := auth.ParseAPIToken(accessToken)
		require.NoError(t, err)

		require.Equal(t, apiKey, v.APIKey())
		_, _, err = v.Verify("")
		require.Error(t, err)

		_, _, err = v.Verify("anothersecret")
		require.Error(t, err)
	})

	t.Run("key has expired", func(t *testing.T) {
		v, err := auth.ParseAPIToken(accessToken)
		require.NoError(t, err)

		_, _, err = v.Verify(secret)
		require.Error(t, err)
	})

	t.Run("unexpired token is verified", func(t *testing.T) {
		claim := auth.VideoGrant{RoomCreate: true}
		at := auth.NewAccessToken(apiKey, secret).
			SetVideoGrant(&claim).
			SetValidFor(time.Minute).
			SetIdentity("me")
		authToken, err := at.ToJWT()
		require.NoError(t, err)

		v, err := auth.ParseAPIToken(authToken)
		require.NoError(t, err)
		require.Equal(t, apiKey, v.APIKey())
		require.Equal(t, "me", v.Identity())

		_, decoded, err := v.Verify(secret)
		require.NoError(t, err)
		require.Equal(t, &claim, decoded.Video)
	})

	t.Run("ensure metadata can be passed through", func(t *testing.T) {
		metadata := map[string]interface{}{
			"user":   "value",
			"number": float64(3),
		}
		md, _ := json.Marshal(metadata)
		attrs := map[string]string{"mykey": "myval", "secondkey": "secondval"}
		at := auth.NewAccessToken(apiKey, secret).
			SetVideoGrant(&auth.VideoGrant{
				RoomAdmin: true,
				Room:      "myroom",
			}).
			SetMetadata(string(md)).
			SetAttributes(attrs)

		authToken, err := at.ToJWT()
		require.NoError(t, err)

		v, err := auth.ParseAPIToken(authToken)
		require.NoError(t, err)

		_, decoded, err := v.Verify(secret)
		require.NoError(t, err)

		require.EqualValues(t, string(md), decoded.Metadata)
		require.EqualValues(t, attrs, decoded.Attributes)
	})

	t.Run("nil permissions are handled", func(t *testing.T) {
		grant := &auth.VideoGrant{
			Room:     "myroom",
			RoomJoin: true,
		}
		grant.SetCanPublishData(false)
		at := auth.NewAccessToken(apiKey, secret).
			SetVideoGrant(grant)
		token, err := at.ToJWT()
		require.NoError(t, err)

		v, err := auth.ParseAPIToken(token)
		require.NoError(t, err)
		_, decoded, err := v.Verify(secret)
		require.NoError(t, err)

		require.Nil(t, decoded.Video.CanSubscribe)
		require.Nil(t, decoded.Video.CanPublish)
		require.False(t, *decoded.Video.CanPublishData)
	})
}
