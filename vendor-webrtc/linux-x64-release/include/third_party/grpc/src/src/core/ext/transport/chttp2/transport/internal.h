//
//
// Copyright 2015 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
//

#ifndef GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_INTERNAL_H
#define GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_INTERNAL_H

#include <grpc/event_engine/event_engine.h>
#include <grpc/event_engine/memory_allocator.h>
#include <grpc/grpc.h>
#include <grpc/slice.h>
#include <grpc/support/port_platform.h>
#include <grpc/support/time.h>
#include <stddef.h>
#include <stdint.h>

#include <atomic>
#include <memory>
#include <optional>
#include <utility>
#include <variant>

#include "absl/container/flat_hash_map.h"
#include "absl/random/random.h"
#include "absl/status/status.h"
#include "absl/strings/string_view.h"
#include "src/core/call/metadata_batch.h"
#include "src/core/channelz/channelz.h"
#include "src/core/ext/transport/chttp2/transport/call_tracer_wrapper.h"
#include "src/core/ext/transport/chttp2/transport/flow_control.h"
#include "src/core/ext/transport/chttp2/transport/frame_goaway.h"
#include "src/core/ext/transport/chttp2/transport/frame_ping.h"
#include "src/core/ext/transport/chttp2/transport/frame_rst_stream.h"
#include "src/core/ext/transport/chttp2/transport/frame_security.h"
#include "src/core/ext/transport/chttp2/transport/frame_settings.h"
#include "src/core/ext/transport/chttp2/transport/frame_window_update.h"
#include "src/core/ext/transport/chttp2/transport/hpack_encoder.h"
#include "src/core/ext/transport/chttp2/transport/hpack_parser.h"
#include "src/core/ext/transport/chttp2/transport/http2_settings.h"
#include "src/core/ext/transport/chttp2/transport/http2_ztrace_collector.h"
#include "src/core/ext/transport/chttp2/transport/legacy_frame.h"
#include "src/core/ext/transport/chttp2/transport/ping_abuse_policy.h"
#include "src/core/ext/transport/chttp2/transport/ping_callbacks.h"
#include "src/core/ext/transport/chttp2/transport/ping_rate_policy.h"
#include "src/core/ext/transport/chttp2/transport/write_size_policy.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/debug/trace.h"
#include "src/core/lib/iomgr/closure.h"
#include "src/core/lib/iomgr/combiner.h"
#include "src/core/lib/iomgr/endpoint.h"
#include "src/core/lib/iomgr/error.h"
#include "src/core/lib/iomgr/iomgr_fwd.h"
#include "src/core/lib/resource_quota/arena.h"
#include "src/core/lib/resource_quota/memory_quota.h"
#include "src/core/lib/slice/slice.h"
#include "src/core/lib/slice/slice_buffer.h"
#include "src/core/lib/surface/init_internally.h"
#include "src/core/lib/transport/connectivity_state.h"
#include "src/core/lib/transport/transport.h"
#include "src/core/lib/transport/transport_framing_endpoint_extension.h"
#include "src/core/telemetry/call_tracer.h"
#include "src/core/telemetry/context_list_entry.h"
#include "src/core/telemetry/stats.h"
#include "src/core/util/bitset.h"
#include "src/core/util/debug_location.h"
#include "src/core/util/ref_counted.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/time.h"

// Flag that this closure barrier may be covering a write in a pollset, and so
//   we should not complete this closure until we can prove that the write got
//   scheduled
#define CLOSURE_BARRIER_MAY_COVER_WRITE (1 << 0)
// First bit of the reference count, stored in the high order bits (with the low
//   bits being used for flags defined above)
#define CLOSURE_BARRIER_FIRST_REF_BIT (1 << 16)

// streams are kept in various linked lists depending on what things need to
// happen to them... this enum labels each list
typedef enum {
  // If a stream is in the following two lists, an explicit ref is associated
  // with the stream
  GRPC_CHTTP2_LIST_WRITABLE,
  GRPC_CHTTP2_LIST_WRITING,
  // No additional ref is taken for the following refs. Make sure to remove the
  // stream from these lists when the stream is removed.
  GRPC_CHTTP2_LIST_STALLED_BY_TRANSPORT,
  GRPC_CHTTP2_LIST_STALLED_BY_STREAM,
  /// streams that are waiting to start because there are too many concurrent
  /// streams on the connection
  GRPC_CHTTP2_LIST_WAITING_FOR_CONCURRENCY,
  STREAM_LIST_COUNT  // must be last
} grpc_chttp2_stream_list_id;

typedef enum {
  GRPC_CHTTP2_WRITE_STATE_IDLE,
  GRPC_CHTTP2_WRITE_STATE_WRITING,
  GRPC_CHTTP2_WRITE_STATE_WRITING_WITH_MORE,
} grpc_chttp2_write_state;

typedef enum {
  GRPC_CHTTP2_OPTIMIZE_FOR_LATENCY,
  GRPC_CHTTP2_OPTIMIZE_FOR_THROUGHPUT,
} grpc_chttp2_optimization_target;

