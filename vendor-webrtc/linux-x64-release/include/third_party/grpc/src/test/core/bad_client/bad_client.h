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

#ifndef GRPC_TEST_CORE_BAD_CLIENT_BAD_CLIENT_H
#define GRPC_TEST_CORE_BAD_CLIENT_BAD_CLIENT_H

#include <grpc/grpc.h>
#include <grpc/slice.h>
#include <stddef.h>
#include <stdint.h>

#define GRPC_BAD_CLIENT_REGISTERED_METHOD "/registered/bar"
#define GRPC_BAD_CLIENT_REGISTERED_HOST "localhost"

// The server side validator function to run
typedef void (*grpc_bad_client_server_side_validator)(grpc_server* server,
                                                      grpc_completion_queue* cq,
                                                      void* registered_method);

// Returns false if we need to read more data.
typedef bool (*grpc_bad_client_client_stream_validator)(
    grpc_slice_buffer* incoming, void* arg);

struct grpc_bad_client_arg {
  grpc_bad_client_client_stream_validator client_validator;
  void* client_validator_arg;
  const char* client_payload;
  size_t client_payload_length;
};

// Flags for grpc_run_bad_client_test
#define GRPC_BAD_CLIENT_DISCONNECT 1
#define GRPC_BAD_CLIENT_LARGE_REQUEST 2
#define GRPC_BAD_CLIENT_MAX_CONCURRENT_REQUESTS_OF_ONE 4

// Test runner.
//
// Create a server, and for each arg in \a args send client_payload. For each
// payload, run client_validator to make sure that the response is as expected.
// Also execute \a server_validator in a separate thread to assert that the
// bytes are handled as expected.
//
// The flags are only applicable to the last validator in the array. (This can
// be changed in the future if necessary)
//
void grpc_run_bad_client_test(
    grpc_bad_client_server_side_validator server_validator,
    grpc_bad_client_arg args[], int num_args, uint32_t flags);

// A hack to let old tests work as before. In these tests, instead of an array,
// the tests provide a single client_validator and payload
//
#define COMBINE1(X, Y) X##Y
#define COMBINE(X, Y) COMBINE1(X, Y)

#define GRPC_RUN_BAD_CLIENT_TEST(server_validator, client_validator, payload,  \
                                 flags)                                        \
  grpc_bad_client_arg COMBINE(bca, __LINE__) = {client_validator, nullptr,     \
                                                payload, sizeof(payload) - 1}; \
  grpc_run_bad_client_test(server_validator, &COMBINE(bca, __LINE__), 1, flags)

// Helper validator functions
// Client side validator for connection preface from server. \a arg is unused
bool client_connection_preface_validator(grpc_slice_buffer* incoming,
                                         void* arg);

// Client side validator for checking if reset stream is present at the end
// of the buffer. \a arg is unused.
//
bool rst_stream_client_validator(grpc_slice_buffer* incoming, void* arg);

// Helper grpc_bad_client_arg arguments for direct use
// Sends a connection preface from the client with an empty settings frame
extern grpc_bad_client_arg connection_preface_arg;

// Server side verifier function that performs a
//  single grpc_server_request_call
void server_verifier_request_call(grpc_server* server,
                                  grpc_completion_queue* cq,
                                  void* registered_method);
#endif  // GRPC_TEST_CORE_BAD_CLIENT_BAD_CLIENT_H
