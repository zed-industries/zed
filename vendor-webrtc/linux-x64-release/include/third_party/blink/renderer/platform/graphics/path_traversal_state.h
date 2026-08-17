/*
 * Copyright (C) 2006, 2007 Eric Seidel <eric@webkit.org>
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PATH_TRAVERSAL_STATE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PATH_TRAVERSAL_STATE_H_

#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "ui/gfx/geometry/point_f.h"

namespace blink {

class PLATFORM_EXPORT PathTraversalState final {
  STACK_ALLOCATED();

 public:
  enum PathTraversalAction {
    kTraversalTotalLength,
    kTraversalPointAtLength,
    kTraversalNormalAngleAtLength
  };

  PathTraversalState(PathTraversalAction);
  PathTraversalState(const PathTraversalState&) = delete;
  PathTraversalState& operator=(const PathTraversalState&) = delete;

  float CloseSubpath();
  float MoveTo(const gfx::PointF&);
  float LineTo(const gfx::PointF&);
  float CubicBezierTo(const gfx::PointF& new_control1,
                      const gfx::PointF& new_control2,
                      const gfx::PointF& new_end);

  void ProcessSegment();

 public:
  PathTraversalAction action_;
  bool success_;

  gfx::PointF current_;
  gfx::PointF start_;

  float total_length_;
  float desired_length_;

  // For normal calculations
  gfx::PointF previous_;
  float normal_angle_;  // degrees
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PATH_TRAVERSAL_STATE_H_
