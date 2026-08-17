// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_URL_LOADER_SYNC_LOAD_RESPONSE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_URL_LOADER_SYNC_LOAD_RESPONSE_H_

#include <optional>

#include "net/dns/public/resolve_error_info.h"
#include "services/network/public/cpp/cors/cors_error_status.h"
#include "services/network/public/mojom/url_response_head.mojom.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/renderer/platform/allow_discouraged_type.h"
#include "third_party/blink/renderer/platform/blob/blob_data.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "url/gurl.h"

namespace blink {

// See the SyncLoad method. (The name of this struct is not
// suffixed with "Info" because it also contains the response data.)
struct BLINK_PLATFORM_EXPORT SyncLoadResponse {
  SyncLoadResponse();
  SyncLoadResponse(SyncLoadResponse&& other);
  ~SyncLoadResponse();

  SyncLoadResponse& operator=(SyncLoadResponse&& other);

  std::optional<net::RedirectInfo> redirect_info;

  network::mojom::URLResponseHeadPtr head =
      network::mojom::URLResponseHead::New();

  // The response error code.
  int error_code;

  // The response extended error code.
  int extended_error_code = 0;

  bool should_collapse_initiator = false;

  // Detailed host resolution error information.
  net::ResolveErrorInfo resolve_error_info;

  // Optional CORS error details.
  std::optional<network::CorsErrorStatus> cors_error;

  // The final URL of the response.  This may differ from the request URL in
  // the case of a server redirect.
  // Use GURL to avoid extra conversion between KURL and GURL because non-blink
  // types are allowed for loader here.
  GURL url ALLOW_DISCOURAGED_TYPE("Avoids conversion in loading code");

  // The response data.
  scoped_refptr<SharedBuffer> data;

  // Used for blob response type XMLHttpRequest.
  scoped_refptr<BlobDataHandle> downloaded_blob;

  // True when cross origin redirects happen with Authorization header.
  // TODO(https://crbug.com/1393520): Remove this field once we get enough
  // stats to make a decision.
  bool has_authorization_header_between_cross_origin_redirect_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_URL_LOADER_SYNC_LOAD_RESPONSE_H_
