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

#ifndef GRPC_SRC_CPP_EXT_PROTO_SERVER_REFLECTION_H
#define GRPC_SRC_CPP_EXT_PROTO_SERVER_REFLECTION_H

#include <grpcpp/grpcpp.h>
#include <grpcpp/impl/codegen/config_protobuf.h>
#include <grpcpp/support/config.h>
#include <grpcpp/support/status.h>
#include <grpcpp/support/sync_stream.h>

#include <memory>
#include <string>
#include <unordered_set>
#include <utility>
#include <vector>

#include "src/proto/grpc/reflection/v1/reflection.grpc.pb.h"
#include "src/proto/grpc/reflection/v1alpha/reflection.grpc.pb.h"

namespace grpc {

class ProtoServerReflectionBackend {
 public:
  ProtoServerReflectionBackend()
      : descriptor_pool_(protobuf::DescriptorPool::generated_pool()) {}

  void SetServiceList(const std::vector<std::string>* services) {
    services_ = services;
  }

  template <typename Request, typename Response>
  Status ServerReflectionInfo(
      ServerReaderWriter<Response, Request>* stream) const;

 private:
  template <typename Response>
  Status ListService(Response* response) const;

  template <typename Response>
  Status GetFileByName(const std::string& file_name, Response* response) const;

  template <typename Response>
  Status GetFileContainingSymbol(const std::string& symbol,
                                 Response* response) const;

  template <typename Request, typename Response>
  Status GetFileContainingExtension(const Request* request,
                                    Response* response) const;

  template <typename Response>
  Status GetAllExtensionNumbers(const std::string& type,
                                Response* response) const;

  template <typename Response>
  void FillFileDescriptorResponse(
      const protobuf::FileDescriptor* file_desc, Response* response,
      std::unordered_set<std::string>* seen_files) const;

  template <typename Response>
  void FillErrorResponse(const Status& status, Response* error_response) const;

  const protobuf::DescriptorPool* descriptor_pool_;
  const std::vector<string>* services_;
};

class ProtoServerReflection final
    : public reflection::v1alpha::ServerReflection::Service {
 public:
  ProtoServerReflection()
      : grpc::ProtoServerReflection(
            std::make_shared<ProtoServerReflectionBackend>()) {}

  explicit ProtoServerReflection(
      std::shared_ptr<ProtoServerReflectionBackend> backend)
      : backend_(std::move(backend)) {}

  // Add the full names of registered services
  void SetServiceList(const std::vector<std::string>* services) {
    backend_->SetServiceList(services);
  }

  // implementation of ServerReflectionInfo(stream ServerReflectionRequest) rpc
  // in ServerReflection service
  Status ServerReflectionInfo(
      ServerContext* context,
      ServerReaderWriter<reflection::v1alpha::ServerReflectionResponse,
                         reflection::v1alpha::ServerReflectionRequest>* stream)
      override;

  std::shared_ptr<ProtoServerReflectionBackend> backend_;
};

class ProtoServerReflectionV1 final
    : public reflection::v1::ServerReflection::Service {
 public:
  explicit ProtoServerReflectionV1(
      std::shared_ptr<ProtoServerReflectionBackend> backend)
      : backend_(std::move(backend)) {}

  // implementation of ServerReflectionInfo(stream ServerReflectionRequest) rpc
  // in ServerReflection service
  Status ServerReflectionInfo(
      ServerContext* /* context */,
      ServerReaderWriter<reflection::v1::ServerReflectionResponse,
                         reflection::v1::ServerReflectionRequest>* stream)
      override;

 private:
  std::shared_ptr<ProtoServerReflectionBackend> backend_;
};

}  // namespace grpc

#endif  // GRPC_SRC_CPP_EXT_PROTO_SERVER_REFLECTION_H
