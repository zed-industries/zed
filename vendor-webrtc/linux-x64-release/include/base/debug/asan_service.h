// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_DEBUG_ASAN_SERVICE_H_
#define BASE_DEBUG_ASAN_SERVICE_H_

#if defined(ADDRESS_SANITIZER)

#include <string>
#include <vector>

#include "base/base_export.h"
#include "base/no_destructor.h"
#include "base/synchronization/lock.h"
#include "base/thread_annotations.h"

namespace base {
namespace debug {

// This implements an abstraction layer for the parts of the AddressSanitizer
// API used to receive callbacks during crash handling. This is used to add
// application-specific information into the AddressSanitizer error messages
// to assist with debugging, and to filter known false-positive crashes during
// fuzz testing.
class BASE_EXPORT AsanService {
 public:
  // We can't use a base::Callback type here as we need execution of these
  // callbacks to be as simple as possible.
  //
  // `reason` points to a string containing the AddressSanitizer error report.
  // `should_exit_cleanly` should be set to true only if the callback determines
  // that this crash is known to be safe - this will override the normal ASan
  // behaviour and instead exit cleanly. If your callback is modifying this
  // parameter, it should log a message explaining why this error is known to
  // be safe.
  using ErrorCallback = void (*)(const char* reason, bool* should_exit_cleanly);

  static AsanService* GetInstance();

  // Registers the global AddressSanitizer error report callback. Any callbacks
  // registered by calls to AddErrorCallback will become active after this is
  // complete. Safe to call from any thread, and safe to call multiple times.
  void Initialize() LOCKS_EXCLUDED(lock_);

  // Writes a message to the same log as AddressSanitizer. This should be used
  // for logging inside callbacks. Safe to call from any thread.
  void Log(const char* format, ...);

  // Adds an error callback that will be called on the faulting thread when
  // Address Sanitizer detects an error. All registered callbacks are called
  // for every error. Safe to call from any thread, and the callback registered
  // must also be safe to call from any thread.
  void AddErrorCallback(ErrorCallback error_callback) LOCKS_EXCLUDED(lock_);

 private:
  friend class AsanServiceTest;
  friend class base::NoDestructor<AsanService>;

  AsanService();
  ~AsanService() = delete;

  void RunErrorCallbacks(const char* reason) LOCKS_EXCLUDED(lock_);

  // This is the error report entrypoint function that is registered with
  // AddressSanitizer.
  static void ErrorReportCallback(const char* reason);

  // Guards all of the internal state, so that we can safely handle concurrent
  // errors on multiple threads.
  Lock lock_;

  // Ensure that we don't try and register callbacks before calling Initialize.
  bool is_initialized_ GUARDED_BY(lock_) = false;

  // The list of currently registered error callbacks.
  std::vector<ErrorCallback> error_callbacks_ GUARDED_BY(lock_);
};

}  // namespace debug
}  // namespace base

#endif  // defined(ADDRESS_SANITIZER)
#endif  // BASE_DEBUG_ASAN_SERVICE_H_
