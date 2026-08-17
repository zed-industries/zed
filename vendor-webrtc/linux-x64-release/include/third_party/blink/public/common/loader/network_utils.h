// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_NETWORK_UTILS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_NETWORK_UTILS_H_

#include "base/memory/scoped_refptr.h"
#include "net/http/http_response_headers.h"
#include "services/network/public/mojom/fetch_api.mojom-forward.h"
#include "third_party/blink/public/common/common_export.h"

namespace net {
class HttpRequestHeaders;
}  // namespace net

namespace blink {
namespace network_utils {

// Returns true if the headers indicate that this resource should always be
// revalidated or not cached.
BLINK_COMMON_EXPORT bool AlwaysAccessNetwork(
    const scoped_refptr<net::HttpResponseHeaders>& headers);

// Returns the accept header for image resources.
BLINK_COMMON_EXPORT const char* ImageAcceptHeader();

// Sets or update Accept header based on `request_destination`.
BLINK_COMMON_EXPORT void SetAcceptHeader(
    net::HttpRequestHeaders& headers,
    network::mojom::RequestDestination request_destination);

// Returns a Accept header string for `request_destination`.
BLINK_COMMON_EXPORT const char* GetAcceptHeaderForDestination(
    network::mojom::RequestDestination request_destination);

}  // namespace network_utils
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_NETWORK_UTILS_H_
