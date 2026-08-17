// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_IMAGE_DOWNLOADER_MULTI_RESOLUTION_IMAGE_RESOURCE_FETCHER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_IMAGE_DOWNLOADER_MULTI_RESOLUTION_IMAGE_RESOURCE_FETCHER_H_

#include <memory>
#include <string>

#include "base/functional/callback.h"
#include "third_party/blink/public/mojom/fetch/fetch_api_request.mojom-blink-forward.h"
#include "third_party/blink/public/platform/web_url_request.h"
#include "third_party/blink/public/web/web_associated_url_loader_options.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {
class KURL;
class LocalFrame;
class WebAssociatedURLLoader;
class WebString;
class WebURLResponse;

// A resource fetcher that returns all (differently-sized) frames in
// an image. Useful for favicons.
class MultiResolutionImageResourceFetcher {
  USING_FAST_MALLOC(MultiResolutionImageResourceFetcher);

 public:
  // The std::string arguments are, in order, the data and the MIME type
  // (Content-Type) of the response.
  using Callback = base::OnceCallback<void(MultiResolutionImageResourceFetcher*,
                                           const std::string& data,
                                           const WebString& mime_type)>;

  // This will be called asynchronously after the URL has been fetched,
  // successfully or not.  If there is a failure, response and data will both be
  // empty.  |response| and |data| are both valid until the URLFetcher instance
  // is destroyed.
  using StartCallback = base::OnceCallback<void(const WebURLResponse& response,
                                                const std::string& data)>;

  MultiResolutionImageResourceFetcher(const KURL& image_url,
                                      LocalFrame* frame,
                                      bool is_favicon,
                                      mojom::blink::FetchCacheMode cache_mode,
                                      Callback callback);

  MultiResolutionImageResourceFetcher(
      const MultiResolutionImageResourceFetcher&) = delete;
  MultiResolutionImageResourceFetcher& operator=(
      const MultiResolutionImageResourceFetcher&) = delete;

  virtual ~MultiResolutionImageResourceFetcher();

  // HTTP status code upon fetch completion.
  int http_status_code() const { return http_status_code_; }

  // Called when ImageDownloaderImpl::ContextDestroyed is called.
  void Dispose();

 private:
  class ClientImpl;

  // ResourceFetcher::Callback. Checks if the fetch succeeded and invokes
  // |callback_|.
  void OnURLFetchComplete(const WebURLResponse& response,
                          const std::string& data);

  void SetSkipServiceWorker(bool skip_service_worker);
  void SetCacheMode(mojom::FetchCacheMode mode);

  // Associate the corresponding URLLoaderOptions to the loader. Must be
  // called before Start. Used if the LoaderType is FRAME_ASSOCIATED_LOADER.
  void SetLoaderOptions(const WebAssociatedURLLoaderOptions& options);

  // Starts the request using the specified frame.  Calls |callback| when
  // done.
  //
  // |fetch_request_mode| is the mode to use. See
  // https://fetch.spec.whatwg.org/#concept-request-mode.
  //
  // |fetch_credentials_mode| is the credentials mode to use. See
  // https://fetch.spec.whatwg.org/#concept-request-credentials-mode
  void Start(LocalFrame* frame,
             bool is_favicon,
             network::mojom::RequestMode request_mode,
             network::mojom::CredentialsMode credentials_mode,
             StartCallback callback);

  // Manually cancel the request.
  void Cancel();

  Callback callback_;

  // HTTP status code upon fetch completion.
  int http_status_code_;

  std::unique_ptr<WebAssociatedURLLoader> loader_;
  std::unique_ptr<ClientImpl> client_;

  // Options to send to the loader.
  WebAssociatedURLLoaderOptions options_;

  // Request to send.
  WebURLRequest request_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_IMAGE_DOWNLOADER_MULTI_RESOLUTION_IMAGE_RESOURCE_FETCHER_H_
