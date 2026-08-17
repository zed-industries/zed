// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com)
// Copyright 2002 Sun Microsystems, Inc. ALL RIGHTS RESERVED.
//
// The binary polynomial arithmetic software is originally written by
// Sheueling Chang Shantz and Douglas Stebila of Sun Microsystems
// Laboratories.
//
// SPDX-License-Identifier: Apache-2.0

#include <assert.h>
#include <errno.h>
#include <limits.h>
#include <stdio.h>
#include <string.h>

#include <algorithm>
#include <limits>
#include <utility>

#include <gtest/gtest.h>

#include <openssl/bio.h>
#include <openssl/bn.h>
#include <openssl/bytestring.h>
#include <openssl/crypto.h>
#include <openssl/err.h>
#include <openssl/mem.h>
#include <openssl/rand.h>

#include "./internal.h"
#include "./rsaz_exp.h"
#include "../../internal.h"
#include "../../test/abi_test.h"
#include "../../test/file_test.h"
#include "../../test/test_util.h"
#include "../../test/wycheproof_util.h"


static int HexToBIGNUMWithReturn(bssl::UniquePtr<BIGNUM> *out, const char *in) {
  BIGNUM *raw = NULL;
  int ret = BN_hex2bn(&raw, in);
  out->reset(raw);
  return ret;
}

// A BIGNUMFileTest wraps a FileTest to give |BIGNUM| values and also allows
// injecting oversized |BIGNUM|s.
class BIGNUMFileTest {
 public:
  BIGNUMFileTest(FileTest *t, unsigned large_mask)
      : t_(t), large_mask_(large_mask), num_bignums_(0) {}

  unsigned num_bignums() const { return num_bignums_; }

  bssl::UniquePtr<BIGNUM> GetBIGNUM(const char *attribute) {
    return GetBIGNUMImpl(attribute, true /* resize */);
  }

  bool GetInt(int *out, const char *attribute) {
    bssl::UniquePtr<BIGNUM> ret =
        GetBIGNUMImpl(attribute, false /* don't resize */);
    if (!ret) {
      return false;
    }

    BN_ULONG word = BN_get_word(ret.get());
    if (word > INT_MAX) {
      return false;
    }

    *out = static_cast<int>(word);
    return true;
  }

 private:
  bssl::UniquePtr<BIGNUM> GetBIGNUMImpl(const char *attribute, bool resize) {
    std::string hex;
    if (!t_->GetAttribute(&hex, attribute)) {
      return nullptr;
    }

    bssl::UniquePtr<BIGNUM> ret;
    if (HexToBIGNUMWithReturn(&ret, hex.c_str()) != static_cast<int>(hex.size())) {
      t_->PrintLine("Could not decode '%s'.", hex.c_str());
      return nullptr;
    }
    if (resize) {
      // Test with an oversized |BIGNUM| if necessary.
      if ((large_mask_ & (1 << num_bignums_)) &&
          !bn_resize_words(ret.get(), ret->width * 2 + 1)) {
        return nullptr;
      }
      num_bignums_++;
    }
    return ret;
  }

  FileTest *t_;
  unsigned large_mask_;
  unsigned num_bignums_;
};

static testing::AssertionResult AssertBIGNUMSEqual(
    const char *operation_expr, const char *expected_expr,
    const char *actual_expr, const char *operation, const BIGNUM *expected,
    const BIGNUM *actual) {
  if (BN_cmp(expected, actual) == 0) {
    return testing::AssertionSuccess();
  }

  bssl::UniquePtr<char> expected_str(BN_bn2hex(expected));
  bssl::UniquePtr<char> actual_str(BN_bn2hex(actual));
  if (!expected_str || !actual_str) {
    return testing::AssertionFailure() << "Error converting BIGNUMs to hex";
  }

  return testing::AssertionFailure()
         << "Wrong value for " << operation
         << "\nActual:   " << actual_str.get() << " (" << actual_expr
         << ")\nExpected: " << expected_str.get() << " (" << expected_expr
         << ")";
}

#define EXPECT_BIGNUMS_EQUAL(op, a, b) \
  EXPECT_PRED_FORMAT3(AssertBIGNUMSEqual, op, a, b)

static void TestSum(BIGNUMFileTest *t, BN_CTX *ctx) {
  bssl::UniquePtr<BIGNUM> a = t->GetBIGNUM("A");
  bssl::UniquePtr<BIGNUM> b = t->GetBIGNUM("B");
  bssl::UniquePtr<BIGNUM> sum = t->GetBIGNUM("Sum");
  ASSERT_TRUE(a);
  ASSERT_TRUE(b);
  ASSERT_TRUE(sum);

  bssl::UniquePtr<BIGNUM> ret(BN_new());
  ASSERT_TRUE(ret);
  ASSERT_TRUE(BN_add(ret.get(), a.get(), b.get()));
  EXPECT_BIGNUMS_EQUAL("A + B", sum.get(), ret.get());

  ASSERT_TRUE(BN_sub(ret.get(), sum.get(), a.get()));
  EXPECT_BIGNUMS_EQUAL("Sum - A", b.get(), ret.get());

  ASSERT_TRUE(BN_sub(ret.get(), sum.get(), b.get()));
  EXPECT_BIGNUMS_EQUAL("Sum - B", a.get(), ret.get());

  // Test that the functions work when |r| and |a| point to the same |BIGNUM|,
  // or when |r| and |b| point to the same |BIGNUM|. TODO: Test the case where
  // all of |r|, |a|, and |b| point to the same |BIGNUM|.
  ASSERT_TRUE(BN_copy(ret.get(), a.get()));
  ASSERT_TRUE(BN_add(ret.get(), ret.get(), b.get()));
  EXPECT_BIGNUMS_EQUAL("A + B (r is a)", sum.get(), ret.get());

  ASSERT_TRUE(BN_copy(ret.get(), b.get()));
  ASSERT_TRUE(BN_add(ret.get(), a.get(), ret.get()));
  EXPECT_BIGNUMS_EQUAL("A + B (r is b)", sum.get(), ret.get());

  ASSERT_TRUE(BN_copy(ret.get(), sum.get()));
  ASSERT_TRUE(BN_sub(ret.get(), ret.get(), a.get()));
  EXPECT_BIGNUMS_EQUAL("Sum - A (r is a)", b.get(), ret.get());

  ASSERT_TRUE(BN_copy(ret.get(), a.get()));
  ASSERT_TRUE(BN_sub(ret.get(), sum.get(), ret.get()));
  EXPECT_BIGNUMS_EQUAL("Sum - A (r is b)", b.get(), ret.get());

  ASSERT_TRUE(BN_copy(ret.get(), sum.get()));
  ASSERT_TRUE(BN_sub(ret.get(), ret.get(), b.get()));
  EXPECT_BIGNUMS_EQUAL("Sum - B (r is a)", a.get(), ret.get());

  ASSERT_TRUE(BN_copy(ret.get(), b.get()));
  ASSERT_TRUE(BN_sub(ret.get(), sum.get(), ret.get()));
  EXPECT_BIGNUMS_EQUAL("Sum - B (r is b)", a.get(), ret.get());

  // Test |BN_uadd| and |BN_usub| with the prerequisites they are documented as
  // having. Note that these functions are frequently used when the
  // prerequisites don't hold. In those cases, they are supposed to work as if
  // the prerequisite hold, but we don't test that yet. TODO: test that.
  if (!BN_is_negative(a.get()) && !BN_is_negative(b.get())) {
    ASSERT_TRUE(BN_uadd(ret.get(), a.get(), b.get()));
    EXPECT_BIGNUMS_EQUAL("A +u B", sum.get(), ret.get());

    ASSERT_TRUE(BN_usub(ret.get(), sum.get(), a.get()));
    EXPECT_BIGNUMS_EQUAL("Sum -u A", b.get(), ret.get());

    ASSERT_TRUE(BN_usub(ret.get(), sum.get(), b.get()));
    EXPECT_BIGNUMS_EQUAL("Sum -u B", a.get(), ret.get());

    // Test that the functions work when |r| and |a| point to the same |BIGNUM|,
    // or when |r| and |b| point to the same |BIGNUM|. TODO: Test the case where
    // all of |r|, |a|, and |b| point to the same |BIGNUM|.
    ASSERT_TRUE(BN_copy(ret.get(), a.get()));
    ASSERT_TRUE(BN_uadd(ret.get(), ret.get(), b.get()));
    EXPECT_BIGNUMS_EQUAL("A +u B (r is a)", sum.get(), ret.get());

    ASSERT_TRUE(BN_copy(ret.get(), b.get()));
    ASSERT_TRUE(BN_uadd(ret.get(), a.get(), ret.get()));
    EXPECT_BIGNUMS_EQUAL("A +u B (r is b)", sum.get(), ret.get());

    ASSERT_TRUE(BN_copy(ret.get(), sum.get()));
    ASSERT_TRUE(BN_usub(ret.get(), ret.get(), a.get()));
    EXPECT_BIGNUMS_EQUAL("Sum -u A (r is a)", b.get(), ret.get());

    ASSERT_TRUE(BN_copy(ret.get(), a.get()));
    ASSERT_TRUE(BN_usub(ret.get(), sum.get(), ret.get()));
    EXPECT_BIGNUMS_EQUAL("Sum -u A (r is b)", b.get(), ret.get());

    ASSERT_TRUE(BN_copy(ret.get(), sum.get()));
    ASSERT_TRUE(BN_usub(ret.get(), ret.get(), b.get()));
    EXPECT_BIGNUMS_EQUAL("Sum -u B (r is a)", a.get(), ret.get());

    ASSERT_TRUE(BN_copy(ret.get(), b.get()));
    ASSERT_TRUE(BN_usub(ret.get(), sum.get(), ret.get()));
    EXPECT_BIGNUMS_EQUAL("Sum -u B (r is b)", a.get(), ret.get());

    ASSERT_TRUE(bn_abs_sub_consttime(ret.get(), sum.get(), a.get(), ctx));
    EXPECT_BIGNUMS_EQUAL("|Sum - A|", b.get(), ret.get());
    ASSERT_TRUE(bn_abs_sub_consttime(ret.get(), a.get(), sum.get(), ctx));
    EXPECT_BIGNUMS_EQUAL("|A - Sum|", b.get(), ret.get());

    ASSERT_TRUE(bn_abs_sub_consttime(ret.get(), sum.get(), b.get(), ctx));
    EXPECT_BIGNUMS_EQUAL("|Sum - B|", a.get(), ret.get());
    ASSERT_TRUE(bn_abs_sub_consttime(ret.get(), b.get(), sum.get(), ctx));
    EXPECT_BIGNUMS_EQUAL("|B - Sum|", a.get(), ret.get());
  }

  // Test with |BN_add_word| and |BN_sub_word| if |b| is small enough.
  BN_ULONG b_word = BN_get_word(b.get());
  if (!BN_is_negative(b.get()) && b_word != (BN_ULONG)-1) {
    ASSERT_TRUE(BN_copy(ret.get(), a.get()));
    ASSERT_TRUE(BN_add_word(ret.get(), b_word));
    EXPECT_BIGNUMS_EQUAL("A + B (word)", sum.get(), ret.get());

    ASSERT_TRUE(BN_copy(ret.get(), sum.get()));
    ASSERT_TRUE(BN_sub_word(ret.get(), b_word));
    EXPECT_BIGNUMS_EQUAL("Sum - B (word)", a.get(), ret.get());
  }
}

static void TestLShift1(BIGNUMFileTest *t, BN_CTX *ctx) {
  bssl::UniquePtr<BIGNUM> a = t->GetBIGNUM("A");
  bssl::UniquePtr<BIGNUM> lshift1 = t->GetBIGNUM("LShift1");
  bssl::UniquePtr<BIGNUM> zero(BN_new());
  ASSERT_TRUE(a);
  ASSERT_TRUE(lshift1);
  ASSERT_TRUE(zero);

  BN_zero(zero.get());

  bssl::UniquePtr<BIGNUM> ret(BN_new()), two(BN_new()), remainder(BN_new());
  ASSERT_TRUE(ret);
  ASSERT_TRUE(two);
  ASSERT_TRUE(remainder);

  ASSERT_TRUE(BN_set_word(two.get(), 2));
  ASSERT_TRUE(BN_add(ret.get(), a.get(), a.get()));
  EXPECT_BIGNUMS_EQUAL("A + A", lshift1.get(), ret.get());

  ASSERT_TRUE(BN_mul(ret.get(), a.get(), two.get(), ctx));
  EXPECT_BIGNUMS_EQUAL("A * 2", lshift1.get(), ret.get());

  ASSERT_TRUE(
      BN_div(ret.get(), remainder.get(), lshift1.get(), two.get(), ctx));
  EXPECT_BIGNUMS_EQUAL("LShift1 / 2", a.get(), ret.get());
  EXPECT_BIGNUMS_EQUAL("LShift1 % 2", zero.get(), remainder.get());

  ASSERT_TRUE(BN_lshift1(ret.get(), a.get()));
  EXPECT_BIGNUMS_EQUAL("A << 1", lshift1.get(), ret.get());

  ASSERT_TRUE(BN_lshift(ret.get(), a.get(), 1));
  EXPECT_BIGNUMS_EQUAL("A << 1 (variable shift)", lshift1.get(), ret.get());

  ASSERT_TRUE(BN_rshift1(ret.get(), lshift1.get()));
  EXPECT_BIGNUMS_EQUAL("LShift >> 1", a.get(), ret.get());

  ASSERT_TRUE(BN_rshift(ret.get(), lshift1.get(), 1));
  EXPECT_BIGNUMS_EQUAL("LShift >> 1 (variable shift)", a.get(), ret.get());

  ASSERT_TRUE(bn_rshift_secret_shift(ret.get(), lshift1.get(), 1, ctx));
  EXPECT_BIGNUMS_EQUAL("LShift >> 1 (secret shift)", a.get(), ret.get());

  // Set the LSB to 1 and test rshift1 again.
  ASSERT_TRUE(BN_set_bit(lshift1.get(), 0));
  ASSERT_TRUE(
      BN_div(ret.get(), nullptr /* rem */, lshift1.get(), two.get(), ctx));
  EXPECT_BIGNUMS_EQUAL("(LShift1 | 1) / 2", a.get(), ret.get());

  ASSERT_TRUE(BN_rshift1(ret.get(), lshift1.get()));
  EXPECT_BIGNUMS_EQUAL("(LShift | 1) >> 1", a.get(), ret.get());

  ASSERT_TRUE(BN_rshift(ret.get(), lshift1.get(), 1));
  EXPECT_BIGNUMS_EQUAL("(LShift | 1) >> 1 (variable shift)", a.get(),
                       ret.get());

  ASSERT_TRUE(bn_rshift_secret_shift(ret.get(), lshift1.get(), 1, ctx));
  EXPECT_BIGNUMS_EQUAL("(LShift | 1) >> 1 (secret shift)", a.get(), ret.get());
}

static void TestLShift(BIGNUMFileTest *t, BN_CTX *ctx) {
  bssl::UniquePtr<BIGNUM> a = t->GetBIGNUM("A");
  bssl::UniquePtr<BIGNUM> lshift = t->GetBIGNUM("LShift");
  ASSERT_TRUE(a);
  ASSERT_TRUE(lshift);
  int n = 0;
  ASSERT_TRUE(t->GetInt(&n, "N"));

  bssl::UniquePtr<BIGNUM> ret(BN_new());
  ASSERT_TRUE(ret);
  ASSERT_TRUE(BN_lshift(ret.get(), a.get(), n));
  EXPECT_BIGNUMS_EQUAL("A << N", lshift.get(), ret.get());

  ASSERT_TRUE(BN_copy(ret.get(), a.get()));
  ASSERT_TRUE(BN_lshift(ret.get(), ret.get(), n));
  EXPECT_BIGNUMS_EQUAL("A << N (in-place)", lshift.get(), ret.get());

  ASSERT_TRUE(BN_rshift(ret.get(), lshift.get(), n));
  EXPECT_BIGNUMS_EQUAL("A >> N", a.get(), ret.get());

  ASSERT_TRUE(bn_rshift_secret_shift(ret.get(), lshift.get(), n, ctx));
  EXPECT_BIGNUMS_EQUAL("A >> N (secret shift)", a.get(), ret.get());
}

static void TestRShift(BIGNUMFileTest *t, BN_CTX *ctx) {
  bssl::UniquePtr<BIGNUM> a = t->GetBIGNUM("A");
  bssl::UniquePtr<BIGNUM> rshift = t->GetBIGNUM("RShift");
  ASSERT_TRUE(a);
  ASSERT_TRUE(rshift);
  int n = 0;
  ASSERT_TRUE(t->GetInt(&n, "N"));

  bssl::UniquePtr<BIGNUM> ret(BN_new());
  ASSERT_TRUE(ret);
  ASSERT_TRUE(BN_rshift(ret.get(), a.get(), n));
  EXPECT_BIGNUMS_EQUAL("A >> N", rshift.get(), ret.get());

  ASSERT_TRUE(BN_copy(ret.get(), a.get()));
  ASSERT_TRUE(BN_rshift(ret.get(), ret.get(), n));
  EXPECT_BIGNUMS_EQUAL("A >> N (in-place)", rshift.get(), ret.get());

  ASSERT_TRUE(bn_rshift_secret_shift(ret.get(), a.get(), n, ctx));
  EXPECT_BIGNUMS_EQUAL("A >> N (secret shift)", rshift.get(), ret.get());

  ASSERT_TRUE(BN_copy(ret.get(), a.get()));
  ASSERT_TRUE(bn_rshift_secret_shift(ret.get(), ret.get(), n, ctx));
  EXPECT_BIGNUMS_EQUAL("A >> N (in-place secret shift)", rshift.get(),
                       ret.get());
}

static void TestSquare(BIGNUMFileTest *t, BN_CTX *ctx) {
  bssl::UniquePtr<BIGNUM> a = t->GetBIGNUM("A");
  bssl::UniquePtr<BIGNUM> square = t->GetBIGNUM("Square");
  bssl::UniquePtr<BIGNUM> zero(BN_new());
  ASSERT_TRUE(a);
  ASSERT_TRUE(square);
  ASSERT_TRUE(zero);

  BN_zero(zero.get());

  bssl::UniquePtr<BIGNUM> ret(BN_new()), remainder(BN_new());
  ASSERT_TRUE(ret);
  ASSERT_TRUE(remainder);
  ASSERT_TRUE(BN_sqr(ret.get(), a.get(), ctx));
  EXPECT_BIGNUMS_EQUAL("A^2", square.get(), ret.get());

  ASSERT_TRUE(BN_mul(ret.get(), a.get(), a.get(), ctx));
  EXPECT_BIGNUMS_EQUAL("A * A", square.get(), ret.get());

  if (!BN_is_zero(a.get())) {
    ASSERT_TRUE(BN_div(ret.get(), remainder.get(), square.get(), a.get(), ctx));
    EXPECT_BIGNUMS_EQUAL("Square / A", a.get(), ret.get());
    EXPECT_BIGNUMS_EQUAL("Square % A", zero.get(), remainder.get());
  }

  BN_set_negative(a.get(), 0);
  ASSERT_TRUE(BN_sqrt(ret.get(), square.get(), ctx));
  EXPECT_BIGNUMS_EQUAL("sqrt(Square)", a.get(), ret.get());

  // BN_sqrt should fail on non-squares and negative numbers.
  if (!BN_is_zero(square.get())) {
    bssl::UniquePtr<BIGNUM> tmp(BN_new());
    ASSERT_TRUE(tmp);
    ASSERT_TRUE(BN_copy(tmp.get(), square.get()));
    BN_set_negative(tmp.get(), 1);

    EXPECT_FALSE(BN_sqrt(ret.get(), tmp.get(), ctx))
        << "BN_sqrt succeeded on a negative number";
    ERR_clear_error();

    BN_set_negative(tmp.get(), 0);
    ASSERT_TRUE(BN_add(tmp.get(), tmp.get(), BN_value_one()));
    EXPECT_FALSE(BN_sqrt(ret.get(), tmp.get(), ctx))
        << "BN_sqrt succeeded on a non-square";
    ERR_clear_error();
  }

#if !defined(BORINGSSL_SHARED_LIBRARY)
  int a_width = bn_minimal_width(a.get());
  if (a_width <= BN_SMALL_MAX_WORDS) {
    for (size_t num_a = a_width; num_a <= BN_SMALL_MAX_WORDS; num_a++) {
      SCOPED_TRACE(num_a);
      size_t num_r = 2 * num_a;
      // Use newly-allocated buffers so ASan will catch out-of-bounds writes.
      std::unique_ptr<BN_ULONG[]> a_words(new BN_ULONG[num_a]),
          r_words(new BN_ULONG[num_r]);
      ASSERT_TRUE(bn_copy_words(a_words.get(), num_a, a.get()));

      bn_mul_small(r_words.get(), num_r, a_words.get(), num_a, a_words.get(),
                   num_a);
      ASSERT_TRUE(bn_set_words(ret.get(), r_words.get(), num_r));
      EXPECT_BIGNUMS_EQUAL("A * A (words)", square.get(), ret.get());

      OPENSSL_memset(r_words.get(), 'A', num_r * sizeof(BN_ULONG));
      bn_sqr_small(r_words.get(), num_r, a_words.get(), num_a);

      ASSERT_TRUE(bn_set_words(ret.get(), r_words.get(), num_r));
      EXPECT_BIGNUMS_EQUAL("A^2 (words)", square.get(), ret.get());
    }
  }
#endif
}

