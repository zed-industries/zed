/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 2000 Simon Hausmann <hausmann@kde.org>
 * Copyright (C) 2006, 2008 Apple Inc. All rights reserved.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FRAME_EDGE_INFO_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FRAME_EDGE_INFO_H_

#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

enum FrameEdge {
  kLeftFrameEdge = 0,
  kRightFrameEdge,
  kTopFrameEdge,
  kBottomFrameEdge,
};
constexpr size_t kFrameEdgeCount = kBottomFrameEdge + 1;

// Represents border and resize capability of a <frame> or a <frameset>.
class FrameEdgeInfo {
  STACK_ALLOCATED();

 public:
  FrameEdgeInfo(bool prevent_resize, bool allow_border)
      : prevent_resize_(kFrameEdgeCount), allow_border_(kFrameEdgeCount) {
    prevent_resize_.Fill(prevent_resize);
    allow_border_.Fill(allow_border);
  }

  bool PreventResize(FrameEdge edge) const { return prevent_resize_[edge]; }
  bool AllowBorder(FrameEdge edge) const { return allow_border_[edge]; }

  void SetPreventResize(FrameEdge edge, bool prevent_resize) {
    prevent_resize_[edge] = prevent_resize;
  }
  void SetAllowBorder(FrameEdge edge, bool allow_border) {
    allow_border_[edge] = allow_border;
  }

 private:
  Vector<bool> prevent_resize_;
  Vector<bool> allow_border_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FRAME_EDGE_INFO_H_
