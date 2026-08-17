/*
 * Copyright (C) 2021 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef INCLUDE_PERFETTO_EXT_BASE_FLAT_HASH_MAP_H_
#define INCLUDE_PERFETTO_EXT_BASE_FLAT_HASH_MAP_H_

#include "perfetto/base/compiler.h"
#include "perfetto/base/logging.h"
#include "perfetto/ext/base/hash.h"
#include "perfetto/ext/base/utils.h"

#include <algorithm>
#include <limits>

namespace perfetto {
namespace base {

// An open-addressing hashmap implementation.
// Pointers are not stable, neither for keys nor values.
// Has similar performances of a RobinHood hash (without the complications)
// and 2x an unordered map.
// Doc: http://go/perfetto-hashtables .
//
// When used to implement a string pool in TraceProcessor, the performance
// characteristics obtained by replaying the set of strings seeen in a 4GB trace
// (226M strings, 1M unique) are the following (see flat_hash_map_benchmark.cc):
// This(Linear+AppendOnly)    879,383,676 ns    258.013M insertions/s
// This(LinearProbe):         909,206,047 ns    249.546M insertions/s
// This(QuadraticProbe):    1,083,844,388 ns    209.363M insertions/s
// std::unordered_map:      6,203,351,870 ns    36.5811M insertions/s
// tsl::robin_map:            931,403,397 ns    243.622M insertions/s
// absl::flat_hash_map:       998,013,459 ns    227.379M insertions/s
// FollyF14FastMap:         1,181,480,602 ns    192.074M insertions/s
//
// TODO(primiano): the table regresses for heavy insert+erase workloads since we
// don't clean up tombstones outside of resizes. In the limit, the entire
// table's capacity is made up of values/tombstones, so each search has to
// exhaustively scan the full capacity.

// The structs below define the probing algorithm used to probe slots upon a
// collision. They are guaranteed to visit all slots as our table size is always
// a power of two (see https://en.wikipedia.org/wiki/Quadratic_probing).

// Linear probing can be faster if the hashing is well distributed and the load
// is not high. For TraceProcessor's StringPool this is the fastest. It can
// degenerate badly if the hashing doesn't spread (e.g., if using directly pids
// as keys, with a no-op hashing function).
struct LinearProbe {
  static inline size_t Calc(size_t key_hash, size_t step, size_t capacity) {
    return (key_hash + step) & (capacity - 1);  // Linear probe
  }
};

// Generates the sequence: 0, 3, 10, 21, 36, 55, ...
// Can be a bit (~5%) slower than LinearProbe because it's less cache hot, but
// avoids degenerating badly if the hash function is bad and causes clusters.
// A good default choice unless benchmarks prove otherwise.
struct QuadraticProbe {
  static inline size_t Calc(size_t key_hash, size_t step, size_t capacity) {
    return (key_hash + 2 * step * step + step) & (capacity - 1);
  }
};

// Tends to perform in the middle between linear and quadratic.
// It's a bit more cache-effective than the QuadraticProbe but can create more
// clustering if the hash function doesn't spread well.
// Generates the sequence: 0, 1, 3, 6, 10, 15, 21, ...
struct QuadraticHalfProbe {
  static inline size_t Calc(size_t key_hash, size_t step, size_t capacity) {
    return (key_hash + (step * step + step) / 2) & (capacity - 1);
  }
};

template <typename Key,
          typename Value,
          typename Hasher = base::Hash<Key>,
          typename Probe = QuadraticProbe,
          bool AppendOnly = false>
class FlatHashMap {
 public:
  class Iterator {
   public:
    explicit Iterator(const FlatHashMap* map) : map_(map) { FindNextNonFree(); }
    ~Iterator() = default;
    Iterator(const Iterator&) = default;
    Iterator& operator=(const Iterator&) = default;
    Iterator(Iterator&&) noexcept = default;
    Iterator& operator=(Iterator&&) noexcept = default;

    Key& key() { return map_->keys_[idx_]; }
    Value& value() { return map_->values_[idx_]; }
    const Key& key() const { return map_->keys_[idx_]; }
    const Value& value() const { return map_->values_[idx_]; }

    explicit operator bool() const { return idx_ != kEnd; }
    Iterator& operator++() {
      PERFETTO_DCHECK(idx_ < map_->capacity_);
      ++idx_;
      FindNextNonFree();
      return *this;
    }

   private:
    static constexpr size_t kEnd = std::numeric_limits<size_t>::max();

    void FindNextNonFree() {
      const auto& tags = map_->tags_;
      for (; idx_ < map_->capacity_; idx_++) {
        if (tags[idx_] != kFreeSlot && (AppendOnly || tags[idx_] != kTombstone))
          return;
      }
      idx_ = kEnd;
    }

    const FlatHashMap* map_ = nullptr;
    size_t idx_ = 0;
  };  // Iterator

  static constexpr int kDefaultLoadLimitPct = 75;
  explicit FlatHashMap(size_t initial_capacity = 0,
                       int load_limit_pct = kDefaultLoadLimitPct)
      : load_limit_percent_(load_limit_pct) {
    if (initial_capacity > 0)
      Reset(initial_capacity);
  }

  // We are calling Clear() so that the destructors for the inserted entries are
  // called (unless they are trivial, in which case it will be a no-op).
  ~FlatHashMap() { Clear(); }

  FlatHashMap(FlatHashMap&& other) noexcept {
    tags_ = std::move(other.tags_);
    keys_ = std::move(other.keys_);
    values_ = std::move(other.values_);
    capacity_ = other.capacity_;
    size_ = other.size_;
    max_probe_length_ = other.max_probe_length_;
    load_limit_ = other.load_limit_;
    load_limit_percent_ = other.load_limit_percent_;

    new (&other) FlatHashMap();
  }

  FlatHashMap& operator=(FlatHashMap&& other) noexcept {
    this->~FlatHashMap();
    new (this) FlatHashMap(std::move(other));
    return *this;
  }

  FlatHashMap(const FlatHashMap&) = delete;
  FlatHashMap& operator=(const FlatHashMap&) = delete;

  std::pair<Value*, bool> Insert(Key key, Value value) {
    const size_t key_hash = Hasher{}(key);
    const uint8_t tag = HashToTag(key_hash);
    static constexpr size_t kSlotNotFound = std::numeric_limits<size_t>::max();

    // This for loop does in reality at most two attempts:
    // The first iteration either:
    //  - Early-returns, because the key exists already,
    //  - Finds an insertion slot and proceeds because the load is < limit.
    // The second iteration is only hit in the unlikely case of this insertion
    // bringing the table beyond the target |load_limit_| (or the edge case
    // of the HT being full, if |load_limit_pct_| = 100).
    // We cannot simply pre-grow the table before insertion, because we must
    // guarantee that calling Insert() with a key that already exists doesn't
    // invalidate iterators.
    size_t insertion_slot;
    size_t probe_len;
    for (;;) {
      PERFETTO_DCHECK((capacity_ & (capacity_ - 1)) == 0);  // Must be a pow2.
      insertion_slot = kSlotNotFound;
      // Start the iteration at the desired slot (key_hash % capacity_)
      // searching either for a free slot or a tombstone. In the worst case we
      // might end up scanning the whole array of slots. The Probe functions are
      // guaranteed to visit all the slots within |capacity_| steps. If we find
      // a free slot, we can stop the search immediately (a free slot acts as an
      // "end of chain for entries having the same hash". If we find a
      // tombstones (a deleted slot) we remember its position, but have to keep
      // searching until a free slot to make sure we don't insert a duplicate
      // key.
      for (probe_len = 0; probe_len < capacity_;) {
        const size_t idx = Probe::Calc(key_hash, probe_len, capacity_);
        PERFETTO_DCHECK(idx < capacity_);
        const uint8_t tag_idx = tags_[idx];
        ++probe_len;
        if (tag_idx == kFreeSlot) {
          // Rationale for "insertion_slot == kSlotNotFound": if we encountered
          // a tombstone while iterating we should reuse that rather than
          // taking another slot.
          if (AppendOnly || insertion_slot == kSlotNotFound)
            insertion_slot = idx;
          break;
        }
        // We should never encounter tombstones in AppendOnly mode.
        PERFETTO_DCHECK(!(tag_idx == kTombstone && AppendOnly));
        if (!AppendOnly && tag_idx == kTombstone) {
          insertion_slot = idx;
          continue;
        }
        if (tag_idx == tag && keys_[idx] == key) {
          // The key is already in the map.
          return std::make_pair(&values_[idx], false);
        }
      }  // for (idx)

      // If we got to this point the key does not exist (otherwise we would have
      // hit the return above) and we are going to insert a new entry.
      // Before doing so, ensure we stay under the target load limit.
      if (PERFETTO_UNLIKELY(size_ >= load_limit_)) {
        MaybeGrowAndRehash(/*grow=*/true);
        continue;
      }
      PERFETTO_DCHECK(insertion_slot != kSlotNotFound);
      break;
    }  // for (attempt)

    PERFETTO_CHECK(insertion_slot < capacity_);

    // We found a free slot (or a tombstone). Proceed with the insertion.
    Value* value_idx = &values_[insertion_slot];
    new (&keys_[insertion_slot]) Key(std::move(key));
    new (value_idx) Value(std::move(value));
    tags_[insertion_slot] = tag;
    PERFETTO_DCHECK(probe_len > 0 && probe_len <= capacity_);
    max_probe_length_ = std::max(max_probe_length_, probe_len);
    size_++;

    return std::make_pair(value_idx, true);
  }

  Value* Find(const Key& key) const {
    const size_t idx = FindInternal(key);
    if (idx == kNotFound)
      return nullptr;
    return &values_[idx];
  }

  bool Erase(const Key& key) {
    if (AppendOnly)
      PERFETTO_FATAL("Erase() not supported because AppendOnly=true");
    size_t idx = FindInternal(key);
    if (idx == kNotFound)
      return false;
    EraseInternal(idx);
    return true;
  }

  void Clear() {
    // Avoid trivial heap operations on zero-capacity std::move()-d objects.
    if (PERFETTO_UNLIKELY(capacity_ == 0))
      return;

    for (size_t i = 0; i < capacity_; ++i) {
      const uint8_t tag = tags_[i];
      if (tag != kFreeSlot && tag != kTombstone)
        EraseInternal(i);
    }
    // Clear all tombstones. We really need to do this for AppendOnly.
    MaybeGrowAndRehash(/*grow=*/false);
  }

  Value& operator[](Key key) {
    auto it_and_inserted = Insert(std::move(key), Value{});
    return *it_and_inserted.first;
  }

  Iterator GetIterator() { return Iterator(this); }
  const Iterator GetIterator() const { return Iterator(this); }

  size_t size() const { return size_; }
  size_t capacity() const { return capacity_; }

  // "protected" here is only for the flat_hash_map_benchmark.cc. Everything
  // below is by all means private.
 protected:
  enum ReservedTags : uint8_t { kFreeSlot = 0, kTombstone = 1 };
  static constexpr size_t kNotFound = std::numeric_limits<size_t>::max();

  size_t FindInternal(const Key& key) const {
    const size_t key_hash = Hasher{}(key);
    const uint8_t tag = HashToTag(key_hash);
    PERFETTO_DCHECK((capacity_ & (capacity_ - 1)) == 0);  // Must be a pow2.
    PERFETTO_DCHECK(max_probe_length_ <= capacity_);
    for (size_t i = 0; i < max_probe_length_; ++i) {
      const size_t idx = Probe::Calc(key_hash, i, capacity_);
      const uint8_t tag_idx = tags_[idx];

      if (tag_idx == kFreeSlot)
        return kNotFound;
      // HashToTag() never returns kTombstone, so the tag-check below cannot
      // possibly match. Also we just want to skip tombstones.
      if (tag_idx == tag && keys_[idx] == key) {
        PERFETTO_DCHECK(tag_idx > kTombstone);
        return idx;
      }
    }  // for (idx)
    return kNotFound;
  }

  void EraseInternal(size_t idx) {
    PERFETTO_DCHECK(tags_[idx] > kTombstone);
    PERFETTO_DCHECK(size_ > 0);
    tags_[idx] = kTombstone;
    keys_[idx].~Key();
    values_[idx].~Value();
    size_--;
  }

  PERFETTO_NO_INLINE void MaybeGrowAndRehash(bool grow) {
    PERFETTO_DCHECK(size_ <= capacity_);
    const size_t old_capacity = capacity_;

    // Grow quickly up to 1MB, then chill.
    const size_t old_size_bytes = old_capacity * (sizeof(Key) + sizeof(Value));
    const size_t grow_factor = old_size_bytes < (1024u * 1024u) ? 8 : 2;
    const size_t new_capacity =
        grow ? std::max(old_capacity * grow_factor, size_t(1024))
             : old_capacity;

    auto old_tags(std::move(tags_));
    auto old_keys(std::move(keys_));
    auto old_values(std::move(values_));
    size_t old_size = size_;

    // This must be a CHECK (i.e. not just a DCHECK) to prevent UAF attacks on
    // 32-bit archs that try to double the size of the table until wrapping.
    PERFETTO_CHECK(new_capacity >= old_capacity);
    Reset(new_capacity);

    size_t new_size = 0;  // Recompute the size.
    for (size_t i = 0; i < old_capacity; ++i) {
      const uint8_t old_tag = old_tags[i];
      if (old_tag != kFreeSlot && old_tag != kTombstone) {
        Insert(std::move(old_keys[i]), std::move(old_values[i]));
        old_keys[i].~Key();  // Destroy the old objects.
        old_values[i].~Value();
        new_size++;
      }
    }
    PERFETTO_DCHECK(new_size == old_size);
    size_ = new_size;
  }

  // Doesn't call destructors. Use Clear() for that.
  PERFETTO_NO_INLINE void Reset(size_t n) {
    PERFETTO_DCHECK((n & (n - 1)) == 0);  // Must be a pow2.

    capacity_ = n;
    max_probe_length_ = 0;
    size_ = 0;
    load_limit_ = n * static_cast<size_t>(load_limit_percent_) / 100;
    load_limit_ = std::min(load_limit_, n);

    tags_.reset(new uint8_t[n]);
    memset(&tags_[0], 0, n);                  // Clear all tags.
    keys_ = AlignedAllocTyped<Key[]>(n);      // Deliberately not 0-initialized.
    values_ = AlignedAllocTyped<Value[]>(n);  // Deliberately not 0-initialized.
  }

  static inline uint8_t HashToTag(size_t full_hash) {
    uint8_t tag = full_hash >> (sizeof(full_hash) * 8 - 8);
    // Ensure the hash is always >= 2. We use 0, 1 for kFreeSlot and kTombstone.
    tag += (tag <= kTombstone) << 1;
    PERFETTO_DCHECK(tag > kTombstone);
    return tag;
  }

  size_t capacity_ = 0;
  size_t size_ = 0;
  size_t max_probe_length_ = 0;
  size_t load_limit_ = 0;  // Updated every time |capacity_| changes.
  int load_limit_percent_ =
      kDefaultLoadLimitPct;  // Load factor limit in % of |capacity_|.

  // These arrays have always the |capacity_| elements.
  // Note: AlignedUniquePtr just allocates memory, doesn't invoke any ctor/dtor.
  std::unique_ptr<uint8_t[]> tags_;
  AlignedUniquePtr<Key[]> keys_;
  AlignedUniquePtr<Value[]> values_;
};

}  // namespace base
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_EXT_BASE_FLAT_HASH_MAP_H_
