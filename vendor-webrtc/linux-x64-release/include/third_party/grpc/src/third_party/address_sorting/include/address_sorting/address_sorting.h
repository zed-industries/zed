/*	$NetBSD: getaddrinfo.c,v 1.82 2006/03/25 12:09:40 rpaulo Exp $	*/
/*	$KAME: getaddrinfo.c,v 1.29 2000/08/31 17:26:57 itojun Exp $	*/
/*
 * Copyright (C) 1995, 1996, 1997, and 1998 WIDE Project.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. Neither the name of the project nor the names of its contributors
 *    may be used to endorse or promote products derived from this software
 *    without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE PROJECT AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE PROJECT OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 *
 */

/*
 * This is an adaptation of Android's implementation of RFC 6724
 * (in Android's getaddrinfo.c). It has some cosmetic differences
 * from Android's getaddrinfo.c, but Android's getaddrinfo.c was
 * used as a guide or example of a way to implement the RFC 6724 spec when
 * this was written.
 */

#ifndef ADDRESS_SORTING_H
#define ADDRESS_SORTING_H

#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct address_sorting_address {
  char addr[128];
  size_t len;
} address_sorting_address;

/* address_sorting_sortable represents one entry in a list of destination
 * IP addresses to sort. It contains the destination IP address
 * "sorting key", along with placeholder and scratch fields. */
typedef struct address_sorting_sortable {
  // input data; sorting key
  address_sorting_address dest_addr;
  // input data; optional value to attach to the sorting key
  void* user_data;
  // internal fields, these must be zero'd when passed to sort function
  address_sorting_address source_addr;
  bool source_addr_exists;
  size_t original_index;
} address_sorting_sortable;

void address_sorting_rfc_6724_sort(address_sorting_sortable* sortables,
                                   size_t sortables_len);

void address_sorting_init();
void address_sorting_shutdown();

struct address_sorting_source_addr_factory;

/* The interfaces below are exposed only for testing */
typedef struct {
  /* Gets the source address that would be used for the passed-in destination
   * address, and fills in *source_addr* with it if one exists.
   * Returns true if a source address exists for the destination address,
   * and false otherwise. */
  bool (*get_source_addr)(struct address_sorting_source_addr_factory* factory,
                          const address_sorting_address* dest_addr,
                          address_sorting_address* source_addr);
  void (*destroy)(struct address_sorting_source_addr_factory* factory);
} address_sorting_source_addr_factory_vtable;

typedef struct address_sorting_source_addr_factory {
  const address_sorting_source_addr_factory_vtable* vtable;
} address_sorting_source_addr_factory;

/* Platform-compatible address family types */
typedef enum {
  ADDRESS_SORTING_AF_INET,
  ADDRESS_SORTING_AF_INET6,
  ADDRESS_SORTING_UNKNOWN_FAMILY,
} address_sorting_family;

/* Indicates whether the address is AF_INET, AF_INET6, or another address
 * family. */
address_sorting_family address_sorting_abstract_get_family(
    const address_sorting_address* address);

void address_sorting_override_source_addr_factory_for_testing(
    address_sorting_source_addr_factory* factory);

bool address_sorting_get_source_addr_for_testing(
    const address_sorting_address* dest, address_sorting_address* source);

#ifdef __cplusplus
}
#endif

#endif  // ADDRESS_SORTING_H
