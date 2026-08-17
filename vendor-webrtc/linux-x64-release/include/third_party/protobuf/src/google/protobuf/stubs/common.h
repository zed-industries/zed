// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Author: kenton@google.com (Kenton Varda) and others
//
// Contains basic types and utilities used by the rest of the library.

#ifndef GOOGLE_PROTOBUF_COMMON_H__
#define GOOGLE_PROTOBUF_COMMON_H__

#include <algorithm>
#include <iostream>
#include <memory>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "google/protobuf/stubs/platform_macros.h"
#include "google/protobuf/stubs/port.h"

#if defined(__APPLE__)
#include <TargetConditionals.h>  // for TARGET_OS_IPHONE
#endif

#if defined(__ANDROID__) || defined(GOOGLE_PROTOBUF_OS_ANDROID) || \
    (defined(TARGET_OS_IPHONE) && TARGET_OS_IPHONE) ||             \
    defined(GOOGLE_PROTOBUF_OS_IPHONE)
#include <pthread.h>
#endif

#include "google/protobuf/port_def.inc"

namespace std {}

namespace google {
namespace protobuf {
namespace internal {

// Some of these constants are macros rather than const ints so that they can
// be used in #if directives.

// The current version, represented as a single integer to make comparison
// easier:  major * 10^6 + minor * 10^3 + micro
#define GOOGLE_PROTOBUF_VERSION 6030002

// A suffix string for alpha, beta or rc releases. Empty for stable releases.
#define GOOGLE_PROTOBUF_VERSION_SUFFIX ""

// Verifies that the protobuf version a program was compiled with matches what
// it is linked/running with. Use the macro below to call this function.
void PROTOBUF_EXPORT VerifyVersion(int protobufVersionCompiledWith,
                                   const char* filename);

// Converts a numeric version number to a string.
std::string PROTOBUF_EXPORT
VersionString(int version);  // NOLINT(runtime/string)

// Prints the protoc compiler version (no major version)
std::string PROTOBUF_EXPORT
ProtocVersionString(int version);  // NOLINT(runtime/string)

}  // namespace internal

// Place this macro in your main() function (or somewhere before you attempt
// to use the protobuf library) to verify that the version you link against
// matches the headers you compiled against.  If a version mismatch is
// detected, the process will abort.
#define GOOGLE_PROTOBUF_VERIFY_VERSION \
  ::google::protobuf::internal::VerifyVersion(GOOGLE_PROTOBUF_VERSION, __FILE__)

// This lives in message_lite.h now, but we leave this here for any users that
// #include common.h and not message_lite.h.
PROTOBUF_EXPORT void ShutdownProtobufLibrary();

namespace internal {

// Strongly references the given variable such that the linker will be forced
// to pull in this variable's translation unit.
template <typename T>
void StrongReference(const T& var) {
  auto volatile unused = &var;
  (void)&unused;  // Use address to avoid an extra load of "unused".
}

}  // namespace internal
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_COMMON_H__
