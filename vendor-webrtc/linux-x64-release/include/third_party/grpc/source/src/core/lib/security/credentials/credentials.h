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

#ifndef GRPC_SRC_CORE_LIB_SECURITY_CREDENTIALS_CREDENTIALS_H
#define GRPC_SRC_CORE_LIB_SECURITY_CREDENTIALS_CREDENTIALS_H

#include <grpc/credentials.h>
#include <grpc/grpc.h>
#include <grpc/grpc_security.h>
#include <grpc/grpc_security_constants.h>
#include <grpc/impl/grpc_types.h>
#include <grpc/support/port_platform.h>

#include <string>
#include <utility>
#include <vector>

#include "absl/log/absl_check.h"
#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/promise/arena_promise.h"
#include "src/core/lib/security/security_connector/security_connector.h"
#include "src/core/lib/slice/slice.h"
#include "src/core/lib/transport/transport.h"
#include "src/core/util/crash.h"
#include "src/core/util/ref_counted.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/unique_type_name.h"

// --- Constants. ---

typedef enum {
  GRPC_CREDENTIALS_OK = 0,
  GRPC_CREDENTIALS_ERROR
} grpc_credentials_status;

#define GRPC_FAKE_TRANSPORT_SECURITY_TYPE "fake"

#define GRPC_AUTHORIZATION_METADATA_KEY "authorization"
#define GRPC_IAM_AUTHORIZATION_TOKEN_METADATA_KEY \
  "x-goog-iam-authorization-token"
#define GRPC_IAM_AUTHORITY_SELECTOR_METADATA_KEY "x-goog-iam-authority-selector"

#define GRPC_SECURE_TOKEN_REFRESH_THRESHOLD_SECS 60

#define GRPC_COMPUTE_ENGINE_METADATA_HOST "metadata.google.internal."
#define GRPC_COMPUTE_ENGINE_METADATA_TOKEN_PATH \
  "/computeMetadata/v1/instance/service-accounts/default/token"

#define GRPC_GOOGLE_OAUTH2_SERVICE_HOST "oauth2.googleapis.com"
#define GRPC_GOOGLE_OAUTH2_SERVICE_TOKEN_PATH "/token"

#define GRPC_SERVICE_ACCOUNT_POST_BODY_PREFIX                         \
  "grant_type=urn%3Aietf%3Aparams%3Aoauth%3Agrant-type%3Ajwt-bearer&" \
  "assertion="

#define GRPC_REFRESH_TOKEN_POST_BODY_FORMAT_STRING \
  "client_id=%s&client_secret=%s&refresh_token=%s&grant_type=refresh_token"

// --- Google utils ---

// It is the caller's responsibility to gpr_free the result if not NULL.
std::string grpc_get_well_known_google_credentials_file_path(void);

// Implementation function for the different platforms.
std::string grpc_get_well_known_google_credentials_file_path_impl(void);

// Override for testing only. Not thread-safe
typedef std::string (*grpc_well_known_credentials_path_getter)(void);
void grpc_override_well_known_credentials_path_getter(
    grpc_well_known_credentials_path_getter getter);

// --- grpc_channel_credentials. ---

#define GRPC_ARG_CHANNEL_CREDENTIALS "grpc.internal.channel_credentials"

// This type is forward declared as a C struct and we cannot define it as a
// class. Otherwise, compiler will complain about type mismatch due to
// -Wmismatched-tags.
struct grpc_channel_credentials
    : grpc_core::RefCounted<grpc_channel_credentials> {
 public:
  static absl::string_view ChannelArgName() {
    return GRPC_ARG_CHANNEL_CREDENTIALS;
  }

  static int ChannelArgsCompare(const grpc_channel_credentials* args1,
                                const grpc_channel_credentials* args2) {
    return args1->cmp(args2);
  }

  // Creates a security connector for the channel. Also updates passed in
  // channel args for the channel.
  virtual grpc_core::RefCountedPtr<grpc_channel_security_connector>
  create_security_connector(
      grpc_core::RefCountedPtr<grpc_call_credentials> call_creds,
      const char* target, grpc_core::ChannelArgs* args) = 0;

  // Creates a version of the channel credentials without any attached call
  // credentials. This can be used in order to open a channel to a non-trusted
  // gRPC load balancer.
  virtual grpc_core::RefCountedPtr<grpc_channel_credentials>
  duplicate_without_call_credentials() {
    // By default we just increment the refcount.
    return Ref();
  }

  // Allows credentials to optionally modify a parent channel's args.
  // By default, leave channel args as is.
  virtual grpc_core::ChannelArgs update_arguments(grpc_core::ChannelArgs args) {
    return args;
  }

  // Compares this grpc_channel_credentials object with \a other.
  // If this method returns 0, it means that gRPC can treat the two channel
  // credentials as effectively the same. This method is used to compare
  // `grpc_channel_credentials` objects when they are present in channel_args.
  // One important usage of this is when channel args are used in SubchannelKey,
  // which leads to a useful property that allows subchannels to be reused when
  // two different `grpc_channel_credentials` objects are used but they compare
  // as equal (assuming other channel args match).
  int cmp(const grpc_channel_credentials* other) const {
    ABSL_CHECK_NE(other, nullptr);
    int r = type().Compare(other->type());
    if (r != 0) return r;
    return cmp_impl(other);
  }

  // The pointer value \a type is used to uniquely identify a creds
  // implementation for down-casting purposes. Every creds implementation should
  // use a unique string instance, which should be returned by all instances of
  // that creds implementation.
  virtual grpc_core::UniqueTypeName type() const = 0;

 private:
  // Implementation for `cmp` method intended to be overridden by subclasses.
  // Only invoked if `type()` and `other->type()` point to the same string.
  virtual int cmp_impl(const grpc_channel_credentials* other) const = 0;
};

