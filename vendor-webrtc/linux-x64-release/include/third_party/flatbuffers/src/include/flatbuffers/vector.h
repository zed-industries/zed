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

#ifndef FLATBUFFERS_VECTOR_H_
#define FLATBUFFERS_VECTOR_H_

#include "flatbuffers/base.h"
#include "flatbuffers/buffer.h"
#include "flatbuffers/stl_emulation.h"

namespace flatbuffers {

struct String;

// An STL compatible iterator implementation for Vector below, effectively
// calling Get() for every element.
template<typename T, typename IT, typename Data = uint8_t *,
         typename SizeT = uoffset_t>
struct VectorIterator {
  typedef std::random_access_iterator_tag iterator_category;
  typedef IT value_type;
  typedef ptrdiff_t difference_type;
  typedef IT *pointer;
  typedef IT &reference;

  static const SizeT element_stride = IndirectHelper<T>::element_stride;

  VectorIterator(Data data, SizeT i) : data_(data + element_stride * i) {}
  VectorIterator(const VectorIterator &other) : data_(other.data_) {}
  VectorIterator() : data_(nullptr) {}

  VectorIterator &operator=(const VectorIterator &other) {
    data_ = other.data_;
    return *this;
  }

  VectorIterator &operator=(VectorIterator &&other) {
    data_ = other.data_;
    return *this;
  }

  bool operator==(const VectorIterator &other) const {
    return data_ == other.data_;
  }

  bool operator!=(const VectorIterator &other) const {
    return data_ != other.data_;
  }

  bool operator<(const VectorIterator &other) const {
    return data_ < other.data_;
  }

  bool operator>(const VectorIterator &other) const {
    return data_ > other.data_;
  }

  bool operator<=(const VectorIterator &other) const {
    return !(data_ > other.data_);
  }

  bool operator>=(const VectorIterator &other) const {
    return !(data_ < other.data_);
  }

  difference_type operator-(const VectorIterator &other) const {
    return (data_ - other.data_) / element_stride;
  }

  // Note: return type is incompatible with the standard
  // `reference operator*()`.
  IT operator*() const { return IndirectHelper<T>::Read(data_, 0); }

  // Note: return type is incompatible with the standard
  // `pointer operator->()`.
  IT operator->() const { return IndirectHelper<T>::Read(data_, 0); }

  VectorIterator &operator++() {
    data_ += element_stride;
    return *this;
  }

  VectorIterator operator++(int) {
    VectorIterator temp(data_, 0);
    data_ += element_stride;
    return temp;
  }

  VectorIterator operator+(const SizeT &offset) const {
    return VectorIterator(data_ + offset * element_stride, 0);
  }

  VectorIterator &operator+=(const SizeT &offset) {
    data_ += offset * element_stride;
    return *this;
  }

  VectorIterator &operator--() {
    data_ -= element_stride;
    return *this;
  }

  VectorIterator operator--(int) {
    VectorIterator temp(data_, 0);
    data_ -= element_stride;
    return temp;
  }

  VectorIterator operator-(const SizeT &offset) const {
    return VectorIterator(data_ - offset * element_stride, 0);
  }

  VectorIterator &operator-=(const SizeT &offset) {
    data_ -= offset * element_stride;
    return *this;
  }

 private:
  Data data_;
};

template<typename T, typename IT, typename SizeT = uoffset_t>
using VectorConstIterator = VectorIterator<T, IT, const uint8_t *, SizeT>;

template<typename Iterator>
struct VectorReverseIterator : public std::reverse_iterator<Iterator> {
  explicit VectorReverseIterator(Iterator iter)
      : std::reverse_iterator<Iterator>(iter) {}

  // Note: return type is incompatible with the standard
  // `reference operator*()`.
  typename Iterator::value_type operator*() const {
    auto tmp = std::reverse_iterator<Iterator>::current;
    return *--tmp;
  }

