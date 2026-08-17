//
//
// Copyright 2017 gRPC authors.
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

#ifndef GRPC_SRC_CORE_CHANNELZ_CHANNELZ_REGISTRY_H
#define GRPC_SRC_CORE_CHANNELZ_CHANNELZ_REGISTRY_H

#include <grpc/support/port_platform.h>

#include <cstdint>
#include <map>
#include <string>

#include "absl/container/btree_map.h"
#include "absl/functional/function_ref.h"
#include "src/core/channelz/channelz.h"
#include "src/core/util/json/json_writer.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/sync.h"

namespace grpc_core {
namespace channelz {

// singleton registry object to track all objects that are needed to support
// channelz bookkeeping. All objects share globally distributed uuids.
class ChannelzRegistry final {
 public:
  static void Register(BaseNode* node) {
    return Default()->InternalRegister(node);
  }
  static void Unregister(BaseNode* node) {
    Default()->InternalUnregister(node);
  }
  static RefCountedPtr<BaseNode> Get(intptr_t uuid) {
    return Default()->InternalGet(uuid);
  }
  static intptr_t NumberNode(BaseNode* node) {
    return Default()->InternalNumberNode(node);
  }

  static RefCountedPtr<SubchannelNode> GetSubchannel(intptr_t uuid) {
    return Default()
        ->InternalGetTyped<SubchannelNode, BaseNode::EntityType::kSubchannel>(
            uuid);
  }

  static RefCountedPtr<ChannelNode> GetChannel(intptr_t uuid) {
    auto node = Default()->InternalGet(uuid);
    if (node == nullptr) return nullptr;
    if (node->type() == BaseNode::EntityType::kTopLevelChannel) {
      return node->RefAsSubclass<ChannelNode>();
    }
    if (node->type() == BaseNode::EntityType::kInternalChannel) {
      return node->RefAsSubclass<ChannelNode>();
    }
    return nullptr;
  }

  static RefCountedPtr<ServerNode> GetServer(intptr_t uuid) {
    return Default()
        ->InternalGetTyped<ServerNode, BaseNode::EntityType::kServer>(uuid);
  }

  static RefCountedPtr<SocketNode> GetSocket(intptr_t uuid) {
    return Default()
        ->InternalGetTyped<SocketNode, BaseNode::EntityType::kSocket>(uuid);
  }

  // Returns the allocated JSON string that represents the proto
  // GetTopChannelsResponse as per channelz.proto.
  static auto GetTopChannels(intptr_t start_channel_id) {
    return Default()
        ->InternalGetObjects<ChannelNode,
                             BaseNode::EntityType::kTopLevelChannel>(
            start_channel_id);
  }

  static std::string GetTopChannelsJson(intptr_t start_channel_id);
  static std::string GetServersJson(intptr_t start_server_id);

  // Returns the allocated JSON string that represents the proto
  // GetServersResponse as per channelz.proto.
  static auto GetServers(intptr_t start_server_id) {
    return Default()
        ->InternalGetObjects<ServerNode, BaseNode::EntityType::kServer>(
            start_server_id);
  }

  static std::tuple<std::vector<RefCountedPtr<BaseNode>>, bool>
  GetChildrenOfType(intptr_t start_node, const BaseNode* parent,
                    BaseNode::EntityType type, size_t max_results) {
    return Default()->InternalGetChildrenOfType(start_node, parent, type,
                                                max_results);
  }

  // Test only helper function to dump the JSON representation to std out.
  // This can aid in debugging channelz code.
  static void LogAllEntities() { Default()->InternalLogAllEntities(); }

  static std::vector<RefCountedPtr<BaseNode>> GetAllEntities() {
    return Default()->InternalGetAllEntities();
  }

  // Test only helper function to reset to initial state.
  static void TestOnlyReset() {
    auto* p = Default();
    p->node_map_ = MakeNodeMap();
  }

 private:
  ChannelzRegistry() : node_map_(MakeNodeMap()) {}

  // Takes a callable F: (RefCountedPtr<BaseNode>) -> bool, and returns
  // a (BaseNode*) -> bool that filters unreffed objects and returns true.
  // The ref must be unreffed outside the NodeMapInterface iteration.
  template <typename F>
  static auto CollectReferences(F fn) {
    return [fn = std::move(fn)](BaseNode* n) {
      auto node = n->RefIfNonZero();
      if (node == nullptr) return true;
      return fn(std::move(node));
    };
  }

  class NodeMapInterface {
   public:
    virtual ~NodeMapInterface() = default;

    virtual void Register(BaseNode* node) = 0;
    virtual void Unregister(BaseNode* node) = 0;

    virtual std::tuple<std::vector<RefCountedPtr<BaseNode>>, bool> QueryNodes(
        intptr_t start_node, absl::FunctionRef<bool(const BaseNode*)> filter,
        size_t max_results) = 0;
    virtual RefCountedPtr<BaseNode> GetNode(intptr_t uuid) = 0;

