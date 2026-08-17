#ifndef __gl_core_h_
#define __gl_core_h_

/*
 * SGI FREE SOFTWARE LICENSE B (Version 2.0, Sept. 18, 2008)
 * Copyright (C) 1991-2000 Silicon Graphics, Inc. All Rights Reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice including the dates of first publication and
 * either this permission notice or a reference to
 * http://oss.sgi.com/projects/FreeB/
 * shall be included in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
 * OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * SILICON GRAPHICS, INC. BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
 * WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF
 * OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 * Except as contained in this notice, the name of Silicon Graphics, Inc.
 * shall not be used in advertising or otherwise to promote the sale, use or
 * other dealings in this Software without prior written authorization from
 * Silicon Graphics, Inc.
 */

#if !defined(_WIN32_WCE)
#include <sys/types.h>
#endif

#define GL_CORE_SGI  1
#define GL_CORE_MESA 2
#define GL_CORE_APPLE 4
#define GL_CORE_WINDOWS 8

typedef struct __GLcontextRec __GLcontext;

/*
** This file defines the interface between the GL core and the surrounding
** "operating system" that supports it (currently the GLX or WGL extensions).
**
** Members (data and function pointers) are documented as imported or
** exported according to how they are used by the core rendering functions.
** Imported members are initialized by the "operating system" and used by
** the core functions.  Exported members are initialized by the core functions
** and used by the "operating system".
*/

/**
 * Mode and limit information for a context.  This information is
 * kept around in the context so that values can be used during
 * command execution, and for returning information about the
 * context to the application.
 *
 * Instances of this structure are shared by the driver and the loader.  To
 * maintain binary compatability, new fields \b must be added only to the
 * end of the structure.
 *
 * \sa _gl_context_modes_create
 */
typedef struct __GLcontextModesRec {
    struct __GLcontextModesRec * next;

    GLboolean rgbMode;
    GLboolean floatMode;
    GLboolean colorIndexMode;
    GLuint doubleBufferMode;
    GLuint stereoMode;

    GLboolean haveAccumBuffer;
    GLboolean haveDepthBuffer;
    GLboolean haveStencilBuffer;

    GLint redBits, greenBits, blueBits, alphaBits;	/* bits per comp */
    GLuint redMask, greenMask, blueMask, alphaMask;
    GLint rgbBits;		/* total bits for rgb */
    GLint indexBits;		/* total bits for colorindex */

    GLint accumRedBits, accumGreenBits, accumBlueBits, accumAlphaBits;
    GLint depthBits;
    GLint stencilBits;

    GLint numAuxBuffers;

    GLint level;

    GLint pixmapMode;

    /* GLX */
    GLint visualID;
    GLint visualType;     /**< One of the GLX X visual types. (i.e.,
			   * \c GLX_TRUE_COLOR, etc.)
			   */

    /* EXT_visual_rating / GLX 1.2 */
    GLint visualRating;

    /* EXT_visual_info / GLX 1.2 */
    GLint transparentPixel;
				/*    colors are floats scaled to ints */
    GLint transparentRed, transparentGreen, transparentBlue, transparentAlpha;
    GLint transparentIndex;

    /* ARB_multisample / SGIS_multisample */
    GLint sampleBuffers;
    GLint samples;

    /* SGIX_fbconfig / GLX 1.3 */
    GLint drawableType;
    GLint renderType;
    GLint xRenderable;
    GLint fbconfigID;

    /* SGIX_pbuffer / GLX 1.3 */
    GLint maxPbufferWidth;
    GLint maxPbufferHeight;
    GLint maxPbufferPixels;
    GLint optimalPbufferWidth;   /* Only for SGIX_pbuffer. */
    GLint optimalPbufferHeight;  /* Only for SGIX_pbuffer. */

    /* SGIX_visual_select_group */
    GLint visualSelectGroup;

    /* OML_swap_method */
    GLint swapMethod;

    GLint screen;

    /* EXT_texture_from_pixmap */
    GLint bindToTextureRgb;
    GLint bindToTextureRgba;
    GLint bindToMipmapTexture;
    GLint bindToTextureTargets;
    GLint yInverted;
} __GLcontextModes;

/* Several fields of __GLcontextModes can take these as values.  Since
 * GLX header files may not be available everywhere they need to be used,
 * redefine them here.
 */
#define GLX_NONE                           0x8000
#define GLX_SLOW_CONFIG                    0x8001
#define GLX_TRUE_COLOR                     0x8002
#define GLX_DIRECT_COLOR                   0x8003
#define GLX_PSEUDO_COLOR                   0x8004
#define GLX_STATIC_COLOR                   0x8005
#define GLX_GRAY_SCALE                     0x8006
#define GLX_STATIC_GRAY                    0x8007
#define GLX_TRANSPARENT_RGB                0x8008
#define GLX_TRANSPARENT_INDEX              0x8009
#define GLX_NON_CONFORMANT_CONFIG          0x800D
#define GLX_SWAP_EXCHANGE_OML              0x8061
#define GLX_SWAP_COPY_OML                  0x8062
#define GLX_SWAP_UNDEFINED_OML             0x8063

#define GLX_DONT_CARE                      0xFFFFFFFF

#define GLX_RGBA_BIT                       0x00000001
#define GLX_COLOR_INDEX_BIT                0x00000002
#define GLX_WINDOW_BIT                     0x00000001
#define GLX_PIXMAP_BIT                     0x00000002
#define GLX_PBUFFER_BIT                    0x00000004

#define GLX_BIND_TO_TEXTURE_RGB_EXT        0x20D0
#define GLX_BIND_TO_TEXTURE_RGBA_EXT       0x20D1
#define GLX_BIND_TO_MIPMAP_TEXTURE_EXT     0x20D2
#define GLX_BIND_TO_TEXTURE_TARGETS_EXT    0x20D3
#define GLX_Y_INVERTED_EXT                 0x20D4

#define GLX_TEXTURE_1D_BIT_EXT             0x00000001
#define GLX_TEXTURE_2D_BIT_EXT             0x00000002
#define GLX_TEXTURE_RECTANGLE_BIT_EXT      0x00000004

#endif /* __gl_core_h_ */
