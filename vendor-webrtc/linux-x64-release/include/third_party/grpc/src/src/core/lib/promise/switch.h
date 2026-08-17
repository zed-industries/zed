// Copyright 2025 gRPC authors.
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
template <auto D, typename F>
struct Case {
  using Factory = OncePromiseFactory<void, F>;
  explicit Case(F f) : factory(std::move(f)) {}
  Factory factory;
  static constexpr const decltype(D) kDiscriminator = D;
};

template <typename F>
struct Default {
  using Factory = OncePromiseFactory<void, F>;
  explicit Default(F f) : factory(std::move(f)) {}
  Factory factory;
};
template <typename Promise, typename D, typename FD>
Promise ConstructSwitchPromise(D discriminator, Default<FD>& default_case) {
  switch (discriminator) {
    default:
      return default_case.factory.Make();
  }
}
template <typename Promise, typename D, typename F0, D d0, typename FD>
Promise ConstructSwitchPromise(D discriminator, Case<d0, F0>& case_0,
                               Default<FD>& default_case) {
  switch (discriminator) {
    case Case<d0, F0>::kDiscriminator:
      return case_0.factory.Make();
    default:
      return default_case.factory.Make();
  }
}
template <typename Promise, typename D, typename F0, D d0, typename F1, D d1,
          typename FD>
Promise ConstructSwitchPromise(D discriminator, Case<d0, F0>& case_0,
                               Case<d1, F1>& case_1,
                               Default<FD>& default_case) {
  switch (discriminator) {
    case Case<d0, F0>::kDiscriminator:
      return case_0.factory.Make();
    case Case<d1, F1>::kDiscriminator:
      return case_1.factory.Make();
    default:
      return default_case.factory.Make();
  }
}
template <typename Promise, typename D, typename F0, D d0, typename F1, D d1,
          typename F2, D d2, typename FD>
Promise ConstructSwitchPromise(D discriminator, Case<d0, F0>& case_0,
                               Case<d1, F1>& case_1, Case<d2, F2>& case_2,
                               Default<FD>& default_case) {
  switch (discriminator) {
    case Case<d0, F0>::kDiscriminator:
      return case_0.factory.Make();
    case Case<d1, F1>::kDiscriminator:
      return case_1.factory.Make();
    case Case<d2, F2>::kDiscriminator:
      return case_2.factory.Make();
    default:
      return default_case.factory.Make();
  }
}
template <typename Promise, typename D, typename F0, D d0, typename F1, D d1,
          typename F2, D d2, typename F3, D d3, typename FD>
Promise ConstructSwitchPromise(D discriminator, Case<d0, F0>& case_0,
                               Case<d1, F1>& case_1, Case<d2, F2>& case_2,
                               Case<d3, F3>& case_3,
                               Default<FD>& default_case) {
  switch (discriminator) {
    case Case<d0, F0>::kDiscriminator:
      return case_0.factory.Make();
    case Case<d1, F1>::kDiscriminator:
      return case_1.factory.Make();
    case Case<d2, F2>::kDiscriminator:
      return case_2.factory.Make();
    case Case<d3, F3>::kDiscriminator:
      return case_3.factory.Make();
    default:
      return default_case.factory.Make();
  }
}
template <typename Promise, typename D, typename F0, D d0, typename F1, D d1,
          typename F2, D d2, typename F3, D d3, typename F4, D d4, typename FD>
Promise ConstructSwitchPromise(D discriminator, Case<d0, F0>& case_0,
                               Case<d1, F1>& case_1, Case<d2, F2>& case_2,
                               Case<d3, F3>& case_3, Case<d4, F4>& case_4,
                               Default<FD>& default_case) {
  switch (discriminator) {
    case Case<d0, F0>::kDiscriminator:
      return case_0.factory.Make();
    case Case<d1, F1>::kDiscriminator:
      return case_1.factory.Make();
    case Case<d2, F2>::kDiscriminator:
      return case_2.factory.Make();
    case Case<d3, F3>::kDiscriminator:
      return case_3.factory.Make();
    case Case<d4, F4>::kDiscriminator:
      return case_4.factory.Make();
    default:
      return default_case.factory.Make();
  }
}
template <typename Promise, typename D, typename F0, D d0, typename F1, D d1,
          typename F2, D d2, typename F3, D d3, typename F4, D d4, typename F5,
          D d5, typename FD>
