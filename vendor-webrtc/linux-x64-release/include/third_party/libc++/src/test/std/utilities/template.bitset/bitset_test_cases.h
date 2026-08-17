//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LIBCPP_TEST_BITSET_TEST_CASES_H
#define LIBCPP_TEST_BITSET_TEST_CASES_H

#include <bitset>
#include <string>
#include <vector>

#include "test_macros.h"

template <int N>
TEST_CONSTEXPR_CXX23 std::vector<std::bitset<N> > get_test_cases();

template <>
TEST_CONSTEXPR_CXX23 inline std::vector<std::bitset<0> > get_test_cases<0>() {
  std::vector<std::bitset<0> > cases;
  cases.push_back(std::bitset<0>());
  return cases;
}

template <>
TEST_CONSTEXPR_CXX23 inline std::vector<std::bitset<1> > get_test_cases<1>() {
  std::vector<std::bitset<1> > cases;
  cases.push_back(std::bitset<1>("0"));
  cases.push_back(std::bitset<1>("1"));
  return cases;
}

template <>
TEST_CONSTEXPR_CXX23 inline std::vector<std::bitset<2> > get_test_cases<2>() {
  std::vector<std::bitset<2> > cases;
  cases.push_back(std::bitset<2>("00"));
  cases.push_back(std::bitset<2>("01"));
  cases.push_back(std::bitset<2>("10"));
  cases.push_back(std::bitset<2>("11"));
  return cases;
}

template <>
TEST_CONSTEXPR_CXX23 inline std::vector<std::bitset<5> > get_test_cases<5>() {
  std::vector<std::bitset<5> > cases;
  cases.push_back(std::bitset<5>("00000"));
  cases.push_back(std::bitset<5>("00001"));
  cases.push_back(std::bitset<5>("10000"));
  cases.push_back(std::bitset<5>("00010"));
  cases.push_back(std::bitset<5>("01000"));
  cases.push_back(std::bitset<5>("00011"));
  cases.push_back(std::bitset<5>("11000"));
  cases.push_back(std::bitset<5>("00100"));
  cases.push_back(std::bitset<5>("11011"));
  cases.push_back(std::bitset<5>("00101"));
  cases.push_back(std::bitset<5>("10100"));
  cases.push_back(std::bitset<5>("00110"));
  cases.push_back(std::bitset<5>("01100"));
  cases.push_back(std::bitset<5>("00111"));
  cases.push_back(std::bitset<5>("11100"));
  cases.push_back(std::bitset<5>("11111"));
  return cases;
}

template <>
TEST_CONSTEXPR_CXX23 inline std::vector<std::bitset<31> > get_test_cases<31>() {
  std::vector<std::bitset<31> > cases;
  cases.push_back(std::bitset<31>("0000000000000000000000000000000"));
  cases.push_back(std::bitset<31>("0000000000000000000000000000001"));
  cases.push_back(std::bitset<31>("1000000000000000000000000000000"));
  cases.push_back(std::bitset<31>("1000000000000000000000000000001"));
  cases.push_back(std::bitset<31>("1000000000000000000001000000001"));
  cases.push_back(std::bitset<31>("0000000000000000111111111111111"));
  cases.push_back(std::bitset<31>("1000000000000000111111111111111"));
  cases.push_back(std::bitset<31>("1111111111111111000000000000000"));
  cases.push_back(std::bitset<31>("1111111111111111000000000000001"));
  cases.push_back(std::bitset<31>("1010101010101010101010101010101"));
  cases.push_back(std::bitset<31>("0101010101010101010101010101010"));
  cases.push_back(std::bitset<31>("1111111111111111111111111111111"));
  return cases;
}

