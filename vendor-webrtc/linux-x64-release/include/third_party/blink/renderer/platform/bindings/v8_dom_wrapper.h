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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_V8_DOM_WRAPPER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_V8_DOM_WRAPPER_H_

#include "base/check_op.h"
#include "base/compiler_specific.h"
#include "third_party/blink/renderer/platform/bindings/binding_security_for_platform.h"
#include "third_party/blink/renderer/platform/bindings/dom_data_store.h"
#include "third_party/blink/renderer/platform/bindings/runtime_call_stats.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/bindings/v8_binding.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "v8/include/v8.h"

namespace blink {

struct WrapperTypeInfo;

// Contains utility methods to create wrappers, associate ScriptWrappable
// objects with wrappers and set fields in wrappers.
class V8DOMWrapper {
  STATIC_ONLY(V8DOMWrapper);

 public:
  PLATFORM_EXPORT static v8::Local<v8::Object> CreateWrapper(
      ScriptState*,
      const WrapperTypeInfo*);
  PLATFORM_EXPORT static bool IsWrapper(v8::Isolate*, v8::Local<v8::Object>);

  // Associates the given ScriptWrappable with the given |wrapper| if the
  // ScriptWrappable is not yet associated with any wrapper.  Returns the
  // wrapper already associated or |wrapper| if not yet associated.
  // The caller should always use the returned value rather than |wrapper|.
  [[nodiscard]] PLATFORM_EXPORT static v8::Local<v8::Object>
  AssociateObjectWithWrapper(v8::Isolate*,
                             ScriptWrappable*,
                             const WrapperTypeInfo*,
                             v8::Local<v8::Object> wrapper);
  static void SetNativeInfo(v8::Isolate* isolate,
                            v8::Local<v8::Object> wrapper,
                            ScriptWrappable* script_wrappable);

  static void ClearNativeInfo(v8::Isolate*,
                              v8::Local<v8::Object>,
                              const WrapperTypeInfo*);

  // HasInternalFieldsSet only checks if the value has the internal fields for
  // wrapper object and type, and does not check if it's valid or not. The value
  // may not be a Blink's wrapper object.  In order to make sure of it, use
  // IsWrapper() instead.
  PLATFORM_EXPORT static bool HasInternalFieldsSet(v8::Isolate*,
                                                   v8::Local<v8::Object>);
};

inline void V8DOMWrapper::SetNativeInfo(
    v8::Isolate* isolate,
    v8::Local<v8::Object> wrapper,
    ScriptWrappable* wrappable) {
  DCHECK(wrappable);
  DCHECK(!WrapperTypeInfo::HasLegacyInternalFieldsSet(wrapper));
  v8::Object::Wrap(isolate, wrapper, wrappable,
                   wrappable->GetWrapperTypeInfo()->this_tag);
}

inline void V8DOMWrapper::ClearNativeInfo(
    v8::Isolate* isolate,
    v8::Local<v8::Object> wrapper,
    const WrapperTypeInfo* wrapper_type_info) {
  v8::Object::Wrap(isolate, wrapper, nullptr, wrapper_type_info->this_tag);
}

inline v8::Local<v8::Object> V8DOMWrapper::AssociateObjectWithWrapper(
    v8::Isolate* isolate,
    ScriptWrappable* impl,
    const WrapperTypeInfo* wrapper_type_info,
    v8::Local<v8::Object> wrapper) {
  RUNTIME_CALL_TIMER_SCOPE(
      isolate, RuntimeCallStats::CounterId::kAssociateObjectWithWrapper);
  if (DOMDataStore::SetWrapper(isolate, impl, wrapper_type_info, wrapper)) {
    SetNativeInfo(isolate, wrapper, impl);
    DCHECK(HasInternalFieldsSet(isolate, wrapper));
  }
  SECURITY_CHECK(ToAnyScriptWrappable(isolate, wrapper) == impl);
  return wrapper;
}

class V8WrapperInstantiationScope final {
  STACK_ALLOCATED();

 public:
  V8WrapperInstantiationScope(ScriptState* script_state)
      : context_(script_state->GetIsolate()->GetCurrentContext()) {
    v8::Local<v8::Context> context_for_wrapper = script_state->GetContext();

    // For performance, we enter the context only if the currently running
    // context is different from the context that we are about to enter.
    if (context_for_wrapper == context_) [[likely]] {
      return;
    }

    did_enter_context_ = true;
    context_ = context_for_wrapper;
    context_->Enter();
  }

  ~V8WrapperInstantiationScope() {
    if (!did_enter_context_) [[likely]] {
      return;
    }
    context_->Exit();
  }

  v8::Local<v8::Context> GetContext() const { return context_; }

 private:
  bool did_enter_context_ = false;
  v8::Local<v8::Context> context_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_V8_DOM_WRAPPER_H_
