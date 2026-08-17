// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CSSPAINT_PAINT_WORKLET_PENDING_GENERATOR_REGISTRY_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CSSPAINT_PAINT_WORKLET_PENDING_GENERATOR_REGISTRY_H_

#include "third_party/blink/renderer/modules/csspaint/css_paint_image_generator_impl.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/string_hash.h"

namespace blink {

class CSSPaintDefinition;

// Keeps pending CSSPaintImageGeneratorImpls until corresponding
// CSSPaintDefinitions are registered. This is primarily owned by the
// PaintWorklet instance.
class PaintWorkletPendingGeneratorRegistry
    : public GarbageCollected<PaintWorkletPendingGeneratorRegistry> {
 public:
  PaintWorkletPendingGeneratorRegistry() = default;

  PaintWorkletPendingGeneratorRegistry(
      const PaintWorkletPendingGeneratorRegistry&) = delete;
  PaintWorkletPendingGeneratorRegistry& operator=(
      const PaintWorkletPendingGeneratorRegistry&) = delete;

  void NotifyGeneratorReady(const String& name);
  void AddPendingGenerator(const String& name, CSSPaintImageGeneratorImpl*);

  void Trace(Visitor*) const;

 private:
  // The map of CSSPaintImageGeneratorImpl which are waiting for a
  // CSSPaintDefinition to be registered. Owners of this registry is expected to
  // outlive the generators hence are held onto with a WeakMember.
  using GeneratorHashSet =
      GCedHeapHashSet<WeakMember<CSSPaintImageGeneratorImpl>>;
  using PendingGeneratorMap = HeapHashMap<String, Member<GeneratorHashSet>>;
  PendingGeneratorMap pending_generators_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CSSPAINT_PAINT_WORKLET_PENDING_GENERATOR_REGISTRY_H_
