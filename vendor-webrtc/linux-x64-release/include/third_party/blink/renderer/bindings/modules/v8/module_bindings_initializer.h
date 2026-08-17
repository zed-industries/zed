// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_MODULES_V8_MODULE_BINDINGS_INITIALIZER_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_MODULES_V8_MODULE_BINDINGS_INITIALIZER_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class ModuleBindingsInitializer {
  STATIC_ONLY(ModuleBindingsInitializer);

 public:
  static void Init();
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_MODULES_V8_MODULE_BINDINGS_INITIALIZER_H_