Promise ConstructSwitchPromise(D discriminator, Case<d0, F0>& case_0,
                               Case<d1, F1>& case_1, Case<d2, F2>& case_2,
                               Case<d3, F3>& case_3, Case<d4, F4>& case_4,
                               Case<d5, F5>& case_5,
                               Default<FD>& default_case) {
  switch (discriminator) {
    case Case<d0, F0>::kDiscriminator:
      return case_0.factory.Make();
    case Case<d1, F1>::kDiscriminator:
      return case_1.factory.Make();
    case Case<d2, F2>::kDiscriminator:
      return case_2.factory.Make();
    case Case<d3, F3>::kDiscriminator:
      return case_3.factory.Make();
    case Case<d4, F4>::kDiscriminator:
      return case_4.factory.Make();
    case Case<d5, F5>::kDiscriminator:
      return case_5.factory.Make();
    default:
      return default_case.factory.Make();
  }
}
template <typename Promise, typename D, typename F0, D d0, typename F1, D d1,
          typename F2, D d2, typename F3, D d3, typename F4, D d4, typename F5,
          D d5, typename F6, D d6, typename FD>
Promise ConstructSwitchPromise(D discriminator, Case<d0, F0>& case_0,
                               Case<d1, F1>& case_1, Case<d2, F2>& case_2,
                               Case<d3, F3>& case_3, Case<d4, F4>& case_4,
                               Case<d5, F5>& case_5, Case<d6, F6>& case_6,
                               Default<FD>& default_case) {
  switch (discriminator) {
    case Case<d0, F0>::kDiscriminator:
      return case_0.factory.Make();
    case Case<d1, F1>::kDiscriminator:
      return case_1.factory.Make();
    case Case<d2, F2>::kDiscriminator:
      return case_2.factory.Make();
    case Case<d3, F3>::kDiscriminator:
      return case_3.factory.Make();
    case Case<d4, F4>::kDiscriminator:
      return case_4.factory.Make();
    case Case<d5, F5>::kDiscriminator:
      return case_5.factory.Make();
    case Case<d6, F6>::kDiscriminator:
      return case_6.factory.Make();
    default:
      return default_case.factory.Make();
  }
}
template <typename Promise, typename D, typename F0, D d0, typename F1, D d1,
          typename F2, D d2, typename F3, D d3, typename F4, D d4, typename F5,
          D d5, typename F6, D d6, typename F7, D d7, typename FD>
Promise ConstructSwitchPromise(D discriminator, Case<d0, F0>& case_0,
                               Case<d1, F1>& case_1, Case<d2, F2>& case_2,
                               Case<d3, F3>& case_3, Case<d4, F4>& case_4,
                               Case<d5, F5>& case_5, Case<d6, F6>& case_6,
                               Case<d7, F7>& case_7,
                               Default<FD>& default_case) {
  switch (discriminator) {
    case Case<d0, F0>::kDiscriminator:
      return case_0.factory.Make();
    case Case<d1, F1>::kDiscriminator:
      return case_1.factory.Make();
    case Case<d2, F2>::kDiscriminator:
      return case_2.factory.Make();
    case Case<d3, F3>::kDiscriminator:
      return case_3.factory.Make();
    case Case<d4, F4>::kDiscriminator:
      return case_4.factory.Make();
    case Case<d5, F5>::kDiscriminator:
      return case_5.factory.Make();
    case Case<d6, F6>::kDiscriminator:
      return case_6.factory.Make();
    case Case<d7, F7>::kDiscriminator:
      return case_7.factory.Make();
    default:
      return default_case.factory.Make();
  }
}
template <typename Promise, typename D, typename F0, D d0, typename F1, D d1,
          typename F2, D d2, typename F3, D d3, typename F4, D d4, typename F5,
          D d5, typename F6, D d6, typename F7, D d7, typename F8, D d8,
          typename FD>
