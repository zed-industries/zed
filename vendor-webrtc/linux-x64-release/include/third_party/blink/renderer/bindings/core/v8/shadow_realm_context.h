// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SHADOW_REALM_CONTEXT_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SHADOW_REALM_CONTEXT_H_

#include "v8/include/v8-forward.h"

namespace blink {

// The callback function to be registered to a v8::Isolate via
// v8::Isolate::SetHostCreateShadowRealmContextCallback. This callback function
// is called at each time when JS code runs "new ShadowRealm()".
// `initiator_context` is the v8::Context which runs "new ShadowRealm()".
v8::MaybeLocal<v8::Context> OnCreateShadowRealmV8Context(
    v8::Local<v8::Context> initiator_context);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SHADOW_REALM_CONTEXT_H_
