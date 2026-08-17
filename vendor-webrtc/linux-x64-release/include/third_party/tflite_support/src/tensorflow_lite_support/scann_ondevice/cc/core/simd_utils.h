/* Copyright 2022 The TensorFlow Authors. All Rights Reserved.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
==============================================================================*/

#ifndef TENSORFLOW_LITE_SUPPORT_SCANN_ONDEVICE_CC_CORE_SIMD_UTILS_H_
#define TENSORFLOW_LITE_SUPPORT_SCANN_ONDEVICE_CC_CORE_SIMD_UTILS_H_

#include <cstdint>
#ifdef __SSE__
#include <x86intrin.h>
#endif
#ifdef __ARM_NEON
#include <arm_neon.h>
#endif

#include <cmath>
#include <memory>

namespace tflite {
namespace scann_ondevice {
namespace core {
class SimdFloat32x1 {
  float value_;

 public:
  static constexpr size_t size() { return 1; }

  void setzero() { value_ = 0.0f; }

  void load(const float* mem) { value_ = *mem; }
  void dequantize_accum_storeu(float* mem, float, float) const {
    *mem += value_;
  }
  SimdFloat32x1& operator+=(const SimdFloat32x1& rhs) {
    value_ += rhs.value_;
    return *this;
  }
};
#ifdef __SSE__
class SimdFloat32x4 {
  __m128 value_;

 public:
  static constexpr size_t size() { return 4; }

  void setzero() { value_ = _mm_setzero_ps(); }
  void load(const float* mem) { value_ = _mm_load_ps(mem); }
  void loadu(const float* mem) { value_ = _mm_loadu_ps(mem); }
  void storeu(float* mem) const { _mm_storeu_ps(mem, value_); }

  void dequantize_accum_storeu(float* mem, float, float) const {
    SimdFloat32x4 simd;
    simd.loadu(mem);
    simd += *this;
    simd.storeu(mem);
  }

  SimdFloat32x4& operator+=(const SimdFloat32x4& rhs) {
    value_ = _mm_add_ps(rhs.value_, value_);
    return *this;
  }
};
#endif
#ifdef __AVX__
class SimdFloat32x8 {
  __m256 value_;

 public:
  static constexpr size_t size() { return 8; }

  void setzero() { value_ = _mm256_setzero_ps(); }
  void load(const float* mem) { value_ = _mm256_load_ps(mem); }

  void loadu(const float* mem) { value_ = _mm256_loadu_ps(mem); }

  void storeu(float* mem) { _mm256_storeu_ps(mem, value_); }

  void dequantize_accum_storeu(float* mem, float, float) const {
    SimdFloat32x8 simd;
    simd.loadu(mem);
    simd += *this;
    simd.storeu(mem);
  }

  SimdFloat32x8& operator+=(const SimdFloat32x8& rhs) {
    value_ = _mm256_add_ps(rhs.value_, value_);
    return *this;
  }
};
#endif
#ifdef __ARM_NEON
class SimdFloat32x4 {
  float32x4_t value_;

 public:
  static constexpr size_t size() { return 4; }

  void setzero() { value_ = vmovq_n_f32(0); }
  void load(const float* mem) { value_ = vld1q_f32(mem); }
  void loadu(const float* mem) { value_ = vld1q_f32(mem); }
  void storeu(float* mem) const { vst1q_f32(mem, value_); }

  void dequantize_accum_storeu(float* mem, float, float) const {
    SimdFloat32x4 simd;
    simd.loadu(mem);
    simd += *this;
    simd.storeu(mem);
  }

  SimdFloat32x4& operator+=(const SimdFloat32x4& rhs) {
    value_ = vaddq_f32(rhs.value_, value_);
    return *this;
  }
};
#endif

class SimdInt16x1 {
  uint16_t value_;

 public:
  static constexpr size_t size() { return 1; }

  void setzero() { value_ = 0; }

  void load(const uint16_t* mem) { value_ = *mem; }

  void load(const uint8_t* mem) { value_ = *mem; }
  void dequantize_accum_storeu(float* mem, float scale, float offset) const {
    *mem += scale * value_ + offset;
  }

  SimdInt16x1& operator+=(const SimdInt16x1& rhs) {
    value_ += rhs.value_;
    return *this;
  }
};
#ifdef __SSE4_1__
class SimdInt16x8 {
  __m128i value_;

 public:
  static constexpr size_t size() { return 8; }

  void setzero() { value_ = _mm_setzero_si128(); }
  void load(const uint16_t* mem) {
    value_ = _mm_load_si128(reinterpret_cast<const __m128i*>(mem));
  }

  void loadu(const uint16_t* mem) {
    value_ = _mm_loadu_si128(reinterpret_cast<const __m128i*>(mem));
  }
  void load(const uint8_t* mem) {
    __m128i tmp = _mm_loadl_epi64(reinterpret_cast<const __m128i*>(mem));
    value_ = _mm_cvtepu8_epi16(tmp);
  }

