// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_LAYER_TREE_HOST_EMBEDDER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_LAYER_TREE_HOST_EMBEDDER_H_

#include "cc/animation/animation_host.h"
#include "cc/test/stub_layer_tree_host_client.h"
#include "cc/test/stub_layer_tree_host_single_thread_client.h"
#include "cc/test/test_task_graph_runner.h"
#include "cc/trees/layer_tree_host.h"
#include "cc/trees/layer_tree_settings.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// A class that owns the lifetime of a cc::LayerTreeHost and its dependencies
// for unit tests that need to instantiate only a cc::LayerTreeHost and not the
// full blink APIs that normally own and embed it.
class LayerTreeHostEmbedder {
  USING_FAST_MALLOC(LayerTreeHostEmbedder);

 public:
  // Default constructor uses stub clients, and default LayerTreeSettings
  // appropriate for blink unit tests.
  LayerTreeHostEmbedder();
  // Constructor to specify the clients, or null to use stubs. Also specify
  // overrides of LayerTreeSettings.
  LayerTreeHostEmbedder(
      cc::LayerTreeHostClient* client,
      cc::LayerTreeHostSingleThreadClient* single_thread_client);

  cc::LayerTreeHost* layer_tree_host() { return layer_tree_host_.get(); }
  cc::AnimationHost* animation_host() { return animation_host_.get(); }

 private:
  cc::StubLayerTreeHostSingleThreadClient layer_tree_host_single_thread_client_;
  cc::StubLayerTreeHostClient layer_tree_host_client_;
  cc::TestTaskGraphRunner task_graph_runner_;
  std::unique_ptr<cc::AnimationHost> animation_host_;
  std::unique_ptr<cc::LayerTreeHost> layer_tree_host_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_LAYER_TREE_HOST_EMBEDDER_H_
