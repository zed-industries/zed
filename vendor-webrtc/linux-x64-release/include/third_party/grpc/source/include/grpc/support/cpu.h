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

#ifndef GRPC_SUPPORT_CPU_H
#define GRPC_SUPPORT_CPU_H

#include <grpc/support/port_platform.h>

#ifdef __cplusplus
extern "C" {
#endif

/** Interface providing CPU information for currently running system */

/** Return the number of CPU cores on the current system. Will return 0 if
   the information is not available. */
GPRAPI unsigned gpr_cpu_num_cores(void);

/** Return the CPU on which the current thread is executing; N.B. This should
   be considered advisory only - it is possible that the thread is switched
   to a different CPU at any time. Returns a value in range
   [0, gpr_cpu_num_cores() - 1] */
GPRAPI unsigned gpr_cpu_current_cpu(void);

#ifdef __cplusplus
}  // extern "C"
#endif

#endif /* GRPC_SUPPORT_CPU_H */
