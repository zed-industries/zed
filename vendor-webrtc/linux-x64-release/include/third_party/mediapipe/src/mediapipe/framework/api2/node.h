#ifndef MEDIAPIPE_FRAMEWORK_API2_NODE_H_
#define MEDIAPIPE_FRAMEWORK_API2_NODE_H_

#include <memory>
#include <type_traits>

#include "absl/status/status.h"
#include "mediapipe/framework/api2/contract.h"
#include "mediapipe/framework/calculator_base.h"
#include "mediapipe/framework/calculator_context.h"
#include "mediapipe/framework/calculator_contract.h"
#include "mediapipe/framework/deps/registration.h"
#include "mediapipe/framework/subgraph.h"

namespace mediapipe {
namespace api2 {

// Node (calculator / subgraph) interface.
//
// A subclass must declare its inputs, outputs, side inputs and side outputs
// (referred as "ports" below) using API2 types: Input, Output, SideInput,
// SideOutput.
//
// In addition, node can declare its timestamp offset and stream handler,
// using TimestampChange(...) and StreamHandler(...) respectively.
//
// To finish node interface declaration subclass must use
// MEDIAPIPE_NODE_INTERFACE macro.
//
// Example:
//
//   class FooNode : public NodeIntf {
//    public:
//     static constexpr mediapipe::api2::Input<InputData> kInput{"IN"};
//     static constexpr mediapipe::api2::Output<OutputData> kOutput{"OUT"};
//
//     MEDIAPIPE_NODE_INTERFACE(FooNode, kInputData, kOutput);
//   };
//
// Example overriding default timestamp offset (0 for NodeIntf/NodeImpl API)
// with the default of CalculatorBase - "arbitrary" (more details on this in
// NodeImpl documentation) and stream handler:
//
//   class FooNode : public NodeIntf {
//    public:
//     static constexpr mediapipe::api2::Input<InputData> kInput{"IN"};
//     static constexpr mediapipe::api2::Output<OutputData> kOutput{"OUT"};
//
//     MEDIAPIPE_NODE_INTERFACE(FooNode, kInputData, kOutput,
//                              TimestampChange::Arbitrary(),
//                              StreamHandler("FixedSizeInputStreamHandler"));
//   };
//
// NOTE: "IN" is a tag for FooNode input stream which can be used while
// authoring MediaPipe graph as CalculatorGraphConfig proto text, for example:
//   input_stream: "in"
//   node {
//     calculator: "FooNode"
//     input_stream: "IN:in"
//     output_stream: "OUT:out"
//   }
//
// or graph builder:
//  Graph graph;
//  Stream<InputData> in = graph.In(0).Cast<InputData>();
//  auto& node = graph.AddNode("FooNode");
//  in.ConnectTo(node.In("IN"));
//  Stream<OutputData> out = node.Out("OUT").Cast<OutputData>();
//
// NOTE: it's recommended to provide meaningful tags for your node ports (e.g.
// helpful for debugging/logging purposes).  In some cases you may still need to
// use empty string tags when migrating older calculators and keeping backward
// compatibility.
class NodeIntf {};

// Node (calculator) implementation.
//
// A subclass must specify node interface it implements as a first template
// parameter and itself as a second template parameter for the registration
// purposes.
//
// The subclass must implement Process() function and can implement Open() to
// do the initialization.
//
// For example:
//   class FooNodeImpl : public NodeImpl<FooNode, FooNodeImpl> {
//    public:
//     absl::Status Process(CalculatorContext* cc) override {
//       int input = kInput(cc).Get();
//       ...
//     }
//   };
//
// Now FooNodeImpl is registered as "FooNode" (taken from `class FooNode...`) in
// MediaPipe registry automatically and can be used in CalculatorGraphConfig and
// Graph builder.
//
// Below is the explanation of how framework calls node functions:
//
//   static absl::Status UpdateContract(CalculatorContract*)
//     (Optional) Invoked on graph initialization, if defined, to update the
//     contract.
//
// Then, for each run of the graph on a set of input side packets, the
// following sequence will occur.
//
//   absl::Status Open(CalculatorContext*)
//     (Optional) To initialize the calculator.
//
//     NOTE : with the new API (NodeIntf/NodeImpl), the
//       default Timestamp Offset of a calculator is 0. (Pay attention when
//       migrating from older calculator APIs, because the default there is
//       "arbitrary" Timestamp Offset.)
//
//       With 0 Timestamp Offset, calculator is expected to send an output
//       packet for every input packet at the input packet timestamp.
//
//       If the calculator returns from Process without adding an output to some
//       or all output streams:
//       - The framework will send a timestamp bound update to downstream
//         calculators that there won't be a packet for that particular
//         timestamp on output streams in question.
//       - Dependent downstream calculator(s) will execute on timestamp bound
//         update if they have other input streams with ready packets at that
//         particular timestamp. Input streams corresponding to output streams
//         in question (with timestamp bound update) will have empty packets, so
//         calculators need to use: IsEmpty before getting data.
//
//     You can disable default 0 Timestamp Offset in the node interface or
//     contract:
//
//       MEDIAPIPE_NODE_INTERFACE(...,
//                        TimestampChange::Arbitrary());
//       MEDIAPIPE_NODE_CONTRACT(...,
//                        TimestampChange::Arbitrary());
//
//     NOTE: Clients can help optimize framework packet queueing by calling
//       SetNextTimestampBound on outputs if applicable (e.g.
//       kOutputPort(cc)->SetNextTimestampBound()).
//
//   absl::Status Process(CalculatorContext*) (repeatedly)
//
//     For Non-Source Nodes (nodes that have input streams):
//
//     By default, invoked when every input stream either has a packet at
//     timestamp T or the framework knows the packet is not expected at that
//     timestamp. The latter occurs during timestamp bound update (Timestamp
//     Offset is 0 (default), explicit call to SetNextTimestampBound() on
//     calculator graph/upstream calculator or receiving a packet with a
//     timestamp > T), and this results in corresponding input stream being
//     empty during Process() call, so clients need to use: IsEmpty before
//     getting data.
//
//     This behavior may be adjusted, by utilizing different input stream
//     handlers (please consult corresponding documentation):
//     - DefaultInputStreamHandler (default)
//     - FixedSizeInputStreamHandler
//     - ImmediateInputStreamHandler
//     - etc. in mediapipe/third_party/framework/stream_handler
//
//     In the first place, consider setting it in the calculator if the
//     calculator is required to have a custom stream handler always:
//
//       MEDIAPIPE_NODE_INTERFACE(...,
//                        StreamHandler("FixedSizeInputStreamHandler"));
//
//     Otherwise, you can set it in CalculatorGraphConfig:
//       node {
//         calculator: "CalculatorRunningAtOneFps"
//         input_stream: "packets_streaming_in_at_ten_fps"
//         input_stream_handler {
//           input_stream_handler: "FixedSizeInputStreamHandler"
//         }
//       }
//
//     or Graph builder:
//       Graph graph;
//       auto& node = graph.AddNode("CalculatorRunningAtOneFps");
//       node.SetInputStreamHandler("FixedSizeInputStreamHandler")
//
//     For Source Nodes (nodes that don't have input streams):
//
//     Continues to have Process() called as long as it returns an
//     absl::OkStatus(). Returning tool::StatusStop() indicates source node is
//     done producing data.
//
//   absl::Status Close(CalculatorContext*)
//
//     After all calls to Process() finish or when all input streams close, the
//     framework calls Close(). This function is always called if Open() was
//     called and succeeded and even if the graph run terminated because of an
//     error. No inputs are available via any input streams during Close(), but
//     it still has access to input side packets and therefore may write
//     outputs. After Close() returns, the calculator should be considered a
//     dead node. The calculator object is destroyed as soon as the graph
//     finishes running.
//
// NOTE: the entire calculator is constructed and destroyed for each graph run
// (set of input side packets, which could mean once per video, or once per
// image). Expensive operations and large objects should be input side packets
// or provided by graph services.
//
// Calculators must be thread-compatible.
// The framework does not call the non-const methods of a calculator from
// multiple threads at the same time. However, the thread that calls the methods
// of a calculator is not fixed. Therefore, calculators should not use
// ThreadLocal objects.
template <class Intf, class Impl = void>
class NodeImpl;

// Node (subgraph) implementation.
//
// A subclass must specify node interface it implements as a first template
// parameter and itself as a second template parameter for the registration
// purposes.
//
// The subclass must implement GetConfig() function to build the subgraph.
//
// For example:
//   class FooSubgraphImpl : public SubgraphImpl<FooNode, FooSubgraphImpl> {
//    public:
//     absl::StatusOr<CalculatorGraphConfig> GetConfig(
//         SubgraphContext* cc) override {
//       Graph graph;
//       ...
//       return graph.GetConfig();
//     }
//   };
//
// Now FooSubgraphImpl is registered as "FooNode" (taken from
// `class FooNode...`) in MediaPipe registry automatically and can be used in
// CalculatorGraphConfig and Graph builder.
template <class Intf, class Impl>
class SubgraphImpl;

// SOFT DEPRECATION: use combination of NodeIntf/NodeImpl instead.
// TODO: deprecate with ABSL_DEPRECATED, migrate MP calculators to
//   NodeIntf/NodeImpl.
class Node : public CalculatorBase {
 public:
  virtual ~Node();
};

}  // namespace api2

namespace internal {

template <class T>
class CalculatorBaseFactoryFor<
    T,
    typename std::enable_if<std::is_base_of<mediapipe::api2::Node, T>{}>::type>
    : public CalculatorBaseFactory {
 public:
  absl::Status GetContract(CalculatorContract* cc) final {
    auto status = T::Contract::GetContract(cc);
    if (status.ok()) {
      status = UpdateContract<T>(cc);
    }
    return status;
  }

  std::unique_ptr<CalculatorBase> CreateCalculator(
      CalculatorContext* calculator_context) final {
    return std::make_unique<T>();
  }

 private:
  template <typename U>
  auto UpdateContract(CalculatorContract* cc)
      -> decltype(U::UpdateContract(cc)) {
    return U::UpdateContract(cc);
  }
  template <typename U>
  absl::Status UpdateContract(...) {
    return {};
  }
};

}  // namespace internal

namespace api2 {
namespace internal {

MEDIAPIPE_STATIC_REGISTRATOR_TEMPLATE(
    NodeRegistrator, mediapipe::CalculatorBaseRegistry, T::kCalculatorName,
    std::make_unique<mediapipe::internal::CalculatorBaseFactoryFor<T>>)

MEDIAPIPE_STATIC_REGISTRATOR_TEMPLATE(SubgraphRegistrator,
                                      mediapipe::SubgraphRegistry,
                                      T::kCalculatorName, std::make_unique<T>)

}  // namespace internal

// FOR INTERNAL USE: use NodeIntf/NodeImpl instead.
//
// By passing the Impl parameter, registration is done automatically. No need
// to use MEDIAPIPE_NODE_IMPLEMENTATION.
// For backward compatibility, Impl can be omitted; use
// MEDIAPIPE_NODE_IMPLEMENTATION with this.
// TODO: migrate and remove.
template <class Impl = void>
class RegisteredNode;

// FOR INTERNAL USE: use NodeIntf/NodeImpl instead.
template <class Impl>
class RegisteredNode : public Node, private internal::NodeRegistrator<Impl> {};

// FOR INTERNAL USE: use NodeIntf/NodeImpl instead.
// No-op version for backwards compatibility.
template <>
class RegisteredNode<void> : public Node {};

template <class Impl>
struct FunctionNode : public RegisteredNode<Impl> {
  absl::Status Process(CalculatorContext* cc) override {
    return internal::ProcessFnCallers(cc, Impl::kContract.process_items());
  }
};

template <class Intf, class Impl>
class NodeImpl : public RegisteredNode<Impl>, public Intf {
 protected:
  // These methods allow accessing a node's ports by tag. This can be useful in
  // a few cases, e.g. if the port is not available as a named constant.
  // They parallel the corresponding methods on builder nodes.
  template <class Tag>
  static constexpr auto Out(Tag t) {
    return Intf::Contract::TaggedOutputs::get(t);
  }