typedef enum {
  GRPC_CHTTP2_PCL_INITIATE = 0,
  GRPC_CHTTP2_PCL_NEXT,
  GRPC_CHTTP2_PCL_INFLIGHT,
  GRPC_CHTTP2_PCL_COUNT  // must be last
} grpc_chttp2_ping_closure_list;

typedef enum {
  GRPC_CHTTP2_INITIATE_WRITE_INITIAL_WRITE,
  GRPC_CHTTP2_INITIATE_WRITE_START_NEW_STREAM,
  GRPC_CHTTP2_INITIATE_WRITE_SEND_MESSAGE,
  GRPC_CHTTP2_INITIATE_WRITE_SEND_INITIAL_METADATA,
  GRPC_CHTTP2_INITIATE_WRITE_SEND_TRAILING_METADATA,
  GRPC_CHTTP2_INITIATE_WRITE_RETRY_SEND_PING,
  GRPC_CHTTP2_INITIATE_WRITE_CONTINUE_PINGS,
  GRPC_CHTTP2_INITIATE_WRITE_GOAWAY_SENT,
  GRPC_CHTTP2_INITIATE_WRITE_RST_STREAM,
  GRPC_CHTTP2_INITIATE_WRITE_CLOSE_FROM_API,
  GRPC_CHTTP2_INITIATE_WRITE_STREAM_FLOW_CONTROL,
  GRPC_CHTTP2_INITIATE_WRITE_TRANSPORT_FLOW_CONTROL,
  GRPC_CHTTP2_INITIATE_WRITE_SEND_SETTINGS,
  GRPC_CHTTP2_INITIATE_WRITE_SETTINGS_ACK,
  GRPC_CHTTP2_INITIATE_WRITE_FLOW_CONTROL_UNSTALLED_BY_SETTING,
  GRPC_CHTTP2_INITIATE_WRITE_FLOW_CONTROL_UNSTALLED_BY_UPDATE,
  GRPC_CHTTP2_INITIATE_WRITE_APPLICATION_PING,
  GRPC_CHTTP2_INITIATE_WRITE_BDP_PING,
  GRPC_CHTTP2_INITIATE_WRITE_KEEPALIVE_PING,
  GRPC_CHTTP2_INITIATE_WRITE_TRANSPORT_FLOW_CONTROL_UNSTALLED,
  GRPC_CHTTP2_INITIATE_WRITE_PING_RESPONSE,
  GRPC_CHTTP2_INITIATE_WRITE_FORCE_RST_STREAM,
} grpc_chttp2_initiate_write_reason;

const char* grpc_chttp2_initiate_write_reason_string(
    grpc_chttp2_initiate_write_reason reason);

// deframer state for the overall http2 stream of bytes
typedef enum {
  // prefix: one entry per http2 connection prefix byte
  GRPC_DTS_CLIENT_PREFIX_0 = 0,
  GRPC_DTS_CLIENT_PREFIX_1,
  GRPC_DTS_CLIENT_PREFIX_2,
  GRPC_DTS_CLIENT_PREFIX_3,
  GRPC_DTS_CLIENT_PREFIX_4,
  GRPC_DTS_CLIENT_PREFIX_5,
  GRPC_DTS_CLIENT_PREFIX_6,
  GRPC_DTS_CLIENT_PREFIX_7,
  GRPC_DTS_CLIENT_PREFIX_8,
  GRPC_DTS_CLIENT_PREFIX_9,
  GRPC_DTS_CLIENT_PREFIX_10,
  GRPC_DTS_CLIENT_PREFIX_11,
  GRPC_DTS_CLIENT_PREFIX_12,
  GRPC_DTS_CLIENT_PREFIX_13,
  GRPC_DTS_CLIENT_PREFIX_14,
  GRPC_DTS_CLIENT_PREFIX_15,
  GRPC_DTS_CLIENT_PREFIX_16,
  GRPC_DTS_CLIENT_PREFIX_17,
  GRPC_DTS_CLIENT_PREFIX_18,
  GRPC_DTS_CLIENT_PREFIX_19,
  GRPC_DTS_CLIENT_PREFIX_20,
  GRPC_DTS_CLIENT_PREFIX_21,
  GRPC_DTS_CLIENT_PREFIX_22,
  GRPC_DTS_CLIENT_PREFIX_23,
  // frame header byte 0...
  // must follow from the prefix states
  GRPC_DTS_FH_0,
  GRPC_DTS_FH_1,
  GRPC_DTS_FH_2,
  GRPC_DTS_FH_3,
  GRPC_DTS_FH_4,
  GRPC_DTS_FH_5,
  GRPC_DTS_FH_6,
  GRPC_DTS_FH_7,
  // ... frame header byte 8
  GRPC_DTS_FH_8,
  // inside a http2 frame
  GRPC_DTS_FRAME
} grpc_chttp2_deframe_transport_state;

struct grpc_chttp2_stream_list {
  grpc_chttp2_stream* head;
  grpc_chttp2_stream* tail;
};
struct grpc_chttp2_stream_link {
  grpc_chttp2_stream* next;
  grpc_chttp2_stream* prev;
};

