/*
 * Copyright (C) 2004, 2005, 2008 Nikolas Zimmermann <zimmermann@kde.org>
 * Copyright (C) 2004, 2005, 2006, 2007 Rob Buis <buis@kde.org>
 * Copyright (C) 2014 Samsung Electronics. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_ZOOM_AND_PAN_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_ZOOM_AND_PAN_H_

#include "third_party/blink/renderer/core/dom/qualified_name.h"
#include "third_party/blink/renderer/core/svg_names.h"

namespace blink {

class ExceptionState;

enum SVGZoomAndPanType {
  kSVGZoomAndPanUnknown = 0,
  kSVGZoomAndPanDisable,
  kSVGZoomAndPanMagnify
};

class SVGZoomAndPan {
 public:
  // Forward declare enumerations in the W3C naming scheme, for IDL generation.
  enum {
    kSvgZoomandpanUnknown = kSVGZoomAndPanUnknown,
    kSvgZoomandpanDisable = kSVGZoomAndPanDisable,
    kSvgZoomandpanMagnify = kSVGZoomAndPanMagnify
  };

  virtual ~SVGZoomAndPan() = default;

  static bool IsKnownAttribute(const QualifiedName&);

  static SVGZoomAndPanType ParseFromNumber(uint16_t number) {
    if (!number || number > kSVGZoomAndPanMagnify)
      return kSVGZoomAndPanUnknown;
    return static_cast<SVGZoomAndPanType>(number);
  }

  static SVGZoomAndPanType Parse(const LChar*& start, const LChar* end);
  static SVGZoomAndPanType Parse(const UChar*& start, const UChar* end);

  bool ParseAttribute(const QualifiedName& name, const AtomicString& value);

  // JS API
  SVGZoomAndPanType zoomAndPan() const { return zoom_and_pan_; }
  virtual void setZoomAndPan(uint16_t value) {
    zoom_and_pan_ = ParseFromNumber(value);
  }
  virtual void setZoomAndPan(uint16_t value, ExceptionState&) {
    setZoomAndPan(value);
  }

 protected:
  SVGZoomAndPan();

 private:
  SVGZoomAndPanType zoom_and_pan_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_ZOOM_AND_PAN_H_
