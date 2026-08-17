//===-- Tests for str{,r}chr and {,r}index functions ------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#include "test/UnitTest/Test.h"

template <auto Func> struct StrchrTest : public LIBC_NAMESPACE::testing::Test {
  void findsFirstCharacter() {
    const char *src = "abcde";

    // Should return original string since 'a' is the first character.
    ASSERT_STREQ(Func(src, 'a'), "abcde");
    // Source string should not change.
    ASSERT_STREQ(src, "abcde");
  }

  void findsMiddleCharacter() {
    const char *src = "abcde";

    // Should return characters after (and including) 'c'.
    ASSERT_STREQ(Func(src, 'c'), "cde");
    // Source string should not change.
    ASSERT_STREQ(src, "abcde");
  }

  void findsLastCharacterThatIsNotNullTerminator() {
    const char *src = "abcde";

    // Should return 'e' and null-terminator.
    ASSERT_STREQ(Func(src, 'e'), "e");
    // Source string should not change.
    ASSERT_STREQ(src, "abcde");
  }

  void findsNullTerminator() {
    const char *src = "abcde";

    // Should return null terminator.
    const char *nul_terminator = Func(src, '\0');
    ASSERT_NE(nul_terminator, nullptr);
    ASSERT_STREQ(nul_terminator, "");
    // Source string should not change.
    ASSERT_STREQ(src, "abcde");
  }

  void characterNotWithinStringShouldReturnNullptr() {
    // Since 'z' is not within the string, should return nullptr.
    ASSERT_EQ(Func("123?", 'z'), nullptr);
  }

  void theSourceShouldNotChange() {
    const char *src = "abcde";
    // When the character is found, the source string should not change.
    Func(src, 'd');
    ASSERT_STREQ(src, "abcde");
    // Same case for when the character is not found.
    Func(src, 'z');
    ASSERT_STREQ(src, "abcde");
    // Same case for when looking for nullptr.
    Func(src, '\0');
    ASSERT_STREQ(src, "abcde");
  }

  void shouldFindFirstOfDuplicates() {
    // '1' is duplicated in the string, but it should find the first copy.
    ASSERT_STREQ(Func("abc1def1ghi", '1'), "1def1ghi");

    const char *dups = "XXXXX";
    // Should return original string since 'X' is the first character.
    ASSERT_STREQ(Func(dups, 'X'), dups);
  }

  void emptyStringShouldOnlyMatchNullTerminator() {
    // Null terminator should match.
    const char empty_string[] = "";
    ASSERT_EQ(static_cast<const char *>(Func(empty_string, '\0')),
              empty_string);
    // All other characters should not match.
    ASSERT_EQ(Func("", 'Z'), nullptr);
    ASSERT_EQ(Func("", '3'), nullptr);
    ASSERT_EQ(Func("", '*'), nullptr);
  }
};

template <auto Func> struct StrrchrTest : public LIBC_NAMESPACE::testing::Test {
  void findsFirstCharacter() {
    const char *src = "abcde";

    // Should return original string since 'a' is the first character.
    ASSERT_STREQ(Func(src, 'a'), "abcde");
    // Source string should not change.
    ASSERT_STREQ(src, "abcde");
  }

  void findsMiddleCharacter() {
    const char *src = "abcde";

    // Should return characters after (and including) 'c'.
    ASSERT_STREQ(Func(src, 'c'), "cde");
    // Source string should not change.
    ASSERT_STREQ(src, "abcde");
  }

  void findsLastCharacterThatIsNotNullTerminator() {
    const char *src = "abcde";

    // Should return 'e' and null-terminator.
    ASSERT_STREQ(Func(src, 'e'), "e");
    // Source string should not change.
    ASSERT_STREQ(src, "abcde");
  }

  void findsNullTerminator() {
    const char *src = "abcde";

    // Should return null terminator.
    const char *nul_terminator = Func(src, '\0');
    ASSERT_NE(nul_terminator, nullptr);
    ASSERT_STREQ(nul_terminator, "");
    // Source string should not change.
    ASSERT_STREQ(src, "abcde");
  }

  void findsLastBehindFirstNullTerminator() {
    static const char src[6] = {'a', 'a', '\0', 'b', '\0', 'c'};
    // 'b' is behind a null terminator, so should not be found.
    ASSERT_EQ(Func(src, 'b'), nullptr);
    // Same goes for 'c'.
    ASSERT_EQ(Func(src, 'c'), nullptr);

    // Should find the second of the two a's.
    ASSERT_STREQ(Func(src, 'a'), "a");
  }

  void characterNotWithinStringShouldReturnNullptr() {
    // Since 'z' is not within the string, should return nullptr.
    ASSERT_EQ(Func("123?", 'z'), nullptr);
  }

  void shouldFindLastOfDuplicates() {
    // '1' is duplicated in the string, but it should find the last copy.
    ASSERT_STREQ(Func("abc1def1ghi", '1'), "1ghi");

    const char *dups = "XXXXX";
    // Should return the last occurrence of 'X'.
    ASSERT_STREQ(Func(dups, 'X'), "X");
  }

  void emptyStringShouldOnlyMatchNullTerminator() {
    // Null terminator should match.
    const char empty_string[] = "";
    ASSERT_EQ(static_cast<const char *>(Func(empty_string, '\0')),
              empty_string);
    // All other characters should not match.
    ASSERT_EQ(Func("", 'A'), nullptr);
    ASSERT_EQ(Func("", '2'), nullptr);
    ASSERT_EQ(Func("", '*'), nullptr);
  }
};

