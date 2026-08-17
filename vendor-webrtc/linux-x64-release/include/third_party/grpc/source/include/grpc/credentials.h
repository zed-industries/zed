/*
 *
 * Copyright 2024 gRPC authors.
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

#ifndef GRPC_CREDENTIALS_H
#define GRPC_CREDENTIALS_H

#include <grpc/grpc.h>
#include <grpc/grpc_security_constants.h>
#include <grpc/support/port_platform.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

/** --- grpc_call_credentials object ---

   A call credentials object represents a way to authenticate on a particular
   call. These credentials can be composed with a channel credentials object
   so that they are sent with every call on this channel.  */

typedef struct grpc_call_credentials grpc_call_credentials;
typedef struct grpc_auth_context grpc_auth_context;

/** Creates a JWT credentials object. May return NULL if the input is invalid.
   - json_key is the JSON key string containing the client's private key.
   - token_lifetime is the lifetime of each Json Web Token (JWT) created with
     this credentials.  It should not exceed grpc_max_auth_token_lifetime or
     will be cropped to this value.  */
GRPCAPI grpc_call_credentials*
grpc_service_account_jwt_access_credentials_create(const char* json_key,
                                                   gpr_timespec token_lifetime,
                                                   void* reserved);

/** Builds External Account credentials.
 - json_string is the JSON string containing the credentials options.
 - scopes_string contains the scopes to be binded with the credentials.
   This API is used for experimental purposes for now and may change in the
 future. */
GRPCAPI grpc_call_credentials* grpc_external_account_credentials_create(
    const char* json_string, const char* scopes_string);

/** Creates an Oauth2 Refresh Token credentials object for connecting to Google.
   May return NULL if the input is invalid.
   WARNING: Do NOT use this credentials to connect to a non-google service as
   this could result in an oauth2 token leak.
   - json_refresh_token is the JSON string containing the refresh token itself
     along with a client_id and client_secret. */
GRPCAPI grpc_call_credentials* grpc_google_refresh_token_credentials_create(
    const char* json_refresh_token, void* reserved);

/** Creates an Oauth2 Access Token credentials with an access token that was
   acquired by an out of band mechanism. */
GRPCAPI grpc_call_credentials* grpc_access_token_credentials_create(
    const char* access_token, void* reserved);

/** Creates an IAM credentials object for connecting to Google. */
GRPCAPI grpc_call_credentials* grpc_google_iam_credentials_create(
    const char* authorization_token, const char* authority_selector,
    void* reserved);

/** Options for creating STS Oauth Token Exchange credentials following the IETF
   draft https://tools.ietf.org/html/draft-ietf-oauth-token-exchange-16.
   Optional fields may be set to NULL or empty string. It is the responsibility
   of the caller to ensure that the subject and actor tokens are refreshed on
   disk at the specified paths. This API is used for experimental purposes for
   now and may change in the future. */
typedef struct {
  const char* token_exchange_service_uri; /* Required. */
  const char* resource;                   /* Optional. */
  const char* audience;                   /* Optional. */
  const char* scope;                      /* Optional. */
  const char* requested_token_type;       /* Optional. */
  const char* subject_token_path;         /* Required. */
  const char* subject_token_type;         /* Required. */
  const char* actor_token_path;           /* Optional. */
  const char* actor_token_type;           /* Optional. */
} grpc_sts_credentials_options;

/** Creates an STS credentials following the STS Token Exchanged specified in
   the IETF draft
   https://tools.ietf.org/html/draft-ietf-oauth-token-exchange-16. This API is
   used for experimental purposes for now and may change in the future. */
GRPCAPI grpc_call_credentials* grpc_sts_credentials_create(
    const grpc_sts_credentials_options* options, void* reserved);

/** Context that can be used by metadata credentials plugin in order to create
   auth related metadata. */
typedef struct {
  /** The fully qualified service url. */
  const char* service_url;

  /** The method name of the RPC being called (not fully qualified).
     The fully qualified method name can be built from the service_url:
     full_qualified_method_name = ctx->service_url + '/' + ctx->method_name. */
  const char* method_name;

  /** The auth_context of the channel which gives the server's identity. */
  const grpc_auth_context* channel_auth_context;

  /** Reserved for future use. */
  void* reserved;
} grpc_auth_metadata_context;

/** Performs a deep copy from \a from to \a to. **/
GRPCAPI void grpc_auth_metadata_context_copy(grpc_auth_metadata_context* from,
                                             grpc_auth_metadata_context* to);

/** Releases internal resources held by \a context. **/
GRPCAPI void grpc_auth_metadata_context_reset(
    grpc_auth_metadata_context* context);

/** Callback function to be called by the metadata credentials plugin
   implementation when the metadata is ready.
   - user_data is the opaque pointer that was passed in the get_metadata method
     of the grpc_metadata_credentials_plugin (see below).
   - creds_md is an array of credentials metadata produced by the plugin. It
     may be set to NULL in case of an error.
   - num_creds_md is the number of items in the creds_md array.
   - status must be GRPC_STATUS_OK in case of success or another specific error
     code otherwise.
   - error_details contains details about the error if any. In case of success
     it should be NULL and will be otherwise ignored. */
typedef void (*grpc_credentials_plugin_metadata_cb)(
    void* user_data, const grpc_metadata* creds_md, size_t num_creds_md,
    grpc_status_code status, const char* error_details);

/** Maximum number of metadata entries returnable by a credentials plugin via
    a synchronous return. */
#define GRPC_METADATA_CREDENTIALS_PLUGIN_SYNC_MAX 4

/** grpc_metadata_credentials plugin is an API user provided structure used to
   create grpc_credentials objects that can be set on a channel (composed) or
   a call. See grpc_credentials_metadata_create_from_plugin below.
   The grpc client stack will call the get_metadata method of the plugin for
   every call in scope for the credentials created from it. */
