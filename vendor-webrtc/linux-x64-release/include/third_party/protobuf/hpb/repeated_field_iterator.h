// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef PROTOBUF_HPB_REPEATED_FIELD_ITERATOR_H_
#define PROTOBUF_HPB_REPEATED_FIELD_ITERATOR_H_

#include <cstddef>
#include <cstring>
#include <iterator>
#include <type_traits>

#include "absl/strings/string_view.h"
#include "google/protobuf/hpb/backend/upb/interop.h"
#include "google/protobuf/hpb/hpb.h"
#include "upb/base/string_view.h"
#include "upb/mem/arena.h"
#include "upb/message/array.h"
#include "upb/message/message.h"

namespace hpb {
namespace internal {

// TODO: Implement std iterator for messages
template <typename T>
class RepeatedFieldScalarProxy;
template <typename T>
class RepeatedFieldStringProxy;

struct IteratorTestPeer;

template <typename T>
class Iterator;

template <typename PolicyT>
class ReferenceProxy;

template <typename PolicyT>
class InjectedRelationalsImpl {
  using RP = ReferenceProxy<PolicyT>;
  using V = typename PolicyT::value_type;
  friend bool operator==(RP a, V b) { return static_cast<V>(a) == b; }
  friend bool operator==(V a, RP b) { return a == static_cast<V>(b); }
  friend bool operator==(RP a, RP b) {
    return static_cast<V>(a) == static_cast<V>(b);
  }
  friend bool operator!=(RP a, V b) { return static_cast<V>(a) != b; }
  friend bool operator!=(V a, RP b) { return a != static_cast<V>(b); }
  friend bool operator!=(RP a, RP b) {
    return static_cast<V>(a) != static_cast<V>(b);
  }
  friend bool operator<(RP a, V b) { return static_cast<V>(a) < b; }
  friend bool operator<(V a, RP b) { return a < static_cast<V>(b); }
  friend bool operator<(RP a, RP b) {
    return static_cast<V>(a) < static_cast<V>(b);
  }
  friend bool operator<=(RP a, V b) { return static_cast<V>(a) <= b; }
  friend bool operator<=(V a, RP b) { return a <= static_cast<V>(b); }
  friend bool operator<=(RP a, RP b) {
    return static_cast<V>(a) <= static_cast<V>(b);
  }
  friend bool operator>(RP a, V b) { return static_cast<V>(a) > b; }
  friend bool operator>(V a, RP b) { return a > static_cast<V>(b); }
  friend bool operator>(RP a, RP b) {
    return static_cast<V>(a) > static_cast<V>(b);
  }
  friend bool operator>=(RP a, V b) { return static_cast<V>(a) >= b; }
  friend bool operator>=(V a, RP b) { return a >= static_cast<V>(b); }
  friend bool operator>=(RP a, RP b) {
    return static_cast<V>(a) >= static_cast<V>(b);
  }
};
class NoInjectedRelationalsImpl {};

// We need to inject relationals for the string references because the
// relationals for string_view are templates and won't allow for implicit
// conversions from ReferenceProxy to string_view before deduction.
template <typename PolicyT>
using InjectedRelationals = std::conditional_t<
    std::is_same_v<std::remove_const_t<typename PolicyT::value_type>,
                   absl::string_view>,
    InjectedRelationalsImpl<PolicyT>, NoInjectedRelationalsImpl>;

template <typename PolicyT>
class ReferenceProxy : InjectedRelationals<PolicyT> {
  using value_type = typename PolicyT::value_type;

 public:
  ReferenceProxy(const ReferenceProxy&) = default;
  ReferenceProxy& operator=(const ReferenceProxy& other) {
    // Assign through the references
    // TODO: Make this better for strings to avoid the copy.
    it_.Set(other.it_.Get());
    return *this;
  }
  friend void swap(ReferenceProxy a, ReferenceProxy b) { a.it_.swap(b.it_); }

  operator value_type() const { return it_.Get(); }
  void operator=(const value_type& value) const { it_.Set(value); }
  void operator=(value_type&& value) const { it_.Set(std::move(value)); }
  Iterator<PolicyT> operator&() const { return Iterator<PolicyT>(it_); }

