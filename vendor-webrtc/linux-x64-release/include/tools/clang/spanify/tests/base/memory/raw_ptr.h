// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
#ifndef TOOLS_CLANG_SPANIFY_TESTS_BASE_MEMORY_RAW_PTR_H_
#define TOOLS_CLANG_SPANIFY_TESTS_BASE_MEMORY_RAW_PTR_H_

namespace base {
template <typename T>
class raw_ptr {
 public:
  raw_ptr() {}

  raw_ptr(T* data) : data_(data) {}

  operator T*() const { return data_; }

  T& operator[](int n) { return data_[n]; }

  constexpr raw_ptr& operator++() {
    data_++;
    return *this;
  }

  constexpr raw_ptr operator++(int /* post_increment */) {
    raw_ptr result = *this;
    ++(*this);
    return result;
  }

  constexpr raw_ptr& operator+=(int delta_elems) {
    data_ += delta_elems;
    return *this;
  }

  friend constexpr raw_ptr operator+(const raw_ptr& p, int delta_elems) {
    T* data = p.data_ + delta_elems;
    return data;
  }

  constexpr T& operator*() const { return *data_; }

  T* get() { return data_; }

  constexpr explicit operator bool() const { return !!data_; }

 private:
  T* data_;
};
}  // namespace base
using base::raw_ptr;

#endif  // TOOLS_CLANG_SPANIFY_TESTS_BASE_MEMORY_RAW_PTR_H_
