/*
 * Copyright (C) 2009 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_STRING_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_STRING_H_

#include <cstring>
#include <limits>
#include <optional>
#include <string>
#include <string_view>

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/public/platform/web_common.h"

#if INSIDE_BLINK
#include "third_party/blink/renderer/platform/wtf/forward.h"  // nogncheck
#endif

namespace WTF {
#if INSIDE_BLINK
class String;
#endif
class StringImpl;
}

namespace blink {

// Use either one of static methods to convert ASCII, Latin1, UTF-8 or
// UTF-16 string into WebString:
//
// * WebString::FromASCII(std::string_view ascii)
// * WebString::FromLatin1(std::string_view latin1)
// * WebString::FromUTF8(std::string_view utf8)
// * WebString::FromUTF16(std::optional<std::u16string_view> utf16)
//
// Similarly, use either of following methods to convert WebString to
// ASCII, Latin1, UTF-8 or UTF-16:
//
// * webstring.Ascii()
// * webstring.Latin1()
// * webstring.Utf8()
// * webstring.Utf16()
// * WebString::ToOptionalString16(webstring)
//
// Note that if you need to convert the UTF8 string converted from WebString
// back to WebString with FromUTF8() you may want to specify Strict
// UTF8ConversionMode when you call Utf8(), as FromUTF8 rejects strings
// with invalid UTF8 characters.
//
// Some types like GURL and base::FilePath can directly take either utf-8 or
// utf-16 strings. Use following methods to convert WebString to/from GURL or
// FilePath rather than going through intermediate string types:
//
// * GURL WebStringToGURL(const WebString&)
// * base::FilePath WebStringToFilePath(const WebString&)
// * WebString FilePathToWebString(const base::FilePath&);
//
// It is inexpensive to copy a WebString object.
//
class BLINK_PLATFORM_EXPORT WebString {
 public:
  enum class UTF8ConversionMode {
    // Ignores errors for invalid characters.
    kLenient,
    // Errors out on invalid characters, returns null string.
    kStrict,
    // Replace invalid characters with 0xFFFD.
    // (This is the same conversion mode as base::UTF16ToUTF8)
    kStrictReplacingErrorsWithFFFD,
  };

  ~WebString();
  WebString();
  explicit WebString(std::u16string_view s);

  WebString(const WebString&);
  WebString(WebString&&);

  WebString& operator=(const WebString&);
  WebString& operator=(WebString&&);

  void Reset();

  bool Equals(const WebString&) const;
  bool Equals(std::string_view characters) const;
  bool Equals(const char* characters) const {
    return Equals(
        characters ? std::string_view(characters) : std::string_view());
  }

  size_t Find(const WebString&) const;
  size_t Find(std::string_view characters) const;

  size_t length() const;

  bool IsEmpty() const { return !length(); }
  bool IsNull() const { return !impl_; }

  std::string Utf8(UTF8ConversionMode = UTF8ConversionMode::kLenient) const;

  WebString Substring(size_t pos,
                      size_t len = std::numeric_limits<size_t>::max()) const;

  static WebString FromUTF8(std::string_view s);

  std::u16string Utf16() const;

  static WebString FromUTF16(std::optional<std::u16string_view>);

  static std::optional<std::u16string> ToOptionalString16(const WebString& s) {
    return s.IsNull() ? std::nullopt : std::make_optional(s.Utf16());
  }

  std::string Latin1() const;

  static WebString FromLatin1(std::string_view s);

  // This asserts if the string contains non-ascii characters.
  // Use this rather than calling base::UTF16ToASCII() which always incurs
  // (likely unnecessary) string16 conversion.
  std::string Ascii() const;

  // Use this rather than calling base::IsStringASCII().
  bool ContainsOnlyASCII() const;

  // Does same as FromLatin1 but asserts if the given string has non-ascii char.
  static WebString FromASCII(std::string_view);

  template <int N>
  WebString(const char (&data)[N])
      : WebString(FromUTF8(std::string_view(data, N - 1))) {}

  template <int N>
  WebString& operator=(const char (&data)[N]) {
    *this = FromUTF8(std::string_view(data, N - 1));
    return *this;
  }

  bool operator<(const WebString& other) const;

#if INSIDE_BLINK
  WebString(const WTF::String&);
  WebString& operator=(const WTF::String&);
  operator WTF::String() const;

  operator WTF::StringView() const;

  WebString(const WTF::AtomicString&);
  WebString& operator=(const WTF::AtomicString&);
  operator WTF::AtomicString() const;
#endif

 private:
  bool Is8Bit() const;

  scoped_refptr<WTF::StringImpl> impl_;
};

#if INSIDE_BLINK
// This can be used as a projection, e.g. when calling base::ToVector().
inline WebString ToWebString(const WTF::String& s) {
  return WebString(s);
}
// To convert a std::vector<WebString> to WTF::Vector<String>, use
//   WTF::Vector<String>(std_vector_web_string).
#endif

inline bool operator==(const WebString& a, const char* b) {
  return a.Equals(b);
}

inline bool operator!=(const WebString& a, const char* b) {
  return !(a == b);
}

inline bool operator==(const char* a, const WebString& b) {
  return b == a;
}

inline bool operator!=(const char* a, const WebString& b) {
  return !(b == a);
}

inline bool operator==(const WebString& a, const WebString& b) {
  return a.Equals(b);
}

inline bool operator!=(const WebString& a, const WebString& b) {
  return !(a == b);
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_STRING_H_
