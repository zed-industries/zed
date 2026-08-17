// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_CUSTOM_HANDLERS_PROTOCOL_HANDLER_UTILS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_CUSTOM_HANDLERS_PROTOCOL_HANDLER_UTILS_H_

#include <string_view>

#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/security/protocol_handler_security_level.h"

class GURL;

namespace blink {

enum class URLSyntaxErrorCode {
  // The URL syntax is valid.
  kNoError,
  // The URL does not contain "%s".
  kMissingToken,
  // The URL parsing fails, according to the HTML specification:
  // https://html.spec.whatwg.org/multipage/urls-and-fetching.html#parse-a-url
  kInvalidUrl,
};

// This function returns whether the specified scheme is valid as a protocol
// handler parameter, as described in steps 1. and 2. of the HTML specification:
// https://html.spec.whatwg.org/multipage/system-state.html#normalize-protocol-handler-parameters
//
// The allow_ext_prefix parameter indicates whether the "ext+" prefix should be
// considered valid for custom schemes. This is to allow custom schemes that
// are reserved for browser extensions, similar to what Mozilla implements:
// https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json/protocol_handlers
//
// The out parameter has_custom_scheme_prefix is set to whether the scheme
// starts with a prefix indicating a custom scheme i.e. an ASCII case
// insensitive match to the string "web+" (or alternatively "ext+" if allowed).
bool BLINK_COMMON_EXPORT
IsValidCustomHandlerScheme(std::string_view scheme,
                           ProtocolHandlerSecurityLevel security_level,
                           bool* has_custom_scheme_prefix = nullptr);

// This function returns whether the specified url has a valid URL syntax as a
// protocol handler parameter, as described in steps 3., 4. and 5. of the HTML
// specification:
// https://html.spec.whatwg.org/multipage/system-state.html#normalize-protocol-handler-parameters
//
// The returned error code would be useful to determine the error message, but
// the spec states that it should throw a SyntaxError DOMException.
URLSyntaxErrorCode BLINK_COMMON_EXPORT
IsValidCustomHandlerURLSyntax(const GURL& full_url,
                              const std::string_view& user_url);
URLSyntaxErrorCode BLINK_COMMON_EXPORT
IsValidCustomHandlerURLSyntax(const GURL& full_url);

// This function returns whether the specified URL is allowed as a protocol
// handler parameter, as described in steps 6 and 7 (except same origin) of the
// HTML specification:
// https://html.spec.whatwg.org/multipage/system-state.html#normalize-protocol-handler-parameters
bool BLINK_COMMON_EXPORT
IsAllowedCustomHandlerURL(const GURL& url,
                          ProtocolHandlerSecurityLevel security_level);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_CUSTOM_HANDLERS_PROTOCOL_HANDLER_UTILS_H_
