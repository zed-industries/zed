/*
 * Copyright (C) 2004, 2005, 2006 Nikolas Zimmermann <zimmermann@kde.org>
 * Copyright (C) 2004, 2005, 2006, 2007, 2010 Rob Buis <buis@kde.org>
 * Copyright (C) 2014 Google, Inc.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_SVG_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_SVG_ELEMENT_H_

#include "third_party/blink/renderer/core/svg/svg_fit_to_view_box.h"
#include "third_party/blink/renderer/core/svg/svg_graphics_element.h"
#include "third_party/blink/renderer/core/svg/svg_point.h"
#include "third_party/blink/renderer/core/svg/svg_zoom_and_pan.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "ui/gfx/geometry/vector2d_f.h"

namespace blink {

class SMILTimeContainer;
class SVGAngleTearOff;
class SVGAnimatedLength;
class SVGLengthTearOff;
class SVGMatrixTearOff;
class SVGNumberTearOff;
class SVGPointTearOff;
class SVGRect;
class SVGTransformTearOff;
class SVGViewSpec;

class SVGSVGElement final : public SVGGraphicsElement,
                            public SVGFitToViewBox,
                            public SVGZoomAndPan {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit SVGSVGElement(Document&);
  ~SVGSVGElement() override;

  std::optional<float> IntrinsicWidth() const;
  std::optional<float> IntrinsicHeight() const;
  const SVGRect& CurrentViewBox() const;
  // This method, as opposed to the one above, also includes the synthesized
  // viewBox if one is active. Because of that it shouldn't be used for sizing
  // calculations.
  gfx::RectF CurrentViewBoxRect() const;
  bool HasEmptyViewBox() const;
  const SVGPreserveAspectRatio* CurrentPreserveAspectRatio() const;

  float currentScale() const;
  void setCurrentScale(float scale);

  gfx::Vector2dF CurrentTranslate() {
    return translation_->Value().OffsetFromOrigin();
  }
  void SetCurrentTranslate(const gfx::Vector2dF&);
  SVGPointTearOff* currentTranslateFromJavascript();

  SMILTimeContainer* TimeContainer() const { return time_container_.Get(); }

  void pauseAnimations();
  void unpauseAnimations();
  bool animationsPaused() const;

  float getCurrentTime() const;
  void setCurrentTime(float seconds);

  // Stubs for the deprecated 'redraw' interface.
  unsigned suspendRedraw(unsigned) { return 1; }
  void unsuspendRedraw(unsigned) {}
  void unsuspendRedrawAll() {}
  void forceRedraw() {}

  StaticNodeTypeList<Element>* getIntersectionList(
      SVGRectTearOff*,
      SVGElement* reference_element) const;
  StaticNodeList* getEnclosureList(SVGRectTearOff*,
                                   SVGElement* reference_element) const;
  bool checkIntersection(SVGElement*, SVGRectTearOff*) const;
  bool checkEnclosure(SVGElement*, SVGRectTearOff*) const;
  void deselectAll();

  static SVGNumberTearOff* createSVGNumber();
  static SVGLengthTearOff* createSVGLength();
  static SVGAngleTearOff* createSVGAngle();
  static SVGPointTearOff* createSVGPoint();
  static SVGMatrixTearOff* createSVGMatrix();
  static SVGRectTearOff* createSVGRect();
  static SVGTransformTearOff* createSVGTransform();
  static SVGTransformTearOff* createSVGTransformFromMatrix(SVGMatrixTearOff*);

  AffineTransform ViewBoxToViewTransform(const gfx::SizeF& viewport_size) const;

  const SVGViewSpec* ParseViewSpec(const String& fragment_identifier,
                                   Element* anchor_node) const;
  void SetViewSpec(const SVGViewSpec*);

  bool ZoomAndPanEnabled() const;

  SVGAnimatedLength* x() const { return x_.Get(); }
  SVGAnimatedLength* y() const { return y_.Get(); }
  SVGAnimatedLength* width() const { return width_.Get(); }
  SVGAnimatedLength* height() const { return height_.Get(); }

  void Trace(Visitor*) const override;

 private:
  void ParseAttribute(const AttributeModificationParams&) override;
  bool IsPresentationAttribute(const QualifiedName&) const override;
  void CollectStyleForPresentationAttribute(
      const QualifiedName&,
      const AtomicString&,
      HeapVector<CSSPropertyValue, 8>&) override;

  void AttachLayoutTree(AttachContext&) override;
  bool LayoutObjectIsNeeded(const DisplayStyle&) const override;
  LayoutObject* CreateLayoutObject(const ComputedStyle&) override;

  InsertionNotificationRequest InsertedInto(ContainerNode&) override;
  void RemovedFrom(ContainerNode&) override;

  void SvgAttributeChanged(const SvgAttributeChangedParams&) override;

  void DidMoveToNewDocument(Document& old_document) override;

  bool SelfHasRelativeLengths() const override;

  bool ShouldSynthesizeViewBox() const;
  void UpdateUserTransform();

  void FinishParsingChildren() override;

  bool CheckEnclosure(const SVGElement&, const gfx::RectF&) const;

  SVGAnimatedPropertyBase* PropertyFromAttribute(
      const QualifiedName& attribute_name) const override;
  void SynchronizeAllSVGAttributes() const override;
  void CollectExtraStyleForPresentationAttribute(
      HeapVector<CSSPropertyValue, 8>& style) override;

  Member<SVGAnimatedLength> x_;
  Member<SVGAnimatedLength> y_;
  Member<SVGAnimatedLength> width_;
  Member<SVGAnimatedLength> height_;

  AffineTransform LocalCoordinateSpaceTransform(CTMScope) const override;

  Member<SMILTimeContainer> time_container_;
  Member<SVGPoint> translation_;
  Member<const SVGViewSpec> view_spec_;
  float current_scale_;

  friend class SVGCurrentTranslateTearOff;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_SVG_ELEMENT_H_
