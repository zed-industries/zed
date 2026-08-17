// Copyright 2024 LiveKit, Inc.
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
	"unsafe"

	"github.com/stretchr/testify/require"
)

type copyTestType struct {
	Vbool          bool
	Varray         [3]int
	Vslice         []int
	Vmap           map[int]int
	Vpointer       *copyTestType
	VunsafePointer unsafe.Pointer
	vunexported    bool
}

func TestDeepCopy(t *testing.T) {
	v := &copyTestType{
		Vbool:  true,
		Varray: [3]int{0, 1, 2},
		Vslice: []int{0, 1, 2},
		Vmap:   map[int]int{0: 0, 1: 1, 2: 2},
		Vpointer: &copyTestType{
			Vbool: true,
		},
	}

	require.EqualValues(t, v, DeepCopy(v))
}
