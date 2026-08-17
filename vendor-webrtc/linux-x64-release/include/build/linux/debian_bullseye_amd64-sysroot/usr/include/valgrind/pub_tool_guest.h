/* -*- mode: C; c-basic-offset: 3; -*- */

/*---------------------------------------------------------------*/
/*--- Provides guest state definition.       pub_tool_guest.h ---*/
/*---------------------------------------------------------------*/

/*
   This file is part of Valgrind, a dynamic binary instrumentation
   framework.

   Copyright (C) 2014-2017 OpenWorks LLP
      info@open-works.net

   This program is free software; you can redistribute it and/or
   modify it under the terms of the GNU General Public License as
   published by the Free Software Foundation; either version 2 of the
   License, or (at your option) any later version.

   This program is distributed in the hope that it will be useful, but
   WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program; if not, see <http://www.gnu.org/licenses/>.

   The GNU General Public License is contained in the file COPYING.

   Neither the names of the U.S. Department of Energy nor the
   University of California nor the names of its contributors may be
   used to endorse or promote products derived from this software
   without prior written permission.
*/

#ifndef __PUB_TOOL_GUEST_H
#define __PUB_TOOL_GUEST_H

#if defined(VGA_x86)
#  include "libvex_guest_x86.h"
   typedef VexGuestX86State   VexGuestArchState;
#elif defined(VGA_amd64)
#  include "libvex_guest_amd64.h"
   typedef VexGuestAMD64State VexGuestArchState;
#elif defined(VGA_ppc32)
#  include "libvex_guest_ppc32.h"
   typedef VexGuestPPC32State VexGuestArchState;
#elif defined(VGA_ppc64be) || defined(VGA_ppc64le)
#  include "libvex_guest_ppc64.h"
   typedef VexGuestPPC64State VexGuestArchState;
#elif defined(VGA_arm)
#  include "libvex_guest_arm.h"
   typedef VexGuestARMState   VexGuestArchState;
#elif defined(VGA_arm64)
#  include "libvex_guest_arm64.h"
   typedef VexGuestARM64State VexGuestArchState;
#elif defined(VGA_s390x)
#  include "libvex_guest_s390x.h"
   typedef VexGuestS390XState VexGuestArchState;
#elif defined(VGA_mips32) || defined(VGA_nanomips)
#  include "libvex_guest_mips32.h"
   typedef VexGuestMIPS32State VexGuestArchState;
#elif defined(VGA_mips64)
#  include "libvex_guest_mips64.h"
   typedef VexGuestMIPS64State VexGuestArchState;
#else
#  error Unknown arch
#endif

#endif   // __PUB_TOOL_GUEST_H

/*--------------------------------------------------------------------*/
/*--- end                                                          ---*/
/*--------------------------------------------------------------------*/
