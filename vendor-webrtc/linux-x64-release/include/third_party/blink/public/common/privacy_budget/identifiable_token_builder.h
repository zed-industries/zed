// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_PRIVACY_BUDGET_IDENTIFIABLE_TOKEN_BUILDER_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_PRIVACY_BUDGET_IDENTIFIABLE_TOKEN_BUILDER_H_

#include <array>
#include <concepts>
#include <string_view>
#include <type_traits>

#include "base/containers/span.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/privacy_budget/identifiability_internal_templates.h"
#include "third_party/blink/public/common/privacy_budget/identifiable_token.h"

namespace blink {

// Builds an IdentifiableToken incrementally.
//
// Use this when the input to a sample is a bunch of disjoint objects, or the
// sample needs to include objects that are incrementally encountered.
//
// Notes:
//   * The digest returned by this class is *NOT* the same as the one
//     IdentifiabilityDigestOfBytes for the same set of bytes. This is due to
//     block based chaining of digests used by this class.
//     IdentifiabilityDigestOfBytes and this class are *NOT* interchangeable.
//
//     TODO(asanka): IdentifiabilityDigestOfBytes() and this class should
//     interop better. Perhaps by making the latter use the former.
//
//   * The digest returned by this class is *NOT* the same as what you would
//     acquire by invoking IdentifiableToken() over the same object.
//     IdentifiableToken() and this class are *NOT* interchangeable.
//
//   * The digest returned by this class only depends on the cumulative sequence
//     of bytes that are fed to it. The partitioning thereof is irrelevant.
//
//   * This object never finalizes. Partial digests can be extracted at any
//     point.
class BLINK_COMMON_EXPORT IdentifiableTokenBuilder {
 public:
  // Convenient alias for a span of const uint8_t.
  using ByteSpan = IdentifiableToken::ByteSpan;

  // Initializes an "empty" incremental digest for the purpose of constructing
  // an identifiability sample.
  IdentifiableTokenBuilder();

  // Initializes an incremental digest and populates it with the data contained
  // in |message|.
  explicit IdentifiableTokenBuilder(ByteSpan message);

  // Copies the intermediate state.
  IdentifiableTokenBuilder(const IdentifiableTokenBuilder&);

  // Feeds data contained in |buffer| to the digest.
  IdentifiableTokenBuilder& AddBytes(ByteSpan buffer);

  // Feeds data contained in |buffer| to the digest, but precedes the buffer
  // contents with an integer indicating the length. Use this when:
  //
  //   * |buffer| is atomic. I.e. it will always be added as a single buffer.
  //
  //   * The boundary between |buffer| and adjacent objects cannot be uniquely
  //     established based on content.
  //
  // E.g.: Ignoring NUL terminators, the pair of strings "abcd", "efgh" will be
  //       assigned token as the strings "abcdefg", "h" if both are added
  //       individually via AddBytes(). But they will have distinct digests if
  //       added via AddAtomic().
  //
  // If the contents of the object cannot be specified in a contiguous span of
  // memory, then consider adding a length directly via AddValue() prior to
  // adding the contents of the buffer. Doing so will achieve the same ends as
  // AddAtomic().
  IdentifiableTokenBuilder& AddAtomic(ByteSpan buffer);
  IdentifiableTokenBuilder& AddAtomic(std::string_view string) {
    return AddAtomic(base::as_byte_span(string));
  }

  // Feeds the underlying value of the |token| itself to the digest. Use this
  // when |token| is computed in parallel in order to preserve the ordering of
  // values that were seen in a concurrent sequence that cannot be
  // deterministically interleaved into the primary stream.
  IdentifiableTokenBuilder& AddToken(IdentifiableToken token) {
    return AddValue(token.value_);
  }

  // Helper for feeding primitive types by value efficiently. Anything more
  // complicated than that should be passed in as a base::span<const uint8_t>.
  //
  // Adds eight bytes to the digest. If the type of the value doesn't consume
  // all of the bytes, pads the remainder with NUL bytes.
  template <typename T>
    requires(std::same_as<T, std::remove_cvref_t<T>> &&
             std::has_unique_object_representations_v<T> &&
             sizeof(T) <= sizeof(uint64_t))
  IdentifiableTokenBuilder& AddValue(T in) {
    AlignPartialBuffer();
    int64_t clean_buffer = internal::DigestOfObjectRepresentation(in);
    return AddBytes(base::byte_span_from_ref(clean_buffer));
  }

