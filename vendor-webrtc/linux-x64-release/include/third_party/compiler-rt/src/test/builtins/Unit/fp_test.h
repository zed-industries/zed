#include <assert.h>
#include <limits.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#include "int_types.h"

#ifdef COMPILER_RT_HAS_FLOAT16
#define TYPE_FP16 _Float16
#else
#define TYPE_FP16 uint16_t
#endif

enum EXPECTED_RESULT {
    LESS_0, LESS_EQUAL_0, EQUAL_0, GREATER_0, GREATER_EQUAL_0, NEQUAL_0
};

static inline TYPE_FP16 fromRep16(uint16_t x)
{
#ifdef COMPILER_RT_HAS_FLOAT16
    TYPE_FP16 ret;
    memcpy(&ret, &x, sizeof(ret));
    return ret;
#else
    return x;
#endif
}

static inline float fromRep32(uint32_t x)
{
    float ret;
    memcpy(&ret, &x, 4);
    return ret;
}

static inline double fromRep64(uint64_t x)
{
    double ret;
    memcpy(&ret, &x, 8);
    return ret;
}

#if defined(CRT_HAS_TF_MODE)
static inline tf_float fromRep128(uint64_t hi, uint64_t lo) {
    __uint128_t x = ((__uint128_t)hi << 64) + lo;
    tf_float ret;
    memcpy(&ret, &x, 16);
    return ret;
}
#endif

static inline uint16_t toRep16(TYPE_FP16 x)
{
#ifdef COMPILER_RT_HAS_FLOAT16
    uint16_t ret;
    memcpy(&ret, &x, sizeof(ret));
    return ret;
#else
    return x;
#endif
}

static inline uint32_t toRep32(float x)
{
    uint32_t ret;
    memcpy(&ret, &x, 4);
    return ret;
}

static inline uint64_t toRep64(double x)
{
    uint64_t ret;
    memcpy(&ret, &x, 8);
    return ret;
}

#if defined(CRT_HAS_TF_MODE)
static inline __uint128_t toRep128(tf_float x) {
    __uint128_t ret;
    memcpy(&ret, &x, 16);
    return ret;
}
#endif

static inline int compareResultH(TYPE_FP16 result,
                                 uint16_t expected)
{
    uint16_t rep = toRep16(result);

    if (rep == expected){
        return 0;
    }
    // test other possible NaN representation(signal NaN)
    else if (expected == 0x7e00U){
        if ((rep & 0x7c00U) == 0x7c00U &&
            (rep & 0x3ffU) > 0){
            return 0;
        }
    }
    return 1;
}

static inline int compareResultF(float result,
                                 uint32_t expected)
{
    uint32_t rep = toRep32(result);

    if (rep == expected){
        return 0;
    }
    // test other possible NaN representation(signal NaN)
    else if (expected == 0x7fc00000U){
        if ((rep & 0x7f800000U) == 0x7f800000U &&
            (rep & 0x7fffffU) > 0){
            return 0;
        }
    }
    return 1;
}

static inline int compareResultD(double result,
                                 uint64_t expected)
{
    uint64_t rep = toRep64(result);

    if (rep == expected){
        return 0;
    }
    // test other possible NaN representation(signal NaN)
    else if (expected == 0x7ff8000000000000UL){
        if ((rep & 0x7ff0000000000000UL) == 0x7ff0000000000000UL &&
            (rep & 0xfffffffffffffUL) > 0){
            return 0;
        }
    }
    return 1;
}

#if defined(CRT_HAS_TF_MODE)
// return 0 if equal
// use two 64-bit integers instead of one 128-bit integer
// because 128-bit integer constant can't be assigned directly
static inline int compareResultF128(tf_float result, uint64_t expectedHi,
                                    uint64_t expectedLo) {
    __uint128_t rep = toRep128(result);
    uint64_t hi = rep >> 64;
    uint64_t lo = rep;

    if (hi == expectedHi && lo == expectedLo) {
        return 0;
    }
    // test other possible NaN representation(signal NaN)
    else if (expectedHi == 0x7fff800000000000UL && expectedLo == 0x0UL) {
        if ((hi & 0x7fff000000000000UL) == 0x7fff000000000000UL &&
            ((hi & 0xffffffffffffUL) > 0 || lo > 0)) {
            return 0;
        }
    }
    return 1;
}
#endif

