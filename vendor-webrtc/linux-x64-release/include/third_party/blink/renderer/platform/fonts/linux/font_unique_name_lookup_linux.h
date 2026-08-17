// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_LINUX_FONT_UNIQUE_NAME_LOOKUP_LINUX_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_LINUX_FONT_UNIQUE_NAME_LOOKUP_LINUX_H_

#include "third_party/blink/renderer/platform/fonts/font_unique_name_lookup.h"

namespace blink {

class FontUniqueNameLookupLinux : public FontUniqueNameLookup {
 public:
  FontUniqueNameLookupLinux() = default;
  FontUniqueNameLookupLinux(const FontUniqueNameLookupLinux&) = delete;
  FontUniqueNameLookupLinux& operator=(const FontUniqueNameLookupLinux&) =
      delete;
  ~FontUniqueNameLookupLinux() override;
  sk_sp<SkTypeface> MatchUniqueName(const String& font_unique_name) override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_LINUX_FONT_UNIQUE_NAME_LOOKUP_LINUX_H_
