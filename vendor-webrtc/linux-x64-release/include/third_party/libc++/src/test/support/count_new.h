//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef COUNT_NEW_H
#define COUNT_NEW_H

#include <algorithm>
#include <cassert>
#include <cerrno>
#include <cstdlib>
#include <new>
#include <type_traits>

#include "test_macros.h"

#if defined(TEST_HAS_SANITIZERS)
#define DISABLE_NEW_COUNT
#endif

namespace detail
{
[[noreturn]] inline void throw_bad_alloc_helper() {
#ifndef TEST_HAS_NO_EXCEPTIONS
  throw std::bad_alloc();
#else
       std::abort();
#endif
}
}

class MemCounter
{
public:
    // Make MemCounter super hard to accidentally construct or copy.
    class MemCounterCtorArg_ {};
    explicit MemCounter(MemCounterCtorArg_) { reset(); }

private:
    MemCounter(MemCounter const &);
    MemCounter & operator=(MemCounter const &);

public:
    // All checks return true when disable_checking is enabled.
    static const bool disable_checking;

    // Disallow any allocations from occurring. Useful for testing that
    // code doesn't perform any allocations.
    bool disable_allocations;

    // number of allocations to throw after. Default (unsigned)-1. If
    // throw_after has the default value it will never be decremented.
    static const unsigned never_throw_value = static_cast<unsigned>(-1);
    unsigned throw_after;

    int outstanding_new;
    int new_called;
    int delete_called;
    int aligned_new_called;
    int aligned_delete_called;
    std::size_t last_new_size;
    std::size_t last_new_align;
    std::size_t last_delete_align;

    int outstanding_array_new;
    int new_array_called;
    int delete_array_called;
    int aligned_new_array_called;
    int aligned_delete_array_called;
    std::size_t last_new_array_size;
    std::size_t last_new_array_align;
    std::size_t last_delete_array_align;

public:
    void newCalled(std::size_t s)
    {
        assert(disable_allocations == false);
        if (throw_after == 0) {
            throw_after = never_throw_value;
            detail::throw_bad_alloc_helper();
        } else if (throw_after != never_throw_value) {
            --throw_after;
        }
        ++new_called;
        ++outstanding_new;
        last_new_size = s;
    }

    void alignedNewCalled(std::size_t s, std::size_t a) {
      newCalled(s);
      ++aligned_new_called;
      last_new_align = a;
    }

    void deleteCalled(void * p)
    {
      if (p) {
        --outstanding_new;
        ++delete_called;
      }
    }

    void alignedDeleteCalled(void *p, std::size_t a) {
      if (p) {
        deleteCalled(p);
        ++aligned_delete_called;
        last_delete_align = a;
      }
    }

    void newArrayCalled(std::size_t s)
    {
        assert(disable_allocations == false);
        if (throw_after == 0) {
            throw_after = never_throw_value;
            detail::throw_bad_alloc_helper();
        } else {
            // don't decrement throw_after here. newCalled will end up doing that.
        }
        ++outstanding_array_new;
        ++new_array_called;
        last_new_array_size = s;
    }

    void alignedNewArrayCalled(std::size_t s, std::size_t a) {
      newArrayCalled(s);
      ++aligned_new_array_called;
      last_new_array_align = a;
    }

    void deleteArrayCalled(void * p)
    {
        assert(p);
        --outstanding_array_new;
        ++delete_array_called;
    }

    void alignedDeleteArrayCalled(void * p, std::size_t a) {
      deleteArrayCalled(p);
      ++aligned_delete_array_called;
      last_delete_array_align = a;
    }

    void disableAllocations()
    {
        disable_allocations = true;
    }

    void enableAllocations()
    {
        disable_allocations = false;
    }