static void TestProduct(BIGNUMFileTest *t, BN_CTX *ctx) {
  bssl::UniquePtr<BIGNUM> a = t->GetBIGNUM("A");
  bssl::UniquePtr<BIGNUM> b = t->GetBIGNUM("B");
  bssl::UniquePtr<BIGNUM> product = t->GetBIGNUM("Product");
  bssl::UniquePtr<BIGNUM> zero(BN_new());
  ASSERT_TRUE(a);
  ASSERT_TRUE(b);
  ASSERT_TRUE(product);
  ASSERT_TRUE(zero);

  BN_zero(zero.get());

  bssl::UniquePtr<BIGNUM> ret(BN_new()), remainder(BN_new());
  ASSERT_TRUE(ret);
  ASSERT_TRUE(remainder);
  ASSERT_TRUE(BN_mul(ret.get(), a.get(), b.get(), ctx));
  EXPECT_BIGNUMS_EQUAL("A * B", product.get(), ret.get());

  if (!BN_is_zero(a.get())) {
    ASSERT_TRUE(
        BN_div(ret.get(), remainder.get(), product.get(), a.get(), ctx));
    EXPECT_BIGNUMS_EQUAL("Product / A", b.get(), ret.get());
    EXPECT_BIGNUMS_EQUAL("Product % A", zero.get(), remainder.get());
  }

  if (!BN_is_zero(b.get())) {
    ASSERT_TRUE(
        BN_div(ret.get(), remainder.get(), product.get(), b.get(), ctx));
    EXPECT_BIGNUMS_EQUAL("Product / B", a.get(), ret.get());
    EXPECT_BIGNUMS_EQUAL("Product % B", zero.get(), remainder.get());
  }

#if !defined(BORINGSSL_SHARED_LIBRARY)
  BN_set_negative(a.get(), 0);
  BN_set_negative(b.get(), 0);
  BN_set_negative(product.get(), 0);

  int a_width = bn_minimal_width(a.get());
  int b_width = bn_minimal_width(b.get());
  if (a_width <= BN_SMALL_MAX_WORDS && b_width <= BN_SMALL_MAX_WORDS) {
    for (size_t num_a = static_cast<size_t>(a_width);
         num_a <= BN_SMALL_MAX_WORDS; num_a++) {
      SCOPED_TRACE(num_a);
      for (size_t num_b = static_cast<size_t>(b_width);
           num_b <= BN_SMALL_MAX_WORDS; num_b++) {
        SCOPED_TRACE(num_b);
        size_t num_r = num_a + num_b;
        // Use newly-allocated buffers so ASan will catch out-of-bounds writes.
        std::unique_ptr<BN_ULONG[]> a_words(new BN_ULONG[num_a]),
            b_words(new BN_ULONG[num_b]), r_words(new BN_ULONG[num_r]);
        ASSERT_TRUE(bn_copy_words(a_words.get(), num_a, a.get()));
        ASSERT_TRUE(bn_copy_words(b_words.get(), num_b, b.get()));

        bn_mul_small(r_words.get(), num_r, a_words.get(), num_a, b_words.get(),
                     num_b);
        ASSERT_TRUE(bn_set_words(ret.get(), r_words.get(), num_r));
        EXPECT_BIGNUMS_EQUAL("A * B (words)", product.get(), ret.get());
      }
    }
  }
#endif
}

static void TestQuotient(BIGNUMFileTest *t, BN_CTX *ctx) {
  bssl::UniquePtr<BIGNUM> a = t->GetBIGNUM("A");
  bssl::UniquePtr<BIGNUM> b = t->GetBIGNUM("B");
  bssl::UniquePtr<BIGNUM> quotient = t->GetBIGNUM("Quotient");
  bssl::UniquePtr<BIGNUM> remainder = t->GetBIGNUM("Remainder");
  ASSERT_TRUE(a);
  ASSERT_TRUE(b);
  ASSERT_TRUE(quotient);
  ASSERT_TRUE(remainder);

  bssl::UniquePtr<BIGNUM> ret(BN_new()), ret2(BN_new());
  ASSERT_TRUE(ret);
  ASSERT_TRUE(ret2);
  ASSERT_TRUE(BN_div(ret.get(), ret2.get(), a.get(), b.get(), ctx));
  EXPECT_BIGNUMS_EQUAL("A / B", quotient.get(), ret.get());
  EXPECT_BIGNUMS_EQUAL("A % B", remainder.get(), ret2.get());

  ASSERT_TRUE(BN_copy(ret.get(), a.get()));
  ASSERT_TRUE(BN_copy(ret2.get(), b.get()));
  ASSERT_TRUE(BN_div(ret.get(), ret2.get(), ret.get(), ret2.get(), ctx));
  EXPECT_BIGNUMS_EQUAL("A / B (in-place)", quotient.get(), ret.get());
  EXPECT_BIGNUMS_EQUAL("A % B (in-place)", remainder.get(), ret2.get());

  ASSERT_TRUE(BN_copy(ret2.get(), a.get()));
  ASSERT_TRUE(BN_copy(ret.get(), b.get()));
  ASSERT_TRUE(BN_div(ret.get(), ret2.get(), ret2.get(), ret.get(), ctx));
  EXPECT_BIGNUMS_EQUAL("A / B (in-place, swapped)", quotient.get(), ret.get());
  EXPECT_BIGNUMS_EQUAL("A % B (in-place, swapped)", remainder.get(),
                       ret2.get());

  ASSERT_TRUE(BN_mul(ret.get(), quotient.get(), b.get(), ctx));
  ASSERT_TRUE(BN_add(ret.get(), ret.get(), remainder.get()));
  EXPECT_BIGNUMS_EQUAL("Quotient * B + Remainder", a.get(), ret.get());

  // The remaining division variants only handle a positive quotient.
  if (BN_is_negative(b.get())) {
    BN_set_negative(b.get(), 0);
    BN_set_negative(quotient.get(), !BN_is_negative(quotient.get()));
  }

  bssl::UniquePtr<BIGNUM> nnmod(BN_new());
  ASSERT_TRUE(nnmod);
  ASSERT_TRUE(BN_copy(nnmod.get(), remainder.get()));
  if (BN_is_negative(nnmod.get())) {
    ASSERT_TRUE(BN_add(nnmod.get(), nnmod.get(), b.get()));
  }
  ASSERT_TRUE(BN_nnmod(ret.get(), a.get(), b.get(), ctx));
  EXPECT_BIGNUMS_EQUAL("A % B (non-negative)", nnmod.get(), ret.get());

  // The remaining division variants only handle a positive numerator.
  if (BN_is_negative(a.get())) {
    BN_set_negative(a.get(), 0);
    BN_set_negative(quotient.get(), 0);
    BN_set_negative(remainder.get(), 0);
  }

  // Test with |BN_mod_word| and |BN_div_word| if the divisor is small enough.
  BN_ULONG b_word = BN_get_word(b.get());
  if (b_word != (BN_ULONG)-1) {
    BN_ULONG remainder_word = BN_get_word(remainder.get());
    ASSERT_NE(remainder_word, (BN_ULONG)-1);
    ASSERT_TRUE(BN_copy(ret.get(), a.get()));
    BN_ULONG ret_word = BN_div_word(ret.get(), b_word);
    EXPECT_EQ(remainder_word, ret_word);
    EXPECT_BIGNUMS_EQUAL("A / B (word)", quotient.get(), ret.get());

    ret_word = BN_mod_word(a.get(), b_word);
    EXPECT_EQ(remainder_word, ret_word);

    if (b_word <= 0xffff) {
      EXPECT_EQ(remainder_word, bn_mod_u16_consttime(a.get(), b_word));
    }
  }

  ASSERT_TRUE(bn_div_consttime(ret.get(), ret2.get(), a.get(), b.get(),
                               /*divisor_min_bits=*/0, ctx));
  EXPECT_BIGNUMS_EQUAL("A / B (constant-time)", quotient.get(), ret.get());
  EXPECT_BIGNUMS_EQUAL("A % B (constant-time)", remainder.get(), ret2.get());

  ASSERT_TRUE(bn_div_consttime(ret.get(), ret2.get(), a.get(), b.get(),
                               /*divisor_min_bits=*/BN_num_bits(b.get()), ctx));
  EXPECT_BIGNUMS_EQUAL("A / B (constant-time, public width)", quotient.get(),
                       ret.get());
  EXPECT_BIGNUMS_EQUAL("A % B (constant-time, public width)", remainder.get(),
                       ret2.get());
}

static void TestModMul(BIGNUMFileTest *t, BN_CTX *ctx) {
  bssl::UniquePtr<BIGNUM> a = t->GetBIGNUM("A");
  bssl::UniquePtr<BIGNUM> b = t->GetBIGNUM("B");
  bssl::UniquePtr<BIGNUM> m = t->GetBIGNUM("M");
  bssl::UniquePtr<BIGNUM> mod_mul = t->GetBIGNUM("ModMul");
  ASSERT_TRUE(a);
  ASSERT_TRUE(b);
  ASSERT_TRUE(m);
  ASSERT_TRUE(mod_mul);

  bssl::UniquePtr<BIGNUM> ret(BN_new());
  ASSERT_TRUE(ret);
  ASSERT_TRUE(BN_mod_mul(ret.get(), a.get(), b.get(), m.get(), ctx));
  EXPECT_BIGNUMS_EQUAL("A * B (mod M)", mod_mul.get(), ret.get());

  if (BN_is_odd(m.get())) {
    // Reduce |a| and |b| and test the Montgomery version.
    bssl::UniquePtr<BN_MONT_CTX> mont(
        BN_MONT_CTX_new_for_modulus(m.get(), ctx));
    ASSERT_TRUE(mont);

    // Sanity-check that the constant-time version computes the same n0 and RR.
    bssl::UniquePtr<BN_MONT_CTX> mont2(
        BN_MONT_CTX_new_consttime(m.get(), ctx));
    ASSERT_TRUE(mont2);
    EXPECT_BIGNUMS_EQUAL("RR (mod M) (constant-time)", &mont->RR, &mont2->RR);
    EXPECT_EQ(mont->n0[0], mont2->n0[0]);
    EXPECT_EQ(mont->n0[1], mont2->n0[1]);

    bssl::UniquePtr<BIGNUM> a_tmp(BN_new()), b_tmp(BN_new());
    ASSERT_TRUE(a_tmp);
    ASSERT_TRUE(b_tmp);
    ASSERT_TRUE(BN_nnmod(a.get(), a.get(), m.get(), ctx));
    ASSERT_TRUE(BN_nnmod(b.get(), b.get(), m.get(), ctx));
    ASSERT_TRUE(BN_to_montgomery(a_tmp.get(), a.get(), mont.get(), ctx));
    ASSERT_TRUE(BN_to_montgomery(b_tmp.get(), b.get(), mont.get(), ctx));
    ASSERT_TRUE(BN_mod_mul_montgomery(ret.get(), a_tmp.get(), b_tmp.get(),
                                      mont.get(), ctx));
    ASSERT_TRUE(BN_from_montgomery(ret.get(), ret.get(), mont.get(), ctx));
    EXPECT_BIGNUMS_EQUAL("A * B (mod M) (Montgomery)", mod_mul.get(),
                         ret.get());

#if !defined(BORINGSSL_SHARED_LIBRARY)
    size_t m_width = static_cast<size_t>(bn_minimal_width(m.get()));
    if (m_width <= BN_SMALL_MAX_WORDS) {
      std::unique_ptr<BN_ULONG[]> a_words(new BN_ULONG[m_width]),
          b_words(new BN_ULONG[m_width]), r_words(new BN_ULONG[m_width]);
      ASSERT_TRUE(bn_copy_words(a_words.get(), m_width, a.get()));
      ASSERT_TRUE(bn_copy_words(b_words.get(), m_width, b.get()));
      bn_to_montgomery_small(a_words.get(), a_words.get(), m_width, mont.get());
      bn_to_montgomery_small(b_words.get(), b_words.get(), m_width, mont.get());
      bn_mod_mul_montgomery_small(r_words.get(), a_words.get(), b_words.get(),
                                  m_width, mont.get());
      // Use the second half of |tmp| so ASan will catch out-of-bounds writes.
      bn_from_montgomery_small(r_words.get(), m_width, r_words.get(), m_width,
                               mont.get());
      ASSERT_TRUE(bn_set_words(ret.get(), r_words.get(), m_width));
      EXPECT_BIGNUMS_EQUAL("A * B (mod M) (Montgomery, words)", mod_mul.get(),
                           ret.get());

      // |bn_from_montgomery_small| must additionally work on double-width
      // inputs. Test this by running |bn_from_montgomery_small| on the result
      // of a product. Note |a_words| * |b_words| has an extra factor of R^2, so
      // we must reduce twice.
      std::unique_ptr<BN_ULONG[]> prod_words(new BN_ULONG[m_width * 2]);
      bn_mul_small(prod_words.get(), m_width * 2, a_words.get(), m_width,
                   b_words.get(), m_width);
      bn_from_montgomery_small(r_words.get(), m_width, prod_words.get(),
                               m_width * 2, mont.get());
      bn_from_montgomery_small(r_words.get(), m_width, r_words.get(), m_width,
                               mont.get());
      ASSERT_TRUE(bn_set_words(ret.get(), r_words.get(), m_width));
      EXPECT_BIGNUMS_EQUAL("A * B (mod M) (Montgomery, words)",
                           mod_mul.get(), ret.get());
    }
#endif
  }
}

static void TestModSquare(BIGNUMFileTest *t, BN_CTX *ctx) {
  bssl::UniquePtr<BIGNUM> a = t->GetBIGNUM("A");
  bssl::UniquePtr<BIGNUM> m = t->GetBIGNUM("M");
  bssl::UniquePtr<BIGNUM> mod_square = t->GetBIGNUM("ModSquare");
  ASSERT_TRUE(a);
  ASSERT_TRUE(m);
  ASSERT_TRUE(mod_square);

  bssl::UniquePtr<BIGNUM> a_copy(BN_new());
  bssl::UniquePtr<BIGNUM> ret(BN_new());
  ASSERT_TRUE(ret);
  ASSERT_TRUE(a_copy);
  ASSERT_TRUE(BN_mod_mul(ret.get(), a.get(), a.get(), m.get(), ctx));
  EXPECT_BIGNUMS_EQUAL("A * A (mod M)", mod_square.get(), ret.get());

  // Repeat the operation with |a_copy|.
  ASSERT_TRUE(BN_copy(a_copy.get(), a.get()));
  ASSERT_TRUE(BN_mod_mul(ret.get(), a.get(), a_copy.get(), m.get(), ctx));
  EXPECT_BIGNUMS_EQUAL("A * A_copy (mod M)", mod_square.get(), ret.get());

  if (BN_is_odd(m.get())) {
    // Reduce |a| and test the Montgomery version.
    bssl::UniquePtr<BN_MONT_CTX> mont(
        BN_MONT_CTX_new_for_modulus(m.get(), ctx));
    bssl::UniquePtr<BIGNUM> a_tmp(BN_new());
    ASSERT_TRUE(mont);
    ASSERT_TRUE(a_tmp);
    ASSERT_TRUE(BN_nnmod(a.get(), a.get(), m.get(), ctx));
    ASSERT_TRUE(BN_to_montgomery(a_tmp.get(), a.get(), mont.get(), ctx));
    ASSERT_TRUE(BN_mod_mul_montgomery(ret.get(), a_tmp.get(), a_tmp.get(),
                                      mont.get(), ctx));
    ASSERT_TRUE(BN_from_montgomery(ret.get(), ret.get(), mont.get(), ctx));
    EXPECT_BIGNUMS_EQUAL("A * A (mod M) (Montgomery)", mod_square.get(),
                         ret.get());

    // Repeat the operation with |a_copy|.
    ASSERT_TRUE(BN_copy(a_copy.get(), a_tmp.get()));
    ASSERT_TRUE(BN_mod_mul_montgomery(ret.get(), a_tmp.get(), a_copy.get(),
                                      mont.get(), ctx));
    ASSERT_TRUE(BN_from_montgomery(ret.get(), ret.get(), mont.get(), ctx));
    EXPECT_BIGNUMS_EQUAL("A * A_copy (mod M) (Montgomery)", mod_square.get(),
                         ret.get());

#if !defined(BORINGSSL_SHARED_LIBRARY)
    size_t m_width = static_cast<size_t>(bn_minimal_width(m.get()));
    if (m_width <= BN_SMALL_MAX_WORDS) {
      std::unique_ptr<BN_ULONG[]> a_words(new BN_ULONG[m_width]),
          a_copy_words(new BN_ULONG[m_width]), r_words(new BN_ULONG[m_width]);
      ASSERT_TRUE(bn_copy_words(a_words.get(), m_width, a.get()));
      bn_to_montgomery_small(a_words.get(), a_words.get(), m_width, mont.get());
      bn_mod_mul_montgomery_small(r_words.get(), a_words.get(), a_words.get(),
                                  m_width, mont.get());
      bn_from_montgomery_small(r_words.get(), m_width, r_words.get(), m_width,
                               mont.get());
      ASSERT_TRUE(bn_set_words(ret.get(), r_words.get(), m_width));
      EXPECT_BIGNUMS_EQUAL("A * A (mod M) (Montgomery, words)",
                           mod_square.get(), ret.get());

      // Repeat the operation with |a_copy_words|.
      OPENSSL_memcpy(a_copy_words.get(), a_words.get(),
                     m_width * sizeof(BN_ULONG));
      bn_mod_mul_montgomery_small(r_words.get(), a_words.get(),
                                  a_copy_words.get(), m_width, mont.get());
      // Use the second half of |tmp| so ASan will catch out-of-bounds writes.
      bn_from_montgomery_small(r_words.get(), m_width, r_words.get(), m_width,
                               mont.get());
      ASSERT_TRUE(bn_set_words(ret.get(), r_words.get(), m_width));
      EXPECT_BIGNUMS_EQUAL("A * A_copy (mod M) (Montgomery, words)",
                           mod_square.get(), ret.get());
    }
#endif
  }
}

static void TestModExp(BIGNUMFileTest *t, BN_CTX *ctx) {
  bssl::UniquePtr<BIGNUM> a = t->GetBIGNUM("A");
  bssl::UniquePtr<BIGNUM> e = t->GetBIGNUM("E");
  bssl::UniquePtr<BIGNUM> m = t->GetBIGNUM("M");
  bssl::UniquePtr<BIGNUM> mod_exp = t->GetBIGNUM("ModExp");
  ASSERT_TRUE(a);
  ASSERT_TRUE(e);
  ASSERT_TRUE(m);
  ASSERT_TRUE(mod_exp);

  bssl::UniquePtr<BIGNUM> ret(BN_new());
  ASSERT_TRUE(ret);
  ASSERT_TRUE(BN_mod_exp(ret.get(), a.get(), e.get(), m.get(), ctx));
  EXPECT_BIGNUMS_EQUAL("A ^ E (mod M)", mod_exp.get(), ret.get());

  // The other implementations require reduced inputs.
  ASSERT_TRUE(BN_nnmod(a.get(), a.get(), m.get(), ctx));

  if (BN_is_odd(m.get())) {
    ASSERT_TRUE(
        BN_mod_exp_mont(ret.get(), a.get(), e.get(), m.get(), ctx, NULL));
    EXPECT_BIGNUMS_EQUAL("A ^ E (mod M) (Montgomery)", mod_exp.get(),
                         ret.get());

    ASSERT_TRUE(BN_mod_exp_mont_consttime(ret.get(), a.get(), e.get(), m.get(),
                                          ctx, NULL));
    EXPECT_BIGNUMS_EQUAL("A ^ E (mod M) (constant-time)", mod_exp.get(),
                         ret.get());

#if !defined(BORINGSSL_SHARED_LIBRARY)
    size_t m_width = static_cast<size_t>(bn_minimal_width(m.get()));
    if (m_width <= BN_SMALL_MAX_WORDS) {
      bssl::UniquePtr<BN_MONT_CTX> mont(
          BN_MONT_CTX_new_for_modulus(m.get(), ctx));
      ASSERT_TRUE(mont.get());
      std::unique_ptr<BN_ULONG[]> r_words(new BN_ULONG[m_width]),
          a_words(new BN_ULONG[m_width]);
      ASSERT_TRUE(bn_copy_words(a_words.get(), m_width, a.get()));
      bn_to_montgomery_small(a_words.get(), a_words.get(), m_width, mont.get());
      bn_mod_exp_mont_small(r_words.get(), a_words.get(), m_width, e->d,
                            e->width, mont.get());
      bn_from_montgomery_small(r_words.get(), m_width, r_words.get(), m_width,
                               mont.get());
      ASSERT_TRUE(bn_set_words(ret.get(), r_words.get(), m_width));
      EXPECT_BIGNUMS_EQUAL("A ^ E (mod M) (Montgomery, words)", mod_exp.get(),
                           ret.get());
    }
#endif
  }
}

static void TestModExp2(BIGNUMFileTest *t, BN_CTX *ctx) {
  bssl::UniquePtr<BIGNUM> a1 = t->GetBIGNUM("A1");
  bssl::UniquePtr<BIGNUM> e1 = t->GetBIGNUM("E1");
  bssl::UniquePtr<BIGNUM> m1 = t->GetBIGNUM("M1");
  bssl::UniquePtr<BIGNUM> mod_exp1 = t->GetBIGNUM("ModExp1");
  ASSERT_TRUE(a1);
  ASSERT_TRUE(e1);
  ASSERT_TRUE(m1);
  ASSERT_TRUE(mod_exp1);

  bssl::UniquePtr<BIGNUM> a2 = t->GetBIGNUM("A2");
  bssl::UniquePtr<BIGNUM> e2 = t->GetBIGNUM("E2");
  bssl::UniquePtr<BIGNUM> m2 = t->GetBIGNUM("M2");
  bssl::UniquePtr<BIGNUM> mod_exp2 = t->GetBIGNUM("ModExp2");
  ASSERT_TRUE(a2);
  ASSERT_TRUE(e2);
  ASSERT_TRUE(m2);
  ASSERT_TRUE(mod_exp2);

  bssl::UniquePtr<BIGNUM> ret1(BN_new());
  ASSERT_TRUE(ret1);

  bssl::UniquePtr<BIGNUM> ret2(BN_new());
  ASSERT_TRUE(ret2);

  ASSERT_TRUE(BN_nnmod(a1.get(), a1.get(), m1.get(), ctx));
  ASSERT_TRUE(BN_nnmod(a2.get(), a2.get(), m2.get(), ctx));

  BN_MONT_CTX *mont1 = NULL;
  BN_MONT_CTX *mont2 = NULL;

  ASSERT_TRUE(mont1 = BN_MONT_CTX_new());
  ASSERT_TRUE(BN_MONT_CTX_set(mont1, m1.get(), ctx));
  ASSERT_TRUE(mont2 = BN_MONT_CTX_new());
  ASSERT_TRUE(BN_MONT_CTX_set(mont2, m2.get(), ctx));

  ASSERT_TRUE(BN_mod_exp_mont_consttime_x2(ret1.get(), a1.get(), e1.get(), m1.get(), mont1,
                                           ret2.get(), a2.get(), e2.get(), m2.get(), mont2,
					   ctx));

  EXPECT_BIGNUMS_EQUAL("A1 ^ E1 (mod M1) (constant-time)", mod_exp1.get(),
		       ret1.get());
  EXPECT_BIGNUMS_EQUAL("A2 ^ E2 (mod M2) (constant-time)", mod_exp2.get(),
		       ret2.get());

  BN_MONT_CTX_free(mont1);
  BN_MONT_CTX_free(mont2);
}

