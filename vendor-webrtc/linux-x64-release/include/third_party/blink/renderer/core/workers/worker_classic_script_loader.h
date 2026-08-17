/*
 * Copyright (C) 2009 Apple Inc. All Rights Reserved.
 * Copyright (C) 2009, 2011 Google Inc. All Rights Reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_WORKER_CLASSIC_SCRIPT_LOADER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_WORKER_CLASSIC_SCRIPT_LOADER_H_

#include <memory>
#include "base/memory/scoped_refptr.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "services/network/public/mojom/fetch_api.mojom-blink-forward.h"
#include "services/network/public/mojom/url_loader_factory.mojom-blink.h"
#include "third_party/blink/public/mojom/loader/resource_load_info_notifier.mojom-shared.h"
#include "third_party/blink/public/platform/cross_variant_mojo_util.h"
#include "third_party/blink/public/platform/web_url_request.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/loader/threadable_loader.h"
#include "third_party/blink/renderer/core/loader/threadable_loader_client.h"
#include "third_party/blink/renderer/platform/loader/allowed_by_nosniff.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_loader_options.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_request.h"
#include "third_party/blink/renderer/platform/loader/fetch/url_loader/worker_main_script_loader.h"
#include "third_party/blink/renderer/platform/loader/fetch/url_loader/worker_main_script_loader_client.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/functional.h"
#include "third_party/blink/renderer/platform/wtf/ref_counted.h"
#include "third_party/blink/renderer/platform/wtf/text/string_builder.h"

namespace blink {

class ContentSecurityPolicy;
class ResourceRequest;
class ResourceResponse;
class ExecutionContext;
class TextResourceDecoder;
struct WorkerMainScriptLoadParameters;

class CORE_EXPORT WorkerClassicScriptLoader final
    : public GarbageCollected<WorkerClassicScriptLoader>,
      public ThreadableLoaderClient,
      public WorkerMainScriptLoaderClient {
 public:
  WorkerClassicScriptLoader();

  // For importScript().
  void LoadSynchronously(ExecutionContext&,
                         ResourceFetcher* fetch_client_settings_object_fetcher,
                         const KURL&,
                         mojom::blink::RequestContextType,
                         network::mojom::RequestDestination);

  // Note that callbacks could be invoked before
  // `LoadTopLevelScriptAsynchronously()` returns.
  //
  // `fetch_client_settings_object_fetcher` is different from
  // `ExecutionContext::Fetcher()` in off-the-main-thread fetch.
  // TODO(crbug.com/40123913): Remove `blob_url_loader_factory` now
  // that PlzDedicatedWorker has shipped.
  //
  // `worker_main_script_load_params` is valid for dedicated workers and shared
  // workers.
  void LoadTopLevelScriptAsynchronously(
      ExecutionContext&,
      ResourceFetcher* fetch_client_settings_object_fetcher,
      const KURL&,
      std::unique_ptr<WorkerMainScriptLoadParameters>
          worker_main_script_load_params,
      mojom::blink::RequestContextType,
      network::mojom::RequestDestination,
      network::mojom::RequestMode,
      network::mojom::CredentialsMode,
      base::OnceClosure response_callback,
      base::OnceClosure finished_callback,
      mojo::PendingRemote<network::mojom::blink::URLLoaderFactory>
          blob_url_loader_factory = {});

  // This will immediately invoke |finishedCallback| if
  // LoadTopLevelScriptAsynchronously() is in progress.
  void Cancel();

  String SourceText();
  const KURL& Url() const { return url_; }
  const KURL& ResponseURL() const;
  bool Failed() const { return failed_; }
  bool Canceled() const { return canceled_; }
  uint64_t Identifier() const { return identifier_; }

  std::unique_ptr<Vector<uint8_t>> ReleaseCachedMetadata() {
    return std::move(cached_metadata_);
  }

  ContentSecurityPolicy* GetContentSecurityPolicy() {
    return content_security_policy_.Get();
  }

  const String& GetReferrerPolicy() const { return referrer_policy_; }

  const Vector<String>* OriginTrialTokens() const {
    return origin_trial_tokens_.get();
  }

  // ThreadableLoaderClient
  void DidReceiveResponse(uint64_t /*identifier*/,
                          const ResourceResponse&) override;
  void DidReceiveData(base::span<const char> data) override;
  void DidReceiveCachedMetadata(mojo_base::BigBuffer) override;
  void DidFinishLoading(uint64_t identifier) override;
  void DidFail(uint64_t, const ResourceError&) override;
  void DidFailRedirectCheck(uint64_t) override;

  // WorkerMainScriptLoaderClient
  // These will be called for dedicated workers and shared workers.
  void DidReceiveDataWorkerMainScript(base::span<const char> span) override;
  void OnFinishedLoadingWorkerMainScript() override;
  void OnFailedLoadingWorkerMainScript() override;

  void Trace(Visitor*) const override;

 private:
  void NotifyError();
  void NotifyFinished();

  void ProcessContentSecurityPolicy(const ResourceResponse&);

  // Callbacks for loadAsynchronously().
  base::OnceClosure response_callback_;
  base::OnceClosure finished_callback_;

  Member<ThreadableLoader> threadable_loader_;

  // These are used for dedicated workers (when PlzDedicatedWorker is enabled)
  // and shared workers.
  Member<WorkerMainScriptLoader> worker_main_script_loader_;
  String response_encoding_;
  std::unique_ptr<TextResourceDecoder> decoder_;
  StringBuilder source_text_;
  KURL url_;
  KURL response_url_;

  // TODO(nhiroki): Consolidate these state flags for cleanup.
  bool failed_ = false;
  bool canceled_ = false;
  bool need_to_cancel_ = false;

  // true when LoadTopLevelScriptAsynchronously() is called,
  // false when LoadSynchronously() is called i.e. for importScripts().
  bool is_top_level_script_ = false;

  uint64_t identifier_ = 0;
  std::unique_ptr<Vector<uint8_t>> cached_metadata_;
  Member<ContentSecurityPolicy> content_security_policy_;
  std::unique_ptr<Vector<String>> origin_trial_tokens_;
  String referrer_policy_;

  Member<ResourceFetcher> fetch_client_settings_object_fetcher_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_WORKER_CLASSIC_SCRIPT_LOADER_H_
