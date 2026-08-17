#ifndef MEDIAPIPE_FRAMEWORK_API2_BUILDER_H_
#define MEDIAPIPE_FRAMEWORK_API2_BUILDER_H_

#include <functional>
#include <map>
#include <memory>
#include <optional>
#include <string>
#include <type_traits>
#include <utility>
#include <vector>

#include "absl/container/btree_map.h"
#include "absl/log/absl_check.h"
#include "absl/memory/memory.h"
#include "absl/status/status.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/string_view.h"
#include "google/protobuf/message_lite.h"
#include "mediapipe/framework/api2/packet.h"
#include "mediapipe/framework/api2/port.h"
#include "mediapipe/framework/calculator_base.h"
#include "mediapipe/framework/calculator_contract.h"
#include "mediapipe/framework/mediapipe_options.pb.h"
#include "mediapipe/framework/port/any_proto.h"
#include "mediapipe/framework/port/ret_check.h"
#include "mediapipe/framework/port/status_builder.h"
#include "mediapipe/framework/port/status_macros.h"
#include "mediapipe/framework/stream_handler.pb.h"

namespace mediapipe {
namespace api2 {
namespace builder {

// Workaround for static_assert(false). Example:
//   dependent_false<T>::value returns false.
// For more information, see:
// https://en.cppreference.com/w/cpp/language/if#Constexpr_If
// TODO: migrate to a common utility when available.
template <class T>
struct dependent_false : std::false_type {};

template <typename T>
T& GetWithAutoGrow(std::vector<std::unique_ptr<T>>* vecp, size_t index) {
  auto& vec = *vecp;
  if (vec.size() <= index) {
    vec.resize(index + 1);
  }
  if (vec[index] == nullptr) {
    vec[index] = std::make_unique<T>();
  }
  return *vec[index];
}

struct TagIndexLocation {
  const std::string& tag;
  std::size_t index;
  std::size_t count;
};

template <typename T>
class TagIndexMap {
 public:
  std::vector<std::unique_ptr<T>>& operator[](absl::string_view tag) {
    return map_[tag];
  }

  absl::Status Visit(
      std::function<absl::Status(const TagIndexLocation&, T&)> fun) const {
    for (auto& tagged : map_) {
      TagIndexLocation loc{tagged.first, 0, tagged.second.size()};
      for (int i = 0; i < tagged.second.size(); ++i) {
        auto* item = tagged.second[i].get();
        loc.index = i;
        // If the item is nullptr, it means that the connection vector for
        // current tag grew by a GetWithAutoGrow() request but hasn't been
        // populated yet.
        if (item != nullptr) {
          MP_RETURN_IF_ERROR(fun(loc, *item));
        }
      }
    }
    return absl::OkStatus();
  }

 private:
  // Note: entries are held by a unique_ptr to ensure pointers remain valid.
  // Should use absl::flat_hash_map but ordering keys for now.
  absl::btree_map<std::string, std::vector<std::unique_ptr<T>>> map_;
};

class Graph;
class NodeBase;
class PacketGenerator;

// These structs are used internally to store information about the endpoints
// of a connection.
struct SourceBase;
struct DestinationBase {
  SourceBase* source = nullptr;
};
struct SourceBase {
  std::vector<DestinationBase*> dests_;
  std::string name_;
};

// Following existing GraphConfig usage, we allow using a multiport as a single
// port as well. This is necessary for generic nodes, since we have no
// information about which ports are meant to be multiports or not, but it is
// also convenient with typed nodes.
template <typename Single>
class MultiPort : public Single {
 public:
  using Base = typename Single::Base;

  explicit MultiPort(std::vector<std::unique_ptr<Base>>* vec)
      : Single(vec), vec_(*vec) {}

  Single operator[](int index) {
    ABSL_CHECK_GE(index, 0);
    return Single{&GetWithAutoGrow(&vec_, index)};
  }

  template <typename U>
  auto Cast() {
    using SingleCastT =
        std::invoke_result_t<decltype(&Single::template Cast<U>), Single*>;
    return MultiPort<SingleCastT>(&vec_);
  }

 private:
  std::vector<std::unique_ptr<Base>>& vec_;
};

namespace internal_builder {

template <typename T, typename U>
using AllowCast = std::integral_constant<bool, (std::is_same_v<T, AnyType> ||
                                                std::is_same_v<U, AnyType>) &&
                                                   !std::is_same_v<T, U>>;

}  // namespace internal_builder

template <bool IsSide, typename T = internal::Generic>
class SourceImpl;

// These classes wrap references to the underlying source/destination
// endpoints, adding type information and the user-visible API.
template <bool IsSide, typename T = internal::Generic>
class DestinationImpl {
 public:
  using Base = DestinationBase;

