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

#ifndef GRPC_SRC_CORE_UTIL_HTTP_CLIENT_PARSER_H
#define GRPC_SRC_CORE_UTIL_HTTP_CLIENT_PARSER_H

#include <grpc/slice.h>
#include <grpc/support/port_platform.h>
#include <stddef.h>
#include <stdint.h>

#include "src/core/lib/debug/trace.h"
#include "src/core/lib/iomgr/error.h"

// Maximum length of a header string of the form 'Key: Value\r\n'
#define GRPC_HTTP_PARSER_MAX_HEADER_LENGTH 4096

// A single header to be passed in a request
typedef struct grpc_http_header {
  char* key;
  char* value;
} grpc_http_header;

typedef enum {
  GRPC_HTTP_FIRST_LINE,
  GRPC_HTTP_HEADERS,
  GRPC_HTTP_BODY,
  GRPC_HTTP_TRAILERS,
  GRPC_HTTP_END,
} grpc_http_parser_state;

typedef enum {
  GRPC_HTTP_CHUNKED_PLAIN,
  GRPC_HTTP_CHUNKED_LENGTH,
  GRPC_HTTP_CHUNKED_IGNORE_ALL_UNTIL_LF,
  GRPC_HTTP_CHUNKED_BODY,
  GRPC_HTTP_CHUNKED_CONSUME_LF,
} grpc_http_parser_chunked_state;

typedef enum {
  GRPC_HTTP_HTTP10,
  GRPC_HTTP_HTTP11,
  GRPC_HTTP_HTTP20,
} grpc_http_version;

typedef enum {
  GRPC_HTTP_RESPONSE,
  GRPC_HTTP_REQUEST,
} grpc_http_type;

// A request
typedef struct grpc_http_request {
  // Method of the request (e.g. GET, POST)
  char* method;
  // The path of the resource to fetch (only used for parsed requests)
  char* path;
  // HTTP version to use
  grpc_http_version version;
  // Headers attached to the request
  size_t hdr_count;
  grpc_http_header* hdrs;
  // Body: length and contents; contents are NOT null-terminated
  size_t body_length;
  char* body;
} grpc_http_request;

// A response
typedef struct grpc_http_response {
  // HTTP status code
  int status = 0;
  // Headers: count and key/values
  size_t hdr_count = 0;
  grpc_http_header* hdrs = nullptr;
  // Body: length and contents; contents are NOT null-terminated
  size_t body_length = 0;
  // State of the chunked parser. Only valid for the response.
  grpc_http_parser_chunked_state chunked_state = GRPC_HTTP_CHUNKED_PLAIN;
  size_t chunk_length = 0;
  char* body = nullptr;
} grpc_http_response;

struct grpc_http_parser {
  grpc_http_parser_state state;
  grpc_http_type type;

  union {
    grpc_http_response* response;
    grpc_http_request* request;
    void* request_or_response;
  } http;
  size_t body_capacity;
  size_t hdr_capacity;

  uint8_t cur_line[GRPC_HTTP_PARSER_MAX_HEADER_LENGTH];
  size_t cur_line_length;
  size_t cur_line_end_length;
};
void grpc_http_parser_init(grpc_http_parser* parser, grpc_http_type type,
                           void* request_or_response);
void grpc_http_parser_destroy(grpc_http_parser* parser);

// Sets \a start_of_body to the offset in \a slice of the start of the body.
grpc_error_handle grpc_http_parser_parse(grpc_http_parser* parser,
                                         const grpc_slice& slice,
                                         size_t* start_of_body);
grpc_error_handle grpc_http_parser_eof(grpc_http_parser* parser);

void grpc_http_request_destroy(grpc_http_request* request);
void grpc_http_response_destroy(grpc_http_response* response);

#endif  // GRPC_SRC_CORE_UTIL_HTTP_CLIENT_PARSER_H
