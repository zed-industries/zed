/*
 *
 * Copyright 2015-2016 gRPC authors.
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

#ifndef GRPC_GRPC_H
#define GRPC_GRPC_H

#include <grpc/byte_buffer.h>
#include <grpc/impl/connectivity_state.h>  // IWYU pragma: export
#include <grpc/impl/grpc_types.h>          // IWYU pragma: export
#include <grpc/impl/propagation_bits.h>
#include <grpc/slice.h>
#include <grpc/status.h>
#include <grpc/support/port_platform.h>
#include <grpc/support/time.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/*! \mainpage GRPC Core
 *
 * The GRPC Core library is a low-level library designed to be wrapped by higher
 * level libraries. The top-level API is provided in grpc.h. Security related
 * functionality lives in grpc_security.h.
 */

GRPCAPI void grpc_metadata_array_init(grpc_metadata_array* array);
GRPCAPI void grpc_metadata_array_destroy(grpc_metadata_array* array);

GRPCAPI void grpc_call_details_init(grpc_call_details* details);
GRPCAPI void grpc_call_details_destroy(grpc_call_details* details);

/** Initialize the grpc library.

    After it's called, a matching invocation to grpc_shutdown() is expected.

    It is not safe to call any other grpc functions before calling this.
    (To avoid overhead, little checking is done, and some things may work. We
    do not warrant that they will continue to do so in future revisions of this
    library). */
GRPCAPI void grpc_init(void);

/** Shut down the grpc library.

    Before it's called, there should haven been a matching invocation to
    grpc_init().

    The last call to grpc_shutdown will initiate cleaning up of grpc library
    internals, which can happen in another thread. Once the clean-up is done,
    no memory is used by grpc, nor are any instructions executing within the
    grpc library.  Prior to calling, all application owned grpc objects must
    have been destroyed. */
GRPCAPI void grpc_shutdown(void);

/** EXPERIMENTAL. Returns 1 if the grpc library has been initialized.
    TODO(ericgribkoff) Decide if this should be promoted to non-experimental as
    part of stabilizing the fork support API, as tracked in
    https://github.com/grpc/grpc/issues/15334 */
GRPCAPI int grpc_is_initialized(void);

/** DEPRECATED. Recommend to use grpc_shutdown only */
GRPCAPI void grpc_shutdown_blocking(void);

/** Return a string representing the current version of grpc */
GRPCAPI const char* grpc_version_string(void);

/** Return a string specifying what the 'g' in gRPC stands for */
GRPCAPI const char* grpc_g_stands_for(void);

/** Returns the completion queue factory based on the attributes. MAY return a
    NULL if no factory can be found */
GRPCAPI const grpc_completion_queue_factory*
grpc_completion_queue_factory_lookup(
    const grpc_completion_queue_attributes* attributes);

/** Helper function to create a completion queue with grpc_cq_completion_type
    of GRPC_CQ_NEXT and grpc_cq_polling_type of GRPC_CQ_DEFAULT_POLLING */
GRPCAPI grpc_completion_queue* grpc_completion_queue_create_for_next(
    void* reserved);

/** Helper function to create a completion queue with grpc_cq_completion_type
    of GRPC_CQ_PLUCK and grpc_cq_polling_type of GRPC_CQ_DEFAULT_POLLING */
GRPCAPI grpc_completion_queue* grpc_completion_queue_create_for_pluck(
    void* reserved);

/** Helper function to create a completion queue with grpc_cq_completion_type
    of GRPC_CQ_CALLBACK and grpc_cq_polling_type of GRPC_CQ_DEFAULT_POLLING.
    This function is experimental. */
GRPCAPI grpc_completion_queue* grpc_completion_queue_create_for_callback(
    grpc_completion_queue_functor* shutdown_callback, void* reserved);

/** Create a completion queue */
GRPCAPI grpc_completion_queue* grpc_completion_queue_create(
    const grpc_completion_queue_factory* factory,
    const grpc_completion_queue_attributes* attributes, void* reserved);

/** Blocks until an event is available, the completion queue is being shut down,
    or deadline is reached.

    Returns a grpc_event with type GRPC_QUEUE_TIMEOUT on timeout,
    otherwise a grpc_event describing the event that occurred.

    Callers must not call grpc_completion_queue_next and
    grpc_completion_queue_pluck simultaneously on the same completion queue. */
