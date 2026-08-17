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

#ifndef BSSL_PKI_TRUST_STORE_IN_MEMORY_H_
#define BSSL_PKI_TRUST_STORE_IN_MEMORY_H_

#include <set>
#include <unordered_map>

#include <openssl/base.h>

#include "trust_store.h"

BSSL_NAMESPACE_BEGIN

// A very simple implementation of a TrustStore, which contains a set of
// certificates and their trustedness.
class OPENSSL_EXPORT TrustStoreInMemory : public TrustStore {
 public:
  TrustStoreInMemory();

  TrustStoreInMemory(const TrustStoreInMemory &) = delete;
  TrustStoreInMemory &operator=(const TrustStoreInMemory &) = delete;

  ~TrustStoreInMemory() override;

  // Returns whether the TrustStore is in the initial empty state.
  bool IsEmpty() const;

  // Empties the trust store, resetting it to original state.
  void Clear();

  // Adds a certificate with the specified trust settings. Both trusted and
  // distrusted certificates require a full DER match.
  void AddCertificate(std::shared_ptr<const ParsedCertificate> cert,
                      const CertificateTrust &trust);

  // Adds a certificate as a trust anchor (only the SPKI and subject will be
  // used during verification).
  void AddTrustAnchor(std::shared_ptr<const ParsedCertificate> cert);

  // Adds a certificate as a trust anchor which will have expiration enforced.
  // See VerifyCertificateChain for details.
  void AddTrustAnchorWithExpiration(
      std::shared_ptr<const ParsedCertificate> cert);

  // Adds a certificate as a trust anchor and extracts anchor constraints from
  // the certificate. See VerifyCertificateChain for details.
  void AddTrustAnchorWithConstraints(
      std::shared_ptr<const ParsedCertificate> cert);

  // TODO(eroman): This is marked "ForTest" as the current implementation
  // requires an exact match on the certificate DER (a wider match by say
  // issuer/serial is probably what we would want for a real implementation).
  void AddDistrustedCertificateForTest(
      std::shared_ptr<const ParsedCertificate> cert);

  // Distrusts the provided SPKI. This will override any other trust (e.g. if a
  // certificate is passed into AddTrustAnchor() and the certificate's SPKI is
  // passed into AddDistrustedCertificateBySPKI(), GetTrust() will return
  // CertificateTrust::ForDistrusted()).
  void AddDistrustedCertificateBySPKI(std::string spki);

  // Adds a certificate to the store, that is neither trusted nor untrusted.
  void AddCertificateWithUnspecifiedTrust(
      std::shared_ptr<const ParsedCertificate> cert);

  // TrustStore implementation:
  void SyncGetIssuersOf(const ParsedCertificate *cert,
                        ParsedCertificateList *issuers) override;
  CertificateTrust GetTrust(const ParsedCertificate *cert) override;

  // Returns true if the trust store contains the given ParsedCertificate
  // (matches by DER).
  bool Contains(const ParsedCertificate *cert) const;

 private:
  struct Entry {
    Entry();
    Entry(const Entry &other);
    ~Entry();

    std::shared_ptr<const ParsedCertificate> cert;
    CertificateTrust trust;
  };

  // Multimap from normalized subject -> Entry.
  std::unordered_multimap<std::string_view, Entry> entries_;

  // Set of distrusted SPKIs.
  std::set<std::string, std::less<>> distrusted_spkis_;

  // Returns the `Entry` matching `cert`, or `nullptr` if not in the trust
  // store.
  const Entry *GetEntry(const ParsedCertificate *cert) const;
};

BSSL_NAMESPACE_END

#endif  // BSSL_PKI_TRUST_STORE_IN_MEMORY_H_
