//
//
// Copyright 2015 gRPC authors.
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
//
//

#ifndef GRPC_SRC_CORE_LIB_IOMGR_POLLING_ENTITY_H
#define GRPC_SRC_CORE_LIB_IOMGR_POLLING_ENTITY_H

#include <grpc/support/port_platform.h>

#include "src/core/lib/iomgr/pollset.h"
#include "src/core/lib/iomgr/pollset_set.h"
#include "src/core/lib/promise/context.h"

typedef enum grpc_pollset_tag {
  GRPC_POLLS_NONE,
  GRPC_POLLS_POLLSET,
  GRPC_POLLS_POLLSET_SET
} grpc_pollset_tag;

// A grpc_polling_entity is a pollset-or-pollset_set container. It allows
// functions that accept a pollset XOR a pollset_set to do so through an
// abstract interface. No ownership is taken.

struct grpc_polling_entity {
  union {
    grpc_pollset* pollset = nullptr;
    grpc_pollset_set* pollset_set;
  } pollent;
  grpc_pollset_tag tag = GRPC_POLLS_NONE;
};

grpc_polling_entity grpc_polling_entity_create_from_pollset_set(
    grpc_pollset_set* pollset_set);
grpc_polling_entity grpc_polling_entity_create_from_pollset(
    grpc_pollset* pollset);

/// If \a pollent contains a pollset, return it. Otherwise, return NULL
grpc_pollset* grpc_polling_entity_pollset(grpc_polling_entity* pollent);

/// If \a pollent contains a pollset_set, return it. Otherwise, return NULL
grpc_pollset_set* grpc_polling_entity_pollset_set(grpc_polling_entity* pollent);

bool grpc_polling_entity_is_empty(const grpc_polling_entity* pollent);

/// Add the pollset or pollset_set in \a pollent to the destination pollset_set
///\a * pss_dst
void grpc_polling_entity_add_to_pollset_set(grpc_polling_entity* pollent,
                                            grpc_pollset_set* pss_dst);

/// Delete the pollset or pollset_set in \a pollent from the destination
/// pollset_set \a * pss_dst
void grpc_polling_entity_del_from_pollset_set(grpc_polling_entity* pollent,
                                              grpc_pollset_set* pss_dst);

std::string grpc_polling_entity_string(grpc_polling_entity* pollent);

namespace grpc_core {
template <>
struct ContextType<grpc_polling_entity> {};
}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_IOMGR_POLLING_ENTITY_H
