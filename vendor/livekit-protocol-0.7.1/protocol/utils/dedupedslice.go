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

type DedupedSlice[T comparable] struct {
	maxLen int

	values []T
}

func NewDedupedSlice[T comparable](maxLen int) *DedupedSlice[T] {
	return &DedupedSlice[T]{
		maxLen: maxLen,
	}
}

func (d *DedupedSlice[T]) Add(val T) bool {
	for _, v := range d.values {
		if val == v {
			return false
		}
	}

	d.values = append(d.values, val)
	if len(d.values) > d.maxLen {
		d.values = d.values[len(d.values)-d.maxLen:]
	}
	return true
}

func (d *DedupedSlice[T]) Get() []T {
	return d.values
}

func (d *DedupedSlice[T]) Len() int {
	return len(d.values)
}

func (d *DedupedSlice[T]) Has(val T) bool {
	for _, v := range d.values {
		if v == val {
			return true
		}
	}

	return false
}

func (d *DedupedSlice[T]) Clear() {
	d.values = nil
}
