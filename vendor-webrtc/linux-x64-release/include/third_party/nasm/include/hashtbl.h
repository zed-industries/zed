/* ----------------------------------------------------------------------- *
 *
 *   Copyright 1996-2018 The NASM Authors - All Rights Reserved
 *   See the file AUTHORS included with the NASM distribution for
 *   the specific copyright holders.
 *
 *   Redistribution and use in source and binary forms, with or without
 *   modification, are permitted provided that the following
 *   conditions are met:
 *
 *   * Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 *   * Redistributions in binary form must reproduce the above
 *     copyright notice, this list of conditions and the following
 *     disclaimer in the documentation and/or other materials provided
 *     with the distribution.
 *
 *     THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND
 *     CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
 *     INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
 *     MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 *     DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR
 *     CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 *     SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
 *     NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 *     LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 *     HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 *     CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR
 *     OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE,
 *     EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 * ----------------------------------------------------------------------- */

/*
 * hashtbl.h
 *
 * Efficient dictionary hash table class.
 */

#ifndef NASM_HASHTBL_H
#define NASM_HASHTBL_H

#include "nasmlib.h"

struct hash_node {
    uint64_t hash;
    const void *key;
    size_t keylen;
    void *data;
};

struct hash_table {
    struct hash_node *table;
    size_t load;
    size_t size;
    size_t max_load;
};

struct hash_insert {
    struct hash_table *head;
    struct hash_node *where;
    struct hash_node node;
};

struct hash_iterator {
    const struct hash_table *head;
    const struct hash_node *next;
};

uint64_t crc64(uint64_t crc, const char *string);
uint64_t crc64i(uint64_t crc, const char *string);
uint64_t crc64b(uint64_t crc, const void *data, size_t len);
uint64_t crc64ib(uint64_t crc, const void *data, size_t len);
#define CRC64_INIT UINT64_C(0xffffffffffffffff)

static inline uint64_t crc64_byte(uint64_t crc, uint8_t v)
{
    extern const uint64_t crc64_tab[256];
    return crc64_tab[(uint8_t)(v ^ crc)] ^ (crc >> 8);
}

uint32_t crc32b(uint32_t crc, const void *data, size_t len);

void **hash_find(struct hash_table *head, const char *string,
		struct hash_insert *insert);
void **hash_findb(struct hash_table *head, const void *key, size_t keylen,
		struct hash_insert *insert);
void **hash_findi(struct hash_table *head, const char *string,
		struct hash_insert *insert);
void **hash_findib(struct hash_table *head, const void *key, size_t keylen,
                   struct hash_insert *insert);
void **hash_add(struct hash_insert *insert, const void *key, void *data);
static inline void hash_iterator_init(const struct hash_table *head,
                                      struct hash_iterator *iterator)
{
    iterator->head = head;
    iterator->next = head->table;
}
const struct hash_node *hash_iterate(struct hash_iterator *iterator);

#define hash_for_each(_head,_it,_np) \
    for (hash_iterator_init((_head), &(_it)), (_np) = hash_iterate(&(_it)) ; \
         (_np) ; (_np) = hash_iterate(&(_it)))

void hash_free(struct hash_table *head);
void hash_free_all(struct hash_table *head, bool free_keys);

#endif /* NASM_HASHTBL_H */
