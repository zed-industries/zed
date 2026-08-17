// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_LOADER_FACTORY_FOR_WORKER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_LOADER_FACTORY_FOR_WORKER_H_

#include <memory>
#include <utility>
#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_fetcher.h"

namespace blink {

class WorkerOrWorkletGlobalScope;
class WebWorkerFetchContext;

// ResourceFetcher::LoaderFactory implementation for workers and worklets.
class LoaderFactoryForWorker : public ResourceFetcher::LoaderFactory {
 public:
  LoaderFactoryForWorker(WorkerOrWorkletGlobalScope& global_scope,
                         scoped_refptr<WebWorkerFetchContext> web_context);

  void Trace(Visitor* visitor) const override;

  // LoaderFactory implementations
  std::unique_ptr<URLLoader> CreateURLLoader(
      const network::ResourceRequest& request,
      const ResourceLoaderOptions& options,
      scoped_refptr<base::SingleThreadTaskRunner> freezable_task_runner,
      scoped_refptr<base::SingleThreadTaskRunner> unfreezable_task_runner,
      BackForwardCacheLoaderHelper*,
      const std::optional<base::UnguessableToken>&
          service_worker_race_network_request_token,
      bool is_from_origin_dirty_style_sheet) override;
  CodeCacheHost* GetCodeCacheHost() override;

 private:
  const Member<WorkerOrWorkletGlobalScope> global_scope_;
  const scoped_refptr<WebWorkerFetchContext> web_context_;
};

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_LOADER_FACTORY_FOR_WORKER_H_