static void TestExp(BIGNUMFileTest *t, BN_CTX *ctx) {
  bssl::UniquePtr<BIGNUM> a = t->GetBIGNUM("A");
  bssl::UniquePtr<BIGNUM> e = t->GetBIGNUM("E");
  bssl::UniquePtr<BIGNUM> exp = t->GetBIGNUM("Exp");
  ASSERT_TRUE(a);
  ASSERT_TRUE(e);
  ASSERT_TRUE(exp);

  bssl::UniquePtr<BIGNUM> ret(BN_new());
  ASSERT_TRUE(ret);
  ASSERT_TRUE(BN_exp(ret.get(), a.get(), e.get(), ctx));
  EXPECT_BIGNUMS_EQUAL("A ^ E", exp.get(), ret.get());
}

static void TestModSqrt(BIGNUMFileTest *t, BN_CTX *ctx) {
  bssl::UniquePtr<BIGNUM> a = t->GetBIGNUM("A");
  bssl::UniquePtr<BIGNUM> p = t->GetBIGNUM("P");
  bssl::UniquePtr<BIGNUM> mod_sqrt = t->GetBIGNUM("ModSqrt");
  bssl::UniquePtr<BIGNUM> mod_sqrt2(BN_new());
  ASSERT_TRUE(a);
  ASSERT_TRUE(p);
  ASSERT_TRUE(mod_sqrt);
  ASSERT_TRUE(mod_sqrt2);
  // There are two possible answers.
  ASSERT_TRUE(BN_sub(mod_sqrt2.get(), p.get(), mod_sqrt.get()));

  // -0 is 0, not P.
  if (BN_is_zero(mod_sqrt.get())) {
    BN_zero(mod_sqrt2.get());
  }

  bssl::UniquePtr<BIGNUM> ret(BN_new());
  ASSERT_TRUE(ret);
  ASSERT_TRUE(BN_mod_sqrt(ret.get(), a.get(), p.get(), ctx));
  if (BN_cmp(ret.get(), mod_sqrt2.get()) != 0) {
    EXPECT_BIGNUMS_EQUAL("sqrt(A) (mod P)", mod_sqrt.get(), ret.get());
  }
}

static void TestNotModSquare(BIGNUMFileTest *t, BN_CTX *ctx) {
  bssl::UniquePtr<BIGNUM> not_mod_square = t->GetBIGNUM("NotModSquare");
  bssl::UniquePtr<BIGNUM> p = t->GetBIGNUM("P");
  bssl::UniquePtr<BIGNUM> ret(BN_new());
  ASSERT_TRUE(not_mod_square);
  ASSERT_TRUE(p);
  ASSERT_TRUE(ret);

  EXPECT_FALSE(BN_mod_sqrt(ret.get(), not_mod_square.get(), p.get(), ctx))
      << "BN_mod_sqrt unexpectedly succeeded.";

  EXPECT_TRUE(ErrorEquals(ERR_peek_error(), ERR_LIB_BN, BN_R_NOT_A_SQUARE));
  ERR_clear_error();
}

static void TestModInv(BIGNUMFileTest *t, BN_CTX *ctx) {
  bssl::UniquePtr<BIGNUM> a = t->GetBIGNUM("A");
  bssl::UniquePtr<BIGNUM> m = t->GetBIGNUM("M");
  bssl::UniquePtr<BIGNUM> mod_inv = t->GetBIGNUM("ModInv");
  ASSERT_TRUE(a);
  ASSERT_TRUE(m);
  ASSERT_TRUE(mod_inv);

  bssl::UniquePtr<BIGNUM> ret(BN_new());
  ASSERT_TRUE(ret);
  ASSERT_TRUE(BN_mod_inverse(ret.get(), a.get(), m.get(), ctx));
  EXPECT_BIGNUMS_EQUAL("inv(A) (mod M)", mod_inv.get(), ret.get());

  ASSERT_TRUE(BN_gcd(ret.get(), a.get(), m.get(), ctx));
  EXPECT_BIGNUMS_EQUAL("GCD(A, M)", BN_value_one(), ret.get());

  ASSERT_TRUE(BN_nnmod(a.get(), a.get(), m.get(), ctx));
  int no_inverse;
  ASSERT_TRUE(
      bn_mod_inverse_consttime(ret.get(), &no_inverse, a.get(), m.get(), ctx));
  EXPECT_BIGNUMS_EQUAL("inv(A) (mod M) (constant-time)", mod_inv.get(),
                       ret.get());

  ASSERT_TRUE(BN_copy(ret.get(), m.get()));
  ASSERT_TRUE(BN_mod_inverse(ret.get(), a.get(), ret.get(), ctx));
  EXPECT_BIGNUMS_EQUAL("inv(A) (mod M) (ret == m)", mod_inv.get(), ret.get());

  ASSERT_TRUE(BN_copy(ret.get(), a.get()));
  ASSERT_TRUE(BN_mod_inverse(ret.get(), ret.get(), m.get(), ctx));
  EXPECT_BIGNUMS_EQUAL("inv(A) (mod M) (ret == a)", mod_inv.get(), ret.get());
}

static void TestGCD(BIGNUMFileTest *t, BN_CTX *ctx) {
  bssl::UniquePtr<BIGNUM> a = t->GetBIGNUM("A");
  bssl::UniquePtr<BIGNUM> b = t->GetBIGNUM("B");
  bssl::UniquePtr<BIGNUM> gcd = t->GetBIGNUM("GCD");
  bssl::UniquePtr<BIGNUM> lcm = t->GetBIGNUM("LCM");
  ASSERT_TRUE(a);
  ASSERT_TRUE(b);
  ASSERT_TRUE(gcd);
  ASSERT_TRUE(lcm);

  bssl::UniquePtr<BIGNUM> ret(BN_new());
  ASSERT_TRUE(ret);
  ASSERT_TRUE(BN_gcd(ret.get(), a.get(), b.get(), ctx));
  EXPECT_BIGNUMS_EQUAL("GCD(A, B)", gcd.get(), ret.get());

  if (!BN_is_one(gcd.get())) {
    EXPECT_FALSE(BN_mod_inverse(ret.get(), a.get(), b.get(), ctx))
        << "A^-1 (mod B) computed, but it does not exist";
    EXPECT_FALSE(BN_mod_inverse(ret.get(), b.get(), a.get(), ctx))
        << "B^-1 (mod A) computed, but it does not exist";

    if (!BN_is_zero(b.get())) {
      bssl::UniquePtr<BIGNUM> a_reduced(BN_new());
      ASSERT_TRUE(a_reduced);
      ASSERT_TRUE(BN_nnmod(a_reduced.get(), a.get(), b.get(), ctx));
      int no_inverse;
      EXPECT_FALSE(bn_mod_inverse_consttime(ret.get(), &no_inverse,
                                            a_reduced.get(), b.get(), ctx))
          << "A^-1 (mod B) computed, but it does not exist";
      EXPECT_TRUE(no_inverse);
    }

    if (!BN_is_zero(a.get())) {
      bssl::UniquePtr<BIGNUM> b_reduced(BN_new());
      ASSERT_TRUE(b_reduced);
      ASSERT_TRUE(BN_nnmod(b_reduced.get(), b.get(), a.get(), ctx));
      int no_inverse;
      EXPECT_FALSE(bn_mod_inverse_consttime(ret.get(), &no_inverse,
                                            b_reduced.get(), a.get(), ctx))
          << "B^-1 (mod A) computed, but it does not exist";
      EXPECT_TRUE(no_inverse);
    }
  }

  int is_relative_prime;
  ASSERT_TRUE(
      bn_is_relatively_prime(&is_relative_prime, a.get(), b.get(), ctx));
  EXPECT_EQ(is_relative_prime, BN_is_one(gcd.get()));

  if (!BN_is_zero(gcd.get())) {
    ASSERT_TRUE(bn_lcm_consttime(ret.get(), a.get(), b.get(), ctx));
    EXPECT_BIGNUMS_EQUAL("LCM(A, B)", lcm.get(), ret.get());
  }
}

class BNTest : public testing::Test {
 protected:
  void SetUp() override {
    ctx_.reset(BN_CTX_new());
    ASSERT_TRUE(ctx_);
  }

  BN_CTX *ctx() { return ctx_.get(); }

 private:
  bssl::UniquePtr<BN_CTX> ctx_;
};

static void RunBNFileTest(FileTest *t, BN_CTX *ctx) {
  static const struct {
    const char *name;
    void (*func)(BIGNUMFileTest *t, BN_CTX *ctx);
  } kTests[] = {
      {"Sum", TestSum},
      {"LShift1", TestLShift1},
      {"LShift", TestLShift},
      {"RShift", TestRShift},
      {"Square", TestSquare},
      {"Product", TestProduct},
      {"Quotient", TestQuotient},
      {"ModMul", TestModMul},
      {"ModSquare", TestModSquare},
      {"ModExp", TestModExp},
      {"ModExp2", TestModExp2},
      {"Exp", TestExp},
      {"ModSqrt", TestModSqrt},
      {"NotModSquare", TestNotModSquare},
      {"ModInv", TestModInv},
      {"GCD", TestGCD},
  };
  void (*func)(BIGNUMFileTest * t, BN_CTX * ctx) = nullptr;
  for (const auto &test : kTests) {
    if (t->GetType() == test.name) {
      func = test.func;
      break;
    }
  }
  if (!func) {
    FAIL() << "Unknown test type: " << t->GetType();
    return;
  }

  // Run the test with normalize-sized |BIGNUM|s.
  BIGNUMFileTest bn_test(t, 0);
  BN_CTX_start(ctx);
  func(&bn_test, ctx);
  BN_CTX_end(ctx);
  unsigned num_bignums = bn_test.num_bignums();

  // Repeat the test with all combinations of large and small |BIGNUM|s.
  for (unsigned large_mask = 1; large_mask < (1u << num_bignums);
       large_mask++) {
    SCOPED_TRACE(large_mask);
    BIGNUMFileTest bn_test2(t, large_mask);
    BN_CTX_start(ctx);
    func(&bn_test2, ctx);
    BN_CTX_end(ctx);
  }
}

TEST_F(BNTest, ExpTestVectors) {
  FileTestGTest("crypto/fipsmodule/bn/test/exp_tests.txt",
                [&](FileTest *t) { RunBNFileTest(t, ctx()); });
}

TEST_F(BNTest, GCDTestVectors) {
  FileTestGTest("crypto/fipsmodule/bn/test/gcd_tests.txt",
                [&](FileTest *t) { RunBNFileTest(t, ctx()); });
}

TEST_F(BNTest, ModExpTestVectors) {
  FileTestGTest("crypto/fipsmodule/bn/test/mod_exp_tests.txt",
                [&](FileTest *t) { RunBNFileTest(t, ctx()); });
}

TEST_F(BNTest, ModExp2TestVectors) {
  FileTestGTest("crypto/fipsmodule/bn/test/mod_exp_x2_tests.txt",
                [&](FileTest *t) { RunBNFileTest(t, ctx()); });
}

TEST_F(BNTest, ModInvTestVectors) {
  FileTestGTest("crypto/fipsmodule/bn/test/mod_inv_tests.txt",
                [&](FileTest *t) { RunBNFileTest(t, ctx()); });
}

TEST_F(BNTest, ModMulTestVectors) {
  FileTestGTest("crypto/fipsmodule/bn/test/mod_mul_tests.txt",
                [&](FileTest *t) { RunBNFileTest(t, ctx()); });
}

TEST_F(BNTest, ModSqrtTestVectors) {
  FileTestGTest("crypto/fipsmodule/bn/test/mod_sqrt_tests.txt",
                [&](FileTest *t) { RunBNFileTest(t, ctx()); });
}

TEST_F(BNTest, ProductTestVectors) {
  FileTestGTest("crypto/fipsmodule/bn/test/product_tests.txt",
                [&](FileTest *t) { RunBNFileTest(t, ctx()); });
}

TEST_F(BNTest, QuotientTestVectors) {
  FileTestGTest("crypto/fipsmodule/bn/test/quotient_tests.txt",
                [&](FileTest *t) { RunBNFileTest(t, ctx()); });
}

TEST_F(BNTest, ShiftTestVectors) {
  FileTestGTest("crypto/fipsmodule/bn/test/shift_tests.txt",
                [&](FileTest *t) { RunBNFileTest(t, ctx()); });
}

TEST_F(BNTest, SumTestVectors) {
  FileTestGTest("crypto/fipsmodule/bn/test/sum_tests.txt",
                [&](FileTest *t) { RunBNFileTest(t, ctx()); });
}

TEST_F(BNTest, BN2BinPadded) {
  uint8_t zeros[256], out[256], reference[128];

  OPENSSL_memset(zeros, 0, sizeof(zeros));

  // Test edge case at 0.
  bssl::UniquePtr<BIGNUM> n(BN_new());
  ASSERT_TRUE(n);
  ASSERT_TRUE(BN_bn2bin_padded(NULL, 0, n.get()));

  OPENSSL_memset(out, -1, sizeof(out));
  ASSERT_TRUE(BN_bn2bin_padded(out, sizeof(out), n.get()));
  EXPECT_EQ(Bytes(zeros), Bytes(out));

  // Test a random numbers at various byte lengths.
  for (size_t bytes = 128 - 7; bytes <= 128; bytes++) {
    ASSERT_TRUE(
        BN_rand(n.get(), bytes * 8, BN_RAND_TOP_ONE, BN_RAND_BOTTOM_ANY));
    ASSERT_EQ(bytes, BN_num_bytes(n.get()));
    ASSERT_EQ(bytes, BN_bn2bin(n.get(), reference));

    // Empty buffer should fail.
    EXPECT_FALSE(BN_bn2bin_padded(NULL, 0, n.get()));

    // One byte short should fail.
    EXPECT_FALSE(BN_bn2bin_padded(out, bytes - 1, n.get()));

    // Exactly right size should encode.
    ASSERT_TRUE(BN_bn2bin_padded(out, bytes, n.get()));
    EXPECT_EQ(Bytes(reference, bytes), Bytes(out, bytes));

    // Pad up one byte extra.
    ASSERT_TRUE(BN_bn2bin_padded(out, bytes + 1, n.get()));
    EXPECT_EQ(0u, out[0]);
    EXPECT_EQ(Bytes(reference, bytes), Bytes(out + 1, bytes));

    // Pad up to 256.
    ASSERT_TRUE(BN_bn2bin_padded(out, sizeof(out), n.get()));
    EXPECT_EQ(Bytes(zeros, sizeof(out) - bytes),
              Bytes(out, sizeof(out) - bytes));
    EXPECT_EQ(Bytes(reference, bytes), Bytes(out + sizeof(out) - bytes, bytes));

    // Repeat some tests with a non-minimal |BIGNUM|.
    EXPECT_TRUE(bn_resize_words(n.get(), 32));

    EXPECT_FALSE(BN_bn2bin_padded(out, bytes - 1, n.get()));

    ASSERT_TRUE(BN_bn2bin_padded(out, bytes + 1, n.get()));
    EXPECT_EQ(0u, out[0]);
    EXPECT_EQ(Bytes(reference, bytes), Bytes(out + 1, bytes));
  }
}

TEST_F(BNTest, LittleEndian) {
  bssl::UniquePtr<BIGNUM> x(BN_new());
  bssl::UniquePtr<BIGNUM> y(BN_new());
  ASSERT_TRUE(x);
  ASSERT_TRUE(y);

  // Test edge case at 0. Fill |out| with garbage to ensure |BN_bn2le_padded|
  // wrote the result.
  uint8_t out[256], zeros[256];
  OPENSSL_memset(out, -1, sizeof(out));
  OPENSSL_memset(zeros, 0, sizeof(zeros));
  ASSERT_TRUE(BN_bn2le_padded(out, sizeof(out), x.get()));
  EXPECT_EQ(Bytes(zeros), Bytes(out));

  ASSERT_TRUE(BN_le2bn(out, sizeof(out), y.get()));
  EXPECT_BIGNUMS_EQUAL("BN_le2bn round-trip", x.get(), y.get());

  // Test random numbers at various byte lengths.
  for (size_t bytes = 128 - 7; bytes <= 128; bytes++) {
    ASSERT_TRUE(
        BN_rand(x.get(), bytes * 8, BN_RAND_TOP_ONE, BN_RAND_BOTTOM_ANY));

    // Fill |out| with garbage to ensure |BN_bn2le_padded| wrote the result.
    OPENSSL_memset(out, -1, sizeof(out));
    ASSERT_TRUE(BN_bn2le_padded(out, sizeof(out), x.get()));

    // Compute the expected value by reversing the big-endian output.
    uint8_t expected[sizeof(out)];
    ASSERT_TRUE(BN_bn2bin_padded(expected, sizeof(expected), x.get()));
    for (size_t i = 0; i < sizeof(expected) / 2; i++) {
      uint8_t tmp = expected[i];
      expected[i] = expected[sizeof(expected) - 1 - i];
      expected[sizeof(expected) - 1 - i] = tmp;
    }

    EXPECT_EQ(Bytes(out), Bytes(expected));

    // Make sure the decoding produces the same BIGNUM.
    ASSERT_TRUE(BN_le2bn(out, bytes, y.get()));
    EXPECT_BIGNUMS_EQUAL("BN_le2bn round-trip", x.get(), y.get());
  }
}

static int DecimalToBIGNUM(bssl::UniquePtr<BIGNUM> *out, const char *in) {
  BIGNUM *raw = NULL;
  int ret = BN_dec2bn(&raw, in);
  out->reset(raw);
  return ret;
}

TEST_F(BNTest, Dec2BN) {
  bssl::UniquePtr<BIGNUM> bn;
  int ret = DecimalToBIGNUM(&bn, "0");
  ASSERT_EQ(1, ret);
  EXPECT_TRUE(BN_is_zero(bn.get()));
  EXPECT_FALSE(BN_is_negative(bn.get()));

  ret = DecimalToBIGNUM(&bn, "256");
  ASSERT_EQ(3, ret);
  EXPECT_TRUE(BN_is_word(bn.get(), 256));
  EXPECT_FALSE(BN_is_negative(bn.get()));

  ret = DecimalToBIGNUM(&bn, "-42");
  ASSERT_EQ(3, ret);
  EXPECT_TRUE(BN_abs_is_word(bn.get(), 42));
  EXPECT_TRUE(BN_is_negative(bn.get()));

  ret = DecimalToBIGNUM(&bn, "-0");
  ASSERT_EQ(2, ret);
  EXPECT_TRUE(BN_is_zero(bn.get()));
  EXPECT_FALSE(BN_is_negative(bn.get()));

  ret = DecimalToBIGNUM(&bn, "42trailing garbage is ignored");
  ASSERT_EQ(2, ret);
  EXPECT_TRUE(BN_abs_is_word(bn.get(), 42));
  EXPECT_FALSE(BN_is_negative(bn.get()));
}

TEST_F(BNTest, Hex2BN) {
  bssl::UniquePtr<BIGNUM> bn;
  int ret = HexToBIGNUMWithReturn(&bn, "0");
  ASSERT_EQ(1, ret);
  EXPECT_TRUE(BN_is_zero(bn.get()));
  EXPECT_FALSE(BN_is_negative(bn.get()));

  ret = HexToBIGNUMWithReturn(&bn, "256");
  ASSERT_EQ(3, ret);
  EXPECT_TRUE(BN_is_word(bn.get(), 0x256));
  EXPECT_FALSE(BN_is_negative(bn.get()));

  ret = HexToBIGNUMWithReturn(&bn, "-42");
  ASSERT_EQ(3, ret);
  EXPECT_TRUE(BN_abs_is_word(bn.get(), 0x42));
  EXPECT_TRUE(BN_is_negative(bn.get()));

  ret = HexToBIGNUMWithReturn(&bn, "-0");
  ASSERT_EQ(2, ret);
  EXPECT_TRUE(BN_is_zero(bn.get()));
  EXPECT_FALSE(BN_is_negative(bn.get()));

  ret = HexToBIGNUMWithReturn(&bn, "abctrailing garbage is ignored");
  ASSERT_EQ(3, ret);
  EXPECT_TRUE(BN_is_word(bn.get(), 0xabc));
  EXPECT_FALSE(BN_is_negative(bn.get()));
}

static bssl::UniquePtr<BIGNUM> ASCIIToBIGNUM(const char *in) {
  BIGNUM *raw = NULL;
  if (!BN_asc2bn(&raw, in)) {
    return nullptr;
  }
  return bssl::UniquePtr<BIGNUM>(raw);
}