GRPCAPI grpc_event grpc_completion_queue_next(grpc_completion_queue* cq,
                                              gpr_timespec deadline,
                                              void* reserved);

/** Blocks until an event with tag 'tag' is available, the completion queue is
    being shutdown or deadline is reached.

    Returns a grpc_event with type GRPC_QUEUE_TIMEOUT on timeout,
    otherwise a grpc_event describing the event that occurred.

    Callers must not call grpc_completion_queue_next and
    grpc_completion_queue_pluck simultaneously on the same completion queue.

    Completion queues support a maximum of GRPC_MAX_COMPLETION_QUEUE_PLUCKERS
    concurrently executing plucks at any time. */
GRPCAPI grpc_event grpc_completion_queue_pluck(grpc_completion_queue* cq,
                                               void* tag, gpr_timespec deadline,
                                               void* reserved);

/** Maximum number of outstanding grpc_completion_queue_pluck executions per
    completion queue */
#define GRPC_MAX_COMPLETION_QUEUE_PLUCKERS 6

/** Begin destruction of a completion queue. Once all possible events are
    drained then grpc_completion_queue_next will start to produce
    GRPC_QUEUE_SHUTDOWN events only. At that point it's safe to call
    grpc_completion_queue_destroy.

    After calling this function applications should ensure that no
    NEW work is added to be published on this completion queue. */
GRPCAPI void grpc_completion_queue_shutdown(grpc_completion_queue* cq);

/** Destroy a completion queue. The caller must ensure that the queue is
    drained and no threads are executing grpc_completion_queue_next */
GRPCAPI void grpc_completion_queue_destroy(grpc_completion_queue* cq);

/*********** EXPERIMENTAL API ************/
/** Initializes a thread local cache for \a cq.
 * grpc_flush_cq_tls_cache() MUST be called on the same thread,
 * with the same cq.
 */
GRPCAPI void grpc_completion_queue_thread_local_cache_init(
    grpc_completion_queue* cq);

/*********** EXPERIMENTAL API ************/
/** Flushes the thread local cache for \a cq.
 * Returns 1 if there was contents in the cache.  If there was an event
 * in \a cq tls cache, its tag is placed in tag, and ok is set to the
 * event success.
 */
GRPCAPI int grpc_completion_queue_thread_local_cache_flush(
    grpc_completion_queue* cq, void** tag, int* ok);

/** Check the connectivity state of a channel. */
GRPCAPI grpc_connectivity_state grpc_channel_check_connectivity_state(
    grpc_channel* channel, int try_to_connect);

/** Watch for a change in connectivity state.
    Once the channel connectivity state is different from last_observed_state,
    tag will be enqueued on cq with success=1.
    If deadline expires BEFORE the state is changed, tag will be enqueued on cq
    with success=0. */
GRPCAPI void grpc_channel_watch_connectivity_state(
    grpc_channel* channel, grpc_connectivity_state last_observed_state,
    gpr_timespec deadline, grpc_completion_queue* cq, void* tag);

/** Check whether a grpc channel supports connectivity watcher */
GRPCAPI int grpc_channel_support_connectivity_watcher(grpc_channel* channel);

/** Create a call given a grpc_channel, in order to call 'method'. All
    completions are sent to 'completion_queue'. 'method' and 'host' need only
    live through the invocation of this function.
    If parent_call is non-NULL, it must be a server-side call. It will be used
    to propagate properties from the server call to this new client call,
    depending on the value of \a propagation_mask (see propagation_bits.h for
    possible values). */
GRPCAPI grpc_call* grpc_channel_create_call(
    grpc_channel* channel, grpc_call* parent_call, uint32_t propagation_mask,
    grpc_completion_queue* completion_queue, grpc_slice method,
    const grpc_slice* host, gpr_timespec deadline, void* reserved);

/** Pre-register a method/host pair on a channel.
    method and host are not owned and must remain alive while the channel is
    alive. */
GRPCAPI void* grpc_channel_register_call(grpc_channel* channel,
                                         const char* method, const char* host,
                                         void* reserved);

