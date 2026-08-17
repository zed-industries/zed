/*
 * Copyright 2020-2021 Google LLC
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

// Utilities for working generically with Eigen types.

#ifndef AUDIO_DSP_EIGEN_TYPES_H_
#define AUDIO_DSP_EIGEN_TYPES_H_

#include <type_traits>
#include <vector>

#include "absl/base/optimization.h"
#include "absl/meta/type_traits.h"
#include "absl/types/span.h"
#include "third_party/eigen3/Eigen/Core"

#include "audio/dsp/porting.h"  // auto-added.

namespace audio_dsp {

// If EigenType `x` is a column vector at compile time, TransposeToRowVector(x)
// returns x.transpose(). Otherwise, `x` is returned unchanged.
template <typename EigenType>
auto TransposeToRowVector(
    EigenType&& x,
    typename std::enable_if<  // If `x` is a column vector at compile time.
        absl::decay_t<EigenType>::ColsAtCompileTime == 1>::type* = nullptr)
    -> decltype(x.transpose()) {
  return x.transpose();  // Return Eigen::Transpose expression object by value.
}
template <typename EigenType>
EigenType&& TransposeToRowVector(
    EigenType&& x,
    typename std::enable_if<  // If `x` is not a column vector.
        absl::decay_t<EigenType>::ColsAtCompileTime != 1>::type* = nullptr) {
  return std::forward<EigenType>(x);  // Forward the object without change.
}

// Traits class to test at compile time whether a type is a contiguous 1D Eigen
// type, such that the ith element is accessible through .data()[i].
//
// Examples:
//   IsContiguous1DEigenType<ArrayXf>::Value  // = true
//   IsContiguous1DEigenType<Map<VectorXf>>::Value  // = true
//   IsContiguous1DEigenType<std::vector<float>>::Value  // = false
template <typename Type, typename = void>
struct IsContiguous1DEigenType { enum { Value = false }; };

namespace internal {
// For use with SFINAE to enable only when T is an Eigen::DenseBase type.
template <typename T> void WellFormedIfDenseEigenType(Eigen::DenseBase<T>&&);
}  // namespace internal

template <typename EigenType>
struct IsContiguous1DEigenType<EigenType,
    decltype(internal::WellFormedIfDenseEigenType(std::declval<EigenType>()))> {
  enum {
    Value = EigenType::IsVectorAtCompileTime &&
        EigenType::InnerStrideAtCompileTime == 1
  };
};

namespace internal {
template <typename Container> class ContainerWrapper;
template <typename Container>
struct ContainerWrapperTraits { enum { Valid = false }; };
}  // namespace internal

// Wrap std::vector, absl::Span, and Eigen types with a uniform interface.
// WrapContainer() returns a `ContainerWrapper` (defined below) to make a
// uniform interface for accessing the size and resizing the wrapped object and
// also to return an Eigen Matrix representation (e.g. an Eigen::Map for vectors
// and Spans).
//
// The following container types are supported:
//
//  * Resizable containers
//    * std::vector
//    * Eigen::Array
//    * Eigen::Matrix, Eigen::Vector, Eigen::RowVector
//
//  * Non-resizable containers
//    * absl::Span
//    * Eigen::VectorBlock: the expression objects created by
//      .head(), .segment(), .tail()
//    * Eigen::Block: the expression objects created by
//      .row(), .leftCols(), .block(), etc.
//    * Eigen::CwiseNullaryOp: expression objects like Zero, Ones, and Random.
//
// This wrapper is useful for writing generic code in a form like:
//
//   template <typename Output>
//   void Foo(Output&& output) {  // Public interface.
//     FooWrapped(WrapContainer(std::forward<Output>(output)));
//   }
//
//   template <typename WrappedOutput>
//   void FooWrapped(WrappedOutput&& output) {
//     if constexpr (Output::Dims == 1) {  // Output is a 1D type.
//       ABSL_CHECK(output.resize(rows * cols));
//     } else {  // Output is a 2D type.
//       ABSL_CHECK(output.resize(rows, cols));
//     }
//     FooImpl(output.AsMatrix(rows));
//   }
//
//   template <typename EigenMatrixType>
//   void FooImpl(EigenMatrixType&& output) { ... }  // Implementation.
//
//   // Calling code.
//   std::vector<float> x;
//   Foo(x);
//
// WARNING: The container being wrapped must live as long as the wrapper. One
// way to ensure this is to pass the wrapper immediately into a function:
//
//   // `input` lives at least throughout the scope of Fun().
//   Fun(WrapContainer(input));  // OK.
//
// For comparison, do not do this:
//
//   // Span returned by MakeSpan is destroyed after this line.
//   auto wrapper = WrapContainer(absl::MakeSpan(buffer));  // WRONG.
//   // (Code using wrapper.)
//
// Code using the wrapper attempts to use a dead reference, which is undefined
// behavior. Also be careful when wrapping Eigen expressions. Do not do this:
//
//   Eigen::MatrixXf m;
//   // Block expression returned by m.row(0) is destroyed after this line.
//   auto wrapper = WrapContainer(m.row(0));  // WRONG.
//   // (Code using wrapper.)
template <typename Container>
auto WrapContainer(Container&& container)
    -> decltype(internal::ContainerWrapper<
                typename std::remove_reference<Container>::type>(
        std::forward<Container>(container))) {
  return internal::ContainerWrapper<
      typename std::remove_reference<Container>::type>(
      std::forward<Container>(container));
}

namespace internal {

// A nonresizable vector at compile time.
template <typename Container, int Case>
struct ContainerWrapperResize1D {
  static bool Resize(Container& c, Eigen::Index new_size) {
    return static_cast<Eigen::Index>(c.size()) == new_size;
  }
};
// A resizable vector at compile time.
template <typename Container>
struct ContainerWrapperResize1D<Container, 1> {
  static bool Resize(Container& c, Eigen::Index new_size) {
    if (ABSL_PREDICT_FALSE(static_cast<Eigen::Index>(c.size()) != new_size)) {
      c.resize(new_size);
    }
    return true;
  }
};
// Not a vector at compile time.
template <typename Container>
struct ContainerWrapperResize1D<Container, 2> {
  static bool Resize(Container& c, Eigen::Index new_size) {
    return false;
  }
};

// A nonresizable 2D type.
template <typename Container, int Case>
struct ContainerWrapperResize2D {
  static bool Resize(Container& c, Eigen::Index new_rows,
                     Eigen::Index new_cols) {
    return c.rows() == new_rows && c.cols() == new_cols;
  }
};
// A resizable 2D type.
template <typename Container>
struct ContainerWrapperResize2D<Container, 1> {
  static bool Resize(Container& c, Eigen::Index new_rows,
                     Eigen::Index new_cols) {
    if (ABSL_PREDICT_FALSE(c.rows() != new_rows || c.cols() != new_cols)) {
      c.resize(new_rows, new_cols);
    }
    return true;
  }
};
// Not a 2D type.
template <typename Container>
struct ContainerWrapperResize2D<Container, 2> {
  static bool Resize(Container& c, Eigen::Index new_rows,
                     Eigen::Index new_cols) {
    return false;
  }
};

// Not a 2D type.
template <typename Container, bool Is2D>
struct ContainerWrapperGetShape {
  static Eigen::Index rows(const Container& c) { return 0; }
  static Eigen::Index cols(const Container& c) { return 0; }
};
// A 2D type.
template <typename Container>
struct ContainerWrapperGetShape<Container, true> {
  static Eigen::Index rows(const Container& c) { return c.rows(); }
  static Eigen::Index cols(const Container& c) { return c.cols(); }
};

template <typename Container_>
class ContainerWrapper {
 public:
  using Container = typename std::remove_reference<Container_>::type;
  using Traits =
      internal::ContainerWrapperTraits<typename absl::decay_t<Container>>;
  static_assert(Traits::Valid, "Invalid type for ContainerWrapper.");
  enum {
    // The number of container dimensions, either 1 or 2. Returns 2 for all
    // Eigen types, even those that are vectors at compile time.
    //
    //   std::vector<T>  => 1
    //   Eigen::MatrixXf => 2
    //   Eigen::VectorXf => 2
    Dims = Traits::Dims,
    // Number of rows at compile time. Returns Eigen::Dynamic if not fixed.
    RowsAtCompileTime = Traits::RowsAtCompileTime,
    // Number of columns at compile time. Returns Eigen::Dynamic if not fixed.
    ColsAtCompileTime = Traits::ColsAtCompileTime,
    // True if the container is 1D or an Eigen vector type.
    IsVectorAtCompileTime = Traits::IsVectorAtCompileTime,
    // Whether the container is resizable. True for non-const std::vector,
    // Eigen::Array, and Eigen::Matrix.
    IsResizable = Traits::IsResizable && !std::is_const<Container>::value,
  };

  explicit ContainerWrapper(Container& c): c_(c) {}
  explicit ContainerWrapper(Container&& c): c_(c) {}

  // size accessor, works for all containers.
  Eigen::Index size() const { return static_cast<Eigen::Index>(c_.size()); }
  // rows and cols accessors, should only be used if Dims == 2.
  Eigen::Index rows() const {
    return ContainerWrapperGetShape<Container, Dims == 2>::rows(c_);
  }
  Eigen::Index cols() const {
    return ContainerWrapperGetShape<Container, Dims == 2>::cols(c_);
  }

  // Resizes a 1D container or checks the size. Returns true on success.
  bool resize(Eigen::Index new_size) {
    return ContainerWrapperResize1D<
      Container, IsVectorAtCompileTime ? IsResizable : 2>::Resize(c_, new_size);
  }

  // Resizes a 2D container or checks the shape. Returns true on success.
  bool resize(Eigen::Index new_rows, Eigen::Index new_cols) {
    return ContainerWrapperResize2D<
      Container, (Dims == 2) ? IsResizable : 2>::Resize(c_, new_rows, new_cols);
  }

  template <int MapRows>
  using Matrix = decltype(Traits::template AsMatrix<MapRows>(
     std::declval<Container&>(), 1));

  // Represent the container as an Eigen Matrix type. For Eigen types, returns
  // `container.matrix()`. Otherwise returns an Eigen::Map mapping the container
  // with `map_rows` rows.
  Matrix<Eigen::Dynamic> AsMatrix(int map_rows) {
    return Traits::template AsMatrix<Eigen::Dynamic>(c_, map_rows);
  }
  // Same as above, but specifying `map_rows` at compile time.
  template <int MapRows>
  Matrix<MapRows> AsMatrix() {
    return Traits::template AsMatrix<MapRows>(c_, MapRows);
  }

 private:
  Container& c_;
};

// Traits ContainerWrapperTraits<C> for different container types C are defined
// as partial template specializations, which ContainerWrapper then looks up as
// `ContainerWrapperTraits<typename std::decay_t<C>>`. Beware that for this look
// up to work, a specialization for containers of type D below must match C
// directly. It is not enough if C is merely convertible to D, not even if C
// inherits from D. Particularly:
//
// * Even though many containers are convertible to absl::Span, they will not
//   find the absl::Span traits.
// * Even though Eigen::PlainObjectBase is the base class of Array and Matrix, a
//   partial specialization for PlainObjectBase wouldn't match Array or Matrix.

// absl::Span<T>.
template <typename ValueType>
struct ContainerWrapperTraits<absl::Span<ValueType>> {
  using Container = absl::Span<ValueType>;
  using Scalar = typename std::remove_const<ValueType>::type;
  enum {
    Valid = true,
    Dims = 1,
    RowsAtCompileTime = Eigen::Dynamic,
    ColsAtCompileTime = Eigen::Dynamic,
    IsVectorAtCompileTime = true,
    IsResizable = false,
  };

  template <int MapRows>
  static auto AsMatrix(Container& c, int map_rows)
      -> Eigen::Map<typename std::conditional<
          std::is_const<ValueType>::value,
          const Eigen::Matrix<Scalar, MapRows, Eigen::Dynamic>,
          Eigen::Matrix<Scalar, MapRows, Eigen::Dynamic>>::type> {
    using Matrix = typename Eigen::Matrix<Scalar, MapRows, Eigen::Dynamic>;
    return Eigen::Map<typename std::conditional<
        std::is_const<ValueType>::value, const Matrix, Matrix>::type>(
        c.data(), map_rows, c.size() / map_rows);
  }
  template <int MapRows>
  static auto AsMatrix(const Container& c, int map_rows)
      -> Eigen::Map<const Eigen::Matrix<Scalar, MapRows, Eigen::Dynamic>> {
    using Matrix = typename Eigen::Matrix<Scalar, MapRows, Eigen::Dynamic>;
    return Eigen::Map<const Matrix>(c.data(), map_rows, c.size() / map_rows);
  }
};

// std::vector<T>.
template <typename ValueType, typename Allocator>
struct ContainerWrapperTraits<std::vector<ValueType, Allocator>> {
  using Container = std::vector<ValueType, Allocator>;
  using Scalar = typename Container::value_type;
  enum {
    Valid = true,
    Dims = 1,
    RowsAtCompileTime = Eigen::Dynamic,
    ColsAtCompileTime = Eigen::Dynamic,
    IsVectorAtCompileTime = true,
    IsResizable = true,
  };

  template <int MapRows>
  static auto AsMatrix(Container& c, int map_rows)
      -> Eigen::Map<Eigen::Matrix<Scalar, MapRows, Eigen::Dynamic>> {
    return Eigen::Map<Eigen::Matrix<Scalar, MapRows, Eigen::Dynamic>>(
        c.data(), map_rows, c.size() / map_rows);
  }
  template <int MapRows>
  static auto AsMatrix(const Container& c, int map_rows)
      -> Eigen::Map<const Eigen::Matrix<Scalar, MapRows, Eigen::Dynamic>> {
    return Eigen::Map<const Eigen::Matrix<Scalar, MapRows, Eigen::Dynamic>>(
        c.data(), map_rows, c.size() / map_rows);
  }
};

// Base traits for Eigen types, reused by the definitions below.
template <typename EigenType, bool IsResizable_>
struct EigenContainerWrapperTraits {
  using Container = EigenType;
  using Scalar = typename EigenType::Scalar;
  enum {
    Valid = true,
    Dims = 2,
    RowsAtCompileTime = Container::RowsAtCompileTime,
    ColsAtCompileTime = Container::ColsAtCompileTime,
    IsVectorAtCompileTime = Container::IsVectorAtCompileTime,
    IsResizable = IsResizable_,
  };

  template <int UnusedMapRows>
  static decltype(std::declval<Container&>().matrix()) AsMatrix(
      Container& c, int) {
    return c.matrix();
  }
  template <int UnusedMapRows>
  static decltype(std::declval<const Container&>().matrix()) AsMatrix(
      const Container& c, int) {
    return c.matrix();
  }
};

// Traits specializations for specific Eigen types.

// Eigen::Array.
template <typename Scalar, int Rows, int Cols, int Options>
struct ContainerWrapperTraits<Eigen::Array<Scalar, Rows, Cols, Options>>
    : public EigenContainerWrapperTraits<
          Eigen::Array<Scalar, Rows, Cols, Options>, /*IsResizable=*/true> {};
