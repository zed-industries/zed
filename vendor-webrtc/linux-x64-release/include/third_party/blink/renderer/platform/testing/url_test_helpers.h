/*
 * Copyright (C) 2012 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_URL_TEST_HELPERS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_URL_TEST_HELPERS_H_

#include "services/network/public/mojom/ip_address_space.mojom-shared.h"
#include "third_party/blink/public/platform/web_string.h"
#include "third_party/blink/public/platform/web_url.h"
#include "third_party/blink/public/platform/web_url_response.h"
#include "third_party/blink/renderer/platform/testing/url_loader_mock_factory.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"

namespace blink {
namespace url_test_helpers {

inline blink::KURL ToKURL(const std::string& url) {
  WTF::String wtf_string(url.c_str());
  return blink::KURL(wtf_string);
}

// Avoid directly using these methods, instead please use ScopedMockedURL (or
// define a variant of ScopedMockedURL classes if necessary) in new code.
//
// Helper functions for mock URLs. These functions set up the desired URL and
// mimeType, with a 200 OK return status.
// webTestDataPath() or platformTestDataPath() in UnitTestHelpers can be used to
// get the appropriate |basePath| and |filePath| for test data directories.
//  - For the mock URL, fullURL == baseURL + fileName.
//  - For the file path, filePath == basePath + ("/" +) fileName.
//
// TODO(kinuko,toyoshim): These helpers should take URLLoaderMockFactory as a
// parameter, or maybe have static {Set,Get}URLLoaderMockFactory() methods
// so that we can deprecate the platform's GetURLLoaderMockFactory().
// (crbug.com/751425)

// Registers from a base URL and a base file path, and returns a calculated full
// URL.
WebURL RegisterMockedURLLoadFromBase(
    const WebString& base_url,
    const WebString& base_path,
    const WebString& file_name,
    const WebString& mime_type = WebString::FromUTF8("text/html"));

// Registers from a full URL and a full file path.
void RegisterMockedURLLoad(
    const WebURL& full_url,
    const WebString& file_path,
    const WebString& mime_type = WebString::FromUTF8("text/html"),
    URLLoaderMockFactory* mock_factory =
        URLLoaderMockFactory::GetSingletonInstance(),
    network::mojom::IPAddressSpace address_space =
        network::mojom::IPAddressSpace::kPublic);

// Unregisters a URL that has been registered, so that the same URL can be
// registered again from the another test.
void RegisterMockedURLUnregister(const WebURL&);

// Registers with a custom response.
void RegisterMockedURLLoadWithCustomResponse(const WebURL& full_url,
                                             const WebString& file_path,
                                             WebURLResponse);

// Registers a mock URL that returns a 404 error.
void RegisterMockedErrorURLLoad(
    const WebURL& full_url,
    URLLoaderMockFactory* mock_factory =
        URLLoaderMockFactory::GetSingletonInstance());

void UnregisterAllURLsAndClearMemoryCache();

void SetLoaderDelegate(URLLoaderTestDelegate* delegate);

void ServeAsynchronousRequests();

}  // namespace url_test_helpers
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_URL_TEST_HELPERS_H_
