// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PRESENTATION_PRESENTATION_CONNECTION_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PRESENTATION_PRESENTATION_CONNECTION_H_

#include "base/task/single_thread_task_runner.h"
#include "mojo/public/cpp/bindings/pending_receiver.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "third_party/blink/public/mojom/presentation/presentation.mojom-blink.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_binary_type.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_state_observer.h"
#include "third_party/blink/renderer/core/fileapi/blob.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace WTF {
class AtomicString;
}  // namespace WTF

namespace blink {

class DOMArrayBuffer;
class DOMArrayBufferView;
enum class FileErrorCode;
class PresentationController;
class PresentationReceiver;
class PresentationRequest;
class V8PresentationConnectionState;
class WebString;

class MODULES_EXPORT PresentationConnection
    : public EventTarget,
      public ExecutionContextLifecycleStateObserver,
      public mojom::blink::PresentationConnection {
  DEFINE_WRAPPERTYPEINFO();

 public:
  ~PresentationConnection() override;

  // EventTarget implementation.
  const AtomicString& InterfaceName() const override;
  ExecutionContext* GetExecutionContext() const override;

  void Trace(Visitor*) const override;

  const String& id() const { return id_; }
  const String& url() const { return url_; }
  V8PresentationConnectionState state() const;

  void send(const String& message, ExceptionState&);
  void send(DOMArrayBuffer*, ExceptionState&);
  void send(NotShared<DOMArrayBufferView>, ExceptionState&);
  void send(Blob*, ExceptionState&);

  // Closes the connection to the ongoing presentation.
  void close();

  // Terminates the ongoing presentation that this PresentationConnection is
  // connected to.
  void terminate();

  V8BinaryType binaryType() const;
  void setBinaryType(const V8BinaryType&);

  DEFINE_ATTRIBUTE_EVENT_LISTENER(message, kMessage)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(connect, kConnect)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(close, kClose)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(terminate, kTerminate)

  // Returns true if this connection's id equals to |id| and its url equals to
  // |url|.
  bool Matches(const String& id, const KURL&) const;

  // Notifies the connection about its state change to 'closed'.
  void DidClose(mojom::blink::PresentationConnectionCloseReason,
                const String& message);

  // mojom::blink::PresentationConnection implementation.
  void OnMessage(mojom::blink::PresentationConnectionMessagePtr) override;
  void DidChangeState(mojom::blink::PresentationConnectionState) override;
  void DidClose(mojom::blink::PresentationConnectionCloseReason) override;

  mojom::blink::PresentationConnectionState GetState() const;

 protected:
  PresentationConnection(LocalDOMWindow&, const String& id, const KURL&);

  // EventTarget implementation.
  void AddedEventListener(const AtomicString& event_type,
                          RegisteredEventListener&) override;

  // ExecutionContextLifecycleObserver implementation.
  void ContextDestroyed() override;

  // ExecutionContextLifecycleStateObserver implementation.
  void ContextLifecycleStateChanged(mojom::FrameLifecycleState state) override;

  String id_;
  KURL url_;
  mojom::blink::PresentationConnectionState state_;

  HeapMojoReceiver<mojom::blink::PresentationConnection, PresentationConnection>
      connection_receiver_;

  // The other end of a PresentationConnection. For controller connections, this
  // can point to the browser (2-UA) or another renderer (1-UA). For receiver
  // connections, this currently only points to another renderer. This remote
  // can be used to send messages directly to the other end.
  HeapMojoRemote<mojom::blink::PresentationConnection> target_connection_;

  void CloseConnection();

 private:
  class BlobLoader;

  enum MessageType {
    kMessageTypeText,
    kMessageTypeArrayBuffer,
    kMessageTypeBlob,
  };

  class Message;

  // Implemented by controller/receiver subclasses to perform additional
  // operations when close() / terminate() is called.
  virtual void CloseInternal() = 0;
  virtual void TerminateInternal() = 0;

  bool CanSendMessage(ExceptionState&);
  void HandleMessageQueue();

  // Callbacks invoked from BlobLoader.
  void DidFinishLoadingBlob(DOMArrayBuffer*);
  void DidFailLoadingBlob(FileErrorCode);

  void SendMessageToTargetConnection(
      mojom::blink::PresentationConnectionMessagePtr);
  void DidReceiveTextMessage(const WebString&);
  void DidReceiveBinaryMessage(base::span<const uint8_t>);

  // Closes the PresentationConnection with the given reason and notifies the
  // target connection.
  void DoClose(mojom::blink::PresentationConnectionCloseReason);

  // Cancel loads and pending messages when the connection is closed.
  void TearDown();

  // For Blob data handling.
  Member<BlobLoader> blob_loader_;
  HeapDeque<Member<Message>> messages_;

  V8BinaryType::Enum binary_type_ = V8BinaryType::Enum::kArraybuffer;

  scoped_refptr<base::SingleThreadTaskRunner> file_reading_task_runner_;
};

// Represents the controller side of a connection of either a 1-UA or 2-UA
// presentation.
class MODULES_EXPORT ControllerPresentationConnection final
    : public PresentationConnection {
 public:
  static ControllerPresentationConnection* Create(
      ExecutionContext*,
      const mojom::blink::PresentationInfo&,
      PresentationRequest*);
  static ControllerPresentationConnection* Create(
      PresentationController*,
      const mojom::blink::PresentationInfo&,
      PresentationRequest*);

  ControllerPresentationConnection(LocalDOMWindow&,
                                   PresentationController*,
                                   const String& id,
                                   const KURL&);
  ~ControllerPresentationConnection() override;

  void Trace(Visitor*) const override;

  // Initializes Mojo message pipes and registers with the PresentationService.
  void Init(mojo::PendingRemote<mojom::blink::PresentationConnection>
                connection_remote,
            mojo::PendingReceiver<mojom::blink::PresentationConnection>
                connection_receiver);

 private:
  // PresentationConnection implementation.
  void CloseInternal() override;
  void TerminateInternal() override;

  Member<PresentationController> controller_;
};

// Represents the receiver side connection of a 1-UA presentation. Instances of
// this class are created as a result of
// PresentationReceiver::OnReceiverConnectionAvailable, which in turn is a
// result of creating the controller side connection of a 1-UA presentation.
class ReceiverPresentationConnection final : public PresentationConnection {
 public:
  static ReceiverPresentationConnection* Create(
      PresentationReceiver*,
      const mojom::blink::PresentationInfo&,
      mojo::PendingRemote<mojom::blink::PresentationConnection>
          controller_connection,
      mojo::PendingReceiver<mojom::blink::PresentationConnection>
          receiver_connection_receiver);

  ReceiverPresentationConnection(LocalDOMWindow&,
                                 PresentationReceiver*,
                                 const String& id,
                                 const KURL&);
  ~ReceiverPresentationConnection() override;

  void Trace(Visitor*) const override;

  void Init(mojo::PendingRemote<mojom::blink::PresentationConnection>
                controller_connection_remote,
            mojo::PendingReceiver<mojom::blink::PresentationConnection>
                receiver_connection_receiver);

  // PresentationConnection override
  void DidChangeState(mojom::blink::PresentationConnectionState) override;
  void DidClose(mojom::blink::PresentationConnectionCloseReason) override;

 private:
  // PresentationConnection implementation.
  void CloseInternal() override;

  // Changes the presentation state to TERMINATED and notifies the sender
  // connection. This method does not dispatch a state change event to the page.
  // This method is only suitable for use when the presentation receiver frame
  // containing the connection object is going away.
  void TerminateInternal() override;

  Member<PresentationReceiver> receiver_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PRESENTATION_PRESENTATION_CONNECTION_H_
