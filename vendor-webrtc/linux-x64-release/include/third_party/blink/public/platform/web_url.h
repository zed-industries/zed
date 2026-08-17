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

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_URL_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_URL_H_

#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_string.h"
#include "url/third_party/mozilla/url_parse.h"

#if !INSIDE_BLINK
#include "url/gurl.h"
#endif

namespace blink {

class KURL;

class BLINK_PLATFORM_EXPORT WebURL {
 public:
  ~WebURL() = default;

  WebURL() : is_valid_(false) {}

  WebURL(const WebURL& url) = default;

  WebURL& operator=(const WebURL& url) = default;

  const WebString& GetString() const { return string_; }

  const url::Parsed& GetParsed() const { return parsed_; }

  bool IsValid() const { return is_valid_; }

  bool IsEmpty() const { return string_.IsEmpty(); }

  bool IsNull() const { return string_.IsEmpty(); }

  bool ProtocolIs(const char* protocol) const;

#if INSIDE_BLINK
  WebURL(const KURL&);
  WebURL& operator=(const KURL&);
  operator KURL() const;
#else
  WebURL(const GURL& url)
      : string_(WebString::FromUTF8(url.possibly_invalid_spec())),
        parsed_(url.parsed_for_possibly_invalid_spec()),
        is_valid_(url.is_valid()) {}

  WebURL& operator=(const GURL& url) {
    string_ = WebString::FromUTF8(url.possibly_invalid_spec());
    parsed_ = url.parsed_for_possibly_invalid_spec();
    is_valid_ = url.is_valid();
    return *this;
  }

  operator GURL() const {
    return IsNull() ? GURL() : GURL(string_.Utf8(), parsed_, is_valid_);
  }
#endif

 private:
  WebString string_;
  url::Parsed parsed_;
  bool is_valid_;
};

// This can be used as a projection, e.g. when calling base::ToVector().
#if INSIDE_BLINK
inline WebURL ToWebURL(const KURL& url) {
  return WebURL(url);
}
// To convert a std::vector<WebURL> to WTF::Vector<KURL>, use
//   WTF::Vector<KURL>(std_vector_web_url).
#else
inline WebURL ToWebURL(const GURL& url) {
  return WebURL(url);
}
inline GURL ToGURL(const WebURL& url) {
  return GURL(url);
}
#endif

inline bool operator==(const WebURL& a, const WebURL& b) {
  return a.GetString().Equals(b.GetString());
}

inline bool operator!=(const WebURL& a, const WebURL& b) {
  return !(a == b);
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_URL_H_
