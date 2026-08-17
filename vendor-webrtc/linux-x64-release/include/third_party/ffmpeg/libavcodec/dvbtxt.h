/*
 * DVB teletext common functions.
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

#ifndef AVCODEC_DVBTXT_H
#define AVCODEC_DVBTXT_H

#include "libavutil/attributes.h"

/* Returns true if data identifier matches a teletext stream according to EN
 * 301 775 section 4.4.2 */
static av_always_inline int ff_data_identifier_is_teletext(int data_identifier)
{
    return (data_identifier >= 0x10 && data_identifier <= 0x1F ||
            data_identifier >= 0x99 && data_identifier <= 0x9B);
}

/* Returns true if data unit id matches EBU teletext data according to
 * EN 301 775 section 4.4.2 */
static av_always_inline int ff_data_unit_id_is_teletext(int data_unit_id)
{
    return (data_unit_id == 0x02 || data_unit_id == 0x03);
}

#endif /* AVCODEC_DVBTXT_H */
