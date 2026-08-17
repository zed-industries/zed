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

#ifndef GRPC_TEST_CORE_END2END_END2END_TESTS_H
#define GRPC_TEST_CORE_END2END_END2END_TESTS_H

#include <grpc/byte_buffer.h>
#include <grpc/compression.h>
#include <grpc/credentials.h>
#include <grpc/event_engine/event_engine.h>
#include <grpc/grpc.h>
#include <grpc/grpc_security.h>
#include <grpc/impl/propagation_bits.h>
#include <grpc/status.h>
#include <grpc/support/alloc.h>
#include <grpc/support/time.h>
#include <stdint.h>
#include <stdio.h>

#include <algorithm>
#include <functional>
#include <initializer_list>
#include <map>
#include <memory>
#include <optional>
#include <ostream>
#include <string>
#include <utility>
#include <variant>
#include <vector>

#include "absl/functional/any_invocable.h"
#include "absl/log/check.h"
#include "absl/memory/memory.h"
#include "absl/meta/type_traits.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/string_view.h"
#include "gtest/gtest.h"
#include "src/core/config/config_vars.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/slice/slice.h"
#include "src/core/lib/slice/slice_internal.h"
#include "src/core/lib/surface/call_test_only.h"
#include "src/core/lib/surface/channel.h"
#include "src/core/util/bitset.h"
#include "src/core/util/debug_location.h"
#include "src/core/util/time.h"
#include "src/core/util/wait_for_single_owner.h"
#include "test/core/call/batch_builder.h"
#include "test/core/end2end/cq_verifier.h"
#include "test/core/end2end/end2end_test_fuzzer.pb.h"
#include "test/core/event_engine/event_engine_test_utils.h"
#include "test/core/test_util/fuzz_config_vars.h"
#include "test/core/test_util/postmortem.h"
#include "test/core/test_util/test_config.h"

#ifdef GRPC_END2END_TEST_INCLUDE_FUZZER
#include "fuzztest/fuzztest.h"
#include "test/core/test_util/fuzz_config_vars_helpers.h"
#endif

#define CA_CERT_PATH "src/core/tsi/test_creds/ca.pem"

// Test feature flags.
#define FEATURE_MASK_DOES_NOT_SUPPORT_RETRY (1 << 0)
#define FEATURE_MASK_SUPPORTS_HOSTNAME_VERIFICATION (1 << 1)
// Feature mask supports call credentials with a minimum security level of
// GRPC_PRIVACY_AND_INTEGRITY.
#define FEATURE_MASK_SUPPORTS_PER_CALL_CREDENTIALS (1 << 2)
// Feature mask supports call credentials with a minimum security level of
// GRPC_SECURITY_NONE.
#define FEATURE_MASK_SUPPORTS_PER_CALL_CREDENTIALS_LEVEL_INSECURE (1 << 3)
#define FEATURE_MASK_SUPPORTS_REQUEST_PROXYING (1 << 4)
#define FEATURE_MASK_SUPPORTS_CLIENT_CHANNEL (1 << 5)
#define FEATURE_MASK_IS_HTTP2 (1 << 6)
#define FEATURE_MASK_ENABLES_TRACES (1 << 7)
#define FEATURE_MASK_1BYTE_AT_A_TIME (1 << 8)
#define FEATURE_MASK_DOES_NOT_SUPPORT_WRITE_BUFFERING (1 << 9)
#define FEATURE_MASK_DOES_NOT_SUPPORT_CLIENT_HANDSHAKE_COMPLETE_FIRST (1 << 10)
#define FEATURE_MASK_IS_MINSTACK (1 << 11)
#define FEATURE_MASK_IS_SECURE (1 << 12)
#define FEATURE_MASK_DO_NOT_FUZZ (1 << 13)
#define FEATURE_MASK_DO_NOT_GTEST (1 << 14)
// Exclude this fixture from experiment runs
#define FEATURE_MASK_EXCLUDE_FROM_EXPERIMENT_RUNS (1 << 15)
#define FEATURE_MASK_IS_CALL_V3 (1 << 16)
#define FEATURE_MASK_IS_LOCAL_TCP_CREDS (1 << 17)