  explicit DestinationImpl(std::vector<std::unique_ptr<Base>>* vec)
      : DestinationImpl(&GetWithAutoGrow(vec, 0)) {}
  explicit DestinationImpl(DestinationBase* base) : base_(*base) {}

  template <typename U,
            std::enable_if_t<internal_builder::AllowCast<T, U>{}, int> = 0>
  DestinationImpl<IsSide, U> Cast() {
    return DestinationImpl<IsSide, U>(&base_);
  }

 private:
  DestinationBase& base_;

  template <bool Source_IsSide, typename Source_T>
  friend class SourceImpl;
};

template <bool IsSide, typename T>
class SourceImpl {
 public:
  using Base = SourceBase;
  using PayloadT = T;

  // Src is used as the return type of fluent methods below. Since these are
  // single-port methods, it is desirable to always decay to a reference to the
  // single-port superclass, even if they are called on a multiport.
  using Src = SourceImpl<IsSide, T>;
  template <typename U>
  using Dst = DestinationImpl<IsSide, U>;

  // clang-format off
  template <typename U>
  struct AllowConnection : public std::integral_constant<bool,
      std::is_same<T, U>{} || std::is_same<T, internal::Generic>{} ||
      std::is_same<U, internal::Generic>{}> {};
  // clang-format on

  explicit SourceImpl(std::vector<std::unique_ptr<Base>>* vec)
      : SourceImpl(&GetWithAutoGrow(vec, 0)) {}
  explicit SourceImpl(SourceBase* base) : base_(base) {}

  // Connects MediaPipe stream or side packet to a destination:
  // - node input (input stream) / side input (input side packet)
  // - graph output (output stream) / side output (output side packet).
  //
  // MediaPipe streams and side packets can be connected to multiple
  // destinations. Side packets and packets added to streams are sent to all
  // connected destinations.
  template <typename U,
            typename std::enable_if<AllowConnection<U>{}, int>::type = 0>
  Src& ConnectTo(const Dst<U>& dest) {
    ABSL_CHECK(dest.base_.source == nullptr);
    dest.base_.source = base_;
    base_->dests_.emplace_back(&dest.base_);
    return *this;
  }

  // Shortcut for `ConnectTo`.
  //
  // Connects MediaPipe stream or side packet to a destination:
  // - node input (input stream) / side input (input side packet)
  // - graph output (output stream) / side output (output side packet).
  //
  // MediaPipe streams and side packets can be connected to multiple
  // destinations. Side packets and packets added to streams are sent to all
  // connected destinations.
  template <typename U>
  Src& operator>>(const Dst<U>& dest) {
    return ConnectTo(dest);
  }

  template <typename U>
  bool operator==(const SourceImpl<IsSide, U>& other) {
    return base_ == other.base_;
  }

  template <typename U>
  bool operator!=(const SourceImpl<IsSide, U>& other) {
    return !(*this == other);
  }

  const std::string& Name() const { return base_->name_; }

  Src& SetName(const char* name) {
    base_->name_ = std::string(name);
    return *this;
  }

  Src& SetName(absl::string_view name) {
    base_->name_ = std::string(name);
    return *this;
  }

  Src& SetName(std::string name) {
    base_->name_ = std::move(name);
    return *this;
  }

  template <typename U,
            std::enable_if_t<internal_builder::AllowCast<T, U>{}, int> = 0>
  SourceImpl<IsSide, U> Cast() {
    return SourceImpl<IsSide, U>(base_);
  }

 private:
  template <bool, typename U>
  friend class SourceImpl;

