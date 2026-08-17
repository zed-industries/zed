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

#ifndef GRPCPP_SUPPORT_CHANNEL_ARGUMENTS_H
#define GRPCPP_SUPPORT_CHANNEL_ARGUMENTS_H

#include <grpc/compression.h>
#include <grpc/grpc.h>
#include <grpcpp/resource_quota.h>
#include <grpcpp/support/config.h>

#include <list>
#include <vector>

namespace grpc {
class ChannelCredentials;
namespace testing {
class ChannelArgumentsTest;
}  // namespace testing

/// Options for channel creation. The user can use generic setters to pass
/// key value pairs down to C channel creation code. For gRPC related options,
/// concrete setters are provided.
class ChannelArguments {
 public:
  ChannelArguments();
  ~ChannelArguments();

  ChannelArguments(const ChannelArguments& other);
  ChannelArguments& operator=(ChannelArguments other) {
    Swap(other);
    return *this;
  }

  void Swap(ChannelArguments& other);

  /// Dump arguments in this instance to \a channel_args. Does not take
  /// ownership of \a channel_args.
  ///
  /// Note that the underlying arguments are shared. Changes made to either \a
  /// channel_args or this instance would be reflected on both.
  void SetChannelArgs(grpc_channel_args* channel_args) const;

  // gRPC specific channel argument setters
  /// Set target name override for SSL host name checking. This option should
  /// be used with caution in production.
  void SetSslTargetNameOverride(const std::string& name);
  // TODO(yangg) add flow control options
  /// Set the compression algorithm for the channel.
  void SetCompressionAlgorithm(grpc_compression_algorithm algorithm);

  /// Set the grpclb fallback timeout (in ms) for the channel. If this amount
  /// of time has passed but we have not gotten any non-empty \a serverlist from
  /// the balancer, we will fall back to use the backend address(es) returned by
  /// the resolver.
  void SetGrpclbFallbackTimeout(int fallback_timeout);

  /// Set a mutator for the underlying socket.
  void SetSocketMutator(grpc_socket_mutator* mutator);

  /// Set the string to prepend to the user agent.
  void SetUserAgentPrefix(const std::string& user_agent_prefix);

  /// Set the buffer pool to be attached to the constructed channel.
  void SetResourceQuota(const grpc::ResourceQuota& resource_quota);

  /// Set the max receive and send message sizes.
  void SetMaxReceiveMessageSize(int size);
  void SetMaxSendMessageSize(int size);

  /// Set LB policy name.
  /// Note that if the name resolver returns only balancer addresses, the
  /// grpclb LB policy will be used, regardless of what is specified here.
  void SetLoadBalancingPolicyName(const std::string& lb_policy_name);

  /// Set service config in JSON form.
  /// Primarily meant for use in unit tests.
  void SetServiceConfigJSON(const std::string& service_config_json);

  // Generic channel argument setter. Only for advanced use cases.
  /// Set an integer argument \a value under \a key.
  void SetInt(const std::string& key, int value);

  // Generic channel argument setter. Only for advanced use cases.
  /// Set a pointer argument \a value under \a key. Ownership is not
  /// transferred.
  void SetPointer(const std::string& key, void* value);

  /// Set a pointer argument \a value under \a key, transferring ownership of
  /// \a value to the \a ChannelArguments object. The \a vtable::Delete function
  /// is responsible for \a value cleanup/destruction when called.
  void SetPointerWithVtable(const std::string& key, void* value,
                            const grpc_arg_pointer_vtable* vtable);

  /// Set a textual argument \a value under \a key.
  void SetString(const std::string& key, const std::string& value);

  /// Return (by value) a C \a grpc_channel_args structure which points to
  /// arguments owned by this \a ChannelArguments instance
  grpc_channel_args c_channel_args() const {
    grpc_channel_args out;
    out.num_args = args_.size();
    out.args = args_.empty() ? nullptr : const_cast<grpc_arg*>(&args_[0]);
    return out;
  }

 private:
  friend class grpc::ChannelCredentials;
  friend class grpc::testing::ChannelArgumentsTest;

  /// Default pointer argument operations.
  struct PointerVtableMembers {
    static void* Copy(void* in) { return in; }
    static void Destroy(void* /*in*/) {}
    static int Compare(void* a, void* b) {
      if (a < b) return -1;
      if (a > b) return 1;
      return 0;
    }
  };

  // Returns empty string when it is not set.
  std::string GetSslTargetNameOverride() const;

  std::vector<grpc_arg> args_;
  std::list<std::string> strings_;
};

}  // namespace grpc

#endif  // GRPCPP_SUPPORT_CHANNEL_ARGUMENTS_H
