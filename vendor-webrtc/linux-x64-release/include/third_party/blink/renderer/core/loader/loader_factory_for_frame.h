// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_LOADER_FACTORY_FOR_FRAME_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_LOADER_FACTORY_FOR_FRAME_H_

#include <memory>
#include <utility>
#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/public/mojom/frame/frame.mojom-blink.h"
#include "third_party/blink/public/mojom/loader/keep_alive_handle.mojom-blink.h"
#include "third_party/blink/public/mojom/loader/keep_alive_handle_factory.mojom-blink.h"
#include "third_party/blink/public/platform/url_loader_throttle_provider.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/loader/fetch/background_code_cache_host.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_fetcher.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class DocumentLoader;
class LocalDOMWindow;
class PrefetchedSignedExchangeManager;

class CORE_EXPORT LoaderFactoryForFrame final
    : public ResourceFetcher::LoaderFactory {
 public:
  static void SetCorsExemptHeaderList(Vector<String>);
  static Vector<String> GetCorsExemptHeaderList();

  LoaderFactoryForFrame(DocumentLoader& loader, LocalDOMWindow& window);

  void Trace(Visitor*) const override;

  // LoaderFactory implementations
  std::unique_ptr<URLLoader> CreateURLLoader(
      const network::ResourceRequest&,
      const ResourceLoaderOptions&,
      scoped_refptr<base::SingleThreadTaskRunner>,
      scoped_refptr<base::SingleThreadTaskRunner>,
      BackForwardCacheLoaderHelper*,
      const std::optional<base::UnguessableToken>&
          service_worker_race_network_request_token,
      bool is_from_origin_dirty_style_sheet) override;
  CodeCacheHost* GetCodeCacheHost() override;

 private:
  mojo::PendingRemote<mojom::blink::KeepAliveHandle> MaybeIssueKeepAliveHandle(
      const network::ResourceRequest& network_request);
  scoped_refptr<BackgroundCodeCacheHost> GetBackgroundCodeCacheHost();

  URLLoaderThrottleProvider* GetURLLoaderThrottleProvider();
  Vector<std::unique_ptr<URLLoaderThrottle>> CreateThrottles(
      const network::ResourceRequest&);

  const Member<DocumentLoader> document_loader_;
  const Member<LocalDOMWindow> window_;
  const Member<PrefetchedSignedExchangeManager>
      prefetched_signed_exchange_manager_;
  HeapMojoRemote<mojom::blink::KeepAliveHandleFactory>
      keep_alive_handle_factory_;
  scoped_refptr<BackgroundCodeCacheHost> background_code_cache_host_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_LOADER_FACTORY_FOR_FRAME_H_
