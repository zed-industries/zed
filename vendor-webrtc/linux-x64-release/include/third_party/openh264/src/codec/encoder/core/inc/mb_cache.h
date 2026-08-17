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

//mb_cache.h
#ifndef WELS_MACROBLOCK_CACHE_H__
#define WELS_MACROBLOCK_CACHE_H__

#include "typedefs.h"
#include "wels_const.h"
#include "macros.h"

namespace WelsEnc {

/*
 *  MB Cache information, such one cache should be defined within a slice
 */
/*
 * Cache for Luma               Cache for Chroma(Cb, Cr)
 *
 *  TL T T T T                  TL T T
 *   L - - - -                   L - -
 *   L - - - -                   L - - TR
 *   L - - - -
 *   L - - - - TR
 *
 */

////////////////////////mapping scan index////////////////////////

extern const uint8_t g_kuiSmb4AddrIn256[16];
extern const uint8_t g_kuiCache12_8x8RefIdx[4];

typedef struct TagDCTCoeff {
//ALIGNED_DECLARE( int16_t, residual_ac[16], 16 ); //I_16x16
int16_t iLumaBlock[16][16]; //based on block4x4 luma DC/AC
//ALIGNED_DECLARE( int16_t, iLumaI16x16Dc[16], 16 ); //I_16x16 DC
int16_t iLumaI16x16Dc[16];
//ALIGNED_DECLARE( int16_t, iChromaDc[2][4], 16 ); //chroma DC
int16_t iChromaBlock[8][16]; //based on block4x4  chroma DC/AC
int16_t iChromaDc[2][4];
} SDCTCoeff ;

typedef struct TagMbCache {
//the followed pData now is promised aligned to 16 bytes
ALIGNED_DECLARE (SMVComponentUnit, sMvComponents, 16);

ALIGNED_DECLARE (int8_t, iNonZeroCoeffCount[48], 16);   // Cache line size
// int8_t iNonZeroCoeffCount[6 * 8];      // Right luma, Chroma(Left Top Cb, Left btm Cr); must follow by iIntraPredMode!
ALIGNED_DECLARE (int8_t, iIntraPredMode[48], 16);
// must follow with iNonZeroCoeffCount!

int32_t    iSadCost[4];                        //avail 1; unavail 0
SMVUnitXY  sMbMvp[MB_BLOCK4x4_NUM];// for write bs

//for residual decoding (recovery) at the side of Encoder
int16_t* pCoeffLevel;           // tmep
//malloc memory for prediction
uint8_t* pSkipMb;

//ALIGNED_DECLARE(uint8_t, pMemPredMb[2][256],  16);//One: Best I_16x16 Luma and refine frac_pixel pBuffer; another: PingPong I_8x8&&Inter Cb + Cr
uint8_t* pMemPredMb;
uint8_t* pMemPredLuma;// inter && intra share same pointer;
//ALIGNED_DECLARE(uint8_t, pMemPredChroma[2][64*2], 16); //another PingPong pBuffer: Best Cb + Cr;
uint8_t* pMemPredChroma;// inter && intra share same pointer;
uint8_t* pBestPredIntraChroma; //Cb:0~63;   Cr:64~127

//ALIGNED_DECLARE(uint8_t, pMemPredBlk4[2][16], 16); //I_4x4
uint8_t* pMemPredBlk4;

uint8_t* pBestPredI4x4Blk4;//I_4x4

//ALIGNED_DECLARE(uint8_t, pBufferInterPredMe[4][400], 16);//inter type pBuffer for ME h & v & hv
uint8_t* pBufferInterPredMe;    // [4][400] is enough because only h&v or v&hv or h&hv. but if both h&v&hv is needed when 8 quart pixel, future we have to use [5][400].

//no scan4[] order, just as memory order to store
//ALIGNED_DECLARE(bool, pPrevIntra4x4PredModeFlag[16], 16);//if 1, means no rem_intra4x4_pred_mode; if 0, means rem_intra4x4_pred_mode != 0
bool* pPrevIntra4x4PredModeFlag;
//ALIGNED_DECLARE(int8_t, pRemIntra4x4PredModeFlag[16], 16);//-1 as default; if pPrevIntra4x4PredModeFlag==0,
//pRemIntra4x4PredModeFlag or added by 1 is the best pred_mode
int8_t* pRemIntra4x4PredModeFlag;

int32_t     iSadCostSkip[4];      //avail 1; unavail 0
bool      bMbTypeSkip[4];         //1: skip; 0: non-skip
int32_t*     pEncSad;

//for residual encoding at the side of Encoder
SDCTCoeff* pDct;

uint8_t      uiNeighborIntra; // LEFT_MB_POS:0x01, TOP_MB_POS:0x02, TOPLEFT_MB_POS = 0x04 ,TOPRIGHT_MB_POS = 0x08;
uint8_t uiLumaI16x16Mode;
uint8_t uiChmaI8x8Mode;

bool         bCollocatedPredFlag;//denote if current MB is collocated predicted (MV==0).
uint32_t     uiRefMbType;

struct {
  /* pointer of current mb location in original frame */
  uint8_t* pEncMb[3];
  /* pointer of current mb location in recovery frame */
  uint8_t* pDecMb[3];
  /* pointer of co-located mb location in reference frame */
  uint8_t* pRefMb[3];
  //for SVC
  uint8_t*      pCsMb[3];//locating current mb's CS in whole frame
//              int16_t *p_rs[3];//locating current mb's RS     in whole frame

} SPicData;
} SMbCache;

}//end of namespace

#endif//WELS_MACROBLOCK_CACHE_H__
