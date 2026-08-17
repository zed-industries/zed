//
//
// Copyright 2018 gRPC authors.
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

#ifndef GRPC_SRC_CORE_CHANNELZ_CHANNELZ_H
#define GRPC_SRC_CORE_CHANNELZ_CHANNELZ_H

#include <grpc/grpc.h>
#include <grpc/impl/connectivity_state.h>
#include <grpc/slice.h>
#include <grpc/support/port_platform.h>
#include <stddef.h>

#include <atomic>
#include <cstdint>
#include <initializer_list>
#include <map>
#include <optional>
#include <set>
#include <string>
#include <utility>

#include "absl/base/thread_annotations.h"
#include "absl/container/flat_hash_set.h"
#include "absl/container/inlined_vector.h"
#include "absl/strings/string_view.h"
#include "src/core/channelz/channel_trace.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/util/json/json.h"
#include "src/core/util/per_cpu.h"
#include "src/core/util/ref_counted.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/string.h"
#include "src/core/util/sync.h"
#include "src/core/util/time.h"
#include "src/core/util/time_precise.h"
#include "src/core/util/useful.h"

// Channel arg key for channelz node.
#define GRPC_ARG_CHANNELZ_CHANNEL_NODE \
  "grpc.internal.no_subchannel.channelz_channel_node"

// Channel arg key for the containing base node
#define GRPC_ARG_CHANNELZ_CONTAINING_BASE_NODE \
  "grpc.internal.no_subchannel.channelz_containing_base_node"

// Channel arg key for indicating an internal channel.
#define GRPC_ARG_CHANNELZ_IS_INTERNAL_CHANNEL \
  "grpc.channelz_is_internal_channel"

/// This is the default value for whether or not to enable channelz. If
/// GRPC_ARG_ENABLE_CHANNELZ is set, it will override this default value.
#define GRPC_ENABLE_CHANNELZ_DEFAULT true

/// This is the default value for the maximum amount of memory used by trace
/// events per channel trace node. If
/// GRPC_ARG_MAX_CHANNEL_TRACE_EVENT_MEMORY_PER_NODE is set, it will override
/// this default value.
#define GRPC_MAX_CHANNEL_TRACE_EVENT_MEMORY_PER_NODE_DEFAULT (1024 * 4)

namespace grpc_core {

namespace channelz {

class SocketNode;
class ListenSocketNode;
class DataSource;
class ZTrace;

namespace testing {
class CallCountingHelperPeer;
class SubchannelNodePeer;
}  // namespace testing

// base class for all channelz entities
class BaseNode : public RefCounted<BaseNode> {
 public:
  // There are only four high level channelz entities. However, to support
  // GetTopChannelsRequest, we split the Channel entity into two different
  // types. All children of BaseNode must be one of these types.
  enum class EntityType {
    kTopLevelChannel,
    kInternalChannel,
    kSubchannel,
    kServer,
    kListenSocket,
    kSocket,
    kCall,
  };

  static absl::string_view EntityTypeString(EntityType type) {
    switch (type) {
      case EntityType::kTopLevelChannel:
        return "top_level_channel";
      case EntityType::kInternalChannel:
        return "internal_channel";
      case EntityType::kSubchannel:
        return "subchannel";
      case EntityType::kServer:
        return "server";
      case EntityType::kListenSocket:
        return "listen_socket";
      case EntityType::kSocket:
        return "socket";
      case EntityType::kCall:
        return "call";
    }
    return "unknown";
  }

 protected:
  BaseNode(EntityType type, std::string name);

 public:
  ~BaseNode() override;

  bool HasParent(const BaseNode* parent) const {
    MutexLock lock(&parent_mu_);
    return parents_.find(parent) != parents_.end();
  }

  void AddParent(BaseNode* parent) {
    MutexLock lock(&parent_mu_);
    parents_.insert(parent->Ref());
  }

  void RemoveParent(BaseNode* parent) {
    MutexLock lock(&parent_mu_);
    parents_.erase(parent);
  }

  static absl::string_view ChannelArgName() {
    return GRPC_ARG_CHANNELZ_CONTAINING_BASE_NODE;
  }
  static int ChannelArgsCompare(const BaseNode* a, const BaseNode* b) {
    return QsortCompare(a, b);
  }