  // Note: return type is incompatible with the standard
  // `pointer operator->()`.
  typename Iterator::value_type operator->() const {
    auto tmp = std::reverse_iterator<Iterator>::current;
    return *--tmp;
  }
};

// This is used as a helper type for accessing vectors.
// Vector::data() assumes the vector elements start after the length field.
template<typename T, typename SizeT = uoffset_t> class Vector {
 public:
  typedef VectorIterator<T, typename IndirectHelper<T>::mutable_return_type,
                         uint8_t *, SizeT>
      iterator;
  typedef VectorConstIterator<T, typename IndirectHelper<T>::return_type, SizeT>
      const_iterator;
  typedef VectorReverseIterator<iterator> reverse_iterator;
  typedef VectorReverseIterator<const_iterator> const_reverse_iterator;

  typedef typename flatbuffers::bool_constant<flatbuffers::is_scalar<T>::value>
      scalar_tag;

  static FLATBUFFERS_CONSTEXPR bool is_span_observable =
      scalar_tag::value && (FLATBUFFERS_LITTLEENDIAN || sizeof(T) == 1);

  SizeT size() const { return EndianScalar(length_); }

  // Returns true if the vector is empty.
  //
  // This just provides another standardized method that is expected of vectors.
  bool empty() const { return size() == 0; }

  // Deprecated: use size(). Here for backwards compatibility.
  FLATBUFFERS_ATTRIBUTE([[deprecated("use size() instead")]])
  SizeT Length() const { return size(); }

  typedef SizeT size_type;
  typedef typename IndirectHelper<T>::return_type return_type;
  typedef typename IndirectHelper<T>::mutable_return_type mutable_return_type;
  typedef return_type value_type;

  return_type Get(SizeT i) const {
    FLATBUFFERS_ASSERT(i < size());
    return IndirectHelper<T>::Read(Data(), i);
  }

  return_type operator[](SizeT i) const { return Get(i); }

  // If this is a Vector of enums, T will be its storage type, not the enum
  // type. This function makes it convenient to retrieve value with enum
  // type E.
  template<typename E> E GetEnum(SizeT i) const {
    return static_cast<E>(Get(i));
  }

  // If this a vector of unions, this does the cast for you. There's no check
  // to make sure this is the right type!
  template<typename U> const U *GetAs(SizeT i) const {
    return reinterpret_cast<const U *>(Get(i));
  }

  // If this a vector of unions, this does the cast for you. There's no check
  // to make sure this is actually a string!
  const String *GetAsString(SizeT i) const {
    return reinterpret_cast<const String *>(Get(i));
  }

  const void *GetStructFromOffset(size_t o) const {
    return reinterpret_cast<const void *>(Data() + o);
  }

  iterator begin() { return iterator(Data(), 0); }
  const_iterator begin() const { return const_iterator(Data(), 0); }

  iterator end() { return iterator(Data(), size()); }
  const_iterator end() const { return const_iterator(Data(), size()); }

  reverse_iterator rbegin() { return reverse_iterator(end()); }
  const_reverse_iterator rbegin() const {
    return const_reverse_iterator(end());
  }

  reverse_iterator rend() { return reverse_iterator(begin()); }
  const_reverse_iterator rend() const {
    return const_reverse_iterator(begin());
  }

  const_iterator cbegin() const { return begin(); }

  const_iterator cend() const { return end(); }

  const_reverse_iterator crbegin() const { return rbegin(); }

  const_reverse_iterator crend() const { return rend(); }

  // Change elements if you have a non-const pointer to this object.
  // Scalars only. See reflection.h, and the documentation.
  void Mutate(SizeT i, const T &val) {
    FLATBUFFERS_ASSERT(i < size());
    WriteScalar(data() + i, val);
  }

  // Change an element of a vector of tables (or strings).
  // "val" points to the new table/string, as you can obtain from
  // e.g. reflection::AddFlatBuffer().
  void MutateOffset(SizeT i, const uint8_t *val) {
    FLATBUFFERS_ASSERT(i < size());
    static_assert(sizeof(T) == sizeof(SizeT), "Unrelated types");
    WriteScalar(data() + i,
                static_cast<SizeT>(val - (Data() + i * sizeof(SizeT))));
  }

  // Get a mutable pointer to tables/strings inside this vector.
  mutable_return_type GetMutableObject(SizeT i) const {
    FLATBUFFERS_ASSERT(i < size());
    return const_cast<mutable_return_type>(IndirectHelper<T>::Read(Data(), i));
  }

  // The raw data in little endian format. Use with care.
  const uint8_t *Data() const {
    return reinterpret_cast<const uint8_t *>(&length_ + 1);
  }

  uint8_t *Data() { return reinterpret_cast<uint8_t *>(&length_ + 1); }

  // Similarly, but typed, much like std::vector::data
  const T *data() const { return reinterpret_cast<const T *>(Data()); }
  T *data() { return reinterpret_cast<T *>(Data()); }

  template<typename K> return_type LookupByKey(K key) const {
    void *search_result = std::bsearch(
        &key, Data(), size(), IndirectHelper<T>::element_stride, KeyCompare<K>);

    if (!search_result) {
      return nullptr;  // Key not found.
    }

    const uint8_t *element = reinterpret_cast<const uint8_t *>(search_result);

    return IndirectHelper<T>::Read(element, 0);
  }

  template<typename K> mutable_return_type MutableLookupByKey(K key) {
    return const_cast<mutable_return_type>(LookupByKey(key));
  }

 protected:
  // This class is only used to access pre-existing data. Don't ever
  // try to construct these manually.
  Vector();

  SizeT length_;

 private:
  // This class is a pointer. Copying will therefore create an invalid object.
  // Private and unimplemented copy constructor.
  Vector(const Vector &);
  Vector &operator=(const Vector &);

  template<typename K> static int KeyCompare(const void *ap, const void *bp) {
    const K *key = reinterpret_cast<const K *>(ap);
    const uint8_t *data = reinterpret_cast<const uint8_t *>(bp);
    auto table = IndirectHelper<T>::Read(data, 0);

    // std::bsearch compares with the operands transposed, so we negate the
    // result here.
    return -table->KeyCompareWithValue(*key);
  }
};

template<typename T> using Vector64 = Vector<T, uoffset64_t>;

template<class U>
FLATBUFFERS_CONSTEXPR_CPP11 flatbuffers::span<U> make_span(Vector<U> &vec)
    FLATBUFFERS_NOEXCEPT {
  static_assert(Vector<U>::is_span_observable,
                "wrong type U, only LE-scalar, or byte types are allowed");
  return span<U>(vec.data(), vec.size());
}

template<class U>
FLATBUFFERS_CONSTEXPR_CPP11 flatbuffers::span<const U> make_span(
    const Vector<U> &vec) FLATBUFFERS_NOEXCEPT {
  static_assert(Vector<U>::is_span_observable,
                "wrong type U, only LE-scalar, or byte types are allowed");
  return span<const U>(vec.data(), vec.size());
}

template<class U>
FLATBUFFERS_CONSTEXPR_CPP11 flatbuffers::span<uint8_t> make_bytes_span(
    Vector<U> &vec) FLATBUFFERS_NOEXCEPT {
  static_assert(Vector<U>::scalar_tag::value,
                "wrong type U, only LE-scalar, or byte types are allowed");
  return span<uint8_t>(vec.Data(), vec.size() * sizeof(U));
}

template<class U>
FLATBUFFERS_CONSTEXPR_CPP11 flatbuffers::span<const uint8_t> make_bytes_span(
    const Vector<U> &vec) FLATBUFFERS_NOEXCEPT {
  static_assert(Vector<U>::scalar_tag::value,
                "wrong type U, only LE-scalar, or byte types are allowed");
  return span<const uint8_t>(vec.Data(), vec.size() * sizeof(U));
}

// Convenient helper functions to get a span of any vector, regardless
// of whether it is null or not (the field is not set).
template<class U>
FLATBUFFERS_CONSTEXPR_CPP11 flatbuffers::span<U> make_span(Vector<U> *ptr)
    FLATBUFFERS_NOEXCEPT {
  static_assert(Vector<U>::is_span_observable,
                "wrong type U, only LE-scalar, or byte types are allowed");
  return ptr ? make_span(*ptr) : span<U>();
}

template<class U>
FLATBUFFERS_CONSTEXPR_CPP11 flatbuffers::span<const U> make_span(
    const Vector<U> *ptr) FLATBUFFERS_NOEXCEPT {
  static_assert(Vector<U>::is_span_observable,
                "wrong type U, only LE-scalar, or byte types are allowed");
  return ptr ? make_span(*ptr) : span<const U>();
}

// Represent a vector much like the template above, but in this case we
// don't know what the element types are (used with reflection.h).
class VectorOfAny {
 public:
  uoffset_t size() const { return EndianScalar(length_); }

  const uint8_t *Data() const {
    return reinterpret_cast<const uint8_t *>(&length_ + 1);
  }
  uint8_t *Data() { return reinterpret_cast<uint8_t *>(&length_ + 1); }

 protected:
  VectorOfAny();

  uoffset_t length_;

 private:
  VectorOfAny(const VectorOfAny &);
  VectorOfAny &operator=(const VectorOfAny &);
};

template<typename T, typename U>
Vector<Offset<T>> *VectorCast(Vector<Offset<U>> *ptr) {
  static_assert(std::is_base_of<T, U>::value, "Unrelated types");
  return reinterpret_cast<Vector<Offset<T>> *>(ptr);
}

template<typename T, typename U>
const Vector<Offset<T>> *VectorCast(const Vector<Offset<U>> *ptr) {
  static_assert(std::is_base_of<T, U>::value, "Unrelated types");
  return reinterpret_cast<const Vector<Offset<T>> *>(ptr);
}

// Convenient helper function to get the length of any vector, regardless
// of whether it is null or not (the field is not set).
template<typename T> static inline size_t VectorLength(const Vector<T> *v) {
  return v ? v->size() : 0;
}

}  // namespace flatbuffers

#endif  // FLATBUFFERS_VERIFIER_H_
