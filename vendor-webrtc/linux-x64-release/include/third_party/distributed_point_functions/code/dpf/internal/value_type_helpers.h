/*
 * Copyright 2021 Google LLC
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

#ifndef DISTRIBUTED_POINT_FUNCTIONS_DPF_INTERNAL_VALUE_TYPE_HELPERS_H_
#define DISTRIBUTED_POINT_FUNCTIONS_DPF_INTERNAL_VALUE_TYPE_HELPERS_H_

#include <stdint.h>

#include <algorithm>
#include <array>
#include <limits>
#include <string>
#include <tuple>
#include <type_traits>
#include <vector>

#include "absl/base/config.h"
#include "absl/log/absl_check.h"
#include "absl/meta/type_traits.h"
#include "absl/numeric/int128.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/str_format.h"
#include "absl/strings/string_view.h"
#include "absl/utility/utility.h"
#include "dpf/distributed_point_function.pb.h"
#include "dpf/int_mod_n.h"
#include "dpf/tuple.h"
#include "dpf/xor_wrapper.h"
#include "google/protobuf/repeated_field.h"

// Contains a collection of helper functions for different DPF value types. This
// includes functions for converting between Value protos and the corresponding
// C++ objects, as well as functions for sampling values from uniformly random
// byte strings.
//
// This file contains the templated declarations, instantiations for all
// supported types, as well as type-independent function declarations.
namespace distributed_point_functions {
namespace dpf_internal {

// A helper struct containing declarations for all templated functions we need.
// This is needed since C++ doesn't support partial function template
// specialization, and should be specialized for all supported types.
template <typename T, typename = void>
struct ValueTypeHelper {
  // General type traits and conversion functions. Should be implemented by all
  // types.

  // Type trait for all supported types. Used to provide meaningful error
  // messages in std::enable_if template guards.
  static constexpr bool IsSupportedType() { return false; }

  // Checks if the template parameter can be converted directly from a string of
  // bytes.
  static constexpr bool CanBeConvertedDirectly();

  // Converts a given Value to the template parameter T.
  static absl::StatusOr<T> FromValue(const Value& value);

  // ToValue Converts the argument to a Value proto.
  static Value ToValue(const T& input);

  // ToValueType<T> Returns a `ValueType` message describing T.
  static ValueType ToValueType();

  // Functions for converting from a byte string to T. There are two approaches:
  // Either converting directly (i.e., each byte is copied 1-to-1 into the
  // result), or by sampling (when a direct conversion is not possible). Types
  // for which CanBeConvertedDirectly() can be true should implement the former,
  // and all types should implement the latter (to support types composed of
  // directly-convertible and not-directly-convertible types).

  // Functions for direct conversions from bytes. Should be implemented when
  // CanBeConvertedDirectly() can be true.

  // Returns the total number of bits in a T.
  static constexpr int TotalBitSize();

  static T DirectlyFromBytes(absl::string_view bytes);

  // Functions for sampling from a string of bytes. Should be implemented by all
  // types.

  // Converts `block` to type T. Then, if `update == true`, fills up `block`
  // from `remaining_bytes` and advances `remaining_bytes` by the amount of
  // bytes read.
  static T SampleAndUpdateBytes(bool update, absl::uint128& block,
                                absl::string_view& remaining_bytes);
};

/******************************************************************************/
// Type traits                                                                //
/******************************************************************************/

// Type trait for all supported types. Used to provide meaningful error messages
// in std::enable_if template guards.
template <typename T>
struct is_supported_type {
  static constexpr bool value =
      dpf_internal::ValueTypeHelper<T>::IsSupportedType();
};
template <typename T>
constexpr bool is_supported_type_v = is_supported_type<T>::value;

// Checks if the template parameter can be converted directly from a string of
// bytes.
template <typename T>
struct can_be_converted_directly {
  static constexpr bool value =
      dpf_internal::ValueTypeHelper<T>::CanBeConvertedDirectly();
};
template <typename T>
constexpr bool can_be_converted_directly_v =
    can_be_converted_directly<T>::value;

// Returns the total number of bits in a T.
template <typename T,
          typename = absl::enable_if_t<can_be_converted_directly_v<T>>>
