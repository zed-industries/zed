// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_TEST_MULTINODE_TEST_H_
#define IPCZ_SRC_TEST_MULTINODE_TEST_H_

#include <memory>
#include <string>
#include <string_view>
#include <type_traits>
#include <utility>
#include <vector>

#include "ipcz/ipcz.h"
#include "test/test_base.h"
#include "test_buildflags.h"
#include "testing/gtest/include/gtest/gtest.h"
#include "third_party/abseil-cpp/absl/base/macros.h"
#include "third_party/abseil-cpp/absl/types/span.h"
#include "util/ref_counted.h"

namespace ipcz::test {

class TestNode;

template <typename TestNodeType>
class MultinodeTest;

namespace internal {

template <typename TestNodeType>
std::unique_ptr<TestNode> MakeTestNode() {
  return std::make_unique<TestNodeType>();
}

template <typename T>
static constexpr bool IsValidTestNodeType = std::is_base_of_v<TestNode, T>;

extern const char kSyncTestDriverName[];
extern const char kAsyncTestDriverName[];
extern const char kAsyncDelegatedAllocTestDriverName[];
extern const char kAsyncForcedBrokeringTestDriverName[];
extern const char kAsyncDelegatedAllocAndForcedBrokeringTestDriverName[];
extern const char kMultiprocessTestDriverName[];

}  // namespace internal

class TestDriver;

using TestNodeFactory = std::unique_ptr<TestNode> (*)();

struct TransportPair {
  IpczDriverHandle ours;
  IpczDriverHandle theirs;
};

// Type used to package metadata about a MULTINODE_TEST_NODE() or
// MULTINODE_TEST() invocation.
struct TestNodeDetails {
  // A unique display name for the defined node body.
  const std::string_view name;

  // A factory function which can be used to instantiate the TestNode. Null for
  // main MULTINODE_TEST() invocations.
  const TestNodeFactory factory;

  // Indicates whether the node is defined as a broker or non-broker node. By
  // default, all nodes on non-brokers except for those emitted by main
  // MULTINODE_TEST() invocations.
  const bool is_broker;
};

// Base class to support tests which exercise behavior across multiple ipcz
// nodes. These may be single-process on a synchronous driver, single-process on
// an asynchronous (e.g. multiprocess) driver, or fully multiprocess.
//
// This class provides convenience methods for creating and connecting nodes
// in various useful configurations. Note that it does NOT inherit from GTest's
// Test class, as multiple instances may run in parallel for a single test, and
// GTest's Test class is not compatible with that behavior.
//
// Instead, while MULTINODE_TEST_NODE() invocations should be based directly on
// TestNode or a derivative thereof. MULTINODE_TEST() invocations for multinode
// tests should be based on derivatives of MultinodeTest<T> (see below this
// class), where T itself is a TestNode or some derivative thereof.
//
// This arrangement allows the main test body and its related
// MULTINODE_TEST_NODE() invocations to be based on the same essential type,
// making multinode tests easier to read and write.
class TestNode : public internal::TestBase {
 public:
  using TransportPair = ::ipcz::test::TransportPair;

  // Exposes interaction with one node spawned by another.
  class TestNodeController : public RefCounted<TestNodeController> {
   public:
    // Blocks until the spawned node has terminated. Returns true if the node
    // executed and terminated cleanly, or false if it encountered at least one
    // test expectation failure while running.
    virtual bool WaitForShutdown() = 0;

    // Creates a new pair of transports suitable for connecting the calling
    // node with the TestNode controlled by this object.
    virtual TransportPair CreateNewTransports() = 0;

   protected:
    friend class RefCounted<TestNodeController>;

    virtual ~TestNodeController() = default;
  };

  virtual ~TestNode();

  // Handle to this node.
  IpczHandle node() const { return node_; }

  // Handle to this node's broker-facing transport, if and only if
  // ConnectToBroker() hasn't been called yet.
  IpczDriverHandle transport() const { return transport_; }

  // Returns metadata regarding the definition of this TestNode type.
  virtual const TestNodeDetails& GetDetails() const = 0;

  // Releases transport() to the caller. After calling this, it is no longer
  // valid to call either transport() or ConnectToBroker(), and this fixture
  // will not automatically close the transport on destruction.
  IpczDriverHandle ReleaseTransport() {
    return std::exchange(transport_, IPCZ_INVALID_DRIVER_HANDLE);
  }