Promise ConstructSwitchPromise(D discriminator, Case<d0, F0>& case_0,
                               Case<d1, F1>& case_1, Case<d2, F2>& case_2,
                               Case<d3, F3>& case_3, Case<d4, F4>& case_4,
                               Case<d5, F5>& case_5, Case<d6, F6>& case_6,
                               Case<d7, F7>& case_7, Case<d8, F8>& case_8,
                               Default<FD>& default_case) {
  switch (discriminator) {
    case Case<d0, F0>::kDiscriminator:
      return case_0.factory.Make();
    case Case<d1, F1>::kDiscriminator:
      return case_1.factory.Make();
    case Case<d2, F2>::kDiscriminator:
      return case_2.factory.Make();
    case Case<d3, F3>::kDiscriminator:
      return case_3.factory.Make();
    case Case<d4, F4>::kDiscriminator:
      return case_4.factory.Make();
    case Case<d5, F5>::kDiscriminator:
      return case_5.factory.Make();
    case Case<d6, F6>::kDiscriminator:
      return case_6.factory.Make();
    case Case<d7, F7>::kDiscriminator:
      return case_7.factory.Make();
    case Case<d8, F8>::kDiscriminator:
      return case_8.factory.Make();
    default:
      return default_case.factory.Make();
  }
}
template <typename Promise, typename D, typename F0, D d0, typename F1, D d1,
          typename F2, D d2, typename F3, D d3, typename F4, D d4, typename F5,
          D d5, typename F6, D d6, typename F7, D d7, typename F8, D d8,
          typename F9, D d9, typename FD>
Promise ConstructSwitchPromise(D discriminator, Case<d0, F0>& case_0,
                               Case<d1, F1>& case_1, Case<d2, F2>& case_2,
                               Case<d3, F3>& case_3, Case<d4, F4>& case_4,
                               Case<d5, F5>& case_5, Case<d6, F6>& case_6,
                               Case<d7, F7>& case_7, Case<d8, F8>& case_8,
                               Case<d9, F9>& case_9,
                               Default<FD>& default_case) {
  switch (discriminator) {
    case Case<d0, F0>::kDiscriminator:
      return case_0.factory.Make();
    case Case<d1, F1>::kDiscriminator:
      return case_1.factory.Make();
    case Case<d2, F2>::kDiscriminator:
      return case_2.factory.Make();
    case Case<d3, F3>::kDiscriminator:
      return case_3.factory.Make();
    case Case<d4, F4>::kDiscriminator:
      return case_4.factory.Make();
    case Case<d5, F5>::kDiscriminator:
      return case_5.factory.Make();
    case Case<d6, F6>::kDiscriminator:
      return case_6.factory.Make();
    case Case<d7, F7>::kDiscriminator:
      return case_7.factory.Make();
    case Case<d8, F8>::kDiscriminator:
      return case_8.factory.Make();
    case Case<d9, F9>::kDiscriminator:
      return case_9.factory.Make();
    default:
      return default_case.factory.Make();
  }
}
template <typename Promise, typename D, typename F0, D d0, typename F1, D d1,
          typename F2, D d2, typename F3, D d3, typename F4, D d4, typename F5,
          D d5, typename F6, D d6, typename F7, D d7, typename F8, D d8,
          typename F9, D d9, typename F10, D d10, typename FD>
Promise ConstructSwitchPromise(D discriminator, Case<d0, F0>& case_0,
                               Case<d1, F1>& case_1, Case<d2, F2>& case_2,
                               Case<d3, F3>& case_3, Case<d4, F4>& case_4,
                               Case<d5, F5>& case_5, Case<d6, F6>& case_6,
                               Case<d7, F7>& case_7, Case<d8, F8>& case_8,
                               Case<d9, F9>& case_9, Case<d10, F10>& case_10,
                               Default<FD>& default_case) {
  switch (discriminator) {
    case Case<d0, F0>::kDiscriminator:
      return case_0.factory.Make();
    case Case<d1, F1>::kDiscriminator:
      return case_1.factory.Make();
    case Case<d2, F2>::kDiscriminator:
      return case_2.factory.Make();
    case Case<d3, F3>::kDiscriminator:
      return case_3.factory.Make();
    case Case<d4, F4>::kDiscriminator:
      return case_4.factory.Make();
    case Case<d5, F5>::kDiscriminator:
      return case_5.factory.Make();
    case Case<d6, F6>::kDiscriminator:
      return case_6.factory.Make();
    case Case<d7, F7>::kDiscriminator:
      return case_7.factory.Make();
    case Case<d8, F8>::kDiscriminator:
      return case_8.factory.Make();
    case Case<d9, F9>::kDiscriminator:
      return case_9.factory.Make();
    case Case<d10, F10>::kDiscriminator:
      return case_10.factory.Make();
    default:
      return default_case.factory.Make();
  }
}
template <typename Promise, typename D, typename F0, D d0, typename F1, D d1,
          typename F2, D d2, typename F3, D d3, typename F4, D d4, typename F5,
          D d5, typename F6, D d6, typename F7, D d7, typename F8, D d8,
          typename F9, D d9, typename F10, D d10, typename F11, D d11,
          typename FD>
