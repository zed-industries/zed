/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 * Copyright (C) 2004, 2008, 2009, 2011 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_AREA_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_AREA_ELEMENT_H_

#include <memory>

#include "third_party/blink/public/mojom/input/focus_type.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/html/html_anchor_element.h"
#include "third_party/blink/renderer/core/layout/geometry/physical_rect.h"
#include "ui/gfx/geometry/vector2d_f.h"

namespace blink {

class HTMLImageElement;
class Path;

class CORE_EXPORT HTMLAreaElement final : public HTMLAnchorElementBase {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit HTMLAreaElement(Document&);
  ~HTMLAreaElement() override;

  bool IsDefault() const { return shape_ == kDefault; }

  // |containerObject| in the following functions is an object (normally a
  // LayoutImage) which references the containing image map of this area. There
  // might be multiple objects referencing the same map. For these functions,
  // the effective geometry of this map will be calculated based on the
  // specified container object, e.g.  the rectangle of the default shape will
  // be the border box rect of the container object, and effective zoom factor
  // of the container object will be applied on non-default shape.
  bool PointInArea(const PhysicalOffset&,
                   const LayoutObject* container_object) const;
  PhysicalRect ComputeAbsoluteRect(const LayoutObject* container_object) const;
  Path GetPath(const LayoutObject* container_object,
               const gfx::Vector2dF& path_offset = gfx::Vector2dF()) const;

  // The parent map's image.
  HTMLImageElement* ImageElement() const;

 private:
  void ParseAttribute(const AttributeModificationParams&) override;
  bool IsKeyboardFocusableSlow(
      UpdateBehavior update_behavior =
          UpdateBehavior::kStyleAndLayout) const override;
  FocusableState IsFocusableState(
      UpdateBehavior update_behavior) const override;
  bool IsFocusableStyle(UpdateBehavior update_behavior =
                            UpdateBehavior::kStyleAndLayout) const override;
  void UpdateSelectionOnFocus(SelectionBehaviorOnFocus,
                              const FocusOptions*) override;
  void SetFocused(bool, mojom::blink::FocusType) override;

  enum Shape { kDefault, kPoly, kRect, kCircle };
  void InvalidateCachedPath();

  mutable std::unique_ptr<Path> path_;
  Vector<double> coords_;
  Shape shape_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_AREA_ELEMENT_H_
