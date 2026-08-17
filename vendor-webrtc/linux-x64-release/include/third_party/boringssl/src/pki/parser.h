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

#ifndef BSSL_DER_PARSER_H_
#define BSSL_DER_PARSER_H_

#include <stdint.h>

#include <optional>

#include <openssl/base.h>
#include <openssl/bytestring.h>

#include "input.h"

BSSL_NAMESPACE_BEGIN
namespace der {

class BitString;
struct GeneralizedTime;

// Parses a DER-encoded ASN.1 structure. DER (distinguished encoding rules)
// encodes each data value with a tag, length, and value (TLV). The tag
// indicates the type of the ASN.1 value. Depending on the type of the value,
// it could contain arbitrary bytes, so the length of the value is encoded
// after the tag and before the value to indicate how many bytes of value
// follow. DER also defines how the values are encoded for particular types.
//
// This Parser places a few restrictions on the DER encoding it can parse. The
// largest restriction is that it only supports tags which have a tag number
// no greater than 30 - these are the tags that fit in a single octet. The
// second restriction is that the maximum length for a value that can be parsed
// is 4GB. Both of these restrictions should be fine for any reasonable input.
//
// The Parser class is mainly focused on parsing the TLV structure of DER
// encoding, and does not directly handle parsing primitive values (other
// functions in the bssl::der namespace are provided for this.) When a Parser
// is created, it is passed in a reference to the encoded data. Because the
// encoded data is not owned by the Parser, the data cannot change during the
// lifespan of the Parser. The Parser functions by keeping a pointer to the
// current TLV which starts at the beginning of the input and advancing through
// the input as each TLV is read. As such, a Parser instance is thread-unsafe.
//
// Most methods for using the Parser write the current tag and/or value to
// the output parameters provided and then advance the input to the next TLV.
// None of the methods explicitly expose the length because it is part of the
// value. All methods return a boolean indicating whether there was a parsing
// error with the current TLV.
//
// Some methods are provided in the Parser class as convenience to both read
// the current TLV from the input and also parse the DER encoded value,
// converting it to a corresponding C++ type. These methods simply combine
// ReadTag() with the appropriate ParseType() free function.
//
// The design of DER encoding allows for nested data structures with
// constructed values, where the value is a series of TLVs. The Parser class
// is not designed to traverse through a nested encoding from a single object,
// but it does facilitate parsing nested data structures through the
// convenience methods ReadSequence() and the more general ReadConstructed(),
// which provide the user with another Parser object to traverse the next
// level of TLVs.
//
// For a brief example of how to use the Parser, suppose we have the following
// ASN.1 type definition:
//
//   Foo ::= SEQUENCE {
//     bar OCTET STRING OPTIONAL,
//     quux OCTET STRING }
//
// If we have a DER-encoded Foo in an Input |encoded_value|, the
// following code shows an example of how to parse the quux field from the
// encoded data.
//
//   bool ReadQuux(Input encoded_value, Input* quux_out) {
//     Parser parser(encoded_value);
//     Parser foo_parser;
//     if (!parser.ReadSequence(&foo_parser))
//       return false;
//     if (!foo_parser->SkipOptionalTag(kOctetString))
//       return false;
//     if (!foo_parser->ReadTag(kOctetString, quux_out))
//       return false;
//     return true;
//   }
class OPENSSL_EXPORT Parser {
 public:
  // Default constructor; equivalent to calling Parser(Input()). This only
  // exists so that a Parser can be stack allocated and passed in to
  // ReadConstructed() and similar methods.
  Parser();

  // Creates a parser to parse over the data represented by input. This class
  // assumes that the underlying data will not change over the lifetime of
  // the Parser object.
  explicit Parser(Input input);

  Parser(const Parser &) = default;
  Parser &operator=(const Parser &) = default;

  // Returns whether there is any more data left in the input to parse. This
  // does not guarantee that the data is parseable.
  bool HasMore();

  // Reads the current TLV from the input and advances. If the tag or length
  // encoding for the current value is invalid, this method returns false and
  // does not advance the input. Otherwise, it returns true, putting the
  // read tag in |tag| and the value in |out|.
  [[nodiscard]] bool ReadTagAndValue(CBS_ASN1_TAG *tag, Input *out);

  // Reads the current TLV from the input and advances. Unlike ReadTagAndValue
  // where only the value is put in |out|, this puts the raw bytes from the
  // tag, length, and value in |out|.
  [[nodiscard]] bool ReadRawTLV(Input *out);

