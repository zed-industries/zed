#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct TypedLength_f32__UnknownUnit {
  float _0;
};

struct TypedLength_f32__LayoutUnit {
  float _0;
};

typedef struct TypedLength_f32__UnknownUnit Length_f32;

typedef struct TypedLength_f32__LayoutUnit LayoutLength;

struct TypedSideOffsets2D_f32__UnknownUnit {
  float top;
  float right;
  float bottom;
  float left;
};

struct TypedSideOffsets2D_f32__LayoutUnit {
  float top;
  float right;
  float bottom;
  float left;
};

typedef struct TypedSideOffsets2D_f32__UnknownUnit SideOffsets2D_f32;

typedef struct TypedSideOffsets2D_f32__LayoutUnit LayoutSideOffsets2D;

struct TypedSize2D_f32__UnknownUnit {
  float width;
  float height;
};

struct TypedSize2D_f32__LayoutUnit {
  float width;
  float height;
};

typedef struct TypedSize2D_f32__UnknownUnit Size2D_f32;

typedef struct TypedSize2D_f32__LayoutUnit LayoutSize2D;

struct TypedPoint2D_f32__UnknownUnit {
  float x;
  float y;
};

struct TypedPoint2D_f32__LayoutUnit {
  float x;
  float y;
};

typedef struct TypedPoint2D_f32__UnknownUnit Point2D_f32;

typedef struct TypedPoint2D_f32__LayoutUnit LayoutPoint2D;

struct TypedRect_f32__UnknownUnit {
  struct TypedPoint2D_f32__UnknownUnit origin;
  struct TypedSize2D_f32__UnknownUnit size;
};

struct TypedRect_f32__LayoutUnit {
  struct TypedPoint2D_f32__LayoutUnit origin;
  struct TypedSize2D_f32__LayoutUnit size;
};

typedef struct TypedRect_f32__UnknownUnit Rect_f32;

typedef struct TypedRect_f32__LayoutUnit LayoutRect;

struct TypedTransform2D_f32__UnknownUnit__LayoutUnit {
  float m11;
  float m12;
  float m21;
  float m22;
  float m31;
  float m32;
};

struct TypedTransform2D_f32__LayoutUnit__UnknownUnit {
  float m11;
  float m12;
  float m21;
  float m22;
  float m31;
  float m32;
};

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
