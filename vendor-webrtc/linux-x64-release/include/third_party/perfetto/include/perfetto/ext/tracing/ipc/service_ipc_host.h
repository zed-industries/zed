/*
 * Copyright (C) 2017 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef INCLUDE_PERFETTO_EXT_TRACING_IPC_SERVICE_IPC_HOST_H_
#define INCLUDE_PERFETTO_EXT_TRACING_IPC_SERVICE_IPC_HOST_H_

#include <list>
#include <memory>

#include "perfetto/base/export.h"
#include "perfetto/ext/base/unix_socket.h"
#include "perfetto/ext/tracing/core/basic_types.h"
#include "perfetto/ext/tracing/core/tracing_service.h"

namespace perfetto {
namespace base {
class TaskRunner;
}  // namespace base.

namespace ipc {
class Host;
}  // namespace ipc

// The argument passed to ServiceIPCHost::Start. Can be either:
// 1. a socket name (e.g., "/dev/unix/socket" for AF_UNIX, "127.0.0.1:1234" for
//    TCP, "vsock://1:1234")
// 2. A FD of a pre-bound socket. This handles the case of Android in-tree
//    builds where init creates the socket and passes the FD in env var
//    (See perfetto.rc).
// 3. A pre-existing ipc::Host object.
struct ListenEndpoint {
  explicit ListenEndpoint(const char* socket_name);
  explicit ListenEndpoint(std::string socket_name);
  explicit ListenEndpoint(base::ScopedSocketHandle);
  explicit ListenEndpoint(std::unique_ptr<ipc::Host>);
  ~ListenEndpoint();

  // Allow move but not copy.
  ListenEndpoint(ListenEndpoint&&) noexcept;
  ListenEndpoint& operator=(ListenEndpoint&&);
  ListenEndpoint(const ListenEndpoint&) noexcept = delete;
  ListenEndpoint& operator=(const ListenEndpoint&) noexcept = delete;

  // Only one of these is ever set.
  std::string sock_name;
  base::ScopedSocketHandle sock_handle;
  std::unique_ptr<ipc::Host> ipc_host;
};

// Creates an instance of the service (business logic + UNIX socket transport).
// Exposed to:
//   The code in the tracing client that will host the service e.g., traced.
// Implemented in:
//   src/tracing/ipc/service/service_ipc_host_impl.cc
class PERFETTO_EXPORT_COMPONENT ServiceIPCHost {
 public:
  static std::unique_ptr<ServiceIPCHost> CreateInstance(
      base::TaskRunner*,
      TracingService::InitOpts = {});
  virtual ~ServiceIPCHost();

  // Start listening on the Producer & Consumer ports. Returns false in case of
  // failure (e.g., something else is listening on |socket_name|).
  virtual bool Start(std::list<ListenEndpoint> producer_sockets,
                     ListenEndpoint consumer_socket) = 0;

  virtual TracingService* service() const = 0;

  // The methods below are for API compatibility with other projects that use
  // some of the old flavours of Start(), back in the days when we supported
  // only one socket or fd.

  // Like the above, but takes two file descriptors to already bound sockets.
  // This is used when building as part of the Android tree, where init opens
  // and binds the socket beore exec()-ing us.
  // Note: An internal Google project uses this (b/390202952).
  bool Start(base::ScopedSocketHandle producer_socket_fd,
             base::ScopedSocketHandle consumer_socket_fd);

  // Allows callers to supply preconstructed Hosts.
  bool Start(std::unique_ptr<ipc::Host> producer_host,
             std::unique_ptr<ipc::Host> consumer_host);

  // Used by tests. producer_socket_name can be a comma-separated list of N
  // endpoints to listen onto.
  bool Start(const char* producer_socket_names,
             const char* consumer_socket_name);

 protected:
  ServiceIPCHost();

 private:
  ServiceIPCHost(const ServiceIPCHost&) = delete;
  ServiceIPCHost& operator=(const ServiceIPCHost&) = delete;
};

}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_EXT_TRACING_IPC_SERVICE_IPC_HOST_H_
