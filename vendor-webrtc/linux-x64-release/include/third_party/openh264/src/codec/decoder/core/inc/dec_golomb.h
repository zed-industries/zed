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
 * \file    golomb.h
 *
 * \brief   Exponential Golomb entropy coding/decoding routine
 *
 * \date    03/13/2009 Created
 *
 *************************************************************************************
 */
#ifndef WELS_EXPONENTIAL_GOLOMB_ENTROPY_CODING_H__
#define WELS_EXPONENTIAL_GOLOMB_ENTROPY_CODING_H__

#include "typedefs.h"
#include "bit_stream.h"
#include "macros.h"
//#include <assert.h>
#include "ls_defines.h"
#include "error_code.h"

namespace WelsDec {

#define WELS_READ_VERIFY(uiRet) do{ \
  uint32_t uiRetTmp = (uint32_t)uiRet; \
  if( uiRetTmp != ERR_NONE ) \
    return uiRetTmp; \
}while(0)
#define GET_WORD(iCurBits, pBufPtr, iLeftBits, iAllowedBytes, iReadBytes) { \
  if (iReadBytes > iAllowedBytes+1) { \
    return ERR_INFO_READ_OVERFLOW; \
  } \
  iCurBits |= ((uint32_t)((pBufPtr[0] << 8) | pBufPtr[1])) << (iLeftBits); \
  iLeftBits -= 16; \
  pBufPtr +=2; \
}
#define NEED_BITS(iCurBits, pBufPtr, iLeftBits, iAllowedBytes, iReadBytes) { \
  if (iLeftBits > 0) { \
    GET_WORD(iCurBits, pBufPtr, iLeftBits, iAllowedBytes, iReadBytes); \
  } \
}
#define UBITS(iCurBits, iNumBits) (iCurBits>>(32-(iNumBits)))
#define DUMP_BITS(iCurBits, pBufPtr, iLeftBits, iNumBits, iAllowedBytes, iReadBytes) { \
  iCurBits <<= (iNumBits); \
  iLeftBits += (iNumBits); \
  NEED_BITS(iCurBits, pBufPtr, iLeftBits, iAllowedBytes, iReadBytes); \
}

static inline int32_t BsGetBits (PBitStringAux pBs, int32_t iNumBits, uint32_t* pCode) {
  intX_t iRc = UBITS (pBs->uiCurBits, iNumBits);
  intX_t iAllowedBytes = pBs->pEndBuf - pBs->pStartBuf; //actual stream bytes
  intX_t iReadBytes = pBs->pCurBuf - pBs->pStartBuf;
  DUMP_BITS (pBs->uiCurBits, pBs->pCurBuf, pBs->iLeftBits, iNumBits, iAllowedBytes, iReadBytes);
  *pCode = (uint32_t)iRc;
  return ERR_NONE;
}

/*
 *  Exponential Golomb codes decoding routines
 */

// for data sharing cross modules and try to reduce size of binary generated, 12/10/2009
extern const uint8_t g_kuiIntra4x4CbpTable[48];
extern const uint8_t g_kuiIntra4x4CbpTable400[16];
extern const uint8_t g_kuiInterCbpTable[48];
extern const uint8_t g_kuiInterCbpTable400[16];

extern const uint8_t g_kuiLeadingZeroTable[256];

static const uint32_t g_kuiPrefix8BitsTable[16] = {
  0, 0, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3
};


static inline uint32_t GetPrefixBits (uint32_t uiValue) {
  uint32_t iNumBit = 0;

  if (uiValue & 0xffff0000) {
    uiValue >>= 16;
    iNumBit += 16;
  }
  if (uiValue & 0xff00) {
    uiValue >>= 8;
    iNumBit += 8;
  }

  if (uiValue & 0xf0) {
    uiValue >>= 4;
    iNumBit += 4;
  }
  iNumBit += g_kuiPrefix8BitsTable[uiValue];

  return (32 - iNumBit);
}

/*
 *  Read one bit from bit stream followed
 */
static inline uint32_t BsGetOneBit (PBitStringAux pBs, uint32_t* pCode) {
  return (BsGetBits (pBs, 1, pCode));
}

