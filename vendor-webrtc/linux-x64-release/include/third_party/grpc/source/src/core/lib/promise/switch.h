// Copyright 2023 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LIB_PROMISE_SWITCH_H
#define GRPC_SRC_CORE_LIB_PROMISE_SWITCH_H

#include <grpc/support/port_platform.h>

#include <memory>
#include <utility>

#include "src/core/lib/promise/detail/promise_factory.h"
#include "src/core/lib/promise/detail/promise_variant.h"
#include "src/core/lib/promise/if.h"
#include "src/core/util/crash.h"

namespace grpc_core {

// Switch promise combinator
//
// Input :
//
// 1. The first input is the Switch discriminator. Only data types that can be
//    used with a C++ switch statement can be passed as the discriminator.
// 2. Then we can pass zero or more objects of type Case as inputs. The Case
//    object is a promise factory.
// 3. One object of type Default.
//
// Returns :
// Returns a promise that is chosen based on the discriminator.
//
// How it works :
// Given a discriminator, the Switch combinator tries to find a matching Case.
// If a matching Case is found, then the promise corresponding to the matching
// Case is returned.
// If a matching Case is not found, the promise corresponding to the Default is
// returned.
//
// Example :
//
// TEST(SwitchTest, Sample) {
//   auto test_switch = [](int discriminator) {
//     return Switch(discriminator,
//                   Case<1>([] { return 100; }),
//                   Case<2>([] { return 200; }),
//                   Case<3>([]() -> Poll<int> { return Pending{}; }),
//                   Default([] { return -1; }));
//   };
//   EXPECT_EQ(test_switch(1)(), Poll<int>(100));
//   EXPECT_EQ(test_switch(2)(), Poll<int>(200));
//   EXPECT_EQ(test_switch(3)(), Poll<int>(Pending{}));
//   EXPECT_EQ(test_switch(4)(), Poll<int>(-1));
// }
//
// All Case objects and the Default object must have the same Poll<T> return
// type.
//
// The fallthrough mechanism is present in C++ switch statements is NOT present
// in the Switch promise combinator.
//
// Our code currently permits you to create multiple cases for the same
// discriminator value. However this should be avoided as it could lead to bugs.

namespace promise_detail {
template <auto kDiscriminator, typename F>
struct Case {
  using Factory = OncePromiseFactory<void, F>;
  explicit Case(F f) : factory(std::move(f)) {}
  Factory factory;
  static bool Matches(decltype(kDiscriminator) value) {
    return value == kDiscriminator;
  }
};

template <typename F>
struct Default {
  using Factory = OncePromiseFactory<void, F>;
  explicit Default(F f) : factory(std::move(f)) {}
  Factory factory;
};

template <typename Promise, typename D, typename F>
Promise ConstructSwitchPromise(D, Default<F>& def) {
  return def.factory.Make();
}

template <typename Promise, typename D, typename Case, typename... OtherCases>
Promise ConstructSwitchPromise(D discriminator, Case& c, OtherCases&... cs) {
  if (Case::Matches(discriminator)) return c.factory.Make();
  return ConstructSwitchPromise<Promise>(discriminator, cs...);
}

template <typename D, typename... Cases>
auto SwitchImpl(D discriminator, Cases&... cases) {
  using Promise = std::variant<typename Cases::Factory::Promise...>;
  return PromiseVariant<Promise>(
      ConstructSwitchPromise<Promise>(discriminator, cases...));
}

}  // namespace promise_detail

template <auto kDiscriminator, typename PromiseFactory>
auto Case(PromiseFactory f) {
  return promise_detail::Case<kDiscriminator, PromiseFactory>{std::move(f)};
}

template <typename PromiseFactory>
auto Default(PromiseFactory f) {
  return promise_detail::Default<PromiseFactory>{std::move(f)};
}

// TODO(ctiller): consider writing a code-generator like we do for seq/join
// so that this lowers into a C switch statement.
template <typename D, typename... C>
auto Switch(D discriminator, C... cases) {
  return promise_detail::SwitchImpl(discriminator, cases...);
}

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_PROMISE_SWITCH_H
