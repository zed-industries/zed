/*
 * Copyright (C) 2003, 2006, 2007, 2008, 2009, 2011, 2012 Apple Inc. All rights
 * reserved.
 *           (C) 2006 Graham Dennis (graham.dennis@gmail.com)
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_SETTINGS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_SETTINGS_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/settings_base.h"
#include "third_party/blink/renderer/platform/fonts/generic_font_family_settings.h"

namespace blink {

class CORE_EXPORT Settings : public SettingsBase {
  USING_FAST_MALLOC(Settings);

 public:
  Settings();

  // Default copy and assignment are forbidden because SettingsDelegate only
  // supports 1:1 relationship with Settings.
  Settings(const Settings&) = delete;
  Settings& operator=(const Settings&) = delete;

  GenericFontFamilySettings& GetGenericFontFamilySettings() {
    return generic_font_family_settings_;
  }
  const GenericFontFamilySettings& GetGenericFontFamilySettings() const {
    return generic_font_family_settings_;
  }
  void NotifyGenericFontFamilyChange() {
    Invalidate(SettingsDelegate::ChangeType::kFontFamily);
  }

  void SetPreferCompositingToLCDTextForTesting(bool enabled);

 private:
  GenericFontFamilySettings generic_font_family_settings_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_SETTINGS_H_
