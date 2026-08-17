/*
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

#ifndef AVCODEC_KBDWIN_H
#define AVCODEC_KBDWIN_H

#include <stdint.h>

/**
 * Maximum window size for ff_kbd_window_init.
 */
#define FF_KBD_WINDOW_MAX 1024

/**
 * Generate a Kaiser-Bessel Derived Window.
 * @param   window  pointer to half window
 * @param   alpha   determines window shape
 * @param   n       size of half window, max FF_KBD_WINDOW_MAX
 */
void ff_kbd_window_init(float *window, float alpha, int n);
void ff_kbd_window_init_fixed(int32_t *window, float alpha, int n);

#endif /* AVCODEC_KBDWIN_H */
