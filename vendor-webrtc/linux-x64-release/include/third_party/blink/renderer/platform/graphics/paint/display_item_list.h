// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_DISPLAY_ITEM_LIST_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_DISPLAY_ITEM_LIST_H_

#include <cstring>  // memcpy, memset
#include <type_traits>

#include "base/check_op.h"
#include "base/compiler_specific.h"
#include "third_party/blink/renderer/platform/graphics/paint/display_item.h"
#include "third_party/blink/renderer/platform/graphics/paint/scrollbar_display_item.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class JSONArray;

// A container for a list of display items of various types.
class PLATFORM_EXPORT DisplayItemList {
  DISALLOW_NEW();

 public:
  DisplayItemList() = default;
  ~DisplayItemList() { clear(); }

  DisplayItemList(const DisplayItemList&) = delete;
  DisplayItemList& operator=(const DisplayItemList&) = delete;
  DisplayItemList(DisplayItemList&&) = delete;
  DisplayItemList& operator=(DisplayItemList&&) = delete;

  void ReserveCapacity(wtf_size_t initial_capacity) {
    items_.reserve(initial_capacity);
  }

  void clear();

  // This private section is before the public APIs because some inline public
  // methods depend on the private definitions.
 private:
  // Declares itself as a forward iterator, but also supports a few more
  // things. The whole random access iterator interface is a bit much.
  template <typename BaseIterator, typename ItemType>
  class IteratorWrapper {
    DISALLOW_NEW();

   public:
    using iterator_category = std::forward_iterator_tag;
    using value_type = ItemType;
    using difference_type = std::ptrdiff_t;
    using pointer = ItemType*;
    using reference = ItemType&;

    IteratorWrapper() = default;
    explicit IteratorWrapper(const BaseIterator& it) : it_(it) {}

    bool operator==(const IteratorWrapper& other) const {
      return it_ == other.it_;
    }
    bool operator!=(const IteratorWrapper& other) const {
      return it_ != other.it_;
    }
    bool operator<(const IteratorWrapper& other) const {
      return it_ < other.it_;
    }
    ItemType& operator*() const { return reinterpret_cast<ItemType&>(*it_); }
    ItemType* operator->() const { return &operator*(); }
    UNSAFE_BUFFER_USAGE IteratorWrapper operator+(std::ptrdiff_t n) const {
      // SAFETY: required from caller, enforced by UNSAFE_BUFFER_USAGE.
      return UNSAFE_BUFFERS(IteratorWrapper(it_ + n));
    }
    UNSAFE_BUFFER_USAGE IteratorWrapper operator++(int) {
      IteratorWrapper tmp = *this;
      // SAFETY: required from caller, enforced by UNSAFE_BUFFER_USAGE
      UNSAFE_BUFFERS(++it_);
      return tmp;
    }
    std::ptrdiff_t operator-(const IteratorWrapper& other) const {
      return it_ - other.it_;
    }
    UNSAFE_BUFFER_USAGE IteratorWrapper& operator++() {
      // SAFETY: required from caller, enforced by UNSAFE_BUFFER_USAGE.
      UNSAFE_BUFFERS(++it_);
      return *this;
    }

   private:
    BaseIterator it_;
  };

  // kAlignment must be a multiple of alignof(derived display item) for each
  // derived display item; the ideal value is the least common multiple.
  // The validity of kAlignment and kMaxItemSize are checked in
  // AllocateAndConstruct().
  static constexpr wtf_size_t kAlignment = alignof(ScrollbarDisplayItem);
  static constexpr wtf_size_t kMaxItemSize = sizeof(ScrollbarDisplayItem);

  struct ItemSlot {
    alignas(kAlignment) uint8_t data[kMaxItemSize];
    DISALLOW_NEW();
  };
  using ItemVector = Vector<ItemSlot>;

