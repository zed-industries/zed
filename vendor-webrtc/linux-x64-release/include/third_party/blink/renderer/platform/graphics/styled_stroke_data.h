// Copyright (C) 2013 Google Inc. All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//    * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
//    * Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following disclaimer
// in the documentation and/or other materials provided with the
// distribution.
//    * Neither the name of Google Inc. nor the names of its
// contributors may be used to endorse or promote products derived from
// this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_STYLED_STROKE_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_STYLED_STROKE_DATA_H_

#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace cc {
class PaintFlags;
}  // namespace cc

namespace blink {

class StrokeData;

enum StrokeStyle {
  kNoStroke,
  kSolidStroke,
  kDottedStroke,
  kDashedStroke,
  kDoubleStroke,
  kWavyStroke,
};

// Stroke geometry information based on a specified style (StrokeStyle).
//
// Used to represent decorations (borders, outline, underline and other text
// decorations).
//
// If full control is required, use StrokeData.
class PLATFORM_EXPORT StyledStrokeData final {
  DISALLOW_NEW();

 public:
  StrokeStyle Style() const { return style_; }
  void SetStyle(StrokeStyle style) { style_ = style; }

  float Thickness() const { return thickness_; }
  void SetThickness(float thickness) { thickness_ = thickness; }

  // Transfer the stroke data to the PaintFlags object.
  void SetupPaint(cc::PaintFlags*) const;

  // Structure that describe the geometry of the object that this stroke will
  // be applied to.
  //
  // If a non-zero `path_length` is provided, the number of dashes/dots on a
  // dashed/dotted line will be adjusted to start and end that length with a
  // dash/dot.
  //
  // If non-zero, `dash_thickness` is the thickness to use when deciding on
  // dash sizes. Used in border painting when we stroke thick to allow for
  // clipping at corners, but still want small dashes.
  //
  // If `closed_path` is true, a gap will be allocated after the last dash, so
  // that all dashes will be evenly spaced on the closed path.
  struct GeometryInfo {
    int path_length = 0;
    int dash_thickness = 0;
    bool closed_path = false;
  };

  // Transfer the stroke data to the PaintFlags object, resolving any
  // DashPathEffect using the specified GeometryInfo.
  void SetupPaint(cc::PaintFlags*, const GeometryInfo&) const;

  // Convert this stroke geometry information to the "resolved" representation.
  StrokeData ConvertToStrokeData(const GeometryInfo&) const;

  // Determine whether a stroked line should be drawn using dashes. In practice,
  // we draw dashes when a dashed stroke is specified or when a dotted stroke
  // is specified but the line width is too small to draw circles.
  static bool StrokeIsDashed(float width, StrokeStyle);

 private:
  // Resolve and set any DashPathEffect on the paint.
  void SetupPaintDashPathEffect(cc::PaintFlags*, const GeometryInfo&) const;

  StrokeStyle style_ = kSolidStroke;
  float thickness_ = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_STYLED_STROKE_DATA_H_
