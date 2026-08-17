// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_COMPOSITING_ANDROID_WEBVIEW_SYNCHRONOUS_COMPOSITOR_REGISTRY_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_COMPOSITING_ANDROID_WEBVIEW_SYNCHRONOUS_COMPOSITOR_REGISTRY_H_

namespace blink {

class SynchronousLayerTreeFrameSink;

// An abstract interface that the SynchronousLayerTreeFrameSink can be registered the
// creation and destruction with. When the SynchronousCompositorProxy is created it
// will receive the allocated frame sink from the registry.
class SynchronousCompositorRegistry {
 public:
  virtual void RegisterLayerTreeFrameSink(
      SynchronousLayerTreeFrameSink* layer_tree_frame_sink) = 0;
  virtual void UnregisterLayerTreeFrameSink(
      SynchronousLayerTreeFrameSink* layer_tree_frame_sink) = 0;

 protected:
  virtual ~SynchronousCompositorRegistry() {}
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WIDGET_COMPOSITING_ANDROID_WEBVIEW_SYNCHRONOUS_COMPOSITOR_REGISTRY_H_
