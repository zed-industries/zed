// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_CALCULATION_EXPRESSION_NODE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_CALCULATION_EXPRESSION_NODE_H_

#include "base/check_op.h"
#include "third_party/blink/renderer/platform/geometry/length.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/ref_counted.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

enum class CalculationOperator {
  kAdd,
  kSubtract,
  kMultiply,  // Division is converted to multiplication and use this value too.
  kInvert,
  kMin,
  kMax,
  kClamp,
  kRoundNearest,
  kRoundUp,
  kRoundDown,
  kRoundToZero,
  kMod,
  kRem,
  kLog,
  kExp,
  kSqrt,
  kHypot,
  kAbs,
  kSign,
  kProgress,
  kContainerProgress,
  kCalcSize,
  kMediaProgress,
  kPow,
  kSin,
  kCos,
  kTan,
  kAsin,
  kAcos,
  kAtan,
  kAtan2,
  kInvalid
};

// Represents an expression composed of numbers, |PixelsAndPercent| and multiple
// types of operators. To be consumed by |Length| values that involve
// non-trivial math functions like min() and max().
class PLATFORM_EXPORT CalculationExpressionNode
    : public RefCounted<CalculationExpressionNode> {
 public:
  virtual float Evaluate(float max_value, const EvaluationInput&) const = 0;
  bool operator==(const CalculationExpressionNode& other) const {
    return Equals(other);
  }
  bool operator!=(const CalculationExpressionNode& other) const {
    return !operator==(other);
  }

  bool HasAuto() const { return has_auto_; }
  bool HasContentOrIntrinsicSize() const { return has_content_or_intrinsic_; }
  bool HasAutoOrContentOrIntrinsicSize() const {
    return has_auto_ || has_content_or_intrinsic_;
  }
  bool HasStretch() const { return has_stretch_; }
  // HasPercent returns whether this node's value expression should be
  // treated as having a percent.  Note that this means that percentages
  // inside of the calculation part of a calc-size() do not make the
  // calc-size() act as though it has a percent.
  bool HasPercent() const { return has_percent_; }
  bool HasPercentOrStretch() const { return has_percent_ || has_stretch_; }
  bool HasColorChannelKeyword() const { return has_color_channel_keyword_; }

  virtual bool HasMinContent() const { return false; }
  virtual bool HasMaxContent() const { return false; }
  virtual bool HasFitContent() const { return false; }

  virtual bool IsNumber() const { return false; }
  virtual bool IsIdentifier() const { return false; }
  virtual bool IsSizingKeyword() const { return false; }
  virtual bool IsColorChannelKeyword() const { return false; }
  virtual bool IsPixelsAndPercent() const { return false; }
  virtual bool IsOperation() const { return false; }

  virtual scoped_refptr<const CalculationExpressionNode> Zoom(
      double factor) const = 0;

  virtual ~CalculationExpressionNode() = default;

#if DCHECK_IS_ON()
  enum class ResultType { kInvalid, kNumber, kPixelsAndPercent, kIdent };

  virtual ResultType ResolvedResultType() const = 0;

 protected:
  ResultType result_type_;
#endif

 protected:
  virtual bool Equals(const CalculationExpressionNode& other) const = 0;

  bool has_content_or_intrinsic_ = false;
  bool has_auto_ = false;
  bool has_percent_ = false;
  bool has_stretch_ = false;
  bool has_color_channel_keyword_ = false;
};

class PLATFORM_EXPORT CalculationExpressionNumberNode final
    : public CalculationExpressionNode {
 public:
  CalculationExpressionNumberNode(float value) : value_(value) {
#if DCHECK_IS_ON()
    result_type_ = ResultType::kNumber;
#endif
  }

  float Value() const { return value_; }

  // Implement |CalculationExpressionNode|:
  float Evaluate(float max_value, const EvaluationInput&) const final;
  bool Equals(const CalculationExpressionNode& other) const final;
  scoped_refptr<const CalculationExpressionNode> Zoom(
      double factor) const final;
  bool IsNumber() const final { return true; }
  ~CalculationExpressionNumberNode() final = default;

#if DCHECK_IS_ON()
  ResultType ResolvedResultType() const final;
#endif

 private:
  float value_;
};

