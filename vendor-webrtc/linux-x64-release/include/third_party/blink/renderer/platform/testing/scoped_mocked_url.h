// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_SCOPED_MOCKED_URL_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_SCOPED_MOCKED_URL_H_

#include "third_party/blink/public/platform/web_string.h"
#include "third_party/blink/public/platform/web_url.h"
#include "third_party/blink/public/platform/web_url_response.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

namespace test {

// Convenience classes that register a mocked URL on construction, and
// unregister it on destruction. This prevent mocked URL from leaking to other
// tests.
class ScopedMockedURL {
  STACK_ALLOCATED();

 public:
  explicit ScopedMockedURL(const WebURL&);
  virtual ~ScopedMockedURL();

 private:
  WebURL url_;
};

class ScopedMockedURLLoad : ScopedMockedURL {
 public:
  ScopedMockedURLLoad(
      const WebURL& full_url,
      const WebString& file_path,
      const WebString& mime_type = WebString::FromUTF8("text/html"));
  ~ScopedMockedURLLoad() override = default;
};

}  // namespace test

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_SCOPED_MOCKED_URL_H_