  // All children must implement this function.
  virtual Json RenderJson() = 0;

  // Renders the json and returns allocated string that must be freed by the
  // caller.
  std::string RenderJsonString();

  EntityType type() const { return type_; }
  intptr_t uuid() {
    const intptr_t id = uuid_.load(std::memory_order_relaxed);
    if (id > 0) return id;
    return UuidSlow();
  }
  const std::string& name() const { return name_; }

  void RunZTrace(absl::string_view name, Timestamp deadline,
                 std::map<std::string, std::string> args,
                 std::shared_ptr<grpc_event_engine::experimental::EventEngine>
                     event_engine,
                 absl::AnyInvocable<void(Json output)> callback);
  Json::Object AdditionalInfo();

 protected:
  void PopulateJsonFromDataSources(Json::Object& json);

 private:
  // to allow the ChannelzRegistry to set uuid_ under its lock.
  friend class ChannelzRegistry;
  // allow data source to register/unregister itself
  friend class DataSource;
  using ParentSet =
      absl::flat_hash_set<RefCountedPtr<BaseNode>, RefCountedPtrHash<BaseNode>,
                          RefCountedPtrEq<BaseNode>>;

  intptr_t UuidSlow();

  const EntityType type_;
  std::atomic<intptr_t> uuid_;
  std::string name_;
  Mutex data_sources_mu_;
  absl::InlinedVector<DataSource*, 3> data_sources_
      ABSL_GUARDED_BY(data_sources_mu_);
  BaseNode* prev_;
  BaseNode* next_;
  mutable Mutex parent_mu_;
  ParentSet parents_ ABSL_GUARDED_BY(parent_mu_);
};

class ZTrace {
 public:
  virtual ~ZTrace() = default;
  virtual void Run(Timestamp deadline, std::map<std::string, std::string> args,
                   std::shared_ptr<grpc_event_engine::experimental::EventEngine>
                       event_engine,
                   absl::AnyInvocable<void(Json)>) = 0;
};

class DataSink {
 public:
  virtual void AddAdditionalInfo(absl::string_view name,
                                 Json::Object additional_info) = 0;
  virtual void AddChildObjects(
      std::vector<RefCountedPtr<BaseNode>> children) = 0;

 protected:
  ~DataSink() = default;
};

class DataSource {
 public:
  explicit DataSource(RefCountedPtr<BaseNode> node);

  // Add any relevant json fragments to the output.
  // This method must not cause the DataSource to be deleted, or else there will
  // be a deadlock.
  virtual void AddData(DataSink&) {}

  // If this data source exports some ztrace, return it here.
  virtual std::unique_ptr<ZTrace> GetZTrace(absl::string_view /*name*/) {
    return nullptr;
  }

 protected:
  ~DataSource();
  RefCountedPtr<BaseNode> channelz_node() { return node_; }

  void ResetDataSource();

 private:
  RefCountedPtr<BaseNode> node_;
};

struct CallCounts {
  int64_t calls_started = 0;
  int64_t calls_succeeded = 0;
  int64_t calls_failed = 0;
  gpr_cycle_counter last_call_started_cycle = 0;

  std::string last_call_started_timestamp() const {
    return gpr_format_timespec(
        gpr_cycle_counter_to_time(last_call_started_cycle));
  }

  void PopulateJson(Json::Object& json) const;
};

// This class is a helper class for channelz entities that deal with Channels,
// Subchannels, and Servers, since those have similar proto definitions.
// This class has the ability to:
//   - track calls_{started,succeeded,failed}
//   - track last_call_started_timestamp
//   - perform rendering of the above items
class CallCountingHelper final {
 public:
  void RecordCallStarted();
  void RecordCallFailed();
  void RecordCallSucceeded();

  CallCounts GetCallCounts() const {
    return {
        calls_started_.load(std::memory_order_relaxed),
        calls_succeeded_.load(std::memory_order_relaxed),
        calls_failed_.load(std::memory_order_relaxed),
        last_call_started_cycle_.load(std::memory_order_relaxed),
    };
  }

