//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_SUPPORT_CONSTEXPR_RANDOM_H
#define TEST_SUPPORT_CONSTEXPR_RANDOM_H

#include <climits>
#include <cstddef>
#include <cstdint>
#include <iterator>
#include <limits>
#include <type_traits>
#include <utility>

#include "test_macros.h"

namespace support {

namespace detail {

template <class>
struct is_valid_integer_type_for_random : std::false_type {};
template <>
struct is_valid_integer_type_for_random<std::int8_t> : std::true_type {};
template <>
struct is_valid_integer_type_for_random<short> : std::true_type {};
template <>
struct is_valid_integer_type_for_random<int> : std::true_type {};
template <>
struct is_valid_integer_type_for_random<long> : std::true_type {};
template <>
struct is_valid_integer_type_for_random<long long> : std::true_type {};
template <>
struct is_valid_integer_type_for_random<std::uint8_t> : std::true_type {};
template <>
struct is_valid_integer_type_for_random<unsigned short> : std::true_type {};
template <>
struct is_valid_integer_type_for_random<unsigned int> : std::true_type {};
template <>
struct is_valid_integer_type_for_random<unsigned long> : std::true_type {};
template <>
struct is_valid_integer_type_for_random<unsigned long long> : std::true_type {};

#ifndef TEST_HAS_NO_INT128
template <>
struct is_valid_integer_type_for_random<__int128_t> : std::true_type {};
template <>
struct is_valid_integer_type_for_random<__uint128_t> : std::true_type {};
#endif // TEST_HAS_NO_INT128

template <class, class = void>
struct is_valid_urng : std::false_type {};
template <class Gen>
struct is_valid_urng<
    Gen,
    typename std::enable_if<std::is_unsigned<typename Gen::result_type>::value &&
                            std::is_same<decltype(std::declval<Gen&>()()), typename Gen::result_type>::value>::type>
    : std::true_type {};

template <class UIntType, UIntType N, std::size_t R>
struct meta_log2_imp;

template <unsigned long long N, std::size_t R>
struct meta_log2_imp<unsigned long long, N, R> {
  static const std::size_t value =
      N & ((unsigned long long)(1) << R) ? R : meta_log2_imp<unsigned long long, N, R - 1>::value;
};

template <unsigned long long N>
struct meta_log2_imp<unsigned long long, N, 0> {
  static const std::size_t value = 0;
};

template <size_t R>
struct meta_log2_imp<unsigned long long, 0, R> {
  static const std::size_t value = R + 1;
};

#ifndef TEST_HAS_NO_INT128
template <__uint128_t N, std::size_t R>
struct meta_log2_imp<__uint128_t, N, R> {
  static const size_t value =
      (N >> 64) ? (64 + meta_log2_imp<unsigned long long, (N >> 64), 63>::value)
                : meta_log2_imp<unsigned long long, N, 63>::value;
};
#endif // TEST_HAS_NO_INT128

template <class UIntType, UIntType N>
struct meta_log2 {
  static const size_t value = meta_log2_imp<
#ifndef TEST_HAS_NO_INT128
      typename std::conditional<sizeof(UIntType) <= sizeof(unsigned long long), unsigned long long, __uint128_t>::type,
#else
      unsigned long long,
#endif // TEST_HAS_NO_INT128
      N,
      sizeof(UIntType) * CHAR_BIT - 1 >::value;
};

#ifdef TEST_COMPILER_MSVC
template <int Width, class T, typename std::enable_if<(Width <= 1), int>::type = 0>
TEST_CONSTEXPR int countl_zero_div_conq(T n) TEST_NOEXCEPT {
  return static_cast<int>(~n) & 1;
}

template <int Width, class T, typename std::enable_if<(Width > 1), int>::type = 0>
TEST_CONSTEXPR int countl_zero_div_conq(T n) TEST_NOEXCEPT {
  return n >= (static_cast<T>(1) << (Width / 2))
           ? detail::countl_zero_div_conq<Width / 2>(n >> (Width / 2))
           : detail::countl_zero_div_conq<Width / 2>(n) + Width / 2;
}
#endif

template <class T, typename std::enable_if<std::is_same<T, unsigned int>::value, int>::type = 0>
TEST_CONSTEXPR int countl_zero(T n) TEST_NOEXCEPT {
#ifdef TEST_COMPILER_MSVC
  return detail::countl_zero_div_conq<std::numeric_limits<T>::digits>(n);
#else
  return __builtin_clz(n);
#endif
}

template <class T, typename std::enable_if<std::is_same<T, unsigned long>::value, int>::type = 0>
TEST_CONSTEXPR_CXX14 int countl_zero(T n) TEST_NOEXCEPT {
#ifdef TEST_COMPILER_MSVC
  return detail::countl_zero_div_conq<std::numeric_limits<T>::digits>(n);
#else
  return __builtin_clzl(n);
#endif
}

template <class T, typename std::enable_if<std::is_same<T, unsigned long long>::value, int>::type = 0>
TEST_CONSTEXPR int countl_zero(T n) TEST_NOEXCEPT {
#ifdef TEST_COMPILER_MSVC
  return detail::countl_zero_div_conq<std::numeric_limits<T>::digits>(n);
#else
  return __builtin_clzll(n);
#endif
}

#ifndef TEST_HAS_NO_INT128
template <class T, typename std::enable_if<std::is_same<T, __uint128_t>::value, int>::type = 0>
TEST_CONSTEXPR int countl_zero(T n) TEST_NOEXCEPT {
  return n > std::numeric_limits<std::uint64_t>::max()
           ? detail::countl_zero(static_cast<std::uint64_t>(n >> 64))
           : detail::countl_zero(static_cast<std::uint64_t>(n)) + 64;
}
#endif // TEST_HAS_NO_INT128

template <class T,
          typename std::enable_if<std::is_same<T, unsigned char>::value || std::is_same<T, unsigned short>::value,
                                  int>::type = 0>
TEST_CONSTEXPR int countl_zero(T n) TEST_NOEXCEPT {
  return detail::countl_zero(static_cast<unsigned int>(n)) -
         (std::numeric_limits<unsigned int>::digits - std::numeric_limits<T>::digits);
}

template <class Engine, class UIntType>
class independent_bits_engine {
public:
  typedef UIntType result_type;

private:
  typedef typename Engine::result_type engine_result_type;
  typedef
      typename std::conditional<sizeof(engine_result_type) <= sizeof(result_type), result_type, engine_result_type>::
          type working_result_type;

