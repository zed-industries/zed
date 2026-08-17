// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef PROTOBUF_HPB_REPEATED_FIELD_H_
#define PROTOBUF_HPB_REPEATED_FIELD_H_

#include <assert.h>

#include <cstddef>
#include <iterator>
#include <type_traits>

#include "absl/strings/string_view.h"
#include "google/protobuf/hpb/backend/upb/interop.h"
#include "google/protobuf/hpb/internal/template_help.h"
#include "google/protobuf/hpb/repeated_field_iterator.h"
#include "upb/base/string_view.h"
#include "upb/mem/arena.h"
#include "upb/message/array.h"
#include "upb/message/copy.h"
#include "upb/message/message.h"

namespace hpb {
namespace internal {

// Shared implementation of repeated fields for absl::string_view and
// message types for mutable and immutable variants.
//
// Immutable (const accessor), constructs this class with a nullptr upb_Array*
// when the underlying array in the message is empty.
//
// Mutable accessors on the other hand, will allocate a new empty non-null
// upb_Array* for the message when the RepeatedFieldProxy is constructed.
template <class T>
class RepeatedFieldProxyBase {
  using Array = hpb::internal::add_const_if_T_is_const<T, upb_Array>;

 public:
  explicit RepeatedFieldProxyBase(Array* arr, upb_Arena* arena)
      : arr_(arr), arena_(arena) {}

  size_t size() const { return arr_ != nullptr ? upb_Array_Size(arr_) : 0; }

  bool empty() const { return size() == 0; }

 protected:
  // Returns upb_Array message member.
  inline upb_Message* GetMessage(size_t n) const;

  Array* arr_;
  upb_Arena* arena_;
};

template <class T>
upb_Message* RepeatedFieldProxyBase<T>::GetMessage(size_t n) const {
  auto** messages =
      static_cast<upb_Message**>(upb_Array_MutableDataPtr(this->arr_));
  return messages[n];
}

template <class T>
class RepeatedFieldProxyMutableBase : public RepeatedFieldProxyBase<T> {
 public:
  RepeatedFieldProxyMutableBase(upb_Array* arr, upb_Arena* arena)
      : RepeatedFieldProxyBase<T>(arr, arena) {}

