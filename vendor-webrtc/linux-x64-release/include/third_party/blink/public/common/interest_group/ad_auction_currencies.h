// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_INTEREST_GROUP_AD_AUCTION_CURRENCIES_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_INTEREST_GROUP_AD_AUCTION_CURRENCIES_H_

#include <optional>
#include <string>

#include "base/check.h"
#include "third_party/blink/public/common/common_export.h"

namespace blink {

// True if the currency code is in format FLEDGE expects.
// Does not accept kUnspecifiedAdCurrency.
BLINK_COMMON_EXPORT bool IsValidAdCurrencyCode(const std::string& input);

// A wrapper type for currency codes used in FLEDGE auctions.
class BLINK_COMMON_EXPORT AdCurrency {
 public:
  // Precondiiton: `IsValidAdCurrencyCode(currency_code)`.
  static AdCurrency From(const std::string& currency_code) {
    DCHECK(IsValidAdCurrencyCode(currency_code)) << currency_code;
    AdCurrency result;
    result.currency_code_ = currency_code;
    return result;
  }

  // This is either a valid currency code or an empty string, unless
  // SetCurrencyCodeForTesting is used.
  const std::string& currency_code() const { return currency_code_; }

  // Lets one set invalid values to cover mojo checking.
  void SetCurrencyCodeForTesting(const std::string& in) { currency_code_ = in; }

  friend BLINK_COMMON_EXPORT bool operator==(const AdCurrency&,
                                             const AdCurrency&);

 private:
  std::string currency_code_;
};

// Returns true if `actual` currency should be accepted under expectation
// `expected; this can happen if they match or if any of them is unspecified.
BLINK_COMMON_EXPORT bool VerifyAdCurrencyCode(
    const std::optional<AdCurrency>& expected,
    const std::optional<AdCurrency>& actual);

// Magical value to denote that a party in the auction didn't specify
// currency value or expectation.  It can't be provided directly, but it's meant
// to be human-readable.
BLINK_COMMON_EXPORT extern const char* const kUnspecifiedAdCurrency;

// Converts a maybe-unspecified ad currency into a form suitable for JS strings
// or error messages --- e.g. unspecified value gets expanded out to
// kUnspecifiedAdCurrency for readability.
BLINK_COMMON_EXPORT std::string PrintableAdCurrency(
    const std::optional<AdCurrency>& currency);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_INTEREST_GROUP_AD_AUCTION_CURRENCIES_H_
