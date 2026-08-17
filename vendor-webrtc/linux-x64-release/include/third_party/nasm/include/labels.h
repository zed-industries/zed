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
 * labels.h  header file for labels.c
 */

#ifndef LABELS_H
#define LABELS_H

#include "compiler.h"

enum mangle_index {
    LM_LPREFIX,                 /* Local variable prefix */
    LM_LSUFFIX,                 /* Local variable suffix */
    LM_GPREFIX,                 /* Global variable prefix */
    LM_GSUFFIX                  /* GLobal variable suffix */
};

enum label_type {
    LBL_none = -1,              /* No label */
    LBL_LOCAL = 0,              /* Must be zero */
    LBL_STATIC,
    LBL_GLOBAL,
    LBL_EXTERN,
    LBL_REQUIRED,               /* Like extern but emit even if unused */
    LBL_COMMON,
    LBL_SPECIAL,                /* Magic symbols like ..start */
    LBL_BACKEND                 /* Backend-defined symbols like ..got */
};

enum label_type lookup_label(const char *label, int32_t *segment, int64_t *offset);
static inline bool is_extern(enum label_type type)
{
    return type == LBL_EXTERN || type == LBL_REQUIRED;
}
void define_label(const char *label, int32_t segment, int64_t offset,
                  bool normal);
void backend_label(const char *label, int32_t segment, int64_t offset);
bool declare_label(const char *label, enum label_type type,
                   const char *special);
void set_label_mangle(enum mangle_index which, const char *what);
int init_labels(void);
void cleanup_labels(void);
const char *local_scope(const char *label);

extern uint64_t global_offset_changed;

#endif /* LABELS_H */
