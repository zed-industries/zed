// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_CUSTOM_CSS_LAYOUT_WORKLET_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_CUSTOM_CSS_LAYOUT_WORKLET_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class ScriptState;
class Worklet;

class CORE_EXPORT CSSLayoutWorklet {
  STATIC_ONLY(CSSLayoutWorklet);

 public:
  static Worklet* layoutWorklet(ScriptState*);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_CUSTOM_CSS_LAYOUT_WORKLET_H_
