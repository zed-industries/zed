// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_TEST_TEST_WEB_FRAME_HELPER_H_
#define THIRD_PARTY_BLINK_PUBLIC_TEST_TEST_WEB_FRAME_HELPER_H_

namespace blink {

class WebLocalFrame;
struct WebNavigationParams;

// Functions in this class may only be used in tests.
class TestWebFrameHelper {
 public:
  // This function loads srcdoc content from the navigation parameters for
  // tests.
  static void FillStaticResponseForSrcdocNavigation(WebLocalFrame*,
                                                    WebNavigationParams*);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_TEST_TEST_WEB_FRAME_HELPER_H_
