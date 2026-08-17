// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_CHILD_FRAME_COMPOSITOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_CHILD_FRAME_COMPOSITOR_H_

class SkBitmap;

namespace cc {
class Layer;
}

namespace blink {

// A ChildFrameCompositor is an owner of a cc::Layer that embeds a child
// frame.
class ChildFrameCompositor {
 public:
  // Get the child frame's cc::Layer.
  virtual const scoped_refptr<cc::Layer>& GetCcLayer() = 0;

  // Passes ownership of a cc::Layer to the ChildFrameCompositor.
  virtual void SetCcLayer(scoped_refptr<cc::Layer> layer,
                          bool is_surface_layer) = 0;

  // Returns a sad page bitmap used when the child frame has crashed.
  virtual SkBitmap* GetSadPageBitmap() = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_CHILD_FRAME_COMPOSITOR_H_
