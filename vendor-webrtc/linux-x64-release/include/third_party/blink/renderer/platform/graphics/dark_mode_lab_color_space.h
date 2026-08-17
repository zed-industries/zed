#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_DARK_MODE_LAB_COLOR_SPACE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_DARK_MODE_LAB_COLOR_SPACE_H_

#include <algorithm>
#include <cmath>

#include "base/check.h"
#include "third_party/skia/include/core/SkM44.h"

namespace blink {

// TODO(prashant.n): Hide implementation details to .cc file.
// Namespace to handle color transformation between RGB and CIE L*a*b* color
// spaces.
namespace lab {

static constexpr SkV3 kIlluminantD50 = {0.964212f, 1.0f, 0.825188f};
static constexpr SkV3 kIlluminantD65 = {0.95042855f, 1.0f, 1.0889004f};

// All matrices here are 3x3 matrices.
// They are stored in SkM44 which is 4x4 matrix in the following form.
// |a b c 0|
// |d e f 0|
// |g h i 0|
// |0 0 0 1|

template <typename T>
inline constexpr T Clamp(T x, T min, T max) {
  return x < min ? min : x > max ? max : x;
}

// See https://en.wikipedia.org/wiki/Chromatic_adaptation#Von_Kries_transform.
inline SkM44 ChromaticAdaptation(const SkM44& matrix,
                                 const SkV3& src_white_point,
                                 const SkV3& dst_white_point) {
  SkV3 src_lms = matrix * src_white_point;
  SkV3 dst_lms = matrix * dst_white_point;
  // |lms| is a diagonal matrix stored as a float[3].
  SkV3 lms = {dst_lms.x / src_lms.x, dst_lms.y / src_lms.y,
              dst_lms.z / src_lms.z};
  SkM44 inverse;
  bool success = matrix.invert(&inverse);
  DCHECK(success);
  return inverse * (SkM44::Scale(lms.x, lms.y, lms.z) * matrix);
}

class DarkModeSRGBColorSpace {
 public:
  DarkModeSRGBColorSpace() {
    bool success = transform_.invert(&inverseTransform_);
    DCHECK(success);
  }

  SkV3 ToLinear(const SkV3& v) const {
    auto EOTF = [](float u) {
      return u < 0.04045f
                 ? Clamp(u / 12.92f, 0.0f, 1.0f)
                 : Clamp(std::pow((u + 0.055f) / 1.055f, 2.4f), 0.0f, 1.0f);
    };
    return {EOTF(v.x), EOTF(v.y), EOTF(v.z)};
  }

  SkV3 FromLinear(const SkV3& v) const {
    auto OETF = [](float u) {
      return (u < 0.0031308f ? Clamp(12.92f * u, 0.0f, 1.0f)
                             : Clamp(1.055f * std::pow(u, 1.0f / 2.4f) - 0.055f,
                                     0.0f, 1.0f));
    };
    return {OETF(v.x), OETF(v.y), OETF(v.z)};
  }

  // See https://en.wikipedia.org/wiki/SRGB#The_reverse_transformation.
  SkV3 ToXYZ(const SkV3& rgb) const { return transform_ * ToLinear(rgb); }

  // See
  // https://en.wikipedia.org/wiki/SRGB#The_forward_transformation_(CIE_XYZ_to_sRGB).
  SkV3 FromXYZ(const SkV3& xyz) const {
    return FromLinear(inverseTransform_ * xyz);
  }

 private:
  SkM44 kBradford = SkM44(0.8951f,
                          0.2664f,
                          -0.1614f,
                          0.0f,
                          -0.7502f,
                          1.7135f,
                          0.0367f,
                          0.0f,
                          0.0389f,
                          0.0685f,
                          1.0296f,
                          0.0f,
                          0.0f,
                          0.0f,
                          0.0f,
                          1.0f);

  SkM44 xyzTransform = SkM44(0.41238642f,
                             0.3575915f,
                             0.18045056f,
                             0.0f,
                             0.21263677f,
                             0.715183f,
                             0.07218022f,
                             0.0f,
                             0.019330615f,
                             0.11919712f,
                             0.95037293f,
                             0.0f,
                             0.0f,
                             0.0f,
                             0.0f,
                             1.0f);

  SkM44 transform_ =
      ChromaticAdaptation(kBradford, kIlluminantD65, kIlluminantD50) *
      xyzTransform;
  SkM44 inverseTransform_;
};

class DarkModeLABColorSpace {
 public:
  // See
  // https://en.wikipedia.org/wiki/CIELAB_color_space#Reverse_transformation.
  SkV3 FromXYZ(const SkV3& v) const {
    auto f = [](float x) {
      return x > kSigma3 ? std::cbrt(x) : x / (3 * kSigma2) + 4.0f / 29.0f;
    };

    float fx = f(v.x / kIlluminantD50.x);
    float fy = f(v.y / kIlluminantD50.y);
    float fz = f(v.z / kIlluminantD50.z);

    float L = 116.0f * fy - 16.0f;
    float a = 500.0f * (fx - fy);
    float b = 200.0f * (fy - fz);

    return {Clamp(L, 0.0f, 100.0f), Clamp(a, -128.0f, 128.0f),
            Clamp(b, -128.0f, 128.0f)};
  }

  // See
  // https://en.wikipedia.org/wiki/CIELAB_color_space#Forward_transformation.
  SkV3 ToXYZ(const SkV3& lab) const {
    auto invf = [](float x) {
      return x > kSigma ? x * x * x : 3.0f * kSigma2 * (x - 4.0f / 29.0f);
    };

    SkV3 v = {Clamp(lab.x, 0.0f, 100.0f), Clamp(lab.y, -128.0f, 128.0f),
              Clamp(lab.z, -128.0f, 128.0f)};

    return {invf((v.x + 16.0f) / 116.0f + (v.y * 0.002f)) * kIlluminantD50.x,
            invf((v.x + 16.0f) / 116.0f) * kIlluminantD50.y,
            invf((v.x + 16.0f) / 116.0f - (v.z * 0.005f)) * kIlluminantD50.z};
  }

 private:
  static const constexpr float kSigma = 6.0f / 29.0f;
  static const constexpr float kSigma2 = 36.0f / 841.0f;
  static const constexpr float kSigma3 = 216.0f / 24389.0f;
};

class DarkModeSRGBLABTransformer {
 public:
  SkV3 SRGBToLAB(const SkV3& rgb) const {
    SkV3 xyz = srgb_space_.ToXYZ(rgb);
    return lab_space_.FromXYZ(xyz);
  }

  SkV3 LABToSRGB(const SkV3& lab) const {
    SkV3 xyz = lab_space_.ToXYZ(lab);
    return srgb_space_.FromXYZ(xyz);
  }

 private:
  DarkModeSRGBColorSpace srgb_space_ = DarkModeSRGBColorSpace();
  DarkModeLABColorSpace lab_space_ = DarkModeLABColorSpace();
};

}  // namespace lab

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_DARK_MODE_LAB_COLOR_SPACE_H_
