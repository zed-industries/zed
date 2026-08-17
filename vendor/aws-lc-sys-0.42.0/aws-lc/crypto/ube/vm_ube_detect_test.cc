// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include <openssl/base.h>
#include <cstdint>

#include <gtest/gtest.h>
#include "vm_ube_detect.h"

#if defined(OPENSSL_LINUX) && defined(AWSLC_VM_UBE_TESTING)
#include <fcntl.h>
#include <cstring>
#include <sys/mman.h>

#define NUMBER_OF_TEST_VALUES 5

typedef struct sgn_test_s {
  void *addr;
} sgn_test_s;

static int init_sgn_file(void** addr);
static int init_sgn_file(void** addr) {
  *addr = nullptr;

  // This file should've been created during test initialization
  const int fd_sgn = open(CRYPTO_get_sysgenid_path(), O_RDWR);
  if (fd_sgn == -1) {
    return 0;
  }

  if (0 != lseek(fd_sgn, 0, SEEK_SET)) {
    close(fd_sgn);
    return 0;
  }

  void* my_addr = mmap(nullptr, sizeof(uint32_t), PROT_WRITE, MAP_SHARED, fd_sgn, 0);
  if (my_addr == MAP_FAILED) {
    close(fd_sgn);
    return 0;
  }

  close(fd_sgn);

  *addr = my_addr;

  return 1;
}

static int init_sgn_test(sgn_test_s* sgn_test);
static int init_sgn_test(sgn_test_s* sgn_test) {
  return init_sgn_file(&sgn_test->addr);
}

static int set_sgn(const sgn_test_s* sgn_test, uint32_t val);
static int set_sgn(const sgn_test_s* sgn_test, uint32_t val) {
  memcpy(sgn_test->addr, &val, sizeof(uint32_t));
  if(0 != msync(sgn_test->addr, sizeof(uint32_t), MS_SYNC)) {
    return 0;
  }
  return 1;
}

TEST(VmUbeGenerationTest, DISABLED_SysGenIDretrievalTesting) {
  sgn_test_s sgn_test;
  ASSERT_TRUE(init_sgn_test(&sgn_test));

  ASSERT_TRUE(set_sgn(&sgn_test, 0));

  EXPECT_EQ(1, CRYPTO_get_vm_ube_supported());
  EXPECT_EQ(1, CRYPTO_get_vm_ube_active());

  uint32_t current_vm_ube_gen_num = 0;
  ASSERT_TRUE(set_sgn(&sgn_test, 7));
  ASSERT_TRUE(CRYPTO_get_vm_ube_generation(&current_vm_ube_gen_num));
  ASSERT_EQ((uint32_t) 7, current_vm_ube_gen_num);

  uint32_t test_sysgenid_values[NUMBER_OF_TEST_VALUES] = {
    0x03, // 2^0 + 2
    0x103, // 2^8 + 3
    0x10004, // 2^16 + 4
    0x1000005, // 2^24 + 5
    0xFFFFFFFF // 2^32 - 1
  };

  for (size_t i = 0; i < NUMBER_OF_TEST_VALUES; i++) {
    // Exercise all bytes of the 32-bit generation number.
    uint32_t new_sysgenid_value_hint = test_sysgenid_values[i];
    ASSERT_TRUE(set_sgn(&sgn_test, new_sysgenid_value_hint));
    ASSERT_TRUE(CRYPTO_get_vm_ube_generation(&current_vm_ube_gen_num));
    EXPECT_EQ(new_sysgenid_value_hint, current_vm_ube_gen_num);
  }
}
#elif defined(OPENSSL_LINUX)
TEST(VmUbeGenerationTest, SysGenIDretrievalLinux) {
  uint32_t current_vm_ube_gen_num = 0xffffffff;
  ASSERT_TRUE(CRYPTO_get_vm_ube_generation(&current_vm_ube_gen_num));
  if (CRYPTO_get_vm_ube_supported()) {
    ASSERT_TRUE(CRYPTO_get_vm_ube_active());
    // If we're on a system where the SysGenId is available, we won't
    // know what sgn value to expect, but we assume it's not 0xffffffff
    ASSERT_NE(0xffffffff, current_vm_ube_gen_num);
  } else {
    ASSERT_FALSE(CRYPTO_get_vm_ube_active());
    ASSERT_EQ((uint32_t) 0, current_vm_ube_gen_num);
  }
}
#else
TEST(VmUbeGenerationTest, SysGenIDretrievalNonLinux) {
  ASSERT_FALSE(CRYPTO_get_vm_ube_supported());
  ASSERT_FALSE(CRYPTO_get_vm_ube_active());
  uint32_t current_vm_ube_gen_num = 0xffffffff;
  ASSERT_TRUE(CRYPTO_get_vm_ube_generation(&current_vm_ube_gen_num));
  ASSERT_EQ((uint32_t) 0, current_vm_ube_gen_num);
}
#endif // defined(OPENSSL_LINUX)
