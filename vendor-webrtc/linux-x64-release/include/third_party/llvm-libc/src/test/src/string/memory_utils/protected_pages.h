//===-- protected_pages.h -------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//
// This file provides protected pages that fault when accessing prior or past
// it. This is useful to check memory functions that must not access outside of
// the provided size limited buffer.
//===----------------------------------------------------------------------===//

#ifndef LIBC_TEST_SRC_STRING_MEMORY_UTILS_PROTECTED_PAGES_H
#define LIBC_TEST_SRC_STRING_MEMORY_UTILS_PROTECTED_PAGES_H

#include "src/__support/macros/properties/os.h" // LIBC_TARGET_OS_IS_LINUX
#if defined(LIBC_FULL_BUILD) || !defined(LIBC_TARGET_OS_IS_LINUX)
#error "Protected pages requires mmap and cannot be used in full build mode."
#endif // defined(LIBC_FULL_BUILD) || !defined(LIBC_TARGET_OS_IS_LINUX)

#include "src/__support/macros/attributes.h" // LIBC_INLINE
#include <stddef.h>                          // size_t
#include <stdint.h>                          // uint8_t
#include <sys/mman.h>                        // mmap, munmap
#include <unistd.h>                          // sysconf, _SC_PAGESIZE

// Returns mmap page size.
LIBC_INLINE size_t GetPageSize() {
  static const size_t PAGE_SIZE = sysconf(_SC_PAGESIZE);
  return PAGE_SIZE;
}

// Represents a page of memory whose access can be configured throught the
// 'WithAccess' function. Accessing data above or below this page will trap as
// it is sandwiched between two pages with no read / write access.
struct Page {
  // Returns an aligned pointer that can be accessed up to page_size. Accessing
  // data at ptr[-1] will fault.
  LIBC_INLINE uint8_t *bottom(size_t size) const {
    if (size >= page_size)
      __builtin_trap();
    return page_ptr;
  }
  // Returns a pointer to a buffer that can be accessed up to size. Accessing
  // data at ptr[size] will trap.
  LIBC_INLINE uint8_t *top(size_t size) const {
    return page_ptr + page_size - size;
  }

  // protection is one of PROT_READ / PROT_WRITE.
  LIBC_INLINE Page &WithAccess(int protection) {
    if (mprotect(page_ptr, page_size, protection) != 0)
      __builtin_trap();
    return *this;
  }

  const size_t page_size;
  uint8_t *const page_ptr;
};

// Allocates 5 consecutive pages that will trap if accessed.
// | page layout | access | page name |
// |-------------|--------|:---------:|
// | 0           | trap   |           |
// | 1           | custom |     A     |
// | 2           | trap   |           |
// | 3           | custom |     B     |
// | 4           | trap   |           |
//
// The pages A and B can be retrieved as with 'GetPageA' / 'GetPageB' and their
// accesses can be customized through the 'WithAccess' function.
struct ProtectedPages {
  static constexpr size_t PAGES = 5;

  ProtectedPages()
      : page_size(GetPageSize()),
        ptr(mmap(/*address*/ nullptr, /*length*/ PAGES * page_size,
                 /*protection*/ PROT_NONE,
                 /*flags*/ MAP_PRIVATE | MAP_ANONYMOUS, /*fd*/ -1,
                 /*offset*/ 0)) {
    if (reinterpret_cast<intptr_t>(ptr) == -1)
      __builtin_trap();
  }
  ~ProtectedPages() { munmap(ptr, PAGES * page_size); }

  LIBC_INLINE Page GetPageA() const { return Page{page_size, page<1>()}; }
  LIBC_INLINE Page GetPageB() const { return Page{page_size, page<3>()}; }

private:
  template <size_t index> LIBC_INLINE uint8_t *page() const {
    static_assert(index < PAGES);
    return static_cast<uint8_t *>(ptr) + (index * page_size);
  }

  const size_t page_size;
  void *const ptr = nullptr;
};

#endif // LIBC_TEST_SRC_STRING_MEMORY_UTILS_PROTECTED_PAGES_H
