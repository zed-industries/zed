// Copyright 2015 The Chromium Authors
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

#ifndef BSSL_PKI_VERIFY_NAME_MATCH_H_
#define BSSL_PKI_VERIFY_NAME_MATCH_H_

#include <string>
#include <vector>

#include <openssl/base.h>

BSSL_NAMESPACE_BEGIN

class CertErrors;

namespace der {
class Input;
}  // namespace der

// Normalizes DER-encoded X.501 Name |name_rdn_sequence| (which should not
// include the Sequence tag).  If successful, returns true and stores the
// normalized DER-encoded Name into |normalized_rdn_sequence| (not including an
// outer Sequence tag). Returns false if there was an error parsing or
// normalizing the input, and adds error information to |errors|. |errors| must
// be non-null.
OPENSSL_EXPORT bool NormalizeName(der::Input name_rdn_sequence,
                                  std::string *normalized_rdn_sequence,
                                  CertErrors *errors);

// Compares DER-encoded X.501 Name values according to RFC 5280 rules.
// |a_rdn_sequence| and |b_rdn_sequence| should be the DER-encoded RDNSequence
// values (not including the Sequence tag).
// Returns true if |a_rdn_sequence| and |b_rdn_sequence| match.
OPENSSL_EXPORT bool VerifyNameMatch(der::Input a_rdn_sequence,
                                    der::Input b_rdn_sequence);

// Compares |name_rdn_sequence| and |parent_rdn_sequence| and return true if
// |name_rdn_sequence| is within the subtree defined by |parent_rdn_sequence| as
// defined by RFC 5280 section 7.1. |name_rdn_sequence| and
// |parent_rdn_sequence| should be the DER-encoded sequence values (not
// including the Sequence tag).
OPENSSL_EXPORT bool VerifyNameInSubtree(der::Input name_rdn_sequence,
                                        der::Input parent_rdn_sequence);

// Helper functions:

// Find all emailAddress attribute values in |name_rdn_sequence|.
// Returns true if parsing was successful, in which case
// |*contained_email_address| will contain zero or more values.  The values
// returned in |*contained_email_addresses| will be UTF8 strings and have been
// checked that they were valid strings for the string type of the attribute
// tag, but otherwise have not been validated.
// Returns false if there was a parsing error.
[[nodiscard]] bool FindEmailAddressesInName(
    der::Input name_rdn_sequence,
    std::vector<std::string> *contained_email_addresses);

BSSL_NAMESPACE_END

#endif  // BSSL_PKI_VERIFY_NAME_MATCH_H_