  void loadu(const uint8_t* mem) {
    __m128i tmp = _mm_loadl_epi64(reinterpret_cast<const __m128i*>(mem));
    value_ = _mm_cvtepu8_epi16(tmp);
  }
  void dequantize_accum_storeu(float* mem, float scale, float offset) const {
    __m128 dst0 = _mm_loadu_ps(mem);
    __m128 dst1 = _mm_loadu_ps(mem + 4);
    __m128i lo_i16 = value_;
    __m128i hi_i16 = _mm_unpackhi_epi64(value_, value_);
    __m128i lo_i32 = _mm_cvtepu16_epi32(lo_i16);
    __m128i hi_i32 = _mm_cvtepu16_epi32(hi_i16);
    __m128 lo_f32 = _mm_cvtepi32_ps(lo_i32);
    __m128 hi_f32 = _mm_cvtepi32_ps(hi_i32);
    __m128 offset_simd = _mm_set1_ps(offset);
    __m128 scale_simd = _mm_set1_ps(scale);
    lo_f32 = _mm_mul_ps(scale_simd, lo_f32);
    hi_f32 = _mm_mul_ps(scale_simd, hi_f32);
    lo_f32 = _mm_add_ps(lo_f32, offset_simd);
    hi_f32 = _mm_add_ps(hi_f32, offset_simd);
    dst0 = _mm_add_ps(dst0, lo_f32);
    dst1 = _mm_add_ps(dst1, hi_f32);
    _mm_storeu_ps(mem, dst0);
    _mm_storeu_ps(mem + 4, dst1);
  }

  SimdInt16x8& operator+=(const SimdInt16x8& rhs) {
    value_ = _mm_add_epi16(rhs.value_, value_);
    return *this;
  }
};
#endif
#ifdef __ARM_NEON
class SimdInt16x8 {
  uint16x8_t value_;

 public:
  static constexpr size_t size() { return 8; }

  void setzero() { value_ = vmovq_n_u16(0); }
  void load(const uint16* mem) { value_ = vld1q_u16(mem); }

  void loadu(const uint16* mem) { value_ = vld1q_u16(mem); }
  void load(const uint8* mem) {
    uint8x8_t tmp = vld1_u8(mem);
    value_ = vmovl_u8(tmp);
  }

  void loadu(const uint8* mem) {
    uint8x8_t tmp = vld1_u8(mem);
    value_ = vmovl_u8(tmp);
  }
  void dequantize_accum_storeu(float* mem, float scale, float offset) const {
    float32x4_t dst0 = vld1q_f32(mem);
    float32x4_t dst1 = vld1q_f32(mem + 4);
    uint16x4_t lo_i16 = vget_low_u16(value_);
    uint16x4_t hi_i16 = vget_high_u16(value_);
    uint32x4_t lo_i32 = vmovl_u16(lo_i16);
    uint32x4_t hi_i32 = vmovl_u16(hi_i16);
    float32x4_t lo_f32 = vcvtq_f32_u32(lo_i32);
    float32x4_t hi_f32 = vcvtq_f32_u32(hi_i32);
    float32x4_t offset_simd = vdupq_n_f32(offset);
    float32x4_t scale_simd = vdupq_n_f32(scale);
    lo_f32 = vmlaq_f32(offset_simd, scale_simd, lo_f32);
    hi_f32 = vmlaq_f32(offset_simd, scale_simd, hi_f32);
    dst0 = vaddq_f32(dst0, lo_f32);
    dst1 = vaddq_f32(dst1, hi_f32);
    vst1q_f32(mem, dst0);
    vst1q_f32(mem + 4, dst1);
  }

  SimdInt16x8& operator+=(const SimdInt16x8& rhs) {
    value_ = vaddq_u16(rhs.value_, value_);
    return *this;
  }
};
#endif
#ifdef __AVX2__
class SimdInt16x16 {
  __m256i value_;

 public:
  static constexpr size_t size() { return 16; }

  void setzero() { value_ = _mm256_setzero_si256(); }

  void load(const uint16* mem) {
    value_ = _mm256_load_si256(reinterpret_cast<const __m256i*>(mem));
  }

  void loadu(const uint16* mem) {
    value_ = _mm256_loadu_si256(reinterpret_cast<const __m256i*>(mem));
  }

  void load(const uint8* mem) {
    __m128i tmp = _mm_load_si128(reinterpret_cast<const __m128i*>(mem));
    value_ = _mm256_cvtepu8_epi16(tmp);
  }

  void loadu(const uint8* mem) {
    __m128i tmp = _mm_loadu_si128(reinterpret_cast<const __m128i*>(mem));
    value_ = _mm256_cvtepu8_epi16(tmp);
  }

  void dequantize_accum_storeu(float* mem, float scale, float offset) const {
    __m256 dst0 = _mm256_loadu_ps(mem);
    __m256 dst1 = _mm256_loadu_ps(mem + 8);
    __m128i lo_i16 = _mm256_castsi256_si128(value_);
    __m128i hi_i16 = _mm256_extractf128_si256(value_, 1);
    __m256i lo_i32 = _mm256_cvtepu16_epi32(lo_i16);
    __m256i hi_i32 = _mm256_cvtepu16_epi32(hi_i16);
    __m256 lo_f32 = _mm256_cvtepi32_ps(lo_i32);
    __m256 hi_f32 = _mm256_cvtepi32_ps(hi_i32);
    __m256 offset_simd = _mm256_set1_ps(offset);
    __m256 scale_simd = _mm256_set1_ps(scale);
    lo_f32 = _mm256_fmadd_ps(scale_simd, lo_f32, offset_simd);
    hi_f32 = _mm256_fmadd_ps(scale_simd, hi_f32, offset_simd);
    dst0 = _mm256_add_ps(dst0, lo_f32);
    dst1 = _mm256_add_ps(dst1, hi_f32);
    _mm256_storeu_ps(mem, dst0);
    _mm256_storeu_ps(mem + 8, dst1);
  }

  SimdInt16x16& operator+=(const SimdInt16x16& rhs) {
    value_ = _mm256_add_epi16(rhs.value_, value_);
    return *this;
  }
};
#endif

}  // namespace core
}  // namespace scann_ondevice
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_SCANN_ONDEVICE_CC_CORE_SIMD_UTILS_H_