static inline int32_t GetLeadingZeroBits (uint32_t iCurBits) { //<=32 bits
  uint32_t  uiValue;

  uiValue = UBITS (iCurBits, 8); //ShowBits( bs, 8 );
  if (uiValue) {
    return g_kuiLeadingZeroTable[uiValue];
  }

  uiValue = UBITS (iCurBits, 16); //ShowBits( bs, 16 );
  if (uiValue) {
    return (g_kuiLeadingZeroTable[uiValue] + 8);
  }

  uiValue = UBITS (iCurBits, 24); //ShowBits( bs, 24 );
  if (uiValue) {
    return (g_kuiLeadingZeroTable[uiValue] + 16);
  }

  uiValue = iCurBits; //ShowBits( bs, 32 );
  if (uiValue) {
    return (g_kuiLeadingZeroTable[uiValue] + 24);
  }
//ASSERT(false);  // should not go here
  return -1;
}

static inline uint32_t BsGetUe (PBitStringAux pBs, uint32_t* pCode) {
  uint32_t iValue = 0;
  int32_t  iLeadingZeroBits = GetLeadingZeroBits (pBs->uiCurBits);
  intX_t iAllowedBytes, iReadBytes;
  iAllowedBytes = pBs->pEndBuf - pBs->pStartBuf; //actual stream bytes

  if (iLeadingZeroBits == -1) { //bistream error
    return ERR_INFO_READ_LEADING_ZERO;//-1
  } else if (iLeadingZeroBits >
             16) { //rarely into this condition (even may be bitstream error), prevent from 16-bit reading overflow
    //using two-step reading instead of one time reading of >16 bits.
    iReadBytes = pBs->pCurBuf - pBs->pStartBuf;
    DUMP_BITS (pBs->uiCurBits, pBs->pCurBuf, pBs->iLeftBits, 16, iAllowedBytes, iReadBytes);
    iReadBytes = pBs->pCurBuf - pBs->pStartBuf;
    DUMP_BITS (pBs->uiCurBits, pBs->pCurBuf, pBs->iLeftBits, iLeadingZeroBits + 1 - 16, iAllowedBytes, iReadBytes);
  } else {
    iReadBytes = pBs->pCurBuf - pBs->pStartBuf;
    DUMP_BITS (pBs->uiCurBits, pBs->pCurBuf, pBs->iLeftBits, iLeadingZeroBits + 1, iAllowedBytes, iReadBytes);
  }
  if (iLeadingZeroBits) {
    iValue = UBITS (pBs->uiCurBits, iLeadingZeroBits);
    iReadBytes = pBs->pCurBuf - pBs->pStartBuf;
    DUMP_BITS (pBs->uiCurBits, pBs->pCurBuf, pBs->iLeftBits, iLeadingZeroBits, iAllowedBytes, iReadBytes);
  }

  *pCode = ((1u << iLeadingZeroBits) - 1 + iValue);
  return ERR_NONE;
}


/*
 *  Read signed exp golomb codes
 */
static inline int32_t BsGetSe (PBitStringAux pBs, int32_t* pCode) {
  uint32_t uiCodeNum;

  WELS_READ_VERIFY (BsGetUe (pBs, &uiCodeNum));

  if (uiCodeNum & 0x01) {
    *pCode = (int32_t) ((uiCodeNum + 1) >> 1);
  } else {
    *pCode = NEG_NUM ((int32_t) (uiCodeNum >> 1));
  }
  return ERR_NONE;
}

/*
 * Get unsigned truncated exp golomb code.
 */
static inline int32_t BsGetTe0 (PBitStringAux pBs, int32_t iRange, uint32_t* pCode) {
  if (iRange == 1) {
    *pCode = 0;
  } else if (iRange == 2) {
    WELS_READ_VERIFY (BsGetOneBit (pBs, pCode));
    *pCode ^= 1;
  } else {
    WELS_READ_VERIFY (BsGetUe (pBs, pCode));
  }
  return ERR_NONE;
}

/*
 *  Get number of trailing bits
 */
static inline int32_t BsGetTrailingBits (uint8_t* pBuf) {
// TODO
  uint32_t uiValue = *pBuf;
  int32_t iRetNum = 0;

  do {
    if (uiValue & 1)
      return iRetNum;
    uiValue >>= 1;
    ++ iRetNum;
  } while (iRetNum < 9);

  return 0;
}

/*
 *      Check whether there is more rbsp data for processing
 */
static inline bool CheckMoreRBSPData (PBitStringAux pBsAux) {
  if ((pBsAux->iBits - ((pBsAux->pCurBuf - pBsAux->pStartBuf - 2) << 3) - pBsAux->iLeftBits) > 1) {
    return true;
  } else {
    return false;
  }
}

//define macros to check syntax elements
#define WELS_CHECK_SE_BOTH_ERROR(val, lower_bound, upper_bound, syntax_name, ret_code) do {\
if ((val < lower_bound) || (val > upper_bound)) {\
  WelsLog(&(pCtx->sLogCtx), WELS_LOG_ERROR, "invalid syntax " syntax_name " %d", val);\
  return ret_code;\
}\
}while(0)