typedef struct {
  /** The implementation of this method has to be non-blocking, but can
     be performed synchronously or asynchronously.

     If processing occurs synchronously, returns non-zero and populates
     creds_md, num_creds_md, status, and error_details.  In this case,
     the caller takes ownership of the entries in creds_md and of
     error_details.  Note that if the plugin needs to return more than
     GRPC_METADATA_CREDENTIALS_PLUGIN_SYNC_MAX entries in creds_md, it must
     return asynchronously.

     If processing occurs asynchronously, returns zero and invokes \a cb
     when processing is completed.  \a user_data will be passed as the
     first parameter of the callback.  NOTE: \a cb MUST be invoked in a
     different thread, not from the thread in which \a get_metadata() is
     invoked.

     \a context is the information that can be used by the plugin to create
     auth metadata. */
  int (*get_metadata)(
      void* state, grpc_auth_metadata_context context,
      grpc_credentials_plugin_metadata_cb cb, void* user_data,
      grpc_metadata creds_md[GRPC_METADATA_CREDENTIALS_PLUGIN_SYNC_MAX],
      size_t* num_creds_md, grpc_status_code* status,
      const char** error_details);

  /** Implements debug string of the given plugin. This method returns an
   * allocated string that the caller needs to free using gpr_free() */
  char* (*debug_string)(void* state);

  /** Destroys the plugin state. */
  void (*destroy)(void* state);

  /** State that will be set as the first parameter of the methods above. */
  void* state;

  /** Type of credentials that this plugin is implementing. */
  const char* type;
} grpc_metadata_credentials_plugin;

/** Creates a credentials object from a plugin with a specified minimum security
 * level. */
GRPCAPI grpc_call_credentials* grpc_metadata_credentials_create_from_plugin(
    grpc_metadata_credentials_plugin plugin,
    grpc_security_level min_security_level, void* reserved);

/** --- channel credentials --- */

/** Releases a call credentials object.
   The creator of the credentials object is responsible for its release. */
GRPCAPI void grpc_call_credentials_release(grpc_call_credentials* creds);

/** Creates default credentials to connect to a google gRPC service.
   WARNING: Do NOT use this credentials to connect to a non-google service as
   this could result in an oauth2 token leak. The security level of the
   resulting connection is GRPC_PRIVACY_AND_INTEGRITY.

   If specified, the supplied call credentials object will be attached to the
   returned channel credentials object. The call_credentials object must remain
   valid throughout the lifetime of the returned grpc_channel_credentials
   object. It is expected that the call credentials object was generated
   according to the Application Default Credentials mechanism and asserts the
   identity of the default service account of the machine. Supplying any other
   sort of call credential will result in undefined behavior, up to and
   including the sudden and unexpected failure of RPCs.

   If nullptr is supplied, the returned channel credentials object will use a
   call credentials object based on the Application Default Credentials
   mechanism.
*/
GRPCAPI grpc_channel_credentials* grpc_google_default_credentials_create(
    grpc_call_credentials* call_credentials);

/** Server certificate config object holds the server's public certificates and
   associated private keys, as well as any CA certificates needed for client
   certificate validation (if applicable). Create using
   grpc_ssl_server_certificate_config_create(). */
typedef struct grpc_ssl_server_certificate_config
    grpc_ssl_server_certificate_config;

/** Object that holds a private key / certificate chain pair in PEM format. */
typedef struct {
  /** private_key is the NULL-terminated string containing the PEM encoding of
     the client's private key. */
  const char* private_key;

  /** cert_chain is the NULL-terminated string containing the PEM encoding of
     the client's certificate chain. */
  const char* cert_chain;
} grpc_ssl_pem_key_cert_pair;

/** Creates a grpc_ssl_server_certificate_config object.
   - pem_roots_cert is the NULL-terminated string containing the PEM encoding of
     the client root certificates. This parameter may be NULL if the server does
     not want the client to be authenticated with SSL.
   - pem_key_cert_pairs is an array private key / certificate chains of the
     server. This parameter cannot be NULL.
   - num_key_cert_pairs indicates the number of items in the private_key_files
     and cert_chain_files parameters. It must be at least 1.
   - It is the caller's responsibility to free this object via
     grpc_ssl_server_certificate_config_destroy(). */
GRPCAPI grpc_ssl_server_certificate_config*
grpc_ssl_server_certificate_config_create(
    const char* pem_root_certs,
    const grpc_ssl_pem_key_cert_pair* pem_key_cert_pairs,
    size_t num_key_cert_pairs);

/** Destroys a grpc_ssl_server_certificate_config object. */
GRPCAPI void grpc_ssl_server_certificate_config_destroy(
    grpc_ssl_server_certificate_config* config);

/** Callback to retrieve updated SSL server certificates, private keys, and
   trusted CAs (for client authentication).
    - user_data parameter, if not NULL, contains opaque data to be used by the
      callback.
    - Use grpc_ssl_server_certificate_config_create to create the config.
    - The caller assumes ownership of the config. */
typedef grpc_ssl_certificate_config_reload_status (
    *grpc_ssl_server_certificate_config_callback)(
    void* user_data, grpc_ssl_server_certificate_config** config);

/** Deprecated in favor of grpc_ssl_verify_peer_options. It will be removed
  after all of its call sites are migrated to grpc_ssl_verify_peer_options.
  Object that holds additional peer-verification options on a secure
  channel. */
typedef struct {
  /** If non-NULL this callback will be invoked with the expected
     target_name, the peer's certificate (in PEM format), and whatever
     userdata pointer is set below. If a non-zero value is returned by this
     callback then it is treated as a verification failure. Invocation of
     the callback is blocking, so any implementation should be light-weight.
     */
  int (*verify_peer_callback)(const char* target_name, const char* peer_pem,
                              void* userdata);
  /** Arbitrary userdata that will be passed as the last argument to
     verify_peer_callback. */
  void* verify_peer_callback_userdata;
  /** A destruct callback that will be invoked when the channel is being
     cleaned up. The userdata argument will be passed to it. The intent is
     to perform any cleanup associated with that userdata. */
  void (*verify_peer_destruct)(void* userdata);
} verify_peer_options;

/** Object that holds additional peer-verification options on a secure
   channel. */
