// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_ICON_SIZES_PARSER_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_ICON_SIZES_PARSER_H_

#include <vector>

#include "third_party/blink/public/platform/web_common.h"

namespace gfx {
class Size;
}  // namespace gfx

namespace blink {

class WebString;

// Helper class for parsing icon sizes. The spec is:
// https://html.spec.whatwg.org/multipage/semantics.html#attr-link-sizes
// TODO(zqzhang): merge with WebIconURL, and rename it "WebIcon"?
class BLINK_PLATFORM_EXPORT WebIconSizesParser {
 public:
  static std::vector<gfx::Size> ParseIconSizes(const WebString& sizes_string);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_ICON_SIZES_PARSER_H_
