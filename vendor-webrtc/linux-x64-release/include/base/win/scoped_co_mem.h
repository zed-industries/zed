// Copyright 2011 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_WIN_SCOPED_CO_MEM_H_
#define BASE_WIN_SCOPED_CO_MEM_H_

#include <objbase.h>

#include <utility>

#include "base/check.h"
#include "base/memory/raw_ptr_exclusion.h"

namespace base {
namespace win {

// Simple scoped memory releaser class for COM allocated memory.
// Example:
//   base::win::ScopedCoMem<ITEMIDLIST> file_item;
//   SHGetSomeInfo(&file_item, ...);
//   ...
//   return;  <-- memory released
template <typename T>
class ScopedCoMem {
 public:
  ScopedCoMem() : mem_ptr_(nullptr) {}

  ScopedCoMem(const ScopedCoMem&) = delete;
  ScopedCoMem& operator=(const ScopedCoMem&) = delete;

  ScopedCoMem(ScopedCoMem&& other)
      : mem_ptr_(std::exchange(other.mem_ptr_, nullptr)) {}
  ScopedCoMem& operator=(ScopedCoMem&& other) {
    Reset(std::exchange(other.mem_ptr_, nullptr));
    return *this;
  }

  ~ScopedCoMem() { Reset(nullptr); }

  T** operator&() {               // NOLINT
    DCHECK(mem_ptr_ == nullptr);  // To catch memory leaks.
    return &mem_ptr_;
  }

  operator T*() { return mem_ptr_; }

  T* operator->() {
    DCHECK(mem_ptr_ != NULL);
    return mem_ptr_;
  }

  const T* operator->() const {
    DCHECK(mem_ptr_ != NULL);
    return mem_ptr_;
  }

  void Reset(T* ptr) {
    if (mem_ptr_) {
      CoTaskMemFree(mem_ptr_);
    }
    mem_ptr_ = ptr;
  }

  T* get() const { return mem_ptr_; }

 private:
  // RAW_PTR_EXCLUSION: This memory is handled by the OS instead
  // of PartitionAlloc, so there's no point rewriting it to a
  // raw_ptr
  RAW_PTR_EXCLUSION T* mem_ptr_;
};

}  // namespace win
}  // namespace base

#endif  // BASE_WIN_SCOPED_CO_MEM_H_
