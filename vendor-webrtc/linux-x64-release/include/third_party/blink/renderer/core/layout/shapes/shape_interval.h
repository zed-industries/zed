/*
 * Copyright (C) 2012 Adobe Systems Incorporated. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1. Redistributions of source code must retain the above
 *    copyright notice, this list of conditions and the following
 *    disclaimer.
 * 2. Redistributions in binary form must reproduce the above
 *    copyright notice, this list of conditions and the following
 *    disclaimer in the documentation and/or other materials
 *    provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
 * FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
 * COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT,
 * INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
 * STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
 * OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SHAPES_SHAPE_INTERVAL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SHAPES_SHAPE_INTERVAL_H_

#include "base/check_op.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

template <typename T>
class ShapeInterval {
  USING_FAST_MALLOC(ShapeInterval);

 public:
  ShapeInterval() : x1_(-1), x2_(-2) {
    // The initial values of m_x1,x2 don't matter (unless you're looking
    // at them in the debugger) so long as isUndefined() is true.
    DCHECK(IsUndefined());
  }

  ShapeInterval(T x1, T x2) : x1_(x1), x2_(x2) { DCHECK_GE(x2, x1); }

  bool IsUndefined() const { return x2_ < x1_; }
  T X1() const { return IsUndefined() ? 0 : x1_; }
  T X2() const { return IsUndefined() ? 0 : x2_; }
  T Width() const { return IsUndefined() ? 0 : x2_ - x1_; }
  bool IsEmpty() const { return IsUndefined() || x1_ == x2_; }

  void Set(T x1, T x2) {
    DCHECK_GE(x2, x1);
    x1_ = x1;
    x2_ = x2;
  }

  bool Overlaps(const ShapeInterval<T>& interval) const {
    if (IsUndefined() || interval.IsUndefined())
      return false;
    return X2() >= interval.X1() && X1() <= interval.X2();
  }

  bool Contains(const ShapeInterval<T>& interval) const {
    if (IsUndefined() || interval.IsUndefined())
      return false;
    return X1() <= interval.X1() && X2() >= interval.X2();
  }

  bool operator==(const ShapeInterval<T>& other) const {
    return X1() == other.X1() && X2() == other.X2();
  }
  bool operator!=(const ShapeInterval<T>& other) const {
    return !operator==(other);
  }

  void Unite(const ShapeInterval<T>& interval) {
    if (interval.IsUndefined())
      return;
    if (IsUndefined())
      Set(interval.X1(), interval.X2());
    else
      Set(std::min<T>(X1(), interval.X1()), std::max<T>(X2(), interval.X2()));
  }

 private:
  T x1_;
  T x2_;
};

typedef ShapeInterval<int> IntShapeInterval;
typedef ShapeInterval<float> FloatShapeInterval;

typedef Vector<IntShapeInterval> IntShapeIntervals;
typedef Vector<FloatShapeInterval> FloatShapeIntervals;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SHAPES_SHAPE_INTERVAL_H_