typedef enum {
  GRPC_CHTTP2_NO_GOAWAY_SEND,
  GRPC_CHTTP2_GRACEFUL_GOAWAY,
  GRPC_CHTTP2_FINAL_GOAWAY_SEND_SCHEDULED,
  GRPC_CHTTP2_FINAL_GOAWAY_SENT,
} grpc_chttp2_sent_goaway_state;

typedef struct grpc_chttp2_write_cb {
  int64_t call_at_byte;
  grpc_closure* closure;
  struct grpc_chttp2_write_cb* next;
} grpc_chttp2_write_cb;

typedef enum {
  GRPC_CHTTP2_KEEPALIVE_STATE_WAITING,
  GRPC_CHTTP2_KEEPALIVE_STATE_PINGING,
  GRPC_CHTTP2_KEEPALIVE_STATE_DYING,
  GRPC_CHTTP2_KEEPALIVE_STATE_DISABLED,
} grpc_chttp2_keepalive_state;

struct grpc_chttp2_transport final : public grpc_core::FilterStackTransport,
                                     public grpc_core::KeepsGrpcInitialized {
  grpc_chttp2_transport(const grpc_core::ChannelArgs& channel_args,
                        grpc_core::OrphanablePtr<grpc_endpoint> endpoint,
                        bool is_client);
  ~grpc_chttp2_transport() override;

  class ChannelzDataSource final : public grpc_core::channelz::DataSource {
   public:
    explicit ChannelzDataSource(grpc_chttp2_transport* transport)
        : grpc_core::channelz::DataSource(transport->channelz_socket),
          transport_(transport) {}

    void AddData(grpc_core::channelz::DataSink& sink) override;
    std::unique_ptr<grpc_core::channelz::ZTrace> GetZTrace(
        absl::string_view name) override;

   private:
    grpc_chttp2_transport* transport_;
  };

  void Orphan() override;

  grpc_core::RefCountedPtr<grpc_chttp2_transport> Ref() {
    return grpc_core::FilterStackTransport::RefAsSubclass<
        grpc_chttp2_transport>();
  }

  size_t SizeOfStream() const override;
  bool HackyDisableStreamOpBatchCoalescingInConnectedChannel() const override;
  void PerformStreamOp(grpc_stream* gs,
                       grpc_transport_stream_op_batch* op) override;
  void DestroyStream(grpc_stream* gs,
                     grpc_closure* then_schedule_closure) override;

  grpc_core::FilterStackTransport* filter_stack_transport() override {
    return this;
  }
  grpc_core::ClientTransport* client_transport() override { return nullptr; }
  grpc_core::ServerTransport* server_transport() override { return nullptr; }

  absl::string_view GetTransportName() const override;
  grpc_core::RefCountedPtr<grpc_core::channelz::SocketNode> GetSocketNode()
      const override {
    return channelz_socket;
  }
  void InitStream(grpc_stream* gs, grpc_stream_refcount* refcount,
                  const void* server_data, grpc_core::Arena* arena) override;
  void SetPollset(grpc_stream* stream, grpc_pollset* pollset) override;
  void SetPollsetSet(grpc_stream* stream,
                     grpc_pollset_set* pollset_set) override;
  void PerformOp(grpc_transport_op* op) override;
  // Callback for transport framing endpoint extension to send security frames
  // received directly from the endpoint on wire.
  void WriteSecurityFrame(grpc_core::SliceBuffer* data);
  void WriteSecurityFrameLocked(grpc_core::SliceBuffer* data);

  grpc_core::OrphanablePtr<grpc_endpoint> ep;
  grpc_core::Mutex ep_destroy_mu;  // Guards endpoint destruction only.

  grpc_core::Slice peer_string;

  grpc_core::TransportFramingEndpointExtension*
      transport_framing_endpoint_extension = nullptr;

  grpc_core::MemoryOwner memory_owner;
  const grpc_core::MemoryAllocator::Reservation self_reservation;
  grpc_core::ReclamationSweep active_reclamation;

  std::shared_ptr<grpc_event_engine::experimental::EventEngine> event_engine;
  grpc_core::Combiner* combiner;

  // On the client side, when the transport is first created, the
  // endpoint will already have been added to this pollset_set, and it
  // needs to stay there until the notify_on_receive_settings callback
  // is invoked.  After that, the polling will be coordinated via the
  // bind_pollset_set transport op, sent by the subchannel when it
  // starts a connectivity watch.
  grpc_pollset_set* interested_parties_until_recv_settings = nullptr;

  grpc_closure* notify_on_receive_settings = nullptr;
  grpc_closure* notify_on_close = nullptr;

  /// has the upper layer closed the transport?
  grpc_error_handle closed_with_error;

  /// various lists of streams
  grpc_chttp2_stream_list lists[STREAM_LIST_COUNT] = {};

  /// maps stream id to grpc_chttp2_stream objects
  absl::flat_hash_map<uint32_t, grpc_chttp2_stream*> stream_map;
  // Count of streams that should be counted against max concurrent streams but
  // are not in stream_map (due to tarpitting).
  size_t extra_streams = 0;

  class RemovedStreamHandle {
   public:
    RemovedStreamHandle() = default;
    explicit RemovedStreamHandle(
        grpc_core::RefCountedPtr<grpc_chttp2_transport> t)
        : transport_(std::move(t)) {
      ++transport_->extra_streams;
    }
    ~RemovedStreamHandle() {
      if (transport_ != nullptr) {
        --transport_->extra_streams;
      }
    }
    RemovedStreamHandle(const RemovedStreamHandle&) = delete;
    RemovedStreamHandle& operator=(const RemovedStreamHandle&) = delete;
    RemovedStreamHandle(RemovedStreamHandle&&) = default;
    RemovedStreamHandle& operator=(RemovedStreamHandle&&) = default;

   private:
    grpc_core::RefCountedPtr<grpc_chttp2_transport> transport_;
  };

  grpc_closure write_action_begin_locked;
  grpc_closure write_action_end_locked;

  grpc_closure read_action_locked;

  /// incoming read bytes
  grpc_slice_buffer read_buffer;

  /// address to place a newly accepted stream - set and unset by
  /// grpc_chttp2_parsing_accept_stream; used by init_stream to
  /// publish the accepted server stream
  grpc_chttp2_stream** accepting_stream = nullptr;

  // accept stream callback
  void (*accept_stream_cb)(void* user_data, grpc_core::Transport* transport,
                           const void* server_data);
  // registered_method_matcher_cb is called before invoking the recv initial
  // metadata callback.
  void (*registered_method_matcher_cb)(
      void* user_data, grpc_core::ServerMetadata* metadata) = nullptr;
  void* accept_stream_cb_user_data;

  /// connectivity tracking
  grpc_core::ConnectivityStateTracker state_tracker;

  /// data to write now
  grpc_core::SliceBuffer outbuf;
  /// hpack encoding
  grpc_core::HPackCompressor hpack_compressor;

  /// data to write next write
  grpc_slice_buffer qbuf;

  size_t max_requests_per_read;

  /// Set to a grpc_error object if a goaway frame is received. By default, set
  /// to absl::OkStatus()
  grpc_error_handle goaway_error;

  grpc_chttp2_sent_goaway_state sent_goaway_state = GRPC_CHTTP2_NO_GOAWAY_SEND;

  /// settings values
  grpc_core::Http2SettingsManager settings;

  grpc_event_engine::experimental::EventEngine::TaskHandle
      settings_ack_watchdog =
          grpc_event_engine::experimental::EventEngine::TaskHandle::kInvalid;

  /// what is the next stream id to be allocated by this peer?
  /// copied to next_stream_id in parsing when parsing commences
  uint32_t next_stream_id = 0;

  /// last new stream id
  uint32_t last_new_stream_id = 0;

  /// Number of incoming streams allowed before a settings ACK is required
  uint32_t num_incoming_streams_before_settings_ack = 0;

  /// ping queues for various ping insertion points
  grpc_core::Chttp2PingAbusePolicy ping_abuse_policy;
  grpc_core::Chttp2PingRatePolicy ping_rate_policy;
  grpc_core::Chttp2PingCallbacks ping_callbacks;
  grpc_event_engine::experimental::EventEngine::TaskHandle
      delayed_ping_timer_handle =
          grpc_event_engine::experimental::EventEngine::TaskHandle::kInvalid;
  grpc_closure retry_initiate_ping_locked;

  /// ping acks
  size_t ping_ack_count = 0;
  size_t ping_ack_capacity = 0;
  uint64_t* ping_acks = nullptr;

  /// parser for headers
  grpc_core::HPackParser hpack_parser;
  /// simple one shot parsers
  union {
    grpc_chttp2_window_update_parser window_update;
    grpc_chttp2_settings_parser settings;
    grpc_chttp2_ping_parser ping;
    grpc_chttp2_rst_stream_parser rst_stream;
  } simple;
  /// parser for goaway frames
  grpc_chttp2_goaway_parser goaway_parser;
  // parser for secure frames
  grpc_chttp2_security_frame_parser security_frame_parser;

  grpc_core::chttp2::TransportFlowControl flow_control;
  /// initial window change. This is tracked as we parse settings frames from
  /// the remote peer. If there is a positive delta, then we will make all
  /// streams readable since they may have become unstalled
  int64_t initial_window_update = 0;

  // deframing
  grpc_chttp2_deframe_transport_state deframe_state = GRPC_DTS_CLIENT_PREFIX_0;
  uint8_t incoming_frame_type = 0;
  uint8_t incoming_frame_flags = 0;
  uint8_t header_eof = 0;
  bool is_first_frame = true;
  uint32_t expect_continuation_stream_id = 0;
  uint32_t incoming_frame_size = 0;

  int min_tarpit_duration_ms;
  int max_tarpit_duration_ms;
  bool allow_tarpit;

  grpc_chttp2_stream* incoming_stream = nullptr;
  // active parser
  struct Parser {
    const char* name;
    grpc_error_handle (*parser)(void* parser_user_data,
                                grpc_chttp2_transport* t, grpc_chttp2_stream* s,
                                const grpc_slice& slice, int is_last);
    void* user_data = nullptr;
  };
  Parser parser;

  grpc_chttp2_write_cb* write_cb_pool = nullptr;

  // bdp estimator
  grpc_closure next_bdp_ping_timer_expired_locked;
  grpc_closure start_bdp_ping_locked;
  grpc_closure finish_bdp_ping_locked;

  // if non-NULL, close the transport with this error when writes are finished
  grpc_error_handle close_transport_on_writes_finished;

  // a list of closures to run after writes are finished
  grpc_closure_list run_after_write = GRPC_CLOSURE_LIST_INIT;

  // buffer pool state
  /// benign cleanup closure
  grpc_closure benign_reclaimer_locked;
  /// destructive cleanup closure
  grpc_closure destructive_reclaimer_locked;

  // next bdp ping timer handle
  grpc_event_engine::experimental::EventEngine::TaskHandle
      next_bdp_ping_timer_handle =
          grpc_event_engine::experimental::EventEngine::TaskHandle::kInvalid;

  // keep-alive ping support
  /// Closure to initialize a keepalive ping
  grpc_closure init_keepalive_ping_locked;
  /// Closure to run when the keepalive ping ack is received
  grpc_closure finish_keepalive_ping_locked;
  /// timer to initiate ping events
  grpc_event_engine::experimental::EventEngine::TaskHandle
      keepalive_ping_timer_handle =
          grpc_event_engine::experimental::EventEngine::TaskHandle::kInvalid;
  /// time duration in between pings
  grpc_core::Duration keepalive_time;
  /// Tracks any adjustments to the absolute timestamp of the next keepalive
  /// timer callback execution.
  grpc_core::Timestamp next_adjusted_keepalive_timestamp;
  /// grace period to wait for data after sending a ping before keepalives
  /// timeout
  grpc_core::Duration keepalive_timeout;
  /// number of stream objects currently allocated by this transport
  std::atomic<size_t> streams_allocated{0};
  /// keep-alive state machine state
  grpc_chttp2_keepalive_state keepalive_state;
  // Soft limit on max header size.
  uint32_t max_header_list_size_soft_limit = 0;
  grpc_core::ContextList* context_list = nullptr;
  grpc_core::RefCountedPtr<grpc_core::channelz::SocketNode> channelz_socket;
  std::unique_ptr<ChannelzDataSource> channelz_data_source;
  uint32_t num_messages_in_next_write = 0;
  /// The number of pending induced frames (SETTINGS_ACK, PINGS_ACK and
  /// RST_STREAM) in the outgoing buffer (t->qbuf). If this number goes beyond
  /// DEFAULT_MAX_PENDING_INDUCED_FRAMES, we pause reading new frames. We would
  /// only continue reading when we are able to write to the socket again,
  /// thereby reducing the number of induced frames.
  uint32_t num_pending_induced_frames = 0;
  uint32_t incoming_stream_id = 0;

  /// grace period after sending a ping to wait for the ping ack
  grpc_core::Duration ping_timeout;
  grpc_event_engine::experimental::EventEngine::TaskHandle
      keepalive_ping_timeout_handle =
          grpc_event_engine::experimental::EventEngine::TaskHandle::kInvalid;
  /// grace period before settings timeout expires
  grpc_core::Duration settings_timeout;

  /// how much data are we willing to buffer when the WRITE_BUFFER_HINT is set?
  uint32_t write_buffer_size = grpc_core::chttp2::kDefaultWindow;

  /// write execution state of the transport
  grpc_chttp2_write_state write_state = GRPC_CHTTP2_WRITE_STATE_IDLE;

  /// policy for how much data we're willing to put into one http2 write
  grpc_core::Chttp2WriteSizePolicy write_size_policy;

  bool reading_paused_on_pending_induced_frames = false;
  /// Based on channel args, preferred_rx_crypto_frame_sizes are advertised to
  /// the peer
  bool enable_preferred_rx_crypto_frame_advertisement = false;
  /// Set to non zero if closures associated with the transport may be
  /// covering a write in a pollset. Such closures cannot be scheduled until
  /// we can prove that the write got scheduled.
  uint8_t closure_barrier_may_cover_write = CLOSURE_BARRIER_MAY_COVER_WRITE;

  /// have we scheduled a benign cleanup?
  bool benign_reclaimer_registered = false;
  /// have we scheduled a destructive cleanup?
  bool destructive_reclaimer_registered = false;

  /// if keepalive pings are allowed when there's no outstanding streams
  bool keepalive_permit_without_calls = false;

  // bdp estimator
  bool bdp_ping_blocked =
      false;  // Is the BDP blocked due to not receiving any data?

  /// is the transport destroying itself?
  uint8_t destroying = false;

  /// is this a client?
  bool is_client;

  /// If start_bdp_ping_locked has been called
  bool bdp_ping_started = false;
  // True if pings should be acked
  bool ack_pings = true;
  /// True if the keepalive system wants to see some data incoming
  bool keepalive_incoming_data_wanted = false;
  /// True if we count stream allocation (instead of HTTP2 concurrency) for
  /// MAX_CONCURRENT_STREAMS
  bool max_concurrent_streams_overload_protection = false;
  bool max_concurrent_streams_reject_on_client = false;

  // What percentage of rst_stream frames on the server should cause a ping
  // frame to be generated.
  uint8_t ping_on_rst_stream_percent;

  // The last time a transport window update was received.
  grpc_core::Timestamp last_window_update_time =
      grpc_core::Timestamp::InfPast();

  grpc_core::Http2StatsCollector http2_stats;
  grpc_core::Http2ZTraceCollector http2_ztrace_collector;

  GPR_NO_UNIQUE_ADDRESS grpc_core::latent_see::Flow write_flow;
};