    void reset()
    {
        disable_allocations = false;
        throw_after = never_throw_value;

        outstanding_new = 0;
        new_called = 0;
        delete_called = 0;
        aligned_new_called = 0;
        aligned_delete_called = 0;
        last_new_size = 0;
        last_new_align = 0;

        outstanding_array_new = 0;
        new_array_called = 0;
        delete_array_called = 0;
        aligned_new_array_called = 0;
        aligned_delete_array_called = 0;
        last_new_array_size = 0;
        last_new_array_align = 0;
    }

public:
    bool checkOutstandingNewEq(int n) const
    {
        return disable_checking || n == outstanding_new;
    }

    bool checkOutstandingNewLessThanOrEqual(int n) const
    {
        return disable_checking || outstanding_new <= n;
    }

    bool checkOutstandingNewNotEq(int n) const
    {
        return disable_checking || n != outstanding_new;
    }

    bool checkNewCalledEq(int n) const
    {
        return disable_checking || n == new_called;
    }

    bool checkNewCalledNotEq(int n) const
    {
        return disable_checking || n != new_called;
    }

    bool checkNewCalledGreaterThan(int n) const
    {
        return disable_checking || new_called > n;
    }

    bool checkDeleteCalledEq(int n) const
    {
        return disable_checking || n == delete_called;
    }

    bool checkDeleteCalledNotEq(int n) const
    {
        return disable_checking || n != delete_called;
    }

    bool checkDeleteCalledGreaterThan(int n) const
    {
        return disable_checking || delete_called > n;
    }

    bool checkAlignedNewCalledEq(int n) const
    {
        return disable_checking || n == aligned_new_called;
    }

    bool checkAlignedNewCalledNotEq(int n) const
    {
        return disable_checking || n != aligned_new_called;
    }

    bool checkAlignedNewCalledGreaterThan(int n) const
    {
        return disable_checking || aligned_new_called > n;
    }

    bool checkAlignedDeleteCalledEq(int n) const
    {
        return disable_checking || n == aligned_delete_called;
    }

    bool checkAlignedDeleteCalledNotEq(int n) const
    {
        return disable_checking || n != aligned_delete_called;
    }

    bool checkLastNewSizeEq(std::size_t n) const
    {
        return disable_checking || n == last_new_size;
    }

    bool checkLastNewSizeNotEq(std::size_t n) const
    {
        return disable_checking || n != last_new_size;
    }

    bool checkLastNewSizeGe(std::size_t n) const
    {
        return disable_checking || last_new_size >= n;
    }

    bool checkLastNewAlignEq(std::size_t n) const
    {
        return disable_checking || n == last_new_align;
    }

    bool checkLastNewAlignNotEq(std::size_t n) const
    {
        return disable_checking || n != last_new_align;
    }

    bool checkLastNewAlignGe(std::size_t n) const
    {
        return disable_checking || last_new_align >= n;
    }

    bool checkLastDeleteAlignEq(std::size_t n) const
    {
        return disable_checking || n == last_delete_align;
    }

    bool checkLastDeleteAlignNotEq(std::size_t n) const
    {
        return disable_checking || n != last_delete_align;
    }

    bool checkOutstandingArrayNewEq(int n) const
    {
        return disable_checking || n == outstanding_array_new;
    }

    bool checkOutstandingArrayNewNotEq(int n) const
    {
        return disable_checking || n != outstanding_array_new;
    }

    bool checkNewArrayCalledEq(int n) const
    {
        return disable_checking || n == new_array_called;
    }

    bool checkNewArrayCalledNotEq(int n) const
    {
        return disable_checking || n != new_array_called;
    }

    bool checkDeleteArrayCalledEq(int n) const
    {
        return disable_checking || n == delete_array_called;
    }

    bool checkDeleteArrayCalledNotEq(int n) const
    {
        return disable_checking || n != delete_array_called;
    }

    bool checkAlignedNewArrayCalledEq(int n) const
    {
        return disable_checking || n == aligned_new_array_called;
    }

    bool checkAlignedNewArrayCalledNotEq(int n) const
    {
        return disable_checking || n != aligned_new_array_called;
    }

    bool checkAlignedNewArrayCalledGreaterThan(int n) const
    {
        return disable_checking || aligned_new_array_called > n;
    }

