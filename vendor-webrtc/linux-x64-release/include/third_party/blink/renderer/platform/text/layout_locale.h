// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_LAYOUT_LOCALE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_LAYOUT_LOCALE_H_

#include "base/memory/raw_ptr.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/text/hyphenation.h"
#include "third_party/blink/renderer/platform/text/quotes_data.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/std_lib_extras.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string_hash.h"
#include "third_party/blink/renderer/platform/wtf/text/case_map.h"
#include "third_party/blink/renderer/platform/wtf/thread_safe_ref_counted.h"

#include <unicode/uscript.h>

struct hb_language_impl_t;

namespace blink {

// A Unicode Line Break Style Identifier (key "lb".)
// https://www.unicode.org/reports/tr35/#UnicodeLineBreakStyleIdentifier
enum class LineBreakStrictness : uint8_t { kDefault, kNormal, kStrict, kLoose };

class PLATFORM_EXPORT LayoutLocale : public RefCounted<LayoutLocale> {
  USING_FAST_MALLOC(LayoutLocale);

 public:
  static const LayoutLocale* Get(const AtomicString& locale);
  static const LayoutLocale& GetDefault();
  static const LayoutLocale& GetSystem();
  static const LayoutLocale& ValueOrDefault(const LayoutLocale* locale) {
    return locale ? *locale : GetDefault();
  }

  bool operator==(const LayoutLocale& other) const {
    return string_ == other.string_;
  }
  bool operator!=(const LayoutLocale& other) const {
    return string_ != other.string_;
  }

  const AtomicString& LocaleString() const { return string_; }
  static const AtomicString& LocaleString(const LayoutLocale* locale) {
    return locale ? locale->string_ : g_null_atom;
  }
  operator const AtomicString&() const { return string_; }
  std::string Ascii() const { return string_.Ascii(); }

  const hb_language_impl_t* HarfbuzzLanguage() const {
    return harfbuzz_language_;
  }
  const char* LocaleForSkFontMgr() const;
  UScriptCode GetScript() const { return script_; }

  // Disambiguation of the Unified Han Ideographs.
  UScriptCode GetScriptForHan() const;
  bool HasScriptForHan() const;
  static const LayoutLocale* LocaleForHan(const LayoutLocale*);
  const char* LocaleForHanForSkFontMgr() const;

  // The normalized locale data to construct |CaseMap| from.
  const CaseMap::Locale& CaseMapLocale() const {
    if (case_map_computed_)
      return locale_for_case_map_;
    ComputeCaseMapLocale();
    return locale_for_case_map_;
  }

  Hyphenation* GetHyphenation() const;
  scoped_refptr<QuotesData> GetQuotesData() const;

  AtomicString LocaleWithBreakKeyword(LineBreakStrictness,
                                      bool use_phrase = false) const;

  static scoped_refptr<LayoutLocale> CreateForTesting(const AtomicString&);
  static void SetHyphenationForTesting(const AtomicString&,
                                       scoped_refptr<Hyphenation>);

  static void AcceptLanguagesChanged(const String&);

  static void ClearForTesting();

 private:
  explicit LayoutLocale(const AtomicString&);

  void ComputeScriptForHan() const;
  void ComputeCaseMapLocale() const;

  AtomicString string_;
  mutable std::string string_for_sk_font_mgr_;
  mutable CaseMap::Locale locale_for_case_map_;
  mutable scoped_refptr<Hyphenation> hyphenation_;
  mutable scoped_refptr<QuotesData> quotes_data_;

  // hb_language_t is defined in hb.h, which not all files can include.
  raw_ptr<const hb_language_impl_t> harfbuzz_language_;

  UScriptCode script_;
  mutable UScriptCode script_for_han_;

  mutable unsigned has_script_for_han_ : 1;
  mutable unsigned hyphenation_computed_ : 1;
  mutable unsigned quotes_data_computed_ : 1;
  mutable unsigned case_map_computed_ : 1;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_LAYOUT_LOCALE_H_
