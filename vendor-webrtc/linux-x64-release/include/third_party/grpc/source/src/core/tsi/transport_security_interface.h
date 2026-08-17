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

#ifndef GRPC_SRC_CORE_TSI_TRANSPORT_SECURITY_INTERFACE_H
#define GRPC_SRC_CORE_TSI_TRANSPORT_SECURITY_INTERFACE_H

#include <grpc/support/port_platform.h>
#include <stdint.h>
#include <stdlib.h>

#include <string>

#include "src/core/lib/debug/trace.h"

// --- tsi result ---

typedef enum {
  TSI_OK = 0,
  TSI_UNKNOWN_ERROR = 1,
  TSI_INVALID_ARGUMENT = 2,
  TSI_PERMISSION_DENIED = 3,
  TSI_INCOMPLETE_DATA = 4,
  TSI_FAILED_PRECONDITION = 5,
  TSI_UNIMPLEMENTED = 6,
  TSI_INTERNAL_ERROR = 7,
  TSI_DATA_CORRUPTED = 8,
  TSI_NOT_FOUND = 9,
  TSI_PROTOCOL_FAILURE = 10,
  TSI_HANDSHAKE_IN_PROGRESS = 11,
  TSI_OUT_OF_RESOURCES = 12,
  TSI_ASYNC = 13,
  TSI_HANDSHAKE_SHUTDOWN = 14,
  TSI_CLOSE_NOTIFY = 15,  // Indicates that the connection should be closed.
  TSI_DRAIN_BUFFER = 16,  // Indicates that the buffer used to store handshake
                          // data should be drained.
} tsi_result;

typedef enum {
  TSI_SECURITY_MIN,
  TSI_SECURITY_NONE = TSI_SECURITY_MIN,
  TSI_INTEGRITY_ONLY,
  TSI_PRIVACY_AND_INTEGRITY,
  TSI_SECURITY_MAX = TSI_PRIVACY_AND_INTEGRITY,
} tsi_security_level;

typedef enum {
  // Default option
  TSI_DONT_REQUEST_CLIENT_CERTIFICATE,
  TSI_REQUEST_CLIENT_CERTIFICATE_BUT_DONT_VERIFY,
  TSI_REQUEST_CLIENT_CERTIFICATE_AND_VERIFY,
  TSI_REQUEST_AND_REQUIRE_CLIENT_CERTIFICATE_BUT_DONT_VERIFY,
  TSI_REQUEST_AND_REQUIRE_CLIENT_CERTIFICATE_AND_VERIFY,
} tsi_client_certificate_request_type;

typedef enum {
  // TSI implementation provides a normal frame protector.  The caller
  // should invoke tsi_handshaker_result_create_frame_protector() to
  // generate the frame protector.
  TSI_FRAME_PROTECTOR_NORMAL,
  // TSI implementation provides a zero-copy frame protector.  The caller
  // should invoke tsi_handshaker_result_create_zero_copy_grpc_protector()
  // to generate the frame protector.
  TSI_FRAME_PROTECTOR_ZERO_COPY,
  // TSI implementation provides both normal and zero-copy frame protectors.
  // The caller should invoke either
  // tsi_handshaker_result_create_frame_protector() or
  // tsi_handshaker_result_create_zero_copy_grpc_protector() to generate
  // the frame protector.
  TSI_FRAME_PROTECTOR_NORMAL_OR_ZERO_COPY,
  // TSI implementation does not provide any frame protector.  This means
  // that it is safe for the caller to send bytes unprotected on the wire.
  TSI_FRAME_PROTECTOR_NONE,
} tsi_frame_protector_type;

typedef enum {
  TSI_TLS1_2,
  TSI_TLS1_3,
} tsi_tls_version;

const char* tsi_result_to_string(tsi_result result);
const char* tsi_security_level_to_string(tsi_security_level security_level);

// -- tsi_zero_copy_grpc_protector object --

// This object protects and unprotects grpc slice buffers with zero or minimized
// memory copy once the handshake is done. Implementations of this object must
// be thread compatible. This object depends on grpc and the details of this
// object is defined in transport_security_grpc.h.

typedef struct tsi_zero_copy_grpc_protector tsi_zero_copy_grpc_protector;

// --- tsi_frame_protector object ---

// This object protects and unprotects buffers once the handshake is done.
// Implementations of this object must be thread compatible.

typedef struct tsi_frame_protector tsi_frame_protector;

