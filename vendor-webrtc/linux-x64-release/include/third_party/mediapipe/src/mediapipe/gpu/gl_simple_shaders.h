// Copyright 2019 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef MEDIAPIPE_GPU_GL_SIMPLE_SHADERS_H_
#define MEDIAPIPE_GPU_GL_SIMPLE_SHADERS_H_

#include "mediapipe/gpu/gl_base.h"

namespace mediapipe {

// Leaves vertex and texture coordinates as they are.
// Input attributes:
//   vec4 position - vertex position in clip space (-1..1)
//   vec4 texture_coordinate - texture coordinate for each vertex in
//                             normalized texture space (0..1)
// Output varying:
//   vec2 sample_coordinate - texture coordinate for shader
extern const GLchar* const kBasicVertexShader;

// Leaves vertex and texture coordinates as they are.
// Input attributes:
//   vec4 position - vertex position
//   vec4 texture_coordinate - texture coordinate
// Input uniform:
//   vec4 scale - scale factor for vertices
// Output varying:
//   vec2 sample_coordinate - texture coordinate for shader
extern const GLchar* const kScaledVertexShader;

// Applies an affine transformation to the vertex and leaves texture coordinates
// as is. Input attributes:
//   vec4 position - vertex position
//   vec4 texture_coordinate - texture coordinate
// Input uniform:
//   mat3 homogeneous affine transform - transformation matrix for vertices
//   vec2 viewport_size - size of the viewport
// Output varying:
//   vec2 sample_coordinate - texture coordinate for shader
extern const GLchar* const kTransformedVertexShader;

// Outputs the texture as it is.
// Input varying:
//   vec2 sample_coordinate - texture coordinate
// Input uniform:
//   sampler2d video_frame - texture
extern const GLchar* const kBasicTexturedFragmentShader;

// Same as kBasicTexturedFragmentShader
// except using OES textures.
extern const GLchar* const kBasicTexturedFragmentShaderOES;

// Paints the fragment with a flat color.
// Input uniform:
//   vec3 color - the RGB color.
extern const GLchar* const kFlatColorFragmentShader;

// Multiplies each R, G, B value for a weight.
// Input varying:
//   vec2 sample_coordinate - texture coordinate
// Input uniform:
//   sampler2d video_frame - texture
//   vec3 weights - r,g,b weights
extern const GLchar* const kRgbWeightFragmentShader;

// Converts a YUV input into RGB.
// Input uniform:
//   sampler2D video_frame_y - Y texture
//   sampler2D video_frame_uv - UV texture
extern const GLchar* const kYUV2TexToRGBFragmentShader;

// A square covering the full clip space.
static const GLfloat kBasicSquareVertices[] = {
    -1.0f, -1.0f,  // bottom left
    1.0f,  -1.0f,  // bottom right
    -1.0f, 1.0f,   // top left
    1.0f,  1.0f,   // top right
};

// Temporary macros to copy vertices.
#define V(source, n) source[2 * (n)], source[2 * (n) + 1]
#define V4(source, a, b, c, d) \
  V(source, a), V(source, b), V(source, c), V(source, d)

// A square covering the full clip space, rotated 90 degrees counterclockwise.
static const GLfloat kBasicSquareVertices90[] = {
    V4(kBasicSquareVertices, 2, 0, 3, 1)};

// A square covering the full clip space, rotated 180 degrees counterclockwise.
static const GLfloat kBasicSquareVertices180[] = {
    V4(kBasicSquareVertices, 3, 2, 1, 0)};

// A square covering the full clip space, rotated 270 degrees counterclockwise.
static const GLfloat kBasicSquareVertices270[] = {
    V4(kBasicSquareVertices, 1, 3, 0, 2)};

// Places a texture on kBasicSquareVertices with normal alignment.
static const GLfloat kBasicTextureVertices[] = {
    0.0f, 0.0f,  // bottom left
    1.0f, 0.0f,  // bottom right
    0.0f, 1.0f,  // top left
    1.0f, 1.0f,  // top right
};

// Places a texture on kBasicSquareVertices, flipped horizontally.
static const GLfloat kBasicTextureVerticesFlipX[] = {
    V4(kBasicTextureVertices, 1, 0, 3, 2)};

// Places a texture on kBasicSquareVertices, flipped vertically.
static const GLfloat kBasicTextureVerticesFlipY[] = {
    V4(kBasicTextureVertices, 2, 3, 0, 1)};

#undef V4
#undef V

// Used in shaders to differentiate desktop OpenGL vs OpenGL ES.
// The newer spec requires this to be the first line of a shader.
// Desktop OpenGL 3.3+ = #version 330
#ifndef GL_ES_VERSION_2_0
#define GLES_VERSION_COMPAT "#version 330 \n"
#else
#define GLES_VERSION_COMPAT "\n"
#endif  // GL_ES_VERSION_2_0

// Used in vertex shaders to differentiate different versions of OpenGL.
extern const GLchar* const kMediaPipeVertexShaderPreamble;

// Used in fragment shaders to differentiate different versions of OpenGL.
extern const GLchar* const kMediaPipeFragmentShaderPreamble;

}  // namespace mediapipe

#endif  // MEDIAPIPE_GPU_GL_SIMPLE_SHADERS_H_
