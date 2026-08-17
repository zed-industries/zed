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

	"github.com/stretchr/testify/require"
)

func TestDedupedSlice(t *testing.T) {
	t.Run("no_overflow", func(t *testing.T) {
		dsInt := NewDedupedSlice[int](10)
		for idx, v := range []int{1, 2, 3, 4, 6, 4, 7} {
			if idx == 5 {
				require.False(t, dsInt.Add(v))
			} else {
				require.True(t, dsInt.Add(v))
			}
		}
		require.EqualValues(t, []int{1, 2, 3, 4, 6, 7}, dsInt.Get())
		require.Equal(t, 6, dsInt.Len())
		require.True(t, dsInt.Has(2))
		require.False(t, dsInt.Has(8))

		dsString := NewDedupedSlice[string](10)
		for idx, v := range []string{"one", "two", "ten", "six", "six", "four"} {
			if idx == 4 {
				require.False(t, dsString.Add(v))
			} else {
				require.True(t, dsString.Add(v))
			}
		}
		require.EqualValues(t, []string{"one", "two", "ten", "six", "four"}, dsString.Get())
		require.NotEqualValues(t, []string{"one", "two", "ten", "six", "six", "four"}, dsString.Get())
		require.Equal(t, 5, dsString.Len())
		require.True(t, dsString.Has("two"))
		require.False(t, dsString.Has("eight"))

		dsString.Clear()
		require.Nil(t, dsString.Get())
		require.Equal(t, 0, dsString.Len())
	})

	t.Run("max_len", func(t *testing.T) {
		dsInt := NewDedupedSlice[int](5)
		for _, v := range []int{1, 2, 3, 4, 6, 4, 7} {
			dsInt.Add(v)
		}
		require.EqualValues(t, []int{2, 3, 4, 6, 7}, dsInt.Get())
		require.Equal(t, 5, dsInt.Len())
		require.True(t, dsInt.Has(2))
		require.False(t, dsInt.Has(1))
	})
}
