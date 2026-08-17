// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_URL_CONVERSION_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_URL_CONVERSION_H_

#include "third_party/blink/public/platform/web_common.h"

class GURL;

namespace blink {

class WebString;

BLINK_PLATFORM_EXPORT GURL WebStringToGURL(const WebString&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_URL_CONVERSION_H_
