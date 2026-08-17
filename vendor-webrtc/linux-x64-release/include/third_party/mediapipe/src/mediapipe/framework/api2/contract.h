#ifndef MEDIAPIPE_FRAMEWORK_API2_CONTRACT_H_
#define MEDIAPIPE_FRAMEWORK_API2_CONTRACT_H_

#include <tuple>
#include <type_traits>
#include <utility>
#include <vector>

#include "mediapipe/framework/api2/const_str.h"
#include "mediapipe/framework/api2/packet.h"
#include "mediapipe/framework/api2/port.h"
#include "mediapipe/framework/api2/tag.h"
#include "mediapipe/framework/api2/tuple.h"
#include "mediapipe/framework/calculator_context.h"
#include "mediapipe/framework/calculator_contract.h"
#include "mediapipe/framework/output_side_packet.h"
#include "mediapipe/framework/port/logging.h"

namespace mediapipe {
namespace api2 {

class StreamHandler {
 public:
  template <std::size_t N>
  explicit constexpr StreamHandler(const char (&name)[N]) : name_(N, name) {}

  const const_str& name() { return name_; }

  absl::Status AddToContract(CalculatorContract* cc) const {
    cc->SetInputStreamHandler(name_.data());
    return {};
  }

 private:
  const const_str name_;
};

class TimestampChange {
 public:
  // Note: we don't use TimestampDiff as an argument because it's not constexpr.
  static constexpr TimestampChange Offset(int64_t offset) {
    return TimestampChange(offset);
  }

  static constexpr TimestampChange Arbitrary() {
    // Same value as used for Timestamp::Unset.
    return TimestampChange(kUnset);
  }

  absl::Status AddToContract(CalculatorContract* cc) const {
    if (offset_ != kUnset) cc->SetTimestampOffset(offset_);
    return {};
  }

 private:
  constexpr TimestampChange(int64_t offset) : offset_(offset) {}
  static constexpr int64_t kUnset = std::numeric_limits<int64_t>::min();
  int64_t offset_;
};

namespace internal {

template <class Base>
struct IsSubclass {
  template <class T>
  using pred = std::is_base_of<Base, std::decay_t<T>>;
};

template <class T, class = void>
struct HasProcessMethod : std::false_type {};

template <class T>
struct HasProcessMethod<
    T,
    std::void_t<decltype(absl::Status(std::declval<std::decay_t<T>>().Process(
        std::declval<mediapipe::CalculatorContext*>())))>> : std::true_type {};

template <class T, class = void>
struct HasNestedItems : std::false_type {};

template <class T>
struct HasNestedItems<
    T, std::void_t<decltype(std::declval<std::decay_t<T>>().nested_items())>>
    : std::true_type {};

// Helper to construct a tuple of Tag types (see tag.h) from a tuple of ports.
template <class TupleRef>
struct TagTuple {
  template <std::size_t J>
  struct S {
    const const_str tag{std::get<J>(TupleRef::get()).tag_};
  };

  template <std::size_t... I>
  static constexpr auto Make(std::index_sequence<I...> indices) {
    return std::make_tuple(mediapipe::api2::internal::tag_build(S<I>{})...);
  }

  static constexpr auto Make() {
    using TupleT = decltype(TupleRef::get());
    return Make(internal::tuple_index_sequence<TupleT>());
  }
};

// Helper to access a tuple of ports by static tag. Attempts to look up a
// missing tag will not compile.
template <class TupleRef>
struct TaggedAccess {
  // This is not functionally necessary (we could do the tag search directly
  // on the port tuple), but it gives a more readable error message when the
  // static_assert below fails.
  static constexpr auto kTagTuple = TagTuple<TupleRef>::Make();

