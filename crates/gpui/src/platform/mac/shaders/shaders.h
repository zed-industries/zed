#include <simd/simd.h>

typedef struct
{
    vector_float2 viewport_size;
} GPUIUniforms;

typedef enum
{
    GPUIQuadInputIndexVertices = 0,
    GPUIQuadInputIndexQuads = 1,
    GPUIQuadInputIndexUniforms = 2,
} GPUIQuadInputIndex;

typedef struct
{
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

typedef enum
{
    GPUIShadowInputIndexVertices = 0,
    GPUIShadowInputIndexShadows = 1,
    GPUIShadowInputIndexUniforms = 2,
} GPUIShadowInputIndex;

typedef struct
{
    vector_float2 origin;
    vector_float2 size;
    float corner_radius;
    float sigma;
    vector_uchar4 color;
} GPUIShadow;

typedef enum
{
    GPUISpriteVertexInputIndexVertices = 0,
    GPUISpriteVertexInputIndexSprites = 1,
    GPUISpriteVertexInputIndexViewportSize = 2,
    GPUISpriteVertexInputIndexAtlasSize = 3,
} GPUISpriteVertexInputIndex;

typedef enum
{
    GPUISpriteFragmentInputIndexAtlas = 0,
} GPUISpriteFragmentInputIndex;

typedef struct
{
    vector_float2 origin;
    vector_float2 target_size;
    vector_float2 source_size;
    vector_float2 atlas_origin;
    vector_uchar4 color;
    uint8_t compute_winding;
} GPUISprite;

typedef enum
{
    GPUIPathAtlasVertexInputIndexVertices = 0,
    GPUIPathAtlasVertexInputIndexAtlasSize = 1,
} GPUIPathAtlasVertexInputIndex;

typedef struct
{
    vector_float2 xy_position;
    vector_float2 st_position;
    vector_float2 clip_rect_origin;
    vector_float2 clip_rect_size;
} GPUIPathVertex;

typedef enum
{
    GPUIImageVertexInputIndexVertices = 0,
    GPUIImageVertexInputIndexImages = 1,
    GPUIImageVertexInputIndexViewportSize = 2,
    GPUIImageVertexInputIndexAtlasSize = 3,
} GPUIImageVertexInputIndex;

typedef enum
{
    GPUIImageFragmentInputIndexAtlas = 0,
} GPUIImageFragmentInputIndex;

typedef struct
{
    vector_float2 origin;
    vector_float2 target_size;
    vector_float2 source_size;
    vector_float2 atlas_origin;
    float border_top;
    float border_right;
    float border_bottom;
    float border_left;
    vector_uchar4 border_color;
    float corner_radius;
} GPUIImage;

typedef enum
{
    GPUIUnderlineInputIndexVertices = 0,
    GPUIUnderlineInputIndexUnderlines = 1,
    GPUIUnderlineInputIndexUniforms = 2,
} GPUIUnderlineInputIndex;

typedef struct
{
    vector_float2 origin;
    vector_float2 size;
    float thickness;
    vector_uchar4 color;
    uint8_t squiggly;
} GPUIUnderline;
