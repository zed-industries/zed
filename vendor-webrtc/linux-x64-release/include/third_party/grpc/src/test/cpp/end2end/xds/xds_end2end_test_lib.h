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

#ifndef GRPC_TEST_CPP_END2END_XDS_XDS_END2END_TEST_LIB_H
#define GRPC_TEST_CPP_END2END_XDS_XDS_END2END_TEST_LIB_H

#include <grpc/grpc.h>
#include <grpc/grpc_security.h>
#include <grpcpp/channel.h>
#include <grpcpp/client_context.h>
#include <grpcpp/ext/call_metric_recorder.h>
#include <grpcpp/ext/server_metric_recorder.h>
#include <grpcpp/xds_server_builder.h>

#include <memory>
#include <optional>
#include <set>
#include <string>
#include <thread>
#include <vector>

#include "absl/log/check.h"
#include "absl/log/log.h"
#include "absl/status/statusor.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/string_view.h"
#include "envoy/config/rbac/v3/rbac.pb.h"
#include "envoy/extensions/filters/http/rbac/v3/rbac.pb.h"
#include "envoy/extensions/filters/network/http_connection_manager/v3/http_connection_manager.pb.h"
#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "src/core/credentials/transport/fake/fake_credentials.h"
#include "src/core/credentials/transport/tls/ssl_utils.h"
#include "src/core/lib/event_engine/default_event_engine.h"
#include "src/cpp/server/secure_server_credentials.h"
#include "src/proto/grpc/testing/echo.pb.h"
#include "test/core/test_util/port.h"
#include "test/core/test_util/resolve_localhost_ip46.h"
#include "test/cpp/end2end/counted_service.h"
#include "test/cpp/end2end/test_service_impl.h"
#include "test/cpp/end2end/xds/xds_server.h"
#include "test/cpp/end2end/xds/xds_utils.h"
#include "xds/data/orca/v3/orca_load_report.pb.h"

namespace grpc {
namespace testing {

// The parameter type for INSTANTIATE_TEST_SUITE_P().
class XdsTestType {
 public:
  enum HttpFilterConfigLocation {
    // Set the HTTP filter config directly in LDS.
    kHttpFilterConfigInListener,
    // Enable the HTTP filter in LDS, but override the filter config in route.
    kHttpFilterConfigInRoute,
  };

  enum BootstrapSource {
    kBootstrapFromChannelArg,
    kBootstrapFromFile,
    kBootstrapFromEnvVar,
  };

  XdsTestType& set_enable_load_reporting() {
    enable_load_reporting_ = true;
    return *this;
  }

  XdsTestType& set_enable_rds_testing() {
    enable_rds_testing_ = true;
    return *this;
  }

  XdsTestType& set_use_csds_streaming() {
    use_csds_streaming_ = true;
    return *this;
  }

  XdsTestType& set_filter_config_setup(HttpFilterConfigLocation setup) {
    filter_config_setup_ = setup;
    return *this;
  }

  XdsTestType& set_bootstrap_source(BootstrapSource bootstrap_source) {
    bootstrap_source_ = bootstrap_source;
    return *this;
  }

  XdsTestType& set_rbac_action(::envoy::config::rbac::v3::RBAC_Action action) {
    rbac_action_ = action;
    return *this;
  }

  XdsTestType& set_rbac_audit_condition(
      ::envoy::config::rbac::v3::RBAC_AuditLoggingOptions_AuditCondition
          audit_condition) {
    rbac_audit_condition_ = audit_condition;
    return *this;
  }

  bool enable_load_reporting() const { return enable_load_reporting_; }
  bool enable_rds_testing() const { return enable_rds_testing_; }
  bool use_csds_streaming() const { return use_csds_streaming_; }
  HttpFilterConfigLocation filter_config_setup() const {
    return filter_config_setup_;
  }
  BootstrapSource bootstrap_source() const { return bootstrap_source_; }
  ::envoy::config::rbac::v3::RBAC_Action rbac_action() const {
    return rbac_action_;
  }
  ::envoy::config::rbac::v3::RBAC_AuditLoggingOptions_AuditCondition
  rbac_audit_condition() const {
    return rbac_audit_condition_;
  }

  std::string AsString() const {
    std::string retval = "V3";
    if (enable_load_reporting_) retval += "WithLoadReporting";
    if (enable_rds_testing_) retval += "Rds";
    if (use_csds_streaming_) retval += "CsdsStreaming";
    if (filter_config_setup_ == kHttpFilterConfigInRoute) {
      retval += "FilterPerRouteOverride";
    }
    if (bootstrap_source_ == kBootstrapFromFile) {
      retval += "BootstrapFromFile";
    } else if (bootstrap_source_ == kBootstrapFromEnvVar) {
      retval += "BootstrapFromEnvVar";
    }
    if (rbac_action_ == ::envoy::config::rbac::v3::RBAC_Action_ALLOW) {
      retval += "RbacAllow";
    } else if (rbac_action_ == ::envoy::config::rbac::v3::RBAC_Action_DENY) {
      retval += "RbacDeny";
    }
    if (rbac_audit_condition_ !=
        ::envoy::config::rbac::v3::
            RBAC_AuditLoggingOptions_AuditCondition_NONE) {
      retval += absl::StrCat("AuditCondition",
                             ::envoy::config::rbac::v3::
                                 RBAC_AuditLoggingOptions_AuditCondition_Name(
                                     rbac_audit_condition_));
    }
    return retval;
  }

