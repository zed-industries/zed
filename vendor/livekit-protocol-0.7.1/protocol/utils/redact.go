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
	"fmt"
	"regexp"
	"strings"
)

// rtmp urls must be of format rtmp(s)://{host}(/{path})/{app}/{stream_key}( live=1)
var rtmpRegexp = regexp.MustCompile("^(rtmps?:\\/\\/)(.*\\/)(.*\\/)(\\S*)( live=1)?$")

func RedactStreamKey(url string) (string, bool) {
	match := rtmpRegexp.FindStringSubmatch(url)
	if len(match) != 6 {
		return url, false
	}

	match[4] = RedactIdentifier(match[4])
	return strings.Join(match[1:], ""), true
}

func RedactIdentifier(identifier string) string {
	var prefix, suffix string
	for i := 3; i > 0; i-- {
		if len(identifier) >= i*3 {
			prefix = identifier[:i]
			suffix = identifier[len(identifier)-i:]
			break
		}
	}

	return fmt.Sprintf("{%s...%s}", prefix, suffix)
}

func Redact(s, name string) string {
	if s != "" {
		return name
	}
	return ""
}
