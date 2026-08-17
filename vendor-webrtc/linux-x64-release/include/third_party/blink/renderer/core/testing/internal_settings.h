/*
 * Copyright (C) 2012 Google Inc. All rights reserved.
 * Copyright (C) 2013 Apple Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE AND ITS CONTRIBUTORS "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE OR ITS CONTRIBUTORS BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_INTERNAL_SETTINGS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_INTERNAL_SETTINGS_H_

#include <unicode/uscript.h>

#include "third_party/blink/renderer/core/testing/internal_settings_generated.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class ExceptionState;
class GenericFontFamilySettings;

class InternalSettings final : public InternalSettingsGenerated {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static InternalSettings* From(Page&);

  explicit InternalSettings(Page&);
  ~InternalSettings() override;

  void ResetToConsistentState();

  void setStandardFontFamily(const AtomicString& family, const String& script);
  void setSerifFontFamily(const AtomicString& family, const String& script);
  void setSansSerifFontFamily(const AtomicString& family, const String& script);
  void setFixedFontFamily(const AtomicString& family, const String& script);
  void setCursiveFontFamily(const AtomicString& family, const String& script);
  void setFantasyFontFamily(const AtomicString& family, const String& script);
  void setMathFontFamily(const AtomicString& family, const String& script);
  void setTextAutosizingWindowSizeOverride(int width, int height);
  void setEditingBehavior(const String&, ExceptionState&);
  void setDisplayModeOverride(const String& display_mode, ExceptionState&);
  void setTextTrackKindUserPreference(const String& preference,
                                      ExceptionState&);
  void setViewportStyle(const String& preference, ExceptionState&);
  void setAutoplayPolicy(const String&, ExceptionState&);
  void setImageAnimationPolicy(const String&, ExceptionState&);
  void setAvailablePointerTypes(const String&, ExceptionState&);
  void setPrimaryPointerType(const String&, ExceptionState&);
  void setAvailableHoverTypes(const String&, ExceptionState&);
  void setPrimaryHoverType(const String&, ExceptionState&);
  void setPreferCompositingToLCDTextEnabled(bool);

 private:
  void SetFontFamily(
      const AtomicString& family,
      const String& script,
      bool (GenericFontFamilySettings::*update_method)(const AtomicString&,
                                                       UScriptCode));

  GenericFontFamilySettings generic_font_family_settings_backup_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_INTERNAL_SETTINGS_H_