template <>
TEST_CONSTEXPR_CXX23 inline std::vector<std::bitset<32> > get_test_cases<32>() {
  std::vector<std::bitset<32> > cases;
  cases.push_back(std::bitset<32>("00000000000000000000000000000000"));
  cases.push_back(std::bitset<32>("00000000000000000000000000000001"));
  cases.push_back(std::bitset<32>("10000000000000000000000000000000"));
  cases.push_back(std::bitset<32>("10000000000000000000000000000001"));
  cases.push_back(std::bitset<32>("10000000000000000000111000000001"));
  cases.push_back(std::bitset<32>("00000000000000001111111111111111"));
  cases.push_back(std::bitset<32>("10000000000000001111111111111111"));
  cases.push_back(std::bitset<32>("11111111111111110000000000000000"));
  cases.push_back(std::bitset<32>("11111111111111110000000000000001"));
  cases.push_back(std::bitset<32>("10101010101010101010101010101010"));
  cases.push_back(std::bitset<32>("01010101010101010101010101010101"));
  cases.push_back(std::bitset<32>("11111111111111111111111111111111"));
  return cases;
}

template <>
TEST_CONSTEXPR_CXX23 inline std::vector<std::bitset<33> > get_test_cases<33>() {
  std::vector<std::bitset<33> > cases;
  cases.push_back(std::bitset<33>("000000000000000000000000000000000"));
  cases.push_back(std::bitset<33>("000000000000000000000000000000001"));
  cases.push_back(std::bitset<33>("100000000000000000000000000000000"));
  cases.push_back(std::bitset<33>("100000000000000000000000000000001"));
  cases.push_back(std::bitset<33>("100000000000000000001110000000001"));
  cases.push_back(std::bitset<33>("000000000000000011111111111111111"));
  cases.push_back(std::bitset<33>("100000000000000011111111111111111"));
  cases.push_back(std::bitset<33>("111111111111111100000000000000000"));
  cases.push_back(std::bitset<33>("111111111111111100000000000000001"));
  cases.push_back(std::bitset<33>("101010101010101010101010101010101"));
  cases.push_back(std::bitset<33>("010101010101010101010101010101010"));
  cases.push_back(std::bitset<33>("111111111111111111111111111111111"));
  return cases;
}

template <>
TEST_CONSTEXPR_CXX23 inline std::vector<std::bitset<63> > get_test_cases<63>() {
  std::vector<std::bitset<63> > cases;
  cases.push_back(std::bitset<63>("000000000000000000000000000000000000000000000000000000000000000"));
  cases.push_back(std::bitset<63>("000000000000000000000000000000000000000000000000000000000000001"));
  cases.push_back(std::bitset<63>("100000000000000000000000000000000000000000000000000000000000000"));
  cases.push_back(std::bitset<63>("100000000000000000000000000000000000000000000000000000000000001"));
  cases.push_back(std::bitset<63>("100000000000000000000000001111100000000000000000000000000000001"));
  cases.push_back(std::bitset<63>("000000000000000000000000000000001111111111111111111111111111111"));
  cases.push_back(std::bitset<63>("100000000000000000000000000000001111111111111111111111111111111"));
  cases.push_back(std::bitset<63>("111111111111111111111111111111110000000000000000000000000000000"));
  cases.push_back(std::bitset<63>("111111111111111111111111111111110000000000000000000000000000001"));
  cases.push_back(std::bitset<63>("101010101010101010101010101010101010101010101010101010101010101"));
  cases.push_back(std::bitset<63>("010101010101010101010101010101010101010101010101010101010101010"));
  cases.push_back(std::bitset<63>("111111111111111111111111111111111111111111111111111111111111111"));
  return cases;
}

