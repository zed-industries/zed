/*
 * Copyright (c) 2014 Lukasz Marek
 *
 * This file is part of FFmpeg.
 *
 * FFmpeg is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * FFmpeg is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with FFmpeg; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
 */

#ifndef AVDEVICE_OPENGL_ENC_SHADERS_H
#define AVDEVICE_OPENGL_ENC_SHADERS_H

#include "libavutil/pixfmt.h"

static const char * const FF_OPENGL_VERTEX_SHADER =
    "uniform mat4 u_projectionMatrix;"
    "uniform mat4 u_modelViewMatrix;"

    "attribute vec4 a_position;"
    "attribute vec2 a_textureCoords;"

    "varying vec2 texture_coordinate;"

    "void main()"
    "{"
        "gl_Position = u_projectionMatrix * (a_position * u_modelViewMatrix);"
        "texture_coordinate = a_textureCoords;"
    "}";

/**
 * Fragment shader for packet RGBA formats.
 */
static const char * const FF_OPENGL_FRAGMENT_SHADER_RGBA_PACKET =
#if defined(GL_ES_VERSION_2_0)
    "precision mediump float;"
#endif
    "uniform sampler2D u_texture0;"
    "uniform mat4 u_colorMap;"

    "varying vec2 texture_coordinate;"

    "void main()"
    "{"
        "gl_FragColor = texture2D(u_texture0, texture_coordinate) * u_colorMap;"
    "}";

/**
 * Fragment shader for packet RGB formats.
 */
static const char * const FF_OPENGL_FRAGMENT_SHADER_RGB_PACKET =
#if defined(GL_ES_VERSION_2_0)
    "precision mediump float;"
#endif
    "uniform sampler2D u_texture0;"
    "uniform mat4 u_colorMap;"

    "varying vec2 texture_coordinate;"

    "void main()"
    "{"
        "gl_FragColor = vec4((texture2D(u_texture0, texture_coordinate) * u_colorMap).rgb, 1.0);"
    "}";

/**
 * Fragment shader for planar RGBA formats.
 */
static const char * const FF_OPENGL_FRAGMENT_SHADER_RGBA_PLANAR =
#if defined(GL_ES_VERSION_2_0)
    "precision mediump float;"
#endif
    "uniform sampler2D u_texture0;"
    "uniform sampler2D u_texture1;"
    "uniform sampler2D u_texture2;"
    "uniform sampler2D u_texture3;"

    "varying vec2 texture_coordinate;"

    "void main()"
    "{"
        "gl_FragColor = vec4(texture2D(u_texture0, texture_coordinate).r,"
                            "texture2D(u_texture1, texture_coordinate).r,"
                            "texture2D(u_texture2, texture_coordinate).r,"
                            "texture2D(u_texture3, texture_coordinate).r);"
    "}";

/**
 * Fragment shader for planar RGB formats.
 */
static const char * const FF_OPENGL_FRAGMENT_SHADER_RGB_PLANAR =
#if defined(GL_ES_VERSION_2_0)
    "precision mediump float;"
#endif
    "uniform sampler2D u_texture0;"
    "uniform sampler2D u_texture1;"
    "uniform sampler2D u_texture2;"

    "varying vec2 texture_coordinate;"

    "void main()"
    "{"
        "gl_FragColor = vec4(texture2D(u_texture0, texture_coordinate).r,"
                            "texture2D(u_texture1, texture_coordinate).r,"
                            "texture2D(u_texture2, texture_coordinate).r,"
                            "1.0);"
    "}";

/**
 * Fragment shader for planar YUV formats.
 */
static const char * const  FF_OPENGL_FRAGMENT_SHADER_YUV_PLANAR =
#if defined(GL_ES_VERSION_2_0)
    "precision mediump float;"
#endif
    "uniform sampler2D u_texture0;"
    "uniform sampler2D u_texture1;"
    "uniform sampler2D u_texture2;"
    "uniform float u_chroma_div_w;"
    "uniform float u_chroma_div_h;"

    "varying vec2 texture_coordinate;"

    "void main()"
    "{"
        "vec3 yuv;"

        "yuv.r = texture2D(u_texture0, texture_coordinate).r - 0.0625;"
        "yuv.g = texture2D(u_texture1, vec2(texture_coordinate.x / u_chroma_div_w, texture_coordinate.y / u_chroma_div_h)).r - 0.5;"
        "yuv.b = texture2D(u_texture2, vec2(texture_coordinate.x / u_chroma_div_w, texture_coordinate.y / u_chroma_div_h)).r - 0.5;"

        "gl_FragColor = clamp(vec4(mat3(1.1643,  1.16430, 1.1643,"
                                       "0.0,    -0.39173, 2.0170,"
                                       "1.5958, -0.81290, 0.0) * yuv, 1.0), 0.0, 1.0);"

    "}";

/**
 * Fragment shader for planar YUVA formats.
 */
static const char * const FF_OPENGL_FRAGMENT_SHADER_YUVA_PLANAR =
#if defined(GL_ES_VERSION_2_0)
    "precision mediump float;"
#endif
    "uniform sampler2D u_texture0;"
    "uniform sampler2D u_texture1;"
    "uniform sampler2D u_texture2;"
    "uniform sampler2D u_texture3;"
    "uniform float u_chroma_div_w;"
    "uniform float u_chroma_div_h;"

    "varying vec2 texture_coordinate;"

    "void main()"
    "{"
        "vec3 yuv;"

        "yuv.r = texture2D(u_texture0, texture_coordinate).r - 0.0625;"
        "yuv.g = texture2D(u_texture1, vec2(texture_coordinate.x / u_chroma_div_w, texture_coordinate.y / u_chroma_div_h)).r - 0.5;"
        "yuv.b = texture2D(u_texture2, vec2(texture_coordinate.x / u_chroma_div_w, texture_coordinate.y / u_chroma_div_h)).r - 0.5;"

        "gl_FragColor = clamp(vec4(mat3(1.1643,  1.16430, 1.1643,"
                                       "0.0,    -0.39173, 2.0170,"
                                       "1.5958, -0.81290, 0.0) * yuv, texture2D(u_texture3, texture_coordinate).r), 0.0, 1.0);"
    "}";

static const char * const FF_OPENGL_FRAGMENT_SHADER_GRAY =
#if defined(GL_ES_VERSION_2_0)
    "precision mediump float;"
#endif
    "uniform sampler2D u_texture0;"
    "varying vec2 texture_coordinate;"
    "void main()"
    "{"
        "float c = texture2D(u_texture0, texture_coordinate).r;"
        "gl_FragColor = vec4(c, c, c, 1.0);"
    "}";

#endif /* AVDEVICE_OPENGL_ENC_SHADERS_H */
