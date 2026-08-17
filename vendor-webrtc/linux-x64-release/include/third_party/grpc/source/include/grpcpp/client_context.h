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

/// A ClientContext allows the person implementing a service client to:
///
/// - Add custom metadata key-value pairs that will propagated to the server
/// side.
/// - Control call settings such as compression and authentication.
/// - Initial and trailing metadata coming from the server.
/// - Get performance metrics (ie, census).
///
/// Context settings are only relevant to the call they are invoked with, that
/// is to say, they aren't sticky. Some of these settings, such as the
/// compression options, can be made persistent at channel construction time
/// (see \a grpc::CreateCustomChannel).
///
/// \warning ClientContext instances should \em not be reused across rpcs.

#ifndef GRPCPP_CLIENT_CONTEXT_H
#define GRPCPP_CLIENT_CONTEXT_H

#include <grpc/impl/compression_types.h>
#include <grpc/impl/propagation_bits.h>
#include <grpcpp/impl/create_auth_context.h>
#include <grpcpp/impl/metadata_map.h>
#include <grpcpp/impl/rpc_method.h>
#include <grpcpp/impl/sync.h>
#include <grpcpp/security/auth_context.h>
#include <grpcpp/support/client_interceptor.h>
#include <grpcpp/support/config.h>
#include <grpcpp/support/slice.h>
#include <grpcpp/support/status.h>
#include <grpcpp/support/string_ref.h>
#include <grpcpp/support/time.h>

#include <map>
#include <memory>
#include <string>

#include "absl/log/absl_check.h"

struct census_context;
struct grpc_call;

namespace grpc {
class ServerContext;
class ServerContextBase;
class CallbackServerContext;

namespace internal {
template <class InputMessage, class OutputMessage>
class CallbackUnaryCallImpl;
template <class Request, class Response>
class ClientCallbackReaderWriterImpl;
template <class Response>
class ClientCallbackReaderImpl;
template <class Request>
class ClientCallbackWriterImpl;
class ClientCallbackUnaryImpl;
class ClientContextAccessor;
class ClientAsyncResponseReaderHelper;
}  // namespace internal

template <class R>
class ClientReader;
template <class W>
class ClientWriter;
template <class W, class R>
class ClientReaderWriter;
template <class R>
class ClientAsyncReader;
template <class W>
class ClientAsyncWriter;
template <class W, class R>
class ClientAsyncReaderWriter;
template <class R>
class ClientAsyncResponseReader;

namespace testing {
class InteropClientContextInspector;
class ClientContextTestPeer;
}  // namespace testing

namespace internal {
class RpcMethod;
template <class InputMessage, class OutputMessage>
class BlockingUnaryCallImpl;
class CallOpClientRecvStatus;
class CallOpRecvInitialMetadata;
class ServerContextImpl;
template <class InputMessage, class OutputMessage>
class CallbackUnaryCallImpl;
template <class Request, class Response>
class ClientCallbackReaderWriterImpl;
template <class Response>
class ClientCallbackReaderImpl;
template <class Request>
class ClientCallbackWriterImpl;
class ClientCallbackUnaryImpl;
class ClientContextAccessor;
}  // namespace internal

class CallCredentials;
class Channel;
class ChannelInterface;
class CompletionQueue;

/// Options for \a ClientContext::FromServerContext specifying which traits from
/// the \a ServerContext to propagate (copy) from it into a new \a
/// ClientContext.
///
/// \see ClientContext::FromServerContext
class PropagationOptions {
 public:
  PropagationOptions() : propagate_(GRPC_PROPAGATE_DEFAULTS) {}

  PropagationOptions& enable_deadline_propagation() {
    propagate_ |= GRPC_PROPAGATE_DEADLINE;
    return *this;
  }

  PropagationOptions& disable_deadline_propagation() {
    propagate_ &= ~GRPC_PROPAGATE_DEADLINE;
    return *this;
  }

  PropagationOptions& enable_census_stats_propagation() {
    propagate_ |= GRPC_PROPAGATE_CENSUS_STATS_CONTEXT;
    return *this;
  }

  PropagationOptions& disable_census_stats_propagation() {
    propagate_ &= ~GRPC_PROPAGATE_CENSUS_STATS_CONTEXT;
    return *this;
  }

  PropagationOptions& enable_census_tracing_propagation() {
    propagate_ |= GRPC_PROPAGATE_CENSUS_TRACING_CONTEXT;
    return *this;
  }

  PropagationOptions& disable_census_tracing_propagation() {
    propagate_ &= ~GRPC_PROPAGATE_CENSUS_TRACING_CONTEXT;
    return *this;
  }