// Outputs protected frames.
// - unprotected_bytes is an input only parameter and points to the data
//   to be protected.
// - unprotected_bytes_size is an input/output parameter used by the caller to
//   specify how many bytes are available in unprotected_bytes. The output
//   value is the number of bytes consumed during the call.
// - protected_output_frames points to a buffer allocated by the caller that
//   will be written.
// - protected_output_frames_size is an input/output parameter used by the
//   caller to specify how many bytes are available in protected_output_frames.
//   As an output, this value indicates the number of bytes written.
// - This method returns TSI_OK in case of success or a specific error code in
//   case of failure. Note that even if all the input unprotected bytes are
//   consumed, they may not have been processed into the returned protected
//   output frames. The caller should call the protect_flush method
//   to make sure that there are no more protected bytes buffered in the
//   protector.

// A typical way to call this method would be:

// ------------------------------------------------------------------------
// unsigned char protected_buffer[4096];
// size_t protected_buffer_size = sizeof(protected_buffer);
// tsi_result result = TSI_OK;
// while (message_size > 0) {
//   size_t protected_buffer_size_to_send = protected_buffer_size;
//   size_t processed_message_size = message_size;
//   result = tsi_frame_protector_protect(protector,
//                                        message_bytes,
//                                        &processed_message_size,
//                                        protected_buffer,
//                                        &protected_buffer_size_to_send);
//   if (result != TSI_OK) break;
//   send_bytes_to_peer(protected_buffer, protected_buffer_size_to_send);
//   message_bytes += processed_message_size;
//   message_size -= processed_message_size;

//   // Don't forget to flush.
//   if (message_size == 0) {
//     size_t still_pending_size;
//     do {
//       protected_buffer_size_to_send = protected_buffer_size;
//       result = tsi_frame_protector_protect_flush(
//           protector, protected_buffer,
//           &protected_buffer_size_to_send, &still_pending_size);
//       if (result != TSI_OK) break;
//       send_bytes_to_peer(protected_buffer, protected_buffer_size_to_send);
//     } while (still_pending_size > 0);
//   }
// }

// if (result != TSI_OK) HandleError(result);
// ------------------------------------------------------------------------
tsi_result tsi_frame_protector_protect(tsi_frame_protector* self,
                                       const unsigned char* unprotected_bytes,
                                       size_t* unprotected_bytes_size,
                                       unsigned char* protected_output_frames,
                                       size_t* protected_output_frames_size);

// Indicates that we need to flush the bytes buffered in the protector and get
// the resulting frame.
// - protected_output_frames points to a buffer allocated by the caller that
//   will be written.
// - protected_output_frames_size is an input/output parameter used by the
//   caller to specify how many bytes are available in protected_output_frames.
// - still_pending_bytes is an output parameter indicating the number of bytes
//   that still need to be flushed from the protector.
tsi_result tsi_frame_protector_protect_flush(
    tsi_frame_protector* self, unsigned char* protected_output_frames,
    size_t* protected_output_frames_size, size_t* still_pending_size);

// Outputs unprotected bytes.
// - protected_frames_bytes is an input only parameter and points to the
//   protected frames to be unprotected.
// - protected_frames_bytes_size is an input/output only parameter used by the
//   caller to specify how many bytes are available in protected_bytes. The
//   output value is the number of bytes consumed during the call.
//   Implementations will buffer up to a frame of protected data.
// - unprotected_bytes points to a buffer allocated by the caller that will be
//   written.
// - unprotected_bytes_size is an input/output parameter used by the caller to
//   specify how many bytes are available in unprotected_bytes. This
//   value is expected to be at most max_protected_frame_size minus overhead
//   which means that max_protected_frame_size is a safe bet. The output value
//   is the number of bytes actually written.
//   If *unprotected_bytes_size is unchanged, there may be more data remaining
//   to unprotect, and the caller should call this function again.

// - This method returns TSI_OK in case of success. Success includes cases where
//   there is not enough data to output a frame in which case
//   unprotected_bytes_size will be set to 0 and cases where the internal buffer
//   needs to be read before new protected data can be processed in which case
//   protected_frames_size will be set to 0.
tsi_result tsi_frame_protector_unprotect(
    tsi_frame_protector* self, const unsigned char* protected_frames_bytes,
    size_t* protected_frames_bytes_size, unsigned char* unprotected_bytes,
    size_t* unprotected_bytes_size);

