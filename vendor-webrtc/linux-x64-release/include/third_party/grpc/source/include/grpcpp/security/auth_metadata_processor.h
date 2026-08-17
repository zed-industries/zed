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

#ifndef GRPCPP_SECURITY_AUTH_METADATA_PROCESSOR_H
#define GRPCPP_SECURITY_AUTH_METADATA_PROCESSOR_H

#include <grpcpp/security/auth_context.h>
#include <grpcpp/support/status.h>
#include <grpcpp/support/string_ref.h>

#include <map>

namespace grpc {

/// Interface allowing custom server-side authorization based on credentials
/// encoded in metadata.  Objects of this type can be passed to
/// \a ServerCredentials::SetAuthMetadataProcessor().
/// Please also check out \a grpc::experimental::Interceptor for another way to
/// do customized operations on the information provided by a specific call.
class AuthMetadataProcessor {
 public:
  typedef std::multimap<grpc::string_ref, grpc::string_ref> InputMetadata;
  typedef std::multimap<std::string, std::string> OutputMetadata;

  virtual ~AuthMetadataProcessor() {}

  /// If this method returns true, the \a Process function will be scheduled in
  /// a different thread from the one processing the call.
  virtual bool IsBlocking() const { return true; }

  /// Processes a Call associated with a connection.
  /// auth_metadata: the authentication metadata associated with the particular
  ///   call
  /// context: contains the connection-level info, e.g. the peer identity. This
  ///   parameter is readable and writable. Note that since the information is
  ///   shared for all calls associated with the connection, if the
  ///   implementation updates the info in a specific call, all the subsequent
  ///   calls will see the updates. A typical usage of context is to use
  ///   |auth_metadata| to infer the peer identity, and augment it with
  ///   properties.
  /// consumed_auth_metadata: contains the metadata that the implementation
  ///   wants to remove from the current call, so that the server application is
  ///   no longer able to see it anymore. A typical usage would be to do token
  ///   authentication in the first call, and then remove the token information
  ///   for all subsequent calls.
  /// response_metadata(CURRENTLY NOT SUPPORTED): the metadata that will be sent
  ///   as part of the response.
  /// return: if the return value is not Status::OK, the rpc call will be
  ///   aborted with the error code and error message sent back to the client.
  virtual grpc::Status Process(const InputMetadata& auth_metadata,
                               grpc::AuthContext* context,
                               OutputMetadata* consumed_auth_metadata,
                               OutputMetadata* response_metadata) = 0;
};

}  // namespace grpc

#endif  // GRPCPP_SECURITY_AUTH_METADATA_PROCESSOR_H