  Engine& eng_;
  std::size_t width_;
  std::size_t wid0_;
  std::size_t round_count_all_;
  std::size_t round_count_regular_;
  working_result_type y0_;
  working_result_type y1_;
  engine_result_type mask0_;
  engine_result_type mask1_;

#if TEST_STD_VER >= 11
  static constexpr working_result_type rp = Engine::max() - Engine::min() + working_result_type(1);
#else
  static const working_result_type rp = Engine::max_value - Engine::min_value + working_result_type(1);
#endif
  static TEST_CONSTEXPR const std::size_t rp_log2  = meta_log2<working_result_type, rp>::value;
  static TEST_CONSTEXPR const std::size_t w_digits = std::numeric_limits<working_result_type>::digits;
  static TEST_CONSTEXPR const std::size_t e_digits = std::numeric_limits<engine_result_type>::digits;

public:
  // constructors and seeding functions
  TEST_CONSTEXPR_CXX14 independent_bits_engine(Engine& eng, std::size_t width)
      : eng_(eng),
        width_(width),
        wid0_(),
        round_count_all_(),
        round_count_regular_(),
        y0_(),
        y1_(),
        mask0_(),
        mask1_() {
    round_count_all_ = width_ / rp_log2 + (width_ % rp_log2 != 0);
    wid0_            = width_ / round_count_all_;
    if TEST_CONSTEXPR_CXX17 (rp == 0) {
      y0_ = rp;
    } else {
      if (wid0_ < w_digits)
        y0_ = (rp >> wid0_) << wid0_;
      else
        y0_ = 0;
    }
    if (rp - y0_ > y0_ / round_count_all_) {
      ++round_count_all_;
      wid0_ = width_ / round_count_all_;
      if (wid0_ < w_digits)
        y0_ = (rp >> wid0_) << wid0_;
      else
        y0_ = 0;
    }
    round_count_regular_ = round_count_all_ - width_ % round_count_all_;
    if (wid0_ < w_digits - 1)
      y1_ = (rp >> (wid0_ + 1)) << (wid0_ + 1);
    else
      y1_ = 0;
    mask0_ = wid0_ > 0 ? static_cast<engine_result_type>(engine_result_type(~0) >> (e_digits - wid0_))
                       : engine_result_type(0);
    mask1_ = wid0_ < e_digits - 1 ? static_cast<engine_result_type>(engine_result_type(~0) >> (e_digits - (wid0_ + 1)))
                                  : engine_result_type(~0);
  }