// Destroys the tsi_frame_protector object.
void tsi_frame_protector_destroy(tsi_frame_protector* self);

// --- tsi_peer objects ---

// tsi_peer objects are a set of properties. The peer owns the properties.

// This property is of type TSI_PEER_PROPERTY_STRING.
#define TSI_CERTIFICATE_TYPE_PEER_PROPERTY "certificate_type"

// This property represents security level of a channel.
#define TSI_SECURITY_LEVEL_PEER_PROPERTY "security_level"

// Property values may contain NULL characters just like C++ strings.
// The length field gives the length of the string.
typedef struct tsi_peer_property {
  char* name;
  struct {
    char* data;
    size_t length;
  } value;
} tsi_peer_property;

struct tsi_peer {
  tsi_peer_property* properties;
  size_t property_count;
};
// Destructs the tsi_peer object.
void tsi_peer_destruct(tsi_peer* self);

//  --- tsi_handshaker_result object ---

// This object contains all necessary handshake results and data such as peer
// info, negotiated keys, unused handshake bytes, when the handshake completes.
// Implementations of this object must be thread compatible.

typedef struct tsi_handshaker_result tsi_handshaker_result;

// This method extracts tsi peer. It returns TSI_OK assuming there is no fatal
// error.
// The caller is responsible for destructing the peer.
tsi_result tsi_handshaker_result_extract_peer(const tsi_handshaker_result* self,
                                              tsi_peer* peer);

// This method indicates what type of frame protector is provided by the
// TSI implementation.
tsi_result tsi_handshaker_result_get_frame_protector_type(
    const tsi_handshaker_result* self,
    tsi_frame_protector_type* frame_protector_type);

// This method creates a tsi_frame_protector object. It returns TSI_OK assuming
// there is no fatal error.
// The caller is responsible for destroying the protector.
tsi_result tsi_handshaker_result_create_frame_protector(
    const tsi_handshaker_result* self, size_t* max_output_protected_frame_size,
    tsi_frame_protector** protector);

// This method returns the unused bytes from the handshake. It returns TSI_OK
// assuming there is no fatal error.
// Ownership of the bytes is retained by the handshaker result. As a
// consequence, the caller must not free the bytes.
tsi_result tsi_handshaker_result_get_unused_bytes(
    const tsi_handshaker_result* self, const unsigned char** bytes,
    size_t* bytes_size);

// This method releases the tsi_handshaker_handshaker object. After this method
// is called, no other method can be called on the object.
void tsi_handshaker_result_destroy(tsi_handshaker_result* self);

// --- tsi_handshaker objects ----

// Implementations of this object must be thread compatible.

// ------------------------------------------------------------------------

// A typical usage supporting both synchronous and asynchronous TSI handshaker
// implementations would be:

// ------------------------------------------------------------------------

// typedef struct {
//   tsi_handshaker *handshaker;
//   tsi_handshaker_result *handshaker_result;
//   unsigned char *handshake_buffer;
//   size_t handshake_buffer_size;
//   ...
// } security_handshaker;

// void do_handshake(security_handshaker *h, ...) {
//   // Start the handshake by the calling do_handshake_next.
//   do_handshake_next(h, NULL, 0);
//   ...
// }

// // This method is the callback function when data is received from the
// // peer. This method will read bytes into the handshake buffer and call
// // do_handshake_next.
// void on_handshake_data_received_from_peer(void *user_data) {
//   security_handshaker *h = (security_handshaker *)user_data;
//   size_t bytes_received_size = h->handshake_buffer_size;
//   read_bytes_from_peer(h->handshake_buffer, &bytes_received_size);
//   do_handshake_next(h, h->handshake_buffer, bytes_received_size);
// }

// // This method processes a step of handshake, calling tsi_handshaker_next.
// void do_handshake_next(security_handshaker *h,
//                        const unsigned char* bytes_received,
//                        size_t bytes_received_size) {
//   tsi_result status = TSI_OK;
//   unsigned char *bytes_to_send = NULL;
//   size_t bytes_to_send_size = 0;
//   tsi_handshaker_result *result = NULL;
//   status = tsi_handshaker_next(
//       handshaker, bytes_received, bytes_received_size, &bytes_to_send,
//       &bytes_to_send_size, &result, on_handshake_next_done, h);
//   // If TSI handshaker is asynchronous, on_handshake_next_done will be
//   // executed inside tsi_handshaker_next.
//   if (status == TSI_ASYNC) return;
//   // If TSI handshaker is synchronous, invoke callback directly in this
//   // thread.
//   on_handshake_next_done(status, (void *)h, bytes_to_send,
//                          bytes_to_send_size, result);
// }

