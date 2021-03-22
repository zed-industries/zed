#include <simd/simd.h>

typedef enum {
    GPUIQuadInputIndexVertices = 0,
    GPUIQuadInputIndexQuads = 1,
    GPUIQuadInputIndexUniforms = 2,
} GPUIQuadInputIndex;

typedef struct {
    vector_float2 origin;
    vector_float2 size;
    vector_uchar4 background_color;
    float border_top;
    float border_right;
    float border_bottom;
    float border_left;
    vector_uchar4 border_color;
    float corner_radius;
} GPUIQuad;

typedef struct {
    vector_float2 viewport_size;
} GPUIQuadUniforms;