 private:
  // testing peer friend.
  friend class testing::CallCountingHelperPeer;

  std::atomic<int64_t> calls_started_{0};
  std::atomic<int64_t> calls_succeeded_{0};
  std::atomic<int64_t> calls_failed_{0};
  std::atomic<gpr_cycle_counter> last_call_started_cycle_{0};
};

class PerCpuCallCountingHelper final {
 public:
  void RecordCallStarted();
  void RecordCallFailed();
  void RecordCallSucceeded();

  CallCounts GetCallCounts() const;

 private:
  // testing peer friend.
  friend class testing::CallCountingHelperPeer;

  // We want to ensure that this per-cpu data structure lands on different
  // cachelines per cpu.
  struct alignas(GPR_CACHELINE_SIZE) PerCpuData {
    std::atomic<int64_t> calls_started{0};
    std::atomic<int64_t> calls_succeeded{0};
    std::atomic<int64_t> calls_failed{0};
    std::atomic<gpr_cycle_counter> last_call_started_cycle{0};
  };
  PerCpu<PerCpuData> per_cpu_data_{PerCpuOptions().SetCpusPerShard(4)};
};

// Handles channelz bookkeeping for channels
class ChannelNode final : public BaseNode {
 public:
  ChannelNode(std::string target, size_t channel_tracer_max_nodes,
              bool is_internal_channel);

  static absl::string_view ChannelArgName() {
    return GRPC_ARG_CHANNELZ_CHANNEL_NODE;
  }
  static int ChannelArgsCompare(const ChannelNode* a, const ChannelNode* b) {
    return QsortCompare(a, b);
  }

  // Returns the string description of the given connectivity state.
  static const char* GetChannelConnectivityStateChangeString(
      grpc_connectivity_state state);

  Json RenderJson() override;

  // proxy methods to composed classes.
  void AddTraceEvent(ChannelTrace::Severity severity, const grpc_slice& data) {
    trace_.AddTraceEvent(severity, data);
  }
  void AddTraceEventWithReference(ChannelTrace::Severity severity,
                                  const grpc_slice& data,
                                  RefCountedPtr<BaseNode> referenced_channel) {
    trace_.AddTraceEventWithReference(severity, data,
                                      std::move(referenced_channel));
  }
  void SetChannelArgs(const ChannelArgs& channel_args) {
    channel_args_ = channel_args;
  }
  void RecordCallStarted() { call_counter_.RecordCallStarted(); }
  void RecordCallFailed() { call_counter_.RecordCallFailed(); }
  void RecordCallSucceeded() { call_counter_.RecordCallSucceeded(); }

  void SetConnectivityState(grpc_connectivity_state state);

  const std::string& target() const { return target_; }
  std::optional<std::string> connectivity_state();
  CallCounts GetCallCounts() const { return call_counter_.GetCallCounts(); }
  std::set<intptr_t> child_channels() const;
  std::set<intptr_t> child_subchannels() const;
  const ChannelTrace& trace() const { return trace_; }
  const ChannelArgs& channel_args() const { return channel_args_; }

 private:
  void PopulateChildRefs(Json::Object* json);

  std::string target_;
  CallCountingHelper call_counter_;
  ChannelTrace trace_;
  ChannelArgs channel_args_;

  // Least significant bit indicates whether the value is set.  Remaining
  // bits are a grpc_connectivity_state value.
  std::atomic<int> connectivity_state_{0};
};

// Handles channelz bookkeeping for subchannels
class SubchannelNode final : public BaseNode {
 public:
  SubchannelNode(std::string target_address, size_t channel_tracer_max_nodes);
  ~SubchannelNode() override;

  // Sets the subchannel's connectivity state without health checking.
  void UpdateConnectivityState(grpc_connectivity_state state);

  // Used when the subchannel's child socket changes. This should be set when
  // the subchannel's transport is created and set to nullptr when the
  // subchannel unrefs the transport.
  void SetChildSocket(RefCountedPtr<SocketNode> socket);

  Json RenderJson() override;