template <>
struct DowncastTraits<CalculationExpressionNumberNode> {
  static bool AllowFrom(const CalculationExpressionNode& node) {
    return node.IsNumber();
  }
};

class PLATFORM_EXPORT CalculationExpressionIdentifierNode final
    : public CalculationExpressionNode {
 public:
  explicit CalculationExpressionIdentifierNode(AtomicString identifier)
      : identifier_(std::move(identifier)) {
#if DCHECK_IS_ON()
    result_type_ = ResultType::kIdent;
#endif
  }

  const AtomicString& Value() const { return identifier_; }

  // Implement |CalculationExpressionNode|:
  float Evaluate(float max_value, const EvaluationInput&) const final {
    return 0.0f;
  }
  bool Equals(const CalculationExpressionNode& other) const final {
    auto* other_identifier =
        DynamicTo<CalculationExpressionIdentifierNode>(other);
    return other_identifier && other_identifier->Value() == Value();
  }
  scoped_refptr<const CalculationExpressionNode> Zoom(
      double factor) const final {
    return this;
  }
  bool IsIdentifier() const final { return true; }

#if DCHECK_IS_ON()
  ResultType ResolvedResultType() const final { return ResultType::kIdent; }
#endif

 private:
  AtomicString identifier_;
};

template <>
struct DowncastTraits<CalculationExpressionIdentifierNode> {
  static bool AllowFrom(const CalculationExpressionNode& node) {
    return node.IsIdentifier();
  }
};

class PLATFORM_EXPORT CalculationExpressionSizingKeywordNode final
    : public CalculationExpressionNode {
 public:
  enum class Keyword : uint8_t {
    kSize,
    kAny,
    kAuto,
    kContent,

    // The keywords below should match those accepted by
    // css_parsing_utils::ValidWidthOrHeightKeyword.
    kMinContent,
    kWebkitMinContent,
    kMaxContent,
    kWebkitMaxContent,
    kFitContent,
    kWebkitFitContent,
    kStretch,
    kWebkitFillAvailable,
  };

  explicit CalculationExpressionSizingKeywordNode(Keyword keyword);

  Keyword Value() const { return keyword_; }

  // Implement |CalculationExpressionNode|:
  float Evaluate(float max_value, const EvaluationInput&) const final;
  bool Equals(const CalculationExpressionNode& other) const final {
    auto* other_sizing_keyword =
        DynamicTo<CalculationExpressionSizingKeywordNode>(other);
    return other_sizing_keyword && other_sizing_keyword->Value() == Value();
  }
  scoped_refptr<const CalculationExpressionNode> Zoom(
      double factor) const final {
    // TODO(https://crbug.com/313072): Is this correct, or do we need to
    // adjust for zoom?
    return this;
  }
  bool IsSizingKeyword() const final { return true; }

  bool HasMinContent() const final {
    return keyword_ == Keyword::kMinContent ||
           keyword_ == Keyword::kWebkitMinContent;
  }
  bool HasMaxContent() const final {
    return keyword_ == Keyword::kMaxContent ||
           keyword_ == Keyword::kWebkitMaxContent;
  }
  bool HasFitContent() const final {
    return keyword_ == Keyword::kFitContent ||
           keyword_ == Keyword::kWebkitFitContent;
  }

#if DCHECK_IS_ON()
  ResultType ResolvedResultType() const final {
    return ResultType::kPixelsAndPercent;
  }
#endif

 private:
  Keyword keyword_;
};

template <>
struct DowncastTraits<CalculationExpressionSizingKeywordNode> {
  static bool AllowFrom(const CalculationExpressionNode& node) {
    return node.IsSizingKeyword();
  }
};

