// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_PAINT_LAYER_PAINTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_PAINT_LAYER_PAINTER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/paint/paint_flags.h"
#include "third_party/blink/renderer/core/paint/paint_layer.h"
#include "third_party/blink/renderer/core/paint/paint_layer_fragment.h"
#include "third_party/blink/renderer/core/paint/paint_result.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class ComputedStyle;
class FragmentData;
class GraphicsContext;
class PhysicalBoxFragment;

// This class is responsible for painting self-painting PaintLayer.
//
// See PainterLayer SELF-PAINTING LAYER section about what 'self-painting'
// means and how it impacts this class.
class CORE_EXPORT PaintLayerPainter {
  STACK_ALLOCATED();

 public:
  PaintLayerPainter(PaintLayer& paint_layer) : paint_layer_(paint_layer) {}

  // Paints the layers from back to front. It assumes that the caller will
  // clip to the bounds of damage rect if necessary.
  PaintResult Paint(GraphicsContext&, PaintFlags = PaintFlag::kNoFlag);

  // Returns true if the painted output of this PaintLayer and its children is
  // invisible and therefore can't impact painted output.
  static bool PaintedOutputInvisible(const ComputedStyle&);

  // Returns the contents visual overflow rect in the coordinate space of the
  // contents.
  static PhysicalRect ContentsVisualRect(const FragmentData&, const LayoutBox&);

 private:
  friend class PaintLayerPainterTest;

  PaintResult PaintChildren(PaintLayerIteration children_to_visit,
                            GraphicsContext&,
                            PaintFlags);
  void PaintFragmentWithPhase(PaintPhase,
                              const FragmentData&,
                              wtf_size_t fragment_data_idx,
                              const PhysicalBoxFragment*,
                              GraphicsContext&,
                              PaintFlags);
  void PaintWithPhase(PaintPhase, GraphicsContext&, PaintFlags);
  void PaintForegroundPhases(GraphicsContext&, PaintFlags);
  void PaintOverlayOverflowControls(GraphicsContext&, PaintFlags);

  PaintLayer& paint_layer_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_PAINT_LAYER_PAINTER_H_
