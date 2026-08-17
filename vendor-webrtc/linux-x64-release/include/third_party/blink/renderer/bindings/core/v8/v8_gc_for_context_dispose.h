/*
 * Copyright (C) 2010 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_GC_FOR_CONTEXT_DISPOSE_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_GC_FOR_CONTEXT_DISPOSE_H_

#include "third_party/blink/renderer/bindings/core/v8/window_proxy.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "v8/include/v8-forward.h"

namespace blink {

// Handler for context disposals. Forwards the notification to V8 but also
// interferes on e.g. Android with performing garbage collections.
class V8GCForContextDispose {
  USING_FAST_MALLOC(V8GCForContextDispose);

 public:
  CORE_EXPORT static V8GCForContextDispose& Instance();

  V8GCForContextDispose(const V8GCForContextDispose&) = delete;
  V8GCForContextDispose& operator=(const V8GCForContextDispose&) = delete;

  // Notification for context disposal.
  void NotifyContextDisposed(v8::Isolate* isolate,
                             bool is_main_frame,
                             WindowProxy::FrameReuseStatus);

  // Called by OomInterventionImpl. If intervention runs on the previous page,
  // it means that the memory usage is high and needs gc during navigation to
  // the next page, so that the memory usage of the two pages would not overlap.
  CORE_EXPORT void SetForcePageNavigationGC();

 private:
  V8GCForContextDispose() = default;  // Use Instance() instead.

  bool force_page_navigation_gc_{false};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_GC_FOR_CONTEXT_DISPOSE_H_
