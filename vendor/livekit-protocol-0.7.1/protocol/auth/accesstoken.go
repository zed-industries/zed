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
	"time"

	"github.com/go-jose/go-jose/v3"
	"github.com/go-jose/go-jose/v3/jwt"

	"github.com/livekit/protocol/livekit"
)

const (
	defaultValidDuration = 6 * time.Hour
)

// AccessToken produces token signed with API key and secret
type AccessToken struct {
	apiKey                    string
	secret                    string
	grant                     ClaimGrants
	validFor                  time.Duration
	allowSensitiveCredentials bool
}

func NewAccessToken(key string, secret string) *AccessToken {
	return &AccessToken{
		apiKey: key,
		secret: secret,
	}
}

func (t *AccessToken) SetIdentity(identity string) *AccessToken {
	t.grant.Identity = identity
	return t
}

func (t *AccessToken) SetValidFor(duration time.Duration) *AccessToken {
	t.validFor = duration
	return t
}

func (t *AccessToken) SetName(name string) *AccessToken {
	t.grant.Name = name
	return t
}

func (t *AccessToken) SetKind(kind livekit.ParticipantInfo_Kind) *AccessToken {
	t.grant.SetParticipantKind(kind)
	return t
}

func (t *AccessToken) SetKindDetail(details ...livekit.ParticipantInfo_KindDetail) *AccessToken {
	t.grant.SetKindDetail(details...)
	return t
}

// Deprecated: use SetVideoGrant instead
func (t *AccessToken) AddGrant(grant *VideoGrant) *AccessToken {
	return t.SetVideoGrant(grant)
}

func (t *AccessToken) SetVideoGrant(grant *VideoGrant) *AccessToken {
	t.grant.Video = grant
	return t
}

// Deprecated: use SetSIPGrant instead
func (t *AccessToken) AddSIPGrant(grant *SIPGrant) *AccessToken {
	return t.SetSIPGrant(grant)
}

func (t *AccessToken) SetSIPGrant(grant *SIPGrant) *AccessToken {
	t.grant.SIP = grant
	return t
}

func (t *AccessToken) SetAgentGrant(grant *AgentGrant) *AccessToken {
	t.grant.Agent = grant
	return t
}

func (t *AccessToken) SetInferenceGrant(grant *InferenceGrant) *AccessToken {
	t.grant.Inference = grant
	return t
}

func (t *AccessToken) SetObservabilityGrant(grant *ObservabilityGrant) *AccessToken {
	t.grant.Observability = grant
	return t
}

func (t *AccessToken) SetMetadata(md string) *AccessToken {
	t.grant.Metadata = md
	return t
}

func (t *AccessToken) SetAttributes(attrs map[string]string) *AccessToken {
	if len(attrs) == 0 {
		return t
	}
	if t.grant.Attributes == nil {
		t.grant.Attributes = make(map[string]string)
	}
	for k, v := range attrs {
		t.grant.Attributes[k] = v
	}
	return t
}

func (t *AccessToken) SetSha256(sha string) *AccessToken {
	t.grant.Sha256 = sha
	return t
}

func (t *AccessToken) SetRoomPreset(preset string) *AccessToken {
	t.grant.RoomPreset = preset
	return t
}

func (t *AccessToken) SetRoomConfig(config *livekit.RoomConfiguration) *AccessToken {
	if config == nil {
		t.grant.RoomConfig = nil
	} else {
		t.grant.RoomConfig = (*RoomConfiguration)(config)
	}
	return t
}

// SetAgents is a shortcut for setting agents in room configuration
func (t *AccessToken) SetAgents(agents ...*livekit.RoomAgentDispatch) *AccessToken {
	if t.grant.RoomConfig == nil {
		t.grant.RoomConfig = &RoomConfiguration{}
	}
	t.grant.RoomConfig.Agents = agents
	return t
}

// SetAllowSensitiveCredentials enables the token to contain sensitive credentials, by default it is disabled.
// When tokens are issued to end-users, it's not a good idea to issue sensitive data such as API keys/secrets in them
// JWT tokens are not encrypted, so anything that is issued in them can be read by anyone.
// When the tokens are used in a server environment (i.e. connecting from SIP or Agents), you can bypass the
// credentials check by enabling this option.
func (t *AccessToken) SetAllowSensitiveCredentials(allow bool) *AccessToken {
	t.allowSensitiveCredentials = allow
	return t
}

func (t *AccessToken) GetGrants() *ClaimGrants {
	return &t.grant
}

func (t *AccessToken) ToJWT() (string, error) {
	if t.apiKey == "" || t.secret == "" {
		return "", ErrKeysMissing
	}

	if t.grant.RoomConfig != nil && !t.allowSensitiveCredentials {
		if err := t.grant.RoomConfig.CheckCredentials(); err != nil {
			return "", err
		}
	}

	sig, err := jose.NewSigner(jose.SigningKey{Algorithm: jose.HS256, Key: []byte(t.secret)},
		(&jose.SignerOptions{}).WithType("JWT"))
	if err != nil {
		return "", err
	}

	validFor := defaultValidDuration
	if t.validFor > 0 {
		validFor = t.validFor
	}

	cl := jwt.Claims{
		Issuer:    t.apiKey,
		NotBefore: jwt.NewNumericDate(time.Now()),
		Expiry:    jwt.NewNumericDate(time.Now().Add(validFor)),
		Subject:   t.grant.Identity,
	}
	return jwt.Signed(sig).Claims(cl).Claims(&t.grant).CompactSerialize()
}
