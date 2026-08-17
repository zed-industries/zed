/*
 * Copyright (C) 2006 Alexey Proskuryakov (ap@webkit.org)
 * Copyright (C) 2009 Google Inc. All rights reserved.
 * Copyright (C) 2011 Apple Inc. All Rights Reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 * 3.  Neither the name of Apple Computer, Inc. ("Apple") nor the names of
 *     its contributors may be used to endorse or promote products derived
 *     from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE AND ITS CONTRIBUTORS "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE OR ITS CONTRIBUTORS BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_NETWORK_HTTP_PARSERS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_NETWORK_HTTP_PARSERS_H_

#include <stdint.h>

#include <memory>
#include <optional>

#include "base/time/time.h"
#include "services/network/public/mojom/content_security_policy.mojom-blink-forward.h"
#include "services/network/public/mojom/no_vary_search.mojom-blink-forward.h"
#include "services/network/public/mojom/parsed_headers.mojom-blink-forward.h"
#include "services/network/public/mojom/sri_message_signature.mojom-blink-forward.h"
#include "services/network/public/mojom/timing_allow_origin.mojom-blink.h"
#include "third_party/blink/renderer/platform/network/content_security_policy_response_headers.h"
#include "third_party/blink/renderer/platform/network/parsed_content_type.h"
#include "third_party/blink/renderer/platform/network/server_timing_header.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/weborigin/security_origin.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"
#include "third_party/blink/renderer/platform/wtf/text/case_folding_hash.h"
#include "third_party/blink/renderer/platform/wtf/text/string_hash.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class HTTPHeaderMap;
class KURL;
class ResourceResponse;
class UseCounter;

enum ContentTypeOptionsDisposition {
  kContentTypeOptionsNone,
  kContentTypeOptionsNosniff
};

using CommaDelimitedHeaderSet = HashSet<String, CaseFoldingHashTraits<String>>;

struct CacheControlHeader {
  DISALLOW_NEW();
  bool parsed : 1;
  bool contains_no_cache : 1;
  bool contains_no_store : 1;
  bool contains_must_revalidate : 1;
  std::optional<base::TimeDelta> max_age;
  std::optional<base::TimeDelta> stale_while_revalidate;

  CacheControlHeader()
      : parsed(false),
        contains_no_cache(false),
        contains_no_store(false),
        contains_must_revalidate(false) {}
};

using ServerTimingHeaderVector = Vector<std::unique_ptr<ServerTimingHeader>>;

PLATFORM_EXPORT bool IsContentDispositionAttachment(const String&);
PLATFORM_EXPORT bool IsValidHTTPHeaderValue(const String&);
// Checks whether the given string conforms to the |token| ABNF production
// defined in the RFC 7230 or not.
//
// The ABNF is for validating octets, but this method takes a String instance
// for convenience which consists of Unicode code points. When this method sees
// non-ASCII characters, it just returns false.
PLATFORM_EXPORT bool IsValidHTTPToken(const String&);
// |matcher| specifies a function to check a whitespace character. if |nullptr|
// is specified, ' ' and '\t' are treated as whitespace characters.
PLATFORM_EXPORT bool ParseHTTPRefresh(const String& refresh,
                                      WTF::CharacterMatchFunctionPtr matcher,
                                      base::TimeDelta& delay,
                                      String& url);
PLATFORM_EXPORT std::optional<base::Time> ParseDate(const String&, UseCounter&);

// Given a Media Type (like "foo/bar; baz=gazonk" - usually from the
// 'Content-Type' HTTP header), extract and return the "type/subtype" portion
// ("foo/bar").
//
// Note:
// - This function does not in any way check that the "type/subtype" pair
//   is well-formed.
// - OWSes at the head and the tail of the region before the first semicolon
//   are trimmed.
PLATFORM_EXPORT AtomicString ExtractMIMETypeFromMediaType(const AtomicString&);

PLATFORM_EXPORT AtomicString MinimizedMIMEType(const AtomicString&);

PLATFORM_EXPORT CacheControlHeader
ParseCacheControlDirectives(const AtomicString& cache_control_header,
                            const AtomicString& pragma_header);
PLATFORM_EXPORT void ParseCommaDelimitedHeader(const String& header_value,
                                               CommaDelimitedHeaderSet&);

PLATFORM_EXPORT ContentTypeOptionsDisposition
ParseContentTypeOptionsHeader(const String& header);

// Returns true and stores the position of the end of the headers to |*end|
// if the headers part ends in |bytes[0..size]|. Returns false otherwise.
PLATFORM_EXPORT bool ParseMultipartFormHeadersFromBody(
    base::span<const uint8_t> bytes,
    HTTPHeaderMap* header_fields,
    wtf_size_t* end);

// Returns true and stores the position of the end of the headers to |*end|
// if the headers part ends in |bytes[0..size]|. Returns false otherwise.
PLATFORM_EXPORT bool ParseMultipartHeadersFromBody(
    base::span<const uint8_t> bytes,
    ResourceResponse*,
    wtf_size_t* end);

// Extracts the values in a Content-Range header and returns true if all three
// values are present and valid for a 206 response; otherwise returns false.
// The following values will be outputted:
// |*first_byte_position| = inclusive position of the first byte of the range
// |*last_byte_position| = inclusive position of the last byte of the range
// |*instance_length| = size in bytes of the object requested
// If this method returns false, then all of the outputs will be -1.
PLATFORM_EXPORT bool ParseContentRangeHeaderFor206(const String& content_range,
                                                   int64_t* first_byte_position,
                                                   int64_t* last_byte_position,
                                                   int64_t* instance_length);

PLATFORM_EXPORT std::unique_ptr<ServerTimingHeaderVector>
ParseServerTimingHeader(const String&);

// Build ParsedHeaders from raw headers. This is the same as
// network::PopulateParsedHeaders(headers, url) but using blink types.
PLATFORM_EXPORT network::mojom::blink::ParsedHeadersPtr ParseHeaders(
    const String& raw_headers,
    const KURL& url);

// Parses Content Security Policies. This is the same as
// network::ParseContentSecurityPolicies but using blink types.
PLATFORM_EXPORT
Vector<network::mojom::blink::ContentSecurityPolicyPtr>
ParseContentSecurityPolicies(
    const String& raw_policies,
    network::mojom::blink::ContentSecurityPolicyType type,
    network::mojom::blink::ContentSecurityPolicySource source,
    const KURL& base_url);

// Parses Content Security Policies. This is the same as
// network::ParseContentSecurityPolicies but using blink types.
PLATFORM_EXPORT
Vector<network::mojom::blink::ContentSecurityPolicyPtr>
ParseContentSecurityPolicies(
    const String& raw_policies,
    network::mojom::blink::ContentSecurityPolicyType type,
    network::mojom::blink::ContentSecurityPolicySource source,
    const SecurityOrigin& self_origin);

// Parses Content Security Policies headers. This uses
// network::ParseContentSecurityPolicies and translates to blink types.
PLATFORM_EXPORT
Vector<network::mojom::blink::ContentSecurityPolicyPtr>
ParseContentSecurityPolicyHeaders(
    const ContentSecurityPolicyResponseHeaders& headers);

// Parses SRI-relevant HTTP Message Signature headers. This wraps
// network::ParseSRIMessageSignaturesFromHeaders with blink types.
PLATFORM_EXPORT
network::mojom::blink::SRIMessageSignaturesPtr
ParseSRIMessageSignaturesFromHeaders(const String& raw_headers);

PLATFORM_EXPORT
network::mojom::blink::TimingAllowOriginPtr ParseTimingAllowOrigin(
    const String& header_value);

PLATFORM_EXPORT
network::mojom::blink::NoVarySearchWithParseErrorPtr ParseNoVarySearch(
    const String& header_value);

PLATFORM_EXPORT
String GetNoVarySearchHintConsoleMessage(
    const network::mojom::NoVarySearchParseError& error);
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_NETWORK_HTTP_PARSERS_H_
