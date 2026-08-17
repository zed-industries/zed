/*
 * Copyright (C) 2004, 2005, 2007 Nikolas Zimmermann <zimmermann@kde.org>
 * Copyright (C) 2004, 2005 Rob Buis <buis@kde.org>
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_FE_IMAGE_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_FE_IMAGE_ELEMENT_H_

#include "third_party/blink/renderer/core/loader/resource/image_resource_observer.h"
#include "third_party/blink/renderer/core/svg/svg_filter_primitive_standard_attributes.h"
#include "third_party/blink/renderer/core/svg/svg_uri_reference.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/prefinalizer.h"

namespace blink {

class ImageResourceContent;
class SVGAnimatedPreserveAspectRatio;

class SVGFEImageElement final : public SVGFilterPrimitiveStandardAttributes,
                                public SVGURIReference,
                                public ImageResourceObserver {
  DEFINE_WRAPPERTYPEINFO();
  // Pre-finalize to promptly remove as a ImageResource client.
  USING_PRE_FINALIZER(SVGFEImageElement, Dispose);

 public:
  bool CurrentFrameHasSingleSecurityOrigin() const;

  explicit SVGFEImageElement(Document&);
  ~SVGFEImageElement() override;
  SVGAnimatedPreserveAspectRatio* preserveAspectRatio() {
    return preserve_aspect_ratio_.Get();
  }
  const SVGElement* TargetElement() const;

  void Dispose();

  void Trace(Visitor*) const override;

 private:
  void SvgAttributeChanged(const SvgAttributeChangedParams&) override;
  void ImageNotifyFinished(ImageResourceContent*) override;
  String DebugName() const override { return "SVGFEImageElement"; }

  FilterEffect* Build(SVGFilterBuilder*, Filter*) override;

  void ClearResourceReferences();
  void FetchImageResource();
  void ClearImageResource();

  void BuildPendingResource() override;
  InsertionNotificationRequest InsertedInto(ContainerNode&) override;
  void RemovedFrom(ContainerNode&) override;
  bool TaintsOrigin() const override;

  SVGAnimatedPropertyBase* PropertyFromAttribute(
      const QualifiedName& attribute_name) const override;
  void SynchronizeAllSVGAttributes() const override;

  Member<SVGAnimatedPreserveAspectRatio> preserve_aspect_ratio_;

  Member<ImageResourceContent> cached_image_;
  Member<IdTargetObserver> target_id_observer_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_FE_IMAGE_ELEMENT_H_
