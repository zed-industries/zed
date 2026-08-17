//===--- rtsan.h - Realtime Sanitizer ---------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//
//
//===----------------------------------------------------------------------===//

#pragma once

#include "sanitizer_common/sanitizer_internal_defs.h"

extern "C" {

// Initialise rtsan interceptors.
// A call to this method is added to the preinit array on Linux systems.
SANITIZER_INTERFACE_ATTRIBUTE void __rtsan_init();

// Initializes rtsan if it has not been initialized yet.
// Used by the RTSan runtime to ensure that rtsan is initialized before any
// other rtsan functions are called.
SANITIZER_INTERFACE_ATTRIBUTE void __rtsan_ensure_initialized();

SANITIZER_INTERFACE_ATTRIBUTE bool __rtsan_is_initialized();

// Enter real-time context.
// When in a real-time context, RTSan interceptors will error if realtime
// violations are detected. Calls to this method are injected at the code
// generation stage when RTSan is enabled.
SANITIZER_INTERFACE_ATTRIBUTE void __rtsan_realtime_enter();

// Exit the real-time context.
// When not in a real-time context, RTSan interceptors will simply forward
// intercepted method calls to the real methods.
SANITIZER_INTERFACE_ATTRIBUTE void __rtsan_realtime_exit();

// See documentation in rtsan_interface.h.
SANITIZER_INTERFACE_ATTRIBUTE void __rtsan_disable();

// See documentation in rtsan_interface.h.
SANITIZER_INTERFACE_ATTRIBUTE void __rtsan_enable();

SANITIZER_INTERFACE_ATTRIBUTE void
__rtsan_notify_intercepted_call(const char *intercepted_function_name);

SANITIZER_INTERFACE_ATTRIBUTE void
__rtsan_notify_blocking_call(const char *blocking_function_name);
} // extern "C"