typedef enum {
  GRPC_METADATA_NOT_PUBLISHED,
  GRPC_METADATA_SYNTHESIZED_FROM_FAKE,
  GRPC_METADATA_PUBLISHED_FROM_WIRE,
  GRPC_METADATA_PUBLISHED_AT_CLOSE
} grpc_published_metadata_method;

struct grpc_chttp2_stream {
  grpc_chttp2_stream(grpc_chttp2_transport* t, grpc_stream_refcount* refcount,
                     const void* server_data, grpc_core::Arena* arena);
  ~grpc_chttp2_stream();

  const grpc_core::RefCountedPtr<grpc_chttp2_transport> t;
  grpc_stream_refcount* refcount;
  grpc_core::Arena* const arena;

  grpc_closure destroy_stream;
  grpc_closure* destroy_stream_arg;

  grpc_chttp2_stream_link links[STREAM_LIST_COUNT];

  /// HTTP2 stream id for this stream, or zero if one has not been assigned
  uint32_t id = 0;

  /// things the upper layers would like to send
  grpc_metadata_batch* send_initial_metadata = nullptr;
  grpc_closure* send_initial_metadata_finished = nullptr;
  grpc_metadata_batch* send_trailing_metadata = nullptr;
  // TODO(yashykt): Find a better name for the below field and others in this
  //                struct to betteer distinguish inputs, return values, and
  //                internal state.
  // sent_trailing_metadata_op allows the transport to fill in to the upper
  // layer whether this stream was able to send its trailing metadata (used for
  // detecting cancellation on the server-side)..
  bool* sent_trailing_metadata_op = nullptr;
  grpc_closure* send_trailing_metadata_finished = nullptr;

