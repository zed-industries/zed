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

#ifndef BSSL_PKI_PARSED_CERTIFICATE_H_
#define BSSL_PKI_PARSED_CERTIFICATE_H_

#include <map>
#include <memory>
#include <optional>
#include <vector>

#include <openssl/base.h>
#include <openssl/span.h>

#include "certificate_policies.h"
#include "input.h"
#include "parse_certificate.h"
#include "signature_algorithm.h"

BSSL_NAMESPACE_BEGIN

struct GeneralNames;
class NameConstraints;
class ParsedCertificate;
class CertErrors;

using ParsedCertificateList =
    std::vector<std::shared_ptr<const ParsedCertificate>>;

// Represents an X.509 certificate, including Certificate, TBSCertificate, and
// standard extensions.
// Creating a ParsedCertificate does not completely parse and validate the
// certificate data. Presence of a member in this class implies the DER was
// parsed successfully to that level, but does not imply the contents of that
// member are valid, unless otherwise specified. See the documentation for each
// member or the documentation of the type it returns.
class OPENSSL_EXPORT ParsedCertificate {
 private:
  // Used to make constructors private while still being compatible with
  // |std::make_shared|.
  class PrivateConstructor {
   private:
    friend ParsedCertificate;
    PrivateConstructor() = default;
  };

 public:
  ~ParsedCertificate();
  // Map from OID to ParsedExtension.
  using ExtensionsMap = std::map<der::Input, ParsedExtension>;

  // Creates a ParsedCertificate given a DER-encoded Certificate. Returns
  // nullptr on failure. Failure will occur if the standard certificate fields
  // and supported extensions cannot be parsed.
  // On either success or failure, if |errors| is non-null it may have error
  // information added to it.
  static std::shared_ptr<const ParsedCertificate> Create(
      bssl::UniquePtr<CRYPTO_BUFFER> cert_data,
      const ParseCertificateOptions &options, CertErrors *errors);

  // Creates a ParsedCertificate by copying the provided |data|, and appends it
  // to |chain|. Returns true if the certificate was successfully parsed and
  // added. If false is return, |chain| is unmodified.
  //
  // On either success or failure, if |errors| is non-null it may have error
  // information added to it.
  static bool CreateAndAddToVector(
      bssl::UniquePtr<CRYPTO_BUFFER> cert_data,
      const ParseCertificateOptions &options,
      std::vector<std::shared_ptr<const bssl::ParsedCertificate>> *chain,
      CertErrors *errors);

  explicit ParsedCertificate(PrivateConstructor);

  ParsedCertificate(const ParsedCertificate &) = delete;
  ParsedCertificate &operator=(const ParsedCertificate &) = delete;

  // Returns the DER-encoded certificate data for this cert.
  der::Input der_cert() const { return cert_; }

  // Returns the CRYPTO_BUFFER backing this object.
  CRYPTO_BUFFER *cert_buffer() const { return cert_data_.get(); }

  // Accessors for raw fields of the Certificate.
  der::Input tbs_certificate_tlv() const { return tbs_certificate_tlv_; }

  der::Input signature_algorithm_tlv() const {
    return signature_algorithm_tlv_;
  }

  const der::BitString &signature_value() const { return signature_value_; }

  // Accessor for struct containing raw fields of the TbsCertificate.
  const ParsedTbsCertificate &tbs() const { return tbs_; }

  // Returns the signatureAlgorithm of the Certificate (not the tbsCertificate).
  // If the signature algorithm is unknown/unsupported, this returns nullopt.
  std::optional<SignatureAlgorithm> signature_algorithm() const {
    return signature_algorithm_;
  }

