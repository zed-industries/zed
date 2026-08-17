// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_I18N_ENCODING_DETECTION_H_
#define BASE_I18N_ENCODING_DETECTION_H_

#include <string>

#include "base/i18n/base_i18n_export.h"

namespace base {

// Detect encoding of |text| and put the name of encoding in |encoding|.
// Returns true on success.
[[nodiscard]] BASE_I18N_EXPORT bool DetectEncoding(const std::string& text,
                                                   std::string* encoding);
}  // namespace base

#endif  // BASE_I18N_ENCODING_DETECTION_H_
