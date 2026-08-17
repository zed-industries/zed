// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_VARINT_SHUFFLE_H__
#define GOOGLE_PROTOBUF_VARINT_SHUFFLE_H__

#include <cassert>
#include <cstddef>
#include <cstdint>
#include <type_traits>
#include <utility>

// Must be included last.
#include "absl/base/optimization.h"
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {
namespace internal {

// Shifts "byte" left by n * 7 bits, filling vacated bits from `ones`.
template <int n>
PROTOBUF_ALWAYS_INLINE int64_t VarintShlByte(int8_t byte, int64_t ones) {
  return static_cast<int64_t>((static_cast<uint64_t>(byte) << n * 7) |
                              (static_cast<uint64_t>(ones) >> (64 - n * 7)));
}

// Shifts "byte" left by n * 7 bits, filling vacated bits from `ones` and
// bitwise ANDs the resulting value into the input/output `res` parameter.
// Returns true if the result was not negative.
template <int n>
PROTOBUF_ALWAYS_INLINE bool VarintShlAnd(int8_t byte, int64_t ones,
                                         int64_t& res) {
  res &= VarintShlByte<n>(byte, ones);
  return res >= 0;
}

// Shifts `byte` left by n * 7 bits, filling vacated bits with ones, and
// puts the new value in the output only parameter `res`.
// Returns true if the result was not negative.
template <int n>
PROTOBUF_ALWAYS_INLINE bool VarintShl(int8_t byte, int64_t ones, int64_t& res) {
  res = VarintShlByte<n>(byte, ones);
  return res >= 0;
}

template <typename VarintType, int limit = 10>
PROTOBUF_ALWAYS_INLINE const char* ShiftMixParseVarint(const char* p,
                                                       int64_t& res1) {
  using Signed = std::make_signed_t<VarintType>;
  constexpr bool kIs64BitVarint = std::is_same<Signed, int64_t>::value;
  constexpr bool kIs32BitVarint = std::is_same<Signed, int32_t>::value;
  static_assert(kIs64BitVarint || kIs32BitVarint, "");

  // The algorithm relies on sign extension for each byte to set all high bits
  // when the varint continues. It also relies on asserting all of the lower
  // bits for each successive byte read. This allows the result to be aggregated
  // using a bitwise AND. For example:
  //
  //          8       1          64     57 ... 24     17  16      9  8       1
  // ptr[0] = 1aaa aaaa ; res1 = 1111 1111 ... 1111 1111  1111 1111  1aaa aaaa
  // ptr[1] = 1bbb bbbb ; res2 = 1111 1111 ... 1111 1111  11bb bbbb  b111 1111
  // ptr[2] = 0ccc cccc ; res3 = 0000 0000 ... 000c cccc  cc11 1111  1111 1111
  //                             ---------------------------------------------
  //        res1 & res2 & res3 = 0000 0000 ... 000c cccc  ccbb bbbb  baaa aaaa
  //
  // On x86-64, a shld from a single register filled with enough 1s in the high
  // bits can accomplish all this in one instruction. It so happens that res1
  // has 57 high bits of ones, which is enough for the largest shift done.
  //
  // Just as importantly, by keeping results in res1, res2, and res3, we take
  // advantage of the superscalar abilities of the CPU.
  const auto next = [&p] { return static_cast<const int8_t>(*p++); };
  const auto last = [&p] { return static_cast<const int8_t>(p[-1]); };

  int64_t res2, res3;  // accumulated result chunks
  res1 = next();
  if (ABSL_PREDICT_TRUE(res1 >= 0)) return p;
  if (limit <= 1) goto limit0;

  // Densify all ops with explicit FALSE predictions from here on, except that
  // we predict length = 5 as a common length for fields like timestamp.
  if (ABSL_PREDICT_FALSE(VarintShl<1>(next(), res1, res2))) goto done1;
  if (limit <= 2) goto limit1;
  if (ABSL_PREDICT_FALSE(VarintShl<2>(next(), res1, res3))) goto done2;
  if (limit <= 3) goto limit2;
  if (ABSL_PREDICT_FALSE(VarintShlAnd<3>(next(), res1, res2))) goto done2;
  if (limit <= 4) goto limit2;
  if (ABSL_PREDICT_TRUE(VarintShlAnd<4>(next(), res1, res3))) goto done2;
  if (limit <= 5) goto limit2;

  if (kIs64BitVarint) {
    if (ABSL_PREDICT_FALSE(VarintShlAnd<5>(next(), res1, res2))) goto done2;
    if (limit <= 6) goto limit2;
    if (ABSL_PREDICT_FALSE(VarintShlAnd<6>(next(), res1, res3))) goto done2;
    if (limit <= 7) goto limit2;
    if (ABSL_PREDICT_FALSE(VarintShlAnd<7>(next(), res1, res2))) goto done2;
    if (limit <= 8) goto limit2;
    if (ABSL_PREDICT_FALSE(VarintShlAnd<8>(next(), res1, res3))) goto done2;
    if (limit <= 9) goto limit2;
  } else {
    // An overlong int32 is expected to span the full 10 bytes
    if (ABSL_PREDICT_FALSE(!(next() & 0x80))) goto done2;
    if (limit <= 6) goto limit2;
    if (ABSL_PREDICT_FALSE(!(next() & 0x80))) goto done2;
    if (limit <= 7) goto limit2;
    if (ABSL_PREDICT_FALSE(!(next() & 0x80))) goto done2;
    if (limit <= 8) goto limit2;
    if (ABSL_PREDICT_FALSE(!(next() & 0x80))) goto done2;
    if (limit <= 9) goto limit2;
  }

  // For valid 64bit varints, the 10th byte/ptr[9] should be exactly 1. In this
  // case, the continuation bit of ptr[8] already set the top bit of res3
  // correctly, so all we have to do is check that the expected case is true.
  if (ABSL_PREDICT_TRUE(next() == 1)) goto done2;

  if (ABSL_PREDICT_FALSE(last() & 0x80)) {
    // If the continue bit is set, it is an unterminated varint.
    return nullptr;
  }

  // A zero value of the first bit of the 10th byte represents an
  // over-serialized varint. This case should not happen, but if does (say, due
  // to a nonconforming serializer), deassert the continuation bit that came
  // from ptr[8].
  if (kIs64BitVarint && (last() & 1) == 0) {
    static constexpr int bits = 64 - 1;
#if defined(__GCC_ASM_FLAG_OUTPUTS__) && defined(__x86_64__)
    // Use a small instruction since this is an uncommon code path.
    asm("btc %[bits], %[res3]" : [res3] "+r"(res3) : [bits] "i"(bits));
#else
    res3 ^= int64_t{1} << bits;
#endif
  }

done2:
  res2 &= res3;
done1:
  res1 &= res2;
  PROTOBUF_ASSUME(p != nullptr);
  return p;
limit2:
  res2 &= res3;
limit1:
  res1 &= res2;
limit0:
  PROTOBUF_ASSUME(p != nullptr);
  PROTOBUF_ASSUME(res1 < 0);
  return p;
}

}  // namespace internal
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_VARINT_SHUFFLE_H__