  // The active TestDriver implementation.
  TestDriver* GetTestDriver() { return test_driver_; }

  // The ipcz driver currently in use, as specified by the active TestDriver.
  const IpczDriver& GetDriver() const;

  // One-time initialization. Called internally during test setup. Should never
  // be called by individual test code.
  void Initialize(TestDriver* test_driver, const std::string& feature_set);

  // May be called at most once by the TestNode body to connect initial
  // `portals` to the node that spawned this one. Extra `flags` may be passed to
  // the corresponding ConnectNode() call.
  void ConnectToParent(absl::Span<IpczHandle> portals,
                       IpczConnectNodeFlags flags = IPCZ_NO_FLAGS);

  // May be called instead of ConnectToParent() when the portal that spawned
  // this one is a broker.
  void ConnectToBroker(absl::Span<IpczHandle> portals);

  // Shorthand for the above, for the common case with only one initial portal.
  IpczHandle ConnectToParent(IpczConnectNodeFlags flags = IPCZ_NO_FLAGS);
  IpczHandle ConnectToBroker();

  // Opens a new portal pair on this node.
  std::pair<IpczHandle, IpczHandle> OpenPortals();

  // Creates a new driver memory object populated with `contents`, boxes it, and
  // returns a handle to the new box.
  IpczHandle BoxBlob(std::string_view contents);

  // Extracts the string contents of a boxed driver memory object.
  std::string UnboxBlob(IpczHandle box);

  // Spawns a new test node of TestNodeType and populates `portals` with a set
  // of initial portals connected to the node, via a new transport.
  template <typename TestNodeType>
  Ref<TestNodeController> SpawnTestNode(
      absl::Span<IpczHandle> portals,
      IpczConnectNodeFlags flags = IPCZ_NO_FLAGS) {
    IpczDriverHandle our_transport;
    auto controller = SpawnTestNodeImpl(TestNodeType::kDetails, our_transport);
    if (TestNodeType::kDetails.is_broker) {
      flags |= IPCZ_CONNECT_NODE_TO_BROKER;
    }
    const IpczResult result = ipcz().ConnectNode(
        node(), our_transport, portals.size(), flags, nullptr, portals.data());
    ABSL_ASSERT(result == IPCZ_RESULT_OK);
    return controller;
  }

  // Shorthand for the above, for the common case with only one initial portal
  // and no need for the test body to retain a controller for the node.
  template <typename TestNodeType>
  IpczHandle SpawnTestNode(IpczConnectNodeFlags flags = IPCZ_NO_FLAGS) {
    IpczHandle portal;
    SpawnTestNode<TestNodeType>({&portal, 1}, flags);
    return portal;
  }

  // Spawns a new test node of TestNodeType, giving it `transport` to use for
  // its broker connection. The caller is resposible for the other end of that
  // connection.
  template <typename TestNodeType>
  Ref<TestNodeController> SpawnTestNodeNoConnect(IpczDriverHandle& transport) {
    return SpawnTestNodeImpl(TestNodeType::kDetails, transport);
  }

  // Forcibly closes this Node, severing all links to other nodes and implicitly
  // disconnecting any portals which relied on those links.
  void CloseThisNode();

  // The TestNode body provided by a MULTINODE_TEST_NODE() invocation. For main
  // test definitions via MULTINODE_TEST() with a MultinodeTest<T> fixture, this
  // is unused in favor of TestBody().
  virtual void NodeBody() {}

  // Creates a pair of transports appropriate for connecting this (broker or
  // non-broker) node to another non-broker node. Most tests should not use this
  // directly, but should instead connect to other nodes using the more
  // convenient helpers ConnectToBroker() or SpawnTestNode().
  TransportPair CreateTransports();

  // Creates a pair of transport appropriate for connecting two broker nodes
  // together.
  TransportPair CreateBrokerToBrokerTransports();

  // Helper used to support multiprocess TestNode invocation.
  int RunAsChild(TestDriver* test_driver, const std::string& feature_set);

  // Sets the transport to use when connecting to a broker via ConnectBroker.
  // Must only be called once.
  void SetTransport(IpczDriverHandle transport);

