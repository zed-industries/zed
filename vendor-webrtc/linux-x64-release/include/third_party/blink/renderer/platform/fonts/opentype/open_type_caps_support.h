// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_OPENTYPE_OPEN_TYPE_CAPS_SUPPORT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_OPENTYPE_OPEN_TYPE_CAPS_SUPPORT_H_

#include "third_party/blink/renderer/platform/fonts/font_description.h"
#include "third_party/blink/renderer/platform/fonts/shaping/case_mapping_harfbuzz_buffer_filler.h"
#include "third_party/blink/renderer/platform/fonts/shaping/harfbuzz_face.h"
#include "third_party/blink/renderer/platform/fonts/small_caps_iterator.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

#include <hb.h>

namespace blink {

class PLATFORM_EXPORT OpenTypeCapsSupport {
  STACK_ALLOCATED();

 public:
  OpenTypeCapsSupport();
  OpenTypeCapsSupport(
      const HarfBuzzFace*,
      FontDescription::FontVariantCaps requested_caps,
      FontDescription::FontSynthesisSmallCaps font_synthesis_small_caps,
      hb_script_t);

  bool NeedsRunCaseSplitting();
  bool NeedsSyntheticFont(SmallCapsIterator::SmallCapsBehavior run_case);
  FontDescription::FontVariantCaps FontFeatureToUse(
      SmallCapsIterator::SmallCapsBehavior run_case);
  CaseMapIntend NeedsCaseChange(SmallCapsIterator::SmallCapsBehavior run_case);

 private:
  enum class FontFormat { kUndetermined, kOpenType, kAat };
  // Lazily intializes font_format_ when needed and returns the format of the
  // underlying HarfBuzzFace/Font.
  FontFormat GetFontFormat() const;
  void DetermineFontSupport(hb_script_t);
  bool SupportsFeature(hb_script_t, uint32_t tag) const;
  bool SupportsAatFeature(uint32_t tag) const;
  bool SupportsOpenTypeFeature(hb_script_t, uint32_t tag) const;
  bool SyntheticSmallCapsAllowed() const;

  const HarfBuzzFace* harfbuzz_face_ = nullptr;
  FontDescription::FontVariantCaps requested_caps_ =
      FontDescription::kCapsNormal;
  FontDescription::FontSynthesisSmallCaps font_synthesis_small_caps_ =
      FontDescription::kAutoFontSynthesisSmallCaps;

  enum class FontSupport {
    kFull,
    kFallback,  // Fall back to 'smcp' or 'smcp' + 'c2sc'
    kNone
  };

  enum class CapsSynthesis {
    kNone,
    kLowerToSmallCaps,
    kUpperToSmallCaps,
    kBothToSmallCaps
  };

  FontSupport font_support_;
  CapsSynthesis caps_synthesis_;
  mutable FontFormat font_format_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_OPENTYPE_OPEN_TYPE_CAPS_SUPPORT_H_