TEST_F(BNTest, ASC2BN) {
  bssl::UniquePtr<BIGNUM> bn = ASCIIToBIGNUM("0");
  ASSERT_TRUE(bn);
  EXPECT_TRUE(BN_is_zero(bn.get()));
  EXPECT_FALSE(BN_is_negative(bn.get()));

  bn = ASCIIToBIGNUM("256");
  ASSERT_TRUE(bn);
  EXPECT_TRUE(BN_is_word(bn.get(), 256));
  EXPECT_FALSE(BN_is_negative(bn.get()));

  bn = ASCIIToBIGNUM("-42");
  ASSERT_TRUE(bn);
  EXPECT_TRUE(BN_abs_is_word(bn.get(), 42));
  EXPECT_TRUE(BN_is_negative(bn.get()));

  bn = ASCIIToBIGNUM("0x1234");
  ASSERT_TRUE(bn);
  EXPECT_TRUE(BN_is_word(bn.get(), 0x1234));
  EXPECT_FALSE(BN_is_negative(bn.get()));

  bn = ASCIIToBIGNUM("0X1234");
  ASSERT_TRUE(bn);
  EXPECT_TRUE(BN_is_word(bn.get(), 0x1234));
  EXPECT_FALSE(BN_is_negative(bn.get()));

  bn = ASCIIToBIGNUM("-0xabcd");
  ASSERT_TRUE(bn);
  EXPECT_TRUE(BN_abs_is_word(bn.get(), 0xabcd));
  EXPECT_FALSE(!BN_is_negative(bn.get()));

  bn = ASCIIToBIGNUM("-0");
  ASSERT_TRUE(bn);
  EXPECT_TRUE(BN_is_zero(bn.get()));
  EXPECT_FALSE(BN_is_negative(bn.get()));

  bn = ASCIIToBIGNUM("123trailing garbage is ignored");
  ASSERT_TRUE(bn);
  EXPECT_TRUE(BN_is_word(bn.get(), 123));
  EXPECT_FALSE(BN_is_negative(bn.get()));
}

struct MPITest {
  const char *base10;
  const char *mpi;
  size_t mpi_len;
};

static const MPITest kMPITests[] = {
  { "0", "\x00\x00\x00\x00", 4 },
  { "1", "\x00\x00\x00\x01\x01", 5 },
  { "-1", "\x00\x00\x00\x01\x81", 5 },
  { "128", "\x00\x00\x00\x02\x00\x80", 6 },
  { "256", "\x00\x00\x00\x02\x01\x00", 6 },
  { "-256", "\x00\x00\x00\x02\x81\x00", 6 },
};

TEST_F(BNTest, MPI) {
  uint8_t scratch[8];

  for (const auto &test : kMPITests) {
    SCOPED_TRACE(test.base10);
    bssl::UniquePtr<BIGNUM> bn(ASCIIToBIGNUM(test.base10));
    ASSERT_TRUE(bn);

    const size_t mpi_len = BN_bn2mpi(bn.get(), NULL);
    ASSERT_LE(mpi_len, sizeof(scratch)) << "MPI size is too large to test";

    const size_t mpi_len2 = BN_bn2mpi(bn.get(), scratch);
    EXPECT_EQ(mpi_len, mpi_len2);
    EXPECT_EQ(Bytes(test.mpi, test.mpi_len), Bytes(scratch, mpi_len));

    bssl::UniquePtr<BIGNUM> bn2(BN_mpi2bn(scratch, mpi_len, NULL));
    ASSERT_TRUE(bn2) << "failed to parse";
    EXPECT_BIGNUMS_EQUAL("BN_mpi2bn", bn.get(), bn2.get());
  }
}

TEST_F(BNTest, Rand) {
  bssl::UniquePtr<BIGNUM> bn(BN_new());
  ASSERT_TRUE(bn);

  static const int kTop[] = {BN_RAND_TOP_ANY, BN_RAND_TOP_ONE, BN_RAND_TOP_TWO};
  static const int kBottom[] = {BN_RAND_BOTTOM_ANY, BN_RAND_BOTTOM_ODD};
  for (unsigned bits = 0; bits < 256; bits++) {
    SCOPED_TRACE(bits);
    for (int top : kTop) {
      SCOPED_TRACE(top);
      for (int bottom : kBottom) {
        SCOPED_TRACE(bottom);

        // Generate 100 numbers and ensure that they have the expected bit
        // patterns. The probability of any one bit not covering both its values
        // is 2^-100.
        bool seen_n_1_clear = false, seen_n_1_set = false;
        bool seen_n_2_clear = false, seen_n_2_set = false;
        bool seen_0_clear = false, seen_0_set = false;
        for (int i = 0; i < 100; i++) {
          ASSERT_TRUE(BN_rand(bn.get(), bits, top, bottom));
          EXPECT_LE(BN_num_bits(bn.get()), bits);
          if (BN_is_bit_set(bn.get(), bits - 1)) {
            seen_n_1_set = true;
          } else {
            seen_n_1_clear = true;
          }
          if (BN_is_bit_set(bn.get(), bits - 2)) {
            seen_n_2_set = true;
          } else {
            seen_n_2_clear = true;
          }
          if (BN_is_bit_set(bn.get(), 0)) {
            seen_0_set = true;
          } else {
            seen_0_clear = true;
          }
        }

        if (bits > 0) {
          EXPECT_TRUE(seen_0_set);
          EXPECT_TRUE(seen_n_1_set);
          if (bits > 1) {
            EXPECT_TRUE(seen_n_2_set);
          }
        }

        if (bits == 0) {
          // Nothing additional to check. The |BN_num_bits| check ensures we
          // always got zero.
        } else if (bits == 1) {
          // Bit zero is bit n-1.
          EXPECT_EQ(bottom == BN_RAND_BOTTOM_ANY && top == BN_RAND_TOP_ANY,
                    seen_0_clear);
        } else if (bits == 2) {
          // Bit zero is bit n-2.
          EXPECT_EQ(bottom == BN_RAND_BOTTOM_ANY && top != BN_RAND_TOP_TWO,
                    seen_0_clear);
          EXPECT_EQ(top == BN_RAND_TOP_ANY, seen_n_1_clear);
        } else {
          EXPECT_EQ(bottom == BN_RAND_BOTTOM_ANY, seen_0_clear);
          EXPECT_EQ(top != BN_RAND_TOP_TWO, seen_n_2_clear);
          EXPECT_EQ(top == BN_RAND_TOP_ANY, seen_n_1_clear);
        }
      }
    }
  }
}

TEST_F(BNTest, RandRange) {
  bssl::UniquePtr<BIGNUM> bn(BN_new()), six(BN_new());
  ASSERT_TRUE(bn);
  ASSERT_TRUE(six);
  ASSERT_TRUE(BN_set_word(six.get(), 6));

  // Generate 1,000 random numbers and ensure they all stay in range. This check
  // may flakily pass when it should have failed but will not flakily fail.
  bool seen[6] = {false, false, false, false, false};
  for (unsigned i = 0; i < 1000; i++) {
    SCOPED_TRACE(i);
    ASSERT_TRUE(BN_rand_range_ex(bn.get(), 1, six.get()));

    BN_ULONG word = BN_get_word(bn.get());
    if (BN_is_negative(bn.get()) ||
        word < 1 ||
        word >= 6) {
      FAIL() << "BN_rand_range_ex generated invalid value: " << word;
    }

    seen[word] = true;
  }

  // Test that all numbers were accounted for. Note this test is probabilistic
  // and may flakily fail when it should have passed. As an upper-bound on the
  // failure probability, we'll never see any one number with probability
  // (4/5)^1000, so the probability of failure is at most 5*(4/5)^1000. This is
  // around 1 in 2^320.
  for (unsigned i = 1; i < 6; i++) {
    EXPECT_TRUE(seen[i]) << "BN_rand_range failed to generated " << i;
  }
}

struct ASN1Test {
  const char *value_ascii;
  const char *der;
  size_t der_len;
};

static const ASN1Test kASN1Tests[] = {
    {"0", "\x02\x01\x00", 3},
    {"1", "\x02\x01\x01", 3},
    {"127", "\x02\x01\x7f", 3},
    {"128", "\x02\x02\x00\x80", 4},
    {"0xdeadbeef", "\x02\x05\x00\xde\xad\xbe\xef", 7},
    {"0x0102030405060708",
     "\x02\x08\x01\x02\x03\x04\x05\x06\x07\x08", 10},
    {"0xffffffffffffffff",
      "\x02\x09\x00\xff\xff\xff\xff\xff\xff\xff\xff", 11},
};

struct ASN1InvalidTest {
  const char *der;
  size_t der_len;
};

static const ASN1InvalidTest kASN1InvalidTests[] = {
    // Bad tag.
    {"\x03\x01\x00", 3},
    // Empty contents.
    {"\x02\x00", 2},
    // Negative numbers.
    {"\x02\x01\x80", 3},
    {"\x02\x01\xff", 3},
    // Unnecessary leading zeros.
    {"\x02\x02\x00\x01", 4},
};

TEST_F(BNTest, ASN1) {
  for (const ASN1Test &test : kASN1Tests) {
    SCOPED_TRACE(test.value_ascii);
    bssl::UniquePtr<BIGNUM> bn = ASCIIToBIGNUM(test.value_ascii);
    ASSERT_TRUE(bn);

    // Test that the input is correctly parsed.
    bssl::UniquePtr<BIGNUM> bn2(BN_new());
    ASSERT_TRUE(bn2);
    CBS cbs;
    CBS_init(&cbs, reinterpret_cast<const uint8_t*>(test.der), test.der_len);
    ASSERT_TRUE(BN_parse_asn1_unsigned(&cbs, bn2.get()));
    EXPECT_EQ(0u, CBS_len(&cbs));
    EXPECT_BIGNUMS_EQUAL("decode ASN.1", bn.get(), bn2.get());

    // Test the value serializes correctly.
    bssl::ScopedCBB cbb;
    uint8_t *der;
    size_t der_len;
    ASSERT_TRUE(CBB_init(cbb.get(), 0));
    ASSERT_TRUE(BN_marshal_asn1(cbb.get(), bn.get()));
    ASSERT_TRUE(CBB_finish(cbb.get(), &der, &der_len));
    bssl::UniquePtr<uint8_t> delete_der(der);
    EXPECT_EQ(Bytes(test.der, test.der_len), Bytes(der, der_len));
  }

  for (const ASN1InvalidTest &test : kASN1InvalidTests) {
    SCOPED_TRACE(Bytes(test.der, test.der_len));;
    bssl::UniquePtr<BIGNUM> bn(BN_new());
    ASSERT_TRUE(bn);
    CBS cbs;
    CBS_init(&cbs, reinterpret_cast<const uint8_t *>(test.der), test.der_len);
    EXPECT_FALSE(BN_parse_asn1_unsigned(&cbs, bn.get()))
        << "Parsed invalid input.";
    ERR_clear_error();
  }

  // Serializing negative numbers is not supported.
  bssl::UniquePtr<BIGNUM> bn = ASCIIToBIGNUM("-1");
  ASSERT_TRUE(bn);
  bssl::ScopedCBB cbb;
  ASSERT_TRUE(CBB_init(cbb.get(), 0));
  EXPECT_FALSE(BN_marshal_asn1(cbb.get(), bn.get()))
      << "Serialized negative number.";
  ERR_clear_error();
}

TEST_F(BNTest, NegativeZero) {
  bssl::UniquePtr<BIGNUM> a(BN_new());
  bssl::UniquePtr<BIGNUM> b(BN_new());
  bssl::UniquePtr<BIGNUM> c(BN_new());
  ASSERT_TRUE(a);
  ASSERT_TRUE(b);
  ASSERT_TRUE(c);

  // Test that BN_mul never gives negative zero.
  ASSERT_TRUE(BN_set_word(a.get(), 1));
  BN_set_negative(a.get(), 1);
  BN_zero(b.get());
  ASSERT_TRUE(BN_mul(c.get(), a.get(), b.get(), ctx()));
  EXPECT_TRUE(BN_is_zero(c.get()));
  EXPECT_FALSE(BN_is_negative(c.get()));

  bssl::UniquePtr<BIGNUM> numerator(BN_new()), denominator(BN_new());
  ASSERT_TRUE(numerator);
  ASSERT_TRUE(denominator);

  // Test that BN_div never gives negative zero in the quotient.
  ASSERT_TRUE(BN_set_word(numerator.get(), 1));
  ASSERT_TRUE(BN_set_word(denominator.get(), 2));
  BN_set_negative(numerator.get(), 1);
  ASSERT_TRUE(
      BN_div(a.get(), b.get(), numerator.get(), denominator.get(), ctx()));
  EXPECT_TRUE(BN_is_zero(a.get()));
  EXPECT_FALSE(BN_is_negative(a.get()));

  // Test that BN_div never gives negative zero in the remainder.
  ASSERT_TRUE(BN_set_word(denominator.get(), 1));
  ASSERT_TRUE(
      BN_div(a.get(), b.get(), numerator.get(), denominator.get(), ctx()));
  EXPECT_TRUE(BN_is_zero(b.get()));
  EXPECT_FALSE(BN_is_negative(b.get()));

  // Test that BN_set_negative will not produce a negative zero.
  BN_zero(a.get());
  BN_set_negative(a.get(), 1);
  EXPECT_FALSE(BN_is_negative(a.get()));

  // Test that forcibly creating a negative zero does not break |BN_bn2hex| or
  // |BN_bn2dec|.
  a->neg = 1;
  bssl::UniquePtr<char> dec(BN_bn2dec(a.get()));
  bssl::UniquePtr<char> hex(BN_bn2hex(a.get()));
  ASSERT_TRUE(dec);
  ASSERT_TRUE(hex);
  EXPECT_STREQ("-0", dec.get());
  EXPECT_STREQ("-0", hex.get());

  // Test that |BN_rshift| and |BN_rshift1| will not produce a negative zero.
  ASSERT_TRUE(BN_set_word(a.get(), 1));
  BN_set_negative(a.get(), 1);

  ASSERT_TRUE(BN_rshift(b.get(), a.get(), 1));
  EXPECT_TRUE(BN_is_zero(b.get()));
  EXPECT_FALSE(BN_is_negative(b.get()));

  ASSERT_TRUE(BN_rshift1(c.get(), a.get()));
  EXPECT_TRUE(BN_is_zero(c.get()));
  EXPECT_FALSE(BN_is_negative(c.get()));

  // Test that |BN_div_word| will not produce a negative zero.
  ASSERT_NE((BN_ULONG)-1, BN_div_word(a.get(), 2));
  EXPECT_TRUE(BN_is_zero(a.get()));
  EXPECT_FALSE(BN_is_negative(a.get()));
}

TEST_F(BNTest, BadModulus) {
  bssl::UniquePtr<BIGNUM> a(BN_new());
  bssl::UniquePtr<BIGNUM> b(BN_new());
  bssl::UniquePtr<BIGNUM> zero(BN_new());
  ASSERT_TRUE(a);
  ASSERT_TRUE(b);
  ASSERT_TRUE(zero);

  BN_zero(zero.get());

  EXPECT_FALSE(BN_div(a.get(), b.get(), BN_value_one(), zero.get(), ctx()));
  ERR_clear_error();

  EXPECT_FALSE(
      BN_mod_mul(a.get(), BN_value_one(), BN_value_one(), zero.get(), ctx()));
  ERR_clear_error();

  EXPECT_FALSE(
      BN_mod_exp(a.get(), BN_value_one(), BN_value_one(), zero.get(), ctx()));
  ERR_clear_error();

  EXPECT_FALSE(BN_mod_exp_mont(a.get(), BN_value_one(), BN_value_one(),
                               zero.get(), ctx(), NULL));
  ERR_clear_error();

  EXPECT_FALSE(BN_mod_exp_mont_consttime(
      a.get(), BN_value_one(), BN_value_one(), zero.get(), ctx(), nullptr));
  ERR_clear_error();

  bssl::UniquePtr<BN_MONT_CTX> mont(
      BN_MONT_CTX_new_for_modulus(zero.get(), ctx()));
  EXPECT_FALSE(mont);
  ERR_clear_error();

  mont.reset(BN_MONT_CTX_new_consttime(b.get(), ctx()));
  EXPECT_FALSE(mont);
  ERR_clear_error();

  // Some operations also may not be used with an even modulus.
  ASSERT_TRUE(BN_set_word(b.get(), 16));

  mont.reset(BN_MONT_CTX_new_for_modulus(b.get(), ctx()));
  EXPECT_FALSE(mont);
  ERR_clear_error();

  mont.reset(BN_MONT_CTX_new_consttime(b.get(), ctx()));
  EXPECT_FALSE(mont);
  ERR_clear_error();

  EXPECT_FALSE(BN_mod_exp_mont(a.get(), BN_value_one(), BN_value_one(), b.get(),
                               ctx(), NULL));
  ERR_clear_error();

  EXPECT_FALSE(BN_mod_exp_mont_consttime(
      a.get(), BN_value_one(), BN_value_one(), b.get(), ctx(), nullptr));
  ERR_clear_error();
}

// Test that a**0 mod 1 == 0.
TEST_F(BNTest, ExpZeroModOne) {
  bssl::UniquePtr<BIGNUM> zero(BN_new()), a(BN_new()), r(BN_new()),
      minus_one(BN_new());
  ASSERT_TRUE(zero);
  ASSERT_TRUE(a);
  ASSERT_TRUE(r);
  ASSERT_TRUE(minus_one);
  ASSERT_TRUE(BN_set_word(minus_one.get(), 1));
  BN_set_negative(minus_one.get(), 1);
  ASSERT_TRUE(BN_rand(a.get(), 1024, BN_RAND_TOP_ONE, BN_RAND_BOTTOM_ANY));
  BN_zero(zero.get());

  ASSERT_TRUE(BN_mod_exp(r.get(), a.get(), zero.get(), BN_value_one(), ctx()));
  EXPECT_TRUE(BN_is_zero(r.get()));
  ASSERT_TRUE(
      BN_mod_exp(r.get(), zero.get(), zero.get(), BN_value_one(), ctx()));
  EXPECT_TRUE(BN_is_zero(r.get()));

  ASSERT_TRUE(BN_mod_exp_mont_word(r.get(), 42, zero.get(), BN_value_one(),
                                   ctx(), nullptr));
  EXPECT_TRUE(BN_is_zero(r.get()));
  ASSERT_TRUE(BN_mod_exp_mont_word(r.get(), 0, zero.get(), BN_value_one(),
                                   ctx(), nullptr));
  EXPECT_TRUE(BN_is_zero(r.get()));

  // |BN_mod_exp_mont| and |BN_mod_exp_mont_consttime| require fully-reduced
  // inputs, so a**0 mod 1 is not a valid call. 0**0 mod 1 is valid, however.
  ASSERT_TRUE(BN_mod_exp_mont(r.get(), zero.get(), zero.get(), BN_value_one(),
                              ctx(), nullptr));
  EXPECT_TRUE(BN_is_zero(r.get()));

  ASSERT_TRUE(BN_mod_exp_mont_consttime(r.get(), zero.get(), zero.get(),
                                        BN_value_one(), ctx(), nullptr));
  EXPECT_TRUE(BN_is_zero(r.get()));
}

TEST_F(BNTest, SmallPrime) {
  static const unsigned kBits = 10;

  bssl::UniquePtr<BIGNUM> r(BN_new());
  ASSERT_TRUE(r);
  ASSERT_TRUE(BN_generate_prime_ex(r.get(), static_cast<int>(kBits), 0, NULL,
                                  NULL, NULL));
  EXPECT_EQ(kBits, BN_num_bits(r.get()));
}

TEST_F(BNTest, CmpWord) {
  static const BN_ULONG kMaxWord = (BN_ULONG)-1;

  bssl::UniquePtr<BIGNUM> r(BN_new());
  ASSERT_TRUE(r);
  ASSERT_TRUE(BN_set_word(r.get(), 0));

  EXPECT_EQ(BN_cmp_word(r.get(), 0), 0);
  EXPECT_LT(BN_cmp_word(r.get(), 1), 0);
  EXPECT_LT(BN_cmp_word(r.get(), kMaxWord), 0);

  ASSERT_TRUE(BN_set_word(r.get(), 100));

  EXPECT_GT(BN_cmp_word(r.get(), 0), 0);
  EXPECT_GT(BN_cmp_word(r.get(), 99), 0);
  EXPECT_EQ(BN_cmp_word(r.get(), 100), 0);
  EXPECT_LT(BN_cmp_word(r.get(), 101), 0);
  EXPECT_LT(BN_cmp_word(r.get(), kMaxWord), 0);

  BN_set_negative(r.get(), 1);

  EXPECT_LT(BN_cmp_word(r.get(), 0), 0);
  EXPECT_LT(BN_cmp_word(r.get(), 100), 0);
  EXPECT_LT(BN_cmp_word(r.get(), kMaxWord), 0);

  ASSERT_TRUE(BN_set_word(r.get(), kMaxWord));

  EXPECT_GT(BN_cmp_word(r.get(), 0), 0);
  EXPECT_GT(BN_cmp_word(r.get(), kMaxWord - 1), 0);
  EXPECT_EQ(BN_cmp_word(r.get(), kMaxWord), 0);

  ASSERT_TRUE(BN_add(r.get(), r.get(), BN_value_one()));

  EXPECT_GT(BN_cmp_word(r.get(), 0), 0);
  EXPECT_GT(BN_cmp_word(r.get(), kMaxWord), 0);

  BN_set_negative(r.get(), 1);

  EXPECT_LT(BN_cmp_word(r.get(), 0), 0);
  EXPECT_LT(BN_cmp_word(r.get(), kMaxWord), 0);
}

TEST_F(BNTest, BN2Dec) {
  static const char *kBN2DecTests[] = {
      "0",
      "1",
      "-1",
      "100",
      "-100",
      "123456789012345678901234567890",
      "-123456789012345678901234567890",
      "123456789012345678901234567890123456789012345678901234567890",
      "-123456789012345678901234567890123456789012345678901234567890",
  };

  for (const char *test : kBN2DecTests) {
    SCOPED_TRACE(test);
    bssl::UniquePtr<BIGNUM> bn;
    int ret = DecimalToBIGNUM(&bn, test);
    ASSERT_NE(0, ret);

    bssl::UniquePtr<char> dec(BN_bn2dec(bn.get()));
    ASSERT_TRUE(dec);
    EXPECT_STREQ(test, dec.get());
  }
}

