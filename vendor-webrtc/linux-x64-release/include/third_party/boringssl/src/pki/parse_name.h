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

#ifndef BSSL_PKI_PARSE_NAME_H_
#define BSSL_PKI_PARSE_NAME_H_

#include <vector>

#include <openssl/base.h>
#include <openssl/bytestring.h>

#include "input.h"
#include "parser.h"

BSSL_NAMESPACE_BEGIN

// id-at-commonName: 2.5.4.3 (RFC 5280)
inline constexpr uint8_t kTypeCommonNameOid[] = {0x55, 0x04, 0x03};
// id-at-surname: 2.5.4.4 (RFC 5280)
inline constexpr uint8_t kTypeSurnameOid[] = {0x55, 0x04, 0x04};
// id-at-serialNumber: 2.5.4.5 (RFC 5280)
inline constexpr uint8_t kTypeSerialNumberOid[] = {0x55, 0x04, 0x05};
// id-at-countryName: 2.5.4.6 (RFC 5280)
inline constexpr uint8_t kTypeCountryNameOid[] = {0x55, 0x04, 0x06};
// id-at-localityName: 2.5.4.7 (RFC 5280)
inline constexpr uint8_t kTypeLocalityNameOid[] = {0x55, 0x04, 0x07};
// id-at-stateOrProvinceName: 2.5.4.8 (RFC 5280)
inline constexpr uint8_t kTypeStateOrProvinceNameOid[] = {0x55, 0x04, 0x08};
// street (streetAddress): 2.5.4.9 (RFC 4519)
inline constexpr uint8_t kTypeStreetAddressOid[] = {0x55, 0x04, 0x09};
// id-at-organizationName: 2.5.4.10 (RFC 5280)
inline constexpr uint8_t kTypeOrganizationNameOid[] = {0x55, 0x04, 0x0a};
// id-at-organizationalUnitName: 2.5.4.11 (RFC 5280)
inline constexpr uint8_t kTypeOrganizationUnitNameOid[] = {0x55, 0x04, 0x0b};
// id-at-title: 2.5.4.12 (RFC 5280)
inline constexpr uint8_t kTypeTitleOid[] = {0x55, 0x04, 0x0c};
// id-at-name: 2.5.4.41 (RFC 5280)
inline constexpr uint8_t kTypeNameOid[] = {0x55, 0x04, 0x29};
// id-at-givenName: 2.5.4.42 (RFC 5280)
inline constexpr uint8_t kTypeGivenNameOid[] = {0x55, 0x04, 0x2a};
// id-at-initials: 2.5.4.43 (RFC 5280)
inline constexpr uint8_t kTypeInitialsOid[] = {0x55, 0x04, 0x2b};
// id-at-generationQualifier: 2.5.4.44 (RFC 5280)
inline constexpr uint8_t kTypeGenerationQualifierOid[] = {0x55, 0x04, 0x2c};
// dc (domainComponent): 0.9.2342.19200300.100.1.25 (RFC 4519)
inline constexpr uint8_t kTypeDomainComponentOid[] = {
    0x09, 0x92, 0x26, 0x89, 0x93, 0xF2, 0x2C, 0x64, 0x01, 0x19};
// RFC 5280 section A.1:
//
// pkcs-9 OBJECT IDENTIFIER ::=
//   { iso(1) member-body(2) us(840) rsadsi(113549) pkcs(1) 9 }
//
// id-emailAddress      AttributeType ::= { pkcs-9 1 }
//
// In dotted form: 1.2.840.113549.1.9.1
inline constexpr uint8_t kTypeEmailAddressOid[] = {0x2A, 0x86, 0x48, 0x86, 0xF7,
                                                   0x0D, 0x01, 0x09, 0x01};

// X509NameAttribute contains a representation of a DER-encoded RFC 2253
// "AttributeTypeAndValue".
//
// AttributeTypeAndValue ::= SEQUENCE {
//     type  AttributeType,
//     value AttributeValue
// }
struct OPENSSL_EXPORT X509NameAttribute {
  X509NameAttribute(der::Input in_type, CBS_ASN1_TAG in_value_tag,
                    der::Input in_value)
      : type(in_type), value_tag(in_value_tag), value(in_value) {}

