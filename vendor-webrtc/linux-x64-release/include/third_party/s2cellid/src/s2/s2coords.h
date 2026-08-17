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

// Author: ericv@google.com (Eric Veach)
//
// This file contains documentation of the various coordinate systems used
// throughout the library.  Most importantly, S2 defines a framework for
// decomposing the unit sphere into a hierarchy of "cells".  Each cell is a
// quadrilateral bounded by four geodesics.  The top level of the hierarchy is
// obtained by projecting the six faces of a cube onto the unit sphere, and
// lower levels are obtained by subdividing each cell into four children
// recursively.  Cells are numbered such that sequentially increasing cells
// follow a continuous space-filling curve over the entire sphere.  The
// transformation is designed to make the cells at each level fairly uniform
// in size.
//
//
////////////////////////// S2Cell Decomposition /////////////////////////
//
// The following methods define the cube-to-sphere projection used by
// the S2Cell decomposition.
//
// In the process of converting a latitude-longitude pair to a 64-bit cell
// id, the following coordinate systems are used:
//
//  (id)
//    An S2CellId is a 64-bit encoding of a face and a Hilbert curve position
//    on that face.  The Hilbert curve position implicitly encodes both the
//    position of a cell and its subdivision level (see s2cellid.h).
//
//  (face, i, j)
//    Leaf-cell coordinates.  "i" and "j" are integers in the range
//    [0,(2**30)-1] that identify a particular leaf cell on the given face.
//    The (i, j) coordinate system is right-handed on each face, and the
//    faces are oriented such that Hilbert curves connect continuously from
//    one face to the next.
//
//  (face, s, t)
//    Cell-space coordinates.  "s" and "t" are real numbers in the range
//    [0,1] that identify a point on the given face.  For example, the point
//    (s, t) = (0.5, 0.5) corresponds to the center of the top-level face
//    cell.  This point is also a vertex of exactly four cells at each
//    subdivision level greater than zero.
//
//  (face, si, ti)
//    Discrete cell-space coordinates.  These are obtained by multiplying
//    "s" and "t" by 2**31 and rounding to the nearest unsigned integer.
//    Discrete coordinates lie in the range [0,2**31].  This coordinate
//    system can represent the edge and center positions of all cells with
//    no loss of precision (including non-leaf cells).  In binary, each
//    coordinate of a level-k cell center ends with a 1 followed by
//    (30 - k) 0s.  The coordinates of its edges end with (at least)
//    (31 - k) 0s.
//
//  (face, u, v)
//    Cube-space coordinates in the range [-1,1].  To make the cells at each
//    level more uniform in size after they are projected onto the sphere,
//    we apply a nonlinear transformation of the form u=f(s), v=f(t).
//    The (u, v) coordinates after this transformation give the actual
//    coordinates on the cube face (modulo some 90 degree rotations) before
//    it is projected onto the unit sphere.
//
//  (face, u, v, w)
//    Per-face coordinate frame.  This is an extension of the (face, u, v)
//    cube-space coordinates that adds a third axis "w" in the direction of
//    the face normal.  It is always a right-handed 3D coordinate system.
//    Cube-space coordinates can be converted to this frame by setting w=1,
//    while (u,v,w) coordinates can be projected onto the cube face by
//    dividing by w, i.e. (face, u/w, v/w).
//
//  (x, y, z)
//    Direction vector (S2Point).  Direction vectors are not necessarily unit
//    length, and are often chosen to be points on the biunit cube
//    [-1,+1]x[-1,+1]x[-1,+1].  They can be be normalized to obtain the
//    corresponding point on the unit sphere.
//
//  (lat, lng)
//    Latitude and longitude (S2LatLng).  Latitudes must be between -90 and
//    90 degrees inclusive, and longitudes must be between -180 and 180
//    degrees inclusive.
//
// Note that the (i, j), (s, t), (si, ti), and (u, v) coordinate systems are
// right-handed on all six faces.

#ifndef S2_S2COORDS_H_
#define S2_S2COORDS_H_

#include <algorithm>
#include <cmath>

#include "base/check_op.h"
#include "s2/r2.h"
#include "s2/s2coords-internal.h"
#include "s2/s2point.h"
#include "s2/util/math/mathutil.h"

