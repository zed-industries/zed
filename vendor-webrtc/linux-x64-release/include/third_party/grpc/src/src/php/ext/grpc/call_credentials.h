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

#ifndef NET_GRPC_PHP_GRPC_CALL_CREDENTIALS_H_
#define NET_GRPC_PHP_GRPC_CALL_CREDENTIALS_H_

#include "php_grpc.h"

#include <grpc/credentials.h>
#include <grpc/grpc_security.h>

/* Class entry for the CallCredentials PHP class */
extern zend_class_entry *grpc_ce_call_credentials;

/* Wrapper struct for grpc_call_credentials that can be associated
 * with a PHP object */
PHP_GRPC_WRAP_OBJECT_START(wrapped_grpc_call_credentials)
  grpc_call_credentials *wrapped;
PHP_GRPC_WRAP_OBJECT_END(wrapped_grpc_call_credentials)

static inline wrapped_grpc_call_credentials
*wrapped_grpc_call_credentials_from_obj(zend_object *obj) {
  return (wrapped_grpc_call_credentials*)(
      (char*)(obj) - XtOffsetOf(wrapped_grpc_call_credentials, std));
}

/* Struct to hold callback function for plugin creds API */
typedef struct plugin_state {
  zend_fcall_info *fci;
  zend_fcall_info_cache *fci_cache;
} plugin_state;

/* Callback function for plugin creds API */
int plugin_get_metadata(
  void *ptr, grpc_auth_metadata_context context,
  grpc_credentials_plugin_metadata_cb cb, void *user_data,
  grpc_metadata creds_md[GRPC_METADATA_CREDENTIALS_PLUGIN_SYNC_MAX],
  size_t *num_creds_md, grpc_status_code *status,
  const char **error_details);

/* Cleanup function for plugin creds API */
void plugin_destroy_state(void *ptr);

/* Initializes the CallCredentials PHP class */
void grpc_init_call_credentials(TSRMLS_D);

#endif /* NET_GRPC_PHP_GRPC_CALL_CREDENTIALS_H_ */
