// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_PARSED_SPECIFIER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_PARSED_SPECIFIER_H_

#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

// ParsedSpecifier represents a parsed specifier, either
// - a non-bare specifier, parsed as a KURL as specced by
//   https://html.spec.whatwg.org/#resolve-a-module-specifier or
// - a bare specifier, stored as a String as-is.

// Non-import-maps cases:
// Bare specifiers should be rejected by callers as resolution errors.
// Then ParsedSpecifier represents the result of
// https://html.spec.whatwg.org/#resolve-a-module-specifier
// and behaves just like a KURL via GetUrl().

// Import-maps cases:
// In the import map spec, specifiers are handled mostly as strings,
// occasionally converted to/from URLs.
// In Blink, we pass ParsedSpecifier throughout the import map resolution,
// instead of passing String with occasionally converting to KURL.
// This avoid duplicated URL parsing.
class ParsedSpecifier final {
  STACK_ALLOCATED();

 public:
  // Parse |specifier|, which may be a non-bare or bare specifier.
  // This implements
  // https://html.spec.whatwg.org/#resolve-a-module-specifier
  // but doesn't reject bare specifiers, which should be rejected by callers
  // if needed.
  static ParsedSpecifier Create(const String& specifier, const KURL& base_url);

  enum class Type { kInvalid, kBare, kURL };

  Type GetType() const { return type_; }

  // Returns the string to be used as the key of import maps.
  // This is the bare specifier itself if type is kBare, or
  // serialized URL if type is kURL.
  AtomicString GetImportMapKeyString() const;

  // Returns the URL, if type is kURL, or an null URL otherwise.
  KURL GetUrl() const;

  bool IsValid() const { return GetType() != Type::kInvalid; }

 private:
  // Invalid specifier.
  ParsedSpecifier() : type_(Type::kInvalid) {}
  // Non-bare specifier.
  explicit ParsedSpecifier(const KURL& url) : type_(Type::kURL), url_(url) {}
  // Bare specifier.
  explicit ParsedSpecifier(const String& bare_specifier)
      : type_(Type::kBare), bare_specifier_(bare_specifier) {}

  const Type type_;
  const KURL url_;
  const String bare_specifier_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_PARSED_SPECIFIER_H_
