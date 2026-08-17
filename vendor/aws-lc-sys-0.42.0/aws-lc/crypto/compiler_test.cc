// Copyright (c) 2017, Google Inc.
// SPDX-License-Identifier: ISC

#include <limits.h>
#include <stdint.h>
#include <string.h>

#include <type_traits>

#include <gtest/gtest.h>

#include "test/test_util.h"


// C and C++ have two forms of unspecified behavior: undefined behavior and
// implementation-defined behavior.
//
// Programs that exhibit undefined behavior are invalid. Compilers are
// permitted to, and often do, arbitrarily miscompile them. BoringSSL thus aims
// to avoid undefined behavior.
//
// Implementation-defined behavior is left up to the compiler to define (or
// leave undefined). These are often platform-specific details, such as how big
// |int| is or how |uintN_t| is implemented. Programs that depend on
// implementation-defined behavior are not necessarily invalid, merely less
// portable. A compiler that provides some implementation-defined behavior is
// not permitted to miscompile code that depends on it.
//
// C allows a much wider range of platform behaviors than would be practical
// for us to support, so we make some assumptions on implementation-defined
// behavior. Platforms that violate those assumptions are not supported. This
// file aims to document and test these assumptions, so that platforms outside
// our scope are flagged.

template <typename T>
static void CheckRepresentation(T value) {
  SCOPED_TRACE(value);

  // Convert to the corresponding two's-complement unsigned value. We use an
  // unsigned value so the right-shift below has defined value. Right-shifts of
  // negative numbers in C are implementation defined.
  //
  // If |T| is already unsigned, this is a no-op, as desired.
  //
  // If |T| is signed, conversion to unsigned is defined to repeatedly add or
  // subtract (numerically, not within |T|) one more than the unsigned type's
  // maximum value until it fits (this must be a power of two). This is the
  // conversion we want.
  using UnsignedT = typename std::make_unsigned<T>::type;
  UnsignedT value_u = static_cast<UnsignedT>(value);
  EXPECT_EQ(sizeof(UnsignedT), sizeof(T));

  uint8_t expected[sizeof(UnsignedT)];
  for (size_t i = 0; i < sizeof(UnsignedT); i++) {
#ifdef OPENSSL_BIG_ENDIAN
    expected[sizeof(UnsignedT) - i - 1] = static_cast<uint8_t>(value_u);
#else
    expected[i] = static_cast<uint8_t>(value_u);
#endif
    // Divide instead of right-shift to appease compilers that warn if |T| is a
    // char. The explicit cast is also needed to appease MSVC if integer
    // promotion happened.
    value_u = static_cast<UnsignedT>(value_u / 256);
  }
  EXPECT_EQ(0u, value_u);

  // Check that |value| has the expected representation.
  EXPECT_EQ(Bytes(expected),
            Bytes(reinterpret_cast<const uint8_t *>(&value), sizeof(value)));
}

