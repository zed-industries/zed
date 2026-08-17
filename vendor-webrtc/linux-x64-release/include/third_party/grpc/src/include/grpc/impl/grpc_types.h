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

#ifndef GRPC_IMPL_GRPC_TYPES_H
#define GRPC_IMPL_GRPC_TYPES_H

// IWYU pragma: private, include <grpc/grpc.h>

#include <grpc/impl/channel_arg_names.h>
#include <grpc/impl/compression_types.h>
#include <grpc/slice.h>
#include <grpc/status.h>
#include <grpc/support/port_platform.h>
#include <grpc/support/time.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
  GRPC_BB_RAW
  /** Future types may include GRPC_BB_PROTOBUF, etc. */
} grpc_byte_buffer_type;

typedef struct grpc_byte_buffer {
  void* reserved;
  grpc_byte_buffer_type type;
  union grpc_byte_buffer_data {
    struct /* internal */ {
      void* reserved[8];
    } reserved;
    struct grpc_compressed_buffer {
      grpc_compression_algorithm compression;
      grpc_slice_buffer slice_buffer;
    } raw;
  } data;
} grpc_byte_buffer;

/** Completion Queues enable notification of the completion of
 * asynchronous actions. */
typedef struct grpc_completion_queue grpc_completion_queue;

/** The Channel interface allows creation of Call objects. */
typedef struct grpc_channel grpc_channel;

/** A server listens to some port and responds to request calls */
typedef struct grpc_server grpc_server;

/** A Call represents an RPC. When created, it is in a configuration state
    allowing properties to be set until it is invoked. After invoke, the Call
    can have messages written to it and read from it. */
typedef struct grpc_call grpc_call;

/** The Socket Mutator interface allows changes on socket options */
typedef struct grpc_socket_mutator grpc_socket_mutator;

/** The Socket Factory interface creates and binds sockets */
typedef struct grpc_socket_factory grpc_socket_factory;

/** Type specifier for grpc_arg */
typedef enum {
  GRPC_ARG_STRING,
  GRPC_ARG_INTEGER,
  GRPC_ARG_POINTER
} grpc_arg_type;

typedef struct grpc_arg_pointer_vtable {
  void* (*copy)(void* p);
  void (*destroy)(void* p);
  int (*cmp)(void* p, void* q);
} grpc_arg_pointer_vtable;

/** A single argument... each argument has a key and a value

    A note on naming keys:
      Keys are namespaced into groups, usually grouped by library, and are
      keys for module XYZ are named XYZ.key1, XYZ.key2, etc. Module names must
      be restricted to the regex [A-Za-z][_A-Za-z0-9]{,15}.
      Key names must be restricted to the regex [A-Za-z][_A-Za-z0-9]{,47}.

    GRPC core library keys are prefixed by grpc.

    Library authors are strongly encouraged to \#define symbolic constants for
    their keys so that it's possible to change them in the future. */
typedef struct {
  grpc_arg_type type;
  char* key;
  union grpc_arg_value {
    char* string;
    int integer;
    struct grpc_arg_pointer {
      void* p;
      const grpc_arg_pointer_vtable* vtable;
    } pointer;
  } value;
} grpc_arg;

/** An array of arguments that can be passed around.

    Used to set optional channel-level configuration.
    These configuration options are modelled as key-value pairs as defined
    by grpc_arg; keys are strings to allow easy backwards-compatible extension
    by arbitrary parties. All evaluation is performed at channel creation
    time (i.e. the keys and values in this structure need only live through the
    creation invocation).

    However, if one of the args has grpc_arg_type==GRPC_ARG_POINTER, then the
    grpc_arg_pointer_vtable must live until the channel args are done being
    used by core (i.e. when the object for use with which they were passed
    is destroyed).

    See the description of the \ref grpc_arg_keys "available args" for more
    details. */
typedef struct {
  size_t num_args;
  grpc_arg* args;
} grpc_channel_args;

/** Result of a grpc call. If the caller satisfies the prerequisites of a
    particular operation, the grpc_call_error returned will be GRPC_CALL_OK.
    Receiving any other value listed here is an indication of a bug in the
    caller. */