// // This is the callback function to execute after tsi_handshaker_next.
// // It is passed to tsi_handshaker_next as a function parameter.
// void on_handshake_next_done(
//     tsi_result status, void *user_data, const unsigned char *bytes_to_send,
//     size_t bytes_to_send_size, tsi_handshaker_result *result) {
//   security_handshaker *h = (security_handshaker *)user_data;
//   if (status == TSI_INCOMPLETE_DATA) {
//     // Schedule an asynchronous read from the peer. If handshake data are
//     // received, on_handshake_data_received_from_peer will be called.
//     async_read_from_peer(..., ..., on_handshake_data_received_from_peer);
//     return;
//   }
//   if (status != TSI_OK) return;

//   if (bytes_to_send_size > 0) {
//     send_bytes_to_peer(bytes_to_send, bytes_to_send_size);
//   }

//   if (result != NULL) {
//     // Handshake completed.
//     h->result = result;
//     // Check the Peer.
//     tsi_peer peer;
//     status = tsi_handshaker_result_extract_peer(result, &peer);
//     if (status != TSI_OK) return;
//     status = check_peer(&peer);
//     tsi_peer_destruct(&peer);
//     if (status != TSI_OK) return;

//     // Create the protector.
//     tsi_frame_protector* protector = NULL;
//     status = tsi_handshaker_result_create_frame_protector(result, NULL,
//                                                           &protector);
//     if (status != TSI_OK) return;

//     // Do not forget to unprotect outstanding data if any.
//     ....
//   }
// }
// ------------------------------------------------------------------------
typedef struct tsi_handshaker tsi_handshaker;

// TODO(jiangtaoli2016): Cleans up deprecated methods when we are ready.

// TO BE DEPRECATED SOON. Use tsi_handshaker_next instead.
// Gets bytes that need to be sent to the peer.
// - bytes is the buffer that will be written with the data to be sent to the
//   peer.
// - bytes_size is an input/output parameter specifying the capacity of the
//   bytes parameter as input and the number of bytes written as output.
// Returns TSI_OK if all the data to send to the peer has been written or if
// nothing has to be sent to the peer (in which base bytes_size outputs to 0),
// otherwise returns TSI_INCOMPLETE_DATA which indicates that this method
// needs to be called again to get all the bytes to send to the peer (there
// was more data to write than the specified bytes_size). In case of a fatal
// error in the handshake, another specific error code is returned.
tsi_result tsi_handshaker_get_bytes_to_send_to_peer(tsi_handshaker* self,
                                                    unsigned char* bytes,
                                                    size_t* bytes_size);

// TO BE DEPRECATED SOON. Use tsi_handshaker_next instead.
// Processes bytes received from the peer.
// - bytes is the buffer containing the data.
// - bytes_size is an input/output parameter specifying the size of the data as
//   input and the number of bytes consumed as output.
// Return TSI_OK if the handshake has all the data it needs to process,
// otherwise return TSI_INCOMPLETE_DATA which indicates that this method
// needs to be called again to complete the data needed for processing. In
// case of a fatal error in the handshake, another specific error code is
// returned.
tsi_result tsi_handshaker_process_bytes_from_peer(tsi_handshaker* self,
                                                  const unsigned char* bytes,
                                                  size_t* bytes_size);

// TO BE DEPRECATED SOON.
// Gets the result of the handshaker.
// Returns TSI_OK if the handshake completed successfully and there has been no
// errors. Returns TSI_HANDSHAKE_IN_PROGRESS if the handshaker is not done yet
// but no error has been encountered so far. Otherwise the handshaker failed
// with the returned error.
tsi_result tsi_handshaker_get_result(tsi_handshaker* self);

// TO BE DEPRECATED SOON.
// Returns 1 if the handshake is in progress, 0 otherwise.
#define tsi_handshaker_is_in_progress(h) \
  (tsi_handshaker_get_result((h)) == TSI_HANDSHAKE_IN_PROGRESS)

// TO BE DEPRECATED SOON. Use tsi_handshaker_result_extract_peer instead.
// This method may return TSI_FAILED_PRECONDITION if
// tsi_handshaker_is_in_progress returns 1, it returns TSI_OK otherwise
// assuming the handshaker is not in a fatal error state.
// The caller is responsible for destructing the peer.
tsi_result tsi_handshaker_extract_peer(tsi_handshaker* self, tsi_peer* peer);