TEST_F(BNTest, SetGetU64) {
  static const struct {
    const char *hex;
    uint64_t value;
  } kU64Tests[] = {
      {"0", UINT64_C(0x0)},
      {"1", UINT64_C(0x1)},
      {"ffffffff", UINT64_C(0xffffffff)},
      {"100000000", UINT64_C(0x100000000)},
      {"ffffffffffffffff", UINT64_C(0xffffffffffffffff)},
  };

  for (const auto& test : kU64Tests) {
    SCOPED_TRACE(test.hex);
    bssl::UniquePtr<BIGNUM> bn(BN_new()), expected;
    ASSERT_TRUE(bn);
    ASSERT_TRUE(BN_set_u64(bn.get(), test.value));
    ASSERT_TRUE(HexToBIGNUMWithReturn(&expected, test.hex));
    EXPECT_BIGNUMS_EQUAL("BN_set_u64", expected.get(), bn.get());

    uint64_t tmp;
    ASSERT_TRUE(BN_get_u64(bn.get(), &tmp));
    EXPECT_EQ(test.value, tmp);

    // BN_get_u64 ignores the sign bit.
    BN_set_negative(bn.get(), 1);
    ASSERT_TRUE(BN_get_u64(bn.get(), &tmp));
    EXPECT_EQ(test.value, tmp);
  }

  // Test that BN_get_u64 fails on large numbers.
  bssl::UniquePtr<BIGNUM> bn(BN_new());
  ASSERT_TRUE(bn);
  ASSERT_TRUE(BN_lshift(bn.get(), BN_value_one(), 64));

  uint64_t tmp;
  EXPECT_FALSE(BN_get_u64(bn.get(), &tmp));

  BN_set_negative(bn.get(), 1);
  EXPECT_FALSE(BN_get_u64(bn.get(), &tmp));
}

TEST_F(BNTest, Pow2) {
  bssl::UniquePtr<BIGNUM> power_of_two(BN_new()), random(BN_new()),
      expected(BN_new()), actual(BN_new());
  ASSERT_TRUE(power_of_two);
  ASSERT_TRUE(random);
  ASSERT_TRUE(expected);
  ASSERT_TRUE(actual);

  // Choose an exponent.
  for (size_t e = 3; e < 512; e += 11) {
    SCOPED_TRACE(e);
    // Choose a bit length for our randoms.
    for (int len = 3; len < 512; len += 23) {
      SCOPED_TRACE(len);
      // Set power_of_two = 2^e.
      ASSERT_TRUE(BN_lshift(power_of_two.get(), BN_value_one(), (int)e));

      // Test BN_is_pow2 on power_of_two.
      EXPECT_TRUE(BN_is_pow2(power_of_two.get()));

      // Pick a large random value, ensuring it isn't a power of two.
      ASSERT_TRUE(
          BN_rand(random.get(), len, BN_RAND_TOP_TWO, BN_RAND_BOTTOM_ANY));

      // Test BN_is_pow2 on |r|.
      EXPECT_FALSE(BN_is_pow2(random.get()));

      // Test BN_mod_pow2 on |r|.
      ASSERT_TRUE(
          BN_mod(expected.get(), random.get(), power_of_two.get(), ctx()));
      ASSERT_TRUE(BN_mod_pow2(actual.get(), random.get(), e));
      EXPECT_BIGNUMS_EQUAL("random (mod power_of_two)", expected.get(),
                           actual.get());

      // Test BN_nnmod_pow2 on |r|.
      ASSERT_TRUE(
          BN_nnmod(expected.get(), random.get(), power_of_two.get(), ctx()));
      ASSERT_TRUE(BN_nnmod_pow2(actual.get(), random.get(), e));
      EXPECT_BIGNUMS_EQUAL("random (mod power_of_two), non-negative",
                           expected.get(), actual.get());

      // Test BN_nnmod_pow2 on -|r|.
      BN_set_negative(random.get(), 1);
      ASSERT_TRUE(
          BN_nnmod(expected.get(), random.get(), power_of_two.get(), ctx()));
      ASSERT_TRUE(BN_nnmod_pow2(actual.get(), random.get(), e));
      EXPECT_BIGNUMS_EQUAL("-random (mod power_of_two), non-negative",
                           expected.get(), actual.get());
    }
  }
}

static const int kPrimes[] = {
    2,     3,     5,     7,     11,    13,    17,    19,    23,    29,    31,
    37,    41,    43,    47,    53,    59,    61,    67,    71,    73,    79,
    83,    89,    97,    101,   103,   107,   109,   113,   127,   131,   137,
    139,   149,   151,   157,   163,   167,   173,   179,   181,   191,   193,
    197,   199,   211,   223,   227,   229,   233,   239,   241,   251,   257,
    263,   269,   271,   277,   281,   283,   293,   307,   311,   313,   317,
    331,   337,   347,   349,   353,   359,   367,   373,   379,   383,   389,
    397,   401,   409,   419,   421,   431,   433,   439,   443,   449,   457,
    461,   463,   467,   479,   487,   491,   499,   503,   509,   521,   523,
    541,   547,   557,   563,   569,   571,   577,   587,   593,   599,   601,
    607,   613,   617,   619,   631,   641,   643,   647,   653,   659,   661,
    673,   677,   683,   691,   701,   709,   719,   727,   733,   739,   743,
    751,   757,   761,   769,   773,   787,   797,   809,   811,   821,   823,
    827,   829,   839,   853,   857,   859,   863,   877,   881,   883,   887,
    907,   911,   919,   929,   937,   941,   947,   953,   967,   971,   977,
    983,   991,   997,   1009,  1013,  1019,  1021,  1031,  1033,  1039,  1049,
    1051,  1061,  1063,  1069,  1087,  1091,  1093,  1097,  1103,  1109,  1117,
    1123,  1129,  1151,  1153,  1163,  1171,  1181,  1187,  1193,  1201,  1213,
    1217,  1223,  1229,  1231,  1237,  1249,  1259,  1277,  1279,  1283,  1289,
    1291,  1297,  1301,  1303,  1307,  1319,  1321,  1327,  1361,  1367,  1373,
    1381,  1399,  1409,  1423,  1427,  1429,  1433,  1439,  1447,  1451,  1453,
    1459,  1471,  1481,  1483,  1487,  1489,  1493,  1499,  1511,  1523,  1531,
    1543,  1549,  1553,  1559,  1567,  1571,  1579,  1583,  1597,  1601,  1607,
    1609,  1613,  1619,  1621,  1627,  1637,  1657,  1663,  1667,  1669,  1693,
    1697,  1699,  1709,  1721,  1723,  1733,  1741,  1747,  1753,  1759,  1777,
    1783,  1787,  1789,  1801,  1811,  1823,  1831,  1847,  1861,  1867,  1871,
    1873,  1877,  1879,  1889,  1901,  1907,  1913,  1931,  1933,  1949,  1951,
    1973,  1979,  1987,  1993,  1997,  1999,  2003,  2011,  2017,  2027,  2029,
    2039,  2053,  2063,  2069,  2081,  2083,  2087,  2089,  2099,  2111,  2113,
    2129,  2131,  2137,  2141,  2143,  2153,  2161,  2179,  2203,  2207,  2213,
    2221,  2237,  2239,  2243,  2251,  2267,  2269,  2273,  2281,  2287,  2293,
    2297,  2309,  2311,  2333,  2339,  2341,  2347,  2351,  2357,  2371,  2377,
    2381,  2383,  2389,  2393,  2399,  2411,  2417,  2423,  2437,  2441,  2447,
    2459,  2467,  2473,  2477,  2503,  2521,  2531,  2539,  2543,  2549,  2551,
    2557,  2579,  2591,  2593,  2609,  2617,  2621,  2633,  2647,  2657,  2659,
    2663,  2671,  2677,  2683,  2687,  2689,  2693,  2699,  2707,  2711,  2713,
    2719,  2729,  2731,  2741,  2749,  2753,  2767,  2777,  2789,  2791,  2797,
    2801,  2803,  2819,  2833,  2837,  2843,  2851,  2857,  2861,  2879,  2887,
    2897,  2903,  2909,  2917,  2927,  2939,  2953,  2957,  2963,  2969,  2971,
    2999,  3001,  3011,  3019,  3023,  3037,  3041,  3049,  3061,  3067,  3079,
    3083,  3089,  3109,  3119,  3121,  3137,  3163,  3167,  3169,  3181,  3187,
    3191,  3203,  3209,  3217,  3221,  3229,  3251,  3253,  3257,  3259,  3271,
    3299,  3301,  3307,  3313,  3319,  3323,  3329,  3331,  3343,  3347,  3359,
    3361,  3371,  3373,  3389,  3391,  3407,  3413,  3433,  3449,  3457,  3461,
    3463,  3467,  3469,  3491,  3499,  3511,  3517,  3527,  3529,  3533,  3539,
    3541,  3547,  3557,  3559,  3571,  3581,  3583,  3593,  3607,  3613,  3617,
    3623,  3631,  3637,  3643,  3659,  3671,  3673,  3677,  3691,  3697,  3701,
    3709,  3719,  3727,  3733,  3739,  3761,  3767,  3769,  3779,  3793,  3797,
    3803,  3821,  3823,  3833,  3847,  3851,  3853,  3863,  3877,  3881,  3889,
    3907,  3911,  3917,  3919,  3923,  3929,  3931,  3943,  3947,  3967,  3989,
    4001,  4003,  4007,  4013,  4019,  4021,  4027,  4049,  4051,  4057,  4073,
    4079,  4091,  4093,  4099,  4111,  4127,  4129,  4133,  4139,  4153,  4157,
    4159,  4177,  4201,  4211,  4217,  4219,  4229,  4231,  4241,  4243,  4253,
    4259,  4261,  4271,  4273,  4283,  4289,  4297,  4327,  4337,  4339,  4349,
    4357,  4363,  4373,  4391,  4397,  4409,  4421,  4423,  4441,  4447,  4451,
    4457,  4463,  4481,  4483,  4493,  4507,  4513,  4517,  4519,  4523,  4547,
    4549,  4561,  4567,  4583,  4591,  4597,  4603,  4621,  4637,  4639,  4643,
    4649,  4651,  4657,  4663,  4673,  4679,  4691,  4703,  4721,  4723,  4729,
    4733,  4751,  4759,  4783,  4787,  4789,  4793,  4799,  4801,  4813,  4817,
    4831,  4861,  4871,  4877,  4889,  4903,  4909,  4919,  4931,  4933,  4937,
    4943,  4951,  4957,  4967,  4969,  4973,  4987,  4993,  4999,  5003,  5009,
    5011,  5021,  5023,  5039,  5051,  5059,  5077,  5081,  5087,  5099,  5101,
    5107,  5113,  5119,  5147,  5153,  5167,  5171,  5179,  5189,  5197,  5209,
    5227,  5231,  5233,  5237,  5261,  5273,  5279,  5281,  5297,  5303,  5309,
    5323,  5333,  5347,  5351,  5381,  5387,  5393,  5399,  5407,  5413,  5417,
    5419,  5431,  5437,  5441,  5443,  5449,  5471,  5477,  5479,  5483,  5501,
    5503,  5507,  5519,  5521,  5527,  5531,  5557,  5563,  5569,  5573,  5581,
    5591,  5623,  5639,  5641,  5647,  5651,  5653,  5657,  5659,  5669,  5683,
    5689,  5693,  5701,  5711,  5717,  5737,  5741,  5743,  5749,  5779,  5783,
    5791,  5801,  5807,  5813,  5821,  5827,  5839,  5843,  5849,  5851,  5857,
    5861,  5867,  5869,  5879,  5881,  5897,  5903,  5923,  5927,  5939,  5953,
    5981,  5987,  6007,  6011,  6029,  6037,  6043,  6047,  6053,  6067,  6073,
    6079,  6089,  6091,  6101,  6113,  6121,  6131,  6133,  6143,  6151,  6163,
    6173,  6197,  6199,  6203,  6211,  6217,  6221,  6229,  6247,  6257,  6263,
    6269,  6271,  6277,  6287,  6299,  6301,  6311,  6317,  6323,  6329,  6337,
    6343,  6353,  6359,  6361,  6367,  6373,  6379,  6389,  6397,  6421,  6427,
    6449,  6451,  6469,  6473,  6481,  6491,  6521,  6529,  6547,  6551,  6553,
    6563,  6569,  6571,  6577,  6581,  6599,  6607,  6619,  6637,  6653,  6659,
    6661,  6673,  6679,  6689,  6691,  6701,  6703,  6709,  6719,  6733,  6737,
    6761,  6763,  6779,  6781,  6791,  6793,  6803,  6823,  6827,  6829,  6833,
    6841,  6857,  6863,  6869,  6871,  6883,  6899,  6907,  6911,  6917,  6947,
    6949,  6959,  6961,  6967,  6971,  6977,  6983,  6991,  6997,  7001,  7013,
    7019,  7027,  7039,  7043,  7057,  7069,  7079,  7103,  7109,  7121,  7127,
    7129,  7151,  7159,  7177,  7187,  7193,  7207,  7211,  7213,  7219,  7229,
    7237,  7243,  7247,  7253,  7283,  7297,  7307,  7309,  7321,  7331,  7333,
    7349,  7351,  7369,  7393,  7411,  7417,  7433,  7451,  7457,  7459,  7477,
    7481,  7487,  7489,  7499,  7507,  7517,  7523,  7529,  7537,  7541,  7547,
    7549,  7559,  7561,  7573,  7577,  7583,  7589,  7591,  7603,  7607,  7621,
    7639,  7643,  7649,  7669,  7673,  7681,  7687,  7691,  7699,  7703,  7717,
    7723,  7727,  7741,  7753,  7757,  7759,  7789,  7793,  7817,  7823,  7829,
    7841,  7853,  7867,  7873,  7877,  7879,  7883,  7901,  7907,  7919,  7927,
    7933,  7937,  7949,  7951,  7963,  7993,  8009,  8011,  8017,  8039,  8053,
    8059,  8069,  8081,  8087,  8089,  8093,  8101,  8111,  8117,  8123,  8147,
    8161,  8167,  8171,  8179,  8191,  8209,  8219,  8221,  8231,  8233,  8237,
    8243,  8263,  8269,  8273,  8287,  8291,  8293,  8297,  8311,  8317,  8329,
    8353,  8363,  8369,  8377,  8387,  8389,  8419,  8423,  8429,  8431,  8443,
    8447,  8461,  8467,  8501,  8513,  8521,  8527,  8537,  8539,  8543,  8563,
    8573,  8581,  8597,  8599,  8609,  8623,  8627,  8629,  8641,  8647,  8663,
    8669,  8677,  8681,  8689,  8693,  8699,  8707,  8713,  8719,  8731,  8737,
    8741,  8747,  8753,  8761,  8779,  8783,  8803,  8807,  8819,  8821,  8831,
    8837,  8839,  8849,  8861,  8863,  8867,  8887,  8893,  8923,  8929,  8933,
    8941,  8951,  8963,  8969,  8971,  8999,  9001,  9007,  9011,  9013,  9029,
    9041,  9043,  9049,  9059,  9067,  9091,  9103,  9109,  9127,  9133,  9137,
    9151,  9157,  9161,  9173,  9181,  9187,  9199,  9203,  9209,  9221,  9227,
    9239,  9241,  9257,  9277,  9281,  9283,  9293,  9311,  9319,  9323,  9337,
    9341,  9343,  9349,  9371,  9377,  9391,  9397,  9403,  9413,  9419,  9421,
    9431,  9433,  9437,  9439,  9461,  9463,  9467,  9473,  9479,  9491,  9497,
    9511,  9521,  9533,  9539,  9547,  9551,  9587,  9601,  9613,  9619,  9623,
    9629,  9631,  9643,  9649,  9661,  9677,  9679,  9689,  9697,  9719,  9721,
    9733,  9739,  9743,  9749,  9767,  9769,  9781,  9787,  9791,  9803,  9811,
    9817,  9829,  9833,  9839,  9851,  9857,  9859,  9871,  9883,  9887,  9901,
    9907,  9923,  9929,  9931,  9941,  9949,  9967,  9973,  10007, 10009, 10037,
    10039, 10061, 10067, 10069, 10079, 10091, 10093, 10099, 10103, 10111, 10133,
    10139, 10141, 10151, 10159, 10163, 10169, 10177, 10181, 10193, 10211, 10223,
    10243, 10247, 10253, 10259, 10267, 10271, 10273, 10289, 10301, 10303, 10313,
    10321, 10331, 10333, 10337, 10343, 10357, 10369, 10391, 10399, 10427, 10429,
    10433, 10453, 10457, 10459, 10463, 10477, 10487, 10499, 10501, 10513, 10529,
    10531, 10559, 10567, 10589, 10597, 10601, 10607, 10613, 10627, 10631, 10639,
    10651, 10657, 10663, 10667, 10687, 10691, 10709, 10711, 10723, 10729, 10733,
    10739, 10753, 10771, 10781, 10789, 10799, 10831, 10837, 10847, 10853, 10859,
    10861, 10867, 10883, 10889, 10891, 10903, 10909, 10937, 10939, 10949, 10957,
    10973, 10979, 10987, 10993, 11003, 11027, 11047, 11057, 11059, 11069, 11071,
    11083, 11087, 11093, 11113, 11117, 11119, 11131, 11149, 11159, 11161, 11171,
    11173, 11177, 11197, 11213, 11239, 11243, 11251, 11257, 11261, 11273, 11279,
    11287, 11299, 11311, 11317, 11321, 11329, 11351, 11353, 11369, 11383, 11393,
    11399, 11411, 11423, 11437, 11443, 11447, 11467, 11471, 11483, 11489, 11491,
    11497, 11503, 11519, 11527, 11549, 11551, 11579, 11587, 11593, 11597, 11617,
    11621, 11633, 11657, 11677, 11681, 11689, 11699, 11701, 11717, 11719, 11731,
    11743, 11777, 11779, 11783, 11789, 11801, 11807, 11813, 11821, 11827, 11831,
    11833, 11839, 11863, 11867, 11887, 11897, 11903, 11909, 11923, 11927, 11933,
    11939, 11941, 11953, 11959, 11969, 11971, 11981, 11987, 12007, 12011, 12037,
    12041, 12043, 12049, 12071, 12073, 12097, 12101, 12107, 12109, 12113, 12119,
    12143, 12149, 12157, 12161, 12163, 12197, 12203, 12211, 12227, 12239, 12241,
    12251, 12253, 12263, 12269, 12277, 12281, 12289, 12301, 12323, 12329, 12343,
    12347, 12373, 12377, 12379, 12391, 12401, 12409, 12413, 12421, 12433, 12437,
    12451, 12457, 12473, 12479, 12487, 12491, 12497, 12503, 12511, 12517, 12527,
    12539, 12541, 12547, 12553, 12569, 12577, 12583, 12589, 12601, 12611, 12613,
    12619, 12637, 12641, 12647, 12653, 12659, 12671, 12689, 12697, 12703, 12713,
    12721, 12739, 12743, 12757, 12763, 12781, 12791, 12799, 12809, 12821, 12823,
    12829, 12841, 12853, 12889, 12893, 12899, 12907, 12911, 12917, 12919, 12923,
    12941, 12953, 12959, 12967, 12973, 12979, 12983, 13001, 13003, 13007, 13009,
    13033, 13037, 13043, 13049, 13063, 13093, 13099, 13103, 13109, 13121, 13127,
    13147, 13151, 13159, 13163, 13171, 13177, 13183, 13187, 13217, 13219, 13229,
    13241, 13249, 13259, 13267, 13291, 13297, 13309, 13313, 13327, 13331, 13337,
    13339, 13367, 13381, 13397, 13399, 13411, 13417, 13421, 13441, 13451, 13457,
    13463, 13469, 13477, 13487, 13499, 13513, 13523, 13537, 13553, 13567, 13577,
    13591, 13597, 13613, 13619, 13627, 13633, 13649, 13669, 13679, 13681, 13687,
    13691, 13693, 13697, 13709, 13711, 13721, 13723, 13729, 13751, 13757, 13759,
    13763, 13781, 13789, 13799, 13807, 13829, 13831, 13841, 13859, 13873, 13877,
    13879, 13883, 13901, 13903, 13907, 13913, 13921, 13931, 13933, 13963, 13967,
    13997, 13999, 14009, 14011, 14029, 14033, 14051, 14057, 14071, 14081, 14083,
    14087, 14107, 14143, 14149, 14153, 14159, 14173, 14177, 14197, 14207, 14221,
    14243, 14249, 14251, 14281, 14293, 14303, 14321, 14323, 14327, 14341, 14347,
    14369, 14387, 14389, 14401, 14407, 14411, 14419, 14423, 14431, 14437, 14447,
    14449, 14461, 14479, 14489, 14503, 14519, 14533, 14537, 14543, 14549, 14551,
    14557, 14561, 14563, 14591, 14593, 14621, 14627, 14629, 14633, 14639, 14653,
    14657, 14669, 14683, 14699, 14713, 14717, 14723, 14731, 14737, 14741, 14747,
    14753, 14759, 14767, 14771, 14779, 14783, 14797, 14813, 14821, 14827, 14831,
    14843, 14851, 14867, 14869, 14879, 14887, 14891, 14897, 14923, 14929, 14939,
    14947, 14951, 14957, 14969, 14983, 15013, 15017, 15031, 15053, 15061, 15073,
    15077, 15083, 15091, 15101, 15107, 15121, 15131, 15137, 15139, 15149, 15161,
    15173, 15187, 15193, 15199, 15217, 15227, 15233, 15241, 15259, 15263, 15269,
    15271, 15277, 15287, 15289, 15299, 15307, 15313, 15319, 15329, 15331, 15349,
    15359, 15361, 15373, 15377, 15383, 15391, 15401, 15413, 15427, 15439, 15443,
    15451, 15461, 15467, 15473, 15493, 15497, 15511, 15527, 15541, 15551, 15559,
    15569, 15581, 15583, 15601, 15607, 15619, 15629, 15641, 15643, 15647, 15649,
    15661, 15667, 15671, 15679, 15683, 15727, 15731, 15733, 15737, 15739, 15749,
    15761, 15767, 15773, 15787, 15791, 15797, 15803, 15809, 15817, 15823, 15859,
    15877, 15881, 15887, 15889, 15901, 15907, 15913, 15919, 15923, 15937, 15959,
    15971, 15973, 15991, 16001, 16007, 16033, 16057, 16061, 16063, 16067, 16069,
    16073, 16087, 16091, 16097, 16103, 16111, 16127, 16139, 16141, 16183, 16187,
    16189, 16193, 16217, 16223, 16229, 16231, 16249, 16253, 16267, 16273, 16301,
    16319, 16333, 16339, 16349, 16361, 16363, 16369, 16381, 16411, 16417, 16421,
    16427, 16433, 16447, 16451, 16453, 16477, 16481, 16487, 16493, 16519, 16529,
    16547, 16553, 16561, 16567, 16573, 16603, 16607, 16619, 16631, 16633, 16649,
    16651, 16657, 16661, 16673, 16691, 16693, 16699, 16703, 16729, 16741, 16747,
    16759, 16763, 16787, 16811, 16823, 16829, 16831, 16843, 16871, 16879, 16883,
    16889, 16901, 16903, 16921, 16927, 16931, 16937, 16943, 16963, 16979, 16981,
    16987, 16993, 17011, 17021, 17027, 17029, 17033, 17041, 17047, 17053, 17077,
    17093, 17099, 17107, 17117, 17123, 17137, 17159, 17167, 17183, 17189, 17191,
    17203, 17207, 17209, 17231, 17239, 17257, 17291, 17293, 17299, 17317, 17321,
    17327, 17333, 17341, 17351, 17359, 17377, 17383, 17387, 17389, 17393, 17401,
    17417, 17419, 17431, 17443, 17449, 17467, 17471, 17477, 17483, 17489, 17491,
    17497, 17509, 17519, 17539, 17551, 17569, 17573, 17579, 17581, 17597, 17599,
    17609, 17623, 17627, 17657, 17659, 17669, 17681, 17683, 17707, 17713, 17729,
    17737, 17747, 17749, 17761, 17783, 17789, 17791, 17807, 17827, 17837, 17839,
    17851, 17863, 17881, 17891, 17903, 17909, 17911, 17921, 17923, 17929, 17939,
    17957, 17959, 17971, 17977, 17981, 17987, 17989, 18013, 18041, 18043, 18047,
    18049, 18059, 18061, 18077, 18089, 18097, 18119, 18121, 18127, 18131, 18133,
    18143, 18149, 18169, 18181, 18191, 18199, 18211, 18217, 18223, 18229, 18233,
    18251, 18253, 18257, 18269, 18287, 18289, 18301, 18307, 18311, 18313, 18329,
    18341, 18353, 18367, 18371, 18379, 18397, 18401, 18413, 18427, 18433, 18439,
    18443, 18451, 18457, 18461, 18481, 18493, 18503, 18517, 18521, 18523, 18539,
    18541, 18553, 18583, 18587, 18593, 18617, 18637, 18661, 18671, 18679, 18691,
    18701, 18713, 18719, 18731, 18743, 18749, 18757, 18773, 18787, 18793, 18797,
    18803, 18839, 18859, 18869, 18899, 18911, 18913, 18917, 18919, 18947, 18959,
    18973, 18979, 19001, 19009, 19013, 19031, 19037, 19051, 19069, 19073, 19079,
    19081, 19087, 19121, 19139, 19141, 19157, 19163, 19181, 19183, 19207, 19211,
    19213, 19219, 19231, 19237, 19249, 19259, 19267, 19273, 19289, 19301, 19309,
    19319, 19333, 19373, 19379, 19381, 19387, 19391, 19403, 19417, 19421, 19423,
    19427, 19429, 19433, 19441, 19447, 19457, 19463, 19469, 19471, 19477, 19483,
    19489, 19501, 19507, 19531, 19541, 19543, 19553, 19559, 19571, 19577, 19583,
    19597, 19603, 19609, 19661, 19681, 19687, 19697, 19699, 19709, 19717, 19727,
    19739, 19751, 19753, 19759, 19763, 19777, 19793, 19801, 19813, 19819, 19841,
    19843, 19853, 19861, 19867, 19889, 19891, 19913, 19919, 19927, 19937, 19949,
    19961, 19963, 19973, 19979, 19991, 19993, 19997,
};

