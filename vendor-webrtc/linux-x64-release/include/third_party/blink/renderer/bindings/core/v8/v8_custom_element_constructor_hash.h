// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_CUSTOM_ELEMENT_CONSTRUCTOR_HASH_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_CUSTOM_ELEMENT_CONSTRUCTOR_HASH_H_

#include "third_party/blink/renderer/bindings/core/v8/v8_custom_element_constructor.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

// Hashes V8CustomElementConstructor pointers by their v8 callback objects.
struct V8CustomElementConstructorHashTraits
    : WTF::MemberHashTraits<V8CustomElementConstructor> {
  static unsigned GetHash(
      const Member<V8CustomElementConstructor>& constructor) {
    return constructor->CallbackObject()->GetIdentityHash();
  }
  static bool Equal(const Member<V8CustomElementConstructor>& a,
                    const Member<V8CustomElementConstructor>& b) {
    if (a->GetIsolate() != b->GetIsolate()) {
      return false;
    }
    return a->CallbackObject() == b->CallbackObject();
  }
  static constexpr bool kSafeToCompareToEmptyOrDeleted = false;
};

// Hash translator for custom element constructors in two forms:
// V8CustomElementConstructor or v8 object.
struct V8CustomElementConstructorHashTranslator {
  STATIC_ONLY(V8CustomElementConstructorHashTranslator);
  static unsigned GetHash(const v8::Local<v8::Object>& constructor) {
    return constructor->GetIdentityHash();
  }
  static bool Equal(const Member<V8CustomElementConstructor>& a,
                    const v8::Local<v8::Object>& b) {
    return a && a->CallbackObject() == b;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_CUSTOM_ELEMENT_CONSTRUCTOR_HASH_H_
