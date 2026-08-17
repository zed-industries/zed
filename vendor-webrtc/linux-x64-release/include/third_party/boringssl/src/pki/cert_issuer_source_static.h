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

#ifndef BSSL_PKI_CERT_ISSUER_SOURCE_STATIC_H_
#define BSSL_PKI_CERT_ISSUER_SOURCE_STATIC_H_

#include <unordered_map>
#include <vector>

#include <openssl/base.h>

#include "cert_issuer_source.h"

BSSL_NAMESPACE_BEGIN

// Synchronously returns issuers from a pre-supplied set.
class OPENSSL_EXPORT CertIssuerSourceStatic : public CertIssuerSource {
 public:
  CertIssuerSourceStatic();

  CertIssuerSourceStatic(const CertIssuerSourceStatic &) = delete;
  CertIssuerSourceStatic &operator=(const CertIssuerSourceStatic &) = delete;

  ~CertIssuerSourceStatic() override;

  // Adds |cert| to the set of certificates that this CertIssuerSource will
  // provide.
  void AddCert(std::shared_ptr<const ParsedCertificate> cert);

  // Clears the set of certificates.
  void Clear();

  // Returns a vector containing all the certificates added to this source.
  std::vector<std::shared_ptr<const ParsedCertificate>> Certs() const;

  size_t size() const { return intermediates_.size(); }

  // CertIssuerSource implementation:
  void SyncGetIssuersOf(const ParsedCertificate *cert,
                        ParsedCertificateList *issuers) override;
  void AsyncGetIssuersOf(const ParsedCertificate *cert,
                         std::unique_ptr<Request> *out_req) override;

 private:
  // The certificates that the CertIssuerSourceStatic can return, keyed on the
  // normalized subject value.
  std::unordered_multimap<std::string_view,
                          std::shared_ptr<const ParsedCertificate>>
      intermediates_;
};

BSSL_NAMESPACE_END

#endif  // BSSL_PKI_CERT_ISSUER_SOURCE_STATIC_H_
