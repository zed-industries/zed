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

#ifndef GRPC_BYTE_BUFFER_READER_H
#define GRPC_BYTE_BUFFER_READER_H

#include <grpc/support/port_platform.h>

#ifdef __cplusplus
extern "C" {
#endif

struct grpc_byte_buffer;

struct grpc_byte_buffer_reader {
  struct grpc_byte_buffer* buffer_in;
  struct grpc_byte_buffer* buffer_out;
  /** Different current objects correspond to different types of byte buffers */
  union grpc_byte_buffer_reader_current {
    /** Index into a slice buffer's array of slices */
    unsigned index;
  } current;
};

#ifdef __cplusplus
}
#endif

#endif /* GRPC_BYTE_BUFFER_READER_H */