    bool checkAlignedDeleteArrayCalledEq(int n) const
    {
        return disable_checking || n == aligned_delete_array_called;
    }

    bool checkAlignedDeleteArrayCalledNotEq(int n) const
    {
        return disable_checking || n != aligned_delete_array_called;
    }

    bool checkLastNewArraySizeEq(std::size_t n) const
    {
        return disable_checking || n == last_new_array_size;
    }

    bool checkLastNewArraySizeNotEq(std::size_t n) const
    {
        return disable_checking || n != last_new_array_size;
    }

    bool checkLastNewArrayAlignEq(std::size_t n) const
    {
        return disable_checking || n == last_new_array_align;
    }

    bool checkLastNewArrayAlignNotEq(std::size_t n) const
    {
        return disable_checking || n != last_new_array_align;
    }
};

#ifdef DISABLE_NEW_COUNT
  const bool MemCounter::disable_checking = true;
#else
  const bool MemCounter::disable_checking = false;
#endif

TEST_DIAGNOSTIC_PUSH
TEST_MSVC_DIAGNOSTIC_IGNORED(4640) // '%s' construction of local static object is not thread safe (/Zc:threadSafeInit-)
inline MemCounter* getGlobalMemCounter() {
  static MemCounter counter((MemCounter::MemCounterCtorArg_()));
  return &counter;
}
TEST_DIAGNOSTIC_POP

MemCounter &globalMemCounter = *getGlobalMemCounter();

#ifndef DISABLE_NEW_COUNT
// operator new(size_t[, nothrow_t]) and operator delete(size_t[, nothrow_t])
void* operator new(std::size_t s) TEST_THROW_SPEC(std::bad_alloc) {
  getGlobalMemCounter()->newCalled(s);
  if (s == 0)
    ++s;
  void* p = std::malloc(s);
  if (p == nullptr)
    detail::throw_bad_alloc_helper();
  return p;
}

void* operator new(std::size_t s, std::nothrow_t const&) TEST_NOEXCEPT {
#  ifdef TEST_HAS_NO_EXCEPTIONS
  getGlobalMemCounter()->newCalled(s);
#  else
  try {
    getGlobalMemCounter()->newCalled(s);
  } catch (std::bad_alloc const&) {
    return nullptr;
  }
#  endif
  return std::malloc(s);
}

void operator delete(void* p) TEST_NOEXCEPT {
  getGlobalMemCounter()->deleteCalled(p);
  std::free(p);
}

void operator delete(void* p, std::nothrow_t const&) TEST_NOEXCEPT {
  getGlobalMemCounter()->deleteCalled(p);
  std::free(p);
}

// operator new[](size_t[, nothrow_t]) and operator delete[](size_t[, nothrow_t])
void* operator new[](std::size_t s) TEST_THROW_SPEC(std::bad_alloc) {
  getGlobalMemCounter()->newArrayCalled(s);
  if (s == 0)
    s++;
  void* p = std::malloc(s);
  if (p == nullptr)
    detail::throw_bad_alloc_helper();
  return p;
}

void* operator new[](std::size_t s, std::nothrow_t const&) TEST_NOEXCEPT {
#  ifdef TEST_HAS_NO_EXCEPTIONS
  getGlobalMemCounter()->newArrayCalled(s);
#  else
  try {
    getGlobalMemCounter()->newArrayCalled(s);
  } catch (std::bad_alloc const&) {
    return nullptr;
  }
#  endif
  return std::malloc(s);
}

void operator delete[](void* p) TEST_NOEXCEPT {
  getGlobalMemCounter()->deleteArrayCalled(p);
  std::free(p);
}

void operator delete[](void* p, std::nothrow_t const&) TEST_NOEXCEPT {
  getGlobalMemCounter()->deleteArrayCalled(p);
  std::free(p);
}

#  ifndef TEST_HAS_NO_ALIGNED_ALLOCATION
#    if defined(_LIBCPP_MSVCRT_LIKE) || (!defined(_LIBCPP_VERSION) && defined(_WIN32))
#      define USE_ALIGNED_ALLOC
#    endif