typedef struct {
  /** If non-NULL this callback will be invoked with the expected
     target_name, the peer's certificate (in PEM format), and whatever
     userdata pointer is set below. If a non-zero value is returned by this
     callback then it is treated as a verification failure. Invocation of
     the callback is blocking, so any implementation should be light-weight.
     */
  int (*verify_peer_callback)(const char* target_name, const char* peer_pem,
                              void* userdata);
  /** Arbitrary userdata that will be passed as the last argument to
     verify_peer_callback. */
  void* verify_peer_callback_userdata;
  /** A destruct callback that will be invoked when the channel is being
     cleaned up. The userdata argument will be passed to it. The intent is
     to perform any cleanup associated with that userdata. */
  void (*verify_peer_destruct)(void* userdata);
} grpc_ssl_verify_peer_options;

/** Deprecated in favor of grpc_ssl_server_credentials_create_ex. It will be
   removed after all of its call sites are migrated to
   grpc_ssl_server_credentials_create_ex. Creates an SSL credentials object.
   The security level of the resulting connection is GRPC_PRIVACY_AND_INTEGRITY.
   - pem_root_certs is the NULL-terminated string containing the PEM encoding
     of the server root certificates. If this parameter is NULL, the
     implementation will first try to dereference the file pointed by the
     GRPC_DEFAULT_SSL_ROOTS_FILE_PATH environment variable, and if that fails,
     try to get the roots set by grpc_override_ssl_default_roots. Eventually,
     if all these fail, it will try to get the roots from a well-known place on
     disk (in the grpc install directory).

     gRPC has implemented root cache if the underlying OpenSSL library supports
     it. The gRPC root certificates cache is only applicable on the default
     root certificates, which is used when this parameter is nullptr. If user
     provides their own pem_root_certs, when creating an SSL credential object,
     gRPC would not be able to cache it, and each subchannel will generate a
     copy of the root store. So it is recommended to avoid providing large room
     pem with pem_root_certs parameter to avoid excessive memory consumption,
     particularly on mobile platforms such as iOS.
   - pem_key_cert_pair is a pointer on the object containing client's private
     key and certificate chain. This parameter can be NULL if the client does
     not have such a key/cert pair.
   - verify_options is an optional verify_peer_options object which holds
     additional options controlling how peer certificates are verified. For
     example, you can supply a callback which receives the peer's certificate
     with which you can do additional verification. Can be NULL, in which
     case verification will retain default behavior. Any settings in
     verify_options are copied during this call, so the verify_options
     object can be released afterwards. */
GRPCAPI grpc_channel_credentials* grpc_ssl_credentials_create(
    const char* pem_root_certs, grpc_ssl_pem_key_cert_pair* pem_key_cert_pair,
    const verify_peer_options* verify_options, void* reserved);

/* Creates an SSL credentials object.
   The security level of the resulting connection is GRPC_PRIVACY_AND_INTEGRITY.
   - pem_root_certs is the NULL-terminated string containing the PEM encoding
     of the server root certificates. If this parameter is NULL, the
     implementation will first try to dereference the file pointed by the
     GRPC_DEFAULT_SSL_ROOTS_FILE_PATH environment variable, and if that fails,
     try to get the roots set by grpc_override_ssl_default_roots. Eventually,
     if all these fail, it will try to get the roots from a well-known place on
     disk (in the grpc install directory).

     gRPC has implemented root cache if the underlying OpenSSL library supports
     it. The gRPC root certificates cache is only applicable on the default
     root certificates, which is used when this parameter is nullptr. If user
     provides their own pem_root_certs, when creating an SSL credential object,
     gRPC would not be able to cache it, and each subchannel will generate a
     copy of the root store. So it is recommended to avoid providing large room
     pem with pem_root_certs parameter to avoid excessive memory consumption,
     particularly on mobile platforms such as iOS.
   - pem_key_cert_pair is a pointer on the object containing client's private
     key and certificate chain. This parameter can be NULL if the client does
     not have such a key/cert pair.
   - verify_options is an optional verify_peer_options object which holds
     additional options controlling how peer certificates are verified. For
     example, you can supply a callback which receives the peer's certificate
     with which you can do additional verification. Can be NULL, in which
     case verification will retain default behavior. Any settings in
     verify_options are copied during this call, so the verify_options
     object can be released afterwards. */
GRPCAPI grpc_channel_credentials* grpc_ssl_credentials_create_ex(
    const char* pem_root_certs, grpc_ssl_pem_key_cert_pair* pem_key_cert_pair,
    const grpc_ssl_verify_peer_options* verify_options, void* reserved);

/** --- server credentials --- */

/** Deprecated in favor of grpc_ssl_server_credentials_create_ex.
   Creates an SSL server_credentials object.
   - pem_roots_cert is the NULL-terminated string containing the PEM encoding of
     the client root certificates. This parameter may be NULL if the server does
     not want the client to be authenticated with SSL.
   - pem_key_cert_pairs is an array private key / certificate chains of the
     server. This parameter cannot be NULL.
   - num_key_cert_pairs indicates the number of items in the private_key_files
     and cert_chain_files parameters. It should be at least 1.
   - force_client_auth, if set to non-zero will force the client to authenticate
     with an SSL cert. Note that this option is ignored if pem_root_certs is
     NULL. */
GRPCAPI grpc_server_credentials* grpc_ssl_server_credentials_create(
    const char* pem_root_certs, grpc_ssl_pem_key_cert_pair* pem_key_cert_pairs,
    size_t num_key_cert_pairs, int force_client_auth, void* reserved);

/** Deprecated in favor of grpc_ssl_server_credentials_create_with_options.
   Same as grpc_ssl_server_credentials_create method except uses
   grpc_ssl_client_certificate_request_type enum to support more ways to
   authenticate client certificates.*/
GRPCAPI grpc_server_credentials* grpc_ssl_server_credentials_create_ex(
    const char* pem_root_certs, grpc_ssl_pem_key_cert_pair* pem_key_cert_pairs,
    size_t num_key_cert_pairs,
    grpc_ssl_client_certificate_request_type client_certificate_request,
    void* reserved);

typedef struct grpc_ssl_server_credentials_options
    grpc_ssl_server_credentials_options;

/** Creates an options object using a certificate config. Use this method when
   the certificates and keys of the SSL server will not change during the
   server's lifetime.
   - Takes ownership of the certificate_config parameter. */
GRPCAPI grpc_ssl_server_credentials_options*
grpc_ssl_server_credentials_create_options_using_config(
    grpc_ssl_client_certificate_request_type client_certificate_request,
    grpc_ssl_server_certificate_config* certificate_config);

