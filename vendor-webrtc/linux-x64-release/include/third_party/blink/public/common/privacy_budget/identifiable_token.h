// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_PRIVACY_BUDGET_IDENTIFIABLE_TOKEN_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_PRIVACY_BUDGET_IDENTIFIABLE_TOKEN_H_

#include <stdint.h>

#include <concepts>
#include <string_view>
#include <type_traits>

#include "base/bit_cast.h"
#include "base/containers/span.h"
#include "base/numerics/safe_conversions.h"
#include "base/types/cxx23_to_underlying.h"
#include "third_party/blink/public/common/privacy_budget/identifiability_internal_templates.h"
#include "third_party/blink/public/common/privacy_budget/identifiability_metrics.h"

namespace blink {

// Constructs a token that can be used for reporting a metric or constructing an
// identifiable surface.
//
// The token construction is a single step conversion that takes one of several
// constrained inputs and emits a value. The method by which the value is
// constructed intentionally cannot be chained. If such behavior is required,
// then this class should be modified to accommodate the new use case rather
// than implementing custom chaining schemes at call sites.
//
// Once constructed, a token can only be consumed by
// IdentifiabiltyMetricsBuilder and IdentifiableSurface. For all others, it is a
// copyable, opaque token.
//
// Reliance on implicit conversion imposes limitations on how
// IdentifiableToken class is to be used. For example the following works:
//
//     std::string foo = ....;
//     IdentifiableToken sample(foo);
//
// .. due to the following implicit conversion:
//
//    1. std::string -> const std::string&
//             : lvalue -> lvalue reference + cv-qualification
//    2. const std::string& -> std::string_view
//             : user-defined conversion via constructor
//               std::string_view(const std::string&)
//
// However, when used within a builder expression, the user-defined conversion
// doesn't occur due to there not being a single user defined conversion from
// std::string -> IdentifiableToken. I.e. the following does not work:
//
//     std::string foo = ....;
//     IdentifiabilityMetricBuilder(...).Set(surface, foo);
//                                                    ^^^
//      The compiler can't deduce a two step user-defined conversion for |foo|.
//
// All overrides of the constructor should ensure that there exists a unique
// representation of the data type being sampled, and that the sample value is
// constructed based on this unique representation.
//
// TODO(asanka): Also require that the representation be portable.
//
// Extending IdentifiableToken to support more data types:
// -----------------------------------------------------------
//
// This class is intentionally placed in blink/public/common due to the
// requirement that these primitives be made available to both the renderer and
// the browser. However, it would be desirable to have renderer or browser
// specific functions for mapping common types in either domain into a sample.
//
// The recommended methods to do so are (one-of):
//
//   1. Use an existing byte span representation.
//
//      E.g.: Assuming |v| is a WTF::Vector
//          IdentifiabilityMetricBuilder(...).Set(
//              ..., base::as_byte_span(v.Data(), v.Size()));
//
//      Note again that serializing to a stream of bytes may not be sufficient
//      if the underlying types don't have a unique representation.
//
//   2. Construct a byte-wise unique representation and invoke
//      IdentifiableToken(ByteSpan) either explicitly or implicitly via
//      user-defined conversions.
//
// Note: Avoid doing template magic. There's already too much here. Templates
//       make it difficult to verify that the correct stable representation is
//       the one getting ingested into the reporting workflow.
//
//       Instead, explicitly invoke some wrapper that emits a ByteSpan (a.k.a.
//       base::span<const uint8_t>.
class IdentifiableToken {
 public:
  // Generic buffer of bytes.
  using ByteSpan = base::span<const uint8_t>;

  // Representation type of the sample.
  using TokenType = int64_t;

  // Required for use in certain data structures. Represents no bytes.
  constexpr IdentifiableToken() : value_(kIdentifiabilityDigestOfNoBytes) {}

  // A byte buffer specified as a span.
  //
  // This is essentially the base case. If it were the base case, then
  // IdentifiableToken would be closer to a proper digest.
  //
  // NOLINTNEXTLINE(google-explicit-constructor)
  IdentifiableToken(ByteSpan span)
      : value_(IdentifiabilityDigestOfBytes(span)) {}

  // Integers, big and small. Includes char.
  template <typename T,
            typename U = std::remove_cvref_t<T>>
    requires std::integral<U>
  constexpr IdentifiableToken(T in)  // NOLINT(google-explicit-constructor)
      : value_(base::IsValueInRangeForNumericType<TokenType, U>(in)
                   ? in
                   : internal::DigestOfObjectRepresentation<U>(in)) {}

