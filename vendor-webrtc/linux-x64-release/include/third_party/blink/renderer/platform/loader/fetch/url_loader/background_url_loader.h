// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_URL_LOADER_BACKGROUND_URL_LOADER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_URL_LOADER_BACKGROUND_URL_LOADER_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/renderer/platform/loader/fetch/url_loader/url_loader.h"

namespace base {
class SingleThreadTaskRunner;
}  // namespace base

namespace network {
struct ResourceRequest;
}  // namespace network

namespace blink {

class BackForwardCacheLoaderHelper;
class BackgroundCodeCacheHost;
class URLLoaderClient;
struct ResourceLoaderOptions;
class WebBackgroundResourceFetchAssets;

// BackgroundURLLoader is used to fetch a resource request on a background
// thread. Used only when BackgroundResourceFetch feature is enabled.
//
// This class has a Context object which is a subclass of ThreadSafeRefCounted.
// This Context receives the response on the background thread and passes the
// response to the URLLoaderClient on the main thread.
//
// Design doc of BackgroundResourceFetch feature:
// https://docs.google.com/document/d/11cEc3KFpM7NwMMY9OkIUkIg5vGisNTXvzLZMgwBM-BM/edit?usp=sharing
// This feature is an internal optimization and doesn't change the exposed JS
// API surfaces.
class BLINK_PLATFORM_EXPORT BackgroundURLLoader : public URLLoader {
 public:
  // This is called from core/ to check if the request is supported by the
  // BackgroundURLLoader, and if this says it's supported and the feature is
  // enabled, the request comes to the BackgroundURLLoader.
  static bool CanHandleRequest(const network::ResourceRequest& request,
                               const ResourceLoaderOptions& options,
                               bool is_prefech_only_document);

  BackgroundURLLoader(
      scoped_refptr<WebBackgroundResourceFetchAssets>
          background_resource_fetch_context,
      const Vector<String>& cors_exempt_header_list,
      scoped_refptr<base::SingleThreadTaskRunner> unfreezable_task_runner,
      BackForwardCacheLoaderHelper* back_forward_cache_loader_helper,
      scoped_refptr<BackgroundCodeCacheHost> background_code_cache_host);
  ~BackgroundURLLoader() override;

  void LoadSynchronously(std::unique_ptr<network::ResourceRequest> request,
                         scoped_refptr<const SecurityOrigin> top_frame_origin,
                         bool download_to_blob,
                         bool no_mime_sniffing,
                         base::TimeDelta timeout_interval,
                         URLLoaderClient* client,
                         WebURLResponse& response,
                         std::optional<WebURLError>& error,
                         scoped_refptr<SharedBuffer>& data,
                         int64_t& encoded_data_length,
                         uint64_t& encoded_body_length,
                         scoped_refptr<BlobDataHandle>& downloaded_blob,
                         std::unique_ptr<ResourceLoadInfoNotifierWrapper>
                             resource_load_info_notifier_wrapper) override;

  void LoadAsynchronously(std::unique_ptr<network::ResourceRequest> request,
                          scoped_refptr<const SecurityOrigin> top_frame_origin,
                          bool no_mime_sniffing,
                          std::unique_ptr<ResourceLoadInfoNotifierWrapper>
                              resource_load_info_notifier_wrapper,
                          CodeCacheHost* code_cache_host,
                          URLLoaderClient* client) override;

  void Freeze(LoaderFreezeMode mode) override;

  void DidChangePriority(WebURLRequest::Priority new_priority,
                         int intra_priority_value) override;

  bool CanHandleResponseOnBackground() override { return true; }
  void SetBackgroundResponseProcessorFactory(
      std::unique_ptr<BackgroundResponseProcessorFactory>
          background_response_processor_factory) override;

  scoped_refptr<base::SingleThreadTaskRunner> GetTaskRunnerForBodyLoader()
      override;

 private:
  class Context;

  scoped_refptr<Context> context_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_URL_LOADER_BACKGROUND_URL_LOADER_H_