static constexpr int TotalBitSize() {
  return ValueTypeHelper<T>::TotalBitSize();
}

/******************************************************************************/
// Integer Helpers                                                            //
/******************************************************************************/

// Type trait for all integer types we support, i.e., 8 to 128 bit types.
template <typename T>
using is_unsigned_integer =
    absl::disjunction<std::is_same<T, uint8_t>, std::is_same<T, uint16_t>,
                      std::is_same<T, uint32_t>, std::is_same<T, uint64_t>,
#ifdef ABSL_HAVE_INTRINSIC_INT128
                      std::is_same<T, unsigned __int128>,
#endif
                      std::is_same<T, absl::uint128>>;
template <typename T>
constexpr bool is_unsigned_integer_v = is_unsigned_integer<T>::value;

// Converts the given Value::Integer to an absl::uint128. Used as a helper
// function in `ConvertValueTo` and `ValueTypesAreEqual`.
//
// Returns INVALID_ARGUMENT if `in` is not a simple integer or IntModN.
absl::StatusOr<absl::uint128> ValueIntegerToUint128(const Value::Integer& in);

// Converts an absl::uint128 to a Value::Integer. Used as a helper function in
// ToValue.
Value::Integer Uint128ToValueInteger(absl::uint128 input);

// Checks if the given value is in range of T, and if so, returns it converted
// to T.
//
// Otherwise returns INVALID_ARGUMENT.
template <typename T, typename = absl::enable_if_t<is_unsigned_integer_v<T>>>
absl::StatusOr<T> Uint128To(absl::uint128 in) {
  // Check whether value is in range if it's smaller than 128 bits.
  if (!std::is_same<T, absl::uint128>::value &&
      absl::Uint128Low64(in) >
          static_cast<uint64_t>(std::numeric_limits<T>::max())) {
    return absl::InvalidArgumentError(absl::StrCat(
        "Value (= ", absl::Uint128Low64(in),
        ") too large for the given type T (size ", sizeof(T), ")"));
  }
  return static_cast<T>(in);
}

// Implementation of ValueTypeHelper for integers.
template <typename T>
struct ValueTypeHelper<T, absl::enable_if_t<is_unsigned_integer_v<T>>> {
  static constexpr bool IsSupportedType() { return true; }

  static constexpr bool CanBeConvertedDirectly() { return true; }

  static absl::StatusOr<T> FromValue(const Value& value) {
    if (value.value_case() != Value::kInteger) {
      return absl::InvalidArgumentError("The given Value is not an integer");
    }
    // We first parse the value into an absl::uint128, then check its range if
    // it is supposed to be smaller than 128 bits.
    absl::StatusOr<absl::uint128> value_128 =
        ValueIntegerToUint128(value.integer());
    if (!value_128.ok()) {
      return value_128.status();
    }
    return Uint128To<T>(*value_128);
  }

  static Value ToValue(T input) {
    Value result;
    *(result.mutable_integer()) = Uint128ToValueInteger(input);
    return result;
  }

  static ValueType ToValueType() {
    ValueType result;
    result.mutable_integer()->set_bitsize(8 * sizeof(T));
    return result;
  }

  static constexpr int TotalBitSize() { return sizeof(T) * 8; }

  static T DirectlyFromBytes(absl::string_view bytes) {
    ABSL_CHECK(bytes.size() == sizeof(T));
    T out{0};
#ifdef ABSL_IS_LITTLE_ENDIAN
    std::copy_n(bytes.begin(), sizeof(T), reinterpret_cast<char*>(&out));
#else
    for (int i = sizeof(T) - 1; i >= 0; --i) {
      out |= absl::bit_cast<uint8_t>(bytes[i]);
      out <<= 8;
    }
#endif
    return out;
  }

  static T SampleAndUpdateBytes(bool update, absl::uint128& block,
                                absl::string_view& remaining_bytes) {
    T result = static_cast<T>(block);

    if (update) {
      // Set sizeof(T) least significant bytes to 0.
      if (sizeof(T) < sizeof(block)) {
        constexpr absl::uint128 mask =
            ~absl::uint128{std::numeric_limits<T>::max()};
        block &= mask;
      } else {
        block = 0;
      }

      // Fill up with `bytes` and advance `bytes` by sizeof(T).
      ABSL_DCHECK(remaining_bytes.size() >= sizeof(T));
      block |= DirectlyFromBytes(remaining_bytes.substr(0, sizeof(T)));
      remaining_bytes = remaining_bytes.substr(sizeof(T));
    }

    return result;
  }
};

