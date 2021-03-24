#include <simd/simd.h>

typedef struct {
    vector_float2 viewport_size;
} GPUIUniforms;

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

typedef enum {
    GPUIShadowInputIndexVertices = 0,
    GPUIShadowInputIndexShadows = 1,
    GPUIShadowInputIndexUniforms = 2,
} GPUIShadowInputIndex;

typedef struct {
    vector_float2 origin;
    vector_float2 size;
    float corner_radius;
    float sigma;
    vector_uchar4 color;
} GPUIShadow;

typedef enum {
    GPUISpriteVertexInputIndexVertices = 0,
    GPUISpriteVertexInputIndexSprites = 1,
    GPUISpriteVertexInputIndexViewportSize = 2,
    GPUISpriteVertexInputIndexAtlasSize = 3,
} GPUISpriteVertexInputIndex;

typedef enum {
    GPUISpriteFragmentInputIndexAtlas = 0,
} GPUISpriteFragmentInputIndex;

typedef struct {
    vector_float2 origin;
    vector_float2 size;
    vector_float2 atlas_origin;
    vector_uchar4 color;
} GPUISprite;
