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

#ifndef NET_GRPC_PHP_GRPC_CHANNEL_H_
#define NET_GRPC_PHP_GRPC_CHANNEL_H_

#include "channel_credentials.h"
#include "php_grpc.h"

#if PHP_MAJOR_VERSION >= 8
#define ZEND_HASH_INIT(hash_table, limit, descriptor, zend_bool) _zend_hash_init(hash_table, limit, descriptor, zend_bool);
#else
#define ZEND_HASH_INIT(hash_table, limit, descriptor, zend_bool) zend_hash_init_ex(hash_table, limit, NULL, descriptor, zend_bool, 0);
#endif

/* Class entry for the PHP Channel class */
extern zend_class_entry *grpc_ce_channel;

typedef struct _grpc_channel_wrapper {
  grpc_channel *wrapped;
  char *key;
  char *target;
  char *args_hashstr;
  char *creds_hashstr;
  size_t ref_count;
  gpr_mu mu;
  grpc_channel_args args;
  wrapped_grpc_channel_credentials *creds;
} grpc_channel_wrapper;

/* Wrapper struct for grpc_channel that can be associated with a PHP object */
PHP_GRPC_WRAP_OBJECT_START(wrapped_grpc_channel)
  grpc_channel_wrapper *wrapper;
PHP_GRPC_WRAP_OBJECT_END(wrapped_grpc_channel)

static inline wrapped_grpc_channel
*wrapped_grpc_channel_from_obj(zend_object *obj) {
  return (wrapped_grpc_channel*)((char*)(obj) -
                                 XtOffsetOf(wrapped_grpc_channel, std));
}

/* Initializes the Channel class */
GRPC_STARTUP_FUNCTION(channel);

/* Iterates through a PHP array and populates args with the contents */
int php_grpc_read_args_array(zval *args_array, grpc_channel_args *args
                             TSRMLS_DC);

void generate_sha1_str(char *sha1str, char *str, php_grpc_int len);

void php_grpc_delete_persistent_list_entry(char *key, php_grpc_int key_len
                                           TSRMLS_DC);

typedef struct _channel_persistent_le {
  grpc_channel_wrapper *channel;
} channel_persistent_le_t;

typedef struct _target_bound_le {
  int upper_bound;
  int current_count;
} target_bound_le_t;

#endif /* NET_GRPC_PHP_GRPC_CHANNEL_H_ */