// S2 is a namespace for constants and simple utility functions that are used
// throughout the S2 library.  The name "S2" is derived from the mathematical
// symbol for the two-dimensional unit sphere (note that the "2" refers to the
// dimension of the surface, not the space it is embedded in).
namespace S2 {

// This is the number of levels needed to specify a leaf cell.  This
// constant is defined here so that the S2::Metric class and the conversion
// functions below can be implemented without including s2cellid.h.  Please
// see s2cellid.h for other useful constants and conversion functions.
int const kMaxCellLevel = 30;

// The maximum index of a valid leaf cell plus one.  The range of valid leaf
// cell indices is [0..kLimitIJ-1].
int const kLimitIJ = 1 << kMaxCellLevel;  // == S2CellId::kMaxSize

// The maximum value of an si- or ti-coordinate.  The range of valid (si,ti)
// values is [0..kMaxSiTi].
unsigned int const kMaxSiTi = 1U << (kMaxCellLevel + 1);

// Convert an s- or t-value to the corresponding u- or v-value.  This is
// a non-linear transformation from [-1,1] to [-1,1] that attempts to
// make the cell sizes more uniform.
double STtoUV(double s);

// The inverse of the STtoUV transformation.  Note that it is not always
// true that UVtoST(STtoUV(x)) == x due to numerical errors.
double UVtoST(double u);

// Convert the i- or j-index of a leaf cell to the minimum corresponding s-
// or t-value contained by that cell.  The argument must be in the range
// [0..2**30], i.e. up to one position beyond the normal range of valid leaf
// cell indices.
double IJtoSTMin(int i);

// Return the i- or j-index of the leaf cell containing the given
// s- or t-value.  If the argument is outside the range spanned by valid
// leaf cell indices, return the index of the closest valid leaf cell (i.e.,
// return values are clamped to the range of valid leaf cell indices).
int STtoIJ(double s);

// Convert an si- or ti-value to the corresponding s- or t-value.
double SiTitoST(unsigned int si);

// Return the si- or ti-coordinate that is nearest to the given s- or
// t-value.  The result may be outside the range of valid (si,ti)-values.
unsigned int STtoSiTi(double s);

// Convert (face, u, v) coordinates to a direction vector (not
// necessarily unit length).
S2Point FaceUVtoXYZ(int face, double u, double v);
S2Point FaceUVtoXYZ(int face, R2Point const& uv);

// If the dot product of p with the given face normal is positive,
// set the corresponding u and v values (which may lie outside the range
// [-1,1]) and return true.  Otherwise return false.
bool FaceXYZtoUV(int face, S2Point const& p, double* pu, double* pv);
bool FaceXYZtoUV(int face, S2Point const& p, R2Point* puv);

// Given a *valid* face for the given point p (meaning that dot product
// of p with the face normal is positive), return the corresponding
// u and v values (which may lie outside the range [-1,1]).
void ValidFaceXYZtoUV(int face, S2Point const& p, double* pu, double* pv);
void ValidFaceXYZtoUV(int face, S2Point const& p, R2Point* puv);

// Transform the given point P to the (u,v,w) coordinate frame of the given
// face (where the w-axis represents the face normal).
S2Point FaceXYZtoUVW(int face, S2Point const& p);

// Return the face containing the given direction vector.  (For points on
// the boundary between faces, the result is arbitrary but repeatable.)
int GetFace(S2Point const& p);

// Convert a direction vector (not necessarily unit length) to
// (face, u, v) coordinates.
int XYZtoFaceUV(S2Point const& p, double* pu, double* pv);
int XYZtoFaceUV(S2Point const& p, R2Point* puv);

// Convert a direction vector (not necessarily unit length) to
// (face, si, ti) coordinates and, if p is exactly equal to the center of a
// cell, return the level of this cell (-1 otherwise).
int XYZtoFaceSiTi(S2Point const& p,
                  int* face,
                  unsigned int* si,
                  unsigned int* ti);

// Convert (face, si, ti) coordinates to a direction vector (not necessarily
// unit length).
S2Point FaceSiTitoXYZ(int face, unsigned int si, unsigned int ti);

// Return the right-handed normal (not necessarily unit length) for an
// edge in the direction of the positive v-axis at the given u-value on
// the given face.  (This vector is perpendicular to the plane through
// the sphere origin that contains the given edge.)
S2Point GetUNorm(int face, double u);

// Return the right-handed normal (not necessarily unit length) for an
// edge in the direction of the positive u-axis at the given v-value on
// the given face.
S2Point GetVNorm(int face, double v);

// Return the unit-length normal, u-axis, or v-axis for the given face.
S2Point GetNorm(int face);
S2Point GetUAxis(int face);
S2Point GetVAxis(int face);

// Return the given axis of the given face (u=0, v=1, w=2).
S2Point GetUVWAxis(int face, int axis);

// With respect to the (u,v,w) coordinate system of a given face, return the
// face that lies in the given direction (negative=0, positive=1) of the
// given axis (u=0, v=1, w=2).  For example, GetUVWFace(4, 0, 1) returns the
// face that is adjacent to face 4 in the positive u-axis direction.
int GetUVWFace(int face, int axis, int direction);

//////////////////   Implementation details follow   ////////////////////

// We have implemented three different projections from cell-space (s,t) to
// cube-space (u,v): linear, quadratic, and tangent.  They have the following
// tradeoffs:
//
//   Linear - This is the fastest transformation, but also produces the least
//   uniform cell sizes.  Cell areas vary by a factor of about 5.2, with the
//   largest cells at the center of each face and the smallest cells in
//   the corners.
//
//   Tangent - Transforming the coordinates via atan() makes the cell sizes
//   more uniform.  The areas vary by a maximum ratio of 1.4 as opposed to a
//   maximum ratio of 5.2.  However, each call to atan() is about as expensive
//   as all of the other calculations combined when converting from points to
//   cell ids, i.e. it reduces performance by a factor of 3.
//
//   Quadratic - This is an approximation of the tangent projection that
//   is much faster and produces cells that are almost as uniform in size.
//   It is about 3 times faster than the tangent projection for converting
//   cell ids to points or vice versa.  Cell areas vary by a maximum ratio of
//   about 2.1.
//
// Here is a table comparing the cell uniformity using each projection.  "Area
// ratio" is the maximum ratio over all subdivision levels of the largest cell
// area to the smallest cell area at that level, "edge ratio" is the maximum
// ratio of the longest edge of any cell to the shortest edge of any cell at
// the same level, and "diag ratio" is the ratio of the longest diagonal of
// any cell to the shortest diagonal of any cell at the same level.  "ToPoint"
// and "FromPoint" are the times in microseconds required to convert cell ids
// to and from points (unit vectors) respectively.  "ToPointRaw" is the time
// to convert to a non-unit-length vector, which is all that is needed for
// some purposes.
//
//               Area    Edge    Diag   ToPointRaw  ToPoint  FromPoint
//              Ratio   Ratio   Ratio             (microseconds)
// -------------------------------------------------------------------
// Linear:      5.200   2.117   2.959      0.020     0.087     0.085
// Tangent:     1.414   1.414   1.704      0.237     0.299     0.258
// Quadratic:   2.082   1.802   1.932      0.033     0.096     0.108
//
// The worst-case cell aspect ratios are about the same with all three
// projections.  The maximum ratio of the longest edge to the shortest edge
// within the same cell is about 1.4 and the maximum ratio of the diagonals
// within the same cell is about 1.7.
//
// This data was produced using s2cell_test and s2cellid_test.

#define S2_LINEAR_PROJECTION 0
#define S2_TAN_PROJECTION 1
#define S2_QUADRATIC_PROJECTION 2

#define S2_PROJECTION S2_QUADRATIC_PROJECTION

#if S2_PROJECTION == S2_LINEAR_PROJECTION

inline double STtoUV(double s) {
  return 2 * s - 1;
}

inline double UVtoST(double u) {
  return 0.5 * (u + 1);
}

#elif S2_PROJECTION == S2_TAN_PROJECTION

inline double STtoUV(double s) {
  // Unfortunately, tan(M_PI_4) is slightly less than 1.0.  This isn't due to
  // a flaw in the implementation of tan(), it's because the derivative of
  // tan(x) at x=pi/4 is 2, and it happens that the two adjacent floating
  // point numbers on either side of the infinite-precision value of pi/4 have
  // tangents that are slightly below and slightly above 1.0 when rounded to
  // the nearest double-precision result.

  s = std::tan(M_PI_2 * s - M_PI_4);
  return s + (1.0 / (GG_LONGLONG(1) << 53)) * s;
}

inline double UVtoST(double u) {
  volatile double a = std::atan(u);
  return (2 * M_1_PI) * (a + M_PI_4);
}

#elif S2_PROJECTION == S2_QUADRATIC_PROJECTION

inline double STtoUV(double s) {
  if (s >= 0.5)
    return (1 / 3.) * (4 * s * s - 1);
  else
    return (1 / 3.) * (1 - 4 * (1 - s) * (1 - s));
}

inline double UVtoST(double u) {
  if (u >= 0)
    return 0.5 * std::sqrt(1 + 3 * u);
  else
    return 1 - 0.5 * std::sqrt(1 - 3 * u);
}

#else

#error Unknown value for S2_PROJECTION

#endif

inline double IJtoSTMin(int i) {
  DCHECK(i >= 0 && i <= kLimitIJ);
  return (1.0 / kLimitIJ) * i;
}

inline int STtoIJ(double s) {
  return std::max(
      0, std::min(kLimitIJ - 1, MathUtil::FastIntRound(kLimitIJ * s - 0.5)));
}

inline double SiTitoST(unsigned int si) {
  DCHECK(si >= 0 && si <= kMaxSiTi);
  return (1.0 / kMaxSiTi) * si;
}

inline unsigned int STtoSiTi(double s) {
  // kMaxSiTi == 2^31, so the result doesn't fit in an int32_t when s == 1.
  return static_cast<unsigned int>(MathUtil::FastInt64Round(s * kMaxSiTi));
}

inline S2Point FaceUVtoXYZ(int face, double u, double v) {
  switch (face) {
    case 0:
      return S2Point(1, u, v);
    case 1:
      return S2Point(-u, 1, v);
    case 2:
      return S2Point(-u, -v, 1);
    case 3:
      return S2Point(-1, -v, -u);
    case 4:
      return S2Point(v, -1, -u);
    default:
      return S2Point(v, u, -1);
  }
}

inline S2Point FaceUVtoXYZ(int face, R2Point const& uv) {
  return FaceUVtoXYZ(face, uv[0], uv[1]);
}

inline void ValidFaceXYZtoUV(int face,
                             S2Point const& p,
                             double* pu,
                             double* pv) {
  DCHECK_GT(p.DotProd(GetNorm(face)), 0);
  switch (face) {
    case 0:
      *pu = p[1] / p[0];
      *pv = p[2] / p[0];
      break;
    case 1:
      *pu = -p[0] / p[1];
      *pv = p[2] / p[1];
      break;
    case 2:
      *pu = -p[0] / p[2];
      *pv = -p[1] / p[2];
      break;
    case 3:
      *pu = p[2] / p[0];
      *pv = p[1] / p[0];
      break;
    case 4:
      *pu = p[2] / p[1];
      *pv = -p[0] / p[1];
      break;
    default:
      *pu = -p[1] / p[2];
      *pv = -p[0] / p[2];
      break;
  }
}

inline void ValidFaceXYZtoUV(int face, S2Point const& p, R2Point* puv) {
  ValidFaceXYZtoUV(face, p, &(*puv)[0], &(*puv)[1]);
}

inline int GetFace(S2Point const& p) {
  int face = p.LargestAbsComponent();
  if (p[face] < 0)
    face += 3;
  return face;
}

inline int XYZtoFaceUV(S2Point const& p, double* pu, double* pv) {
  int face = GetFace(p);
  ValidFaceXYZtoUV(face, p, pu, pv);
  return face;
}

inline int XYZtoFaceUV(S2Point const& p, R2Point* puv) {
  return XYZtoFaceUV(p, &(*puv)[0], &(*puv)[1]);
}

inline bool FaceXYZtoUV(int face, S2Point const& p, double* pu, double* pv) {
  if (face < 3) {
    if (p[face] <= 0)
      return false;
  } else {
    if (p[face - 3] >= 0)
      return false;
  }
  ValidFaceXYZtoUV(face, p, pu, pv);
  return true;
}

inline bool FaceXYZtoUV(int face, S2Point const& p, R2Point* puv) {
  return FaceXYZtoUV(face, p, &(*puv)[0], &(*puv)[1]);
}

inline S2Point GetUNorm(int face, double u) {
  switch (face) {
    case 0:
      return S2Point(u, -1, 0);
    case 1:
      return S2Point(1, u, 0);
    case 2:
      return S2Point(1, 0, u);
    case 3:
      return S2Point(-u, 0, 1);
    case 4:
      return S2Point(0, -u, 1);
    default:
      return S2Point(0, -1, -u);
  }
}

inline S2Point GetVNorm(int face, double v) {
  switch (face) {
    case 0:
      return S2Point(-v, 0, 1);
    case 1:
      return S2Point(0, -v, 1);
    case 2:
      return S2Point(0, -1, -v);
    case 3:
      return S2Point(v, -1, 0);
    case 4:
      return S2Point(1, v, 0);
    default:
      return S2Point(1, 0, v);
  }
}

inline S2Point GetNorm(int face) {
  return GetUVWAxis(face, 2);
}

inline S2Point GetUAxis(int face) {
  return GetUVWAxis(face, 0);
}

inline S2Point GetVAxis(int face) {
  return GetUVWAxis(face, 1);
}

inline S2Point GetUVWAxis(int face, int axis) {
  double const* p = internal::kFaceUVWAxes[face][axis];
  return S2Point(p[0], p[1], p[2]);
}

inline int GetUVWFace(int face, int axis, int direction) {
  DCHECK(face >= 0 && face <= 5);
  DCHECK(axis >= 0 && axis <= 2);
  DCHECK(direction >= 0 && direction <= 1);
  return internal::kFaceUVWFaces[face][axis][direction];
}

}  // namespace S2

#endif  // S2_S2COORDS_H_
