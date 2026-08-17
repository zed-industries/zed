//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef BENCHMARK_GENERATE_INPUT_H
#define BENCHMARK_GENERATE_INPUT_H

#include <algorithm>
#include <climits>
#include <concepts>
#include <cstddef>
#include <initializer_list>
#include <random>
#include <string>
#include <vector>

static const char Letters[] = {
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K',
    'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f',
    'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'};
static const std::size_t LettersSize = sizeof(Letters);

inline std::default_random_engine& getRandomEngine() {
  static std::default_random_engine RandEngine(std::random_device{}());
  return RandEngine;
}

inline char getRandomChar() {
  std::uniform_int_distribution<> LettersDist(0, LettersSize - 1);
  return Letters[LettersDist(getRandomEngine())];
}

template <class IntT>
inline IntT getRandomInteger(IntT Min, IntT Max) {
  std::uniform_int_distribution<unsigned long long> dist(Min, Max);
  return static_cast<IntT>(dist(getRandomEngine()));
}

inline std::string getRandomString(std::size_t Len) {
  std::string str(Len, 0);
  std::generate_n(str.begin(), Len, &getRandomChar);
  return str;
}

template <class IntT>
inline std::vector<IntT> getDuplicateIntegerInputs(std::size_t N) {
  std::vector<IntT> inputs(N, static_cast<IntT>(-1));
  return inputs;
}

template <class IntT>
inline std::vector<IntT> getSortedIntegerInputs(std::size_t N) {
  std::vector<IntT> inputs;
  inputs.reserve(N);
  for (std::size_t i = 0; i < N; i += 1)
    inputs.push_back(i);
  return inputs;
}

template <class IntT>
std::vector<IntT> getSortedLargeIntegerInputs(std::size_t N) {
  std::vector<IntT> inputs;
  inputs.reserve(N);
  for (std::size_t i = 0; i < N; ++i)
    inputs.push_back(i + N);
  return inputs;
}

template <class IntT>
std::vector<IntT> getSortedTopBitsIntegerInputs(std::size_t N) {
  std::vector<IntT> inputs = getSortedIntegerInputs<IntT>(N);
  for (auto& E : inputs)
    E <<= ((sizeof(IntT) / 2) * CHAR_BIT);
  return inputs;
}

template <class IntT>
inline std::vector<IntT> getReverseSortedIntegerInputs(std::size_t N) {
  std::vector<IntT> inputs;
  inputs.reserve(N);
  std::size_t i = N;
  while (i > 0) {
    --i;
    inputs.push_back(i);
  }
  return inputs;
}

template <class IntT>
std::vector<IntT> getPipeOrganIntegerInputs(std::size_t N) {
  std::vector<IntT> v;
  v.reserve(N);
  for (std::size_t i = 0; i < N / 2; ++i)
    v.push_back(i);
  for (std::size_t i = N / 2; i < N; ++i)
    v.push_back(N - i);
  return v;
}

template <class IntT>
std::vector<IntT> getRandomIntegerInputs(std::size_t N) {
  std::vector<IntT> inputs;
  inputs.reserve(N);
  for (std::size_t i = 0; i < N; ++i)
    inputs.push_back(getRandomInteger<IntT>(0, std::numeric_limits<IntT>::max()));
  return inputs;
}

inline std::vector<std::string> getRandomStringInputsWithLength(std::size_t N, std::size_t len) { // N-by-len
  std::vector<std::string> inputs;
  inputs.reserve(N);
  for (std::size_t i = 0; i < N; ++i)
    inputs.push_back(getRandomString(len));
  return inputs;
}

inline std::vector<std::string> getDuplicateStringInputs(std::size_t N) {
  std::vector<std::string> inputs(N, getRandomString(1024));
  return inputs;
}

inline std::vector<std::string> getRandomStringInputs(std::size_t N) {
  return getRandomStringInputsWithLength(N, 1024);
}

template <class IntT>
std::vector<std::vector<IntT>> getRandomIntegerInputsWithLength(std::size_t N, std::size_t len) { // N-by-len
  std::vector<std::vector<IntT>> inputs;
  inputs.reserve(N);
  for (std::size_t i = 0; i < N; ++i)
    inputs.push_back(getRandomIntegerInputs<IntT>(len));
  return inputs;
}

inline std::vector<std::string> getSSORandomStringInputs(size_t N) {
  std::vector<std::string> inputs;
  for (size_t i = 0; i < N; ++i)
    inputs.push_back(getRandomString(10)); // SSO
  return inputs;
}

inline std::vector<std::string> getPrefixedRandomStringInputs(size_t N) {
  std::vector<std::string> inputs;
  inputs.reserve(N);
  constexpr int kSuffixLength = 32;
  const std::string prefix    = getRandomString(1024 - kSuffixLength);
  for (std::size_t i = 0; i < N; ++i)
    inputs.push_back(prefix + getRandomString(kSuffixLength));
  return inputs;
}

inline std::vector<std::string> getSortedStringInputs(std::size_t N) {
  std::vector<std::string> inputs = getRandomStringInputs(N);
  std::sort(inputs.begin(), inputs.end());
  return inputs;
}

inline std::vector<std::string> getReverseSortedStringInputs(std::size_t N) {
  std::vector<std::string> inputs = getSortedStringInputs(N);
  std::reverse(inputs.begin(), inputs.end());
  return inputs;
}

inline std::vector<const char*> getRandomCStringInputs(std::size_t N) {
  static std::vector<std::string> inputs = getRandomStringInputs(N);
  std::vector<const char*> cinputs;
  for (auto const& str : inputs)
    cinputs.push_back(str.c_str());
  return cinputs;
}

template <class T>
struct Generate {
  // When the contents don't matter
  static T arbitrary();

  // Prefer a cheap-to-construct element if possible
  static T cheap();

  // Prefer an expensive-to-construct element if possible
  static T expensive();
};

template <class T>
  requires std::integral<T>
struct Generate<T> {
  static T arbitrary() { return 42; }
  static T cheap() { return 42; }
  static T expensive() { return 42; }
  static T random() { return getRandomInteger<T>(std::numeric_limits<T>::min(), std::numeric_limits<T>::max()); }
};

template <>
struct Generate<std::string> {
  static std::string arbitrary() { return "hello world"; }
  static std::string cheap() { return "small"; }
  static std::string expensive() { return std::string(256, 'x'); }
  static std::string random() {
    auto length = getRandomInteger<std::size_t>(1, 1024);
    return getRandomString(length);
  }
};

template <class T>
T random_different_from(std::initializer_list<T> others) {
  T value;
  do {
    value = Generate<T>::random();
  } while (std::find(others.begin(), others.end(), value) != others.end());
  return value;
}

#endif // BENCHMARK_GENERATE_INPUT_H