  // generating functions
  TEST_CONSTEXPR_CXX14 result_type operator()() { return generate(std::integral_constant<bool, (rp != 0)>()); }

private:
  TEST_CONSTEXPR_CXX14 result_type generate(std::false_type) { return static_cast<result_type>(eng_() & mask0_); }

  TEST_CONSTEXPR_CXX14 result_type generate(std::true_type) {
    const std::size_t r_digits = std::numeric_limits<result_type>::digits;
    result_type result         = 0;
    for (std::size_t k = 0; k < round_count_regular_; ++k) {
      engine_result_type eng_result = 0;
      do {
        eng_result = static_cast<engine_result_type>(eng_() - Engine::min());
      } while (eng_result >= y0_);
      if (wid0_ < r_digits)
        result <<= wid0_;
      else
        result = 0;
      result += eng_result & mask0_;
    }
    for (std::size_t k = round_count_regular_; k < round_count_all_; ++k) {
      engine_result_type eng_result = 0;
      do {
        eng_result = static_cast<engine_result_type>(eng_() - Engine::min());
      } while (eng_result >= y1_);
      if (wid0_ < r_digits - 1)
        result <<= wid0_ + 1;
      else
        result = 0;
      result += eng_result & mask1_;
    }
    return result;
  }
};

} // namespace detail

template <class IntType = int>
class uniform_int_distribution {
  static_assert(detail::is_valid_integer_type_for_random<IntType>::value, "IntType must be a supported integer type");

public:
  // types
  typedef IntType result_type;

  class param_type {
    result_type a_;
    result_type b_;

  public:
    typedef uniform_int_distribution distribution_type;

#if TEST_STD_VER >= 11
    constexpr param_type() : param_type(0) {}
#else
    param_type() : a_(0), b_(std::numeric_limits<result_type>::max()) {}
#endif
    TEST_CONSTEXPR explicit param_type(result_type ax, result_type bx = std::numeric_limits<result_type>::max())
        : a_(ax), b_(bx) {}

    TEST_CONSTEXPR result_type a() const { return a_; }
    TEST_CONSTEXPR result_type b() const { return b_; }

#if TEST_STD_VER >= 20
    friend bool operator==(const param_type&, const param_type&) = default;
#else
    TEST_CONSTEXPR friend bool operator==(const param_type& lhs, const param_type& rhs) {
      return lhs.a_ == rhs.a_ && lhs.b_ == rhs.b_;
    }
    TEST_CONSTEXPR friend bool operator!=(const param_type& lhs, const param_type& rhs) { return !(lhs == rhs); }
#endif
  };

private:
  param_type param_;

public:
  // constructors and reset functions
#if TEST_STD_VER >= 11
  uniform_int_distribution() = default;
#else
  uniform_int_distribution() {}
#endif
  TEST_CONSTEXPR explicit uniform_int_distribution(result_type ax,
                                                   result_type bx = std::numeric_limits<result_type>::max())
      : param_(ax, bx) {}
  TEST_CONSTEXPR explicit uniform_int_distribution(const param_type& param) : param_(param) {}
  TEST_CONSTEXPR_CXX14 void reset() {}

