// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_SVG_TEXT_QUERY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_SVG_TEXT_QUERY_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace gfx {
class PointF;
class RectF;
}  // namespace gfx

namespace blink {

class LayoutObject;

// An implementation of SVG DOM functions to retrieve text geometry.
class SvgTextQuery {
  STACK_ALLOCATED();

 public:
  explicit SvgTextQuery(LayoutObject& query_root) : query_root_(query_root) {}

  unsigned NumberOfCharacters() const;
  float SubStringLength(unsigned start_index, unsigned length) const;
  gfx::PointF StartPositionOfCharacter(unsigned index) const;
  gfx::PointF EndPositionOfCharacter(unsigned index) const;
  gfx::RectF ExtentOfCharacter(unsigned index) const;
  float RotationOfCharacter(unsigned index) const;
  int CharacterNumberAtPosition(const gfx::PointF& position) const;

 private:
  LayoutObject& query_root_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_SVG_TEXT_QUERY_H_
