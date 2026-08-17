// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_TESTING_MOCK_RESOURCE_FETCH_CONTEXT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_TESTING_MOCK_RESOURCE_FETCH_CONTEXT_H_

#include "testing/gmock/include/gmock/gmock.h"
#include "third_party/blink/public/web/web_associated_url_loader.h"
#include "third_party/blink/public/web/web_associated_url_loader_options.h"
#include "third_party/blink/renderer/platform/media/resource_fetch_context.h"

namespace blink {

class MockResourceFetchContext : public ResourceFetchContext {
 public:
  MockResourceFetchContext();
  MockResourceFetchContext(const MockResourceFetchContext&) = delete;
  MockResourceFetchContext& operator=(const MockResourceFetchContext&) = delete;
  ~MockResourceFetchContext() override;

  MOCK_METHOD1(CreateUrlLoader,
               std::unique_ptr<WebAssociatedURLLoader>(
                   const WebAssociatedURLLoaderOptions&));
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_TESTING_MOCK_RESOURCE_FETCH_CONTEXT_H_