typedef enum grpc_call_error {
  /** everything went ok */
  GRPC_CALL_OK = 0,
  /** something failed, we don't know what */
  GRPC_CALL_ERROR,
  /** this method is not available on the server */
  GRPC_CALL_ERROR_NOT_ON_SERVER,
  /** this method is not available on the client */
  GRPC_CALL_ERROR_NOT_ON_CLIENT,
  /** this method must be called before server_accept */
  GRPC_CALL_ERROR_ALREADY_ACCEPTED,
  /** this method must be called before invoke */
  GRPC_CALL_ERROR_ALREADY_INVOKED,
  /** this method must be called after invoke */
  GRPC_CALL_ERROR_NOT_INVOKED,
  /** this call is already finished
      (writes_done or write_status has already been called) */
  GRPC_CALL_ERROR_ALREADY_FINISHED,
  /** there is already an outstanding read/write operation on the call */
  GRPC_CALL_ERROR_TOO_MANY_OPERATIONS,
  /** the flags value was illegal for this call */
  GRPC_CALL_ERROR_INVALID_FLAGS,
  /** invalid metadata was passed to this call */
  GRPC_CALL_ERROR_INVALID_METADATA,
  /** invalid message was passed to this call */
  GRPC_CALL_ERROR_INVALID_MESSAGE,
  /** completion queue for notification has not been registered
   * with the server */
  GRPC_CALL_ERROR_NOT_SERVER_COMPLETION_QUEUE,
  /** this batch of operations leads to more operations than allowed */
  GRPC_CALL_ERROR_BATCH_TOO_BIG,
  /** payload type requested is not the type registered */
  GRPC_CALL_ERROR_PAYLOAD_TYPE_MISMATCH,
  /** completion queue has been shutdown */
  GRPC_CALL_ERROR_COMPLETION_QUEUE_SHUTDOWN
} grpc_call_error;

/** Default send/receive message size limits in bytes. -1 for unlimited. */
/** TODO(roth) Make this match the default receive limit after next release */
#define GRPC_DEFAULT_MAX_SEND_MESSAGE_LENGTH (-1)
#define GRPC_DEFAULT_MAX_RECV_MESSAGE_LENGTH (4 * 1024 * 1024)

/** Write Flags: */
/** Hint that the write may be buffered and need not go out on the wire
    immediately. GRPC is free to buffer the message until the next non-buffered
    write, or until writes_done, but it need not buffer completely or at all. */
#define GRPC_WRITE_BUFFER_HINT (0x00000001u)
/** Force compression to be disabled for a particular write
    (start_write/add_metadata). Illegal on invoke/accept. */
#define GRPC_WRITE_NO_COMPRESS (0x00000002u)
/** Force this message to be written to the socket before completing it */
#define GRPC_WRITE_THROUGH (0x00000004u)
/** Mask of all valid flags. */
#define GRPC_WRITE_USED_MASK \
  (GRPC_WRITE_BUFFER_HINT | GRPC_WRITE_NO_COMPRESS | GRPC_WRITE_THROUGH)

/** Initial metadata flags */
/** These flags are to be passed to the `grpc_op::flags` field */
/** Signal that the call should not return UNAVAILABLE before it has started */
#define GRPC_INITIAL_METADATA_WAIT_FOR_READY (0x00000020u)
/** Signal that GRPC_INITIAL_METADATA_WAIT_FOR_READY was explicitly set
    by the calling application. */
#define GRPC_INITIAL_METADATA_WAIT_FOR_READY_EXPLICITLY_SET (0x00000080u)

/** Mask of all valid flags */
#define GRPC_INITIAL_METADATA_USED_MASK                  \
  (GRPC_INITIAL_METADATA_WAIT_FOR_READY_EXPLICITLY_SET | \
   GRPC_INITIAL_METADATA_WAIT_FOR_READY | GRPC_WRITE_THROUGH)

/** A single metadata element */
typedef struct grpc_metadata {
  /** the key, value values are expected to line up with grpc_mdelem: if
     changing them, update metadata.h at the same time. */
  grpc_slice key;
  grpc_slice value;

  /** The following fields are reserved for grpc internal use.
      There is no need to initialize them, and they will be set to garbage
      during calls to grpc. */
  struct /* internal */ {
    void* obfuscated[4];
  } internal_data;
} grpc_metadata;

/** The type of completion (for grpc_event) */
typedef enum grpc_completion_type {
  /** Shutting down */
  GRPC_QUEUE_SHUTDOWN,
  /** No event before timeout */
  GRPC_QUEUE_TIMEOUT,
  /** Operation completion */
  GRPC_OP_COMPLETE
} grpc_completion_type;

/** The result of an operation.

    Returned by a completion queue when the operation started with tag. */
typedef struct grpc_event {
  /** The type of the completion. */
  grpc_completion_type type;
  /** If the grpc_completion_type is GRPC_OP_COMPLETE, this field indicates
      whether the operation was successful or not; 0 in case of failure and
      non-zero in case of success.
      If grpc_completion_type is GRPC_QUEUE_SHUTDOWN or GRPC_QUEUE_TIMEOUT, this
      field is guaranteed to be 0 */
  int success;
  /** The tag passed to grpc_call_start_batch etc to start this operation.
      *Only* GRPC_OP_COMPLETE has a tag. For all other grpc_completion_type
      values, tag is uninitialized. */
  void* tag;
} grpc_event;

typedef struct {
  size_t count;
  size_t capacity;
  grpc_metadata* metadata;
} grpc_metadata_array;

