/* Copyright © 2006 Jamey Sharp.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 * 
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 * 
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
 * ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 * 
 * Except as contained in this notice, the names of the authors or their
 * institutions shall not be used in advertising or otherwise to promote the
 * sale, use or other dealings in this Software without prior written
 * authorization from the authors.
 */

#ifndef XCB_RENDERUTIL
#define XCB_RENDERUTIL
#include <xcb/render.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef enum xcb_pict_format_t {
	XCB_PICT_FORMAT_ID         = (1 << 0),
	XCB_PICT_FORMAT_TYPE       = (1 << 1),
	XCB_PICT_FORMAT_DEPTH      = (1 << 2),
	XCB_PICT_FORMAT_RED        = (1 << 3),
	XCB_PICT_FORMAT_RED_MASK   = (1 << 4),
	XCB_PICT_FORMAT_GREEN      = (1 << 5),
	XCB_PICT_FORMAT_GREEN_MASK = (1 << 6),
	XCB_PICT_FORMAT_BLUE       = (1 << 7),
	XCB_PICT_FORMAT_BLUE_MASK  = (1 << 8),
	XCB_PICT_FORMAT_ALPHA      = (1 << 9),
	XCB_PICT_FORMAT_ALPHA_MASK = (1 << 10),
	XCB_PICT_FORMAT_COLORMAP   = (1 << 11)
} xcb_pict_format_t;

typedef enum xcb_pict_standard_t {
	XCB_PICT_STANDARD_ARGB_32,
	XCB_PICT_STANDARD_RGB_24,
	XCB_PICT_STANDARD_A_8,
	XCB_PICT_STANDARD_A_4,
	XCB_PICT_STANDARD_A_1
} xcb_pict_standard_t;


xcb_render_pictvisual_t *
xcb_render_util_find_visual_format (const xcb_render_query_pict_formats_reply_t *formats,
			       const xcb_visualid_t visual);

xcb_render_pictforminfo_t *
xcb_render_util_find_format (const xcb_render_query_pict_formats_reply_t	*formats,
			 unsigned long				mask,
			 const xcb_render_pictforminfo_t		*ptemplate,
			 int					count);

xcb_render_pictforminfo_t *
xcb_render_util_find_standard_format (const xcb_render_query_pict_formats_reply_t	*formats,
				 xcb_pict_standard_t					format);

const xcb_render_query_version_reply_t *
xcb_render_util_query_version (xcb_connection_t *c);

const xcb_render_query_pict_formats_reply_t *
xcb_render_util_query_formats (xcb_connection_t *c);

int
xcb_render_util_disconnect (xcb_connection_t *c);

/* wrappers for xcb_render_composite_glyphs_8/16/32 */

typedef struct xcb_render_util_composite_text_stream_t xcb_render_util_composite_text_stream_t;

xcb_render_util_composite_text_stream_t *
xcb_render_util_composite_text_stream (
	xcb_render_glyphset_t initial_glyphset,
	uint32_t              total_glyphs,
	uint32_t              total_glyphset_changes );

void
xcb_render_util_glyphs_8 (
	xcb_render_util_composite_text_stream_t *stream,
	int16_t  dx,
	int16_t  dy,
	uint32_t count,
	const uint8_t *glyphs );

void
xcb_render_util_glyphs_16 (
	xcb_render_util_composite_text_stream_t *stream,
	int16_t  dx,
	int16_t  dy,
	uint32_t count,
	const uint16_t *glyphs );

void
xcb_render_util_glyphs_32 (
	xcb_render_util_composite_text_stream_t *stream,
	int16_t  dx,
	int16_t  dy,
	uint32_t count,
	const uint32_t *glyphs );

void
xcb_render_util_change_glyphset (
	xcb_render_util_composite_text_stream_t *stream,
	xcb_render_glyphset_t glyphset );

xcb_void_cookie_t
xcb_render_util_composite_text (
	xcb_connection_t        *xc,
	uint8_t                  op,
	xcb_render_picture_t     src,
	xcb_render_picture_t     dst,
	xcb_render_pictformat_t  mask_format,
	int16_t                  src_x,
	int16_t                  src_y,
	xcb_render_util_composite_text_stream_t *stream );

xcb_void_cookie_t
xcb_render_util_composite_text_checked (
	xcb_connection_t        *xc,
	uint8_t                  op,
	xcb_render_picture_t     src,
	xcb_render_picture_t     dst,
	xcb_render_pictformat_t  mask_format,
	int16_t                  src_x,
	int16_t                  src_y,
	xcb_render_util_composite_text_stream_t *stream );

void
xcb_render_util_composite_text_free (
	xcb_render_util_composite_text_stream_t *stream );

#ifdef __cplusplus
}
#endif

#endif /* XCB_RENDERUTIL */