// Eigen::Matrix.
template <typename Scalar, int Rows, int Cols, int Options>
struct ContainerWrapperTraits<Eigen::Matrix<Scalar, Rows, Cols, Options>>
    : public EigenContainerWrapperTraits<
          Eigen::Matrix<Scalar, Rows, Cols, Options>, /*IsResizable=*/true> {};
// Eigen::VectorBlock.
template <typename VectorType, int BlockSize>
struct ContainerWrapperTraits<Eigen::VectorBlock<VectorType, BlockSize>>
    : public EigenContainerWrapperTraits<
          Eigen::VectorBlock<VectorType, BlockSize>,
          /*IsResizable=*/false> {};
// Eigen::Block.
template <typename XprType, int BlockRows, int BlockCols, bool InnerPanel>
struct ContainerWrapperTraits<
    Eigen::Block<XprType, BlockRows, BlockCols, InnerPanel>>
    : public EigenContainerWrapperTraits<
          Eigen::Block<XprType, BlockRows, BlockCols, InnerPanel>,
          /*IsResizable=*/false> {};
// Eigen::CwiseNullaryOp.
template <typename NullaryOp, typename PlainObjectType>
struct ContainerWrapperTraits<Eigen::CwiseNullaryOp<NullaryOp, PlainObjectType>>
    : public EigenContainerWrapperTraits<
          Eigen::CwiseNullaryOp<NullaryOp, PlainObjectType>,
          /*IsResizable=*/false> {};

}  // namespace internal

}  // namespace audio_dsp

#endif  // AUDIO_DSP_EIGEN_TYPES_H_
