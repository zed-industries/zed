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

package utils

import (
	"crypto/rand"
	"io"

	"github.com/jxskiss/base62"
)

func RandomSecret() string {
	// 256 bit secret
	buf := make([]byte, 32)
	_, err := io.ReadFull(rand.Reader, buf)
	// cannot error
	if err != nil {
		panic("could not read random")
	}
	return base62.EncodeToString(buf)
}
