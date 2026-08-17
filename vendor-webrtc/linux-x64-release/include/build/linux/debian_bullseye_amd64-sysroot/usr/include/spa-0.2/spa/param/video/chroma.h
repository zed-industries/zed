/* Simple Plugin API
 *
 * Copyright © 2018 Wim Taymans
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice (including the next
 * paragraph) shall be included in all copies or substantial portions of the
 * Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 */

#ifndef SPA_VIDEO_CHROMA_H
#define SPA_VIDEO_CHROMA_H

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \addtogroup spa_param
 * \{
 */

/** Various Chroma settings.
 */
enum spa_video_chroma_site {
	SPA_VIDEO_CHROMA_SITE_UNKNOWN = 0,		/**< unknown cositing */
	SPA_VIDEO_CHROMA_SITE_NONE = (1 << 0),		/**< no cositing */
	SPA_VIDEO_CHROMA_SITE_H_COSITED = (1 << 1),	/**< chroma is horizontally cosited */
	SPA_VIDEO_CHROMA_SITE_V_COSITED = (1 << 2),	/**< chroma is vertically cosited */
	SPA_VIDEO_CHROMA_SITE_ALT_LINE = (1 << 3),	/**< chroma samples are sited on alternate lines */
	/* some common chroma cositing */
	/** chroma samples cosited with luma samples */
	SPA_VIDEO_CHROMA_SITE_COSITED = (SPA_VIDEO_CHROMA_SITE_H_COSITED | SPA_VIDEO_CHROMA_SITE_V_COSITED),
	/** jpeg style cositing, also for mpeg1 and mjpeg */
	SPA_VIDEO_CHROMA_SITE_JPEG = (SPA_VIDEO_CHROMA_SITE_NONE),
	/** mpeg2 style cositing */
	SPA_VIDEO_CHROMA_SITE_MPEG2 = (SPA_VIDEO_CHROMA_SITE_H_COSITED),
	/**< DV style cositing */
	SPA_VIDEO_CHROMA_SITE_DV = (SPA_VIDEO_CHROMA_SITE_COSITED | SPA_VIDEO_CHROMA_SITE_ALT_LINE),
};

/**
 * \}
 */

#ifdef __cplusplus
} /* extern "C" */
#endif

#endif /* SPA_VIDEO_CHROMA_H */
