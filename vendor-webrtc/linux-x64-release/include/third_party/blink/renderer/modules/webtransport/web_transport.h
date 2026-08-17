// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBTRANSPORT_WEB_TRANSPORT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBTRANSPORT_WEB_TRANSPORT_H_

#include <stdint.h>

#include "base/containers/span.h"
#include "base/time/time.h"
#include "base/types/pass_key.h"
#include "mojo/public/cpp/bindings/pending_receiver.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "mojo/public/cpp/system/data_pipe.h"
#include "services/network/public/mojom/web_transport.mojom-blink.h"
#include "third_party/blink/public/mojom/webtransport/web_transport_connector.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_property.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_web_transport_connection_stats.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_web_transport_datagram_stats.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/prefinalizer.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/scheduler/public/frame_scheduler.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {
class BidirectionalStream;
class DatagramDuplexStream;
class ExceptionState;
class IncomingStream;
class OutgoingStream;
class ReadableStream;
class ReadableByteStreamController;
class ScriptState;
class WebTransportCloseInfo;
class WebTransportOptions;
class WritableStream;

// https://wicg.github.io/web-transport/#web-transport
class MODULES_EXPORT WebTransport final
    : public ScriptWrappable,
      public ActiveScriptWrappable<WebTransport>,
      public ExecutionContextLifecycleObserver,
      public network::mojom::blink::WebTransportHandshakeClient,
      public network::mojom::blink::WebTransportClient {
  DEFINE_WRAPPERTYPEINFO();
  USING_PRE_FINALIZER(WebTransport, Dispose);

 public:
  using PassKey = base::PassKey<WebTransport>;
  static WebTransport* Create(ScriptState*,
                              const String& url,
                              WebTransportOptions*,
                              ExceptionState&);

  WebTransport(PassKey, ScriptState*, const String& url);
  ~WebTransport() override;

  // WebTransport IDL implementation.
  ScriptPromise<WritableStream> createUnidirectionalStream(ScriptState*,
                                                           ExceptionState&);
  ReadableStream* incomingUnidirectionalStreams();

  ScriptPromise<BidirectionalStream> createBidirectionalStream(ScriptState*,
                                                               ExceptionState&);
  ReadableStream* incomingBidirectionalStreams();

  DatagramDuplexStream* datagrams();
  WritableStream* datagramWritable();
  ReadableStream* datagramReadable();
  void close(WebTransportCloseInfo*);
  ScriptPromise<IDLUndefined> ready(ScriptState*);
  ScriptPromise<WebTransportCloseInfo> closed(ScriptState*);
  void setDatagramWritableQueueExpirationDuration(double ms);
  ScriptPromise<WebTransportConnectionStats> getStats(ScriptState*);

  // WebTransportHandshakeClient implementation
  void OnConnectionEstablished(
      mojo::PendingRemote<network::mojom::blink::WebTransport>,
      mojo::PendingReceiver<network::mojom::blink::WebTransportClient>,
      network::mojom::blink::HttpResponseHeadersPtr response_headers,
      network::mojom::blink::WebTransportStatsPtr initial_stats) override;
  void OnHandshakeFailed(network::mojom::blink::WebTransportErrorPtr) override;

  // WebTransportClient implementation
  void OnDatagramReceived(base::span<const uint8_t> data) override;
  void OnIncomingStreamClosed(uint32_t stream_id, bool fin_received) override;
  void OnOutgoingStreamClosed(uint32_t stream_id) override;
  void OnReceivedResetStream(uint32_t stream_id,
                             uint32_t stream_error_code) override;
  void OnReceivedStopSending(uint32_t stream_id,
                             uint32_t stream_error_code) override;
  void OnClosed(
      network::mojom::blink::WebTransportCloseInfoPtr close_info,
      network::mojom::blink::WebTransportStatsPtr final_stats) override;

  // Implementation of ExecutionContextLifecycleObserver
  void ContextDestroyed() final;

  // Implementation of WebTransport::HasPendingActivity()
  bool HasPendingActivity() const override;

  // Forwards a SendFin() message to the mojo interface.
  void SendFin(uint32_t stream_id);

  // Forwards a AbortStream() message to the mojo interface.
  void ResetStream(uint32_t stream_id, uint32_t code);

  // Forwards a StopSending() message to the mojo interface.
  void StopSending(uint32_t stream_id, uint32_t code);

  // Removes the reference to a stream.
  void ForgetIncomingStream(uint32_t stream_id);
  // Removes the reference to a stream.
  void ForgetOutgoingStream(uint32_t stream_id);

  // ScriptWrappable implementation
  void Trace(Visitor* visitor) const override;

 private:
  class DatagramUnderlyingSink;
  class DatagramUnderlyingSource;
  class StreamVendingUnderlyingSource;
  class ReceiveStreamVendor;
  class BidirectionalStreamVendor;

  WebTransport(ScriptState*, const String& url, ExecutionContext* context);

  void Init(const String& url_for_diagnostics,
            const WebTransportOptions&,
            ExceptionState&);

  void Dispose();
  void Cleanup(WebTransportCloseInfo*,
               v8::Local<v8::Value> error,
               bool abruptly);
  void OnConnectionError();
  void RejectPendingStreamResolvers(v8::Local<v8::Value> error);
  void HandlePendingGetStatsResolvers(v8::Local<v8::Value> error);
  void OnCreateSendStreamResponse(ScriptPromiseResolver<WritableStream>*,
                                  mojo::ScopedDataPipeProducerHandle,
                                  bool succeeded,
                                  uint32_t stream_id);
  void OnCreateBidirectionalStreamResponse(
      ScriptPromiseResolver<BidirectionalStream>*,
      mojo::ScopedDataPipeProducerHandle,
      mojo::ScopedDataPipeConsumerHandle,
      bool succeeded,
      uint32_t stream_id);
  void OnGetStatsResponse(network::mojom::blink::WebTransportStatsPtr);

  bool DoesSubresourceFilterBlockConnection(const KURL& url);

  WebTransportConnectionStats* ConvertStatsFromMojom(
      network::mojom::blink::WebTransportStatsPtr in);

  Member<DatagramDuplexStream> datagrams_;

  Member<ReadableStream> received_datagrams_;
  Member<ReadableByteStreamController> received_datagrams_controller_;
  Member<DatagramUnderlyingSource> datagram_underlying_source_;

  // This corresponds to the [[SentDatagrams]] internal slot in the standard.
  Member<WritableStream> outgoing_datagrams_;
  Member<DatagramUnderlyingSink> datagram_underlying_sink_;

  base::TimeDelta outgoing_datagram_expiration_duration_;

  const Member<ScriptState> script_state_;

  const KURL url_;

  // Map from stream_id to IncomingStream.
  // Intentionally keeps streams reachable by GC as long as they are open.
  // This doesn't support stream ids of 0xfffffffe or larger.
  // TODO(ricea): Find out if such large stream ids are possible.
  HeapHashMap<uint32_t,
              Member<IncomingStream>,
              IntWithZeroKeyHashTraits<uint32_t>>
      incoming_stream_map_;

  // Map from stream_id to OutgoingStream.
  // Intentionally keeps streams reachable by GC as long as they are open.
  // This doesn't support stream ids of 0xfffffffe or larger.
  // TODO(ricea): Find out if such large stream ids are possible.
  HeapHashMap<uint32_t,
              Member<OutgoingStream>,
              IntWithZeroKeyHashTraits<uint32_t>>
      outgoing_stream_map_;

  // A map from stream id to whether the fin signal was received. When
  // OnIncomingStreamClosed is called with a stream ID which doesn't have its
  // corresponding incoming stream, the event is recorded here.
  HashMap<uint32_t, bool, IntWithZeroKeyHashTraits<uint32_t>>
      closed_potentially_pending_streams_;

  HeapMojoRemote<mojom::blink::WebTransportConnector> connector_;
  HeapMojoRemote<network::mojom::blink::WebTransport> transport_remote_;
  HeapMojoReceiver<network::mojom::blink::WebTransportHandshakeClient,
                   WebTransport>
      handshake_client_receiver_;
  HeapMojoReceiver<network::mojom::blink::WebTransportClient, WebTransport>
      client_receiver_;
  using ReadyProperty = ScriptPromiseProperty<IDLUndefined, IDLAny>;
  Member<ReadyProperty> ready_;
  Member<ScriptPromiseProperty<WebTransportCloseInfo, IDLAny>> closed_;
  // True if [[State]] is "connecting".
  bool connection_pending_ = true;

  // The most recent result for getStats() call, used for cases when the
  // stats are requested after the transport is closed.
  Member<WebTransportConnectionStats> latest_stats_;
  // Tracks resolvers for in-progress getStats() calls.
  HeapVector<Member<ScriptPromiseResolver<WebTransportConnectionStats>>>
      pending_get_stats_resolvers_;

  // Tracks resolvers for in-progress createSendStream() and
  // createBidirectionalStream() operations so they can be rejected.
  HeapHashSet<Member<ScriptPromiseResolverBase>> create_stream_resolvers_;

  // The [[ReceivedStreams]] slot.
  // https://w3c.github.io/webtransport/#webtransport-receivedstreams
  Member<ReadableStream> received_streams_;
  Member<StreamVendingUnderlyingSource> received_streams_underlying_source_;

  Member<ReadableStream> received_bidirectional_streams_;
  Member<StreamVendingUnderlyingSource>
      received_bidirectional_streams_underlying_source_;

  const uint64_t inspector_transport_id_;

  FrameScheduler::SchedulingAffectingFeatureHandle
      feature_handle_for_scheduler_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBTRANSPORT_WEB_TRANSPORT_H_