/** Create a call given a handle returned from grpc_channel_register_call.
    \sa grpc_channel_create_call. */
GRPCAPI grpc_call* grpc_channel_create_registered_call(
    grpc_channel* channel, grpc_call* parent_call, uint32_t propagation_mask,
    grpc_completion_queue* completion_queue, void* registered_call_handle,
    gpr_timespec deadline, void* reserved);

/** Allocate memory in the grpc_call arena: this memory is automatically
    discarded at call completion */
GRPCAPI void* grpc_call_arena_alloc(grpc_call* call, size_t size);

/** Start a batch of operations defined in the array ops; when complete, post a
    completion of type 'tag' to the completion queue bound to the call.
    The order of ops specified in the batch has no significance.
    Only one operation of each type can be active at once in any given
    batch.
    If a call to grpc_call_start_batch returns GRPC_CALL_OK you must call
    grpc_completion_queue_next or grpc_completion_queue_pluck on the completion
    queue associated with 'call' for work to be performed. If a call to
    grpc_call_start_batch returns any value other than GRPC_CALL_OK it is
    guaranteed that no state associated with 'call' is changed and it is not
    appropriate to call grpc_completion_queue_next or
    grpc_completion_queue_pluck consequent to the failed grpc_call_start_batch
    call.
    If a call to grpc_call_start_batch with an empty batch returns
    GRPC_CALL_OK, the tag is put in the completion queue immediately.
    THREAD SAFETY: access to grpc_call_start_batch in multi-threaded environment
    needs to be synchronized. As an optimization, you may synchronize batches
    containing just send operations independently from batches containing just
    receive operations. Access to grpc_call_start_batch with an empty batch is
    thread-compatible. */
GRPCAPI grpc_call_error grpc_call_start_batch(grpc_call* call,
                                              const grpc_op* ops, size_t nops,
                                              void* tag, void* reserved);

/** Returns a newly allocated string representing the endpoint to which this
    call is communicating with. The string is in the uri format accepted by
    grpc_channel_create.
    The returned string should be disposed of with gpr_free().

    WARNING: this value is never authenticated or subject to any security
    related code. It must not be used for any authentication related
    functionality. Instead, use grpc_auth_context. */
GRPCAPI char* grpc_call_get_peer(grpc_call* call);

struct census_context;

/** Set census context for a call; Must be called before first call to
   grpc_call_start_batch(). */
GRPCAPI void grpc_census_call_set_context(grpc_call* call,
                                          struct census_context* context);

/** Retrieve the calls current census context. */
GRPCAPI struct census_context* grpc_census_call_get_context(grpc_call* call);

/** Return a newly allocated string representing the target a channel was
    created for. */
GRPCAPI char* grpc_channel_get_target(grpc_channel* channel);

/** Request info about the channel.
    \a channel_info indicates what information is being requested and
    how that information will be returned.
    \a channel_info is owned by the caller. */
GRPCAPI void grpc_channel_get_info(grpc_channel* channel,
                                   const grpc_channel_info* channel_info);

/** EXPERIMENTAL.  Resets the channel's connect backoff.
    TODO(roth): When we see whether this proves useful, either promote
    to non-experimental or remove it. */
GRPCAPI void grpc_channel_reset_connect_backoff(grpc_channel* channel);

/** --- grpc_channel_credentials object. ---

   A channel credentials object represents a way to authenticate a client on a
   channel. Different types of channel credentials are declared in
   grpc_security.h. */

typedef struct grpc_channel_credentials grpc_channel_credentials;

/** Releases a channel credentials object.
   The creator of the credentials object is responsible for its release. */

GRPCAPI void grpc_channel_credentials_release(grpc_channel_credentials* creds);

/** --- grpc_server_credentials object. ---

   A server credentials object represents a way to authenticate a server.
   Different types of server credentials are declared in grpc_security.h. */

typedef struct grpc_server_credentials grpc_server_credentials;

/** Releases a server_credentials object.
   The creator of the server_credentials object is responsible for its release.
   */
GRPCAPI void grpc_server_credentials_release(grpc_server_credentials* creds);

