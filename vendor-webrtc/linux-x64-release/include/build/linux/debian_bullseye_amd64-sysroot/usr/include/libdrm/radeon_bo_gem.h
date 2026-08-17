/*
 * Copyright © 2008 Dave Airlie
 * Copyright © 2008 Jérôme Glisse
 * All Rights Reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining
 * a copy of this software and associated documentation files (the
 * "Software"), to deal in the Software without restriction, including
 * without limitation the rights to use, copy, modify, merge, publish,
 * distribute, sub license, and/or sell copies of the Software, and to
 * permit persons to whom the Software is furnished to do so, subject to
 * the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
 * OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NON-INFRINGEMENT. IN NO EVENT SHALL THE COPYRIGHT HOLDERS, AUTHORS
 * AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE
 * USE OR OTHER DEALINGS IN THE SOFTWARE.
 *
 * The above copyright notice and this permission notice (including the
 * next paragraph) shall be included in all copies or substantial portions
 * of the Software.
 */
/*
 * Authors:
 *      Dave Airlie
 *      Jérôme Glisse <glisse@freedesktop.org>
 */
#ifndef RADEON_BO_GEM_H
#define RADEON_BO_GEM_H

#include "radeon_bo.h"

struct radeon_bo_manager *radeon_bo_manager_gem_ctor(int fd);
void radeon_bo_manager_gem_dtor(struct radeon_bo_manager *bom);

uint32_t radeon_gem_name_bo(struct radeon_bo *bo);
void *radeon_gem_get_reloc_in_cs(struct radeon_bo *bo);
int radeon_gem_set_domain(struct radeon_bo *bo, uint32_t read_domains, uint32_t write_domain);
int radeon_gem_get_kernel_name(struct radeon_bo *bo, uint32_t *name);
int radeon_gem_prime_share_bo(struct radeon_bo *bo, int *handle);
struct radeon_bo *radeon_gem_bo_open_prime(struct radeon_bo_manager *bom,
					   int fd_handle,
					   uint32_t size);
#endif