  PropagationOptions& enable_cancellation_propagation() {
    propagate_ |= GRPC_PROPAGATE_CANCELLATION;
    return *this;
  }

  PropagationOptions& disable_cancellation_propagation() {
    propagate_ &= ~GRPC_PROPAGATE_CANCELLATION;
    return *this;
  }

  uint32_t c_bitmask() const { return propagate_; }

 private:
  uint32_t propagate_;
};

/// A ClientContext allows the person implementing a service client to:
///
/// - Add custom metadata key-value pairs that will propagated to the server
///   side.
/// - Control call settings such as compression and authentication.
/// - Initial and trailing metadata coming from the server.
/// - Get performance metrics (ie, census).
///
/// Context settings are only relevant to the call they are invoked with, that
/// is to say, they aren't sticky. Some of these settings, such as the
/// compression options, can be made persistent at channel construction time
/// (see \a grpc::CreateCustomChannel).
///
/// \warning ClientContext instances should \em not be reused across rpcs.
/// \warning The ClientContext instance used for creating an rpc must remain
///          alive and valid for the lifetime of the rpc.
class ClientContext {
 public:
  ClientContext();
  ~ClientContext();

  /// Create a new \a ClientContext as a child of an incoming server call,
  /// according to \a options (\see PropagationOptions).
  ///
  /// \param server_context The source server context to use as the basis for
  /// constructing the client context.
  /// \param options The options controlling what to copy from the \a
  /// server_context.
  ///
  /// \return A newly constructed \a ClientContext instance based on \a
  /// server_context, with traits propagated (copied) according to \a options.
  static std::unique_ptr<ClientContext> FromServerContext(
      const grpc::ServerContextBase& server_context,
      PropagationOptions options = PropagationOptions());
  static std::unique_ptr<ClientContext> FromCallbackServerContext(
      const grpc::CallbackServerContext& server_context,
      PropagationOptions options = PropagationOptions());

  /// Add the (\a meta_key, \a meta_value) pair to the metadata associated with
  /// a client call. These are made available at the server side by the \a
  /// grpc::ServerContext::client_metadata() method.
  ///
  /// \warning This method should only be called before invoking the rpc.
  ///
  /// \param meta_key The metadata key. If \a meta_value is binary data, it must
  /// end in "-bin".
  /// \param meta_value The metadata value. If its value is binary, the key name
  /// must end in "-bin".
  ///
  /// Metadata must conform to the following format:
  ///
  ///\verbatim
  /// Custom-Metadata -> Binary-Header / ASCII-Header
  /// Binary-Header -> {Header-Name "-bin" } {binary value}
  /// ASCII-Header -> Header-Name ASCII-Value
  /// Header-Name -> 1*( %x30-39 / %x61-7A / "_" / "-" / ".") ; 0-9 a-z _ - .
  /// ASCII-Value -> 1*( %x20-%x7E ) ; space and printable ASCII
  /// Custom-Metadata -> Binary-Header / ASCII-Header
  ///\endverbatim
  ///
  void AddMetadata(const std::string& meta_key, const std::string& meta_value);

  /// Return a collection of initial metadata key-value pairs. Note that keys
  /// may happen more than once (ie, a \a std::multimap is returned).
  ///
  /// \warning This method should only be called after initial metadata has been
  /// received. For streaming calls, see \a
  /// ClientReaderInterface::WaitForInitialMetadata().
  ///
  /// \return A multimap of initial metadata key-value pairs from the server.
  const std::multimap<grpc::string_ref, grpc::string_ref>&
  GetServerInitialMetadata() const {
    ABSL_CHECK(initial_metadata_received_);
    return *recv_initial_metadata_.map();
  }

  /// Return a collection of trailing metadata key-value pairs. Note that keys
  /// may happen more than once (ie, a \a std::multimap is returned).
  ///
  /// \warning This method is only callable once the stream has finished.
  ///
  /// \return A multimap of metadata trailing key-value pairs from the server.
  const std::multimap<grpc::string_ref, grpc::string_ref>&
  GetServerTrailingMetadata() const {
    // TODO(yangg) check finished
    return *trailing_metadata_.map();
  }

  /// Set the deadline for the client call.
  ///
  /// \warning This method should only be called before invoking the rpc.
  ///
  /// \param deadline the deadline for the client call. Units are determined by
  /// the type used. The deadline is an absolute (not relative) time.
  template <typename T>
  void set_deadline(const T& deadline) {
    grpc::TimePoint<T> deadline_tp(deadline);
    deadline_ = deadline_tp.raw_time();
  }