typedef struct {
  grpc_slice method;
  grpc_slice host;
  gpr_timespec deadline;
} grpc_call_details;

typedef enum {
  /** Send initial metadata: one and only one instance MUST be sent for each
      call, unless the call was cancelled - in which case this can be skipped.
      This op completes after all bytes of metadata have been accepted by
      outgoing flow control. */
  GRPC_OP_SEND_INITIAL_METADATA = 0,
  /** Send a message: 0 or more of these operations can occur for each call.
      This op completes after all bytes for the message have been accepted by
      outgoing flow control. */
  GRPC_OP_SEND_MESSAGE,
  /** Send a close from the client: one and only one instance MUST be sent from
      the client, unless the call was cancelled - in which case this can be
      skipped. This op completes after all bytes for the call
      (including the close) have passed outgoing flow control. */
  GRPC_OP_SEND_CLOSE_FROM_CLIENT,
  /** Send status from the server: one and only one instance MUST be sent from
      the server unless the call was cancelled - in which case this can be
      skipped. This op completes after all bytes for the call
      (including the status) have passed outgoing flow control. */
  GRPC_OP_SEND_STATUS_FROM_SERVER,
  /** Receive initial metadata: one and only one MUST be made on the client,
      must not be made on the server.
      This op completes after all initial metadata has been read from the
      peer. */
  GRPC_OP_RECV_INITIAL_METADATA,
  /** Receive a message: 0 or more of these operations can occur for each call.
      This op completes after all bytes of the received message have been
      read, or after a half-close has been received on this call. */
  GRPC_OP_RECV_MESSAGE,
  /** Receive status on the client: one and only one must be made on the client.
      This operation always succeeds, meaning ops paired with this operation
      will also appear to succeed, even though they may not have. In that case
      the status will indicate some failure.
      This op completes after all activity on the call has completed. */
  GRPC_OP_RECV_STATUS_ON_CLIENT,
  /** Receive close on the server: one and only one must be made on the
      server. This op completes after the close has been received by the
      server. This operation always succeeds, meaning ops paired with
      this operation will also appear to succeed, even though they may not
      have. */
  GRPC_OP_RECV_CLOSE_ON_SERVER
} grpc_op_type;

struct grpc_byte_buffer;

/** Operation data: one field for each op type (except SEND_CLOSE_FROM_CLIENT
   which has no arguments) */
typedef struct grpc_op {
  /** Operation type, as defined by grpc_op_type */
  grpc_op_type op;
  /** Write flags bitset for grpc_begin_messages */
  uint32_t flags;
  /** Reserved for future usage */
  void* reserved;
  union grpc_op_data {
    /** Reserved for future usage */
    struct /* internal */ {
      void* reserved[8];
    } reserved;
    struct grpc_op_send_initial_metadata {
      size_t count;
      grpc_metadata* metadata;
      /** If \a is_set, \a compression_level will be used for the call.
       * Otherwise, \a compression_level won't be considered */
      struct grpc_op_send_initial_metadata_maybe_compression_level {
        uint8_t is_set;
        grpc_compression_level level;
      } maybe_compression_level;
    } send_initial_metadata;
    struct grpc_op_send_message {
      /** This op takes ownership of the slices in send_message.  After
       * a call completes, the contents of send_message are not guaranteed
       * and likely empty.  The original owner should still call
       * grpc_byte_buffer_destroy() on this object however.
       */
      struct grpc_byte_buffer* send_message;
    } send_message;
    struct grpc_op_send_status_from_server {
      size_t trailing_metadata_count;
      grpc_metadata* trailing_metadata;
      grpc_status_code status;
      /** optional: set to NULL if no details need sending, non-NULL if they do
       * pointer will not be retained past the start_batch call
       */
      grpc_slice* status_details;
    } send_status_from_server;
    /** ownership of the array is with the caller, but ownership of the elements
        stays with the call object (ie key, value members are owned by the call
        object, recv_initial_metadata->array is owned by the caller).
        After the operation completes, call grpc_metadata_array_destroy on this
        value, or reuse it in a future op. */
    struct grpc_op_recv_initial_metadata {
      grpc_metadata_array* recv_initial_metadata;
    } recv_initial_metadata;
    /** ownership of the byte buffer is moved to the caller; the caller must
        call grpc_byte_buffer_destroy on this value, or reuse it in a future op.
        The returned byte buffer will be NULL if trailing metadata was
        received instead of a message.
       */
    struct grpc_op_recv_message {
      struct grpc_byte_buffer** recv_message;
    } recv_message;
    struct grpc_op_recv_status_on_client {
      /** ownership of the array is with the caller, but ownership of the
          elements stays with the call object (ie key, value members are owned
          by the call object, trailing_metadata->array is owned by the caller).
          After the operation completes, call grpc_metadata_array_destroy on
          this value, or reuse it in a future op. */
      grpc_metadata_array* trailing_metadata;
      grpc_status_code* status;
      grpc_slice* status_details;
      /** If this is not nullptr, it will be populated with the full fidelity
       * error string for debugging purposes. The application is responsible
       * for freeing the data by using gpr_free(). */
      const char** error_string;
    } recv_status_on_client;
    struct grpc_op_recv_close_on_server {
      /** out argument, set to 1 if the call failed at the server for
          a reason other than a non-OK status (cancel, deadline
          exceeded, network failure, etc.), 0 otherwise (RPC processing ran to
          completion and was able to provide any status from the server) */
      int* cancelled;
    } recv_close_on_server;
  } data;
} grpc_op;

