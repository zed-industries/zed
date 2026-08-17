// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_DEDICATED_WORKER_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_DEDICATED_WORKER_H_

#include <memory>

#include "third_party/blink/public/mojom/browser_interface_broker.mojom-shared.h"
#include "third_party/blink/public/mojom/frame/back_forward_cache_controller.mojom-shared.h"
#include "third_party/blink/public/mojom/frame/reporting_observer.mojom-shared.h"
#include "third_party/blink/public/mojom/worker/dedicated_worker_host.mojom-shared.h"
#include "third_party/blink/public/platform/cross_variant_mojo_util.h"
#include "third_party/blink/public/platform/web_security_origin.h"

namespace blink {

struct WorkerMainScriptLoadParameters;

// WebDedicatedWorker is the interface to access blink::DedicatedWorker from
// content::DedicatedWorkerHostFactoryClient.
class WebDedicatedWorker {
 public:
  virtual ~WebDedicatedWorker() = default;

  // Called when content::DedicatedWorkerHost is created in the browser process.
  virtual void OnWorkerHostCreated(
      CrossVariantMojoRemote<mojom::BrowserInterfaceBrokerInterfaceBase>
          browser_interface_broker,
      CrossVariantMojoRemote<mojom::DedicatedWorkerHostInterfaceBase>
          dedicated_worker_host,
      const WebSecurityOrigin& origin) = 0;

  // Called when content::DedicatedWorkerHost started loading the main worker
  // script in the browser process, and the script information is sent back to
  // the content::DedicatedWorkerHostFactoryClient.
  virtual void OnScriptLoadStarted(
      std::unique_ptr<WorkerMainScriptLoadParameters>
          worker_main_script_load_params,
      CrossVariantMojoRemote<mojom::BackForwardCacheControllerHostInterfaceBase>
          back_forward_cache_controller_host,
      CrossVariantMojoReceiver<mojom::ReportingObserverInterfaceBase>
          coep_reporting_observer,
      CrossVariantMojoReceiver<mojom::ReportingObserverInterfaceBase>
          dip_reporting_observer) = 0;

  // Called when content::DedicatedWorkerHost failed to start loading the main
  // worker script in the browser process.
  virtual void OnScriptLoadStartFailed() = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_DEDICATED_WORKER_H_
