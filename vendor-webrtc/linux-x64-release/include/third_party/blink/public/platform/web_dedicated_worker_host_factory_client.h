// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_DEDICATED_WORKER_HOST_FACTORY_CLIENT_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_DEDICATED_WORKER_HOST_FACTORY_CLIENT_H_

#include "base/memory/scoped_refptr.h"
#include "base/unguessable_token.h"
#include "net/storage_access_api/status.h"
#include "services/network/public/mojom/fetch_api.mojom-shared.h"
#include "services/network/public/mojom/referrer_policy.mojom-shared.h"
#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/public/mojom/blob/blob_url_store.mojom-shared.h"
#include "third_party/blink/public/mojom/frame/back_forward_cache_controller.mojom-shared.h"
#include "third_party/blink/public/mojom/frame/lifecycle.mojom-shared.h"
#include "third_party/blink/public/platform/cross_variant_mojo_util.h"
#include "third_party/blink/public/platform/web_fetch_client_settings_object.h"

namespace base {
class SingleThreadTaskRunner;
}

namespace network {
struct CrossOriginEmbedderPolicy;
}

namespace blink {

class WebURL;
class WebWorkerFetchContext;

// WebDedicatedWorkerHostFactoryClient is the interface to access
// content::DedicatedWorkerHostFactoryClient from blink::DedicatedWorker.
class WebDedicatedWorkerHostFactoryClient {
 public:
  using CreateWorkerHostCallback = base::OnceCallback<void(
      const network::CrossOriginEmbedderPolicy&,
      CrossVariantMojoRemote<
          mojom::BackForwardCacheControllerHostInterfaceBase>)>;

  virtual ~WebDedicatedWorkerHostFactoryClient() = default;

  // Requests the creation of DedicatedWorkerHost in the browser process.
  virtual void CreateWorkerHost(
      const DedicatedWorkerToken& dedicated_worker_token,
      const blink::WebURL& script_url,
      network::mojom::CredentialsMode credentials_mode,
      const blink::WebFetchClientSettingsObject& fetch_client_settings_object,
      CrossVariantMojoRemote<mojom::BlobURLTokenInterfaceBase> blob_url_token,
      net::StorageAccessApiStatus storage_access_api_status) = 0;

  // Clones the given WebWorkerFetchContext for nested workers.
  virtual scoped_refptr<WebWorkerFetchContext> CloneWorkerFetchContext(
      WebWorkerFetchContext*,
      scoped_refptr<base::SingleThreadTaskRunner>) = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_DEDICATED_WORKER_HOST_FACTORY_CLIENT_H_
