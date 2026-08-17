/*
 * Copyright (C) 2017 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef INCLUDE_PERFETTO_EXT_BASE_UTILS_H_
#define INCLUDE_PERFETTO_EXT_BASE_UTILS_H_

#include <errno.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

#include <atomic>
#include <functional>
#include <memory>
#include <string>

#include "perfetto/base/build_config.h"
#include "perfetto/base/compiler.h"
#include "perfetto/ext/base/sys_types.h"

#if PERFETTO_BUILDFLAG(PERFETTO_OS_WIN)
// Even if Windows has errno.h, the all syscall-restart behavior does not apply.
// Trying to handle EINTR can cause more harm than good if errno is left stale.
// Chromium does the same.
#define PERFETTO_EINTR(x) (x)
#else
#define PERFETTO_EINTR(x)                                   \
  ([&] {                                                    \
    decltype(x) eintr_wrapper_result;                       \
    do {                                                    \
      eintr_wrapper_result = (x);                           \
    } while (eintr_wrapper_result == -1 && errno == EINTR); \
    return eintr_wrapper_result;                            \
  }())
#endif

namespace perfetto {
namespace base {

namespace internal {
extern std::atomic<uint32_t> g_cached_page_size;
uint32_t GetSysPageSizeSlowpath();
}  // namespace internal

// Returns the system's page size. Use this when dealing with mmap, madvise and
// similar mm-related syscalls.
// This function might be called in hot paths. Avoid calling getpagesize() all
// the times, in many implementations getpagesize() calls sysconf() which is
// not cheap.
inline uint32_t GetSysPageSize() {
  const uint32_t page_size =
      internal::g_cached_page_size.load(std::memory_order_relaxed);
  return page_size != 0 ? page_size : internal::GetSysPageSizeSlowpath();
}

template <typename T, size_t TSize>
constexpr size_t ArraySize(const T (&)[TSize]) {
  return TSize;
}

// Function object which invokes 'free' on its parameter, which must be
// a pointer. Can be used to store malloc-allocated pointers in std::unique_ptr:
//
// std::unique_ptr<int, base::FreeDeleter> foo_ptr(
//     static_cast<int*>(malloc(sizeof(int))));
struct FreeDeleter {
  inline void operator()(void* ptr) const { free(ptr); }
};

template <typename T>
constexpr T AssumeLittleEndian(T value) {
#if !PERFETTO_IS_LITTLE_ENDIAN()
  static_assert(false, "Unimplemented on big-endian archs");
#endif
  return value;
}

// Round up |size| to a multiple of |alignment| (must be a power of two).
inline constexpr size_t AlignUp(size_t size, size_t alignment) {
  return (size + alignment - 1) & ~(alignment - 1);
}

// TODO(primiano): clean this up and move all existing usages to the constexpr
// version above.
template <size_t alignment>
constexpr size_t AlignUp(size_t size) {
  static_assert((alignment & (alignment - 1)) == 0, "alignment must be a pow2");
  return AlignUp(size, alignment);
}

inline bool IsAgain(int err) {
  return err == EAGAIN || err == EWOULDBLOCK;
}

// setenv(2)-equivalent. Deals with Windows vs Posix discrepancies.
void SetEnv(const std::string& key, const std::string& value);

// unsetenv(2)-equivalent. Deals with Windows vs Posix discrepancies.
void UnsetEnv(const std::string& key);

// Calls mallopt(M_PURGE, 0) on Android. Does nothing on other platforms.
// This forces the allocator to release freed memory. This is used to work
// around various Scudo inefficiencies. See b/170217718.
void MaybeReleaseAllocatorMemToOS();

// geteuid() on POSIX OSes, returns 0 on Windows (See comment in utils.cc).
uid_t GetCurrentUserId();

// Forks the process.
// Parent: prints the PID of the child, calls |parent_cb| and exits from the
//         process with its return value.
// Child: redirects stdio onto /dev/null, chdirs into / and returns.
void Daemonize(std::function<int()> parent_cb);

// Returns the path of the current executable, e.g. /foo/bar/exe.
std::string GetCurExecutablePath();

// Returns the directory where the current executable lives in, e.g. /foo/bar.
// This is independent of cwd().
std::string GetCurExecutableDir();

// Memory returned by AlignedAlloc() must be freed via AlignedFree() not just
// free. It makes a difference on Windows where _aligned_malloc() and
// _aligned_free() must be paired.
// Prefer using the AlignedAllocTyped() below which takes care of the pairing.
void* AlignedAlloc(size_t alignment, size_t size);
void AlignedFree(void*);

// Detects Sync-mode MTE (currently being tested in some Android builds).
// This is known to use extra memory for the stack history buffer.
bool IsSyncMemoryTaggingEnabled();

// A RAII version of the above, which takes care of pairing Aligned{Alloc,Free}.
template <typename T>
struct AlignedDeleter {
  inline void operator()(T* ptr) const { AlignedFree(ptr); }
};

// The remove_extent<T> here and below is to allow defining unique_ptr<T[]>.
// As per https://en.cppreference.com/w/cpp/memory/unique_ptr the Deleter takes
// always a T*, not a T[]*.
template <typename T>
using AlignedUniquePtr =
    std::unique_ptr<T, AlignedDeleter<typename std::remove_extent<T>::type>>;

template <typename T>
AlignedUniquePtr<T> AlignedAllocTyped(size_t n_membs) {
  using TU = typename std::remove_extent<T>::type;
  return AlignedUniquePtr<T>(
      static_cast<TU*>(AlignedAlloc(alignof(TU), sizeof(TU) * n_membs)));
}

// A RAII wrapper to invoke a function when leaving a function/scope.
template <typename Func>
class OnScopeExitWrapper {
 public:
  explicit OnScopeExitWrapper(Func f) : f_(std::move(f)), active_(true) {}
  OnScopeExitWrapper(OnScopeExitWrapper&& other) noexcept
      : f_(std::move(other.f_)), active_(other.active_) {
    other.active_ = false;
  }
  ~OnScopeExitWrapper() {
    if (active_)
      f_();
  }

 private:
  Func f_;
  bool active_;
};

template <typename Func>
PERFETTO_WARN_UNUSED_RESULT OnScopeExitWrapper<Func> OnScopeExit(Func f) {
  return OnScopeExitWrapper<Func>(std::move(f));
}

// Returns a xxd-style hex dump (hex + ascii chars) of the input data.
std::string HexDump(const void* data, size_t len, size_t bytes_per_line = 16);
inline std::string HexDump(const std::string& data,
                           size_t bytes_per_line = 16) {
  return HexDump(data.data(), data.size(), bytes_per_line);
}

}  // namespace base
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_EXT_BASE_UTILS_H_
