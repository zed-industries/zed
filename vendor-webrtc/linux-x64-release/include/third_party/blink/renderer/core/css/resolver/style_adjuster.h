/*
 * Copyright (C) 2013 Google, Inc.
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 * Copyright (C) 2003, 2004, 2005, 2006, 2007, 2008, 2009, 2010, 2011 Apple Inc.
 * All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_STYLE_ADJUSTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_STYLE_ADJUSTER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/pseudo_element.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class Element;
class ComputedStyle;
class ComputedStyleBuilder;
class StyleResolverState;
class SVGElement;

// Certain CSS Properties/Values do not apply to certain elements
// and the web expects that we expose "adjusted" values when
// for those property/element pairs.
class StyleAdjuster {
  STATIC_ONLY(StyleAdjuster);

 public:
  CORE_EXPORT static void AdjustComputedStyle(StyleResolverState&, Element*);
  static void AdjustStyleForCombinedText(ComputedStyleBuilder&);
  static void AdjustStyleForEditing(ComputedStyleBuilder&, Element*);
  static void AdjustStyleForTextCombine(ComputedStyleBuilder&);
  static void AdjustStyleForSvgElement(
      const SVGElement& element,
      ComputedStyleBuilder& builder,
      const ComputedStyle& layout_parent_style);
  static void AdjustStyleForDisplay(ComputedStyleBuilder&,
                                    const ComputedStyle& layout_parent_style,
                                    const Element*,
                                    Document*);

 private:
  static bool IsEditableElement(Element*, const ComputedStyleBuilder&);
  static bool IsPasswordFieldWithUnrevealedPassword(Element*);
  static void AdjustEffectiveTouchAction(ComputedStyleBuilder&,
                                         const ComputedStyle& parent_style,
                                         Element* element,
                                         bool is_svg_root);
  static void AdjustOverflow(ComputedStyleBuilder&, Element* element);
  static void AdjustForForcedColorsMode(ComputedStyleBuilder&, Document&);
  static void AdjustForSVGTextElement(ComputedStyleBuilder&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_STYLE_ADJUSTER_H_
