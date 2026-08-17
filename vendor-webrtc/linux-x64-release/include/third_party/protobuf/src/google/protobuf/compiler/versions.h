// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
// https://developers.google.com/protocol-buffers/
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//     * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
//     * Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following disclaimer
// in the documentation and/or other materials provided with the
// distribution.
//     * Neither the name of Google Inc. nor the names of its
// contributors may be used to endorse or promote products derived from
// this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

#ifndef GOOGLE_PROTOBUF_VERSIONS_H__
#define GOOGLE_PROTOBUF_VERSIONS_H__

#include "absl/strings/string_view.h"
#include "google/protobuf/compiler/plugin.pb.h"

// Must be included last.
#include "google/protobuf/port_def.inc"

// Defines compiler version strings for Protobuf code generators.
//
// When they are suffixed with "-dev", they reflect the version of the next
// release, otherwise the current released version.
//
// Currently, they are embedded into comments at each gencode for public
// Protobuf C++, Java and Python. Further, we will add version strings for rest
// of languages in version.json, and they will be used to validate version
// compatibility between gencode and runtime.
//
// Versions of other plugins should not put versions here since they will not
// caught by Protobuf releases. Plugin owners should define their versions
// separately.
//
// Please avoid changing them manually, as they should be updated automatically
// by Protobuf release process.
#define PROTOBUF_CPP_VERSION_STRING "6.30.2"
#define PROTOBUF_JAVA_VERSION_STRING "4.30.2"
#define PROTOBUF_PYTHON_VERSION_STRING "6.30.2"
#define PROTOBUF_RUST_VERSION_STRING "4.30.2"


namespace google {
namespace protobuf {
namespace compiler {
namespace internal {
// For internal use to parse the Protobuf language version strings.
PROTOC_EXPORT Version ParseProtobufVersion(absl::string_view version);
}  // namespace internal

// Gets the version message according to the version strings defined above.
const Version& GetProtobufCPPVersion(bool oss_runtime);
const Version& GetProtobufJavaVersion(bool oss_runtime);
const Version& GetProtobufPythonVersion(bool oss_runtime);
}  // namespace compiler
}  // namespace protobuf
}  // namespace google
#include "google/protobuf/port_undef.inc"
#endif  // GOOGLE_PROTOBUF_VERSIONS_H__