#define FAIL_AUTH_CHECK_SERVER_ARG_NAME "fail_auth_check"

namespace grpc_core {

class CoreTestFixture {
 public:
  virtual ~CoreTestFixture() = default;

  virtual grpc_server* MakeServer(
      const ChannelArgs& args, grpc_completion_queue* cq,
      absl::AnyInvocable<void(grpc_server*)>& pre_server_start) = 0;
  virtual grpc_channel* MakeClient(const ChannelArgs& args,
                                   grpc_completion_queue* cq) = 0;
};

Slice RandomSlice(size_t length);
Slice RandomBinarySlice(size_t length);

struct CoreTestConfiguration {
  // A descriptive name for this test fixture.
  const char* name;

  // Which features are supported by this fixture. See feature flags above.
  uint32_t feature_mask;

  // If the call host is setup by the fixture (for example, via the
  // GRPC_SSL_TARGET_NAME_OVERRIDE_ARG channel arg), which value should the
  // test expect to find in call_details.host
  const char* overridden_call_host;

  std::function<std::unique_ptr<CoreTestFixture>(
      const ChannelArgs& client_args, const ChannelArgs& server_args)>
      create_fixture;
};

const CoreTestConfiguration* CoreTestConfigurationNamed(absl::string_view name);

template <typename Sink>
void AbslStringify(Sink& sink, const CoreTestConfiguration* config) {
  sink.Append(
      absl::StrCat("CoreTestConfigurationNamed(\"", config->name, "\")"));
}

inline std::ostream& operator<<(std::ostream& out,
                                const CoreTestConfiguration* config) {
  return out << "CoreTestConfigurationNamed(\"" << config->name << "\")";
}

// Base class for e2e tests.
//
// Initialization:
//
// At the start of a test, nothing is initialized. CoreConfiguration is reset,
// and there's certainly no server nor client.
// A test can then register whatever builders it wants into the
// CoreConfiguration and have them picked up. If it does not, it will get the
// default CoreConfiguration.
//
// The test may choose to then create a client and server with InitClient() and
// InitServer(). It does not matter which order they are called, nor whether one
// or both are called. It's necessary to call these if the test demands that
// non-empty channel args should be passed to either the client or server.
//
// If a test does not call InitClient() or InitServer(), then upon the first
// call to either NewClientCall() or NewServerCall(), the client and server will
// be instantiated - this saves substantial boilerplate in the most common case
// for our tests.
//
// Notes:
// - older compilers fail matching absl::string_view with some gmock matchers on
//   older compilers, and it's tremendously convenient to be able to do so. So
//   we use std::string for return types here - performance isn't particularly
//   important, so an extra copy is fine.
class CoreEnd2endTest {
 public:
  CoreEnd2endTest(const CoreTestConfiguration* config,
                  const core_end2end_test_fuzzer::Msg* fuzzing_args,
                  absl::string_view suite_name);
  ~CoreEnd2endTest();

  void SetCqVerifierStepFn(
      absl::AnyInvocable<
          void(grpc_event_engine::experimental::EventEngine::Duration) const>
          step_fn) {
    step_fn_ = std::move(step_fn);
  }
  void SetQuiesceEventEngine(
      absl::AnyInvocable<
          void(std::shared_ptr<grpc_event_engine::experimental::EventEngine>&&)>
          quiesce_event_engine) {
    quiesce_event_engine_ = std::move(quiesce_event_engine);
  }

  class Call;
  struct RegisteredCall {
    void* p;
  };

  // Safe notification to use for core e2e tests.
  // Since when we're fuzzing we don't run background threads, the normal
  // Notification type isn't safe to wait on (for some background timer to fire
  // for instance...), consequently we need to use this.
  class TestNotification {
   public:
    explicit TestNotification(CoreEnd2endTest* test) : test_(test) {}

