// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TRACED_LAYOUT_OBJECT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TRACED_LAYOUT_OBJECT_H_

#include <memory>
#include "third_party/blink/renderer/platform/instrumentation/tracing/traced_value.h"

namespace blink {

class LayoutView;

class TracedLayoutObject {
 public:
  static std::unique_ptr<TracedValue> Create(const LayoutView&,
                                             bool trace_geometry = true);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_TRACED_LAYOUT_OBJECT_H_