/******************************************************************************/
// IntModN Helpers                                                            //
/******************************************************************************/

template <typename BaseInteger, typename ModulusType, ModulusType kModulus>
struct ValueTypeHelper<
    dpf_internal::IntModNImpl<BaseInteger, ModulusType, kModulus>, void> {
  using IntModNType =
      dpf_internal::IntModNImpl<BaseInteger, ModulusType, kModulus>;

  static constexpr bool IsSupportedType() {
    return is_unsigned_integer_v<BaseInteger> &&
           is_unsigned_integer_v<ModulusType>;
  }

  static constexpr bool CanBeConvertedDirectly() { return false; }

  static absl::StatusOr<IntModNType> FromValue(const Value& value) {
    if (value.value_case() != Value::kIntModN) {
      return absl::InvalidArgumentError("The given Value is not an IntModN");
    }
    absl::StatusOr<absl::uint128> value_128 =
        ValueIntegerToUint128(value.int_mod_n());
    if (!value_128.ok()) {
      return value_128.status();
    }
    if (*value_128 >= absl::uint128{kModulus}) {
      return absl::InvalidArgumentError(absl::StrFormat(
          "The given value (= %d) is larger than kModulus (= %d)", *value_128,
          absl::uint128{kModulus}));
    }
    return IntModNType(static_cast<BaseInteger>(*value_128));
  }

  static Value ToValue(IntModNType input) {
    Value result;
    *(result.mutable_int_mod_n()) = Uint128ToValueInteger(input.value());
    return result;
  }

  static ValueType ToValueType() {
    ValueType result;
    *(result.mutable_int_mod_n()->mutable_base_integer()) =
        ValueTypeHelper<BaseInteger>::ToValueType().integer();
    *(result.mutable_int_mod_n()->mutable_modulus()) =
        ValueTypeHelper<ModulusType>::ToValue(kModulus).integer();
    return result;
  }

  static IntModNType SampleAndUpdateBytes(bool update, absl::uint128& block,
                                          absl::string_view& remaining_bytes) {
    // Optimization for native uint128. This is equivalent to what's done in
    // int128.cc, but since division is not defined in the header, the compiler
    // cannot optimize the division and modulus into a single operation.
#ifdef ABSL_HAVE_INTRINSIC_INT128
    absl::uint128 quotient = static_cast<unsigned __int128>(block) / kModulus,
                  remainder = static_cast<unsigned __int128>(block) % kModulus;
#else
    absl::uint128 quotient = block / kModulus, remainder = block % kModulus;
#endif
    IntModNType result(static_cast<BaseInteger>(remainder));

    if (update) {
      if (sizeof(BaseInteger) < sizeof(block)) {
        block = quotient << (sizeof(BaseInteger) * 8);
      } else {
        block = 0;
      }
      block |= ValueTypeHelper<BaseInteger>::DirectlyFromBytes(
          remaining_bytes.substr(0, sizeof(BaseInteger)));
      remaining_bytes = remaining_bytes.substr(sizeof(BaseInteger));
    }

    return result;
  }
};

/******************************************************************************/
// Tuple Helpers                                                              //
/******************************************************************************/

// Helper struct for computing the bit size of a tuple type at compile time
// without C++17 fold expressions.
template <typename FirstElementType, typename... ElementType>
struct TupleBitSizeHelper {
  static constexpr int TotalBitSize() {
    return TupleBitSizeHelper<FirstElementType>::TotalBitSize() +
           TupleBitSizeHelper<ElementType...>::TotalBitSize();
  }
};
template <typename ElementType>
struct TupleBitSizeHelper<ElementType> {
  static constexpr int TotalBitSize() {
    return ValueTypeHelper<ElementType>::TotalBitSize();
  }
};

template <typename... ElementType>
struct ValueTypeHelper<Tuple<ElementType...>, void> {
  using TupleType = Tuple<ElementType...>;

