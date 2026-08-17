/*
 * Copyright 2020 Google LLC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// Traits class for generically implementing scalar and multichannel filters.

#ifndef AUDIO_LINEAR_FILTERS_FILTER_TRAITS_H_
#define AUDIO_LINEAR_FILTERS_FILTER_TRAITS_H_

#include <type_traits>

#include "audio/dsp/types.h"
#include "glog/logging.h"
#include "third_party/eigen3/Eigen/Core"

#include "audio/dsp/porting.h"  // auto-added.


namespace linear_filters {
namespace internal {

// Gets the filter coefficient type for scalar input.
// GetCoefficientType<float>::Type is float.
// GetCoefficientType<double>::Type is double.
// GetCoefficientType<complex<float>>::Type is float.
template <typename ScalarType>
struct GetCoefficientType {
  using Type = typename audio_dsp::RealType<ScalarType>::Type;
};

// This (and the following) struct determine the filter coefficient type for
// Eigen Array and Matrix input.
// GetCoefficientType<ArrayXf>::Type is float.
// GetCoefficientType<MatrixXd>::Type is double.
// GetCoefficientType<ArrayXXcd>::Type is float.
template <typename Scalar, int Rows, int Cols, int Options, int MaxRows,
          int MaxCols>
struct GetCoefficientType<
    typename Eigen::Array<Scalar, Rows, Cols, Options, MaxRows, MaxCols>> {
  using Type = typename audio_dsp::RealType<Scalar>::Type;
};

template <typename Scalar, int Rows, int Cols, int Options, int MaxRows,
          int MaxCols>
struct GetCoefficientType<
    typename Eigen::Matrix<Scalar, Rows, Cols, Options, MaxRows, MaxCols>> {
  using Type = typename audio_dsp::RealType<Scalar>::Type;
};

// IsEigenType<SomeType>::Value determines whether SomeType is an Eigen::Array
// or Eigen::Matrix type. The default definition is false.
template <typename NonEigenType>
struct IsEigenType {
  static constexpr bool Value = false;
};

// Define IsEigenType<Eigen::Array<...>>::Value = true.
template <typename Scalar, int Rows, int Cols,
          int Options, int MaxRows, int MaxCols>
struct IsEigenType<typename Eigen::Array<
    Scalar, Rows, Cols, Options, MaxRows, MaxCols>> {
  static constexpr bool Value = true;
};

// Define IsEigenType<Eigen::Matrix<...>>::Value = true. This makes sure that
// IsEigenType works correctly with Eigen::Vector* types.
template <typename Scalar, int Rows, int Cols,
          int Options, int MaxRows, int MaxCols>
struct IsEigenType<typename Eigen::Matrix<
    Scalar, Rows, Cols, Options, MaxRows, MaxCols>> {
  static constexpr bool Value = true;
};

// Traits for single channel filters where SampleType is a scalar type.
template <typename _SampleType, bool IsEigenType>
struct FilterTraits {
 public:
  // These using definitions record the template parameters in
  // FilterTraits::CoefficientType and FilterTraits::SampleType. The template
  // parameters are prefixed with underscore since otherwise we would get
  // "declaration shadows template parameter" compile errors.
  using CoefficientType = typename GetCoefficientType<_SampleType>::Type;
  using SampleType = _SampleType;

  static constexpr int kNumChannelsAtCompileTime = 1;
  // Determine a type appropriate for accumulating the filter output as the type
  // resulting from the product of CoefficientType * SampleType.
  using AccumType = decltype(CoefficientType() * SampleType());
  // Biquads only.
  using BiquadStateType =
      Eigen::Matrix<AccumType, kNumChannelsAtCompileTime, 2>;
  // Ladder filters only.
  using LadderStateType =
      Eigen::Matrix<AccumType, kNumChannelsAtCompileTime, Eigen::Dynamic>;

  // The following helper structs are implemented differently for scalar vs.
  // multichannel filters. See the specialization of FilterTraits below for
  // IsEigenType = true.

  // Helper to get the scalar type from a sample type. For a single channel
  // filter, GetScalarType<ScalarType>::Type is trivially ScalarType.
  template <typename ScalarType>
  struct GetScalarType {
    using Type = ScalarType;
  };

  // Helper to get a data pointer to a sample. For a single channel filter,
  // GetData(scalar) returns &scalar.
  template <typename ScalarType>
  static const ScalarType* GetData(const ScalarType& scalar) {
    return &scalar;
  }
  // Like GetData(), but returning a nonconst pointer.
  template <typename ScalarType>
  static ScalarType* GetMutableData(ScalarType* scalar) {
    return scalar;
  }

  // Interpret scalar as a 1x1 const Eigen Array. This wrapper is needed because
  // some Eigen operations require both operands to be Eigen types.
  template <typename ScalarType>
  static Eigen::Map<const Eigen::Array<ScalarType, 1, 1>> AsEigenArray(
      const ScalarType& scalar) {
    return Eigen::Map<const Eigen::Array<ScalarType, 1, 1>>(&scalar, 1, 1);
  }
  // Interpret scalar as a 1x1 mutable Eigen Array.
  template <typename ScalarType>
  static Eigen::Array<ScalarType, 1, 1>* AsMutableEigenArray(
      ScalarType* scalar) {
    return reinterpret_cast<Eigen::Array<ScalarType, 1, 1>*>(scalar);
  }

  // Statically determine the number of channels at compile time.
  template <typename InputType, typename OutputType>
  static constexpr int GetFixedNumChannels() {
    return 1;  // Always 1 for scalar SampleType.
  }

  // Process a block of samples using filter->ProcessSample().
  template <typename FilterType, typename InputType, typename OutputType>
  static void ProcessBlock(FilterType* filter,
                           const InputType& input, OutputType* output) {
    ABSL_DCHECK(output != nullptr);
    ABSL_DCHECK_EQ(filter->num_channels(), 1);
    output->resize(input.size());
    for (int n = 0; n < input.size(); ++n) {
      filter->ProcessSample(input[n], &(*output)[n]);
    }
  }
};

// Traits for multichannel filters where SampleType is an Eigen Array or Vector.
template <typename _SampleType>
struct FilterTraits<_SampleType, true> {
 public:
  using CoefficientType = typename GetCoefficientType<_SampleType>::Type;
  using SampleType = _SampleType;
  static constexpr int kNumChannelsAtCompileTime =
      SampleType::RowsAtCompileTime;
  using AccumType = decltype(CoefficientType() * SampleType()[0]);
  // Biquad filters only.
  using BiquadStateType =
      Eigen::Matrix<AccumType, kNumChannelsAtCompileTime, 2>;
  // Ladder filters only.
  using LadderStateType =
      Eigen::Matrix<AccumType, kNumChannelsAtCompileTime, Eigen::Dynamic>;

  // Helper to get the scalar type from a sample type. For a multichannel
  // filter, GetScalarType<ArrayType>::Type is ArrayType::Scalar.
  template <typename ArrayType>
  struct GetScalarType {
    using Type = typename ArrayType::Scalar;
  };

  // Get a const data pointer to a sample.
  template <typename ArrayType>
  static const typename ArrayType::Scalar* GetData(const ArrayType& array) {
    return array.data();
  }
  // Like GetData(), but returning a nonconst pointer.
  template <typename ArrayType>
  static typename ArrayType::Scalar* GetMutableData(ArrayType* array) {
    return array->data();
  }

  // Trivial function to interpet a sample as a const Eigen array.
  template <typename ArrayType>
  static const ArrayType& AsEigenArray(const ArrayType& array) {
    return array;
  }
  // Trivial function to interpet a sample as a mutable Eigen array.
  template <typename ArrayType>
  static ArrayType* AsMutableEigenArray(ArrayType* array) {
    return array;
  }

  // If possible, statically determine the number of channels from
  // kNumChannelsAtCompileTime, or if kNumChannelsAtCompileTime is
  // Eigen::Dynamic, from InputType::RowsAtCompileTime or
  // OutputType::RowsAtCompileTime.
  template <typename InputType, typename OutputType>
  static constexpr int GetFixedNumChannels() {
    return (kNumChannelsAtCompileTime != Eigen::Dynamic)
        ? kNumChannelsAtCompileTime
        : ((InputType::RowsAtCompileTime != Eigen::Dynamic)
           ? static_cast<int>(InputType::RowsAtCompileTime)
           : static_cast<int>(OutputType::RowsAtCompileTime));
  }

  // Process a block of samples where the number of channels is determined
  // dynamically at run time.
  template <typename FilterType, typename InputType, typename OutputType>
  static inline void ProcessBlockDynamic(
      FilterType* filter, const InputType& input, OutputType* output) {
    for (int n = 0; n < input.cols(); ++n) {
      // We can't pass &output->col(n) below because that uses an address of
      // temporary. So instead a map of output->col(n) is passed.
      Eigen::Map<Eigen::Array<typename OutputType::Scalar,
          Eigen::Dynamic, 1>> output_col_n(
              output->col(n).data(), input.rows());
      filter->ProcessSample(input.col(n), &output_col_n);
    }
  }

  // Process a block of samples where the number of channels is fixed at compile
  // time to kFixedNumChannels.
  template <int kFixedNumChannels, typename FilterType,
            typename InputType, typename OutputType>
  static inline void ProcessBlockFixed(
      FilterType* filter, const InputType& input, OutputType* output) {
    for (int n = 0; n < input.cols(); ++n) {
      // Map input and output with kFixedNumChannels to ensure that
      // ProcessSamples() knows the number of channels at compile time
      // (Necessary e.g. for the special cases in ProcessBlockDynamicHelper).
      Eigen::Map<
          const Eigen::Array<typename InputType::Scalar, kFixedNumChannels, 1>>
          input_sample(input.col(n).data(), kFixedNumChannels, 1);
      Eigen::Map<
          Eigen::Array<typename OutputType::Scalar, kFixedNumChannels, 1>>
          output_sample(output->col(n).data(), kFixedNumChannels, 1);
      filter->ProcessSample(input_sample, &output_sample);
    }
  }

  // Helper to process a block with a dynamic number of channels. [This struct
  // is needed because std::conditional selects between types, not functions.]
  struct ProcessBlockDynamicHelper {
    template <int kFixedNumChannels, typename FilterType,
              typename InputType, typename OutputType>
    static void Run(FilterType* filter,
                    const InputType& input, OutputType* output) {
      // Even though the number of channels is dynamic, it is possible to apply
      // Eigen::Map to interpret the input and output arrays as having a fixed
      // number of rows at compile time for 10x performance improvement. The
      // switch statement makes special cases to do this for 1 to 4 channels.

      if (filter->num_channels() == 1) {
        ProcessBlockFixed<1>(filter, input, output);
        return;
      }
      ABSL_CHECK_EQ(input.innerStride(), 1)
          << "Cannot operate on map with inner stride.";
      ABSL_CHECK_EQ(output->innerStride(), 1)
          << "Cannot operate on map with inner stride.";
      switch (filter->num_channels()) {
        case 2:
          ProcessBlockFixed<2>(filter, input, output);
          break;
        case 3:
          ProcessBlockFixed<3>(filter, input, output);
          break;
        case 4:
          ProcessBlockFixed<4>(filter, input, output);
          break;
        default:
          // Fall back to general implementation for dynamic number of channels.
          ProcessBlockDynamic(filter, input, output);
      }
    }
  };

  // Helper to process a block with a fixed number of channels, which just calls
  // ProcessBlockFixed().
  struct ProcessBlockFixedHelper {
    template <int kFixedNumChannels, typename FilterType,
              typename InputType, typename OutputType>
    static void Run(FilterType* filter,
                    const InputType& input, OutputType* output) {
      ProcessBlockFixed<kFixedNumChannels>(filter, input, output);
    }
  };

  // Process a block of samples using filter->ProcessSample(). If possible, the
  // number of channels is determined at compile time for more efficient
  // computation.
  template <typename FilterType, typename InputType, typename OutputType>
  static void ProcessBlock(FilterType* filter,
                           const InputType& input, OutputType* output) {
    ABSL_CHECK_EQ(filter->num_channels(), input.rows());
    ABSL_DCHECK(output != nullptr);
    output->resize(input.rows(), input.cols());
    constexpr int kFixedNumChannels =
        GetFixedNumChannels<InputType, OutputType>();
    // This conditional is effectively a compile-time version of
    //   if (kFixedNumChannels == Eigen::Dynamic) {
    //     ProcessBlockDynamicHelper::Run<kFixedNumChannels>(
    //         filter, input, output);
    //   } else {
    //     ProcessBlockFixedHelper::Run<kFixedNumChannels>(
    //         filter, input, output);
    //   }
    // For some SampleType, instantiating ProcessBlockDynamicHelper would
    // produce compile errors, so this selection must be done statically.
    std::conditional<kFixedNumChannels == Eigen::Dynamic,
        ProcessBlockDynamicHelper,
        ProcessBlockFixedHelper
        >::type::template Run<kFixedNumChannels>(filter, input, output);
  }
};

}  // namespace internal
}  // namespace linear_filters

#endif  // AUDIO_LINEAR_FILTERS_FILTER_TRAITS_H_
