// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_GLYPH_OFFSET_ARRAY_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_GLYPH_OFFSET_ARRAY_H_

#include "third_party/blink/renderer/platform/fonts/shaping/glyph_data.h"
#include "third_party/blink/renderer/platform/fonts/shaping/glyph_data_range.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"

namespace blink {

// A array of glyph offsets. If all offsets are zero, we don't allocate
// storage for reducing memory usage.
class PLATFORM_EXPORT GlyphOffsetArray final {
  DISALLOW_NEW();

 public:
  GlyphOffsetArray() = default;
  GlyphOffsetArray(const GlyphOffsetArray& other) : storage_(other.storage_) {}

  // The `span` of `GlyphOffset` if `HasStorage()`, or an empty span.
  explicit operator base::span<const GlyphOffset>() const {
    return base::span{storage_};
  }

  // A return value of |GetOffsets()| to represent optional |GlyphOffset|
  // array.
  template <bool has_non_zero_glyph_offsets>
  struct iterator final {};

  template <bool has_non_zero_glyph_offsets>
  iterator<has_non_zero_glyph_offsets> GetIterator() const {
    return iterator<has_non_zero_glyph_offsets>(*this);
  }

  size_t ByteSize() const {
    return HasStorage() ? AllocatedSize() * sizeof(GlyphOffset) : 0;
  }

  void CopyFrom(const GlyphOffsetArray& other1,
                wtf_size_t size1,
                const GlyphOffsetArray& other2,
                wtf_size_t size2) {
    other1.CheckSize(size1);
    other2.CheckSize(size2);
    DCHECK(size1);
    DCHECK(size2);
    const wtf_size_t size = size1 + size2;
    if (other1.HasStorage()) {
      AllocateStorageIfNeeded(size);
      std::ranges::copy(other1.storage_, GetStorage());
    }
    if (other2.HasStorage()) {
      AllocateStorageIfNeeded(size);
      std::ranges::copy(other2.storage_, UNSAFE_TODO(GetStorage() + size1));
    }
  }

  void CopyFromRange(const GlyphDataRange& range) {
    if (!range.HasOffsets() || range.IsEmpty()) {
      storage_.clear();
      return;
    }
    AllocateStorage(range.size());
    std::ranges::copy(range.Offsets(), GetStorage());
  }

  GlyphOffset* GetStorage() { return storage_.data(); }
  const GlyphOffset* GetStorage() const { return storage_.data(); }
  bool HasStorage() const { return !storage_.empty(); }
  wtf_size_t AllocatedSize() const { return storage_.size(); }

  void Reverse() { storage_.Reverse(); }

  void Shrink(unsigned new_size) {
    DCHECK_GE(new_size, 1u);
    if (!HasStorage()) {
      return;
    }
    storage_.Shrink(new_size);
  }

  // Functions to change one element.
  void AddHeightAt(unsigned index, float delta, wtf_size_t size) {
    DCHECK_NE(delta, 0.0f);
    AllocateStorageIfNeeded(size);
    storage_[index].set_y(storage_[index].y() + delta);
  }

  void AddWidthAt(unsigned index, float delta, wtf_size_t size) {
    DCHECK_NE(delta, 0.0f);
    AllocateStorageIfNeeded(size);
    storage_[index].set_x(storage_[index].x() + delta);
  }

  void SetAt(unsigned index, GlyphOffset offset, wtf_size_t size) {
    if (!HasStorage()) {
      if (offset.IsZero()) {
        return;
      }
      AllocateStorage(size);
    }
    storage_[index] = offset;
  }

  void Trace(Visitor* visitor) const { visitor->Trace(storage_); }

 private:
  void CheckSize(wtf_size_t size) const {
    CHECK(!HasStorage() || size == AllocatedSize());
  }

  // Note: HarfBuzzShaperTest.ShapeVerticalUpright uses non-zero glyph offset.
  void AllocateStorage(wtf_size_t size) {
    DCHECK_GE(size, 1u);
    DCHECK(!HasStorage());
    storage_.resize(size);
  }

  void AllocateStorageIfNeeded(wtf_size_t size) {
    CheckSize(size);
    if (!HasStorage()) {
      AllocateStorage(size);
    }
  }

  HeapVector<GlyphOffset> storage_;
};

// For non-zero glyph offset array
template <>
struct GlyphOffsetArray::iterator<true> final {
  STACK_ALLOCATED();

 public:
  explicit iterator(base::span<const GlyphOffset> offsets)
      : iterator_(offsets.begin()) {
    // An empty span should use `has_non_zero_glyph_offsets = false`.
    DCHECK(!offsets.empty());
  }

  // The constructor for ShapeResult
  explicit iterator(const GlyphOffsetArray& array)
      : iterator(static_cast<base::span<const GlyphOffset>>(array)) {}

  // The constructor for ShapeResultView
  explicit iterator(const GlyphDataRange& range) : iterator(range.Offsets()) {}

  GlyphOffset operator*() const { return *iterator_; }
  void operator++() { ++iterator_; }
  void operator+=(ptrdiff_t s) { iterator_ += s; }

  GlyphOffset operator[](size_t i) const { return *(iterator_ + i); }

 private:
  base::span<const GlyphOffset>::iterator iterator_;
};

// For zero glyph offset array
template <>
struct GlyphOffsetArray::iterator<false> final {
  explicit iterator(const GlyphOffsetArray& array) {
    DCHECK(!array.HasStorage());
  }
  explicit iterator(const GlyphDataRange& range) {
    DCHECK(!range.HasOffsets());
  }

  GlyphOffset operator*() const { return GlyphOffset(); }
  void operator++() {}
  void operator+=(ptrdiff_t) {}
  GlyphOffset operator[](size_t) const { return GlyphOffset(); }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_GLYPH_OFFSET_ARRAY_H_
