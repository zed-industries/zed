// Copyright 2016 Google Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
////////////////////////////////////////////////////////////////////////////////

#ifndef COMPACT_ENC_DET_COMPACT_ENC_DET_H_
#define COMPACT_ENC_DET_COMPACT_ENC_DET_H_

#include "util/encodings/encodings.h"  // for Encoding
#include "util/languages/languages.h"  // for Language

#include <string.h>

namespace CompactEncDet {
  // We may want different statistics, depending on whether the text being
  // identfied is from the web, from email, etc.  This is currently ignored,
  // except WEB_CORPUS enables ignoring chars inside tags.
  enum TextCorpusType {
    WEB_CORPUS,
    XML_CORPUS,
    QUERY_CORPUS,       // Use this for vanilla plaintext
    EMAIL_CORPUS,
    NUM_CORPA,          // always last
  };

  // Scan raw bytes and detect most likely encoding
  // Design goals:
  //   Skip over big initial stretches of seven-bit ASCII bytes very quickly
  //   Thread safe
  //   Works equally well on
  //    50-byte queries,
  //    5000-byte email and
  //    50000-byte web pages
  // Length 0 input returns ASCII (aka ISO-8859-1 or Latin1)
  //
  // Inputs: text and text_length
  //  web page's url (preferred) or just
  //    top-level domain name (e.g. "com") or NULL as a hint
  //  web page's HTTPheader charset= string (e.g. "Latin1") or NULL as a hint
  //  web page's <meta> tag charset= string (e.g. "utf-8") or NULL as a hint
  //  an Encoding or UNKNOWN_ENCODING as a hint
  //  a Language or UNKNOWN_LANGUAGE as a hint
  //  corpus type from the list above. Currently ignored; may select
  //    different probability tables in the future
  //  ignore_7bit if true says to NOT return the pure seven-bit encodings
  //    ISO-2022-JP (aka JIS), ISO-2022-CN, ISO-2022-KR, HZ, and UTF-7.
  //    This may save a little scoring time on pure printable ASCII input text
  // Outputs: bytes_consumed says how much of text_length was actually examined
  //  is_reliable set true if the returned encoding is at least 2**10 time more
  //  probable then the second-best encoding
  // Return value: the most likely encoding for the input text
  //
  // Setting ignore_7bit_mail_encodings effectively turns off detection of
  // UTF-7, HZ, and ISO-2022-xx. It is recommended that this flag be true
  // when corpus_type is QUERY_CORPUS.
  Encoding DetectEncoding(
      const char* text, int text_length, const char* url_hint,
      const char* http_charset_hint, const char* meta_charset_hint,
      const int encoding_hint,
      const Language language_hint,  // User interface lang
      const TextCorpusType corpus_type, bool ignore_7bit_mail_encodings,
      int* bytes_consumed, bool* is_reliable);

  // Support functions for unit test program
  int BackmapEncodingToRankedEncoding(Encoding enc);
  Encoding TopEncodingOfLangHint(const char* name);
  Encoding TopEncodingOfTLDHint(const char* name);
  Encoding TopEncodingOfCharsetHint(const char* name);
  const char* Version(void);
}      // End namespace CompactEncDet

#endif  // COMPACT_ENC_DET_COMPACT_ENC_DET_H_