Promise ConstructSwitchPromise(D discriminator, Case<d0, F0>& case_0,
                               Case<d1, F1>& case_1, Case<d2, F2>& case_2,
                               Case<d3, F3>& case_3, Case<d4, F4>& case_4,
                               Case<d5, F5>& case_5, Case<d6, F6>& case_6,
                               Case<d7, F7>& case_7, Case<d8, F8>& case_8,
                               Case<d9, F9>& case_9, Case<d10, F10>& case_10,
                               Case<d11, F11>& case_11,
                               Default<FD>& default_case) {
  switch (discriminator) {
    case Case<d0, F0>::kDiscriminator:
      return case_0.factory.Make();
    case Case<d1, F1>::kDiscriminator:
      return case_1.factory.Make();
    case Case<d2, F2>::kDiscriminator:
      return case_2.factory.Make();
    case Case<d3, F3>::kDiscriminator:
      return case_3.factory.Make();
    case Case<d4, F4>::kDiscriminator:
      return case_4.factory.Make();
    case Case<d5, F5>::kDiscriminator:
      return case_5.factory.Make();
    case Case<d6, F6>::kDiscriminator:
      return case_6.factory.Make();
    case Case<d7, F7>::kDiscriminator:
      return case_7.factory.Make();
    case Case<d8, F8>::kDiscriminator:
      return case_8.factory.Make();
    case Case<d9, F9>::kDiscriminator:
      return case_9.factory.Make();
    case Case<d10, F10>::kDiscriminator:
      return case_10.factory.Make();
    case Case<d11, F11>::kDiscriminator:
      return case_11.factory.Make();
    default:
      return default_case.factory.Make();
  }
}
template <typename Promise, typename D, typename F0, D d0, typename F1, D d1,
          typename F2, D d2, typename F3, D d3, typename F4, D d4, typename F5,
          D d5, typename F6, D d6, typename F7, D d7, typename F8, D d8,
          typename F9, D d9, typename F10, D d10, typename F11, D d11,
          typename F12, D d12, typename FD>
Promise ConstructSwitchPromise(D discriminator, Case<d0, F0>& case_0,
                               Case<d1, F1>& case_1, Case<d2, F2>& case_2,
                               Case<d3, F3>& case_3, Case<d4, F4>& case_4,
                               Case<d5, F5>& case_5, Case<d6, F6>& case_6,
                               Case<d7, F7>& case_7, Case<d8, F8>& case_8,
                               Case<d9, F9>& case_9, Case<d10, F10>& case_10,
                               Case<d11, F11>& case_11, Case<d12, F12>& case_12,
                               Default<FD>& default_case) {
  switch (discriminator) {
    case Case<d0, F0>::kDiscriminator:
      return case_0.factory.Make();
    case Case<d1, F1>::kDiscriminator:
      return case_1.factory.Make();
    case Case<d2, F2>::kDiscriminator:
      return case_2.factory.Make();
    case Case<d3, F3>::kDiscriminator:
      return case_3.factory.Make();
    case Case<d4, F4>::kDiscriminator:
      return case_4.factory.Make();
    case Case<d5, F5>::kDiscriminator:
      return case_5.factory.Make();
    case Case<d6, F6>::kDiscriminator:
      return case_6.factory.Make();
    case Case<d7, F7>::kDiscriminator:
      return case_7.factory.Make();
    case Case<d8, F8>::kDiscriminator:
      return case_8.factory.Make();
    case Case<d9, F9>::kDiscriminator:
      return case_9.factory.Make();
    case Case<d10, F10>::kDiscriminator:
      return case_10.factory.Make();
    case Case<d11, F11>::kDiscriminator:
      return case_11.factory.Make();
    case Case<d12, F12>::kDiscriminator:
      return case_12.factory.Make();
    default:
      return default_case.factory.Make();
  }
}
template <typename Promise, typename D, typename F0, D d0, typename F1, D d1,
          typename F2, D d2, typename F3, D d3, typename F4, D d4, typename F5,
          D d5, typename F6, D d6, typename F7, D d7, typename F8, D d8,
          typename F9, D d9, typename F10, D d10, typename F11, D d11,
          typename F12, D d12, typename F13, D d13, typename FD>