/** Creates a secure channel using the passed-in credentials. Additional
    channel level configuration MAY be provided by grpc_channel_args, though
    the expectation is that most clients will want to simply pass NULL. The
    user data in 'args' need only live through the invocation of this function.
    However, if any args of the 'pointer' type are passed, then the referenced
    vtable must be maintained by the caller until grpc_channel_destroy
    terminates. See grpc_channel_args definition for more on this. */
GRPCAPI grpc_channel* grpc_channel_create(const char* target,
                                          grpc_channel_credentials* creds,
                                          const grpc_channel_args* args);

/** Create a lame client: this client fails every operation attempted on it. */
GRPCAPI grpc_channel* grpc_lame_client_channel_create(
    const char* target, grpc_status_code error_code, const char* error_message);

/** Close and destroy a grpc channel */
GRPCAPI void grpc_channel_destroy(grpc_channel* channel);

/** Error handling for grpc_call
   Most grpc_call functions return a grpc_error. If the error is not GRPC_OK
   then the operation failed due to some unsatisfied precondition.
   If a grpc_call fails, it's guaranteed that no change to the call state
   has been made. */

/** Cancel an RPC.
    Can be called multiple times, from any thread.
    THREAD-SAFETY grpc_call_cancel and grpc_call_cancel_with_status
    are thread-safe, and can be called at any point before grpc_call_unref
    is called.*/
GRPCAPI grpc_call_error grpc_call_cancel(grpc_call* call, void* reserved);

/** Cancel an RPC.
    Can be called multiple times, from any thread.
    If a status has not been received for the call, set it to the status code
    and description passed in.
    Importantly, this function does not send status nor description to the
    remote endpoint.
    Note that \a description doesn't need be a static string.
    It doesn't need to be alive after the call to
    grpc_call_cancel_with_status completes.
    */
GRPCAPI grpc_call_error grpc_call_cancel_with_status(grpc_call* call,
                                                     grpc_status_code status,
                                                     const char* description,
                                                     void* reserved);

/* Returns whether or not the call's receive message operation failed because of
 * an error (as opposed to a graceful end-of-stream) */
GRPCAPI int grpc_call_failed_before_recv_message(const grpc_call* c);

/** Ref a call.
    THREAD SAFETY: grpc_call_ref is thread-compatible */
GRPCAPI void grpc_call_ref(grpc_call* call);

/** Unref a call.
    THREAD SAFETY: grpc_call_unref is thread-compatible */
GRPCAPI void grpc_call_unref(grpc_call* call);

typedef struct grpc_call_credentials grpc_call_credentials;

/** Sets a credentials to a call. Can only be called on the client side before
   grpc_call_start_batch. */
GRPCAPI grpc_call_error grpc_call_set_credentials(grpc_call* call,
                                                  grpc_call_credentials* creds);

/** Request notification of a new call.
    Once a call is received, a notification tagged with \a tag_new is added to
    \a cq_for_notification. \a call, \a details and \a request_metadata are
    updated with the appropriate call information. \a cq_bound_to_call is bound
    to \a call, and batch operation notifications for that call will be posted
    to \a cq_bound_to_call.
    Note that \a cq_for_notification must have been registered to the server via
    \a grpc_server_register_completion_queue. */
GRPCAPI grpc_call_error grpc_server_request_call(
    grpc_server* server, grpc_call** call, grpc_call_details* details,
    grpc_metadata_array* request_metadata,
    grpc_completion_queue* cq_bound_to_call,
    grpc_completion_queue* cq_for_notification, void* tag_new);

/** How to handle payloads for a registered method */
typedef enum {
  /** Don't try to read the payload */
  GRPC_SRM_PAYLOAD_NONE,
  /** Read the initial payload as a byte buffer */
  GRPC_SRM_PAYLOAD_READ_INITIAL_BYTE_BUFFER
} grpc_server_register_method_payload_handling;

/** Registers a method in the server.
    Methods to this (host, method) pair will not be reported by
    grpc_server_request_call, but instead be reported by
    grpc_server_request_registered_call when passed the appropriate
    registered_method (as returned by this function).
    Must be called before grpc_server_start.
    Returns NULL on failure. */
GRPCAPI void* grpc_server_register_method(
    grpc_server* server, const char* method, const char* host,
    grpc_server_register_method_payload_handling payload_handling,
    uint32_t flags);

