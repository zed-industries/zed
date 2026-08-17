// Copyright 2016 The Chromium Authors
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

#ifndef BSSL_PKI_CERT_ERROR_ID_H_
#define BSSL_PKI_CERT_ERROR_ID_H_

#include <openssl/base.h>

BSSL_NAMESPACE_BEGIN

// Each "class" of certificate error/warning has its own unique ID. This is
// essentially like an error code, however the value is not stable. Under the
// hood these IDs are pointers and use the process's address space to ensure
// uniqueness.
//
// Equality of CertErrorId can be done using the == operator.
//
// To define new error IDs use the macro DEFINE_CERT_ERROR_ID().
using CertErrorId = const void *;

// DEFINE_CERT_ERROR_ID() creates a CertErrorId given a non-null C-string
// literal. The string should be a textual name for the error which will appear
// when pretty-printing errors for debugging. It should be ASCII.
//
// TODO(crbug.com/634443): Implement this -- add magic to ensure that storage
//                         of identical strings isn't pool.
#define DEFINE_CERT_ERROR_ID(name, c_str_literal) \
  const CertErrorId name = c_str_literal

// Returns a debug string for a CertErrorId. In practice this returns the
// string literal given to DEFINE_CERT_ERROR_ID(), which is human-readable.
OPENSSL_EXPORT const char *CertErrorIdToDebugString(CertErrorId id);

BSSL_NAMESPACE_END

#endif  // BSSL_PKI_CERT_ERROR_ID_H_
