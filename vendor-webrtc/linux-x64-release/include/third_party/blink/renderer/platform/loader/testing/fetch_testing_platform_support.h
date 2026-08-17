// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_TESTING_FETCH_TESTING_PLATFORM_SUPPORT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_TESTING_FETCH_TESTING_PLATFORM_SUPPORT_H_

#include <memory>

#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/testing/testing_platform_support.h"

namespace blink {

class URLLoaderMockFactory;

class FetchTestingPlatformSupport : public TestingPlatformSupport {
 public:
  FetchTestingPlatformSupport();
  FetchTestingPlatformSupport(const FetchTestingPlatformSupport&) = delete;
  FetchTestingPlatformSupport& operator=(const FetchTestingPlatformSupport&) =
      delete;
  ~FetchTestingPlatformSupport() override;

  URLLoaderMockFactory* GetURLLoaderMockFactory();

 private:
  class FetchTestingURLLoaderMockFactory;

  std::unique_ptr<URLLoaderMockFactory> url_loader_mock_factory_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_TESTING_FETCH_TESTING_PLATFORM_SUPPORT_H_