static inline int compareResultCMP(int result,
                                   enum EXPECTED_RESULT expected)
{
    switch(expected){
        case LESS_0:
            if (result < 0)
                return 0;
            break;
        case LESS_EQUAL_0:
            if (result <= 0)
                return 0;
            break;
        case EQUAL_0:
            if (result == 0)
                return 0;
            break;
        case NEQUAL_0:
            if (result != 0)
                return 0;
            break;
        case GREATER_EQUAL_0:
            if (result >= 0)
                return 0;
            break;
        case GREATER_0:
            if (result > 0)
                return 0;
            break;
        default:
            return 1;
    }
    return 1;
}

static inline char *expectedStr(enum EXPECTED_RESULT expected)
{
    switch(expected){
        case LESS_0:
            return "<0";
        case LESS_EQUAL_0:
            return "<=0";
        case EQUAL_0:
            return "=0";
        case NEQUAL_0:
            return "!=0";
        case GREATER_EQUAL_0:
            return ">=0";
        case GREATER_0:
            return ">0";
        default:
            return "";
    }
    return "";
}

static inline TYPE_FP16 makeQNaN16(void)
{
    return fromRep16(0x7e00U);
}

static inline float makeQNaN32(void)
{
    return fromRep32(0x7fc00000U);
}

static inline double makeQNaN64(void)
{
    return fromRep64(0x7ff8000000000000UL);
}

#if HAS_80_BIT_LONG_DOUBLE
static inline xf_float F80FromRep80(uint16_t hi, uint64_t lo) {
  uqwords bits;
  bits.high.all = hi;
  bits.low.all = lo;
  xf_float ret;
  static_assert(sizeof(xf_float) <= sizeof(uqwords), "wrong representation");
  memcpy(&ret, &bits, sizeof(ret));
  return ret;
}

static inline uqwords F80ToRep80(xf_float x) {
  uqwords ret;
  memset(&ret, 0, sizeof(ret));
  memcpy(&ret, &x, sizeof(x));
  // Any bits beyond the first 16 in high are undefined.
  ret.high.all = (uint16_t)ret.high.all;
  return ret;
}

static inline int compareResultF80(xf_float result, uint16_t expectedHi,
                                   uint64_t expectedLo) {
  uqwords rep = F80ToRep80(result);
  // F80 high occupies the lower 16 bits of high.
  assert((uint64_t)(uint16_t)rep.high.all == rep.high.all);
  return !(rep.high.all == expectedHi && rep.low.all == expectedLo);
}

static inline xf_float makeQNaN80(void) {
  return F80FromRep80(0x7fffu, 0xc000000000000000UL);
}

static inline xf_float makeNaN80(uint64_t rand) {
  return F80FromRep80(0x7fffu,
                      0x8000000000000000 | (rand & 0x3fffffffffffffff));
}

static inline xf_float makeInf80(void) {
  return F80FromRep80(0x7fffu, 0x8000000000000000UL);
}

static inline xf_float makeNegativeInf80(void) {
  return F80FromRep80(0xffffu, 0x8000000000000000UL);
}
#endif

#if defined(CRT_HAS_TF_MODE)
static inline tf_float makeQNaN128(void) {
    return fromRep128(0x7fff800000000000UL, 0x0UL);
}
#endif

static inline TYPE_FP16 makeNaN16(uint16_t rand)
{
    return fromRep16(0x7c00U | (rand & 0x7fffU));
}

static inline float makeNaN32(uint32_t rand)
{
    return fromRep32(0x7f800000U | (rand & 0x7fffffU));
}

static inline double makeNaN64(uint64_t rand)
{
    return fromRep64(0x7ff0000000000000UL | (rand & 0xfffffffffffffUL));
}

#if defined(CRT_HAS_TF_MODE)
static inline tf_float makeNaN128(uint64_t rand) {
    return fromRep128(0x7fff000000000000UL | (rand & 0xffffffffffffUL), 0x0UL);
}
#endif

static inline TYPE_FP16 makeInf16(void)
{
    return fromRep16(0x7c00U);
}

static inline TYPE_FP16 makeNegativeInf16(void) { return fromRep16(0xfc00U); }

static inline float makeInf32(void)
{
    return fromRep32(0x7f800000U);
}

static inline float makeNegativeInf32(void)
{
    return fromRep32(0xff800000U);
}

static inline double makeInf64(void)
{
    return fromRep64(0x7ff0000000000000UL);
}

static inline double makeNegativeInf64(void)
{
    return fromRep64(0xfff0000000000000UL);
}

#if defined(CRT_HAS_TF_MODE)
static inline tf_float makeInf128(void) {
    return fromRep128(0x7fff000000000000UL, 0x0UL);
}

static inline tf_float makeNegativeInf128(void) {
    return fromRep128(0xffff000000000000UL, 0x0UL);
}
#endif