/** Creates an options object using a certificate config fetcher. Use this
   method to reload the certificates and keys of the SSL server without
   interrupting the operation of the server. Initial certificate config will be
   fetched during server initialization.
   - user_data parameter, if not NULL, contains opaque data which will be passed
     to the fetcher (see definition of
     grpc_ssl_server_certificate_config_callback). */
GRPCAPI grpc_ssl_server_credentials_options*
grpc_ssl_server_credentials_create_options_using_config_fetcher(
    grpc_ssl_client_certificate_request_type client_certificate_request,
    grpc_ssl_server_certificate_config_callback cb, void* user_data);

/** Destroys a grpc_ssl_server_credentials_options object. */
GRPCAPI void grpc_ssl_server_credentials_options_destroy(
    grpc_ssl_server_credentials_options* options);

/** Creates an SSL server_credentials object using the provided options struct.
    - Takes ownership of the options parameter. */
GRPCAPI grpc_server_credentials*
grpc_ssl_server_credentials_create_with_options(
    grpc_ssl_server_credentials_options* options);

/** --- Auth Metadata Processing --- */

/** Callback function that is called when the metadata processing is done.
   - Consumed metadata will be removed from the set of metadata available on the
     call. consumed_md may be NULL if no metadata has been consumed.
   - Response metadata will be set on the response. response_md may be NULL.
   - status is GRPC_STATUS_OK for success or a specific status for an error.
     Common error status for auth metadata processing is either
     GRPC_STATUS_UNAUTHENTICATED in case of an authentication failure or
     GRPC_STATUS PERMISSION_DENIED in case of an authorization failure.
   - error_details gives details about the error. May be NULL. */
typedef void (*grpc_process_auth_metadata_done_cb)(
    void* user_data, const grpc_metadata* consumed_md, size_t num_consumed_md,
    const grpc_metadata* response_md, size_t num_response_md,
    grpc_status_code status, const char* error_details);

/** Pluggable server-side metadata processor object. */
typedef struct {
  /** The context object is read/write: it contains the properties of the
     channel peer and it is the job of the process function to augment it with
     properties derived from the passed-in metadata.
     The lifetime of these objects is guaranteed until cb is invoked. */
  void (*process)(void* state, grpc_auth_context* context,
                  const grpc_metadata* md, size_t num_md,
                  grpc_process_auth_metadata_done_cb cb, void* user_data);
  void (*destroy)(void* state);
  void* state;
} grpc_auth_metadata_processor;

GRPCAPI void grpc_server_credentials_set_auth_metadata_processor(
    grpc_server_credentials* creds, grpc_auth_metadata_processor processor);

/** --- composite credentials --- */

/** Creates a composite call credentials object. */
GRPCAPI grpc_call_credentials* grpc_composite_call_credentials_create(
    grpc_call_credentials* creds1, grpc_call_credentials* creds2,
    void* reserved);

/** Creates a compute engine credentials object for connecting to Google.
   WARNING: Do NOT use this credentials to connect to a non-google service as
   this could result in an oauth2 token leak. */
GRPCAPI grpc_call_credentials* grpc_google_compute_engine_credentials_create(
    void* reserved);

/** Creates a composite channel credentials object. The security level of
 * resulting connection is determined by channel_creds. */
GRPCAPI grpc_channel_credentials* grpc_composite_channel_credentials_create(
    grpc_channel_credentials* channel_creds, grpc_call_credentials* call_creds,
    void* reserved);

/** --- ALTS channel/server credentials --- **/

/**
 * Main interface for ALTS credentials options. The options will contain
 * information that will be passed from grpc to TSI layer such as RPC protocol
 * versions. ALTS client (channel) and server credentials will have their own
 * implementation of this interface. The APIs listed in this header are
 * thread-compatible. It is used for experimental purpose for now and subject
 * to change.
 */
typedef struct grpc_alts_credentials_options grpc_alts_credentials_options;

/**
 * This method creates a grpc ALTS credentials client options instance.
 * It is used for experimental purpose for now and subject to change.
 */
GRPCAPI grpc_alts_credentials_options*
grpc_alts_credentials_client_options_create(void);

/**
 * This method creates a grpc ALTS credentials server options instance.
 * It is used for experimental purpose for now and subject to change.
 */
GRPCAPI grpc_alts_credentials_options*
grpc_alts_credentials_server_options_create(void);

/**
 * This method adds a target service account to grpc client's ALTS credentials
 * options instance. It is used for experimental purpose for now and subject
 * to change.
 *
 * - options: grpc ALTS credentials options instance.
 * - service_account: service account of target endpoint.
 */
GRPCAPI void grpc_alts_credentials_client_options_add_target_service_account(
    grpc_alts_credentials_options* options, const char* service_account);

/**
 * This method destroys a grpc_alts_credentials_options instance by
 * de-allocating all of its occupied memory. It is used for experimental purpose
 * for now and subject to change.
 *
 * - options: a grpc_alts_credentials_options instance that needs to be
 *   destroyed.
 */
GRPCAPI void grpc_alts_credentials_options_destroy(
    grpc_alts_credentials_options* options);

/**
 * This method creates an ALTS channel credential object. The security
 * level of the resulting connection is GRPC_PRIVACY_AND_INTEGRITY.
 * It is used for experimental purpose for now and subject to change.
 *
 * - options: grpc ALTS credentials options instance for client.
 *
 * It returns the created ALTS channel credential object.
 */
GRPCAPI grpc_channel_credentials* grpc_alts_credentials_create(
    const grpc_alts_credentials_options* options);

/**
 * This method creates an ALTS server credential object. It is used for
 * experimental purpose for now and subject to change.
 *
 * - options: grpc ALTS credentials options instance for server.
 *
 * It returns the created ALTS server credential object.
 */
GRPCAPI grpc_server_credentials* grpc_alts_server_credentials_create(
    const grpc_alts_credentials_options* options);

/**
 * EXPERIMENTAL API - Subject to change
 *
 * A struct that can be specified by callers to configure underlying TLS
 * behaviors.
 */
typedef struct grpc_tls_credentials_options grpc_tls_credentials_options;