  // For use as the final parameter in INSTANTIATE_TEST_SUITE_P().
  static std::string Name(const ::testing::TestParamInfo<XdsTestType>& info) {
    return info.param.AsString();
  }

 private:
  bool enable_load_reporting_ = false;
  bool enable_rds_testing_ = false;
  bool use_csds_streaming_ = false;
  HttpFilterConfigLocation filter_config_setup_ = kHttpFilterConfigInListener;
  BootstrapSource bootstrap_source_ = kBootstrapFromChannelArg;
  ::envoy::config::rbac::v3::RBAC_Action rbac_action_ =
      ::envoy::config::rbac::v3::RBAC_Action_LOG;
  ::envoy::config::rbac::v3::RBAC_AuditLoggingOptions_AuditCondition
      rbac_audit_condition_ = ::envoy::config::rbac::v3::
          RBAC_AuditLoggingOptions_AuditCondition_NONE;
};

// A base class for xDS end-to-end tests.
//
// An xDS server is provided in balancer_.  It is automatically started
// for every test.  Additional xDS servers can be started if needed by
// calling CreateAndStartBalancer().
//
// A default set of LDS, RDS, and CDS resources are created for gRPC
// clients, available in default_listener_, default_route_config_, and
// default_cluster_.  These resources are automatically loaded into
// balancer_ but can be modified by individual tests.  No EDS resource
// is provided by default.  There are also default LDS and RDS resources
// for the gRPC server side in default_server_listener_ and
// default_server_route_config_.  Methods are provided for constructing new
// resources that can be added to the xDS server as needed.
//
// This class provides a mechanism for running backend servers, which will
// be stored in backends_.  No servers are created or started by default,
// but tests can call CreateAndStartBackends() to start however many
// backends they want.  There are also a number of methods for accessing
// backends by index, which is the index into the backends_ vector.
// For methods that take a start_index and stop_index, this refers to
// the indexes in the range [start_index, stop_index).  If stop_index
// is 0, backends_.size() is used.  Backends may or may not be
// xDS-enabled, at the discretion of the test.
class XdsEnd2endTest : public ::testing::TestWithParam<XdsTestType>,
                       public XdsResourceUtils {
 protected:
  // TLS certificate paths.
  static const char kCaCertPath[];
  static const char kServerCertPath[];
  static const char kServerKeyPath[];

  // Message used in EchoRequest to the backend.
  static const char kRequestMessage[];

  // A base class for server threads.
  class ServerThread {
   public:
    // A status notifier for xDS-enabled servers.
    //
    // TODO(yashykt): This notifier records the most recent state seen
    // for every URI and then lets the caller wait until the status for
    // that URI is the expected one.  If we are expecting an update that
    // has the same status as the previous one, then we really have no
    // way of knowing whether the second update has actually been sent.
    // A better approach here would be to queue the updates received by
    // the notifier and then have a method to get the next update from
    // the queue, if any.
    // Also, we should change the callers to check not just the status
    // but also the corresponding error message, so that we can verify
    // that we're emitting useful error messages for our users.
    class XdsServingStatusNotifier
        : public grpc::XdsServerServingStatusNotifierInterface {
     public:
      void OnServingStatusUpdate(std::string uri,
                                 ServingStatusUpdate update) override;

      GRPC_MUST_USE_RESULT bool WaitOnServingStatusChange(
          const std::string& uri, grpc::StatusCode expected_status,
          absl::Duration timeout = absl::Seconds(10));

     private:
      grpc_core::Mutex mu_;
      grpc_core::CondVar cond_;
      std::map<std::string, grpc::Status> status_map ABSL_GUARDED_BY(mu_);
    };

    // If use_xds_enabled_server is true, the server will use xDS.
    // If credentials is null, fake credentials will be used.
    explicit ServerThread(
        XdsEnd2endTest* test_obj, bool use_xds_enabled_server = false,
        std::shared_ptr<ServerCredentials> credentials = nullptr);

    virtual ~ServerThread() {
      // Shutdown should be called manually. Shutdown calls virtual methods and
      // can't be called from the base class destructor.
      CHECK(!running_);
    }

    void Start();
    void Shutdown();

    std::string target() const { return absl::StrCat("localhost:", port_); }

    int port() const { return port_; }

    XdsServingStatusNotifier* notifier() { return &notifier_; }

    GRPC_MUST_USE_RESULT bool WaitOnServingStatusChange(
        grpc::StatusCode expected_status,
        absl::Duration timeout = absl::Seconds(10)) {
      return notifier_.WaitOnServingStatusChange(
          grpc_core::LocalIpAndPort(port_), expected_status, timeout);
    }

    void set_allow_put_requests(bool allow_put_requests) {
      allow_put_requests_ = allow_put_requests;
    }

    void StopListening();

    void StopListeningAndSendGoaways();

   private:
    class XdsChannelArgsServerBuilderOption;

    virtual const char* Type() = 0;
    virtual void RegisterAllServices(ServerBuilder* builder) = 0;
    virtual void StartAllServices() = 0;
    virtual void ShutdownAllServices() = 0;

    void Serve(grpc_core::Mutex* mu, grpc_core::CondVar* cond);

    XdsEnd2endTest* test_obj_;
    const bool use_xds_enabled_server_;
    const std::shared_ptr<ServerCredentials> credentials_;
    const int port_;
    std::unique_ptr<Server> server_;
    XdsServingStatusNotifier notifier_;
    std::unique_ptr<std::thread> thread_;
    bool running_ = false;
    bool allow_put_requests_ = false;
  };

  // A server thread for a backend server.
  class BackendServerThread : public ServerThread {
   public:
    // A wrapper around the backend echo test service impl that counts
    // requests and responses.
    template <typename RpcService>
    class BackendServiceImpl
        : public CountedService<TestMultipleServiceImpl<RpcService>> {
     public:
      BackendServiceImpl() {}

      Status Echo(ServerContext* context, const EchoRequest* request,
                  EchoResponse* response) override {
        auto peer_identity = context->auth_context()->GetPeerIdentity();
        CountedService<
            TestMultipleServiceImpl<RpcService>>::IncreaseRequestCount();
        {
          grpc_core::MutexLock lock(&mu_);
          clients_.insert(context->peer());
          last_peer_identity_.clear();
          for (const auto& entry : peer_identity) {
            last_peer_identity_.emplace_back(entry.data(), entry.size());
          }
        }
        if (request->has_param() && request->param().has_backend_metrics()) {
          const auto& request_metrics = request->param().backend_metrics();
          auto* recorder = context->ExperimentalGetCallMetricRecorder();
          if (request_metrics.cpu_utilization() != 0) {
            recorder->RecordCpuUtilizationMetric(
                request_metrics.cpu_utilization());
          }
          if (request_metrics.mem_utilization() != 0) {
            recorder->RecordMemoryUtilizationMetric(
                request_metrics.mem_utilization());
          }
          if (request_metrics.application_utilization() != 0) {
            recorder->RecordApplicationUtilizationMetric(
                request_metrics.application_utilization());
          }
          for (const auto& [key, value] : request_metrics.named_metrics()) {
            char* key_copy = static_cast<char*>(
                grpc_call_arena_alloc(context->c_call(), key.size() + 1));
            strncpy(key_copy, key.data(), key.size());
            key_copy[key.size()] = '\0';
            recorder->RecordNamedMetric(key_copy, value);
          }
        }
        const auto status = TestMultipleServiceImpl<RpcService>::Echo(
            context, request, response);
        CountedService<
            TestMultipleServiceImpl<RpcService>>::IncreaseResponseCount();
        return status;
      }

      Status Echo1(ServerContext* context, const EchoRequest* request,
                   EchoResponse* response) override {
        return Echo(context, request, response);
      }

      Status Echo2(ServerContext* context, const EchoRequest* request,
                   EchoResponse* response) override {
        return Echo(context, request, response);
      }

      void Start() {}
      void Shutdown() {}

      std::set<std::string> clients() {
        grpc_core::MutexLock lock(&mu_);
        return clients_;
      }

      const std::vector<std::string>& last_peer_identity() {
        grpc_core::MutexLock lock(&mu_);
        return last_peer_identity_;
      }

     private:
      grpc_core::Mutex mu_;
      std::set<std::string> clients_ ABSL_GUARDED_BY(&mu_);
      std::vector<std::string> last_peer_identity_ ABSL_GUARDED_BY(&mu_);
    };

    // If use_xds_enabled_server is true, the server will use xDS.
    BackendServerThread(XdsEnd2endTest* test_obj, bool use_xds_enabled_server,
                        std::shared_ptr<ServerCredentials> credentials);

    BackendServiceImpl<grpc::testing::EchoTestService::Service>*
    backend_service() {
      return &backend_service_;
    }
    BackendServiceImpl<grpc::testing::EchoTest1Service::Service>*
    backend_service1() {
      return &backend_service1_;
    }
    BackendServiceImpl<grpc::testing::EchoTest2Service::Service>*
    backend_service2() {
      return &backend_service2_;
    }
    grpc::experimental::ServerMetricRecorder* server_metric_recorder() const {
      return server_metric_recorder_.get();
    }

   private:
    const char* Type() override { return "Backend"; }
    void RegisterAllServices(ServerBuilder* builder) override;
    void StartAllServices() override;
    void ShutdownAllServices() override;

    BackendServiceImpl<grpc::testing::EchoTestService::Service>
        backend_service_;
    BackendServiceImpl<grpc::testing::EchoTest1Service::Service>
        backend_service1_;
    BackendServiceImpl<grpc::testing::EchoTest2Service::Service>
        backend_service2_;
    std::unique_ptr<experimental::ServerMetricRecorder> server_metric_recorder_;
  };

  // A server thread for the xDS server.
  class BalancerServerThread : public ServerThread {
   public:
    explicit BalancerServerThread(
        XdsEnd2endTest* test_obj, absl::string_view debug_label,
        std::shared_ptr<ServerCredentials> credentials);

    AdsServiceImpl* ads_service() { return ads_service_.get(); }
    LrsServiceImpl* lrs_service() { return lrs_service_.get(); }

   private:
    const char* Type() override { return "Balancer"; }
    void RegisterAllServices(ServerBuilder* builder) override;
    void StartAllServices() override;
    void ShutdownAllServices() override;

    std::shared_ptr<AdsServiceImpl> ads_service_;
    std::shared_ptr<LrsServiceImpl> lrs_service_;
  };

  // RPC services used to talk to the backends.
  enum RpcService {
    SERVICE_ECHO,
    SERVICE_ECHO1,
    SERVICE_ECHO2,
  };

  // RPC methods used to talk to the backends.
  enum RpcMethod {
    METHOD_ECHO,
    METHOD_ECHO1,
    METHOD_ECHO2,
  };

  // If balancer_credentials is null, it defaults to fake credentials.
  explicit XdsEnd2endTest(
      std::shared_ptr<ServerCredentials> balancer_credentials = nullptr);

  void SetUp() override { InitClient(); }
  void TearDown() override;

  //
  // xDS server management
  //

  // Creates and starts a new balancer, running in its own thread.
  // Most tests will not need to call this; instead, they can use
  // balancer_, which is already populated with default resources.
  // If credentials is null, it defaults to fake credentials.
  std::unique_ptr<BalancerServerThread> CreateAndStartBalancer(
      absl::string_view debug_label = "",
      std::shared_ptr<ServerCredentials> credentials = nullptr);

  // Sets the Listener and RouteConfiguration resource on the specified
  // balancer.  If RDS is in use, they will be set as separate resources;
  // otherwise, the RouteConfig will be inlined into the Listener.
  void SetListenerAndRouteConfiguration(
      BalancerServerThread* balancer, Listener listener,
      const RouteConfiguration& route_config,
      const HcmAccessor& hcm_accessor = ClientHcmAccessor()) {
    XdsResourceUtils::SetListenerAndRouteConfiguration(
        balancer->ads_service(), std::move(listener), route_config,
        GetParam().enable_rds_testing(), hcm_accessor);
  }

  // A convenient wrapper for setting the Listener and
  // RouteConfiguration resources on the server side.
  void SetServerListenerNameAndRouteConfiguration(
      BalancerServerThread* balancer, Listener listener, int port,
      const RouteConfiguration& route_config) {
    SetListenerAndRouteConfiguration(
        balancer, PopulateServerListenerNameAndPort(listener, port),
        route_config, ServerHcmAccessor());
  }

  // Sets the RouteConfiguration resource on the specified balancer.
  // If RDS is in use, it will be set directly as an independent
  // resource; otherwise, it will be inlined into a Listener resource
  // (either listener_to_copy, or if that is null, default_listener_).
  void SetRouteConfiguration(BalancerServerThread* balancer,
                             const RouteConfiguration& route_config,
                             const Listener* listener_to_copy = nullptr) {
    XdsResourceUtils::SetRouteConfiguration(
        balancer->ads_service(), route_config, GetParam().enable_rds_testing(),
        listener_to_copy);
  }

  // Helper method for generating an endpoint for a backend, for use in
  // constructing an EDS resource.
  EdsResourceArgs::Endpoint CreateEndpoint(
      size_t backend_idx,
      ::envoy::config::core::v3::HealthStatus health_status =
          ::envoy::config::core::v3::HealthStatus::UNKNOWN,
      int lb_weight = 1, std::vector<size_t> additional_backend_indexes = {},
      absl::string_view hostname = "",
      const std::map<std::string, std::string /*JSON*/>& metadata = {}) {
    std::vector<int> additional_ports;
    additional_ports.reserve(additional_backend_indexes.size());
    for (size_t idx : additional_backend_indexes) {
      additional_ports.push_back(backends_[idx]->port());
    }
    return EdsResourceArgs::Endpoint(backends_[backend_idx]->port(),
                                     health_status, lb_weight, additional_ports,
                                     hostname, metadata);
  }

  // Creates a vector of endpoints for a specified range of backends,
  // for use in constructing an EDS resource.
  std::vector<EdsResourceArgs::Endpoint> CreateEndpointsForBackends(
      size_t start_index = 0, size_t stop_index = 0,
      ::envoy::config::core::v3::HealthStatus health_status =
          ::envoy::config::core::v3::HealthStatus::UNKNOWN,
      int lb_weight = 1);

  // Returns an endpoint for an unused port, for use in constructing an
  // EDS resource.
  EdsResourceArgs::Endpoint MakeNonExistentEndpoint() {
    return EdsResourceArgs::Endpoint(grpc_pick_unused_port_or_die());
  }

  //
  // Backend management
  //

  // Creates num_backends backends and stores them in backends_, but does
  // not actually start them.  If xds_enabled is true, the backends are
  // xDS-enabled.  If credentials is null, it defaults to fake credentials.
  void CreateBackends(
      size_t num_backends, bool xds_enabled = false,
      std::shared_ptr<ServerCredentials> credentials = nullptr) {
    for (size_t i = 0; i < num_backends; ++i) {
      backends_.emplace_back(
          new BackendServerThread(this, xds_enabled, credentials));
    }
  }

  // Starts all backends in backends_.
  void StartAllBackends() {
    for (auto& backend : backends_) backend->Start();
  }

  // Same as CreateBackends(), but also starts the backends.
  void CreateAndStartBackends(
      size_t num_backends, bool xds_enabled = false,
      std::shared_ptr<ServerCredentials> credentials = nullptr) {
    CreateBackends(num_backends, xds_enabled, std::move(credentials));
    StartAllBackends();
  }

  // Starts the backend at backends_[index].
  void StartBackend(size_t index) { backends_[index]->Start(); }

  // Shuts down all backends in backends_.
  void ShutdownAllBackends() {
    for (auto& backend : backends_) backend->Shutdown();
  }

  // Shuts down the backend at backends_[index].
  void ShutdownBackend(size_t index) { backends_[index]->Shutdown(); }

  // Resets the request counters for backends in the specified range.
  void ResetBackendCounters(size_t start_index = 0, size_t stop_index = 0);

  // Returns true if the specified backend has received requests for the
  // specified service.
  bool SeenBackend(size_t backend_idx,
                   const RpcService rpc_service = SERVICE_ECHO);

  // Returns true if all backends in the specified range have received
  // requests for the specified service.
  bool SeenAllBackends(size_t start_index = 0, size_t stop_index = 0,
                       const RpcService rpc_service = SERVICE_ECHO);

  // Returns a vector containing the port for every backend in the
  // specified range.
  std::vector<int> GetBackendPorts(size_t start_index = 0,
                                   size_t stop_index = 0) const;

  //
  // Client management
  //

  // Initializes global state for the client, such as the bootstrap file
  // and channel args for the XdsClient.  Then calls ResetStub().
  // All tests must call this exactly once at the start of the test.
  // If credentials is null, fake credentials will be used.
  void InitClient(std::optional<XdsBootstrapBuilder> builder = std::nullopt,
                  std::string lb_expected_authority = "",
                  int xds_resource_does_not_exist_timeout_ms = 0,
                  std::string balancer_authority_override = "",
                  ChannelArguments* args = nullptr,
                  std::shared_ptr<ChannelCredentials> credentials = nullptr);

  XdsBootstrapBuilder MakeBootstrapBuilder() {
    return XdsBootstrapBuilder().SetServers(
        {absl::StrCat("localhost:", balancer_->port())});
  }

  // Sets channel_, stub_, stub1_, and stub2_.
  // If credentials is null, fake credentials will be used.
  void ResetStub(int failover_timeout_ms = 0, ChannelArguments* args = nullptr,
                 std::shared_ptr<ChannelCredentials> credentials = nullptr);

  // Creates a new client channel.  Requires that InitClient() has
  // already been called.
  // If credentials is null, fake credentials will be used.
  std::shared_ptr<Channel> CreateChannel(
      int failover_timeout_ms = 0, const char* server_name = kServerName,
      const char* xds_authority = "", ChannelArguments* args = nullptr,
      std::shared_ptr<ChannelCredentials> credentials = nullptr);

  //
  // Sending RPCs
  //

  // Options used for sending an RPC.
  struct RpcOptions {
    RpcService service = SERVICE_ECHO;
    RpcMethod method = METHOD_ECHO;
    // Will be multiplied by grpc_test_slowdown_factor().
    int timeout_ms = 5000;
    bool wait_for_ready = false;
    std::vector<std::pair<std::string, std::string>> metadata;
    // These options are used by the backend service impl.
    bool server_fail = false;
    int server_sleep_us = 0;
    int client_cancel_after_us = 0;
    bool skip_cancelled_check = false;
    StatusCode server_expected_error = StatusCode::OK;
    std::optional<xds::data::orca::v3::OrcaLoadReport> backend_metrics;
    bool server_notify_client_when_started = false;
    bool echo_host_from_authority_header = false;
    bool echo_metadata_initially = false;

    RpcOptions() {}

    RpcOptions& set_rpc_service(RpcService rpc_service) {
      service = rpc_service;
      return *this;
    }

    RpcOptions& set_rpc_method(RpcMethod rpc_method) {
      method = rpc_method;
      return *this;
    }

    RpcOptions& set_timeout_ms(int rpc_timeout_ms) {
      timeout_ms = rpc_timeout_ms;
      return *this;
    }

    RpcOptions& set_timeout(grpc_core::Duration timeout) {
      timeout_ms = timeout.millis();
      return *this;
    }

    RpcOptions& set_wait_for_ready(bool rpc_wait_for_ready) {
      wait_for_ready = rpc_wait_for_ready;
      return *this;
    }

    RpcOptions& set_metadata(
        std::vector<std::pair<std::string, std::string>> rpc_metadata) {
      metadata = std::move(rpc_metadata);
      return *this;
    }

    RpcOptions& set_server_fail(bool rpc_server_fail) {
      server_fail = rpc_server_fail;
      return *this;
    }

    RpcOptions& set_server_sleep_us(int rpc_server_sleep_us) {
      server_sleep_us = rpc_server_sleep_us;
      return *this;
    }

    RpcOptions& set_client_cancel_after_us(int rpc_client_cancel_after_us) {
      client_cancel_after_us = rpc_client_cancel_after_us;
      return *this;
    }

    RpcOptions& set_skip_cancelled_check(bool rpc_skip_cancelled_check) {
      skip_cancelled_check = rpc_skip_cancelled_check;
      return *this;
    }

    RpcOptions& set_server_expected_error(StatusCode code) {
      server_expected_error = code;
      return *this;
    }

    RpcOptions& set_backend_metrics(
        std::optional<xds::data::orca::v3::OrcaLoadReport> metrics) {
      backend_metrics = std::move(metrics);
      return *this;
    }

    RpcOptions& set_server_notify_client_when_started(
        bool rpc_server_notify_client_when_started) {
      server_notify_client_when_started = rpc_server_notify_client_when_started;
      return *this;
    }

    RpcOptions& set_echo_host_from_authority_header(bool value) {
      echo_host_from_authority_header = value;
      return *this;
    }

    RpcOptions& set_echo_metadata_initially(bool value) {
      echo_metadata_initially = value;
      return *this;
    }

    // Populates context and request.
    void SetupRpc(ClientContext* context, EchoRequest* request) const;
  };

  // Sends an RPC with the specified options.
  // If response is non-null, it will be populated with the response.
  // Returns the status of the RPC.
  Status SendRpc(const RpcOptions& rpc_options = RpcOptions(),
                 EchoResponse* response = nullptr,
                 std::multimap<std::string, std::string>*
                     server_initial_metadata = nullptr);

  // Internal helper function for SendRpc().
  template <typename Stub>
  static Status SendRpcMethod(Stub* stub, const RpcOptions& rpc_options,
                              ClientContext* context, EchoRequest& request,
                              EchoResponse* response) {
    switch (rpc_options.method) {
      case METHOD_ECHO:
        return stub->Echo(context, request, response);
      case METHOD_ECHO1:
        return stub->Echo1(context, request, response);
      case METHOD_ECHO2:
        return stub->Echo2(context, request, response);
    }
    GPR_UNREACHABLE_CODE(return grpc::Status::OK);
  }

  // Send RPCs in a loop until either continue_predicate() returns false
  // or timeout_ms elapses.
  struct RpcResult {
    Status status;
    EchoResponse response;
  };
  void SendRpcsUntil(const grpc_core::DebugLocation& debug_location,
                     std::function<bool(const RpcResult&)> continue_predicate,
                     int timeout_ms = 15000,
                     const RpcOptions& rpc_options = RpcOptions());

  // Sends the specified number of RPCs and fails if the RPC fails.
  void CheckRpcSendOk(const grpc_core::DebugLocation& debug_location,
                      const size_t times = 1,
                      const RpcOptions& rpc_options = RpcOptions());

  // Sends one RPC, which must fail with the specified status code and
  // a message matching the specified regex.
  void CheckRpcSendFailure(const grpc_core::DebugLocation& debug_location,
                           StatusCode expected_status,
                           absl::string_view expected_message_regex,
                           const RpcOptions& rpc_options = RpcOptions());

  // Sends RPCs until either a timeout or an RPC fail, in which case the
  // failure must match the specified status and message regex.
  void SendRpcsUntilFailure(const grpc_core::DebugLocation& debug_location,
                            StatusCode expected_status,
                            absl::string_view expected_message_regex,
                            int timeout_ms = 15000,
                            const RpcOptions& rpc_options = RpcOptions());

  // Sends num_rpcs RPCs, counting how many of them fail with a message
  // matching the specified expected_message_prefix.
  // Any failure with a non-matching status or message is a test failure.
  size_t SendRpcsAndCountFailuresWithMessage(
      const grpc_core::DebugLocation& debug_location, size_t num_rpcs,
      StatusCode expected_status, absl::string_view expected_message_prefix,
      const RpcOptions& rpc_options = RpcOptions());

  // A class for running a long-running RPC in its own thread.
  // TODO(roth): Maybe consolidate this and SendConcurrentRpcs()
  // somehow?  LongRunningRpc has a cleaner API, but SendConcurrentRpcs()
  // uses the callback API, which is probably better.
  class LongRunningRpc {
   public:
    // Starts the RPC.
    void StartRpc(grpc::testing::EchoTestService::Stub* stub,
                  const RpcOptions& rpc_options =
                      RpcOptions().set_timeout_ms(0).set_client_cancel_after_us(
                          1 * 1000 * 1000));

    // Cancels the RPC.
    void CancelRpc();

    // Gets the RPC's status.  Blocks if the RPC is not yet complete.
    Status GetStatus();

   private:
    std::thread sender_thread_;
    ClientContext context_;
    Status status_;
  };

  // Starts a set of concurrent RPCs.
  struct ConcurrentRpc {
    ClientContext context;
    Status status;
    grpc_core::Duration elapsed_time;
    EchoResponse response;
  };
  std::vector<std::unique_ptr<ConcurrentRpc>> SendConcurrentRpcs(
      const grpc_core::DebugLocation& debug_location,
      grpc::testing::EchoTestService::Stub* stub, size_t num_rpcs,
      const RpcOptions& rpc_options);

  //
  // Waiting for individual backends to be seen by the client
  //

  struct WaitForBackendOptions {
    // If true, resets the backend counters before returning.
    bool reset_counters = true;
    // How long to wait for the backend(s) to see requests.
    // Will be multiplied by grpc_test_slowdown_factor().
    int timeout_ms = 15000;

    WaitForBackendOptions() {}

    WaitForBackendOptions& set_reset_counters(bool enable) {
      reset_counters = enable;
      return *this;
    }

    WaitForBackendOptions& set_timeout_ms(int ms) {
      timeout_ms = ms;
      return *this;
    }
  };

  // Sends RPCs until all of the backends in the specified range see requests.
  // The check_status callback will be invoked to check the status of
  // every RPC; if null, the default is to check that the RPC succeeded.
  // Returns the total number of RPCs sent.
  size_t WaitForAllBackends(
      const grpc_core::DebugLocation& debug_location, size_t start_index = 0,
      size_t stop_index = 0,
      std::function<void(const RpcResult&)> check_status = nullptr,
      const WaitForBackendOptions& wait_options = WaitForBackendOptions(),
      const RpcOptions& rpc_options = RpcOptions());

  // Sends RPCs until the backend at index backend_idx sees requests.
  void WaitForBackend(
      const grpc_core::DebugLocation& debug_location, size_t backend_idx,
      std::function<void(const RpcResult&)> check_status = nullptr,
      const WaitForBackendOptions& wait_options = WaitForBackendOptions(),
      const RpcOptions& rpc_options = RpcOptions()) {
    WaitForAllBackends(debug_location, backend_idx, backend_idx + 1,
                       check_status, wait_options, rpc_options);
  }

  //
  // Waiting for xDS NACKs
  //
  // These methods send RPCs in a loop until they see a NACK from the
  // xDS server, or until a timeout expires.

  // Sends RPCs until get_state() returns a response.
  // TODO(roth): Does this actually need to send RPCs, or can it just
  // use a condition variable to wait?  I suspect that we need to be
  // sending RPCs for polling reasons, but that should go away when we
  // finish the EventEngine migration.  Once that's done, try changing
  // this to not send RPCs.
  // Also, consider refactoring to also support waiting for ACKs, since
  // there are several use-cases where tests are doing that.
  std::optional<AdsServiceImpl::ResponseState> WaitForNack(
      const grpc_core::DebugLocation& debug_location,
      std::function<std::optional<AdsServiceImpl::ResponseState>()> get_state,
      const RpcOptions& rpc_options = RpcOptions(),
      StatusCode expected_status = StatusCode::UNAVAILABLE);

  // Sends RPCs until an LDS NACK is seen.
  std::optional<AdsServiceImpl::ResponseState> WaitForLdsNack(
      const grpc_core::DebugLocation& debug_location,
      const RpcOptions& rpc_options = RpcOptions(),
      StatusCode expected_status = StatusCode::UNAVAILABLE) {
    return WaitForNack(
        debug_location,
        [&]() { return balancer_->ads_service()->lds_response_state(); },
        rpc_options, expected_status);
  }

  // Sends RPCs until an RDS NACK is seen.
  std::optional<AdsServiceImpl::ResponseState> WaitForRdsNack(
      const grpc_core::DebugLocation& debug_location,
      const RpcOptions& rpc_options = RpcOptions(),
      StatusCode expected_status = StatusCode::UNAVAILABLE) {
    return WaitForNack(
        debug_location,
        [&]() { return RouteConfigurationResponseState(balancer_.get()); },
        rpc_options, expected_status);
  }

  // Sends RPCs until a CDS NACK is seen.
  std::optional<AdsServiceImpl::ResponseState> WaitForCdsNack(
      const grpc_core::DebugLocation& debug_location,
      const RpcOptions& rpc_options = RpcOptions(),
      StatusCode expected_status = StatusCode::UNAVAILABLE) {
    return WaitForNack(
        debug_location,
        [&]() { return balancer_->ads_service()->cds_response_state(); },
        rpc_options, expected_status);
  }

  // Sends RPCs until an EDS NACK is seen.
  std::optional<AdsServiceImpl::ResponseState> WaitForEdsNack(
      const grpc_core::DebugLocation& debug_location,
      const RpcOptions& rpc_options = RpcOptions()) {
    return WaitForNack(
        debug_location,
        [&]() { return balancer_->ads_service()->eds_response_state(); },
        rpc_options);
  }

  // Convenient front-end to wait for RouteConfiguration to be NACKed,
  // regardless of whether it's sent in LDS or RDS.
  std::optional<AdsServiceImpl::ResponseState> WaitForRouteConfigNack(
      const grpc_core::DebugLocation& debug_location,
      const RpcOptions& rpc_options = RpcOptions(),
      StatusCode expected_status = StatusCode::UNAVAILABLE) {
    if (GetParam().enable_rds_testing()) {
      return WaitForRdsNack(debug_location, rpc_options, expected_status);
    }
    return WaitForLdsNack(debug_location, rpc_options, expected_status);
  }

  // Convenient front-end for accessing xDS response state for a
  // RouteConfiguration, regardless of whether it's sent in LDS or RDS.
  std::optional<AdsServiceImpl::ResponseState> RouteConfigurationResponseState(
      BalancerServerThread* balancer) const {
    AdsServiceImpl* ads_service = balancer->ads_service();
    if (GetParam().enable_rds_testing()) {
      return ads_service->rds_response_state();
    }
    return ads_service->lds_response_state();
  }

  //
  // Miscellaneous helper methods
  //

  // There is slight difference between time fetched by GPR and by C++ system
  // clock API. It's unclear if they are using the same syscall, but we do know
  // GPR round the number at millisecond-level. This creates a 1ms difference,
  // which could cause flake.
  static grpc_core::Timestamp NowFromCycleCounter() {
    return grpc_core::Timestamp::FromTimespecRoundDown(
        gpr_now(GPR_CLOCK_MONOTONIC));
  }

  // Sets duration_proto to duration times grpc_test_slowdown_factor().
  static void SetProtoDuration(grpc_core::Duration duration,
                               google::protobuf::Duration* duration_proto);

  // Returns the number of RPCs needed to pass error_tolerance at 99.99994%
  // chance. Rolling dices in drop/fault-injection generates a binomial
  // distribution (if our code is not horribly wrong). Let's make "n" the number
  // of samples, "p" the probability. If we have np>5 & n(1-p)>5, we can
  // approximately treat the binomial distribution as a normal distribution.
  //
  // For normal distribution, we can easily look up how many standard deviation
  // we need to reach 99.995%. Based on Wiki's table
  // https://en.wikipedia.org/wiki/68%E2%80%9395%E2%80%9399.7_rule, we need 5.00
  // sigma (standard deviation) to cover the probability area of 99.99994%. In
  // another word, for a sample with size "n" probability "p" error-tolerance
  // "k", we want the error always land within 5.00 sigma. The sigma of
  // binomial distribution and be computed as sqrt(np(1-p)). Hence, we have
  // the equation:
  //
  //   kn <= 5.00 * sqrt(np(1-p))
  // TODO(yashykt): The above explanation assumes a normal distribution, but we
  // use a uniform distribution instead. We need a better estimate of how many
  // RPCs are needed with what error tolerance.
  static size_t ComputeIdealNumRpcs(double p, double error_tolerance) {
    CHECK_GE(p, 0);
    CHECK_LE(p, 1);
    size_t num_rpcs =
        ceil(p * (1 - p) * 5.00 * 5.00 / error_tolerance / error_tolerance);
    num_rpcs += 1000;  // Add 1K as a buffer to avoid flakiness.
    LOG(INFO) << "Sending " << num_rpcs << " RPCs for percentage=" << p
              << " error_tolerance=" << error_tolerance;
    return num_rpcs;
  }

  // Returns a regex that can be matched against an RPC failure status
  // message for a connection failure.
  static std::string MakeConnectionFailureRegex(
      absl::string_view prefix, bool has_resolution_note = true);

  // Returns a regex that can be matched against an RPC failure status
  // message for a Tls handshake failure.
  static std::string MakeTlsHandshakeFailureRegex(absl::string_view prefix);

  // Returns a private key pair, read from local files.
  static grpc_core::PemKeyCertPairList ReadTlsIdentityPair(
      const char* key_path, const char* cert_path);

  // Internal helper function for creating TLS and mTLS creds.
  // Not intended to be used by tests.
  static std::vector<experimental::IdentityKeyCertPair>
  MakeIdentityKeyCertPairForTlsCreds();

  // Returns XdsCredentials with mTLS fallback creds.
  static std::shared_ptr<ChannelCredentials> CreateXdsChannelCredentials();
  static std::shared_ptr<ChannelCredentials> CreateTlsChannelCredentials();

  // Creates various types of server credentials.
  static std::shared_ptr<ServerCredentials> CreateFakeServerCredentials();
  static std::shared_ptr<ServerCredentials> CreateMtlsServerCredentials();
  static std::shared_ptr<ServerCredentials> CreateTlsServerCredentials();

  // event_engine_scope_ always has to be at the top of the list to make sure
  // that all other objects are destroyed before this and other event engine
  // refs are released.
  // We are using DefaultEventEngineScope to make sure that all work from the
  // current test is done before the next test starts. Without this, we run into
  // errors such as https://github.com/grpc/grpc/issues/38588. Note that we are
  // using all refs to current event engine going down to 0 as a proxy for
  // checking that all work from the current test is done, but this is not a
  // guaranteed way since there might still be threads that are not using event
  // engine but are still accessing/modifying the system state.
  grpc_event_engine::experimental::DefaultEventEngineScope scoped_event_engine_;

  std::unique_ptr<BalancerServerThread> balancer_;

  std::shared_ptr<Channel> channel_;
  std::unique_ptr<grpc::testing::EchoTestService::Stub> stub_;
  std::unique_ptr<grpc::testing::EchoTest1Service::Stub> stub1_;
  std::unique_ptr<grpc::testing::EchoTest2Service::Stub> stub2_;

  std::vector<std::unique_ptr<BackendServerThread>> backends_;

  // Default xDS resources.
  Listener default_listener_;
  RouteConfiguration default_route_config_;
  Listener default_server_listener_;
  RouteConfiguration default_server_route_config_;
  Cluster default_cluster_;

  int xds_drain_grace_time_ms_ = 10 * 60 * 1000;  // 10 mins

  bool bootstrap_contents_from_env_var_;
  std::string bootstrap_;
  char* bootstrap_file_ = nullptr;
  absl::InlinedVector<grpc_arg, 3> xds_channel_args_to_add_;
  grpc_channel_args xds_channel_args_;
};

}  // namespace testing
}  // namespace grpc

#endif  // GRPC_TEST_CPP_END2END_XDS_XDS_END2END_TEST_LIB_H