#define STRCHR_TEST(name, func)                                                \
  using LlvmLibc##name##Test = StrchrTest<func>;                               \
  TEST_F(LlvmLibc##name##Test, FindsFirstCharacter) { findsFirstCharacter(); } \
  TEST_F(LlvmLibc##name##Test, FindsMiddleCharacter) {                         \
    findsMiddleCharacter();                                                    \
  }                                                                            \
  TEST_F(LlvmLibc##name##Test, FindsLastCharacterThatIsNotNullTerminator) {    \
    findsLastCharacterThatIsNotNullTerminator();                               \
  }                                                                            \
  TEST_F(LlvmLibc##name##Test, FindsNullTerminator) { findsNullTerminator(); } \
  TEST_F(LlvmLibc##name##Test, CharacterNotWithinStringShouldReturnNullptr) {  \
    characterNotWithinStringShouldReturnNullptr();                             \
  }                                                                            \
  TEST_F(LlvmLibc##name##Test, TheSourceShouldNotChange) {                     \
    theSourceShouldNotChange();                                                \
  }                                                                            \
  TEST_F(LlvmLibc##name##Test, ShouldFindFirstOfDuplicates) {                  \
    shouldFindFirstOfDuplicates();                                             \
  }                                                                            \
  TEST_F(LlvmLibc##name##Test, EmptyStringShouldOnlyMatchNullTerminator) {     \
    emptyStringShouldOnlyMatchNullTerminator();                                \
  }

#define STRRCHR_TEST(name, func)                                               \
  using LlvmLibc##name##Test = StrrchrTest<func>;                              \
  TEST_F(LlvmLibc##name##Test, FindsFirstCharacter) { findsFirstCharacter(); } \
  TEST_F(LlvmLibc##name##Test, FindsMiddleCharacter) {                         \
    findsMiddleCharacter();                                                    \
  }                                                                            \
  TEST_F(LlvmLibc##name##Test, FindsLastCharacterThatIsNotNullTerminator) {    \
    findsLastCharacterThatIsNotNullTerminator();                               \
  }                                                                            \
  TEST_F(LlvmLibc##name##Test, FindsNullTerminator) { findsNullTerminator(); } \
  TEST_F(LlvmLibc##name##Test, FindsLastBehindFirstNullTerminator) {           \
    findsLastBehindFirstNullTerminator();                                      \
  }                                                                            \
  TEST_F(LlvmLibc##name##Test, CharacterNotWithinStringShouldReturnNullptr) {  \
    characterNotWithinStringShouldReturnNullptr();                             \
  }                                                                            \
  TEST_F(LlvmLibc##name##Test, ShouldFindLastOfDuplicates) {                   \
    shouldFindLastOfDuplicates();                                              \
  }                                                                            \
  TEST_F(LlvmLibc##name##Test, EmptyStringShouldOnlyMatchNullTerminator) {     \
    emptyStringShouldOnlyMatchNullTerminator();                                \
  }