  // Enums. Punt to the underlying type.
  template <typename T>
    requires std::is_enum_v<std::remove_cvref_t<T>>
  constexpr IdentifiableToken(T in)  // NOLINT(google-explicit-constructor)
      : IdentifiableToken(base::to_underlying(in)) {}

  // All floating point values get converted to double before encoding.
  //
  // Why? We'd like to minimize accidental divergence of values due to the data
  // type that the callsite happened to be using at the time.
  //
  // On some platforms sizeof(long double) gives us 16 (i.e. 128 bits), while
  // only 10 of those bytes are initialized. If the whole sizeof(long double)
  // buffer were to be ingested, then the uninitialized memory will cause the
  // resulting digest to be useless.
  //
  // Furthermore, `DigestOfObjectRepresentation()` requires
  // `std::has_unique_object_representations_v<>`, which doesn't hold for
  // floating-point values. Work around by reinterpreting as an integral type of
  // the same size, without changing the underlying bit pattern.
  static_assert(sizeof(double) == sizeof(int64_t));
  template <typename T>
    requires std::floating_point<std::remove_cvref_t<T>>
  constexpr IdentifiableToken(T in)  // NOLINT(google-explicit-constructor)
      : value_(internal::DigestOfObjectRepresentation(
            base::bit_cast<int64_t>(static_cast<double>(in)))) {}

  // StringPiece. Decays to base::span<> but requires an explicit constructor
  // invocation.
  //
  // Care must be taken when using string types with IdentifiableToken() since
  // there's not privacy expectation in the resulting token value. If the string
  // used as an input is privacy sensitive, it should not be passed in as-is.
  explicit IdentifiableToken(std::string_view s)
      : IdentifiableToken(base::as_byte_span(s)) {
    // The cart is before the horse, but it's a static_assert<>.
    static_assert(
        std::is_same<ByteSpan, decltype(base::as_byte_span(s))>::value,
        "base::as_bytes() doesn't return ByteSpan");
  }

  // Span of known trivial types except for BytesSpan, which is the base case.
  template <typename T, size_t Extent>
    requires(!std::same_as<ByteSpan::element_type, T> &&
             std::has_unique_object_representations_v<std::remove_cvref_t<T>>)
  // NOLINTNEXTLINE(google-explicit-constructor)
  IdentifiableToken(base::span<T, Extent> span)
      : IdentifiableToken(base::as_bytes(span)) {}

  // A span of non-trivial things where each thing can be digested individually.
  template <typename T, size_t Extent>
    requires(!std::same_as<ByteSpan::element_type, T> &&
             !std::has_unique_object_representations_v<std::remove_cvref_t<T>>)
  // NOLINTNEXTLINE(google-explicit-constructor)
  IdentifiableToken(base::span<T, Extent> span) {
    TokenType cur_digest = 0;
    for (const auto& element : span) {
      TokenType digests[2];
      digests[0] = cur_digest;
      digests[1] = IdentifiableToken(element).value_;
      cur_digest = IdentifiabilityDigestOfBytes(base::as_byte_span(digests));
    }
    value_ = cur_digest;
  }

  // Parameter pack where each parameter can be digested individually. Requires
  // at least two parameters.
  template <typename T1, typename T2, typename... Trest>
  constexpr IdentifiableToken(T1 first, T2 second, Trest... rest) {
    TokenType samples[] = {IdentifiableToken(first).value_,
                           IdentifiableToken(second).value_,
                           (IdentifiableToken(rest).value_)...};
    value_ = IdentifiableToken(base::span(samples)).value_;
  }

  constexpr bool operator<(const IdentifiableToken& that) const {
    return value_ < that.value_;
  }

  constexpr bool operator<=(const IdentifiableToken& that) const {
    return value_ <= that.value_;
  }

  constexpr bool operator>(const IdentifiableToken& that) const {
    return value_ > that.value_;
  }

  constexpr bool operator>=(const IdentifiableToken& that) const {
    return value_ >= that.value_;
  }

  constexpr bool operator==(const IdentifiableToken& that) const {
    return value_ == that.value_;
  }

  constexpr bool operator!=(const IdentifiableToken& that) const {
    return value_ != that.value_;
  }

  // Returns a value that can be passed into the UKM metrics recording
  // interfaces.
  int64_t ToUkmMetricValue() const { return value_; }

 private:
  friend class IdentifiabilityMetricBuilder;
  friend class IdentifiableSurface;
  friend class IdentifiableTokenBuilder;

  // TODO(asanka): This should be const. Switch over once the incremental digest
  // functions land.
  TokenType value_ = 0;
};

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_PRIVACY_BUDGET_IDENTIFIABLE_TOKEN_H_
