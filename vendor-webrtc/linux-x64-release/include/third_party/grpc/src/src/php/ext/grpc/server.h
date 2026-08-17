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

#ifndef NET_GRPC_PHP_GRPC_SERVER_H_
#define NET_GRPC_PHP_GRPC_SERVER_H_

#include "php_grpc.h"

/* Class entry for the Server PHP class */
extern zend_class_entry *grpc_ce_server;

/* Wrapper struct for grpc_server that can be associated with a PHP object */
PHP_GRPC_WRAP_OBJECT_START(wrapped_grpc_server)
  grpc_server *wrapped;
PHP_GRPC_WRAP_OBJECT_END(wrapped_grpc_server)

static inline wrapped_grpc_server
*wrapped_grpc_server_from_obj(zend_object *obj) {
  return (wrapped_grpc_server*)((char*)(obj) -
                                XtOffsetOf(wrapped_grpc_server, std));
}

/* Initializes the Server class */
void grpc_init_server(TSRMLS_D);

#endif /* NET_GRPC_PHP_GRPC_SERVER_H_ */