  template <class Tag>
  static constexpr auto In(Tag t) {
    return Intf::Contract::TaggedInputs::get(t);
  }

  template <class Tag>
  static constexpr auto SideOut(Tag t) {
    return Intf::Contract::TaggedSideOutputs::get(t);
  }

  template <class Tag>
  static constexpr auto SideIn(Tag t) {
    return Intf::Contract::TaggedSideInputs::get(t);
  }

  // Convenience.
  template <class Tag, class CC>
  static auto Out(Tag t, CC cc) {
    return Out(t)(cc);
  }
  template <class Tag, class CC>
  static auto In(Tag t, CC cc) {
    return In(t)(cc);
  }
  template <class Tag, class CC>
  static auto SideOut(Tag t, CC cc) {
    return SideOut(t)(cc);
  }
  template <class Tag, class CC>
  static auto SideIn(Tag t, CC cc) {
    return SideIn(t)(cc);
  }
};

// This macro is used to define the contract, without also giving the
// node a type name. It can be used directly in pure interfaces.
#define MEDIAPIPE_NODE_CONTRACT(...)                                          \
  static constexpr auto kContract =                                           \
      mediapipe::api2::internal::MakeContract(__VA_ARGS__);                   \
  using Contract =                                                            \
      typename mediapipe::api2::internal::TaggedContract<decltype(kContract), \
                                                         kContract>;

// This macro is used to define the contract and the type name of a node.
// This saves the name of the calculator, making it available to the
// implementation too, and to the registration macro for it. The reason is
// that the name must be available with the contract (so that it can be used
// to build a graph config, for instance); however, it is the implementation
// that needs to be registered.
// TODO: rename to MEDIAPIPE_NODE_DECLARATION?
// TODO: more detailed explanation.
#define MEDIAPIPE_NODE_INTERFACE(name, ...)        \
  static constexpr char kCalculatorName[] = #name; \
  MEDIAPIPE_NODE_CONTRACT(__VA_ARGS__)

// TODO: verify that the subgraph config fully implements the
// declared interface.
template <class Intf, class Impl>
class SubgraphImpl : public Subgraph,
                     public Intf,
                     private internal::SubgraphRegistrator<Impl> {};

// DEPRECATED: use NodeIntf/NodeImpl and automatic registration it provides.
//   Consult NodeIntf/NodeImpl for more details.
//
// This macro is used to register a calculator that does not use automatic
// registration.
#define MEDIAPIPE_NODE_IMPLEMENTATION(Impl)                       \
  MEDIAPIPE_REGISTER_FACTORY_FUNCTION_QUALIFIED(                  \
      mediapipe::CalculatorBaseRegistry, calculator_registration, \
      Impl::kCalculatorName,                                      \
      std::make_unique<mediapipe::internal::CalculatorBaseFactoryFor<Impl>>)

// DEPRECATED: use NodeIntf/NodeImpl and automatic registration it provides.
//   Consult NodeIntf/NodeImpl for more details.
//
// This macro is used to register a non-split-contract calculator.
#define MEDIAPIPE_REGISTER_NODE(name)                                    \
  MEDIAPIPE_REGISTER_FACTORY_FUNCTION_QUALIFIED(                         \
      mediapipe::CalculatorBaseRegistry, calculator_registration, #name, \
      std::make_unique<mediapipe::internal::CalculatorBaseFactoryFor<name>>)

// DEPRECATED: use NodeIntf/SubgraphImpl and automatic registration it provides.
//   Consult NodeIntf/NodeImpl for more details.
//
// This macro is used to define a subgraph that does not use automatic
// registration.
#define MEDIAPIPE_SUBGRAPH_IMPLEMENTATION(Impl)           \
  MEDIAPIPE_REGISTER_FACTORY_FUNCTION_QUALIFIED(          \
      mediapipe::SubgraphRegistry, subgraph_registration, \
      Impl::kCalculatorName, std::make_unique<Impl>)

}  // namespace api2
}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_API2_NODE_H_
