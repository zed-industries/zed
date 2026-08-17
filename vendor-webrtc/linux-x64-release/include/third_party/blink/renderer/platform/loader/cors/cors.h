// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_CORS_CORS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_CORS_CORS_H_

#include "services/network/public/cpp/cors/cors_error_status.h"
#include "services/network/public/mojom/cors.mojom-blink-forward.h"
#include "services/network/public/mojom/fetch_api.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/fetch/fetch_api_request.mojom-blink-forward.h"
#include "third_party/blink/renderer/platform/network/http_header_set.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class HTTPHeaderMap;
class KURL;
class ResourceResponse;
class SecurityOrigin;

enum class CorsFlag : uint8_t {
  Unset,
  Set,
};

// CORS related utility functions.
namespace cors {

// Thin wrapper functions below are for calling ::network::cors functions from
// Blink core.
PLATFORM_EXPORT bool IsCorsEnabledRequestMode(network::mojom::RequestMode);
PLATFORM_EXPORT bool IsCorsSafelistedMethod(const String& method);
PLATFORM_EXPORT bool IsCorsSafelistedContentType(const String&);
PLATFORM_EXPORT bool IsNoCorsSafelistedHeader(const String& name,
                                              const String& value);
PLATFORM_EXPORT bool IsPrivilegedNoCorsHeaderName(const String& name);
PLATFORM_EXPORT bool IsNoCorsSafelistedHeaderName(const String& name);
PLATFORM_EXPORT Vector<String> PrivilegedNoCorsHeaderNames();
PLATFORM_EXPORT bool IsForbiddenRequestHeader(const String& name,
                                              const String& value);
PLATFORM_EXPORT bool ContainsOnlyCorsSafelistedHeaders(const HTTPHeaderMap&);

PLATFORM_EXPORT bool IsOkStatus(int status);

// Calculates and returns the CORS flag used in several "fetch" algorithms in
// https://fetch.spec.whatwg.org/. This function is corresponding to the CORS
// flag setting logic in https://fetch.spec.whatwg.org/#main-fetch.
// This function can return true even when |request_mode| is |kSameOrigin|.
// |origin| must not be nullptr when |request_mode| is neither |kNoCors| nor
// |kNavigate|.
// This should be identical to CalculateCorsFlag defined in
// //services/network/cors/cors_url_loader.cc.
PLATFORM_EXPORT bool CalculateCorsFlag(
    const KURL& url,
    const SecurityOrigin* initiator_origin,
    const SecurityOrigin* isolated_world_origin,
    network::mojom::RequestMode request_mode);

PLATFORM_EXPORT HTTPHeaderSet
ExtractCorsExposedHeaderNamesList(network::mojom::CredentialsMode,
                                  const ResourceResponse&);

PLATFORM_EXPORT bool IsCorsSafelistedResponseHeader(const String&);

// Checks whether request mode 'no-cors' is allowed for a certain context.
PLATFORM_EXPORT bool IsNoCorsAllowedContext(mojom::blink::RequestContextType);

}  // namespace cors

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_CORS_CORS_H_