 public:
  using value_type = DisplayItem;
  using iterator = IteratorWrapper<ItemVector::iterator, DisplayItem>;
  using const_iterator =
      IteratorWrapper<ItemVector::const_iterator, const DisplayItem>;
  iterator begin() { return iterator(items_.begin()); }
  iterator end() { return iterator(items_.end()); }
  const_iterator begin() const { return const_iterator(items_.begin()); }
  const_iterator end() const { return const_iterator(items_.end()); }

  UNSAFE_BUFFER_USAGE DisplayItem& front() { return *begin(); }
  UNSAFE_BUFFER_USAGE const DisplayItem& front() const { return *begin(); }
  UNSAFE_BUFFER_USAGE DisplayItem& back() {
    DCHECK(size());
    // SAFETY: required from caller, enforced by UNSAFE_BUFFER_USAGE
    return UNSAFE_BUFFERS((*this)[size() - 1]);
  }
  UNSAFE_BUFFER_USAGE const DisplayItem& back() const {
    DCHECK(size());
    // SAFETY: required from caller, enforced by UNSAFE_BUFFER_USAGE
    return UNSAFE_BUFFERS((*this)[size() - 1]);
  }

  UNSAFE_BUFFER_USAGE DisplayItem& operator[](wtf_size_t index) {
    // SAFETY: required from caller, enforced by UNSAFE_BUFFER_USAGE.
    return UNSAFE_BUFFERS(*(begin() + index));
  }

  UNSAFE_BUFFER_USAGE const DisplayItem& operator[](wtf_size_t index) const {
    // SAFETY: required from caller, enforced by UNSAFE_BUFFER_USAGE.
    UNSAFE_BUFFERS(return *(begin() + index));
  }

  wtf_size_t size() const { return items_.size(); }
  bool IsEmpty() const { return !size(); }

  size_t MemoryUsageInBytes() const {
    return sizeof(*this) + items_.CapacityInBytes();
  }

  // Useful for iterating with a range-based for loop.
  template <typename Iterator>
  class Range {
    DISALLOW_NEW();

   public:
    Range(const Iterator& begin, const Iterator& end)
        : begin_(begin), end_(end) {}
    Iterator begin() const { return begin_; }
    Iterator end() const { return end_; }
    wtf_size_t size() const {
      return base::checked_cast<wtf_size_t>(end_ - begin_);
    }

    // To meet the requirement of gmock ElementsAre().
    using value_type = DisplayItem;
    using const_iterator = DisplayItemList::const_iterator;

   private:
    Iterator begin_;
    Iterator end_;
  };

  // In most cases, we should use PaintChunkSubset::Iterator::DisplayItems()
  // instead of these.
  UNSAFE_BUFFER_USAGE Range<iterator> ItemsInRange(wtf_size_t begin_index,
                                                   wtf_size_t end_index) {
    // SAFETY: required from caller, enforced by UNSAFE_BUFFER_USAGE,
    return UNSAFE_BUFFERS(
        Range<iterator>(begin() + begin_index, begin() + end_index));
  }
  UNSAFE_BUFFER_USAGE Range<const_iterator> ItemsInRange(
      wtf_size_t begin_index,
      wtf_size_t end_index) const {
    // SAFETY: required from caller, enforced by UNSAFE_BUFFER_USAGE,
    return UNSAFE_BUFFERS(
        Range<const_iterator>(begin() + begin_index, begin() + end_index));
  }

  template <class DerivedItemType, typename... Args>
  DerivedItemType& AllocateAndConstruct(Args&&... args) {
    static_assert(WTF::IsSubclass<DerivedItemType, DisplayItem>::value,
                  "Must use subclass of DisplayItem.");
    static_assert(sizeof(DerivedItemType) <= kMaxItemSize,
                  "DisplayItem subclass is larger than kMaxItemSize.");
    static_assert(kAlignment % alignof(DerivedItemType) == 0,
                  "Derived type requires stronger alignment.");
    ItemSlot* result = AllocateItemSlot();
    new (result) DerivedItemType(std::forward<Args>(args)...);
    return *reinterpret_cast<DerivedItemType*>(result);
  }

