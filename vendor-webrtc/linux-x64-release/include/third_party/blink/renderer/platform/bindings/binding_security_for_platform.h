// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_BINDING_SECURITY_FOR_PLATFORM_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_BINDING_SECURITY_FOR_PLATFORM_H_

#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "v8/include/v8.h"

namespace blink {

// BindingSecurityForPlatform provides utility functions that determine access
// permission between two realms.
//
// This class is a collection of trampolines to the actual implementations in
// BindingSecurity in core/ component.
class PLATFORM_EXPORT BindingSecurityForPlatform {
  STATIC_ONLY(BindingSecurityForPlatform);

 public:
  // This should be used only when checking a general access from one context to
  // another context. For access to a receiver object or returned object, you
  // should use BindingSecurity::ShouldAllowAccessTo family.
  static bool ShouldAllowAccessToV8Context(
      v8::Local<v8::Context> accessing_context,
      v8::MaybeLocal<v8::Context> target_context);

 private:
  using ShouldAllowAccessToV8ContextFunction =
      bool (*)(v8::Local<v8::Context> accessing_context,
               v8::MaybeLocal<v8::Context> target_context);
  static void SetShouldAllowAccessToV8Context(
      ShouldAllowAccessToV8ContextFunction);

  static ShouldAllowAccessToV8ContextFunction should_allow_access_to_v8context_;

  friend class BindingSecurity;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_BINDING_SECURITY_FOR_PLATFORM_H_
