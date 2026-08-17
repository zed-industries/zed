/*
 * Copyright (C) 2020 Apple Inc. All rights reserved.
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
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_ENCODING_TABLES_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_ENCODING_TABLES_H_

#include <stddef.h>

#include <algorithm>
#include <array>
#include <iterator>
#include <optional>
#include <type_traits>
#include <utility>

#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_uchar.h"
#include "third_party/blink/renderer/platform/wtf/wtf_export.h"

namespace WTF {

// Following functions are helpers for TextCodecCJK, and not intended to be used
// by others.
constexpr size_t kJis0208EncodeIndexSize = 7724;
using Jis0208EncodeIndex =
    std::array<std::pair<uint16_t, UChar>, kJis0208EncodeIndexSize>;
const WTF_EXPORT Jis0208EncodeIndex& EnsureJis0208EncodeIndexForEncode();
const WTF_EXPORT Jis0208EncodeIndex& EnsureJis0208EncodeIndexForDecode();

constexpr size_t kJis0212EncodeIndexSize = 6067;
using Jis0212EncodeIndex =
    std::array<std::pair<uint16_t, UChar>, kJis0212EncodeIndexSize>;
const WTF_EXPORT Jis0212EncodeIndex& EnsureJis0212EncodeIndexForEncode();
const WTF_EXPORT Jis0212EncodeIndex& EnsureJis0212EncodeIndexForDecode();

constexpr size_t kEucKrEncodeIndexSize = 17048;
using EucKrEncodeIndex =
    std::array<std::pair<uint16_t, UChar>, kEucKrEncodeIndexSize>;
const WTF_EXPORT EucKrEncodeIndex& EnsureEucKrEncodeIndexForEncode();
const WTF_EXPORT EucKrEncodeIndex& EnsureEucKrEncodeIndexForDecode();

constexpr size_t kGb18030EncodeIndexSize = 23940;
using Gb18030EncodeIndex =
    std::array<std::pair<uint16_t, UChar>, kGb18030EncodeIndexSize>;
using Gb18030EncodeTable = std::array<UChar, kGb18030EncodeIndexSize>;
const WTF_EXPORT Gb18030EncodeIndex& EnsureGb18030EncodeIndexForEncode();
const WTF_EXPORT Gb18030EncodeTable& EnsureGb18030EncodeTable();

// Functions for using sorted arrays of pairs as a map.
// FIXME: Consider moving these functions to std_lib_extras.h for uses other
// than encoding tables.

struct CompareFirst {
  template <typename TypeA, typename TypeB>
  bool operator()(const TypeA& a, const TypeB& b) {
    return a.first < b.first;
  }
};

struct EqualFirst {
  template <typename TypeA, typename TypeB>
  bool operator()(const TypeA& a, const TypeB& b) {
    return a.first == b.first;
  }
};

struct CompareSecond {
  template <typename TypeA, typename TypeB>
  bool operator()(const TypeA& a, const TypeB& b) {
    return a.second < b.second;
  }
};

template <typename T>
struct FirstAdapter {
  const T& first;
};
template <typename T>
FirstAdapter<T> MakeFirstAdapter(const T& value) {
  return {value};
}

template <typename T>
struct SecondAdapter {
  const T& second;
};
template <typename T>
SecondAdapter<T> MakeSecondAdapter(const T& value) {
  return {value};
}

template <typename CollectionType>
bool SortedFirstsAreUnique(const CollectionType& collection) {
  return std::adjacent_find(std::begin(collection), std::end(collection),
                            EqualFirst{}) == std::end(collection);
}

template <typename CollectionType, typename KeyType>
static auto FindFirstInSortedPairs(const CollectionType& collection,
                                   const KeyType& key)
    -> std::optional<decltype(std::begin(collection)->second)> {
  if constexpr (std::is_integral_v<KeyType>) {
    if (key != decltype(std::begin(collection)->first)(key))
      return std::nullopt;
  }
  auto iterator = std::lower_bound(std::begin(collection), std::end(collection),
                                   MakeFirstAdapter(key), CompareFirst{});
  if (iterator == std::end(collection) || key < iterator->first)
    return std::nullopt;
  return iterator->second;
}

template <typename CollectionType, typename KeyType>
static auto FindInSortedPairs(const CollectionType& collection,
                              const KeyType& key)
    -> std::pair<decltype(std::begin(collection)),
                 decltype(std::begin(collection))> {
  if constexpr (std::is_integral_v<KeyType>) {
    if (key != decltype(std::begin(collection)->first)(key))
      return {std::end(collection), std::end(collection)};
  }
  return std::equal_range(std::begin(collection), std::end(collection),
                          MakeFirstAdapter(key), CompareFirst{});
}

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_TEXT_ENCODING_TABLES_H_
