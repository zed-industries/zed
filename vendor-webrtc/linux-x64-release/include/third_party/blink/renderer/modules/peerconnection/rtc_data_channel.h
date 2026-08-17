/*
 * Copyright (C) 2012 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. AND ITS CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE INC. OR ITS CONTRIBUTORS BE LIABLE FOR
 * ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
 * OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_DATA_CHANNEL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_DATA_CHANNEL_H_

#include "base/gtest_prod_util.h"
#include "base/sequence_checker.h"
#include "base/task/single_thread_task_runner.h"
#include "base/threading/thread_checker.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_binary_type.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/fileapi/file_error.h"
#include "third_party/blink/renderer/core/fileapi/file_reader_client.h"
#include "third_party/blink/renderer/core/fileapi/file_reader_loader.h"
#include "third_party/blink/renderer/core/typed_arrays/array_buffer_view_helpers.h"
#include "third_party/blink/renderer/modules/event_target_modules.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_deque.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/prefinalizer.h"
#include "third_party/blink/renderer/platform/heap/self_keep_alive.h"
#include "third_party/blink/renderer/platform/scheduler/public/frame_scheduler.h"
#include "third_party/blink/renderer/platform/timer.h"
#include "third_party/webrtc/api/data_channel_interface.h"
#include "third_party/webrtc/api/peer_connection_interface.h"

namespace blink {

class Blob;
class DOMArrayBuffer;
class DOMArrayBufferView;
class ExceptionState;
class V8RTCDataChannelState;
class V8RTCPriorityType;

class MODULES_EXPORT RTCDataChannel final
    : public EventTarget,
      public ActiveScriptWrappable<RTCDataChannel>,
      public ExecutionContextLifecycleObserver {
  DEFINE_WRAPPERTYPEINFO();
  USING_PRE_FINALIZER(RTCDataChannel, Dispose);

 public:
  // Wraps the current thread with a webrtc::ThreadWrapper, if it isn't already
  // wrapped. This is necessary when calling some of channel()'s methods.
  // This only has an effect the first time it is called from a new
  // DedicatedWorker thread, after deserializing an RTCDataChannel.
  static void EnsureThreadWrappersForWorkerThread();

  RTCDataChannel(ExecutionContext*,
                 webrtc::scoped_refptr<webrtc::DataChannelInterface> channel);
  ~RTCDataChannel() override;

  String label() const;

  // DEPRECATED
  bool reliable() const;

  bool ordered() const;
  std::optional<uint16_t> maxPacketLifeTime() const;
  std::optional<uint16_t> maxRetransmits() const;
  String protocol() const;
  bool negotiated() const;
  std::optional<uint16_t> id() const;
  V8RTCDataChannelState readyState() const;
  unsigned bufferedAmount() const;

  unsigned bufferedAmountLowThreshold() const;
  void setBufferedAmountLowThreshold(unsigned);

  V8BinaryType binaryType() const;
  void setBinaryType(const V8BinaryType&);

  V8RTCPriorityType priority() const;

  // Functions called from RTCPeerConnection's DidAddRemoteDataChannel
  // in order to make things happen in the specified order when announcing
  // a remote channel.
  void SetStateToOpenWithoutEvent();
  void DispatchOpenEvent();

  void send(const String&, ExceptionState&);
  void send(DOMArrayBuffer*, ExceptionState&);
  void send(NotShared<DOMArrayBufferView>, ExceptionState&);
  void send(Blob*, ExceptionState&);

  void close();

  bool IsTransferable();
  webrtc::scoped_refptr<webrtc::DataChannelInterface>
  TransferUnderlyingChannel();

  DEFINE_ATTRIBUTE_EVENT_LISTENER(open, kOpen)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(bufferedamountlow, kBufferedamountlow)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(error, kError)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(close, kClose)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(closing, kClosing)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(message, kMessage)

  // EventTarget
  const AtomicString& InterfaceName() const override;
  ExecutionContext* GetExecutionContext() const override;

  // ExecutionContextLifecycleObserver
  void ContextDestroyed() override;

  // ScriptWrappable
  bool HasPendingActivity() const override;

  void Trace(Visitor*) const override;

  void ProcessSendQueue();

 private:
  friend class Observer;
  // Implementation of webrtc::DataChannelObserver that receives events on
  // webrtc's signaling thread and forwards them over to the main thread for
  // handling. Since the |blink_channel_|'s lifetime is scoped potentially
  // narrower than the |webrtc_channel_|, the observer is reference counted to
  // make sure all callbacks have a valid pointer but won't do anything if the
  // |blink_channel_| has gone away.
  class Observer : public WTF::ThreadSafeRefCounted<RTCDataChannel::Observer>,
                   public webrtc::DataChannelObserver {
   public:
    Observer(scoped_refptr<base::SingleThreadTaskRunner> main_thread,
             RTCDataChannel* blink_channel,
             webrtc::scoped_refptr<webrtc::DataChannelInterface> channel);
    ~Observer() override;

    // Returns a reference to |webrtc_channel_|. Typically called from the main
    // thread except for on observer registration, done in a synchronous call to
    // the signaling thread (safe because the call is synchronous).
    const webrtc::scoped_refptr<webrtc::DataChannelInterface>& channel() const;

    // Returns true if a valid `blink_channel_` is held and `Unregister()`
    // hasn't been called. A return value of false indicates that the `Observer`
    // can be safely discarded.
    bool is_registered() const;

    // Clears the |blink_channel_| reference, disassociates this observer from
    // the |webrtc_channel_| and releases the |webrtc_channel_| pointer. Must be
    // called on the main thread.
    void Unregister();

    // webrtc::DataChannelObserver implementation, called from signaling thread.
    void OnStateChange() override;
    void OnBufferedAmountChange(uint64_t sent_data_size) override;
    void OnMessage(const webrtc::DataBuffer& buffer) override;
    bool IsOkToCallOnTheNetworkThread() override;

   private:
    // webrtc::DataChannelObserver implementation on the main thread.
    void OnStateChangeImpl(webrtc::DataChannelInterface::DataState state);
    void OnBufferedAmountChangeImpl(unsigned sent_data_size);
    void OnMessageImpl(webrtc::DataBuffer buffer);

    const scoped_refptr<base::SingleThreadTaskRunner> main_thread_;
    WeakPersistent<RTCDataChannel> blink_channel_;
    const webrtc::scoped_refptr<webrtc::DataChannelInterface> webrtc_channel_;
  };

  void RegisterObserver();

  void OnStateChange(webrtc::DataChannelInterface::DataState state);
  void OnBufferedAmountChange(unsigned previous_amount);
  void OnMessage(webrtc::DataBuffer buffer);

  void Dispose();

  const webrtc::scoped_refptr<webrtc::DataChannelInterface>& channel() const;
  bool ValidateSendLength(uint64_t length, ExceptionState& exception_state);
  void SendRawData(const char* data, size_t length);
  void SendDataBuffer(webrtc::DataBuffer data_buffer);

  // Initializes |feature_handle_for_scheduler_|, which must not yet have been
  // initialized.
  void CreateFeatureHandleForScheduler();

  webrtc::DataChannelInterface::DataState state_ =
      webrtc::DataChannelInterface::kConnecting;

  V8BinaryType::Enum binary_type_ = V8BinaryType::Enum::kArraybuffer;

  FRIEND_TEST_ALL_PREFIXES(RTCDataChannelTest, Open);
  FRIEND_TEST_ALL_PREFIXES(RTCDataChannelTest, Close);
  FRIEND_TEST_ALL_PREFIXES(RTCDataChannelTest, Message);
  FRIEND_TEST_ALL_PREFIXES(RTCDataChannelTest, BufferedAmountLow);

  // This handle notifies the scheduler about a connected data channel
  // associated with a frame. The handle should be destroyed when the channel
  // is closed.
  FrameScheduler::SchedulingAffectingFeatureHandle
      feature_handle_for_scheduler_;

  // Once an id has been assigned, we'll set this value and use it instead
  // of querying the channel (which requires thread hop). This is a cached
  // value to optimize a const getter, and therefore `mutable`.
  mutable std::optional<uint16_t> id_;
  unsigned buffered_amount_low_threshold_ = 0u;
  unsigned buffered_amount_ = 0u;
  bool stopped_ = false;
  bool closed_from_owner_ = false;

  class PendingMessage;

  class BlobReader : public GarbageCollected<BlobReader>,
                     public ExecutionContextLifecycleObserver,
                     public FileReaderAccumulator {
   public:
    static BlobReader* Create(ExecutionContext* context,
                              RTCDataChannel* data_channel,
                              PendingMessage* message) {
      return MakeGarbageCollected<BlobReader>(context, data_channel, message);
    }

    BlobReader(ExecutionContext* context,
               RTCDataChannel* data_channel,
               PendingMessage* message);
    ~BlobReader() override;

    void Start(Blob* blob);
    bool HasFinishedLoading() const;

    // FileReaderAccumulator
    void DidFinishLoading(FileReaderData data) override;
    void DidFail(FileErrorCode error) override;

    // ExecutionContextLifecycleObserver
    void ContextDestroyed() override;

    // GarbageCollected
    void Trace(Visitor*) const override;

   private:
    void Dispose();

    Member<FileReaderLoader> loader_;
    Member<RTCDataChannel> data_channel_;
    Member<PendingMessage> message_;

    SelfKeepAlive<BlobReader> keep_alive_;
    SEQUENCE_CHECKER(sequence_checker_);
  };

  class PendingMessage final : public GarbageCollected<PendingMessage> {
   public:
    enum class Type {
      kBufferReady,
      kBufferPending,
      kCloseEvent,
      kBlobFailure,
    };

    void Trace(Visitor* visitor) const;

    Type type_;
    std::optional<webrtc::DataBuffer> buffer_;
    Member<BlobReader> blob_reader_;
  };
  HeapDeque<Member<PendingMessage>> pending_messages_;

  bool was_transferred_ = false;
  bool is_transferable_ = true;
  // Keep the `observer_` reference const to make it clear that we don't want
  // to free the underlying channel (or callback observer) until the
  // `RTCDataChannel` instance goes away. This allows properties to be queried
  // after the state reaches `kClosed`.
  const scoped_refptr<Observer> observer_;
  SEQUENCE_CHECKER(sequence_checker_);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_DATA_CHANNEL_H_
