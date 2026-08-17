/*
 * Copyright (c) 2013, Opera Software ASA. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. Neither the name of Opera Software ASA nor the names of its
 *    contributors may be used to endorse or promote products derived
 *    from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
 * FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
 * COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT,
 * INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
 * STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
 * OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_VTT_VTT_CUE_BOX_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_VTT_VTT_CUE_BOX_H_

#include "third_party/blink/renderer/core/html/html_div_element.h"

namespace blink {

class VttCueLayoutAlgorithm;
struct VTTDisplayParameters;

// VTTCueBox represents the bounding box of a VTTCue.
//
// It is an element in a media element shadow tree, and is always a child of
// TextTrackContainer. It always has a single VTTCueBackgroundBox child.
class VTTCueBox final : public HTMLDivElement {
 public:
  explicit VTTCueBox(Document&);
  void Trace(Visitor* visitor) const override;
  bool IsVTTCueBox() const override { return true; }

  void ApplyCSSProperties(const VTTDisplayParameters&);

  float SnapToLinesPosition() const { return snap_to_lines_position_; }
  bool IsAdjusted() const { return adjusted_position_ != LayoutUnit::Min(); }
  // IsAdjusted() becomes true after calling this.
  LayoutUnit& StartAdjustment(LayoutUnit new_value,
                              base::PassKey<VttCueLayoutAlgorithm>);
  // IsAdjusted() becomes false after calling this.
  void RevertAdjustment();
  // Returns adjusted_position_ if IsAdjusted(), or the initial position
  // without adjustment otherwise.
  LayoutUnit AdjustedPosition(LayoutUnit full_dimention,
                              base::PassKey<VttCueLayoutAlgorithm>) const;

 private:
  LayoutObject* CreateLayoutObject(const ComputedStyle&) override;
  InsertionNotificationRequest InsertedInto(
      ContainerNode& insertion_point) override;
  void RemovedFrom(ContainerNode& insertion_point) override;

  bool IsInRegion() const;

  Member<ResizeObserver> box_size_observer_;
  // The computed line position for snap-to-lines layout, and NaN for
  // non-snap-to-lines layout where no adjustment should take place.
  // This is set in applyCSSProperties and propagated to VttCueLayoutAlgorithm.
  float snap_to_lines_position_;

  // Percentage position before adjustment.
  float original_percent_position_;

  // Pixel position after adjustment. LayoutUnit::Min() means this box is not
  // adjusted yet.
  LayoutUnit adjusted_position_ = LayoutUnit::Min();
};

template <>
struct DowncastTraits<VTTCueBox> {
  static bool AllowFrom(const Node& node) {
    return node.IsElementNode() && To<Element>(node).IsVTTCueBox();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_VTT_VTT_CUE_BOX_H_
