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

#ifndef NET_GRPC_PHP_GRPC_CHANNEL_CREDENTIALS_H_
#define NET_GRPC_PHP_GRPC_CHANNEL_CREDENTIALS_H_

#include "php_grpc.h"

#include <grpc/credentials.h>
#include <grpc/grpc_security.h>

/* Class entry for the ChannelCredentials PHP class */
extern zend_class_entry *grpc_ce_channel_credentials;

/* Wrapper struct for grpc_channel_credentials that can be associated
 * with a PHP object */
PHP_GRPC_WRAP_OBJECT_START(wrapped_grpc_channel_credentials) 
  grpc_channel_credentials *wrapped;
  char *hashstr;
  zend_bool has_call_creds;
PHP_GRPC_WRAP_OBJECT_END(wrapped_grpc_channel_credentials)

static inline wrapped_grpc_channel_credentials
*wrapped_grpc_channel_credentials_from_obj(zend_object *obj) {
  return (wrapped_grpc_channel_credentials *)(
      (char*)(obj) - XtOffsetOf(wrapped_grpc_channel_credentials, std));
}

/* Initializes the ChannelCredentials PHP class */
void grpc_init_channel_credentials(TSRMLS_D);

#endif /* NET_GRPC_PHP_GRPC_CHANNEL_CREDENTIALS_H_ */
