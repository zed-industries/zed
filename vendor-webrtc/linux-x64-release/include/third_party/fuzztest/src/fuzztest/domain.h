// Copyright 2022 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// IWYU pragma: private, include "fuzztest/fuzztest.h"
// IWYU pragma: friend fuzztest/.*

#ifndef FUZZTEST_FUZZTEST_DOMAIN_H_
#define FUZZTEST_FUZZTEST_DOMAIN_H_

#include "./fuzztest/domain_core.h"  // IWYU pragma: export
#include "./fuzztest/internal/domains/in_regexp_impl.h"
#include "./fuzztest/internal/domains/protobuf_domain_impl.h"

namespace fuzztest {

// This namespace is here only as a way to disable ADL (argument-dependent
// lookup). Names should be used from the fuzztest:: namespace.
namespace internal_no_adl {

// InRegexp(regexp) represents strings that are sentences of a given regular
// expression `regexp`. The regular expression can use any syntax accepted by
// RE2 (https://github.com/google/re2/wiki/Syntax).
//
// Example usage:
//
//   InRegexp("[0-9]{4}-[0-9]{2}-[0-9]{2}")  // Date-like strings.
//
inline auto InRegexp(std::string_view regex) {
  return internal::InRegexpImpl(regex);
}

// ProtobufOf(std::function<const Message*()> get_prototype) creates a
// unique_ptr<Message> domain for the protobuf prototype (the default protobuf
// message) returned by `get_prototype()`.
//
// REQUIRES: `get_prototype` never returns nullptr.
//
// Example usage:
//
//   const Message* GetPrototypeMessage() {
//     const std::string name = GetPrototypeNameFromFlags();
//     const Descriptor* descriptor =
//         DescriptorPool::generated_pool()->FindMessageTypeByName(name);
//     return MessageFactory::generated_factory()->GetPrototype(descriptor);
//   }
//
//   ProtobufOf(GetPrototypeMessage)
template <typename PrototypeMessageProvider,
          typename T = std::remove_cv_t<std::remove_pointer_t<
              decltype(std::declval<PrototypeMessageProvider>()())>>>
auto ProtobufOf(PrototypeMessageProvider get_prototype) {
  constexpr bool kIsMessageClass = !std::is_copy_constructible_v<T>;
  if constexpr (kIsMessageClass) {
    return internal::ProtobufDomainUntypedImpl<T>(
        fuzztest::internal::PrototypePtr<T>(get_prototype),
        /*use_lazy_initialization=*/false);
  } else {  // T is derived class of Message
    using Message = typename T::Message;
    return internal::ProtobufDomainUntypedImpl<Message>(
        fuzztest::internal::PrototypePtr<Message>(get_prototype),
        /*use_lazy_initialization=*/false);
  }
}

}  // namespace internal_no_adl

// Inject the names from internal_no_adl into fuzztest, without allowing for
// ADL. Note that an `inline` namespace would not have this effect (ie it would
// still allow ADL to trigger).
using namespace internal_no_adl;  // NOLINT

}  // namespace fuzztest

#endif  // FUZZTEST_FUZZTEST_DOMAIN_H_