    bool WaitForNotificationWithTimeout(absl::Duration wait_time) {
      if (test_->fuzzing_) {
        Timestamp end = Timestamp::Now() + Duration::NanosecondsRoundUp(
                                               ToInt64Nanoseconds(wait_time));
        while (true) {
          if (base_.HasBeenNotified()) return true;
          auto now = Timestamp::Now();
          if (now >= end) return false;
          test_->step_fn_(now - end);
        }
      } else {
        return base_.WaitForNotificationWithTimeout(wait_time);
      }
    }

    void Notify() { base_.Notify(); }

   private:
    Notification base_;
    CoreEnd2endTest* const test_;
  };

  // CallBuilder - results in a call to either grpc_channel_create_call or
  // grpc_channel_create_registered_call.
  // Affords a fluent interface to specify optional arguments.
  class ClientCallBuilder {
   public:
    ClientCallBuilder(CoreEnd2endTest& test, std::string method)
        : test_(test),
          call_selector_(UnregisteredCall{std::move(method), std::nullopt}) {}
    ClientCallBuilder(CoreEnd2endTest& test, RegisteredCall registered_call)
        : test_(test), call_selector_(registered_call.p) {}

    // Specify the host (otherwise nullptr is passed)
    ClientCallBuilder& Host(std::string host) {
      std::get<UnregisteredCall>(call_selector_).host = std::move(host);
      return *this;
    }
    // Specify the timeout (otherwise gpr_inf_future is passed) - this time is
    // scaled according to the test environment.
    ClientCallBuilder& Timeout(Duration timeout) {
      if (timeout == Duration::Infinity()) {
        deadline_ = gpr_inf_future(GPR_CLOCK_REALTIME);
        return *this;
      }
      deadline_ = grpc_timeout_milliseconds_to_deadline(timeout.millis());
      return *this;
    }
    // Finally create the call.
    Call Create();

   private:
    CoreEnd2endTest& test_;
    struct UnregisteredCall {
      std::string method;
      std::optional<std::string> host;
    };
    std::variant<void*, UnregisteredCall> call_selector_;
    grpc_call* parent_call_ = nullptr;
    uint32_t propagation_mask_ = GRPC_PROPAGATE_DEFAULTS;
    gpr_timespec deadline_ = gpr_inf_future(GPR_CLOCK_REALTIME);
  };

  // Wrapper around a grpc_call.
  // Instantiated by ClientCallBuilder via NewClientCall for client calls.
  // Wrapped by IncomingCall for server calls.
  class Call {
   public:
    Call(grpc_call* call, CoreEnd2endTest* test) : call_(call), test_(test) {}
    Call(const Call&) = delete;
    Call& operator=(const Call&) = delete;
    Call(Call&& other) noexcept
        : call_(std::exchange(other.call_, nullptr)),
          test_(std::exchange(other.test_, nullptr)) {}
    ~Call() {
      if (call_ != nullptr) grpc_call_unref(call_);
    }
    // Construct a batch with a tag - upon destruction of the BatchBuilder the
    // operation will occur.
    BatchBuilder NewBatch(int tag) {
      return BatchBuilder(call_, &test_->cq_verifier(), tag);
    }
    // Cancel the call
    void Cancel() { grpc_call_cancel(call_, nullptr); }
    void CancelWithStatus(grpc_status_code status, const char* message) {
      grpc_call_cancel_with_status(call_, status, message, nullptr);
    }
    // Access the peer structure (returns a string that can be matched, etc) -
    // or nullopt if grpc_call_get_peer returns nullptr.
    std::optional<std::string> GetPeer() {
      char* peer = grpc_call_get_peer(call_);
      if (peer == nullptr) return std::nullopt;
      std::string result(peer);
      gpr_free(peer);
      return result;
    }

    // Set call credentials.
    // Takes ownership of creds.
    void SetCredentials(grpc_call_credentials* creds) {
      EXPECT_EQ(grpc_call_set_credentials(call_, creds), GRPC_CALL_OK);
      grpc_call_credentials_release(creds);
    }

