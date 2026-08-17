/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef _SECITEM_H_
#define _SECITEM_H_

#include "utilrename.h"

/*
 * secitem.h - public data structures and prototypes for handling
 *             SECItems
 */

#include "plarena.h"
#include "plhash.h"
#include "seccomon.h"

SEC_BEGIN_PROTOS

/*
** Allocate an item.  If "arena" is not NULL, then allocate from there,
** otherwise allocate from the heap.  If "item" is not NULL, allocate
** only the data buffer for the item, not the item itself.  If "len" is
** 0, do not allocate the data buffer for the item; simply set the data
** field to NULL and the len field to 0.  The item structure is allocated
** zero-filled; the data buffer is not zeroed.  The caller is responsible
** for initializing the type field of the item.
**
** The resulting item is returned; NULL if any error occurs.
**
** XXX This probably should take a SECItemType, but since that is mostly
** unused and our improved APIs (aka Stan) are looming, I left it out.
*/
extern SECItem *SECITEM_AllocItem(PLArenaPool *arena, SECItem *item,
                                  unsigned int len);

/* Allocate and make an item with the requested contents.
 *
 * We seem to have mostly given up on SECItemType, so the result is
 * always siBuffer.
 */
extern SECStatus SECITEM_MakeItem(PLArenaPool *arena, SECItem *dest,
                                  const unsigned char *data, unsigned int len);

/*
** This is a legacy function containing bugs. It doesn't update item->len,
** and it has other issues as described in bug 298649 and bug 298938.
** However, the function is  kept unchanged for consumers that might depend
** on the broken behaviour. New code should call SECITEM_ReallocItemV2.
**
** Reallocate the data for the specified "item".  If "arena" is not NULL,
** then reallocate from there, otherwise reallocate from the heap.
** In the case where oldlen is 0, the data is allocated (not reallocated).
** In any case, "item" is expected to be a valid SECItem pointer;
** SECFailure is returned if it is not.  If the allocation succeeds,
** SECSuccess is returned.
*/
extern SECStatus SECITEM_ReallocItem(/* deprecated function */
                                     PLArenaPool *arena, SECItem *item,
                                     unsigned int oldlen, unsigned int newlen);

/*
** Reallocate the data for the specified "item".  If "arena" is not NULL,
** then reallocate from there, otherwise reallocate from the heap.
** If item->data is NULL, the data is allocated (not reallocated).
** In any case, "item" is expected to be a valid SECItem pointer;
** SECFailure is returned if it is not, and the item will remain unchanged.
** If the allocation succeeds, the item is updated and SECSuccess is returned.
 */
extern SECStatus SECITEM_ReallocItemV2(PLArenaPool *arena, SECItem *item,
                                       unsigned int newlen);

/*
** Compare two items returning the difference between them.
*/
extern SECComparison SECITEM_CompareItem(const SECItem *a, const SECItem *b);

/*
** Compare two items -- if they are the same, return true; otherwise false.
*/
extern PRBool SECITEM_ItemsAreEqual(const SECItem *a, const SECItem *b);

/*
** Copy "from" to "to"
*/
extern SECStatus SECITEM_CopyItem(PLArenaPool *arena, SECItem *to,
                                  const SECItem *from);

/*
** Allocate an item and copy "from" into it.
*/
extern SECItem *SECITEM_DupItem(const SECItem *from);

/*
** Allocate an item and copy "from" into it.  The item itself and the
** data it points to are both allocated from the arena.  If arena is
** NULL, this function is equivalent to SECITEM_DupItem.
*/
extern SECItem *SECITEM_ArenaDupItem(PLArenaPool *arena, const SECItem *from);

/*
** Free "zap". If freeit is PR_TRUE then "zap" itself is freed.
*/
extern void SECITEM_FreeItem(SECItem *zap, PRBool freeit);

/*
** Zero and then free "zap". If freeit is PR_TRUE then "zap" itself is freed.
*/
extern void SECITEM_ZfreeItem(SECItem *zap, PRBool freeit);

PLHashNumber PR_CALLBACK SECITEM_Hash(const void *key);

PRIntn PR_CALLBACK SECITEM_HashCompare(const void *k1, const void *k2);

extern SECItemArray *SECITEM_AllocArray(PLArenaPool *arena,
                                        SECItemArray *array,
                                        unsigned int len);
extern SECItemArray *SECITEM_DupArray(PLArenaPool *arena,
                                      const SECItemArray *from);
extern void SECITEM_FreeArray(SECItemArray *array, PRBool freeit);
extern void SECITEM_ZfreeArray(SECItemArray *array, PRBool freeit);

SEC_END_PROTOS

#endif /* _SECITEM_H_ */
