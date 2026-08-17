#ifndef MEDIAPIPE_FRAMEWORK_API2_TUPLE_H_
#define MEDIAPIPE_FRAMEWORK_API2_TUPLE_H_

#include <tuple>
#include <type_traits>
#include <utility>

#include "absl/meta/type_traits.h"

// This file contains utilities for working with constexpr tuples.

namespace mediapipe {
namespace api2 {
namespace internal {

// Defines a std::index_sequence with indices for each item of the tuple.
template <class Tuple>
using tuple_index_sequence =
    std::make_index_sequence<std::tuple_size_v<std::decay_t<Tuple>>>;

// Concatenates multiple std::index_sequences.
template <std::size_t... I>
constexpr auto index_sequence_cat(std::index_sequence<I...>)
    -> std::index_sequence<I...> {
  return {};
}

template <std::size_t... I, std::size_t... J, class... Tail>
constexpr auto index_sequence_cat(std::index_sequence<I...>,
                                  std::index_sequence<J...>, Tail... tail) {
  return index_sequence_cat(std::index_sequence<I..., J...>(), tail...);
}

template <template <typename...> class Pred, typename Tuple, std::size_t... I>
constexpr auto filtered_tuple_indices_impl(Tuple&& t,
                                           std::index_sequence<I...>) {
  return index_sequence_cat(
      std::conditional_t<
          Pred<std::tuple_element_t<I, std::decay_t<Tuple>>>::value,
          std::index_sequence<I>, std::index_sequence<>>{}...);
}

// Returns a std::index_sequence with the indices of the tuple items whose
// type satisfied Pred.
template <template <typename...> class Pred, typename Tuple>
constexpr auto filtered_tuple_indices(Tuple&& tuple) {
  return filtered_tuple_indices_impl<Pred>(tuple,
                                           tuple_index_sequence<Tuple>());
}

// Convenience type to pass any type as a value.
template <typename T>
struct Wrap {
  using type = T;
};

template <class F, typename Tuple, std::size_t... I>
constexpr auto filtered_tuple_indices_impl(Tuple&& t,
                                           std::index_sequence<I...>) {
  return index_sequence_cat(
      std::conditional_t<
          F{}(Wrap<std::tuple_element_t<I, std::decay_t<Tuple>>>{}),
          std::index_sequence<I>, std::index_sequence<>>{}...);
}

// Returns a std::index_sequence with the indices of the tuple items for which
// F{}(Wrap<item_type>) returns true.
template <class F, typename Tuple>
constexpr auto filtered_tuple_indices(Tuple&& tuple) {
  return filtered_tuple_indices_impl<F>(std::forward<Tuple>(tuple),
                                        tuple_index_sequence<Tuple>());
}

// Returns a tuple of references to the tuple items with the specified indices.
template <typename Tuple, std::size_t... I>
constexpr auto select_tuple_indices(Tuple&& tuple, std::index_sequence<I...>) {
  return std::forward_as_tuple(std::get<I>(std::forward<Tuple>(tuple))...);
}

// Returns a tuple of references to the tuple items whose types satisfy Pred.
template <template <typename...> class Pred, typename Tuple>
constexpr auto filter_tuple(Tuple&& t) {
  return select_tuple_indices(std::forward<Tuple>(t),
                              filtered_tuple_indices<Pred>(t));
}

// Returns a tuple of references to the tuple items for which
// F{}(Wrap<item_type>) returns true.
template <typename F, typename Tuple>
constexpr auto filter_tuple(Tuple&& t) {
  return select_tuple_indices(
      std::forward<Tuple>(t),
      filtered_tuple_indices<F>(std::forward<Tuple>(t)));
}

// TODO: ensure only one of these is enabled?
template <class F, class T, class I>
constexpr auto call_with_optional_index(F&& f, T&& t, I i)
    -> absl::void_t<decltype(f(std::forward<T>(t), i))> {
  return f(std::forward<T>(t), i);
}

template <class F, class T, class I>
constexpr auto call_with_optional_index(F&& f, T&& t, I i)
    -> absl::void_t<decltype(f(std::forward<T>(t)))> {
  return f(std::forward<T>(t));
}

template <class F, class Tuple, std::size_t... I>
constexpr void tuple_for_each_impl(F&& f, Tuple&& tuple,
                                   std::index_sequence<I...>) {
  int unpack[] = {
      (call_with_optional_index(std::forward<F>(f),
                                std::get<I>(std::forward<Tuple>(tuple)),
                                std::integral_constant<std::size_t, I>{}),
       0)...};
  (void)unpack;
}

// Invokes f for each item in tuple.
// If f takes one argument, it will be called as f(item).
// If f takes two arguments, it will be called as
//   f(item, std::integral_constant<std::size_t, index>{}).
template <class F, class Tuple>
constexpr void tuple_for_each(F&& f, Tuple&& tuple) {
  return tuple_for_each_impl(std::forward<F>(f), std::forward<Tuple>(tuple),
                             tuple_index_sequence<Tuple>());
}

template <class F, class Tuple, std::size_t... I>
constexpr auto map_tuple_impl(F&& f, Tuple&& tuple, std::index_sequence<I...>) {
  return std::make_tuple(f(std::get<I>(std::forward<Tuple>(tuple)))...);
}

// Returns a tuple where each item is the result of calling f on the
// corresponding item of the provided tuple.
template <class F, class Tuple>
constexpr auto map_tuple(F&& f, Tuple&& tuple) {
  return map_tuple_impl(std::forward<F>(f), std::forward<Tuple>(tuple),
                        tuple_index_sequence<Tuple>());
}

template <class F, class Tuple, std::size_t... I>
constexpr auto tuple_apply_impl(F&& f, Tuple&& tuple,
                                std::index_sequence<I...>) {
  return f(std::get<I>(std::forward<Tuple>(tuple))...);
}

// Invokes f passing the tuple's items as arguments.
template <class F, class Tuple>
constexpr auto tuple_apply(F&& f, Tuple&& tuple) {
  return tuple_apply_impl(std::forward<F>(f), std::forward<Tuple>(tuple),
                          tuple_index_sequence<Tuple>());
}

// Returns the index [0, tuple_size) of the first item for which f returns true,
// or tuple_size if no such item is found.
template <class F, class Tuple, std::size_t i = 0>
constexpr std::enable_if_t<i == std::tuple_size_v<std::decay_t<Tuple>>,
                           std::size_t>
tuple_find(F&& f, Tuple&& tuple) {
  return i;
}

template <class F, class Tuple, std::size_t i = 0>
constexpr std::enable_if_t<i != std::tuple_size_v<std::decay_t<Tuple>>,
                           std::size_t>
tuple_find(F&& f, Tuple&& tuple) {
  if (f(std::get<i>(std::forward<Tuple>(tuple)))) {
    return i;
  }
  return tuple_find<F, Tuple, i + 1>(std::forward<F>(f),
                                     std::forward<Tuple>(tuple));
}

template <class Tuple>
constexpr auto flatten_tuple(Tuple&& tuple) {
  return tuple_apply([](auto&&... args) { return std::tuple_cat(args...); },
                     tuple);
}

}  // namespace internal
}  // namespace api2
}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_API2_TUPLE_H_
