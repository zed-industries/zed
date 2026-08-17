/*
 * Copyright (C) 2010 Nokia Corporation and/or its subsidiary(-ies).
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_METER_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_METER_ELEMENT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/html/html_element.h"

namespace blink {

class HTMLDivElement;

class CORE_EXPORT HTMLMeterElement final : public HTMLElement {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit HTMLMeterElement(Document&);
  ~HTMLMeterElement() override;

  enum GaugeRegion {
    kGaugeRegionOptimum,
    kGaugeRegionSuboptimal,
    kGaugeRegionEvenLessGood
  };

  double value() const;
  void setValue(double);

  double min() const;
  void setMin(double);

  double max() const;
  void setMax(double);

  double low() const;
  void setLow(double);

  double high() const;
  void setHigh(double);

  double optimum() const;
  void setOptimum(double);

  double ValueRatio() const;
  GaugeRegion GetGaugeRegion() const;

  bool CanContainRangeEndPoint() const override;

  void AdjustStyle(ComputedStyleBuilder& builder) override;

  void Trace(Visitor*) const override;

 private:
  bool AreAuthorShadowsAllowed() const override { return false; }

  bool IsLabelable() const override { return true; }

  LayoutObject* CreateLayoutObject(const ComputedStyle&) override;
  void DidRecalcStyle(const StyleRecalcChange) override;
  void ParseAttribute(const AttributeModificationParams&) override;

  void DidElementStateChange();
  void UpdateValueAppearance(double percentage);
  void DidAddUserAgentShadowRoot(ShadowRoot&) override;

  Member<HTMLDivElement> value_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_METER_ELEMENT_H_