/** --- TLS channel/server credentials ---
 * It is used for experimental purpose for now and subject to change. */

/**
 * EXPERIMENTAL API - Subject to change
 *
 * A struct provides ways to gain credential data that will be used in the TLS
 * handshake.
 */
typedef struct grpc_tls_certificate_provider grpc_tls_certificate_provider;

/**
 * EXPERIMENTAL API - Subject to change
 *
 * A struct that stores the credential data presented to the peer in handshake
 * to show local identity.
 */
typedef struct grpc_tls_identity_pairs grpc_tls_identity_pairs;

/**
 * EXPERIMENTAL API - Subject to change
 *
 * Creates a grpc_tls_identity_pairs that stores a list of identity credential
 * data, including identity private key and identity certificate chain.
 */
GRPCAPI grpc_tls_identity_pairs* grpc_tls_identity_pairs_create();

/**
 * EXPERIMENTAL API - Subject to change
 *
 * Adds a identity private key and a identity certificate chain to
 * grpc_tls_identity_pairs. This function will make an internal copy of
 * |private_key| and |cert_chain|.
 */
GRPCAPI void grpc_tls_identity_pairs_add_pair(grpc_tls_identity_pairs* pairs,
                                              const char* private_key,
                                              const char* cert_chain);

/**
 * EXPERIMENTAL API - Subject to change
 *
 * Destroys a grpc_tls_identity_pairs object. If this object is passed to a
 * provider initiation function, the ownership is transferred so this function
 * doesn't need to be called. Otherwise the creator of the
 * grpc_tls_identity_pairs object is responsible for its destruction.
 */
GRPCAPI void grpc_tls_identity_pairs_destroy(grpc_tls_identity_pairs* pairs);

/**
 * EXPERIMENTAL API - Subject to change
 *
 * Creates a grpc_tls_certificate_provider that will load credential data from
 * static string during initialization. This provider will always return the
 * same cert data for all cert names.
 * root_certificate and pem_key_cert_pairs can be nullptr, indicating the
 * corresponding credential data is not needed.
 * This function will make a copy of |root_certificate|.
 * The ownership of |pem_key_cert_pairs| is transferred.
 */
GRPCAPI grpc_tls_certificate_provider*
grpc_tls_certificate_provider_static_data_create(
    const char* root_certificate, grpc_tls_identity_pairs* pem_key_cert_pairs);

/**
 * EXPERIMENTAL API - Subject to change
 *
 * Creates a grpc_tls_certificate_provider that will watch the credential
 * changes on the file system. This provider will always return the up-to-date
 * cert data for all the cert names callers set through
 * |grpc_tls_credentials_options|. Note that this API only supports one key-cert
 * file and hence one set of identity key-cert pair, so SNI(Server Name
 * Indication) is not supported.
 * - private_key_path is the file path of the private key. This must be set if
 *   |identity_certificate_path| is set. Otherwise, it could be null if no
 *   identity credentials are needed.
 * - identity_certificate_path is the file path of the identity certificate
 *   chain. This must be set if |private_key_path| is set. Otherwise, it could
 *   be null if no identity credentials are needed.
 * - root_cert_path is the file path to the root certificate bundle. This
 *   may be null if no root certs are needed.
 * - refresh_interval_sec is the refreshing interval that we will check the
 *   files for updates.
 * It does not take ownership of parameters.
 */
GRPCAPI grpc_tls_certificate_provider*
grpc_tls_certificate_provider_file_watcher_create(
    const char* private_key_path, const char* identity_certificate_path,
    const char* root_cert_path, unsigned int refresh_interval_sec);

/**
 * EXPERIMENTAL API - Subject to change
 *
 * Releases a grpc_tls_certificate_provider object. The creator of the
 * grpc_tls_certificate_provider object is responsible for its release.
 */
GRPCAPI void grpc_tls_certificate_provider_release(
    grpc_tls_certificate_provider* provider);

/**
 * EXPERIMENTAL API - Subject to change
 *
 * The read-only request information exposed in a verification call.
 * Callers should not directly manage the ownership of it. We will make sure it
 * is always available inside verify() or cancel() call, and will destroy the
 * object at the end of custom verification.
 */
typedef struct grpc_tls_custom_verification_check_request {
  /* The target name of the server when the client initiates the connection. */
  /* This field will be nullptr if on the server side. */
  const char* target_name;
  /* The information contained in the certificate chain sent from the peer. */
  struct peer_info {
    /* The Common Name field on the peer leaf certificate. */
    const char* common_name;
    /* The list of Subject Alternative Names on the peer leaf certificate. */
    struct san_names {
      char** uri_names;
      size_t uri_names_size;
      char** dns_names;
      size_t dns_names_size;
      char** email_names;
      size_t email_names_size;
      char** ip_names;
      size_t ip_names_size;
    } san_names;
    /* The raw peer leaf certificate. */
    const char* peer_cert;
    /* The raw peer certificate chain. Note that it is not always guaranteed to
     * get the peer full chain. For more, please refer to
     * GRPC_X509_PEM_CERT_CHAIN_PROPERTY_NAME defined in file
     * grpc_security_constants.h.
     * TODO(ZhenLian): Consider fixing this in the future. */
    const char* peer_cert_full_chain;
    /* The verified root cert subject.
     * This value will only be filled if the cryptographic peer certificate
     * verification was successful */
    const char* verified_root_cert_subject;
  } peer_info;
} grpc_tls_custom_verification_check_request;

/**
 * EXPERIMENTAL API - Subject to change
 *
 * A callback function provided by gRPC as a parameter of the |verify| function
 * in grpc_tls_certificate_verifier_external. If |verify| is expected to be run
 * asynchronously, the implementer of |verify| will need to invoke this callback
 * with |callback_arg| and proper verification status at the end to bring the
 * control back to gRPC C core.
 */
typedef void (*grpc_tls_on_custom_verification_check_done_cb)(
    grpc_tls_custom_verification_check_request* request, void* callback_arg,
    grpc_status_code status, const char* error_details);

/**
 * EXPERIMENTAL API - Subject to change
 *
 * The internal verifier type that will be used inside core.
 */
typedef struct grpc_tls_certificate_verifier grpc_tls_certificate_verifier;