  static constexpr bool IsSupportedType() {
    return absl::conjunction<is_supported_type<ElementType>...>::value;
  }

  static constexpr bool CanBeConvertedDirectly() {
    return absl::conjunction<can_be_converted_directly<ElementType>...>::value;
  }

  static absl::StatusOr<TupleType> FromValue(const Value& value) {
    if (value.value_case() != Value::kTuple) {
      return absl::InvalidArgumentError("The given Value is not a tuple");
    }
    constexpr auto tuple_size =
        static_cast<int>(std::tuple_size<typename TupleType::Base>());
    if (value.tuple().elements_size() != tuple_size) {
      return absl::InvalidArgumentError(
          "The tuple in the given Value has the wrong number of elements");
    }

    // Create a Tuple by unpacking value.tuple().elements(). If we encounter an
    // error, return it at the end.
    absl::Status status = absl::OkStatus();
    int element_index = 0;
    // The braced initializer list ensures elements are created in the correct
    // order (unlike std::make_tuple).
    TupleType result = {[&value, &status, &element_index] {
      if (status.ok()) {
        absl::StatusOr<ElementType> element =
            ValueTypeHelper<ElementType>::FromValue(
                value.tuple().elements(element_index));
        element_index++;
        if (element.ok()) {
          return *element;
        } else {
          status = element.status();
        }
      }
      return ElementType{};
    }()...};
    if (status.ok()) {
      return result;
    } else {
      return status;
    }
  }

  static Value ToValue(const TupleType& input) {
    Value result;
    absl::apply(
        [&result](const ElementType&... elements) {
          // Create an unused std::tuple to iterate over `elements` in its
          // constructor. This can be replaced by a fold expression in C++17.
          std::tuple<ElementType...>{
              (*(result.mutable_tuple()->add_elements()) =
                   ValueTypeHelper<ElementType>::ToValue(elements),
               ElementType{})...};
        },
        input.value());
    return result;
  }

  static ValueType ToValueType() {
    ValueType result;
    ValueType::Tuple* tuple = result.mutable_tuple();
    // Create an unused std::tuple to iterate over `elements` in its
    // constructor. This can be replaced by a fold expression in C++17.
    std::tuple<ElementType...>{
        (*(tuple->add_elements()) = ValueTypeHelper<ElementType>::ToValueType(),
         ElementType{})...};
    return result;
  }

  static constexpr int TotalBitSize() {
    // This helper can be replaced by a fold expression in C++17.
    return TupleBitSizeHelper<ElementType...>::TotalBitSize();
  }

  static TupleType DirectlyFromBytes(absl::string_view bytes) {
    ABSL_CHECK(8 * bytes.size() >= TotalBitSize());
    int offset = 0;
    absl::Status status = absl::OkStatus();
    // Braced-init-list ensures the elements are constructed in-order.
    return TupleType{[&bytes, &offset, &status] {
      constexpr int element_size_bytes =
          (ValueTypeHelper<ElementType>::TotalBitSize() + 7) / 8;
      ElementType element = ValueTypeHelper<ElementType>::DirectlyFromBytes(
          bytes.substr(offset, element_size_bytes));
      offset += element_size_bytes;
      return element;
    }()...};
  }

  static TupleType SampleAndUpdateBytes(bool update, absl::uint128& block,
                                        absl::string_view& remaining_bytes) {
    int element_counter = 0;
    // Braced-init-list ensures the elements are constructed in-order.
    return TupleType{[update, &element_counter, &block,
                      &remaining_bytes]() -> ElementType {
      // If `update` is true, update after all elements. Otherwise, don't update
      // after the last one.
      constexpr int num_elements = std::tuple_size<typename TupleType::Base>();
      bool update2 = update || (++element_counter < num_elements);
      return ValueTypeHelper<ElementType>::SampleAndUpdateBytes(
          update2, block, remaining_bytes);
    }()...};
  }
};

/******************************************************************************/
// XorWrapper Helpers                                                         //
/******************************************************************************/

template <typename T>
struct ValueTypeHelper<XorWrapper<T>, void> {
  static constexpr bool IsSupportedType() {
    return ValueTypeHelper<T>::IsSupportedType();
  }

