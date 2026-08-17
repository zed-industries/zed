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

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_GC_CONTROLLER_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_GC_CONTROLLER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "v8/include/v8-profiler.h"
#include "v8/include/v8.h"

namespace blink {

class Node;

class CORE_EXPORT V8GCController {
  STATIC_ONLY(V8GCController);

 public:
  // Information about whether a wrapper is attached to the main DOM tree
  // or not. It is computed as follows:
  // 1) A ExecutionContext with IsContextDestroyed() = true is detached.
  // 2) A ExecutionContext with IsContextDestroyed() = false is attached.
  // 3) A Node that is reachable from a detached ExecutionContext is detached.
  // 4) A Node that is reachable from an attached ExecutionContext is attached.
  // 5) Any non-Node wrappers return unknown.
  static v8::EmbedderGraph::Node::Detachedness DetachednessFromWrapper(
      v8::Isolate*,
      const v8::Local<v8::Value>&,
      uint16_t,
      void*);

  // Prologue and epilogue callbacks for V8 garbage collections.
  static void GcPrologue(v8::Isolate*, v8::GCType, v8::GCCallbackFlags);
  static void GcEpilogue(v8::Isolate*, v8::GCType, v8::GCCallbackFlags);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_GC_CONTROLLER_H_