  // Configures handling of PrintableString in the attribute value. Do
  // not use non-default handling without consulting //net owners. With
  // kAsUTF8Hack, PrintableStrings are interpreted as UTF-8 strings.
  enum class PrintableStringHandling { kDefault, kAsUTF8Hack };

  // Attempts to convert the value represented by this struct into a
  // UTF-8 string and store it in |out|, returning whether the conversion
  // was successful.
  [[nodiscard]] bool ValueAsString(std::string *out) const;

  // Attempts to convert the value represented by this struct into a
  // UTF-8 string and store it in |out|, returning whether the conversion
  // was successful. Allows configuring some non-standard string handling
  // options.
  //
  // Do not use without consulting //net owners.
  [[nodiscard]] bool ValueAsStringWithUnsafeOptions(
      PrintableStringHandling printable_string_handling,
      std::string *out) const;

  // Attempts to convert the value represented by this struct into a
  // std::string and store it in |out|, returning whether the conversion was
  // successful. Due to some encodings being incompatible, the caller must
  // verify the attribute |value_tag|.
  //
  // Note: Don't use this function unless you know what you're doing. Use
  // ValueAsString instead.
  //
  // Note: The conversion doesn't verify that the value corresponds to the
  // ASN.1 definition of the value type.
  [[nodiscard]] bool ValueAsStringUnsafe(std::string *out) const;

  // Formats the NameAttribute per RFC2253 into an ASCII string and stores
  // the result in |out|, returning whether the conversion was successful.
  [[nodiscard]] bool AsRFC2253String(std::string *out) const;

  der::Input type;
  CBS_ASN1_TAG value_tag;
  der::Input value;
};

typedef std::vector<X509NameAttribute> RelativeDistinguishedName;
typedef std::vector<RelativeDistinguishedName> RDNSequence;

// Parses all the ASN.1 AttributeTypeAndValue elements in |parser| and stores
// each as an AttributeTypeAndValue object in |out|.
//
// AttributeTypeAndValue is defined in RFC 5280 section 4.1.2.4:
//
// AttributeTypeAndValue ::= SEQUENCE {
//   type     AttributeType,
//   value    AttributeValue }
//
// AttributeType ::= OBJECT IDENTIFIER
//
// AttributeValue ::= ANY -- DEFINED BY AttributeType
//
// DirectoryString ::= CHOICE {
//       teletexString           TeletexString (SIZE (1..MAX)),
//       printableString         PrintableString (SIZE (1..MAX)),
//       universalString         UniversalString (SIZE (1..MAX)),
//       utf8String              UTF8String (SIZE (1..MAX)),
//       bmpString               BMPString (SIZE (1..MAX)) }
//
// The type of the component AttributeValue is determined by the AttributeType;
// in general it will be a DirectoryString.
[[nodiscard]] OPENSSL_EXPORT bool ReadRdn(der::Parser *parser,
                                          RelativeDistinguishedName *out);

// Parses a DER-encoded "Name" as specified by 5280. Returns true on success
// and sets the results in |out|.
[[nodiscard]] OPENSSL_EXPORT bool ParseName(der::Input name_tlv,
                                            RDNSequence *out);
// Parses a DER-encoded "Name" value (without the sequence tag & length) as
// specified by 5280. Returns true on success and sets the results in |out|.
[[nodiscard]] OPENSSL_EXPORT bool ParseNameValue(der::Input name_value,
                                                 RDNSequence *out);

// Formats a RDNSequence |rdn_sequence| per RFC2253 as an ASCII string and
// stores the result into |out|, and returns whether the conversion was
// successful.
[[nodiscard]] OPENSSL_EXPORT bool ConvertToRFC2253(
    const RDNSequence &rdn_sequence, std::string *out);
BSSL_NAMESPACE_END

#endif  // BSSL_PKI_PARSE_NAME_H_