  // Never null.
  SourceBase* base_;
};

// A source and a destination correspond to an output/input stream on a node,
// and a side source and side destination correspond to an output/input side
// packet.
// For graph inputs/outputs, however, the inputs are sources, and the outputs
// are destinations. This is because graph ports are connected "from inside"
// when building the graph.
template <typename T = internal::Generic>
using Source = SourceImpl<false, T>;

// Represents a stream of packets of a particular type.
//
// The intended use:
// - decouple input/output streams from graph/node during graph construction
// - pass streams around and connect them as needed, extracting reusable parts
//   to utility/convenience functions or classes.
//
// For example:
//   Stream<Image> Resize(Stream<Image> image, const Size& size, Graph& graph) {
//     auto& scaler_node = graph.AddNode("GlScalerCalculator");
//     auto& opts = scaler_node.GetOptions<GlScalerCalculatorOptions>();
//     opts.set_output_width(size.width);
//     opts.set_output_height(size.height);
//     a >> scaler_node.In("IMAGE");
//     return scaler_node.Out("IMAGE").Cast<Image>();
//   }
//
// Where graph can use it as:
//   Graph graph;
//   Stream<Image> input_image = graph.In("INPUT_IMAGE").Cast<Image>();
//   Stream<Image> resized_image = Resize(input_image, {64, 64}, graph);
template <typename T>
using Stream = Source<T>;

template <typename T = internal::Generic>
using MultiSource = MultiPort<Source<T>>;

template <typename T = internal::Generic>
using SideSource = SourceImpl<true, T>;

// Represents a side packet of a particular type.
//
// The intended use:
// - decouple input/output side packets from graph/node during graph
//   construction
// - pass side packets around and connect them as needed, extracting reusable
//   parts utility/convenience functions or classes.
//
// For example:
//   SidePacket<TfLiteModelPtr> GetModel(SidePacket<Resource> model_resource,
//                                       Graph& graph) {
//     auto& model_node = graph.AddNode("TfLiteModelCalculator");
//     model_resource >> model_node.SideIn("MODEL_RESOURCE");
//     return model_node.SideOut("MODEL").Cast<TfLiteModelPtr>();
//   }
//
// Where graph can use it as:
//   Graph graph;
//   SidePacket<Resource> model_resource =
//     graph.SideIn("MODEL_RESOURCE").Cast<Resource>();
//   SidePacket<TfLiteModelPtr> model = GetModel(model_resource, graph);
template <typename T>
using SidePacket = SideSource<T>;

template <typename T = internal::Generic>
using MultiSideSource = MultiPort<SideSource<T>>;

template <typename T = internal::Generic>
using Destination = DestinationImpl<false, T>;
template <typename T = internal::Generic>
using SideDestination = DestinationImpl<true, T>;
template <typename T = internal::Generic>
using MultiDestination = MultiPort<Destination<T>>;
template <typename T = internal::Generic>
using MultiSideDestination = MultiPort<SideDestination<T>>;

namespace internal_builder {

template <typename OptionsT>
OptionsT& GetOptions(std::optional<mediapipe::MediaPipeOptions>& options) {
  if (!options.has_value()) {
    options = mediapipe::MediaPipeOptions();
  }
  return *options->MutableExtension(OptionsT::ext);
}

}  // namespace internal_builder

class Executor {
 public:
  template <typename OptionsT>
  OptionsT& GetOptions() {
    return internal_builder::GetOptions<OptionsT>(options_);
  }

 private:
  explicit Executor(std::string type) : type_(std::move(type)) {}

  std::string type_;
  std::string name_;

  std::optional<mediapipe::MediaPipeOptions> options_;

  friend class Graph;
};

class NodeBase;

class InputStreamHandler {
 public:
  template <typename OptionsT>
  OptionsT& GetOptions() {
    return internal_builder::GetOptions<OptionsT>(options_);
  }

 protected:
  explicit InputStreamHandler() = default;

  std::string type_;
  std::optional<mediapipe::MediaPipeOptions> options_;

  friend class NodeBase;
  friend class Graph;
};

class OutputStreamHandler {
 public:
  template <typename OptionsT>
  OptionsT& GetOptions() {
    return internal_builder::GetOptions<OptionsT>(options_);
  }

 protected:
  explicit OutputStreamHandler() = default;

  std::string type_;
  std::optional<mediapipe::MediaPipeOptions> options_;

  friend class NodeBase;
  friend class Graph;
};

class NodeBase {
 public:
  NodeBase() = default;
  ~NodeBase() = default;
  NodeBase(NodeBase&&) = default;
  NodeBase& operator=(NodeBase&&) = default;
  // Explicitly delete copies to improve error messages.
  NodeBase(const NodeBase&) = delete;
  NodeBase& operator=(const NodeBase&) = delete;

  // TODO: right now access to an indexed port is made directly by
  // specifying both a tag and an index. It would be better to represent this
  // as a two-step lookup, first getting a multi-port, and then accessing one
  // of its entries by index. However, for nodes without visible contracts we
  // can't know whether a tag is indexable or not, so we would need the
  // multi-port to also be usable as a port directly (representing index 0).
  MultiSource<> Out(absl::string_view tag) {
    return MultiSource<>(&out_streams_[tag]);
  }

  MultiDestination<> In(absl::string_view tag) {
    return MultiDestination<>(&in_streams_[tag]);
  }

  MultiSideSource<> SideOut(absl::string_view tag) {
    return MultiSideSource<>(&out_sides_[tag]);
  }

  MultiSideDestination<> SideIn(absl::string_view tag) {
    return MultiSideDestination<>(&in_sides_[tag]);
  }

