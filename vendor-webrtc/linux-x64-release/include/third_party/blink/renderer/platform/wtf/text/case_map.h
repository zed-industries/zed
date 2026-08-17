// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_CASE_MAP_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_CASE_MAP_H_

#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/wtf_export.h"

namespace WTF {

class String;
class TextOffsetMap;

// This class performs the full Unicode case-mapping.
//
// See LowerASCII/UpperASCII() variants for faster, ASCII-only,
// locale-independent case-mapping.
class WTF_EXPORT CaseMap {
 public:
  // This is a specialized locale class that holds normalized locale for
  // |CaseMap|.
  class WTF_EXPORT WTF_EXPORT Locale {
   public:
    Locale() : case_map_locale_(nullptr) {}
    explicit Locale(const AtomicString& locale);

   private:
    const char* case_map_locale_;

    static const char* turkic_or_azeri_;
    static const char* greek_;
    static const char* lithuanian_;

    friend class CaseMap;
  };

  // Construct from the normalized locale.
  explicit CaseMap(const Locale& locale)
      : case_map_locale_(locale.case_map_locale_) {}

  // Construct from a locale string. The given locale is normalized.
  explicit CaseMap(const AtomicString& locale) : CaseMap(Locale(locale)) {}

  String ToLower(const String& source,
                 TextOffsetMap* offset_map = nullptr) const;
  String ToUpper(const String& source,
                 TextOffsetMap* offset_map = nullptr) const;

  // Fast code path for simple cases, only for root locale.
  // TODO(crbug.com/627682): This should move to private, once
  // |DeprecatedLower()| is deprecated.
  static scoped_refptr<StringImpl> FastToLowerInvariant(StringImpl* source);

 private:
  scoped_refptr<StringImpl> ToLower(StringImpl* source,
                                    TextOffsetMap* offset_map = nullptr) const;
  scoped_refptr<StringImpl> ToUpper(StringImpl* source,
                                    TextOffsetMap* offset_map = nullptr) const;

  // Fast code path for simple cases for root locale. When the fast code path is
  // not possible, fallback to full Unicode casing using the root locale.
  static scoped_refptr<StringImpl> ToLowerInvariant(StringImpl* source,
                                                    TextOffsetMap* offset_map);
  static scoped_refptr<StringImpl> ToUpperInvariant(StringImpl* source,
                                                    TextOffsetMap* offset_map);

  // Try the fast code path. Returns |nullptr| when the string cannot use the
  // fast code path. The caller is responsible to fallback to full algorithm.
  static scoped_refptr<StringImpl> TryFastToLowerInvariant(StringImpl* source);

  const char* case_map_locale_;
};

}  // namespace WTF

using WTF::CaseMap;

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_CASE_MAP_H_
