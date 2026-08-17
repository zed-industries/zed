/*
 * Copyright (C) 2006, 2007, 2008, 2009, 2010 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_STEP_RANGE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_STEP_RANGE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/decimal.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

enum AnyStepHandling { kRejectAny, kAnyIsDefaultStep };

class CORE_EXPORT StepRange {
  DISALLOW_NEW();

 public:
  enum StepValueShouldBe {
    kStepValueShouldBeReal,
    kParsedStepValueShouldBeInteger,
    kScaledStepValueShouldBeInteger,
  };

  struct StepDescription {
    USING_FAST_MALLOC(StepDescription);

   public:
    int default_step;
    int default_step_base;
    int step_scale_factor;
    StepValueShouldBe step_value_should_be;

    StepDescription(
        int default_step,
        int default_step_base,
        int step_scale_factor,
        StepValueShouldBe step_value_should_be = kStepValueShouldBeReal)
        : default_step(default_step),
          default_step_base(default_step_base),
          step_scale_factor(step_scale_factor),
          step_value_should_be(step_value_should_be) {}

    StepDescription()
        : default_step(1),
          default_step_base(0),
          step_scale_factor(1),
          step_value_should_be(kStepValueShouldBeReal) {}

    Decimal DefaultValue() const { return default_step * step_scale_factor; }
  };

  StepRange();
  StepRange(const StepRange&);
  StepRange(const Decimal& step_base,
            const Decimal& minimum,
            const Decimal& maximum,
            bool has_range_limitations,
            bool supports_reversed_range,
            const Decimal& step,
            const StepDescription&);

  Decimal AlignValueForStep(const Decimal& current_value,
                            const Decimal& new_value) const;
  Decimal ClampValue(const Decimal& value) const;
  bool HasStep() const { return has_step_; }
  Decimal Maximum() const { return maximum_; }
  Decimal Minimum() const { return minimum_; }
  // https://html.spec.whatwg.org/C/#have-range-limitations
  bool HasRangeLimitations() const { return has_range_limitations_; }
  // https://html.spec.whatwg.org/C/#has-a-reversed-range
  bool HasReversedRange() const;
  static Decimal ParseStep(AnyStepHandling,
                           const StepDescription&,
                           const String&);
  Decimal Step() const { return step_; }
  Decimal StepBase() const { return step_base_; }
  bool StepMismatch(const Decimal&) const;
  // Returns the maximum step-matched value between minimum() and
  // maximum(). If there's no such value, this returns Decimal::nan().
  Decimal StepSnappedMaximum() const;

  // Clamp the middle value according to the step
  Decimal DefaultValue() const { return ClampValue((minimum_ + maximum_) / 2); }

  // Map value into 0-1 range
  Decimal ProportionFromValue(const Decimal& value) const {
    if (minimum_ == maximum_)
      return 0;

    return (value - minimum_) / (maximum_ - minimum_);
  }

  // Map from 0-1 range to value
  Decimal ValueFromProportion(const Decimal& proportion) const {
    return minimum_ + proportion * (maximum_ - minimum_);
  }

 private:
  StepRange& operator=(const StepRange&) = delete;
  Decimal AcceptableError() const;
  Decimal RoundByStep(const Decimal& value, const Decimal& base) const;

  const Decimal maximum_;  // maximum must be >= minimum.
  const Decimal minimum_;
  const Decimal step_;
  const Decimal step_base_;
  const StepDescription step_description_;
  const bool has_step_;
  const bool has_range_limitations_;
  const bool supports_reversed_range_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_STEP_RANGE_H_