  // Returns the DER-encoded raw subject value (including the outer sequence
  // tag). This is guaranteed to be valid DER, though the contents of unhandled
  // string types are treated as raw bytes.
  der::Input subject_tlv() const { return tbs_.subject_tlv; }
  // Returns the DER-encoded normalized subject value (not including outer
  // Sequence tag). This is guaranteed to be valid DER, though the contents of
  // unhandled string types are treated as raw bytes.
  der::Input normalized_subject() const {
    return StringAsBytes(normalized_subject_);
  }
  // Returns the DER-encoded raw issuer value (including the outer sequence
  // tag). This is guaranteed to be valid DER, though the contents of unhandled
  // string types are treated as raw bytes.
  der::Input issuer_tlv() const { return tbs_.issuer_tlv; }
  // Returns the DER-encoded normalized issuer value (not including outer
  // Sequence tag). This is guaranteed to be valid DER, though the contents of
  // unhandled string types are treated as raw bytes.
  der::Input normalized_issuer() const {
    return StringAsBytes(normalized_issuer_);
  }

  // Returns true if the certificate has a BasicConstraints extension.
  bool has_basic_constraints() const { return has_basic_constraints_; }

  // Returns the ParsedBasicConstraints struct. Caller must check
  // has_basic_constraints() before accessing this.
  const ParsedBasicConstraints &basic_constraints() const {
    BSSL_CHECK(has_basic_constraints_);
    return basic_constraints_;
  }

  // Returns true if the certificate has a KeyUsage extension.
  bool has_key_usage() const { return has_key_usage_; }

  // Returns the KeyUsage BitString. Caller must check
  // has_key_usage() before accessing this.
  const der::BitString &key_usage() const {
    BSSL_CHECK(has_key_usage_);
    return key_usage_;
  }

  // Returns true if the certificate has a ExtendedKeyUsage extension.
  bool has_extended_key_usage() const { return has_extended_key_usage_; }

  // Returns the ExtendedKeyUsage key purpose OIDs. Caller must check
  // has_extended_key_usage() before accessing this.
  const std::vector<der::Input> &extended_key_usage() const {
    BSSL_CHECK(has_extended_key_usage_);
    return extended_key_usage_;
  }

  // Returns true if the certificate has a SubjectAltName extension.
  bool has_subject_alt_names() const { return subject_alt_names_ != nullptr; }

  // Returns the ParsedExtension struct for the SubjectAltName extension.
  // If the cert did not have a SubjectAltName extension, this will be a
  // default-initialized ParsedExtension struct.
  const ParsedExtension &subject_alt_names_extension() const {
    return subject_alt_names_extension_;
  }

  // Returns the GeneralNames class parsed from SubjectAltName extension, or
  // nullptr if no SubjectAltName extension was present.
  const GeneralNames *subject_alt_names() const {
    return subject_alt_names_.get();
  }

  // Returns true if the certificate has a NameConstraints extension.
  bool has_name_constraints() const { return name_constraints_ != nullptr; }

  // Returns the parsed NameConstraints extension. Must not be called if
  // has_name_constraints() is false.
  const NameConstraints &name_constraints() const {
    BSSL_CHECK(name_constraints_);
    return *name_constraints_;
  }

  // Returns true if the certificate has an AuthorityInfoAccess extension.
  bool has_authority_info_access() const { return has_authority_info_access_; }

  // Returns the ParsedExtension struct for the AuthorityInfoAccess extension.
  const ParsedExtension &authority_info_access_extension() const {
    return authority_info_access_extension_;
  }

  // Returns any caIssuers URIs from the AuthorityInfoAccess extension.
  const std::vector<std::string_view> &ca_issuers_uris() const {
    return ca_issuers_uris_;
  }

  // Returns any OCSP URIs from the AuthorityInfoAccess extension.
  const std::vector<std::string_view> &ocsp_uris() const { return ocsp_uris_; }

  // Returns true if the certificate has a Policies extension.
  bool has_policy_oids() const { return has_policy_oids_; }

  // Returns the policy OIDs. Caller must check has_policy_oids() before
  // accessing this.
  const std::vector<der::Input> &policy_oids() const {
    BSSL_CHECK(has_policy_oids());
    return policy_oids_;
  }

  // Returns true if the certificate has a PolicyConstraints extension.
  bool has_policy_constraints() const { return has_policy_constraints_; }

  // Returns the ParsedPolicyConstraints struct. Caller must check
  // has_policy_constraints() before accessing this.
  const ParsedPolicyConstraints &policy_constraints() const {
    BSSL_CHECK(has_policy_constraints_);
    return policy_constraints_;
  }

