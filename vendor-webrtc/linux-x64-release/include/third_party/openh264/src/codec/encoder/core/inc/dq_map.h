/*!
 * \copy
 *     Copyright (c)  2009-2013, Cisco Systems
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
 * \file    dq_map.h
 *
 * \brief   Dependency Quality layer IDC mapping for cross layer selection and jumpping.
 *          DQ layer idc map for svc encoding, might be a better scheme than that of design before,
 *          can aware idc of referencing layer and that idc of successive layer to be coded
 *
 * \date    4/22/2009 Created
 *
 *************************************************************************************
 */
#if !defined(WELS_ENCODER_DEPENDENCY_QUAILITY_IDC_MAP_H__)
#define WELS_ENCODER_DEPENDENCY_QUAILITY_IDC_MAP_H__

namespace WelsEnc {
/*
 *  Dependency Quality IDC
 */

typedef struct TagDqIdc {
uint16_t    iPpsId;         // pPps id
uint8_t     iSpsId;         // pSps id
int8_t      uiSpatialId;    // spatial id
} SDqIdc;

}

#endif//WELS_ENCODER_DEPENDENCY_QUAILITY_IDC_MAP_H__
