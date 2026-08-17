/* Simple Plugin API
 *
 * Copyright © 2018 Wim Taymans
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice (including the next
 * paragraph) shall be included in all copies or substantial portions of the
 * Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 */

#ifndef SPA_LIST_H
#define SPA_LIST_H

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \defgroup spa_list List
 * Doubly linked list data structure
 */

/**
 * \addtogroup spa_list List
 * \{
 */

struct spa_list {
	struct spa_list *next;
	struct spa_list *prev;
};

#define SPA_LIST_INIT(list) (struct spa_list){ list, list }

static inline void spa_list_init(struct spa_list *list)
{
	*list = SPA_LIST_INIT(list);
}

#define spa_list_is_empty(l)  ((l)->next == (l))

static inline void spa_list_insert(struct spa_list *list, struct spa_list *elem)
{
	elem->prev = list;
	elem->next = list->next;
	list->next = elem;
	elem->next->prev = elem;
}

static inline void spa_list_insert_list(struct spa_list *list, struct spa_list *other)
{
	if (spa_list_is_empty(other))
		return;
	other->next->prev = list;
	other->prev->next = list->next;
	list->next->prev = other->prev;
	list->next = other->next;
}

static inline void spa_list_remove(struct spa_list *elem)
{
	elem->prev->next = elem->next;
	elem->next->prev = elem->prev;
}

#define spa_list_first(head, type, member)				\
	SPA_CONTAINER_OF((head)->next, type, member)

#define spa_list_last(head, type, member)				\
	SPA_CONTAINER_OF((head)->prev, type, member)

#define spa_list_append(list, item)					\
	spa_list_insert((list)->prev, item)

#define spa_list_prepend(list, item)					\
	spa_list_insert(list, item)

#define spa_list_is_end(pos, head, member)				\
	(&(pos)->member == (head))

#define spa_list_next(pos, member)					\
	SPA_CONTAINER_OF((pos)->member.next, __typeof__(*pos), member)

#define spa_list_prev(pos, member)					\
	SPA_CONTAINER_OF((pos)->member.prev, __typeof__(*pos), member)

#define spa_list_consume(pos, head, member)				\
	for (pos = spa_list_first(head, __typeof__(*pos), member);	\
	     !spa_list_is_empty(head);					\
	     pos = spa_list_first(head, __typeof__(*pos), member))

#define spa_list_for_each_next(pos, head, curr, member)			\
	for (pos = spa_list_first(curr, __typeof__(*pos), member);	\
	     !spa_list_is_end(pos, head, member);			\
	     pos = spa_list_next(pos, member))

#define spa_list_for_each_prev(pos, head, curr, member)			\
	for (pos = spa_list_last(curr, __typeof__(*pos), member);	\
	     !spa_list_is_end(pos, head, member);			\
	     pos = spa_list_prev(pos, member))

#define spa_list_for_each(pos, head, member)				\
	spa_list_for_each_next(pos, head, head, member)

#define spa_list_for_each_reverse(pos, head, member)			\
	spa_list_for_each_prev(pos, head, head, member)

#define spa_list_for_each_safe_next(pos, tmp, head, curr, member)	\
	for (pos = spa_list_first(curr, __typeof__(*pos), member);	\
	     tmp = spa_list_next(pos, member),				\
	     !spa_list_is_end(pos, head, member);			\
	     pos = tmp)

#define spa_list_for_each_safe_prev(pos, tmp, head, curr, member)	\
	for (pos = spa_list_last(curr, __typeof__(*pos), member);	\
	     tmp = spa_list_prev(pos, member),				\
	     !spa_list_is_end(pos, head, member);			\
	     pos = tmp)

#define spa_list_for_each_safe(pos, tmp, head, member)			\
	spa_list_for_each_safe_next(pos, tmp, head, head, member)

#define spa_list_for_each_safe_reverse(pos, tmp, head, member)		\
	spa_list_for_each_safe_prev(pos, tmp, head, head, member)

#define spa_list_cursor_start(cursor, head, member)                     \
        spa_list_prepend(head, &(cursor).member)

#define spa_list_for_each_cursor(pos, cursor, head, member)             \
        for(pos = spa_list_first(&(cursor).member, __typeof__(*(pos)), member); \
            spa_list_remove(&(pos)->member),                            \
            spa_list_append(&(cursor).member, &(pos)->member),          \
            !spa_list_is_end(pos, head, member);                        \
            pos = spa_list_next(&cursor, member))

#define spa_list_cursor_end(cursor, member)                             \
        spa_list_remove(&(cursor).member)

/**
 * \}
 */

#ifdef __cplusplus
}  /* extern "C" */
#endif

#endif /* SPA_LIST_H */
