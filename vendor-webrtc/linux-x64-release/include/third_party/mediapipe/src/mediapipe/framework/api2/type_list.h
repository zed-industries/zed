#ifndef MEDIAPIPE_FRAMEWORK_API2_TYPE_LIST_H_
#define MEDIAPIPE_FRAMEWORK_API2_TYPE_LIST_H_

#include <string>
#include <type_traits>
#include <utility>

namespace mediapipe {
namespace api2 {
namespace types {

// A list of types. This allows us to store a template parameter pack.
template <typename... Args>
struct List {};

// Concatenate two lists.
template <typename... As, typename... Bs>
auto concat(List<As...>, List<Bs...>) -> List<As..., Bs...> {
  return {};
}

// Filter a list using a predicate.
template <template <typename> class Pred, typename... Args>
auto filter(List<Args...>) -> List<Args...> {
  return {};
}

template <template <typename> class Pred, typename Head, typename... Tail>
auto filter(List<Head, Tail...>) -> decltype(concat(
    typename std::conditional<Pred<Head>::value, List<Head>, List<>>::type{},
    filter<Pred>(List<Tail...>{}))) {
  return {};
}

template <typename Pred>
auto filter(Pred, List<>) -> List<> {
  return {};
}

template <typename Pred, typename Head, typename... Tail>
auto filter(Pred pred, List<Head, Tail...>) -> decltype(concat(
    typename std::conditional<pred(Head{}), List<Head>, List<>>::type{},
    filter(pred, List<Tail...>{}))) {
  return {};
}

// Invoke a template using a list's types as parameters.
template <template <typename...> class T, typename... Args>
auto apply(List<Args...>) -> T<Args...> {
  return {};
}

// Wraps a single type. The wrapper can always be instantiated as a value,
// even if T cannot.
template <typename T>
struct Wrap {
  using type = T;
};

// Find first match for a predicate.
template <template <typename> class Pred, typename... Args>
auto find(List<Args...>) -> Wrap<void> {
  return {};
}

template <template <typename> class Pred, typename Head, typename... Tail>
auto find(List<Head, Tail...>) ->
    typename std::conditional<Pred<Head>::value, Wrap<Head>,
                              decltype(find<Pred>(List<Tail...>{}))>::type {
  return {};
}

template <class Pred, typename... Args>
auto find(Pred, List<Args...>) -> Wrap<void> {
  return {};
}

template <class Pred, typename Head, typename... Tail>
auto find(Pred pred, List<Head, Tail...>) ->
    typename std::conditional<pred(Head{}), Wrap<Head>,
                              decltype(find(pred, List<Tail...>{}))>::type {
  return {};
}

// Apply a function to each item in a list.
template <template <typename> class Fun, typename... Items>
auto map(List<Items...>) -> List<typename Fun<Items>::type...> {
  return {};
}

// Get the list's head.
template <typename... Args>
constexpr auto head(List<Args...>) -> Wrap<void> {
  return {};
}

template <typename H, typename... T>
constexpr auto head(List<H, T...>) -> Wrap<H> {
  return {};
}

// Get the list's length.
template <typename... Args>
constexpr std::size_t length(List<Args...>) {
  return 0;
}

template <typename H, typename... T>
constexpr std::size_t length(List<H, T...>) {
  return length(List<T...>{}) + 1;
}

// Add indices.
template <std::size_t I, typename T>
struct IndexedType {
  static constexpr std::size_t kIndex = I;
  using type = T;
};

template <typename... Args, std::size_t... Is>
auto enumerate_impl(List<Args...>, std::index_sequence<Is...>)
    -> List<IndexedType<Is, Args>...> {
  return {};
}

template <typename... Args>
auto enumerate(List<Args...> a)
    -> decltype(enumerate_impl(a, std::index_sequence_for<Args...>{})) {
  return {};
}

}  // namespace types
}  // namespace api2
}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_API2_TYPE_LIST_H_