  static constexpr bool CanBeConvertedDirectly() {
    return ValueTypeHelper<T>::CanBeConvertedDirectly();
  }

  static absl::StatusOr<XorWrapper<T>> FromValue(const Value& value) {
    absl::StatusOr<absl::uint128> wrapped128 =
        ValueIntegerToUint128(value.xor_wrapper());
    if (!wrapped128.ok()) {
      return wrapped128.status();
    }
    absl::StatusOr<T> wrapped = Uint128To<T>(*wrapped128);
    if (!wrapped.ok()) {
      return wrapped.status();
    }
    return XorWrapper<T>(*wrapped);
  }

  static Value ToValue(const XorWrapper<T>& input) {
    Value result;
    *(result.mutable_xor_wrapper()) = Uint128ToValueInteger(input.value());
    return result;
  }

  static ValueType ToValueType() {
    ValueType result;
    *(result.mutable_xor_wrapper()) =
        ValueTypeHelper<T>::ToValueType().integer();
    return result;
  }

  static constexpr int TotalBitSize() {
    return ValueTypeHelper<T>::TotalBitSize();
  }

  static XorWrapper<T> DirectlyFromBytes(absl::string_view bytes) {
    return XorWrapper<T>(ValueTypeHelper<T>::DirectlyFromBytes(bytes));
  }

  static XorWrapper<T> SampleAndUpdateBytes(
      bool update, absl::uint128& block, absl::string_view& remaining_bytes) {
    return XorWrapper<T>(ValueTypeHelper<T>::SampleAndUpdateBytes(
        update, block, remaining_bytes));
  }
};

/******************************************************************************/
// Free standing helpers. These should always come last. When adding          //
// additional types, add them above.                                          //
/******************************************************************************/

// Computes the number of values of type T that fit into an absl::uint128.
// Returns a value >= 1 if batching is supported, and 1 otherwise.
template <typename T,
          absl::enable_if_t<can_be_converted_directly_v<T>, int> = 0>
constexpr int ElementsPerBlock() {
  if (TotalBitSize<T>() <= 128) {
    return static_cast<int>(8 * sizeof(absl::uint128)) / TotalBitSize<T>();
  }
  return 1;
}
template <typename T,
          absl::enable_if_t<!can_be_converted_directly_v<T>, int> = 0>
constexpr int ElementsPerBlock() {
  return 1;
}

// Creates a value of type T from the given `bytes`. If possible, converts bytes
// directly using DirectlyFromBytes. Otherwise, uses SampleAndUpdateBytes.
//
// Crashes if `bytes.size()` is too small for the output type.
template <typename T,
          absl::enable_if_t<can_be_converted_directly_v<T>, int> = 0>
T FromBytes(absl::string_view bytes) {
  return ValueTypeHelper<T>::DirectlyFromBytes(bytes);
}
template <typename T,
          absl::enable_if_t<!can_be_converted_directly_v<T>, int> = 0>
T FromBytes(absl::string_view bytes) {
  absl::uint128 block =
      FromBytes<absl::uint128>(bytes.substr(0, sizeof(absl::uint128)));
  bytes = bytes.substr(sizeof(absl::uint128));
  return ValueTypeHelper<T>::SampleAndUpdateBytes(false, block, bytes);
}

// Converts a `repeated Value` proto field to a std::array with element type T.
//
// Returns INVALID_ARGUMENT in case the input has the wrong size, or if the
// conversion fails.
template <typename T>
absl::StatusOr<std::array<T, ElementsPerBlock<T>()>> ValuesToArray(
    const ::google::protobuf::RepeatedPtrField<Value>& values) {
  if (values.size() != ElementsPerBlock<T>()) {
    return absl::InvalidArgumentError(absl::StrCat(
        "values.size() (= ", values.size(),
        ") does not match ElementsPerBlock<T>() (= ", ElementsPerBlock<T>(),
        ")"));
  }
  std::array<T, ElementsPerBlock<T>()> result;
  for (int i = 0; i < ElementsPerBlock<T>(); ++i) {
    absl::StatusOr<T> element = ValueTypeHelper<T>::FromValue(values[i]);
    if (element.ok()) {
      result[i] = std::move(*element);
    } else {
      return element.status();
    }
  }
  return result;
}

