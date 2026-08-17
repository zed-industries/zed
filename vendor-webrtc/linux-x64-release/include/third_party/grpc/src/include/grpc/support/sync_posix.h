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

#ifndef GRPC_SUPPORT_SYNC_POSIX_H
#define GRPC_SUPPORT_SYNC_POSIX_H

#include <grpc/support/port_platform.h>
#include <grpc/support/sync_generic.h>
#include <pthread.h>

#ifdef GRPC_ASAN_ENABLED
/* The member |leak_checker| is used to check whether there is a memory leak
 * caused by upper layer logic that's missing the |gpr_xx_destroy| call
 * to the object before freeing it.
 * This issue was reported at https://github.com/grpc/grpc/issues/17563
 * and discussed at https://github.com/grpc/grpc/pull/17586
 */
typedef struct {
  pthread_mutex_t mutex;
  int* leak_checker;
} gpr_mu;

typedef struct {
  pthread_cond_t cond_var;
  int* leak_checker;
} gpr_cv;
#else
typedef pthread_mutex_t gpr_mu;
typedef pthread_cond_t gpr_cv;
#endif
typedef pthread_once_t gpr_once;

#define GPR_ONCE_INIT PTHREAD_ONCE_INIT

#endif /* GRPC_SUPPORT_SYNC_POSIX_H */
