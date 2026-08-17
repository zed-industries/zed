// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_TESTING_MOCK_WEB_ASSOCIATED_URL_LOADER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_TESTING_MOCK_WEB_ASSOCIATED_URL_LOADER_H_

#include "base/task/single_thread_task_runner.h"
#include "testing/gmock/include/gmock/gmock.h"
#include "third_party/blink/public/web/web_associated_url_loader.h"

namespace blink {

class MockWebAssociatedURLLoader : public WebAssociatedURLLoader {
 public:
  MockWebAssociatedURLLoader();
  MockWebAssociatedURLLoader(const MockWebAssociatedURLLoader&) = delete;
  MockWebAssociatedURLLoader& operator=(const MockWebAssociatedURLLoader&) =
      delete;
  ~MockWebAssociatedURLLoader() override;

  MOCK_METHOD2(LoadAsynchronously,
               void(const WebURLRequest& request,
                    WebAssociatedURLLoaderClient* client));
  MOCK_METHOD0(Cancel, void());
  MOCK_METHOD1(SetDefersLoading, void(bool value));
  MOCK_METHOD1(SetLoadingTaskRunner,
               void(base::SingleThreadTaskRunner* task_runner));
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_TESTING_MOCK_WEB_ASSOCIATED_URL_LOADER_H_
