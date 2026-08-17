/*
    Copyright (C) 2004, 2005, 2007 Nikolas Zimmermann <zimmermann@kde.org>
                  2004, 2005 Rob Buis <buis@kde.org>
    Copyright (C) Research In Motion Limited 2010. All rights reserved.

    Based on khtml code by:
    Copyright (C) 2000-2003 Lars Knoll (knoll@kde.org)
              (C) 2000 Antti Koivisto (koivisto@kde.org)
              (C) 2000-2003 Dirk Mueller (mueller@kde.org)
              (C) 2002-2003 Apple Computer, Inc.

    This library is free software; you can redistribute it and/or
    modify it under the terms of the GNU Library General Public
    License as published by the Free Software Foundation; either
    version 2 of the License, or (at your option) any later version.

    This library is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
    Library General Public License for more details.

    You should have received a copy of the GNU Library General Public License
    along with this library; see the file COPYING.LIB.  If not, write to
    the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
    Boston, MA 02110-1301, USA.
*/

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_SVG_PAINT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_SVG_PAINT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/style_color.h"
#include "third_party/blink/renderer/core/style/style_svg_resource.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

enum class SVGPaintType {
  kColor,
  kNone,
  kContextFill,
  kContextStroke,
  kUriNone,
  kUriColor,
  kUri
};

struct SVGPaint {
  DISALLOW_NEW();

 public:
  CORE_EXPORT SVGPaint();
  explicit SVGPaint(Color color);
  SVGPaint(const SVGPaint& paint);
  CORE_EXPORT ~SVGPaint();
  CORE_EXPORT SVGPaint& operator=(const SVGPaint& paint);

  void Trace(Visitor* visitor) const {
    visitor->Trace(color);
    visitor->Trace(resource);
  }

  CORE_EXPORT bool operator==(const SVGPaint&) const;
  bool operator!=(const SVGPaint& other) const { return !(*this == other); }

  bool IsNone() const { return type == SVGPaintType::kNone; }
  bool IsColor() const { return type == SVGPaintType::kColor; }
  // Used by CSSPropertyEquality::PropertiesEqual.
  bool EqualTypeOrColor(const SVGPaint& other) const {
    return type == other.type &&
           (type != SVGPaintType::kColor || color == other.color);
  }
  bool HasColor() const { return IsColor() || type == SVGPaintType::kUriColor; }
  bool HasUrl() const { return type >= SVGPaintType::kUriNone; }
  bool HasCurrentColor() const { return HasColor() && color.IsCurrentColor(); }
  StyleSVGResource* Resource() const { return resource.Get(); }

  const StyleColor& GetColor() const { return color; }
  const AtomicString& GetUrl() const;

  StyleColor color;
  Member<StyleSVGResource> resource{nullptr};
  SVGPaintType type{SVGPaintType::kNone};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_SVG_PAINT_H_
