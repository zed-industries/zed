// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_SOURCE_LOCATION_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_SOURCE_LOCATION_H_

#include "third_party/blink/public/platform/web_string.h"

namespace blink {

// PlzNavigate
// This struct is passed to the browser when navigating, so that console error
// messages due to the navigation do not lose the source location information.
struct WebSourceLocation {
  WebString url;
  unsigned line_number = 0;
  unsigned column_number = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_SOURCE_LOCATION_H_