/**
 * EXPERIMENTAL API - Subject to change
 *
 * A struct containing all the necessary functions a custom external verifier
 * needs to implement to be able to be converted to an internal verifier.
 */
typedef struct grpc_tls_certificate_verifier_external {
  void* user_data;
  /**
   * A function pointer containing the verification logic that will be
   * performed after the TLS handshake is done. It could be processed
   * synchronously or asynchronously.
   * - If expected to be processed synchronously, the implementer should
   *   populate the verification result through |sync_status| and
   *   |sync_error_details|, and then return true.
   * - If expected to be processed asynchronously, the implementer should return
   *   false immediately, and then in the asynchronous thread invoke |callback|
   *   with the verification result. The implementer MUST NOT invoke the async
   *   |callback| in the same thread before |verify| returns, otherwise it can
   *   lead to deadlocks.
   *
   * user_data: any argument that is passed in the user_data of
   *            grpc_tls_certificate_verifier_external during construction time
   *            can be retrieved later here.
   * request: request information exposed to the function implementer.
   * callback: the callback that the function implementer needs to invoke, if
   *           return a non-zero value. It is usually invoked when the
   *           asynchronous verification is done, and serves to bring the
   *           control back to gRPC.
   * callback_arg: A pointer to the internal ExternalVerifier instance. This is
   *               mainly used as an argument in |callback|, if want to invoke
   *               |callback| in async mode.
   * sync_status: indicates if a connection should be allowed. This should only
   *              be used if the verification check is done synchronously.
   * sync_error_details: the error generated while verifying a connection. This
   *                     should only be used if the verification check is done
   *                     synchronously. the implementation must allocate the
   *                     error string via gpr_malloc() or gpr_strdup().
   * return: return 0 if |verify| is expected to be executed asynchronously,
   *         otherwise return a non-zero value.
   */
  int (*verify)(void* user_data,
                grpc_tls_custom_verification_check_request* request,
                grpc_tls_on_custom_verification_check_done_cb callback,
                void* callback_arg, grpc_status_code* sync_status,
                char** sync_error_details);
  /**
   * A function pointer that cleans up the caller-specified resources when the
   * verifier is still running but the whole connection got cancelled. This
   * could happen when the verifier is doing some async operations, and the
   * whole handshaker object got destroyed because of connection time limit is
   * reached, or any other reasons. In such cases, function implementers might
   * want to be notified, and properly clean up some resources.
   *
   * user_data: any argument that is passed in the user_data of
   *            grpc_tls_certificate_verifier_external during construction time
   *            can be retrieved later here.
   * request: request information exposed to the function implementer. It will
   *          be the same request object that was passed to verify(), and it
   *          tells the cancel() which request to cancel.
   */
  void (*cancel)(void* user_data,
                 grpc_tls_custom_verification_check_request* request);
  /**
   * A function pointer that does some additional destruction work when the
   * verifier is destroyed. This is used when the caller wants to associate some
   * objects to the lifetime of external_verifier, and destroy them when
   * external_verifier got destructed. For example, in C++, the class containing
   * user-specified callback functions should not be destroyed before
   * external_verifier, since external_verifier will invoke them while being
   * used.
   * Note that the caller MUST delete the grpc_tls_certificate_verifier_external
   * object itself in this function, otherwise it will cause memory leaks. That
   * also means the user_data has to carries at least a self pointer, for the
   * callers to later delete it in destruct().
   *
   * user_data: any argument that is passed in the user_data of
   *            grpc_tls_certificate_verifier_external during construction time
   *            can be retrieved later here.
   */
  void (*destruct)(void* user_data);
} grpc_tls_certificate_verifier_external;

/**
 * EXPERIMENTAL API - Subject to change
 *
 * Converts an external verifier to an internal verifier.
 * Note that we will not take the ownership of the external_verifier. Callers
 * will need to delete external_verifier in its own destruct function.
 */
grpc_tls_certificate_verifier* grpc_tls_certificate_verifier_external_create(
    grpc_tls_certificate_verifier_external* external_verifier);

/**
 * EXPERIMENTAL API - Subject to change
 *
 * Factory function for an internal verifier that won't perform any
 * post-handshake verification. Note: using this solely without any other
 * authentication mechanisms on the peer identity will leave your applications
 * to the MITM(Man-In-The-Middle) attacks. Users should avoid doing so in
 * production environments.
 */
grpc_tls_certificate_verifier* grpc_tls_certificate_verifier_no_op_create();

/**
 * EXPERIMENTAL API - Subject to change
 *
 * Factory function for an internal verifier that will do the default hostname
 * check.
 */
grpc_tls_certificate_verifier* grpc_tls_certificate_verifier_host_name_create();

/**
 * EXPERIMENTAL API - Subject to change
 *
 * Releases a grpc_tls_certificate_verifier object. The creator of the
 * grpc_tls_certificate_verifier object is responsible for its release.
 */
void grpc_tls_certificate_verifier_release(
    grpc_tls_certificate_verifier* verifier);

/**
 * EXPERIMENTAL API - Subject to change
 *
 * Performs the verification logic of an internal verifier.
 * This is typically used when composing the internal verifiers as part of the
 * custom verification.
 * If |grpc_tls_certificate_verifier_verify| returns true, inspect the
 * verification result through request->status and request->error_details.
 * Otherwise, inspect through the parameter of |callback|.
 */
int grpc_tls_certificate_verifier_verify(
    grpc_tls_certificate_verifier* verifier,
    grpc_tls_custom_verification_check_request* request,
    grpc_tls_on_custom_verification_check_done_cb callback, void* callback_arg,
    grpc_status_code* sync_status, char** sync_error_details);

/**
 * EXPERIMENTAL API - Subject to change
 *
 * Performs the cancellation logic of an internal verifier.
 * This is typically used when composing the internal verifiers as part of the
 * custom verification.
 */
void grpc_tls_certificate_verifier_cancel(
    grpc_tls_certificate_verifier* verifier,
    grpc_tls_custom_verification_check_request* request);

/**
 * EXPERIMENTAL API - Subject to change
 *
 * Creates an grpc_tls_credentials_options.
 */
GRPCAPI grpc_tls_credentials_options* grpc_tls_credentials_options_create(void);

