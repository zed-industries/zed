// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WIN_WEB_SANDBOX_SUPPORT_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WIN_WEB_SANDBOX_SUPPORT_H_

#include <memory>
#include <utility>
#include <vector>

#include "base/win/windows_types.h"
#include "third_party/blink/public/platform/web_string.h"

// Repeat some Windows typedefs used by the locale functions that
// are not in base/win/windows_types.h
using LCID = DWORD;
using LCTYPE = DWORD;

namespace blink {

// Put methods here that are required due to sandbox restrictions.
class WebSandboxSupport {
 public:
  // Bundles the result of several calls to locale functions.
  struct LocaleInitData {
    // LOCALE_IDIGITSUBSTITUTION.
    DWORD digit_substitution;
    // LOCALE_SNATIVEDIGITS. Empty if digit_substitution == 1 (0to9).
    WebString digits;
    // LOCALE_SDECIMAL.
    WebString decimal;
    // LOCALE_STHOUSAND.
    WebString thousand;
    // LOCALE_SNEGATIVESIGN.
    WebString negative_sign;
    // LOCALE_INEGNUMBER.
    DWORD negnumber;
  };

  virtual ~WebSandboxSupport() {}

  // Can locale-related calls be proxied?
  virtual bool IsLocaleProxyEnabled() = 0;

  // Returns locale information. Fails unless IsLocaleProxyEnabled().
  virtual std::pair<LCID, unsigned> LcidAndFirstDayOfWeek(
      WebString locale,
      WebString default_language,
      bool force_defaults) = 0;

  // Returns digit symbols and sign prefixes and suffixes. Fails unless
  // IsLocaleProxyEnabled().
  virtual std::unique_ptr<LocaleInitData> DigitsAndSigns(
      LCID lcid,
      bool force_defaults) = 0;

  virtual std::vector<WebString> MonthLabels(LCID lcid,
                                             bool force_defaults) = 0;
  virtual std::vector<WebString> WeekDayShortLabels(LCID lcid,
                                                    bool force_defaults) = 0;
  virtual std::vector<WebString> ShortMonthLabels(LCID lcid,
                                                  bool force_defaults) = 0;
  virtual std::vector<WebString> AmPmLabels(LCID lcid, bool force_defaults) = 0;

  // Note: only specified allowed types can be provided for `type`.
  virtual WebString LocaleString(LCID lcid,
                                 LCTYPE type,
                                 bool force_defaults) = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WIN_WEB_SANDBOX_SUPPORT_H_
