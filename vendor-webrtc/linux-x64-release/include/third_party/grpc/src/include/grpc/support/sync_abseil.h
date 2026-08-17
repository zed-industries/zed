/*
 *
 * Copyright 2020 gRPC authors.
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

#ifndef GRPC_SUPPORT_SYNC_ABSEIL_H
#define GRPC_SUPPORT_SYNC_ABSEIL_H

#include <grpc/support/port_platform.h>
#include <grpc/support/sync_generic.h>

#ifdef GPR_ABSEIL_SYNC

typedef intptr_t gpr_mu;
typedef intptr_t gpr_cv;
typedef int32_t gpr_once;

#define GPR_ONCE_INIT 0

#endif

#endif /* GRPC_SUPPORT_SYNC_ABSEIL_H */
