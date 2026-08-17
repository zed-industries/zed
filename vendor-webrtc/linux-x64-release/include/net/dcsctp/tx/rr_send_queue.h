/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef NET_DCSCTP_TX_RR_SEND_QUEUE_H_
#define NET_DCSCTP_TX_RR_SEND_QUEUE_H_

#include <cstdint>
#include <deque>
#include <map>
#include <memory>
#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "absl/algorithm/container.h"
#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "net/dcsctp/common/internal_types.h"
#include "net/dcsctp/public/dcsctp_message.h"
#include "net/dcsctp/public/dcsctp_socket.h"
#include "net/dcsctp/public/types.h"
#include "net/dcsctp/tx/send_queue.h"
#include "net/dcsctp/tx/stream_scheduler.h"

namespace dcsctp {

// The Round Robin SendQueue holds all messages that the client wants to send,
// but that haven't yet been split into chunks and fully sent on the wire.
//
// As defined in https://datatracker.ietf.org/doc/html/rfc8260#section-3.2,
// it will cycle to send messages from different streams. It will send all
// fragments from one message before continuing with a different message on
// possibly a different stream, until support for message interleaving has been
// implemented.
//
// As messages can be (requested to be) sent before the connection is properly
// established, this send queue is always present - even for closed connections.
//
// The send queue may trigger callbacks:
//  * `OnBufferedAmountLow`, `OnTotalBufferedAmountLow`
//    These will be triggered as defined in their documentation.
//  * `OnLifecycleMessageExpired(/*maybe_delivered=*/false)`, `OnLifecycleEnd`
//    These will be triggered when messages have been expired, abandoned or
//    discarded from the send queue. If a message is fully produced, meaning
//    that the last fragment has been produced, the responsibility to send
//    lifecycle events is then transferred to the retransmission queue, which
//    is the one asking to produce the message.
class RRSendQueue : public SendQueue {
 public:
  RRSendQueue(absl::string_view log_prefix,
              DcSctpSocketCallbacks* callbacks,
              size_t mtu,
              StreamPriority default_priority,
              size_t total_buffered_amount_low_threshold);

  // Indicates if the buffer is full. Note that it's up to the caller to ensure
  // that the buffer is not full prior to adding new items to it.
  bool IsFull() const;
  // Indicates if the buffer is empty.
  bool IsEmpty() const;

  // Adds the message to be sent using the `send_options` provided. The current
  // time should be in `now`. Note that it's the responsibility of the caller to
  // ensure that the buffer is not full (by calling `IsFull`) before adding
  // messages to it.
  void Add(webrtc::Timestamp now,
           DcSctpMessage message,
           const SendOptions& send_options = {});

  // Implementation of `SendQueue`.
  std::optional<DataToSend> Produce(webrtc::Timestamp now,
                                    size_t max_size) override;
  bool Discard(StreamID stream_id, OutgoingMessageId message_id) override;
  void PrepareResetStream(StreamID streams) override;
  bool HasStreamsReadyToBeReset() const override;
  std::vector<StreamID> GetStreamsReadyToBeReset() override;
  void CommitResetStreams() override;
  void RollbackResetStreams() override;
  void Reset() override;
  size_t buffered_amount(StreamID stream_id) const override;
  size_t total_buffered_amount() const override {
    return total_buffered_amount_.value();
  }
  size_t buffered_amount_low_threshold(StreamID stream_id) const override;
  void SetBufferedAmountLowThreshold(StreamID stream_id, size_t bytes) override;
  void EnableMessageInterleaving(bool enabled) override {
    scheduler_.EnableMessageInterleaving(enabled);
  }

  void SetStreamPriority(StreamID stream_id, StreamPriority priority);
  StreamPriority GetStreamPriority(StreamID stream_id) const;
  HandoverReadinessStatus GetHandoverReadiness() const;
  void AddHandoverState(DcSctpSocketHandoverState& state);
  void RestoreFromState(const DcSctpSocketHandoverState& state);

 private:
  struct MessageAttributes {
    IsUnordered unordered;
    MaxRetransmits max_retransmissions;
    webrtc::Timestamp expires_at;
    LifecycleId lifecycle_id;
  };

  // Represents a value and a "low threshold" that when the value reaches or
  // goes under the "low threshold", will trigger `on_threshold_reached`
  // callback.
  class ThresholdWatcher {
   public:
    explicit ThresholdWatcher(std::function<void()> on_threshold_reached)
        : on_threshold_reached_(std::move(on_threshold_reached)) {}
    // Increases the value.
    void Increase(size_t bytes) { value_ += bytes; }
    // Decreases the value and triggers `on_threshold_reached` if it's at or
    // below `low_threshold()`.
    void Decrease(size_t bytes);

    size_t value() const { return value_; }
    size_t low_threshold() const { return low_threshold_; }
    void SetLowThreshold(size_t low_threshold);

   private:
    const std::function<void()> on_threshold_reached_;
    size_t value_ = 0;
    size_t low_threshold_ = 0;
  };

