/*
 * Copyright 2021 Google Inc. All rights reserved.
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

#ifndef FLATBUFFERS_ARRAY_H_
#define FLATBUFFERS_ARRAY_H_

#include <cstdint>
#include <memory>

#include "flatbuffers/base.h"
#include "flatbuffers/stl_emulation.h"
#include "flatbuffers/vector.h"

namespace flatbuffers {

// This is used as a helper type for accessing arrays.
template<typename T, uint16_t length> class Array {
  // Array<T> can carry only POD data types (scalars or structs).
  typedef typename flatbuffers::bool_constant<flatbuffers::is_scalar<T>::value>
      scalar_tag;
  typedef
      typename flatbuffers::conditional<scalar_tag::value, T, const T *>::type
          IndirectHelperType;

 public:
  typedef uint16_t size_type;
  typedef typename IndirectHelper<IndirectHelperType>::return_type return_type;
  typedef VectorConstIterator<T, return_type, uoffset_t> const_iterator;
  typedef VectorReverseIterator<const_iterator> const_reverse_iterator;

  // If T is a LE-scalar or a struct (!scalar_tag::value).
  static FLATBUFFERS_CONSTEXPR bool is_span_observable =
      (scalar_tag::value && (FLATBUFFERS_LITTLEENDIAN || sizeof(T) == 1)) ||
      !scalar_tag::value;

  FLATBUFFERS_CONSTEXPR uint16_t size() const { return length; }

  return_type Get(uoffset_t i) const {
    FLATBUFFERS_ASSERT(i < size());
    return IndirectHelper<IndirectHelperType>::Read(Data(), i);
  }

  return_type operator[](uoffset_t i) const { return Get(i); }

  // If this is a Vector of enums, T will be its storage type, not the enum
  // type. This function makes it convenient to retrieve value with enum
  // type E.
  template<typename E> E GetEnum(uoffset_t i) const {
    return static_cast<E>(Get(i));
  }

  const_iterator begin() const { return const_iterator(Data(), 0); }
  const_iterator end() const { return const_iterator(Data(), size()); }

  const_reverse_iterator rbegin() const {
    return const_reverse_iterator(end());
  }
  const_reverse_iterator rend() const {
    return const_reverse_iterator(begin());
  }

  const_iterator cbegin() const { return begin(); }
  const_iterator cend() const { return end(); }

  const_reverse_iterator crbegin() const { return rbegin(); }
  const_reverse_iterator crend() const { return rend(); }

  // Get a mutable pointer to elements inside this array.
  // This method used to mutate arrays of structs followed by a @p Mutate
  // operation. For primitive types use @p Mutate directly.
  // @warning Assignments and reads to/from the dereferenced pointer are not
  //  automatically converted to the correct endianness.
  typename flatbuffers::conditional<scalar_tag::value, void, T *>::type
  GetMutablePointer(uoffset_t i) const {
    FLATBUFFERS_ASSERT(i < size());
    return const_cast<T *>(&data()[i]);
  }

  // Change elements if you have a non-const pointer to this object.
  void Mutate(uoffset_t i, const T &val) { MutateImpl(scalar_tag(), i, val); }

  // The raw data in little endian format. Use with care.
  const uint8_t *Data() const { return data_; }

  uint8_t *Data() { return data_; }

  // Similarly, but typed, much like std::vector::data
  const T *data() const { return reinterpret_cast<const T *>(Data()); }
  T *data() { return reinterpret_cast<T *>(Data()); }

  // Copy data from a span with endian conversion.
  // If this Array and the span overlap, the behavior is undefined.
  void CopyFromSpan(flatbuffers::span<const T, length> src) {
    const auto p1 = reinterpret_cast<const uint8_t *>(src.data());
    const auto p2 = Data();
    FLATBUFFERS_ASSERT(!(p1 >= p2 && p1 < (p2 + length)) &&
                       !(p2 >= p1 && p2 < (p1 + length)));
    (void)p1;
    (void)p2;
    CopyFromSpanImpl(flatbuffers::bool_constant<is_span_observable>(), src);
  }

 protected:
  void MutateImpl(flatbuffers::true_type, uoffset_t i, const T &val) {
    FLATBUFFERS_ASSERT(i < size());
    WriteScalar(data() + i, val);
  }

  void MutateImpl(flatbuffers::false_type, uoffset_t i, const T &val) {
    *(GetMutablePointer(i)) = val;
  }

  void CopyFromSpanImpl(flatbuffers::true_type,
                        flatbuffers::span<const T, length> src) {
    // Use std::memcpy() instead of std::copy() to avoid performance degradation
    // due to aliasing if T is char or unsigned char.
    // The size is known at compile time, so memcpy would be inlined.
    std::memcpy(data(), src.data(), length * sizeof(T));
  }

  // Copy data from flatbuffers::span with endian conversion.
  void CopyFromSpanImpl(flatbuffers::false_type,
                        flatbuffers::span<const T, length> src) {
    for (size_type k = 0; k < length; k++) { Mutate(k, src[k]); }
  }

  // This class is only used to access pre-existing data. Don't ever
  // try to construct these manually.
  // 'constexpr' allows us to use 'size()' at compile time.
  // @note Must not use 'FLATBUFFERS_CONSTEXPR' here, as const is not allowed on
  //  a constructor.
#if defined(__cpp_constexpr)
  constexpr Array();
#else
  Array();
#endif

  uint8_t data_[length * sizeof(T)];

 private:
  // This class is a pointer. Copying will therefore create an invalid object.
  // Private and unimplemented copy constructor.
  Array(const Array &);
  Array &operator=(const Array &);
};

// Specialization for Array[struct] with access using Offset<void> pointer.
// This specialization used by idl_gen_text.cpp.
template<typename T, uint16_t length, template<typename> class OffsetT>
class Array<OffsetT<T>, length> {
  static_assert(flatbuffers::is_same<T, void>::value, "unexpected type T");

 public:
  typedef const void *return_type;
  typedef uint16_t size_type;

  const uint8_t *Data() const { return data_; }

  // Make idl_gen_text.cpp::PrintContainer happy.
  return_type operator[](uoffset_t) const {
    FLATBUFFERS_ASSERT(false);
    return nullptr;
  }

 private:
  // This class is only used to access pre-existing data.
  Array();
  Array(const Array &);
  Array &operator=(const Array &);

  uint8_t data_[1];
};

template<class U, uint16_t N>
FLATBUFFERS_CONSTEXPR_CPP11 flatbuffers::span<U, N> make_span(Array<U, N> &arr)
    FLATBUFFERS_NOEXCEPT {
  static_assert(
      Array<U, N>::is_span_observable,
      "wrong type U, only plain struct, LE-scalar, or byte types are allowed");
  return span<U, N>(arr.data(), N);
}

template<class U, uint16_t N>
FLATBUFFERS_CONSTEXPR_CPP11 flatbuffers::span<const U, N> make_span(
    const Array<U, N> &arr) FLATBUFFERS_NOEXCEPT {
  static_assert(
      Array<U, N>::is_span_observable,
      "wrong type U, only plain struct, LE-scalar, or byte types are allowed");
  return span<const U, N>(arr.data(), N);
}

template<class U, uint16_t N>
FLATBUFFERS_CONSTEXPR_CPP11 flatbuffers::span<uint8_t, sizeof(U) * N>
make_bytes_span(Array<U, N> &arr) FLATBUFFERS_NOEXCEPT {
  static_assert(Array<U, N>::is_span_observable,
                "internal error, Array<T> might hold only scalars or structs");
  return span<uint8_t, sizeof(U) * N>(arr.Data(), sizeof(U) * N);
}

template<class U, uint16_t N>
FLATBUFFERS_CONSTEXPR_CPP11 flatbuffers::span<const uint8_t, sizeof(U) * N>
make_bytes_span(const Array<U, N> &arr) FLATBUFFERS_NOEXCEPT {
  static_assert(Array<U, N>::is_span_observable,
                "internal error, Array<T> might hold only scalars or structs");
  return span<const uint8_t, sizeof(U) * N>(arr.Data(), sizeof(U) * N);
}

// Cast a raw T[length] to a raw flatbuffers::Array<T, length>
// without endian conversion. Use with care.
// TODO: move these Cast-methods to `internal` namespace.
template<typename T, uint16_t length>
Array<T, length> &CastToArray(T (&arr)[length]) {
  return *reinterpret_cast<Array<T, length> *>(arr);
}

template<typename T, uint16_t length>
const Array<T, length> &CastToArray(const T (&arr)[length]) {
  return *reinterpret_cast<const Array<T, length> *>(arr);
}

template<typename E, typename T, uint16_t length>
Array<E, length> &CastToArrayOfEnum(T (&arr)[length]) {
  static_assert(sizeof(E) == sizeof(T), "invalid enum type E");
  return *reinterpret_cast<Array<E, length> *>(arr);
}

template<typename E, typename T, uint16_t length>
const Array<E, length> &CastToArrayOfEnum(const T (&arr)[length]) {
  static_assert(sizeof(E) == sizeof(T), "invalid enum type E");
  return *reinterpret_cast<const Array<E, length> *>(arr);
}

template<typename T, uint16_t length>
bool operator==(const Array<T, length> &lhs,
                const Array<T, length> &rhs) noexcept {
  return std::addressof(lhs) == std::addressof(rhs) ||
         (lhs.size() == rhs.size() &&
          std::memcmp(lhs.Data(), rhs.Data(), rhs.size() * sizeof(T)) == 0);
}

}  // namespace flatbuffers

#endif  // FLATBUFFERS_ARRAY_H_
