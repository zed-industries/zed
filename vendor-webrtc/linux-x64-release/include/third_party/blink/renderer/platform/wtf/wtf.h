/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_WTF_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_WTF_H_

#include "base/threading/platform_thread.h"
#include "build/build_config.h"
#include "third_party/blink/renderer/platform/wtf/wtf_export.h"

namespace WTF {

WTF_EXPORT extern base::PlatformThreadId g_main_thread_identifier;

// This function must be called exactly once from the main thread before using
// anything else in WTF.
WTF_EXPORT void Initialize();

// thread_local variables can't be exported on Windows, so we use an extra
// function call on component builds. Also, thread_local on Android is emulated
// by the runtime lib; gettid(3) in bionic already caches tid in a TLS variable.
#if BUILDFLAG(IS_ANDROID) || (defined(COMPONENT_BUILD) && BUILDFLAG(IS_WIN))
WTF_EXPORT bool IsMainThread();
#else
WTF_EXPORT constinit extern thread_local bool g_is_main_thread;
inline bool IsMainThread() {
  return g_is_main_thread;
}
#endif

}  // namespace WTF

using WTF::IsMainThread;

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_WTF_H_