  template <typename B, typename T, bool kIsOptional, bool kIsMultiple>
  auto operator[](const PortCommon<B, T, kIsOptional, kIsMultiple>& port) {
    using PayloadT =
        typename PortCommon<B, T, kIsOptional, kIsMultiple>::PayloadT;
    if constexpr (std::is_same_v<B, OutputBase>) {
      auto* base = &out_streams_[port.Tag()];
      if constexpr (kIsMultiple) {
        return MultiSource<PayloadT>(base);
      } else {
        return Source<PayloadT>(base);
      }
    } else if constexpr (std::is_same_v<B, InputBase>) {
      auto* base = &in_streams_[port.Tag()];
      if constexpr (kIsMultiple) {
        return MultiDestination<PayloadT>(base);
      } else {
        return Destination<PayloadT>(base);
      }
    } else if constexpr (std::is_same_v<B, SideOutputBase>) {
      auto* base = &out_sides_[port.Tag()];
      if constexpr (kIsMultiple) {
        return MultiSideSource<PayloadT>(base);
      } else {
        return SideSource<PayloadT>(base);
      }
    } else if constexpr (std::is_same_v<B, SideInputBase>) {
      auto* base = &in_sides_[port.Tag()];
      if constexpr (kIsMultiple) {
        return MultiSideDestination<PayloadT>(base);
      } else {
        return SideDestination<PayloadT>(base);
      }
    } else {
      static_assert(dependent_false<B>::value, "Type not supported.");
    }
  }

  // Convenience methods for accessing purely index-based ports.
  Source<> Out(int index) { return Out("")[index]; }

  Destination<> In(int index) { return In("")[index]; }

  SideSource<> SideOut(int index) { return SideOut("")[index]; }

  SideDestination<> SideIn(int index) { return SideIn("")[index]; }

  // Get mutable node options of type Options.
  template <
      typename OptionsT,
      typename std::enable_if<std::is_base_of<
          google::protobuf::MessageLite, OptionsT>::value>::type* = nullptr>
  OptionsT& GetOptions() {
    return GetOptionsInternal<OptionsT>(nullptr);
  }

  // Use this API when the proto extension does not follow the "ext" naming
  // convention.
  template <typename ExtensionT>
  auto& GetOptions(const ExtensionT& ext) {
    if (!calculator_option_.has_value()) {
      calculator_option_ = CalculatorOptions();
    }
    return *calculator_option_->MutableExtension(ext);
  }

  void SetExecutor(Executor& executor) { executor_ = &executor; }

  InputStreamHandler& SetInputStreamHandler(absl::string_view type) {
    if (!input_stream_handler_) {
      input_stream_handler_ = InputStreamHandler();
    }
    input_stream_handler_->type_ = std::string(type.data(), type.size());
    return *input_stream_handler_;
  }

  OutputStreamHandler& SetOutputStreamHandler(absl::string_view type) {
    if (!output_stream_handler_) {
      output_stream_handler_ = OutputStreamHandler();
    }
    output_stream_handler_->type_ = std::string(type.data(), type.size());
    return *output_stream_handler_;
  }

 protected:
  // GetOptionsInternal resolutes the overload greedily, which finds the first
  // match then succeed (template specialization tries all matches, thus could
  // be ambiguous)
  template <typename OptionsT>
  OptionsT& GetOptionsInternal(decltype(&OptionsT::ext) /*unused*/) {
    return GetOptions(OptionsT::ext);
  }
  template <typename OptionsT>
  OptionsT& GetOptionsInternal(...) {
    if (node_options_.count(kTypeId<OptionsT>)) {
      return *static_cast<OptionsT*>(
          node_options_[kTypeId<OptionsT>].message.get());
    }
    auto option = std::make_unique<OptionsT>();
    OptionsT* option_ptr = option.get();
    node_options_[kTypeId<OptionsT>] = {
        std::move(option),
        [option_ptr](protobuf::Any& any) { return any.PackFrom(*option_ptr); }};
    return *option_ptr;
  }

  NodeBase(std::string type) : type_(std::move(type)) {}

  std::string type_;
  TagIndexMap<DestinationBase> in_streams_;
  TagIndexMap<SourceBase> out_streams_;
  TagIndexMap<DestinationBase> in_sides_;
  TagIndexMap<SourceBase> out_sides_;
  std::optional<CalculatorOptions> calculator_option_;
  // Stores real proto config, and lambda for packing config into Any.
  // We need the lambda because PackFrom() does not work with MessageLite.
  struct MessageAndPacker {
    std::unique_ptr<google::protobuf::MessageLite> message;
    std::function<bool(protobuf::Any&)> packer;
  };
  std::map<TypeId, MessageAndPacker> node_options_;

  Executor* executor_ = nullptr;

  std::optional<InputStreamHandler> input_stream_handler_;
  std::optional<OutputStreamHandler> output_stream_handler_;

