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
	"io"

	"gopkg.in/yaml.v3"
)

type FileBasedKeyProvider struct {
	keys map[string]string
}

func NewFileBasedKeyProviderFromReader(r io.Reader) (p *FileBasedKeyProvider, err error) {
	keys := make(map[string]string)
	decoder := yaml.NewDecoder(r)
	if err = decoder.Decode(&keys); err != nil {
		return
	}
	p = &FileBasedKeyProvider{
		keys: keys,
	}

	return
}

func NewFileBasedKeyProviderFromMap(keys map[string]string) *FileBasedKeyProvider {
	return &FileBasedKeyProvider{
		keys: keys,
	}
}

func (p *FileBasedKeyProvider) GetSecret(key string) string {
	return p.keys[key]
}

func (p *FileBasedKeyProvider) NumKeys() int {
	return len(p.keys)
}

type SimpleKeyProvider struct {
	apiKey    string
	apiSecret string
}

func NewSimpleKeyProvider(apiKey, apiSecret string) *SimpleKeyProvider {
	return &SimpleKeyProvider{
		apiKey:    apiKey,
		apiSecret: apiSecret,
	}
}

func (p *SimpleKeyProvider) GetSecret(key string) string {
	if key == p.apiKey {
		return p.apiSecret
	}
	return ""
}

func (p *SimpleKeyProvider) NumKeys() int {
	return 1
}
