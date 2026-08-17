/*!
 * \copy
 *     Copyright (c)  2013, Cisco Systems
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
 */

//macroblock.h
#ifndef WELS_MACROBLOCK_H__
#define WELS_MACROBLOCK_H__

#include "typedefs.h"
#include "wels_const.h"
#include "wels_common_basis.h"
#include "macros.h"

namespace WelsEnc {

//struct Mb_s;

/* MB syntax and context, refer to Page 399 in JVT X201wcm */
// keep the most essential level pData structure be 64 Bytes, which matches cache line size; if so, the order with structure maybe negligible.
// pls take care when modify MB structure size
typedef struct TagMB {
/*************************mb_layer() syntax and generated********************************/
/*mb_layer():*/
Mb_Type         uiMbType;       // including MB detailed partition type, number and type of reference list
uint8_t         uiSubMbType[4]; // sub MB types
int32_t         iMbXY;          // offset position of MB top left point based
int16_t         iMbX;           // position of MB in horizontal axis [0..32767]
int16_t         iMbY;           // position of MB in vertical axis [0..32767]

uint8_t         uiNeighborAvail;        // avail && same_slice: LEFT_MB_POS:0x01, TOP_MB_POS:0x02, TOPRIGHT_MB_POS = 0x04 ,TOPLEFT_MB_POS = 0x08;
uint8_t         uiCbp;

SMVUnitXY*      sMv;
int8_t*         pRefIndex;

int32_t*        pSadCost;          // mb sad. set to 0 for intra mb
int8_t*         pIntra4x4PredMode; // [MB_BLOCK4x4_NUM]
int8_t*         pNonZeroCount;     // [MB_LUMA_CHROMA_BLOCK4x4_NUM]

SMVUnitXY       sP16x16Mv;

uint8_t         uiLumaQp;       // uiLumaQp: pPps->iInitialQp + sSliceHeader->delta_qp + mb->dquant.
uint8_t         uiChromaQp;
uint16_t        uiSliceIdc;     // 2^16=65536 > MaxFS(36864) of level 5.1; AVC: iFirstMbInSlice?; SVC: (iFirstMbInSlice << 7) | ((uiDependencyId << 4) | uiQualityId);
uint32_t        uiChromPredMode;
int32_t         iLumaDQp;
SMVUnitXY       sMvd[MB_BLOCK4x4_NUM]; //only for CABAC writing; storage structure the same as sMv, in 4x4 scan order.
int32_t         iCbpDc;
//uint8_t         reserved_filling_bytes[1];      // not deleting this line for further changes of this structure. filling bytes reserved to make structure aligned with 4 bytes, higher cache hit on less structure size by 2 cache lines( 2 * 64 bytes) once hit
} SMB, *PMb;

}

#endif//WELS_MACROBLOCK_H__