// TO BE DEPRECATED SOON. Use tsi_handshaker_result_create_frame_protector
// instead.
// This method creates a tsi_frame_protector object after the handshake phase
// is done. After this method has been called successfully, the only method
// that can be called on this object is Destroy.
// - max_output_protected_frame_size is an input/output parameter specifying the
//   desired max output protected frame size as input and outputting the actual
//   max output frame size as the output. Passing NULL is OK and will result in
//   the implementation choosing the default maximum protected frame size. Note
//   that this size only applies to outgoing frames (generated with
//   tsi_frame_protector_protect) and not incoming frames (input of
//   tsi_frame_protector_unprotect).
// - protector is an output parameter pointing to the newly created
//   tsi_frame_protector object.
// This method may return TSI_FAILED_PRECONDITION if
// tsi_handshaker_is_in_progress returns 1, it returns TSI_OK otherwise assuming
// the handshaker is not in a fatal error state.
// The caller is responsible for destroying the protector.
tsi_result tsi_handshaker_create_frame_protector(
    tsi_handshaker* self, size_t* max_output_protected_frame_size,
    tsi_frame_protector** protector);

// Callback function definition for tsi_handshaker_next.
// - status indicates the status of the next operation.
// - user_data is the argument to callback function passed from the caller.
// - bytes_to_send is the data buffer to be sent to the peer.
// - bytes_to_send_size is the size of data buffer to be sent to the peer.
// - handshaker_result is the result of handshake when the handshake completes,
//   is NULL otherwise.
typedef void (*tsi_handshaker_on_next_done_cb)(
    tsi_result status, void* user_data, const unsigned char* bytes_to_send,
    size_t bytes_to_send_size, tsi_handshaker_result* handshaker_result);

// Conduct a next step of the handshake.
// - received_bytes is the buffer containing the data received from the peer.
// - received_bytes_size is the size of the data received from the peer.
// - bytes_to_send is the data buffer to be sent to the peer.
// - bytes_to_send_size is the size of data buffer to be sent to the peer.
// - handshaker_result is the result of handshake if the handshake completes.
// - cb is the callback function defined above. It can be NULL for synchronous
//   TSI handshaker implementation.
// - user_data is the argument to callback function passed from the caller.
// - error, if non-null, will be populated with a human-readable error
//   message whenever the result value is something other than TSI_OK,
//   TSI_ASYNC, or TSI_INCOMPLETE_DATA.  The object pointed to by this
//   argument is owned by the caller and must continue to exist until after the
//   handshake is finished.  Some TSI implementations cache this value,
//   so callers must pass the same value to all calls to tsi_handshaker_next()
//   for a given handshake.
// This method returns TSI_ASYNC if the TSI handshaker implementation is
// asynchronous, and in this case, the callback is guaranteed to run in another
// thread owned by TSI. It returns TSI_OK if the handshake completes or if
// there are data to send to the peer, otherwise returns TSI_INCOMPLETE_DATA
// which indicates that this method needs to be called again with more data
// from the peer. In case of a fatal error in the handshake, another specific
// error code is returned.
// The caller is responsible for destroying the handshaker_result. However,
// the caller should not free bytes_to_send, as the buffer is owned by the
// tsi_handshaker object.
tsi_result tsi_handshaker_next(tsi_handshaker* self,
                               const unsigned char* received_bytes,
                               size_t received_bytes_size,
                               const unsigned char** bytes_to_send,
                               size_t* bytes_to_send_size,
                               tsi_handshaker_result** handshaker_result,
                               tsi_handshaker_on_next_done_cb cb,
                               void* user_data, std::string* error = nullptr);

// This method shuts down a TSI handshake that is in progress.
//
// This method will be invoked when TSI handshake should be terminated before
// being finished in order to free any resources being used.
//
void tsi_handshaker_shutdown(tsi_handshaker* self);

// This method releases the tsi_handshaker object. After this method is called,
// no other method can be called on the object.
void tsi_handshaker_destroy(tsi_handshaker* self);

// This method initializes the necessary shared objects used for tsi
// implementation.
void tsi_init();

// This method destroys the shared objects created by tsi_init.
void tsi_destroy();

#endif  // GRPC_SRC_CORE_TSI_TRANSPORT_SECURITY_INTERFACE_H