/**
 * EXPERIMENTAL API - Subject to change
 *
 * Creates a TLS channel credential object based on the
 * grpc_tls_credentials_options specified by callers. The
 * grpc_channel_credentials will take the ownership of the |options|. The
 * security level of the resulting connection is GRPC_PRIVACY_AND_INTEGRITY.
 */
grpc_channel_credentials* grpc_tls_credentials_create(
    grpc_tls_credentials_options* options);

/**
 * EXPERIMENTAL API - Subject to change
 *
 * Creates a TLS server credential object based on the
 * grpc_tls_credentials_options specified by callers. The
 * grpc_server_credentials will take the ownership of the |options|.
 */
grpc_server_credentials* grpc_tls_server_credentials_create(
    grpc_tls_credentials_options* options);

/**
 * EXPERIMENTAL API - Subject to change
 *
 * Sets the minimum TLS version that will be negotiated during the TLS
 * handshake. If not set, the underlying SSL library will set it to TLS v1.2.
 */
GRPCAPI void grpc_tls_credentials_options_set_min_tls_version(
    grpc_tls_credentials_options* options, grpc_tls_version min_tls_version);

/**
 * EXPERIMENTAL API - Subject to change
 *
 * Sets the maximum TLS version that will be negotiated during the TLS
 * handshake. If not set, the underlying SSL library will set it to TLS v1.3.
 */
GRPCAPI void grpc_tls_credentials_options_set_max_tls_version(
    grpc_tls_credentials_options* options, grpc_tls_version max_tls_version);

/**
 * EXPERIMENTAL API - Subject to change
 *
 * Copies a grpc_tls_credentials_options.
 */
GRPCAPI grpc_tls_credentials_options* grpc_tls_credentials_options_copy(
    grpc_tls_credentials_options* options);

/**
 * EXPERIMENTAL API - Subject to change
 *
 * Destroys a grpc_tls_credentials_options.
 */
GRPCAPI void grpc_tls_credentials_options_destroy(
    grpc_tls_credentials_options* options);

/**
 * EXPERIMENTAL API - Subject to change
 *
 * A struct provides ways to gain credential data that will be used in the TLS
 * handshake.
 */
typedef struct grpc_tls_certificate_provider grpc_tls_certificate_provider;

/**
 * EXPERIMENTAL API - Subject to change
 *
 * Sets the credential provider in the options.
 * The |options| will implicitly take a new ref to the |provider|.
 */
GRPCAPI void grpc_tls_credentials_options_set_certificate_provider(
    grpc_tls_credentials_options* options,
    grpc_tls_certificate_provider* provider);

/**
 * EXPERIMENTAL API - Subject to change
 *
 * If set, gRPC stack will keep watching the root certificates with
 * name |root_cert_name|.
 * If this is not set on the client side, we will use the root certificates
 * stored in the default system location, since client side must provide root
 * certificates in TLS.
 * If this is not set on the server side, we will not watch any root certificate
 * updates, and assume no root certificates needed for the server(single-side
 * TLS). Default root certs on the server side is not supported.
 */
GRPCAPI void grpc_tls_credentials_options_watch_root_certs(
    grpc_tls_credentials_options* options);

/**
 * EXPERIMENTAL API - Subject to change
 *
 * Sets the name of the root certificates being watched.
 * If not set, We will use a default empty string as the root certificate name.
 */
GRPCAPI void grpc_tls_credentials_options_set_root_cert_name(
    grpc_tls_credentials_options* options, const char* root_cert_name);

/**
 * EXPERIMENTAL API - Subject to change
 *
 * If set, gRPC stack will keep watching the identity key-cert pairs
 * with name |identity_cert_name|.
 * This is required on the server side, and optional on the client side.
 */
GRPCAPI void grpc_tls_credentials_options_watch_identity_key_cert_pairs(
    grpc_tls_credentials_options* options);

/**
 * EXPERIMENTAL API - Subject to change
 *
 * Sets the name of the identity certificates being watched.
 * If not set, We will use a default empty string as the identity certificate
 * name.
 */
GRPCAPI void grpc_tls_credentials_options_set_identity_cert_name(
    grpc_tls_credentials_options* options, const char* identity_cert_name);

/**
 * EXPERIMENTAL API - Subject to change
 *
 * Sets the options of whether to request and/or verify client certs. This shall
 * only be called on the server side.
 */
GRPCAPI void grpc_tls_credentials_options_set_cert_request_type(
    grpc_tls_credentials_options* options,
    grpc_ssl_client_certificate_request_type type);

/** Deprecated in favor of grpc_tls_credentials_options_set_crl_provider. The
 * crl provider interface provides a significantly more flexible approach to
 * using CRLs. See gRFC A69 for details.
 * EXPERIMENTAL API - Subject to change
 *
 * If set, gRPC will read all hashed x.509 CRL files in the directory and
 * enforce the CRL files on all TLS handshakes. Only supported for OpenSSL
 * version > 1.1.
 * It is used for experimental purpose for now and subject to change.
 */
GRPCAPI void grpc_tls_credentials_options_set_crl_directory(
    grpc_tls_credentials_options* options, const char* crl_directory);

/**
 * EXPERIMENTAL API - Subject to change
 *
 * Sets the options of whether to verify server certs on the client side.
 * Passing in a non-zero value indicates verifying the certs.
 */
GRPCAPI void grpc_tls_credentials_options_set_verify_server_cert(
    grpc_tls_credentials_options* options, int verify_server_cert);

/**
 * EXPERIMENTAL API - Subject to change
 *
 * Sets whether or not a TLS server should send a list of CA names in the
 * ServerHello. This list of CA names is read from the server's trust bundle, so
 * that the client can use this list as a hint to know which certificate it
 * should send to the server.
 *
 * WARNING: This API is extremely dangerous and should not be used. If the
 * server's trust bundle is too large, then the TLS server will be unable to
 * form a ServerHello, and hence will be unusable. The definition of "too large"
 * depends on the underlying SSL library being used and on the size of the CN
 * fields of the certificates in the trust bundle.
 */
GRPCAPI void grpc_tls_credentials_options_set_send_client_ca_list(
    grpc_tls_credentials_options* options, bool send_client_ca_list);