  int64_t next_message_end_offset;
  int64_t flow_controlled_bytes_written = 0;
  int64_t flow_controlled_bytes_flowed = 0;
  grpc_closure* send_message_finished = nullptr;

  grpc_metadata_batch* recv_initial_metadata;
  grpc_closure* recv_initial_metadata_ready = nullptr;
  bool* trailing_metadata_available = nullptr;
  std::optional<grpc_core::SliceBuffer>* recv_message = nullptr;
  uint32_t* recv_message_flags = nullptr;
  bool* call_failed_before_recv_message = nullptr;
  grpc_closure* recv_message_ready = nullptr;
  grpc_metadata_batch* recv_trailing_metadata;
  grpc_closure* recv_trailing_metadata_finished = nullptr;

  grpc_transport_stream_stats* collecting_stats = nullptr;
  grpc_transport_stream_stats stats = grpc_transport_stream_stats();

  /// Is this stream closed for writing.
  bool write_closed = false;
  /// Is this stream reading half-closed.
  bool read_closed = false;
  /// Are all published incoming byte streams closed.
  bool all_incoming_byte_streams_finished = false;
  /// Has this stream seen an error.
  /// If true, then pending incoming frames can be thrown away.
  bool seen_error = false;
  /// Are we buffering writes on this stream? If yes, we won't become writable
  /// until there's enough queued up in the flow_controlled_buffer
  bool write_buffering = false;

