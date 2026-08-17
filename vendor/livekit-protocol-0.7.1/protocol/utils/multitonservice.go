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
	"sync"
)

type KillableService interface {
	Kill()
}

type MultitonService[K comparable] struct {
	mu sync.Mutex
	m  map[K]KillableService
}

func (s *MultitonService[K]) Replace(k K, v KillableService) func() {
	s.mu.Lock()
	defer s.mu.Unlock()

	if s.m == nil {
		s.m = map[K]KillableService{}
	}

	if prev, ok := s.m[k]; ok {
		prev.Kill()
	}

	s.m[k] = v

	return func() {
		s.mu.Lock()
		defer s.mu.Unlock()
		if s.m[k] == v {
			v.Kill()
			delete(s.m, k)
		}
	}
}

func (s *MultitonService[K]) Kill() {
	s.mu.Lock()
	defer s.mu.Unlock()
	for _, v := range s.m {
		v.Kill()
	}
	s.m = nil
}