TEST(CompilerTest, IntegerRepresentation) {
  static_assert(CHAR_BIT == 8, "BoringSSL only supports 8-bit chars");
  static_assert(UCHAR_MAX == 0xff, "BoringSSL only supports 8-bit chars");

  // Require that |unsigned char| and |uint8_t| be the same type. We require
  // that type-punning through |uint8_t| is not a strict aliasing violation. In
  // principle, type-punning should be done with |memcpy|, which would make this
  // moot.
  //
  // However, C made too many historical mistakes with the types and signedness
  // of character strings. As a result, aliasing between all variations on 8-bit
  // chars are a practical necessity for all real C code. We do not support
  // toolchains that break this assumption.
  static_assert(
      std::is_same<unsigned char, uint8_t>::value,
      "BoringSSL requires uint8_t and unsigned char be the same type");
  uint8_t u8 = 0;
  unsigned char *ptr = &u8;
  (void)ptr;

  // Sized integers have the expected size.
  static_assert(sizeof(uint8_t) == 1u, "uint8_t has the wrong size");
  static_assert(sizeof(uint16_t) == 2u, "uint16_t has the wrong size");
  static_assert(sizeof(uint32_t) == 4u, "uint32_t has the wrong size");
  static_assert(sizeof(uint64_t) == 8u, "uint64_t has the wrong size");

  // size_t does not exceed uint64_t.
  static_assert(sizeof(size_t) <= 8u, "size_t must not exceed uint64_t");

  // Require that |int| be exactly 32 bits. OpenSSL historically mixed up
  // |unsigned| and |uint32_t|, so we require it be at least 32 bits. Requiring
  // at most 32-bits is a bit more subtle. C promotes arithemetic operands to
  // |int| when they fit. But this means, if |int| is 2N bits wide, multiplying
  // two maximum-sized |uintN_t|s is undefined by integer overflow!
  //
  // We attempt to handle this for |uint16_t|, assuming a 32-bit |int|, but we
  // make no attempts to correct for this with |uint32_t| for a 64-bit |int|.
  // Thus BoringSSL does not support ILP64 platforms.
  //
  // This test is on |INT_MAX| and |INT32_MAX| rather than sizeof because it is
  // theoretically allowed for sizeof(int) to be 4 but include padding bits.
  static_assert(INT_MAX == INT32_MAX, "BoringSSL requires int be 32-bit");
  static_assert(UINT_MAX == UINT32_MAX,
                "BoringSSL requires unsigned be 32-bit");

  CheckRepresentation(static_cast<signed char>(127));
  CheckRepresentation(static_cast<signed char>(1));
  CheckRepresentation(static_cast<signed char>(0));
  CheckRepresentation(static_cast<signed char>(-1));
  CheckRepresentation(static_cast<signed char>(-42));
  CheckRepresentation(static_cast<signed char>(-128));

  CheckRepresentation(static_cast<int>(INT_MAX));
  CheckRepresentation(static_cast<int>(0x12345678));
  CheckRepresentation(static_cast<int>(1));
  CheckRepresentation(static_cast<int>(0));
  CheckRepresentation(static_cast<int>(-1));
  CheckRepresentation(static_cast<int>(-0x12345678));
  CheckRepresentation(static_cast<int>(INT_MIN));

  CheckRepresentation(static_cast<unsigned>(UINT_MAX));
  CheckRepresentation(static_cast<unsigned>(0x12345678));
  CheckRepresentation(static_cast<unsigned>(1));
  CheckRepresentation(static_cast<unsigned>(0));

  CheckRepresentation(static_cast<long>(LONG_MAX));
  CheckRepresentation(static_cast<long>(0x12345678));
  CheckRepresentation(static_cast<long>(1));
  CheckRepresentation(static_cast<long>(0));
  CheckRepresentation(static_cast<long>(-1));
  CheckRepresentation(static_cast<long>(-0x12345678));
  CheckRepresentation(static_cast<long>(LONG_MIN));

  CheckRepresentation(static_cast<unsigned long>(ULONG_MAX));
  CheckRepresentation(static_cast<unsigned long>(0x12345678));
  CheckRepresentation(static_cast<unsigned long>(1));
  CheckRepresentation(static_cast<unsigned long>(0));

  CheckRepresentation(static_cast<int16_t>(0x7fff));
  CheckRepresentation(static_cast<int16_t>(0x1234));
  CheckRepresentation(static_cast<int16_t>(1));
  CheckRepresentation(static_cast<int16_t>(0));
  CheckRepresentation(static_cast<int16_t>(-1));
  CheckRepresentation(static_cast<int16_t>(-0x7fff - 1));

  CheckRepresentation(static_cast<uint16_t>(0xffff));
  CheckRepresentation(static_cast<uint16_t>(0x1234));
  CheckRepresentation(static_cast<uint16_t>(1));
  CheckRepresentation(static_cast<uint16_t>(0));

  CheckRepresentation(static_cast<int32_t>(0x7fffffff));
  CheckRepresentation(static_cast<int32_t>(0x12345678));
  CheckRepresentation(static_cast<int32_t>(1));
  CheckRepresentation(static_cast<int32_t>(0));
  CheckRepresentation(static_cast<int32_t>(-1));
  CheckRepresentation(static_cast<int32_t>(-0x7fffffff - 1));

  CheckRepresentation(static_cast<uint32_t>(0xffffffff));
  CheckRepresentation(static_cast<uint32_t>(0x12345678));
  CheckRepresentation(static_cast<uint32_t>(1));
  CheckRepresentation(static_cast<uint32_t>(0));

  CheckRepresentation(static_cast<int64_t>(0x7fffffffffffffff));
  CheckRepresentation(static_cast<int64_t>(0x123456789abcdef0));
  CheckRepresentation(static_cast<int64_t>(1));
  CheckRepresentation(static_cast<int64_t>(0));
  CheckRepresentation(static_cast<int64_t>(-1));
  CheckRepresentation(static_cast<int64_t>(-0x7fffffffffffffff - 1));

  CheckRepresentation(static_cast<uint64_t>(0xffffffffffffffff));
  CheckRepresentation(static_cast<uint64_t>(0x12345678abcdef0));
  CheckRepresentation(static_cast<uint64_t>(1));
  CheckRepresentation(static_cast<uint64_t>(0));
}

