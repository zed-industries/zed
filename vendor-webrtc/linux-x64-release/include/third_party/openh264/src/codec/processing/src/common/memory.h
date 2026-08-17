/*!
 * \copy
 *     Copyright (c)  2011-2013, Cisco Systems
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
 * \file        :  memory.h
 *
 * \brief       :  memory definition for wels video processor class
 *
 * \date        :  2011/02/22
 *
 * \description :
 *
 *************************************************************************************
 */

#ifndef WELSVP_MEMORY_H
#define WELSVP_MEMORY_H

#include "util.h"
#include "typedef.h"

WELSVP_NAMESPACE_BEGIN

inline void* WelsMemset (void* pPointer, int32_t iValue, uint32_t uiSize) {
  return ::memset (pPointer, iValue, uiSize);
}

inline void* WelsMemcpy (void* pDst, const void* kpSrc, uint32_t uiSize) {
  return ::memcpy (pDst, kpSrc, uiSize);
}

inline int32_t WelsMemcmp (const void* kpBuf1, const void* kpBuf2, uint32_t uiSize) {
  return ::memcmp (kpBuf1, kpBuf2, uiSize);
}

/*!
*************************************************************************************
* \brief    malloc with zero filled utilization in Wels
*
* \param    i_size  uiSize of memory block required
*
* \return   allocated memory pointer exactly, failed in case of NULL return
*
* \note N/A
*************************************************************************************
*/
void* WelsMalloc (const uint32_t kuiSize, char* pTag = NULL);

/*!
*************************************************************************************
* \brief    free utilization in Wels
*
* \param    p   data pointer to be free.
*           i.e, uint8_t *p = actual data to be free, argv = &p.
*
* \return   NONE
*
* \note N/A
*************************************************************************************
*/
void WelsFree (void* pPointer, char* pTag = NULL);

/*!
*************************************************************************************
* \brief    reallocation in Wels. Do nothing and continue using old block
*       in case the block is large enough currently
*
* \param    p       memory block required in old time
* \param    i_size  new uiSize of memory block requested
* \param    sz_real pointer to the old uiSize of memory block
*
* \return   reallocated memory pointer exactly, failed in case of NULL return
*
* \note N/A
*************************************************************************************
*/
void* WelsRealloc (void*  pPointer, uint32_t* pRealSize, const uint32_t kuiSize, char* pTag = NULL);

//////////////////////////////////////////////////////////////////////////////////////
WELSVP_NAMESPACE_END

#endif


