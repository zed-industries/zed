//
//
// Copyright 2016 gRPC authors.
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

#ifndef GRPC_SRC_CPP_SERVER_HEALTH_DEFAULT_HEALTH_CHECK_SERVICE_H
#define GRPC_SRC_CPP_SERVER_HEALTH_DEFAULT_HEALTH_CHECK_SERVICE_H

#include <grpcpp/grpcpp.h>
#include <grpcpp/health_check_service_interface.h>
#include <grpcpp/impl/service_type.h>
#include <grpcpp/impl/sync.h>
#include <grpcpp/support/byte_buffer.h>
#include <grpcpp/support/server_callback.h>
#include <grpcpp/support/status.h>
#include <stddef.h>

#include <map>
#include <memory>
#include <string>

#include "absl/base/thread_annotations.h"
#include "src/core/util/ref_counted.h"
#include "src/core/util/ref_counted_ptr.h"

namespace grpc {

// Default implementation of HealthCheckServiceInterface. Server will create and
// own it.
class DefaultHealthCheckService final : public HealthCheckServiceInterface {
 public:
  enum ServingStatus { NOT_FOUND, SERVING, NOT_SERVING };

  // The service impl to register with the server.
  class HealthCheckServiceImpl : public Service {
   public:
    // Reactor for handling Watch streams.
    class WatchReactor : public ServerWriteReactor<ByteBuffer>,
                         public grpc_core::RefCounted<WatchReactor> {
     public:
      WatchReactor(HealthCheckServiceImpl* service, const ByteBuffer* request);

      void SendHealth(ServingStatus status);

      void OnWriteDone(bool ok) override;
      void OnCancel() override;
      void OnDone() override;

     private:
      void SendHealthLocked(ServingStatus status)
          ABSL_EXCLUSIVE_LOCKS_REQUIRED(&mu_);

      void MaybeFinishLocked(Status status) ABSL_EXCLUSIVE_LOCKS_REQUIRED(&mu_);

      HealthCheckServiceImpl* service_;
      std::string service_name_;
      ByteBuffer response_;

      grpc::internal::Mutex mu_;
      bool write_pending_ ABSL_GUARDED_BY(mu_) = false;
      ServingStatus pending_status_ ABSL_GUARDED_BY(mu_) = NOT_FOUND;
      bool finish_called_ ABSL_GUARDED_BY(mu_) = false;
    };

    explicit HealthCheckServiceImpl(DefaultHealthCheckService* database);

    ~HealthCheckServiceImpl() override;

   private:
    // Request handler for Check method.
    static ServerUnaryReactor* HandleCheckRequest(
        DefaultHealthCheckService* database, CallbackServerContext* context,
        const ByteBuffer* request, ByteBuffer* response);

    // Returns true on success.
    static bool DecodeRequest(const ByteBuffer& request,
                              std::string* service_name);
    static bool EncodeResponse(ServingStatus status, ByteBuffer* response);

    DefaultHealthCheckService* database_;

    grpc::internal::Mutex mu_;
    grpc::internal::CondVar shutdown_condition_;
    bool shutdown_ ABSL_GUARDED_BY(mu_) = false;
    size_t num_watches_ ABSL_GUARDED_BY(mu_) = 0;
  };

  DefaultHealthCheckService();

  void SetServingStatus(const std::string& service_name, bool serving) override;
  void SetServingStatus(bool serving) override;

  void Shutdown() override;

  ServingStatus GetServingStatus(const std::string& service_name) const;

  HealthCheckServiceImpl* GetHealthCheckService();

 private:
  // Stores the current serving status of a service and any call
  // handlers registered for updates when the service's status changes.
  class ServiceData {
   public:
    void SetServingStatus(ServingStatus status);
    ServingStatus GetServingStatus() const { return status_; }
    void AddWatch(
        grpc_core::RefCountedPtr<HealthCheckServiceImpl::WatchReactor> watcher);
    void RemoveWatch(HealthCheckServiceImpl::WatchReactor* watcher);
    bool Unused() const { return watchers_.empty() && status_ == NOT_FOUND; }

   private:
    ServingStatus status_ = NOT_FOUND;
    std::map<HealthCheckServiceImpl::WatchReactor*,
             grpc_core::RefCountedPtr<HealthCheckServiceImpl::WatchReactor>>
        watchers_;
  };

  void RegisterWatch(
      const std::string& service_name,
      grpc_core::RefCountedPtr<HealthCheckServiceImpl::WatchReactor> watcher);

  void UnregisterWatch(const std::string& service_name,
                       HealthCheckServiceImpl::WatchReactor* watcher);

  mutable grpc::internal::Mutex mu_;
  bool shutdown_ ABSL_GUARDED_BY(&mu_) = false;
  std::map<std::string, ServiceData> services_map_ ABSL_GUARDED_BY(&mu_);
  std::unique_ptr<HealthCheckServiceImpl> impl_;
};

}  // namespace grpc

#endif  // GRPC_SRC_CPP_SERVER_HEALTH_DEFAULT_HEALTH_CHECK_SERVICE_H
