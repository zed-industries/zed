/*
 * Copyright (C) 2003, 2004, 2005, 2006, 2007, 2008, 2011, 2012 Apple Inc.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WEBORIGIN_KURL_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WEBORIGIN_KURL_H_

#include <iosfwd>
#include <memory>

#include "base/compiler_specific.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/cross_thread_copier.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/perfetto/include/perfetto/tracing/traced_value_forward.h"
#include "url/third_party/mozilla/url_parse.h"
#include "url/url_canon.h"
#include "url/url_util.h"

// KURL stands for the URL parser in KDE's HTML Widget (KHTML). The name hasn't
// changed since Blink forked WebKit, which in turn forked KHTML.
//
// KURL is Blink's URL class and is the analog to GURL in other Chromium
// code. KURL and GURL both share the same underlying URL parser, whose code is
// located in //url, but KURL is backed by Blink specific WTF::Strings. This
// means that KURLs are usually cheap to copy due to WTF::Strings being
// internally ref-counted. However, please don't copy KURLs if you can use a
// const ref, since the size of the parsed structure and related metadata is
// non-trivial.
//
// KURL also has a few other optimizations, including:
// - Fast comparisons since the string spec is stored as an AtomicString.
// - Cached bit for whether the KURL is http/https
// - Internal reference to the URL protocol (scheme) to avoid String allocation
//   for the callers that require it. Common protocols like http and https are
//   stored as shared static strings.
namespace WTF {
class TextEncoding;
}

class GURL;

namespace blink {

class PLATFORM_EXPORT KURL {
  USING_FAST_MALLOC(KURL);

 public:
  KURL();
  KURL(const KURL&);

  // This should only be used to convert a GURL returned from a layer which
  // operates in the base terms (e.g. from common/* code).
  explicit KURL(const GURL&);

  KURL& operator=(const KURL&);

  // The argument is an absolute URL string. The string is assumed to be
  // output of KURL::GetString() called on a valid KURL object, or
  // indiscernible from such.
  //
  // It is usually best to avoid repeatedly parsing a String, unless memory
  // saving outweigh the possible slow-downs.
  explicit KURL(const String&);

  // Resolves the relative URL with the given base URL. If provided, the
  // TextEncoding is used to encode non-ASCII characters. The base URL can be
  // null or empty, in which case the relative URL will be interpreted as
  // absolute.
  // FIXME: If the base URL is invalid, this always creates an invalid
  // URL. Instead I think it would be better to treat all invalid base URLs
  // the same way we treate null and empty base URLs.
  KURL(const KURL& base, const String& relative);
  KURL(const KURL& base, const String& relative, const WTF::TextEncoding&);

  // For conversions from other structures that have already parsed and
  // canonicalized the URL. The input must be exactly what KURL would have
  // done with the same input.
  KURL(const AtomicString& canonical_string, const url::Parsed&, bool is_valid);

  ~KURL();

  KURL UrlStrippedForUseAsReferrer() const;
  String StrippedForUseAsReferrer() const;
  String StrippedForUseAsHref() const;

  // FIXME: The above functions should be harmonized so that passing a
  // base of null or the empty string gives the same result as the
  // standard String constructor.

  bool IsNull() const;
  bool IsEmpty() const;
  bool IsValid() const;

  // Returns true if this URL has a path. Note that "http://foo.com/" has a
  // path of "/", so this function will return true. Only invalid or
  // non-hierarchical (like "javascript:") URLs will have no path.
  bool HasPath() const;

  // Returns true if you can set the host and port for the URL.
  //
  // Note: this returns true for "filesystem" and false for "blob" currently,
  // due to peculiarities of how schemes are registered in url/ -- neither
  // of these schemes can have hostnames on the outer URL.
  bool CanSetHostOrPort() const;
  bool CanSetPathname() const;

  // Return true if a host can be removed from the URL.
  //
  // URL Standard: https://url.spec.whatwg.org/#host-state
  //
  // > 3.2: Otherwise, if state override is given, buffer is the empty string,
  // > and either url includes credentials or url’s port is non-null, return.
  //
  // Examples:
  //
  // Setting an empty host is allowed:
  //
  // > const url = new URL("git://h/")
  // > url.host = "";
  // > assertEquals(url.href, "git:///");
  //
  // Setting an empty host is disallowed:
  //
  // > const url = new URL("git://u@h/")
  // > url.host = "";
  // > assertEquals(url.href, "git://u@h/");
  bool CanRemoveHost() const;

  // Return true if this URL is hierarchical, which is equivalent to standard
  // URLs.
  //
  // Important note: If kStandardCompliantNonSpecialSchemeURLParsing flag is
  // enabled, returns true also for non-special URLs which don't have an opaque
  // path.
  bool IsHierarchical() const;

  // Return true if this URL is a standard URL.
  bool IsStandard() const;

  // The returned `AtomicString` is guaranteed to consist of only ASCII
  // characters, but may be 8-bit or 16-bit.
  const AtomicString& GetString() const { return string_; }

  String ElidedString() const;

  String Protocol() const;
  StringView Host() const LIFETIME_BOUND;

  // Returns 0 when there is no port or the default port was specified, or the
  // URL is invalid.
  //
  // We treat URLs with out-of-range port numbers as invalid URLs, and they
  // will be rejected by the canonicalizer.
  uint16_t Port() const;
  bool HasPort() const;
  StringView User() const LIFETIME_BOUND;
  StringView Pass() const LIFETIME_BOUND;
  StringView GetPath() const LIFETIME_BOUND;
  // This method handles "parameters" separated by a semicolon.
  StringView LastPathComponent() const LIFETIME_BOUND;
  StringView Query() const LIFETIME_BOUND;
  StringView QueryWithLeadingQuestionMark() const LIFETIME_BOUND;
  StringView FragmentIdentifier() const LIFETIME_BOUND;
  StringView FragmentIdentifierWithLeadingNumberSign() const LIFETIME_BOUND;
  bool HasFragmentIdentifier() const;

  StringView BaseAsString() const LIFETIME_BOUND;

  // Returns true if the current URL's protocol is the same as the StringView
  // argument. The argument must be lower-case.
  bool ProtocolIs(const StringView protocol) const;
  bool ProtocolIsData() const { return ProtocolIs("data"); }
  // This includes at least about:blank and about:srcdoc.
  bool ProtocolIsAbout() const { return ProtocolIs("about"); }
  bool ProtocolIsJavaScript() const;
  bool ProtocolIsInHTTPFamily() const;
  bool IsLocalFile() const;
  bool IsAboutBlankURL() const;   // Is about:blank, ignoring query/ref strings.
  bool IsAboutSrcdocURL() const;  // Is about:srcdoc, ignoring query/ref
                                  // strings..

  bool SetProtocol(const String&);
  void SetHost(const String&);

  void RemovePort();
  void SetPort(uint16_t);
  void SetPort(const String&);

  // Input is like "foo.com" or "foo.com:8000".
  void SetHostAndPort(const String&);

  void SetUser(const String&);
  void SetPass(const String&);

  // If you pass an empty path for HTTP or HTTPS URLs, the resulting path
  // will be "/".
  void SetPath(const String&);

  // The query may begin with a question mark, or, if not, one will be added
  // for you. Setting the query to the empty string will leave a "?" in the
  // URL (with nothing after it). To clear the query, pass a null string.
  void SetQuery(const String&);

  void SetFragmentIdentifier(const String&);
  void RemoveFragmentIdentifier();

  PLATFORM_EXPORT friend bool EqualIgnoringFragmentIdentifier(const KURL&,
                                                              const KURL&);

  unsigned HostStart() const;
  unsigned HostEnd() const;

  unsigned PathStart() const;
  unsigned PathEnd() const;
  unsigned PathAfterLastSlash() const;

  operator const String&() const { return GetString(); }

  const url::Parsed& GetParsed() const { return parsed_; }

  const KURL* InnerURL() const { return inner_url_.get(); }

  bool PotentiallyDanglingMarkup() const {
    return parsed_.potentially_dangling_markup;
  }

  // Returns a GURL with the same properties. This can be used in platform/ and
  // web/. However, in core/ and modules/, this should only be used to pass
  // a GURL to a layer that is expecting one instead of a KURL or a WebURL.
  explicit operator GURL() const;

  void WriteIntoTrace(perfetto::TracedValue context) const;

 private:
  friend struct WTF::HashTraits<blink::KURL>;

  void Init(const KURL& base,
            const String& relative,
            const WTF::TextEncoding* query_encoding);

  bool IsAboutURL(const char* allowed_path) const;

  StringView ComponentStringView(const url::Component&) const;
  String ComponentString(const url::Component&) const;
  StringView StringViewForInvalidComponent() const;

  // If |preserve_validity| is true, refuse to make changes that would make the
  // KURL invalid.
  template <typename CHAR>
  void ReplaceComponents(const url::Replacements<CHAR>&,
                         bool preserve_validity = false);

  void InitInnerURL();
  void InitProtocolMetadata();

  // Asserts that `string_` is an ASCII string in DCHECK builds.
  void AssertStringSpecIsASCII();

  // URL Standard: https://url.spec.whatwg.org/#include-credentials
  bool IncludesCredentials() const {
    return !User().empty() || !Pass().empty();
  }

  // URL Standard: https://url.spec.whatwg.org/#url-opaque-path
  bool HasOpaquePath() const { return parsed_.has_opaque_path; }

  bool is_valid_;
  bool protocol_is_in_http_family_;

  // Keep a separate string for the protocol to avoid copious copies for
  // protocol().
  String protocol_;

  url::Parsed parsed_;
  AtomicString string_;
  std::unique_ptr<KURL> inner_url_;
};

PLATFORM_EXPORT bool operator==(const KURL&, const KURL&);
PLATFORM_EXPORT bool operator==(const KURL&, const String&);
PLATFORM_EXPORT bool operator==(const String&, const KURL&);
PLATFORM_EXPORT bool operator!=(const KURL&, const KURL&);
PLATFORM_EXPORT bool operator!=(const KURL&, const String&);
PLATFORM_EXPORT bool operator!=(const String&, const KURL&);

// Pretty printer for gtest and base/logging.*.  It prepends and appends
// double-quotes, and escapes characters other than ASCII printables.
PLATFORM_EXPORT std::ostream& operator<<(std::ostream&, const KURL&);

PLATFORM_EXPORT bool EqualIgnoringFragmentIdentifier(const KURL&, const KURL&);

PLATFORM_EXPORT const KURL& BlankURL();
PLATFORM_EXPORT const KURL& SrcdocURL();
PLATFORM_EXPORT const KURL& NullURL();

// Functions to do URL operations on strings.
// These are operations that aren't faster on a parsed URL.
// These are also different from the KURL functions in that they don't require
// the string to be a valid and parsable URL.  This is especially important
// because valid javascript URLs are not necessarily considered valid by KURL.

PLATFORM_EXPORT bool ProtocolIs(const String& url, const char* protocol);
PLATFORM_EXPORT bool ProtocolIsJavaScript(const String& url);

PLATFORM_EXPORT bool IsValidProtocol(const String&);

using DecodeURLMode = url::DecodeURLMode;
// Unescapes the given string using URL escaping rules.
//
// DANGER: If the URL has "%00" in it, the resulting string will have embedded
// null characters!
//
// This function is also used to decode javascript: URLs and as a general
// purpose unescaping function.
//
// Caution: Specifying kUTF8OrIsomorphic to the second argument doesn't conform
// to specifications in many cases.
PLATFORM_EXPORT String DecodeURLEscapeSequences(const StringView&,
                                                DecodeURLMode mode);

PLATFORM_EXPORT String EncodeWithURLEscapeSequences(const StringView&);

// Checks an arbitrary string for invalid escape sequences.
//
// A valid percent-encoding is '%' followed by exactly two hex-digits. This
// function returns true if an occurrence of '%' is found and followed by
// anything other than two hex-digits.
PLATFORM_EXPORT bool HasInvalidURLEscapeSequences(const String&);

}  // namespace blink

namespace WTF {

// Defined in kurl_hash.h.
template <>
struct HashTraits<blink::KURL>;

template <>
struct CrossThreadCopier<blink::KURL>
    : public CrossThreadCopierPassThrough<blink::KURL> {
  STATIC_ONLY(CrossThreadCopier);
};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WEBORIGIN_KURL_H_
