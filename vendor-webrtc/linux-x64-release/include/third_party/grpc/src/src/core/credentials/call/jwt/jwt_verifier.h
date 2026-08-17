//
//
// Copyright 2015 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
//

#ifndef GRPC_SRC_CORE_CREDENTIALS_CALL_JWT_JWT_VERIFIER_H
#define GRPC_SRC_CORE_CREDENTIALS_CALL_JWT_JWT_VERIFIER_H

#include <grpc/support/port_platform.h>
#include <grpc/support/time.h>
#include <stddef.h>

#include "src/core/lib/iomgr/iomgr_fwd.h"
#include "src/core/util/json/json.h"
#include "src/core/util/time.h"

// --- Constants. ---

#define GRPC_OPENID_CONFIG_URL_SUFFIX "/.well-known/openid-configuration"
#define GRPC_GOOGLE_SERVICE_ACCOUNTS_EMAIL_DOMAIN "gserviceaccount.com"
#define GRPC_GOOGLE_SERVICE_ACCOUNTS_KEY_URL_PREFIX \
  "www.googleapis.com/robot/v1/metadata/x509"

// --- grpc_jwt_verifier_status. ---

typedef enum {
  GRPC_JWT_VERIFIER_OK = 0,
  GRPC_JWT_VERIFIER_BAD_SIGNATURE,
  GRPC_JWT_VERIFIER_BAD_FORMAT,
  GRPC_JWT_VERIFIER_BAD_AUDIENCE,
  GRPC_JWT_VERIFIER_KEY_RETRIEVAL_ERROR,
  GRPC_JWT_VERIFIER_TIME_CONSTRAINT_FAILURE,
  GRPC_JWT_VERIFIER_BAD_SUBJECT,
  GRPC_JWT_VERIFIER_GENERIC_ERROR
} grpc_jwt_verifier_status;

const char* grpc_jwt_verifier_status_to_string(grpc_jwt_verifier_status status);

// --- grpc_jwt_claims. ---

typedef struct grpc_jwt_claims grpc_jwt_claims;

void grpc_jwt_claims_destroy(grpc_jwt_claims* claims);

// Returns the whole JSON tree of the claims.
const grpc_core::Json* grpc_jwt_claims_json(const grpc_jwt_claims* claims);

// Access to registered claims in https://tools.ietf.org/html/rfc7519#page-9
const char* grpc_jwt_claims_subject(const grpc_jwt_claims* claims);
const char* grpc_jwt_claims_issuer(const grpc_jwt_claims* claims);
const char* grpc_jwt_claims_id(const grpc_jwt_claims* claims);
const char* grpc_jwt_claims_audience(const grpc_jwt_claims* claims);
gpr_timespec grpc_jwt_claims_issued_at(const grpc_jwt_claims* claims);
gpr_timespec grpc_jwt_claims_expires_at(const grpc_jwt_claims* claims);
gpr_timespec grpc_jwt_claims_not_before(const grpc_jwt_claims* claims);

// --- grpc_jwt_verifier. ---

typedef struct grpc_jwt_verifier grpc_jwt_verifier;

struct grpc_jwt_verifier_email_domain_key_url_mapping {
  // The email domain is the part after the @ sign.
  const char* email_domain;

  // The key url prefix will be used to get the public key from the issuer:
  // https://<key_url_prefix>/<issuer_email>
  // Therefore the key_url_prefix must NOT contain https://.
  const char* key_url_prefix;
};
// Globals to control the verifier. Not thread-safe.
extern gpr_timespec grpc_jwt_verifier_clock_skew;
extern grpc_core::Duration grpc_jwt_verifier_max_delay;

// The verifier can be created with some custom mappings to help with key
// discovery in the case where the issuer is an email address.
// mappings can be NULL in which case num_mappings MUST be 0.
// A verifier object has one built-in mapping (unless overridden):
// GRPC_GOOGLE_SERVICE_ACCOUNTS_EMAIL_DOMAIN ->
// GRPC_GOOGLE_SERVICE_ACCOUNTS_KEY_URL_PREFIX.
grpc_jwt_verifier* grpc_jwt_verifier_create(
    const grpc_jwt_verifier_email_domain_key_url_mapping* mappings,
    size_t num_mappings);

// The verifier must not be destroyed if there are still outstanding callbacks.
void grpc_jwt_verifier_destroy(grpc_jwt_verifier* verifier);

// User provided callback that will be called when the verification of the JWT
// is done (maybe in another thread).
// It is the responsibility of the callee to call grpc_jwt_claims_destroy on
// the claims.
typedef void (*grpc_jwt_verification_done_cb)(void* user_data,
                                              grpc_jwt_verifier_status status,
                                              grpc_jwt_claims* claims);

// Verifies for the JWT for the given expected audience.
void grpc_jwt_verifier_verify(grpc_jwt_verifier* verifier,
                              grpc_pollset* pollset, const char* jwt,
                              const char* audience,
                              grpc_jwt_verification_done_cb cb,
                              void* user_data);

// --- TESTING ONLY exposed functions. ---

grpc_jwt_claims* grpc_jwt_claims_from_json(grpc_core::Json json);
grpc_jwt_verifier_status grpc_jwt_claims_check(const grpc_jwt_claims* claims,
                                               const char* audience);
const char* grpc_jwt_issuer_email_domain(const char* issuer);

#endif  // GRPC_SRC_CORE_CREDENTIALS_CALL_JWT_JWT_VERIFIER_H