// Converts a given string to an array of exactly ElementsPerBlock<T>() elements
// of type T.
//
// Crashes if `bytes.size()` is too small for the output type.
template <typename T,
          absl::enable_if_t<can_be_converted_directly_v<T>, int> = 0>
std::array<T, ElementsPerBlock<T>()> ConvertBytesToArrayOf(
    absl::string_view bytes) {
  std::array<T, ElementsPerBlock<T>()> out;
  const int element_size_bytes = (TotalBitSize<T>() + 7) / 8;
  ABSL_CHECK(bytes.size() >= ElementsPerBlock<T>() * element_size_bytes);
  for (int i = 0; i < ElementsPerBlock<T>(); ++i) {
    out[i] =
        FromBytes<T>(bytes.substr(i * element_size_bytes, element_size_bytes));
  }
  return out;
}
template <typename T,
          absl::enable_if_t<!can_be_converted_directly_v<T>, int> = 0>
std::array<T, ElementsPerBlock<T>()> ConvertBytesToArrayOf(
    absl::string_view bytes) {
  static_assert(ElementsPerBlock<T>() == 1,
                "T does not support batching, but ElementsPerBlock<T> != 1");
  return {FromBytes<T>(bytes)};
}

// Computes the value correction word given two seeds `seed_a`, `seed_b` for
// parties a and b, such that the element at `block_index` is equal to `beta`.
// If `invert` is true, the result is multiplied element-wise by -1. Templated
// to use the correct integer type without needing modular reduction.
//
// Returns multiple values in case of packing, and a single value otherwise.
template <typename T>
absl::StatusOr<std::vector<Value>> ComputeValueCorrectionFor(
    absl::string_view seed_a, absl::string_view seed_b, int block_index,
    const Value& beta, bool invert) {
  absl::StatusOr<T> beta_T = ValueTypeHelper<T>::FromValue(beta);
  if (!beta_T.ok()) {
    return beta_T.status();
  }

  constexpr int elements_per_block = ElementsPerBlock<T>();

  // Compute values from seeds. Both arrays will have multiple elements if T
  // supports batching, and a single one otherwise.
  std::array<T, elements_per_block> ints_a = ConvertBytesToArrayOf<T>(seed_a),
                                    ints_b = ConvertBytesToArrayOf<T>(seed_b);

  // Add beta to the right position.
  ints_b[block_index] += *beta_T;

  // Add up shares, invert if needed.
  for (int i = 0; i < elements_per_block; i++) {
    ints_b[i] = ints_b[i] - ints_a[i];
    if (invert) {
      ints_b[i] = -ints_b[i];
    }
  }

  // Convert to a vector of Value protos and return.
  std::vector<Value> result;
  result.reserve(ints_b.size());
  for (const T& element : ints_b) {
    result.push_back(ValueTypeHelper<T>::ToValue(element));
  }
  return result;
}

// Computes the number of pseudorandom bits needed to get a uniform element of
// the given `ValueType`. For types whose elements can be bijectively mapped to
// strings (e.g., unsigned integers and tuples of integers), this is equivalent
// to the bit size of the value type. For all other types, returns the number of
// bits needed so that converting a uniform string with the given number of bits
// to an element of `value_type` results in a distribution with total variation
// distance < 2^(-`security_parameter`) from uniform.
//
// Returns INVALID_ARGUMENT in case value_type does not represent a known type,
// or if sampling with the required security parameter is not possible.
absl::StatusOr<int> BitsNeeded(const ValueType& value_type,
                               double security_parameter);

// Returns `true` if `lhs` and `rhs` describe the same types, and `false`
// otherwise.
//
// Returns INVALID_ARGUMENT if an error occurs while parsing either argument.
absl::StatusOr<bool> ValueTypesAreEqual(const ValueType& lhs,
                                        const ValueType& rhs);

}  // namespace dpf_internal
}  // namespace distributed_point_functions

#endif  // DISTRIBUTED_POINT_FUNCTIONS_DPF_INTERNAL_VALUE_TYPE_HELPERS_H_
