
//*********************************************************************
//* C_Base64 - a simple base64 encoder and decoder.
//*
//*     Copyright (c) 1999, Bob Withers - bwit@pobox.com
//*
//* This code may be freely used for any purpose, either personal
//* or commercial, provided the authors copyright notice remains
//* intact.
//*********************************************************************

#ifndef RTC_BASE_THIRD_PARTY_BASE64_BASE64_H_
#define RTC_BASE_THIRD_PARTY_BASE64_BASE64_H_

#include <cstddef>
#include <cstdint>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"

namespace webrtc {

class Base64 {
 public:
  enum DecodeOption {
    DO_PARSE_STRICT = 1,  // Parse only base64 characters
    DO_PARSE_WHITE = 2,   // Parse only base64 and whitespace characters
    DO_PARSE_ANY = 3,     // Parse all characters
    DO_PARSE_MASK = 3,

    DO_PAD_YES = 4,  // Padding is required
    DO_PAD_ANY = 8,  // Padding is optional
    DO_PAD_NO = 12,  // Padding is disallowed
    DO_PAD_MASK = 12,

    DO_TERM_BUFFER = 16,  // Must termiante at end of buffer
    DO_TERM_CHAR = 32,    // May terminate at any character boundary
    DO_TERM_ANY = 48,     // May terminate at a sub-character bit offset
    DO_TERM_MASK = 48,

    // Strictest interpretation
    DO_STRICT = DO_PARSE_STRICT | DO_PAD_YES | DO_TERM_BUFFER,

    DO_LAX = DO_PARSE_ANY | DO_PAD_ANY | DO_TERM_CHAR,
  };
  typedef int DecodeFlags;

  static bool IsBase64Char(char ch);

  // Get the char next to the `ch` from the Base64Table.
  // If the `ch` is the last one in the Base64Table then returns
  // the first one from the table.
  // Expects the `ch` be a base64 char.
  // The result will be saved in `next_ch`.
  // Returns true on success.
  static bool GetNextBase64Char(char ch, char* next_ch);

  // Determines whether the given string consists entirely of valid base64
  // encoded characters.
  static bool IsBase64Encoded(absl::string_view str);

  static void EncodeFromArray(const void* data,
                              size_t len,
                              std::string* result);
  static bool DecodeFromArray(const char* data,
                              size_t len,
                              DecodeFlags flags,
                              std::string* result,
                              size_t* data_used);
  static bool DecodeFromArray(const char* data,
                              size_t len,
                              DecodeFlags flags,
                              std::vector<char>* result,
                              size_t* data_used);
  static bool DecodeFromArray(const char* data,
                              size_t len,
                              DecodeFlags flags,
                              std::vector<uint8_t>* result,
                              size_t* data_used);

  // Convenience Methods
  static inline std::string Encode(absl::string_view data) {
    std::string result;
    EncodeFromArray(data.data(), data.size(), &result);
    return result;
  }
  static inline std::string Decode(absl::string_view data, DecodeFlags flags) {
    std::string result;
    DecodeFromArray(data.data(), data.size(), flags, &result, nullptr);
    return result;
  }
  static inline bool Decode(absl::string_view data,
                            DecodeFlags flags,
                            std::string* result,
                            size_t* data_used) {
    return DecodeFromArray(data.data(), data.size(), flags, result, data_used);
  }
  static inline bool Decode(absl::string_view data,
                            DecodeFlags flags,
                            std::vector<char>* result,
                            size_t* data_used) {
    return DecodeFromArray(data.data(), data.size(), flags, result, data_used);
  }

 private:
  static const char Base64Table[];
  static const unsigned char DecodeTable[];

  static size_t GetNextQuantum(DecodeFlags parse_flags,
                               bool illegal_pads,
                               const char* data,
                               size_t len,
                               size_t* dpos,
                               unsigned char qbuf[4],
                               bool* padded);
  template <typename T>
  static bool DecodeFromArrayTemplate(const char* data,
                                      size_t len,
                                      DecodeFlags flags,
                                      T* result,
                                      size_t* data_used);
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::Base64;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif /* RTC_BASE_THIRD_PARTY_BASE64_BASE64_H_ */
