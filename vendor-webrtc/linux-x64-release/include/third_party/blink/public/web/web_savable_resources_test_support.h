// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_SAVABLE_RESOURCES_TEST_SUPPORT_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_SAVABLE_RESOURCES_TEST_SUPPORT_H_

#include "third_party/blink/public/platform/web_common.h"

namespace blink {

class WebString;
class WebElement;

BLINK_EXPORT WebString
GetSubResourceLinkFromElementForTesting(const WebElement& element);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_SAVABLE_RESOURCES_TEST_SUPPORT_H_
