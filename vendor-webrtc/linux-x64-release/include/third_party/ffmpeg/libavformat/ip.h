/*
 * IP common code
 *
 * This file is part of FFmpeg.
 *
 * FFmpeg is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public License
 * as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * FFmpeg is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with FFmpeg; if not, write to the Free Software * Foundation, Inc.,
 * 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
 */

#ifndef AVFORMAT_IP_H
#define AVFORMAT_IP_H

#include "network.h"

/**
 * Structure for storing IP (UDP) source filters or block lists.
 */
typedef struct IPSourceFilters {
    int nb_include_addrs;
    int nb_exclude_addrs;
    struct sockaddr_storage *include_addrs;
    struct sockaddr_storage *exclude_addrs;
} IPSourceFilters;

/**
 * Checks the source address against a given IP source filter.
 * @return 0 if packet should be processed based on the filter, 1 if the packet
 *         can be dropped.
 */
int ff_ip_check_source_lists(struct sockaddr_storage *source_addr_ptr, IPSourceFilters *s);

/**
 * Resolves hostname into an addrinfo structure.
 * @return addrinfo structure which should be freed by the user, NULL in case
 *         of error.
 */
struct addrinfo *ff_ip_resolve_host(void *log_ctx,
                                    const char *hostname, int port,
                                    int type, int family, int flags);

/**
 * Parses the address[,address] source list in buf and adds it to the filters
 * in the IPSourceFilters structure.
 * @return 0 on success, < 0 AVERROR code on error.
 */
int ff_ip_parse_sources(void *log_ctx, const char *buf, IPSourceFilters *filters);

/**
 * Parses the address[,address] source block list in buf and adds it to the
 * filters in the IPSourceFilters structure.
 * @return 0 on success, < 0 AVERROR code on error.
 */
int ff_ip_parse_blocks(void *log_ctx, const char *buf, IPSourceFilters *filters);

/**
 * Resets the IP filter list and frees the internal fields of an
 * IPSourceFilters structure.
 */
void ff_ip_reset_filters(IPSourceFilters *filters);

#endif /* AVFORMAT_IP_H */
