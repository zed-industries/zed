// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_WIN_SCOPED_HANDLE_H_
#define BASE_WIN_SCOPED_HANDLE_H_

#include <ostream>
#include <string_view>

#include "base/base_export.h"
#include "base/check_op.h"
#include "base/dcheck_is_on.h"
#include "base/gtest_prod_util.h"
#include "base/location.h"
#include "base/types/expected.h"
#include "base/win/windows_handle_util.h"
#include "base/win/windows_types.h"
#include "build/build_config.h"

// TODO(rvargas): remove this with the rest of the verifier.
#if defined(COMPILER_MSVC)
#include <intrin.h>
#define BASE_WIN_GET_CALLER _ReturnAddress()
#elif defined(COMPILER_GCC)
#define BASE_WIN_GET_CALLER \
  __builtin_extract_return_addr(__builtin_return_address(0))
#endif

namespace base {
namespace win {

enum class HandleOperation {
  kHandleAlreadyTracked,
  kCloseHandleNotTracked,
  kCloseHandleNotOwner,
  kCloseHandleHook,
  kDuplicateHandleHook
};

std::ostream& operator<<(std::ostream& os, HandleOperation operation);

// Generic wrapper for raw handles that takes care of closing handles
// automatically. The class interface follows the style of
// the ScopedFILE class with two additions:
//   - IsValid() method can tolerate multiple invalid handle values such as NULL
//     and INVALID_HANDLE_VALUE (-1) for Win32 handles.
//   - Set() (and the constructors and assignment operators that call it)
//     preserve the Windows LastError code. This ensures that GetLastError() can
//     be called after stashing a handle in a GenericScopedHandle object. Doing
//     this explicitly is necessary because of bug 528394 and VC++ 2015.
template <class Traits, class Verifier>
class GenericScopedHandle {
 public:
  using Handle = typename Traits::Handle;

  GenericScopedHandle() : handle_(Traits::NullHandle()) {}

  explicit GenericScopedHandle(Handle handle) : handle_(Traits::NullHandle()) {
    Set(handle);
  }

  GenericScopedHandle(GenericScopedHandle&& other)
      : handle_(Traits::NullHandle()) {
    Set(other.Take());
  }

  GenericScopedHandle(const GenericScopedHandle&) = delete;
  GenericScopedHandle& operator=(const GenericScopedHandle&) = delete;

  ~GenericScopedHandle() { Close(); }

  bool is_valid() const { return Traits::IsHandleValid(handle_); }

  // TODO(crbug.com/40212898): Migrate callers to is_valid().
  bool IsValid() const { return is_valid(); }

  GenericScopedHandle& operator=(GenericScopedHandle&& other) {
    DCHECK_NE(this, &other);
    Set(other.Take());
    return *this;
  }

  void Set(Handle handle) {
    if (handle_ != handle) {
      // Preserve old LastError to avoid bug 528394.
      auto last_error = ::GetLastError();
      Close();

      if (Traits::IsHandleValid(handle)) {
        handle_ = handle;
        Verifier::StartTracking(handle, this, BASE_WIN_GET_CALLER,
                                GetProgramCounter());
      }
      ::SetLastError(last_error);
    }
  }

  Handle get() const { return handle_; }

  // TODO(crbug.com/40212898): Migrate callers to get().
  Handle Get() const { return get(); }

  // Transfers ownership away from this object.
  [[nodiscard]] Handle release() {
    Handle temp = handle_;
    handle_ = Traits::NullHandle();
    if (Traits::IsHandleValid(temp)) {
      Verifier::StopTracking(temp, this, BASE_WIN_GET_CALLER,
                             GetProgramCounter());
    }
    return temp;
  }

  // TODO(crbug.com/40212898): Migrate callers to release().
  [[nodiscard]] Handle Take() { return release(); }

  // Explicitly closes the owned handle.
  void Close() {
    if (Traits::IsHandleValid(handle_)) {
      Verifier::StopTracking(handle_, this, BASE_WIN_GET_CALLER,
                             GetProgramCounter());

      Traits::CloseHandle(handle_);
      handle_ = Traits::NullHandle();
    }
  }

