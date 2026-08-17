// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include <gtest/gtest.h>
#include <openssl/mem.h>

int alloc_count = 0;
size_t test_size = 0;
int free_count = 0;
int size_count = 0;
int realloc_count = 0;

extern "C" {
  void *new_malloc_impl(size_t size, const char *file, int line);
  void new_free_impl(void *ptr, const char *file, int line);
  void *new_realloc_impl(void *ptr, size_t size, const char *file, int line);
}

void *new_malloc_impl(size_t size, const char *file, int line) {
  alloc_count++;
  test_size = size;
  return malloc(size);
}

void new_free_impl(void *ptr, const char *file, int line) {
  free_count++;
  free(ptr);
}

void *new_realloc_impl(void *ptr, size_t size, const char *file, int line) {
  realloc_count++;
  return realloc(ptr, size);
}

// This test is copy of |MemTest.BasicOverrides| in mem_test.cc.
// |MemTest.BasicOverrides| changed the |OPENSSL_malloc/free/realloc| by overriding related weak symbols.
// This test achieved the mem behavior change by calling |CRYPTO_set_mem_functions|.
TEST(MemTest, BasicMemSet) {
  // The FIPS build which runs the power on self tests can call a lot of functions
  // before this test. Therefore, all the expected counts are relative to the
  // starting point
  int starting_alloc = alloc_count;
  int starting_free = free_count;
  int starting_realloc = realloc_count;

  void *malloc_ptr = nullptr;
  // Check setup conditions
  ASSERT_EQ(starting_alloc, alloc_count);
  ASSERT_EQ(starting_free, free_count);
  ASSERT_EQ(starting_realloc, realloc_count);
  ASSERT_EQ(0, size_count);
  ASSERT_EQ(malloc_ptr, nullptr);

  // Call |CRYPTO_set_mem_functions| to override |OPENSSL_malloc/realloc/free|.
  ASSERT_EQ(1, CRYPTO_set_mem_functions(new_malloc_impl, new_realloc_impl, new_free_impl));

  // Verify malloc calls |new_malloc_impl| and doesn't do anything else
  test_size = 10;
  malloc_ptr = OPENSSL_malloc(test_size);
  ASSERT_NE(malloc_ptr, nullptr);
  ASSERT_EQ(starting_alloc + 1, alloc_count);
  ASSERT_EQ(starting_free, free_count);
  ASSERT_EQ(starting_realloc, realloc_count);
  ASSERT_EQ(0, size_count);

  // Calling realloc with null actually calls |OPENSSL_malloc| which calls
  // |new_realloc_impl|
  void *null_realloc_ptr = nullptr;
  test_size = 200;
  void *realloc_ptr_1 = OPENSSL_realloc(null_realloc_ptr, test_size);
  ASSERT_NE(realloc_ptr_1, nullptr);
  ASSERT_EQ(starting_alloc + 2, alloc_count);
  ASSERT_EQ(starting_free, free_count);
  ASSERT_EQ(starting_realloc, realloc_count);
  ASSERT_EQ(0, size_count);
  ASSERT_EQ(nullptr, null_realloc_ptr);

  // Calling realloc with a not null pointer calls |new_realloc_impl|
  test_size = 300;
  void *realloc_ptr_2 = OPENSSL_realloc(malloc_ptr, test_size);
  ASSERT_EQ(starting_alloc + 2, alloc_count);
  ASSERT_EQ(starting_free, free_count);
  ASSERT_EQ(starting_realloc + 1, realloc_count);
  ASSERT_EQ(0, size_count);

  // Check |new_free_impl| calls our symbol as expected
  OPENSSL_free(realloc_ptr_1);
  ASSERT_EQ(starting_alloc + 2, alloc_count);
  ASSERT_EQ(starting_free + 1, free_count);
  ASSERT_EQ(starting_realloc + 1, realloc_count);
  ASSERT_EQ(0, size_count);
  OPENSSL_free(realloc_ptr_2);
}
