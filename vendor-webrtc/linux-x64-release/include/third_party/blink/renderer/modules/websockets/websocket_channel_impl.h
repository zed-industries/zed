/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBSOCKETS_WEBSOCKET_CHANNEL_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBSOCKETS_WEBSOCKET_CHANNEL_IMPL_H_

#include <stdint.h>

#include <memory>
#include <utility>

#include "base/containers/span.h"
#include "base/gtest_prod_util.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/scoped_refptr.h"
#include "base/task/single_thread_task_runner.h"
#include "mojo/public/cpp/bindings/pending_receiver.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "mojo/public/cpp/system/simple_watcher.h"
#include "services/network/public/mojom/websocket.mojom-blink.h"
#include "third_party/blink/public/mojom/websockets/websocket_connector.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/fileapi/blob.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/websockets/websocket_channel.h"
#include "third_party/blink/renderer/modules/websockets/websocket_message_chunk_accumulator.h"
#include "third_party/blink/renderer/platform/bindings/source_location.h"
#include "third_party/blink/renderer/platform/bindings/v8_external_memory_accounter.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/prefinalizer.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"
#include "third_party/blink/renderer/platform/scheduler/public/frame_scheduler.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/deque.h"
#include "third_party/blink/renderer/platform/wtf/gc_plugin.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "third_party/blink/renderer/platform/wtf/wtf_size_t.h"

namespace v8 {
class Isolate;
}  // namespace v8