#    if defined(__APPLE__)
#      if (defined(__ENVIRONMENT_MAC_OS_X_VERSION_MIN_REQUIRED__) &&                                                   \
           __ENVIRONMENT_MAC_OS_X_VERSION_MIN_REQUIRED__ < 101500)
#        define TEST_HAS_NO_C11_ALIGNED_ALLOC
#      endif
#    elif defined(__ANDROID__) && __ANDROID_API__ < 28
#      define TEST_HAS_NO_C11_ALIGNED_ALLOC
#    endif

inline void* allocate_aligned_impl(std::size_t size, std::align_val_t align) {
  const std::size_t alignment = static_cast<std::size_t>(align);
  void* ret                   = nullptr;
#    ifdef USE_ALIGNED_ALLOC
  ret = _aligned_malloc(size, alignment);
#    elif TEST_STD_VER >= 17 && !defined(TEST_HAS_NO_C11_ALIGNED_ALLOC)
  size_t rounded_size = (size + alignment - 1) & ~(alignment - 1);
  ret                 = aligned_alloc(alignment, size > rounded_size ? size : rounded_size);
#    else
  assert(posix_memalign(&ret, std::max(alignment, sizeof(void*)), size) != EINVAL);
#    endif
  return ret;
}

inline void free_aligned_impl(void* ptr, std::align_val_t) {
  if (ptr) {
#    ifdef USE_ALIGNED_ALLOC
    ::_aligned_free(ptr);
#    else
    ::free(ptr);
#    endif
  }
}

// operator new(size_t, align_val_t[, nothrow_t]) and operator delete(size_t, align_val_t[, nothrow_t])
void* operator new(std::size_t s, std::align_val_t av) TEST_THROW_SPEC(std::bad_alloc) {
  getGlobalMemCounter()->alignedNewCalled(s, static_cast<std::size_t>(av));
  void* p = allocate_aligned_impl(s, av);
  if (p == nullptr)
    detail::throw_bad_alloc_helper();
  return p;
}

void* operator new(std::size_t s, std::align_val_t av, std::nothrow_t const&) TEST_NOEXCEPT {
#    ifdef TEST_HAS_NO_EXCEPTIONS
  getGlobalMemCounter()->alignedNewCalled(s, static_cast<std::size_t>(av));
#    else
  try {
    getGlobalMemCounter()->alignedNewCalled(s, static_cast<std::size_t>(av));
  } catch (std::bad_alloc const&) {
    return nullptr;
  }
#    endif
  return allocate_aligned_impl(s, av);
}

void operator delete(void* p, std::align_val_t av) TEST_NOEXCEPT {
  getGlobalMemCounter()->alignedDeleteCalled(p, static_cast<std::size_t>(av));
  free_aligned_impl(p, av);
}

void operator delete(void* p, std::align_val_t av, std::nothrow_t const&) TEST_NOEXCEPT {
  getGlobalMemCounter()->alignedDeleteCalled(p, static_cast<std::size_t>(av));
  free_aligned_impl(p, av);
}

// operator new[](size_t, align_val_t[, nothrow_t]) and operator delete[](size_t, align_val_t[, nothrow_t])
void* operator new[](std::size_t s, std::align_val_t av) TEST_THROW_SPEC(std::bad_alloc) {
  getGlobalMemCounter()->alignedNewArrayCalled(s, static_cast<std::size_t>(av));
  void* p = allocate_aligned_impl(s, av);
  if (p == nullptr)
    detail::throw_bad_alloc_helper();
  return p;
}

void* operator new[](std::size_t s, std::align_val_t av, std::nothrow_t const&) TEST_NOEXCEPT {
#    ifdef TEST_HAS_NO_EXCEPTIONS
  getGlobalMemCounter()->alignedNewArrayCalled(s, static_cast<std::size_t>(av));
#    else
  try {
    getGlobalMemCounter()->alignedNewArrayCalled(s, static_cast<std::size_t>(av));
  } catch (std::bad_alloc const&) {
    return nullptr;
  }
#    endif
  return allocate_aligned_impl(s, av);
}