TEST_F(BNTest, PrimeChecking) {
  bssl::UniquePtr<BIGNUM> p(BN_new());
  ASSERT_TRUE(p);
  int is_probably_prime_1 = 0, is_probably_prime_2 = 0;
  enum bn_primality_result_t result_3;

  const int max_prime = kPrimes[OPENSSL_ARRAY_SIZE(kPrimes)-1];
  size_t next_prime_index = 0;

  for (int i = 0; i <= max_prime; i++) {
    SCOPED_TRACE(i);
    bool is_prime = false;

    if (i == kPrimes[next_prime_index]) {
      is_prime = true;
      next_prime_index++;
    }

    ASSERT_TRUE(BN_set_word(p.get(), i));
    ASSERT_TRUE(BN_primality_test(
        &is_probably_prime_1, p.get(), BN_prime_checks_for_generation, ctx(),
        false /* do_trial_division */, nullptr /* callback */));
    EXPECT_EQ(is_prime ? 1 : 0, is_probably_prime_1);
    ASSERT_TRUE(BN_primality_test(
        &is_probably_prime_2, p.get(), BN_prime_checks_for_generation, ctx(),
        true /* do_trial_division */, nullptr /* callback */));
    EXPECT_EQ(is_prime ? 1 : 0, is_probably_prime_2);
    if (i > 3 && i % 2 == 1) {
      ASSERT_TRUE(BN_enhanced_miller_rabin_primality_test(
          &result_3, p.get(), BN_prime_checks_for_generation, ctx(),
          nullptr /* callback */));
      EXPECT_EQ(is_prime, result_3 == bn_probably_prime);
    }
  }

  // Negative numbers are not prime.
  ASSERT_TRUE(BN_set_word(p.get(), 7));
  BN_set_negative(p.get(), 1);
  ASSERT_TRUE(BN_primality_test(
      &is_probably_prime_1, p.get(), BN_prime_checks_for_generation, ctx(),
      false /* do_trial_division */, nullptr /* callback */));
  EXPECT_EQ(0, is_probably_prime_1);
  ASSERT_TRUE(BN_primality_test(
      &is_probably_prime_2, p.get(), BN_prime_checks_for_generation, ctx(),
      true /* do_trial_division */, nullptr /* callback */));
  EXPECT_EQ(0, is_probably_prime_2);

  static const char *kComposites[] = {
      // The following composite numbers come from http://oeis.org/A014233 and
      // are such that the first several primes are not a Miller-Rabin composite
      // witness.
      "2047",
      "1373653",
      "25326001",
      "3215031751",
      "2152302898747",
      "3474749660383",
      "341550071728321",
      "3825123056546413051",
      "318665857834031151167461",
      "3317044064679887385961981",

      // The following composite numbers come from https://oeis.org/A033181
      // which lists Euler pseudoprimes. These are false positives for the
      // Fermat primality
      // test.
      "1729",
      "2465",
      "15841",
      "41041",
      "46657",
      "75361",
      "162401",
      "172081",
      "399001",
      "449065",
      "488881",
      "530881",
      "656601",
      "670033",
      "838201",
      "997633",
      "1050985",
      "1615681",
      "1773289",
      "1857241",
      "2113921",
      "2433601",
      "2455921",
      "2704801",
      "3057601",
      "3224065",
      "3581761",
      "3664585",
      "3828001",
      "4463641",
      "4903921",
  };
  for (const char *str : kComposites) {
    SCOPED_TRACE(str);
    EXPECT_NE(0, DecimalToBIGNUM(&p, str));

    ASSERT_TRUE(BN_primality_test(
        &is_probably_prime_1, p.get(), BN_prime_checks_for_generation, ctx(),
        false /* do_trial_division */, nullptr /* callback */));
    EXPECT_EQ(0, is_probably_prime_1);

    ASSERT_TRUE(BN_primality_test(
        &is_probably_prime_2, p.get(), BN_prime_checks_for_generation, ctx(),
        true /* do_trial_division */, nullptr /* callback */));
    EXPECT_EQ(0, is_probably_prime_2);

    ASSERT_TRUE(BN_enhanced_miller_rabin_primality_test(
        &result_3, p.get(), BN_prime_checks_for_generation, ctx(),
        nullptr /* callback */));
    EXPECT_EQ(bn_composite, result_3);
  }

  static const char *kPrimesHex[] = {
      // Various primes extracted from openssl genrsa:
      // 512-bit primes.
      "ebb00348b1308e29166f0401f7415cc3bf9c746460bcadfd1ad6838b6472f48f3afba0c1"
      "446eddc4708c68e307a882771794fbba45799f5b062e090613ee8203",
      "d9a896e15c5d0091e81825948f3111c615a32aa0bd9305b9591232138388176fe22ff765"
      "63c893b95c0f9898029be67543144c5e76c837333f109a0ffc0fa3db",
      "fdecb71e997f234111706cabdfdc515b7e7a2a8d77b3c3a4b4819493d39de84e791be692"
      "9ce1c3f5136808504f351eca19884894f581f96fba2b8d652265efe9",
      "dc37a778aa89eb4048267573421ac5b9d81a231d05191393bdf06a6a64c684968fd17c4f"
      "41fbd5745df2ee447fcc04693e2e3fecec270145388032149da63b3f",
      "fbf34841baa2dd4ecf9055328f4902532d80e82f6d8ea186311564b3680b39ea2162fed4"
      "701f02bec9d5be19f2e505c58a68620ee8873e8ab8fe98506a8bf9bb",
      "c3b3c3156c9d0bf3b27f9bf8274ddc8c8505bacbb4a9595d90354d1a472553d6ae3daa97"
      "1396c0361f6355531de29bf8ef1d7b471b5f2267d4b49cbe48ced5f1",
      "f8d1216de820efb437ca8070c5f4f34838c46cf354c998e253557cfc400eae7883d0a758"
      "0b2e617cca527d9d6c598cbc03ca743791f88a5a065fea9583068f1b",
      "cc12d224273b56e6765f6b42583d8da3c89ff531f14961351b5173a9017579cd7bb736e2"
      "78e626a426ee5a583b8d6c7b3006687ca9df596902a281e9e9cf3ad5",

      // 1024-bit primes.
      "f3244013a1b0ec2fe53a684260077d2afc3b35ed77026c594091d92b2eb47fd1266095b8"
      "7456cc451942f907079b8a9cd333d4bf22a892dbc632904a6423c5b19bb41fd43764a558"
      "0e9a5960d84fadbebfbbfaa5ec39acb78a94937d11d7a62c54a0f983bc8b5507479290de"
      "f4e979d3f24ce81f4c506ba3bfca4f402a3b11cf",
      "e4a70bdbb96fefd5732e9e94f9d04b9ef16635642ee728d40626861db00d57950697e892"
      "d0306de25ee35d5ccce1220e1b19fd2f98af2fdcac5796d860fd75aec31ed48baf5b39cf"
      "77ebda6727e33e6f72735ab0121395deb54fd430212499043cd1e11f7d5852f146997952"
      "d9959c83542b6cbad3c3a2ebb8698a0172e0c6d1",
      "e85ad4595ea74bf886977f4a06120b6ae28ec2d7ee44b4bc8658a8a90a2a55311814dfed"
      "ebd08f93e8241dcc87d91d6f6b498c6ec0576a7dad6e5d53b71f89fb985de290c0f02a78"
      "f2143217c0b7ae1487a751ec27dfbd46046a06f5ebe337e05ed5d6fe8620b7f82b349c37"
      "924d96128e42307fd708a74d608848cbdf6bc799",
      "cc890f5fe88bfc4028a2ab5eff9dea7b150ffe75fb29f1904adb4709e86f74eaed44218c"
      "d8058341a4b828d4fefeed5e34f50198bf643040037933f4305e1e01c3518279b9fa4131"
      "e5afbc462efe9b5ddc4ab91ec2c12abf95b526bb2a6bd7b2bb1ce8203364502f7c3b87ff"
      "585c94765505c20f728078a46759615ad23d4fb7",
      "ebd8cd32804c6c1e7264de4f9bf1e4d2dbdaa23292c8f4688aa2770f664fe03513974e13"
      "0a10ccc6b6ca95846dfecbd2d42285cf0212ff427ddb7cc222bfa459215ad4cc0f1f5fc7"
      "4186bbbe96ca4de0d7c793ee050f8e10a242ab9bf03aae5b017b42c405ccee34f59ff501"
      "5dbe4cab310bbb3ab50604f663cdb5af070d4a8d",
      "e1dab2efc6ba8c980b86164e11fc6c6c4abb53701031de431db2b608ec75fd03c7cf07e6"
      "e9d6c36da2a2aafe759f9c3e1522237d4dcae66ef03c86481428d58d4bcdffb919bb8da4"
      "4b0ac1cc922d2d904c543b1a09961faf7304af4482dc839091b258523ab5e36302e1157f"
      "3e6810513922c5d5c1f559e3a90b91e4cf2f0c9f",
      "d76a082eb03584a6253555cf9813206a06c9fc2112b6425e030f12d7d807656175f4c58e"
      "e367826ec0d89f03339fb520d7c8a735905e458f849827581e9db22fde302fc55db031fd"
      "8f3afe1910eaaa8ed4d122de99fa0a66bf69b932ce84d095ffcb3f98e231199817ebc316"
      "460df0c0769fef3f91777a9cf86ccf2e8233818b",
      "d506fd2c6557a7f8cd0ac8f0f098bffdede4ee79f74ce6e9478d8651058ec56aa1f4683c"
      "20729ee8d11d14b34170ce0cf419a7b22943d5fb443afb22e6a430fe993ac64737428f50"
      "37d19398ee226484b5ca64af71012245d87aefbcbd71e867f6fbcc52e0e1c49f1363aec1"
      "88c776abb67cda2fd6ce7be4bdbeee57fbafb07b",

      // 1536-bit primes.
      "f6aa5b151ea2cd151a720174d58c157e8dbbf3dbd93b102fcfb7ad3767cca8543d4fb168"
      "7fb907561da1330c7878853859bc2b4b9d639d9b9bba4fce3a95cfd9151c19365e6ad634"
      "7edc87acd4b79d2a7ce942c2a391c475cef2d4e347675487cc36a43f157562e32aff9d74"
      "e15f228a0ecc8eca2392e04ddea8eda995789c94b9f85dde65e66b074c7843260ebdcd60"
      "1cd49e2bf3ab83780281e4a56ada38b16e085f00c05bcce442daf1c9374a3ec2a2345309"
      "5570aaa6bb3a3e4945312aed",
      "e396e3ede4b0a33fe90b749b3dbc01fdb7d15e37cc3febe3f2b0ee6140204666fa4acb93"
      "da893d0ce19d9e5eb09b7395394ced79261ba8b1a40ee977d1954a98031256c0e3f83c5b"
      "ee234afddb80d4251b5f6f7493b3eb6156011e202fd4d8319445eb5bb3c0782e9e75077c"
      "87f9f3a25a2d117793fc98441ce74255d7bd55bdb0f17710737ab4aaca99271600f03503"
      "91ffbc9a5d5458414716e0c26b239096f6c6e4a680b0cccaebc4f200fa0500618d719493"
      "becacf936525680233273679",
      "e5e7d43632d844bd04fce45213257415a4c9c3f4bf9b6a1b74e8c31e3c66fbf3b42da531"
      "aaa9cdaac160d565cd81430983c18120e98be41df6d178d0e974cc9ce6ced673423c7727"
      "267ba1ba07b457a1557bffaf2c90957372c0f5f08c4940ccd858e0bc392e3050bb2adae8"
      "0f509dc129a49279c01c55434b383d359b7b255f55c33be445a3dc05e0c1b3d7486a8142"
      "675a3b6e7b3d3d27fbf54764d9f73ea98304612e5e1a4d566986efa53b62ad18f4ecad64"
      "f197c7d48a2732745a1e5ec9",
      "daa7795c70b8df8af978f9e66a19eed2a92b6f665aee3d58f3e450ac0f18772ed5cf8b2b"
      "381eb55facd93b32106d0d703f2316b50069b6db38cd62b12a4b7fdd6f8f93c4f110091a"
      "d972e5808afd6acf6bd6eaa0b846b50b7fe1786702a3382b8b637b8ea91ffe3225e9ad50"
      "3f1f9593ea6f19d6dc2d556e5d6f3a26134df4a964e67d789e7849eaf698c976ef592052"
      "6b023f2f96e96e2b89adf0ee4544e32029cfca972f824cb7af805c556a6143dcb93cb6b7"
      "91ebb8dba30cbc94dff782f3",
      "f48f534acee47a482ba43abc70aa8c7d4b6df27b957583fa2b23cbc1d34d9da7eb89fa3f"
      "881b9db1dfa8925f38328574ca8ff7256ae0bf163ee61b471d29f5e72d98f92775693091"
      "2bfbddb695a64137783232596d6c7892b89b4fb54abd5b077ccf532aaf5b9b29cf25b366"
      "3845987a0a947b97000c05bfc7a239e1cb962cc43e1dceaf91935353d2d6dad7eda20798"
      "9a2f0f8e367f3df5c1ee3b56209bd85832c35ff2cd7b9a67db801691c946b0a7a9a875e8"
      "9e1f65198caf1ca6f3037ff9",
      "ee5bc8c8d3ecd753b4c0e4e5934d8e44a9ab5d8dda127db28b32bfb357636d0c144dee78"
      "8c2a901af3b02439a8a3d2125954feeac722a72272f5595a91cf4ee5ae8e69159986cc50"
      "054c3a259c80ed84e7b793733eed05330b2a2ad11dee4140b5fe1f3706a0b1b28407e84c"
      "27e19e3a3d9d640629c35deaa9061d33b5888a88e4220340f488f764219f9e8edb2b1d04"
      "15253f5fd53835cdc6935898ecba173c5b2db3a6578fdc16e1221cac1e454864ada9f772"
      "1ecd24bc77ed5cf353d5f909",
      "f2f5ca816781cbae4fcea9587321497c252bfe84127f2d8ac7d6da7a34d1faa2f428911d"
      "a876a42299d2cb4af35c944df51f1421b74fe11b047f871b37f1f37a0c6d0753c28a3e52"
      "91a9cf54c5892408591bc932269626d1392f8c8c67d87300febbc63e4a779104ba6191f8"
      "a5bbfbcf6c675a6ad8a853ac1e9a86dc16a95a9566b5287b7862f6a962bf79626a82961f"
      "c378b4751da35e25d761469ad4e22072bd43951631a96026b37d7932ca8fabf22fb757b4"
      "e903252c416f0f96ca0eb663",
      "e01c620e4b80840816a99b5c1eed80c8bfdc040253889b2ce81e78de2f5511ea453d1492"
      "56bb53b64f4f43441e464867cfd40571c2c5527f1c79eb4b8b1022018e362ae51f13b8b5"
      "2426239c09369370575d873755e3bee630424e35a8024f76553f5635d26d791b5e4a8903"
      "d09be560c322837c29283aee2feb6864b724007334f1af2008db7eaf773d9f4e1e8fc396"
      "07969c43d7c1d106274fa24c3068d347244d5821e10153b5e1e84fef7c08c19e4f79b71e"
      "ebd1205c057812a74f6e09ab",

      // 2048-bit primes.
      "ff9166fd6945a3f692e99001528d5f4db6a36990f755275c3b34bded64bdd9c8e0cd190b"
      "3df421be41525d496478bb2c07400ea1abe2bda65aa95efaecfada8230df64405ace2594"
      "3193755ecf24db8fe8cda7a399cebe66f6d760cd9815bdcc65a5ad53c5b97dad21deed9b"
      "e24ba048f621a095b3ffc48d05de12e16fb53d1e81ba0ed20c601599ce3833c7f36bc481"
      "ab84ba7f38e3baeb19ad27e45dfd74fd5d03073426200c4b5ebf3323b3e16a0534b8df9b"
      "0359c8e56f2e8c3950803b28954f8b6f14cee76623481f3479638c4908ce88ee56a5940b"
      "c9e79198fedf83e5f931740346916d745c6279f13f4ca59e1534dba4f3eaeee8d20ddf20"
      "6459fac7",
      "d17eaffaad2b87da90b280b3879908ef3ed395b0d7cf12daee62dd4a0bf73e536f912635"
      "f109908c8ceb26f31950dbcce65e443e452ac0eddf35aef2ae03a15f57bbb5d7800c9d61"
      "bae6d87f10927643bd5a2cd77bd5a70d84b0da28494e5cb7cd7ced9dd0a57177cade57d9"
      "53c80efa99ff09588dc7f6cab76d18fc86ccfc74fe5acca9aba2b4c143977d7abdae2a67"
      "7cb50810f6b60ccc0f77f75e9ea5733d8c7d6795f95350d91fafacd9d9ad00bafaadf558"
      "d95237ff53f090c674c326f38f728dbc4a42f2978d91c19686f3793862375adb2bc8b241"
      "ce9816e8e36ff105bb06e7a77ea0077371b28bbdf745dd0bf537e43a0bed8ddeff5eb29e"
      "28931d17",
      "df859ae517fac8682a715f666c70ad29421cb8a0186fe6016c5bd8a0fabf65ee2b018fcc"
      "53c50a29daf82a2a9f7bceac45c13a2458af34998cf16eecec02fe3254758eff63b60e25"
      "3e118fb1494d78de1d38b49ac0b528a04208d2b57d95a9edd7b7b02afeb2c47a628bef6b"
      "4a6a0f7b91cb5b8d5900f8ad3f332360a07f3ac00907cadfe6cacc7e696e897ca541a2e7"
      "12a5d419215712716b71e2a2a8b8c809bbf0cc3b24e55e7ec72cfdc5e8c9651f8a2f36a8"
      "abd0ebd77ddf59b7f096b788f8081e22465e4a6082c3ad4bcdf27bf5f51f3326eb87ac9e"
      "330fb6d68645299da63a1d977fb246e176afcfbc2474fca3ae40d75125f755f5a50c3080"
      "e7816235",
      "c6aea46d1fb7d2d1107e31399cc613a1db56174c96898e3e32688ce2a26c000486528f05"
      "4cc0dc3e448016944528183a2a90ca54a1029aedc519fe6d7b599097b214aab0d16b35cb"
      "b7948e2e301f4fe65fc35340a82eb25111150cd968e12ec063ac0901ec4bf5d490a39714"
      "b128848ee3852dce7bfdd66a4751abe8f365d1e83fd7a86a192d02bc892c6cd9558bacdf"
      "c55a61cb06be8d74c44c2d03245d9b5f003c7280e82f3f1204dc7abc3e5fa11f2168bc17"
      "c73fb1dc8b84e632a26420b32118fc8aa6a98c037b662d676370d10bfb47955e9b4f4c64"
      "062d32345677199b36abe1d6b1bb0badbb57ae4a65b643da7f122c1b38dad9df0318d3c9"
      "d96a96bf",
      "da64c031f133da1d014777b6f8c8d599f54b7e67dc3ac3883f0b78cfe27d1cb1849c72a3"
      "37a6d6a0ee53633c8382a416e8851fe9c81141121d702fa8b12dc6ba62a3dbb87faec66c"
      "6389e9e1df47015db6ff12ded83d2fc242e58e55cf7924b70e4cf463559705e382745006"
      "1aa88b38d3795042ab0e8657ed1c77e91e39d5a29e86f9572a3ce91b8d0ca12ef6ee5f1f"
      "f3930c5de357eaabe7497d7319461be00cbb1db36329baa6c298608aa7288a6926396abc"
      "9a662dc2c413311ec821cb4564c247fcdd32d57cae8dd37882377f9139aea9a5a6ae1e01"
      "1a356fc395682f64c08cb3130711bb759d16ed2eaf0da976876f156aa0965cb7292a5726"
      "1ad31ab7",
      "ce705e04e5abb0d0f3058bff82c457ef6308f2b4279026c906c0679f382d92c96ae0d11f"
      "3004dfbdfd7950cc4f0aa1bcb7b06e4be6628b249e90339d8e1891e512c40f7b38ce9ad4"
      "ad7c37791b833cf668b4807c2b4d4638cb10af745e349c70ae7bc8396611725c43899131"
      "751729e98651b4250d680ddb1f208e971b8abaca2ba79a7665dd71fa532702f54930865c"
      "52ca536f04218aeb626ff94bc4e0886ffbccba910f879e000f363b0864dfc883d2de2af5"
      "70c2c4125c5b0e478f87f7b934b66af864fb63f4d13fa21db3e4cef03c395fe207764ae3"
      "1b64bbc301cdeb795c580885605b11bcaa53d32a1fa72381e524ef269748ce77deb0cd37"
      "ceb403ab",
      "f4f7bb8ab2983afc83b6ac060dcc4d96331dbbf800b321bbde2d8f8a9fa750e7c2b42fc4"
      "6baf9a167a7389812f65b52b283ad5dd95709e81f8f602031ee8a5f4929bee7b3da97b92"
      "f53f61ff25de8170aeef9a6c464d4be77fa3e5aea041f51d49932d30480f33bb44fd3af5"
      "e7bfad562acaaed5069b2dc003fdb207ee7db9061d02136cb4b59c2ba071ca6aa2747675"
      "bf86d601a9197d92091b36299cad0d6adceca87b16ee54b48ee19a9e9df20955cdc1ca2c"
      "fa07fd2b054377d6242fb1ae69209ac5ac2d98a2929dec9eb076e0c9d74083bab0797851"
      "b6eca68e3de7440001706cebee6adc8b317b0ef8332863aad26ec18f8156998566f32207"
      "3777e817",
      "da20f268b7254f3ed0ad35372ad4c78c1fc89465fc1a256ee0064b3c11980917d4d0b6fe"
      "c8546c5e4cea1e18ccd23f20dc096506062afeb57be9edd2443ec1cecd84108911c99ac0"
      "2d388bc7c415aa41b7a4396c3ed823f3c0921163e85e2dec186862e945affa069dee3dea"
      "3b382d7c5a9695aa76e2e25a516457d4eee12ef0c18bf09076c8f739189887492e4aecae"
      "2999ec305c2e66d444d14251caa1b546deb3c07c6d9c0ed9d1a33f405e780661684be318"
      "61db7030b2f0b5b6e6f1616ab017955a6025c89c6945329aa10567a5f26724dc074cae1a"
      "623c64fcda5241674bb4c9954342b1bac8cb13a4b98e893ee42b4ccebf788c2267de2d70"
      "8a5b93ed",
  };
  for (const char *str : kPrimesHex) {
    SCOPED_TRACE(str);
    EXPECT_NE(0, HexToBIGNUMWithReturn(&p, str));

    ASSERT_TRUE(BN_primality_test(
        &is_probably_prime_1, p.get(), BN_prime_checks_for_generation, ctx(),
        false /* do_trial_division */, nullptr /* callback */));
    EXPECT_EQ(1, is_probably_prime_1);

    ASSERT_TRUE(BN_primality_test(
        &is_probably_prime_2, p.get(), BN_prime_checks_for_generation, ctx(),
        true /* do_trial_division */, nullptr /* callback */));
    EXPECT_EQ(1, is_probably_prime_2);

    ASSERT_TRUE(BN_enhanced_miller_rabin_primality_test(
        &result_3, p.get(), BN_prime_checks_for_generation, ctx(),
        nullptr /* callback */));
    EXPECT_EQ(bn_probably_prime, result_3);
  }

  // BN_primality_test works with null |BN_CTX|.
  ASSERT_TRUE(BN_set_word(p.get(), 5));
  ASSERT_TRUE(
      BN_primality_test(&is_probably_prime_1, p.get(),
                        BN_prime_checks_for_generation, nullptr /* ctx */,
                        false /* do_trial_division */, nullptr /* callback */));
  EXPECT_EQ(1, is_probably_prime_1);
}

