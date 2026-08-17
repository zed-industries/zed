// Copyright 2005 Google Inc. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS-IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

#ifndef S2_S2COORDS_INTERNAL_H_
#define S2_S2COORDS_INTERNAL_H_
// Author: ericv@google.com (Eric Veach)

namespace S2 {
namespace internal {

// The canonical Hilbert traversal order looks like an inverted 'U':
// the subcells are visited in the order (0,0), (0,1), (1,1), (1,0).
// The following tables encode the traversal order for various
// orientations of the Hilbert curve (axes swapped and/or directions
// of the axes reversed).

// Together these flags define a cell orientation.  If 'kSwapMask'
// is true, then canonical traversal order is flipped around the
// diagonal (i.e. i and j are swapped with each other).  If
// 'kInvertMask' is true, then the traversal order is rotated by 180
// degrees (i.e. the bits of i and j are inverted, or equivalently,
// the axis directions are reversed).
int constexpr kSwapMask = 0x01;
int constexpr kInvertMask = 0x02;

// kIJtoPos[orientation][ij] -> pos
//
// Given a cell orientation and the (i,j)-index of a subcell (0=(0,0),
// 1=(0,1), 2=(1,0), 3=(1,1)), return the order in which this subcell is
// visited by the Hilbert curve (a position in the range [0..3]).
extern int const kIJtoPos[4][4];

// kPosToIJ[orientation][pos] -> ij
//
// Return the (i,j) index of the subcell at the given position 'pos' in the
// Hilbert curve traversal order with the given orientation.  This is the
// inverse of the previous table:
//
//   kPosToIJ[r][kIJtoPos[r][ij]] == ij
extern int const kPosToIJ[4][4];

// kPosToOrientation[pos] -> orientation_modifier
//
// Return a modifier indicating how the orientation of the child subcell
// with the given traversal position [0..3] is related to the orientation
// of the parent cell.  The modifier should be XOR-ed with the parent
// orientation to obtain the curve orientation in the child.
extern int const kPosToOrientation[4];

// The U,V,W axes for each face.
extern double const kFaceUVWAxes[6][3][3];

// The precomputed neighbors of each face (see GetUVWFace).
extern int const kFaceUVWFaces[6][3][2];

}  // namespace internal
}  // namespace S2

#endif  // S2_S2COORDS_INTERNAL_H_