void operator delete[](void* p, std::align_val_t av) TEST_NOEXCEPT {
  getGlobalMemCounter()->alignedDeleteArrayCalled(p, static_cast<std::size_t>(av));
  free_aligned_impl(p, av);
}

void operator delete[](void* p, std::align_val_t av, std::nothrow_t const&) TEST_NOEXCEPT {
  getGlobalMemCounter()->alignedDeleteArrayCalled(p, static_cast<std::size_t>(av));
  free_aligned_impl(p, av);
}

#  endif // TEST_HAS_NO_ALIGNED_ALLOCATION

#endif // DISABLE_NEW_COUNT

struct DisableAllocationGuard {
    explicit DisableAllocationGuard(bool disable = true) : m_disabled(disable)
    {
        // Don't re-disable if already disabled.
        if (globalMemCounter.disable_allocations == true) m_disabled = false;
        if (m_disabled) globalMemCounter.disableAllocations();
    }

    void release() {
        if (m_disabled) globalMemCounter.enableAllocations();
        m_disabled = false;
    }

    ~DisableAllocationGuard() {
        release();
    }

private:
    bool m_disabled;

    DisableAllocationGuard(DisableAllocationGuard const&);
    DisableAllocationGuard& operator=(DisableAllocationGuard const&);
};

#if TEST_STD_VER >= 20

struct ConstexprDisableAllocationGuard {
    TEST_CONSTEXPR_CXX14 explicit ConstexprDisableAllocationGuard(bool disable = true) : m_disabled(disable)
    {
        if (!TEST_IS_CONSTANT_EVALUATED) {
            // Don't re-disable if already disabled.
            if (globalMemCounter.disable_allocations == true) m_disabled = false;
            if (m_disabled) globalMemCounter.disableAllocations();
        } else {
            m_disabled = false;
        }
    }

    TEST_CONSTEXPR_CXX14 void release() {
        if (!TEST_IS_CONSTANT_EVALUATED) {
            if (m_disabled) globalMemCounter.enableAllocations();
            m_disabled = false;
        }
    }

    TEST_CONSTEXPR_CXX20 ~ConstexprDisableAllocationGuard() {
        release();
    }

private:
    bool m_disabled;

    ConstexprDisableAllocationGuard(ConstexprDisableAllocationGuard const&);
    ConstexprDisableAllocationGuard& operator=(ConstexprDisableAllocationGuard const&);
};

#endif

struct RequireAllocationGuard {
    explicit RequireAllocationGuard(std::size_t RequireAtLeast = 1)
            : m_req_alloc(RequireAtLeast),
              m_new_count_on_init(globalMemCounter.new_called),
              m_outstanding_new_on_init(globalMemCounter.outstanding_new),
              m_exactly(false)
    {
    }

    void requireAtLeast(std::size_t N) { m_req_alloc = N; m_exactly = false; }
    void requireExactly(std::size_t N) { m_req_alloc = N; m_exactly = true; }

    ~RequireAllocationGuard() {
        assert(globalMemCounter.checkOutstandingNewEq(static_cast<int>(m_outstanding_new_on_init)));
        std::size_t Expect = m_new_count_on_init + m_req_alloc;
        assert(globalMemCounter.checkNewCalledEq(static_cast<int>(Expect)) ||
               (!m_exactly && globalMemCounter.checkNewCalledGreaterThan(static_cast<int>(Expect))));
    }

private:
    std::size_t m_req_alloc;
    const std::size_t m_new_count_on_init;
    const std::size_t m_outstanding_new_on_init;
    bool m_exactly;
    RequireAllocationGuard(RequireAllocationGuard const&);
    RequireAllocationGuard& operator=(RequireAllocationGuard const&);
};

#endif /* COUNT_NEW_H */