  template <class Tag>
  static constexpr auto& get(Tag tag) {
    constexpr auto i =
        internal::tuple_find([tag](auto x) { return x == tag; }, kTagTuple);
    static_assert(i < std::tuple_size_v<decltype(kTagTuple)>, "tag not found");
    return std::get<i>(TupleRef::get());
  }
};

template <class... T>
constexpr auto ExtractNestedItems(std::tuple<T...> tuple) {
  return internal::flatten_tuple(internal::map_tuple(
      [](auto&& item) {
        if constexpr (HasNestedItems<decltype(item)>{}) {
          return std::tuple_cat(std::make_tuple(item), item.nested_items());
        } else {
          return std::make_tuple(item);
        }
      },
      tuple));
}

// Internal contract type. Takes a list of ports or other contract items.
template <typename... T>
class Contract {
 public:
  constexpr Contract(std::tuple<T...> tuple) : items(tuple) {}
  constexpr Contract(T&&... args)
      : Contract(std::tuple<T...>{std::move(args)...}) {}

  absl::Status GetContract(mediapipe::CalculatorContract* cc) const {
    std::vector<absl::Status> statuses;
    auto store_status = [&statuses](absl::Status status) {
      if (!status.ok()) statuses.push_back(std::move(status));
    };
    internal::tuple_for_each(
        [cc, &store_status](auto&& item) {
          store_status(item.AddToContract(cc));
        },
        all_items);

    if (timestamp_change_count() == 0) {
      // Default to SetOffset(0);
      store_status(TimestampChange::Offset(0).AddToContract(cc));
    }

    if (statuses.empty()) return {};
    if (statuses.size() == 1) return statuses[0];
    return tool::CombinedStatus("Multiple errors", statuses);
  }

  std::tuple<T...> items;

  // TODO: when forwarding nested items (e.g. ports), check for conflicts.
  decltype(ExtractNestedItems(items)) all_items{ExtractNestedItems(items)};

  constexpr auto inputs() const {
    return internal::filter_tuple<IsSubclass<InputBase>::pred>(all_items);
  }
  constexpr auto outputs() const {
    return internal::filter_tuple<IsSubclass<OutputBase>::pred>(all_items);
  }
  constexpr auto side_inputs() const {
    return internal::filter_tuple<IsSubclass<SideInputBase>::pred>(all_items);
  }
  constexpr auto side_outputs() const {
    return internal::filter_tuple<IsSubclass<SideOutputBase>::pred>(all_items);
  }

  constexpr auto timestamp_change_count() const {
    return internal::filtered_tuple_indices<IsSubclass<TimestampChange>::pred>(
               all_items)
        .size();
  }

  constexpr auto process_items() const {
    return internal::filter_tuple<HasProcessMethod>(all_items);
  }
};

// Helpers to construct a Contract.
template <typename... T>
constexpr auto MakeContract(T&&... args) {
  return Contract<T...>(std::forward<T>(args)...);
}

template <typename... T>
constexpr auto MakeContract(const std::tuple<T...>& tuple) {
  return Contract<T...>(tuple);
}

// Helper for accessing the ports of a Contract by static tags.
template <typename C2T, const C2T& c2>
class TaggedContract {
 public:
  constexpr TaggedContract() = default;

  static absl::Status GetContract(mediapipe::CalculatorContract* cc) {
    return c2.GetContract(cc);
  }

  template <class Tuple, Tuple (C2T::*member)() const>
  struct GetMember {
    static constexpr auto get() { return (c2.*member)(); }
  };

  using TaggedInputs =
      TaggedAccess<GetMember<decltype(c2.inputs()), &C2T::inputs>>;
  using TaggedOutputs =
      TaggedAccess<GetMember<decltype(c2.outputs()), &C2T::outputs>>;
  using TaggedSideInputs =
      TaggedAccess<GetMember<decltype(c2.side_inputs()), &C2T::side_inputs>>;
  using TaggedSideOutputs =
      TaggedAccess<GetMember<decltype(c2.side_outputs()), &C2T::side_outputs>>;
};

// Support for function-based Process.

template <class T>
struct IsInputPort
    : std::bool_constant<std::is_base_of<InputBase, std::decay_t<T>>{} ||
                         std::is_base_of<SideInputBase, std::decay_t<T>>{}> {};

template <class T>
struct IsOutputPort
    : std::bool_constant<std::is_base_of<OutputBase, std::decay_t<T>>{} ||
                         std::is_base_of<SideOutputBase, std::decay_t<T>>{}> {};

// Helper class that converts a port specification into a function argument.
template <class P>
class PortArg {
 public:
  PortArg(CalculatorContext* cc, const P& port) : cc_(cc), port_(port) {}

