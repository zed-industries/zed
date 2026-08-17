// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_I18N_TIMEZONE_H_
#define BASE_I18N_TIMEZONE_H_

#include <string>

#include "base/i18n/base_i18n_export.h"

namespace base {

// Checks the system timezone and turns it into a two-character ISO 3166 country
// code. This may fail (for example, it used to always fail on Android), in
// which case it will return an empty string. It'll also return an empty string
// when the timezone is Etc/UTC or Etc/UCT, but will return 'GB" for Etc/GMT
// because people in the UK tends to select Etc/GMT by mistake instead of
// Europe/London (British Time).
BASE_I18N_EXPORT std::string CountryCodeForCurrentTimezone();

}  // namespace base

#endif  // BASE_I18N_TIMEZONE_H_
