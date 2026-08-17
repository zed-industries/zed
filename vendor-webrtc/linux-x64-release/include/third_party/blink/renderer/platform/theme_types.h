/*
 * Copyright (C) 2008, 2009, 2010 Apple Inc. All Rights Reserved.
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
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_THEME_TYPES_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_THEME_TYPES_H_

namespace blink {

// ComputedStyle::EffectiveAppearance() returns the effective appearance
// to render the element by matching the element's computed style to an
// AppearanceValue.
// kAuto is never returned by ComputedStyle::EffectiveAppearance()
// CSS `appearance` values do not match 1-to-1 with AppearanceValue,
// since some AppearanceValue's do not have equivalent CSS appearance
// values, e.g. kSliderThumbHorizontal.
enum class AppearanceValue {
  kNone,
  kAuto,
  kCheckbox,
  kRadio,
  kButton,
  kListbox,
  kMediaControl,
  kMenulist,
  kMenulistButton,
  kMeter,
  kProgressBar,
  kSearchField,
  kTextField,
  kTextArea,
  kInnerSpinButton,
  kMediaSlider,
  kMediaSliderThumb,
  kMediaVolumeSlider,
  kMediaVolumeSliderThumb,
  kPushButton,
  kSquareButton,
  kSliderHorizontal,
  kSliderThumbHorizontal,
  kSliderThumbVertical,
  kSearchFieldCancelButton,
  kSliderVertical,
  kBaseSelect,
};

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_THEME_TYPES_H_
