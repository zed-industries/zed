from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  cdef struct TypedLength_f32__UnknownUnit:
    float _0;

  cdef struct TypedLength_f32__LayoutUnit:
    float _0;

  ctypedef TypedLength_f32__UnknownUnit Length_f32;

  ctypedef TypedLength_f32__LayoutUnit LayoutLength;

  cdef struct TypedSideOffsets2D_f32__UnknownUnit:
    float top;
    float right;
    float bottom;
    float left;

  cdef struct TypedSideOffsets2D_f32__LayoutUnit:
    float top;
    float right;
    float bottom;
    float left;

  ctypedef TypedSideOffsets2D_f32__UnknownUnit SideOffsets2D_f32;

  ctypedef TypedSideOffsets2D_f32__LayoutUnit LayoutSideOffsets2D;

  cdef struct TypedSize2D_f32__UnknownUnit:
    float width;
    float height;

  cdef struct TypedSize2D_f32__LayoutUnit:
    float width;
    float height;

  ctypedef TypedSize2D_f32__UnknownUnit Size2D_f32;

  ctypedef TypedSize2D_f32__LayoutUnit LayoutSize2D;

  cdef struct TypedPoint2D_f32__UnknownUnit:
    float x;
    float y;

  cdef struct TypedPoint2D_f32__LayoutUnit:
    float x;
    float y;

  ctypedef TypedPoint2D_f32__UnknownUnit Point2D_f32;

  ctypedef TypedPoint2D_f32__LayoutUnit LayoutPoint2D;

  cdef struct TypedRect_f32__UnknownUnit:
    TypedPoint2D_f32__UnknownUnit origin;
    TypedSize2D_f32__UnknownUnit size;

  cdef struct TypedRect_f32__LayoutUnit:
    TypedPoint2D_f32__LayoutUnit origin;
    TypedSize2D_f32__LayoutUnit size;

  ctypedef TypedRect_f32__UnknownUnit Rect_f32;

  ctypedef TypedRect_f32__LayoutUnit LayoutRect;

  cdef struct TypedTransform2D_f32__UnknownUnit__LayoutUnit:
    float m11;
    float m12;
    float m21;
    float m22;
    float m31;
    float m32;

  cdef struct TypedTransform2D_f32__LayoutUnit__UnknownUnit:
    float m11;
    float m12;
    float m21;
    float m22;
    float m31;
    float m32;

  void root(TypedLength_f32__UnknownUnit length_a,
            TypedLength_f32__LayoutUnit length_b,
            Length_f32 length_c,
            LayoutLength length_d,
            TypedSideOffsets2D_f32__UnknownUnit side_offsets_a,
            TypedSideOffsets2D_f32__LayoutUnit side_offsets_b,
            SideOffsets2D_f32 side_offsets_c,
            LayoutSideOffsets2D side_offsets_d,
            TypedSize2D_f32__UnknownUnit size_a,
            TypedSize2D_f32__LayoutUnit size_b,
            Size2D_f32 size_c,
            LayoutSize2D size_d,
            TypedPoint2D_f32__UnknownUnit point_a,
            TypedPoint2D_f32__LayoutUnit point_b,
            Point2D_f32 point_c,
            LayoutPoint2D point_d,
            TypedRect_f32__UnknownUnit rect_a,
            TypedRect_f32__LayoutUnit rect_b,
            Rect_f32 rect_c,
            LayoutRect rect_d,
            TypedTransform2D_f32__UnknownUnit__LayoutUnit transform_a,
            TypedTransform2D_f32__LayoutUnit__UnknownUnit transform_b);