template <>
TEST_CONSTEXPR_CXX23 inline std::vector<std::bitset<64> > get_test_cases<64>() {
  std::vector<std::bitset<64> > cases;
  cases.push_back(std::bitset<64>("0000000000000000000000000000000000000000000000000000000000000000"));
  cases.push_back(std::bitset<64>("0000000000000000000000000000000000000000000000000000000000000001"));
  cases.push_back(std::bitset<64>("1000000000000000000000000000000000000000000000000000000000000000"));
  cases.push_back(std::bitset<64>("1000000000000000000000000000000000000000000000000000000000000001"));
  cases.push_back(std::bitset<64>("1000000000000000000000000011111000000000000000000000000000000001"));
  cases.push_back(std::bitset<64>("0000000000000000000000000000000011111111111111111111111111111111"));
  cases.push_back(std::bitset<64>("1000000000000000000000000000000011111111111111111111111111111111"));
  cases.push_back(std::bitset<64>("1111111111111111111111111111111100000000000000000000000000000000"));
  cases.push_back(std::bitset<64>("1111111111111111111111111111111100000000000000000000000000000001"));
  cases.push_back(std::bitset<64>("1010101010101010101010101010101010101010101010101010101010101010"));
  cases.push_back(std::bitset<64>("0101010101010101010101010101010101010101010101010101010101010101"));
  cases.push_back(std::bitset<64>("1111111111111111111111111111111111111111111111111111111111111111"));
  return cases;
}

template <>
TEST_CONSTEXPR_CXX23 inline std::vector<std::bitset<65> > get_test_cases<65>() {
  std::vector<std::bitset<65> > cases;
  cases.push_back(std::bitset<65>("00000000000000000000000000000000000000000000000000000000000000000"));
  cases.push_back(std::bitset<65>("00000000000000000000000000000000000000000000000000000000000000001"));
  cases.push_back(std::bitset<65>("10000000000000000000000000000000000000000000000000000000000000000"));
  cases.push_back(std::bitset<65>("10000000000000000000000000000000000000000000000000000000000000001"));
  cases.push_back(std::bitset<65>("10000000000000000000000000011111000000000000000000000000000000001"));
  cases.push_back(std::bitset<65>("00000000000000000000000000000000011111111111111111111111111111111"));
  cases.push_back(std::bitset<65>("10000000000000000000000000000000011111111111111111111111111111111"));
  cases.push_back(std::bitset<65>("11111111111111111111111111111111000000000000000000000000000000000"));
  cases.push_back(std::bitset<65>("11111111111111111111111111111111000000000000000000000000000000001"));
  cases.push_back(std::bitset<65>("10101010101010101010101010101010101010101010101010101010101010101"));
  cases.push_back(std::bitset<65>("01010101010101010101010101010101010101010101010101010101010101010"));
  cases.push_back(std::bitset<65>("11111111111111111111111111111111111111111111111111111111111111111"));
  return cases;
}

TEST_CONSTEXPR_CXX23 inline std::string str_repeat(std::string s, unsigned int n) {
  std::string res = s;
  for (; n != 0; --n)
    res += s;
  return res;
}

template <>
TEST_CONSTEXPR_CXX23 inline std::vector<std::bitset<1000> > get_test_cases<1000>() {
  std::vector<std::bitset<1000> > cases;
  cases.push_back(std::bitset<1000>(std::string(1000, '0')));
  cases.push_back(std::bitset<1000>(std::string(999, '0') + std::string(1, '1')));
  cases.push_back(std::bitset<1000>(std::string(1, '1') + std::string(999, '0')));
  cases.push_back(std::bitset<1000>(std::string(1, '1') + std::string(998, '0') + std::string(1, '1')));
  cases.push_back(std::bitset<1000>(std::string(1, '1') + std::string(400, '0') + std::string(99, '1') +
                                    std::string(499, '0') + std::string(1, '1')));
  cases.push_back(std::bitset<1000>(std::string(500, '0') + std::string(500, '1')));
  cases.push_back(std::bitset<1000>(std::string(1, '1') + std::string(499, '0') + std::string(500, '1')));
  cases.push_back(std::bitset<1000>(std::string(500, '1') + std::string(500, '0')));
  cases.push_back(std::bitset<1000>(std::string(500, '1') + std::string(499, '0') + std::string(1, '1')));
  cases.push_back(std::bitset<1000>(str_repeat("10", 500)));
  cases.push_back(std::bitset<1000>(str_repeat("01", 500)));
  cases.push_back(std::bitset<1000>(std::string(1000, '1')));

  return cases;
}

#endif // !LIBCPP_TEST_BITSET_TEST_CASES_H
