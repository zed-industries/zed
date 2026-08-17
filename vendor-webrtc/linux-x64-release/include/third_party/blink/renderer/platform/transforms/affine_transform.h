/*
 * Copyright (C) 2005, 2006 Apple Computer, Inc.  All rights reserved.
 *               2010 Dirk Schulze <krit@webkit.org>
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TRANSFORMS_AFFINE_TRANSFORM_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TRANSFORMS_AFFINE_TRANSFORM_H_

#include <string.h>  // for memcpy
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/math_extras.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "ui/gfx/geometry/double4.h"

namespace gfx {
class PointF;
class QuadF;
class Rect;
class RectF;
class Transform;
}  // namespace gfx

class SkMatrix;
class SkM44;

namespace blink {

class PLATFORM_EXPORT AffineTransform {
  DISALLOW_NEW();

 public:
  constexpr AffineTransform() : transform_{1, 0, 0, 1, 0, 0} {}
  constexpr AffineTransform(double a,
                            double b,
                            double c,
                            double d,
                            double e,
                            double f)
      : transform_{a, b, c, d, e, f} {}

  void SetMatrix(double a, double b, double c, double d, double e, double f) {
    *this = AffineTransform(a, b, c, d, e, f);
  }

  [[nodiscard]] gfx::PointF MapPoint(const gfx::PointF&) const;

  // Rounds the resulting mapped rectangle out. This is helpful for bounding
  // box computations but may not be what is wanted in other contexts.
  [[nodiscard]] gfx::Rect MapRect(const gfx::Rect&) const;

  [[nodiscard]] gfx::RectF MapRect(const gfx::RectF&) const;
  [[nodiscard]] gfx::QuadF MapQuad(const gfx::QuadF&) const;

  bool IsIdentity() const {
    return gfx::AllTrue(
        gfx::LoadDouble4(transform_) == gfx::Double4{1, 0, 0, 1} &
        gfx::LoadDouble4(&transform_[2]) == gfx::Double4{0, 1, 0, 0});
  }

  bool IsIdentityOrTranslation() const {
    return gfx::AllTrue(gfx::LoadDouble4(transform_) ==
                        gfx::Double4{1, 0, 0, 1});
  }

  double A() const { return transform_[0]; }
  void SetA(double a) { transform_[0] = a; }
  double B() const { return transform_[1]; }
  void SetB(double b) { transform_[1] = b; }
  double C() const { return transform_[2]; }
  void SetC(double c) { transform_[2] = c; }
  double D() const { return transform_[3]; }
  void SetD(double d) { transform_[3] = d; }
  double E() const { return transform_[4]; }
  void SetE(double e) { transform_[4] = e; }
  double F() const { return transform_[5]; }
  void SetF(double f) { transform_[5] = f; }

  void MakeIdentity() { *this = AffineTransform(); }

  // this' = this * other
  AffineTransform& PreConcat(const AffineTransform& other);
  // this' = other * this
  AffineTransform& PostConcat(const AffineTransform& other);

  // The semantics of the following methods are the same as PreConcat(), i.e.
  // this' = this * operation.
  AffineTransform& Scale(double);
  AffineTransform& Scale(double sx, double sy);
  AffineTransform& ScaleNonUniform(double sx, double sy);
  AffineTransform& Rotate(double a);
  AffineTransform& RotateRadians(double a);
  AffineTransform& RotateFromVector(double x, double y);
  AffineTransform& Translate(double tx, double ty);
  AffineTransform& Shear(double sx, double sy);
  AffineTransform& FlipX();
  AffineTransform& FlipY();
  AffineTransform& Skew(double angle_x, double angle_y);
  AffineTransform& SkewX(double angle);
  AffineTransform& SkewY(double angle);

  double XScaleSquared() const;
  double XScale() const;
  double YScaleSquared() const;
  double YScale() const;

  double Det() const;
  bool IsInvertible() const;
  [[nodiscard]] AffineTransform Inverse() const;

  // Creates an AffineTransform by extracting affine components from
  // gfx::Transform and ignoring other components.
  [[nodiscard]] static AffineTransform FromTransform(const gfx::Transform&);

  [[nodiscard]] gfx::Transform ToTransform() const;
  [[nodiscard]] SkMatrix ToSkMatrix() const;
  [[nodiscard]] SkM44 ToSkM44() const;

  bool operator==(const AffineTransform& m2) const {
    return gfx::AllTrue(gfx::LoadDouble4(transform_) ==
                            gfx::LoadDouble4(m2.transform_) &
                        gfx::LoadDouble4(&transform_[2]) ==
                            gfx::LoadDouble4(&m2.transform_[2]));
  }

  bool operator!=(const AffineTransform& other) const {
    return !(*this == other);
  }

  // *this = *this * t (i.e., a multRight)
  AffineTransform& operator*=(const AffineTransform& t) { return PreConcat(t); }

  // result = *this * t (i.e., a multRight)
  AffineTransform operator*(const AffineTransform& t) const {
    AffineTransform result = *this;
    result *= t;
    return result;
  }

  [[nodiscard]] static constexpr AffineTransform MakeSkewX(double angle) {
    return AffineTransform(1, 0, std::tan(Deg2rad(angle)), 1, 0, 0);
  }
  [[nodiscard]] static constexpr AffineTransform MakeSkewY(double angle) {
    return AffineTransform(1, std::tan(Deg2rad(angle)), 0, 1, 0, 0);
  }
  [[nodiscard]] static constexpr AffineTransform Translation(double x,
                                                             double y) {
    return AffineTransform(1, 0, 0, 1, x, y);
  }
  [[nodiscard]] static constexpr AffineTransform MakeScale(double s) {
    return MakeScaleNonUniform(s, s);
  }
  [[nodiscard]] static constexpr AffineTransform MakeScaleNonUniform(
      double sx,
      double sy) {
    return AffineTransform(sx, 0, 0, sy, 0, 0);
  }
  [[nodiscard]] static AffineTransform MakeRotationAroundPoint(double angle,
                                                               double cx,
                                                               double cy) {
    AffineTransform result = Translation(cx, cy);
    result.Rotate(angle);
    result.Translate(-cx, -cy);
    return result;
  }

  // The 2d version of gfx::Transform::Zoom().
  AffineTransform& Zoom(double zoom_factor);

  // If |as_matrix| is true, the transform is returned as a matrix in row-major
  // order. Otherwise, the transform's decomposition is returned which shows
  // the translation, scale, etc.
  String ToString(bool as_matrix = false) const;

 private:
  static float ClampToFloat(double value) {
    return ClampToWithNaNTo0<float>(value);
  }

  double transform_[6];
};

PLATFORM_EXPORT std::ostream& operator<<(std::ostream&, const AffineTransform&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TRANSFORMS_AFFINE_TRANSFORM_H_
