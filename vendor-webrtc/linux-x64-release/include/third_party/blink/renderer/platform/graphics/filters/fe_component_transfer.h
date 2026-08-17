/*
 * Copyright (C) 2004, 2005, 2006, 2007 Nikolas Zimmermann <zimmermann@kde.org>
 * Copyright (C) 2004, 2005 Rob Buis <buis@kde.org>
 * Copyright (C) 2005 Eric Seidel <eric@webkit.org>
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
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_FILTERS_FE_COMPONENT_TRANSFER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_FILTERS_FE_COMPONENT_TRANSFER_H_

#include "base/containers/span.h"
#include "third_party/blink/renderer/platform/graphics/filters/filter_effect.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

enum ComponentTransferType {
  FECOMPONENTTRANSFER_TYPE_UNKNOWN = 0,
  FECOMPONENTTRANSFER_TYPE_IDENTITY = 1,
  FECOMPONENTTRANSFER_TYPE_TABLE = 2,
  FECOMPONENTTRANSFER_TYPE_DISCRETE = 3,
  FECOMPONENTTRANSFER_TYPE_LINEAR = 4,
  FECOMPONENTTRANSFER_TYPE_GAMMA = 5
};

struct ComponentTransferFunction {
  DISALLOW_NEW();
  ComponentTransferFunction()
      : type(FECOMPONENTTRANSFER_TYPE_UNKNOWN),
        slope(0),
        intercept(0),
        amplitude(0),
        exponent(0),
        offset(0) {}

  ComponentTransferType type;

  bool operator==(const ComponentTransferFunction& o) const {
    if (type != o.type)
      return false;
    switch (type) {
      case FECOMPONENTTRANSFER_TYPE_IDENTITY:
        return true;
      case FECOMPONENTTRANSFER_TYPE_LINEAR:
        return (slope == o.slope && intercept == o.intercept);
      case FECOMPONENTTRANSFER_TYPE_GAMMA:
        return (amplitude == o.amplitude && exponent == o.exponent &&
                offset == o.offset);
      case FECOMPONENTTRANSFER_TYPE_TABLE:
      case FECOMPONENTTRANSFER_TYPE_DISCRETE:
        return (table_values == o.table_values);
      case FECOMPONENTTRANSFER_TYPE_UNKNOWN:
        return false;
    }
  }

  float slope;
  float intercept;
  float amplitude;
  float exponent;
  float offset;

  Vector<float> table_values;
};

using ComponentTransferLutType = base::span<uint8_t, 256>;

class PLATFORM_EXPORT FEComponentTransfer final : public FilterEffect {
 public:
  FEComponentTransfer(Filter*,
                      const ComponentTransferFunction& red_func,
                      const ComponentTransferFunction& green_func,
                      const ComponentTransferFunction& blue_func,
                      const ComponentTransferFunction& alpha_func);

  StringBuilder& ExternalRepresentation(StringBuilder&,
                                        wtf_size_t indent) const override;

 private:
  sk_sp<PaintFilter> CreateImageFilter() override;

  bool AffectsTransparentPixels() const override;

  void GetValues(ComponentTransferLutType r_values,
                 ComponentTransferLutType g_values,
                 ComponentTransferLutType b_values,
                 ComponentTransferLutType a_values);

  ComponentTransferFunction red_func_;
  ComponentTransferFunction green_func_;
  ComponentTransferFunction blue_func_;
  ComponentTransferFunction alpha_func_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_FILTERS_FE_COMPONENT_TRANSFER_H_
