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

#ifndef BSSL_PKI_CERT_ISSUER_SOURCE_H_
#define BSSL_PKI_CERT_ISSUER_SOURCE_H_

#include <memory>
#include <vector>

#include <openssl/base.h>

#include "parsed_certificate.h"

BSSL_NAMESPACE_BEGIN

// Interface for looking up issuers of a certificate during path building.
// Provides a synchronous and asynchronous method for retrieving issuers, so the
// path builder can try to complete synchronously first. The caller is expected
// to call SyncGetIssuersOf first, see if it can make progress with those
// results, and if not, then fall back to calling AsyncGetIssuersOf.
// An implementations may choose to return results from either one of the Get
// methods, or from both.
class OPENSSL_EXPORT CertIssuerSource {
 public:
  class OPENSSL_EXPORT Request {
   public:
    Request() = default;

    Request(const Request &) = delete;
    Request &operator=(const Request &) = delete;

    // Destruction of the Request cancels it.
    virtual ~Request() = default;

    // Retrieves issuers and appends them to |issuers|.
    //
    // GetNext should be called again to retrieve any remaining issuers.
    //
    // If no issuers are left then |issuers| will not be modified. This
    // indicates that the issuers have been exhausted and GetNext() should
    // not be called again.
    virtual void GetNext(ParsedCertificateList *issuers) = 0;
  };

  virtual ~CertIssuerSource() = default;

  // Finds certificates whose Subject matches |cert|'s Issuer.
  // Matches are appended to |issuers|. Any existing contents of |issuers| will
  // not be modified. If the implementation does not support synchronous
  // lookups, or if there are no matches, |issuers| is not modified.
  virtual void SyncGetIssuersOf(const ParsedCertificate *cert,
                                ParsedCertificateList *issuers) = 0;

  // Finds certificates whose Subject matches |cert|'s Issuer.
  // If the implementation does not support asynchronous lookups or can
  // determine synchronously that it would return no results, |*out_req|
  // will be set to nullptr.
  //
  // Otherwise a request is started and saved to |out_req|. The results can be
  // read through the Request interface.
  virtual void AsyncGetIssuersOf(const ParsedCertificate *cert,
                                 std::unique_ptr<Request> *out_req) = 0;
};

BSSL_NAMESPACE_END

#endif  // BSSL_PKI_CERT_ISSUER_SOURCE_H_