  void clear() { upb_Array_Resize(this->arr_, 0, this->arena_); }
};

// RepeatedField proxy for repeated messages.
template <class T>
class RepeatedFieldProxy
    : public std::conditional_t<std::is_const_v<T>, RepeatedFieldProxyBase<T>,
                                RepeatedFieldProxyMutableBase<T>> {
  static_assert(!std::is_same_v<T, absl::string_view>, "");
  static_assert(!std::is_same_v<T, const absl::string_view>, "");
  static_assert(!std::is_arithmetic_v<T>, "");
  static constexpr bool kIsConst = std::is_const_v<T>;

 public:
  using value_type = std::remove_const_t<T>;
  using size_type = size_t;
  using difference_type = ptrdiff_t;
  using iterator = internal::Iterator<MessageIteratorPolicy<T>>;
  using reference = typename iterator::reference;
  using pointer = typename iterator::pointer;
  using reverse_iterator = std::reverse_iterator<iterator>;

  explicit RepeatedFieldProxy(const upb_Array* arr, upb_Arena* arena)
      : RepeatedFieldProxyBase<T>(arr, arena) {}
  RepeatedFieldProxy(upb_Array* arr, upb_Arena* arena)
      : RepeatedFieldProxyMutableBase<T>(arr, arena) {}
  // Constructor used by ::hpb::Ptr.
  RepeatedFieldProxy(const RepeatedFieldProxy&) = default;

  // T::CProxy [] operator specialization.
  typename T::CProxy operator[](size_t n) const {
    upb_MessageValue message_value = upb_Array_Get(this->arr_, n);
    return ::hpb::interop::upb::MakeCHandle<typename std::remove_const_t<T>>(
        (upb_Message*)message_value.msg_val, this->arena_);
  }

  // TODO : Audit/Finalize based on Iterator Design.
  // T::Proxy [] operator specialization.
  template <int&... DeductionBlocker, bool b = !kIsConst,
            typename = std::enable_if_t<b>>
  typename T::Proxy operator[](size_t n) {
    return hpb::interop::upb::MakeHandle<T>(this->GetMessage(n), this->arena_);
  }

  // Mutable message reference specialization.
  template <int&... DeductionBlocker, bool b = !kIsConst,
            typename = std::enable_if_t<b>>
  void push_back(const T& t) {
    upb_MessageValue message_value;
    message_value.msg_val = upb_Message_DeepClone(
        ::hpb::internal::PrivateAccess::GetInternalMsg(&t),
        ::hpb::interop::upb::GetMiniTable(&t), this->arena_);
    upb_Array_Append(this->arr_, message_value, this->arena_);
  }

  // Mutable message add using move.
  template <int&... DeductionBlocker, bool b = !kIsConst,
            typename = std::enable_if_t<b>>
  void push_back(T&& msg) {
    upb_MessageValue message_value;
    message_value.msg_val =
        ::hpb::internal::PrivateAccess::GetInternalMsg(&msg);
    upb_Arena_Fuse(hpb::interop::upb::GetArena(&msg), this->arena_);
    upb_Array_Append(this->arr_, message_value, this->arena_);
    T moved_msg = std::move(msg);
  }

  iterator begin() const {
    return iterator(
        {static_cast<upb_Message**>(
             this->arr_ ? const_cast<void*>(upb_Array_DataPtr(this->arr_))
                        : nullptr),
         this->arena_});
  }
  iterator end() const { return begin() + this->size(); }
  reverse_iterator rbegin() const { return reverse_iterator(end()); }
  reverse_iterator rend() const { return reverse_iterator(begin()); }

 private:
  friend class ::hpb::Ptr<T>;
};

// RepeatedField proxy for repeated strings.
template <class T>
class RepeatedFieldStringProxy
    : public std::conditional_t<std::is_const_v<T>, RepeatedFieldProxyBase<T>,
                                RepeatedFieldProxyMutableBase<T>> {
  static_assert(std::is_same_v<T, absl::string_view> ||
                    std::is_same_v<T, const absl::string_view>,
                "");
  static constexpr bool kIsConst = std::is_const_v<T>;

 public:
  using value_type = std::remove_const_t<T>;
  using size_type = size_t;
  using difference_type = ptrdiff_t;
  using iterator = internal::Iterator<StringIteratorPolicy<T>>;
  using reference = typename iterator::reference;
  using pointer = typename iterator::pointer;
  using reverse_iterator = std::reverse_iterator<iterator>;

  // Immutable constructor.
  explicit RepeatedFieldStringProxy(const upb_Array* arr, upb_Arena* arena)
      : RepeatedFieldProxyBase<T>(arr, arena) {}
  // Mutable constructor.
  RepeatedFieldStringProxy(upb_Array* arr, upb_Arena* arena)
      : RepeatedFieldProxyMutableBase<T>(arr, arena) {}
  // Constructor used by ::hpb::Ptr.
  RepeatedFieldStringProxy(const RepeatedFieldStringProxy&) = default;

  reference operator[](size_t n) const { return begin()[n]; }

  template <int&... DeductionBlocker, bool b = !kIsConst,
            typename = std::enable_if_t<b>>
  void push_back(T t) {
    upb_MessageValue message_value;
    // Copy string to arena.
    assert(this->arena_);
    char* data = (char*)upb_Arena_Malloc(this->arena_, t.size());
    assert(data);
    memcpy(data, t.data(), t.size());
    message_value.str_val = upb_StringView_FromDataAndSize(data, t.size());
    upb_Array_Append(this->arr_, message_value, this->arena_);
  }

  iterator begin() const { return iterator({this->arr_, this->arena_, 0}); }
  iterator end() const {
    return iterator({this->arr_, this->arena_, this->size()});
  }
  reverse_iterator rbegin() const { return reverse_iterator(end()); }
  reverse_iterator rend() const { return reverse_iterator(begin()); }
};

// RepeatedField proxy for repeated scalar types.
template <typename T>
class RepeatedFieldScalarProxy
    : public std::conditional_t<std::is_const_v<T>, RepeatedFieldProxyBase<T>,
                                RepeatedFieldProxyMutableBase<T>> {
  static_assert(std::is_arithmetic_v<T>, "");
  static constexpr bool kIsConst = std::is_const_v<T>;

 public:
  using value_type = std::remove_const_t<T>;
  using size_type = size_t;
  using difference_type = ptrdiff_t;
  using iterator = internal::Iterator<ScalarIteratorPolicy<T>>;
  using reference = typename iterator::reference;
  using pointer = typename iterator::pointer;
  using reverse_iterator = std::reverse_iterator<iterator>;

  explicit RepeatedFieldScalarProxy(const upb_Array* arr, upb_Arena* arena)
      : RepeatedFieldProxyBase<T>(arr, arena) {}
  RepeatedFieldScalarProxy(upb_Array* arr, upb_Arena* arena)
      : RepeatedFieldProxyMutableBase<T>(arr, arena) {}
  // Constructor used by ::hpb::Ptr.
  RepeatedFieldScalarProxy(const RepeatedFieldScalarProxy&) = default;

  T operator[](size_t n) const {
    upb_MessageValue message_value = upb_Array_Get(this->arr_, n);
    typename std::remove_const_t<T> val;
    memcpy(&val, &message_value, sizeof(T));
    return val;
  }

  template <int&... DeductionBlocker, bool b = !kIsConst,
            typename = std::enable_if_t<b>>
  void push_back(T t) {
    upb_MessageValue message_value;
    memcpy(&message_value, &t, sizeof(T));
    upb_Array_Append(this->arr_, message_value, this->arena_);
  }

  iterator begin() const { return iterator({unsafe_array()}); }
  iterator cbegin() const { return begin(); }
  iterator end() const { return iterator({unsafe_array() + this->size()}); }
  iterator cend() const { return end(); }

  // Reverse iterator support.
  reverse_iterator rbegin() const { return reverse_iterator(end()); }
  reverse_iterator rend() const { return reverse_iterator(begin()); }
  reverse_iterator crbegin() const { return reverse_iterator(end()); }
  reverse_iterator crend() const { return reverse_iterator(begin()); }

 private:
  T* unsafe_array() const {
    if (kIsConst) {
      const void* unsafe_ptr = ::upb_Array_DataPtr(this->arr_);
      return static_cast<T*>(const_cast<void*>(unsafe_ptr));
    }
    if (!kIsConst) {
      void* unsafe_ptr =
          ::upb_Array_MutableDataPtr(const_cast<upb_Array*>(this->arr_));
      return static_cast<T*>(unsafe_ptr);
    }
  }
};

}  // namespace internal

template <typename T>
class RepeatedField {
  static constexpr bool kIsString = std::is_same_v<T, absl::string_view>;
  static constexpr bool kIsScalar = std::is_arithmetic_v<T>;

