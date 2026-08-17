// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_NO_NETWORK_URL_LOADER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_NO_NETWORK_URL_LOADER_H_

#include "base/task/single_thread_task_runner.h"
#include "services/network/public/cpp/resource_request.h"
#include "third_party/blink/renderer/core/loader/empty_clients.h"
#include "third_party/blink/renderer/platform/loader/fetch/url_loader/url_loader.h"
#include "third_party/blink/renderer/platform/scheduler/test/fake_task_runner.h"

namespace blink {

// A URLLoader simulating that requests time out forever due to no network.
// Useful for perftests that don't really want to benchmark URL loading.
class NoNetworkURLLoader : public URLLoader {
 public:
  NoNetworkURLLoader() = default;
  NoNetworkURLLoader(const NoNetworkURLLoader&) = delete;
  NoNetworkURLLoader& operator=(const NoNetworkURLLoader&) = delete;

  // URLLoader member functions:
  void LoadSynchronously(std::unique_ptr<network::ResourceRequest> request,
                         scoped_refptr<const SecurityOrigin> top_frame_origin,
                         bool download_to_blob,
                         bool no_mime_sniffing,
                         base::TimeDelta timeout_interval,
                         URLLoaderClient* client,
                         WebURLResponse& response,
                         std::optional<WebURLError>&,
                         scoped_refptr<SharedBuffer>&,
                         int64_t& encoded_data_length,
                         uint64_t& encoded_body_length,
                         scoped_refptr<BlobDataHandle>& downloaded_blob,
                         std::unique_ptr<blink::ResourceLoadInfoNotifierWrapper>
                             resource_load_info_notifier_wrapper) override {
    // Nothing should call this in our test.
    NOTREACHED();
  }
  void LoadAsynchronously(
      std::unique_ptr<network::ResourceRequest> request,
      scoped_refptr<const SecurityOrigin> top_frame_origin,
      bool no_mime_sniffing,
      std::unique_ptr<blink::ResourceLoadInfoNotifierWrapper>
          resource_load_info_notifier_wrapper,
      CodeCacheHost* code_cache_host,
      URLLoaderClient* client) override {
    // We simply never call back, simulating load times that are larger
    // than the test runtime.
  }
  void Freeze(LoaderFreezeMode mode) override {
    // Ignore.
  }
  void DidChangePriority(WebURLRequest::Priority new_priority,
                         int intra_priority_value) override {
    // Ignore.
  }
  scoped_refptr<base::SingleThreadTaskRunner> GetTaskRunnerForBodyLoader()
      override {
    return base::MakeRefCounted<scheduler::FakeTaskRunner>();
  }
};

// A LocalFrameClient that uses NoNetworkURLLoader, so that nothing external
// is ever loaded.
class NoNetworkLocalFrameClient : public EmptyLocalFrameClient {
 public:
  NoNetworkLocalFrameClient() = default;

 private:
  std::unique_ptr<URLLoader> CreateURLLoaderForTesting() override {
    return std::make_unique<NoNetworkURLLoader>();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_NO_NETWORK_URL_LOADER_H_
