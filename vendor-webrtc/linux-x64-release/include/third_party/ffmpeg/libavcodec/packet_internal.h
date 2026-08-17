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

#ifndef AVCODEC_PACKET_INTERNAL_H
#define AVCODEC_PACKET_INTERNAL_H

#include <stdint.h>

#include "packet.h"

#define AVPACKET_IS_EMPTY(pkt) (!(pkt)->data && !(pkt)->side_data_elems)

typedef struct PacketListEntry {
    struct PacketListEntry *next;
    AVPacket pkt;
} PacketListEntry;

typedef struct PacketList {
    PacketListEntry *head, *tail;
} PacketList;

/**
 * Append an AVPacket to the list.
 *
 * @param list  A PacketList
 * @param pkt   The packet being appended. The data described in it will
 *              be made reference counted if it isn't already.
 * @param copy  A callback to copy the contents of the packet to the list.
                May be null, in which case the packet's reference will be
                moved to the list.
 * @return 0 on success, negative AVERROR value on failure. On failure,
           the packet and the list are unchanged.
 */
int avpriv_packet_list_put(PacketList *list, AVPacket *pkt,
                           int (*copy)(AVPacket *dst, const AVPacket *src),
                           int flags);

/**
 * Remove the oldest AVPacket in the list and return it.
 *
 * @note The pkt will be overwritten completely on success. The caller
 *       owns the packet and must unref it by itself.
 *
 * @param head A pointer to a PacketList struct
 * @param pkt  Pointer to an AVPacket struct
 * @return 0 on success, and a packet is returned. AVERROR(EAGAIN) if
 *         the list was empty.
 */
int avpriv_packet_list_get(PacketList *list, AVPacket *pkt);

/**
 * Wipe the list and unref all the packets in it.
 */
void avpriv_packet_list_free(PacketList *list);

int ff_side_data_set_encoder_stats(AVPacket *pkt, int quality, int64_t *error, int error_count, int pict_type);

int ff_side_data_set_prft(AVPacket *pkt, int64_t timestamp);

#endif // AVCODEC_PACKET_INTERNAL_H