    virtual intptr_t NumberNode(BaseNode* node) = 0;
  };

  class LegacyNodeMap final : public NodeMapInterface {
   public:
    void Register(BaseNode* node) override;
    void Unregister(BaseNode* node) override;

    std::tuple<std::vector<RefCountedPtr<BaseNode>>, bool> QueryNodes(
        intptr_t start_node, absl::FunctionRef<bool(const BaseNode*)> filter,
        size_t max_results) override;
    RefCountedPtr<BaseNode> GetNode(intptr_t uuid) override;

    intptr_t NumberNode(BaseNode* node) override { return node->uuid_; }

   private:
    Mutex mu_;
    std::map<intptr_t, BaseNode*> node_map_ ABSL_GUARDED_BY(mu_);
    intptr_t uuid_generator_ ABSL_GUARDED_BY(mu_) = 1;
  };

  class ShardedNodeMap final : public NodeMapInterface {
   public:
    void Register(BaseNode* node) override;
    void Unregister(BaseNode* node) override;

    std::tuple<std::vector<RefCountedPtr<BaseNode>>, bool> QueryNodes(
        intptr_t start_node, absl::FunctionRef<bool(const BaseNode*)> filter,
        size_t max_results) override;
    RefCountedPtr<BaseNode> GetNode(intptr_t uuid) override;

    intptr_t NumberNode(BaseNode* node) override;

   private:
    struct BaseNodeList {
      Mutex mu;
      BaseNode* nursery ABSL_GUARDED_BY(mu) = nullptr;
      BaseNode* numbered ABSL_GUARDED_BY(mu) = nullptr;
    };

    static size_t NodeShardIndex(BaseNode* node);
    static void AddNodeToHead(BaseNode* node, BaseNode*& head);
    static void RemoveNodeFromHead(BaseNode* node, BaseNode*& head);

    static constexpr size_t kNodeShards = 63;
    int64_t uuid_generator_{1};
    BaseNodeList node_list_[kNodeShards];
    Mutex index_mu_;
    absl::btree_map<intptr_t, BaseNode*> index_ ABSL_GUARDED_BY(index_mu_);
  };

  static std::unique_ptr<NodeMapInterface> MakeNodeMap();

  // Returned the singleton instance of ChannelzRegistry;
  static ChannelzRegistry* Default();

  // globally registers an Entry. Returns its unique uuid
  void InternalRegister(BaseNode* node) { node_map_->Register(node); }

  // globally unregisters the object that is associated to uuid. Also does
  // sanity check that an object doesn't try to unregister the wrong type.
  void InternalUnregister(BaseNode* node) { node_map_->Unregister(node); }

  intptr_t InternalNumberNode(BaseNode* node) {
    return node_map_->NumberNode(node);
  }

  // if object with uuid has previously been registered as the correct type,
  // returns the void* associated with that uuid. Else returns nullptr.
  RefCountedPtr<BaseNode> InternalGet(intptr_t uuid) {
    return node_map_->GetNode(uuid);
  }

  std::tuple<std::vector<RefCountedPtr<BaseNode>>, bool>
  InternalGetChildrenOfType(intptr_t start_node, const BaseNode* parent,
                            BaseNode::EntityType type, size_t max_results) {
    return node_map_->QueryNodes(
        start_node,
        [type, parent](const BaseNode* n) {
          return n->type() == type && n->HasParent(parent);
        },
        max_results);
  }

  template <typename T, BaseNode::EntityType entity_type>
  RefCountedPtr<T> InternalGetTyped(intptr_t uuid) {
    RefCountedPtr<BaseNode> node = InternalGet(uuid);
    if (node == nullptr || node->type() != entity_type) {
      return nullptr;
    }
    return node->RefAsSubclass<T>();
  }

  template <typename T, BaseNode::EntityType entity_type>
  std::tuple<std::vector<RefCountedPtr<T>>, bool> InternalGetObjects(
      intptr_t start_id) {
    const int kPaginationLimit = 100;
    std::vector<RefCountedPtr<T>> top_level_channels;
    const auto [nodes, end] = node_map_->QueryNodes(
        start_id,
        [](const BaseNode* node) { return node->type() == entity_type; },
        kPaginationLimit);
    for (const auto& p : nodes) {
      top_level_channels.emplace_back(p->template RefAsSubclass<T>());
    }
    return std::tuple(std::move(top_level_channels), end);
  }

  void InternalLogAllEntities();
  std::vector<RefCountedPtr<BaseNode>> InternalGetAllEntities();

  std::unique_ptr<NodeMapInterface> node_map_;
};

// `additionalInfo` section is not yet in the protobuf format, so we
// provide a utility to strip it for compatibility.
std::string StripAdditionalInfoFromJson(absl::string_view json);

}  // namespace channelz
}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_CHANNELZ_CHANNELZ_REGISTRY_H