  // proxy methods to composed classes.
  void AddTraceEvent(ChannelTrace::Severity severity, const grpc_slice& data) {
    trace_.AddTraceEvent(severity, data);
  }
  void SetChannelArgs(const ChannelArgs& channel_args) {
    channel_args_ = channel_args;
  }
  void AddTraceEventWithReference(ChannelTrace::Severity severity,
                                  const grpc_slice& data,
                                  RefCountedPtr<BaseNode> referenced_channel) {
    trace_.AddTraceEventWithReference(severity, data,
                                      std::move(referenced_channel));
  }
  void RecordCallStarted() { call_counter_.RecordCallStarted(); }
  void RecordCallFailed() { call_counter_.RecordCallFailed(); }
  void RecordCallSucceeded() { call_counter_.RecordCallSucceeded(); }

  const std::string& target() const { return target_; }
  std::string connectivity_state() const;
  CallCounts GetCallCounts() const { return call_counter_.GetCallCounts(); }
  RefCountedPtr<SocketNode> child_socket() const {
    MutexLock lock(&socket_mu_);
    return child_socket_;
  }
  const ChannelTrace& trace() const { return trace_; }
  const ChannelArgs& channel_args() const { return channel_args_; }

 private:
  // Allows the channel trace test to access trace_.
  friend class testing::SubchannelNodePeer;

  std::atomic<grpc_connectivity_state> connectivity_state_{GRPC_CHANNEL_IDLE};
  mutable Mutex socket_mu_;
  RefCountedPtr<SocketNode> child_socket_ ABSL_GUARDED_BY(socket_mu_);
  std::string target_;
  CallCountingHelper call_counter_;
  ChannelTrace trace_;
  ChannelArgs channel_args_;
};

// Handles channelz bookkeeping for servers
class ServerNode final : public BaseNode {
 public:
  explicit ServerNode(size_t channel_tracer_max_nodes);

  ~ServerNode() override;

  Json RenderJson() override;

  std::string RenderServerSockets(intptr_t start_socket_id,
                                  intptr_t max_results);

  // proxy methods to composed classes.
  void AddTraceEvent(ChannelTrace::Severity severity, const grpc_slice& data) {
    trace_.AddTraceEvent(severity, data);
  }
  void AddTraceEventWithReference(ChannelTrace::Severity severity,
                                  const grpc_slice& data,
                                  RefCountedPtr<BaseNode> referenced_channel) {
    trace_.AddTraceEventWithReference(severity, data,
                                      std::move(referenced_channel));
  }
  void SetChannelArgs(const ChannelArgs& channel_args) {
    channel_args_ = channel_args;
  }
  void RecordCallStarted() { call_counter_.RecordCallStarted(); }
  void RecordCallFailed() { call_counter_.RecordCallFailed(); }
  void RecordCallSucceeded() { call_counter_.RecordCallSucceeded(); }

  CallCounts GetCallCounts() const { return call_counter_.GetCallCounts(); }

  std::map<intptr_t, RefCountedPtr<ListenSocketNode>> child_listen_sockets()
      const;
  std::map<intptr_t, RefCountedPtr<SocketNode>> child_sockets() const;

  const ChannelTrace& trace() const { return trace_; }
  const ChannelArgs& channel_args() const { return channel_args_; }

 private:
  PerCpuCallCountingHelper call_counter_;
  ChannelTrace trace_;
  ChannelArgs channel_args_;
};

#define GRPC_ARG_CHANNELZ_SECURITY "grpc.internal.channelz_security"

// Handles channelz bookkeeping for sockets
class SocketNode final : public BaseNode {
 public:
  struct Security : public RefCounted<Security> {
    struct Tls {
      // This is a workaround for https://bugs.llvm.org/show_bug.cgi?id=50346
      Tls() {}

      enum class NameType { kUnset = 0, kStandardName = 1, kOtherName = 2 };
      NameType type = NameType::kUnset;
      // Holds the value of standard_name or other_names if type is not kUnset.
      std::string name;
      std::string local_certificate;
      std::string remote_certificate;

      Json RenderJson();
    };
    enum class ModelType { kUnset = 0, kTls = 1, kOther = 2 };
    ModelType type = ModelType::kUnset;
    std::optional<Tls> tls;
    std::optional<Json> other;

