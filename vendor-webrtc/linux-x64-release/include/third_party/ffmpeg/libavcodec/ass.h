/*
 * SSA/ASS common functions
 * Copyright (c) 2010  Aurelien Jacobs <aurel@gnuage.org>
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

#ifndef AVCODEC_ASS_H
#define AVCODEC_ASS_H

#include "avcodec.h"
#include "libavutil/bprint.h"

#define ASS_DEFAULT_PLAYRESX 384
#define ASS_DEFAULT_PLAYRESY 288

/**
 * @name Default values for ASS style
 * @{
 */
#define ASS_DEFAULT_FONT        "Arial"
#define ASS_DEFAULT_FONT_SIZE   16
#define ASS_DEFAULT_COLOR       0xffffff
#define ASS_DEFAULT_BACK_COLOR  0
#define ASS_DEFAULT_BOLD        0
#define ASS_DEFAULT_ITALIC      0
#define ASS_DEFAULT_UNDERLINE   0
#define ASS_DEFAULT_ALIGNMENT   2
#define ASS_DEFAULT_BORDERSTYLE 1
/** @} */

typedef struct FFASSDecoderContext {
    int readorder;
} FFASSDecoderContext;

/**
 * Generate a suitable AVCodecContext.subtitle_header for SUBTITLE_ASS.
 * Can specify all fields explicitly
 *
 * @param avctx pointer to the AVCodecContext
 * @param play_res_x subtitle frame width
 * @param play_res_y subtitle frame height
 * @param font name of the default font face to use
 * @param font_size default font size to use
 * @param primary_color default text color to use (ABGR)
 * @param secondary_color default secondary text color to use (ABGR)
 * @param outline_color default outline color to use (ABGR)
 * @param back_color default background color to use (ABGR)
 * @param bold 1 for bold text, 0 for normal text
 * @param italic 1 for italic text, 0 for normal text
 * @param underline 1 for underline text, 0 for normal text
 * @param border_style 1 for outline, 3 for opaque box
 * @param alignment position of the text (left, center, top...), defined after
 *                  the layout of the numpad (1-3 sub, 4-6 mid, 7-9 top)
 * @return >= 0 on success otherwise an error code <0
 */
int ff_ass_subtitle_header_full(AVCodecContext *avctx,
                                int play_res_x, int play_res_y,
                                const char *font, int font_size,
                                int primary_color, int secondary_color,
                                int outline_color, int back_color,
                                int bold, int italic, int underline,
                                int border_style, int alignment);
/**
 * Generate a suitable AVCodecContext.subtitle_header for SUBTITLE_ASS.
 *
 * @param avctx pointer to the AVCodecContext
 * @param font name of the default font face to use
 * @param font_size default font size to use
 * @param color default text color to use (ABGR)
 * @param back_color default background color to use (ABGR)
 * @param bold 1 for bold text, 0 for normal text
 * @param italic 1 for italic text, 0 for normal text
 * @param underline 1 for underline text, 0 for normal text
 * @param alignment position of the text (left, center, top...), defined after
 *                  the layout of the numpad (1-3 sub, 4-6 mid, 7-9 top)
 * @return >= 0 on success otherwise an error code <0
 */
int ff_ass_subtitle_header(AVCodecContext *avctx,
                           const char *font, int font_size,
                           int color, int back_color,
                           int bold, int italic, int underline,
                           int border_style, int alignment);

/**
 * Generate a suitable AVCodecContext.subtitle_header for SUBTITLE_ASS
 * with default style.
 *
 * @param avctx pointer to the AVCodecContext
 * @return >= 0 on success otherwise an error code <0
 */
int ff_ass_subtitle_header_default(AVCodecContext *avctx);

/**
 * Craft an ASS dialog string.
 */
char *ff_ass_get_dialog(int readorder, int layer, const char *style,
                        const char *speaker, const char *text);

/**
 * Add an ASS dialog to a subtitle.
 */
int ff_ass_add_rect(AVSubtitle *sub, const char *dialog,
                    int readorder, int layer, const char *style,
                    const char *speaker);

/**
 * Add an ASS dialog to a subtitle.
 */
int ff_ass_add_rect2(AVSubtitle *sub, const char *dialog,
                     int readorder, int layer, const char *style,
                     const char *speaker, unsigned *nb_rect_allocated);

/**
 * Helper to flush a text subtitles decoder making use of the
 * FFASSDecoderContext.
 */
void ff_ass_decoder_flush(AVCodecContext *avctx);

/**
 * Escape a text subtitle using ASS syntax into an AVBPrint buffer.
 * Newline characters will be escaped to \N.
 *
 * @param buf pointer to an initialized AVBPrint buffer
 * @param p source text
 * @param size size of the source text
 * @param linebreaks additional newline chars, which will be escaped to \N
 * @param keep_ass_markup braces and backslash will not be escaped if set
 */
void ff_ass_bprint_text_event(AVBPrint *buf, const char *p, int size,
                             const char *linebreaks, int keep_ass_markup);
#endif /* AVCODEC_ASS_H */