/** Request notification of a new pre-registered call. 'cq_for_notification'
    must have been registered to the server via
    grpc_server_register_completion_queue. */
GRPCAPI grpc_call_error grpc_server_request_registered_call(
    grpc_server* server, void* registered_method, grpc_call** call,
    gpr_timespec* deadline, grpc_metadata_array* request_metadata,
    grpc_byte_buffer** optional_payload,
    grpc_completion_queue* cq_bound_to_call,
    grpc_completion_queue* cq_for_notification, void* tag_new);

/** Create a server. Additional configuration for each incoming channel can
    be specified with args. If no additional configuration is needed, args can
    be NULL. The user data in 'args' need only live through the invocation of
    this function. However, if any args of the 'pointer' type are passed, then
    the referenced vtable must be maintained by the caller until
    grpc_server_destroy terminates. See grpc_channel_args definition for more
    on this. */
GRPCAPI grpc_server* grpc_server_create(const grpc_channel_args* args,
                                        void* reserved);

/** Register a completion queue with the server. Must be done for any
    notification completion queue that is passed to grpc_server_request_*_call
    and to grpc_server_shutdown_and_notify. Must be performed prior to
    grpc_server_start. */
GRPCAPI void grpc_server_register_completion_queue(grpc_server* server,
                                                   grpc_completion_queue* cq,
                                                   void* reserved);

// More members might be added in later, so users should take care to memset
// this to 0 before using it.
typedef struct {
  grpc_status_code code;
  const char* error_message;
} grpc_serving_status_update;

// There might be more methods added later, so users should take care to memset
// this to 0 before using it.
typedef struct {
  void (*on_serving_status_update)(void* user_data, const char* uri,
                                   grpc_serving_status_update update);
  void* user_data;
} grpc_server_xds_status_notifier;

typedef struct grpc_server_config_fetcher grpc_server_config_fetcher;

/** EXPERIMENTAL.  Creates an xDS config fetcher. */
GRPCAPI grpc_server_config_fetcher* grpc_server_config_fetcher_xds_create(
    grpc_server_xds_status_notifier notifier, const grpc_channel_args* args);

/** EXPERIMENTAL.  Destroys a config fetcher. */
GRPCAPI void grpc_server_config_fetcher_destroy(
    grpc_server_config_fetcher* config_fetcher);

/** EXPERIMENTAL.  Sets the server's config fetcher.  Takes ownership.
    Must be called before adding ports */
GRPCAPI void grpc_server_set_config_fetcher(
    grpc_server* server, grpc_server_config_fetcher* config_fetcher);

/** Add a HTTP2 over an encrypted link over tcp listener.
   Returns bound port number on success, 0 on failure.
   REQUIRES: server not started */
GRPCAPI int grpc_server_add_http2_port(grpc_server* server, const char* addr,
                                       grpc_server_credentials* creds);

/** Start a server - tells all listeners to start listening */
GRPCAPI void grpc_server_start(grpc_server* server);

/** Begin shutting down a server.
    After completion, no new calls or connections will be admitted.
    Existing calls will be allowed to complete.
    Send a GRPC_OP_COMPLETE event when there are no more calls being serviced.
    Shutdown is idempotent, and all tags will be notified at once if multiple
    grpc_server_shutdown_and_notify calls are made. 'cq' must have been
    registered to this server via grpc_server_register_completion_queue. */
GRPCAPI void grpc_server_shutdown_and_notify(grpc_server* server,
                                             grpc_completion_queue* cq,
                                             void* tag);

/** Cancel all in-progress calls.
    Only usable after shutdown. */
GRPCAPI void grpc_server_cancel_all_calls(grpc_server* server);

/** Destroy a server.
    Shutdown must have completed beforehand (i.e. all tags generated by
    grpc_server_shutdown_and_notify must have been received, and at least
    one call to grpc_server_shutdown_and_notify must have been made). */
GRPCAPI void grpc_server_destroy(grpc_server* server);

/** Enable or disable a tracer.

    Tracers (usually controlled by the environment variable GRPC_TRACE)
    allow printf-style debugging on GRPC internals, and are useful for
    tracking down problems in the field.

    Use of this function is not strictly thread-safe, but the
    thread-safety issues raised by it should not be of concern. */