TEST_F(BNTest, MillerRabinIteration) {
  FileTestGTest(
      "crypto/fipsmodule/bn/test/miller_rabin_tests.txt", [&](FileTest *t) {
        BIGNUMFileTest bn_test(t, /*large_mask=*/0);

        bssl::UniquePtr<BIGNUM> w = bn_test.GetBIGNUM("W");
        ASSERT_TRUE(w);
        bssl::UniquePtr<BIGNUM> b = bn_test.GetBIGNUM("B");
        ASSERT_TRUE(b);
        bssl::UniquePtr<BN_MONT_CTX> mont(
            BN_MONT_CTX_new_consttime(w.get(), ctx()));
        ASSERT_TRUE(mont);

        bssl::BN_CTXScope scope(ctx());
        BN_MILLER_RABIN miller_rabin;
        ASSERT_TRUE(bn_miller_rabin_init(&miller_rabin, mont.get(), ctx()));
        int possibly_prime;
        ASSERT_TRUE(bn_miller_rabin_iteration(&miller_rabin, &possibly_prime,
                                              b.get(), mont.get(), ctx()));

        std::string result;
        ASSERT_TRUE(t->GetAttribute(&result, "Result"));
        EXPECT_EQ(result, possibly_prime ? "PossiblyPrime" : "Composite");
      });
}

// These tests are very slow, so we disable them by default to avoid timing out
// downstream consumers. They are enabled when running tests standalone via
// all_tests.go.
TEST_F(BNTest, DISABLED_WycheproofPrimality) {
  FileTestGTest(
      "third_party/wycheproof_testvectors/primality_test.txt",
      [&](FileTest *t) {
        WycheproofResult result;
        ASSERT_TRUE(GetWycheproofResult(t, &result));
        bssl::UniquePtr<BIGNUM> value = GetWycheproofBIGNUM(t, "value", false);
        ASSERT_TRUE(value);

        for (int checks :
             {BN_prime_checks_for_validation, BN_prime_checks_for_generation}) {
          SCOPED_TRACE(checks);
          if (checks == BN_prime_checks_for_generation &&
              std::find(result.flags.begin(), result.flags.end(),
                        "WorstCaseMillerRabin") != result.flags.end()) {
            // Skip the worst case Miller-Rabin cases.
            // |BN_prime_checks_for_generation| relies on such values being rare
            // when generating primes.
            continue;
          }

          int is_probably_prime;
          ASSERT_TRUE(BN_primality_test(&is_probably_prime, value.get(), checks,
                                        ctx(),
                                        /*do_trial_division=*/false, nullptr));
          EXPECT_EQ(result.IsValid() ? 1 : 0, is_probably_prime);

          ASSERT_TRUE(BN_primality_test(&is_probably_prime, value.get(), checks,
                                        ctx(),
                                        /*do_trial_division=*/true, nullptr));
          EXPECT_EQ(result.IsValid() ? 1 : 0, is_probably_prime);
        }
      });
}

TEST_F(BNTest, NumBitsWord) {
  constexpr BN_ULONG kOne = 1;

  // 2^(N-1) takes N bits.
  for (unsigned i = 1; i < BN_BITS2; i++) {
    EXPECT_EQ(i, BN_num_bits_word(kOne << (i - 1))) << i;
  }

  // 2^N - 1 takes N bits.
  for (unsigned i = 0; i < BN_BITS2; i++) {
    EXPECT_EQ(i, BN_num_bits_word((kOne << i) - 1)) << i;
  }

  for (unsigned i = 1; i < 100; i++) {
    // Generate a random value of a random length.
    uint8_t buf[1 + sizeof(BN_ULONG)];
    RAND_bytes(buf, sizeof(buf));

    BN_ULONG w;
    memcpy(&w, &buf[1], sizeof(w));

    const unsigned num_bits = buf[0] % (BN_BITS2 + 1);
    if (num_bits == BN_BITS2) {
      w |= kOne << (BN_BITS2 - 1);
    } else if (num_bits == 0) {
      w = 0;
    } else {
      w &= (kOne << num_bits) - 1;
      w |= kOne << (num_bits - 1);
    }

    EXPECT_EQ(num_bits, BN_num_bits_word(w)) << w;
  }
}

#if !defined(BORINGSSL_SHARED_LIBRARY)
TEST_F(BNTest, LessThanWords) {
  // kTestVectors is an array of 256-bit values in sorted order.
  static const BN_ULONG kTestVectors[][256 / BN_BITS2] = {
      {TOBN(0x00000000, 0x00000000), TOBN(0x00000000, 0x00000000),
       TOBN(0x00000000, 0x00000000), TOBN(0x00000000, 0x00000000)},
      {TOBN(0x00000000, 0x00000001), TOBN(0x00000000, 0x00000000),
       TOBN(0x00000000, 0x00000000), TOBN(0x00000000, 0x00000000)},
      {TOBN(0x00000000, 0x00000002), TOBN(0x00000000, 0x00000000),
       TOBN(0x00000000, 0x00000000), TOBN(0x00000000, 0x00000000)},
      {TOBN(0x00000000, 0x0000ffff), TOBN(0x00000000, 0x00000000),
       TOBN(0x00000000, 0x00000000), TOBN(0x00000000, 0x00000000)},
      {TOBN(0x00000000, 0x83339914), TOBN(0x00000000, 0x00000000),
       TOBN(0x00000000, 0x00000000), TOBN(0x00000000, 0x00000000)},
      {TOBN(0x00000000, 0xfffffffe), TOBN(0x00000000, 0x00000000),
       TOBN(0x00000000, 0x00000000), TOBN(0x00000000, 0x00000000)},
      {TOBN(0x00000000, 0xffffffff), TOBN(0x00000000, 0x00000000),
       TOBN(0x00000000, 0x00000000), TOBN(0x00000000, 0x00000000)},
      {TOBN(0xed17ac85, 0x83339914), TOBN(0x00000000, 0x00000000),
       TOBN(0x00000000, 0x00000000), TOBN(0x00000000, 0x00000000)},
      {TOBN(0xffffffff, 0xffffffff), TOBN(0x00000000, 0x00000000),
       TOBN(0x00000000, 0x00000000), TOBN(0x00000000, 0x00000000)},
      {TOBN(0x00000000, 0x83339914), TOBN(0x00000000, 0x00000001),
       TOBN(0x00000000, 0x00000000), TOBN(0x00000000, 0x00000000)},
      {TOBN(0xffffffff, 0xffffffff), TOBN(0xffffffff, 0xffffffff),
       TOBN(0x00000000, 0x00000000), TOBN(0x00000000, 0x00000000)},
      {TOBN(0xffffffff, 0xffffffff), TOBN(0xffffffff, 0xffffffff),
       TOBN(0xffffffff, 0xffffffff), TOBN(0x00000000, 0x00000000)},
      {TOBN(0x00000000, 0x00000000), TOBN(0x1d6f60ba, 0x893ba84c),
       TOBN(0x597d89b3, 0x754abe9f), TOBN(0xb504f333, 0xf9de6484)},
      {TOBN(0x00000000, 0x83339915), TOBN(0x1d6f60ba, 0x893ba84c),
       TOBN(0x597d89b3, 0x754abe9f), TOBN(0xb504f333, 0xf9de6484)},
      {TOBN(0xed17ac85, 0x00000000), TOBN(0x1d6f60ba, 0x893ba84c),
       TOBN(0x597d89b3, 0x754abe9f), TOBN(0xb504f333, 0xf9de6484)},
      {TOBN(0xed17ac85, 0x83339915), TOBN(0x1d6f60ba, 0x893ba84c),
       TOBN(0x597d89b3, 0x754abe9f), TOBN(0xb504f333, 0xf9de6484)},
      {TOBN(0xed17ac85, 0xffffffff), TOBN(0x1d6f60ba, 0x893ba84c),
       TOBN(0x597d89b3, 0x754abe9f), TOBN(0xb504f333, 0xf9de6484)},
      {TOBN(0xffffffff, 0x83339915), TOBN(0x1d6f60ba, 0x893ba84c),
       TOBN(0x597d89b3, 0x754abe9f), TOBN(0xb504f333, 0xf9de6484)},
      {TOBN(0xffffffff, 0xffffffff), TOBN(0x1d6f60ba, 0x893ba84c),
       TOBN(0x597d89b3, 0x754abe9f), TOBN(0xb504f333, 0xf9de6484)},
      {TOBN(0x00000000, 0x00000000), TOBN(0x00000000, 0x00000000),
       TOBN(0x00000000, 0x00000000), TOBN(0xffffffff, 0xffffffff)},
      {TOBN(0x00000000, 0x00000000), TOBN(0x00000000, 0x00000000),
       TOBN(0xffffffff, 0xffffffff), TOBN(0xffffffff, 0xffffffff)},
      {TOBN(0x00000000, 0x00000001), TOBN(0x00000000, 0x00000000),
       TOBN(0xffffffff, 0xffffffff), TOBN(0xffffffff, 0xffffffff)},
      {TOBN(0x00000000, 0x00000000), TOBN(0xffffffff, 0xffffffff),
       TOBN(0xffffffff, 0xffffffff), TOBN(0xffffffff, 0xffffffff)},
      {TOBN(0xffffffff, 0xffffffff), TOBN(0xffffffff, 0xffffffff),
       TOBN(0xffffffff, 0xffffffff), TOBN(0xffffffff, 0xffffffff)},
  };

  // Determine where the single-word values stop.
  size_t one_word;
  for (one_word = 0; one_word < OPENSSL_ARRAY_SIZE(kTestVectors); one_word++) {
    int is_word = 1;
    for (size_t i = 1; i < OPENSSL_ARRAY_SIZE(kTestVectors[one_word]); i++) {
      if (kTestVectors[one_word][i] != 0) {
        is_word = 0;
        break;
      }
    }
    if (!is_word) {
      break;
    }
  }

  for (size_t i = 0; i < OPENSSL_ARRAY_SIZE(kTestVectors); i++) {
    SCOPED_TRACE(i);
    for (size_t j = 0; j < OPENSSL_ARRAY_SIZE(kTestVectors); j++) {
      SCOPED_TRACE(j);
      EXPECT_EQ(i < j ? 1 : 0,
                bn_less_than_words(kTestVectors[i], kTestVectors[j],
                                   OPENSSL_ARRAY_SIZE(kTestVectors[i])));
      for (size_t k = 0; k < one_word; k++) {
        SCOPED_TRACE(k);
        EXPECT_EQ(k <= i && i < j ? 1 : 0,
                  bn_in_range_words(kTestVectors[i], kTestVectors[k][0],
                                    kTestVectors[j],
                                    OPENSSL_ARRAY_SIZE(kTestVectors[i])));
      }
    }
  }

  EXPECT_EQ(0, bn_less_than_words(NULL, NULL, 0));
  EXPECT_EQ(0, bn_in_range_words(NULL, 0, NULL, 0));
}
#endif  // !BORINGSSL_SHARED_LIBRARY

TEST_F(BNTest, NonMinimal) {
  bssl::UniquePtr<BIGNUM> ten(BN_new());
  ASSERT_TRUE(ten);
  ASSERT_TRUE(BN_set_word(ten.get(), 10));

  bssl::UniquePtr<BIGNUM> ten_copy(BN_dup(ten.get()));
  ASSERT_TRUE(ten_copy);

  bssl::UniquePtr<BIGNUM> eight(BN_new());
  ASSERT_TRUE(eight);
  ASSERT_TRUE(BN_set_word(eight.get(), 8));

  bssl::UniquePtr<BIGNUM> forty_two(BN_new());
  ASSERT_TRUE(forty_two);
  ASSERT_TRUE(BN_set_word(forty_two.get(), 42));

  bssl::UniquePtr<BIGNUM> two_exp_256(BN_new());
  ASSERT_TRUE(two_exp_256);
  ASSERT_TRUE(BN_lshift(two_exp_256.get(), BN_value_one(), 256));

  bssl::UniquePtr<BIGNUM> zero(BN_new());
  ASSERT_TRUE(zero);
  BN_zero(zero.get());

  for (size_t width = 1; width < 10; width++) {
    SCOPED_TRACE(width);
    // Make |ten| and |zero| wider.
    EXPECT_TRUE(bn_resize_words(ten.get(), width));
    EXPECT_EQ(static_cast<int>(width), ten->width);
    EXPECT_TRUE(bn_resize_words(zero.get(), width));
    EXPECT_EQ(static_cast<int>(width), zero->width);

    EXPECT_TRUE(BN_abs_is_word(ten.get(), 10));
    EXPECT_TRUE(BN_is_word(ten.get(), 10));
    EXPECT_EQ(10u, BN_get_word(ten.get()));
    uint64_t v;
    ASSERT_TRUE(BN_get_u64(ten.get(), &v));
    EXPECT_EQ(10u, v);

    EXPECT_TRUE(BN_equal_consttime(ten.get(), ten_copy.get()));
    EXPECT_TRUE(BN_equal_consttime(ten_copy.get(), ten.get()));
    EXPECT_EQ(BN_cmp(ten.get(), ten_copy.get()), 0);
    EXPECT_EQ(BN_cmp(ten_copy.get(), ten.get()), 0);

    EXPECT_FALSE(BN_equal_consttime(ten.get(), eight.get()));
    EXPECT_LT(BN_cmp(eight.get(), ten.get()), 0);
    EXPECT_GT(BN_cmp(ten.get(), eight.get()), 0);

    EXPECT_FALSE(BN_equal_consttime(ten.get(), forty_two.get()));
    EXPECT_GT(BN_cmp(forty_two.get(), ten.get()), 0);
    EXPECT_LT(BN_cmp(ten.get(), forty_two.get()), 0);

    EXPECT_FALSE(BN_equal_consttime(ten.get(), two_exp_256.get()));
    EXPECT_GT(BN_cmp(two_exp_256.get(), ten.get()), 0);
    EXPECT_LT(BN_cmp(ten.get(), two_exp_256.get()), 0);

    EXPECT_EQ(4u, BN_num_bits(ten.get()));
    EXPECT_EQ(1u, BN_num_bytes(ten.get()));
    EXPECT_FALSE(BN_is_pow2(ten.get()));

    bssl::UniquePtr<char> hex(BN_bn2hex(ten.get()));
    EXPECT_STREQ("0A", hex.get());
    hex.reset(BN_bn2hex(zero.get()));
    EXPECT_STREQ("0", hex.get());

    bssl::UniquePtr<BIO> bio(BIO_new(BIO_s_mem()));
    ASSERT_TRUE(bio);
    ASSERT_TRUE(BN_print(bio.get(), ten.get()));
    const uint8_t *ptr;
    size_t len;
    ASSERT_TRUE(BIO_mem_contents(bio.get(), &ptr, &len));
    // TODO(davidben): |BN_print| removes leading zeros within a byte, while
    // |BN_bn2hex| rounds up to a byte, except for zero which it prints as
    // "0". Fix this discrepancy?
    EXPECT_EQ(Bytes("A"), Bytes(ptr, len));

    bio.reset(BIO_new(BIO_s_mem()));
    ASSERT_TRUE(bio);
    ASSERT_TRUE(BN_print(bio.get(), zero.get()));
    ASSERT_TRUE(BIO_mem_contents(bio.get(), &ptr, &len));
    EXPECT_EQ(Bytes("0"), Bytes(ptr, len));
  }

  // |ten| may be resized back down to one word.
  EXPECT_TRUE(bn_resize_words(ten.get(), 1));
  EXPECT_EQ(1, ten->width);

  // But not to zero words, which it does not fit.
  EXPECT_FALSE(bn_resize_words(ten.get(), 0));

  EXPECT_TRUE(BN_is_pow2(eight.get()));
  EXPECT_TRUE(bn_resize_words(eight.get(), 4));
  EXPECT_EQ(4, eight->width);
  EXPECT_TRUE(BN_is_pow2(eight.get()));

  // |BN_MONT_CTX| is always stored minimally and uses the same R independent of
  // input width. Additionally, mont->RR is always the same width as mont->N,
  // even if it fits in a smaller value.
  static const uint8_t kP[] = {
      0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
      0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
      0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01,
  };
  bssl::UniquePtr<BIGNUM> p(BN_bin2bn(kP, sizeof(kP), nullptr));
  ASSERT_TRUE(p);

  // Test both the constant-time and variable-time functions at both minimal and
  // non-minimal |p|.
  bssl::UniquePtr<BN_MONT_CTX> mont(
      BN_MONT_CTX_new_for_modulus(p.get(), ctx()));
  ASSERT_TRUE(mont);
  bssl::UniquePtr<BN_MONT_CTX> mont2(
      BN_MONT_CTX_new_consttime(p.get(), ctx()));
  ASSERT_TRUE(mont2);

  ASSERT_TRUE(bn_resize_words(p.get(), 32));
  bssl::UniquePtr<BN_MONT_CTX> mont3(
      BN_MONT_CTX_new_for_modulus(p.get(), ctx()));
  ASSERT_TRUE(mont3);
  bssl::UniquePtr<BN_MONT_CTX> mont4(
      BN_MONT_CTX_new_consttime(p.get(), ctx()));
  ASSERT_TRUE(mont4);

  EXPECT_EQ(mont->N.width, mont2->N.width);
  EXPECT_EQ(mont->N.width, mont3->N.width);
  EXPECT_EQ(mont->N.width, mont4->N.width);
  EXPECT_EQ(0, BN_cmp(&mont->RR, &mont2->RR));
  EXPECT_EQ(0, BN_cmp(&mont->RR, &mont3->RR));
  EXPECT_EQ(0, BN_cmp(&mont->RR, &mont4->RR));
  EXPECT_EQ(mont->N.width, mont->RR.width);
  EXPECT_EQ(mont->N.width, mont2->RR.width);
  EXPECT_EQ(mont->N.width, mont3->RR.width);
  EXPECT_EQ(mont->N.width, mont4->RR.width);
}

