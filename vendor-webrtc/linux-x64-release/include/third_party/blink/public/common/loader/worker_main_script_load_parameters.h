// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_WORKER_MAIN_SCRIPT_LOAD_PARAMETERS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_WORKER_MAIN_SCRIPT_LOAD_PARAMETERS_H_

#include <vector>

#include "net/url_request/redirect_info.h"
#include "services/network/public/mojom/url_loader.mojom.h"
#include "services/network/public/mojom/url_response_head.mojom.h"
#include "third_party/blink/public/common/common_export.h"
#include "url/gurl.h"

namespace blink {

// Used to load the main script for dedicated workers, shared workers, and
// service workers, which is pre-requested by browser process.
struct BLINK_COMMON_EXPORT WorkerMainScriptLoadParameters {
 public:
  WorkerMainScriptLoadParameters() = default;
  ~WorkerMainScriptLoadParameters() = default;

  int request_id;
  network::mojom::URLResponseHeadPtr response_head;
  mojo::ScopedDataPipeConsumerHandle response_body;
  network::mojom::URLLoaderClientEndpointsPtr url_loader_client_endpoints;
  std::vector<net::RedirectInfo> redirect_infos;
  std::vector<network::mojom::URLResponseHeadPtr> redirect_responses;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_LOADER_WORKER_MAIN_SCRIPT_LOAD_PARAMETERS_H_
