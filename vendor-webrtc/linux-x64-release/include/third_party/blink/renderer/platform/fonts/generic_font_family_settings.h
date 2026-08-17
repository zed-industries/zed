/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_GENERIC_FONT_FAMILY_SETTINGS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_GENERIC_FONT_FAMILY_SETTINGS_H_

#include "base/feature_list.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string_hash.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

#include <unicode/uscript.h>

namespace blink {

PLATFORM_EXPORT BASE_DECLARE_FEATURE(kGenericFontSettingCache);

class PLATFORM_EXPORT GenericFontFamilySettings {
  DISALLOW_NEW();

 public:
  GenericFontFamilySettings() = default;

  explicit GenericFontFamilySettings(const GenericFontFamilySettings&);

  bool UpdateStandard(const AtomicString&, UScriptCode = USCRIPT_COMMON);
  const AtomicString& Standard(UScriptCode = USCRIPT_COMMON) const;

  // crbug.com/40535332. In the argument of UpdateFixed(), Osaka font should
  // be represented as "Osaka", and Fixed() returns "Osaka-Mono" for it.
  bool UpdateFixed(const AtomicString&, UScriptCode = USCRIPT_COMMON);
  const AtomicString& Fixed(UScriptCode = USCRIPT_COMMON) const;

  bool UpdateSerif(const AtomicString&, UScriptCode = USCRIPT_COMMON);
  const AtomicString& Serif(UScriptCode = USCRIPT_COMMON) const;

  bool UpdateSansSerif(const AtomicString&, UScriptCode = USCRIPT_COMMON);
  const AtomicString& SansSerif(UScriptCode = USCRIPT_COMMON) const;

  bool UpdateCursive(const AtomicString&, UScriptCode = USCRIPT_COMMON);
  const AtomicString& Cursive(UScriptCode = USCRIPT_COMMON) const;

  bool UpdateFantasy(const AtomicString&, UScriptCode = USCRIPT_COMMON);
  const AtomicString& Fantasy(UScriptCode = USCRIPT_COMMON) const;

  bool UpdateMath(const AtomicString&, UScriptCode = USCRIPT_COMMON);
  const AtomicString& Math(UScriptCode = USCRIPT_COMMON) const;

  // Only called by InternalSettings to clear font family maps.
  void Reset();

  GenericFontFamilySettings& operator=(const GenericFontFamilySettings&);

 private:
  // UScriptCode uses -1 and 0 for UScriptInvalidCode and UScriptCommon.
  // We need to use -2 and -3 for empty value and deleted value.
  using UScriptCodeHashTraits = IntHashTraits<int, -1, -3>;

  typedef HashMap<int, AtomicString, UScriptCodeHashTraits> ScriptFontFamilyMap;

  void SetGenericFontFamilyMap(ScriptFontFamilyMap&,
                               const AtomicString&,
                               UScriptCode);

  // Warning: Calling this method might result in a synchronous IPC call, which
  // waits until the browser process to load a font and blocks the current
  // thread.
  const AtomicString& GenericFontFamilyForScript(const ScriptFontFamilyMap&,
                                                 UScriptCode) const;

  // Returns true if the first available font of `new_family` could be different
  // from `old_first_available_family`.
  bool ShouldUpdateFontFamily(const AtomicString& old_first_available_family,
                              const AtomicString& new_family) const;

  ScriptFontFamilyMap standard_font_family_map_;
  ScriptFontFamilyMap serif_font_family_map_;
  ScriptFontFamilyMap fixed_font_family_map_;
  ScriptFontFamilyMap sans_serif_font_family_map_;
  ScriptFontFamilyMap cursive_font_family_map_;
  ScriptFontFamilyMap fantasy_font_family_map_;
  ScriptFontFamilyMap math_font_family_map_;

  // For the given font families, caches the first available font. If none of
  // them is available, the value will be the first font of the given
  // families. To save memory, the key should contain more than one font, in
  // the format of ",font1, font2, ...".
  mutable HashMap<AtomicString, AtomicString>
      first_available_font_for_families_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_GENERIC_FONT_FAMILY_SETTINGS_H_
