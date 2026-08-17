/*
 * Copyright © 2008 Nicolai Haehnle
 * Copyright © 2008 Jérôme Glisse
 * All Rights Reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the
 * "Software"), to deal in the Software without restriction, including
 * without limitation the rights to use, copy, modify, merge, publish,
 * distribute, sub license, and/or sell copies of the Software, and to
 * permit persons to whom the Software is furnished to do so, subject to
 * the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDERS, AUTHORS AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM,
 * DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
 * OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE
 * USE OR OTHER DEALINGS IN THE SOFTWARE.
 *
 * The above copyright notice and this permission notice (including the
 * next paragraph) shall be included in all copies or substantial portions
 * of the Software.
 */
/*
 * Authors:
 *      Aapo Tahkola <aet@rasterburn.org>
 *      Nicolai Haehnle <prefect_@gmx.net>
 *      Jérôme Glisse <glisse@freedesktop.org>
 */
#ifndef RADEON_CS_H
#define RADEON_CS_H

#include <stdint.h>
#include <string.h>
#include "drm.h"
#include "radeon_drm.h"
#include "radeon_bo.h"

struct radeon_cs_reloc {
    struct radeon_bo    *bo;
    uint32_t            read_domain;
    uint32_t            write_domain;
    uint32_t            flags;
};


#define RADEON_CS_SPACE_OK 0
#define RADEON_CS_SPACE_OP_TO_BIG 1
#define RADEON_CS_SPACE_FLUSH 2

struct radeon_cs {
    uint32_t *packets;
    unsigned cdw;
    unsigned ndw;
    unsigned                    section_ndw;
    unsigned                    section_cdw;
};

#define MAX_SPACE_BOS (32)

struct radeon_cs_manager;

extern struct radeon_cs *radeon_cs_create(struct radeon_cs_manager *csm,
                                          uint32_t ndw);

extern int radeon_cs_begin(struct radeon_cs *cs,
                           uint32_t ndw,
                           const char *file,
                           const char *func, int line);
extern int radeon_cs_end(struct radeon_cs *cs,
                         const char *file,
                         const char *func,
                         int line);
extern int radeon_cs_emit(struct radeon_cs *cs);
extern int radeon_cs_destroy(struct radeon_cs *cs);
extern int radeon_cs_erase(struct radeon_cs *cs);
extern int radeon_cs_need_flush(struct radeon_cs *cs);
extern void radeon_cs_print(struct radeon_cs *cs, FILE *file);
extern void radeon_cs_set_limit(struct radeon_cs *cs, uint32_t domain, uint32_t limit);
extern void radeon_cs_space_set_flush(struct radeon_cs *cs, void (*fn)(void *), void *data);
extern int radeon_cs_write_reloc(struct radeon_cs *cs,
                                 struct radeon_bo *bo,
                                 uint32_t read_domain,
                                 uint32_t write_domain,
                                 uint32_t flags);
extern uint32_t radeon_cs_get_id(struct radeon_cs *cs);
/*
 * add a persistent BO to the list
 * a persistent BO is one that will be referenced across flushes,
 * i.e. colorbuffer, textures etc.
 * They get reset when a new "operation" happens, where an operation
 * is a state emission with a color/textures etc followed by a bunch of vertices.
 */
void radeon_cs_space_add_persistent_bo(struct radeon_cs *cs,
                                       struct radeon_bo *bo,
                                       uint32_t read_domains,
                                       uint32_t write_domain);

/* reset the persistent BO list */
void radeon_cs_space_reset_bos(struct radeon_cs *cs);

/* do a space check with the current persistent BO list */
int radeon_cs_space_check(struct radeon_cs *cs);

/* do a space check with the current persistent BO list and a temporary BO
 * a temporary BO is like a DMA buffer, which  gets flushed with the
 * command buffer */
int radeon_cs_space_check_with_bo(struct radeon_cs *cs,
                                  struct radeon_bo *bo,
                                  uint32_t read_domains,
                                  uint32_t write_domain);

static inline void radeon_cs_write_dword(struct radeon_cs *cs, uint32_t dword)
{
    cs->packets[cs->cdw++] = dword;
    if (cs->section_ndw) {
        cs->section_cdw++;
    }
}

static inline void radeon_cs_write_qword(struct radeon_cs *cs, uint64_t qword)
{
    memcpy(cs->packets + cs->cdw, &qword, sizeof(uint64_t));
    cs->cdw += 2;
    if (cs->section_ndw) {
        cs->section_cdw += 2;
    }
}

static inline void radeon_cs_write_table(struct radeon_cs *cs,
                                         const void *data, uint32_t size)
{
    memcpy(cs->packets + cs->cdw, data, size * 4);
    cs->cdw += size;
    if (cs->section_ndw) {
        cs->section_cdw += size;
    }
}
#endif
