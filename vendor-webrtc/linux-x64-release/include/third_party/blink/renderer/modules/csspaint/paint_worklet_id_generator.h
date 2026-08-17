// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CSSPAINT_PAINT_WORKLET_ID_GENERATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CSSPAINT_PAINT_WORKLET_ID_GENERATOR_H_

#include "third_party/blink/renderer/modules/modules_export.h"

namespace blink {

class MODULES_EXPORT PaintWorkletIdGenerator {
 public:
  static int NextId();
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CSSPAINT_PAINT_WORKLET_ID_GENERATOR_H_