 private:
  // Spawns a new node using an appropriate configuration for the current
  // driver. Returns a controller which can be used to interact with the node
  // outside of ipcz (e.g. to wait on its termination). `details` describes the
  // node to be launched, and `our_transport` on output receives a locally owned
  // transport to the spawned node.
  Ref<TestNodeController> SpawnTestNodeImpl(const TestNodeDetails& details,
                                            IpczDriverHandle& our_transport);

  TestDriver* test_driver_ = nullptr;
  std::string feature_set_{"Default"};
  IpczHandle node_ = IPCZ_INVALID_HANDLE;
  IpczDriverHandle transport_ = IPCZ_INVALID_DRIVER_HANDLE;
  std::vector<Ref<TestNodeController>> spawned_nodes_;
};

// Actual parameterized GTest Test fixture for multinode tests. This or a
// subclass of it is required for MULTINODE_TEST() invocations to function as
// proper multinode tests.
template <typename TestNodeType = TestNode>
class MultinodeTest : public TestNodeType, public ::testing::Test {
 public:
  static_assert(internal::IsValidTestNodeType<TestNodeType>,
                "MultinodeTest<T> requires T to be a subclass of TestNode.");
};

// TestDriver specifies an IpczDriver implementation to use for multinode tests.
// It also implements launching and joining of other test nodes. A TestDriver
// can be registered by statically initializing a corresponding
// TestDriverRegistration. All multinode tests are run against all registered
// TestDrivers.
class TestDriver {
 public:
  // A reference to the actual IpczDriver implementation used by this
  // TestDriver.
  virtual const IpczDriver& GetIpczDriver() const = 0;

  // A unique name for this test driver.
  virtual const char* GetName() const = 0;

  // Creates a new pair of transports suitable for connecting a broker node to
  // another node. Called by `source`, who will adopt `ours` from the returned
  // pair and pass `theirs` to some newly spawned test node. If
  // `for_broker_target` is true, `theirs` will be suitable for passing to a
  // broker node; otherwise it must be passed to a non-broker.
  virtual TransportPair CreateTransports(TestNode& source,
                                         bool for_broker_target) const = 0;

  // Spawns a new TestNode instance for a TestNode described by `details`,
  // passing `their_transport` to the new node so it can establish a connection.
  // `our_transport` is the local driver transport endpoint that will be used to
  // connect to the new node.
  virtual Ref<TestNode::TestNodeController> SpawnTestNode(
      TestNode& source,
      const TestNodeDetails& details,
      const std::string& feature_set,
      IpczDriverHandle our_transport,
      IpczDriverHandle their_transport) = 0;

  // Returns any extra flags to be provided to ConnectNode() when connecting to
  // the main test node from a node spawned by the test.
  virtual IpczConnectNodeFlags GetExtraClientConnectNodeFlags() const = 0;

  // If the test driver launches test nodes in a separate subprocess, this is
  // called to retrieve the driver transport which the test node should use to
  // connect to the broker.
  virtual IpczDriverHandle GetClientTestNodeTransport() = 0;
};

// Registers a TestDriver globally so that all MULTINODE_TEST() invocations are
// parameterized over it.
class TestDriverRegistrationImpl {
 public:
  explicit TestDriverRegistrationImpl(TestDriver& driver);
};

template <typename TestDriverType>
class TestDriverRegistration {
 public:
  template <typename... Args>
  explicit TestDriverRegistration(Args&&... args)
      : driver(std::forward<Args>(args)...) {}

  TestDriverType driver;
  TestDriverRegistrationImpl registration{driver};
};

void RegisterMultinodeTestNode(std::string_view node_name,
                               TestNodeFactory factory);

template <typename NodeType>
class MultinodeTestNodeRegistration {
 public:
  MultinodeTestNodeRegistration() {
    RegisterMultinodeTestNode(NodeType::kDetails.name,
                              NodeType::kDetails.factory);
  }
};

using MultinodeTestFactory =
    std::function<testing::Test*(TestDriver*, const std::string& feature_set)>;
void RegisterMultinodeTest(const char* test_suite_name,
                           const char* test_name,
                           const char* filename,
                           int line,
                           MultinodeTestFactory factory);

// Registers a MULTINODE_TEST() test to be run when all tests are run. This
// registers a unique instance of the test for each registered test driver.
template <typename Test>
class MultinodeTestRegistration {
 public:
  MultinodeTestRegistration(const char* test_suite_name,
                            const char* test_name,
                            const char* filename,
                            int line) {
    RegisterMultinodeTest(
        test_suite_name, test_name, filename, line,
        [](TestDriver* driver, const std::string& feature_set) {
          return new Test(driver, feature_set);
        });
  }
};

// Must be called before RUN_ALL_TESTS() is invoked in order for any defined
// multinode tests to be run.
void RegisterMultinodeTests();

}  // namespace ipcz::test

