package utils

import (
	"slices"
	"sync"
	"time"

	"github.com/livekit/protocol/utils/mono"
)

const (
	defaultTTL = 3 * time.Second
)

type timeSizeCacheItem[T any] struct {
	addedAt int64
	item    *T
	size    int
}

type TimeSizeCacheParams struct {
	TTL     time.Duration
	MaxSize int
}

type TimeSizeCache[T any] struct {
	params TimeSizeCacheParams

	lock      sync.Mutex
	items     []timeSizeCacheItem[T]
	itemsSize int
}

func NewTimeSizeCache[T any](params TimeSizeCacheParams) *TimeSizeCache[T] {
	return &TimeSizeCache[T]{
		params: params,
	}
}

func (t *TimeSizeCache[T]) Add(item *T, size int) {
	t.AddAt(item, size, mono.UnixNano())
}

func (t *TimeSizeCache[T]) AddAt(item *T, size int, atNano int64) {
	t.lock.Lock()
	defer t.lock.Unlock()

	t.items = append(t.items, timeSizeCacheItem[T]{
		addedAt: atNano,
		item:    item,
		size:    size,
	})
	t.itemsSize += size

	// prune if size exceeds threshold
	if t.params.MaxSize != 0 && t.itemsSize > t.params.MaxSize {
		idx := 0
		for t.itemsSize > t.params.MaxSize {
			t.itemsSize -= t.items[idx].size
			idx++
		}
		t.items = slices.Delete(t.items, 0, idx)
	}
}

func (t *TimeSizeCache[T]) Get() []*T {
	t.lock.Lock()
	defer t.lock.Unlock()

	items := make([]*T, len(t.items))
	for idx, tsci := range t.items {
		items[idx] = tsci.item
	}

	return items
}

func (t *TimeSizeCache[T]) Prune() {
	t.PruneAt(mono.UnixNano())
}

func (t *TimeSizeCache[T]) PruneAt(atNano int64) {
	t.lock.Lock()
	defer t.lock.Unlock()

	// age based pruning
	threshold := atNano - t.getTTL().Nanoseconds()
	for idx, item := range t.items {
		if item.addedAt >= threshold {
			t.items = slices.Delete(t.items, 0, idx)
			return
		}
		t.itemsSize -= item.size
	}

	// all items have aged
	t.items = nil
	t.itemsSize = 0
}

func (t *TimeSizeCache[T]) getTTL() time.Duration {
	ttl := t.params.TTL
	if ttl == 0 {
		ttl = defaultTTL
	}
	return ttl
}