 public:
  using Proxy = std::conditional_t<
      kIsScalar, internal::RepeatedFieldScalarProxy<T>,
      std::conditional_t<kIsString, internal::RepeatedFieldStringProxy<T>,
                         internal::RepeatedFieldProxy<T>>>;
  using CProxy = std::conditional_t<
      kIsScalar, internal::RepeatedFieldScalarProxy<const T>,
      std::conditional_t<kIsString, internal::RepeatedFieldStringProxy<const T>,
                         internal::RepeatedFieldProxy<const T>>>;
  // TODO: T supports incomplete type from fwd.h forwarding headers
  // We would like to reference T::CProxy. Validate forwarding header design.
  using ValueProxy = std::conditional_t<
      kIsScalar, T,
      std::conditional_t<kIsString, absl::string_view, ::hpb::Ptr<T>>>;
  using ValueCProxy = std::conditional_t<
      kIsScalar, const T,
      std::conditional_t<kIsString, absl::string_view, ::hpb::Ptr<const T>>>;
  using Access = std::conditional_t<
      kIsScalar, internal::RepeatedFieldScalarProxy<T>,
      std::conditional_t<kIsString, internal::RepeatedFieldStringProxy<T>,
                         internal::RepeatedFieldProxy<T>>>;
};

}  // namespace hpb

#endif  // PROTOBUF_HPB_REPEATED_FIELD_H_
