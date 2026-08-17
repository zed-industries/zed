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
	"sort"
	"strconv"
	"testing"

	"github.com/stretchr/testify/require"
)

func TestParallel(t *testing.T) {
	for _, cs := range []int{1, 10, 100, 10000} {
		t.Run(strconv.Itoa(cs), func(t *testing.T) {
			sendSlice := make([]int, 0, cs)
			recvSlice := make([]int, 0, cs)
			recvCh := make(chan int, cs)
			for i := 0; i < cs; i++ {
				sendSlice = append(sendSlice, i)
			}

			ParallelExec(sendSlice, 10, 2, func(i int) {
				recvCh <- i
			})
			close(recvCh)

			for i := range recvCh {
				recvSlice = append(recvSlice, i)
			}

			sort.Ints(recvSlice)

			require.Equal(t, sendSlice, recvSlice)
		})
	}
}
