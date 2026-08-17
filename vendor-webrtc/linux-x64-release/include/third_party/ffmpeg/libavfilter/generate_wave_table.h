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

#ifndef AVFILTER_GENERATE_WAVE_TABLE_H
#define AVFILTER_GENERATE_WAVE_TABLE_H

#include "libavutil/samplefmt.h"

enum WaveType {
    WAVE_SIN,
    WAVE_TRI,
    WAVE_NB,
};

void ff_generate_wave_table(enum WaveType wave_type,
                            enum AVSampleFormat sample_fmt,
                            void *table, int table_size,
                            double min, double max, double phase);

#endif /* AVFILTER_GENERATE_WAVE_TABLE_H */
