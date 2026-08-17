#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct TypedLength_f32__UnknownUnit {
  float _0;
} TypedLength_f32__UnknownUnit;

typedef struct TypedLength_f32__LayoutUnit {
  float _0;
} TypedLength_f32__LayoutUnit;

typedef struct TypedLength_f32__UnknownUnit Length_f32;

typedef struct TypedLength_f32__LayoutUnit LayoutLength;

typedef struct TypedSideOffsets2D_f32__UnknownUnit {
  float top;
  float right;
  float bottom;
  float left;
} TypedSideOffsets2D_f32__UnknownUnit;

typedef struct TypedSideOffsets2D_f32__LayoutUnit {
  float top;
  float right;
  float bottom;
  float left;
} TypedSideOffsets2D_f32__LayoutUnit;

typedef struct TypedSideOffsets2D_f32__UnknownUnit SideOffsets2D_f32;

typedef struct TypedSideOffsets2D_f32__LayoutUnit LayoutSideOffsets2D;

typedef struct TypedSize2D_f32__UnknownUnit {
  float width;
  float height;
} TypedSize2D_f32__UnknownUnit;

typedef struct TypedSize2D_f32__LayoutUnit {
  float width;
  float height;
} TypedSize2D_f32__LayoutUnit;

typedef struct TypedSize2D_f32__UnknownUnit Size2D_f32;

typedef struct TypedSize2D_f32__LayoutUnit LayoutSize2D;

typedef struct TypedPoint2D_f32__UnknownUnit {
  float x;
  float y;
} TypedPoint2D_f32__UnknownUnit;

typedef struct TypedPoint2D_f32__LayoutUnit {
  float x;
  float y;
} TypedPoint2D_f32__LayoutUnit;

typedef struct TypedPoint2D_f32__UnknownUnit Point2D_f32;

typedef struct TypedPoint2D_f32__LayoutUnit LayoutPoint2D;

typedef struct TypedRect_f32__UnknownUnit {
  struct TypedPoint2D_f32__UnknownUnit origin;
  struct TypedSize2D_f32__UnknownUnit size;
} TypedRect_f32__UnknownUnit;

typedef struct TypedRect_f32__LayoutUnit {
  struct TypedPoint2D_f32__LayoutUnit origin;
  struct TypedSize2D_f32__LayoutUnit size;
} TypedRect_f32__LayoutUnit;

typedef struct TypedRect_f32__UnknownUnit Rect_f32;

typedef struct TypedRect_f32__LayoutUnit LayoutRect;

typedef struct TypedTransform2D_f32__UnknownUnit__LayoutUnit {
  float m11;
  float m12;
  float m21;
  float m22;
  float m31;
  float m32;
} TypedTransform2D_f32__UnknownUnit__LayoutUnit;

typedef struct TypedTransform2D_f32__LayoutUnit__UnknownUnit {
  float m11;
  float m12;
  float m21;
  float m22;
  float m31;
  float m32;
} TypedTransform2D_f32__LayoutUnit__UnknownUnit;

void root(struct TypedLength_f32__UnknownUnit length_a,
          struct TypedLength_f32__LayoutUnit length_b,
          Length_f32 length_c,
          LayoutLength length_d,
          struct TypedSideOffsets2D_f32__UnknownUnit side_offsets_a,
          struct TypedSideOffsets2D_f32__LayoutUnit side_offsets_b,
          SideOffsets2D_f32 side_offsets_c,
          LayoutSideOffsets2D side_offsets_d,
          struct TypedSize2D_f32__UnknownUnit size_a,
          struct TypedSize2D_f32__LayoutUnit size_b,
          Size2D_f32 size_c,
          LayoutSize2D size_d,
          struct TypedPoint2D_f32__UnknownUnit point_a,
          struct TypedPoint2D_f32__LayoutUnit point_b,
          Point2D_f32 point_c,
          LayoutPoint2D point_d,
          struct TypedRect_f32__UnknownUnit rect_a,
          struct TypedRect_f32__LayoutUnit rect_b,
          Rect_f32 rect_c,
          LayoutRect rect_d,
          struct TypedTransform2D_f32__UnknownUnit__LayoutUnit transform_a,
          struct TypedTransform2D_f32__LayoutUnit__UnknownUnit transform_b);
