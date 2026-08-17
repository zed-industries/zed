/*
 * Copyright 2023 LiveKit, Inc
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package utils

import (
	"sync"
	"time"

	"github.com/frostbyte73/core"
	"google.golang.org/protobuf/proto"
)

// ProtoProxy is a caching proxy for protobuf messages that may be expensive
// to compute. It is used to avoid unnecessary re-generation of Protobufs
type ProtoProxy[T proto.Message] struct {
	message         T
	updateFn        func() T
	fuse            core.Fuse
	updateChan      chan struct{}
	awaitChan       chan struct{}
	done            chan struct{}
	queueUpdate     chan struct{}
	dirty           bool
	refreshedAt     time.Time
	refreshInterval time.Duration
	lock            sync.RWMutex
}

// NewProtoProxy creates a new ProtoProxy that regenerates underlying values at a cadence of refreshInterval
// this should be used for updates that should be sent periodically, but does not have the urgency of immediate delivery
// updateFn should provide computations required to generate the protobuf
// if refreshInterval is 0, then proxy will only update on MarkDirty(true)
func NewProtoProxy[T proto.Message](refreshInterval time.Duration, updateFn func() T) *ProtoProxy[T] {
	p := &ProtoProxy[T]{
		updateChan:      make(chan struct{}, 1),
		updateFn:        updateFn,
		done:            make(chan struct{}),
		refreshInterval: refreshInterval,
		queueUpdate:     make(chan struct{}, 1),
	}
	p.performUpdate(true, time.Now())
	if refreshInterval > 0 {
		go p.worker()
	}
	return p
}

func (p *ProtoProxy[T]) MarkDirty(immediate bool) <-chan struct{} {
	p.lock.Lock()
	p.dirty = true
	shouldUpdate := immediate || time.Since(p.refreshedAt) > p.refreshInterval

	if p.awaitChan == nil {
		p.awaitChan = make(chan struct{})
	}
	awaitChan := p.awaitChan
	p.lock.Unlock()

	if shouldUpdate {
		select {
		case p.queueUpdate <- struct{}{}:
		default:
		}
	}
	return awaitChan
}

func (p *ProtoProxy[T]) Updated() <-chan struct{} {
	return p.updateChan
}

func (p *ProtoProxy[T]) Get() T {
	p.lock.RLock()
	defer p.lock.RUnlock()
	return CloneProto(p.message)
}

func (p *ProtoProxy[T]) Stop() {
	p.fuse.Break()

	// goroutine is not started when refreshInterval == 0
	if p.refreshInterval > 0 {
		<-p.done
	}

	p.lock.Lock()
	defer p.lock.Unlock()
	if awaitChan := p.awaitChan; awaitChan != nil {
		p.awaitChan = nil
		close(awaitChan)
	}
}

func (p *ProtoProxy[T]) performUpdate(skipNotify bool, refreshTime time.Time) {
	// set dirty back *before* calling updateFn because otherwise it could
	// wipe out another thread setting dirty to true while updateFn is executing
	p.lock.Lock()
	p.dirty = false

	if awaitChan := p.awaitChan; awaitChan != nil {
		p.awaitChan = nil
		defer close(awaitChan)
	}
	p.lock.Unlock()

	msg := p.updateFn()

	p.lock.Lock()
	if proto.Equal(p.message, msg) {
		// no change, skip the notification
		p.lock.Unlock()
		return
	}
	p.message = msg
	// only updating refreshedAt if we have notified, so it shouldn't push
	// out the next notification out by another interval
	p.refreshedAt = refreshTime
	p.lock.Unlock()

	if !skipNotify {
		select {
		case p.updateChan <- struct{}{}:
		default:
		}
	}
}

func (p *ProtoProxy[T]) worker() {
	ticker := time.NewTicker(p.refreshInterval)
	defer ticker.Stop()
	defer close(p.done)

	for {
		select {
		case <-p.fuse.Watch():
			return
		case now := <-ticker.C:
			p.lock.RLock()
			shouldUpdate := p.dirty && time.Since(p.refreshedAt) >= p.refreshInterval
			p.lock.RUnlock()
			if shouldUpdate {
				p.performUpdate(false, now)
			}
		case <-p.queueUpdate:
			p.performUpdate(false, time.Now())
		}
	}
}
