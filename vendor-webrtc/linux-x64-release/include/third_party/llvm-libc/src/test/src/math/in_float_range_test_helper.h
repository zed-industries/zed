//
// Created by kirill on 8/30/22.
//

#ifndef LLVM_LIBC_TEST_SRC_MATH_IN_FLOAT_RANGE_TEST_HELPER_H
#define LLVM_LIBC_TEST_SRC_MATH_IN_FLOAT_RANGE_TEST_HELPER_H

#include <stdint.h>

#define CHECK_DATA(start, stop, mfp_op, f, f_check, count, prec)               \
  {                                                                            \
    uint64_t ustart = FPBits(start).uintval();                                 \
    uint64_t ustop = FPBits(stop).uintval();                                   \
    for (uint64_t i = 0;; ++i) {                                               \
      uint64_t v = ustart + (ustop - ustart) * i / count;                      \
      if (v > ustop)                                                           \
        break;                                                                 \
      float x = FPBits(uint32_t(v)).get_val();                                 \
      if ((f_check)(x)) {                                                      \
        EXPECT_MPFR_MATCH_ALL_ROUNDING(mfp_op, x, static_cast<float>((f)(x)),  \
                                       (prec));                                \
      }                                                                        \
    }                                                                          \
  }

#endif // LLVM_LIBC_TEST_SRC_MATH_IN_FLOAT_RANGE_TEST_HELPER_H
