/*
 * Copyright (C) 2004, 2005, 2006, 2008 Nikolas Zimmermann <zimmermann@kde.org>
 * Copyright (C) 2004, 2005, 2006 Rob Buis <buis@kde.org>
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_IMAGE_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_IMAGE_ELEMENT_H_

#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/html/canvas/image_element_base.h"
#include "third_party/blink/renderer/core/svg/svg_graphics_element.h"
#include "third_party/blink/renderer/core/svg/svg_image_loader.h"
#include "third_party/blink/renderer/core/svg/svg_uri_reference.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class SVGAnimatedLength;
class SVGAnimatedPreserveAspectRatio;

class CORE_EXPORT SVGImageElement final
    : public SVGGraphicsElement,
      public ImageElementBase,
      public SVGURIReference,
      public ActiveScriptWrappable<SVGImageElement> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit SVGImageElement(Document&);

  void Trace(Visitor*) const override;

  bool CurrentFrameHasSingleSecurityOrigin() const;

  SVGAnimatedLength* x() const { return x_.Get(); }
  SVGAnimatedLength* y() const { return y_.Get(); }
  SVGAnimatedLength* width() const { return width_.Get(); }
  SVGAnimatedLength* height() const { return height_.Get(); }
  SVGAnimatedPreserveAspectRatio* preserveAspectRatio() {
    return preserve_aspect_ratio_.Get();
  }

  bool HasPendingActivity() const final {
    return GetImageLoader().HasPendingActivity();
  }

  ScriptPromise<IDLUndefined> decode(ScriptState*, ExceptionState&);

  // Exposed for testing.
  ImageResourceContent* CachedImage() const {
    return GetImageLoader().GetContent();
  }

  void SetImageForTest(ImageResourceContent* content) {
    GetImageLoader().SetImageForTest(content);
  }

  bool SelfHasRelativeLengths() const override;

 private:
  bool IsStructurallyExternal() const override {
    return !HrefString().IsNull();
  }

  void SvgAttributeChanged(const SvgAttributeChangedParams&) override;
  void ParseAttribute(const AttributeModificationParams&) override;

  void AttachLayoutTree(AttachContext&) override;

  LayoutObject* CreateLayoutObject(const ComputedStyle&) override;

  const AtomicString ImageSourceURL() const override;

  bool HaveLoadedRequiredResources() override;

  void DidMoveToNewDocument(Document& old_document) override;
  SVGImageLoader& GetImageLoader() const override { return *image_loader_; }

  SVGAnimatedPropertyBase* PropertyFromAttribute(
      const QualifiedName& attribute_name) const override;
  void SynchronizeAllSVGAttributes() const override;
  void CollectExtraStyleForPresentationAttribute(
      HeapVector<CSSPropertyValue, 8>& style) override;

  Member<SVGAnimatedLength> x_;
  Member<SVGAnimatedLength> y_;
  Member<SVGAnimatedLength> width_;
  Member<SVGAnimatedLength> height_;
  Member<SVGAnimatedPreserveAspectRatio> preserve_aspect_ratio_;

  Member<SVGImageLoader> image_loader_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_IMAGE_ELEMENT_H_
