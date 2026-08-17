// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_INTERNALS_PROFILER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_INTERNALS_PROFILER_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class Internals;
class ScriptState;

class InternalsProfiler {
  STATIC_ONLY(InternalsProfiler);

 public:
  static void collectSample(ScriptState*, Internals&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_INTERNALS_PROFILER_H_