    // Retrieve the auth context.
    std::unique_ptr<grpc_auth_context, void (*)(grpc_auth_context*)>
    GetAuthContext() {
      return std::unique_ptr<grpc_auth_context, void (*)(grpc_auth_context*)>(
          grpc_call_auth_context(call_), grpc_auth_context_release);
    }

    grpc_call** call_ptr() { return &call_; }
    grpc_call* c_call() const { return call_; }

   private:
    grpc_call* call_ = nullptr;
    CoreEnd2endTest* test_;
  };

  // Wrapper around a server call.
  class IncomingCall {
   public:
    IncomingCall(CoreEnd2endTest& test, int tag);
    IncomingCall(CoreEnd2endTest& test, void* method, IncomingMessage* message,
                 int tag);
    IncomingCall(const IncomingCall&) = delete;
    IncomingCall& operator=(const IncomingCall&) = delete;
    IncomingCall(IncomingCall&&) noexcept = default;

    // Construct a batch with a tag - upon destruction of the BatchBuilder the
    // operation will occur. Must have received the call first!
    BatchBuilder NewBatch(int tag) { return impl_->call.NewBatch(tag); }
    void Cancel() { impl_->call.Cancel(); }

    // Return the method being called.
    std::string method() const {
      return std::string(StringViewFromSlice(impl_->call_details.method));
    }

    // Return the host being called.
    std::string host() const {
      return std::string(StringViewFromSlice(impl_->call_details.host));
    }

    // Return some initial metadata.
    std::optional<std::string> GetInitialMetadata(absl::string_view key) const;

    // Return the peer address.
    std::optional<std::string> GetPeer() { return impl_->call.GetPeer(); }

    // Return the auth context.
    std::unique_ptr<grpc_auth_context, void (*)(grpc_auth_context*)>
    GetAuthContext() {
      return impl_->call.GetAuthContext();
    }

    // Return the underlying C call object
    grpc_call* c_call() { return impl_->call.c_call(); }

    // Return the encodings accepted by the peer.
    BitSet<GRPC_COMPRESS_ALGORITHMS_COUNT> GetEncodingsAcceptedByPeer() {
      return BitSet<GRPC_COMPRESS_ALGORITHMS_COUNT>::FromInt(
          grpc_call_test_only_get_encodings_accepted_by_peer(c_call()));
    }

   private:
    struct Impl {
      explicit Impl(CoreEnd2endTest* test) : call(nullptr, test) {
        grpc_call_details_init(&call_details);
        grpc_metadata_array_init(&request_metadata);
      }
      ~Impl() {
        grpc_call_details_destroy(&call_details);
        grpc_metadata_array_destroy(&request_metadata);
      }
      Call call;
      grpc_call_details call_details;
      grpc_metadata_array request_metadata;
    };
    std::unique_ptr<Impl> impl_;
  };

  class ServerRegisteredMethod {
   public:
    ServerRegisteredMethod(
        CoreEnd2endTest* test, absl::string_view name,
        grpc_server_register_method_payload_handling payload_handling);

    void* handle() { return *handle_; }

   private:
    std::shared_ptr<void*> handle_ = std::make_shared<void*>(nullptr);
  };

  ServerRegisteredMethod RegisterServerMethod(
      absl::string_view name,
      grpc_server_register_method_payload_handling payload_handling) {
    return ServerRegisteredMethod(this, name, payload_handling);
  }

  // Begin construction of a client call.
  ClientCallBuilder NewClientCall(std::string method) {
    return ClientCallBuilder(*this, std::move(method));
  }
  ClientCallBuilder NewClientCall(RegisteredCall registered_method) {
    return ClientCallBuilder(*this, registered_method);
  }
  // Request a call on the server - notifies `tag` when complete.
  IncomingCall RequestCall(int tag) { return IncomingCall(*this, tag); }
  // Request a call on the server - notifies `tag` when complete.
  IncomingCall RequestRegisteredCall(ServerRegisteredMethod method, int tag) {
    return IncomingCall(*this, method.handle(), nullptr, tag);
  }
  IncomingCall RequestRegisteredCall(ServerRegisteredMethod method,
                                     IncomingMessage* message, int tag) {
    return IncomingCall(*this, method.handle(), message, tag);
  }