  // have we sent or received the EOS bit?
  bool eos_received = false;
  bool eos_sent = false;

  grpc_core::BitSet<STREAM_LIST_COUNT> included;

  /// the error that resulted in this stream being read-closed
  grpc_error_handle read_closed_error;
  /// the error that resulted in this stream being write-closed
  grpc_error_handle write_closed_error;

  grpc_published_metadata_method published_metadata[2] = {};

  grpc_metadata_batch initial_metadata_buffer;
  grpc_metadata_batch trailing_metadata_buffer;

  grpc_slice_buffer frame_storage;  // protected by t combiner

  grpc_core::Timestamp deadline = grpc_core::Timestamp::InfFuture();

  /// number of bytes received - reset at end of parse thread execution
  int64_t received_bytes = 0;

  grpc_core::chttp2::StreamFlowControl flow_control;

  grpc_slice_buffer flow_controlled_buffer;

  grpc_chttp2_write_cb* on_flow_controlled_cbs = nullptr;
  grpc_chttp2_write_cb* on_write_finished_cbs = nullptr;
  grpc_chttp2_write_cb* finish_after_write = nullptr;
  size_t sending_bytes = 0;

  /// Byte counter for number of bytes written
  size_t byte_counter = 0;

  /// Number of times written
  int64_t write_counter = 0;

