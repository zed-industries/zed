/*
 * Copyright (C) 2009 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_BLINK_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_BLINK_H_

#include <vector>

#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_string.h"
#include "v8/include/v8-isolate.h"

namespace mojo {
class BinderMap;
}

namespace blink {

namespace scheduler {
class WebThreadScheduler;
}  // namespace scheduler

class Platform;

// Initialize the entire Blink (wtf, platform, core, modules and web).
// If you just need wtf and platform, use Platform::Initialize instead.
//
// Must be called on the thread that will be the main thread before
// using any other public APIs. The provided Platform must be non-null and
// must remain valid until the current thread calls shutdown.
BLINK_EXPORT void Initialize(
    Platform*,
    mojo::BinderMap*,
    scheduler::WebThreadScheduler* main_thread_scheduler);

// The same as above, but this only supports simple single-threaded execution
// environment. The main thread WebThread object is owned by Platform when this
// version is used. This version is mainly for tests and other components
// requiring only the simple environment. This does not create the
// `v8::Isolate`, callers should call `CreateMainThreadIsolate` after calling
// this method.
//
// When this version is used, your Platform implementation needs to follow
// a certain convention on CurrentThread(); see the comments at
// Platform::CreateMainThreadAndInitialize().
BLINK_EXPORT void CreateMainThreadAndInitialize(Platform*, mojo::BinderMap*);

// Performs initialization required for Blink (wtf, core, modules and
// web), but without initializing the main thread isolate. This allows
// for CreateMainThreadIsolate() below to be called.
BLINK_EXPORT void InitializeWithoutIsolateForTesting(
    Platform*,
    mojo::BinderMap*,
    scheduler::WebThreadScheduler* main_thread_scheduler);

// Initializes and returns the Main Thread Isolate. InitializeCommon()
// must be called before this.
BLINK_EXPORT v8::Isolate* CreateMainThreadIsolate();

// Alters the rendering of content to conform to a fixed set of rules.
BLINK_EXPORT void SetWebTestMode(bool);
BLINK_EXPORT bool WebTestMode();

// Alters whether the browser can handle focus events while running web tests.
BLINK_EXPORT void SetBrowserCanHandleFocusForWebTest(bool);

// Alters the rendering of fonts for web tests.
BLINK_EXPORT void SetFontAntialiasingEnabledForTest(bool);
BLINK_EXPORT bool FontAntialiasingEnabledForTest();

// Purge the plugin list cache. This can cause a web-visible and out-of-spec
// change to |navigator.plugins| if the plugin list has changed (see
// https://crbug.com/735854). |reloadPages| is unsupported and must be false.
BLINK_EXPORT void ResetPluginCache(bool reload_pages = false);

// The embedder should call this periodically in an attempt to balance overall
// performance and memory usage.
BLINK_EXPORT void DecommitFreeableMemory();

// Send memory pressure notification to isolates.
// This should be use as last resort only to prevent an OOM. Avoid using this
// as a general way of reducing memory footprint.
BLINK_EXPORT void MemoryPressureNotificationToAllIsolates(
    v8::MemoryPressureLevel);

// Send a request to the all isolates to prioritize energy efficiency
// because the embedder is running in battery saver mode.
BLINK_EXPORT void SetBatterySaverModeForAllIsolates(
    bool battery_saver_mode_enabled);

// Logs stats. Intended to be called during shutdown.
BLINK_EXPORT void LogStatsDuringShutdown();

// Allows disabling domain relaxation.
BLINK_EXPORT void SetDomainRelaxationForbiddenForTest(bool forbidden,
                                                      const WebString& scheme);
// Undos all calls to SetDomainRelaxationForbiddenForTest().
BLINK_EXPORT void ResetDomainRelaxationForTest();

// Force the webgl context to fail so that webglcontextcreationerror
// event gets generated/tested.
BLINK_EXPORT void ForceNextWebGLContextCreationToFailForTest();

// Force the drawing buffer used by webgl contexts to fail so that the webgl
// context's ability to deal with that failure gracefully can be tested.
BLINK_EXPORT void ForceNextDrawingBufferCreationToFailForTest();

// Set whether this renderer process is "cross-origin isolated". This
// corresponds to agent cluster's "cross-origin isolated" concept.
// TODO(yhirano): Have the spec URL.
// This property is process global because we ensure that a renderer process
// host only cross-origin isolated agents or only non-cross-origin isolated
// agents, not both.
// This is called at most once. This is called earlier than any frame commit.
BLINK_EXPORT void SetIsCrossOriginIsolated(bool value);

// Allows disabling web security. One example of this is that it enables APIs
// that would otherwise require cross-origin-isolated contexts.
BLINK_EXPORT void SetIsWebSecurityDisabled(bool value);

// Set whether this renderer process is allowed to use Isolated Context APIs.
// Similarly to the `SetIsCrossOriginIsolated()` method above, this flag is
// process global, and called at most once, prior to committing a frame.
//
// TODO(mkwst): We need a specification for this restriction.
BLINK_EXPORT void SetIsIsolatedContext(bool value);
BLINK_EXPORT bool IsIsolatedContext();

// Set a list of CORS exempt headers. This list is used for fetching resources
// from frames.
BLINK_EXPORT void SetCorsExemptHeaderList(
    const std::vector<WebString>& web_cors_exempt_header_list);

// Notification the process hosting blink is in the foreground/background.
BLINK_EXPORT void OnProcessForegrounded();
BLINK_EXPORT void OnProcessBackgrounded();

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_BLINK_H_
