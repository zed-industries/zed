// Copyright 2021 gRPC authors.
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

#ifndef GRPC_SRC_CORE_LIB_PROMISE_DETAIL_BASIC_SEQ_H
#define GRPC_SRC_CORE_LIB_PROMISE_DETAIL_BASIC_SEQ_H

#include <grpc/support/port_platform.h>

#include "src/core/lib/promise/detail/promise_factory.h"
#include "src/core/lib/promise/poll.h"
#include "src/core/util/construct_destruct.h"

namespace grpc_core {
namespace promise_detail {

template <typename FactoryFn>
auto BindFactoryFnArgs(FactoryFn fn) {
  return [fn = std::move(fn)](auto x) mutable {
    return fn(std::get<0>(x), std::move(std::get<1>(x)));
  };
}

// Models a sequence of unknown size
// At each element, the accumulator A and the current value V is passed to some
// function of type IterTraits::Factory as f(V, IterTraits::Argument); f is
// expected to return a promise that resolves to IterTraits::Wrapped.
template <template <typename> class Traits, typename Iter, typename FactoryFn,
          typename Argument>
class BasicSeqIter {
 private:
  using BoundFactoryFn = decltype(BindFactoryFnArgs(std::declval<FactoryFn>()));
  using TplArg = std::tuple<decltype((*std::declval<Iter>())), Argument>;
  using Factory = RepeatedPromiseFactory<TplArg, BoundFactoryFn>;
  using State = typename Factory::Promise;
  using StateResult = typename State::Result;

 public:
  BasicSeqIter(Iter begin, Iter end, FactoryFn f, Argument arg)
      : cur_(begin), end_(end), f_(BindFactoryFnArgs(std::move(f))) {
    if (cur_ == end_) {
      Construct(&result_, std::move(arg));
    } else {
      Construct(&state_, f_.Make(TplArg(*cur_, std::move(arg))));
    }
  }

  ~BasicSeqIter() {
    if (cur_ == end_) {
      Destruct(&result_);
    } else {
      Destruct(&state_);
    }
  }

  BasicSeqIter(const BasicSeqIter& other) = delete;
  BasicSeqIter& operator=(const BasicSeqIter&) = delete;

  BasicSeqIter(BasicSeqIter&& other) noexcept
      : cur_(other.cur_), end_(other.end_), f_(std::move(other.f_)) {
    if (cur_ == end_) {
      Construct(&result_, std::move(other.result_));
    } else {
      Construct(&state_, std::move(other.state_));
    }
  }
  BasicSeqIter& operator=(BasicSeqIter&& other) noexcept {
    cur_ = other.cur_;
    end_ = other.end_;
    if (cur_ == end_) {
      Construct(&result_, std::move(other.result_));
    } else {
      Construct(&state_, std::move(other.state_));
    }
    return *this;
  }

  Poll<StateResult> operator()() {
    if (cur_ == end_) {
      return std::move(result_);
    }
    return PollNonEmpty();
  }

 private:
  Poll<StateResult> PollNonEmpty() {
    Poll<StateResult> r = state_();
    if (r.pending()) return r;
    using Tr = Traits<StateResult>;
    return Tr::template CheckResultAndRunNext<StateResult>(
        std::move(r.value()), [this](auto arg) -> Poll<StateResult> {
          auto next = cur_;
          ++next;
          if (next == end_) {
            return std::move(arg);
          }
          cur_ = next;
          state_.~State();
          struct WrapperFactory {
            BasicSeqIter* owner;
            State Make(typename Tr::UnwrappedType r) {
              return owner->f_.Make(TplArg(*owner->cur_, std::move(r)));
            }
          };
          WrapperFactory wrapper_factory{this};
          Construct(&state_, Tr::CallFactory(&wrapper_factory, std::move(arg)));
          return PollNonEmpty();
        });
  }

  Iter cur_;
  const Iter end_;
  GPR_NO_UNIQUE_ADDRESS Factory f_;
  union {
    GPR_NO_UNIQUE_ADDRESS State state_;
    GPR_NO_UNIQUE_ADDRESS Argument result_;
  };
};

}  // namespace promise_detail
}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_PROMISE_DETAIL_BASIC_SEQ_H
