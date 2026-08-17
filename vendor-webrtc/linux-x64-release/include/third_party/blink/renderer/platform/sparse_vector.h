// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_SPARSE_VECTOR_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_SPARSE_VECTOR_H_

#include <limits>
#include <type_traits>

#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

namespace internal {

template <typename VectorType>
class NonTraceableSparseVectorFields {
  DISALLOW_NEW();

 protected:
  VectorType fields_;
};

template <typename VectorType>
class TraceableSparseVectorFields {
  DISALLOW_NEW();

 public:
  void Trace(Visitor* visitor) const { visitor->Trace(fields_); }

 protected:
  VectorType fields_;
};

}  // namespace internal

// This class is logically like an array of FieldType instances, indexed by the
// FieldId enum, but is optimized to save memory when only a small number of
// the possible fields are being used.
//
// This class is implemented using a vector containing the FieldType instances
// that are being used, and a bitfield to indicate which fields are being used.
// The popcount cpu instruction is critical for the O(1) lookup performance, as
// and allows us to determine the vector index of a field using the bitfield.
// For example:
// enum class FieldId {
//   kFirst = 0, ... kMiddle = 7, ... kLast = 15, kNumFields = kLast + 1,
// }
// SparseVector<FieldId, int> sparse_vector;
// A sparse vector with kFirst, kMiddle, and kLast populated will have:
// fields_bitfield_: 0b00000000000000001000000100000001
// Vector: [int for kFirst, int for kMiddle, int for kLast]
//
// Template parameters:
//   FieldId: must be an enum type (enforced below) and have a kNumFields entry
//            with value equal to 1 + maximum value in the enum.
//   FieldType: the intention is that it can be an arbitrary type, typically
//              either an class or a managed pointer to a class.
//   inline_capacity (optional): size of the inline buffer.
//   VectorType (optional): The type of the internal vector.
//   BitfieldType (optional): The type of the internal bitfield. Must be big
//                            enough to contain FieldId::kNumFields bits.
//
// Time complexity:
//   Query: O(1)
//   Modify existing field: O(1)
//   Add/erase: O(1) (at the end of the vector) to O(N) (N = number of
//              existing fields)
// It's efficient when the number of existing fields is much smaller than
// FieldId::kNumFields, add/erase operations mostly happen at the end of the
// vector, and the set of existing fields seldom changes.
template <typename FieldId,
          typename FieldType,
          wtf_size_t inline_capacity = 0,
          typename VectorType =
              std::conditional_t<WTF::IsMemberType<FieldType>::value ||
                                     WTF::IsTraceable<FieldType>::value,
                                 HeapVector<FieldType, inline_capacity>,
                                 Vector<FieldType, inline_capacity>>,
          typename BitfieldType =
              std::conditional_t<static_cast<size_t>(FieldId::kNumFields) <=
                                     std::numeric_limits<uint32_t>::digits,
                                 uint32_t,
                                 uint64_t>>
class SparseVector :
    // The conditional inheritance approach prevents types like
    // SparseVector<Enum, int> from being treated as traceable by the blinkgc
    // clang plugin. `requires` clause on the `Trace` method doesn't work.
    public std::conditional_t<
        WTF::IsTraceable<VectorType>::value,
        internal::TraceableSparseVectorFields<VectorType>,
        internal::NonTraceableSparseVectorFields<VectorType>> {
  static_assert(std::is_enum_v<FieldId>);
  static_assert(std::is_unsigned_v<BitfieldType>);
  static constexpr wtf_size_t kMaxSize =
      static_cast<wtf_size_t>(FieldId::kNumFields);
  static_assert(std::numeric_limits<BitfieldType>::digits >= kMaxSize,
                "BitfieldType must be big enough to have a bit for each "
                "field in FieldId.");
  static_assert(inline_capacity <= kMaxSize);

 public:
  wtf_size_t capacity() const { return this->fields_.capacity(); }
  wtf_size_t size() const { return this->fields_.size(); }
  bool empty() const { return this->fields_.empty(); }

  void reserve(wtf_size_t capacity) {
    CHECK_LE(capacity, kMaxSize);
    this->fields_.reserve(capacity);
  }

  void clear() {
    this->fields_.clear();
    fields_bitfield_ = 0;
  }

  // Returns whether the field exists. Time complexity is O(1).
  bool HasField(FieldId field_id) const {
    return fields_bitfield_ & FieldIdMask(field_id);
  }

  // Returns whether any field between `first` and `last` (both inclusive)
  // exists. Time complexity is O(1).
  bool HasFieldInRange(FieldId first, FieldId last) const {
    return fields_bitfield_ & RangeMask(first, last);
  }

  // Returns the value of existing field. Time complexity is O(1).
  const FieldType& GetField(FieldId field_id) const {
    DCHECK(HasField(field_id));
    return this->fields_[GetFieldIndex(field_id)];
  }
  FieldType& GetField(FieldId field_id) {
    DCHECK(HasField(field_id));
    return this->fields_[GetFieldIndex(field_id)];
  }

  // Adds a field if it's not existing (time complexity is O(1) (if the new
  // field is at the end) to O(N) (N = number of existing fields)), or modifies
  // the existing field (time complexity is O(1)).
  void SetField(FieldId field_id, FieldType&& field) {
    if (HasField(field_id)) {
      this->fields_[GetFieldIndex(field_id)] = std::forward<FieldType>(field);
    } else {
      fields_bitfield_ = fields_bitfield_ | FieldIdMask(field_id);
      this->fields_.insert(GetFieldIndex(field_id),
                           std::forward<FieldType>(field));
    }
  }

  // Erases a field. Returns `true` if the field was previously existing and
  // is actually erased. Time complexity is O(1) (if the field doesn't exist
  // or the erased field is at the end) to O(N) (N = number of existing fields).
  bool EraseField(FieldId field_id) {
    if (HasField(field_id)) {
      this->fields_.EraseAt(GetFieldIndex(field_id));
      fields_bitfield_ = fields_bitfield_ & ~FieldIdMask(field_id);
      return true;
    }
    return false;
  }

 private:
  static BitfieldType FieldIdMask(FieldId field_id) {
    CHECK_LT(static_cast<wtf_size_t>(field_id), kMaxSize);
    return static_cast<BitfieldType>(1) << static_cast<wtf_size_t>(field_id);
  }

  // Returns a mask that has entries for all field IDs lower than `upper`.
  static BitfieldType LowerFieldIdsMask(FieldId upper) {
    return FieldIdMask(upper) - 1;
  }

  static BitfieldType RangeMask(FieldId first, FieldId last) {
    return (FieldIdMask(last) | LowerFieldIdsMask(last)) &
           ~LowerFieldIdsMask(first);
  }

  // Returns the index in `fields_` that `field_id` is stored in. If `fields_`
  // isn't storing a field for `field_id`, then this returns the index which
  // the data for `field_id` should be inserted into.
  wtf_size_t GetFieldIndex(FieldId field_id) const {
    // Then count the total population of field IDs lower than that one we
    // are looking for. The target field ID should be located at the index of
    // of the total population.
    return std::popcount(fields_bitfield_ & LowerFieldIdsMask(field_id));
  }

  BitfieldType fields_bitfield_ = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_SPARSE_VECTOR_H_