GRPCAPI int grpc_tracer_set_enabled(const char* name, int enabled);

/** Check whether a metadata key is legal (will be accepted by core) */
GRPCAPI int grpc_header_key_is_legal(grpc_slice slice);

/** Check whether a non-binary metadata value is legal (will be accepted by
    core) */
GRPCAPI int grpc_header_nonbin_value_is_legal(grpc_slice slice);

/** Check whether a metadata key corresponds to a binary value */
GRPCAPI int grpc_is_binary_header(grpc_slice slice);

/** Convert grpc_call_error values to a string */
GRPCAPI const char* grpc_call_error_to_string(grpc_call_error error);

/** Create a buffer pool */
GRPCAPI grpc_resource_quota* grpc_resource_quota_create(const char* trace_name);

/** Add a reference to a buffer pool */
GRPCAPI void grpc_resource_quota_ref(grpc_resource_quota* resource_quota);

/** Drop a reference to a buffer pool */
GRPCAPI void grpc_resource_quota_unref(grpc_resource_quota* resource_quota);

/** Update the size of a buffer pool */
GRPCAPI void grpc_resource_quota_resize(grpc_resource_quota* resource_quota,
                                        size_t new_size);

/** Update the size of the maximum number of threads allowed */
GRPCAPI void grpc_resource_quota_set_max_threads(
    grpc_resource_quota* resource_quota, int new_max_threads);

/** EXPERIMENTAL.  Dumps xDS configs as a serialized ClientConfig proto.
    The full name of the proto is envoy.service.status.v3.ClientConfig. */
GRPCAPI grpc_slice grpc_dump_xds_configs(void);

/** Fetch a vtable for a grpc_channel_arg that points to a grpc_resource_quota
 */
GRPCAPI const grpc_arg_pointer_vtable* grpc_resource_quota_arg_vtable(void);

/************* CHANNELZ API *************/
/** Channelz is under active development. The following APIs will see some
    churn as the feature is implemented. This comment will be removed once
    channelz is officially supported, and these APIs become stable. For now
    you may track the progress by following this github issue:
    https://github.com/grpc/grpc/issues/15340

    the following APIs return allocated JSON strings that match the response
    objects from the channelz proto, found here:
    https://github.com/grpc/grpc/blob/master/src/proto/grpc/channelz/channelz.proto.

    For easy conversion to protobuf, The JSON is formatted according to:
    https://developers.google.com/protocol-buffers/docs/proto3#json. */

/* Gets all root channels (i.e. channels the application has directly
   created). This does not include subchannels nor non-top level channels.
   The returned string is allocated and must be freed by the application. */
GRPCAPI char* grpc_channelz_get_top_channels(intptr_t start_channel_id);

/* Gets all servers that exist in the process. */
GRPCAPI char* grpc_channelz_get_servers(intptr_t start_server_id);

/* Returns a single Server, or else a NOT_FOUND code. */
GRPCAPI char* grpc_channelz_get_server(intptr_t server_id);

/* Gets all server sockets that exist in the server. */
GRPCAPI char* grpc_channelz_get_server_sockets(intptr_t server_id,
                                               intptr_t start_socket_id,
                                               intptr_t max_results);

/* Returns a single Channel, or else a NOT_FOUND code. The returned string
   is allocated and must be freed by the application. */
GRPCAPI char* grpc_channelz_get_channel(intptr_t channel_id);

/* Returns a single Subchannel, or else a NOT_FOUND code. The returned string
   is allocated and must be freed by the application. */
GRPCAPI char* grpc_channelz_get_subchannel(intptr_t subchannel_id);

/* Returns a single Socket, or else a NOT_FOUND code. The returned string
   is allocated and must be freed by the application. */
GRPCAPI char* grpc_channelz_get_socket(intptr_t socket_id);

/**
 * EXPERIMENTAL - Subject to change.
 * Fetch a vtable for grpc_channel_arg that points to
 * grpc_authorization_policy_provider.
 */
GRPCAPI const grpc_arg_pointer_vtable*
grpc_authorization_policy_provider_arg_vtable(void);

#ifdef __cplusplus
}
#endif

#endif /* GRPC_GRPC_H */