TEST(CompilerTest, PointerRepresentation) {
  // Converting pointers to integers and doing arithmetic on those values are
  // both defined. Converting those values back into pointers is undefined,
  // but, for aliasing checks, we require that the implementation-defined
  // result of that computation commutes with pointer arithmetic.
  char chars[256];
  for (size_t i = 0; i < sizeof(chars); i++) {
    EXPECT_EQ(reinterpret_cast<uintptr_t>(chars) + i,
              reinterpret_cast<uintptr_t>(chars + i));
  }

  int ints[256];
  for (size_t i = 0; i < OPENSSL_ARRAY_SIZE(ints); i++) {
    EXPECT_EQ(reinterpret_cast<uintptr_t>(ints) + i * sizeof(int),
              reinterpret_cast<uintptr_t>(ints + i));
  }

  // nullptr must be represented by all zeros in memory. This is necessary so
  // structs may be initialized by memset(0).
  int *null = nullptr;
  uint8_t bytes[sizeof(null)] = {0};
  EXPECT_EQ(Bytes(bytes),
            Bytes(reinterpret_cast<uint8_t *>(&null), sizeof(null)));
}

static bool verify_memory_alignment(void *aligned_ptr,
                                    size_t requested_alignment) {
  if ((((uintptr_t) aligned_ptr) % requested_alignment) == 0) {
    return true;
  }
  else {
    std::cerr << "requested_alignment = " << requested_alignment << std::endl;
    std::cerr << "aligned_ptr = " << reinterpret_cast<void *>(aligned_ptr) << std::endl;
    std::cerr << "aligned_ptr % requested_alignment = " << ((uintptr_t) aligned_ptr) % requested_alignment << std::endl;
  }

  return false;
}

typedef struct struct_array_type {
  uint8_t buffer_uint8_t[10];
  char buffer_char[21];
} struct_array_st;

typedef union union_array_type {
  char buffer_char[9];
  uint8_t buffer_uint8_t[16];
} union_array_st;

