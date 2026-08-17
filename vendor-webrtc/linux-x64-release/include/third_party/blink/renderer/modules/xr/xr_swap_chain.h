// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_SWAP_CHAIN_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_SWAP_CHAIN_H_

#include "third_party/blink/renderer/modules/xr/xr_composition_layer.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

// An XRSwapChain manages the creation and lifetime of textures used by an
// XRCompositorLayer. The general lifetime is expected to be:
//
// OnFrameStart() - Called prior to the XRFrame callback for any active layers
// GetCurrentTexture() - Called when a sub image is requested for the layer
//  > ProduceTexture() - Called by GetCurrentTexture if a new texture is needed
// OnFrameEnd() - Called prior to the frame submission
//  > ResetCurrentTexture() - Called by OnFrameEnd if a new texture is required
//                            for the next frame.

template <typename Texture>
class XRSwapChain : public GarbageCollected<XRSwapChain<Texture>> {
 public:
  XRSwapChain() {}
  virtual ~XRSwapChain() = default;

  Texture* GetCurrentTexture() {
    texture_queried_ = true;
    if (!current_texture_) {
      current_texture_ = ProduceTexture();
    }
    return current_texture_;
  }
  virtual void OnFrameStart() { texture_queried_ = false; }
  virtual void OnFrameEnd() { ResetCurrentTexture(); }

  // Manage the XRCompositorLayer this swap chain is associated with.
  virtual void SetLayer(XRCompositionLayer* layer) { layer_ = layer; }
  XRCompositionLayer* layer() { return layer_.Get(); }

  // Indicates if the texture was queried during the most recent frame.
  bool texture_was_queried() const { return texture_queried_; }

  virtual void Trace(Visitor* visitor) const {
    visitor->Trace(current_texture_);
    visitor->Trace(layer_);
  }

 protected:
  // Produces a new Texture for the swap chain. Will not be called again unless
  // the current texture is reset.
  virtual Texture* ProduceTexture() = 0;

  // Resets the cached texture so that next GetCurrentTexture call will trigger
  // a ProduceTexture call.
  Texture* ResetCurrentTexture() {
    Texture* texture = current_texture_.Get();
    current_texture_ = nullptr;
    return texture;
  }

  // Return the current texture if one has been produced and not reset.
  Texture* current_texture() { return current_texture_; }

 private:
  Member<Texture> current_texture_;
  Member<XRCompositionLayer> layer_;

  bool texture_queried_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_SWAP_CHAIN_H_
