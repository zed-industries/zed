#include <simd/simd.h>

typedef enum {
    GPUIQuadInputIndexVertices = 0,
    GPUIQuadInputIndexQuads = 1,
    GPUIQuadInputIndexUniforms = 2,
} GPUIQuadInputIndex;

typedef struct {
    vector_float2 origin;
    vector_float2 size;
    vector_float4 background_color;
} GPUIQuad;

typedef struct {
    vector_float2 viewport_size;
} GPUIQuadUniforms;
