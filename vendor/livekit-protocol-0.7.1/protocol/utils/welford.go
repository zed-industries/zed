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
)

// Welford implements Welford's online algorithm for variance
// SEE: https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance#Welford's_online_algorithm
type Welford struct {
	count float64
	mean  float64
	m2    float64
}

func (w *Welford) Reset() {
	w.count = 0
	w.mean = 0
	w.m2 = 0
}

func (w *Welford) Update(v float64) {
	w.count++
	d := v - w.mean
	w.mean += d / w.count
	d2 := v - w.mean
	w.m2 += d * d2
}

func (w Welford) Value() (mean, variance, sampleVariance float64) {
	if w.count < 2 {
		return w.mean, 0, 0
	}
	return w.mean, w.m2 / w.count, w.m2 / (w.count - 1)
}

func (w Welford) Count() float64 {
	return w.count
}

func (w Welford) Mean() float64 {
	return w.mean
}

func (w Welford) Variance() float64 {
	return w.m2 / (w.count - 1)
}

func (w Welford) StdDev() float64 {
	return math.Sqrt(w.Variance())
}

func WelfordMerge(ws ...Welford) Welford {
	switch len(ws) {
	case 0:
		return Welford{}
	case 1:
		return ws[0]
	case 2:
		if ws[0].count == 0 {
			return ws[1]
		}
		if ws[1].count == 0 {
			return ws[0]
		}
		count := ws[0].count + ws[1].count
		delta := ws[1].mean - ws[0].mean
		return Welford{
			count: count,
			mean:  (ws[0].mean*ws[0].count + ws[1].mean*ws[1].count) / count,
			m2:    ws[0].m2 + ws[1].m2 + delta*delta*ws[0].count*ws[1].count/count,
		}
	default:
		n := len(ws) >> 1
		return WelfordMerge(WelfordMerge(ws[:n]...), WelfordMerge(ws[n:]...))
	}
}
