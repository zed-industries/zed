/*
 * This file is part of FFmpeg.
 *
 * FFmpeg is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * FFmpeg is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with FFmpeg; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
 */

#ifndef AVUTIL_AARCH64_CPU_H
#define AVUTIL_AARCH64_CPU_H

#include "libavutil/cpu.h"
#include "libavutil/cpu_internal.h"

#define have_armv8(flags) CPUEXT(flags, ARMV8)
#define have_neon(flags) CPUEXT(flags, NEON)
#define have_vfp(flags)  CPUEXT(flags, VFP)
#define have_dotprod(flags) CPUEXT(flags, DOTPROD)
#define have_i8mm(flags)    CPUEXT(flags, I8MM)
#define have_sve(flags)     CPUEXT(flags, SVE)
#define have_sve2(flags)    CPUEXT(flags, SVE2)

#if HAVE_SVE
int ff_aarch64_sve_length(void);
#endif

#endif /* AVUTIL_AARCH64_CPU_H */
