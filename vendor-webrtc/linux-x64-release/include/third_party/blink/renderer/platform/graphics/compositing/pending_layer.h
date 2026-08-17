// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_COMPOSITING_PENDING_LAYER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_COMPOSITING_PENDING_LAYER_H_

#include "base/check_op.h"
#include "cc/input/layer_selection_bound.h"
#include "third_party/blink/renderer/platform/graphics/compositing/content_layer_client_impl.h"
#include "third_party/blink/renderer/platform/graphics/lcd_text_preference.h"
#include "third_party/blink/renderer/platform/graphics/paint/paint_chunk_subset.h"
#include "third_party/blink/renderer/platform/graphics/paint/property_tree_state.h"
#include "ui/gfx/geometry/rect_f.h"
#include "ui/gfx/geometry/vector2d_f.h"

namespace cc {
class LayerTreeHost;
}

namespace blink {

class PendingLayer;
using PendingLayers = HeapVector<PendingLayer>;

// A pending layer is a collection of paint chunks that will end up in the same
// cc::Layer.
class PLATFORM_EXPORT PendingLayer {
  DISALLOW_NEW();

 public:
  enum CompositingType {
    kScrollHitTestLayer,
    kForeignLayer,
    kScrollbarLayer,
    kOverlap,
    kOther,
  };

  PendingLayer(const PaintArtifact&,
               const PaintChunk& first_chunk,
               CompositingType = kOther);

  void Trace(Visitor*) const;

  // Returns the offset/bounds for the final cc::Layer, rounded if needed.
  std::pair<gfx::Vector2dF, gfx::Size> Bounds() const;

  const gfx::RectF& BoundsForTesting() const { return bounds_; }

  const gfx::RectF& RectKnownToBeOpaque() const {
    return rect_known_to_be_opaque_;
  }
  bool TextKnownToBeOnOpaqueBackground() const {
    return text_known_to_be_on_opaque_background_;
  }
  const PaintChunkSubset& Chunks() const { return chunks_; }
  PropertyTreeState GetPropertyTreeState() const {
    return PropertyTreeState(property_tree_state_);
  }
  const gfx::Vector2dF& OffsetOfDecompositedTransforms() const {
    return offset_of_decomposited_transforms_;
  }
  PaintPropertyChangeType ChangeOfDecompositedTransforms() const {
    return change_of_decomposited_transforms_;
  }
  CompositingType GetCompositingType() const { return compositing_type_; }
  cc::HitTestOpaqueness GetHitTestOpaqueness() const {
    return hit_test_opaqueness_;
  }
  bool HasText() const { return has_text_; }

  void SetCompositingTypeToOverlap() {
    DCHECK_EQ(compositing_type_, kOther);
    compositing_type_ = kOverlap;
  }

  void SetPaintArtifact(const PaintArtifact& paint_artifact) {
    chunks_.SetPaintArtifact(paint_artifact);
  }

  using IsCompositedScrollFunction =
      PropertyTreeState::IsCompositedScrollFunction;

  // Merges |guest| into |this| if it can, by appending chunks of |guest|
  // after chunks of |this|, with appropriate space conversion applied to
  // both layers from their original property tree states to |merged_state|.
  // Returns whether the merge is successful.
  bool Merge(const PendingLayer& guest,
             LCDTextPreference lcd_text_preference,
             float device_pixel_ratio,
             IsCompositedScrollFunction);

  // Returns true if `guest` that could be upcasted with decomposited blend
  // mode can be merged into `this`.
  bool CanMergeWithDecompositedBlendMode(const PendingLayer& guest,
                                         const PropertyTreeState& upcast_state,
                                         IsCompositedScrollFunction) const;

  // Mutate this layer's property tree state to a more general (shallower)
  // state, thus the name "upcast". The concrete effect of this is to
  // "decomposite" some of the properties, so that fewer properties will be
  // applied by the compositor, and more properties will be applied internally
  // to the chunks as Skia commands.
  void Upcast(const PropertyTreeState&);

  const PaintChunk& FirstPaintChunk() const;
  const DisplayItem& FirstDisplayItem() const;

  bool Matches(const PendingLayer& old_pending_layer) const;

  const TransformPaintPropertyNode& ScrollTranslationForScrollHitTestLayer()
      const;

  std::unique_ptr<JSONObject> ToJSON() const;
  String DebugName() const;
  DOMNodeId OwnerNodeId() const;

  void ForceDrawsContent() { draws_content_ = true; }
  bool DrawsContent() const { return draws_content_; }

  static bool RequiresOwnLayer(CompositingType type) {
    return type != kOverlap && type != kOther;
  }

  bool ChunkRequiresOwnLayer() const {
    bool result = RequiresOwnLayer(compositing_type_);
#if DCHECK_IS_ON()
    if (result) {
      DCHECK(!content_layer_client_);
      DCHECK_EQ(chunks_.size(), 1u);
    } else {
      DCHECK(!cc_layer_ || UsesSolidColorLayer());
      DCHECK_GE(chunks_.size(), 1u);
    }
#endif
    return result;
  }

  bool MightOverlap(const PendingLayer& other) const;

  static void DecompositeTransforms(PendingLayers& pending_layers);