Promise ConstructSwitchPromise(D discriminator, Case<d0, F0>& case_0,
                               Case<d1, F1>& case_1, Case<d2, F2>& case_2,
                               Case<d3, F3>& case_3, Case<d4, F4>& case_4,
                               Case<d5, F5>& case_5, Case<d6, F6>& case_6,
                               Case<d7, F7>& case_7, Case<d8, F8>& case_8,
                               Case<d9, F9>& case_9, Case<d10, F10>& case_10,
                               Case<d11, F11>& case_11, Case<d12, F12>& case_12,
                               Case<d13, F13>& case_13,
                               Default<FD>& default_case) {
  switch (discriminator) {
    case Case<d0, F0>::kDiscriminator:
      return case_0.factory.Make();
    case Case<d1, F1>::kDiscriminator:
      return case_1.factory.Make();
    case Case<d2, F2>::kDiscriminator:
      return case_2.factory.Make();
    case Case<d3, F3>::kDiscriminator:
      return case_3.factory.Make();
    case Case<d4, F4>::kDiscriminator:
      return case_4.factory.Make();
    case Case<d5, F5>::kDiscriminator:
      return case_5.factory.Make();
    case Case<d6, F6>::kDiscriminator:
      return case_6.factory.Make();
    case Case<d7, F7>::kDiscriminator:
      return case_7.factory.Make();
    case Case<d8, F8>::kDiscriminator:
      return case_8.factory.Make();
    case Case<d9, F9>::kDiscriminator:
      return case_9.factory.Make();
    case Case<d10, F10>::kDiscriminator:
      return case_10.factory.Make();
    case Case<d11, F11>::kDiscriminator:
      return case_11.factory.Make();
    case Case<d12, F12>::kDiscriminator:
      return case_12.factory.Make();
    case Case<d13, F13>::kDiscriminator:
      return case_13.factory.Make();
    default:
      return default_case.factory.Make();
  }
}
template <typename Promise, typename D, typename F0, D d0, typename F1, D d1,
          typename F2, D d2, typename F3, D d3, typename F4, D d4, typename F5,
          D d5, typename F6, D d6, typename F7, D d7, typename F8, D d8,
          typename F9, D d9, typename F10, D d10, typename F11, D d11,
          typename F12, D d12, typename F13, D d13, typename F14, D d14,
          typename FD>
