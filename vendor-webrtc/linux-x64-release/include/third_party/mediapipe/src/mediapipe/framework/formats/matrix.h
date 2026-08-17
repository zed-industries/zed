// Copyright 2019 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Define mediapipe::Matrix, a MediaPipe alias for an Eigen Matrix, and
// the conversion functions between MatrixData proto and Matrix.
//
// This is the preferred format for processing chunks of
// (multichannel) audio.

#ifndef MEDIAPIPE_FRAMEWORK_FORMATS_MATRIX_H_
#define MEDIAPIPE_FRAMEWORK_FORMATS_MATRIX_H_

// TODO: Eigen looks for __EXCEPTIONS to determine if exceptions are enabled.
// This file may be compiled with -ObjC++ on macOS, which sets __EXCEPTIONS,
// causing the build to fail. The right solution is likely to change Eigen
// to look for a different define, or perhaps an override to ignore
// __EXCEPTIONS.
#if defined(__EXCEPTIONS)
#undef __EXCEPTIONS
#endif

#include <string>

#include "Eigen/Core"
#include "mediapipe/framework/formats/matrix_data.pb.h"
#include "mediapipe/framework/port.h"

namespace mediapipe {

typedef Eigen::MatrixXf Matrix;

// Produce a MatrixData proto from an Eigen Matrix. Useful when wanting to
// copy a repeated float field.
void MatrixDataProtoFromMatrix(const Matrix& matrix, MatrixData* matrix_data);
// Produce a Matrix from a MatrixData proto representation. Useful when putting
// audio into a Matrix proto.
void MatrixFromMatrixDataProto(const MatrixData& matrix_data, Matrix* matrix);

#if !defined(MEDIAPIPE_MOBILE) && !defined(MEDIAPIPE_LITE) && \
    !defined(MEDIAPIPE_PROTO_LITE)
// Produce a Text format MatrixData string.  Mainly useful for test code.
std::string MatrixAsTextProto(const Matrix& matrix);
// Produce a Matrix from a text format MatrixData proto representation.
void MatrixFromTextProto(const std::string& text_proto, Matrix* matrix);
#endif  // !defined(MEDIAPIPE_MOBILE) && !defined(MEDIAPIPE_LITE)
}  // namespace mediapipe

namespace Eigen {

// Fixes the logging of Matrix inside EXPECT_EQ and friends. Without this, a
// failed EXPECT_EQ(m1, m2) logs Matrices as byte objects.
//
// NOTE: this functionality applies only to Matrix types, not expressions. To
// make use of this for expressions, add .eval() to them
// (e.g. "EXPECT_EQ(sum, (a + b).eval())").
template <typename S, int R, int C, int O, int MR, int MC>
void PrintTo(const Eigen::Matrix<S, R, C, O, MR, MC>& m, std::ostream* os) {
  // Log the matrix at new line.
  *os << "\n" << m;
}

}  // namespace Eigen

#endif  // MEDIAPIPE_FRAMEWORK_FORMATS_MATRIX_H_