  // Pull in CqVerifier types for ergonomics
  using ExpectedResult = CqVerifier::ExpectedResult;
  using Maybe = CqVerifier::Maybe;
  using PerformAction = CqVerifier::PerformAction;
  using MaybePerformAction = CqVerifier::MaybePerformAction;
  using AnyStatus = CqVerifier::AnyStatus;
  // Expect a tag with some result.
  void Expect(int tag, ExpectedResult result, SourceLocation whence = {}) {
    expectations_++;
    cq_verifier().Expect(CqVerifier::tag(tag), std::move(result), whence);
  }
  // Step the system until expectations are met or until timeout is reached.
  // If there are no expectations logged, then step for 1 second and verify that
  // no events occur.
  void Step(std::optional<Duration> timeout = std::nullopt,
            SourceLocation whence = {}) {
    if (expectations_ == 0) {
      cq_verifier().VerifyEmpty(timeout.value_or(Duration::Seconds(1)), whence);
      return;
    }
    expectations_ = 0;
    cq_verifier().Verify(timeout.value_or(fuzzing_ ? Duration::Minutes(10)
                                                   : Duration::Seconds(10)),
                         whence);
  }

  // Initialize the client.
  // If called, then InitServer must be called to create a server (otherwise one
  // will be provided).
  void InitClient(const ChannelArgs& args) {
    initialized_ = true;
    if (client_ != nullptr) ShutdownAndDestroyClient();
    auto& f = fixture();
    client_ = f.MakeClient(args, cq_);
    CHECK_NE(client_, nullptr);
  }
  // Initialize the server.
  // If called, then InitClient must be called to create a client (otherwise one
  // will be provided).
  void InitServer(const ChannelArgs& args) {
    initialized_ = true;
    if (server_ != nullptr) ShutdownAndDestroyServer();
    auto& f = fixture();
    server_ = f.MakeServer(args, cq_, pre_server_start_);
    CHECK_NE(server_, nullptr);
  }
  // Remove the client.
  void ShutdownAndDestroyClient() {
    if (client_ == nullptr) return;
    grpc_channel_destroy(client_);
    client_ = nullptr;
  }
  // Shutdown the server; notify tag on completion.
  void ShutdownServerAndNotify(int tag) {
    grpc_server_shutdown_and_notify(server_, cq_, CqVerifier::tag(tag));
  }
  // Destroy the server.
  void DestroyServer() {
    if (server_ == nullptr) return;
    grpc_server_destroy(server_);
    server_ = nullptr;
  }
  // Shutdown then destroy the server.
  void ShutdownAndDestroyServer() {
    if (server_ == nullptr) return;
    ShutdownServerAndNotify(-1);
    Expect(-1, AnyStatus{});
    Step();
    DestroyServer();
  }
  // Cancel any calls on the server.
  void CancelAllCallsOnServer() { grpc_server_cancel_all_calls(server_); }
  // Ping the server from the client
  void PingServerFromClient(int tag) {
    grpc_channel_ping(client_, cq_, CqVerifier::tag(tag), nullptr);
  }
  // Register a call on the client, return its handle.
  RegisteredCall RegisterCallOnClient(const char* method, const char* host) {
    ForceInitialized();
    return RegisteredCall{
        grpc_channel_register_call(client_, method, host, nullptr)};
  }

  // Return the current connectivity state of the client.
  grpc_connectivity_state CheckConnectivityState(bool try_to_connect) {
    return grpc_channel_check_connectivity_state(client_, try_to_connect);
  }

  // Watch the connectivity state of the client.
  void WatchConnectivityState(grpc_connectivity_state last_observed_state,
                              Duration deadline, int tag) {
    grpc_channel_watch_connectivity_state(
        client_, last_observed_state,
        grpc_timeout_milliseconds_to_deadline(deadline.millis()), cq_,
        CqVerifier::tag(tag));
  }

