// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_COMPOSITING_LAYERS_AS_JSON_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_COMPOSITING_LAYERS_AS_JSON_H_

#include <memory>

#include "base/memory/stack_allocated.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace cc {
class Layer;
}

namespace blink {

class ContentLayerClientImpl;
class JSONArray;
class JSONObject;
class TransformPaintPropertyNode;

// These values need to be kept consistent with the layer tree flags in
// core/testing/Internals.idl.
enum {
  kLayerTreeNormal = 0,
  // Dump extra debugging info like layer addresses.
  kLayerTreeIncludesDebugInfo = 1 << 0,
  kLayerTreeIncludesInvalidations = 1 << 1,
  kLayerTreeIncludesDetailedInvalidations = 1 << 2,
  kLayerTreeIncludesPaintingPhases = 1 << 3,
  kLayerTreeIncludesAllLayers = 1 << 4,
  kLayerTreeIncludesCompositingReasons = 1 << 5,
  kLayerTreeIncludesPaintRecords = 1 << 6,
  // Outputs all layers as a layer tree. The default is output children
  // (excluding the root) as a layer list, in paint (preorder) order.
  kOutputAsLayerTree = 0x4000,
};
typedef unsigned LayerTreeFlags;

class PLATFORM_EXPORT LayersAsJSON {
  STACK_ALLOCATED();

 public:
  LayersAsJSON(LayerTreeFlags);

  void AddLayer(const cc::Layer& layer,
                const TransformPaintPropertyNode& transform,
                const ContentLayerClientImpl* content_layer_client);

  std::unique_ptr<JSONObject> Finalize();

 private:
  int AddTransformJSON(const TransformPaintPropertyNode&);

  LayerTreeFlags flags_;
  int next_transform_id_;
  std::unique_ptr<JSONArray> layers_json_;
  HeapHashMap<Member<const TransformPaintPropertyNode>, int> transform_id_map_;
  std::unique_ptr<JSONArray> transforms_json_;
  HashMap<int, int> rendering_context_map_;
};

PLATFORM_EXPORT std::unique_ptr<JSONObject> CCLayerAsJSON(
    const cc::Layer& layer,
    LayerTreeFlags flags);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_COMPOSITING_LAYERS_AS_JSON_H_
