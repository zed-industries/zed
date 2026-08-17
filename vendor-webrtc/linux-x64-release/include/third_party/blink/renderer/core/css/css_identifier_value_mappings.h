/*
 * Copyright (C) 2007 Alexey Proskuryakov <ap@nypop.com>.
 * Copyright (C) 2008, 2009, 2010, 2011 Apple Inc. All rights reserved.
 * Copyright (C) 2009 Torch Mobile Inc. All rights reserved.
 * (http://www.torchmobile.com/)
 * Copyright (C) 2009 Jeff Schiller <codedread@gmail.com>
 * Copyright (C) Research In Motion Limited 2010. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY EXPRESS OR
 * IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES
 * OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
 * IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT,
 * INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
 * NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_IDENTIFIER_VALUE_MAPPINGS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_IDENTIFIER_VALUE_MAPPINGS_H_

#include "base/notreached.h"
#include "cc/input/scroll_snap_data.h"
#include "third_party/blink/renderer/core/animation/timeline_offset.h"
#include "third_party/blink/renderer/core/animation/timing.h"
#include "third_party/blink/renderer/core/css/css_identifier_value.h"
#include "third_party/blink/renderer/core/css/css_reflection_direction.h"
#include "third_party/blink/renderer/core/css/css_value.h"
#include "third_party/blink/renderer/core/css_value_keywords.h"
#include "third_party/blink/renderer/core/html/forms/html_select_element.h"
#include "third_party/blink/renderer/core/scroll/scrollable_area.h"
#include "third_party/blink/renderer/core/style/computed_style_constants.h"
#include "third_party/blink/renderer/core/style/position_area.h"
#include "third_party/blink/renderer/platform/fonts/font_description.h"
#include "third_party/blink/renderer/platform/fonts/font_smoothing_mode.h"
#include "third_party/blink/renderer/platform/fonts/font_variant_emoji.h"
#include "third_party/blink/renderer/platform/fonts/text_rendering_mode.h"
#include "third_party/blink/renderer/platform/geometry/length.h"
#include "third_party/blink/renderer/platform/graphics/touch_action.h"
#include "third_party/blink/renderer/platform/text/writing_mode.h"
#include "third_party/blink/renderer/platform/theme_types.h"

namespace blink {

// TODO(sashab): Move these to CSSIdentifierValueMappings.h, and update to use
// the CSSValuePool.
template <>
inline CSSIdentifierValue::CSSIdentifierValue(CSSReflectionDirection e)
    : CSSValue(kIdentifierClass) {
  switch (e) {
    case kReflectionAbove:
      value_id_ = CSSValueID::kAbove;
      break;
    case kReflectionBelow:
      value_id_ = CSSValueID::kBelow;
      break;
    case kReflectionLeft:
      value_id_ = CSSValueID::kLeft;
      break;
    case kReflectionRight:
      value_id_ = CSSValueID::kRight;
  }
}

template <>
inline CSSReflectionDirection CSSIdentifierValue::ConvertTo() const {
  switch (value_id_) {
    case CSSValueID::kAbove:
      return kReflectionAbove;
    case CSSValueID::kBelow:
      return kReflectionBelow;
    case CSSValueID::kLeft:
      return kReflectionLeft;
    case CSSValueID::kRight:
      return kReflectionRight;
    default:
      break;
  }

  NOTREACHED();
}

template <>
inline EBorderStyle CSSIdentifierValue::ConvertTo() const {
  if (value_id_ == CSSValueID::kAuto) {  // Valid for CSS outline-style
    return EBorderStyle::kDotted;
  }
  return detail::cssValueIDToPlatformEnumGenerated<EBorderStyle>(value_id_);
}

template <>
inline OutlineIsAuto CSSIdentifierValue::ConvertTo() const {
  if (value_id_ == CSSValueID::kAuto) {
    return OutlineIsAuto::kOn;
  }
  return OutlineIsAuto::kOff;
}

template <>
inline CSSIdentifierValue::CSSIdentifierValue(
    CompositingOperator compositing_operator)
    : CSSValue(kIdentifierClass) {
  switch (compositing_operator) {
    case CompositingOperator::kAdd:
      value_id_ = CSSValueID::kAdd;
      break;
    case CompositingOperator::kSubtract:
      value_id_ = CSSValueID::kSubtract;
      break;
    case CompositingOperator::kIntersect:
      value_id_ = CSSValueID::kIntersect;
      break;
    case CompositingOperator::kExclude:
      value_id_ = CSSValueID::kExclude;
      break;
    case CompositingOperator::kClear:
      value_id_ = CSSValueID::kClear;
      break;
    case CompositingOperator::kCopy:
      value_id_ = CSSValueID::kCopy;
      break;
    case CompositingOperator::kSourceOver:
      value_id_ = CSSValueID::kSourceOver;
      break;
    case CompositingOperator::kSourceIn:
      value_id_ = CSSValueID::kSourceIn;
      break;
    case CompositingOperator::kSourceOut:
      value_id_ = CSSValueID::kSourceOut;
      break;
    case CompositingOperator::kSourceAtop:
      value_id_ = CSSValueID::kSourceAtop;
      break;
    case CompositingOperator::kDestinationOver:
      value_id_ = CSSValueID::kDestinationOver;
      break;
    case CompositingOperator::kDestinationIn:
      value_id_ = CSSValueID::kDestinationIn;
      break;
    case CompositingOperator::kDestinationOut:
      value_id_ = CSSValueID::kDestinationOut;
      break;
    case CompositingOperator::kDestinationAtop:
      value_id_ = CSSValueID::kDestinationAtop;
      break;
    case CompositingOperator::kXOR:
      value_id_ = CSSValueID::kXor;
      break;
    case CompositingOperator::kPlusLighter:
      value_id_ = CSSValueID::kPlusLighter;
      break;
    default:
      NOTREACHED();
  }
}

template <>
inline CompositingOperator CSSIdentifierValue::ConvertTo() const {
  switch (value_id_) {
    case CSSValueID::kAdd:
      return CompositingOperator::kAdd;
    case CSSValueID::kSubtract:
      return CompositingOperator::kSubtract;
    case CSSValueID::kIntersect:
      return CompositingOperator::kIntersect;
    case CSSValueID::kExclude:
      return CompositingOperator::kExclude;
    case CSSValueID::kClear:
      return CompositingOperator::kClear;
    case CSSValueID::kCopy:
      return CompositingOperator::kCopy;
    case CSSValueID::kSourceOver:
      return CompositingOperator::kSourceOver;
    case CSSValueID::kSourceIn:
      return CompositingOperator::kSourceIn;
    case CSSValueID::kSourceOut:
      return CompositingOperator::kSourceOut;
    case CSSValueID::kSourceAtop:
      return CompositingOperator::kSourceAtop;
    case CSSValueID::kDestinationOver:
      return CompositingOperator::kDestinationOver;
    case CSSValueID::kDestinationIn:
      return CompositingOperator::kDestinationIn;
    case CSSValueID::kDestinationOut:
      return CompositingOperator::kDestinationOut;
    case CSSValueID::kDestinationAtop:
      return CompositingOperator::kDestinationAtop;
    case CSSValueID::kXor:
      return CompositingOperator::kXOR;
    case CSSValueID::kPlusLighter:
      return CompositingOperator::kPlusLighter;
    default:
      break;
  }

  NOTREACHED();
}

template <>
inline CSSIdentifierValue::CSSIdentifierValue(AppearanceValue e)
    : CSSValue(kIdentifierClass) {
  switch (e) {
    case AppearanceValue::kNone:
    // Non standard appearance values that are not listed as
    // compat-auto must be rendered as none.
    // https://drafts.csswg.org/css-ui/#appearance-switching
    case AppearanceValue::kInnerSpinButton:
    case AppearanceValue::kMediaSlider:
    case AppearanceValue::kMediaSliderThumb:
    case AppearanceValue::kMediaVolumeSlider:
    case AppearanceValue::kMediaVolumeSliderThumb:
    case AppearanceValue::kPushButton:
    case AppearanceValue::kSearchFieldCancelButton:
    case AppearanceValue::kSliderThumbHorizontal:
    case AppearanceValue::kSliderThumbVertical:
    case AppearanceValue::kSliderHorizontal:
    case AppearanceValue::kSquareButton:
      value_id_ = CSSValueID::kNone;
      break;
    case AppearanceValue::kAuto:
      value_id_ = CSSValueID::kAuto;
      break;
    case AppearanceValue::kCheckbox:
      value_id_ = CSSValueID::kCheckbox;
      break;
    case AppearanceValue::kRadio:
      value_id_ = CSSValueID::kRadio;
      break;
    case AppearanceValue::kButton:
      value_id_ = CSSValueID::kButton;
      break;
    case AppearanceValue::kListbox:
      value_id_ = CSSValueID::kListbox;
      break;
    case AppearanceValue::kMediaControl:
      value_id_ = CSSValueID::kInternalMediaControl;
      break;
    case AppearanceValue::kMenulist:
      value_id_ = CSSValueID::kMenulist;
      break;
    case AppearanceValue::kMenulistButton:
      value_id_ = CSSValueID::kMenulistButton;
      break;
    case AppearanceValue::kMeter:
      value_id_ = CSSValueID::kMeter;
      break;
    case AppearanceValue::kProgressBar:
      value_id_ = CSSValueID::kProgressBar;
      break;
    case AppearanceValue::kSliderVertical:
      value_id_ = CSSValueID::kSliderVertical;
      break;
    case AppearanceValue::kSearchField:
      value_id_ = CSSValueID::kSearchfield;
      break;
    case AppearanceValue::kTextField:
      value_id_ = CSSValueID::kTextfield;
      break;
    case AppearanceValue::kTextArea:
      value_id_ = CSSValueID::kTextarea;
      break;
    case AppearanceValue::kBaseSelect:
      // This can't check for origin trials, unfortunately.
      DCHECK(HTMLSelectElement::CustomizableSelectEnabledNoDocument());
      value_id_ = CSSValueID::kBaseSelect;
      break;
  }
}

template <>
inline AppearanceValue CSSIdentifierValue::ConvertTo() const {
  switch (value_id_) {
    case CSSValueID::kNone:
      return AppearanceValue::kNone;
    case CSSValueID::kAuto:
      return AppearanceValue::kAuto;
    case CSSValueID::kCheckbox:
      return AppearanceValue::kCheckbox;
    case CSSValueID::kRadio:
      return AppearanceValue::kRadio;
    case CSSValueID::kButton:
      return AppearanceValue::kButton;
    case CSSValueID::kListbox:
      return AppearanceValue::kListbox;
    case CSSValueID::kInternalMediaControl:
      return AppearanceValue::kMediaControl;
    case CSSValueID::kMenulist:
      return AppearanceValue::kMenulist;
    case CSSValueID::kMenulistButton:
      return AppearanceValue::kMenulistButton;
    case CSSValueID::kMeter:
      return AppearanceValue::kMeter;
    case CSSValueID::kProgressBar:
      return AppearanceValue::kProgressBar;
    case CSSValueID::kSliderVertical:
      return AppearanceValue::kSliderVertical;
    case CSSValueID::kSearchfield:
      return AppearanceValue::kSearchField;
    case CSSValueID::kTextfield:
      return AppearanceValue::kTextField;
    case CSSValueID::kTextarea:
      return AppearanceValue::kTextArea;
    case CSSValueID::kBaseSelect:
      return AppearanceValue::kBaseSelect;
    default:
      NOTREACHED();
  }
}

template <>
inline CSSIdentifierValue::CSSIdentifierValue(EFillAttachment e)
    : CSSValue(kIdentifierClass) {
  switch (e) {
    case EFillAttachment::kScroll:
      value_id_ = CSSValueID::kScroll;
      break;
    case EFillAttachment::kLocal:
      value_id_ = CSSValueID::kLocal;
      break;
    case EFillAttachment::kFixed:
      value_id_ = CSSValueID::kFixed;
      break;
  }
}

template <>
inline EFillAttachment CSSIdentifierValue::ConvertTo() const {
  switch (value_id_) {
    case CSSValueID::kScroll:
      return EFillAttachment::kScroll;
    case CSSValueID::kLocal:
      return EFillAttachment::kLocal;
    case CSSValueID::kFixed:
      return EFillAttachment::kFixed;
    default:
      break;
  }

  NOTREACHED();
}

template <>
inline CSSIdentifierValue::CSSIdentifierValue(EFillBox e)
    : CSSValue(kIdentifierClass) {
  switch (e) {
    case EFillBox::kBorder:
      value_id_ = CSSValueID::kBorderBox;
      break;
    case EFillBox::kPadding:
      value_id_ = CSSValueID::kPaddingBox;
      break;
    case EFillBox::kContent:
      value_id_ = CSSValueID::kContentBox;
      break;
    case EFillBox::kText:
      value_id_ = CSSValueID::kText;
      break;
    case EFillBox::kFillBox:
      value_id_ = CSSValueID::kFillBox;
      break;
    case EFillBox::kStrokeBox:
      value_id_ = CSSValueID::kStrokeBox;
      break;
    case EFillBox::kViewBox:
      value_id_ = CSSValueID::kViewBox;
      break;
    case EFillBox::kNoClip:
      value_id_ = CSSValueID::kNoClip;
      break;
  }
}

template <>
inline EFillBox CSSIdentifierValue::ConvertTo() const {
  switch (value_id_) {
    case CSSValueID::kBorder:
    case CSSValueID::kBorderBox:
      return EFillBox::kBorder;
    case CSSValueID::kPadding:
    case CSSValueID::kPaddingBox:
      return EFillBox::kPadding;
    case CSSValueID::kContent:
    case CSSValueID::kContentBox:
      return EFillBox::kContent;
    case CSSValueID::kText:
      return EFillBox::kText;
    case CSSValueID::kFillBox:
      return EFillBox::kFillBox;
    case CSSValueID::kStrokeBox:
      return EFillBox::kStrokeBox;
    case CSSValueID::kViewBox:
      return EFillBox::kViewBox;
    case CSSValueID::kNoClip:
      return EFillBox::kNoClip;
    default:
      break;
  }

  NOTREACHED();
}

template <>
inline CSSIdentifierValue::CSSIdentifierValue(EFillRepeat e)
    : CSSValue(kIdentifierClass) {
  switch (e) {
    case EFillRepeat::kRepeatFill:
      value_id_ = CSSValueID::kRepeat;
      break;
    case EFillRepeat::kNoRepeatFill:
      value_id_ = CSSValueID::kNoRepeat;
      break;
    case EFillRepeat::kRoundFill:
      value_id_ = CSSValueID::kRound;
      break;
    case EFillRepeat::kSpaceFill:
      value_id_ = CSSValueID::kSpace;
      break;
  }
}

template <>
inline EFillRepeat CSSIdentifierValue::ConvertTo() const {
  switch (value_id_) {
    case CSSValueID::kRepeat:
      return EFillRepeat::kRepeatFill;
    case CSSValueID::kNoRepeat:
      return EFillRepeat::kNoRepeatFill;
    case CSSValueID::kRound:
      return EFillRepeat::kRoundFill;
    case CSSValueID::kSpace:
      return EFillRepeat::kSpaceFill;
    default:
      break;
  }
  NOTREACHED();
}

template <>
inline CSSIdentifierValue::CSSIdentifierValue(EFillMaskMode e)
    : CSSValue(kIdentifierClass) {
  switch (e) {
    case EFillMaskMode::kAlpha:
      value_id_ = CSSValueID::kAlpha;
      break;
    case EFillMaskMode::kLuminance:
      value_id_ = CSSValueID::kLuminance;
      break;
    case EFillMaskMode::kMatchSource:
      value_id_ = CSSValueID::kMatchSource;
      break;
    default:
      NOTREACHED();
  }
}

template <>
inline EFillMaskMode CSSIdentifierValue::ConvertTo() const {
  switch (value_id_) {
    case CSSValueID::kAlpha:
      return EFillMaskMode::kAlpha;
    case CSSValueID::kLuminance:
      return EFillMaskMode::kLuminance;
    case CSSValueID::kMatchSource:
      return EFillMaskMode::kMatchSource;
    default:
      NOTREACHED();
  }
}

template <>
inline CSSIdentifierValue::CSSIdentifierValue(BackgroundEdgeOrigin e)
    : CSSValue(kIdentifierClass) {
  switch (e) {
    case BackgroundEdgeOrigin::kTop:
      value_id_ = CSSValueID::kTop;
      break;
    case BackgroundEdgeOrigin::kRight:
      value_id_ = CSSValueID::kRight;
      break;
    case BackgroundEdgeOrigin::kBottom:
      value_id_ = CSSValueID::kBottom;
      break;
    case BackgroundEdgeOrigin::kLeft:
      value_id_ = CSSValueID::kLeft;
      break;
  }
}

template <>
inline BackgroundEdgeOrigin CSSIdentifierValue::ConvertTo() const {
  switch (value_id_) {
    case CSSValueID::kTop:
      return BackgroundEdgeOrigin::kTop;
    case CSSValueID::kRight:
      return BackgroundEdgeOrigin::kRight;
    case CSSValueID::kBottom:
      return BackgroundEdgeOrigin::kBottom;
    case CSSValueID::kLeft:
      return BackgroundEdgeOrigin::kLeft;
    default:
      break;
  }

  NOTREACHED();
}

template <>
inline CSSIdentifierValue::CSSIdentifierValue(EFloat e)
    : CSSValue(kIdentifierClass) {
  switch (e) {
    case EFloat::kNone:
      value_id_ = CSSValueID::kNone;
      break;
    case EFloat::kLeft:
      value_id_ = CSSValueID::kLeft;
      break;
    case EFloat::kRight:
      value_id_ = CSSValueID::kRight;
      break;
    case EFloat::kInlineStart:
      value_id_ = CSSValueID::kInlineStart;
      break;
    case EFloat::kInlineEnd:
      value_id_ = CSSValueID::kInlineEnd;
      break;
  }
}

template <>
inline EFloat CSSIdentifierValue::ConvertTo() const {
  switch (value_id_) {
    case CSSValueID::kLeft:
      return EFloat::kLeft;
    case CSSValueID::kRight:
      return EFloat::kRight;
    case CSSValueID::kInlineStart:
      return EFloat::kInlineStart;
    case CSSValueID::kInlineEnd:
      return EFloat::kInlineEnd;
    case CSSValueID::kNone:
      return EFloat::kNone;
    default:
      break;
  }

  NOTREACHED();
}

template <>
inline CSSIdentifierValue::CSSIdentifierValue(EPosition e)
    : CSSValue(kIdentifierClass) {
  switch (e) {
    case EPosition::kStatic:
      value_id_ = CSSValueID::kStatic;
      break;
    case EPosition::kRelative:
      value_id_ = CSSValueID::kRelative;
      break;
    case EPosition::kAbsolute:
      value_id_ = CSSValueID::kAbsolute;
      break;
    case EPosition::kFixed:
      value_id_ = CSSValueID::kFixed;
      break;
    case EPosition::kSticky:
      value_id_ = CSSValueID::kSticky;
      break;
  }
}

template <>
inline EPosition CSSIdentifierValue::ConvertTo() const {
  switch (value_id_) {
    case CSSValueID::kStatic:
      return EPosition::kStatic;
    case CSSValueID::kRelative:
      return EPosition::kRelative;
    case CSSValueID::kAbsolute:
      return EPosition::kAbsolute;
    case CSSValueID::kFixed:
      return EPosition::kFixed;
    case CSSValueID::kSticky:
      return EPosition::kSticky;
    default:
      break;
  }

  NOTREACHED();
}

template <>
inline CSSIdentifierValue::CSSIdentifierValue(ETableLayout e)
    : CSSValue(kIdentifierClass) {
  switch (e) {
    case ETableLayout::kAuto:
      value_id_ = CSSValueID::kAuto;
      break;
    case ETableLayout::kFixed:
      value_id_ = CSSValueID::kFixed;
      break;
  }
}

template <>
inline ETableLayout CSSIdentifierValue::ConvertTo() const {
  switch (value_id_) {
    case CSSValueID::kFixed:
      return ETableLayout::kFixed;
    case CSSValueID::kAuto:
      return ETableLayout::kAuto;
    default:
      break;
  }

  NOTREACHED();
}

template <>
inline CSSIdentifierValue::CSSIdentifierValue(EVerticalAlign a)
    : CSSValue(kIdentifierClass) {
  switch (a) {
    case EVerticalAlign::kTop:
      value_id_ = CSSValueID::kTop;
      break;
    case EVerticalAlign::kBottom:
      value_id_ = CSSValueID::kBottom;
      break;
    case EVerticalAlign::kMiddle:
      value_id_ = CSSValueID::kMiddle;
      break;
    case EVerticalAlign::kBaseline:
      value_id_ = CSSValueID::kBaseline;
      break;
    case EVerticalAlign::kTextBottom:
      value_id_ = CSSValueID::kTextBottom;
      break;
    case EVerticalAlign::kTextTop:
      value_id_ = CSSValueID::kTextTop;
      break;
    case EVerticalAlign::kSub:
      value_id_ = CSSValueID::kSub;
      break;
    case EVerticalAlign::kSuper:
      value_id_ = CSSValueID::kSuper;
      break;
    case EVerticalAlign::kBaselineMiddle:
      value_id_ = CSSValueID::kWebkitBaselineMiddle;
      break;
    case EVerticalAlign::kLength:
      value_id_ = CSSValueID::kInvalid;
  }
}

template <>
inline EVerticalAlign CSSIdentifierValue::ConvertTo() const {
  switch (value_id_) {
    case CSSValueID::kTop:
      return EVerticalAlign::kTop;
    case CSSValueID::kBottom:
      return EVerticalAlign::kBottom;
    case CSSValueID::kMiddle:
      return EVerticalAlign::kMiddle;
    case CSSValueID::kBaseline:
      return EVerticalAlign::kBaseline;
    case CSSValueID::kTextBottom:
      return EVerticalAlign::kTextBottom;
    case CSSValueID::kTextTop:
      return EVerticalAlign::kTextTop;
    case CSSValueID::kSub:
      return EVerticalAlign::kSub;
    case CSSValueID::kSuper:
      return EVerticalAlign::kSuper;
    case CSSValueID::kWebkitBaselineMiddle:
      return EVerticalAlign::kBaselineMiddle;
    default:
      break;
  }

  NOTREACHED();
}

template <>
inline CSSIdentifierValue::CSSIdentifierValue(TextEmphasisFill fill)
    : CSSValue(kIdentifierClass) {
  switch (fill) {
    case TextEmphasisFill::kFilled:
      value_id_ = CSSValueID::kFilled;
      break;
    case TextEmphasisFill::kOpen:
      value_id_ = CSSValueID::kOpen;
      break;
  }
}

template <>
inline TextEmphasisFill CSSIdentifierValue::ConvertTo() const {
  switch (value_id_) {
    case CSSValueID::kFilled:
      return TextEmphasisFill::kFilled;
    case CSSValueID::kOpen:
      return TextEmphasisFill::kOpen;
    default:
      break;
  }

  NOTREACHED();
}

template <>
inline CSSIdentifierValue::CSSIdentifierValue(TextEmphasisMark mark)
    : CSSValue(kIdentifierClass) {
  switch (mark) {
    case TextEmphasisMark::kDot:
      value_id_ = CSSValueID::kDot;
      break;
    case TextEmphasisMark::kCircle:
      value_id_ = CSSValueID::kCircle;
      break;
    case TextEmphasisMark::kDoubleCircle:
      value_id_ = CSSValueID::kDoubleCircle;
      break;
    case TextEmphasisMark::kTriangle:
      value_id_ = CSSValueID::kTriangle;
      break;
    case TextEmphasisMark::kSesame:
      value_id_ = CSSValueID::kSesame;
      break;
    case TextEmphasisMark::kNone:
    case TextEmphasisMark::kAuto:
    case TextEmphasisMark::kCustom:
      NOTREACHED();
  }
}

template <>
inline TextEmphasisMark CSSIdentifierValue::ConvertTo() const {
  switch (value_id_) {
    case CSSValueID::kNone:
      return TextEmphasisMark::kNone;
    case CSSValueID::kDot:
      return TextEmphasisMark::kDot;
    case CSSValueID::kCircle:
      return TextEmphasisMark::kCircle;
    case CSSValueID::kDoubleCircle:
      return TextEmphasisMark::kDoubleCircle;
    case CSSValueID::kTriangle:
      return TextEmphasisMark::kTriangle;
    case CSSValueID::kSesame:
      return TextEmphasisMark::kSesame;
    default:
      break;
  }

  NOTREACHED();
}

template <>
inline CSSIdentifierValue::CSSIdentifierValue(
    FontSizeAdjust::Metric font_size_adjust_metric)
    : CSSValue(kIdentifierClass) {
  switch (font_size_adjust_metric) {
    case FontSizeAdjust::Metric::kExHeight:
      value_id_ = CSSValueID::kExHeight;
      return;
    case FontSizeAdjust::Metric::kCapHeight:
      value_id_ = CSSValueID::kCapHeight;
      return;
    case FontSizeAdjust::Metric::kChWidth:
      value_id_ = CSSValueID::kChWidth;
      return;
    case FontSizeAdjust::Metric::kIcWidth:
      value_id_ = CSSValueID::kIcWidth;
      return;
    case FontSizeAdjust::Metric::kIcHeight:
      value_id_ = CSSValueID::kIcHeight;
      return;
  }

  NOTREACHED();
}

template <>
inline FontSizeAdjust::Metric CSSIdentifierValue::ConvertTo() const {
  switch (value_id_) {
    case CSSValueID::kExHeight:
      return FontSizeAdjust::Metric::kExHeight;
    case CSSValueID::kCapHeight:
      return FontSizeAdjust::Metric::kCapHeight;
    case CSSValueID::kChWidth:
      return FontSizeAdjust::Metric::kChWidth;
    case CSSValueID::kIcWidth:
      return FontSizeAdjust::Metric::kIcWidth;
    case CSSValueID::kIcHeight:
      return FontSizeAdjust::Metric::kIcHeight;
    default:
      break;
  }

  NOTREACHED();
}

template <>
inline CSSIdentifierValue::CSSIdentifierValue(
    FontDescription::FontSynthesisWeight font_synthesis_weight)
    : CSSValue(kIdentifierClass) {
  switch (font_synthesis_weight) {
    case FontDescription::kAutoFontSynthesisWeight:
      value_id_ = CSSValueID::kAuto;
      return;
    case FontDescription::kNoneFontSynthesisWeight:
      value_id_ = CSSValueID::kNone;
      return;
  }

  NOTREACHED();
}

template <>
inline FontDescription::FontSynthesisWeight CSSIdentifierValue::ConvertTo()
    const {
  switch (value_id_) {
    case CSSValueID::kAuto:
      return FontDescription::kAutoFontSynthesisWeight;
    case CSSValueID::kNone:
      return FontDescription::kNoneFontSynthesisWeight;
    default:
      break;
  }

  NOTREACHED();
}

template <>
inline CSSIdentifierValue::CSSIdentifierValue(
    FontDescription::FontSynthesisStyle font_synthesis_style)
    : CSSValue(kIdentifierClass) {
  switch (font_synthesis_style) {
    case FontDescription::kAutoFontSynthesisStyle:
      value_id_ = CSSValueID::kAuto;
      return;
    case FontDescription::kNoneFontSynthesisStyle:
      value_id_ = CSSValueID::kNone;
      return;
  }

  NOTREACHED();
}

template <>
inline FontDescription::FontSynthesisStyle CSSIdentifierValue::ConvertTo()
    const {
  switch (value_id_) {
    case CSSValueID::kAuto:
      return FontDescription::kAutoFontSynthesisStyle;
    case CSSValueID::kNone:
      return FontDescription::kNoneFontSynthesisStyle;
    default:
      break;
  }

  NOTREACHED();
}

template <>
inline CSSIdentifierValue::CSSIdentifierValue(
    FontDescription::FontSynthesisSmallCaps font_synthesis_small_caps)
    : CSSValue(kIdentifierClass) {
  switch (font_synthesis_small_caps) {
    case FontDescription::kAutoFontSynthesisSmallCaps:
      value_id_ = CSSValueID::kAuto;
      return;
    case FontDescription::kNoneFontSynthesisSmallCaps:
      value_id_ = CSSValueID::kNone;
      return;
  }

  NOTREACHED();
}

template <>
inline FontDescription::FontSynthesisSmallCaps CSSIdentifierValue::ConvertTo()
    const {
  switch (value_id_) {
    case CSSValueID::kAuto:
      return FontDescription::kAutoFontSynthesisSmallCaps;
    case CSSValueID::kNone:
      return FontDescription::kNoneFontSynthesisSmallCaps;
    default:
      break;
  }

  NOTREACHED();
}

template <>
inline CSSIdentifierValue::CSSIdentifierValue(EFillSizeType fill_size)
    : CSSValue(kIdentifierClass) {
  switch (fill_size) {
    case EFillSizeType::kContain:
      value_id_ = CSSValueID::kContain;
      break;
    case EFillSizeType::kCover:
      value_id_ = CSSValueID::kCover;
      break;
    case EFillSizeType::kSizeNone:
    case EFillSizeType::kSizeLength:
    default:
      NOTREACHED();
  }
}

template <>
inline CSSIdentifierValue::CSSIdentifierValue(FontSmoothingMode smoothing)
    : CSSValue(kIdentifierClass) {
  switch (smoothing) {
    case kAutoSmoothing:
      value_id_ = CSSValueID::kAuto;
      return;
    case kNoSmoothing:
      value_id_ = CSSValueID::kNone;
      return;
    case kAntialiased:
      value_id_ = CSSValueID::kAntialiased;
      return;
    case kSubpixelAntialiased:
      value_id_ = CSSValueID::kSubpixelAntialiased;
      return;
  }

  NOTREACHED();
}

template <>
inline FontSmoothingMode CSSIdentifierValue::ConvertTo() const {
  switch (value_id_) {
    case CSSValueID::kAuto:
      return kAutoSmoothing;
    case CSSValueID::kNone:
      return kNoSmoothing;
    case CSSValueID::kAntialiased:
      return kAntialiased;
    case CSSValueID::kSubpixelAntialiased:
      return kSubpixelAntialiased;
    default:
      break;
  }

  NOTREACHED();
}

template <>
inline CSSIdentifierValue::CSSIdentifierValue(FontVariantEmoji variant_emoji)
    : CSSValue(kIdentifierClass) {
  switch (variant_emoji) {
    case kNormalVariantEmoji:
      value_id_ = CSSValueID::kNormal;
      return;
    case kTextVariantEmoji:
      value_id_ = CSSValueID::kText;
      return;
    case kEmojiVariantEmoji:
      value_id_ = CSSValueID::kEmoji;
      return;
    case kUnicodeVariantEmoji:
      value_id_ = CSSValueID::kUnicode;
      return;
  }

  NOTREACHED();
}

template <>
inline FontVariantEmoji CSSIdentifierValue::ConvertTo() const {
  switch (value_id_) {
    case CSSValueID::kNormal:
      return kNormalVariantEmoji;
    case CSSValueID::kText:
      return kTextVariantEmoji;
    case CSSValueID::kEmoji:
      return kEmojiVariantEmoji;
    case CSSValueID::kUnicode:
      return kUnicodeVariantEmoji;
    default:
      break;
  }

  NOTREACHED();
}

template <>
inline CSSIdentifierValue::CSSIdentifierValue(TextRenderingMode e)
    : CSSValue(kIdentifierClass) {
  switch (e) {
    case kAutoTextRendering:
      value_id_ = CSSValueID::kAuto;
      break;
    case kOptimizeSpeed:
      value_id_ = CSSValueID::kOptimizespeed;
      break;
    case kOptimizeLegibility:
      value_id_ = CSSValueID::kOptimizelegibility;
      break;
    case kGeometricPrecision:
      value_id_ = CSSValueID::kGeometricprecision;
      break;
  }
}

template <>
inline TextRenderingMode CSSIdentifierValue::ConvertTo() const {
  switch (value_id_) {
    case CSSValueID::kAuto:
      return kAutoTextRendering;
    case CSSValueID::kOptimizespeed:
      return kOptimizeSpeed;
    case CSSValueID::kOptimizelegibility:
      return kOptimizeLegibility;
    case CSSValueID::kGeometricprecision:
      return kGeometricPrecision;
    default:
      break;
  }

  NOTREACHED();
}

template <>
inline EOrder CSSIdentifierValue::ConvertTo() const {
  switch (value_id_) {
    case CSSValueID::kLogical:
      return EOrder::kLogical;
    case CSSValueID::kVisual:
      return EOrder::kVisual;
    default:
      break;
  }

  NOTREACHED();
}

template <>
inline CSSIdentifierValue::CSSIdentifierValue(EOrder e)
    : CSSValue(kIdentifierClass) {
  switch (e) {
    case EOrder::kLogical:
      value_id_ = CSSValueID::kLogical;
      break;
    case EOrder::kVisual:
      value_id_ = CSSValueID::kVisual;
      break;
  }
}

template <>
inline CSSIdentifierValue::CSSIdentifierValue(LineCap e)
    : CSSValue(kIdentifierClass) {
  switch (e) {
    case kButtCap:
      value_id_ = CSSValueID::kButt;
      break;
    case kRoundCap:
      value_id_ = CSSValueID::kRound;
      break;
    case kSquareCap:
      value_id_ = CSSValueID::kSquare;
      break;
  }
}

template <>
inline LineCap CSSIdentifierValue::ConvertTo() const {
  switch (value_id_) {
    case CSSValueID::kButt:
      return kButtCap;
    case CSSValueID::kRound:
      return kRoundCap;
    case CSSValueID::kSquare:
      return kSquareCap;
    default:
      break;
  }

  NOTREACHED();
}

template <>
inline CSSIdentifierValue::CSSIdentifierValue(LineJoin e)
    : CSSValue(kIdentifierClass) {
  switch (e) {
    case kMiterJoin:
      value_id_ = CSSValueID::kMiter;
      break;
    case kRoundJoin:
      value_id_ = CSSValueID::kRound;
      break;
    case kBevelJoin:
      value_id_ = CSSValueID::kBevel;
      break;
  }
}

template <>
inline LineJoin CSSIdentifierValue::ConvertTo() const {
  switch (value_id_) {
    case CSSValueID::kMiter:
      return kMiterJoin;
    case CSSValueID::kRound:
      return kRoundJoin;
    case CSSValueID::kBevel:
      return kBevelJoin;
    default:
      break;
  }

  NOTREACHED();
}

template <>
inline CSSIdentifierValue::CSSIdentifierValue(WindRule e)
    : CSSValue(kIdentifierClass) {
  switch (e) {
    case RULE_NONZERO:
      value_id_ = CSSValueID::kNonzero;
      break;
    case RULE_EVENODD:
      value_id_ = CSSValueID::kEvenodd;
      break;
  }
}

template <>
inline WindRule CSSIdentifierValue::ConvertTo() const {
  switch (value_id_) {
    case CSSValueID::kNonzero:
      return RULE_NONZERO;
    case CSSValueID::kEvenodd:
      return RULE_EVENODD;
    default:
      break;
  }

  NOTREACHED();
}

template <>
inline CSSIdentifierValue::CSSIdentifierValue(EPaintOrderType e)
    : CSSValue(kIdentifierClass) {
  switch (e) {
    case PT_FILL:
      value_id_ = CSSValueID::kFill;
      break;
    case PT_STROKE:
      value_id_ = CSSValueID::kStroke;
      break;
    case PT_MARKERS:
      value_id_ = CSSValueID::kMarkers;
      break;
    default:
      NOTREACHED();
  }
}

template <>
inline EPaintOrderType CSSIdentifierValue::ConvertTo() const {
  switch (value_id_) {
    case CSSValueID::kFill:
      return PT_FILL;
    case CSSValueID::kStroke:
      return PT_STROKE;
    case CSSValueID::kMarkers:
      return PT_MARKERS;
    default:
      break;
  }

  NOTREACHED();
}

template <>
inline TouchAction CSSIdentifierValue::ConvertTo() const {
  switch (value_id_) {
    case CSSValueID::kNone:
      return TouchAction::kNone;
    case CSSValueID::kAuto:
      return TouchAction::kAuto;
    case CSSValueID::kPanLeft:
      return TouchAction::kPanLeft;
    case CSSValueID::kPanRight:
      return TouchAction::kPanRight;
    case CSSValueID::kPanX:
      return TouchAction::kPanX;
    case CSSValueID::kPanUp:
      return TouchAction::kPanUp;
    case CSSValueID::kPanDown:
      return TouchAction::kPanDown;
    case CSSValueID::kPanY:
      return TouchAction::kPanY;
    case CSSValueID::kManipulation:
      return TouchAction::kManipulation;
    case CSSValueID::kPinchZoom:
      return TouchAction::kPinchZoom;
    default:
      break;
  }

  NOTREACHED();
}

template <>
inline CSSIdentifierValue::CSSIdentifierValue(CSSBoxType css_box)
    : CSSValue(kIdentifierClass) {
  switch (css_box) {
    case CSSBoxType::kMargin:
      value_id_ = CSSValueID::kMarginBox;
      break;
    case CSSBoxType::kBorder:
      value_id_ = CSSValueID::kBorderBox;
      break;
    case CSSBoxType::kPadding:
      value_id_ = CSSValueID::kPaddingBox;
      break;
    case CSSBoxType::kContent:
      value_id_ = CSSValueID::kContentBox;
      break;
    case CSSBoxType::kMissing:
      // The missing box should convert to a null value.
      NOTREACHED();
  }
}

template <>
inline CSSBoxType CSSIdentifierValue::ConvertTo() const {
  switch (GetValueID()) {
    case CSSValueID::kMarginBox:
      return CSSBoxType::kMargin;
    case CSSValueID::kBorderBox:
      return CSSBoxType::kBorder;
    case CSSValueID::kPaddingBox:
      return CSSBoxType::kPadding;
    case CSSValueID::kContentBox:
      return CSSBoxType::kContent;
    default:
      break;
  }
  NOTREACHED();
}

template <>
inline CSSIdentifierValue::CSSIdentifierValue(ItemPosition item_position)
    : CSSValue(kIdentifierClass) {
  switch (item_position) {
    case ItemPosition::kLegacy:
      value_id_ = CSSValueID::kLegacy;
      break;
    case ItemPosition::kAuto:
      value_id_ = CSSValueID::kAuto;
      break;
    case ItemPosition::kNormal:
      value_id_ = CSSValueID::kNormal;
      break;
    case ItemPosition::kStretch:
      value_id_ = CSSValueID::kStretch;
      break;
    case ItemPosition::kBaseline:
      value_id_ = CSSValueID::kBaseline;
      break;
    case ItemPosition::kLastBaseline:
      value_id_ = CSSValueID::kLastBaseline;
      break;
    case ItemPosition::kAnchorCenter:
      value_id_ = CSSValueID::kAnchorCenter;
      break;
    case ItemPosition::kCenter:
      value_id_ = CSSValueID::kCenter;
      break;
    case ItemPosition::kStart:
      value_id_ = CSSValueID::kStart;
      break;
    case ItemPosition::kEnd:
      value_id_ = CSSValueID::kEnd;
      break;
    case ItemPosition::kSelfStart:
      value_id_ = CSSValueID::kSelfStart;
      break;
    case ItemPosition::kSelfEnd:
      value_id_ = CSSValueID::kSelfEnd;
      break;
    case ItemPosition::kFlexStart:
      value_id_ = CSSValueID::kFlexStart;
      break;
    case ItemPosition::kFlexEnd:
      value_id_ = CSSValueID::kFlexEnd;
      break;
    case ItemPosition::kLeft:
      value_id_ = CSSValueID::kLeft;
      break;
    case ItemPosition::kRight:
      value_id_ = CSSValueID::kRight;
      break;
  }
}

template <>
inline ItemPosition CSSIdentifierValue::ConvertTo() const {
  switch (value_id_) {
    case CSSValueID::kLegacy:
      return ItemPosition::kLegacy;
    case CSSValueID::kAuto:
      return ItemPosition::kAuto;
    case CSSValueID::kNormal:
      return ItemPosition::kNormal;
    case CSSValueID::kStretch:
      return ItemPosition::kStretch;
    case CSSValueID::kBaseline:
      return ItemPosition::kBaseline;
    case CSSValueID::kFirstBaseline:
      return ItemPosition::kBaseline;
    case CSSValueID::kLastBaseline:
      return ItemPosition::kLastBaseline;
    case CSSValueID::kAnchorCenter:
      return ItemPosition::kAnchorCenter;
    case CSSValueID::kCenter:
      return ItemPosition::kCenter;
    case CSSValueID::kStart:
      return ItemPosition::kStart;
    case CSSValueID::kEnd:
      return ItemPosition::kEnd;
    case CSSValueID::kSelfStart:
      return ItemPosition::kSelfStart;
    case CSSValueID::kSelfEnd:
      return ItemPosition::kSelfEnd;
    case CSSValueID::kFlexStart:
      return ItemPosition::kFlexStart;
    case CSSValueID::kFlexEnd:
      return ItemPosition::kFlexEnd;
    case CSSValueID::kLeft:
      return ItemPosition::kLeft;
    case CSSValueID::kRight:
      return ItemPosition::kRight;
    default:
      break;
  }
  NOTREACHED();
}

template <>
inline CSSIdentifierValue::CSSIdentifierValue(ContentPosition content_position)
    : CSSValue(kIdentifierClass) {
  switch (content_position) {
    case ContentPosition::kNormal:
      value_id_ = CSSValueID::kNormal;
      break;
    case ContentPosition::kBaseline:
      value_id_ = CSSValueID::kBaseline;
      break;
    case ContentPosition::kLastBaseline:
      value_id_ = CSSValueID::kLastBaseline;
      break;
    case ContentPosition::kCenter:
      value_id_ = CSSValueID::kCenter;
      break;
    case ContentPosition::kStart:
      value_id_ = CSSValueID::kStart;
      break;
    case ContentPosition::kEnd:
      value_id_ = CSSValueID::kEnd;
      break;
    case ContentPosition::kFlexStart:
      value_id_ = CSSValueID::kFlexStart;
      break;
    case ContentPosition::kFlexEnd:
      value_id_ = CSSValueID::kFlexEnd;
      break;
    case ContentPosition::kLeft:
      value_id_ = CSSValueID::kLeft;
      break;
    case ContentPosition::kRight:
      value_id_ = CSSValueID::kRight;
      break;
  }
}

template <>
inline ContentPosition CSSIdentifierValue::ConvertTo() const {
  switch (value_id_) {
    case CSSValueID::kNormal:
      return ContentPosition::kNormal;
    case CSSValueID::kBaseline:
      return ContentPosition::kBaseline;
    case CSSValueID::kFirstBaseline:
      return ContentPosition::kBaseline;
    case CSSValueID::kLastBaseline:
      return ContentPosition::kLastBaseline;
    case CSSValueID::kCenter:
      return ContentPosition::kCenter;
    case CSSValueID::kStart:
      return ContentPosition::kStart;
    case CSSValueID::kEnd:
      return ContentPosition::kEnd;
    case CSSValueID::kFlexStart:
      return ContentPosition::kFlexStart;
    case CSSValueID::kFlexEnd:
      return ContentPosition::kFlexEnd;
    case CSSValueID::kLeft:
      return ContentPosition::kLeft;
    case CSSValueID::kRight:
      return ContentPosition::kRight;
    default:
      break;
  }
  NOTREACHED();
}

template <>
inline CSSIdentifierValue::CSSIdentifierValue(
    ContentDistributionType content_distribution)
    : CSSValue(kIdentifierClass) {
  switch (content_distribution) {
    case ContentDistributionType::kDefault:
      value_id_ = CSSValueID::kDefault;
      break;
    case ContentDistributionType::kSpaceBetween:
      value_id_ = CSSValueID::kSpaceBetween;
      break;
    case ContentDistributionType::kSpaceAround:
      value_id_ = CSSValueID::kSpaceAround;
      break;
    case ContentDistributionType::kSpaceEvenly:
      value_id_ = CSSValueID::kSpaceEvenly;
      break;
    case ContentDistributionType::kStretch:
      value_id_ = CSSValueID::kStretch;
      break;
  }
}

template <>
inline ContentDistributionType CSSIdentifierValue::ConvertTo() const {
  switch (value_id_) {
    case CSSValueID::kSpaceBetween:
      return ContentDistributionType::kSpaceBetween;
    case CSSValueID::kSpaceAround:
      return ContentDistributionType::kSpaceAround;
    case CSSValueID::kSpaceEvenly:
      return ContentDistributionType::kSpaceEvenly;
    case CSSValueID::kStretch:
      return ContentDistributionType::kStretch;
    default:
      break;
  }
  NOTREACHED();
}

template <>
inline CSSIdentifierValue::CSSIdentifierValue(
    OverflowAlignment overflow_alignment)
    : CSSValue(kIdentifierClass) {
  switch (overflow_alignment) {
    case OverflowAlignment::kDefault:
      value_id_ = CSSValueID::kDefault;
      break;
    case OverflowAlignment::kUnsafe:
      value_id_ = CSSValueID::kUnsafe;
      break;
    case OverflowAlignment::kSafe:
      value_id_ = CSSValueID::kSafe;
      break;
  }
}

template <>
inline OverflowAlignment CSSIdentifierValue::ConvertTo() const {
  switch (value_id_) {
    case CSSValueID::kUnsafe:
      return OverflowAlignment::kUnsafe;
    case CSSValueID::kSafe:
      return OverflowAlignment::kSafe;
    default:
      break;
  }
  NOTREACHED();
}

template <>
inline CSSIdentifierValue::CSSIdentifierValue(
    mojom::blink::ScrollBehavior behavior)
    : CSSValue(kIdentifierClass) {
  switch (behavior) {
    case mojom::blink::ScrollBehavior::kAuto:
      value_id_ = CSSValueID::kAuto;
      break;
    case mojom::blink::ScrollBehavior::kSmooth:
      value_id_ = CSSValueID::kSmooth;
      break;
    case mojom::blink::ScrollBehavior::kInstant:
      // Behavior 'instant' is only allowed in ScrollOptions arguments passed to
      // CSSOM scroll APIs.
      NOTREACHED();
  }
}

template <>
inline mojom::blink::ScrollBehavior CSSIdentifierValue::ConvertTo() const {
  switch (GetValueID()) {
    case CSSValueID::kAuto:
      return mojom::blink::ScrollBehavior::kAuto;
    case CSSValueID::kSmooth:
      return mojom::blink::ScrollBehavior::kSmooth;
    default:
      break;
  }
  NOTREACHED();
}

template <>
inline CSSIdentifierValue::CSSIdentifierValue(cc::SnapAxis axis)
    : CSSValue(kIdentifierClass) {
  switch (axis) {
    case cc::SnapAxis::kX:
      value_id_ = CSSValueID::kX;
      break;
    case cc::SnapAxis::kY:
      value_id_ = CSSValueID::kY;
      break;
    case cc::SnapAxis::kBlock:
      value_id_ = CSSValueID::kBlock;
      break;
    case cc::SnapAxis::kInline:
      value_id_ = CSSValueID::kInline;
      break;
    case cc::SnapAxis::kBoth:
      value_id_ = CSSValueID::kBoth;
      break;
  }
}

template <>
inline cc::SnapAxis CSSIdentifierValue::ConvertTo() const {
  switch (GetValueID()) {
    case CSSValueID::kX:
      return cc::SnapAxis::kX;
    case CSSValueID::kY:
      return cc::SnapAxis::kY;
    case CSSValueID::kBlock:
      return cc::SnapAxis::kBlock;
    case CSSValueID::kInline:
      return cc::SnapAxis::kInline;
    case CSSValueID::kBoth:
      return cc::SnapAxis::kBoth;
    default:
      break;
  }
  NOTREACHED();
}

template <>
inline CSSIdentifierValue::CSSIdentifierValue(cc::SnapStrictness strictness)
    : CSSValue(kIdentifierClass) {
  switch (strictness) {
    case cc::SnapStrictness::kProximity:
      value_id_ = CSSValueID::kProximity;
      break;
    case cc::SnapStrictness::kMandatory:
      value_id_ = CSSValueID::kMandatory;
      break;
  }
}

template <>
inline cc::SnapStrictness CSSIdentifierValue::ConvertTo() const {
  switch (GetValueID()) {
    case CSSValueID::kProximity:
      return cc::SnapStrictness::kProximity;
    case CSSValueID::kMandatory:
      return cc::SnapStrictness::kMandatory;
    default:
      break;
  }
  NOTREACHED();
}

template <>
inline CSSIdentifierValue::CSSIdentifierValue(cc::SnapAlignment alignment)
    : CSSValue(kIdentifierClass) {
  switch (alignment) {
    case cc::SnapAlignment::kNone:
      value_id_ = CSSValueID::kNone;
      break;
    case cc::SnapAlignment::kStart:
      value_id_ = CSSValueID::kStart;
      break;
    case cc::SnapAlignment::kEnd:
      value_id_ = CSSValueID::kEnd;
      break;
    case cc::SnapAlignment::kCenter:
      value_id_ = CSSValueID::kCenter;
      break;
  }
}

template <>
inline cc::SnapAlignment CSSIdentifierValue::ConvertTo() const {
  switch (GetValueID()) {
    case CSSValueID::kNone:
      return cc::SnapAlignment::kNone;
    case CSSValueID::kStart:
      return cc::SnapAlignment::kStart;
    case CSSValueID::kEnd:
      return cc::SnapAlignment::kEnd;
    case CSSValueID::kCenter:
      return cc::SnapAlignment::kCenter;
    default:
      break;
  }
  NOTREACHED();
}

template <>
inline Containment CSSIdentifierValue::ConvertTo() const {
  switch (GetValueID()) {
    case CSSValueID::kNone:
      return kContainsNone;
    case CSSValueID::kStrict:
      return kContainsStrict;
    case CSSValueID::kContent:
      return kContainsContent;
    case CSSValueID::kPaint:
      return kContainsPaint;
    case CSSValueID::kStyle:
      return kContainsStyle;
    case CSSValueID::kLayout:
      return kContainsLayout;
    case CSSValueID::kSize:
      return kContainsSize;
    case CSSValueID::kInlineSize:
      return kContainsInlineSize;
    default:
      break;
  }
  NOTREACHED();
}

template <>
inline EContainerType CSSIdentifierValue::ConvertTo() const {
  switch (GetValueID()) {
    case CSSValueID::kNormal:
      return kContainerTypeNormal;
    case CSSValueID::kInlineSize:
      return kContainerTypeInlineSize;
    case CSSValueID::kSize:
      return kContainerTypeSize;
    case CSSValueID::kScrollState:
      return kContainerTypeScrollState;
    default:
      break;
  }
  NOTREACHED();
}

template <>
inline CSSIdentifierValue::CSSIdentifierValue(TextUnderlinePosition position)
    : CSSValue(kIdentifierClass) {
  switch (position) {
    case TextUnderlinePosition::kAuto:
      value_id_ = CSSValueID::kAuto;
      break;
    case TextUnderlinePosition::kFromFont:
      value_id_ = CSSValueID::kFromFont;
      break;
    case TextUnderlinePosition::kUnder:
      value_id_ = CSSValueID::kUnder;
      break;
    case TextUnderlinePosition::kLeft:
      value_id_ = CSSValueID::kLeft;
      break;
    case TextUnderlinePosition::kRight:
      value_id_ = CSSValueID::kRight;
      break;
  }
}

template <>
inline CoordBox CSSIdentifierValue::ConvertTo() const {
  switch (GetValueID()) {
    case CSSValueID::kContentBox:
      return CoordBox::kContentBox;
    case CSSValueID::kPaddingBox:
      return CoordBox::kPaddingBox;
    case CSSValueID::kBorderBox:
      return CoordBox::kBorderBox;
    case CSSValueID::kFillBox:
      return CoordBox::kFillBox;
    case CSSValueID::kStrokeBox:
      return CoordBox::kStrokeBox;
    case CSSValueID::kViewBox:
      return CoordBox::kViewBox;
    default:
      break;
  }
  NOTREACHED();
}

template <>
inline CSSIdentifierValue::CSSIdentifierValue(CoordBox coord_box)
    : CSSValue(kIdentifierClass) {
  switch (coord_box) {
    case CoordBox::kContentBox:
      value_id_ = CSSValueID::kContentBox;
      break;
    case CoordBox::kPaddingBox:
      value_id_ = CSSValueID::kPaddingBox;
      break;
    case CoordBox::kBorderBox:
      value_id_ = CSSValueID::kBorderBox;
      break;
    case CoordBox::kFillBox:
      value_id_ = CSSValueID::kFillBox;
      break;
    case CoordBox::kStrokeBox:
      value_id_ = CSSValueID::kStrokeBox;
      break;
    case CoordBox::kViewBox:
      value_id_ = CSSValueID::kViewBox;
      break;
  }
}

template <>
inline GeometryBox CSSIdentifierValue::ConvertTo() const {
  switch (GetValueID()) {
    case CSSValueID::kBorderBox:
      return GeometryBox::kBorderBox;
    case CSSValueID::kPaddingBox:
      return GeometryBox::kPaddingBox;
    case CSSValueID::kContentBox:
      return GeometryBox::kContentBox;
    case CSSValueID::kMarginBox:
      return GeometryBox::kMarginBox;
    case CSSValueID::kFillBox:
      return GeometryBox::kFillBox;
    case CSSValueID::kStrokeBox:
      return GeometryBox::kStrokeBox;
    case CSSValueID::kViewBox:
      return GeometryBox::kViewBox;
    default:
      break;
  }
  NOTREACHED();
}

template <>
inline CSSIdentifierValue::CSSIdentifierValue(GeometryBox geometry_box)
    : CSSValue(kIdentifierClass) {
  switch (geometry_box) {
    case GeometryBox::kBorderBox:
      value_id_ = CSSValueID::kBorderBox;
      break;
    case GeometryBox::kPaddingBox:
      value_id_ = CSSValueID::kPaddingBox;
      break;
    case GeometryBox::kContentBox:
      value_id_ = CSSValueID::kContentBox;
      break;
    case GeometryBox::kMarginBox:
      value_id_ = CSSValueID::kMarginBox;
      break;
    case GeometryBox::kFillBox:
      value_id_ = CSSValueID::kFillBox;
      break;
    case GeometryBox::kStrokeBox:
      value_id_ = CSSValueID::kStrokeBox;
      break;
    case GeometryBox::kViewBox:
      value_id_ = CSSValueID::kViewBox;
      break;
  }
}

template <>
inline TextUnderlinePosition CSSIdentifierValue::ConvertTo() const {
  switch (GetValueID()) {
    case CSSValueID::kAuto:
      return TextUnderlinePosition::kAuto;
    case CSSValueID::kFromFont:
      return TextUnderlinePosition::kFromFont;
    case CSSValueID::kUnder:
      return TextUnderlinePosition::kUnder;
    case CSSValueID::kLeft:
      return TextUnderlinePosition::kLeft;
    case CSSValueID::kRight:
      return TextUnderlinePosition::kRight;
    default:
      break;
  }
  NOTREACHED();
}

template <>
inline CSSIdentifierValue::CSSIdentifierValue(ScrollbarGutter scrollbar_gutter)
    : CSSValue(kIdentifierClass) {
  switch (scrollbar_gutter) {
    case kScrollbarGutterAuto:
      value_id_ = CSSValueID::kAuto;
      break;
    case kScrollbarGutterStable:
      value_id_ = CSSValueID::kStable;
      break;
    case kScrollbarGutterBothEdges:
      value_id_ = CSSValueID::kBothEdges;
      break;
  }
}

template <>
inline ScrollbarGutter CSSIdentifierValue::ConvertTo() const {
  switch (GetValueID()) {
    case CSSValueID::kAuto:
      return kScrollbarGutterAuto;
    case CSSValueID::kStable:
      return kScrollbarGutterStable;
    case CSSValueID::kBothEdges:
      return kScrollbarGutterBothEdges;
    default:
      break;
  }
  NOTREACHED();
}

template <>
inline CSSIdentifierValue::CSSIdentifierValue(TimelineAxis axis)
    : CSSValue(kIdentifierClass) {
  switch (axis) {
    case TimelineAxis::kBlock:
      value_id_ = CSSValueID::kBlock;
      break;
    case TimelineAxis::kInline:
      value_id_ = CSSValueID::kInline;
      break;
    case TimelineAxis::kX:
      value_id_ = CSSValueID::kX;
      break;
    case TimelineAxis::kY:
      value_id_ = CSSValueID::kY;
      break;
  }
}

template <>
inline TimelineAxis CSSIdentifierValue::ConvertTo() const {
  switch (GetValueID()) {
    case CSSValueID::kBlock:
      return TimelineAxis::kBlock;
    case CSSValueID::kInline:
      return TimelineAxis::kInline;
    case CSSValueID::kX:
      return TimelineAxis::kX;
    case CSSValueID::kY:
      return TimelineAxis::kY;
    default:
      break;
  }
  NOTREACHED();
}

template <>
inline CSSIdentifierValue::CSSIdentifierValue(TimelineScroller scroller)
    : CSSValue(kIdentifierClass) {
  switch (scroller) {
    case TimelineScroller::kRoot:
      value_id_ = CSSValueID::kRoot;
      break;
    case TimelineScroller::kNearest:
      value_id_ = CSSValueID::kNearest;
      break;
    case TimelineScroller::kSelf:
      value_id_ = CSSValueID::kSelf;
      break;
  }
}

template <>
inline TimelineScroller CSSIdentifierValue::ConvertTo() const {
  switch (GetValueID()) {
    case CSSValueID::kRoot:
      return TimelineScroller::kRoot;
    case CSSValueID::kNearest:
      return TimelineScroller::kNearest;
    case CSSValueID::kSelf:
      return TimelineScroller::kSelf;
    default:
      break;
  }
  NOTREACHED();
}

template <>
inline CSSIdentifierValue::CSSIdentifierValue(
    TimelineOffset::NamedRange named_range)
    : CSSValue(kIdentifierClass) {
  switch (named_range) {
    case TimelineOffset::NamedRange::kCover:
      value_id_ = CSSValueID::kCover;
      break;
    case TimelineOffset::NamedRange::kContain:
      value_id_ = CSSValueID::kContain;
      break;
    case TimelineOffset::NamedRange::kEntry:
      value_id_ = CSSValueID::kEntry;
      break;
    case TimelineOffset::NamedRange::kEntryCrossing:
      value_id_ = CSSValueID::kEntryCrossing;
      break;
    case TimelineOffset::NamedRange::kExit:
      value_id_ = CSSValueID::kExit;
      break;
    case TimelineOffset::NamedRange::kExitCrossing:
      value_id_ = CSSValueID::kExitCrossing;
      break;
    case TimelineOffset::NamedRange::kScroll:
      CHECK(RuntimeEnabledFeatures::ScrollTimelineNamedRangeScrollEnabled());
      value_id_ = CSSValueID::kScroll;
      break;
    default:
      NOTREACHED();
  }
}

template <>
inline TimelineOffset::NamedRange CSSIdentifierValue::ConvertTo() const {
  switch (GetValueID()) {
    case CSSValueID::kCover:
      return TimelineOffset::NamedRange::kCover;
    case CSSValueID::kContain:
      return TimelineOffset::NamedRange::kContain;
    case CSSValueID::kEntry:
      return TimelineOffset::NamedRange::kEntry;
    case CSSValueID::kEntryCrossing:
      return TimelineOffset::NamedRange::kEntryCrossing;
    case CSSValueID::kExit:
      return TimelineOffset::NamedRange::kExit;
    case CSSValueID::kExitCrossing:
      return TimelineOffset::NamedRange::kExitCrossing;
    case CSSValueID::kScroll:
      CHECK(RuntimeEnabledFeatures::ScrollTimelineNamedRangeScrollEnabled());
      return TimelineOffset::NamedRange::kScroll;
    default:
      break;
  }
  NOTREACHED();
}

template <>
inline CSSIdentifierValue::CSSIdentifierValue(ScrollStartValueType value_type)
    : CSSValue(kIdentifierClass) {
  switch (value_type) {
    case ScrollStartValueType::kAuto:
      value_id_ = CSSValueID::kAuto;
      break;
    case ScrollStartValueType::kStart:
      value_id_ = CSSValueID::kStart;
      break;
    case ScrollStartValueType::kCenter:
      value_id_ = CSSValueID::kCenter;
      break;
    case ScrollStartValueType::kEnd:
      value_id_ = CSSValueID::kEnd;
      break;
    case ScrollStartValueType::kTop:
      value_id_ = CSSValueID::kTop;
      break;
    case ScrollStartValueType::kBottom:
      value_id_ = CSSValueID::kBottom;
      break;
    case ScrollStartValueType::kLeft:
      value_id_ = CSSValueID::kLeft;
      break;
    case ScrollStartValueType::kRight:
      value_id_ = CSSValueID::kRight;
      break;
    case ScrollStartValueType::kLengthOrPercentage:
      NOTREACHED();
  }
}

template <>
inline ScrollStartValueType CSSIdentifierValue::ConvertTo() const {
  switch (GetValueID()) {
    case CSSValueID::kAuto:
      return ScrollStartValueType::kAuto;
    case CSSValueID::kStart:
      return ScrollStartValueType::kStart;
    case CSSValueID::kCenter:
      return ScrollStartValueType::kCenter;
    case CSSValueID::kEnd:
      return ScrollStartValueType::kEnd;
    case CSSValueID::kTop:
      return ScrollStartValueType::kTop;
    case CSSValueID::kBottom:
      return ScrollStartValueType::kBottom;
    case CSSValueID::kLeft:
      return ScrollStartValueType::kLeft;
    case CSSValueID::kRight:
      return ScrollStartValueType::kRight;
    default:
      NOTREACHED();
  }
}

template <>
inline CSSIdentifierValue::CSSIdentifierValue(EScrollInitialTarget target)
    : CSSValue(kIdentifierClass) {
  switch (target) {
    case EScrollInitialTarget::kNone:
      value_id_ = CSSValueID::kNone;
      break;
    case EScrollInitialTarget::kNearest:
      value_id_ = CSSValueID::kNearest;
      break;
  };
}

template <>
inline EScrollInitialTarget CSSIdentifierValue::ConvertTo() const {
  switch (GetValueID()) {
    case CSSValueID::kNone:
      return EScrollInitialTarget::kNone;
    case CSSValueID::kNearest:
      return EScrollInitialTarget::kNearest;
    default:
      NOTREACHED();
  };
}

template <>
inline CSSIdentifierValue::CSSIdentifierValue(PositionAreaRegion region)
    : CSSValue(kIdentifierClass) {
  switch (region) {
    case PositionAreaRegion::kNone:
      value_id_ = CSSValueID::kNone;
      break;
    case PositionAreaRegion::kAll:
      value_id_ = CSSValueID::kSpanAll;
      break;
    case PositionAreaRegion::kCenter:
      value_id_ = CSSValueID::kCenter;
      break;
    case PositionAreaRegion::kStart:
      value_id_ = CSSValueID::kStart;
      break;
    case PositionAreaRegion::kEnd:
      value_id_ = CSSValueID::kEnd;
      break;
    case PositionAreaRegion::kSelfStart:
      value_id_ = CSSValueID::kSelfStart;
      break;
    case PositionAreaRegion::kSelfEnd:
      value_id_ = CSSValueID::kSelfEnd;
      break;
    case PositionAreaRegion::kInlineStart:
      value_id_ = CSSValueID::kInlineStart;
      break;
    case PositionAreaRegion::kInlineEnd:
      value_id_ = CSSValueID::kInlineEnd;
      break;
    case PositionAreaRegion::kSelfInlineStart:
      value_id_ = CSSValueID::kSelfInlineStart;
      break;
    case PositionAreaRegion::kSelfInlineEnd:
      value_id_ = CSSValueID::kSelfInlineEnd;
      break;
    case PositionAreaRegion::kBlockStart:
      value_id_ = CSSValueID::kBlockStart;
      break;
    case PositionAreaRegion::kBlockEnd:
      value_id_ = CSSValueID::kBlockEnd;
      break;
    case PositionAreaRegion::kSelfBlockStart:
      value_id_ = CSSValueID::kSelfBlockStart;
      break;
    case PositionAreaRegion::kSelfBlockEnd:
      value_id_ = CSSValueID::kSelfBlockEnd;
      break;
    case PositionAreaRegion::kTop:
      value_id_ = CSSValueID::kTop;
      break;
    case PositionAreaRegion::kBottom:
      value_id_ = CSSValueID::kBottom;
      break;
    case PositionAreaRegion::kLeft:
      value_id_ = CSSValueID::kLeft;
      break;
    case PositionAreaRegion::kRight:
      value_id_ = CSSValueID::kRight;
      break;
    case PositionAreaRegion::kXStart:
      value_id_ = CSSValueID::kXStart;
      break;
    case PositionAreaRegion::kXEnd:
      value_id_ = CSSValueID::kXEnd;
      break;
    case PositionAreaRegion::kYStart:
      value_id_ = CSSValueID::kYStart;
      break;
    case PositionAreaRegion::kYEnd:
      value_id_ = CSSValueID::kYEnd;
      break;
    case PositionAreaRegion::kXSelfStart:
      value_id_ = CSSValueID::kXSelfStart;
      break;
    case PositionAreaRegion::kXSelfEnd:
      value_id_ = CSSValueID::kXSelfEnd;
      break;
    case PositionAreaRegion::kYSelfStart:
      value_id_ = CSSValueID::kYSelfStart;
      break;
    case PositionAreaRegion::kYSelfEnd:
      value_id_ = CSSValueID::kYSelfEnd;
      break;
  }
}

template <>
inline PositionAreaRegion CSSIdentifierValue::ConvertTo() const {
  switch (GetValueID()) {
    case CSSValueID::kNone:
      return PositionAreaRegion::kNone;
    case CSSValueID::kSpanAll:
      return PositionAreaRegion::kAll;
    case CSSValueID::kCenter:
      return PositionAreaRegion::kCenter;
    case CSSValueID::kStart:
      return PositionAreaRegion::kStart;
    case CSSValueID::kEnd:
      return PositionAreaRegion::kEnd;
    case CSSValueID::kSelfStart:
      return PositionAreaRegion::kSelfStart;
    case CSSValueID::kSelfEnd:
      return PositionAreaRegion::kSelfEnd;
    case CSSValueID::kInlineStart:
      return PositionAreaRegion::kInlineStart;
    case CSSValueID::kInlineEnd:
      return PositionAreaRegion::kInlineEnd;
    case CSSValueID::kSelfInlineStart:
      return PositionAreaRegion::kSelfInlineStart;
    case CSSValueID::kSelfInlineEnd:
      return PositionAreaRegion::kSelfInlineEnd;
    case CSSValueID::kBlockStart:
      return PositionAreaRegion::kBlockStart;
    case CSSValueID::kBlockEnd:
      return PositionAreaRegion::kBlockEnd;
    case CSSValueID::kSelfBlockStart:
      return PositionAreaRegion::kSelfBlockStart;
    case CSSValueID::kSelfBlockEnd:
      return PositionAreaRegion::kSelfBlockEnd;
    case CSSValueID::kTop:
      return PositionAreaRegion::kTop;
    case CSSValueID::kBottom:
      return PositionAreaRegion::kBottom;
    case CSSValueID::kLeft:
      return PositionAreaRegion::kLeft;
    case CSSValueID::kRight:
      return PositionAreaRegion::kRight;
    case CSSValueID::kXStart:
      return PositionAreaRegion::kXStart;
    case CSSValueID::kXEnd:
      return PositionAreaRegion::kXEnd;
    case CSSValueID::kYStart:
      return PositionAreaRegion::kYStart;
    case CSSValueID::kYEnd:
      return PositionAreaRegion::kYEnd;
    case CSSValueID::kXSelfStart:
      return PositionAreaRegion::kXSelfStart;
    case CSSValueID::kXSelfEnd:
      return PositionAreaRegion::kXSelfEnd;
    case CSSValueID::kYSelfStart:
      return PositionAreaRegion::kYSelfStart;
    case CSSValueID::kYSelfEnd:
      return PositionAreaRegion::kYSelfEnd;
    default:
      NOTREACHED();
  };
}

template <>
inline CSSIdentifierValue::CSSIdentifierValue(PositionVisibility visibility)
    : CSSValue(kIdentifierClass) {
  switch (visibility) {
    case PositionVisibility::kAlways:
      value_id_ = CSSValueID::kAlways;
      break;
    // TODO(crbug.com/332933527): Support kAnchorsValid.
    case PositionVisibility::kAnchorsVisible:
      value_id_ = CSSValueID::kAnchorsVisible;
      break;
    case PositionVisibility::kNoOverflow:
      value_id_ = CSSValueID::kNoOverflow;
      break;
  }
}

template <>
inline PositionVisibility CSSIdentifierValue::ConvertTo() const {
  switch (GetValueID()) {
    case CSSValueID::kAlways:
      return PositionVisibility::kAlways;
    // TODO(crbug.com/332933527): Support kAnchorsValid.
    case CSSValueID::kAnchorsVisible:
      return PositionVisibility::kAnchorsVisible;
    case CSSValueID::kNoOverflow:
      return PositionVisibility::kNoOverflow;
    default:
      NOTREACHED();
  }
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_IDENTIFIER_VALUE_MAPPINGS_H_
