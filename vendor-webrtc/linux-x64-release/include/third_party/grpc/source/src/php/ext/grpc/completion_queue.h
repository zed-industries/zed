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

#ifndef GRPC_PHP_GRPC_COMPLETION_QUEUE_H_
#define GRPC_PHP_GRPC_COMPLETION_QUEUE_H_

#include <php.h>

#include <grpc/grpc.h>

#if PHP_MAJOR_VERSION >= 8
#define TSRMLS_D
#endif

/* The global completion queue for all operations */
extern grpc_completion_queue *completion_queue;

/* Initializes the completion queue */
void grpc_php_init_completion_queue(TSRMLS_D);

/* Shut down the completion queue */
void grpc_php_shutdown_completion_queue(TSRMLS_D);

#endif /* GRPC_PHP_GRPC_COMPLETION_QUEUE_H_ */
