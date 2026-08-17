#ifndef MEDIAPIPE_FRAMEWORK_API2_TAG_H_
#define MEDIAPIPE_FRAMEWORK_API2_TAG_H_

#include <utility>

#include "mediapipe/framework/api2/const_str.h"

namespace mediapipe {
namespace api2 {

// This template is used to define a separate type for each tag.
// This makes it possible to obtain results of different types depending on
// the tag. See MPP_TAG below for usage examples.
template <char... C>
struct Tag {
  static constexpr char const kChars[sizeof...(C) + 1] = {C..., '\0'};
  static constexpr const_str const kStr{kChars};
  static const std::string str() {
    return std::string(kStr.data(), kStr.len());
  }

  template <char... Q>
  constexpr bool operator==(const Tag<Q...>& other) const {
    return kStr == other.kStr;
  }
  template <char... Q>
  constexpr bool operator!=(const Tag<Q...>& other) const {
    return !(*this == other);
  }
};

template <char... C>
constexpr bool is_tag(Tag<C...>) {
  return true;
}

template <typename A>
constexpr bool is_tag(A) {
  return false;
}

namespace internal {

template <typename S, std::size_t... I>
constexpr auto tag_build_impl(S, std::index_sequence<I...>)
    -> Tag<S().tag[I]...> {
  return {};
}

template <typename S>
constexpr auto tag_build(S) {
  return tag_build_impl(S(), std::make_index_sequence<S().tag.len()>{});
}

}  // namespace internal

// Use this to create typed tag objects.
// For example:
//   auto kFOO = MPP_TAG(FOO);
//   auto kBAR = MPP_TAG(BAR);
#define MPP_TAG(s)                                      \
  ([] {                                                 \
    struct S {                                          \
      const ::mediapipe::api2::const_str tag{s};        \
    };                                                  \
    return ::mediapipe::api2::internal::tag_build(S()); \
  }())

}  // namespace api2
}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_API2_TAG_H_
