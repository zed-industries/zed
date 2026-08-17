/*
 * Copyright (C) 2009, 2011 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_URL_LOADER_URL_LOADER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_URL_LOADER_URL_LOADER_H_

#include <stdint.h>

#include <memory>
#include <optional>

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/public/mojom/loader/keep_alive_handle.mojom-blink.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_url_request.h"
#include "third_party/blink/renderer/platform/loader/fetch/loader_freeze_mode.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace base {
class SingleThreadTaskRunner;
class TimeDelta;
class WaitableEvent;
}  // namespace base

namespace network {
class SharedURLLoaderFactory;
struct ResourceRequest;
}  // namespace network

namespace blink {

class BackForwardCacheLoaderHelper;
class BackgroundResponseProcessorFactory;
class BlobDataHandle;
class CodeCacheHost;
class ResourceLoadInfoNotifierWrapper;
class ResourceRequestSender;
class SecurityOrigin;
class URLLoaderClient;
class URLLoaderThrottle;
struct WebURLError;
class WebURLResponse;

class BLINK_PLATFORM_EXPORT URLLoader {
 public:
  // When non-null |keep_alive_handle| is specified, this loader prolongs
  // this render process's lifetime.
  URLLoader(
      const Vector<String>& cors_exempt_header_list,
      base::WaitableEvent* terminate_sync_load_event,
      scoped_refptr<base::SingleThreadTaskRunner> freezable_task_runner,
      scoped_refptr<base::SingleThreadTaskRunner> unfreezable_task_runner,
      scoped_refptr<network::SharedURLLoaderFactory> url_loader_factory,
      mojo::PendingRemote<mojom::blink::KeepAliveHandle> keep_alive_handle,
      BackForwardCacheLoaderHelper* back_forward_cache_loader_helper,
      Vector<std::unique_ptr<URLLoaderThrottle>> throttles);
  URLLoader(const URLLoader&) = delete;
  URLLoader& operator=(const URLLoader&) = delete;
  URLLoader();

  // The URLLoader may be deleted in a call to its client.
  virtual ~URLLoader();

  // Load the request synchronously, returning results directly to the
  // caller upon completion.  There is no mechanism to interrupt a
  // synchronous load!!
  // If `download_to_blob` is true, the response will instead be
  // redirected to a blob, which is passed out in `downloaded_blob`.
  virtual void LoadSynchronously(
      std::unique_ptr<network::ResourceRequest> request,
      scoped_refptr<const SecurityOrigin> top_frame_origin,
      bool download_to_blob,
      bool no_mime_sniffing,
      base::TimeDelta timeout_interval,
      URLLoaderClient* client,
      WebURLResponse& response,
      std::optional<WebURLError>& error,
      scoped_refptr<SharedBuffer>& data,
      int64_t& encoded_data_length,
      uint64_t& encoded_body_length,
      scoped_refptr<BlobDataHandle>& downloaded_blob,
      std::unique_ptr<ResourceLoadInfoNotifierWrapper>
          resource_load_info_notifier_wrapper);

  // Load the request asynchronously, sending notifications to the given
  // client.  The client will receive no further notifications if the
  // loader is disposed before it completes its work.
  virtual void LoadAsynchronously(
      std::unique_ptr<network::ResourceRequest> request,
      scoped_refptr<const SecurityOrigin> top_frame_origin,
      bool no_mime_sniffing,
      std::unique_ptr<ResourceLoadInfoNotifierWrapper>
          resource_load_info_notifier_wrapper,
      CodeCacheHost* code_cache_host,
      URLLoaderClient* client);

  // Freezes the loader. See blink/renderer/platform/loader/README.md for the
  // general concept of "freezing" in the loading module. See
  // blink/public/platform/web_loader_freezing_mode.h for `mode`.
  virtual void Freeze(LoaderFreezeMode mode);

  // Notifies the loader that the priority of a WebURLRequest has changed from
  // its previous value. For example, a preload request starts with low
  // priority, but may increase when the resource is needed for rendering.
  virtual void DidChangePriority(WebURLRequest::Priority new_priority,
                                 int intra_priority_value);

  // Returns the task runner for this request.
  virtual scoped_refptr<base::SingleThreadTaskRunner>
  GetTaskRunnerForBodyLoader();

  // For BackgroundResourceFetch feature.
  // Returns true if the loader can handle the response on a background thread.
  virtual bool CanHandleResponseOnBackground() { return false; }
  // Set a BackgroundResponseProcessorFactory to process the response on a
  // background thread.
  virtual void SetBackgroundResponseProcessorFactory(
      std::unique_ptr<BackgroundResponseProcessorFactory>
          background_response_processor_factory);

  void SetResourceRequestSenderForTesting(
      std::unique_ptr<ResourceRequestSender> resource_request_sender);

 private:
  class Context;
  class RequestPeerImpl;

  void Cancel();

  scoped_refptr<Context> context_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_URL_LOADER_URL_LOADER_H_