// Defines the main body of a non-broker test node for a multinode test. The
// named node can be spawned by another node using SpawnTestNode<T> where T is
// the unique name given by `node_name` here. `fixture` must be
/// ipcz::test::TestNode or a subclass thereof.
#define MULTINODE_TEST_NODE_IMPL(fixture, node_name, is_broker_value)      \
  class node_name : public fixture {                                       \
    static_assert(::ipcz::test::internal::IsValidTestNodeType<fixture>,    \
                  "MULTINODE_TEST_NODE() requires a fixture derived from " \
                  "ipcz::test::TestNode.");                                \
                                                                           \
   public:                                                                 \
    static constexpr ::ipcz::test::TestNodeDetails kDetails = {            \
        .name = #fixture "_" #node_name "_Node",                           \
        .factory = &::ipcz::test::internal::MakeTestNode<node_name>,       \
        .is_broker = is_broker_value,                                      \
    };                                                                     \
    const ::ipcz::test::TestNodeDetails& GetDetails() const override {     \
      return kDetails;                                                     \
    }                                                                      \
    void NodeBody() override;                                              \
  };                                                                       \
  ::ipcz::test::MultinodeTestNodeRegistration<node_name>                   \
      kRegister_##node_name;                                               \
  void node_name::NodeBody()

#define MULTINODE_TEST_NODE(fixture, node_name) \
  MULTINODE_TEST_NODE_IMPL(fixture, node_name, /*is_broker=*/false)

#define MULTINODE_TEST_BROKER_NODE(fixture, node_name) \
  MULTINODE_TEST_NODE_IMPL(fixture, node_name, /*is_broker=*/true)

#define MULTINODE_TEST_NAME(name) #name
#define MULTINODE_TEST_CLASS_NAME(name) name##_Test
#define MULTINODE_TEST_REGISTRATION_NAME(name) kRegister_##name##_Test

#define MULTINODE_TEST(fixture, test_name)                                 \
  class MULTINODE_TEST_CLASS_NAME(test_name) : public fixture {            \
   public:                                                                 \
    static constexpr ::ipcz::test::TestNodeDetails kDetails = {            \
        .name = #fixture "_" #test_name "_Node",                           \
        .factory = nullptr,                                                \
        .is_broker = true,                                                 \
    };                                                                     \
    explicit MULTINODE_TEST_CLASS_NAME(test_name)(                         \
        ::ipcz::test::TestDriver * test_driver,                            \
        const std::string& feature_set) {                                  \
      TestNode::Initialize(test_driver, feature_set);                      \
    }                                                                      \
    ~MULTINODE_TEST_CLASS_NAME(test_name)() override = default;            \
    MULTINODE_TEST_CLASS_NAME(test_name)                                   \
    (const MULTINODE_TEST_CLASS_NAME(test_name) &) = delete;               \
    void operator=(const MULTINODE_TEST_CLASS_NAME(test_name) &) = delete; \
    const ::ipcz::test::TestNodeDetails& GetDetails() const override {     \
      return kDetails;                                                     \
    }                                                                      \
                                                                           \
   private:                                                                \
    void TestBody() override;                                              \
  };                                                                       \
  namespace {                                                              \
  ::ipcz::test::MultinodeTestRegistration<                                 \
      MULTINODE_TEST_CLASS_NAME(test_name)>                                \
      MULTINODE_TEST_REGISTRATION_NAME(test_name){                         \
          #fixture, MULTINODE_TEST_NAME(test_name), __FILE__, __LINE__};   \
  }                                                                        \
  void MULTINODE_TEST_CLASS_NAME(test_name)::TestBody()

#endif  // IPCZ_SRC_TEST_MULTINODE_TEST_H_
