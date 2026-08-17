// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_EMBEDDER_GRAPH_BUILDER_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_EMBEDDER_GRAPH_BUILDER_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "v8/include/v8-profiler.h"
#include "v8/include/v8.h"

namespace blink {

class EmbedderGraphBuilder {
  STATIC_ONLY(EmbedderGraphBuilder);

 public:
  static void BuildEmbedderGraphCallback(v8::Isolate*,
                                         v8::EmbedderGraph*,
                                         void*);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_EMBEDDER_GRAPH_BUILDER_H_