  friend class Graph;
};

template <class Calc = internal::Generic>
class Node;
#if __cplusplus >= 201703L
// Deduction guide to silence -Wctad-maybe-unsupported.
explicit Node() -> Node<internal::Generic>;
#endif  // C++17

template <>
class Node<internal::Generic> : public NodeBase {
 public:
  Node(std::string type) : NodeBase(std::move(type)) {}
};

using GenericNode = Node<internal::Generic>;

template <class Calc>
class Node : public NodeBase {
 public:
  Node()
      : NodeBase(
            FunctionRegistry<NodeBase>::GetLookupName(Calc::kCalculatorName)) {}

  // Overrides the built-in calculator type string with the provided argument.
  // Can be used to create nodes from pure interfaces.
  // TODO: only use this for pure interfaces
  Node(std::string type_override) : NodeBase(std::move(type_override)) {}

  // These methods only allow access to ports declared in the contract.
  // The argument must be a tag object created with the MPP_TAG macro.
  // These objects encode the tag in their type, which allows us to return
  // a result with the appropriate payload type depending on the tag.
  template <class Tag>
  auto Out(Tag tag) {
    constexpr auto& port = Calc::Contract::TaggedOutputs::get(tag);
    return NodeBase::operator[](port);
  }

  template <class Tag>
  auto In(Tag tag) {
    constexpr auto& port = Calc::Contract::TaggedInputs::get(tag);
    return NodeBase::operator[](port);
  }

  template <class Tag>
  auto SideOut(Tag tag) {
    constexpr auto& port = Calc::Contract::TaggedSideOutputs::get(tag);
    return NodeBase::operator[](port);
  }

  template <class Tag>
  auto SideIn(Tag tag) {
    constexpr auto& port = Calc::Contract::TaggedSideInputs::get(tag);
    return NodeBase::operator[](port);
  }

  // We could allow using the non-checked versions with typed nodes too, but
  // we don't.
  // using NodeBase::Out;
  // using NodeBase::In;
  // using NodeBase::SideOut;
  // using NodeBase::SideIn;
};

// For legacy PacketGenerators.
class PacketGenerator {
 public:
  PacketGenerator(std::string type) : type_(std::move(type)) {}

  MultiSideSource<> SideOut(absl::string_view tag) {
    return MultiSideSource<>(&out_sides_[tag]);
  }

  MultiSideDestination<> SideIn(absl::string_view tag) {
    return MultiSideDestination<>(&in_sides_[tag]);
  }

  // Convenience methods for accessing purely index-based ports.
  SideSource<> SideOut(int index) { return SideOut("")[index]; }
  SideDestination<> SideIn(int index) { return SideIn("")[index]; }

  template <typename T>
  T& GetOptions() {
    return GetOptions(T::ext);
  }

  // Use this API when the proto extension does not follow the "ext" naming
  // convention.
  template <typename E>
  auto& GetOptions(const E& extension) {
    options_used_ = true;
    return *options_.MutableExtension(extension);
  }

  template <typename B, typename T, bool kIsOptional, bool kIsMultiple>
  auto operator[](const PortCommon<B, T, kIsOptional, kIsMultiple>& port) {
    using PayloadT =
        typename PortCommon<B, T, kIsOptional, kIsMultiple>::PayloadT;
    if constexpr (std::is_same_v<B, SideOutputBase>) {
      auto* base = &out_sides_[port.Tag()];
      if constexpr (kIsMultiple) {
        return MultiSideSource<PayloadT>(base);
      } else {
        return SideSource<PayloadT>(base);
      }
    } else if constexpr (std::is_same_v<B, SideInputBase>) {
      auto* base = &in_sides_[port.Tag()];
      if constexpr (kIsMultiple) {
        return MultiSideDestination<PayloadT>(base);
      } else {
        return SideDestination<PayloadT>(base);
      }
    } else {
      static_assert(dependent_false<B>::value, "Type not supported.");
    }
  }

 private:
  std::string type_;
  TagIndexMap<DestinationBase> in_sides_;
  TagIndexMap<SourceBase> out_sides_;
  mediapipe::PacketGeneratorOptions options_;
  // ideally we'd just check if any extensions are set on options_
  bool options_used_ = false;
  friend class Graph;
};

class Graph {
 public:
  Graph() = default;
  ~Graph() = default;
  Graph(Graph&&) = default;
  Graph& operator=(Graph&&) = default;
  // Explicitly delete copies to improve error messages.
  Graph(const Graph&) = delete;
  Graph& operator=(const Graph&) = delete;

  void SetType(std::string type) { type_ = std::move(type); }

  // Creates a node of a specific type. Should be used for calculators whose
  // contract is available.
  template <class Calc>
  Node<Calc>& AddNode() {
    auto node = std::make_unique<Node<Calc>>();
    auto node_p = node.get();
    nodes_.emplace_back(std::move(node));
    return *node_p;
  }

