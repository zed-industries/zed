// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifdef UNSAFE_BUFFERS_BUILD
// TODO(crbug.com/40284755): Remove this and spanify to fix the errors.
#pragma allow_unsafe_buffers
#endif

#ifndef BASE_WIN_SCOPED_HANDLE_VERIFIER_H_
#define BASE_WIN_SCOPED_HANDLE_VERIFIER_H_

#include <memory>
#include <unordered_map>

#include "base/base_export.h"
#include "base/debug/stack_trace.h"
#include "base/hash/hash.h"
#include "base/memory/raw_ptr.h"
#include "base/synchronization/lock_impl.h"
#include "base/win/windows_types.h"

namespace base {
namespace win {
enum class HandleOperation;
namespace internal {

struct HandleHash {
  size_t operator()(const HANDLE& handle) const {
    return base::FastHash(byte_span_from_ref(handle));
  }
};

struct ScopedHandleVerifierInfo {
  ScopedHandleVerifierInfo(const void* owner,
                           const void* pc1,
                           const void* pc2,
                           std::unique_ptr<debug::StackTrace> stack,
                           DWORD thread_id);
  ~ScopedHandleVerifierInfo();

  ScopedHandleVerifierInfo(const ScopedHandleVerifierInfo&) = delete;
  ScopedHandleVerifierInfo& operator=(const ScopedHandleVerifierInfo&) = delete;
  ScopedHandleVerifierInfo(ScopedHandleVerifierInfo&&) noexcept;
  ScopedHandleVerifierInfo& operator=(ScopedHandleVerifierInfo&&) noexcept;

  raw_ptr<const void> owner;
  raw_ptr<const void> pc1;
  raw_ptr<const void> pc2;
  std::unique_ptr<debug::StackTrace> stack;
  DWORD thread_id;
};

// Implements the actual object that is verifying handles for this process.
// The active instance is shared across the module boundary but there is no
// way to delete this object from the wrong side of it (or any side, actually).
// We need [[clang::lto_visibility_public]] because instances of this class are
// passed across module boundaries. This means different modules must have
// compatible definitions of the class even when whole program optimization is
// enabled - which is what this attribute accomplishes. The pragma stops MSVC
// from emitting an unrecognized attribute warning.
#pragma warning(push)
#pragma warning(disable : 5030)
class [[clang::lto_visibility_public, nodiscard]] ScopedHandleVerifier {
#pragma warning(pop)
 public:
  ScopedHandleVerifier(const ScopedHandleVerifier&) = delete;
  ScopedHandleVerifier& operator=(const ScopedHandleVerifier&) = delete;

  // Retrieves the current verifier.
  static ScopedHandleVerifier* Get();

  // The methods required by HandleTraits. They are virtual because we need to
  // forward the call execution to another module, instead of letting the
  // compiler call the version that is linked in the current module.
  virtual bool CloseHandle(HANDLE handle);
  virtual void StartTracking(HANDLE handle,
                             const void* owner,
                             const void* pc1,
                             const void* pc2);
  virtual void StopTracking(HANDLE handle,
                            const void* owner,
                            const void* pc1,
                            const void* pc2);
  virtual void Disable();
  virtual void OnHandleBeingClosed(HANDLE handle, HandleOperation operation);
  virtual HMODULE GetModule() const;

 private:
  explicit ScopedHandleVerifier(bool enabled);
  ~ScopedHandleVerifier();  // Not implemented.

  void StartTrackingImpl(HANDLE handle,
                         const void* owner,
                         const void* pc1,
                         const void* pc2);
  void StopTrackingImpl(HANDLE handle,
                        const void* owner,
                        const void* pc1,
                        const void* pc2);
  void OnHandleBeingClosedImpl(HANDLE handle, HandleOperation operation);

  static base::internal::LockImpl* GetLock();
  static void InstallVerifier();
  static void ThreadSafeAssignOrCreateScopedHandleVerifier(
      ScopedHandleVerifier* existing_verifier,
      bool enabled);

  base::debug::StackTrace creation_stack_;
  bool enabled_;
  raw_ptr<base::internal::LockImpl> lock_;
  std::unordered_map<HANDLE, ScopedHandleVerifierInfo, HandleHash> map_;
};

// This testing function returns the module that the HandleVerifier concrete
// implementation was instantiated in.
BASE_EXPORT HMODULE GetHandleVerifierModuleForTesting();

}  // namespace internal
}  // namespace win
}  // namespace base

#endif  // BASE_WIN_SCOPED_HANDLE_VERIFIER_H_