/** Information requested from the channel. */
typedef struct {
  /** If non-NULL, will be set to point to a string indicating the LB
   * policy name.  Caller takes ownership. */
  char** lb_policy_name;
  /** If non-NULL, will be set to point to a string containing the
   * service config used by the channel in JSON form. */
  char** service_config_json;
} grpc_channel_info;

typedef struct grpc_resource_quota grpc_resource_quota;

/** Completion queues internally MAY maintain a set of file descriptors in a
    structure called 'pollset'. This enum specifies if a completion queue has an
    associated pollset and any restrictions on the type of file descriptors that
    can be present in the pollset.

    I/O progress can only be made when grpc_completion_queue_next() or
    grpc_completion_queue_pluck() are called on the completion queue (unless the
    grpc_cq_polling_type is GRPC_CQ_NON_POLLING) and hence it is very important
    to actively call these APIs */
typedef enum {
  /** The completion queue will have an associated pollset and there is no
      restriction on the type of file descriptors the pollset may contain */
  GRPC_CQ_DEFAULT_POLLING,

  /** Similar to GRPC_CQ_DEFAULT_POLLING except that the completion queues will
      not contain any 'listening file descriptors' (i.e file descriptors used to
      listen to incoming channels) */
  GRPC_CQ_NON_LISTENING,

  /** The completion queue will not have an associated pollset. Note that
      grpc_completion_queue_next() or grpc_completion_queue_pluck() MUST still
      be called to pop events from the completion queue; it is not required to
      call them actively to make I/O progress */
  GRPC_CQ_NON_POLLING
} grpc_cq_polling_type;

/** Specifies the type of APIs to use to pop events from the completion queue */
typedef enum {
  /** Events are popped out by calling grpc_completion_queue_next() API ONLY */
  GRPC_CQ_NEXT,

  /** Events are popped out by calling grpc_completion_queue_pluck() API ONLY*/
  GRPC_CQ_PLUCK,

  /** Events trigger a callback specified as the tag */
  GRPC_CQ_CALLBACK
} grpc_cq_completion_type;

/** Specifies an interface class to be used as a tag for callback-based
 * completion queues. This can be used directly, as the first element of a
 * struct in C, or as a base class in C++. Its "run" value should be assigned to
 * some non-member function, such as a static method. */
typedef struct grpc_completion_queue_functor {
  /** The run member specifies a function that will be called when this
      tag is extracted from the completion queue. Its arguments will be a
      pointer to this functor and a boolean that indicates whether the
      operation succeeded (non-zero) or failed (zero) */
  void (*functor_run)(struct grpc_completion_queue_functor*, int);

  /** The inlineable member specifies whether this functor can be run inline.
      This should only be used for trivial internally-defined functors. */
  int inlineable;

  /** The following fields are not API. They are meant for internal use. */
  int internal_success;
  struct grpc_completion_queue_functor* internal_next;
} grpc_completion_queue_functor;

#define GRPC_CQ_CURRENT_VERSION 2
#define GRPC_CQ_VERSION_MINIMUM_FOR_CALLBACKABLE 2
typedef struct grpc_completion_queue_attributes {
  /** The version number of this structure. More fields might be added to this
     structure in future. */
  int version; /** Set to GRPC_CQ_CURRENT_VERSION */

  grpc_cq_completion_type cq_completion_type;

  grpc_cq_polling_type cq_polling_type;

  /* END OF VERSION 1 CQ ATTRIBUTES */

  /* START OF VERSION 2 CQ ATTRIBUTES */
  /** When creating a callbackable CQ, pass in a functor to get invoked when
   * shutdown is complete */
  grpc_completion_queue_functor* cq_shutdown_cb;

  /* END OF VERSION 2 CQ ATTRIBUTES */
} grpc_completion_queue_attributes;

/** The completion queue factory structure is opaque to the callers of grpc */
typedef struct grpc_completion_queue_factory grpc_completion_queue_factory;

#ifdef __cplusplus
}
#endif

#endif /* GRPC_IMPL_GRPC_TYPES_H */
