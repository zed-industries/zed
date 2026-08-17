// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CANVAS_CANVAS_RESOURCE_TRACKER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CANVAS_CANVAS_RESOURCE_TRACKER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/v8_per_isolate_data.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace v8 {
class Isolate;
}

namespace blink {

class CanvasRenderingContextHost;
class ExecutionContext;

class CORE_EXPORT CanvasResourceTracker final
    : public V8PerIsolateData::UserData {
 public:
  using ResourceMap = HeapHashMap<WeakMember<CanvasRenderingContextHost>,
                                  WeakMember<ExecutionContext>>;

  static CanvasResourceTracker* For(v8::Isolate*);

  void Add(CanvasRenderingContextHost*, ExecutionContext*);
  const ResourceMap& GetResourceMap() const;

  void Trace(Visitor*) const override;

 private:
  ResourceMap resource_map_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CANVAS_CANVAS_RESOURCE_TRACKER_H_
