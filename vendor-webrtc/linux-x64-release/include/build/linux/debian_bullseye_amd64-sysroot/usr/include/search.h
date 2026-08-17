/* Declarations for System V style searching functions.
   Copyright (C) 1995-2020 Free Software Foundation, Inc.
   This file is part of the GNU C Library.

   The GNU C Library is free software; you can redistribute it and/or
   modify it under the terms of the GNU Lesser General Public
   License as published by the Free Software Foundation; either
   version 2.1 of the License, or (at your option) any later version.

   The GNU C Library is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public
   License along with the GNU C Library; if not, see
   <https://www.gnu.org/licenses/>.  */

#ifndef _SEARCH_H
#define	_SEARCH_H 1

#include <features.h>

#define __need_size_t
#include <stddef.h>

__BEGIN_DECLS

#if defined __USE_MISC || defined __USE_XOPEN_EXTENDED
/* Prototype structure for a linked-list data structure.
   This is the type used by the `insque' and `remque' functions.  */

# ifdef __USE_GNU
struct qelem
  {
    struct qelem *q_forw;
    struct qelem *q_back;
    char q_data[1];
  };
# endif


/* Insert ELEM into a doubly-linked list, after PREV.  */
extern void insque (void *__elem, void *__prev) __THROW;

/* Unlink ELEM from the doubly-linked list that it is in.  */
extern void remque (void *__elem) __THROW;
#endif


/* For use with hsearch(3).  */
#ifndef __COMPAR_FN_T
# define __COMPAR_FN_T
typedef int (*__compar_fn_t) (const void *, const void *);

# ifdef	__USE_GNU
typedef __compar_fn_t comparison_fn_t;
# endif
#endif

/* Action which shall be performed in the call the hsearch.  */
typedef enum
  {
    FIND,
    ENTER
  }
ACTION;

typedef struct entry
  {
    char *key;
    void *data;
  }
ENTRY;

/* Opaque type for internal use.  */
struct _ENTRY;

/* Family of hash table handling functions.  The functions also
   have reentrant counterparts ending with _r.  The non-reentrant
   functions all work on a signle internal hashing table.  */

/* Search for entry matching ITEM.key in internal hash table.  If
   ACTION is `FIND' return found entry or signal error by returning
   NULL.  If ACTION is `ENTER' replace existing data (if any) with
   ITEM.data.  */
extern ENTRY *hsearch (ENTRY __item, ACTION __action) __THROW;

/* Create a new hashing table which will at most contain NEL elements.  */
extern int hcreate (size_t __nel) __THROW;

/* Destroy current internal hashing table.  */
extern void hdestroy (void) __THROW;

#ifdef __USE_GNU
/* Data type for reentrant functions.  */
struct hsearch_data
  {
    struct _ENTRY *table;
    unsigned int size;
    unsigned int filled;
  };

/* Reentrant versions which can handle multiple hashing tables at the
   same time.  */
extern int hsearch_r (ENTRY __item, ACTION __action, ENTRY **__retval,
		      struct hsearch_data *__htab) __THROW;
extern int hcreate_r (size_t __nel, struct hsearch_data *__htab) __THROW;
extern void hdestroy_r (struct hsearch_data *__htab) __THROW;
#endif


/* The tsearch routines are very interesting. They make many
   assumptions about the compiler.  It assumes that the first field
   in node must be the "key" field, which points to the datum.
   Everything depends on that.  */
/* For tsearch */
typedef enum
{
  preorder,
  postorder,
  endorder,
  leaf
}
VISIT;

/* Search for an entry matching the given KEY in the tree pointed to
   by *ROOTP and insert a new element if not found.  */
extern void *tsearch (const void *__key, void **__rootp,
		      __compar_fn_t __compar);

/* Search for an entry matching the given KEY in the tree pointed to
   by *ROOTP.  If no matching entry is available return NULL.  */
extern void *tfind (const void *__key, void *const *__rootp,
		    __compar_fn_t __compar);

/* Remove the element matching KEY from the tree pointed to by *ROOTP.  */
extern void *tdelete (const void *__restrict __key,
		      void **__restrict __rootp,
		      __compar_fn_t __compar);

#ifndef __ACTION_FN_T
# define __ACTION_FN_T
typedef void (*__action_fn_t) (const void *__nodep, VISIT __value,
			       int __level);
#endif

/* Walk through the whole tree and call the ACTION callback for every node
   or leaf.  */
extern void twalk (const void *__root, __action_fn_t __action);

#ifdef __USE_GNU
/* Like twalk, but pass down a closure parameter instead of the
   level.  */
extern void twalk_r (const void *__root,
		     void (*) (const void *__nodep, VISIT __value,
			       void *__closure),
		     void *__closure);

/* Callback type for function to free a tree node.  If the keys are atomic
   data this function should do nothing.  */
typedef void (*__free_fn_t) (void *__nodep);

/* Destroy the whole tree, call FREEFCT for each node or leaf.  */
extern void tdestroy (void *__root, __free_fn_t __freefct);
#endif


/* Perform linear search for KEY by comparing by COMPAR in an array
   [BASE,BASE+NMEMB*SIZE).  */
extern void *lfind (const void *__key, const void *__base,
		    size_t *__nmemb, size_t __size, __compar_fn_t __compar);

/* Perform linear search for KEY by comparing by COMPAR function in
   array [BASE,BASE+NMEMB*SIZE) and insert entry if not found.  */
extern void *lsearch (const void *__key, void *__base,
		      size_t *__nmemb, size_t __size, __compar_fn_t __compar);

__END_DECLS

#endif /* search.h */
