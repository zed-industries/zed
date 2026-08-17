// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_THREAD_LOCAL_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_THREAD_LOCAL_H_

#include "base/compiler_specific.h"
#include "build/build_config.h"
#include "third_party/blink/renderer/platform/heap/heap_buildflags.h"
#include "third_party/blink/renderer/platform/platform_export.h"

// On component builds, always hide the thread_local variable behind a call.
// This avoids complexity with "global-dyn" and allows to use "local-dyn"
// instead, across all platforms. On non-component (release) builds, don't hide
// the variable behind the call (to improve performance in access time), but use
// different tls models on different platforms. On Windows, since chrome is
// linked into the chrome.dll which is always linked to chrome.exe at static
// link time (DT_NEEDED in ELF terms), use "init-exec". On Android, since the
// library can be opened with "dlopen" (through JNI), use "local-dyn". On other
// systems (Linux/ChromeOS/MacOS) use the fastest "local-exec".

//         |_____component_____|___non-component___|
// ________|_tls_model__|_hide_|_tls_model__|_hide_|
// Windows | local-dyn  | yes  | init-exec  |  no  |
// Android | local-dyn  | yes  | local-dyn  |  no  |
// Other   | local-dyn  | yes  | local-exec |  no  |

// The call is still cheaper than multiple calls through WTF/base/pthread*
// layers.
#if BUILDFLAG(BLINK_HEAP_INSIDE_SHARED_LIBRARY)
#define BLINK_HEAP_HIDE_THREAD_LOCAL_IN_LIBRARY 1
#else
#define BLINK_HEAP_HIDE_THREAD_LOCAL_IN_LIBRARY 0
#endif

#if BLINK_HEAP_HIDE_THREAD_LOCAL_IN_LIBRARY
#define BLINK_HEAP_THREAD_LOCAL_MODEL "local-dynamic"
#else
#if BUILDFLAG(IS_WIN)
#define BLINK_HEAP_THREAD_LOCAL_MODEL "initial-exec"
#elif BUILDFLAG(IS_ANDROID)
#define BLINK_HEAP_THREAD_LOCAL_MODEL "local-dynamic"
#else
#define BLINK_HEAP_THREAD_LOCAL_MODEL "local-exec"
#endif
#endif

#if defined(BLINK_HEAP_HIDE_THREAD_LOCAL_IN_LIBRARY)

#define BLINK_HEAP_DECLARE_THREAD_LOCAL_GETTER(Name, Type, Member) \
  NOINLINE static Type Name();
#define BLINK_HEAP_DEFINE_THREAD_LOCAL_GETTER(Name, Type, Member) \
  NOINLINE Type Name() {                                          \
    return Member;                                                \
  }

#else  // !defined(BLINK_HEAP_HIDE_THREAD_LOCAL_IN_LIBRARY)

#define BLINK_HEAP_DECLARE_THREAD_LOCAL_GETTER(Name, Type, Member) \
  ALWAYS_INLINE static Type Name() {                               \
    return Member;                                                 \
  }
#define BLINK_HEAP_DEFINE_THREAD_LOCAL_GETTER(Name, Type, Member)

#endif  // defined(BLINK_HEAP_HIDE_THREAD_LOCAL_IN_LIBRARY)

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_THREAD_LOCAL_H_
