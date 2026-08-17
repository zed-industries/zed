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

#ifndef GRPC_SRC_CORE_LIB_SECURITY_SECURITY_CONNECTOR_SECURITY_CONNECTOR_H
#define GRPC_SRC_CORE_LIB_SECURITY_SECURITY_CONNECTOR_SECURITY_CONNECTOR_H

#include <grpc/credentials.h>
#include <grpc/grpc.h>
#include <grpc/grpc_security.h>
#include <grpc/support/port_platform.h>

#include <memory>

#include "absl/status/status.h"
#include "absl/strings/string_view.h"
#include "src/core/handshaker/handshaker.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/debug/trace.h"
#include "src/core/lib/iomgr/closure.h"
#include "src/core/lib/iomgr/endpoint.h"
#include "src/core/lib/iomgr/error.h"
#include "src/core/lib/iomgr/iomgr_fwd.h"
#include "src/core/lib/promise/arena_promise.h"
#include "src/core/tsi/transport_security_interface.h"
#include "src/core/util/ref_counted.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/unique_type_name.h"

// --- URL schemes. ---

#define GRPC_SSL_URL_SCHEME "https"
#define GRPC_FAKE_SECURITY_URL_SCHEME "http+fake_security"

typedef enum { GRPC_SECURITY_OK = 0, GRPC_SECURITY_ERROR } grpc_security_status;

// --- security_connector object. ---

//  A security connector object represents away to configure the underlying
//  transport security mechanism and check the resulting trusted peer.

#define GRPC_ARG_SECURITY_CONNECTOR "grpc.internal.security_connector"

class grpc_security_connector
    : public grpc_core::RefCounted<grpc_security_connector> {
 public:
  explicit grpc_security_connector(absl::string_view url_scheme)
      : grpc_core::RefCounted<grpc_security_connector>(
            GRPC_TRACE_FLAG_ENABLED(security_connector_refcount)
                ? "security_connector_refcount"
                : nullptr),
        url_scheme_(url_scheme) {}

  static absl::string_view ChannelArgName() {
    return GRPC_ARG_SECURITY_CONNECTOR;
  }

  // Checks the peer. Callee takes ownership of the peer object.
  // The channel args represent the args after the handshaking is performed.
  // When done, sets *auth_context and invokes on_peer_checked.
  virtual void check_peer(
      tsi_peer peer, grpc_endpoint* ep, const grpc_core::ChannelArgs& args,
      grpc_core::RefCountedPtr<grpc_auth_context>* auth_context,
      grpc_closure* on_peer_checked) = 0;

  // Cancels the pending check_peer() request associated with on_peer_checked.
  // If there is no such request pending, this is a no-op.
  virtual void cancel_check_peer(grpc_closure* on_peer_checked,
                                 grpc_error_handle error) = 0;

  // Compares two security connectors.
  virtual int cmp(const grpc_security_connector* other) const = 0;

  static int ChannelArgsCompare(const grpc_security_connector* a,
                                const grpc_security_connector* b) {
    return a->cmp(b);
  }

  absl::string_view url_scheme() const { return url_scheme_; }

  virtual grpc_core::UniqueTypeName type() const = 0;

 private:
  absl::string_view url_scheme_;
};

// Util to encapsulate the connector in a channel arg.
grpc_arg grpc_security_connector_to_arg(grpc_security_connector* sc);

// Util to get the connector from a channel arg.
grpc_security_connector* grpc_security_connector_from_arg(const grpc_arg* arg);

// Util to find the connector from channel args.
grpc_security_connector* grpc_security_connector_find_in_args(
    const grpc_channel_args* args);

// --- channel_security_connector object. ---

//  A channel security connector object represents a way to configure the
//  underlying transport security mechanism on the client side.

class grpc_channel_security_connector : public grpc_security_connector {
 public:
  grpc_channel_security_connector(
      absl::string_view url_scheme,
      grpc_core::RefCountedPtr<grpc_channel_credentials> channel_creds,
      grpc_core::RefCountedPtr<grpc_call_credentials> request_metadata_creds);

  /// Checks that the host that will be set for a call is acceptable.
  /// Returns ok if the host is acceptable, otherwise returns an error.
  virtual grpc_core::ArenaPromise<absl::Status> CheckCallHost(
      absl::string_view host, grpc_auth_context* auth_context) = 0;

  /// Registers handshakers with \a handshake_mgr.
  virtual void add_handshakers(const grpc_core::ChannelArgs& args,
                               grpc_pollset_set* interested_parties,
                               grpc_core::HandshakeManager* handshake_mgr) = 0;

  const grpc_channel_credentials* channel_creds() const {
    return channel_creds_.get();
  }
  grpc_channel_credentials* mutable_channel_creds() {
    return channel_creds_.get();
  }
  const grpc_call_credentials* request_metadata_creds() const {
    return request_metadata_creds_.get();
  }
  grpc_call_credentials* mutable_request_metadata_creds() {
    return request_metadata_creds_.get();
  }

  grpc_core::UniqueTypeName type() const override;

 protected:
  // Helper methods to be used in subclasses.
  int channel_security_connector_cmp(
      const grpc_channel_security_connector* other) const;

  // grpc_channel_args* channel_args() const { return channel_args_.get(); }
  //// Should be called as soon as the channel args are not needed to reduce
  //// memory usage.
  // void clear_channel_arg() { channel_args_.reset(); }

 private:
  grpc_core::RefCountedPtr<grpc_channel_credentials> channel_creds_;
  grpc_core::RefCountedPtr<grpc_call_credentials> request_metadata_creds_;
  std::unique_ptr<grpc_channel_args> channel_args_;
};

// --- server_security_connector object. ---

//  A server security connector object represents a way to configure the
//  underlying transport security mechanism on the server side.

class grpc_server_security_connector : public grpc_security_connector {
 public:
  grpc_server_security_connector(
      absl::string_view url_scheme,
      grpc_core::RefCountedPtr<grpc_server_credentials> server_creds);

  virtual void add_handshakers(const grpc_core::ChannelArgs& args,
                               grpc_pollset_set* interested_parties,
                               grpc_core::HandshakeManager* handshake_mgr) = 0;

  const grpc_server_credentials* server_creds() const {
    return server_creds_.get();
  }
  grpc_server_credentials* mutable_server_creds() {
    return server_creds_.get();
  }

  grpc_core::UniqueTypeName type() const override;

 protected:
  // Helper methods to be used in subclasses.
  int server_security_connector_cmp(
      const grpc_server_security_connector* other) const;

 private:
  grpc_core::RefCountedPtr<grpc_server_credentials> server_creds_;
};

#endif  // GRPC_SRC_CORE_LIB_SECURITY_SECURITY_CONNECTOR_SECURITY_CONNECTOR_H
