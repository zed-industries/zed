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

#ifndef GRPC_SUPPORT_SYNC_WINDOWS_H
#define GRPC_SUPPORT_SYNC_WINDOWS_H

#include <grpc/support/port_platform.h>

#ifdef GPR_WINDOWS

#include <grpc/support/sync_generic.h>

typedef struct {
  CRITICAL_SECTION cs; /* Not an SRWLock until Vista is unsupported */
  int locked;
} gpr_mu;

typedef CONDITION_VARIABLE gpr_cv;

typedef INIT_ONCE gpr_once;
#define GPR_ONCE_INIT INIT_ONCE_STATIC_INIT

#endif /* GPR_WINDOWS */

#endif /* GRPC_SUPPORT_SYNC_WINDOWS_H */
