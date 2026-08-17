
/*--------------------------------------------------------------------*/
/*--- asm-only vkiscnums stuff.           pub_tool_vkiscnums_asm.h ---*/
/*--------------------------------------------------------------------*/

/*
   This file is part of Valgrind, a dynamic binary instrumentation
   framework.

   Copyright (C) 2005-2017 Nicholas Nethercote
      njn@valgrind.org
   Copyright (C) 2006-2017 OpenWorks LLP
      info@open-works.co.uk

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
*/

#ifndef __PUB_TOOL_VKISCNUMS_ASM_H
#define __PUB_TOOL_VKISCNUMS_ASM_H

#if defined(VGP_x86_linux)
#  include "vki/vki-scnums-shared-linux.h"
#  include "vki/vki-scnums-32bit-linux.h"
#  include "vki/vki-scnums-x86-linux.h"

#elif defined(VGP_amd64_linux)
#  include "vki/vki-scnums-shared-linux.h"
#  include "vki/vki-scnums-amd64-linux.h"

#elif defined(VGP_ppc32_linux)
#  include "vki/vki-scnums-shared-linux.h"
#  include "vki/vki-scnums-32bit-linux.h"
#  include "vki/vki-scnums-ppc32-linux.h"

#elif defined(VGP_ppc64be_linux) || defined(VGP_ppc64le_linux)
#  include "vki/vki-scnums-shared-linux.h"
#  include "vki/vki-scnums-ppc64-linux.h"

#elif defined(VGP_s390x_linux)
#  include "vki/vki-scnums-shared-linux.h"
#  include "vki/vki-scnums-s390x-linux.h"

#elif defined(VGP_arm_linux)
#  include "vki/vki-scnums-shared-linux.h"
#  include "vki/vki-scnums-32bit-linux.h"
#  include "vki/vki-scnums-arm-linux.h"

#elif defined(VGP_arm64_linux)
#  include "vki/vki-scnums-shared-linux.h"
#  include "vki/vki-scnums-arm64-linux.h"

#elif defined(VGP_mips32_linux)
#  include "vki/vki-scnums-shared-linux.h"
#  include "vki/vki-scnums-32bit-linux.h"
#  include "vki/vki-scnums-mips32-linux.h"

#elif defined(VGP_nanomips_linux)
#  include "vki/vki-scnums-nanomips-linux.h"

#elif defined(VGP_mips64_linux)
#  include "vki/vki-scnums-shared-linux.h"
#  include "vki/vki-scnums-mips64-linux.h"

#elif defined(VGP_x86_darwin) || defined(VGP_amd64_darwin)
#  include "vki/vki-scnums-darwin.h"

#elif defined(VGP_x86_solaris) || (VGP_amd64_solaris)
#  include "vki/vki-scnums-solaris.h"

#else
#  error Unknown platform
#endif

#endif   // __PUB_TOOL_VKISCNUMS_ASM_H

/*--------------------------------------------------------------------*/
/*--- end                                                          ---*/
/*--------------------------------------------------------------------*/
