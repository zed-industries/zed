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

#ifndef NET_GRPC_PHP_GRPC_BYTE_BUFFER_H_
#define NET_GRPC_PHP_GRPC_BYTE_BUFFER_H_

#include <grpc/grpc.h>

grpc_byte_buffer *string_to_byte_buffer(char *string, size_t length);

zend_string* byte_buffer_to_zend_string(grpc_byte_buffer *buffer);

#endif /* NET_GRPC_PHP_GRPC_BYTE_BUFFER_H_ */
