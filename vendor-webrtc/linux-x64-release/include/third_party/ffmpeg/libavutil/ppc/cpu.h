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

#ifndef AVUTIL_PPC_CPU_H
#define AVUTIL_PPC_CPU_H

#include "libavutil/cpu.h"
#include "libavutil/cpu_internal.h"

#define PPC_ALTIVEC(flags) CPUEXT(flags, ALTIVEC)
#define PPC_VSX(flags) CPUEXT(flags, VSX)
#define PPC_POWER8(flags) CPUEXT(flags, POWER8)

#endif /* AVUTIL_PPC_CPU_H */