class PLATFORM_EXPORT CalculationExpressionColorChannelKeywordNode final
    : public CalculationExpressionNode {
 public:
  explicit CalculationExpressionColorChannelKeywordNode(
      ColorChannelKeyword channel);

  ColorChannelKeyword Value() const { return channel_; }

  // Implement |CalculationExpressionNode|:
  float Evaluate(float max_value, const EvaluationInput&) const final;
  bool Equals(const CalculationExpressionNode& other) const final {
    auto* other_color_channel_keyword =
        DynamicTo<CalculationExpressionColorChannelKeywordNode>(other);
    return other_color_channel_keyword &&
           other_color_channel_keyword->Value() == Value();
  }
  scoped_refptr<const CalculationExpressionNode> Zoom(
      double factor) const final {
    return this;
  }
  bool IsColorChannelKeyword() const final { return true; }

#if DCHECK_IS_ON()
  ResultType ResolvedResultType() const final { return ResultType::kNumber; }
#endif

 private:
  ColorChannelKeyword channel_;
};

template <>
struct DowncastTraits<CalculationExpressionColorChannelKeywordNode> {
  static bool AllowFrom(const CalculationExpressionNode& node) {
    return node.IsColorChannelKeyword();
  }
};

class PLATFORM_EXPORT CalculationExpressionPixelsAndPercentNode final
    : public CalculationExpressionNode {
 public:
  CalculationExpressionPixelsAndPercentNode(PixelsAndPercent value)
      : value_(value) {
#if DCHECK_IS_ON()
    result_type_ = ResultType::kPixelsAndPercent;
#endif
    if (value.has_explicit_percent) {
      has_percent_ = true;
    }
  }

  float Pixels() const { return value_.pixels; }
  float Percent() const { return value_.percent; }
  PixelsAndPercent GetPixelsAndPercent() const { return value_; }
  bool HasExplicitPixels() const { return value_.has_explicit_pixels; }
  bool HasExplicitPercent() const { return value_.has_explicit_percent; }

  // Implement |CalculationExpressionNode|:
  float Evaluate(float max_value, const EvaluationInput&) const final;
  bool Equals(const CalculationExpressionNode& other) const final;
  scoped_refptr<const CalculationExpressionNode> Zoom(
      double factor) const final;
  bool IsPixelsAndPercent() const final { return true; }
  ~CalculationExpressionPixelsAndPercentNode() final = default;

#if DCHECK_IS_ON()
  ResultType ResolvedResultType() const final;
#endif

 private:
  PixelsAndPercent value_;
};

template <>
struct DowncastTraits<CalculationExpressionPixelsAndPercentNode> {
  static bool AllowFrom(const CalculationExpressionNode& node) {
    return node.IsPixelsAndPercent();
  }
};

class PLATFORM_EXPORT CalculationExpressionOperationNode final
    : public CalculationExpressionNode {
 public:
  using Children = Vector<scoped_refptr<const CalculationExpressionNode>>;

  static scoped_refptr<const CalculationExpressionNode> CreateSimplified(
      Children&& children,
      CalculationOperator op);

  CalculationExpressionOperationNode(Children&& children,
                                     CalculationOperator op);

  const Children& GetChildren() const { return children_; }
  CalculationOperator GetOperator() const { return operator_; }

  // Implement |CalculationExpressionNode|:
  float Evaluate(float max_value, const EvaluationInput&) const final;
  bool Equals(const CalculationExpressionNode& other) const final;
  scoped_refptr<const CalculationExpressionNode> Zoom(
      double factor) const final;
  bool IsOperation() const final { return true; }
  bool HasMinContent() const final;
  bool HasMaxContent() const final;
  bool HasFitContent() const final;
  ~CalculationExpressionOperationNode() final = default;

#if DCHECK_IS_ON()
  ResultType ResolvedResultType() const final;
#endif

 private:
  Children children_;
  CalculationOperator operator_;
};

template <>
struct DowncastTraits<CalculationExpressionOperationNode> {
  static bool AllowFrom(const CalculationExpressionNode& node) {
    return node.IsOperation();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_CALCULATION_EXPRESSION_NODE_H_
