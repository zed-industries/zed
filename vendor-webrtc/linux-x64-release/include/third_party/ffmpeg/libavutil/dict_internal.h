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

#ifndef AVUTIL_DICT_INTERNAL_H
#define AVUTIL_DICT_INTERNAL_H

#include <stdint.h>

#include "dict.h"

/**
 * Set a dictionary value to an ISO-8601 compliant timestamp string.
 *
 * @param dict pointer to a pointer to a dictionary struct. If *dict is NULL
 *             a dictionary struct is allocated and put in *dict.
 * @param key metadata key
 * @param timestamp unix timestamp in microseconds
 * @return <0 on error
 */
int avpriv_dict_set_timestamp(AVDictionary **dict, const char *key, int64_t timestamp);

#endif /* AVUTIL_DICT_INTERNAL_H */
