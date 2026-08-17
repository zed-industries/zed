// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_BIT_FIELD_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_BIT_FIELD_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/atomic_operations.h"

namespace WTF {

enum class BitFieldValueConstness {
  kNonConst,
  kConst,
};

namespace internal {

template <class BitFieldType>
class BitFieldBase;

// Helper class for defining values in a bit field. This helper provides
// utilities to read, write and update the value in the bit field.
template <class ValueType,
          size_t offset,
          size_t size,
          class BitFieldType,
          BitFieldValueConstness is_const = BitFieldValueConstness::kNonConst>
class BitFieldValue final {
  static_assert(std::is_fundamental<ValueType>::value,
                "Fields in a bit field must be of a primitive type.");
  static_assert(std::is_fundamental<BitFieldType>::value,
                "Bit fields must be of a primitive type.");
  static_assert(std::is_unsigned<BitFieldType>::value,
                "Bit field must be of an unsigned type");
  static_assert(sizeof(ValueType) <= sizeof(BitFieldType),
                "Value in bit field cannot be bigger than the bit field");
  static_assert(
      offset < 8 * sizeof(BitFieldType),
      "Field offset in bit field must be smaller than the bit field size");
  static_assert(
      size < 8 * sizeof(BitFieldType),
      "Field size in bit field must be smaller than the bit field size");
  static_assert(offset + size <= 8 * sizeof(BitFieldType),
                "Field in bit field cannot overflow the bit field");
  static_assert(size > 0, "Bit field fields cannot have 0 size.");

 public:
  using Type = ValueType;

  template <class OtherValueType,
            int other_size,
            BitFieldValueConstness other_is_const =
                BitFieldValueConstness::kNonConst>
  using DefineNextValue = BitFieldValue<OtherValueType,
                                        offset + size,
                                        other_size,
                                        BitFieldType,
                                        other_is_const>;

  // Create a bit field with the given value.
  static constexpr BitFieldType encode(ValueType value) {
    DCHECK(is_valid(value));
    return static_cast<BitFieldType>(value) << offset;
  }

  // Update a bit field with the given value.
  static constexpr BitFieldType update(BitFieldType previous, ValueType value) {
    return (previous & ~kMask) | encode(value);
  }

  // Read the value from the bit field.
  static constexpr ValueType decode(BitFieldType value) {
    return static_cast<ValueType>((value & kMask) >> offset);
  }

 private:
  static constexpr BitFieldValueConstness kIsConst = is_const;

  static constexpr BitFieldType kValidationMask =
      (BitFieldType{1} << size) - BitFieldType{1};
  static constexpr BitFieldType kMask = (kValidationMask) << offset;
  static_assert(kMask != 0, "Mask in which all bits are 0 is not allowed.");
  static_assert(~kMask != 0, "Mask in which all bits are 1 is not allowed.");

  // Confirm that the provided value fits into the bit field.
  static constexpr bool is_valid(ValueType value) {
    return (static_cast<BitFieldType>(value) & ~kValidationMask) == 0;
  }

  friend class BitFieldBase<BitFieldType>;
};

}  // namespace internal

// BitField intended to be used by a single thread.
template <class BitFieldType>
class WTF_EXPORT SingleThreadedBitField {
  static_assert(std::is_fundamental<BitFieldType>::value,
                "Bit fields must be of a primitive type.");
  static_assert(std::is_unsigned<BitFieldType>::value,
                "Bit field must be of an unsigned type");

 public:
  template <class Type,
            int size,
            BitFieldValueConstness is_const = BitFieldValueConstness::kNonConst>
  using DefineFirstValue =
      internal::BitFieldValue<Type, 0, size, BitFieldType, is_const>;

  explicit SingleThreadedBitField() : SingleThreadedBitField(0) {}
  explicit SingleThreadedBitField(BitFieldType bits) : bits_(bits) {}

  template <typename Value>
  typename Value::Type get() const {
    return Value::decode(bits_);
  }

  template <typename Value>
  void set(typename Value::Type value) {
    bits_ = Value::update(bits_, value);
  }

  BitFieldType bits() const { return bits_; }

 protected:
  BitFieldType bits_;
};

// BitField that can be written by a single thread but read by multiple threads.
template <class BitFieldType>
class WTF_EXPORT ConcurrentlyReadBitField
    : public SingleThreadedBitField<BitFieldType> {
  using Base = SingleThreadedBitField<BitFieldType>;
  using Base::bits_;

 public:
  explicit ConcurrentlyReadBitField() : Base(0) {}
  explicit ConcurrentlyReadBitField(BitFieldType bits) : Base(bits) {}

  template <typename Value>
  typename Value::Type get_concurrently() const {
    return Value::decode(AsAtomicPtr(&bits_)->load(std::memory_order_relaxed));
  }

  template <typename Value>
  void set(typename Value::Type value) {
    AsAtomicPtr(&bits_)->store(Value::update(bits_, value),
                               std::memory_order_relaxed);
  }
};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_BIT_FIELD_H_
