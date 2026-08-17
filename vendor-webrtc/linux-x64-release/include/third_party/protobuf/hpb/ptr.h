// Protocol Buffers - Google's data interchange format
// Copyright 2024 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_HPB_PTR_H__
#define GOOGLE_PROTOBUF_HPB_PTR_H__

#include <memory>
#include <type_traits>

namespace hpb {

template <typename T>
using Proxy = std::conditional_t<std::is_const<T>::value,
                                 typename std::remove_const_t<T>::CProxy,
                                 typename T::Proxy>;

// Provides convenient access to Proxy and CProxy message types.
//
// Using rebinding and handling of const, Ptr<Message> and Ptr<const Message>
// allows copying const with T* const and avoids using non-copyable Proxy types
// directly.
template <typename T>
class Ptr final {
 public:
  Ptr() = delete;

  // Implicit conversions
  Ptr(T* m) : p_(m) {}                // NOLINT
  Ptr(const Proxy<T>* p) : p_(*p) {}  // NOLINT
  Ptr(Proxy<T> p) : p_(p) {}          // NOLINT
  Ptr(const Ptr& m) = default;

  Ptr& operator=(Ptr v) & {
    Proxy<T>::Rebind(p_, v.p_);
    return *this;
  }

  Proxy<T> operator*() const { return p_; }
  Proxy<T>* operator->() const {
    return const_cast<Proxy<T>*>(std::addressof(p_));
  }

#ifdef __clang__
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wclass-conversion"
#endif
  template <typename U = T, std::enable_if_t<!std::is_const<U>::value, int> = 0>
  operator Ptr<const T>() const {
    Proxy<const T> p(p_);
    return Ptr<const T>(&p);
  }
#ifdef __clang__
#pragma clang diagnostic pop
#endif

 private:
  friend class Ptr<const T>;
  friend typename T::Access;

  Proxy<T> p_;
};

// Suppress -Wctad-maybe-unsupported with our manual deduction guide
template <typename T>
Ptr(T* m) -> Ptr<T>;

}  // namespace hpb

#endif  // GOOGLE_PROTOBUF_HPB_PTR_H__
