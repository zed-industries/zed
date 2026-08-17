// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PRESENTATION_MOCK_WEB_PRESENTATION_CLIENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PRESENTATION_MOCK_WEB_PRESENTATION_CLIENT_H_

#include "testing/gmock/include/gmock/gmock.h"
#include "third_party/blink/public/platform/modules/presentation/web_presentation_client.h"

namespace blink {

class MockWebPresentationClient : public WebPresentationClient {
 public:
  MOCK_METHOD1(SetReceiver, void(WebPresentationReceiver*));
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PRESENTATION_MOCK_WEB_PRESENTATION_CLIENT_H_
