/*
 * Copyright (C) 2006 Oliver Hunt <ojh16@student.canterbury.ac.nz>
 * Copyright (C) 2006 Apple Computer Inc.
 * Copyright (C) 2009 Google Inc.
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
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_TSPAN_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_TSPAN_H_

#include "third_party/blink/renderer/core/layout/svg/layout_svg_inline.h"

namespace blink {
class LayoutSVGTSpan final : public LayoutSVGInline {
 public:
  explicit LayoutSVGTSpan(Element*);

  bool IsSVGTSpan() const final {
    NOT_DESTROYED();
    return true;
  }
  bool IsChildAllowed(LayoutObject*, const ComputedStyle&) const override;

  const char* GetName() const override {
    NOT_DESTROYED();
    return "LayoutSVGTSpan";
  }
};
}

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_TSPAN_H_