  /// Trigger wait-for-ready or not on this request.
  /// See https://github.com/grpc/grpc/blob/master/doc/wait-for-ready.md.
  /// If set, if an RPC is made when a channel's connectivity state is
  /// TRANSIENT_FAILURE or CONNECTING, the call will not "fail fast",
  /// and the channel will wait until the channel is READY before making the
  /// call.
  void set_wait_for_ready(bool wait_for_ready) {
    wait_for_ready_ = wait_for_ready;
    wait_for_ready_explicitly_set_ = true;
  }

  /// DEPRECATED: Use set_wait_for_ready() instead.
  void set_fail_fast(bool fail_fast) { set_wait_for_ready(!fail_fast); }

  /// Return the deadline for the client call.
  std::chrono::system_clock::time_point deadline() const {
    return grpc::Timespec2Timepoint(deadline_);
  }

  /// Return a \a gpr_timespec representation of the client call's deadline.
  gpr_timespec raw_deadline() const { return deadline_; }

  /// Set the per call authority header (see
  /// https://tools.ietf.org/html/rfc7540#section-8.1.2.3).
  void set_authority(const std::string& authority) { authority_ = authority; }

  /// Return the authentication context for the associated client call.
  /// It is only valid to call this during the lifetime of the client call.
  ///
  /// \see grpc::AuthContext.
  std::shared_ptr<const grpc::AuthContext> auth_context() const {
    if (auth_context_ == nullptr) {
      auth_context_ = grpc::CreateAuthContext(call_);
    }
    return auth_context_;
  }

  /// Set credentials for the client call.
  ///
  /// A credentials object encapsulates all the state needed by a client to
  /// authenticate with a server and make various assertions, e.g., about the
  /// client’s identity, role, or whether it is authorized to make a particular
  /// call.
  ///
  /// It is legal to call this only before initial metadata is sent.
  ///
  /// \see  https://grpc.io/docs/guides/auth.html
  void set_credentials(const std::shared_ptr<grpc::CallCredentials>& creds);

  /// EXPERIMENTAL debugging API
  ///
  /// Returns the credentials for the client call. This should be used only in
  /// tests and for diagnostic purposes, and should not be used by application
  /// logic.
  std::shared_ptr<grpc::CallCredentials> credentials() { return creds_; }

  /// Return the compression algorithm the client call will request be used.
  /// Note that the gRPC runtime may decide to ignore this request, for example,
  /// due to resource constraints.
  grpc_compression_algorithm compression_algorithm() const {
    return compression_algorithm_;
  }

  /// Set \a algorithm to be the compression algorithm used for the client call.
  ///
  /// \param algorithm The compression algorithm used for the client call.
  void set_compression_algorithm(grpc_compression_algorithm algorithm);

  /// Flag whether the initial metadata should be \a corked
  ///
  /// If \a corked is true, then the initial metadata will be coalesced with the
  /// write of first message in the stream. As a result, any tag set for the
  /// initial metadata operation (starting a client-streaming or bidi-streaming
  /// RPC) will not actually be sent to the completion queue or delivered
  /// via Next.
  ///
  /// \param corked The flag indicating whether the initial metadata is to be
  /// corked or not.
  void set_initial_metadata_corked(bool corked) {
    initial_metadata_corked_ = corked;
  }

  /// Return the peer uri in a string.
  /// It is only valid to call this during the lifetime of the client call.
  ///
  /// \warning This value is never authenticated or subject to any security
  /// related code. It must not be used for any authentication related
  /// functionality. Instead, use auth_context.
  ///
  /// \return The call's peer URI.
  std::string peer() const;

  /// Sets the census context.
  /// It is only valid to call this before the client call is created. A common
  /// place of setting census context is from within the DefaultConstructor
  /// method of GlobalCallbacks.
  void set_census_context(struct census_context* ccp) { census_context_ = ccp; }

  /// Returns the census context that has been set, or nullptr if not set.
  struct census_context* census_context() const { return census_context_; }

  /// Send a best-effort out-of-band cancel on the call associated with
  /// this client context.  The call could be in any stage; e.g., if it is
  /// already finished, it may still return success.
  ///
  /// There is no guarantee the call will be cancelled.
  ///
  /// Note that TryCancel() does not change any of the tags that are pending
  /// on the completion queue. All pending tags will still be delivered
  /// (though their ok result may reflect the effect of cancellation).
  ///
  /// This method is thread-safe, and can be called multiple times from any
  /// thread.
  void TryCancel();

  /// Global Callbacks
  ///
  /// Can be set exactly once per application to install hooks whenever
  /// a client context is constructed and destructed.
  class GlobalCallbacks {
   public:
    virtual ~GlobalCallbacks() {}
    virtual void DefaultConstructor(ClientContext* context) = 0;
    virtual void Destructor(ClientContext* context) = 0;
  };
  static void SetGlobalCallbacks(GlobalCallbacks* callbacks);