  // This is valid only when SetCclayer() or SetContentLayerClient() has been
  // called.
  cc::Layer& CcLayer() const {
    if (content_layer_client_)
      return content_layer_client_->Layer();
    DCHECK(cc_layer_);
    return *cc_layer_;
  }

  ContentLayerClientImpl* GetContentLayerClient() const {
    return content_layer_client_.Get();
  }

  // For this PendingLayer, creates a composited layer or uses the existing
  // one in |old_pending_layer|, and updates the layer according to the current
  // contents and properties of this PendingLayer.
  void UpdateCompositedLayer(PendingLayer* old_pending_layer,
                             cc::LayerSelection&,
                             bool tracks_raster_invalidations,
                             cc::LayerTreeHost*);

  // A lighter version of UpdateCompositedLayer(). Called when the existing
  // composited layer has only repainted since the last update
  void UpdateCompositedLayerForRepaint(const PaintArtifact& repainted_artifact,
                                       cc::LayerSelection&);

  // Another lighter version of UpdateCompositedLayers(). Called after
  // raster-inducing scrolls that don't need repaint or PaintArtifactCompositor
  // update.
  void UpdateForRasterInducingScroll();

  SkColor4f ComputeBackgroundColor() const;

  // True if a solid color chunk exists that makes this entire layer
  // draw a solid color (see comment above `solid_color_chunk_index_`).
  bool IsSolidColor() const { return solid_color_chunk_index_ != kNotFound; }

 private:
  // Checks basic merge-ability with `guest` and calls
  // PropertyTreeState::CanUpcastWith().
  std::optional<PropertyTreeState> CanUpcastWith(
      const PendingLayer& guest,
      const PropertyTreeState& guest_state,
      IsCompositedScrollFunction is_comosited_scroll) const;

  bool CanMerge(const PendingLayer& guest,
                LCDTextPreference lcd_text_preference,
                float device_pixel_ratio,
                IsCompositedScrollFunction,
                gfx::RectF& merged_bounds,
                PropertyTreeState& merged_state,
                gfx::RectF& merged_rect_known_to_be_opaque,
                bool& merged_text_known_to_be_on_opaque_background,
                wtf_size_t& merged_solid_color_chunk_index,
                cc::HitTestOpaqueness& merged_hit_test_opaqueness) const;

  gfx::RectF MapRectKnownToBeOpaque(
      const PropertyTreeState& new_state,
      const FloatClipRect& mapped_layer_bounds) const;

  bool PropertyTreeStateChanged(const PendingLayer* old_pending_layer) const;

  // The following methods are called by UpdateCompositedLayer(), each for a
  // particular type of composited layer.
  void UpdateForeignLayer();
  void UpdateScrollHitTestLayer(PendingLayer* old_pending_layer);
  void UpdateScrollbarLayer(PendingLayer* old_pending_layer);
  void UpdateContentLayer(PendingLayer* old_pending_layer,
                          bool tracks_raster_invalidations);
  void UpdateSolidColorLayer(PendingLayer* old_pending_layer);

  void UpdateLayerProperties(cc::LayerSelection&, bool selection_only);

  bool UsesSolidColorLayer() const;
  SkColor4f GetSolidColor() const;

  // The rects are in the space of property_tree_state.
  gfx::RectF bounds_;
  gfx::RectF rect_known_to_be_opaque_;
  bool has_text_ = false;
  bool draws_content_ = false;
  bool text_known_to_be_on_opaque_background_ = false;
  bool has_decomposited_blend_mode_ = false;
  // If not kNotFound, this is the index of the chunk that makes this layer
  // solid color. The solid color chunk must be the last drawable chunk and
  // must draw a solid color that fully covers this pending layer.
  wtf_size_t solid_color_chunk_index_ = kNotFound;
  PaintChunkSubset chunks_;
  TraceablePropertyTreeState property_tree_state_;
  gfx::Vector2dF offset_of_decomposited_transforms_;
  PaintPropertyChangeType change_of_decomposited_transforms_ =
      PaintPropertyChangeType::kUnchanged;
  CompositingType compositing_type_ = kOther;
  cc::HitTestOpaqueness hit_test_opaqueness_ =
      cc::HitTestOpaqueness::kTransparent;

  // Contains non-composited hit_test_data.scroll_translation of PaintChunks.
  // This is a vector instead of a set because the size is small vs the cost of
  // hashing.
  HeapVector<Member<const TransformPaintPropertyNode>>
      non_composited_scroll_translations_;

  // This is set to non-null after layerization if ChunkRequiresOwnLayer() or
  // UsesSolidColorLayer() is true.
  scoped_refptr<cc::Layer> cc_layer_;
  // This is set to non-null after layerization if ChunkRequiresOwnLayer() and
  // UsesSolidColorLayer() are false.
  Member<ContentLayerClientImpl> content_layer_client_;
};

PLATFORM_EXPORT std::ostream& operator<<(std::ostream&, const PendingLayer&);

}  // namespace blink

WTF_ALLOW_CLEAR_UNUSED_SLOTS_WITH_MEM_FUNCTIONS(blink::PendingLayer)

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_COMPOSITING_PENDING_LAYER_H_