 private:
  friend IteratorTestPeer;
  friend ReferenceProxy<typename PolicyT::AddConst>;
  friend Iterator<PolicyT>;

  explicit ReferenceProxy(typename PolicyT::Payload elem) : it_(elem) {}
  typename PolicyT::Payload it_;
};

template <template <typename> class PolicyTemplate, typename T>
class ReferenceProxy<PolicyTemplate<const T>>
    : InjectedRelationals<PolicyTemplate<const T>> {
  using PolicyT = PolicyTemplate<const T>;
  using value_type = typename PolicyT::value_type;

 public:
  ReferenceProxy(ReferenceProxy<PolicyTemplate<T>> p) : it_(p.it_) {}
  ReferenceProxy(const ReferenceProxy&) = default;
  ReferenceProxy& operator=(const ReferenceProxy&) = delete;

  operator value_type() const { return it_.Get(); }
  Iterator<PolicyT> operator&() const { return Iterator<PolicyT>(it_); }

 private:
  friend IteratorTestPeer;
  friend Iterator<PolicyT>;

  explicit ReferenceProxy(typename PolicyT::Payload elem) : it_(elem) {}
  typename PolicyT::Payload it_;
};

template <typename PolicyT>
class Iterator {
 public:
  using iterator_category = std::random_access_iterator_tag;
  using value_type = std::remove_const_t<typename PolicyT::value_type>;
  using difference_type = std::ptrdiff_t;
  using pointer = Iterator;
  using reference =
      std::conditional_t<PolicyT::kUseReferenceProxy, ReferenceProxy<PolicyT>,
                         typename PolicyT::value_type>;

  constexpr Iterator() noexcept : it_(nullptr) {}
  Iterator(const Iterator& other) = default;
  Iterator& operator=(const Iterator& other) = default;
  template <
      typename P = PolicyT,
      typename = std::enable_if_t<std::is_const<typename P::value_type>::value>>
  Iterator(const Iterator<typename P::RemoveConst>& other) : it_(other.it_) {}

  constexpr reference operator*() const noexcept {
    if constexpr (PolicyT::kUseReferenceProxy) {
      return reference(it_);
    } else {
      return it_.Get();
    }
  }
  // No operator-> needed because T is a scalar.

 private:
  // Hide the internal type.
  using iterator = Iterator;

 public:
  // {inc,dec}rementable
  constexpr iterator& operator++() noexcept {
    it_.AddOffset(1);
    return *this;
  }
  constexpr iterator operator++(int) noexcept {
    auto copy = *this;
    ++*this;
    return copy;
  }
  constexpr iterator& operator--() noexcept {
    it_.AddOffset(-1);
    return *this;
  }
  constexpr iterator operator--(int) noexcept {
    auto copy = *this;
    --*this;
    return copy;
  }

  // equality_comparable
  friend constexpr bool operator==(const iterator& x,
                                   const iterator& y) noexcept {
    return x.it_.Index() == y.it_.Index();
  }
  friend constexpr bool operator!=(const iterator& x,
                                   const iterator& y) noexcept {
    return !(x == y);
  }

  // less_than_comparable
  friend constexpr bool operator<(const iterator& x,
                                  const iterator& y) noexcept {
    return x.it_.Index() < y.it_.Index();
  }
  friend constexpr bool operator<=(const iterator& x,
                                   const iterator& y) noexcept {
    return !(y < x);
  }
  friend constexpr bool operator>(const iterator& x,
                                  const iterator& y) noexcept {
    return y < x;
  }
  friend constexpr bool operator>=(const iterator& x,
                                   const iterator& y) noexcept {
    return !(x < y);
  }

  constexpr iterator& operator+=(difference_type d) noexcept {
    it_.AddOffset(d);
    return *this;
  }
  constexpr iterator operator+(difference_type d) const noexcept {
    auto copy = *this;
    copy += d;
    return copy;
  }
  friend constexpr iterator operator+(const difference_type d,
                                      iterator it) noexcept {
    return it + d;
  }

  constexpr iterator& operator-=(difference_type d) noexcept {
    it_.AddOffset(-d);
    return *this;
  }
  constexpr iterator operator-(difference_type d) const noexcept {
    auto copy = *this;
    copy -= d;
    return copy;
  }