  // Returns true if the certificate has a PolicyMappings extension.
  bool has_policy_mappings() const { return has_policy_mappings_; }

  // Returns the PolicyMappings extension. Caller must check
  // has_policy_mappings() before accessing this.
  const std::vector<ParsedPolicyMapping> &policy_mappings() const {
    BSSL_CHECK(has_policy_mappings_);
    return policy_mappings_;
  }

  // Returns the Inhibit Any Policy extension.
  const std::optional<uint8_t> &inhibit_any_policy() const {
    return inhibit_any_policy_;
  }

  // Returns the AuthorityKeyIdentifier extension, or nullopt if there wasn't
  // one.
  const std::optional<ParsedAuthorityKeyIdentifier> &authority_key_identifier()
      const {
    return authority_key_identifier_;
  }

  // Returns the SubjectKeyIdentifier extension, or nullopt if there wasn't
  // one.
  const std::optional<der::Input> &subject_key_identifier() const {
    return subject_key_identifier_;
  }

  // Returns a map of all the extensions in the certificate.
  const ExtensionsMap &extensions() const { return extensions_; }

  // Gets the value for extension matching |extension_oid|. Returns false if the
  // extension is not present.
  bool GetExtension(der::Input extension_oid,
                    ParsedExtension *parsed_extension) const;

 private:
  // The backing store for the certificate data.
  bssl::UniquePtr<CRYPTO_BUFFER> cert_data_;

  // Points to the raw certificate DER.
  der::Input cert_;

  der::Input tbs_certificate_tlv_;
  der::Input signature_algorithm_tlv_;
  der::BitString signature_value_;
  ParsedTbsCertificate tbs_;

  // The signatureAlgorithm from the Certificate.
  std::optional<SignatureAlgorithm> signature_algorithm_;

  // Normalized DER-encoded Subject (not including outer Sequence tag).
  std::string normalized_subject_;
  // Normalized DER-encoded Issuer (not including outer Sequence tag).
  std::string normalized_issuer_;

  // BasicConstraints extension.
  bool has_basic_constraints_ = false;
  ParsedBasicConstraints basic_constraints_;

  // KeyUsage extension.
  bool has_key_usage_ = false;
  der::BitString key_usage_;

  // ExtendedKeyUsage extension.
  bool has_extended_key_usage_ = false;
  std::vector<der::Input> extended_key_usage_;

  // Raw SubjectAltName extension.
  ParsedExtension subject_alt_names_extension_;
  // Parsed SubjectAltName extension.
  std::unique_ptr<GeneralNames> subject_alt_names_;

  // NameConstraints extension.
  std::unique_ptr<NameConstraints> name_constraints_;

  // AuthorityInfoAccess extension.
  bool has_authority_info_access_ = false;
  ParsedExtension authority_info_access_extension_;
  // CaIssuers and Ocsp URIs parsed from the AuthorityInfoAccess extension. Note
  // that the AuthorityInfoAccess may have contained other AccessDescriptions
  // which are not represented here.
  std::vector<std::string_view> ca_issuers_uris_;
  std::vector<std::string_view> ocsp_uris_;

  // Policies extension. This list will already have been checked for
  // duplicates.
  bool has_policy_oids_ = false;
  std::vector<der::Input> policy_oids_;

  // Policy constraints extension.
  bool has_policy_constraints_ = false;
  ParsedPolicyConstraints policy_constraints_;

  // Policy mappings extension.
  bool has_policy_mappings_ = false;
  std::vector<ParsedPolicyMapping> policy_mappings_;

  // Inhibit Any Policy extension.
  std::optional<uint8_t> inhibit_any_policy_;

  // AuthorityKeyIdentifier extension.
  std::optional<ParsedAuthorityKeyIdentifier> authority_key_identifier_;

  // SubjectKeyIdentifier extension.
  std::optional<der::Input> subject_key_identifier_;

  // All of the extensions.
  ExtensionsMap extensions_;
};

BSSL_NAMESPACE_END

#endif  // BSSL_PKI_PARSED_CERTIFICATE_H_