 private:
  FRIEND_TEST_ALL_PREFIXES(ScopedHandleDeathTest, HandleVerifierWrongOwner);
  FRIEND_TEST_ALL_PREFIXES(ScopedHandleDeathTest,
                           HandleVerifierUntrackedHandle);
  Handle handle_;
};

#undef BASE_WIN_GET_CALLER

// The traits class for Win32 handles that can be closed via CloseHandle() API.
class HandleTraits {
 public:
  using Handle = HANDLE;

  HandleTraits() = delete;
  HandleTraits(const HandleTraits&) = delete;
  HandleTraits& operator=(const HandleTraits&) = delete;

  // Closes the handle.
  static bool BASE_EXPORT CloseHandle(HANDLE handle);

  // Returns true if `handle` is neither null nor a pseudo handle like
  // GetCurrentProcess() or INVALID_HANDLE_VALUE. This means it has a value in
  // the range used for real handle values. It is still possible for `handle` to
  // not be associated with an actual open handle.
  // Note: Use TakeHandleOfType() to adopt a handle of a particular type with
  // additional validation.
  static bool IsHandleValid(HANDLE handle) {
    return handle != nullptr && !base::win::IsPseudoHandle(handle);
  }

  // Returns NULL handle value.
  static HANDLE NullHandle() { return nullptr; }
};

// Do-nothing verifier.
class DummyVerifierTraits {
 public:
  using Handle = HANDLE;

  DummyVerifierTraits() = delete;
  DummyVerifierTraits(const DummyVerifierTraits&) = delete;
  DummyVerifierTraits& operator=(const DummyVerifierTraits&) = delete;

  static void StartTracking(HANDLE handle,
                            const void* owner,
                            const void* pc1,
                            const void* pc2) {}
  static void StopTracking(HANDLE handle,
                           const void* owner,
                           const void* pc1,
                           const void* pc2) {}
};

// Performs actual run-time tracking.
class BASE_EXPORT VerifierTraits {
 public:
  using Handle = HANDLE;

  VerifierTraits() = delete;
  VerifierTraits(const VerifierTraits&) = delete;
  VerifierTraits& operator=(const VerifierTraits&) = delete;

  static void StartTracking(HANDLE handle,
                            const void* owner,
                            const void* pc1,
                            const void* pc2);
  static void StopTracking(HANDLE handle,
                           const void* owner,
                           const void* pc1,
                           const void* pc2);
};

using UncheckedScopedHandle =
    GenericScopedHandle<HandleTraits, DummyVerifierTraits>;
using CheckedScopedHandle = GenericScopedHandle<HandleTraits, VerifierTraits>;

#if DCHECK_IS_ON()
using ScopedHandle = CheckedScopedHandle;
#else
using ScopedHandle = UncheckedScopedHandle;
#endif

// This function may be called by the embedder to disable the use of
// VerifierTraits at runtime. It has no effect if DummyVerifierTraits is used
// for ScopedHandle.
BASE_EXPORT void DisableHandleVerifier();

// This should be called whenever the OS is closing a handle, if extended
// verification of improper handle closing is desired. If |handle| is being
// tracked by the handle verifier and ScopedHandle is not the one closing it,
// a CHECK is generated.
BASE_EXPORT void OnHandleBeingClosed(HANDLE handle, HandleOperation operation);

// Returns a smart pointer wrapping `handle` if it references an object of type
// `object_type_name`. Crashes the process if `handle` is valid but of an
// unexpected type. This function will fail with STATUS_INVALID_HANDLE if called
// with the pseudo handle returned by `::GetCurrentProcess()` or
// `GetCurrentProcessHandle()`.
BASE_EXPORT expected<ScopedHandle, NTSTATUS> TakeHandleOfType(
    HANDLE handle,
    std::wstring_view object_type_name);

}  // namespace win
}  // namespace base

#endif  // BASE_WIN_SCOPED_HANDLE_H_