  grpc_core::Chttp2CallTracerWrapper call_tracer_wrapper;
  // null by default, set by the transport data source upon first query
  grpc_core::RefCountedPtr<grpc_core::channelz::CallNode> channelz_call_node;

  // TODO(yashykt): Remove call_tracer field after transition to call v3. (See
  // https://github.com/grpc/grpc/pull/38729 for more information.)
  // In the transport, we use tracers for two things, recording byte stats and
  // recording annotations. Recording byte stats is safe as long as call_tracer
  // is non null. Recording annotations on the other hand is only safe after
  // send_initial_metadata. On the client, this is not an issue since that is
  // the first operation. On the server, we would ideally be able to record
  // annotations as soon as we have parsed initial metadata, but in our legacy
  // stack, we create the stream before parsing headers. In the new v3 stack,
  // that won't be an issue.
  grpc_core::CallTracerInterface* call_tracer = nullptr;
  // TODO(yashykt): Remove this once call_tracer_transport_fix is rolled out
  grpc_core::CallTracerAnnotationInterface* parent_call_tracer = nullptr;

  // time this stream was created
  gpr_timespec creation_time = gpr_now(GPR_CLOCK_MONOTONIC);

  bool parsed_trailers_only = false;

  bool final_metadata_requested = false;
  bool received_last_frame = false;  // protected by t combiner

  /// how many header frames have we received?
  uint8_t header_frames_received = 0;

  bool sent_initial_metadata = false;
  bool sent_trailing_metadata = false;

  /// Whether the bytes needs to be traced using Fathom
  bool traced = false;

  // The last time a stream window update was received.
  grpc_core::Timestamp last_window_update_time =
      grpc_core::Timestamp::InfPast();
};

#define GRPC_ARG_PING_TIMEOUT_MS "grpc.http2.ping_timeout_ms"

// EXPERIMENTAL: provide protection against overloading a server with too many
// requests: wait for streams to be deallocated before they stop counting
// against MAX_CONCURRENT_STREAMS
#define GRPC_ARG_MAX_CONCURRENT_STREAMS_OVERLOAD_PROTECTION \
  "grpc.http.overload_protection"

// EXPERIMENTAL: Fail requests at the client if the client is over max
// concurrent streams, so they may be retried elsewhere.
#define GRPC_ARG_MAX_CONCURRENT_STREAMS_REJECT_ON_CLIENT \
  "grpc.http.max_concurrent_streams_reject_on_client"

/// Transport writing call flow:
/// grpc_chttp2_initiate_write() is called anywhere that we know bytes need to
/// go out on the wire.
/// If no other write has been started, a task is enqueued onto our workqueue.
/// When that task executes, it obtains the global lock, and gathers the data
/// to write.
/// The global lock is dropped and we do the syscall to write.
/// After writing, a follow-up check is made to see if another round of writing
/// should be performed.

/// The actual call chain is documented in the implementation of this function.
///
void grpc_chttp2_initiate_write(grpc_chttp2_transport* t,
                                grpc_chttp2_initiate_write_reason reason);

struct grpc_chttp2_begin_write_result {
  /// are we writing?
  bool writing;
  /// if writing: was it a complete flush (false) or a partial flush (true)
  bool partial;
  /// did we queue any completions as part of beginning the write
  bool early_results_scheduled;
};
grpc_chttp2_begin_write_result grpc_chttp2_begin_write(
    grpc_chttp2_transport* t);
void grpc_chttp2_end_write(grpc_chttp2_transport* t, grpc_error_handle error);

/// Process one slice of incoming data
/// Returns:
///  - a count of parsed bytes in the event of a partial read: the caller should
///    offload responsibilities to another thread to continue parsing.
///  - or a status in the case of a completed read
std::variant<size_t, absl::Status> grpc_chttp2_perform_read(
    grpc_chttp2_transport* t, const grpc_slice& slice,
    size_t& requests_started);

//******** Flow Control **************

// Takes in a flow control action and performs all the needed operations.
void grpc_chttp2_act_on_flowctl_action(
    const grpc_core::chttp2::FlowControlAction& action,
    grpc_chttp2_transport* t, grpc_chttp2_stream* s);

//******** End of Flow Control **************

inline grpc_chttp2_stream* grpc_chttp2_parsing_lookup_stream(
    grpc_chttp2_transport* t, uint32_t id) {
  auto it = t->stream_map.find(id);
  if (it == t->stream_map.end()) return nullptr;
  return it->second;
}
grpc_chttp2_stream* grpc_chttp2_parsing_accept_stream(grpc_chttp2_transport* t,
                                                      uint32_t id);

void grpc_chttp2_add_incoming_goaway(grpc_chttp2_transport* t,
                                     uint32_t goaway_error,
                                     uint32_t last_stream_id,
                                     absl::string_view goaway_text);