  // Basic methods for reading or skipping the current TLV, with an
  // expectation of what the current tag should be. It should be possible
  // to parse any structure with these 4 methods; convenience methods are also
  // provided to make some cases easier.

  // If the current tag in the input is |tag|, it puts the corresponding value
  // in |out| and advances the input to the next TLV. If the current tag is
  // something else, then |out| is set to nullopt and the input is not
  // advanced. Like ReadTagAndValue, it returns false if the encoding is
  // invalid and does not advance the input.
  [[nodiscard]] bool ReadOptionalTag(CBS_ASN1_TAG tag, std::optional<Input> *out);

  // If the current tag in the input is |tag|, it puts the corresponding value
  // in |out|, sets |was_present| to true, and advances the input to the next
  // TLV. If the current tag is something else, then |was_present| is set to
  // false and the input is not advanced. Like ReadTagAndValue, it returns
  // false if the encoding is invalid and does not advance the input.
  // DEPRECATED: use the std::optional version above in new code.
  // TODO(mattm): convert the existing callers and remove this override.
  [[nodiscard]] bool ReadOptionalTag(CBS_ASN1_TAG tag, Input *out, bool *was_present);

  // Like ReadOptionalTag, but the value is discarded.
  [[nodiscard]] bool SkipOptionalTag(CBS_ASN1_TAG tag, bool *was_present);

  // If the current tag matches |tag|, it puts the current value in |out|,
  // advances the input, and returns true. Otherwise, it returns false.
  [[nodiscard]] bool ReadTag(CBS_ASN1_TAG tag, Input *out);

  // Advances the input and returns true if the current tag matches |tag|;
  // otherwise it returns false.
  [[nodiscard]] bool SkipTag(CBS_ASN1_TAG tag);

  // Convenience methods to combine parsing the TLV with parsing the DER
  // encoding for a specific type.

  // Reads the current TLV from the input, checks that the tag matches |tag|
  // and is a constructed tag, and creates a new Parser from the value.
  [[nodiscard]] bool ReadConstructed(CBS_ASN1_TAG tag, Parser *out);

  // A more specific form of ReadConstructed that expects the current tag
  // to be 0x30 (SEQUENCE).
  [[nodiscard]] bool ReadSequence(Parser *out);

  // Expects the current tag to be kInteger, and calls ParseUint8 on the
  // current value. Note that DER-encoded integers are arbitrary precision,
  // so this method will fail for valid input that represents an integer
  // outside the range of an uint8_t.
  //
  // Note that on failure the Parser is left in an undefined state (the
  // input may or may not have been advanced).
  [[nodiscard]] bool ReadUint8(uint8_t *out);

  // Expects the current tag to be kInteger, and calls ParseUint64 on the
  // current value. Note that DER-encoded integers are arbitrary precision,
  // so this method will fail for valid input that represents an integer
  // outside the range of an uint64_t.
  //
  // Note that on failure the Parser is left in an undefined state (the
  // input may or may not have been advanced).
  [[nodiscard]] bool ReadUint64(uint64_t *out);

  // Reads a BIT STRING. On success returns BitString. On failure, returns
  // std::nullopt.
  //
  // Note that on failure the Parser is left in an undefined state (the
  // input may or may not have been advanced).
  [[nodiscard]] std::optional<BitString> ReadBitString();

  // Reads a GeneralizeTime. On success fills |out| and returns true.
  //
  // Note that on failure the Parser is left in an undefined state (the
  // input may or may not have been advanced).
  [[nodiscard]] bool ReadGeneralizedTime(GeneralizedTime *out);

  // Lower level methods. The previous methods couple reading data from the
  // input with advancing the Parser's internal pointer to the next TLV; these
  // lower level methods decouple those two steps into methods that read from
  // the current TLV and a method that advances the internal pointer to the
  // next TLV.

  // Reads the current TLV from the input, putting the tag in |tag| and the raw
  // value in |out|, but does not advance the input. Returns true if the tag
  // and length are successfully read and the output exists.
  [[nodiscard]] bool PeekTagAndValue(CBS_ASN1_TAG *tag, Input *out);

  // Advances the input to the next TLV. This method only needs to be called
  // after PeekTagAndValue; all other methods will advance the input if they
  // read something.
  bool Advance();

 private:
  CBS cbs_;
  size_t advance_len_ = 0;
};

}  // namespace der
BSSL_NAMESPACE_END

#endif  // BSSL_DER_PARSER_H_
