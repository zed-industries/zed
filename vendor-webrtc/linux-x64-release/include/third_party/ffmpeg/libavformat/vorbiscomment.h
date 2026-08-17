/*
 * VorbisComment writer
 * Copyright (c) 2009 James Darnley
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

#ifndef AVFORMAT_VORBISCOMMENT_H
#define AVFORMAT_VORBISCOMMENT_H

#include "avformat.h"
#include "metadata.h"

/**
 * Calculate the length in bytes of a VorbisComment. This is the minimum
 * size required by ff_vorbiscomment_write().
 *
 * @param m The metadata structure to be parsed. For no metadata, set to NULL.
 * @param vendor_string The vendor string to be added into the VorbisComment.
 * For no string, set to an empty string.
 * @return The length in bytes.
 */
int64_t ff_vorbiscomment_length(const AVDictionary *m, const char *vendor_string,
                                AVChapter **chapters, unsigned int nb_chapters);

/**
 * Write a VorbisComment into an AVIOContext. The output size can be obtained
 * in advance by passing the same chapters, AVDictionary and vendor_string to
 * ff_vorbiscomment_length()
 *
 * @param pb The AVIOContext to write the output.
 * @param m The metadata struct to write.
 * @param vendor_string The vendor string to write.
 * @param chapters The chapters to write.
 * @param nb_chapters The number of chapters to write.
 */
int ff_vorbiscomment_write(AVIOContext *pb, const AVDictionary *m,
                           const char *vendor_string,
                           AVChapter **chapters, unsigned int nb_chapters);

extern const AVMetadataConv ff_vorbiscomment_metadata_conv[];

#endif /* AVFORMAT_VORBISCOMMENT_H */
