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

#ifndef NASM_RBTREE_H
#define NASM_RBTREE_H

#include "compiler.h"

/*
 * This structure should be embedded in a larger data structure;
 * the final output from rb_search() can then be converted back
 * to the larger data structure via container_of().
 *
 * An empty tree is simply represented by a NULL pointer.
 */

/* Note: the values of these flags is significant */
enum rbtree_node_flags {
    RBTREE_NODE_BLACK	= 1,  /* Node color is black */
    RBTREE_NODE_PRED    = 2,  /* Left pointer is an uplink */
    RBTREE_NODE_SUCC    = 4   /* Right pointer is an uplink */
};

struct rbtree {
    uint64_t key;
    struct rbtree_metadata {
        struct rbtree *left, *right;
        enum rbtree_node_flags flags;
    } m;
};

/*
 * Add a node to a tree. Returns the new root pointer.
 * The key value in the structure needs to be preinitialized;
 * the rest of the structure should be zero.
 */
struct rbtree *rb_insert(struct rbtree *, struct rbtree *);

/*
 * Find a node in the tree corresponding to the key immediately
 * <= the passed-in key value.
 */
struct rbtree *rb_search(const struct rbtree *, uint64_t);

/*
 * Find a node in the tree exactly matching the key value.
 */
struct rbtree *rb_search_exact(const struct rbtree *, uint64_t);

/*
 * Return the immediately previous or next node in key order.
 * Returns NULL if this node is the end of the tree.
 * These operations are safe for complee (but not partial!)
 * tree walk-with-destruction in key order.
 */
struct rbtree *rb_prev(const struct rbtree *);
struct rbtree *rb_next(const struct rbtree *);

/*
 * Return the very first or very last node in key order.
 */
struct rbtree *rb_first(const struct rbtree *);
struct rbtree *rb_last(const struct rbtree *);

/*
 * Left and right nodes, if real. These operations are
 * safe for tree destruction, but not for splitting a tree.
 */
static inline struct rbtree *rb_left(const struct rbtree *rb)
{
    return (rb->m.flags & RBTREE_NODE_PRED) ? NULL : rb->m.left;
}
static inline struct rbtree *rb_right(const struct rbtree *rb)
{
    return (rb->m.flags & RBTREE_NODE_SUCC) ? NULL : rb->m.right;
}

#endif /* NASM_RBTREE_H */
