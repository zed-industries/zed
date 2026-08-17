// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_SANITIZERS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_SANITIZERS_H_

#include "base/memory/asan_interface.h"

#if defined(ADDRESS_SANITIZER)
#include <sanitizer/asan_interface.h>
#endif

#if defined(ADDRESS_SANITIZER)
#define ASAN_REGION_IS_POISONED(addr, size) \
  __asan_region_is_poisoned(addr, size)
#define NO_SANITIZE_ADDRESS __attribute__((no_sanitize_address))
class AsanUnpoisonScope {
 public:
  AsanUnpoisonScope(const void* addr, size_t size)
      : addr_(addr), size_(size), was_poisoned_(false) {
    if (!ASAN_REGION_IS_POISONED(const_cast<void*>(addr_), size_))
      return;
    ASAN_UNPOISON_MEMORY_REGION(addr_, size_);
    was_poisoned_ = true;
  }
  ~AsanUnpoisonScope() {
    if (was_poisoned_)
      ASAN_POISON_MEMORY_REGION(addr_, size_);
  }

 private:
  const void* addr_;
  size_t size_;
  bool was_poisoned_;
};
#else
#define ASAN_REGION_IS_POISONED(addr, size) \
  ((void)(addr), (void)(size), (void*)nullptr)
#define NO_SANITIZE_ADDRESS
class AsanUnpoisonScope {
 public:
  AsanUnpoisonScope(const void*, size_t) {}
  ~AsanUnpoisonScope() {}
};
#endif

#if defined(LEAK_SANITIZER)
#include <sanitizer/lsan_interface.h>
#endif

#if defined(MEMORY_SANITIZER)
#include <sanitizer/msan_interface.h>
#define NO_SANITIZE_MEMORY __attribute__((no_sanitize_memory))
#else
#define NO_SANITIZE_MEMORY
#endif

#if defined(THREAD_SANITIZER)
#define NO_SANITIZE_THREAD __attribute__((no_sanitize_thread))
#else
#define NO_SANITIZE_THREAD
#endif

#if defined(__clang__)
#define NO_SANITIZE_HWADDRESS __attribute__((no_sanitize("hwaddress")))
#else
#define NO_SANITIZE_HWADDRESS
#endif

// NO_SANITIZE_UNRELATED_CAST - Disable runtime checks related to casts between
// unrelated objects (-fsanitize=cfi-unrelated-cast or -fsanitize=vptr).
#if defined(__clang__)
#define NO_SANITIZE_UNRELATED_CAST \
  __attribute__((no_sanitize("cfi-unrelated-cast", "vptr")))
#define NO_SANITIZE_CFI_ICALL __attribute__((no_sanitize("cfi-icall")))
#else
#define NO_SANITIZE_UNRELATED_CAST
#define NO_SANITIZE_CFI_ICALL
#endif

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_SANITIZERS_H_