  using PayloadT = typename P::PayloadT;

  operator const PayloadT&() { return port_(cc_).Get(); }

  operator Packet<typename P::value_t>() { return port_(cc_); }

  operator PacketBase() { return port_(cc_).packet(); }

 private:
  CalculatorContext* cc_;
  const P& port_;
};

template <class P>
auto MakePortArg(CalculatorContext* cc, const P& port) {
  return PortArg<P>(cc, port);
}

// Helper class that takes a function result and sends it into outputs.
template <class... P>
class OutputSender {
 public:
  OutputSender(P&&... args) : outputs_(args...) {}
  OutputSender(std::tuple<P...>&& args) : outputs_(args) {}

  template <class R, std::enable_if_t<sizeof...(P) == 1, int> = 0>
  absl::Status operator()(CalculatorContext* cc, absl::StatusOr<R>&& result) {
    if (result.ok()) {
      return this(cc, result.value());
    } else {
      return result.status();
    }
  }

  template <class R, std::enable_if_t<sizeof...(P) == 1, int> = 0>
  absl::Status operator()(CalculatorContext* cc, R&& result) {
    std::get<0>(outputs_)(cc).Send(std::forward<R>(result));
    return {};
  }

  template <class... R>
  absl::Status operator()(CalculatorContext* cc,
                          absl::StatusOr<std::tuple<R...>>&& result) {
    if (result.ok()) {
      return this(cc, result.value());
    } else {
      return result.status();
    }
  }

  template <class... R>
  absl::Status operator()(CalculatorContext* cc, std::tuple<R...>&& result) {
    static_assert(sizeof...(P) == sizeof...(R), "");
    internal::tuple_for_each(
        [cc, &result](const auto& port, auto i_const) {
          constexpr std::size_t i = decltype(i_const)::value;
          port(cc).Send(std::get<i>(result));
        },
        outputs_);
    return {};
  }

  std::tuple<P...> outputs_;
};

template <class... P>
auto MakeOutputSender(P&&... args) {
  return OutputSender<P...>(std::forward<P>(args)...);
}

template <class... P>
auto MakeOutputSender(std::tuple<P...>&& args) {
  return OutputSender<P...>(std::forward<std::tuple<P...>>(args));
}

// Contract item that specifies that certain I/O ports are handled by invoking
// a specific function.
template <class F, class... P>
class FunCaller {
 public:
  constexpr FunCaller(F&& f, P&&... args) : f_(f), args_(args...) {}

  auto operator()(CalculatorContext* cc) const {
    auto output_sender = MakeOutputSender(outputs());
    // tuple_apply gives better error messages than std::apply if the argument
    // types don't match.
    return output_sender(
        cc, internal::tuple_apply(f_, internal::map_tuple(
                                          [cc](const auto& port) {
                                            return MakePortArg(cc, port);
                                          },
                                          inputs())));
  }

  auto inputs() const { return internal::filter_tuple<IsInputPort>(args_); }
  auto outputs() const { return internal::filter_tuple<IsOutputPort>(args_); }

  absl::Status AddToContract(CalculatorContract* cc) const { return {}; }

  absl::Status Process(CalculatorContext* cc) const { return (*this)(cc); }

  constexpr std::tuple<P...> nested_items() const { return args_; }

  F f_;
  std::tuple<P...> args_;
};

// Helper function to invoke function callers in Process.

// TODO: implement multiple callers for syncsets.
template <class... T>
absl::Status ProcessFnCallers(CalculatorContext* cc, std::tuple<T...> callers);

inline absl::Status ProcessFnCallers(CalculatorContext* cc, std::tuple<>) {
  return absl::InternalError("Process unimplemented");
}

template <class T>
absl::Status ProcessFnCallers(CalculatorContext* cc, std::tuple<T> callers) {
  return std::get<0>(callers).Process(cc);
}

}  // namespace internal

// Function used to add a process function to a calculator contract.
template <class F, class... P>
constexpr auto ProcessFn(F&& f, P&&... args) {
  return internal::FunCaller<F, P...>(std::forward<F>(f),
                                      std::forward<P>(args)...);
}

}  // namespace api2
}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_API2_CONTRACT_H_
