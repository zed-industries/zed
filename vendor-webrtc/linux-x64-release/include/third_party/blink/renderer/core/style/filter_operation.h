/*
 * Copyright (C) 2011 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_FILTER_OPERATION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_FILTER_OPERATION_H_

#include "base/notreached.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/style/shadow_data.h"
#include "third_party/blink/renderer/platform/geometry/length.h"
#include "third_party/blink/renderer/platform/geometry/length_point.h"
#include "third_party/blink/renderer/platform/graphics/box_reflection.h"
#include "third_party/blink/renderer/platform/graphics/color.h"
#include "third_party/blink/renderer/platform/graphics/filters/fe_component_transfer.h"
#include "third_party/blink/renderer/platform/graphics/filters/fe_convolve_matrix.h"
#include "third_party/blink/renderer/platform/graphics/filters/fe_turbulence.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "ui/gfx/geometry/rect_f.h"

#include <iosfwd>

namespace blink {

class Filter;
class SVGResource;
class SVGResourceClient;

// CSS Filters

class CORE_EXPORT FilterOperation : public GarbageCollected<FilterOperation> {
 public:
  FilterOperation(const FilterOperation&) = delete;
  FilterOperation& operator=(const FilterOperation&) = delete;

  enum class OperationType {
    kReference,  // url(#somefilter)
    kGrayscale,
    kSepia,
    kSaturate,
    kHueRotate,
    kLuminanceToAlpha,
    kInvert,
    kOpacity,
    kBrightness,
    kContrast,
    kBlur,
    kDropShadow,
    kBoxReflect,
    kColorMatrix,
    kComponentTransfer,
    kConvolveMatrix,
    kTurbulence,
  };

  static bool CanInterpolate(FilterOperation::OperationType type) {
    switch (type) {
      case OperationType::kGrayscale:
      case OperationType::kSepia:
      case OperationType::kSaturate:
      case OperationType::kHueRotate:
      case OperationType::kLuminanceToAlpha:
      case OperationType::kInvert:
      case OperationType::kOpacity:
      case OperationType::kBrightness:
      case OperationType::kContrast:
      case OperationType::kBlur:
      case OperationType::kDropShadow:
      case OperationType::kColorMatrix:
      case OperationType::kTurbulence:
        return true;
      case OperationType::kReference:
      case OperationType::kComponentTransfer:
      case OperationType::kConvolveMatrix:
      case OperationType::kBoxReflect:
        return false;
    }
    NOTREACHED();
  }

  virtual ~FilterOperation() = default;
  virtual void Trace(Visitor* visitor) const {}

  bool operator==(const FilterOperation& o) const {
    return IsSameType(o) && IsEqualAssumingSameType(o);
  }
  bool operator!=(const FilterOperation& o) const { return !(*this == o); }

  OperationType GetType() const { return type_; }
  virtual bool IsSameType(const FilterOperation& o) const {
    return o.GetType() == type_;
  }

  // True if the alpha channel of any pixel can change under this operation.
  virtual bool AffectsOpacity() const { return false; }
  // True if the the value of one pixel can affect the value of another pixel
  // under this operation, such as blur.
  virtual bool MovesPixels() const { return false; }
  // True if the operation depends on the 'currentcolor' value.
  virtual bool UsesCurrentColor() const { return false; }

  // Maps "forward" to determine which pixels in a destination rect are
  // affected by pixels in the source rect.
  // See also FilterEffect::MapRect.
  virtual gfx::RectF MapRect(const gfx::RectF& rect) const { return rect; }

  // For debugging/logging only.
  virtual String DebugString() const { return "<unknown>"; }

 protected:
  explicit FilterOperation(OperationType type) : type_(type) {}

  virtual bool IsEqualAssumingSameType(const FilterOperation&) const = 0;

  OperationType type_;
};

inline std::ostream& operator<<(std::ostream& stream,
                                const FilterOperation& operation) {
  return stream << operation.DebugString();
}

class CORE_EXPORT ReferenceFilterOperation : public FilterOperation {
 public:
  ReferenceFilterOperation(const AtomicString& url, SVGResource*);

  bool AffectsOpacity() const override { return true; }
  bool MovesPixels() const override { return true; }
  bool UsesCurrentColor() const override {
    // This is pessimistic. A reference filter _may_ contain a primitive that
    // references 'currentcolor'. If `filter_` is set it could be used to
    // produce a less pessimistic result, but additional pre-processing would
    // be required since enough information isn't preserved.
    return true;
  }

  gfx::RectF MapRect(const gfx::RectF&) const override;

  const AtomicString& Url() const { return url_; }

  Filter* GetFilter() const { return filter_.Get(); }
  void SetFilter(Filter* filter) { filter_ = filter; }

  SVGResource* Resource() const { return resource_.Get(); }

  void AddClient(SVGResourceClient&);
  void RemoveClient(SVGResourceClient&);

  void Trace(Visitor*) const override;

  String DebugString() const override { return "<ref: " + url_ + ">"; }

 protected:
  bool IsEqualAssumingSameType(const FilterOperation&) const override;

 private:
  AtomicString url_;
  Member<SVGResource> resource_;
  Member<Filter> filter_;
};

template <>
struct DowncastTraits<ReferenceFilterOperation> {
  static bool AllowFrom(const FilterOperation& op) {
    return op.GetType() == FilterOperation::OperationType::kReference;
  }
};

// GRAYSCALE, SEPIA, SATURATE, HUE_ROTATE and LUMINANCE_TO_ALPHA are variations
// on a basic color matrix effect.  For HUE_ROTATE, the angle of rotation is
// stored in amount_. For LUMINANCE_TO_ALPHA amount_ is unused.
class CORE_EXPORT BasicColorMatrixFilterOperation : public FilterOperation {
 public:
  BasicColorMatrixFilterOperation(double amount, OperationType type)
      : FilterOperation(type), amount_(amount) {}

  double Amount() const { return amount_; }

  String DebugString() const override {
    char buf[256];
    snprintf(buf, sizeof(buf), "<basic color matrix op %d, amount=%f>",
             static_cast<int>(type_), amount_);
    return buf;
  }

 protected:
  bool IsEqualAssumingSameType(const FilterOperation& o) const override {
    const BasicColorMatrixFilterOperation* other =
        static_cast<const BasicColorMatrixFilterOperation*>(&o);
    return amount_ == other->amount_;
  }

 private:
  double amount_;
};

// Generic color matrices
class CORE_EXPORT ColorMatrixFilterOperation : public FilterOperation {
 public:
  ColorMatrixFilterOperation(Vector<float> values, OperationType type)
      : FilterOperation(type), values_(std::move(values)) {}

  const Vector<float>& Values() const { return values_; }

  String DebugString() const override {
    char buf[256];
    snprintf(buf, sizeof(buf), "<color matrix op %d>", static_cast<int>(type_));
    return buf;
  }

 protected:
  bool IsEqualAssumingSameType(const FilterOperation& o) const override {
    const ColorMatrixFilterOperation* other =
        static_cast<const ColorMatrixFilterOperation*>(&o);
    return values_ == other->values_;
  }

 private:
  Vector<float> values_;
};

inline bool IsBasicColorMatrixFilterOperation(
    const FilterOperation& operation) {
  FilterOperation::OperationType type = operation.GetType();
  return type == FilterOperation::OperationType::kGrayscale ||
         type == FilterOperation::OperationType::kSepia ||
         type == FilterOperation::OperationType::kSaturate ||
         type == FilterOperation::OperationType::kHueRotate ||
         type == FilterOperation::OperationType::kLuminanceToAlpha;
}

template <>
struct DowncastTraits<BasicColorMatrixFilterOperation> {
  static bool AllowFrom(const FilterOperation& op) {
    return IsBasicColorMatrixFilterOperation(op);
  }
};

template <>
struct DowncastTraits<ColorMatrixFilterOperation> {
  static bool AllowFrom(const FilterOperation& op) {
    return op.GetType() == FilterOperation::OperationType::kColorMatrix;
  }
};

// INVERT, BRIGHTNESS, CONTRAST and OPACITY are variations on a basic component
// transfer effect.
class CORE_EXPORT BasicComponentTransferFilterOperation
    : public FilterOperation {
 public:
  BasicComponentTransferFilterOperation(double amount, OperationType type)
      : FilterOperation(type), amount_(amount) {}

  double Amount() const { return amount_; }

  bool AffectsOpacity() const override {
    return type_ == OperationType::kOpacity;
  }

 protected:
  bool IsEqualAssumingSameType(const FilterOperation& o) const override {
    const BasicComponentTransferFilterOperation* other =
        static_cast<const BasicComponentTransferFilterOperation*>(&o);
    return amount_ == other->amount_;
  }

 private:
  double amount_;
};

inline bool IsBasicComponentTransferFilterOperation(
    const FilterOperation& operation) {
  FilterOperation::OperationType type = operation.GetType();
  return type == FilterOperation::OperationType::kInvert ||
         type == FilterOperation::OperationType::kOpacity ||
         type == FilterOperation::OperationType::kBrightness ||
         type == FilterOperation::OperationType::kContrast;
}

template <>
struct DowncastTraits<BasicComponentTransferFilterOperation> {
  static bool AllowFrom(const FilterOperation& op) {
    return IsBasicComponentTransferFilterOperation(op);
  }
};

class CORE_EXPORT BlurFilterOperation : public FilterOperation {
 public:
  explicit BlurFilterOperation(const Length& std_deviation_x,
                               const Length& std_deviation_y)
      : FilterOperation(OperationType::kBlur),
        std_deviation_(std_deviation_x, std_deviation_y) {}

  explicit BlurFilterOperation(const Length& std_deviation)
      : BlurFilterOperation(std_deviation, std_deviation) {}

  const Length& StdDeviation() const {
    // CSS only supports isotropic blurs (with matching X and Y), so this
    // accessor should be safe in CSS-specific code. Canvas filters allow
    // anisotropic blurs (to match SVG) and so this accessor should not be used
    // in canvas-filter code.
    DCHECK_EQ(std_deviation_.X(), std_deviation_.Y())
        << "use StdDeviationXY() instead";
    return std_deviation_.X();
  }
  const LengthPoint& StdDeviationXY() const {
    // This accessor is always safe to use.
    return std_deviation_;
  }

  bool AffectsOpacity() const override { return true; }
  bool MovesPixels() const override { return true; }
  gfx::RectF MapRect(const gfx::RectF&) const override;

  String DebugString() const override { return "<blur>"; }

 protected:
  bool IsEqualAssumingSameType(const FilterOperation& o) const override {
    const BlurFilterOperation* other =
        static_cast<const BlurFilterOperation*>(&o);
    return std_deviation_ == other->std_deviation_;
  }

 private:
  LengthPoint std_deviation_;
};

template <>
struct DowncastTraits<BlurFilterOperation> {
  static bool AllowFrom(const FilterOperation& op) {
    return op.GetType() == FilterOperation::OperationType::kBlur;
  }
};

class CORE_EXPORT DropShadowFilterOperation : public FilterOperation {
 public:
  explicit DropShadowFilterOperation(const ShadowData& shadow)
      : FilterOperation(OperationType::kDropShadow), shadow_(shadow) {}

  void Trace(Visitor* visitor) const override {
    visitor->Trace(shadow_);
    FilterOperation::Trace(visitor);
  }

  const ShadowData& Shadow() const { return shadow_; }

  bool AffectsOpacity() const override { return true; }
  bool MovesPixels() const override { return true; }
  bool UsesCurrentColor() const override {
    return shadow_.GetColor().IsCurrentColor();
  }

  gfx::RectF MapRect(const gfx::RectF&) const override;

  String DebugString() const override {
    std::stringstream ss;
    ss << shadow_.GetColor();
    char buf[256];
    snprintf(buf, sizeof(buf),
             "<drop shadow: x=%f y=%f blur=%f spread=%f opacity=%f color=%s>",
             shadow_.X(), shadow_.Y(), shadow_.Blur(), shadow_.Spread(),
             shadow_.Opacity(), ss.str().c_str());
    return buf;
  }

 protected:
  bool IsEqualAssumingSameType(const FilterOperation& o) const override {
    const DropShadowFilterOperation* other =
        static_cast<const DropShadowFilterOperation*>(&o);
    return shadow_ == other->shadow_;
  }

 private:
  ShadowData shadow_;
};

template <>
struct DowncastTraits<DropShadowFilterOperation> {
  static bool AllowFrom(const FilterOperation& op) {
    return op.GetType() == FilterOperation::OperationType::kDropShadow;
  }
};

class CORE_EXPORT BoxReflectFilterOperation : public FilterOperation {
 public:
  explicit BoxReflectFilterOperation(const BoxReflection& reflection)
      : FilterOperation(OperationType::kBoxReflect), reflection_(reflection) {}

  const BoxReflection& Reflection() const { return reflection_; }

  bool AffectsOpacity() const override { return true; }
  bool MovesPixels() const override { return true; }
  gfx::RectF MapRect(const gfx::RectF&) const override;

 protected:
  bool IsEqualAssumingSameType(const FilterOperation&) const override;

  String DebugString() const override { return "<box reflect>"; }

 private:
  BoxReflection reflection_;
};

template <>
struct DowncastTraits<BoxReflectFilterOperation> {
  static bool AllowFrom(const FilterOperation& op) {
    return op.GetType() == FilterOperation::OperationType::kBoxReflect;
  }
};

class CORE_EXPORT ConvolveMatrixFilterOperation : public FilterOperation {
 public:
  ConvolveMatrixFilterOperation(const gfx::Size& kernel_size,
                                float divisor,
                                float bias,
                                const gfx::Point& target_offset,
                                FEConvolveMatrix::EdgeModeType edge_mode,
                                bool preserve_alpha,
                                const Vector<float>& kernel_matrix)
      : FilterOperation(OperationType::kConvolveMatrix),
        kernel_size_(kernel_size),
        divisor_(divisor),
        bias_(bias),
        target_offset_(target_offset),
        edge_mode_(edge_mode),
        preserve_alpha_(preserve_alpha),
        kernel_matrix_(kernel_matrix) {}

  const gfx::Size& KernelSize() const { return kernel_size_; }
  float Divisor() const { return divisor_; }
  float Bias() const { return bias_; }
  const gfx::Point& TargetOffset() const { return target_offset_; }
  FEConvolveMatrix::EdgeModeType EdgeMode() const { return edge_mode_; }
  bool PreserveAlpha() const { return preserve_alpha_; }
  const Vector<float>& KernelMatrix() const { return kernel_matrix_; }

  String DebugString() const override { return "<convolve>"; }

 protected:
  bool IsEqualAssumingSameType(const FilterOperation& o) const override {
    const ConvolveMatrixFilterOperation* other =
        static_cast<const ConvolveMatrixFilterOperation*>(&o);
    return (kernel_size_ == other->kernel_size_ &&
            divisor_ == other->divisor_ && bias_ == other->bias_ &&
            target_offset_ == other->target_offset_ &&
            edge_mode_ == other->edge_mode_ &&
            preserve_alpha_ == other->preserve_alpha_ &&
            kernel_matrix_ == other->kernel_matrix_);
  }

 private:
  gfx::Size kernel_size_;
  float divisor_;
  float bias_;
  gfx::Point target_offset_;
  FEConvolveMatrix::EdgeModeType edge_mode_;
  bool preserve_alpha_;
  Vector<float> kernel_matrix_;
};

template <>
struct DowncastTraits<ConvolveMatrixFilterOperation> {
  static bool AllowFrom(const FilterOperation& op) {
    return op.GetType() == FilterOperation::OperationType::kConvolveMatrix;
  }
};

class CORE_EXPORT ComponentTransferFilterOperation : public FilterOperation {
 public:
  ComponentTransferFilterOperation(const ComponentTransferFunction& red_func,
                                   const ComponentTransferFunction& green_func,
                                   const ComponentTransferFunction& blue_func,
                                   const ComponentTransferFunction& alpha_func)
      : FilterOperation(OperationType::kComponentTransfer),
        red_func_(red_func),
        green_func_(green_func),
        blue_func_(blue_func),
        alpha_func_(alpha_func) {}

  ComponentTransferFunction RedFunc() const { return red_func_; }
  ComponentTransferFunction GreenFunc() const { return green_func_; }
  ComponentTransferFunction BlueFunc() const { return blue_func_; }
  ComponentTransferFunction AlphaFunc() const { return alpha_func_; }
  String DebugString() const override { return "<component transfer>"; }

 protected:
  bool IsEqualAssumingSameType(const FilterOperation& o) const override {
    const ComponentTransferFilterOperation* other =
        static_cast<const ComponentTransferFilterOperation*>(&o);
    return (
        red_func_ == other->red_func_ && green_func_ == other->green_func_ &&
        blue_func_ == other->blue_func_ && alpha_func_ == other->alpha_func_);
  }

 private:
  ComponentTransferFunction red_func_;
  ComponentTransferFunction green_func_;
  ComponentTransferFunction blue_func_;
  ComponentTransferFunction alpha_func_;
};

template <>
struct DowncastTraits<ComponentTransferFilterOperation> {
  static bool AllowFrom(const FilterOperation& op) {
    return op.GetType() == FilterOperation::OperationType::kComponentTransfer;
  }
};

class CORE_EXPORT TurbulenceFilterOperation : public FilterOperation {
 public:
  TurbulenceFilterOperation(TurbulenceType type,
                            float base_frequency_x,
                            float base_frequency_y,
                            int num_octaves,
                            float seed,
                            bool stitch_tiles)
      : FilterOperation(OperationType::kTurbulence),
        type_(type),
        base_frequency_x_(base_frequency_x),
        base_frequency_y_(base_frequency_y),
        num_octaves_(num_octaves),
        seed_(seed),
        stitch_tiles_(stitch_tiles) {}

  TurbulenceType Type() const { return type_; }
  float BaseFrequencyX() const { return base_frequency_x_; }
  float BaseFrequencyY() const { return base_frequency_y_; }
  int NumOctaves() const { return num_octaves_; }
  float Seed() const { return seed_; }
  bool StitchTiles() const { return stitch_tiles_; }
  String DebugString() const override { return "<turbulence>"; }

 protected:
  bool IsEqualAssumingSameType(const FilterOperation& o) const override {
    const TurbulenceFilterOperation* other =
        static_cast<const TurbulenceFilterOperation*>(&o);
    return (type_ == other->type_ &&
            base_frequency_x_ == other->base_frequency_x_ &&
            base_frequency_y_ == other->base_frequency_y_ &&
            num_octaves_ == other->num_octaves_ && seed_ == other->seed_ &&
            stitch_tiles_ == other->stitch_tiles_);
  }

 private:
  TurbulenceType type_;
  float base_frequency_x_;
  float base_frequency_y_;
  int num_octaves_;
  float seed_;
  bool stitch_tiles_;
};

template <>
struct DowncastTraits<TurbulenceFilterOperation> {
  static bool AllowFrom(const FilterOperation& op) {
    return op.GetType() == FilterOperation::OperationType::kTurbulence;
  }
};

#undef DEFINE_FILTER_OPERATION_TYPE_CASTS

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_FILTER_OPERATION_H_
