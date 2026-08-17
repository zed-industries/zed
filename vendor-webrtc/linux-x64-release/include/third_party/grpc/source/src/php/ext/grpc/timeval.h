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

#ifndef NET_GRPC_PHP_GRPC_TIMEVAL_H_
#define NET_GRPC_PHP_GRPC_TIMEVAL_H_

#include "php_grpc.h"

/* Class entry for the Timeval PHP Class */
extern zend_class_entry *grpc_ce_timeval;

/* Wrapper struct for timeval that can be associated with a PHP object */
PHP_GRPC_WRAP_OBJECT_START(wrapped_grpc_timeval)
  gpr_timespec wrapped;
PHP_GRPC_WRAP_OBJECT_END(wrapped_grpc_timeval)

static inline wrapped_grpc_timeval
*wrapped_grpc_timeval_from_obj(zend_object *obj) {
  return (wrapped_grpc_timeval*)((char*)(obj) -
                                 XtOffsetOf(wrapped_grpc_timeval, std));
}

/* Initialize the Timeval PHP class */
void grpc_init_timeval(TSRMLS_D);

/* Shutdown the Timeval PHP class */
void grpc_shutdown_timeval(TSRMLS_D);

/* Creates a Timeval object that wraps the given timeval struct */
zval *grpc_php_wrap_timeval(gpr_timespec wrapped TSRMLS_DC);

#endif /* NET_GRPC_PHP_GRPC_TIMEVAL_H_ */