  // Conversion operator captures an intermediate digest.
  //
  // The sample captures all the data that's been fed into the digest so far,
  // but doesn't finalize the digest. It is valid to continue adding data after
  // constructing an intermediate sample.
  //
  // (google-explicit-constructor also flags user-defined conversion operators.)
  // NOLINTNEXTLINE(google-explicit-constructor)
  operator IdentifiableToken() const;

  // Captures an intermediate digest.
  //
  // The sample captures all the data that's been fed into the digest so far,
  // but doesn't finalize the digest. It is valid to continue adding data after
  // constructing an intermediate sample.
  IdentifiableToken GetToken() const;

  // No comparisons.
  bool operator==(const IdentifiableTokenBuilder&) const = delete;
  bool operator<(const IdentifiableTokenBuilder&) const = delete;

  // A big random prime. It's also the digest returned for an empty block.
  static constexpr uint64_t kChainingValueSeed = UINT64_C(6544625333304541877);

 private:
  // Block size. Must be a multiple of 64. Higher block sizes consume more
  // memory. The extra cost is unlikely to be worth it.
  //
  // Under the covers we use CityHash64. It can pretty efficiently digest
  // 64-byte blocks.
  static constexpr size_t kBlockSizeInBytes = 64;

  // Target alignment for new buffers. This is set to 8 for all platforms and
  // must always stay constant across platforms.
  static constexpr size_t kBlockAlignment = 8;

  // An array of exactly |kBlockSizeInBytes| bytes.
  using BlockBuffer = std::array<uint8_t, kBlockSizeInBytes>;

  // A view of a full block.
  using ConstFullBlockSpan = base::span<const uint8_t, kBlockSizeInBytes>;

  // Returns true if the partial buffer is aligned on |kBlockAlignment|
  // boundary.
  bool IsAligned() const;

  // Appends enough NUL bytes to |partial_| until the next insertion point is
  // aligned on a |kBlockAlignment| boundary.
  //
  // If the partial buffer is non-empty, its size is unlikely to be aligned at
  // machine word boundary. This makes subsequent append operations slow for
  // data types that are already aligned.
  //
  // This should only be called prior to adding an atomic buffer.
  void AlignPartialBuffer();

  // Captures the |kBlockSizeInBytes| bytes of data in |block| into the digest.
  // |block| must be exactly this many bytes.
  void DigestBlock(ConstFullBlockSpan block);

  // Captures as many bytes as possible from |message| into the partial block in
  // |partial_|. It captures a maximum of |kBlockSizeInBytes - 1| bytes.
  //
  // Returns a span covering the remainder of |message| that was not consumed.
  ByteSpan SkimIntoPartial(ByteSpan message);

  // Returns a span for the contents of the partial block.
  //
  // Can be called at any point. Does not change the state of the partial
  // buffer.
  ByteSpan GetPartialBlock() const;

  // Returns a span that includes the contents of the partial block and backed
  // by |partial_|.
  //
  // NOTE: Should only be called once |kBlockSizeInBytes| bytes have been
  // accumulated. Resets |partial_size_| upon completion.
  //
  // NOTE: Any subsequent AddBytes(), AddValue(), AddAtomic() calls will
  // invalidate the returned FullBlock.
  ConstFullBlockSpan TakeCompletedBlock();

  // Size of partially filled buffer.
  size_t PartialSize() const;

  // Accumulates smaller pieces of data until we have a full block.
  alignas(int64_t) BlockBuffer partial_;

  // Next available position in `partial_`. std::array iterators are never
  // invalidated.
  BlockBuffer::iterator position_ = partial_.begin();

  // Merkle-Damgård chaining.
  uint64_t chaining_value_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_PRIVACY_BUDGET_IDENTIFIABLE_TOKEN_BUILDER_H_
