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

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_BINDING_SECURITY_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_BINDING_SECURITY_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/binding_security_for_platform.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "v8/include/v8.h"

namespace blink {

class DOMWindow;
class LocalDOMWindow;
class Location;
class Node;
class ScriptState;

// BindingSecurity provides utility functions that determine access permission
// between two realms. For example, is the current Window allowed to access the
// target window?
class CORE_EXPORT BindingSecurity {
  STATIC_ONLY(BindingSecurity);

 public:
  static void Init();

  // Checks if the caller (|accessing_window|) is allowed to access the JS
  // receiver object (|target|), where the receiver object is the JS object
  // for which the DOM attribute or DOM operation is being invoked (in the
  // form of receiver.domAttr or receiver.domOp()).
  // Note that only Window and Location objects are cross-origin accessible, so
  // the receiver object must be of type DOMWindow or Location.
  //
  // DOMWindow
  static bool ShouldAllowAccessTo(const LocalDOMWindow* accessing_window,
                                  const DOMWindow* target);

  // Location
  static bool ShouldAllowAccessTo(const LocalDOMWindow* accessing_window,
                                  const Location* target);

  // Checks if the caller (|accessing_window|) is allowed to access the JS
  // returned object (|target|), where the returned object is the JS object
  // which is returned as a result of invoking a DOM attribute or DOM
  // operation (in the form of
  //   var x = receiver.domAttr // or receiver.domOp()
  // where |x| is the returned object).
  // See window.frameElement for example, which may return a frame object.
  // The object returned from window.frameElement must be the same origin if
  // it's not null.
  //
  // Node
  static bool ShouldAllowAccessTo(const LocalDOMWindow* accessing_window,
                                  const Node* target);

  // This function should be used only when checking a general access from
  // one context to another context. For access to a receiver object or
  // returned object, you should use the above overloads.
  static bool ShouldAllowAccessToV8Context(
      v8::Local<v8::Context> accessing_context,
      v8::MaybeLocal<v8::Context> target_context);

  static bool ShouldAllowAccessToV8Context(ScriptState* accessing_script_state,
                                           ScriptState* target_script_state) {
    DCHECK(accessing_script_state);

    // Fast path for the most likely case.
    if (accessing_script_state == target_script_state) [[likely]] {
      return true;
    }
    return ShouldAllowAccessToV8ContextInternal(accessing_script_state,
                                                target_script_state);
  }

  static void FailedAccessCheckFor(v8::Local<v8::Object> holder);

 private:
  static bool ShouldAllowAccessToV8ContextInternal(
      ScriptState* accessing_script_state,
      ScriptState* target_script_state);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_BINDING_SECURITY_H_
