// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_SAFE_URL_PATTERN_MOJOM_TRAITS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_SAFE_URL_PATTERN_MOJOM_TRAITS_H_

#include <string_view>

#include "mojo/public/cpp/bindings/struct_traits.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/safe_url_pattern.h"
#include "third_party/blink/public/mojom/safe_url_pattern.mojom.h"
#include "third_party/liburlpattern/pattern.h"

namespace mojo {
namespace internal {

inline std::string_view TruncateString(const std::string& string) {
  return std::string_view(string).substr(0, 4 * 1024);
}

}  // namespace internal

template <>
struct BLINK_COMMON_EXPORT
    EnumTraits<blink::mojom::Modifier, ::liburlpattern::Modifier> {
  static blink::mojom::Modifier ToMojom(
      const ::liburlpattern::Modifier& modifier) {
    switch (modifier) {
      case liburlpattern::Modifier::kZeroOrMore:
        return blink::mojom::Modifier::kZeroOrMore;
      case liburlpattern::Modifier::kOptional:
        return blink::mojom::Modifier::kOptional;
      case liburlpattern::Modifier::kOneOrMore:
        return blink::mojom::Modifier::kOneOrMore;
      case liburlpattern::Modifier::kNone:
        return blink::mojom::Modifier::kNone;
    }
  }

  static bool FromMojom(blink::mojom::Modifier data,
                        ::liburlpattern::Modifier* out);
};

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::FixedPatternDataView, ::liburlpattern::Part> {
  static std::string_view value(const ::liburlpattern::Part& part) {
    return internal::TruncateString(part.value);
  }

  static bool Read(blink::mojom::FixedPatternDataView data,
                   ::liburlpattern::Part* out);
};

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::WildcardPatternDataView, ::liburlpattern::Part> {
  static std::string_view name(const ::liburlpattern::Part& part) {
    return internal::TruncateString(part.name);
  }
  static std::string_view prefix(const ::liburlpattern::Part& part) {
    return internal::TruncateString(part.prefix);
  }
  static std::string_view value(const ::liburlpattern::Part& part) {
    return internal::TruncateString(part.value);
  }
  static std::string_view suffix(const ::liburlpattern::Part& part) {
    return internal::TruncateString(part.suffix);
  }

  static bool Read(blink::mojom::WildcardPatternDataView data,
                   ::liburlpattern::Part* out);
};

template <>
struct BLINK_COMMON_EXPORT
    UnionTraits<blink::mojom::PatternTemplateDataView, ::liburlpattern::Part> {
  static blink::mojom::PatternTemplateDataView::Tag GetTag(
      const ::liburlpattern::Part& value);

  static const ::liburlpattern::Part& fixed(
      const ::liburlpattern::Part& value) {
    return value;
  }

  static const ::liburlpattern::Part& full_wildcard(
      const ::liburlpattern::Part& value) {
    return value;
  }

  static const ::liburlpattern::Part& segment_wildcard(
      const ::liburlpattern::Part& value) {
    return value;
  }

  static bool Read(blink::mojom::PatternTemplateDataView data,
                   ::liburlpattern::Part* out);
};

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::SafeUrlPatternPartDataView,
                 ::liburlpattern::Part> {
  static const liburlpattern::Part& pattern(
      const ::liburlpattern::Part& pattern) {
    return pattern;
  }

  static liburlpattern::Modifier modifier(
      const ::liburlpattern::Part& pattern) {
    return pattern.modifier;
  }

  static bool Read(blink::mojom::SafeUrlPatternPartDataView data,
                   ::liburlpattern::Part* out);
};

template <>
struct BLINK_COMMON_EXPORT StructTraits<blink::mojom::SafeUrlPatternDataView,
                                        ::blink::SafeUrlPattern> {
  static const std::vector<liburlpattern::Part>& protocol(
      const ::blink::SafeUrlPattern& pattern) {
    return pattern.protocol;
  }
  static const std::vector<liburlpattern::Part>& username(
      const ::blink::SafeUrlPattern& pattern) {
    return pattern.username;
  }
  static const std::vector<liburlpattern::Part>& password(
      const ::blink::SafeUrlPattern& pattern) {
    return pattern.password;
  }
  static const std::vector<liburlpattern::Part>& hostname(
      const ::blink::SafeUrlPattern& pattern) {
    return pattern.hostname;
  }
  static const std::vector<liburlpattern::Part>& port(
      const ::blink::SafeUrlPattern& pattern) {
    return pattern.port;
  }
  static const std::vector<liburlpattern::Part>& pathname(
      const ::blink::SafeUrlPattern& pattern) {
    return pattern.pathname;
  }
  static const std::vector<liburlpattern::Part>& search(
      const ::blink::SafeUrlPattern& pattern) {
    return pattern.search;
  }
  static const std::vector<liburlpattern::Part>& hash(
      const ::blink::SafeUrlPattern& pattern) {
    return pattern.hash;
  }
  static const blink::SafeUrlPatternOptions& options(
      const ::blink::SafeUrlPattern& pattern) {
    return pattern.options;
  }

  static bool Read(blink::mojom::SafeUrlPatternDataView data,
                   ::blink::SafeUrlPattern* out);
};

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::SafeUrlPatternOptionsDataView,
                 blink::SafeUrlPatternOptions> {
  static bool ignore_case(const ::blink::SafeUrlPatternOptions& data) {
    return data.ignore_case;
  }

  static bool Read(blink::mojom::SafeUrlPatternOptionsDataView data,
                   ::blink::SafeUrlPatternOptions* out);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_SAFE_URL_PATTERN_MOJOM_TRAITS_H_
