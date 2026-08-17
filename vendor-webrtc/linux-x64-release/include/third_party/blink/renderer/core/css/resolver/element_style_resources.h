/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 * Copyright (C) 2003, 2004, 2005, 2006, 2007, 2008, 2009, 2010, 2011 Apple Inc.
 * All rights reserved.
 * Copyright (C) 2013 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_ELEMENT_STYLE_RESOURCES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_ELEMENT_STYLE_RESOURCES_H_

#include "third_party/blink/renderer/core/css/css_property_names.h"
#include "third_party/blink/renderer/core/css/css_to_length_conversion_data.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"

namespace blink {

class CSSValue;
class ComputedStyleBuilder;
class Element;
class SVGResource;
class StyleImage;

namespace cssvalue {
class CSSURIValue;
}

class PreCachedContainerSizes {
  STACK_ALLOCATED();

 public:
  using ContainerSizes = CSSToLengthConversionData::ContainerSizes;

  PreCachedContainerSizes() = default;
  explicit PreCachedContainerSizes(
      const CSSToLengthConversionData* conversion_data)
      : conversion_data_(conversion_data) {}

  const ContainerSizes& Get() const;

 private:
  const CSSToLengthConversionData* conversion_data_{nullptr};
  mutable std::optional<ContainerSizes> cache_;
};

// Holds information about resources, requested by stylesheets.
// Lifetime: per-element style resolve.
class ElementStyleResources {
  STACK_ALLOCATED();

 public:
  ElementStyleResources(Element&, float device_scale_factor);
  ElementStyleResources(const ElementStyleResources&) = delete;
  ElementStyleResources& operator=(const ElementStyleResources&) = delete;

  StyleImage* GetStyleImage(CSSPropertyID, const CSSValue&);

  SVGResource* GetSVGResourceFromValue(CSSPropertyID,
                                       const cssvalue::CSSURIValue&);

  void LoadPendingResources(ComputedStyleBuilder&, const CSSLengthResolver&);

  void UpdateLengthConversionData(const CSSToLengthConversionData*);

 private:
  bool IsPending(const CSSValue&) const;
  StyleImage* CachedStyleImage(const CSSValue&) const;

  StyleImage* LoadMaskSource(CSSValue&);

  void LoadPendingSVGResources(ComputedStyleBuilder&);
  void LoadPendingImages(ComputedStyleBuilder&, const CSSLengthResolver&);

  Element& element_;
  HashSet<CSSPropertyID> pending_image_properties_;
  HashSet<CSSPropertyID> pending_svg_resource_properties_;
  float device_scale_factor_;
  PreCachedContainerSizes pre_cached_container_sizes_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_ELEMENT_STYLE_RESOURCES_H_