  /// Should be used for framework-level extensions only.
  /// Applications never need to call this method.
  grpc_call* c_call() { return call_; }

  /// EXPERIMENTAL debugging API
  ///
  /// if status is not ok() for an RPC, this will return a detailed string
  /// of the gRPC Core error that led to the failure. It should not be relied
  /// upon for anything other than gaining more debug data in failure cases.
  std::string debug_error_string() const { return debug_error_string_; }

 private:
  // Disallow copy and assign.
  ClientContext(const ClientContext&);
  ClientContext& operator=(const ClientContext&);

  friend class grpc::testing::InteropClientContextInspector;
  friend class grpc::testing::ClientContextTestPeer;
  friend class grpc::internal::CallOpClientRecvStatus;
  friend class grpc::internal::CallOpRecvInitialMetadata;
  friend class grpc::Channel;
  template <class R>
  friend class grpc::ClientReader;
  template <class W>
  friend class grpc::ClientWriter;
  template <class W, class R>
  friend class grpc::ClientReaderWriter;
  template <class R>
  friend class grpc::ClientAsyncReader;
  template <class W>
  friend class grpc::ClientAsyncWriter;
  template <class W, class R>
  friend class grpc::ClientAsyncReaderWriter;
  template <class R>
  friend class grpc::ClientAsyncResponseReader;
  friend class grpc::internal::ClientAsyncResponseReaderHelper;
  template <class InputMessage, class OutputMessage>
  friend class grpc::internal::BlockingUnaryCallImpl;
  template <class InputMessage, class OutputMessage>
  friend class grpc::internal::CallbackUnaryCallImpl;
  template <class Request, class Response>
  friend class grpc::internal::ClientCallbackReaderWriterImpl;
  template <class Response>
  friend class grpc::internal::ClientCallbackReaderImpl;
  template <class Request>
  friend class grpc::internal::ClientCallbackWriterImpl;
  friend class grpc::internal::ClientCallbackUnaryImpl;
  friend class grpc::internal::ClientContextAccessor;

  // Used by friend class CallOpClientRecvStatus
  void set_debug_error_string(const std::string& debug_error_string) {
    debug_error_string_ = debug_error_string;
  }

  grpc_call* call() const { return call_; }
  void set_call(grpc_call* call, const std::shared_ptr<grpc::Channel>& channel);

  grpc::experimental::ClientRpcInfo* set_client_rpc_info(
      const char* method, const char* suffix_for_stats,
      grpc::internal::RpcMethod::RpcType type, grpc::ChannelInterface* channel,
      const std::vector<std::unique_ptr<
          grpc::experimental::ClientInterceptorFactoryInterface>>& creators,
      size_t interceptor_pos) {
    rpc_info_ = grpc::experimental::ClientRpcInfo(this, type, method,
                                                  suffix_for_stats, channel);
    rpc_info_.RegisterInterceptors(creators, interceptor_pos);
    return &rpc_info_;
  }

  uint32_t initial_metadata_flags() const {
    return (wait_for_ready_ ? GRPC_INITIAL_METADATA_WAIT_FOR_READY : 0) |
           (wait_for_ready_explicitly_set_
                ? GRPC_INITIAL_METADATA_WAIT_FOR_READY_EXPLICITLY_SET
                : 0);
  }

  std::string authority() { return authority_; }

  void SendCancelToInterceptors();

  static std::unique_ptr<ClientContext> FromInternalServerContext(
      const grpc::ServerContextBase& server_context,
      PropagationOptions options);

  bool initial_metadata_received_;
  bool wait_for_ready_;
  bool wait_for_ready_explicitly_set_;
  std::shared_ptr<grpc::Channel> channel_;
  grpc::internal::Mutex mu_;
  grpc_call* call_;
  bool call_canceled_;
  gpr_timespec deadline_;
  grpc::string authority_;
  std::shared_ptr<grpc::CallCredentials> creds_;
  mutable std::shared_ptr<const grpc::AuthContext> auth_context_;
  struct census_context* census_context_;
  std::multimap<std::string, std::string> send_initial_metadata_;
  mutable grpc::internal::MetadataMap recv_initial_metadata_;
  mutable grpc::internal::MetadataMap trailing_metadata_;

  grpc_call* propagate_from_call_;
  PropagationOptions propagation_options_;

  grpc_compression_algorithm compression_algorithm_;
  bool initial_metadata_corked_;

  std::string debug_error_string_;

  grpc::experimental::ClientRpcInfo rpc_info_;
};

}  // namespace grpc

#endif  // GRPCPP_CLIENT_CONTEXT_H
