/*
 * Copyright 2021 Google LLC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef DISTRIBUTED_POINT_FUNCTIONS_DPF_INTERNAL_INT_MOD_N_H_
#define DISTRIBUTED_POINT_FUNCTIONS_DPF_INTERNAL_INT_MOD_N_H_

#include <algorithm>
#include <string>
#include <type_traits>

#include "absl/base/config.h"
#include "absl/container/inlined_vector.h"
#include "absl/log/absl_check.h"
#include "absl/numeric/int128.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/string_view.h"
#include "absl/types/span.h"

namespace distributed_point_functions {

namespace dpf_internal {

// Base class holding common functions of IntModN that are independent of the
// template parameter.
class IntModNBase {
 public:
  // Computes the security level achievable when sampling `num_samples` elements
  // with the given `kModulus`.
  //
  static double GetSecurityLevel(int num_samples, absl::uint128 modulus);

  // Checks if the given parameters are consistent and valid for an IntModN.
  //
  // Returns OK for valid parameters, and INVALID_ARGUMENT otherwise.
  static absl::Status CheckParameters(int num_samples, int base_integer_bitsize,
                                      absl::uint128 modulus,
                                      double security_parameter);

  // Computes the number of bytes required to sample `num_samples` integers
  // modulo `kModulus` with an underlying integer type of
  // `base_integer_bitsize`.
  //
  // Returns INVALID_ARGUMENT if the achievable security level with the given
  // parameters is less than `security_parameter`, or if the parameters are
  // invalid.
  static absl::StatusOr<int> GetNumBytesRequired(int num_samples,
                                                 int base_integer_bitsize,
                                                 absl::uint128 modulus,
                                                 double security_parameter);

  // Creates a value of type T from the given `bytes`, using little-endian
  // encoding. Called by SampleFromBytes. Crashes if bytes.size() != sizeof(T).
  //
  // This is a reimplementation of dpf_internal::ConvertBytesTo for integers,
  // to avoid depending on value_type_helpers here.
  template <typename T>
  static T ConvertBytesTo(absl::string_view bytes) {
    ABSL_CHECK(bytes.size() == sizeof(T));
    T out{0};
#ifdef ABSL_IS_LITTLE_ENDIAN
    std::copy_n(bytes.begin(), sizeof(T), reinterpret_cast<char*>(&out));
#else
    for (int i = sizeof(T) - 1; i >= 0; --i) {
      out |= absl::bit_cast<uint8_t>(bytes[i]);
      out <<= 8;
    }
#endif
    return out;
  }
};

template <typename BaseInteger, typename ModulusType, ModulusType kModulus>
class IntModNImpl : public IntModNBase {
  static_assert(sizeof(BaseInteger) <= sizeof(absl::uint128),
                "BaseInteger may be at most 128 bits large");
  static_assert(
      std::is_same<BaseInteger, absl::uint128>::value ||
#ifdef ABSL_HAVE_INTRINSIC_INT128
          // std::is_unsigned_v<unsigned __int128> is not true everywhere:
          // https://quuxplusone.github.io/blog/2019/02/28/is-int128-integral/#signedness
          std::is_same<BaseInteger, unsigned __int128>::value ||
#endif
          std::is_unsigned<BaseInteger>::value,
      "BaseInteger must be unsigned");
  static_assert(kModulus <= ModulusType(BaseInteger(-1)),
                "kModulus must fit in BaseInteger");

 public:
  using Base = BaseInteger;

  constexpr IntModNImpl() : value_(0) {}
  explicit constexpr IntModNImpl(BaseInteger value)
      : value_(value % kModulus) {}

  // Copyable.
  constexpr IntModNImpl(const IntModNImpl& a) = default;

  constexpr IntModNImpl& operator=(const IntModNImpl& a) = default;

  // Assignment operators.
  constexpr IntModNImpl& operator=(const BaseInteger& a) {
    value_ = a % kModulus;
    return *this;
  }

  constexpr IntModNImpl& operator+=(const IntModNImpl& a) {
    AddBaseInteger(a.value_);
    return *this;
  }

  constexpr IntModNImpl& operator-=(const IntModNImpl& a) {
    SubtractBaseInteger(a.value_);
    return *this;
  }

  // Returns the underlying representation as a BaseInteger.
  constexpr BaseInteger value() const { return value_; }

  // Returns the modulus of this IntModNImpl type.
  static constexpr BaseInteger modulus() { return kModulus; }

  // Returns the number of (pseudo)random bytes required to extract
  // `num_samples` samples r1, ..., rn
  // so that the stream r1, ..., rn is close to a truly (pseudo) random
  // sequence up to total variation distance < 2^(-`security_parameter`)
  static absl::StatusOr<int> GetNumBytesRequired(int num_samples,
                                                 double security_parameter) {
    return IntModNBase::GetNumBytesRequired(
        num_samples, 8 * sizeof(BaseInteger), kModulus, security_parameter);
  }

  // Extracts `samples.size()` samples r1, ..., rn so that the stream r1, ...,
  // rn is close to a truly (pseudo) random sequence up to total variation
  // distance < 2^(-`security_parameter`). Returns r1, ..., rn in `samples`.
  //
  // The optional template argument allows users to specify the number of
  // samples at compile time, which can save heap allocations.
  //
  // Caution: For performance reasons, this function does not check whether
  // `bytes` is long enough for the required number of samples and security
  // parameter. Use `GetNumBytesRequired` or `SampleFromBytes` if such checks
  // are needed.
  //
  template <int kCompiledNumSamples = 1>
  static void UnsafeSampleFromBytes(absl::string_view bytes,
                                    double security_parameter,
                                    absl::Span<IntModNImpl> samples) {
    static_assert(kCompiledNumSamples >= 1,
                  "kCompiledNumSamples must be positive");
    absl::uint128 r = ConvertBytesTo<absl::uint128>(bytes.substr(0, 16));
    absl::InlinedVector<BaseInteger, std::max(1, kCompiledNumSamples - 1)>
        randomness(samples.size() - 1);
    for (int i = 0; i < static_cast<int>(randomness.size()); ++i) {
      randomness[i] = ConvertBytesTo<BaseInteger>(
          bytes.substr(16 + i * sizeof(BaseInteger), sizeof(BaseInteger)));
    }
    for (int i = 0; i < static_cast<int>(samples.size()); ++i) {
      samples[i] = IntModNImpl(static_cast<BaseInteger>(r % kModulus));
      if (i < static_cast<int>(randomness.size())) {
        r /= kModulus;
        if (sizeof(BaseInteger) < sizeof(absl::uint128)) {
          r <<= (sizeof(BaseInteger) * 8);
        }
        r |= randomness[i];
      }
    }
  }

  //  Checks that length(`bytes`) is enough to extract
  // `samples.size()` samples r1, ..., rn
  //  so that the stream r1, ..., rn is close to a truly (pseudo) random
  //  sequence up to total variation distance < 2^(-`security_parameter`) and
  //  fails if that is not the case.
  //  Otherwise returns r1, ..., rn in `samples`.
  static absl::Status SampleFromBytes(absl::string_view bytes,
                                      double security_parameter,
                                      absl::Span<IntModNImpl> samples) {
    if (samples.empty()) {
      return absl::InvalidArgumentError(
          "The number of samples required must be > 0");
    }
    absl::StatusOr<int> num_bytes_lower_bound =
        GetNumBytesRequired(samples.size(), security_parameter);
    if (!num_bytes_lower_bound.ok()) {
      return num_bytes_lower_bound.status();
    }
    if (*num_bytes_lower_bound > bytes.size()) {
      return absl::InvalidArgumentError(
          absl::StrCat("The number of bytes provided (", bytes.size(),
                       ") is insufficient for the required "
                       "statistical security and number of samples."));
    }
    UnsafeSampleFromBytes(bytes, security_parameter, samples);
    return absl::OkStatus();
  }

 private:
  constexpr void SubtractBaseInteger(const BaseInteger& a) {
    if (value_ >= a) {
      value_ -= a;
    } else {
      value_ = kModulus - a + value_;
    }
  }

  constexpr void AddBaseInteger(const BaseInteger& a) {
    SubtractBaseInteger(kModulus - a);
  }

  BaseInteger value_;
};

template <typename BaseInteger, typename ModulusType, ModulusType kModulus>
constexpr IntModNImpl<BaseInteger, ModulusType, kModulus> operator+(
    IntModNImpl<BaseInteger, ModulusType, kModulus> a,
    const IntModNImpl<BaseInteger, ModulusType, kModulus>& b) {
  a += b;
  return a;
}

template <typename BaseInteger, typename ModulusType, ModulusType kModulus>
constexpr IntModNImpl<BaseInteger, ModulusType, kModulus> operator-(
    IntModNImpl<BaseInteger, ModulusType, kModulus> a,
    const IntModNImpl<BaseInteger, ModulusType, kModulus>& b) {
  a -= b;
  return a;
}

template <typename BaseInteger, typename ModulusType, ModulusType kModulus>
constexpr IntModNImpl<BaseInteger, ModulusType, kModulus> operator-(
    IntModNImpl<BaseInteger, ModulusType, kModulus> a) {
  IntModNImpl<BaseInteger, ModulusType, kModulus> result(BaseInteger{0});
  result -= a;
  return result;
}

template <typename BaseInteger, typename ModulusType, ModulusType kModulus>
constexpr bool operator==(
    const IntModNImpl<BaseInteger, ModulusType, kModulus>& a,
    const IntModNImpl<BaseInteger, ModulusType, kModulus>& b) {
  return a.value() == b.value();
}

template <typename BaseInteger, typename ModulusType, ModulusType kModulus>
constexpr bool operator!=(
    const IntModNImpl<BaseInteger, ModulusType, kModulus>& a,
    const IntModNImpl<BaseInteger, ModulusType, kModulus>& b) {
  return !(a == b);
}

}  // namespace dpf_internal

// Since `absl::uint128` is not an alias to `unsigned __int128`, but a struct,
// we cannot use it as a template parameter type. So if we have an intrinsic
// int128, we always use that as the modulus type. Otherwise, the modulus type
// is the same as BaseInteger.
#ifdef ABSL_HAVE_INTRINSIC_INT128
template <typename BaseInteger, unsigned __int128 kModulus>
using IntModN =
    dpf_internal::IntModNImpl<BaseInteger, unsigned __int128, kModulus>;
#else
template <typename BaseInteger, BaseInteger kModulus>
using IntModN = dpf_internal::IntModNImpl<BaseInteger, BaseInteger, kModulus>;
#endif

}  // namespace distributed_point_functions
#endif  // DISTRIBUTED_POINT_FUNCTIONS_DPF_INTERNAL_INT_MOD_N_H_
