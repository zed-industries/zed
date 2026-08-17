// Copyright 2017 The BoringSSL Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef HEADER_SSL_TEST_FUZZER_TAGS
#define HEADER_SSL_TEST_FUZZER_TAGS

#include <stdint.h>


// SSL fuzzer tag constants.
//
// The TLS client and server fuzzers coordinate with bssl_shim on a common
// format to encode configuration parameters in a fuzzer file. To add a new
// configuration, define a tag, update |SetupTest| in fuzzer.h to parse it, and
// update |SettingsWriter| in bssl_shim to serialize it. Finally, record
// transcripts from a test run, and use the BORINGSSL_FUZZER_DEBUG environment
// variable to confirm the transcripts are compatible.

// kDataTag denotes that the remainder of the input should be passed to the TLS
// stack.
static const uint16_t kDataTag = 0;

// kSessionTag is followed by a u24-length-prefixed serialized SSL_SESSION to
// resume.
static const uint16_t kSessionTag = 1;

// kRequestClientCert denotes that the server should request client
// certificates.
static const uint16_t kRequestClientCert = 2;

// kHandoffTag is followed by the output of |SSL_serialize_handoff|.
static const uint16_t kHandoffTag = 3;

// kHandbackTag is followed by te output of |SSL_serialize_handback|.
static const uint16_t kHandbackTag = 4;

// kHintsTag is followed by the output of |SSL_serialize_handshake_hints|.
static const uint16_t kHintsTag = 5;

#endif  // HEADER_SSL_TEST_FUZZER_TAGS
