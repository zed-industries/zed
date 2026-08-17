// Copyright 2022 gRPC authors.
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

#ifndef GRPC_SRC_CORE_UTIL_PACKED_TABLE_H
#define GRPC_SRC_CORE_UTIL_PACKED_TABLE_H

#include <grpc/support/port_platform.h>

#include "src/core/util/sorted_pack.h"
#include "src/core/util/table.h"

namespace grpc_core {

namespace packed_table_detail {
template <typename A, typename B>
struct Cmp {
  static constexpr bool kValue = alignof(A) > alignof(B) ||
                                 (alignof(A) == alignof(B) &&
                                  sizeof(A) > sizeof(B));
};
};  // namespace packed_table_detail

template <typename... T>
using PackedTable =
    typename WithSortedPack<Table, packed_table_detail::Cmp, T...>::Type;

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_UTIL_PACKED_TABLE_H
