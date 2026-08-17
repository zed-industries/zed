// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_RESPONSE_BODY_LOADER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_RESPONSE_BODY_LOADER_H_

#include "base/containers/span.h"
#include "base/memory/scoped_refptr.h"
#include "mojo/public/cpp/system/data_pipe.h"
#include "third_party/blink/public/mojom/navigation/renderer_eviction_reason.mojom-blink-forward.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/loader/fetch/bytes_consumer.h"
#include "third_party/blink/renderer/platform/loader/fetch/loader_freeze_mode.h"
#include "third_party/blink/renderer/platform/loader/fetch/response_body_loader_client.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace base {
class SingleThreadTaskRunner;
}  // namespace base

namespace blink {

class BackForwardCacheLoaderHelper;
class ResponseBodyLoader;

// See ResponseBodyLoader for details. This is a virtual interface to expose
// only Drain functions.
class PLATFORM_EXPORT ResponseBodyLoaderDrainableInterface
    : public GarbageCollected<ResponseBodyLoaderDrainableInterface> {
 public:
  virtual ~ResponseBodyLoaderDrainableInterface() = default;

  // Drains the response body and returns it. This function must not be called
  // when the load has already been started or aborted, or the body has already
  // been drained. This function may return an invalid handle when it is
  // unable to convert the body to a data pipe, even when the body itself is
  // valid. In that case, this function is no-op. If this function returns a
  // valid handle, the caller is responsible for reading the body and providing
  // the information to the client this function provides.
  // Note that the notification from the client is *synchronously* propergated
  // to the original client ResponseBodyLoader owns,  e.g., when the caller
  // calls ResponseBodyLoaderClient::DidCancelLoadingBody, it synchronously
  // cancels the resource loading (if |this| is associated with
  // blink::ResourceLoader). A user of this function should ensure that calling
  // the client's method doesn't lead to a reentrant problem.
  // Note that the drained datapipe is not subject to the freezing effect.
  virtual mojo::ScopedDataPipeConsumerHandle DrainAsDataPipe(
      ResponseBodyLoaderClient** client) = 0;

  // Drains the response body and returns it. This function must not be called
  // when the load has already been started or aborted, or the body has already
  // been drained. Unlike DrainAsDataPipe, this function always succeeds.
  // This ResponseBodyLoader will still monitor the loading signals, and report
  // them back to the associated client asynchronously.
  // Note that the drained BytesConsumer is subject to the freezing effect, and
  // the loading is cancelled when freezing is for back-forward cache.
  virtual BytesConsumer& DrainAsBytesConsumer() = 0;

  virtual void Trace(Visitor*) const {}
};

// ResponseBodyLoader reads the response body and reports the contents to the
// associated client. There are two ways:
//  - By calling Start(), ResponseBodyLoader reads the response body. and
//    reports the contents to the client. Abort() aborts reading.
//  - By calling DrainAsDataPipe, a user can "drain" the contents from
//    ResponseBodyLoader. The caller is responsible for reading the body and
//    providing the information to the client this function provides.
//  - By calling DrainBytesConsumer, a user can "drain" the contents from
//    ResponseBodyLoader.
// A ResponseBodyLoader is bound to the thread on which it is created.
class PLATFORM_EXPORT ResponseBodyLoader final
    : public ResponseBodyLoaderDrainableInterface,
      private ResponseBodyLoaderClient,
      private BytesConsumer::Client {
 public:
  ResponseBodyLoader(
      BytesConsumer&,
      ResponseBodyLoaderClient&,
      scoped_refptr<base::SingleThreadTaskRunner>,
      BackForwardCacheLoaderHelper* back_forward_cache_loader_helper);

  // ResponseBodyLoaderDrainableInterface implementation.
  mojo::ScopedDataPipeConsumerHandle DrainAsDataPipe(
      ResponseBodyLoaderClient**) override;
  BytesConsumer& DrainAsBytesConsumer() override;

  // Starts loading.
  void Start();

  // Aborts loading. This is expected to be called from the client's side, and
  // does not report the failure to the client. This doesn't affect a
  // drained data pipe. This function cannot be called when suspended.
  void Abort();

  // Suspends loading.
  void Suspend(LoaderFreezeMode);

  // Resumes loading.
  void Resume();

  bool IsAborted() const { return aborted_; }
  bool IsSuspended() const {
    return suspended_state_ != LoaderFreezeMode::kNone;
  }
  bool IsSuspendedForBackForwardCache() const {
    return suspended_state_ == LoaderFreezeMode::kBufferIncoming;
  }
  bool IsDrained() const {
    return drained_as_datapipe_ || drained_as_bytes_consumer_;
  }

  // Evicts the back-forward cache entry if the response body has already been
  // passed and drained as bytes consumer.
  void EvictFromBackForwardCacheIfDrainedAsBytesConsumer();

  void Trace(Visitor*) const override;

 private:
  class Buffer;
  class DelegatingBytesConsumer;

  // ResponseBodyLoaderClient implementation.
  void DidReceiveData(base::span<const char> data) override;
  void DidReceiveDecodedData(
      const String& data,
      std::unique_ptr<ParkableStringImpl::SecureDigest> digest) override;
  void DidFinishLoadingBody() override;
  void DidFailLoadingBody() override;
  void DidCancelLoadingBody() override;

  void EvictFromBackForwardCache(mojom::blink::RendererEvictionReason);
  void DidBufferLoadWhileInBackForwardCache(size_t num_bytes);

  // BytesConsumer::Client implementation.
  void OnStateChange() override;
  String DebugName() const override { return "ResponseBodyLoader"; }

  const scoped_refptr<base::SingleThreadTaskRunner> task_runner_;
  Member<Buffer> body_buffer_;
  Member<BytesConsumer> bytes_consumer_;
  Member<DelegatingBytesConsumer> delegating_bytes_consumer_;
  const Member<ResponseBodyLoaderClient> client_;
  WeakMember<BackForwardCacheLoaderHelper> back_forward_cache_loader_helper_;
  LoaderFreezeMode suspended_state_ = LoaderFreezeMode::kNone;
  bool started_ = false;
  bool aborted_ = false;
  bool drained_as_datapipe_ = false;
  bool drained_as_bytes_consumer_ = false;
  bool finish_signal_is_pending_ = false;
  bool fail_signal_is_pending_ = false;
  bool cancel_signal_is_pending_ = false;
  bool in_two_phase_read_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_RESPONSE_BODY_LOADER_H_