#define CHECK_STACK_ALIGNMENT(type, power_of_two, memory_size) \
  stack_align_type buffer_##type##_##power_of_two##_##memory_size[power_of_two + memory_size]; \
  type *aligned_##type##_##power_of_two##_##memory_size = (type *) align_pointer(buffer_##type##_##power_of_two##_##memory_size, power_of_two); \
  ASSERT_TRUE(aligned_##type##_##power_of_two##_##memory_size); \
  ASSERT_TRUE(verify_memory_alignment(aligned_##type##_##power_of_two##_##memory_size, power_of_two));

// Macro lists produced with the following Python script:
//  MACRO_NAME = 'CHECK_STACK_ALIGNMENT'
//  type_names = ['uint8_t', 'char', 'struct_array_st', 'union_array_type']
//  power_of_twos = [1, 2, 4, 8, 16, 32, 64]
//  for type_name in type_names:
//    for power_of_two in power_of_twos:
//      for memory_size in range(1, power_of_two):
//        print '{}({}, {}, {})'.format(MACRO_NAME, type_name, power_of_two, memory_size)

// Windows doesn't like the big virtual functions produced. So, split this into
// several test fixtures.

TEST(AlignmentTest, StackBufferManualAlignmentTypes1) {

  CHECK_STACK_ALIGNMENT(uint8_t, 2, 1)
  CHECK_STACK_ALIGNMENT(uint8_t, 4, 1)
  CHECK_STACK_ALIGNMENT(uint8_t, 4, 2)
  CHECK_STACK_ALIGNMENT(uint8_t, 4, 3)
  CHECK_STACK_ALIGNMENT(uint8_t, 8, 1)
  CHECK_STACK_ALIGNMENT(uint8_t, 8, 2)
  CHECK_STACK_ALIGNMENT(uint8_t, 8, 3)
  CHECK_STACK_ALIGNMENT(uint8_t, 8, 4)
  CHECK_STACK_ALIGNMENT(uint8_t, 8, 5)
  CHECK_STACK_ALIGNMENT(uint8_t, 8, 6)
  CHECK_STACK_ALIGNMENT(uint8_t, 8, 7)
  CHECK_STACK_ALIGNMENT(uint8_t, 16, 1)
  CHECK_STACK_ALIGNMENT(uint8_t, 16, 2)
  CHECK_STACK_ALIGNMENT(uint8_t, 16, 3)
  CHECK_STACK_ALIGNMENT(uint8_t, 16, 4)
  CHECK_STACK_ALIGNMENT(uint8_t, 16, 5)
  CHECK_STACK_ALIGNMENT(uint8_t, 16, 6)
  CHECK_STACK_ALIGNMENT(uint8_t, 16, 7)
  CHECK_STACK_ALIGNMENT(uint8_t, 16, 8)
  CHECK_STACK_ALIGNMENT(uint8_t, 16, 9)
  CHECK_STACK_ALIGNMENT(uint8_t, 16, 10)
  CHECK_STACK_ALIGNMENT(uint8_t, 16, 11)
  CHECK_STACK_ALIGNMENT(uint8_t, 16, 12)
  CHECK_STACK_ALIGNMENT(uint8_t, 16, 13)
  CHECK_STACK_ALIGNMENT(uint8_t, 16, 14)
  CHECK_STACK_ALIGNMENT(uint8_t, 16, 15)
  CHECK_STACK_ALIGNMENT(uint8_t, 32, 1)
  CHECK_STACK_ALIGNMENT(uint8_t, 32, 2)
  CHECK_STACK_ALIGNMENT(uint8_t, 32, 3)
  CHECK_STACK_ALIGNMENT(uint8_t, 32, 4)
  CHECK_STACK_ALIGNMENT(uint8_t, 32, 5)
  CHECK_STACK_ALIGNMENT(uint8_t, 32, 6)
  CHECK_STACK_ALIGNMENT(uint8_t, 32, 7)
  CHECK_STACK_ALIGNMENT(uint8_t, 32, 8)
  CHECK_STACK_ALIGNMENT(uint8_t, 32, 9)
  CHECK_STACK_ALIGNMENT(uint8_t, 32, 10)
  CHECK_STACK_ALIGNMENT(uint8_t, 32, 11)
  CHECK_STACK_ALIGNMENT(uint8_t, 32, 12)
  CHECK_STACK_ALIGNMENT(uint8_t, 32, 13)
  CHECK_STACK_ALIGNMENT(uint8_t, 32, 14)
  CHECK_STACK_ALIGNMENT(uint8_t, 32, 15)
  CHECK_STACK_ALIGNMENT(uint8_t, 32, 16)
  CHECK_STACK_ALIGNMENT(uint8_t, 32, 17)
  CHECK_STACK_ALIGNMENT(uint8_t, 32, 18)
  CHECK_STACK_ALIGNMENT(uint8_t, 32, 19)
  CHECK_STACK_ALIGNMENT(uint8_t, 32, 20)
  CHECK_STACK_ALIGNMENT(uint8_t, 32, 21)
  CHECK_STACK_ALIGNMENT(uint8_t, 32, 22)
  CHECK_STACK_ALIGNMENT(uint8_t, 32, 23)
  CHECK_STACK_ALIGNMENT(uint8_t, 32, 24)
  CHECK_STACK_ALIGNMENT(uint8_t, 32, 25)
  CHECK_STACK_ALIGNMENT(uint8_t, 32, 26)
  CHECK_STACK_ALIGNMENT(uint8_t, 32, 27)
  CHECK_STACK_ALIGNMENT(uint8_t, 32, 28)
  CHECK_STACK_ALIGNMENT(uint8_t, 32, 29)
  CHECK_STACK_ALIGNMENT(uint8_t, 32, 30)
  CHECK_STACK_ALIGNMENT(uint8_t, 32, 31)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 1)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 2)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 3)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 4)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 5)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 6)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 7)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 8)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 9)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 10)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 11)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 12)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 13)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 14)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 15)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 16)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 17)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 18)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 19)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 20)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 21)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 22)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 23)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 24)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 25)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 26)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 27)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 28)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 29)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 30)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 31)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 32)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 33)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 34)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 35)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 36)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 37)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 38)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 39)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 40)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 41)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 42)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 43)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 44)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 45)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 46)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 47)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 48)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 49)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 50)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 51)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 52)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 53)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 54)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 55)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 56)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 57)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 58)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 59)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 60)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 61)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 62)
  CHECK_STACK_ALIGNMENT(uint8_t, 64, 63)
  CHECK_STACK_ALIGNMENT(char, 2, 1)
  CHECK_STACK_ALIGNMENT(char, 4, 1)
  CHECK_STACK_ALIGNMENT(char, 4, 2)
  CHECK_STACK_ALIGNMENT(char, 4, 3)
  CHECK_STACK_ALIGNMENT(char, 8, 1)
  CHECK_STACK_ALIGNMENT(char, 8, 2)
  CHECK_STACK_ALIGNMENT(char, 8, 3)
  CHECK_STACK_ALIGNMENT(char, 8, 4)
  CHECK_STACK_ALIGNMENT(char, 8, 5)
  CHECK_STACK_ALIGNMENT(char, 8, 6)
  CHECK_STACK_ALIGNMENT(char, 8, 7)
  CHECK_STACK_ALIGNMENT(char, 16, 1)
  CHECK_STACK_ALIGNMENT(char, 16, 2)
  CHECK_STACK_ALIGNMENT(char, 16, 3)
  CHECK_STACK_ALIGNMENT(char, 16, 4)
  CHECK_STACK_ALIGNMENT(char, 16, 5)
  CHECK_STACK_ALIGNMENT(char, 16, 6)
  CHECK_STACK_ALIGNMENT(char, 16, 7)
  CHECK_STACK_ALIGNMENT(char, 16, 8)
  CHECK_STACK_ALIGNMENT(char, 16, 9)
  CHECK_STACK_ALIGNMENT(char, 16, 10)
  CHECK_STACK_ALIGNMENT(char, 16, 11)
  CHECK_STACK_ALIGNMENT(char, 16, 12)
  CHECK_STACK_ALIGNMENT(char, 16, 13)
  CHECK_STACK_ALIGNMENT(char, 16, 14)
  CHECK_STACK_ALIGNMENT(char, 16, 15)
  CHECK_STACK_ALIGNMENT(char, 32, 1)
  CHECK_STACK_ALIGNMENT(char, 32, 2)
  CHECK_STACK_ALIGNMENT(char, 32, 3)
  CHECK_STACK_ALIGNMENT(char, 32, 4)
  CHECK_STACK_ALIGNMENT(char, 32, 5)
  CHECK_STACK_ALIGNMENT(char, 32, 6)
  CHECK_STACK_ALIGNMENT(char, 32, 7)
  CHECK_STACK_ALIGNMENT(char, 32, 8)
  CHECK_STACK_ALIGNMENT(char, 32, 9)
  CHECK_STACK_ALIGNMENT(char, 32, 10)
  CHECK_STACK_ALIGNMENT(char, 32, 11)
  CHECK_STACK_ALIGNMENT(char, 32, 12)
  CHECK_STACK_ALIGNMENT(char, 32, 13)
  CHECK_STACK_ALIGNMENT(char, 32, 14)
  CHECK_STACK_ALIGNMENT(char, 32, 15)
  CHECK_STACK_ALIGNMENT(char, 32, 16)
  CHECK_STACK_ALIGNMENT(char, 32, 17)
  CHECK_STACK_ALIGNMENT(char, 32, 18)
  CHECK_STACK_ALIGNMENT(char, 32, 19)
  CHECK_STACK_ALIGNMENT(char, 32, 20)
  CHECK_STACK_ALIGNMENT(char, 32, 21)
  CHECK_STACK_ALIGNMENT(char, 32, 22)
  CHECK_STACK_ALIGNMENT(char, 32, 23)
  CHECK_STACK_ALIGNMENT(char, 32, 24)
  CHECK_STACK_ALIGNMENT(char, 32, 25)
  CHECK_STACK_ALIGNMENT(char, 32, 26)
  CHECK_STACK_ALIGNMENT(char, 32, 27)
  CHECK_STACK_ALIGNMENT(char, 32, 28)
  CHECK_STACK_ALIGNMENT(char, 32, 29)
  CHECK_STACK_ALIGNMENT(char, 32, 30)
  CHECK_STACK_ALIGNMENT(char, 32, 31)
  CHECK_STACK_ALIGNMENT(char, 64, 1)
  CHECK_STACK_ALIGNMENT(char, 64, 2)
  CHECK_STACK_ALIGNMENT(char, 64, 3)
  CHECK_STACK_ALIGNMENT(char, 64, 4)
  CHECK_STACK_ALIGNMENT(char, 64, 5)
  CHECK_STACK_ALIGNMENT(char, 64, 6)
  CHECK_STACK_ALIGNMENT(char, 64, 7)
  CHECK_STACK_ALIGNMENT(char, 64, 8)
  CHECK_STACK_ALIGNMENT(char, 64, 9)
  CHECK_STACK_ALIGNMENT(char, 64, 10)
  CHECK_STACK_ALIGNMENT(char, 64, 11)
  CHECK_STACK_ALIGNMENT(char, 64, 12)
  CHECK_STACK_ALIGNMENT(char, 64, 13)
  CHECK_STACK_ALIGNMENT(char, 64, 14)
  CHECK_STACK_ALIGNMENT(char, 64, 15)
  CHECK_STACK_ALIGNMENT(char, 64, 16)
  CHECK_STACK_ALIGNMENT(char, 64, 17)
  CHECK_STACK_ALIGNMENT(char, 64, 18)
  CHECK_STACK_ALIGNMENT(char, 64, 19)
  CHECK_STACK_ALIGNMENT(char, 64, 20)
  CHECK_STACK_ALIGNMENT(char, 64, 21)
  CHECK_STACK_ALIGNMENT(char, 64, 22)
  CHECK_STACK_ALIGNMENT(char, 64, 23)
  CHECK_STACK_ALIGNMENT(char, 64, 24)
  CHECK_STACK_ALIGNMENT(char, 64, 25)
  CHECK_STACK_ALIGNMENT(char, 64, 26)
  CHECK_STACK_ALIGNMENT(char, 64, 27)
  CHECK_STACK_ALIGNMENT(char, 64, 28)
  CHECK_STACK_ALIGNMENT(char, 64, 29)
  CHECK_STACK_ALIGNMENT(char, 64, 30)
  CHECK_STACK_ALIGNMENT(char, 64, 31)
  CHECK_STACK_ALIGNMENT(char, 64, 32)
  CHECK_STACK_ALIGNMENT(char, 64, 33)
  CHECK_STACK_ALIGNMENT(char, 64, 34)
  CHECK_STACK_ALIGNMENT(char, 64, 35)
  CHECK_STACK_ALIGNMENT(char, 64, 36)
  CHECK_STACK_ALIGNMENT(char, 64, 37)
  CHECK_STACK_ALIGNMENT(char, 64, 38)
  CHECK_STACK_ALIGNMENT(char, 64, 39)
  CHECK_STACK_ALIGNMENT(char, 64, 40)
  CHECK_STACK_ALIGNMENT(char, 64, 41)
  CHECK_STACK_ALIGNMENT(char, 64, 42)
  CHECK_STACK_ALIGNMENT(char, 64, 43)
  CHECK_STACK_ALIGNMENT(char, 64, 44)
  CHECK_STACK_ALIGNMENT(char, 64, 45)
  CHECK_STACK_ALIGNMENT(char, 64, 46)
  CHECK_STACK_ALIGNMENT(char, 64, 47)
  CHECK_STACK_ALIGNMENT(char, 64, 48)
  CHECK_STACK_ALIGNMENT(char, 64, 49)
  CHECK_STACK_ALIGNMENT(char, 64, 50)
  CHECK_STACK_ALIGNMENT(char, 64, 51)
  CHECK_STACK_ALIGNMENT(char, 64, 52)
  CHECK_STACK_ALIGNMENT(char, 64, 53)
  CHECK_STACK_ALIGNMENT(char, 64, 54)
  CHECK_STACK_ALIGNMENT(char, 64, 55)
  CHECK_STACK_ALIGNMENT(char, 64, 56)
  CHECK_STACK_ALIGNMENT(char, 64, 57)
  CHECK_STACK_ALIGNMENT(char, 64, 58)
  CHECK_STACK_ALIGNMENT(char, 64, 59)
  CHECK_STACK_ALIGNMENT(char, 64, 60)
  CHECK_STACK_ALIGNMENT(char, 64, 61)
  CHECK_STACK_ALIGNMENT(char, 64, 62)
  CHECK_STACK_ALIGNMENT(char, 64, 63)

}