  // generating functions
  template <class URNG>
  TEST_CONSTEXPR_CXX14 result_type operator()(URNG& gen) {
    return (*this)(gen, param_);
  }

#if TEST_HAS_FEATURE(no_sanitize) && !defined(TEST_COMPILER_GCC)
#  define TEST_DISABLE_UBSAN_UNSIGNED_INTEGER_CHECK __attribute__((__no_sanitize__("unsigned-integer-overflow")))
#else
#  define TEST_DISABLE_UBSAN_UNSIGNED_INTEGER_CHECK
#endif
  template <class URNG>
  TEST_CONSTEXPR_CXX14 result_type operator()(URNG& gen, const param_type& param)
      TEST_DISABLE_UBSAN_UNSIGNED_INTEGER_CHECK {
    static_assert(detail::is_valid_urng<URNG>::value, "invalid uniform random bit generator used");
    typedef typename std::conditional<sizeof(result_type) <= sizeof(std::uint32_t),
                                      std::uint32_t,
                                      typename std::make_unsigned<result_type>::type>::type UIntType;
    const UIntType rp = UIntType(param.b()) - UIntType(param.a()) + UIntType(1);
    if (rp == 1)
      return param.a();
    const std::size_t ur_digits = std::numeric_limits<UIntType>::digits;
    typedef detail::independent_bits_engine<URNG, UIntType> Eng;
    if (rp == 0)
      return static_cast<result_type>(Eng(gen, ur_digits)());
    std::size_t width = ur_digits - detail::countl_zero(rp) - 1;
    if ((rp & (std::numeric_limits<UIntType>::max() >> (ur_digits - width))) != 0)
      ++width;
    Eng eng(gen, width);
    UIntType eng_result = 0;
    do {
      eng_result = eng();
    } while (eng_result >= rp);
    return static_cast<result_type>(eng_result + param.a());
  }
#undef TEST_DISABLE_UBSAN_UNSIGNED_INTEGER_CHECK

  // property functions
  TEST_CONSTEXPR result_type a() const { return param_.a(); }
  TEST_CONSTEXPR result_type b() const { return param_.b(); }

  TEST_CONSTEXPR param_type param() const { return param_; }
  TEST_CONSTEXPR_CXX14 void param(const param_type& param) { param_ = param; }

  TEST_CONSTEXPR result_type min() const { return a(); }
  TEST_CONSTEXPR result_type max() const { return b(); }

#if TEST_STD_VER >= 20
  friend bool operator==(const uniform_int_distribution&, const uniform_int_distribution&) = default;
#else
  TEST_CONSTEXPR friend bool operator==(const uniform_int_distribution& lhs, const uniform_int_distribution& rhs) {
    return lhs.param_ == rhs.param_;
  }
  TEST_CONSTEXPR friend bool operator!=(const uniform_int_distribution& lhs, const uniform_int_distribution& rhs) {
    return !(lhs == rhs);
  }
#endif
};

class simple_random_generator { // A linear congruential generator, using the constants used by MS UCRT.
private:
  std::uint32_t status_;

public:
  typedef std::uint16_t result_type;

  static TEST_CONSTEXPR result_type min() TEST_NOEXCEPT { return 0; }
  static TEST_CONSTEXPR result_type max() TEST_NOEXCEPT { return static_cast<result_type>(-1); }
#if TEST_STD_VER < 11
  static const result_type min_value = 0;
  static const result_type max_value = static_cast<result_type>(-1);
#endif
  static TEST_CONSTEXPR const result_type default_seed = 19937;

#if TEST_STD_VER >= 11
  constexpr simple_random_generator() noexcept : simple_random_generator(default_seed) {}
#else
  simple_random_generator() throw() : status_(default_seed) {}
#endif
  TEST_CONSTEXPR explicit simple_random_generator(std::uint16_t s) TEST_NOEXCEPT : status_(s) {}

  TEST_CONSTEXPR_CXX14 result_type operator()() TEST_NOEXCEPT {
    status_ = status_ * 214013u + 2531011u;
    return static_cast<result_type>(status_ >> 16);
  }
};

template <class RandomAccessIterator, class UniformRandomNumberGenerator>
TEST_CONSTEXPR_CXX14 void
#if TEST_STD_VER >= 11
shuffle(RandomAccessIterator first, RandomAccessIterator last, UniformRandomNumberGenerator&& gen)
#else
shuffle(RandomAccessIterator first, RandomAccessIterator last, UniformRandomNumberGenerator& gen)
#endif
{
  typedef typename std::iterator_traits<RandomAccessIterator>::difference_type difference_type;
  typedef uniform_int_distribution<ptrdiff_t> dist;
  typedef typename dist::param_type param_type;

  RandomAccessIterator last_iter = last;
  difference_type diff           = last_iter - first;
  if (diff > 1) {
    dist uid;
    for (--last_iter, (void)--diff; first < last; ++first, (void)--diff) {
      difference_type index = uid(gen, param_type(0, diff));
      if (index != difference_type(0)) {
        using std::swap;
        swap(*first, *(first + index));
      }
    }
  }
}

} // namespace support

#endif // TEST_SUPPORT_CONSTEXPR_RANDOM_H
