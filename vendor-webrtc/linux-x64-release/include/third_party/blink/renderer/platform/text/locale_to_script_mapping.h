/*
 * Copyright (C) 2011 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_LOCALE_TO_SCRIPT_MAPPING_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_LOCALE_TO_SCRIPT_MAPPING_H_

#include <unicode/uscript.h>

#include "base/check.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

PLATFORM_EXPORT UScriptCode
LocaleToScriptCodeForFontSelection(const WTF::String&);
PLATFORM_EXPORT UScriptCode ScriptNameToCode(const WTF::String&);

PLATFORM_EXPORT UScriptCode ScriptCodeForHanFromSubtags(const WTF::String&,
                                                        char delimiter = '-');

inline bool IsUnambiguousHanScript(UScriptCode script) {
  // localeToScriptCodeForFontSelection() does not return these values.
  DCHECK(script != USCRIPT_HIRAGANA && script != USCRIPT_KATAKANA);
  return script == USCRIPT_KATAKANA_OR_HIRAGANA ||
         script == USCRIPT_SIMPLIFIED_HAN ||
         script == USCRIPT_TRADITIONAL_HAN || script == USCRIPT_HANGUL;
}
}

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_LOCALE_TO_SCRIPT_MAPPING_H_