TEST(AlignmentTest, StackBufferManualAlignmentTypes2) {

  CHECK_STACK_ALIGNMENT(struct_array_st, 2, 1)
  CHECK_STACK_ALIGNMENT(struct_array_st, 4, 1)
  CHECK_STACK_ALIGNMENT(struct_array_st, 4, 2)
  CHECK_STACK_ALIGNMENT(struct_array_st, 4, 3)
  CHECK_STACK_ALIGNMENT(struct_array_st, 8, 1)
  CHECK_STACK_ALIGNMENT(struct_array_st, 8, 2)
  CHECK_STACK_ALIGNMENT(struct_array_st, 8, 3)
  CHECK_STACK_ALIGNMENT(struct_array_st, 8, 4)
  CHECK_STACK_ALIGNMENT(struct_array_st, 8, 5)
  CHECK_STACK_ALIGNMENT(struct_array_st, 8, 6)
  CHECK_STACK_ALIGNMENT(struct_array_st, 8, 7)
  CHECK_STACK_ALIGNMENT(struct_array_st, 16, 1)
  CHECK_STACK_ALIGNMENT(struct_array_st, 16, 2)
  CHECK_STACK_ALIGNMENT(struct_array_st, 16, 3)
  CHECK_STACK_ALIGNMENT(struct_array_st, 16, 4)
  CHECK_STACK_ALIGNMENT(struct_array_st, 16, 5)
  CHECK_STACK_ALIGNMENT(struct_array_st, 16, 6)
  CHECK_STACK_ALIGNMENT(struct_array_st, 16, 7)
  CHECK_STACK_ALIGNMENT(struct_array_st, 16, 8)
  CHECK_STACK_ALIGNMENT(struct_array_st, 16, 9)
  CHECK_STACK_ALIGNMENT(struct_array_st, 16, 10)
  CHECK_STACK_ALIGNMENT(struct_array_st, 16, 11)
  CHECK_STACK_ALIGNMENT(struct_array_st, 16, 12)
  CHECK_STACK_ALIGNMENT(struct_array_st, 16, 13)
  CHECK_STACK_ALIGNMENT(struct_array_st, 16, 14)
  CHECK_STACK_ALIGNMENT(struct_array_st, 16, 15)
  CHECK_STACK_ALIGNMENT(struct_array_st, 32, 1)
  CHECK_STACK_ALIGNMENT(struct_array_st, 32, 2)
  CHECK_STACK_ALIGNMENT(struct_array_st, 32, 3)
  CHECK_STACK_ALIGNMENT(struct_array_st, 32, 4)
  CHECK_STACK_ALIGNMENT(struct_array_st, 32, 5)
  CHECK_STACK_ALIGNMENT(struct_array_st, 32, 6)
  CHECK_STACK_ALIGNMENT(struct_array_st, 32, 7)
  CHECK_STACK_ALIGNMENT(struct_array_st, 32, 8)
  CHECK_STACK_ALIGNMENT(struct_array_st, 32, 9)
  CHECK_STACK_ALIGNMENT(struct_array_st, 32, 10)
  CHECK_STACK_ALIGNMENT(struct_array_st, 32, 11)
  CHECK_STACK_ALIGNMENT(struct_array_st, 32, 12)
  CHECK_STACK_ALIGNMENT(struct_array_st, 32, 13)
  CHECK_STACK_ALIGNMENT(struct_array_st, 32, 14)
  CHECK_STACK_ALIGNMENT(struct_array_st, 32, 15)
  CHECK_STACK_ALIGNMENT(struct_array_st, 32, 16)
  CHECK_STACK_ALIGNMENT(struct_array_st, 32, 17)
  CHECK_STACK_ALIGNMENT(struct_array_st, 32, 18)
  CHECK_STACK_ALIGNMENT(struct_array_st, 32, 19)
  CHECK_STACK_ALIGNMENT(struct_array_st, 32, 20)
  CHECK_STACK_ALIGNMENT(struct_array_st, 32, 21)
  CHECK_STACK_ALIGNMENT(struct_array_st, 32, 22)
  CHECK_STACK_ALIGNMENT(struct_array_st, 32, 23)
  CHECK_STACK_ALIGNMENT(struct_array_st, 32, 24)
  CHECK_STACK_ALIGNMENT(struct_array_st, 32, 25)
  CHECK_STACK_ALIGNMENT(struct_array_st, 32, 26)
  CHECK_STACK_ALIGNMENT(struct_array_st, 32, 27)
  CHECK_STACK_ALIGNMENT(struct_array_st, 32, 28)
  CHECK_STACK_ALIGNMENT(struct_array_st, 32, 29)
  CHECK_STACK_ALIGNMENT(struct_array_st, 32, 30)
  CHECK_STACK_ALIGNMENT(struct_array_st, 32, 31)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 1)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 2)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 3)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 4)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 5)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 6)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 7)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 8)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 9)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 10)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 11)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 12)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 13)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 14)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 15)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 16)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 17)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 18)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 19)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 20)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 21)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 22)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 23)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 24)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 25)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 26)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 27)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 28)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 29)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 30)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 31)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 32)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 33)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 34)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 35)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 36)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 37)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 38)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 39)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 40)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 41)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 42)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 43)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 44)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 45)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 46)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 47)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 48)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 49)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 50)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 51)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 52)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 53)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 54)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 55)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 56)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 57)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 58)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 59)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 60)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 61)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 62)
  CHECK_STACK_ALIGNMENT(struct_array_st, 64, 63)
  CHECK_STACK_ALIGNMENT(union_array_type, 2, 1)
  CHECK_STACK_ALIGNMENT(union_array_type, 4, 1)
  CHECK_STACK_ALIGNMENT(union_array_type, 4, 2)
  CHECK_STACK_ALIGNMENT(union_array_type, 4, 3)
  CHECK_STACK_ALIGNMENT(union_array_type, 8, 1)
  CHECK_STACK_ALIGNMENT(union_array_type, 8, 2)
  CHECK_STACK_ALIGNMENT(union_array_type, 8, 3)
  CHECK_STACK_ALIGNMENT(union_array_type, 8, 4)
  CHECK_STACK_ALIGNMENT(union_array_type, 8, 5)
  CHECK_STACK_ALIGNMENT(union_array_type, 8, 6)
  CHECK_STACK_ALIGNMENT(union_array_type, 8, 7)
  CHECK_STACK_ALIGNMENT(union_array_type, 16, 1)
  CHECK_STACK_ALIGNMENT(union_array_type, 16, 2)
  CHECK_STACK_ALIGNMENT(union_array_type, 16, 3)
  CHECK_STACK_ALIGNMENT(union_array_type, 16, 4)
  CHECK_STACK_ALIGNMENT(union_array_type, 16, 5)
  CHECK_STACK_ALIGNMENT(union_array_type, 16, 6)
  CHECK_STACK_ALIGNMENT(union_array_type, 16, 7)
  CHECK_STACK_ALIGNMENT(union_array_type, 16, 8)
  CHECK_STACK_ALIGNMENT(union_array_type, 16, 9)
  CHECK_STACK_ALIGNMENT(union_array_type, 16, 10)
  CHECK_STACK_ALIGNMENT(union_array_type, 16, 11)
  CHECK_STACK_ALIGNMENT(union_array_type, 16, 12)
  CHECK_STACK_ALIGNMENT(union_array_type, 16, 13)
  CHECK_STACK_ALIGNMENT(union_array_type, 16, 14)
  CHECK_STACK_ALIGNMENT(union_array_type, 16, 15)
  CHECK_STACK_ALIGNMENT(union_array_type, 32, 1)
  CHECK_STACK_ALIGNMENT(union_array_type, 32, 2)
  CHECK_STACK_ALIGNMENT(union_array_type, 32, 3)
  CHECK_STACK_ALIGNMENT(union_array_type, 32, 4)
  CHECK_STACK_ALIGNMENT(union_array_type, 32, 5)
  CHECK_STACK_ALIGNMENT(union_array_type, 32, 6)
  CHECK_STACK_ALIGNMENT(union_array_type, 32, 7)
  CHECK_STACK_ALIGNMENT(union_array_type, 32, 8)
  CHECK_STACK_ALIGNMENT(union_array_type, 32, 9)
  CHECK_STACK_ALIGNMENT(union_array_type, 32, 10)
  CHECK_STACK_ALIGNMENT(union_array_type, 32, 11)
  CHECK_STACK_ALIGNMENT(union_array_type, 32, 12)
  CHECK_STACK_ALIGNMENT(union_array_type, 32, 13)
  CHECK_STACK_ALIGNMENT(union_array_type, 32, 14)
  CHECK_STACK_ALIGNMENT(union_array_type, 32, 15)
  CHECK_STACK_ALIGNMENT(union_array_type, 32, 16)
  CHECK_STACK_ALIGNMENT(union_array_type, 32, 17)
  CHECK_STACK_ALIGNMENT(union_array_type, 32, 18)
  CHECK_STACK_ALIGNMENT(union_array_type, 32, 19)
  CHECK_STACK_ALIGNMENT(union_array_type, 32, 20)
  CHECK_STACK_ALIGNMENT(union_array_type, 32, 21)
  CHECK_STACK_ALIGNMENT(union_array_type, 32, 22)
  CHECK_STACK_ALIGNMENT(union_array_type, 32, 23)
  CHECK_STACK_ALIGNMENT(union_array_type, 32, 24)
  CHECK_STACK_ALIGNMENT(union_array_type, 32, 25)
  CHECK_STACK_ALIGNMENT(union_array_type, 32, 26)
  CHECK_STACK_ALIGNMENT(union_array_type, 32, 27)
  CHECK_STACK_ALIGNMENT(union_array_type, 32, 28)
  CHECK_STACK_ALIGNMENT(union_array_type, 32, 29)
  CHECK_STACK_ALIGNMENT(union_array_type, 32, 30)
  CHECK_STACK_ALIGNMENT(union_array_type, 32, 31)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 1)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 2)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 3)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 4)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 5)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 6)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 7)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 8)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 9)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 10)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 11)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 12)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 13)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 14)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 15)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 16)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 17)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 18)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 19)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 20)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 21)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 22)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 23)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 24)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 25)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 26)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 27)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 28)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 29)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 30)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 31)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 32)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 33)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 34)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 35)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 36)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 37)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 38)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 39)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 40)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 41)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 42)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 43)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 44)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 45)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 46)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 47)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 48)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 49)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 50)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 51)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 52)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 53)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 54)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 55)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 56)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 57)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 58)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 59)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 60)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 61)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 62)
  CHECK_STACK_ALIGNMENT(union_array_type, 64, 63)
}