  // Creates a node of a specific type. Should be used for pure interfaces,
  // which do not have a built-in type string.
  // `type` is a calculator type-name with dot-separated namespaces.
  template <class Calc>
  Node<Calc>& AddNode(absl::string_view type) {
    auto node =
        std::make_unique<Node<Calc>>(std::string(type.data(), type.size()));
    auto node_p = node.get();
    nodes_.emplace_back(std::move(node));
    return *node_p;
  }

  // Creates a generic node, with no compile-time checking of inputs and
  // outputs. This can be used for calculators whose contract is not visible.
  // `type` is a calculator type-name with dot-separated namespaces.
  GenericNode& AddNode(absl::string_view type) {
    auto node =
        std::make_unique<GenericNode>(std::string(type.data(), type.size()));
    auto node_p = node.get();
    nodes_.emplace_back(std::move(node));
    return *node_p;
  }

  // For legacy PacketGenerators.
  PacketGenerator& AddPacketGenerator(absl::string_view type) {
    auto node = std::make_unique<PacketGenerator>(
        std::string(type.data(), type.size()));
    auto node_p = node.get();
    packet_gens_.emplace_back(std::move(node));
    return *node_p;
  }

  Executor& AddExecutor(absl::string_view type) {
    auto executor =
        absl::WrapUnique(new Executor(std::string(type.data(), type.size())));
    auto* executor_p = executor.get();
    executors_.emplace_back(std::move(executor));
    return *executor_p;
  }

  // Graph ports, non-typed.
  MultiSource<> In(absl::string_view graph_input) {
    return graph_boundary_.Out(graph_input);
  }

  MultiDestination<> Out(absl::string_view graph_output) {
    return graph_boundary_.In(graph_output);
  }

  MultiSideSource<> SideIn(absl::string_view graph_input) {
    return graph_boundary_.SideOut(graph_input);
  }

  MultiSideDestination<> SideOut(absl::string_view graph_output) {
    return graph_boundary_.SideIn(graph_output);
  }

  // Convenience methods for accessing purely index-based ports.
  Source<> In(int index) { return In("")[index]; }

  Destination<> Out(int index) { return Out("")[index]; }

  SideSource<> SideIn(int index) { return SideIn("")[index]; }

  SideDestination<> SideOut(int index) { return SideOut("")[index]; }

  // Graph ports, typed.
  // TODO: make graph_boundary_ a typed node!
  template <class PortT, class Payload = typename PortT::PayloadT>
  auto In(const PortT& graph_input) {
    return (*this)[graph_input];
  }

  template <class PortT, class Payload = typename PortT::PayloadT>
  auto Out(const PortT& graph_output) {
    return (*this)[graph_output];
  }

  template <class PortT, class Payload = typename PortT::PayloadT>
  auto SideIn(const PortT& graph_input) {
    return (*this)[graph_input];
  }

  template <class PortT, class Payload = typename PortT::PayloadT>
  auto SideOut(const PortT& graph_output) {
    return (*this)[graph_output];
  }

  template <typename B, typename T, bool kIsOptional, bool kIsMultiple>
  auto operator[](const PortCommon<B, T, kIsOptional, kIsMultiple>& port) {
    using PayloadT =
        typename PortCommon<B, T, kIsOptional, kIsMultiple>::PayloadT;
    if constexpr (std::is_same_v<B, OutputBase>) {
      auto* base = &graph_boundary_.in_streams_[port.Tag()];
      if constexpr (kIsMultiple) {
        return MultiDestination<PayloadT>(base);
      } else {
        return Destination<PayloadT>(base);
      }
    } else if constexpr (std::is_same_v<B, InputBase>) {
      auto* base = &graph_boundary_.out_streams_[port.Tag()];
      if constexpr (kIsMultiple) {
        return MultiSource<PayloadT>(base);
      } else {
        return Source<PayloadT>(base);
      }
    } else if constexpr (std::is_same_v<B, SideOutputBase>) {
      auto* base = &graph_boundary_.in_sides_[port.Tag()];
      if constexpr (kIsMultiple) {
        return MultiSideDestination<PayloadT>(base);
      } else {
        return SideDestination<PayloadT>(base);
      }
    } else if constexpr (std::is_same_v<B, SideInputBase>) {
      auto* base = &graph_boundary_.out_sides_[port.Tag()];
      if constexpr (kIsMultiple) {
        return MultiSideSource<PayloadT>(base);
      } else {
        return SideSource<PayloadT>(base);
      }
    } else {
      static_assert(dependent_false<B>::value, "Type not supported.");
    }
  }