  // Return the client channel.
  grpc_channel* client() {
    ForceInitialized();
    return client_;
  }

  // Return the server channel.
  grpc_server* server() {
    ForceInitialized();
    return server_;
  }

  grpc_completion_queue* cq() {
    ForceInitialized();
    return cq_;
  }

  // Given a duration, return a timestamp that is that duration in the future -
  // with dilation according to test environment (eg sanitizers)
  Timestamp TimestampAfterDuration(Duration duration) {
    return Timestamp::FromTimespecRoundUp(
        grpc_timeout_milliseconds_to_deadline(duration.millis()));
  }

  void SetPostGrpcInitFunc(absl::AnyInvocable<void()> fn) {
    CHECK(fixture_ == nullptr);
    post_grpc_init_func_ = std::move(fn);
  }

  const CoreTestConfiguration* test_config() const { return test_config_; }

  bool fuzzing() const { return fuzzing_; }

  CoreTestFixture& fixture() {
    if (fixture_ == nullptr) {
      grpc_init();
      post_grpc_init_func_();
      cq_ = grpc_completion_queue_create_for_next(nullptr);
      fixture_ = test_config()->create_fixture(ChannelArgs(), ChannelArgs());
    }
    return *fixture_;
  }

 private:
  void ForceInitialized();

  CqVerifier& cq_verifier() {
    if (cq_verifier_ == nullptr) {
      fixture();  // ensure cq_ present
      cq_verifier_ = absl::make_unique<CqVerifier>(
          cq_,
          fuzzing_ ? CqVerifier::FailUsingGprCrashWithStdio
                   : CqVerifier::FailUsingGprCrash,
          step_fn_ == nullptr
              ? nullptr
              : absl::AnyInvocable<void(
                    grpc_event_engine::experimental::EventEngine::Duration)
                                       const>(
                    [this](
                        grpc_event_engine::experimental::EventEngine::Duration
                            d) { step_fn_(d); }));
    }
    return *cq_verifier_;
  }

