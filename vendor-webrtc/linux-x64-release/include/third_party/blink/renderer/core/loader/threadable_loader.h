/*
 * Copyright (C) 2009, 2012 Google Inc. All rights reserved.
 * Copyright (C) 2013, Intel Corporation
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_THREADABLE_LOADER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_THREADABLE_LOADER_H_

#include "base/task/single_thread_task_runner.h"
#include "base/time/time.h"
#include "services/network/public/mojom/fetch_api.mojom-blink.h"
#include "third_party/blink/public/mojom/fetch/fetch_api_request.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/loader/fetch/raw_resource.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_error.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_loader_options.h"
#include "third_party/blink/renderer/platform/network/http_header_map.h"
#include "third_party/blink/renderer/platform/timer.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class ExecutionContext;
class ResourceRequest;
class ThreadableLoaderClient;

// Useful for doing loader operations from any thread (not threadsafe, just able
// to run on threads other than the main thread).
//
// Can perform requests either synchronously or asynchronously. Requests are
// asynchronous by default, and this behavior can be controlled by passing
// a ResourceLoaderOptions with synchronous_policy == kRequestSynchronously to
// the constructor.
// In either case, Start() must be called to actaully begin the request.
class CORE_EXPORT ThreadableLoader final
    : public GarbageCollected<ThreadableLoader>,
      private RawResourceClient {
 public:
  // ThreadableLoaderClient methods are never called before Start() call.
  //
  // Loading is separated into the constructor and the Start() method in order
  // to:
  // - reduce work done in a constructor
  // - not to ask the users to handle failures in the constructor and other
  //   async failures separately
  //
  // Loading completes when one of the following methods are called:
  // - DidFinishLoading()
  // - DidFail()
  // - DidFailRedirectCheck()
  // After any of these methods is called, the loader won't call any of the
  // ThreadableLoaderClient methods.
  //
  // When ThreadableLoader::Cancel() is called,
  // ThreadableLoaderClient::DidFail() is called with a ResourceError
  // with IsCancellation() returning true, if any of DidFinishLoading()
  // or DidFail.*() methods have not been called yet. (DidFail() may be
  // called with a ResourceError with IsCancellation() returning true
  // also for cancellation happened inside the loader.)
  //
  // ThreadableLoaderClient methods may call Cancel().
  //
  // The specified ResourceFetcher if non-null, or otherwise
  // ExecutionContext::Fetcher() is used.
  ThreadableLoader(ExecutionContext&,
                   ThreadableLoaderClient*,
                   const ResourceLoaderOptions&,
                   ResourceFetcher* = nullptr);
  ThreadableLoader(const ThreadableLoader&) = delete;
  ThreadableLoader& operator=(const ThreadableLoader&) = delete;
  ~ThreadableLoader() override;

  // Must be called to actually begin the request.
  void Start(ResourceRequest);

  // A ThreadableLoader may have a timeout specified. It is possible, in some
  // cases, for the timeout to be overridden after the request is sent (for
  // example, XMLHttpRequests may override their timeout setting after sending).
  //
  // If the request has already started, the new timeout will be relative to the
  // time the request started.
  //
  // Passing a timeout of zero means there should be no timeout.
  void SetTimeout(const base::TimeDelta& timeout);

  void Cancel();

  // Detach the loader from the request. This function is for "keepalive"
  // requests. No notification will be sent to the client, but the request
  // will be processed.
  void Detach();

  void SetDefersLoading(bool);

  // Return the task runner this class uses for processing network data.
  scoped_refptr<base::SingleThreadTaskRunner> GetTaskRunner();

  void Trace(Visitor* visitor) const override;

 private:
  void Clear();

  void DidTimeout(TimerBase*);

  void DispatchDidFail(const ResourceError&);

  // ResourceClient implementation:
  void NotifyFinished(Resource*) override;
  String DebugName() const override { return "ThreadableLoader"; }

  // RawResourceClient implementation:
  void DataSent(Resource*,
                uint64_t bytes_sent,
                uint64_t total_bytes_to_be_sent) override;
  void ResponseReceived(Resource*, const ResourceResponse&) override;
  void ResponseBodyReceived(Resource*, BytesConsumer& body) override;
  void CachedMetadataReceived(Resource*, mojo_base::BigBuffer) override;
  void DataReceived(Resource*, base::span<const char> data) override;
  bool RedirectReceived(Resource*,
                        const ResourceRequest&,
                        const ResourceResponse&) override;
  void RedirectBlocked() override;
  void DataDownloaded(Resource*, uint64_t) override;
  void DidDownloadToBlob(Resource*, scoped_refptr<BlobDataHandle>) override;

  const ResourceLoaderOptions resource_loader_options_;

  Member<ThreadableLoaderClient> client_;
  Member<ExecutionContext> execution_context_;
  Member<ResourceFetcher> resource_fetcher_;

  // Saved so that we can use the original mode in ResponseReceived() where
  // |resource| might be a reused one (e.g. preloaded resource) which can have a
  // different mode.
  network::mojom::RequestMode request_mode_;

  // Set via SetTimeout() by a user before Start().
  base::TimeDelta timeout_;
  // Used to detect |timeout_| is over.
  HeapTaskRunnerTimer<ThreadableLoader> timeout_timer_;

  // Time an asynchronous fetch request is started
  base::TimeTicks request_started_;

  RawResourceClientStateChecker checker_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_THREADABLE_LOADER_H_
