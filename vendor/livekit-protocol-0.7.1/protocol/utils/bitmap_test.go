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
	"math"
	"testing"

	"github.com/stretchr/testify/require"
)

func TestBitmap(t *testing.T) {
	b := NewBitmap[uint32](153)
	require.Equal(t, 4, cap(b.bits)) // rounded up to next power of 2, i. e. 256
	require.Equal(t, 4, len(b.bits))

	e := make([]uint64, 4)
	require.Equal(t, e, b.bits)

	b.Set(666666)
	require.True(t, b.IsSet(666666))
	require.True(t, b.IsSet(666666+256)) // aliased
	require.True(t, b.IsSet(666666-256)) // aliased
	require.False(t, b.IsSet(666666-1))
	require.False(t, b.IsSet(666666+1))

	copy(e, b.bits)
	b.Set(42) // same as above - aliased
	require.Equal(t, e, b.bits)

	// same slot range
	b.SetRange(24, 63)
	// different slot range
	b.SetRange(64, 240)
	require.False(t, b.IsSet(241))
	require.False(t, b.IsSet(23))
	require.True(t, b.IsSet(24))
	require.True(t, b.IsSet(240))
	e[0] = 0xFFFF_FFFF_FF00_0000
	e[1] = ^uint64(0)
	e[2] = ^uint64(0)
	e[3] = 0x0001_FFFF_FFFF_FFFF
	require.Equal(t, e, b.bits)

	b.Clear(6700) // aliases to 44
	e[0] = 0xFFFF_EFFF_FF00_0000
	require.Equal(t, e, b.bits)
	require.False(t, b.IsSet(44))

	// same slot range
	b.ClearRange(24, 34)
	e[0] = 0xFFFF_EFF8_0000_0000
	require.Equal(t, e, b.bits)
	require.False(t, b.IsSet(24))
	require.True(t, b.IsSet(35))

	// different slot range
	b.ClearRange(95, 234)
	e[1] = 0x0000_0000_7FFF_FFFF
	e[2] = 0x0
	e[3] = 0x0001_F800_0000_0000
	require.Equal(t, e, b.bits)
	require.True(t, b.IsSet(94))
	require.False(t, b.IsSet(95))
	require.False(t, b.IsSet(234))
	require.True(t, b.IsSet(235))

	// set large range
	b.SetRange(0, 1000)
	e[0] = 0xFFFF_FFFF_FFFF_FFFF
	e[1] = 0xFFFF_FFFF_FFFF_FFFF
	e[2] = 0xFFFF_FFFF_FFFF_FFFF
	e[3] = 0xFFFF_FFFF_FFFF_FFFF
	require.Equal(t, e, b.bits)

	// clear large range
	b.ClearRange(0, 1000)
	e[0] = 0x0000_0000_0000_0000
	e[1] = 0x0000_0000_0000_0000
	e[2] = 0x0000_0000_0000_0000
	e[3] = 0x0000_0000_0000_0000
	require.Equal(t, e, b.bits)

	// large range changes touch each word once
	sm, ls, rs, lo, ro := b.getSlotsAndOffsets(0, math.MaxUint32)
	require.Equal(t, []int{3, 0, 4, 0, 0}, []int{sm, ls, rs, lo, ro})
}

func BenchmarkBitmap(b *testing.B) {
	m := NewBitmap[uint64](128)
	b.Run("SetRange", func(b *testing.B) {
		for range b.N {
			m.SetRange(100, 1100)
		}
	})
	b.Run("Set", func(b *testing.B) {
		for i := range b.N {
			m.Set(uint64(i))
		}
	})
	b.Run("IsSet/Set", func(b *testing.B) {
		for i := range b.N {
			_ = m.IsSet(uint64(i))
			m.Set(uint64(i))
		}
	})
	b.Run("GetAndSet", func(b *testing.B) {
		for i := range b.N {
			m.GetAndSet(uint64(i))
		}
	})
}
