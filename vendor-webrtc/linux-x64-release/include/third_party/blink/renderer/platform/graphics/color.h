/*
 * Copyright (C) 2003, 2004, 2005, 2006, 2010 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_COLOR_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_COLOR_H_

#include <array>
#include <iosfwd>
#include <optional>
#include <string_view>
#include <tuple>

#include "base/containers/span.h"
#include "base/gtest_prod_util.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/hash_functions.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_uchar.h"
#include "third_party/skia/include/core/SkColor.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

typedef unsigned RGBA32;  // RGBA quadruplet

struct NamedColor {
  DISALLOW_NEW();
  const char* name;
  unsigned argb_value;
};

PLATFORM_EXPORT const NamedColor* FindColor(std::string_view str);

class PLATFORM_EXPORT Color {
  DISALLOW_NEW();

 public:
  // This enum represent the color space of the color itself. This is also used
  // for serialization purposes and for initialization. Don't change the order
  // of this enum, as how it's ordered helps performance (the compiler can
  // decide that the first few elements are for ColorFunctionSpace and the last
  // few elements are for RGB-like serialization.)
  // For details on serialization, see:
  // https://www.w3.org/TR/css-color-4/#serializing-color-values
  // https://www.w3.org/TR/css-color-5/#serial-relative-color
  enum class ColorSpace : uint8_t {
    // All these are to be serialized with the color() syntax of a given
    // predefined color space. The
    // values of `params0_`, `params1_`, and `params2_` are red, green, and blue
    // values in the color space specified by `color_space_`.
    kSRGB,
    kSRGBLinear,
    kDisplayP3,
    kA98RGB,
    kProPhotoRGB,
    kRec2020,
    kXYZD50,
    kXYZD65,
    // Serializes to lab(). The value of `param0_` is lightness and is
    // guaranteed to be non-negative. The value of `param1_` and `param2_` are
    // the a-axis and b-axis values and are unbounded.
    kLab,
    // Serializes to oklab(). Parameter meanings are the same as for kLab.
    kOklab,
    // Serializes to lch(). The value of `param0_` is lightness and is
    // guaranteed to be non-negative. The value of `param1_` is chroma and is
    // also guaranteed to be non-negative. The value of `param2_` is hue, and
    // is unbounded.
    kLch,
    // Serializes to oklch(). Parameter meanings are the same as for kLCH.
    kOklch,
    // Serializes to rgb() or rgba().
    // The values of `params0_`, `params1_`, and `params2_` are red, green, and
    // blue sRGB values, and are guaranteed to be present and in the [0, 1]
    // interval.
    kSRGBLegacy,
    // Serializes to rgb() or rgba() for non-relative colors and to hsl() for
    // unresolved relative colors.
    // The values of `params0_`, `params1_`, and `params2_` are Hue, Saturation,
    // and Ligthness. These can be none. Hue is a namber in the range from 0.0
    // to 6.0, and the rest are in the rance from 0.0 to 1.0.
    // interval.
    kHSL,
    // Serializes to rgb() or rgba() for non-relative colors and to hwb() for
    // unresolved relative colors.
    // The values of `params0_`, `params1_`, and `params2_` are Hue, White,
    // and Black. These can be none. Hue is a namber in the range from 0.0
    // to 6.0, and the rest are in the rance from 0.0 to 1.0.
    // interval.
    kHWB,
    // An uninitialized color.
    kNone,
  };

  // For testing purposes and for serializer.
  static WTF::String ColorSpaceToString(Color::ColorSpace color_space);

  // https://www.w3.org/TR/css-color-4/#predefined
  static bool IsPredefinedColorSpace(ColorSpace color_space) {
    return color_space == ColorSpace::kSRGB ||
           color_space == ColorSpace::kSRGBLinear ||
           color_space == ColorSpace::kDisplayP3 ||
           color_space == ColorSpace::kA98RGB ||
           color_space == ColorSpace::kProPhotoRGB ||
           color_space == ColorSpace::kRec2020 ||
           color_space == ColorSpace::kXYZD50 ||
           color_space == ColorSpace::kXYZD65;
  }

  static bool IsLightnessFirstComponent(ColorSpace color_space) {
    return color_space == ColorSpace::kLab ||
           color_space == ColorSpace::kOklab ||
           color_space == ColorSpace::kLch || color_space == ColorSpace::kOklch;
  }

  static bool IsChromaSecondComponent(ColorSpace color_space) {
    return color_space == ColorSpace::kLch || color_space == ColorSpace::kOklch;
  }

  // https://www.w3.org/TR/css-color-4/#legacy-color-syntax
  // Returns true if the color is of a type that predates CSS Color 4. Includes
  // rgb(), rgba(), hex color, named color, hsl() and hwb() types. These colors
  // interpolate and serialize differently from other color types.
  static bool IsLegacyColorSpace(ColorSpace color_space) {
    return color_space == ColorSpace::kSRGBLegacy ||
           color_space == ColorSpace::kHSL || color_space == ColorSpace::kHWB;
  }

  static bool ColorSpaceHasHue(ColorSpace color_space) {
    return color_space == Color::ColorSpace::kLch ||
           color_space == Color::ColorSpace::kOklch ||
           color_space == Color::ColorSpace::kHSL ||
           color_space == Color::ColorSpace::kHWB;
  }

  // The default constructor creates a transparent color.
  constexpr Color()
      : param0_is_none_(0),
        param1_is_none_(0),
        param2_is_none_(0),
        alpha_is_none_(0) {}

  // TODO(crbug.com/ 1333988): We have to reevaluate how we input int RGB and
  // RGBA values into blink::color. We should remove the int inputs in the
  // interface, to avoid callers to have the double values and convert them to
  // int for then being converted again internally to float. We should deprecate
  // FromRGB, FromRGBA and FromRGBAFloat methods, to only allow for
  // FromRGBALegacy. We could also merge all the constructor methods to one
  // CreateColor(colorSpace, components...) method, that will internally create
  // methods depending of the color space and properly store the none-ness of
  // the components.

  Color(int r, int g, int b);
  Color(int r, int g, int b, int a);

  // Create a color using rgb() syntax.
  static constexpr Color FromRGB(int r, int g, int b) {
    return Color(0xFF000000 | ClampInt255(r) << 16 | ClampInt255(g) << 8 |
                 ClampInt255(b));
  }

  // Create a color using rgba() syntax.
  static constexpr Color FromRGBA(int r, int g, int b, int a) {
    return Color(ClampInt255(a) << 24 | ClampInt255(r) << 16 |
                 ClampInt255(g) << 8 | ClampInt255(b));
  }

  // Create a color using the rgba() syntax, with float arguments. All
  // parameters will be clamped to the [0, 1] interval.
  static constexpr Color FromRGBAFloat(float r, float g, float b, float a) {
    return Color(SkColor4f{r, g, b, a});
  }

  // Create a color from a generic color space. Parameters that are none should
  // be specified as std::nullopt. The value for `alpha` will be clamped to the
  // [0, 1] interval. For colorspaces with Luminance the first channel will be
  // clamped to be non-negative. For colorspaces with chroma in param1 that
  // parameter will also be clamped to be non-negative.
  static Color FromColorSpace(ColorSpace space,
                              std::optional<float> param0,
                              std::optional<float> param1,
                              std::optional<float> param2,
                              std::optional<float> alpha);
  static Color FromColorSpace(ColorSpace space,
                              std::optional<float> param0,
                              std::optional<float> param1,
                              std::optional<float> param2) {
    return FromColorSpace(space, param0, param1, param2, 1.0f);
  }

  // Create a color using the hsl() syntax.
  static Color FromHSLA(std::optional<float> h,
                        std::optional<float> s,
                        std::optional<float> l,
                        std::optional<float> a);

  // Create a color using the hwb() syntax.
  static Color FromHWBA(std::optional<float> h,
                        std::optional<float> w,
                        std::optional<float> b,
                        std::optional<float> a);

  enum class HueInterpolationMethod : uint8_t {
    kShorter,
    kLonger,
    kIncreasing,
    kDecreasing,
  };

  // Creates a color with the Color-Mix method in CSS Color 5. This will produce
  // an interpolation between two colors, and apply an alpha multiplier if the
  // proportion was not 100% when parsing.
  static Color FromColorMix(ColorSpace interpolation_space,
                            std::optional<HueInterpolationMethod> hue_method,
                            Color color1,
                            Color color2,
                            float percentage,
                            float alpha_multiplier);

  // Produce a color that is the result of mixing color1 and color2.
  //
  // interpolation_space: The space in which to perform the interpolation. Both
  // input colors are converted to this space before interpolation and the
  // resulting color will be in this space as well.
  //
  // hue_method: See https://www.w3.org/TR/css-color-4/#hue-interpolation.
  //
  // percentage: How far to interpolate between color1 and color2. 0.0 returns
  // color1 and 1.0 returns color2. It is unbounded, so it is possible to
  // interpolate beyond these bounds with percentages outside the range [0, 1].
  static Color InterpolateColors(
      ColorSpace interpolation_space,
      std::optional<HueInterpolationMethod> hue_method,
      Color color1,
      Color color2,
      float percentage);

  // TODO(crbug.com/1308932): These three functions are just helpers for
  // while we're converting platform/graphics to float color.
  static constexpr Color FromSkColor4f(SkColor4f fc) { return Color(fc); }
  static constexpr Color FromSkColor(SkColor color) { return Color(color); }
  static constexpr Color FromRGBA32(RGBA32 color) { return Color(color); }

  // Convert a Color to SkColor4f, for use in painting and compositing. Once a
  // Color has been converted to SkColor4f it should not be converted back.
  SkColor4f toSkColor4f() const;

  // Convert a color to SkColor4f, for use as a gradient stop. Unlike the above
  // function, this may avoid operations like gamut mapping, to ensure that
  // round-trip conversions be preserved.
  SkColor4f ToGradientStopSkColor4f(ColorSpace interpolation_space) const;

  // Return true if the oklab and oklch spaces have gamut mapping baked into
  // them.
  static bool IsBakedGamutMappingEnabled();

  WTF::String SerializeInternal() const;
  // Returns the color serialized according to HTML5:
  // http://www.whatwg.org/specs/web-apps/current-work/#serialization-of-a-color
  WTF::String SerializeAsCSSColor() const;
  // Canvas colors are serialized somewhat differently:
  // https://html.spec.whatwg.org/multipage/canvas.html#serialisation-of-a-color
  WTF::String SerializeAsCanvasColor() const;
  // For appending color interpolation spaces and hue interpolation methods to
  // the serialization of gradients and color-mix functions.
  static WTF::String SerializeInterpolationSpace(
      Color::ColorSpace color_space,
      Color::HueInterpolationMethod hue_interpolation_method =
          Color::HueInterpolationMethod::kShorter);

  // Returns the color serialized as either #RRGGBB or #RRGGBBAA. The latter
  // format is not a valid CSS color, and should only be seen in DRT dumps.
  WTF::String NameForLayoutTreeAsText() const;

  // Returns whether parsing succeeded. The resulting Color is arbitrary
  // if parsing fails.
  bool SetFromString(const WTF::String&);
  bool SetNamedColor(const WTF::String&);

  bool IsFullyTransparent() const { return Alpha() <= 0.0f; }
  bool IsOpaque() const { return Alpha() >= 1.0f; }

  float Param0() const { return param0_; }
  float Param1() const { return param1_; }
  float Param2() const { return param2_; }
  float Alpha() const { return alpha_; }

  // Gradient interpolation needs to know if parameters are "none".
  bool Param0IsNone() const { return param0_is_none_; }
  bool Param1IsNone() const { return param1_is_none_; }
  bool Param2IsNone() const { return param2_is_none_; }
  bool AlphaIsNone() const { return alpha_is_none_; }
  bool HasNoneParams() const {
    return Param0IsNone() || Param1IsNone() || Param2IsNone() || AlphaIsNone();
  }

  void SetAlpha(float alpha) { alpha_ = alpha; }

  // Access the color as though it were created using rgba syntax. This will
  // clamp all colors to an 8-bit sRGB representation. All callers of these
  // functions should be audited. The function Rgb(), despite the name, does
  // not drop the alpha value.
  int Red() const;
  int Green() const;
  int Blue() const;

  // No colorspace conversions affect alpha.
  int AlphaAsInteger() const {
    return static_cast<int>(lrintf(alpha_ * 255.0f));
  }

  RGBA32 Rgb() const;
  void GetRGBA(float& r, float& g, float& b, float& a) const;
  void GetRGBA(double& r, double& g, double& b, double& a) const;

  // Access the color as though it were created using the hsl() syntax.
  void GetHSL(double& h, double& s, double& l) const;

  // Access the color as though it were created using the hwb() syntax.
  void GetHWB(double& h, double& w, double& b) const;

  Color Light() const;
  Color Dark() const;

  // This is an implementation of Porter-Duff's "source-over" equation
  // TODO(https://crbug.com/1333988): Implement CSS Color level 4 blending,
  // including a color interpolation method parameter.
  Color Blend(const Color&) const;
  Color BlendWithWhite() const;

  static bool ParseHexColor(const StringView&, Color&);
  static bool ParseHexColor(base::span<const LChar>, Color&);
  static bool ParseHexColor(base::span<const UChar>, Color&);

  static const Color kBlack;
  static const Color kWhite;
  static const Color kDarkGray;
  static const Color kGray;
  static const Color kLightGray;
  static const Color kTransparent;

  inline bool operator==(const Color& other) const {
    return color_space_ == other.color_space_ &&
           param0_is_none_ == other.param0_is_none_ &&
           param1_is_none_ == other.param1_is_none_ &&
           param2_is_none_ == other.param2_is_none_ &&
           alpha_is_none_ == other.alpha_is_none_ && param0_ == other.param0_ &&
           param1_ == other.param1_ && param2_ == other.param2_ &&
           alpha_ == other.alpha_;
  }
  inline bool operator!=(const Color& other) const { return !(*this == other); }

  unsigned GetHash() const;

  // What colorspace space a color wants to interpolate in. This is not
  // equivalent to the colorspace of the color itself.
  // https://www.w3.org/TR/css-color-4/#interpolation
  Color::ColorSpace GetColorInterpolationSpace() const;

  ColorSpace GetColorSpace() const { return color_space_; }
  void ConvertToColorSpace(ColorSpace destination_color_space,
                           bool resolve_missing_components = true);

  void ConvertToColorSpaceForInterpolation(ColorSpace destination_color_space);

  // Colors can parse calc(NaN) and calc(Infinity). At computed value time this
  // function is called which resolves all NaNs to zero and +/-infinities to
  // maximum/minimum values, if they exist. It leaves finite values unchanged.
  // See https://github.com/w3c/csswg-drafts/issues/8629
  void ResolveNonFiniteValues();

  FRIEND_TEST_ALL_PREFIXES(BlinkColor, ColorMixNone);
  FRIEND_TEST_ALL_PREFIXES(BlinkColor, ColorInterpolation);
  FRIEND_TEST_ALL_PREFIXES(BlinkColor, HueInterpolation);
  FRIEND_TEST_ALL_PREFIXES(BlinkColor, Premultiply);
  FRIEND_TEST_ALL_PREFIXES(BlinkColor, Unpremultiply);
  FRIEND_TEST_ALL_PREFIXES(BlinkColor, ConvertToColorSpace);
  FRIEND_TEST_ALL_PREFIXES(BlinkColor, toSkColor4fValidation);
  FRIEND_TEST_ALL_PREFIXES(BlinkColor, ExportAsXYZD50Floats);
  FRIEND_TEST_ALL_PREFIXES(BlinkColor, ResolveMissingComponents);
  FRIEND_TEST_ALL_PREFIXES(BlinkColor, SubstituteMissingParameters);

 private:
  WTF::String SerializeLegacyColorAsCSSColor() const;
  constexpr explicit Color(RGBA32 color)
      : param0_is_none_(0),
        param1_is_none_(0),
        param2_is_none_(0),
        alpha_is_none_(0),
        param0_(((color >> 16) & 0xFF)),
        param1_(((color >> 8) & 0xFF)),
        param2_(((color >> 0) & 0xFF)),
        alpha_(((color >> 24) & 0xFF) / 255.f) {}
  constexpr explicit Color(SkColor4f color)
      : param0_is_none_(0),
        param1_is_none_(0),
        param2_is_none_(0),
        alpha_is_none_(0),
        param0_(color.fR * 255.0),
        param1_(color.fG * 255.0),
        param2_(color.fB * 255.0),
        alpha_(color.fA) {}
  static constexpr int ClampInt255(int x) {
    return x < 0 ? 0 : (x > 255 ? 255 : x);
  }
  void GetHueMaxMin(double&, double&, double&) const;

  std::tuple<float, float, float> ExportAsXYZD50Floats() const;

  // Common helper function to toSkColor4f and ToGradientStopSkColor4f.
  SkColor4f ToSkColor4fInternal(bool gamut_map_oklab_oklch) const;

  float PremultiplyColor();
  void UnpremultiplyColor();
  void ResolveMissingComponents();

  // HueInterpolation assumes value1 and value2 are degrees, it will interpolate
  // value1 and value2 as per CSS Color 4 spec.
  static float HueInterpolation(float value1,
                                float value2,
                                float percentage,
                                HueInterpolationMethod hue_method);

  // According the Spec https://www.w3.org/TR/css-color-4/#interpolation-missing
  // we have to do a special treatment of when to carry forward the 'noneness'
  // of a component, given if it's an 'analog component'.

  // Get the analogous missing components for this color with respect to the
  // specified interpolation color space. Returns an array with the 'none'-ness
  // for each parameter in the interpolation color space, suitable for passing
  // to CarryForwardAnalogousMissingComponents() after converting the color to
  // the interpolation color space.
  std::array<bool, 3> GetAnalogousMissingComponents(
      ColorSpace interpolation_space) const;

  // Set param[0..2]_is_none_ to true if the corresponding entry in the array
  // is true.
  void CarryForwardAnalogousMissingComponents(
      const std::array<bool, 3>& missing_components);

  // https://www.w3.org/TR/css-color-4/#interpolation-missing
  // If a color with a carried forward missing component is interpolated
  // with another color which is not missing that component, the missing
  // component is treated as having the other color’s component value.
  static bool SubstituteMissingParameters(Color& color1, Color& color2);

  ColorSpace color_space_ = ColorSpace::kSRGBLegacy;

  // Whether or not color parameters were specified as none (this only affects
  // interpolation behavior, the parameter values area always valid).
  uint8_t param0_is_none_ : 1;
  uint8_t param1_is_none_ : 1;
  uint8_t param2_is_none_ : 1;
  uint8_t alpha_is_none_ : 1;

  // The color parameters.
  float param0_ = 0.f;
  float param1_ = 0.f;
  float param2_ = 0.f;

  // The alpha value for the color is guaranteed to be in the [0, 1] interval.
  float alpha_ = 0.f;
};

// For unit tests and similar.
PLATFORM_EXPORT std::ostream& operator<<(std::ostream& os, const Color& color);

PLATFORM_EXPORT int DifferenceSquared(const Color&, const Color&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_COLOR_H_