namespace blink {

class BaseFetchContext;
enum class FileErrorCode;
class WebSocketChannelClient;
class WebSocketHandshakeThrottle;

// This is an implementation of WebSocketChannel. This is created on the main
// thread for Document, or on the worker thread for WorkerGlobalScope. All
// functions must be called on the execution context's thread.
class MODULES_EXPORT WebSocketChannelImpl final
    : public WebSocketChannel,
      public network::mojom::blink::WebSocketHandshakeClient,
      public network::mojom::blink::WebSocketClient {
  USING_PRE_FINALIZER(WebSocketChannelImpl, Dispose);

 public:
  // Public for use in tests.
  static constexpr size_t kMaxWebSocketsPerRenderProcess = 255u;

  // You can specify the source file and the line number information
  // explicitly by passing the last parameter.
  // In the usual case, they are set automatically and you don't have to
  // pass it.
  static WebSocketChannelImpl* Create(ExecutionContext* context,
                                      WebSocketChannelClient* client,
                                      std::unique_ptr<SourceLocation> location);
  static WebSocketChannelImpl* CreateForTesting(
      ExecutionContext*,
      WebSocketChannelClient*,
      std::unique_ptr<SourceLocation>,
      std::unique_ptr<WebSocketHandshakeThrottle>);

  WebSocketChannelImpl(ExecutionContext*,
                       WebSocketChannelClient*,
                       std::unique_ptr<SourceLocation>);
  ~WebSocketChannelImpl() override;

  // WebSocketChannel functions.
  bool Connect(const KURL&, const String& protocol) override;
  SendResult Send(const std::string& message,
                  base::OnceClosure completion_callback) override;
  SendResult Send(const DOMArrayBuffer&,
                  size_t byte_offset,
                  size_t byte_length,
                  base::OnceClosure completion_callback) override;
  void Send(scoped_refptr<BlobDataHandle>) override;
  // Start closing handshake. Use the CloseEventCodeNotSpecified for the code
  // argument to omit payload.
  void Close(int code, const String& reason) override;
  void Fail(const String& reason,
            mojom::ConsoleMessageLevel,
            std::unique_ptr<SourceLocation>) override;
  void Disconnect() override;
  void CancelHandshake() override;
  void ApplyBackpressure() override;
  void RemoveBackpressure() override;

  // network::mojom::blink::WebSocketHandshakeClient methods:
  void OnOpeningHandshakeStarted(
      network::mojom::blink::WebSocketHandshakeRequestPtr) override;
  void OnFailure(const WTF::String& message,
                 int net_error,
                 int response_code) override;
  void OnConnectionEstablished(
      mojo::PendingRemote<network::mojom::blink::WebSocket> websocket,
      mojo::PendingReceiver<network::mojom::blink::WebSocketClient>
          client_receiver,
      network::mojom::blink::WebSocketHandshakeResponsePtr,
      mojo::ScopedDataPipeConsumerHandle readable,
      mojo::ScopedDataPipeProducerHandle writable) override;

  // network::mojom::blink::WebSocketClient methods:
  void OnDataFrame(bool fin,
                   network::mojom::blink::WebSocketMessageType,
                   uint64_t data_length) override;
  void OnDropChannel(bool was_clean,
                     uint16_t code,
                     const String& reason) override;
  void OnClosingHandshake() override;

  void Trace(Visitor*) const override;

 private:
  struct DataFrame final {
    DataFrame(bool fin,
              network::mojom::blink::WebSocketMessageType type,
              size_t data_length)
        : fin(fin), type(type), data_length(data_length) {}

    bool fin;
    network::mojom::blink::WebSocketMessageType type;
    size_t data_length;
  };

  // Used by BlobLoader and Message, so defined here so that it can be shared.
  class MessageDataDeleter {
   public:
    // This constructor exists to permit default construction of the MessageData
    // type, but the deleter cannot be called when it was used.
    MessageDataDeleter() : isolate_(nullptr), size_(0) {}

    MessageDataDeleter(v8::Isolate* isolate, size_t size);

    MessageDataDeleter(MessageDataDeleter&&) = default;
    MessageDataDeleter& operator=(MessageDataDeleter&&) = default;

    void operator()(char* p) const;

   private:
    raw_ptr<v8::Isolate> isolate_;
    size_t size_;
    NO_UNIQUE_ADDRESS mutable V8ExternalMemoryAccounterBase
        external_memory_accounter_;
  };

  using MessageData = std::unique_ptr<char[], MessageDataDeleter>;

  static MessageData CreateMessageData(v8::Isolate*, size_t);

  friend class WebSocketChannelImplHandshakeThrottleTest;
  FRIEND_TEST_ALL_PREFIXES(WebSocketChannelImplHandshakeThrottleTest,
                           ThrottleSucceedsFirst);
  FRIEND_TEST_ALL_PREFIXES(WebSocketChannelImplHandshakeThrottleTest,
                           HandshakeSucceedsFirst);
  FRIEND_TEST_ALL_PREFIXES(WebSocketChannelImplHandshakeThrottleTest,
                           ThrottleReportsErrorBeforeConnect);
  FRIEND_TEST_ALL_PREFIXES(WebSocketChannelImplHandshakeThrottleTest,
                           ThrottleReportsErrorAfterConnect);

  class BlobLoader;
  class Message;
  struct ConnectInfo;

  enum MessageType {
    kMessageTypeText,
    kMessageTypeBlob,
    kMessageTypeArrayBuffer,
    kMessageTypeClose,
  };

  struct ReceivedMessage {
    bool is_message_text;
    Vector<char> data;
  };

  class Message final {
    DISALLOW_NEW();

   public:
    using DidCallSendMessage =
        base::StrongAlias<class DidCallSendMessageTag, bool>;

    // Initializes message as a string
    Message(v8::Isolate*,
            const std::string&,
            base::OnceClosure completion_callback,
            DidCallSendMessage did_call_send_message);

    // Initializes message as a blob
    explicit Message(scoped_refptr<BlobDataHandle>);

    // Initializes message from the contents of a blob
    Message(MessageData, size_t);

    // Initializes message as a ArrayBuffer
    Message(v8::Isolate*,
            base::span<const char> message,
            base::OnceClosure completion_callback,
            DidCallSendMessage did_call_send_message);

    // Initializes a Blank message
    Message(MessageType type,
            base::span<const char> message,
            base::OnceClosure completion_callback);

    // Close message
    Message(uint16_t code, const String& reason);

    Message(const Message&) = delete;
    Message& operator=(const Message&) = delete;

    Message(Message&&);
    Message& operator=(Message&&);

    MessageType Type() const;
    scoped_refptr<BlobDataHandle> GetBlobDataHandle();
    DidCallSendMessage GetDidCallSendMessage() const;
    uint16_t Code() const;
    String Reason() const;
    base::OnceClosure CompletionCallback();

    // Returns a mutable |pending_payload_|. Since calling code always mutates
    // the value, |pending_payload_| only has a mutable getter.
    base::span<const char>& MutablePendingPayload();

    void SetDidCallSendMessage(DidCallSendMessage did_call_send_message);

   private:
    MessageData message_data_;
    MessageType type_;

    scoped_refptr<BlobDataHandle> blob_data_handle_;
    base::span<const char> pending_payload_;
    DidCallSendMessage did_call_send_message_ = DidCallSendMessage(false);
    uint16_t code_ = 0;
    String reason_;
    base::OnceClosure completion_callback_;
  };

  // A handle to a global count of the number of WebSockets that have been
  // created. Can be used to limit the total number of WebSockets that have been
  // created in this render process.
  class ConnectionCountTrackerHandle {
    DISALLOW_NEW();

   public:
    enum class CountStatus {
      kOkayToConnect,
      kShouldNotConnect,
    };

    ConnectionCountTrackerHandle() = default;
    ~ConnectionCountTrackerHandle() = default;

    ConnectionCountTrackerHandle(const ConnectionCountTrackerHandle&) = delete;
    ConnectionCountTrackerHandle& operator=(
        const ConnectionCountTrackerHandle&) = delete;

    // Increments the count and returns SHOULD_NOT_CONNECT if it exceeds
    // kMaxWebSocketsPerRenderProcess. Should only be called once.
    CountStatus IncrementAndCheckStatus();

    // Decrements the count. Should be called at least once. If there is no
    // matching call to IncrementAndCheckStatus() it does nothing, so it is safe
    // to call multiple times.
    void Decrement();

   private:
    bool incremented_ = false;
  };

  // The state is defined to see the conceptual state more clearly than checking
  // various members (for DCHECKs for example). This is only used internally.
  enum class State {
    // The channel is running an opening handshake. This is the initial state.
    // It becomes |kOpen| when the connection is established. It becomes
    // |kDisconnected| when detecting an error.
    kConnecting,
    // The channel is ready to send / receive messages. It becomes
    // |kDisconnected| when the connection is closed or when an error happens.
    kOpen,
    // The channel is not ready for communication. The channel stays in this
    // state forever.
    kDisconnected,
  };
  State GetState() const;

  bool MaybeSendSynchronously(network::mojom::blink::WebSocketMessageType,
                              base::span<const char>* data);
  void ProcessSendQueue();
  bool SendMessageData(base::span<const char>* data);
  void FailAsError(const String& reason) {
    Fail(reason, mojom::ConsoleMessageLevel::kError,
         location_at_construction_->Clone());
  }
  void AbortAsyncOperations();
  void HandleDidClose(bool was_clean, uint16_t code, const String& reason);

  // Completion callback. It is called with the results of throttling.
  void OnCompletion(const std::optional<WebString>& error);

  // Methods for BlobLoader.
  void DidFinishLoadingBlob(MessageData, size_t);
  void BlobTooLarge();
  void DidFailLoadingBlob(FileErrorCode);

  void TearDownFailedConnection();
  bool ShouldDisallowConnection(const KURL&);

  BaseFetchContext* GetBaseFetchContext() const;

  // Called when |readable_| becomes readable.
  void OnReadable(MojoResult result, const mojo::HandleSignalsState& state);
  void ConsumePendingDataFrames();
  void ConsumeDataFrame(bool fin,
                        network::mojom::blink::WebSocketMessageType type,
                        const char* data,
                        size_t data_size);
  // Called when |writable_| becomes writable.
  void OnWritable(MojoResult result, const mojo::HandleSignalsState& state);
  MojoResult ProduceData(base::span<const char>* data,
                         uint64_t* consumed_buffered_amount);
  String GetTextMessage(const Vector<base::span<const char>>& chunks,
                        wtf_size_t size);
  void OnConnectionError(const base::Location& set_from,
                         uint32_t custom_reason,
                         const std::string& description);
  void Dispose();

  const Member<WebSocketChannelClient> client_;
  KURL url_;
  uint64_t identifier_;
  Member<BlobLoader> blob_loader_;
  WTF::Deque<Message> messages_;
  Member<WebSocketMessageChunkAccumulator> message_chunks_;
  const Member<ExecutionContext> execution_context_;

  bool backpressure_ = false;
  bool receiving_message_type_is_text_ = false;
  bool received_text_is_all_ascii_ = true;
  bool throttle_passed_ = false;
  bool has_initiated_opening_handshake_ = false;
  size_t sent_size_of_top_message_ = 0;
  FrameScheduler::SchedulingAffectingFeatureHandle
      feature_handle_for_scheduler_;
  WTF::String failure_message_;

  const std::unique_ptr<const SourceLocation> location_at_construction_;
  network::mojom::blink::WebSocketHandshakeRequestPtr handshake_request_;
  std::unique_ptr<WebSocketHandshakeThrottle> handshake_throttle_;
  // This field is only initialised if the object is still waiting for a
  // throttle response when DidConnect is called.
  std::unique_ptr<ConnectInfo> connect_info_;

  HeapMojoRemote<network::mojom::blink::WebSocket> websocket_;
  HeapMojoReceiver<network::mojom::blink::WebSocketHandshakeClient,
                   WebSocketChannelImpl>
      handshake_client_receiver_;
  HeapMojoReceiver<network::mojom::blink::WebSocketClient, WebSocketChannelImpl>
      client_receiver_;

  mojo::ScopedDataPipeConsumerHandle readable_;
  mojo::SimpleWatcher readable_watcher_;
  WTF::Deque<DataFrame> pending_data_frames_;

  mojo::ScopedDataPipeProducerHandle writable_;
  mojo::SimpleWatcher writable_watcher_;
  bool wait_for_writable_ = false;
  ConnectionCountTrackerHandle connection_count_tracker_handle_;

  const scoped_refptr<base::SingleThreadTaskRunner> file_reading_task_runner_;
};

MODULES_EXPORT std::ostream& operator<<(std::ostream&,
                                        const WebSocketChannelImpl*);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBSOCKETS_WEBSOCKET_CHANNEL_IMPL_H_
