#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
  float _0;
} TypedLength_f32__UnknownUnit;

typedef struct {
  float _0;
} TypedLength_f32__LayoutUnit;

typedef TypedLength_f32__UnknownUnit Length_f32;

typedef TypedLength_f32__LayoutUnit LayoutLength;

typedef struct {
  float top;
  float right;
  float bottom;
  float left;
} TypedSideOffsets2D_f32__UnknownUnit;

typedef struct {
  float top;
  float right;
  float bottom;
  float left;
} TypedSideOffsets2D_f32__LayoutUnit;

typedef TypedSideOffsets2D_f32__UnknownUnit SideOffsets2D_f32;

typedef TypedSideOffsets2D_f32__LayoutUnit LayoutSideOffsets2D;

typedef struct {
  float width;
  float height;
} TypedSize2D_f32__UnknownUnit;

typedef struct {
  float width;
  float height;
} TypedSize2D_f32__LayoutUnit;

typedef TypedSize2D_f32__UnknownUnit Size2D_f32;

typedef TypedSize2D_f32__LayoutUnit LayoutSize2D;

typedef struct {
  float x;
  float y;
} TypedPoint2D_f32__UnknownUnit;

typedef struct {
  float x;
  float y;
} TypedPoint2D_f32__LayoutUnit;

typedef TypedPoint2D_f32__UnknownUnit Point2D_f32;

typedef TypedPoint2D_f32__LayoutUnit LayoutPoint2D;

typedef struct {
  TypedPoint2D_f32__UnknownUnit origin;
  TypedSize2D_f32__UnknownUnit size;
} TypedRect_f32__UnknownUnit;

typedef struct {
  TypedPoint2D_f32__LayoutUnit origin;
  TypedSize2D_f32__LayoutUnit size;
} TypedRect_f32__LayoutUnit;

typedef TypedRect_f32__UnknownUnit Rect_f32;

typedef TypedRect_f32__LayoutUnit LayoutRect;

typedef struct {
  float m11;
  float m12;
  float m21;
  float m22;
  float m31;
  float m32;
} TypedTransform2D_f32__UnknownUnit__LayoutUnit;

typedef struct {
  float m11;
  float m12;
  float m21;
  float m22;
  float m31;
  float m32;
} TypedTransform2D_f32__LayoutUnit__UnknownUnit;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

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

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
