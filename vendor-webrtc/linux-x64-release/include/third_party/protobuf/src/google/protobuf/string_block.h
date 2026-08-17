// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd
//
// This file defines the internal StringBlock class

#ifndef GOOGLE_PROTOBUF_STRING_BLOCK_H__
#define GOOGLE_PROTOBUF_STRING_BLOCK_H__

#include <algorithm>
#include <cstddef>
#include <cstdint>
#include <string>

#include "absl/base/attributes.h"
#include "absl/log/absl_check.h"
#include "google/protobuf/arena_align.h"
#include "google/protobuf/port.h"

// Must be included last.
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {
namespace internal {

// StringBlock provides heap allocated, dynamically sized blocks (mini arenas)
// for allocating std::string instances. StringBlocks are allocated through
// the `New` function, and must be freed using the `Delete` function.
// StringBlocks are automatically sized from 256B to 8KB depending on the
// `next` instance provided in the `New` function to keep the average maximum
// unused space limited to 25%, or up to 4KB.
class alignas(std::string) StringBlock {
 public:
  StringBlock() = delete;
  StringBlock(const StringBlock&) = delete;
  StringBlock& operator=(const StringBlock&) = delete;

  // Returns the size of the next string block based on the size information
  // stored in `block`. `block` may be null in which case the size of the
  // initial string block is returned.
  static size_t NextSize(StringBlock* block);

  // Allocates a new StringBlock pointing to `next`, which can be null.
  // The size of the returned block depends on the allocated size of `next`.
  static StringBlock* New(StringBlock* next);

  // Allocates a new string block `in place`. `n` must be the value returned
  // from a previous call to `StringBlock::NextSize(next)`
  static StringBlock* Emplace(void* p, size_t n, StringBlock* next);

  // Deletes `block` if `block` is heap allocated. `block` must not be null.
  // Returns the allocated size of `block`, or 0 if the block was emplaced.
  static size_t Delete(StringBlock* block);

  StringBlock* next() const;

  // Returns the string instance at offset `offset`.
  // `offset` must be a multiple of sizeof(std::string), and be less than or
  // equal to `effective_size()`. `AtOffset(effective_size())` returns the
  // end of the allocated string instances and must not be de-referenced.
  ABSL_ATTRIBUTE_RETURNS_NONNULL std::string* AtOffset(size_t offset);

  // Returns a pointer to the first string instance in this block.
  ABSL_ATTRIBUTE_RETURNS_NONNULL std::string* begin();

  // Returns a pointer directly beyond the last string instance in this block.
  ABSL_ATTRIBUTE_RETURNS_NONNULL std::string* end();

  // Returns the total allocation size of this instance.
  size_t allocated_size() const { return allocated_size_; }

  // Returns true if this block is heap allocated, false if emplaced.
  bool heap_allocated() const { return heap_allocated_; }

  // Returns the effective size available for allocation string instances.
  // This value is guaranteed to be a multiple of sizeof(std::string), and
  // guaranteed to never be zero.
  size_t effective_size() const;

 private:
  using size_type = uint16_t;

  static_assert(alignof(std::string) <= sizeof(void*), "");
  static_assert(alignof(std::string) <= ArenaAlignDefault::align, "");

  ~StringBlock() = default;

  explicit StringBlock(StringBlock* next, bool heap_allocated, size_type size,
                       size_type next_size) noexcept
      : next_(next),
        allocated_size_(size),
        next_size_(next_size),
        heap_allocated_(heap_allocated) {}

  static constexpr size_type min_size() { return size_type{256}; }
  static constexpr size_type max_size() { return size_type{8192}; }

  // Returns `size` rounded down such that we can fit a perfect number
  // of std::string instances inside a StringBlock of that size.
  static constexpr size_type RoundedSize(size_type size);

  // Returns the size of the next block.
  size_t next_size() const { return next_size_; }

  StringBlock* const next_;
  const size_type allocated_size_;
  const size_type next_size_;
  const bool heap_allocated_;
};

constexpr StringBlock::size_type StringBlock::RoundedSize(size_type size) {
  return size - (size - sizeof(StringBlock)) % sizeof(std::string);
}

inline size_t StringBlock::NextSize(StringBlock* block) {
  return block ? block->next_size() : min_size();
}

inline StringBlock* StringBlock::Emplace(void* p, size_t n, StringBlock* next) {
  const auto count = static_cast<size_type>(n);
  ABSL_DCHECK_EQ(count, NextSize(next));
  size_type doubled = count * 2;
  size_type next_size = next ? std::min(doubled, max_size()) : min_size();
  return new (p) StringBlock(next, false, RoundedSize(count), next_size);
}

inline StringBlock* StringBlock::New(StringBlock* next) {
  // Compute required size, rounding down to a multiple of sizeof(std:string)
  // so that we can optimize the allocation path. I.e., we incur a (constant
  // size) MOD() operation cost here to avoid any MUL() later on.
  size_type size = min_size();
  size_type next_size = min_size();
  if (next) {
    size = next->next_size_;
    next_size = std::min<size_type>(size * 2, max_size());
  }
  size = RoundedSize(size);
  void* p = ::operator new(size);
  return new (p) StringBlock(next, true, size, next_size);
}

inline size_t StringBlock::Delete(StringBlock* block) {
  ABSL_DCHECK(block != nullptr);
  if (!block->heap_allocated_) return size_t{0};
  size_t size = block->allocated_size();
  internal::SizedDelete(block, size);
  return size;
}

inline StringBlock* StringBlock::next() const { return next_; }

inline size_t StringBlock::effective_size() const {
  return allocated_size_ - sizeof(StringBlock);
}

ABSL_ATTRIBUTE_RETURNS_NONNULL inline std::string* StringBlock::AtOffset(
    size_t offset) {
  ABSL_DCHECK_LE(offset, effective_size());
  return reinterpret_cast<std::string*>(reinterpret_cast<char*>(this + 1) +
                                        offset);
}

ABSL_ATTRIBUTE_RETURNS_NONNULL inline std::string* StringBlock::begin() {
  return AtOffset(0);
}

ABSL_ATTRIBUTE_RETURNS_NONNULL inline std::string* StringBlock::end() {
  return AtOffset(effective_size());
}

}  // namespace internal
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_STRING_BLOCK_H__
