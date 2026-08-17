/*
 * Copyright (C) 2019 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_CONTAINERS_STRING_POOL_H_
#define SRC_TRACE_PROCESSOR_CONTAINERS_STRING_POOL_H_

#include <array>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <functional>
#include <memory>
#include <mutex>
#include <optional>
#include <utility>
#include <vector>

#include "perfetto/base/logging.h"
#include "perfetto/base/thread_annotations.h"
#include "perfetto/ext/base/flat_hash_map.h"
#include "perfetto/ext/base/hash.h"
#include "perfetto/ext/base/string_view.h"
#include "perfetto/public/compiler.h"
#include "src/trace_processor/containers/null_term_string_view.h"

namespace perfetto::trace_processor {

// Interns strings in a string pool and hands out compact StringIds which can
// be used to retrieve the string in O(1).
class StringPool {
 private:
  struct Block;

  // StringPool IDs are 32-bit. If the MSB is 1, the remaining bits of the ID
  // are an index into the |large_strings_| vector. Otherwise, the next 6 bits
  // are the index of the Block in the pool, and the remaining 25 bits the
  // offset of the encoded string inside the pool.
  //
  // [31] [30:25] [24:0]
  //  |      |       |
  //  |      |       +---- offset in block (or LSB of large string index).
  //  |      +------------ block index (or MSB of large string index).
  //  +------------------- 1: large string, 0: string in a Block.
  static constexpr size_t kNumBlockIndexBits = 6;
  static constexpr uint32_t kNumBlockOffsetBits = 25;

  static constexpr size_t kLargeStringFlagBitMask = 1u << 31;
  static constexpr size_t kBlockOffsetBitMask = (1u << kNumBlockOffsetBits) - 1;
  static constexpr size_t kBlockIndexBitMask =
      0xffffffff & ~kLargeStringFlagBitMask & ~kBlockOffsetBitMask;

  static constexpr uint32_t kBlockSizeBytes = kBlockOffsetBitMask + 1;  // 32 MB

  static constexpr uint32_t kMaxBlockCount = 1u << kNumBlockIndexBits;

  static constexpr size_t kMinLargeStringSizeBytes = kBlockSizeBytes / 8;

 public:
  struct Id {
    Id() = default;

    constexpr bool operator==(const Id& other) const { return other.id == id; }
    constexpr bool operator!=(const Id& other) const {
      return !(other == *this);
    }
    constexpr bool operator<(const Id& other) const { return id < other.id; }

    PERFETTO_ALWAYS_INLINE constexpr bool is_null() const { return id == 0u; }

    PERFETTO_ALWAYS_INLINE constexpr bool is_large_string() const {
      return id & kLargeStringFlagBitMask;
    }

    PERFETTO_ALWAYS_INLINE constexpr uint32_t block_offset() const {
      return id & kBlockOffsetBitMask;
    }

    PERFETTO_ALWAYS_INLINE constexpr uint32_t block_index() const {
      return (id & kBlockIndexBitMask) >> kNumBlockOffsetBits;
    }

    PERFETTO_ALWAYS_INLINE constexpr uint32_t large_string_index() const {
      PERFETTO_DCHECK(is_large_string());
      return id & ~kLargeStringFlagBitMask;
    }

    PERFETTO_ALWAYS_INLINE constexpr uint32_t raw_id() const { return id; }

    static constexpr Id LargeString(size_t index) {
      PERFETTO_DCHECK(index <= static_cast<uint32_t>(index));
      PERFETTO_DCHECK(!(index & kLargeStringFlagBitMask));
      return Id(kLargeStringFlagBitMask | static_cast<uint32_t>(index));
    }

    PERFETTO_ALWAYS_INLINE static constexpr Id BlockString(size_t index,
                                                           uint32_t offset) {
      PERFETTO_DCHECK(index < (1u << (kNumBlockIndexBits + 1)));
      PERFETTO_DCHECK(offset < (1u << (kNumBlockOffsetBits + 1)));
      return Id(~kLargeStringFlagBitMask &
                (static_cast<uint32_t>(index << kNumBlockOffsetBits) |
                 (offset & kBlockOffsetBitMask)));
    }

    PERFETTO_ALWAYS_INLINE static constexpr Id Raw(uint32_t raw) {
      return Id(raw);
    }

    PERFETTO_ALWAYS_INLINE static constexpr Id Null() { return Id(0u); }

   private:
    constexpr explicit Id(uint32_t i) : id(i) {}

    uint32_t id;
  };

  // Iterator over all the small strings in the pool.
  class SmallStringIterator {
   public:
    explicit operator bool() const { return current_block_ptr_ != nullptr; }
    SmallStringIterator& operator++() {
      // Find the size of the string at the current offset in the block
      // and increment the offset by that size.
      uint32_t str_size = 0;
      current_block_ptr_ = ReadSize(current_block_ptr_, &str_size);
      current_block_ptr_ += str_size + 1;

      // If we're out of bounds for this block, go to the start of the next
      // block.
      const uint8_t* current_block_end = block_end_ptrs_[current_block_index_];
      PERFETTO_DCHECK(current_block_ptr_ <= current_block_end);
      if (current_block_ptr_ == current_block_end) {
        current_block_ptr_ = block_start_ptrs_[++current_block_index_];
      }
      return *this;
    }

    NullTermStringView StringView() {
      const uint8_t* current_block_start_ptr =
          block_start_ptrs_[current_block_index_];
      if (current_block_index_ == 0 &&
          current_block_ptr_ == current_block_start_ptr) {
        return {};
      }
      return GetFromBlockPtr(current_block_ptr_);
    }

    Id StringId() {
      const uint8_t* current_block_start_ptr =
          block_start_ptrs_[current_block_index_];

      // If we're at (0, 0), we have the null string which has id 0.
      if (current_block_index_ == 0 &&
          current_block_ptr_ == current_block_start_ptr) {
        return Id::Null();
      }
      return Id::BlockString(
          current_block_index_,
          static_cast<uint32_t>(current_block_ptr_ - current_block_start_ptr));
    }

   private:
    friend class StringPool;

    explicit SmallStringIterator(
        std::array<const uint8_t*, kMaxBlockCount> block_start_ptrs,
        std::array<const uint8_t*, kMaxBlockCount> block_end_ptrs)
        : block_start_ptrs_(block_start_ptrs), block_end_ptrs_(block_end_ptrs) {
      current_block_ptr_ = block_start_ptrs[0];
    }

    std::array<const uint8_t*, kMaxBlockCount> block_start_ptrs_;
    std::array<const uint8_t*, kMaxBlockCount> block_end_ptrs_;
    uint32_t current_block_index_ = 0;
    const uint8_t* current_block_ptr_ = nullptr;
  };

  StringPool();

  // Interns `str` in the string pool and returns the canonical id of that
  // string.
  Id InternString(base::StringView str) {
    if (str.data() == nullptr) {
      return Id::Null();
    }
    auto hash = str.Hash();

    // Perform a hashtable insertion with a null ID just to check if the string
    // is already inserted. If it's not, overwrite 0 with the actual Id.
    MaybeLockGuard guard{mutex_, should_acquire_mutex_};
    auto [id, inserted] = string_index_.Insert(hash, Id());
    if (PERFETTO_LIKELY(!inserted)) {
      PERFETTO_DCHECK(Get(*id) == str);
      return *id;
    }
    *id = InsertString(str);
    return *id;
  }

  // Given a string, returns the id for the string if it exists in the string
  // pool or std::nullopt otherwise.
  std::optional<Id> GetId(base::StringView str) const {
    if (str.data() == nullptr) {
      return Id::Null();
    }
    auto hash = str.Hash();

    MaybeLockGuard guard{mutex_, should_acquire_mutex_};
    Id* id = string_index_.Find(hash);
    if (id) {
      PERFETTO_DCHECK(Get(*id) == str);
      return *id;
    }
    return std::nullopt;
  }

  // Given a StringId, returns the string for that id.
  //
  // Implementation warning: this function is *extremely* performance sensitive
  // and as such *must* remain lock free (at least for small strings).
  PERFETTO_ALWAYS_INLINE NullTermStringView Get(Id id) const {
    if (PERFETTO_UNLIKELY(id.is_null())) {
      return {};
    }
    if (PERFETTO_UNLIKELY(id.is_large_string())) {
      return GetLargeString(id);
    }
    // Warning: do not introduce any locks here, this is a hyper fast-path.
    return GetFromBlockPtr(IdToPtr(id));
  }

  SmallStringIterator CreateSmallStringIterator() const {
    MaybeLockGuard guard{mutex_, should_acquire_mutex_};
    std::array<const uint8_t*, kMaxBlockCount> block_start_ptrs;
    for (uint32_t i = 0; i < kMaxBlockCount; ++i) {
      block_start_ptrs[i] = blocks_[i].get();
    }
    std::array<const uint8_t*, kMaxBlockCount> block_end_ptrs;
    for (uint32_t i = 0; i < kMaxBlockCount; ++i) {
      block_end_ptrs[i] = block_end_ptrs_[i];
    }
    return SmallStringIterator(block_start_ptrs, block_end_ptrs);
  }

  size_t size() const {
    MaybeLockGuard guard{mutex_, should_acquire_mutex_};
    return string_index_.size();
  }

  // Maximum id of a small string in the string pool.
  StringPool::Id MaxSmallStringId() const {
    MaybeLockGuard guard{mutex_, should_acquire_mutex_};
    const auto* block_start = blocks_[block_index_].get();
    const auto* block_end = block_end_ptrs_[block_index_];
    return Id::BlockString(block_index_,
                           static_cast<uint32_t>(block_end - block_start));
  }

  // Returns whether there is at least one large string in a string pool
  bool HasLargeString() const {
    MaybeLockGuard guard{mutex_, should_acquire_mutex_};
    return !large_strings_.empty();
  }

  // Sets the locking mode of the string pool.
  void set_locking(bool should_lock) { should_acquire_mutex_ = should_lock; }

 private:
  using StringHash = uint64_t;

  // Helper class to allow for an optionally acquiring a mutex while being
  // correctly annotated with all the annotations to make clang annotations
  // happy.
  struct PERFETTO_SCOPED_LOCKABLE MaybeLockGuard {
    explicit MaybeLockGuard(std::mutex& mutex, bool should_acquire)
        PERFETTO_EXCLUSIVE_LOCK_FUNCTION(mutex)
        : mutex_(mutex), should_acquire_(should_acquire) {
      if (PERFETTO_UNLIKELY(should_acquire_)) {
        mutex.lock();
      }
    }
    ~MaybeLockGuard() PERFETTO_UNLOCK_FUNCTION() {
      if (PERFETTO_UNLIKELY(should_acquire_)) {
        mutex_.unlock();
      }
    }
    std::mutex& mutex_;
    bool should_acquire_;
  };

  friend class Iterator;
  friend class StringPoolTest;

  // Number of bytes to reserve for size and null terminator.
  static constexpr uint8_t kMetadataSize = 5;

  // Inserts the string and return its Id.
  Id InsertString(base::StringView) PERFETTO_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

  // Inserts the string and return its Id.
  Id InsertLargeString(base::StringView)
      PERFETTO_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

  // Inserts the string into the current block and returns offset in the block.
  uint32_t InsertInCurrentBlock(base::StringView str)
      PERFETTO_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

  // The returned pointer points to the start of the string metadata (i.e. the
  // first byte of the size).
  PERFETTO_ALWAYS_INLINE const uint8_t* IdToPtr(Id id) const
      PERFETTO_LOCKS_EXCLUDED(mutex_) {
    PERFETTO_DCHECK(!id.is_large_string());

    // Warning: this function is *extremely performance sensitive and as such
    // *must not* take any locks.

    // Note: this read is *safe* because we cannot have an id pointing to a
    // block which was not initialized and once a block is initialized, it's
    // never touched again for the lifetime of the string pool.
    const uint8_t* ptr =
        PERFETTO_TS_UNCHECKED_READ(blocks_)[id.block_index()].get();
    return ptr + id.block_offset();
  }

  // |ptr| should point to the start of the string metadata (i.e. the first byte
  // of the size).
  // Returns pointer to the start of the string.
  PERFETTO_ALWAYS_INLINE static const uint8_t* ReadSize(const uint8_t* ptr,
                                                        uint32_t* size) {
    memcpy(size, ptr, sizeof(uint32_t));
    return ptr + sizeof(uint32_t);
  }

  // |ptr| should point to the start of the string size.
  PERFETTO_ALWAYS_INLINE static NullTermStringView GetFromBlockPtr(
      const uint8_t* ptr) {
    uint32_t size = 0;
    const uint8_t* str_ptr = ReadSize(ptr, &size);
    return {reinterpret_cast<const char*>(str_ptr), size};
  }

  // Lookup a string in the |large_strings_| vector. |id| should have the MSB
  // set.
  PERFETTO_NO_INLINE NullTermStringView GetLargeString(Id id) const {
    PERFETTO_DCHECK(id.is_large_string());

    MaybeLockGuard guard{mutex_, should_acquire_mutex_};
    size_t index = id.large_string_index();
    PERFETTO_DCHECK(index < large_strings_.size());
    const auto& [ptr, size] = large_strings_[index];
    return {ptr.get(), size};
  }

  mutable std::mutex mutex_;

  // Returns whether to actually take a lock on the `mutex_` when operating on
  // data structures which require it.
  bool should_acquire_mutex_ = false;

  // The actual memory storing the strings.
  std::array<std::unique_ptr<uint8_t[]>, kMaxBlockCount> blocks_
      PERFETTO_GUARDED_BY(mutex_);

  // The current end of block pointers for each block.
  std::array<uint8_t*, kMaxBlockCount> block_end_ptrs_
      PERFETTO_GUARDED_BY(mutex_){};

  // Any string that is too large to fit into a Block is stored separately
  std::vector<std::pair<std::unique_ptr<char[]>, size_t>> large_strings_
      PERFETTO_GUARDED_BY(mutex_);

  // The block we are currently pointing at.
  uint32_t block_index_ PERFETTO_GUARDED_BY(mutex_) = 0;

  // Maps hashes of strings to the Id in the string pool.
  base::FlatHashMap<StringHash,
                    Id,
                    base::AlreadyHashed<StringHash>,
                    base::LinearProbe,
                    /*AppendOnly=*/true>
      string_index_ PERFETTO_GUARDED_BY(mutex_){/*initial_capacity=*/4096u};
};

}  // namespace perfetto::trace_processor

template <>
struct std::hash<::perfetto::trace_processor::StringPool::Id> {
  using argument_type = ::perfetto::trace_processor::StringPool::Id;
  using result_type = size_t;

  result_type operator()(const argument_type& r) const {
    return std::hash<uint32_t>{}(r.raw_id());
  }
};

#endif  // SRC_TRACE_PROCESSOR_CONTAINERS_STRING_POOL_H_