Promise ConstructSwitchPromise(D discriminator, Case<d0, F0>& case_0,
                               Case<d1, F1>& case_1, Case<d2, F2>& case_2,
                               Case<d3, F3>& case_3, Case<d4, F4>& case_4,
                               Case<d5, F5>& case_5, Case<d6, F6>& case_6,
                               Case<d7, F7>& case_7, Case<d8, F8>& case_8,
                               Case<d9, F9>& case_9, Case<d10, F10>& case_10,
                               Case<d11, F11>& case_11, Case<d12, F12>& case_12,
                               Case<d13, F13>& case_13, Case<d14, F14>& case_14,
                               Default<FD>& default_case) {
  switch (discriminator) {
    case Case<d0, F0>::kDiscriminator:
      return case_0.factory.Make();
    case Case<d1, F1>::kDiscriminator:
      return case_1.factory.Make();
    case Case<d2, F2>::kDiscriminator:
      return case_2.factory.Make();
    case Case<d3, F3>::kDiscriminator:
      return case_3.factory.Make();
    case Case<d4, F4>::kDiscriminator:
      return case_4.factory.Make();
    case Case<d5, F5>::kDiscriminator:
      return case_5.factory.Make();
    case Case<d6, F6>::kDiscriminator:
      return case_6.factory.Make();
    case Case<d7, F7>::kDiscriminator:
      return case_7.factory.Make();
    case Case<d8, F8>::kDiscriminator:
      return case_8.factory.Make();
    case Case<d9, F9>::kDiscriminator:
      return case_9.factory.Make();
    case Case<d10, F10>::kDiscriminator:
      return case_10.factory.Make();
    case Case<d11, F11>::kDiscriminator:
      return case_11.factory.Make();
    case Case<d12, F12>::kDiscriminator:
      return case_12.factory.Make();
    case Case<d13, F13>::kDiscriminator:
      return case_13.factory.Make();
    case Case<d14, F14>::kDiscriminator:
      return case_14.factory.Make();
    default:
      return default_case.factory.Make();
  }
}
template <typename Promise, typename D, typename F0, D d0, typename F1, D d1,
          typename F2, D d2, typename F3, D d3, typename F4, D d4, typename F5,
          D d5, typename F6, D d6, typename F7, D d7, typename F8, D d8,
          typename F9, D d9, typename F10, D d10, typename F11, D d11,
          typename F12, D d12, typename F13, D d13, typename F14, D d14,
          typename F15, D d15, typename FD>
Promise ConstructSwitchPromise(D discriminator, Case<d0, F0>& case_0,
                               Case<d1, F1>& case_1, Case<d2, F2>& case_2,
                               Case<d3, F3>& case_3, Case<d4, F4>& case_4,
                               Case<d5, F5>& case_5, Case<d6, F6>& case_6,
                               Case<d7, F7>& case_7, Case<d8, F8>& case_8,
                               Case<d9, F9>& case_9, Case<d10, F10>& case_10,
                               Case<d11, F11>& case_11, Case<d12, F12>& case_12,
                               Case<d13, F13>& case_13, Case<d14, F14>& case_14,
                               Case<d15, F15>& case_15,
                               Default<FD>& default_case) {
  switch (discriminator) {
    case Case<d0, F0>::kDiscriminator:
      return case_0.factory.Make();
    case Case<d1, F1>::kDiscriminator:
      return case_1.factory.Make();
    case Case<d2, F2>::kDiscriminator:
      return case_2.factory.Make();
    case Case<d3, F3>::kDiscriminator:
      return case_3.factory.Make();
    case Case<d4, F4>::kDiscriminator:
      return case_4.factory.Make();
    case Case<d5, F5>::kDiscriminator:
      return case_5.factory.Make();
    case Case<d6, F6>::kDiscriminator:
      return case_6.factory.Make();
    case Case<d7, F7>::kDiscriminator:
      return case_7.factory.Make();
    case Case<d8, F8>::kDiscriminator:
      return case_8.factory.Make();
    case Case<d9, F9>::kDiscriminator:
      return case_9.factory.Make();
    case Case<d10, F10>::kDiscriminator:
      return case_10.factory.Make();
    case Case<d11, F11>::kDiscriminator:
      return case_11.factory.Make();
    case Case<d12, F12>::kDiscriminator:
      return case_12.factory.Make();
    case Case<d13, F13>::kDiscriminator:
      return case_13.factory.Make();
    case Case<d14, F14>::kDiscriminator:
      return case_14.factory.Make();
    case Case<d15, F15>::kDiscriminator:
      return case_15.factory.Make();
    default:
      return default_case.factory.Make();
  }
}
template <typename Promise, typename D, typename F0, D d0, typename F1, D d1,
          typename F2, D d2, typename F3, D d3, typename F4, D d4, typename F5,
          D d5, typename F6, D d6, typename F7, D d7, typename F8, D d8,
          typename F9, D d9, typename F10, D d10, typename F11, D d11,
          typename F12, D d12, typename F13, D d13, typename F14, D d14,
          typename F15, D d15, typename F16, D d16, typename FD>
