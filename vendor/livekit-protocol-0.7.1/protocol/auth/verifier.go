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

	"github.com/go-jose/go-jose/v3/jwt"
)

type APIKeyTokenVerifier struct {
	token    *jwt.JSONWebToken
	identity string
	apiKey   string
}

// ParseAPIToken parses an encoded JWT token and
func ParseAPIToken(raw string) (*APIKeyTokenVerifier, error) {
	tok, err := jwt.ParseSigned(raw)
	if err != nil {
		return nil, err
	}

	out := jwt.Claims{}
	if err := tok.UnsafeClaimsWithoutVerification(&out); err != nil {
		return nil, err
	}

	v := &APIKeyTokenVerifier{
		token:    tok,
		apiKey:   out.Issuer,
		identity: out.Subject,
	}
	if v.identity == "" {
		v.identity = out.ID
	}
	return v, nil
}

// APIKey returns the API key this token was signed with
func (v *APIKeyTokenVerifier) APIKey() string {
	return v.apiKey
}

func (v *APIKeyTokenVerifier) Identity() string {
	return v.identity
}

func (v *APIKeyTokenVerifier) Verify(key interface{}) (*jwt.Claims, *ClaimGrants, error) {
	if key == nil || key == "" {
		return nil, nil, ErrKeysMissing
	}
	if s, ok := key.(string); ok {
		key = []byte(s)
	}
	out := jwt.Claims{}
	claims := ClaimGrants{}
	if err := v.token.Claims(key, &out, &claims); err != nil {
		return nil, nil, err
	}
	if err := out.Validate(jwt.Expected{Issuer: v.apiKey, Time: time.Now()}); err != nil {
		return nil, nil, err
	}

	// copy over identity
	claims.Identity = v.identity
	return &out, &claims, nil
}
