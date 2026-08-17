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

#ifndef GRPC_RB_H_
#define GRPC_RB_H_

#include <ruby/ruby.h>

#include <grpc/support/time.h>
#include <stdlib.h>

/* grpc_rb_mGrpcCore is the module containing the ruby wrapper GRPC classes. */
extern VALUE grpc_rb_mGrpcCore;

/* grpc_rb_sNewServerRpc is the struct that holds new server rpc details. */
extern VALUE grpc_rb_sNewServerRpc;

/* grpc_rb_sStruct is the struct that holds status details. */
extern VALUE grpc_rb_sStatus;

/* sym_code is the symbol for the code attribute of grpc_rb_sStatus. */
extern VALUE sym_code;

/* sym_details is the symbol for the details attribute of grpc_rb_sStatus. */
extern VALUE sym_details;

/* sym_metadata is the symbol for the metadata attribute of grpc_rb_sStatus. */
extern VALUE sym_metadata;

/* GC_NOT_MARKED is used in calls to Data_Wrap_Struct to indicate that the
   wrapped struct does not need to participate in ruby gc. */
#define GRPC_RB_GC_NOT_MARKED (RUBY_DATA_FUNC)(NULL)

/* GC_DONT_FREED is used in calls to Data_Wrap_Struct to indicate that the
   wrapped struct should not be freed the wrapped ruby object is released by
   the garbage collector. */
#define GRPC_RB_GC_DONT_FREE (RUBY_DATA_FUNC)(NULL)

/* GRPC_RB_MEMSIZE_UNAVAILABLE is used in rb_data_type_t to indicate that the
 * number of bytes used by the wrapped struct is not available. */
#define GRPC_RB_MEMSIZE_UNAVAILABLE (size_t (*)(const void*))(NULL)

/* A ruby object alloc func that fails by raising an exception. */
VALUE grpc_rb_cannot_alloc(VALUE cls);

/* A ruby object init func that fails by raising an exception. */
VALUE grpc_rb_cannot_init(VALUE self);

/* A ruby object clone init func that fails by raising an exception. */
VALUE grpc_rb_cannot_init_copy(VALUE copy, VALUE self);

/* grpc_rb_time_timeval creates a gpr_timespec from a ruby time object. */
gpr_timespec grpc_rb_time_timeval(VALUE time, int interval);

void grpc_ruby_fork_guard();

/* To be called once and only once before entering code section that is
 * definitely not fork-safe. Used in conjunction with GRPC.prefork
 * to catch for-unsafe processes and raise errors. */
void grpc_rb_fork_unsafe_begin();

/* To be called once and only once after each grpc_rb_fork_unsafe_begin*/
void grpc_rb_fork_unsafe_end();

void grpc_ruby_init();

#define GRPC_RUBY_ASSERT(x)                                       \
  if (!(x)) {                                                     \
    fprintf(stderr, "%s:%d assert failed\n", __FILE__, __LINE__); \
    abort();                                                      \
  }

#endif /* GRPC_RB_H_ */
