/*!
 * \copy
 *     Copyright (c)  2008-2013, Cisco Systems
 *     All rights reserved.
 *
 *     Redistribution and use in source and binary forms, with or without
 *     modification, are permitted provided that the following conditions
 *     are met:
 *
 *        * Redistributions of source code must retain the above copyright
 *          notice, this list of conditions and the following disclaimer.
 *
 *        * Redistributions in binary form must reproduce the above copyright
 *          notice, this list of conditions and the following disclaimer in
 *          the documentation and/or other materials provided with the
 *          distribution.
 *
 *     THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 *     "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 *     LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
 *     FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
 *     COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT,
 *     INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING,
 *     BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 *     LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 *     CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 *     LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN
 *     ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 *     POSSIBILITY OF SUCH DAMAGE.
 *
 *
 *  memmgr_nal_unit.h
 *
 *  Abstract
 *      memory manager utils for NAL Unit list available
 *
 *  History
 *      07/10/2008 Created
 *
 *****************************************************************************/
#ifndef WELS_MEMORY_MANAGER_NAL_UNIT_H__
#define WELS_MEMORY_MANAGER_NAL_UNIT_H__

#include "typedefs.h"
#include "wels_common_basis.h"
#include "nalu.h"
#include "memory_align.h"

namespace WelsDec {

int32_t MemInitNalList (PAccessUnit* ppAu, const uint32_t kuiSize, CMemoryAlign* pMa);

int32_t MemFreeNalList (PAccessUnit* ppAu, CMemoryAlign* pMa);

/*
 *  MemGetNextNal
 *  Get next NAL Unit for using.
 *  Need expand NAL Unit list if exceeding count number of available NAL Units withing an Access Unit
 */
PNalUnit MemGetNextNal (PAccessUnit* ppAu, CMemoryAlign* pMa);

} // namespace WelsDec

#endif//WELS_MEMORY_MANAGER_NAL_UNIT_H__


