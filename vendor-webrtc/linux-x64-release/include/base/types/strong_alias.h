// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TYPES_STRONG_ALIAS_H_
#define BASE_TYPES_STRONG_ALIAS_H_

#include <compare>
#include <functional>
#include <ostream>
#include <type_traits>
#include <utility>

#include "base/trace_event/base_tracing_forward.h"
#include "base/types/supports_ostream_operator.h"

namespace base {

// A type-safe alternative for a typedef or a 'using' directive.
//
// C++ currently does not support type-safe typedefs, despite multiple proposals
// (ex. http://www.open-std.org/jtc1/sc22/wg21/docs/papers/2013/n3515.pdf). The
// next best thing is to try and emulate them in library code.
//
// The motivation is to disallow several classes of errors:
//
// using Orange = int;
// using Apple = int;
// Apple apple(2);
// Orange orange = apple;  // Orange should not be able to become an Apple.
// Orange x = orange + apple;  // Shouldn't add Oranges and Apples.
// if (orange > apple);  // Shouldn't compare Apples to Oranges.
// void foo(Orange);
// void foo(Apple);  // Redefinition.
// etc.
//
// StrongAlias may instead be used as follows:
//
// using Orange = StrongAlias<class OrangeTag, int>;
// using Apple = StrongAlias<class AppleTag, int>;
// using Banana = StrongAlias<class BananaTag, std::string>;
// Apple apple(2);
// Banana banana("Hello");
// Orange orange = apple;  // Does not compile.
// Orange other_orange = orange;  // Compiles, types match.
// Orange x = orange + apple;  // Does not compile.
// Orange y = Orange(orange.value() + apple.value());  // Compiles.
// Orange z = Orange(banana->size() + *other_orange);  // Compiles.
// if (orange > apple);  // Does not compile.
// if (orange > other_orange);  // Compiles.
// void foo(Orange);
// void foo(Apple);  // Compiles into separate overload.
//
// StrongAlias is a zero-cost abstraction, it's compiled away.
//
// TagType is an empty tag class (also called "phantom type") that only serves
// the type system to differentiate between different instantiations of the
// template.
// UnderlyingType may be almost any value type. Note that some methods of the
// StrongAlias may be unavailable (ie. produce elaborate compilation errors when
// used) if UnderlyingType doesn't support them.
//
// StrongAlias only directly exposes comparison operators (for convenient use in
// ordered containers). It's impossible, without reflection, to expose all
// methods of the UnderlyingType in StrongAlias's interface. It's also
// potentially unwanted (ex. you don't want to be able to add two StrongAliases
// that represent socket handles). A getter and dereference operators are
// provided in case you need to access the UnderlyingType.
//
// See also
// - //styleguide/c++/blink-c++.md which provides recommendation and examples of
//   using StrongAlias<Tag, bool> instead of a bare bool.
// - IdType<...> which provides helpers for specializing StrongAlias to be
//   used as an id.
// - TokenType<...> which provides helpers for specializing StrongAlias to be
//   used as a wrapper of base::UnguessableToken.
template <typename TagType, typename UnderlyingType>
class StrongAlias {
 public:
  using underlying_type = UnderlyingType;

  StrongAlias() = default;
  constexpr explicit StrongAlias(const UnderlyingType& v) : value_(v) {}
  constexpr explicit StrongAlias(UnderlyingType&& v) noexcept
      : value_(std::move(v)) {}

  constexpr UnderlyingType* operator->() { return &value_; }
  constexpr const UnderlyingType* operator->() const { return &value_; }

  constexpr UnderlyingType& operator*() & { return value_; }
  constexpr const UnderlyingType& operator*() const& { return value_; }
  constexpr UnderlyingType&& operator*() && { return std::move(value_); }
  constexpr const UnderlyingType&& operator*() const&& {
    return std::move(value_);
  }

  constexpr UnderlyingType& value() & { return value_; }
  constexpr const UnderlyingType& value() const& { return value_; }
  constexpr UnderlyingType&& value() && { return std::move(value_); }
  constexpr const UnderlyingType&& value() const&& { return std::move(value_); }

  constexpr explicit operator const UnderlyingType&() const& { return value_; }

  // Comparison operators that default to the behavior of `UnderlyingType`.
  // Note that if you wish to compare `StrongAlias<UnderlyingType>`, e.g.,
  // by using `operator<` in a `std::set`, then `UnderlyingType` must
  // implement `operator<=>`. If you cannot modify `UnderlyingType` (e.g.,
  // because it is from an external library), then a work-around is to create a
  // thin wrapper `W` around it, define `operator<=>` for the wrapper and create
  // a `StrongAlias<W>`.
  friend auto operator<=>(const StrongAlias& lhs,
                          const StrongAlias& rhs) = default;
  friend bool operator==(const StrongAlias& lhs,
                         const StrongAlias& rhs) = default;

  // If UnderlyingType can be serialised into trace, its alias is also
  // serialisable.
  template <class U = UnderlyingType>
  typename perfetto::check_traced_value_support<U>::type WriteIntoTrace(
      perfetto::TracedValue&& context) const {
    perfetto::WriteIntoTracedValue(std::move(context), value_);
  }

 protected:
  UnderlyingType value_;
};

// Stream operator for convenience, streams the UnderlyingType.
template <typename TagType, typename UnderlyingType>
  requires(internal::SupportsOstreamOperator<UnderlyingType>)
std::ostream& operator<<(std::ostream& stream,
                         const StrongAlias<TagType, UnderlyingType>& alias) {
  return stream << alias.value();
}

}  // namespace base

template <typename TagType, typename UnderlyingType>
struct std::hash<base::StrongAlias<TagType, UnderlyingType>> {
  size_t operator()(
      const base::StrongAlias<TagType, UnderlyingType>& id) const {
    return std::hash<UnderlyingType>()(id.value());
  }
};

#endif  // BASE_TYPES_STRONG_ALIAS_H_