  PostMortem post_mortem_;
  const CoreTestConfiguration* const test_config_;
  const bool fuzzing_;
  std::unique_ptr<CoreTestFixture> fixture_;
  grpc_completion_queue* cq_ = nullptr;
  grpc_server* server_ = nullptr;
  grpc_channel* client_ = nullptr;
  std::unique_ptr<CqVerifier> cq_verifier_;
  absl::AnyInvocable<void(grpc_server*)> pre_server_start_ = [](grpc_server*) {
  };
  int expectations_ = 0;
  bool initialized_ = false;
  absl::AnyInvocable<void()> post_grpc_init_func_ = []() {};
  absl::AnyInvocable<void(
      grpc_event_engine::experimental::EventEngine::Duration) const>
      step_fn_ = nullptr;
  absl::AnyInvocable<void(
      std::shared_ptr<grpc_event_engine::experimental::EventEngine>)>
      quiesce_event_engine_ =
          WaitForSingleOwner<grpc_event_engine::experimental::EventEngine>;
};

// Define names for additional test suites.
// These make no changes to the actual class, but define new names to register
// tests against. Each new name gets a differing set of configurations in
// end2end_test_main.cc to customize the set of fixtures the tests run against.

// NOLINTBEGIN(bugprone-macro-parentheses)
#define DECLARE_SUITE(name)                                                    \
  class name : public ::testing::TestWithParam<const CoreTestConfiguration*> { \
   public:                                                                     \
    static std::vector<const CoreTestConfiguration*> AllSuiteConfigs(          \
        bool fuzzing);                                                         \
  };

// Test suite for tests that should run in all configurations
DECLARE_SUITE(CoreEnd2endTests);
// Test suite for tests that rely on a secure transport
DECLARE_SUITE(SecureEnd2endTests);
// Test suite for tests that send rather large messages/metadata
DECLARE_SUITE(CoreLargeSendTests);
// Test suite for tests that need a client channel
DECLARE_SUITE(CoreClientChannelTests);
// Test suite for tests that require deadline handling
DECLARE_SUITE(CoreDeadlineTests);
// Test suite for tests that require deadline handling
DECLARE_SUITE(CoreDeadlineSingleHopTests);
// Test suite for http2 tests that only work over a single hop (unproxyable)
DECLARE_SUITE(Http2SingleHopTests);
// Test suite for fullstack single hop http2 tests (require client channel)
DECLARE_SUITE(Http2FullstackSingleHopTests);
// Test suite for tests that require retry features
DECLARE_SUITE(RetryTests);
// Test suite for write buffering
DECLARE_SUITE(WriteBufferingTests);
// Test suite for http2 tests
DECLARE_SUITE(Http2Tests);
// Test suite for http2 tests that require retry features
DECLARE_SUITE(RetryHttp2Tests);
// Test suite for tests that require resource quota
DECLARE_SUITE(ResourceQuotaTests);
// Test suite for tests that require a transport that supports secure call
// credentials
DECLARE_SUITE(PerCallCredsTests);
// Test suite for tests that require a transport that supports insecure call
// credentials
DECLARE_SUITE(PerCallCredsOnInsecureTests);
// Test suite for tests that verify lack of logging in particular situations
DECLARE_SUITE(NoLoggingTests);
// Test suite for tests that verify proxy authentication
DECLARE_SUITE(ProxyAuthTests);

#undef DECLARE_SUITE
// NOLINTEND(bugprone-macro-parentheses)

core_end2end_test_fuzzer::Msg ParseTestProto(std::string text);

// This function should be provided by the config_src C++ file for the
// end2end test suite binary being compiled. It provides an extension
// point so that different test configurations can be executed by the
// core test suite.
std::vector<CoreTestConfiguration> End2endTestConfigs();

// Helper function to add a nullptr to a vector of CoreConfiguration pointers
// if the vector is empty - for fuzzers to avoid ElementOf from asserting.
inline auto MaybeAddNullConfig(
    std::vector<const CoreTestConfiguration*> configs) {
  if (configs.empty()) {
    configs.push_back(nullptr);
  }
  return configs;
}

}  // namespace grpc_core

// If this test fixture is being run under minstack, skip the test.
#define SKIP_IF_MINSTACK()                                    \
  if (test_config()->feature_mask & FEATURE_MASK_IS_MINSTACK) \
  GTEST_SKIP() << "Skipping test for minstack"

#define SKIP_IF_FUZZING() \
  if (fuzzing()) GTEST_SKIP() << "Skipping test for fuzzing"

#define SKIP_IF_V3()                                           \
  if (test_config()->feature_mask & FEATURE_MASK_IS_CALL_V3) { \
    GTEST_SKIP() << "Disabled for initial v3 testing";         \
  }

#define SKIP_IF_LOCAL_TCP_CREDS()                                      \
  if (test_config()->feature_mask & FEATURE_MASK_IS_LOCAL_TCP_CREDS) { \
    GTEST_SKIP() << "Disabled for Local TCP Connection";               \
  }

#define SKIP_IF_CORE_CONFIGURATION_RESET_DISABLED() \
  if (!core_configuration_reset()) {                \
    GTEST_SKIP() << "Skipping test for fuzzing";    \
  }