// TODO(roth): Once we eliminate insecure builds, find a better way to
// plumb credentials so that it doesn't need to flow through channel
// args.  For example, we'll want to expose it to LB policies by adding
// methods on the helper API.

// Util to encapsulate the channel credentials in a channel arg.
grpc_arg grpc_channel_credentials_to_arg(grpc_channel_credentials* credentials);

// Util to get the channel credentials from a channel arg.
grpc_channel_credentials* grpc_channel_credentials_from_arg(
    const grpc_arg* arg);

// Util to find the channel credentials from channel args.
grpc_channel_credentials* grpc_channel_credentials_find_in_args(
    const grpc_channel_args* args);

// --- grpc_core::CredentialsMetadataArray. ---

namespace grpc_core {
using CredentialsMetadataArray = std::vector<std::pair<Slice, Slice>>;
}

// --- grpc_call_credentials. ---

// This type is forward declared as a C struct and we cannot define it as a
// class. Otherwise, compiler will complain about type mismatch due to
// -Wmismatched-tags.
struct grpc_call_credentials
    : public grpc_core::DualRefCounted<grpc_call_credentials> {
 public:
  // TODO(roth): Consider whether security connector actually needs to
  // be part of this interface.  Currently, it is here only for the
  // url_scheme() method, which we might be able to instead add as an
  // auth context property.
  struct GetRequestMetadataArgs {
    grpc_core::RefCountedPtr<grpc_channel_security_connector>
        security_connector;
    grpc_core::RefCountedPtr<grpc_auth_context> auth_context;
  };

  // The pointer value \a type is used to uniquely identify a creds
  // implementation for down-casting purposes. Every creds implementation should
  // use a unique string instance, which should be returned by all instances of
  // that creds implementation.
  explicit grpc_call_credentials(
      grpc_security_level min_security_level = GRPC_PRIVACY_AND_INTEGRITY)
      : min_security_level_(min_security_level) {}

  ~grpc_call_credentials() override = default;

  virtual grpc_core::ArenaPromise<
      absl::StatusOr<grpc_core::ClientMetadataHandle>>
  GetRequestMetadata(grpc_core::ClientMetadataHandle initial_metadata,
                     const GetRequestMetadataArgs* args) = 0;

  virtual grpc_security_level min_security_level() const {
    return min_security_level_;
  }

  // Compares this grpc_call_credentials object with \a other.
  // If this method returns 0, it means that gRPC can treat the two call
  // credentials as effectively the same..
  int cmp(const grpc_call_credentials* other) const {
    ABSL_CHECK_NE(other, nullptr);
    int r = type().Compare(other->type());
    if (r != 0) return r;
    return cmp_impl(other);
  }

  virtual std::string debug_string() {
    return "grpc_call_credentials did not provide debug string";
  }

  // The pointer value \a type is used to uniquely identify a creds
  // implementation for down-casting purposes. Every creds implementation should
  // use a unique string instance, which should be returned by all instances of
  // that creds implementation.
  virtual grpc_core::UniqueTypeName type() const = 0;

 private:
  // Implementation for `cmp` method intended to be overridden by subclasses.
  // Only invoked if `type()` and `other->type()` point to the same string.
  virtual int cmp_impl(const grpc_call_credentials* other) const = 0;

  const grpc_security_level min_security_level_;
};

// Metadata-only credentials with the specified key and value where
// asynchronicity can be simulated for testing.
grpc_call_credentials* grpc_md_only_test_credentials_create(
    const char* md_key, const char* md_value);

// --- grpc_server_credentials. ---

#define GRPC_SERVER_CREDENTIALS_ARG "grpc.internal.server_credentials"

// This type is forward declared as a C struct and we cannot define it as a
// class. Otherwise, compiler will complain about type mismatch due to
// -Wmismatched-tags.
struct grpc_server_credentials
    : public grpc_core::RefCounted<grpc_server_credentials> {
 public:
  ~grpc_server_credentials() override { DestroyProcessor(); }

  static absl::string_view ChannelArgName() {
    return GRPC_SERVER_CREDENTIALS_ARG;
  }

  static int ChannelArgsCompare(const grpc_server_credentials* a,
                                const grpc_server_credentials* b) {
    return grpc_core::QsortCompare(a, b);
  }

  // Ownership of \a args is not passed.
  virtual grpc_core::RefCountedPtr<grpc_server_security_connector>
  create_security_connector(const grpc_core::ChannelArgs& args) = 0;

  virtual grpc_core::UniqueTypeName type() const = 0;

  const grpc_auth_metadata_processor& auth_metadata_processor() const {
    return processor_;
  }
  void set_auth_metadata_processor(
      const grpc_auth_metadata_processor& processor);

 private:
  void DestroyProcessor() {
    if (processor_.destroy != nullptr && processor_.state != nullptr) {
      processor_.destroy(processor_.state);
    }
  }

  grpc_auth_metadata_processor processor_ =
      grpc_auth_metadata_processor();  // Zero-initialize the C struct.
};

grpc_arg grpc_server_credentials_to_arg(grpc_server_credentials* c);
grpc_server_credentials* grpc_server_credentials_from_arg(const grpc_arg* arg);
grpc_server_credentials* grpc_find_server_credentials_in_args(
    const grpc_channel_args* args);

#endif  // GRPC_SRC_CORE_LIB_SECURITY_CREDENTIALS_CREDENTIALS_H