#define WELS_CHECK_SE_LOWER_ERROR(val, lower_bound, syntax_name, ret_code) do {\
if (val < lower_bound) {\
  WelsLog(&(pCtx->sLogCtx), WELS_LOG_ERROR, "invalid syntax " syntax_name " %d", val);\
  return ret_code;\
}\
}while(0)

#define WELS_CHECK_SE_UPPER_ERROR(val, upper_bound, syntax_name, ret_code) do {\
if (val > upper_bound) {\
  WelsLog(&(pCtx->sLogCtx), WELS_LOG_ERROR, "invalid syntax " syntax_name " %d", val);\
  return ret_code;\
}\
}while(0)

#define WELS_CHECK_SE_BOTH_ERROR_NOLOG(val, lower_bound, upper_bound, syntax_name, ret_code) do {\
if ((val < lower_bound) || (val > upper_bound)) {\
  return ret_code;\
}\
}while(0)

#define WELS_CHECK_SE_LOWER_ERROR_NOLOG(val, lower_bound, syntax_name, ret_code) do {\
if (val < lower_bound) {\
  return ret_code;\
}\
}while(0)

#define WELS_CHECK_SE_UPPER_ERROR_NOLOG(val, upper_bound, syntax_name, ret_code) do {\
if (val > upper_bound) {\
  return ret_code;\
}\
}while(0)


#define WELS_CHECK_SE_BOTH_WARNING(val, lower_bound, upper_bound, syntax_name) do {\
if ((val < lower_bound) || (val > upper_bound)) {\
  WelsLog(&(pCtx->sLogCtx), WELS_LOG_WARNING, "invalid syntax " syntax_name " %d", val);\
}\
}while(0)

#define WELS_CHECK_SE_LOWER_WARNING(val, lower_bound, syntax_name) do {\
if (val < lower_bound) {\
  WelsLog(&(pCtx->sLogCtx), WELS_LOG_WARNING, "invalid syntax " syntax_name " %d", val);\
}\
}while(0)

#define WELS_CHECK_SE_UPPER_WARNING(val, upper_bound, syntax_name) do {\
if (val > upper_bound) {\
  WelsLog(&(pCtx->sLogCtx), WELS_LOG_WARNING, "invalid syntax " syntax_name " %d", val);\
}\
}while(0)
// below define syntax element offset
// for bit_depth_luma_minus8 and bit_depth_chroma_minus8
#define BIT_DEPTH_LUMA_OFFSET 8
#define BIT_DEPTH_CHROMA_OFFSET 8
// for log2_max_frame_num_minus4
#define LOG2_MAX_FRAME_NUM_OFFSET 4
// for log2_max_pic_order_cnt_lsb_minus4
#define LOG2_MAX_PIC_ORDER_CNT_LSB_OFFSET 4
// for pic_width_in_mbs_minus1
#define PIC_WIDTH_IN_MBS_OFFSET 1
// for pic_height_in_map_units_minus1
#define PIC_HEIGHT_IN_MAP_UNITS_OFFSET 1
// for bit_depth_aux_minus8
#define BIT_DEPTH_AUX_OFFSET 8
// for num_slice_groups_minus1
#define NUM_SLICE_GROUPS_OFFSET 1
// for run_length_minus1
#define RUN_LENGTH_OFFSET 1
// for slice_group_change_rate_minus1
#define SLICE_GROUP_CHANGE_RATE_OFFSET 1
// for pic_size_in_map_units_minus1
#define PIC_SIZE_IN_MAP_UNITS_OFFSET 1
// for num_ref_idx_l0_default_active_minus1 and num_ref_idx_l1_default_active_minus1
#define NUM_REF_IDX_L0_DEFAULT_ACTIVE_OFFSET 1
#define NUM_REF_IDX_L1_DEFAULT_ACTIVE_OFFSET 1
// for pic_init_qp_minus26 and pic_init_qs_minus26
#define PIC_INIT_QP_OFFSET 26
#define PIC_INIT_QS_OFFSET 26
// for num_ref_idx_l0_active_minus1 and num_ref_idx_l1_active_minus1
#define NUM_REF_IDX_L0_ACTIVE_OFFSET 1
#define NUM_REF_IDX_L1_ACTIVE_OFFSET 1

// From Level 5.2
#define MAX_MB_SIZE 36864
// for aspect_ratio_idc
#define EXTENDED_SAR 255

} // namespace WelsDec

#endif//WELS_EXPONENTIAL_GOLOMB_ENTROPY_CODING_H__