  // Returns the graph config. This can be used to instantiate and run the
  // graph.
  CalculatorGraphConfig GetConfig() {
    CalculatorGraphConfig config;
    if (!type_.empty()) {
      config.set_type(type_);
    }

    // Name and add executors.
    int executor_index = 0;
    for (std::unique_ptr<Executor>& executor : executors_) {
      // Names starting from "__" are historically reserved for internal
      // executors.
      executor->name_ = absl::StrCat("_b_executor_", executor_index++);

      auto* out_executor = config.add_executor();
      out_executor->set_name(executor->name_);
      out_executor->set_type(executor->type_);
      if (executor->options_) {
        *out_executor->mutable_options() = *executor->options_;
      }
    }

    ABSL_CHECK_OK(FixUnnamedConnections());
    ABSL_CHECK_OK(UpdateBoundaryConfig(&config));
    for (const std::unique_ptr<NodeBase>& node : nodes_) {
      auto* out_node = config.add_node();
      ABSL_CHECK_OK(UpdateNodeConfig(*node, out_node));
    }
    for (const std::unique_ptr<PacketGenerator>& node : packet_gens_) {
      auto* out_node = config.add_packet_generator();
      ABSL_CHECK_OK(UpdateNodeConfig(*node, out_node));
    }
    return config;
  }

 private:
  absl::Status FixUnnamedConnections(NodeBase* node, int* unnamed_count) {
    MP_RETURN_IF_ERROR(node->out_streams_.Visit(
        [&](const TagIndexLocation& loc, SourceBase& source) -> absl::Status {
          if (source.name_.empty()) {
            source.name_ = absl::StrCat("__stream_", (*unnamed_count)++);
          }
          return absl::OkStatus();
        }));

    MP_RETURN_IF_ERROR(node->out_sides_.Visit(
        [&](const TagIndexLocation& loc, SourceBase& source) -> absl::Status {
          if (source.name_.empty()) {
            source.name_ = absl::StrCat("__side_packet_", (*unnamed_count)++);
          }
          return absl::OkStatus();
        }));
    return absl::OkStatus();
  }

  absl::Status FixUnnamedConnections() {
    int unnamed_count = 0;
    MP_RETURN_IF_ERROR(FixUnnamedConnections(&graph_boundary_, &unnamed_count));
    for (std::unique_ptr<NodeBase>& node : nodes_) {
      MP_RETURN_IF_ERROR(FixUnnamedConnections(node.get(), &unnamed_count));
    }
    for (std::unique_ptr<PacketGenerator>& node : packet_gens_) {
      MP_RETURN_IF_ERROR(node->out_sides_.Visit(
          [&](const TagIndexLocation& loc, SourceBase& source) -> absl::Status {
            if (source.name_.empty()) {
              source.name_ = absl::StrCat("__side_packet_", unnamed_count++);
            }
            return absl::OkStatus();
          }));
    }
    return absl::OkStatus();
  }

  std::string TaggedName(const TagIndexLocation& loc, absl::string_view name) {
    if (loc.tag.empty()) {
      // ParseTagIndexName does not allow using explicit indices without tags,
      // while ParseTagIndex does.
      // TODO: decide whether we should just allow it.
      return std::string(name);
    } else {
      if (loc.count <= 1) {
        return absl::StrCat(loc.tag, ":", name);
      } else {
        return absl::StrCat(loc.tag, ":", loc.index, ":", name);
      }
    }
  }

  absl::Status UpdateNodeConfig(const NodeBase& node,
                                CalculatorGraphConfig::Node* config) {
    config->set_calculator(node.type_);
    MP_RETURN_IF_ERROR(node.in_streams_.Visit(
        [&](const TagIndexLocation& loc,
            const DestinationBase& endpoint) -> absl::Status {
          RET_CHECK(endpoint.source != nullptr)
              << node.type_ << ": Missing source for input stream with tag "
              << (loc.tag.empty() ? "(empty)" : loc.tag) << " at index "
              << loc.index;
          config->add_input_stream(TaggedName(loc, endpoint.source->name_));
          return absl::OkStatus();
        }));
    MP_RETURN_IF_ERROR(node.out_streams_.Visit(
        [&](const TagIndexLocation& loc,
            const SourceBase& endpoint) -> absl::Status {
          config->add_output_stream(TaggedName(loc, endpoint.name_));
          return absl::OkStatus();
        }));
    MP_RETURN_IF_ERROR(node.in_sides_.Visit(
        [&](const TagIndexLocation& loc,
            const DestinationBase& endpoint) -> absl::Status {
          RET_CHECK(endpoint.source != nullptr)
              << node.type_
              << ": Missing source for input side packet stream with tag "
              << loc.tag << " at index " << loc.index;
          config->add_input_side_packet(
              TaggedName(loc, endpoint.source->name_));
          return absl::OkStatus();
        }));
    MP_RETURN_IF_ERROR(
        node.out_sides_.Visit([&](const TagIndexLocation& loc,
                                  const SourceBase& endpoint) -> absl::Status {
          config->add_output_side_packet(TaggedName(loc, endpoint.name_));
          return absl::OkStatus();
        }));
    if (node.calculator_option_.has_value()) {
      *config->mutable_options() = *node.calculator_option_;
    }
    for (auto& [type_id, message_and_packer] : node.node_options_) {
      RET_CHECK(message_and_packer.packer(*config->add_node_options()));
    }
    if (node.executor_ != nullptr) {
      config->set_executor(node.executor_->name_);
    }
    if (node.input_stream_handler_) {
      config->mutable_input_stream_handler()->set_input_stream_handler(
          node.input_stream_handler_->type_);
      if (node.input_stream_handler_->options_) {
        *config->mutable_input_stream_handler()->mutable_options() =
            *node.input_stream_handler_->options_;
      }
    }
    if (node.output_stream_handler_) {
      config->mutable_output_stream_handler()->set_output_stream_handler(
          node.output_stream_handler_->type_);
      if (node.output_stream_handler_->options_) {
        *config->mutable_output_stream_handler()->mutable_options() =
            *node.output_stream_handler_->options_;
      }
    }
    return {};
  }