    Json RenderJson();

    static absl::string_view ChannelArgName() {
      return GRPC_ARG_CHANNELZ_SECURITY;
    }

    static int ChannelArgsCompare(const Security* a, const Security* b) {
      return QsortCompare(a, b);
    }

    grpc_arg MakeChannelArg() const;

    static RefCountedPtr<Security> GetFromChannelArgs(
        const grpc_channel_args* args);
  };

  SocketNode(std::string local, std::string remote, std::string name,
             RefCountedPtr<Security> security);
  ~SocketNode() override {}

  Json RenderJson() override;

  void RecordStreamStartedFromLocal();
  void RecordStreamStartedFromRemote();
  void RecordStreamSucceeded() {
    streams_succeeded_.fetch_add(1, std::memory_order_relaxed);
  }
  void RecordStreamFailed() {
    streams_failed_.fetch_add(1, std::memory_order_relaxed);
  }
  void RecordMessagesSent(uint32_t num_sent);
  void RecordMessageReceived();
  void RecordKeepaliveSent() {
    keepalives_sent_.fetch_add(1, std::memory_order_relaxed);
  }

  const std::string& remote() { return remote_; }

  int64_t streams_started() const {
    return streams_started_.load(std::memory_order_relaxed);
  }
  int64_t streams_succeeded() const {
    return streams_succeeded_.load(std::memory_order_relaxed);
  }
  int64_t streams_failed() const {
    return streams_failed_.load(std::memory_order_relaxed);
  }
  int64_t messages_sent() const {
    return messages_sent_.load(std::memory_order_relaxed);
  }
  int64_t messages_received() const {
    return messages_received_.load(std::memory_order_relaxed);
  }
  int64_t keepalives_sent() const {
    return keepalives_sent_.load(std::memory_order_relaxed);
  }
  auto last_local_stream_created_timestamp() const {
    return CycleCounterToTimestamp(
        last_local_stream_created_cycle_.load(std::memory_order_relaxed));
  }
  auto last_remote_stream_created_timestamp() const {
    return CycleCounterToTimestamp(
        last_remote_stream_created_cycle_.load(std::memory_order_relaxed));
  }
  auto last_message_sent_timestamp() const {
    return CycleCounterToTimestamp(
        last_message_sent_cycle_.load(std::memory_order_relaxed));
  }
  auto last_message_received_timestamp() const {
    return CycleCounterToTimestamp(
        last_message_received_cycle_.load(std::memory_order_relaxed));
  }
  const std::string& local() const { return local_; }
  const std::string& remote() const { return remote_; }

 private:
  std::optional<std::string> CycleCounterToTimestamp(
      gpr_cycle_counter cycle_counter) const {
    return gpr_format_timespec(gpr_cycle_counter_to_time(cycle_counter));
  }

  std::atomic<int64_t> streams_started_{0};
  std::atomic<int64_t> streams_succeeded_{0};
  std::atomic<int64_t> streams_failed_{0};
  std::atomic<int64_t> messages_sent_{0};
  std::atomic<int64_t> messages_received_{0};
  std::atomic<int64_t> keepalives_sent_{0};
  std::atomic<gpr_cycle_counter> last_local_stream_created_cycle_{0};
  std::atomic<gpr_cycle_counter> last_remote_stream_created_cycle_{0};
  std::atomic<gpr_cycle_counter> last_message_sent_cycle_{0};
  std::atomic<gpr_cycle_counter> last_message_received_cycle_{0};
  std::string local_;
  std::string remote_;
  RefCountedPtr<Security> const security_;
};

// Handles channelz bookkeeping for listen sockets
class ListenSocketNode final : public BaseNode {
 public:
  ListenSocketNode(std::string local_addr, std::string name);
  ~ListenSocketNode() override {}

  Json RenderJson() override;

 private:
  std::string local_addr_;
};

class CallNode final : public BaseNode {
 public:
  explicit CallNode(std::string name)
      : BaseNode(EntityType::kCall, std::move(name)) {}

  Json RenderJson() override;
};

}  // namespace channelz
}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_CHANNELZ_CHANNELZ_H
