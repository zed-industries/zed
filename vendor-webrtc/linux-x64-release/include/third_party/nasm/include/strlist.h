/* ----------------------------------------------------------------------- *
 *
 *   Copyright 1996-2020 The NASM Authors - All Rights Reserved
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
 * strlist.h - list of unique, ordered strings
 */

#ifndef NASM_STRLIST_H
#define NASM_STRLIST_H

#include "compiler.h"
#include "nasmlib.h"
#include "hashtbl.h"

struct strlist_entry {
	struct strlist_entry	*next;
	size_t			offset;
	size_t			size;
	intorptr		pvt;
	char			str[1];
};

struct strlist {
	struct hash_table	hash;
	struct strlist_entry	*head, **tailp;
	size_t			nstr, size;
	bool			uniq;
};

static inline const struct strlist_entry *
strlist_head(const struct strlist *list)
{
	return list ? list->head : NULL;
}
static inline struct strlist_entry *strlist_tail(struct strlist *list)
{
	if (!list || !list->head)
		return NULL;
	return container_of(list->tailp, struct strlist_entry, next);
}
static inline size_t strlist_count(const struct strlist *list)
{
	return list ? list->nstr : 0;
}
static inline size_t strlist_size(const struct strlist *list)
{
	return list ? list->size : 0;
}

struct strlist * safe_alloc strlist_alloc(bool uniq);
const struct strlist_entry *strlist_add(struct strlist *list, const char *str);
const struct strlist_entry * printf_func(2, 3)
	strlist_printf(struct strlist *list, const char *fmt, ...);
const struct strlist_entry * vprintf_func(2)
	strlist_vprintf(struct strlist *list, const char *fmt, va_list ap);
const struct strlist_entry *
strlist_find(const struct strlist *list, const char *str);
void * safe_alloc strlist_linearize(const struct strlist *list, char sep);
void strlist_write(const struct strlist *list, const char *sep, FILE *f);
void strlist_free(struct strlist **listp);
#define strlist_for_each(p,h) list_for_each((p), strlist_head(h))
 
#endif /* NASM_STRLIST_H */