Promise ConstructSwitchPromise(D discriminator, Case<d0, F0>& case_0,
                               Case<d1, F1>& case_1, Case<d2, F2>& case_2,
                               Case<d3, F3>& case_3, Case<d4, F4>& case_4,
                               Case<d5, F5>& case_5, Case<d6, F6>& case_6,
                               Case<d7, F7>& case_7, Case<d8, F8>& case_8,
                               Case<d9, F9>& case_9, Case<d10, F10>& case_10,
                               Case<d11, F11>& case_11, Case<d12, F12>& case_12,
                               Case<d13, F13>& case_13, Case<d14, F14>& case_14,
                               Case<d15, F15>& case_15, Case<d16, F16>& case_16,
                               Default<FD>& default_case) {
  switch (discriminator) {
    case Case<d0, F0>::kDiscriminator:
      return case_0.factory.Make();
    case Case<d1, F1>::kDiscriminator:
      return case_1.factory.Make();
    case Case<d2, F2>::kDiscriminator:
      return case_2.factory.Make();
    case Case<d3, F3>::kDiscriminator:
      return case_3.factory.Make();
    case Case<d4, F4>::kDiscriminator:
      return case_4.factory.Make();
    case Case<d5, F5>::kDiscriminator:
      return case_5.factory.Make();
    case Case<d6, F6>::kDiscriminator:
      return case_6.factory.Make();
    case Case<d7, F7>::kDiscriminator:
      return case_7.factory.Make();
    case Case<d8, F8>::kDiscriminator:
      return case_8.factory.Make();
    case Case<d9, F9>::kDiscriminator:
      return case_9.factory.Make();
    case Case<d10, F10>::kDiscriminator:
      return case_10.factory.Make();
    case Case<d11, F11>::kDiscriminator:
      return case_11.factory.Make();
    case Case<d12, F12>::kDiscriminator:
      return case_12.factory.Make();
    case Case<d13, F13>::kDiscriminator:
      return case_13.factory.Make();
    case Case<d14, F14>::kDiscriminator:
      return case_14.factory.Make();
    case Case<d15, F15>::kDiscriminator:
      return case_15.factory.Make();
    case Case<d16, F16>::kDiscriminator:
      return case_16.factory.Make();
    default:
      return default_case.factory.Make();
  }
}
template <typename Promise, typename D, typename F0, D d0, typename F1, D d1,
          typename F2, D d2, typename F3, D d3, typename F4, D d4, typename F5,
          D d5, typename F6, D d6, typename F7, D d7, typename F8, D d8,
          typename F9, D d9, typename F10, D d10, typename F11, D d11,
          typename F12, D d12, typename F13, D d13, typename F14, D d14,
          typename F15, D d15, typename F16, D d16, typename F17, D d17,
          typename FD>
