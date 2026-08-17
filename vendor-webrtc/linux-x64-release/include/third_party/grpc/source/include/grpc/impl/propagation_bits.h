/*
 *
 * Copyright 2016 gRPC authors.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */

#ifndef GRPC_IMPL_PROPAGATION_BITS_H
#define GRPC_IMPL_PROPAGATION_BITS_H

// IWYU pragma: private

#include <grpc/support/port_platform.h>

#ifdef __cplusplus
extern "C" {
#endif

/** Propagation bits: this can be bitwise or-ed to form propagation_mask for
 * grpc_call */
/** Propagate deadline */
#define GRPC_PROPAGATE_DEADLINE ((uint32_t)1)
/** Propagate census context */
#define GRPC_PROPAGATE_CENSUS_STATS_CONTEXT ((uint32_t)2)
#define GRPC_PROPAGATE_CENSUS_TRACING_CONTEXT ((uint32_t)4)
/** Propagate cancellation */
#define GRPC_PROPAGATE_CANCELLATION ((uint32_t)8)

/** Default propagation mask: clients of the core API are encouraged to encode
   deltas from this in their implementations... ie write:
   GRPC_PROPAGATE_DEFAULTS & ~GRPC_PROPAGATE_DEADLINE to disable deadline
   propagation. Doing so gives flexibility in the future to define new
   propagation types that are default inherited or not. */
#define GRPC_PROPAGATE_DEFAULTS                                                \
  ((uint32_t)((                                                                \
      0xffff | GRPC_PROPAGATE_DEADLINE | GRPC_PROPAGATE_CENSUS_STATS_CONTEXT | \
      GRPC_PROPAGATE_CENSUS_TRACING_CONTEXT | GRPC_PROPAGATE_CANCELLATION)))

#ifdef __cplusplus
}
#endif

#endif /* GRPC_IMPL_PROPAGATION_BITS_H */
