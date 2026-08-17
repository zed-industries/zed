/*
 *
 * Copyright 2015 gRPC authors.
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

#ifndef GRPC_SUPPORT_SYNC_GENERIC_H
#define GRPC_SUPPORT_SYNC_GENERIC_H

/* Generic type definitions for gpr_sync. */

#include <grpc/support/atm.h>
#include <grpc/support/port_platform.h>

/* gpr_event */
typedef struct {
  gpr_atm state;
} gpr_event;

#define GPR_EVENT_INIT {0}

/* gpr_refcount */
typedef struct {
  gpr_atm count;
} gpr_refcount;

/* gpr_stats_counter */
typedef struct {
  gpr_atm value;
} gpr_stats_counter;

#define GPR_STATS_INIT {0}

#endif /* GRPC_SUPPORT_SYNC_GENERIC_H */
