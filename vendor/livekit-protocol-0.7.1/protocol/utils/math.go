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

	"golang.org/x/exp/constraints"
)

func LogisticFunc(x0, L, k float64) func(x float64) float64 {
	return func(x float64) float64 {
		return L / (1 + math.Exp(-k*(x-x0)))
	}
}

func FastLogisticFunc(x0, L, k float64) func(x float64) float64 {
	return func(x float64) float64 {
		return L / 2 * (1 + k*(x-x0)/(1+math.Abs(k*(x-x0))))
	}
}

func Abs[T constraints.Signed | constraints.Float](v T) T {
	return max(v, -v)
}
