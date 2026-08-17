/*
 * Copyright (c) 2016 Reimar Döffinger <Reimar.Doeffinger@gmx.de>
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

#ifndef AVCODEC_CBRT_DATA_H
#define AVCODEC_CBRT_DATA_H

#include <stdint.h>

#include "config.h"

#if CONFIG_HARDCODED_TABLES
#define ff_cbrt_tableinit_fixed()
#define ff_cbrt_tableinit()
extern const uint32_t ff_cbrt_tab[1 << 13];
extern const uint32_t ff_cbrt_tab_fixed[1 << 13];
#else
void ff_cbrt_tableinit(void);
void ff_cbrt_tableinit_fixed(void);
extern uint32_t ff_cbrt_tab[1 << 13];
extern uint32_t ff_cbrt_tab_fixed[1 << 13];
#endif

#endif
