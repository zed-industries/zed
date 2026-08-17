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

#ifndef GRPC_RB_CALL_H_
#define GRPC_RB_CALL_H_

#include <ruby/ruby.h>

#include <grpc/grpc.h>

extern const rb_data_type_t grpc_rb_md_ary_data_type;

extern VALUE grpc_rb_cMdAry;

/* Gets the wrapped call from a VALUE. */
grpc_call* grpc_rb_get_wrapped_call(VALUE v);

/* Gets the VALUE corresponding to given grpc_call. */
VALUE grpc_rb_wrap_call(grpc_call* c, grpc_completion_queue* q);

/* Provides the details of an call error */
const char* grpc_call_error_detail_of(grpc_call_error err);

/* Converts a metadata array to a hash. */
VALUE grpc_rb_md_ary_to_h(grpc_metadata_array* md_ary);

/* grpc_rb_md_ary_convert converts a ruby metadata hash into
   a grpc_metadata_array.
*/
void grpc_rb_md_ary_convert(VALUE md_ary_hash, grpc_metadata_array* md_ary);

void grpc_rb_metadata_array_destroy_including_entries(
    grpc_metadata_array* md_ary);

/* grpc_rb_eCallError is the ruby class of the exception thrown during call
   operations. */
extern VALUE grpc_rb_eCallError;

/* Initializes the Call class. */
void Init_grpc_call();

#endif /* GRPC_RB_CALL_H_ */
