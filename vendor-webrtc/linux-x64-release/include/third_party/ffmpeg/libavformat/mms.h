/*
 * MMS protocol common definitions.
 * Copyright (c) 2010 Zhentan Feng <spyfeng at gmail dot com>
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
#ifndef AVFORMAT_MMS_H
#define AVFORMAT_MMS_H

#include "url.h"

typedef struct MMSStream {
    int id;
}MMSStream;

typedef struct MMSContext {
    URLContext *mms_hd;                  ///< TCP connection handle
    MMSStream *streams;

    /** Buffer for outgoing packets. */
    /*@{*/
    uint8_t *write_out_ptr;              ///< Pointer for writing the buffer.
    uint8_t out_buffer[512];            ///< Buffer for outgoing packet.
    /*@}*/

    /** Buffer for incoming packets. */
    /*@{*/
    uint8_t in_buffer[65536];            ///< Buffer for incoming packets.
    uint8_t *read_in_ptr;                ///< Pointer for reading from incoming buffer.
    int remaining_in_len;                ///< Reading length from incoming buffer.
    /*@}*/

    /** Internal handling of the ASF header */
    /*@{*/
    uint8_t *asf_header;                 ///< Stored ASF header.
    int asf_header_size;                 ///< Size of stored ASF header.
    int header_parsed;                   ///< The header has been received and parsed.
    int asf_packet_len;
    int asf_header_read_size;
    /*@}*/

    int stream_num;                      ///< stream numbers.
    unsigned int nb_streams_allocated;   ///< allocated size of streams
} MMSContext;

int ff_mms_asf_header_parser(MMSContext * mms);
int ff_mms_read_data(MMSContext *mms, uint8_t *buf, const int size);
int ff_mms_read_header(MMSContext * mms, uint8_t * buf, const int size);

#endif /* AVFORMAT_MMS_H */
