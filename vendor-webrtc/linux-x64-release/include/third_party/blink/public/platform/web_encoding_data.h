// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_ENCODING_DATA_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_ENCODING_DATA_H_

#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_string.h"

namespace blink {

struct BLINK_PLATFORM_EXPORT WebEncodingData {
  WebString encoding;
  bool was_detected_heuristically = false;
  bool saw_decoding_error = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_ENCODING_DATA_H_