  DisplayItem& AppendByMoving(DisplayItem& item) {
    DCHECK(!item.IsTombstone());
    return MoveItem(item, AllocateItemSlot());
  }

  DisplayItem& ReplaceLastByMoving(DisplayItem& item) {
    DCHECK(!item.IsTombstone());
    DisplayItem& last = UNSAFE_TODO(back());
    last.Destruct();
    return MoveItem(item, reinterpret_cast<ItemSlot*>(&last));
  }

  void AppendSubsequenceByMoving(DisplayItemList& from,
                                 wtf_size_t begin_index,
                                 wtf_size_t end_index) {
    DCHECK_GE(end_index, begin_index);
    if (end_index == begin_index)
      return;
    DCHECK_LT(begin_index, from.size());
    DCHECK_LE(end_index, from.size());

    wtf_size_t count = end_index - begin_index;
    ItemSlot* new_start_slot = AllocateItemSlots(count);
    size_t bytes_to_move =
        UNSAFE_TODO(reinterpret_cast<uint8_t*>(new_start_slot + count) -
                    reinterpret_cast<uint8_t*>(new_start_slot));
    UNSAFE_TODO(memcpy(static_cast<void*>(new_start_slot),
                       static_cast<void*>(&from[begin_index]), bytes_to_move));
    // This creates tombstones in the original items. Unlike AppendByMoving()
    // for individual items, we won't use the moved items in a subsequence
    // for raster invalidation, so we don't need to keep the other fields of
    // the display items. DisplayItemTest.AllZeroIsTombstone ensures that the
    // cleared items are tombstones.
    UNSAFE_TODO(
        memset(static_cast<void*>(&from[begin_index]), 0, bytes_to_move));
  }

#if DCHECK_IS_ON()
  enum JsonOption {
    kDefault,
    // Only show a compact representation of the display item list. This flag
    // cannot be used with kShowPaintRecords.
    kCompact,
    kShowPaintRecords,
  };

  static std::unique_ptr<JSONArray> DisplayItemsAsJSON(
      const PaintArtifact&,
      wtf_size_t first_item_index,
      const Range<const_iterator>& display_items,
      JsonOption);
#else  // DCHECK_IS_ON()
  enum JsonOption { kDefault };
#endif

 private:
  static_assert(std::is_trivially_copyable<value_type>::value,
                "DisplayItemList uses `memcpy` in several member functions; "
                "the `value_type` used by it must be trivially copyable");

  ItemSlot* AllocateItemSlot() { return &items_.emplace_back(); }

  ItemSlot* AllocateItemSlots(wtf_size_t count) {
    items_.Grow(size() + count);
    return UNSAFE_TODO(&items_.back() - (count - 1));
  }

  DisplayItem& MoveItem(DisplayItem& item, ItemSlot* new_item_slot) {
    UNSAFE_TODO(memcpy(static_cast<void*>(new_item_slot),
                       static_cast<void*>(&item), kMaxItemSize));

    // Created a tombstone/"dead display item" that can be safely destructed but
    // should never be used except for debugging and raster invalidation.
    item.CreateTombstone();
    DCHECK(item.IsTombstone());
    // Original values for other fields are kept for debugging and raster
    // invalidation.
    DisplayItem& new_item = *reinterpret_cast<DisplayItem*>(new_item_slot);
    DCHECK_EQ(item.VisualRect(), new_item.VisualRect());
    DCHECK_EQ(item.GetRasterEffectOutset(), new_item.GetRasterEffectOutset());
    return new_item;
  }

  ItemVector items_;
};

using DisplayItemIterator = DisplayItemList::const_iterator;
using DisplayItemRange = DisplayItemList::Range<DisplayItemIterator>;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_DISPLAY_ITEM_LIST_H_
