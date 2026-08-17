// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_WEB_ASSOCIATED_URL_LOADER_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_WEB_ASSOCIATED_URL_LOADER_IMPL_H_

#include "base/memory/scoped_refptr.h"
#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/public/web/web_associated_url_loader.h"
#include "third_party/blink/public/web/web_associated_url_loader_options.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"

namespace blink {

class ThreadableLoader;
class WebAssociatedURLLoaderClient;
class ExecutionContext;

// This class is used to implement WebFrame::createAssociatedURLLoader.
class CORE_EXPORT WebAssociatedURLLoaderImpl final
    : public WebAssociatedURLLoader {
  USING_FAST_MALLOC(WebAssociatedURLLoaderImpl);

 public:
  WebAssociatedURLLoaderImpl(ExecutionContext*,
                             const WebAssociatedURLLoaderOptions&);
  WebAssociatedURLLoaderImpl(const WebAssociatedURLLoaderImpl&) = delete;
  WebAssociatedURLLoaderImpl& operator=(const WebAssociatedURLLoaderImpl&) =
      delete;
  ~WebAssociatedURLLoaderImpl() override;

  void LoadAsynchronously(const WebURLRequest&,
                          WebAssociatedURLLoaderClient*) override;
  void Cancel() override;
  void SetDefersLoading(bool) override;
  void SetLoadingTaskRunner(base::SingleThreadTaskRunner*) override;

  // Called by ClientAdapter to handle completion of loading.
  void ClientAdapterDone();

 private:
  class ClientAdapter;
  class Observer;

  void ContextDestroyed();
  void CancelLoader();
  void DisposeObserver();

  WebAssociatedURLLoaderClient* ReleaseClient() {
    WebAssociatedURLLoaderClient* client = client_;
    client_ = nullptr;
    return client;
  }

  WebAssociatedURLLoaderClient* client_;
  WebAssociatedURLLoaderOptions options_;

  // Converts ThreadableLoaderClient method calls into URLLoaderClient method
  // calls.
  Persistent<ClientAdapter> client_adapter_;
  Persistent<ThreadableLoader> loader_;

  // A ExecutionContextLifecycleObserver for cancelling |loader_| when the
  // context is detached.
  Persistent<Observer> observer_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_WEB_ASSOCIATED_URL_LOADER_IMPL_H_
