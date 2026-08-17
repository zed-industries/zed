// Copyright 2011 The Chromium Authors
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

#ifndef BSSL_PKI_PEM_H_
#define BSSL_PKI_PEM_H_

#include <stddef.h>

#include <string>
#include <string_view>
#include <vector>

#include <openssl/base.h>

BSSL_NAMESPACE_BEGIN

// PEMTokenizer is a utility class for the parsing of data encapsulated
// using RFC 1421, Privacy Enhancement for Internet Electronic Mail. It
// does not implement the full specification, most notably it does not
// support the Encapsulated Header Portion described in Section 4.4.
class OPENSSL_EXPORT PEMTokenizer {
 public:
  // Create a new PEMTokenizer that iterates through |str| searching for
  // instances of PEM encoded blocks that are of the |allowed_block_types|.
  // |str| must remain valid for the duration of the PEMTokenizer.
  PEMTokenizer(std::string_view str,
               const std::vector<std::string> &allowed_block_types);

  PEMTokenizer(const PEMTokenizer &) = delete;
  PEMTokenizer &operator=(const PEMTokenizer &) = delete;

  ~PEMTokenizer();

  // Attempts to decode the next PEM block in the string. Returns false if no
  // PEM blocks can be decoded. The decoded PEM block will be available via
  // data().
  bool GetNext();

  // Returns the PEM block type (eg: CERTIFICATE) of the last successfully
  // decoded PEM block.
  // GetNext() must have returned true before calling this method.
  const std::string &block_type() const { return block_type_; }

  // Returns the raw, Base64-decoded data of the last successfully decoded
  // PEM block.
  // GetNext() must have returned true before calling this method.
  const std::string &data() const { return data_; }

 private:
  void Init(std::string_view str,
            const std::vector<std::string> &allowed_block_types);

  // A simple cache of the allowed PEM header and footer for a given PEM
  // block type, so that it is only computed once.
  struct PEMType;

  // The string to search, which must remain valid for as long as this class
  // is around.
  std::string_view str_;

  // The current position within |str_| that searching should begin from,
  // or std::string_view::npos if iteration is complete
  std::string_view::size_type pos_;

  // The type of data that was encoded, as indicated in the PEM
  // Pre-Encapsulation Boundary (eg: CERTIFICATE, PKCS7, or
  // PRIVACY-ENHANCED MESSAGE).
  std::string block_type_;

  // The types of PEM blocks that are allowed. PEM blocks that are not of
  // one of these types will be skipped.
  std::vector<PEMType> block_types_;

  // The raw (Base64-decoded) data of the last successfully decoded block.
  std::string data_;
};

// Encodes |data| in the encapsulated message format described in RFC 1421,
// with |type| as the PEM block type (eg: CERTIFICATE).
OPENSSL_EXPORT std::string PEMEncode(std::string_view data,
                                     const std::string &type);

BSSL_NAMESPACE_END

#endif  // BSSL_PKI_PEM_H_
