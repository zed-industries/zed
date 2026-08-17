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

#ifndef GRPC_SRC_CORE_UTIL_STRING_H
#define GRPC_SRC_CORE_UTIL_STRING_H

#include <grpc/support/port_platform.h>
#include <grpc/support/time.h>
#include <stddef.h>
#include <stdint.h>

#include <string>

// String utility functions

// Flags for gpr_dump function.
#define GPR_DUMP_HEX 0x00000001
#define GPR_DUMP_ASCII 0x00000002

// Converts array buf, of length len, into a C string according to the flags.
// Result should be freed with gpr_free()
char* gpr_dump(const char* buf, size_t len, uint32_t flags);
// Converts array buf, of length len, into a C string according to the flags.
// The length of the returned buffer is stored in out_len.
// Result should be freed with gpr_free()
char* gpr_dump_return_len(const char* buf, size_t len, uint32_t flags,
                          size_t* out_len);

// Parses an array of bytes into an integer (base 10). Returns 1 on success,
// 0 on failure.
int gpr_parse_bytes_to_uint32(const char* buf, size_t len, uint32_t* result);

// Minimum buffer size for calling ltoa
#define GPR_LTOA_MIN_BUFSIZE (3 * sizeof(long))

// Convert a long to a string in base 10; returns the length of the
// output string (or 0 on failure).
// output must be at least GPR_LTOA_MIN_BUFSIZE bytes long.
int gpr_ltoa(long value, char* output);

// Minimum buffer size for calling int64toa
#define GPR_INT64TOA_MIN_BUFSIZE (3 * sizeof(int64_t))

// Convert an int64 to a string in base 10; returns the length of the
// output string (or 0 on failure).
// output must be at least GPR_INT64TOA_MIN_BUFSIZE bytes long.
// NOTE: This function ensures sufficient bit width even on Win x64,
// where long is 32bit is size.
int int64_ttoa(int64_t value, char* output);

// Parses a non-negative number from a value string.  Returns -1 on error.
int gpr_parse_nonnegative_int(const char* value);

// Reverse a run of bytes
void gpr_reverse_bytes(char* str, int len);

// Pad a string with flag characters. The given length specifies the minimum
// field width. The input string is never truncated.
char* gpr_leftpad(const char* str, char flag, size_t length);

// Join a set of strings, returning the resulting string.
// Total combined length (excluding null terminator) is returned in final_length
// if it is non-null.
char* gpr_strjoin(const char** strs, size_t nstrs, size_t* final_length);

// Join a set of strings using a separator, returning the resulting string.
// Total combined length (excluding null terminator) is returned in final_length
// if it is non-null.
char* gpr_strjoin_sep(const char** strs, size_t nstrs, const char* sep,
                      size_t* final_length);

void gpr_string_split(const char* input, const char* sep, char*** strs,
                      size_t* nstrs);

// Returns a string that represents tm according to RFC-3339, and,
// more specifically, follows:
// https://developers.google.com/protocol-buffers/docs/proto3#json

// Uses RFC 3339, where generated output will always be Z-normalized and uses
// 0, 3, 6 or 9 fractional digits.
std::string gpr_format_timespec(gpr_timespec);

/// Case insensitive string comparison... return <0 if lower(a)<lower(b), ==0 if
/// lower(a)==lower(b), >0 if lower(a)>lower(b)
int gpr_stricmp(const char* a, const char* b);
int gpr_strincmp(const char* a, const char* b, size_t n);

void* gpr_memrchr(const void* s, int c, size_t n);

// Try to parse given string into a boolean value.
// When parsed successfully, dst will have the value and returns true.
// Otherwise, it returns false.
bool gpr_parse_bool_value(const char* value, bool* dst);

#endif  // GRPC_SRC_CORE_UTIL_STRING_H