TEST_F(BNTest, CountLowZeroBits) {
  bssl::UniquePtr<BIGNUM> bn(BN_new());
  ASSERT_TRUE(bn);

  for (int i = 0; i < BN_BITS2; i++) {
    SCOPED_TRACE(i);
    for (int set_high_bits = 0; set_high_bits < 2; set_high_bits++) {
      BN_ULONG word = ((BN_ULONG)1) << i;
      if (set_high_bits) {
        BN_ULONG junk;
        RAND_bytes(reinterpret_cast<uint8_t *>(&junk), sizeof(junk));
        word |= junk & ~(word - 1);
      }
      SCOPED_TRACE(word);

      ASSERT_TRUE(BN_set_word(bn.get(), word));
      EXPECT_EQ(i, BN_count_low_zero_bits(bn.get()));
      ASSERT_TRUE(bn_resize_words(bn.get(), 16));
      EXPECT_EQ(i, BN_count_low_zero_bits(bn.get()));

      ASSERT_TRUE(BN_set_word(bn.get(), word));
      ASSERT_TRUE(BN_lshift(bn.get(), bn.get(), BN_BITS2 * 5));
      EXPECT_EQ(i + BN_BITS2 * 5, BN_count_low_zero_bits(bn.get()));
      ASSERT_TRUE(bn_resize_words(bn.get(), 16));
      EXPECT_EQ(i + BN_BITS2 * 5, BN_count_low_zero_bits(bn.get()));

      ASSERT_TRUE(BN_set_word(bn.get(), word));
      ASSERT_TRUE(BN_set_bit(bn.get(), BN_BITS2 * 5));
      EXPECT_EQ(i, BN_count_low_zero_bits(bn.get()));
      ASSERT_TRUE(bn_resize_words(bn.get(), 16));
      EXPECT_EQ(i, BN_count_low_zero_bits(bn.get()));
    }
  }

  BN_zero(bn.get());
  EXPECT_EQ(0, BN_count_low_zero_bits(bn.get()));
  ASSERT_TRUE(bn_resize_words(bn.get(), 16));
  EXPECT_EQ(0, BN_count_low_zero_bits(bn.get()));
}

TEST_F(BNTest, WriteIntoNegative) {
  bssl::UniquePtr<BIGNUM> r(BN_new());
  ASSERT_TRUE(r);
  bssl::UniquePtr<BIGNUM> two(BN_new());
  ASSERT_TRUE(two);
  ASSERT_TRUE(BN_set_word(two.get(), 2));
  bssl::UniquePtr<BIGNUM> three(BN_new());
  ASSERT_TRUE(three);
  ASSERT_TRUE(BN_set_word(three.get(), 3));
  bssl::UniquePtr<BIGNUM> seven(BN_new());
  ASSERT_TRUE(seven);
  ASSERT_TRUE(BN_set_word(seven.get(), 7));

  ASSERT_TRUE(BN_set_word(r.get(), 1));
  BN_set_negative(r.get(), 1);
  ASSERT_TRUE(BN_mod_add_quick(r.get(), two.get(), three.get(), seven.get()));
  EXPECT_TRUE(BN_is_word(r.get(), 5));
  EXPECT_FALSE(BN_is_negative(r.get()));

  BN_set_negative(r.get(), 1);
  ASSERT_TRUE(BN_mod_sub_quick(r.get(), two.get(), three.get(), seven.get()));
  EXPECT_TRUE(BN_is_word(r.get(), 6));
  EXPECT_FALSE(BN_is_negative(r.get()));
}

TEST_F(BNTest, ModSqrtInvalid) {
  bssl::UniquePtr<BIGNUM> bn2140141 = ASCIIToBIGNUM("2140141");
  ASSERT_TRUE(bn2140141);
  bssl::UniquePtr<BIGNUM> bn2140142 = ASCIIToBIGNUM("2140142");
  ASSERT_TRUE(bn2140142);
  bssl::UniquePtr<BIGNUM> bn4588033 = ASCIIToBIGNUM("4588033");
  ASSERT_TRUE(bn4588033);

  // |BN_mod_sqrt| may fail or return an arbitrary value, so we do not use
  // |TestModSqrt| or |TestNotModSquare|. We only promise it will not crash or
  // infinite loop. (For some invalid inputs, it may even be non-deterministic.)
  // See CVE-2022-0778.
  BN_free(BN_mod_sqrt(nullptr, bn2140141.get(), bn4588033.get(), ctx()));
  BN_free(BN_mod_sqrt(nullptr, bn2140142.get(), bn4588033.get(), ctx()));
}

// Test that constructing Montgomery contexts for large bignums is not possible.
// Our Montgomery reduction implementation stack-allocates temporaries, so we
// cap how large of moduli we accept.
TEST_F(BNTest, MontgomeryLarge) {
  std::vector<uint8_t> large_bignum_bytes(16 * 1024, 0xff);
  bssl::UniquePtr<BIGNUM> large_bignum(
      BN_bin2bn(large_bignum_bytes.data(), large_bignum_bytes.size(), nullptr));
  ASSERT_TRUE(large_bignum);
  bssl::UniquePtr<BN_MONT_CTX> mont(
      BN_MONT_CTX_new_for_modulus(large_bignum.get(), ctx()));
  EXPECT_FALSE(mont);

  // The same limit should apply when |BN_mod_exp_mont_consttime| internally
  // constructs a |BN_MONT_CTX|.
  bssl::UniquePtr<BIGNUM> r(BN_new());
  ASSERT_TRUE(r);
  EXPECT_FALSE(BN_mod_exp_mont_consttime(r.get(), BN_value_one(),
                                         large_bignum.get(), large_bignum.get(),
                                         ctx(), nullptr));
}

TEST_F(BNTest, FormatWord) {
  char buf[32];
  snprintf(buf, sizeof(buf), BN_DEC_FMT1, BN_ULONG{1234});
  EXPECT_STREQ(buf, "1234");
  snprintf(buf, sizeof(buf), BN_HEX_FMT1, BN_ULONG{1234});
  EXPECT_STREQ(buf, "4d2");

  // |BN_HEX_FMT2| is zero-padded up to the maximum value.
#if defined(OPENSSL_64_BIT)
  snprintf(buf, sizeof(buf), BN_HEX_FMT2, BN_ULONG{1234});
  EXPECT_STREQ(buf, "00000000000004d2");
  snprintf(buf, sizeof(buf), BN_HEX_FMT2, std::numeric_limits<BN_ULONG>::max());
  EXPECT_STREQ(buf, "ffffffffffffffff");
#else
  snprintf(buf, sizeof(buf), BN_HEX_FMT2, BN_ULONG{1234});
  EXPECT_STREQ(buf, "000004d2");
  snprintf(buf, sizeof(buf), BN_HEX_FMT2, std::numeric_limits<BN_ULONG>::max());
  EXPECT_STREQ(buf, "ffffffff");
#endif
}

TEST_F(BNTest, GetMinimalWidth) {
  bssl::UniquePtr<BIGNUM> bn(BN_new());
  ASSERT_TRUE(bn);

  // Zero needs 0 words
  BN_zero(bn.get());
  EXPECT_EQ(0, BN_get_minimal_width(bn.get()));

  // 1 needs 1 word
  ASSERT_TRUE(BN_one(bn.get()));
  EXPECT_EQ(1, BN_get_minimal_width(bn.get()));

  // Test negative numbers
  ASSERT_TRUE(BN_set_word(bn.get(), 1));
  BN_set_negative(bn.get(), 1);
  EXPECT_EQ(1, BN_get_minimal_width(bn.get()));

  // Test powers of two
  ASSERT_TRUE(BN_set_word(bn.get(), 1));
  for (size_t i = 0; i < 256; i++) {
    // For 2^i, we need ceil((i+1)/BN_BITS2) words
    int expected_words = (i + BN_BITS2) / BN_BITS2;
    EXPECT_EQ(expected_words, BN_get_minimal_width(bn.get()))
        << "Failed for 2^" << i;
    ASSERT_TRUE(BN_lshift1(bn.get(), bn.get()));
  }

  // Test values near word boundaries
  // For 32-bit: word = 32 bits
  // For 64-bit: word = 64 bits
  struct {
    const char* hex;
    int expected_32bit;
    int expected_64bit;
  } kTests[] = {
    // 8-bit number
    {"ff", 1, 1},
    // 32-bit number
    {"ffffffff", 1, 1},
    // 33-bit number
    {"100000000", 2, 1},
    // 64-bit number
    {"ffffffff00000000", 2, 1},
    // 64-bit number
    {"ffffffffffffffff", 2, 1},
    // 65-bit number
    {"10000000000000000", 3, 2},
    // Multiple word number
    {"ffffffffffffffffffffffffffffffff", 4, 2},
  };

  for (const auto& test : kTests) {
    SCOPED_TRACE(test.hex);
    HexToBIGNUMWithReturn(&bn, test.hex);
#if defined(OPENSSL_32_BIT)
    EXPECT_EQ(test.expected_32bit, BN_get_minimal_width(bn.get()));
#elif defined(OPENSSL_64_BIT)
    EXPECT_EQ(test.expected_64bit, BN_get_minimal_width(bn.get()));
#endif
  }
}

#if defined(OPENSSL_BN_ASM_MONT) && defined(SUPPORTS_ABI_TEST)
TEST_F(BNTest, BNMulMontABI) {
  for (size_t words : {4, 5, 6, 7, 8, 16, 32}) {
    SCOPED_TRACE(words);

    bssl::UniquePtr<BIGNUM> m(BN_new());
    ASSERT_TRUE(m);
    ASSERT_TRUE(BN_set_bit(m.get(), 0));
    ASSERT_TRUE(BN_set_bit(m.get(), words * BN_BITS2 - 1));
    bssl::UniquePtr<BN_MONT_CTX> mont(
        BN_MONT_CTX_new_for_modulus(m.get(), ctx()));
    ASSERT_TRUE(mont);

    std::vector<BN_ULONG> r(words), a(words), b(words);
    a[0] = 1;
    b[0] = 42;

#if defined(OPENSSL_X86_64)
#if !defined(MY_ASSEMBLER_IS_TOO_OLD_FOR_512AVX)
    if (bn_mulx4x_mont_capable(words)) {
      CHECK_ABI(bn_mulx4x_mont, r.data(), a.data(), b.data(), mont->N.d,
                mont->n0, words);
      CHECK_ABI(bn_mulx4x_mont, r.data(), a.data(), a.data(), mont->N.d,
                mont->n0, words);
    }
#endif // !defined(MY_ASSEMBLER_IS_TOO_OLD_FOR_512AVX)
    if (bn_mul4x_mont_capable(words)) {
      CHECK_ABI(bn_mul4x_mont, r.data(), a.data(), b.data(), mont->N.d,
                mont->n0, words);
      CHECK_ABI(bn_mul4x_mont, r.data(), a.data(), a.data(), mont->N.d,
                mont->n0, words);
    }
    CHECK_ABI(bn_mul_mont_nohw, r.data(), a.data(), b.data(), mont->N.d,
              mont->n0, words);
    CHECK_ABI(bn_mul_mont_nohw, r.data(), a.data(), a.data(), mont->N.d,
              mont->n0, words);
#if !defined(MY_ASSEMBLER_IS_TOO_OLD_FOR_512AVX)
    if (bn_sqr8x_mont_capable(words)) {
      CHECK_ABI(bn_sqr8x_mont, r.data(), a.data(), bn_mulx_adx_capable(),
                mont->N.d, mont->n0, words);
    }
#endif // !defined(MY_ASSEMBLER_IS_TOO_OLD_FOR_512AVX)
#elif defined(OPENSSL_ARM)
    if (bn_mul8x_mont_neon_capable(words)) {
      CHECK_ABI(bn_mul8x_mont_neon, r.data(), a.data(), b.data(), mont->N.d,
                mont->n0, words);
      CHECK_ABI(bn_mul8x_mont_neon, r.data(), a.data(), a.data(), mont->N.d,
                mont->n0, words);
    }
    CHECK_ABI(bn_mul_mont_nohw, r.data(), a.data(), b.data(), mont->N.d,
              mont->n0, words);
    CHECK_ABI(bn_mul_mont_nohw, r.data(), a.data(), a.data(), mont->N.d,
              mont->n0, words);
#else
    CHECK_ABI(bn_mul_mont, r.data(), a.data(), b.data(), mont->N.d, mont->n0,
              words);
    CHECK_ABI(bn_mul_mont, r.data(), a.data(), a.data(), mont->N.d, mont->n0,
              words);
#endif
  }
}
#endif   // OPENSSL_BN_ASM_MONT && SUPPORTS_ABI_TEST

#if defined(OPENSSL_BN_ASM_MONT5) && defined(SUPPORTS_ABI_TEST)
TEST_F(BNTest, BNMulMont5ABI) {
  for (size_t words : {4, 5, 6, 7, 8, 16, 32}) {
    SCOPED_TRACE(words);

    bssl::UniquePtr<BIGNUM> m(BN_new());
    ASSERT_TRUE(m);
    ASSERT_TRUE(BN_set_bit(m.get(), 0));
    ASSERT_TRUE(BN_set_bit(m.get(), words * BN_BITS2 - 1));
    bssl::UniquePtr<BN_MONT_CTX> mont(
        BN_MONT_CTX_new_for_modulus(m.get(), ctx()));
    ASSERT_TRUE(mont);

    std::vector<BN_ULONG> r(words), a(words), b(words), table(words * 32);
    a[0] = 1;
    b[0] = 42;

    bn_mul_mont(r.data(), a.data(), b.data(), mont->N.d, mont->n0, words);
    CHECK_ABI(bn_scatter5, r.data(), words, table.data(), 13);
    for (size_t i = 0; i < 32; i++) {
      bn_mul_mont(r.data(), a.data(), b.data(), mont->N.d, mont->n0, words);
      bn_scatter5(r.data(), words, table.data(), i);
    }
    CHECK_ABI(bn_gather5, r.data(), words, table.data(), 13);

    CHECK_ABI(bn_mul_mont_gather5, r.data(), r.data(), table.data(), m->d,
              mont->n0, words, 13);
    CHECK_ABI(bn_mul_mont_gather5, r.data(), a.data(), table.data(), m->d,
              mont->n0, words, 13);

    if (words % 8 == 0) {
      CHECK_ABI(bn_power5, r.data(), r.data(), table.data(), m->d, mont->n0,
                words, 13);
      CHECK_ABI(bn_power5, r.data(), a.data(), table.data(), m->d, mont->n0,
                words, 13);
    }
  }
}
#endif  // OPENSSL_BN_ASM_MONT5 && SUPPORTS_ABI_TEST

#if defined(RSAZ_ENABLED) && defined(SUPPORTS_ABI_TEST)
TEST_F(BNTest, RSAZABI) {
  if (!rsaz_avx2_capable()) {
    return;
  }

  stack_align_type buffer_table[64 + (sizeof(BN_ULONG) * (32*18))] = {0};
  stack_align_type buffer_rsaz1[64 + (sizeof(BN_ULONG) * 40)];
  stack_align_type buffer_rsaz2[64 + (sizeof(BN_ULONG) * 40)];
  stack_align_type buffer_rsaz3[64 + (sizeof(BN_ULONG) * 40)];
  stack_align_type buffer_n_rsaz[64 + (sizeof(BN_ULONG) * 40)];

  BN_ULONG *aligned_table = (BN_ULONG *) align_pointer(buffer_table, 64);
  BN_ULONG *aligned_rsaz1 = (BN_ULONG *) align_pointer(buffer_rsaz1, 64);
  BN_ULONG *aligned_rsaz2 = (BN_ULONG *) align_pointer(buffer_rsaz2, 64);
  BN_ULONG *aligned_rsaz3 = (BN_ULONG *) align_pointer(buffer_rsaz3, 64);
  BN_ULONG *aligned_n_rsaz = (BN_ULONG *) align_pointer(buffer_n_rsaz, 64);

  BN_ULONG norm[16], n_norm[16];

  OPENSSL_memset(norm, 0x42, sizeof(norm));
  OPENSSL_memset(n_norm, 0x99, sizeof(n_norm));

  bssl::UniquePtr<BIGNUM> n(BN_new());
  ASSERT_TRUE(n);
  ASSERT_TRUE(bn_set_words(n.get(), n_norm, 16));
  bssl::UniquePtr<BN_MONT_CTX> mont(
      BN_MONT_CTX_new_for_modulus(n.get(), nullptr));
  ASSERT_TRUE(mont);
  const BN_ULONG k = mont->n0[0];

  CHECK_ABI(rsaz_1024_norm2red_avx2, aligned_rsaz1, norm);
  CHECK_ABI(rsaz_1024_norm2red_avx2, aligned_n_rsaz, n_norm);
  CHECK_ABI(rsaz_1024_sqr_avx2, aligned_rsaz2, aligned_rsaz1, aligned_n_rsaz, k, 1);
  CHECK_ABI(rsaz_1024_sqr_avx2, aligned_rsaz3, aligned_rsaz2, aligned_n_rsaz, k, 4);
  CHECK_ABI(rsaz_1024_mul_avx2, aligned_rsaz3, aligned_rsaz1, aligned_rsaz2, aligned_n_rsaz, k);
  CHECK_ABI(rsaz_1024_scatter5_avx2, aligned_table, aligned_rsaz3, 7);
  CHECK_ABI(rsaz_1024_gather5_avx2, aligned_rsaz1, aligned_table, 7);
  CHECK_ABI(rsaz_1024_red2norm_avx2, norm, aligned_rsaz1);

#ifdef RSAZ_512_ENABLED
  if (CRYPTO_is_AVX512IFMA_capable()) {
	  
#define TWOK (40 * 2)
#define TWOK_TABLE (2 * 20 * (1<<5))
#define THREEK (64 * 2)
#define THREEK_TABLE (2 * 32 * (1<<5))
#define FOURK (80 * 2)
#define FOURK_TABLE (2 * 40 * (1<<5))

    int storage_bytes =
      ((TWOK * 2)   + // res2 / red_y2
       TWOK_TABLE   + // red_table2k
      (THREEK * 2)  + // res3 / red_y3
      THREEK_TABLE  + // red_table3k
      (FOURK * 2)   + // res4 / red_y4
       FOURK_TABLE) * // red_table4k
      sizeof(uint64_t);

    uint64_t *storage = (uint64_t*)OPENSSL_malloc(storage_bytes);

    uint64_t *res2, *res3, *res4,
      *red_y2, *red_y3, *red_y4,
      *red_table2k, *red_table3k, *red_table4k;

    res2 = storage;
    red_y2 = storage + TWOK;
    red_table2k = red_y2 + TWOK;
    res3 = red_table2k + TWOK_TABLE;
    red_y3 = res3 + THREEK;
    red_table3k = red_y3 + THREEK;
    res4 = red_table3k + THREEK_TABLE;
    red_y4 = res4 + FOURK;
    red_table4k = red_y4 + FOURK;

    uint64_t a = 0;
    uint64_t b = 0;
    uint64_t m = 0;
    uint64_t k0 = 0;
    uint64_t k2[2] = {0};
    int idx1 = 0;
    int idx2 = 0;

    CHECK_ABI(rsaz_amm52x20_x1_ifma256, res2, &a, &b, &m, k0);
    CHECK_ABI(rsaz_amm52x20_x2_ifma256, res2, &a, &b, &m, k2);
    CHECK_ABI(extract_multiplier_2x20_win5, red_y2, red_table2k, idx1, idx2);

    CHECK_ABI(rsaz_amm52x30_x1_ifma256, res3, &a, &b, &m, k0);
    CHECK_ABI(rsaz_amm52x30_x2_ifma256, res3, &a, &b, &m, k2);
    CHECK_ABI(extract_multiplier_2x30_win5, red_y3, red_table3k, idx1, idx2);

    CHECK_ABI(rsaz_amm52x40_x1_ifma256, res4, &a, &b, &m, k0);
    CHECK_ABI(rsaz_amm52x40_x2_ifma256, res4, &a, &b, &m, k2);
    CHECK_ABI(extract_multiplier_2x40_win5, red_y4, red_table4k, idx1, idx2);

    OPENSSL_free(storage);
  }
#endif // RSAZ_512_ENABLED
}
#endif   // RSAZ_ENABLED && SUPPORTS_ABI_TEST