void grpc_chttp2_parsing_become_skip_parser(grpc_chttp2_transport* t);

void grpc_chttp2_complete_closure_step(grpc_chttp2_transport* t,
                                       grpc_closure** pclosure,
                                       grpc_error_handle error,
                                       const char* desc,
                                       grpc_core::DebugLocation whence = {});

void grpc_chttp2_keepalive_timeout(
    grpc_core::RefCountedPtr<grpc_chttp2_transport> t);
void grpc_chttp2_ping_timeout(
    grpc_core::RefCountedPtr<grpc_chttp2_transport> t);

void grpc_chttp2_settings_timeout(
    grpc_core::RefCountedPtr<grpc_chttp2_transport> t);

#define GRPC_HEADER_SIZE_IN_BYTES 5
#define MAX_SIZE_T (~(size_t)0)

#define GRPC_CHTTP2_CLIENT_CONNECT_STRING "PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n"
#define GRPC_CHTTP2_CLIENT_CONNECT_STRLEN \
  (sizeof(GRPC_CHTTP2_CLIENT_CONNECT_STRING) - 1)

#define GRPC_CHTTP2_IF_TRACING(severity) \
  LOG_IF(severity, GRPC_TRACE_FLAG_ENABLED(http))

void grpc_chttp2_fake_status(grpc_chttp2_transport* t,
                             grpc_chttp2_stream* stream,
                             grpc_error_handle error);
grpc_chttp2_transport::RemovedStreamHandle grpc_chttp2_mark_stream_closed(
    grpc_chttp2_transport* t, grpc_chttp2_stream* s, int close_reads,
    int close_writes, grpc_error_handle error);
void grpc_chttp2_start_writing(grpc_chttp2_transport* t);

#ifndef NDEBUG
#define GRPC_CHTTP2_STREAM_REF(stream, reason) \
  grpc_chttp2_stream_ref(stream, reason)
#define GRPC_CHTTP2_STREAM_UNREF(stream, reason) \
  grpc_chttp2_stream_unref(stream, reason)
void grpc_chttp2_stream_ref(grpc_chttp2_stream* s, const char* reason);
void grpc_chttp2_stream_unref(grpc_chttp2_stream* s, const char* reason);
#else
#define GRPC_CHTTP2_STREAM_REF(stream, reason) grpc_chttp2_stream_ref(stream)
#define GRPC_CHTTP2_STREAM_UNREF(stream, reason) \
  grpc_chttp2_stream_unref(stream)
void grpc_chttp2_stream_ref(grpc_chttp2_stream* s);
void grpc_chttp2_stream_unref(grpc_chttp2_stream* s);
#endif

void grpc_chttp2_ack_ping(grpc_chttp2_transport* t, uint64_t id);

/// Sends GOAWAY with error code ENHANCE_YOUR_CALM and additional debug data
/// resembling "too_many_pings" followed by immediately closing the connection.
void grpc_chttp2_exceeded_ping_strikes(grpc_chttp2_transport* t);

/// Resets ping clock. Should be called when flushing window updates,
/// initial/trailing metadata or data frames. For a server, it resets the number
/// of ping strikes and the last_ping_recv_time. For a ping sender, it resets
/// pings_before_data_required.
void grpc_chttp2_reset_ping_clock(grpc_chttp2_transport* t);

/// add a ref to the stream and add it to the writable list;
/// ref will be dropped in writing.c
void grpc_chttp2_mark_stream_writable(grpc_chttp2_transport* t,
                                      grpc_chttp2_stream* s);

void grpc_chttp2_cancel_stream(grpc_chttp2_transport* t, grpc_chttp2_stream* s,
                               grpc_error_handle due_to_error, bool tarpit);

void grpc_chttp2_maybe_complete_recv_initial_metadata(grpc_chttp2_transport* t,
                                                      grpc_chttp2_stream* s);
void grpc_chttp2_maybe_complete_recv_message(grpc_chttp2_transport* t,
                                             grpc_chttp2_stream* s);
void grpc_chttp2_maybe_complete_recv_trailing_metadata(grpc_chttp2_transport* t,
                                                       grpc_chttp2_stream* s);

void grpc_chttp2_fail_pending_writes(grpc_chttp2_transport* t,
                                     grpc_chttp2_stream* s,
                                     grpc_error_handle error);

/// Set the default keepalive configurations, must only be called at
/// initialization
void grpc_chttp2_config_default_keepalive_args(grpc_channel_args* args,
                                               bool is_client);
void grpc_chttp2_config_default_keepalive_args(
    const grpc_core::ChannelArgs& channel_args, bool is_client);

void grpc_chttp2_retry_initiate_ping(
    grpc_core::RefCountedPtr<grpc_chttp2_transport> t);

void schedule_bdp_ping_locked(
    grpc_core::RefCountedPtr<grpc_chttp2_transport> t);

uint32_t grpc_chttp2_min_read_progress_size(grpc_chttp2_transport* t);

#endif  // GRPC_SRC_CORE_EXT_TRANSPORT_CHTTP2_TRANSPORT_INTERNAL_H