  absl::Status UpdateNodeConfig(const PacketGenerator& node,
                                PacketGeneratorConfig* config) {
    config->set_packet_generator(node.type_);
    MP_RETURN_IF_ERROR(node.in_sides_.Visit(
        [&](const TagIndexLocation& loc,
            const DestinationBase& endpoint) -> absl::Status {
          RET_CHECK(endpoint.source != nullptr)
              << node.type_
              << ": Missing source for input side packet stream with tag "
              << (loc.tag.empty() ? "(empty)" : loc.tag) << " at index "
              << loc.index;
          config->add_input_side_packet(
              TaggedName(loc, endpoint.source->name_));
          return absl::OkStatus();
        }));
    MP_RETURN_IF_ERROR(
        node.out_sides_.Visit([&](const TagIndexLocation& loc,
                                  const SourceBase& endpoint) -> absl::Status {
          config->add_output_side_packet(TaggedName(loc, endpoint.name_));
          return absl::OkStatus();
        }));
    if (node.options_used_) {
      *config->mutable_options() = node.options_;
    }
    return {};
  }

  // For special boundary node.
  absl::Status UpdateBoundaryConfig(CalculatorGraphConfig* config) {
    MP_RETURN_IF_ERROR(graph_boundary_.in_streams_.Visit(
        [&](const TagIndexLocation& loc,
            const DestinationBase& endpoint) -> absl::Status {
          RET_CHECK(endpoint.source != nullptr)
              << type_ << ": Missing source for graph output stream with tag "
              << (loc.tag.empty() ? "(empty)" : loc.tag) << " at index "
              << loc.index;
          config->add_output_stream(TaggedName(loc, endpoint.source->name_));
          return absl::OkStatus();
        }));
    MP_RETURN_IF_ERROR(graph_boundary_.out_streams_.Visit(
        [&](const TagIndexLocation& loc,
            const SourceBase& endpoint) -> absl::Status {
          config->add_input_stream(TaggedName(loc, endpoint.name_));
          return absl::OkStatus();
        }));
    MP_RETURN_IF_ERROR(graph_boundary_.in_sides_.Visit(
        [&](const TagIndexLocation& loc,
            const DestinationBase& endpoint) -> absl::Status {
          RET_CHECK(endpoint.source != nullptr)
              << type_
              << ": Missing source for graph output side packet stream with "
                 "tag "
              << (loc.tag.empty() ? "(empty)" : loc.tag) << " at index "
              << loc.index;
          config->add_output_side_packet(
              TaggedName(loc, endpoint.source->name_));
          return absl::OkStatus();
        }));
    MP_RETURN_IF_ERROR(graph_boundary_.out_sides_.Visit(
        [&](const TagIndexLocation& loc,
            const SourceBase& endpoint) -> absl::Status {
          config->add_input_side_packet(TaggedName(loc, endpoint.name_));

          return absl::OkStatus();
        }));
    return absl::OkStatus();
  }

  std::string type_;
  std::vector<std::unique_ptr<Executor>> executors_;
  std::vector<std::unique_ptr<NodeBase>> nodes_;
  std::vector<std::unique_ptr<PacketGenerator>> packet_gens_;
  // Special node representing graph inputs and outputs.
  NodeBase graph_boundary_{"__GRAPH__"};
};

}  // namespace builder
}  // namespace api2
}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_API2_BUILDER_H_