  // indexable
  constexpr reference operator[](difference_type d) const noexcept {
    auto copy = *this;
    copy += d;
    return *copy;
  }

  // random access iterator
  friend constexpr difference_type operator-(iterator x, iterator y) noexcept {
    return x.it_.Index() - y.it_.Index();
  }

 private:
  friend IteratorTestPeer;
  friend ReferenceProxy<PolicyT>;
  friend Iterator<typename PolicyT::AddConst>;
  template <typename U>
  friend class RepeatedFieldScalarProxy;
  template <typename U>
  friend class RepeatedFieldStringProxy;
  template <typename U>
  friend class RepeatedFieldProxy;

  // Create from internal::RepeatedFieldScalarProxy.
  explicit Iterator(typename PolicyT::Payload it) noexcept : it_(it) {}

  // The internal iterator.
  typename PolicyT::Payload it_;
};

template <typename T>
struct ScalarIteratorPolicy {
  static constexpr bool kUseReferenceProxy = true;
  using value_type = T;
  using RemoveConst = ScalarIteratorPolicy<std::remove_const_t<T>>;
  using AddConst = ScalarIteratorPolicy<const T>;

  struct Payload {
    T* value;
    void AddOffset(ptrdiff_t offset) { value += offset; }
    T Get() const { return *value; }
    void Set(T new_value) const { *value = new_value; }
    T* Index() const { return value; }

    void swap(Payload& other) {
      using std::swap;
      swap(*value, *other.value);
    }

    operator typename ScalarIteratorPolicy<const T>::Payload() const {
      return {value};
    }
  };
};

template <typename T>
struct StringIteratorPolicy {
  static constexpr bool kUseReferenceProxy = true;
  using value_type = T;
  using RemoveConst = StringIteratorPolicy<std::remove_const_t<T>>;
  using AddConst = StringIteratorPolicy<const T>;

  struct Payload {
    using Array =
        std::conditional_t<std::is_const_v<T>, const upb_Array, upb_Array>;
    Array* arr;
    upb_Arena* arena;
    size_t index;

    void AddOffset(ptrdiff_t offset) { index += offset; }
    absl::string_view Get() const {
      upb_MessageValue message_value = upb_Array_Get(arr, index);
      return absl::string_view(message_value.str_val.data,
                               message_value.str_val.size);
    }
    void Set(absl::string_view new_value) const {
      char* data =
          static_cast<char*>(upb_Arena_Malloc(arena, new_value.size()));
      memcpy(data, new_value.data(), new_value.size());
      upb_MessageValue message_value;
      message_value.str_val =
          upb_StringView_FromDataAndSize(data, new_value.size());
      upb_Array_Set(arr, index, message_value);
    }
    size_t Index() const { return index; }

    void swap(Payload& other) {
      upb_MessageValue a = upb_Array_Get(this->arr, this->index);
      upb_MessageValue b = upb_Array_Get(other.arr, other.index);
      upb_Array_Set(this->arr, this->index, b);
      upb_Array_Set(other.arr, other.index, a);
    }

    operator typename StringIteratorPolicy<const T>::Payload() const {
      return {arr, arena, index};
    }
  };
};

template <typename T>
struct MessageIteratorPolicy {
  static constexpr bool kUseReferenceProxy = false;
  using value_type = std::conditional_t<std::is_const_v<T>, typename T::CProxy,
                                        typename T::Proxy>;
  using RemoveConst = MessageIteratorPolicy<std::remove_const_t<T>>;
  using AddConst = MessageIteratorPolicy<const T>;

  struct Payload {
    using Array =
        std::conditional_t<std::is_const_v<T>, const upb_Array, upb_Array>;
    upb_Message** arr;
    upb_Arena* arena;

    void AddOffset(ptrdiff_t offset) { arr += offset; }
    auto Get() const {
      if constexpr (std::is_const_v<T>) {
        return ::hpb::interop::upb::MakeCHandle<
            typename std::remove_const_t<T>>(*arr, arena);
      } else {
        return hpb::interop::upb::MakeHandle<T>(*arr, arena);
      }
    }
    auto Index() const { return arr; }
  };
};

}  // namespace internal
}  // namespace hpb

#endif  // PROTOBUF_HPB_REPEATED_FIELD_ITERATOR_H_
