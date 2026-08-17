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
#include <map>
#include <optional>
#include <set>
#include <string>
#include <utility>

#include "absl/base/thread_annotations.h"
#include "absl/strings/string_view.h"
#include "src/core/channelz/channel_trace.h"
#include "src/core/util/json/json.h"
#include "src/core/util/per_cpu.h"
#include "src/core/util/ref_counted.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/sync.h"
#include "src/core/util/time_precise.h"
#include "src/core/util/useful.h"

// Channel arg key for channelz node.
#define GRPC_ARG_CHANNELZ_CHANNEL_NODE "grpc.internal.channelz_channel_node"

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
    kSocket,
  };

 protected:
  BaseNode(EntityType type, std::string name);

 public:
  ~BaseNode() override;

  // All children must implement this function.
  virtual Json RenderJson() = 0;

  // Renders the json and returns allocated string that must be freed by the
  // caller.
  std::string RenderJsonString();

  EntityType type() const { return type_; }
  intptr_t uuid() const { return uuid_; }
  const std::string& name() const { return name_; }

 private:
  // to allow the ChannelzRegistry to set uuid_ under its lock.
  friend class ChannelzRegistry;
  const EntityType type_;
  intptr_t uuid_;
  std::string name_;
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

  // Common rendering of the call count data and last_call_started_timestamp.
  void PopulateCallCounts(Json::Object* json);

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

  // Common rendering of the call count data and last_call_started_timestamp.
  void PopulateCallCounts(Json::Object* json);

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
  void RecordCallStarted() { call_counter_.RecordCallStarted(); }
  void RecordCallFailed() { call_counter_.RecordCallFailed(); }
  void RecordCallSucceeded() { call_counter_.RecordCallSucceeded(); }

  void SetConnectivityState(grpc_connectivity_state state);

  // TODO(roth): take in a RefCountedPtr to the child channel so we can retrieve
  // the human-readable name.
  void AddChildChannel(intptr_t child_uuid);
  void RemoveChildChannel(intptr_t child_uuid);

  // TODO(roth): take in a RefCountedPtr to the child subchannel so we can
  // retrieve the human-readable name.
  void AddChildSubchannel(intptr_t child_uuid);
  void RemoveChildSubchannel(intptr_t child_uuid);

 private:
  void PopulateChildRefs(Json::Object* json);

  std::string target_;
  CallCountingHelper call_counter_;
  ChannelTrace trace_;

  // Least significant bit indicates whether the value is set.  Remaining
  // bits are a grpc_connectivity_state value.
  std::atomic<int> connectivity_state_{0};

  Mutex child_mu_;  // Guards sets below.
  std::set<intptr_t> child_channels_;
  std::set<intptr_t> child_subchannels_;
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
  void AddTraceEventWithReference(ChannelTrace::Severity severity,
                                  const grpc_slice& data,
                                  RefCountedPtr<BaseNode> referenced_channel) {
    trace_.AddTraceEventWithReference(severity, data,
                                      std::move(referenced_channel));
  }
  void RecordCallStarted() { call_counter_.RecordCallStarted(); }
  void RecordCallFailed() { call_counter_.RecordCallFailed(); }
  void RecordCallSucceeded() { call_counter_.RecordCallSucceeded(); }

 private:
  // Allows the channel trace test to access trace_.
  friend class testing::SubchannelNodePeer;

  std::atomic<grpc_connectivity_state> connectivity_state_{GRPC_CHANNEL_IDLE};
  Mutex socket_mu_;
  RefCountedPtr<SocketNode> child_socket_ ABSL_GUARDED_BY(socket_mu_);
  std::string target_;
  CallCountingHelper call_counter_;
  ChannelTrace trace_;
};

// Handles channelz bookkeeping for servers
class ServerNode final : public BaseNode {
 public:
  explicit ServerNode(size_t channel_tracer_max_nodes);

  ~ServerNode() override;

  Json RenderJson() override;

  std::string RenderServerSockets(intptr_t start_socket_id,
                                  intptr_t max_results);

  void AddChildSocket(RefCountedPtr<SocketNode> node);

  void RemoveChildSocket(intptr_t child_uuid);

  void AddChildListenSocket(RefCountedPtr<ListenSocketNode> node);

  void RemoveChildListenSocket(intptr_t child_uuid);

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
  void RecordCallStarted() { call_counter_.RecordCallStarted(); }
  void RecordCallFailed() { call_counter_.RecordCallFailed(); }
  void RecordCallSucceeded() { call_counter_.RecordCallSucceeded(); }

 private:
  PerCpuCallCountingHelper call_counter_;
  ChannelTrace trace_;
  Mutex child_mu_;  // Guards child maps below.
  std::map<intptr_t, RefCountedPtr<SocketNode>> child_sockets_;
  std::map<intptr_t, RefCountedPtr<ListenSocketNode>> child_listen_sockets_;
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

 private:
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

}  // namespace channelz
}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_CHANNELZ_CHANNELZ_H