/** --- SSL Session Cache. ---

    A SSL session cache object represents a way to cache client sessions
    between connections. Only ticket-based resumption is supported. */

typedef struct grpc_ssl_session_cache grpc_ssl_session_cache;

/** Create LRU cache for client-side SSL sessions with the given capacity.
    If capacity is < 1, a default capacity is used instead. */
GRPCAPI grpc_ssl_session_cache* grpc_ssl_session_cache_create_lru(
    size_t capacity);

/** Destroy SSL session cache. */
GRPCAPI void grpc_ssl_session_cache_destroy(grpc_ssl_session_cache* cache);

/** Create a channel arg with the given cache object. */
GRPCAPI grpc_arg
grpc_ssl_session_cache_create_channel_arg(grpc_ssl_session_cache* cache);

/** Callback for getting the SSL roots override from the application.
   In case of success, *pem_roots_certs must be set to a NULL terminated string
   containing the list of PEM encoded root certificates. The ownership is passed
   to the core and freed (laster by the core) with gpr_free.
   If this function fails and GRPC_DEFAULT_SSL_ROOTS_FILE_PATH environment is
   set to a valid path, it will override the roots specified this func */
typedef grpc_ssl_roots_override_result (*grpc_ssl_roots_override_callback)(
    char** pem_root_certs);

/** Setup a callback to override the default TLS/SSL roots.
   This function is not thread-safe and must be called at initialization time
   before any ssl credentials are created to have the desired side effect.
   If GRPC_DEFAULT_SSL_ROOTS_FILE_PATH environment is set to a valid path, the
   callback will not be called. */
GRPCAPI void grpc_set_ssl_roots_override_callback(
    grpc_ssl_roots_override_callback cb);

GRPCAPI gpr_timespec grpc_max_auth_token_lifetime(void);

/** --- insecure credentials --- */

/**
 * EXPERIMENTAL API - Subject to change
 *
 * This method creates an insecure channel credentials object.
 */
GRPCAPI grpc_channel_credentials* grpc_insecure_credentials_create();

/**
 * EXPERIMENTAL API - Subject to change
 *
 * This method creates an insecure server credentials object.
 */
GRPCAPI grpc_server_credentials* grpc_insecure_server_credentials_create();

/**
 * EXPERIMENTAL API - Subject to change
 *
 * This method creates an xDS channel credentials object.
 *
 * Creating a channel with credentials of this type indicates that the channel
 * should get credentials configuration from the xDS control plane.
 *
 * \a fallback_credentials are used if the channel target does not have the
 * 'xds:///' scheme or if the xDS control plane does not provide information on
 * how to fetch credentials dynamically. Does NOT take ownership of the \a
 * fallback_credentials. (Internally takes a ref to the object.)
 */
GRPCAPI grpc_channel_credentials* grpc_xds_credentials_create(
    grpc_channel_credentials* fallback_credentials);

/**
 * EXPERIMENTAL API - Subject to change
 *
 * This method creates an xDS server credentials object.
 *
 * \a fallback_credentials are used if the xDS control plane does not provide
 * information on how to fetch credentials dynamically.
 *
 * Does NOT take ownership of the \a fallback_credentials. (Internally takes
 * a ref to the object.)
 */
GRPCAPI grpc_server_credentials* grpc_xds_server_credentials_create(
    grpc_server_credentials* fallback_credentials);

/** --- Local channel/server credentials --- **/

/**
 * This method creates a local channel credential object. The security level
 * of the resulting connection is GRPC_PRIVACY_AND_INTEGRITY for UDS and
 * GRPC_SECURITY_NONE for LOCAL_TCP. It is used for experimental purpose
 * for now and subject to change.
 *
 * - type: local connection type
 *
 * It returns the created local channel credential object.
 */
GRPCAPI grpc_channel_credentials* grpc_local_credentials_create(
    grpc_local_connect_type type);

/**
 * This method creates a local server credential object. It is used for
 * experimental purpose for now and subject to change.
 *
 * - type: local connection type
 *
 * It returns the created local server credential object.
 */
GRPCAPI grpc_server_credentials* grpc_local_server_credentials_create(
    grpc_local_connect_type type);

/**
 * EXPERIMENTAL API - Subject to change
 *
 * The internal verifier type that will be used inside core.
 */
typedef struct grpc_tls_certificate_verifier grpc_tls_certificate_verifier;

/**
 * EXPERIMENTAL API - Subject to change
 *
 * Sets the verifier in options. The |options| will implicitly take a new ref to
 * the |verifier|. If not set on the client side, we will verify server's
 * certificates, and check the default hostname. If not set on the server side,
 * we will verify client's certificates.
 */
void grpc_tls_credentials_options_set_certificate_verifier(
    grpc_tls_credentials_options* options,
    grpc_tls_certificate_verifier* verifier);

/**
 * EXPERIMENTAL API - Subject to change
 *
 * Sets the options of whether to check the hostname of the peer on a per-call
 * basis. This is usually used in a combination with virtual hosting at the
 * client side, where each individual call on a channel can have a different
 * host associated with it.
 * This check is intended to verify that the host specified for the individual
 * call is covered by the cert that the peer presented.
 * The default is a non-zero value, which indicates performing such checks.
 */
GRPCAPI void grpc_tls_credentials_options_set_check_call_host(
    grpc_tls_credentials_options* options, int check_call_host);

/** --- TLS session key logging. ---
 * Experimental API to control tls session key logging. Tls session key logging
 * is expected to be used only for debugging purposes and never in production.
 * Tls session key logging is only enabled when:
 *  At least one grpc_tls_credentials_options object is assigned a tls session
 *  key logging file path using the API specified below.
 */

/**
 * EXPERIMENTAL API - Subject to change.
 * Configures a grpc_tls_credentials_options object with tls session key
 * logging capability. TLS channels using these credentials have tls session
 * key logging enabled.
 * - options is the grpc_tls_credentials_options object
 * - path is a string pointing to the location where TLS session keys would be
 *   stored.
 */
GRPCAPI void grpc_tls_credentials_options_set_tls_session_key_log_file_path(
    grpc_tls_credentials_options* options, const char* path);

#ifdef __cplusplus
}
#endif

#endif /* GRPC_CREDENTIALS_H */
