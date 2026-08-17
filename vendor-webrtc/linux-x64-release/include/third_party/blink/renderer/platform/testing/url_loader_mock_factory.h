// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_URL_LOADER_MOCK_FACTORY_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_URL_LOADER_MOCK_FACTORY_H_

#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_data.h"
#include "third_party/blink/public/platform/web_string.h"
#include "third_party/blink/renderer/platform/testing/url_loader_test_delegate.h"

#include <memory>

namespace blink {

struct WebNavigationParams;
class WebURL;
class URLLoader;
class WebURLResponse;
struct WebURLError;

class URLLoaderMockFactory {
 public:
  static URLLoaderMockFactory* GetSingletonInstance();

  virtual ~URLLoaderMockFactory() = default;

  // Create a URLLoader that takes care of mocked requests.
  virtual std::unique_ptr<URLLoader> CreateURLLoader() = 0;

  // Registers a response and the file to be served when the specified URL
  // is loaded. If no file is specified then the response content will be empty.
  // UnregisterURL() should be called for each test entry before registering
  // another response for the same URL from another test.
  virtual void RegisterURL(const WebURL&,
                           const WebURLResponse&,
                           const WebString& file_path) = 0;

  // Registers an error to be served when the specified URL is requested.
  // UnregisterURL() should be called for each test entry before registering
  // another response for the same URL from another test.
  virtual void RegisterErrorURL(const WebURL&,
                                const WebURLResponse&,
                                const WebURLError&) = 0;

  // Unregisters the given URL so it is no longer mocked.
  virtual void UnregisterURL(const WebURL&) = 0;

  // Registers a response and the file to be served when a URL with the given
  // protocol is loaded. If no file is specified then the response content will
  // be empty. UnregisterProtocol() should be called for each test entry before
  // registering another response for the same protocol from another test.
  virtual void RegisterURLProtocol(const WebString& protocol,
                                   const WebURLResponse&,
                                   const WebString& file_path) = 0;

  // Unregisters the given URL protocol so it is no longer mocked.
  virtual void UnregisterURLProtocol(const WebString& protocol) = 0;

  // Unregisters URLs so they are no longer mocked. This also clears the
  // MemoryCache.
  virtual void UnregisterAllURLsAndClearMemoryCache() = 0;

  // Causes all pending asynchronous requests to be served. When this method
  // returns all the pending requests have been processed.
  // Note: this may not work as expected if more requests could be made
  // asynchronously from different threads.
  // DO NOT USE THIS for Frame loading; always use methods defined in
  // frame_test_helpers instead.
  virtual void ServeAsynchronousRequests() = 0;

  // Set a delegate that allows callbacks for all URLLoaderClients to be
  // intercepted.
  virtual void SetLoaderDelegate(URLLoaderTestDelegate*) = 0;

  // Fills navigation params by loading a mocked response.
  virtual void FillNavigationParamsResponse(WebNavigationParams*) = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_URL_LOADER_MOCK_FACTORY_H_
