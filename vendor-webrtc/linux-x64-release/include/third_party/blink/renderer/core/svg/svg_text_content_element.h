/*
 * Copyright (C) 2004, 2005, 2008 Nikolas Zimmermann <zimmermann@kde.org>
 * Copyright (C) 2004, 2005, 2006, 2008 Rob Buis <buis@kde.org>
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_TEXT_CONTENT_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_TEXT_CONTENT_ELEMENT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/svg/svg_animated_enumeration.h"
#include "third_party/blink/renderer/core/svg/svg_graphics_element.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class DOMPointInit;
class ExceptionState;
class LineLayoutItem;
class SVGAnimatedLength;
class SVGPointTearOff;

enum SVGLengthAdjustType {
  kSVGLengthAdjustUnknown,
  kSVGLengthAdjustSpacing,
  kSVGLengthAdjustSpacingAndGlyphs
};
DECLARE_SVG_ENUM_MAP(SVGLengthAdjustType);

class CORE_EXPORT SVGTextContentElement : public SVGGraphicsElement {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // Forward declare enumerations in the W3C naming scheme, for IDL generation.
  enum {
    kLengthadjustUnknown = kSVGLengthAdjustUnknown,
    kLengthadjustSpacing = kSVGLengthAdjustSpacing,
    kLengthadjustSpacingandglyphs = kSVGLengthAdjustSpacingAndGlyphs
  };

  unsigned getNumberOfChars();
  float getComputedTextLength();
  float getSubStringLength(unsigned charnum, unsigned nchars, ExceptionState&);
  SVGPointTearOff* getStartPositionOfChar(unsigned charnum, ExceptionState&);
  SVGPointTearOff* getEndPositionOfChar(unsigned charnum, ExceptionState&);
  SVGRectTearOff* getExtentOfChar(unsigned charnum, ExceptionState&);
  float getRotationOfChar(unsigned charnum, ExceptionState&);
  int getCharNumAtPosition(DOMPointInit*, ExceptionState&);
  void selectSubString(unsigned charnum, unsigned nchars, ExceptionState&);

  static SVGTextContentElement* ElementFromLineLayoutItem(
      const LineLayoutItem&);

  SVGAnimatedLength* textLength() { return text_length_.Get(); }
  bool TextLengthIsSpecifiedByUser() {
    return text_length_is_specified_by_user_;
  }
  SVGAnimatedEnumeration<SVGLengthAdjustType>* lengthAdjust() {
    return length_adjust_.Get();
  }

  void Trace(Visitor*) const override;

 protected:
  SVGTextContentElement(const QualifiedName&, Document&);

  bool IsPresentationAttribute(const QualifiedName&) const final;
  void CollectStyleForPresentationAttribute(
      const QualifiedName&,
      const AtomicString&,
      HeapVector<CSSPropertyValue, 8>&) final;
  void SvgAttributeChanged(const SvgAttributeChangedParams&) override;

  bool SelfHasRelativeLengths() const override;

  SVGAnimatedPropertyBase* PropertyFromAttribute(
      const QualifiedName& attribute_name) const override;
  void SynchronizeAllSVGAttributes() const override;

 private:
  bool IsTextContent() const final { return true; }

  Member<SVGAnimatedLength> text_length_;
  bool text_length_is_specified_by_user_;
  Member<SVGAnimatedEnumeration<SVGLengthAdjustType>> length_adjust_;
};

inline bool IsSVGTextContentElement(const SVGElement& element) {
  return element.IsTextContent();
}

template <>
struct DowncastTraits<SVGTextContentElement> {
  static bool AllowFrom(const Node& node) {
    auto* svg_element = DynamicTo<SVGElement>(node);
    return svg_element && IsSVGTextContentElement(*svg_element);
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_TEXT_CONTENT_ELEMENT_H_
