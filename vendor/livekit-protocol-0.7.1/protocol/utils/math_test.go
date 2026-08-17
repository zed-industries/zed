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
	"testing"
)

func BenchmarkLogisticFunc(b *testing.B) {
	b.Run("LogisticFunc", func(b *testing.B) {
		l := LogisticFunc(0, 1, 1)
		step := 4.0 / float64(b.N)
		var i float64
		for range b.N {
			_ = l(i)
			i += step
		}
	})
	b.Run("FastLogisticFunc", func(b *testing.B) {
		l := FastLogisticFunc(0, 1, 1)
		step := 4.0 / float64(b.N)
		var i float64
		for range b.N {
			_ = l(i)
			i += step
		}
	})
}