  // Per-stream information.
  class OutgoingStream : public StreamScheduler::StreamProducer {
   public:
    OutgoingStream(
        RRSendQueue* parent,
        StreamScheduler* scheduler,
        StreamID stream_id,
        StreamPriority priority,
        std::function<void()> on_buffered_amount_low,
        const DcSctpSocketHandoverState::OutgoingStream* state = nullptr)
        : parent_(*parent),
          scheduler_stream_(scheduler->CreateStream(this, stream_id, priority)),
          next_unordered_mid_(MID(state ? state->next_unordered_mid : 0)),
          next_ordered_mid_(MID(state ? state->next_ordered_mid : 0)),
          next_ssn_(SSN(state ? state->next_ssn : 0)),
          buffered_amount_(std::move(on_buffered_amount_low)) {}

    StreamID stream_id() const { return scheduler_stream_->stream_id(); }

    // Enqueues a message to this stream.
    void Add(DcSctpMessage message, MessageAttributes attributes);

    // Implementing `StreamScheduler::StreamProducer`.
    std::optional<SendQueue::DataToSend> Produce(webrtc::Timestamp now,
                                                 size_t max_size) override;
    size_t bytes_to_send_in_next_message() const override;

    const ThresholdWatcher& buffered_amount() const { return buffered_amount_; }
    ThresholdWatcher& buffered_amount() { return buffered_amount_; }

    // Discards a partially sent message, see `SendQueue::Discard`.
    bool Discard(OutgoingMessageId message_id);

    // Pauses this stream, which is used before resetting it.
    void Pause();

    // Resumes a paused stream.
    void Resume();

    bool IsReadyToBeReset() const {
      return pause_state_ == PauseState::kPaused;
    }

    bool IsResetting() const { return pause_state_ == PauseState::kResetting; }

    void SetAsResetting() {
      RTC_DCHECK(pause_state_ == PauseState::kPaused);
      pause_state_ = PauseState::kResetting;
    }

    // Resets this stream, meaning MIDs and SSNs are set to zero.
    void Reset();

    // Indicates if this stream has a partially sent message in it.
    bool has_partially_sent_message() const;

    StreamPriority priority() const { return scheduler_stream_->priority(); }
    void SetPriority(StreamPriority priority) {
      scheduler_stream_->SetPriority(priority);
    }

    void AddHandoverState(
        DcSctpSocketHandoverState::OutgoingStream& state) const;

   private:
    // Streams are paused before they can be reset. To reset a stream, the
    // socket sends an outgoing stream reset command with the TSN of the last
    // fragment of the last message, so that receivers and senders can agree on
    // when it stopped. And if the send queue is in the middle of sending a
    // message, and without fragments not yet sent and without TSNs allocated to
    // them, it will keep sending data until that message has ended.
    enum class PauseState {
      // The stream is not paused, and not scheduled to be reset.
      kNotPaused,
      // The stream has requested to be reset/paused but is still producing
      // fragments of a message that hasn't ended yet. When it does, it will
      // transition to the `kPaused` state.
      kPending,
      // The stream is fully paused and can be reset.
      kPaused,
      // The stream has been added to an outgoing stream reset request and a
      // response from the peer hasn't been received yet.
      kResetting,
    };

    // An enqueued message and metadata.
    struct Item {
      explicit Item(OutgoingMessageId message_id,
                    DcSctpMessage msg,
                    MessageAttributes attributes)
          : message_id(message_id),
            message(std::move(msg)),
            attributes(std::move(attributes)),
            remaining_offset(0),
            remaining_size(message.payload().size()) {}
      OutgoingMessageId message_id;
      DcSctpMessage message;
      MessageAttributes attributes;
      // The remaining payload (offset and size) to be sent, when it has been
      // fragmented.
      size_t remaining_offset;
      size_t remaining_size;
      // If set, an allocated Message ID and SSN. Will be allocated when the
      // first fragment is sent.
      std::optional<MID> mid = std::nullopt;
      std::optional<SSN> ssn = std::nullopt;
      // The current Fragment Sequence Number, incremented for each fragment.
      FSN current_fsn = FSN(0);
    };

    bool IsConsistent() const;
    void HandleMessageExpired(OutgoingStream::Item& item);

    RRSendQueue& parent_;

    const std::unique_ptr<StreamScheduler::Stream> scheduler_stream_;

    PauseState pause_state_ = PauseState::kNotPaused;
    // MIDs are different for unordered and ordered messages sent on a stream.
    MID next_unordered_mid_;
    MID next_ordered_mid_;

    SSN next_ssn_;
    // Enqueued messages, and metadata.
    std::deque<Item> items_;

    // The current amount of buffered data.
    ThresholdWatcher buffered_amount_;
  };

  bool IsConsistent() const;
  OutgoingStream& GetOrCreateStreamInfo(StreamID stream_id);
  std::optional<DataToSend> Produce(
      std::map<StreamID, OutgoingStream>::iterator it,
      webrtc::Timestamp now,
      size_t max_size);

  const absl::string_view log_prefix_;
  DcSctpSocketCallbacks& callbacks_;
  const StreamPriority default_priority_;
  OutgoingMessageId current_message_id = OutgoingMessageId(0);
  StreamScheduler scheduler_;

  // The total amount of buffer data, for all streams.
  ThresholdWatcher total_buffered_amount_;

  // All streams, and messages added to those.
  std::map<StreamID, OutgoingStream> streams_;
};
}  // namespace dcsctp

#endif  // NET_DCSCTP_TX_RR_SEND_QUEUE_H_
