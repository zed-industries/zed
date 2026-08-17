// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_WEB_FONT_TYPEFACE_FACTORY_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_WEB_FONT_TYPEFACE_FACTORY_H_

#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/skia/include/core/SkFontMgr.h"

#include "build/build_config.h"
#include "third_party/blink/public/common/buildflags.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class FontFormatCheck;
// Decides which Skia backend to use for instantiating a web font. In the
// regular case, this would be default font manager used for the platform.
// However, for COLRv1 fonts, variable fonts, color bitmap font formats and CFF2
// fonts we want to use FreeType/Fontations on Windows and Mac.
class PLATFORM_EXPORT WebFontTypefaceFactory {
  STACK_ALLOCATED();

 public:
  using InstantiationFunction = sk_sp<SkTypeface> (*)(sk_sp<SkData>);

  using FontInstantiator = struct {
    InstantiationFunction make_system;
    InstantiationFunction make_fontations;
#if BUILDFLAG(IS_WIN) || BUILDFLAG(IS_APPLE)
    InstantiationFunction make_fallback;
#endif
  };

  static bool CreateTypeface(const sk_sp<SkData>, sk_sp<SkTypeface>&);
  static bool CreateTypeface(const sk_sp<SkData>,
                             sk_sp<SkTypeface>&,
                             const FontFormatCheck& format_check,
                             const FontInstantiator& instantiator);

 private:
  // These values are written to logs.  New enum values can be added, but
  // existing enums must never be renumbered or deleted and reused.
  enum class InstantiationResult {
    kErrorInstantiatingVariableFont = 0,
    kSuccessConventionalWebFont = 1,
    kSuccessVariableWebFont = 2,
    kSuccessCbdtCblcColorFont = 3,
    kSuccessCff2Font = 4,
    kSuccessSbixFont = 5,
    kSuccessColrCpalFont = 6,
    kSuccessColrV1Font = 7,
    kMaxValue = kSuccessColrV1Font
  };

  static void ReportInstantiationResult(InstantiationResult);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_WEB_FONT_TYPEFACE_FACTORY_H_