Promise ConstructSwitchPromise(
    D discriminator, Case<d0, F0>& case_0, Case<d1, F1>& case_1,
    Case<d2, F2>& case_2, Case<d3, F3>& case_3, Case<d4, F4>& case_4,
    Case<d5, F5>& case_5, Case<d6, F6>& case_6, Case<d7, F7>& case_7,
    Case<d8, F8>& case_8, Case<d9, F9>& case_9, Case<d10, F10>& case_10,
    Case<d11, F11>& case_11, Case<d12, F12>& case_12, Case<d13, F13>& case_13,
    Case<d14, F14>& case_14, Case<d15, F15>& case_15, Case<d16, F16>& case_16,
    Case<d17, F17>& case_17, Default<FD>& default_case) {
  switch (discriminator) {
    case Case<d0, F0>::kDiscriminator:
      return case_0.factory.Make();
    case Case<d1, F1>::kDiscriminator:
      return case_1.factory.Make();
    case Case<d2, F2>::kDiscriminator:
      return case_2.factory.Make();
    case Case<d3, F3>::kDiscriminator:
      return case_3.factory.Make();
    case Case<d4, F4>::kDiscriminator:
      return case_4.factory.Make();
    case Case<d5, F5>::kDiscriminator:
      return case_5.factory.Make();
    case Case<d6, F6>::kDiscriminator:
      return case_6.factory.Make();
    case Case<d7, F7>::kDiscriminator:
      return case_7.factory.Make();
    case Case<d8, F8>::kDiscriminator:
      return case_8.factory.Make();
    case Case<d9, F9>::kDiscriminator:
      return case_9.factory.Make();
    case Case<d10, F10>::kDiscriminator:
      return case_10.factory.Make();
    case Case<d11, F11>::kDiscriminator:
      return case_11.factory.Make();
    case Case<d12, F12>::kDiscriminator:
      return case_12.factory.Make();
    case Case<d13, F13>::kDiscriminator:
      return case_13.factory.Make();
    case Case<d14, F14>::kDiscriminator:
      return case_14.factory.Make();
    case Case<d15, F15>::kDiscriminator:
      return case_15.factory.Make();
    case Case<d16, F16>::kDiscriminator:
      return case_16.factory.Make();
    case Case<d17, F17>::kDiscriminator:
      return case_17.factory.Make();
    default:
      return default_case.factory.Make();
  }
}
template <typename Promise, typename D, typename F0, D d0, typename F1, D d1,
          typename F2, D d2, typename F3, D d3, typename F4, D d4, typename F5,
          D d5, typename F6, D d6, typename F7, D d7, typename F8, D d8,
          typename F9, D d9, typename F10, D d10, typename F11, D d11,
          typename F12, D d12, typename F13, D d13, typename F14, D d14,
          typename F15, D d15, typename F16, D d16, typename F17, D d17,
          typename F18, D d18, typename FD>
Promise ConstructSwitchPromise(D discriminator, Case<d0, F0>& case_0,
                               Case<d1, F1>& case_1, Case<d2, F2>& case_2,
                               Case<d3, F3>& case_3, Case<d4, F4>& case_4,
                               Case<d5, F5>& case_5, Case<d6, F6>& case_6,
                               Case<d7, F7>& case_7, Case<d8, F8>& case_8,
                               Case<d9, F9>& case_9, Case<d10, F10>& case_10,
                               Case<d11, F11>& case_11, Case<d12, F12>& case_12,
                               Case<d13, F13>& case_13, Case<d14, F14>& case_14,
                               Case<d15, F15>& case_15, Case<d16, F16>& case_16,
                               Case<d17, F17>& case_17, Case<d18, F18>& case_18,
                               Default<FD>& default_case) {
  switch (discriminator) {
    case Case<d0, F0>::kDiscriminator:
      return case_0.factory.Make();
    case Case<d1, F1>::kDiscriminator:
      return case_1.factory.Make();
    case Case<d2, F2>::kDiscriminator:
      return case_2.factory.Make();
    case Case<d3, F3>::kDiscriminator:
      return case_3.factory.Make();
    case Case<d4, F4>::kDiscriminator:
      return case_4.factory.Make();
    case Case<d5, F5>::kDiscriminator:
      return case_5.factory.Make();
    case Case<d6, F6>::kDiscriminator:
      return case_6.factory.Make();
    case Case<d7, F7>::kDiscriminator:
      return case_7.factory.Make();
    case Case<d8, F8>::kDiscriminator:
      return case_8.factory.Make();
    case Case<d9, F9>::kDiscriminator:
      return case_9.factory.Make();
    case Case<d10, F10>::kDiscriminator:
      return case_10.factory.Make();
    case Case<d11, F11>::kDiscriminator:
      return case_11.factory.Make();
    case Case<d12, F12>::kDiscriminator:
      return case_12.factory.Make();
    case Case<d13, F13>::kDiscriminator:
      return case_13.factory.Make();
    case Case<d14, F14>::kDiscriminator:
      return case_14.factory.Make();
    case Case<d15, F15>::kDiscriminator:
      return case_15.factory.Make();
    case Case<d16, F16>::kDiscriminator:
      return case_16.factory.Make();
    case Case<d17, F17>::kDiscriminator:
      return case_17.factory.Make();
    case Case<d18, F18>::kDiscriminator:
      return case_18.factory.Make();
    default:
      return default_case.factory.Make();
  }
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

template <typename D, typename... C>
auto Switch(D discriminator, C... cases) {
  return promise_detail::SwitchImpl(discriminator, cases...);
}

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_PROMISE_SWITCH_H
