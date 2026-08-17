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

#ifndef BSSL_DER_PARSE_VALUES_H_
#define BSSL_DER_PARSE_VALUES_H_

#include <stdint.h>

#include <optional>

#include <openssl/base.h>

#include "input.h"

BSSL_NAMESPACE_BEGIN
namespace der {

// Reads a DER-encoded ASN.1 BOOLEAN value from |in| and puts the resulting
// value in |out|. Returns whether the encoded value could successfully be
// read.
[[nodiscard]] OPENSSL_EXPORT bool ParseBool(Input in, bool *out);

// Like ParseBool, except it is more relaxed in what inputs it accepts: Any
// value that is a valid BER encoding will be parsed successfully.
[[nodiscard]] OPENSSL_EXPORT bool ParseBoolRelaxed(Input in, bool *out);

// Checks the validity of a DER-encoded ASN.1 INTEGER value from |in|, and
// determines the sign of the number. Returns true on success and
// fills |negative|. Otherwise returns false and does not modify the out
// parameter.
//
//    in: The value portion of an INTEGER.
//    negative: Out parameter that is set to true if the number is negative
//        and false otherwise (zero is non-negative).
[[nodiscard]] OPENSSL_EXPORT bool IsValidInteger(Input in, bool *negative);

// Reads a DER-encoded ASN.1 INTEGER value from |in| and puts the resulting
// value in |out|. ASN.1 INTEGERs are arbitrary precision; this function is
// provided as a convenience when the caller knows that the value is unsigned
// and is between 0 and 2^64-1. This function returns false if the value is too
// big to fit in a uint64_t, is negative, or if there is an error reading the
// integer.
[[nodiscard]] OPENSSL_EXPORT bool ParseUint64(Input in, uint64_t *out);

// Same as ParseUint64() but for a uint8_t.
[[nodiscard]] OPENSSL_EXPORT bool ParseUint8(Input in, uint8_t *out);

// The BitString class is a helper for representing a valid parsed BIT STRING.
//
// * The bits are ordered within each octet of bytes() from most to least
//   significant, as in the DER encoding.
//
// * There may be at most 7 unused bits.
class OPENSSL_EXPORT BitString {
 public:
  BitString() = default;

  // |unused_bits| represents the number of bits in the last octet of |bytes|,
  // starting from the least significant bit, that are unused. It MUST be < 8.
  // And if bytes is empty, then it MUST be 0.
  BitString(Input bytes, uint8_t unused_bits);

  Input bytes() const { return bytes_; }
  uint8_t unused_bits() const { return unused_bits_; }

  // Returns true if the bit string contains 1 at the specified position.
  // Otherwise returns false.
  //
  // A return value of false can mean either:
  //  * The bit value at |bit_index| is 0.
  //  * There is no bit at |bit_index| (index is beyond the end).
  [[nodiscard]] bool AssertsBit(size_t bit_index) const;

 private:
  Input bytes_;
  uint8_t unused_bits_ = 0;

  // Default assignment and copy constructor are OK.
};

// Reads a DER-encoded ASN.1 BIT STRING value from |in| and returns the
// resulting octet string and number of unused bits.
//
// On failure, returns std::nullopt.
[[nodiscard]] OPENSSL_EXPORT std::optional<BitString> ParseBitString(Input in);

struct OPENSSL_EXPORT GeneralizedTime {
  uint16_t year;
  uint8_t month;
  uint8_t day;
  uint8_t hours;
  uint8_t minutes;
  uint8_t seconds;

  // Returns true if the value is in UTCTime's range.
  bool InUTCTimeRange() const;
};

OPENSSL_EXPORT bool operator<(const GeneralizedTime &lhs,
                              const GeneralizedTime &rhs);
OPENSSL_EXPORT bool operator<=(const GeneralizedTime &lhs,
                               const GeneralizedTime &rhs);
OPENSSL_EXPORT bool operator>(const GeneralizedTime &lhs,
                              const GeneralizedTime &rhs);
OPENSSL_EXPORT bool operator>=(const GeneralizedTime &lhs,
                               const GeneralizedTime &rhs);

// Reads a DER-encoded ASN.1 UTCTime value from |in| and puts the resulting
// value in |out|, returning true if the UTCTime could be parsed successfully.
[[nodiscard]] OPENSSL_EXPORT bool ParseUTCTime(Input in, GeneralizedTime *out);

// Reads a DER-encoded ASN.1 GeneralizedTime value from |in| and puts the
// resulting value in |out|, returning true if the GeneralizedTime could
// be parsed successfully. This function is even more restrictive than the
// DER rules - it follows the rules from RFC5280, which does not allow for
// fractional seconds.
[[nodiscard]] OPENSSL_EXPORT bool ParseGeneralizedTime(Input in,
                                                       GeneralizedTime *out);

// Reads a DER-encoded ASN.1 IA5String value from |in| and stores the result in
// |out| as ASCII, returning true if successful.
[[nodiscard]] OPENSSL_EXPORT bool ParseIA5String(Input in, std::string *out);

// Reads a DER-encoded ASN.1 VisibleString value from |in| and stores the result
// in |out| as ASCII, returning true if successful.
[[nodiscard]] OPENSSL_EXPORT bool ParseVisibleString(Input in,
                                                     std::string *out);

// Reads a DER-encoded ASN.1 PrintableString value from |in| and stores the
// result in |out| as ASCII, returning true if successful.
[[nodiscard]] OPENSSL_EXPORT bool ParsePrintableString(Input in,
                                                       std::string *out);

// Reads a DER-encoded ASN.1 TeletexString value from |in|, treating it as
// Latin-1, and stores the result in |out| as UTF-8, returning true if
// successful.
//
// This is for compatibility with legacy implementations that would use Latin-1
// encoding but tag it as TeletexString.
[[nodiscard]] OPENSSL_EXPORT bool ParseTeletexStringAsLatin1(Input in,
                                                             std::string *out);

// Reads a DER-encoded ASN.1 UniversalString value from |in| and stores the
// result in |out| as UTF-8, returning true if successful.
[[nodiscard]] OPENSSL_EXPORT bool ParseUniversalString(Input in,
                                                       std::string *out);

// Reads a DER-encoded ASN.1 BMPString value from |in| and stores the
// result in |out| as UTF-8, returning true if successful.
[[nodiscard]] OPENSSL_EXPORT bool ParseBmpString(Input in, std::string *out);

}  // namespace der
BSSL_NAMESPACE_END

#endif  // BSSL_DER_PARSE_VALUES_H_
