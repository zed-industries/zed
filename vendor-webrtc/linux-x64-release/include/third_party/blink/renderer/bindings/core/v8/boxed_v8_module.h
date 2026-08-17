// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_BOXED_V8_MODULE_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_BOXED_V8_MODULE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/trace_wrapper_v8_reference.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/hash_functions.h"
#include "v8/include/v8.h"

namespace blink {

// BoxedV8Module wraps a handle to a v8::Module for use on heap.
//
// Hash of Member<BoxedV8Module> overrides HashTraits.
// Therefore, BoxedV8Module can be a key/value type of WTF::HashMap,
// HashSet,HashTable by using BoxedV8ModuleHash.
class CORE_EXPORT BoxedV8Module final : public GarbageCollected<BoxedV8Module> {
 public:
  BoxedV8Module(v8::Isolate* isolate, v8::Local<v8::Module> module)
      : record_(isolate, module),
        identity_hash_(static_cast<unsigned>(module->GetIdentityHash())) {}

  void Trace(Visitor* visitor) const { visitor->Trace(record_); }

  v8::Local<v8::Module> NewLocal(v8::Isolate* isolate) const {
    return record_.Get(isolate);
  }

 private:
  TraceWrapperV8Reference<v8::Module> record_;
  const unsigned identity_hash_;
  friend struct HashTraits<Member<BoxedV8Module>>;
};

}  // namespace blink

namespace WTF {

template <>
struct HashTraits<blink::Member<blink::BoxedV8Module>>
    : MemberHashTraits<blink::BoxedV8Module> {
  static unsigned GetHash(const blink::Member<blink::BoxedV8Module>& key) {
    return key->identity_hash_;
  }

  static bool Equal(const blink::Member<blink::BoxedV8Module>& a,
                    const blink::Member<blink::BoxedV8Module>& b) {
    return a->record_ == b->record_;
  }

  static constexpr bool kSafeToCompareToEmptyOrDeleted = false;
};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_BOXED_V8_MODULE_H_
