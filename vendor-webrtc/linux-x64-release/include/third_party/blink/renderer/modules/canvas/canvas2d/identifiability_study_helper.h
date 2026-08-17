// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_CANVAS2D_IDENTIFIABILITY_STUDY_HELPER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_CANVAS2D_IDENTIFIABILITY_STUDY_HELPER_H_

#include <stdint.h>

#include <array>
#include <initializer_list>

#include "base/compiler_specific.h"
#include "third_party/blink/public/common/privacy_budget/identifiability_study_settings.h"
#include "third_party/blink/public/common/privacy_budget/identifiable_surface.h"
#include "third_party/blink/public/common/privacy_budget/identifiable_token.h"
#include "third_party/blink/public/common/privacy_budget/identifiable_token_builder.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/forward.h"  // IWYU pragma: keep (blink::Visitor)
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

// https://github.com/include-what-you-use/include-what-you-use/issues/1546
// IWYU pragma: no_forward_declare WTF::internal::__thisIsHereToForceASemicolonAfterThisMacro

// IWYU pragma: no_include "third_party/blink/renderer/platform/heap/visitor.h"

namespace blink {

// Text operations supported on different canvas types; the intent is to use
// these values (and any input supplied to these operations) to build a running
// hash that reprensents the sequence of text operations performed on the
// canvas. A hash of all other canvas operations is maintained by hashing the
// serialized PaintOps produced by the canvas in CanvasResourceProvider.
//
// If a canvas method to exfiltrate the canvas buffer is called by a script
// (getData(), etc.), this hash will be uploaded to UKM along with a hash of the
// canvas buffer data.
//
// **Don't renumber after the privacy budget study has started to ensure
// consistency.**
enum class CanvasOps {
  // CanvasPath operations.
  kClosePath = 0,
  kMoveTo,
  kLineTo,
  kQuadradicCurveTo,
  kBezierCurveTo,
  kArcTo,
  kArc,
  kEllipse,
  kRect,
  // Path2D operations.
  kAddPath,
  // Canvas2DRecorderContext methods.
  kSetStrokeStyle,
  kSetFillStyle,
  kSetLineWidth,
  kSetLineCap,
  kSetLineJoin,
  kSetMiterLimit,
  kSetLineDash,
  kSetLineDashOffset,
  kSetShadowOffsetX,
  kSetShadowOffsetY,
  kSetShadowBlur,
  kSetShadowColor,
  kSetGlobalAlpha,
  kSetGlobalCompositeOpertion,
  kSetFilter,
  kSave,
  kRestore,
  kScale,
  kRotate,
  kTranslate,
  kTransform,
  kResetTransform,
  kBeginPath,
  kFill,
  kFill__Path,
  kStroke,
  kStroke__Path,
  kClip,
  kClip__Path,
  kClearRect,
  kFillRect,
  kStrokeRect,
  kDrawImage,
  kCreateLinearGradient,
  kCreateRadialGradient,
  kCreatePattern,
  kPutImageData,
  kSetImageSmoothingEnabled,
  kSetImageSmoothingQuality,
  kSetTextAlign,
  kSetTextBaseline,
  kReset,
  // CanvasRenderingContext2D / OffscreenCanvasRenderingContext2D methods.
  kSetFont,
  kFillText,
  kStrokeText,
  kDrawFocusIfNeeded,  // CanvasRenderingContext2D only.
  // CanvasGradient methods.
  kAddColorStop,
};

// A helper class to simplify maintaining the current text digest for the canvas
// context. An operation count is also maintained to limit the performance
// impact of the study.
class IdentifiabilityStudyHelper final {
  DISALLOW_NEW();

 public:
  // UpdateBuilder() should be called iff ShouldUpdateBuilder() is true, to
  // avoid unnecessary copies of parameters and hashing when GetToken() won't be
  // called.
  ALWAYS_INLINE bool ShouldUpdateBuilder() {
    if (!is_canvas_type_allowed_) [[likely]] {
      return false;
    }
    if (!execution_context_ ||
        execution_context_->IsInRequestAnimationFrame() ||
        operation_count_ >= max_operations_) {
      encountered_skipped_ops_ = true;
      return false;
    }
    return true;
  }

  // Do *not* call this method if ShouldUpdateBuilder() is false -- updates
  // the internal digest based on the series of digestable parameters.
  template <typename... Ts>
  void UpdateBuilder(Ts... tokens) {
    AddTokens({tokens...});
    operation_count_++;
  }

  // Returns an IdentifiableToken representing the internal computed digest.
  IdentifiableToken GetToken() const {
    if (position_ == 0) {
      return chaining_value_;
    }
    return DigestPartialData();
  }

  [[nodiscard]] bool encountered_skipped_ops() const {
    return encountered_skipped_ops_;
  }

  [[nodiscard]] bool encountered_sensitive_ops() const {
    return encountered_sensitive_ops_;
  }

  [[nodiscard]] bool encountered_partially_digested_image() const {
    return encountered_partially_digested_image_;
  }

  void set_encountered_skipped_ops() { encountered_skipped_ops_ = true; }

  void set_encountered_sensitive_ops() { encountered_sensitive_ops_ = true; }

  void set_encountered_partially_digested_image() {
    encountered_partially_digested_image_ = true;
  }

  void SetExecutionContext(ExecutionContext* context) {
    execution_context_ = context;
  }

  ExecutionContext* execution_context() const {
    return execution_context_.Get();
  }

  // For testing, allows scoped changing the max number of operations for all
  // IdentifiabilityStudyHelper instances.
  class ScopedMaxOperationsSetter {
   public:
    explicit ScopedMaxOperationsSetter(int new_value)
        : old_max_operations_(IdentifiabilityStudyHelper::max_operations_) {
      IdentifiabilityStudyHelper::max_operations_ = new_value;
    }
    ~ScopedMaxOperationsSetter() {
      IdentifiabilityStudyHelper::max_operations_ = old_max_operations_;
    }

   private:
    const int old_max_operations_;
  };

  void Trace(Visitor* visitor) const;

 private:
  // Note that primitives are implicitly converted to IdentifiableTokens.
  void MODULES_EXPORT
  AddTokens(std::initializer_list<IdentifiableToken> tokens);

  uint64_t MODULES_EXPORT DigestPartialData() const;

  const bool is_canvas_type_allowed_ =
      IdentifiabilityStudySettings::Get()->ShouldSampleType(
          blink::IdentifiableSurface::Type::kCanvasReadback);

  Member<ExecutionContext> execution_context_;

  static MODULES_EXPORT int max_operations_;

  int operation_count_ = 0;

  // If true, at least one op was skipped completely, for performance reasons.
  bool encountered_skipped_ops_ = false;

  // If true, encountered at least one "sensitive" operation -- for instance,
  // strings may contain PII, so we only use a 16-bit digest for such strings.
  //
  // This must be set manually by calling set_encountered_sensitive_ops().
  bool encountered_sensitive_ops_ = false;

  // If true, at least one op was partially-digested -- for instance, images
  // drawn to the canvas have their width, height, etc. digested, but not the
  // image contents, for performance and complexity reasons.
  bool encountered_partially_digested_image_ = false;

  std::array<int64_t, 8> partial_;
  wtf_size_t position_ = 0;
  uint64_t chaining_value_ = IdentifiableTokenBuilder::kChainingValueSeed;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_CANVAS2D_IDENTIFIABILITY_STUDY_HELPER_H_