#ifndef GRPC_END2END_TEST_INCLUDE_FUZZER
#define CORE_END2END_FUZZER(suite, name)
#else
#define CORE_END2END_FUZZER(suite, name)                                  \
  FUZZ_TEST(Fuzzers, suite##_##name)                                      \
      .WithDomains(::fuzztest::ElementOf(grpc_core::MaybeAddNullConfig(   \
                       suite::AllSuiteConfigs(true))),                    \
                   ::fuzztest::Arbitrary<core_end2end_test_fuzzer::Msg>() \
                       .WithProtobufField("config_vars", AnyConfigVars()));
#endif

// NOLINTBEGIN(bugprone-macro-parentheses)
#if defined(FUZZING_BUILD_MODE_UNSAFE_FOR_PRODUCTION) || \
    defined(GRPC_END2END_TEST_NO_GTEST)
#define CORE_END2END_TEST_P(suite, name)
#else
#define CORE_END2END_TEST_P(suite, name)                                     \
  TEST_P(suite, name) {                                                      \
    if ((GetParam()->feature_mask & FEATURE_MASK_IS_CALL_V3) &&              \
        (grpc_core::ConfigVars::Get().PollStrategy() == "poll")) {           \
      GTEST_SKIP() << "call-v3 not supported with poll poller";              \
    }                                                                        \
    CoreEnd2endTest_##suite##_##name(GetParam(), nullptr, #suite).RunTest(); \
  }
#endif

#ifdef FUZZING_BUILD_MODE_UNSAFE_FOR_PRODUCTION
#define CORE_END2END_INSTANTIATE_TEST_SUITE_P(suite)
#else
#define CORE_END2END_INSTANTIATE_TEST_SUITE_P(suite)                           \
  INSTANTIATE_TEST_SUITE_P(, suite,                                            \
                           ::testing::ValuesIn(suite::AllSuiteConfigs(false)), \
                           [](auto info) { return info.param->name; });        \
  GTEST_ALLOW_UNINSTANTIATED_PARAMETERIZED_TEST(suite);
#endif

#define CORE_END2END_TEST(suite, name)                                         \
  class CoreEnd2endTest_##suite##_##name final                                 \
      : public grpc_core::CoreEnd2endTest {                                    \
   public:                                                                     \
    using grpc_core::CoreEnd2endTest::CoreEnd2endTest;                         \
    void RunTest();                                                            \
  };                                                                           \
  void suite##_##name(const grpc_core::CoreTestConfiguration* config,          \
                      core_end2end_test_fuzzer::Msg msg) {                     \
    if (config == nullptr) return;                                             \
    if (absl::StartsWith(#name, "DISABLED_")) GTEST_SKIP() << "disabled test"; \
    if (!IsEventEngineListenerEnabled() || !IsEventEngineClientEnabled() ||    \
        !IsEventEngineDnsEnabled()) {                                          \
      GTEST_SKIP() << "fuzzers need event engine";                             \
    }                                                                          \
    if (IsEventEngineDnsNonClientChannelEnabled()) {                           \
      GTEST_SKIP() << "event_engine_dns_non_client_channel experiment breaks " \
                      "fuzzing currently";                                     \
    }                                                                          \
    CoreEnd2endTest_##suite##_##name(config, &msg, #suite).RunTest();          \
    grpc_event_engine::experimental::ShutdownDefaultEventEngine();             \
  }                                                                            \
  CORE_END2END_TEST_P(suite, name)                                             \
  CORE_END2END_FUZZER(suite, name)                                             \
  void CoreEnd2endTest_##suite##_##name::RunTest()

#define CORE_END2END_TEST_INCOMPATIBLE_WITH_FUZZING(suite, name) \
  class CoreEnd2endTest_##suite##_##name final                   \
      : public grpc_core::CoreEnd2endTest {                      \
   public:                                                       \
    using grpc_core::CoreEnd2endTest::CoreEnd2endTest;           \
    void RunTest();                                              \
  };                                                             \
  CORE_END2END_TEST_P(suite, name)                               \
  void CoreEnd2endTest_##suite##_##name::RunTest()

#define CORE_END2END_TEST_SUITE(suite, configs)                                \
  CORE_END2END_INSTANTIATE_TEST_SUITE_P(suite)                                 \
  std::vector<const grpc_core::CoreTestConfiguration*> suite::AllSuiteConfigs( \
      bool fuzzing) {                                                          \
    return configs;                                                            \
  }
// NOLINTEND(bugprone-macro-parentheses)

#endif  // GRPC_TEST_CORE_END2END_END2END_TESTS_H
