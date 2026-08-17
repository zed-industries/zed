// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MEMORY_WEAK_AUTO_RESET_H_
#define BASE_MEMORY_WEAK_AUTO_RESET_H_

#include "base/memory/weak_ptr.h"

namespace base {

// Sets a field of an object to a specified value, then returns it to its
// original value when the WeakAutoReset instance goes out of scope. Because a
// weak pointer is used, if the target object is destroyed, no attempt is made
// to restore the original value and no UAF occurs.
//
// Note that as of C++17 we can use CTAD to infer template parameters from
// constructor args; it is valid to write:
//   WeakAutoReset war(myobj->GetWeakPtr(), &MyClass::member_, new_value);
// without specifying explicit types in the classname.
template <class T, class U>
class WeakAutoReset {
 public:
  // Create an empty object that does nothing, you may move a value into this
  // object via assignment.
  WeakAutoReset() = default;

  // Sets member `field` of object pointed to by `ptr` to `new_value`. `ptr`
  // must be valid at time of construction. If `ptr` is still valid when this
  // object goes out of scope, the member will be returned to its original
  // value.
  WeakAutoReset(base::WeakPtr<T> ptr, U T::*field, U new_value)
      : ptr_(ptr),
        field_(field),
        old_value_(std::exchange(ptr.get()->*field, std::move(new_value))) {}

  // Move constructor.
  WeakAutoReset(WeakAutoReset&& other)
      : ptr_(std::move(other.ptr_)),
        field_(std::exchange(other.field_, nullptr)),
        old_value_(std::move(other.old_value_)) {}

  // Move assignment operator.
  WeakAutoReset& operator=(WeakAutoReset&& other) {
    if (this != &other) {
      // If we're already tracking a value, make sure to restore it before
      // overwriting our target.
      Reset();
      ptr_ = std::move(other.ptr_);
      field_ = std::exchange(other.field_, nullptr);
      old_value_ = std::move(other.old_value_);
    }
    return *this;
  }

  ~WeakAutoReset() { Reset(); }

 private:
  void Reset() {
    if (ptr_) {
      ptr_.get()->*field_ = std::move(old_value_);
    }
  }

  base::WeakPtr<T> ptr_;
  U T::*field_ = nullptr;
  U old_value_ = U();
};

}  // namespace base

#endif  // BASE_MEMORY_WEAK_AUTO_RESET_H_
