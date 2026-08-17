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

#ifndef BSSL_PKI_CERT_ERROR_PARAMS_H_
#define BSSL_PKI_CERT_ERROR_PARAMS_H_

#include <memory>
#include <string>

#include <openssl/base.h>

BSSL_NAMESPACE_BEGIN

namespace der {
class Input;
}

// CertErrorParams is a base class for describing extra parameters attached to
// a CertErrorNode.
//
// An example use for parameters is to identify the OID for an unconsumed
// critical extension. This parameter could then be pretty printed when
// diagnosing the error.
class OPENSSL_EXPORT CertErrorParams {
 public:
  CertErrorParams();

  CertErrorParams(const CertErrorParams &) = delete;
  CertErrorParams &operator=(const CertErrorParams &) = delete;

  virtual ~CertErrorParams();

  // Creates a representation of this parameter as a string, which may be
  // used for pretty printing the error.
  virtual std::string ToDebugString() const = 0;
};

// Creates a parameter object that holds a copy of |der|, and names it |name|
// in debug string outputs.
OPENSSL_EXPORT std::unique_ptr<CertErrorParams> CreateCertErrorParams1Der(
    const char *name, der::Input der);

// Same as CreateCertErrorParams1Der() but has a second DER blob.
OPENSSL_EXPORT std::unique_ptr<CertErrorParams> CreateCertErrorParams2Der(
    const char *name1, der::Input der1, const char *name2, der::Input der2);

// Creates a parameter object that holds a single size_t value. |name| is used
// when pretty-printing the parameters.
OPENSSL_EXPORT std::unique_ptr<CertErrorParams> CreateCertErrorParams1SizeT(
    const char *name, size_t value);

// Same as CreateCertErrorParams1SizeT() but has a second size_t.
OPENSSL_EXPORT std::unique_ptr<CertErrorParams> CreateCertErrorParams2SizeT(
    const char *name1, size_t value1, const char *name2, size_t value2);

BSSL_NAMESPACE_END

#endif  // BSSL_PKI_CERT_ERROR_PARAMS_H_
