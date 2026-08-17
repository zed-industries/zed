/*
 * Copyright (C) 2011 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_CALCULATION_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_CALCULATION_VALUE_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/platform/geometry/length.h"
#include "third_party/blink/renderer/platform/geometry/length_functions.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/ref_counted.h"

namespace blink {

class CalculationExpressionNode;

class PLATFORM_EXPORT CalculationValue : public RefCounted<CalculationValue> {
  USING_FAST_MALLOC(CalculationValue);

 public:
  static scoped_refptr<const CalculationValue> Create(
      PixelsAndPercent value,
      Length::ValueRange range) {
    return base::AdoptRef(new CalculationValue(value, range));
  }

  // If |expression| simply wraps a |PixelsAndPercent| value, this function
  // takes that value directly and discards |expression|.
  static scoped_refptr<const CalculationValue> CreateSimplified(
      scoped_refptr<const CalculationExpressionNode> expression,
      Length::ValueRange range);

  ~CalculationValue();

  float Evaluate(float max_value, const EvaluationInput& = {}) const;
  bool operator==(const CalculationValue& o) const;
  bool IsExpression() const { return is_expression_; }
  bool IsNonNegative() const { return is_non_negative_; }
  Length::ValueRange GetValueRange() const {
    return is_non_negative_ ? Length::ValueRange::kNonNegative
                            : Length::ValueRange::kAll;
  }
  bool HasAuto() const;
  bool HasContentOrIntrinsicSize() const;
  bool HasAutoOrContentOrIntrinsicSize() const;
  bool HasPercent() const;
  bool HasPercentOrStretch() const;
  bool HasStretch() const;

  bool HasMinContent() const;
  bool HasMaxContent() const;
  bool HasFitContent() const;

  bool HasOnlyFixedAndPercent() const;

  float Pixels() const {
    DCHECK(!IsExpression());
    return data_.value.pixels;
  }
  float Percent() const {
    DCHECK(!IsExpression());
    return data_.value.percent;
  }
  PixelsAndPercent GetPixelsAndPercent() const {
    DCHECK(!IsExpression());
    return data_.value;
  }
  bool HasExplicitPixels() const {
    DCHECK(!IsExpression());
    return data_.value.has_explicit_pixels;
  }
  bool HasExplicitPercent() const {
    DCHECK(!IsExpression());
    return data_.value.has_explicit_percent;
  }

  // If |this| is an expression, returns the underlying expression. Otherwise,
  // creates one from the underlying |PixelsAndPercent| value.
  scoped_refptr<const CalculationExpressionNode> GetOrCreateExpression() const;

  scoped_refptr<const CalculationValue> Blend(const CalculationValue& from,
                                              double progress,
                                              Length::ValueRange) const;
  scoped_refptr<const CalculationValue> SubtractFromOneHundredPercent() const;
  scoped_refptr<const CalculationValue> Add(const CalculationValue&) const;
  scoped_refptr<const CalculationValue> Zoom(double factor) const;

 private:
  CalculationValue(PixelsAndPercent value, Length::ValueRange range)
      : data_(value),
        is_expression_(false),
        is_non_negative_(range == Length::ValueRange::kNonNegative) {}

  CalculationValue(scoped_refptr<const CalculationExpressionNode> expression,
                   Length::ValueRange range);

  union DataUnion {
    explicit DataUnion(PixelsAndPercent value) : value(value) {}
    explicit DataUnion(
        scoped_refptr<const CalculationExpressionNode> expression);
    ~DataUnion();

    PixelsAndPercent value;
    scoped_refptr<const CalculationExpressionNode> expression;
  } data_;
  unsigned is_expression_ : 1;
  unsigned is_non_negative_ : 1;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_CALCULATION_VALUE_H_
