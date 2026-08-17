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

#ifndef GRPC_RB_BYTE_BUFFER_H_
#define GRPC_RB_BYTE_BUFFER_H_

#include <ruby/ruby.h>

#include <grpc/grpc.h>

/* Converts a char* with a length to a grpc_byte_buffer */
grpc_byte_buffer* grpc_rb_s_to_byte_buffer(char* string, size_t length);

/* Converts a grpc_byte_buffer to a ruby string */
VALUE grpc_rb_byte_buffer_to_s(grpc_byte_buffer* buffer);

/* Converts a grpc_slice to a ruby string */
VALUE grpc_rb_slice_to_ruby_string(grpc_slice slice);

#endif /* GRPC_RB_BYTE_BUFFER_H_ */
