// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_URL_LOADER_MOCK_FACTORY_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_URL_LOADER_MOCK_FACTORY_IMPL_H_

#include <optional>

#include "base/files/file_path.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/weak_ptr.h"
#include "third_party/blink/public/platform/web_url.h"
#include "third_party/blink/public/platform/web_url_error.h"
#include "third_party/blink/public/platform/web_url_request.h"
#include "third_party/blink/public/platform/web_url_response.h"
#include "third_party/blink/renderer/platform/testing/url_loader_mock_factory.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/weborigin/kurl_hash.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"

namespace network {
struct ResourceRequest;
}  // namespace network

namespace blink {

class TestingPlatformSupport;
class URLLoader;
class URLLoaderMock;
class URLLoaderTestDelegate;

// A factory that creates URLLoaderMock to simulate resource loading in tests.
// Since there are restriction and rules to follow, please read comments in
// URLLoaderMockFactory carefully to use this class correctly.
class URLLoaderMockFactoryImpl : public URLLoaderMockFactory {
 public:
  URLLoaderMockFactoryImpl(TestingPlatformSupport*);
  URLLoaderMockFactoryImpl(const URLLoaderMockFactoryImpl&) = delete;
  URLLoaderMockFactoryImpl& operator=(const URLLoaderMockFactoryImpl&) = delete;
  ~URLLoaderMockFactoryImpl() override;

  // URLLoaderMockFactory:
  std::unique_ptr<URLLoader> CreateURLLoader() override;
  void RegisterURL(const WebURL& url,
                   const WebURLResponse& response,
                   const WebString& file_path = WebString()) override;
  void RegisterErrorURL(const WebURL& url,
                        const WebURLResponse& response,
                        const WebURLError& error) override;
  void UnregisterURL(const WebURL& url) override;
  void RegisterURLProtocol(const WebString& protocol,
                           const WebURLResponse& response,
                           const WebString& file_path) override;
  void UnregisterURLProtocol(const WebString& protocol) override;
  void UnregisterAllURLsAndClearMemoryCache() override;
  void ServeAsynchronousRequests() override;
  void SetLoaderDelegate(URLLoaderTestDelegate* delegate) override {
    delegate_ = delegate;
  }
  void FillNavigationParamsResponse(WebNavigationParams*) override;

  // Returns true if |url| was registered for being mocked.
  bool IsMockedURL(const WebURL& url);

  // Called by the loader to load a resource.
  void LoadSynchronously(std::unique_ptr<network::ResourceRequest> request,
                         WebURLResponse* response,
                         std::optional<WebURLError>* error,
                         scoped_refptr<SharedBuffer>& data,
                         int64_t* encoded_data_length);
  void LoadAsynchronouly(std::unique_ptr<network::ResourceRequest> request,
                         URLLoaderMock* loader);

  // Removes the loader from the list of pending loaders.
  void CancelLoad(URLLoaderMock* loader);

 private:
  struct ResponseInfo {
    WebURLResponse response;
    base::FilePath file_path;
  };

  virtual void RunUntilIdle();

  // Loads the specified request and populates the response, error and data
  // accordingly.
  void LoadRequest(const WebURL& url,
                   WebURLResponse* response,
                   std::optional<WebURLError>* error,
                   scoped_refptr<SharedBuffer>& data);

  // Checks if the loader is pending. Otherwise, it may have been deleted.
  bool IsPending(base::WeakPtr<URLLoaderMock> loader);

  // Looks up an URL in the mock URL table.
  //
  // If the URL is found, returns true and sets |error| and |response_info|.
  bool LookupURL(const WebURL& url,
                 std::optional<WebURLError>* error,
                 ResponseInfo* response_info);

  // Reads 'file_path' and puts its content in 'data'.
  // Returns true if it successfully read the file.
  static bool ReadFile(const base::FilePath& file_path,
                       scoped_refptr<SharedBuffer>& data);

  raw_ptr<URLLoaderTestDelegate> delegate_ = nullptr;

  // The loaders that have not being served data yet.
  using LoaderToRequestMap =
      HashMap<URLLoaderMock*, std::unique_ptr<network::ResourceRequest>>;
  LoaderToRequestMap pending_loaders_;

  // All values must be valid, but we use Optional because HashMap requires
  // "empty value".
  typedef HashMap<KURL, std::optional<WebURLError>> URLToErrorMap;
  URLToErrorMap url_to_error_info_;

  // Table of the registered URLs and the responses that they should receive.
  using URLToResponseMap = HashMap<KURL, ResponseInfo>;
  URLToResponseMap url_to_response_info_;

  // Table of the registered URL protocols and the responses that they should
  // receive.
  using ProtocolToResponseMap = HashMap<String, ResponseInfo>;
  ProtocolToResponseMap protocol_to_response_info_;

  raw_ptr<TestingPlatformSupport> platform_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_URL_LOADER_MOCK_FACTORY_IMPL_H_
