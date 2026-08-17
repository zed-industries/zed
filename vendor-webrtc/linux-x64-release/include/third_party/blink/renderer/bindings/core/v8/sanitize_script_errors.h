// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SANITIZE_SCRIPT_ERRORS_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SANITIZE_SCRIPT_ERRORS_H_

namespace blink {

// https://html.spec.whatwg.org/C/webappapis.html#muted-errors
// "A boolean which, if true, means that error information will not be provided
// for errors in this script. This is used to mute errors for cross-origin
// scripts, since that can leak private information."
//
// For example:
//  - A classic script from a cross-origin url without a "crossorigin" attribute
//    has "kSanitize" flag.
//  - A classic script from a cross-origin url with a "crossorigin" attribute
//    has "kDoNotSanitize" flag.
//  - A classic script from a same-origin url has "kDoNotSanitize" flag.
//
// "Muting" here usually means hiding error content, not hiding error
// existence. When an error is muted, a sanitized error instance is dispatched
// instead of the original error. But in the promise unhandled rejection case,
// error existence is hidden when kSanitize is specified.
enum class SanitizeScriptErrors {
  // "muted errors" is false
  kDoNotSanitize,
  // "muted errors" is true
  kSanitize
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_SANITIZE_SCRIPT_ERRORS_H_
