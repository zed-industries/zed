// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_WIN_COM_INIT_CHECK_HOOK_H_
#define BASE_WIN_COM_INIT_CHECK_HOOK_H_

#include "base/base_export.h"
#include "base/check_op.h"
#include "build/build_config.h"

namespace device {
class XrDeviceService;
}  // namespace device

namespace base {
namespace win {

// Hotpatching is only supported in Intel 32-bit x86 processors because Windows
// binaries contain a convenient 2 byte hotpatch noop. This doesn't exist in
// 64-bit binaries.

#if DCHECK_IS_ON() && defined(ARCH_CPU_X86_FAMILY) &&        \
    defined(ARCH_CPU_32_BITS) && !defined(OFFICIAL_BUILD) && \
    !defined(COM_INIT_CHECK_HOOK_DISABLED)  // See crbug/737090 for details.
#define COM_INIT_CHECK_HOOK_ENABLED
#endif

// Manages the installation of consistency DCHECK hooks of COM APIs that require
// COM to be initialized and only works if COM_INIT_CHECK_HOOK_ENABLED is
// defined. Care should be taken if this is instantiated with multiple threads
// running as the hotpatch does not apply atomically.
class BASE_EXPORT ComInitCheckHook {
 public:
  ComInitCheckHook();

  ComInitCheckHook(const ComInitCheckHook&) = delete;
  ComInitCheckHook& operator=(const ComInitCheckHook&) = delete;

  ~ComInitCheckHook();

 private:
  // For components that cannot use COM_INIT_CHECK_HOOK_DISABLED, call
  // DisableCOMChecksForProcess() below. This should only be for code that calls
  // into Windows components that don't explicitly initialize the MTA in the
  // Windows thread pool.
  friend class device::XrDeviceService;

  static void DisableCOMChecksForProcess();
};

}  // namespace win
}  // namespace base

#endif  // BASE_WIN_COM_INIT_CHECK_HOOK_H_
