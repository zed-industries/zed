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
 * \file    nal_encap.h
 *
 * \brief   NAL pRawNal pData encapsulation
 *
 * \date    2/4/2009 Created
 *
 *************************************************************************************
 */
#ifndef WELS_NAL_UNIT_ENCAPSULATION_H__
#define WELS_NAL_UNIT_ENCAPSULATION_H__

#include "typedefs.h"
#include "wels_common_defs.h"
#include "wels_const.h"

using namespace WelsCommon;

//SBitStringAux
namespace WelsEnc {

#define NAL_HEADER_SIZE (4)
/*
 *  Raw payload pData for NAL unit, AVC/SVC compatible
 */
typedef struct TagWelsNalRaw {
uint8_t*                pRawData;       // pRawNal payload for slice pData
int32_t                 iPayloadSize;   // size of pRawNal pData

SNalUnitHeaderExt       sNalExt;        // NAL header information

int32_t iStartPos; //NAL start position in buffer
} SWelsNalRaw;

/*
 *  Encoder majoy output pData
 */
typedef struct TagWelsEncoderOutput {
uint8_t*        pBsBuffer;              // overall bitstream pBuffer allocation for a coded picture, recycling use intend.
uint32_t        uiSize;                 // size of allocation pBuffer above

SBitStringAux   sBsWrite;

// SWelsNalRaw             raw_nals[MAX_DEPENDENCY_LAYER*2+MAX_DEPENDENCY_LAYER*MAX_QUALITY_LEVEL]; // AVC: max up to SPS+PPS+max_slice_idc (2 + 8) for FMO;
SWelsNalRaw*    sNalList;               // nal list, adaptive for AVC/SVC in case single slice, multiple slices or fmo
int32_t*        pNalLen;
int32_t         iCountNals;             // count number of NAL in list
// SVC: num_sps (MAX_D) + num_pps (MAX_D) + num_vcl (MAX_D * MAX_Q)
int32_t         iNalIndex;              // coding NAL currently, 0 based
int32_t         iLayerBsIndex;          // layer index of  bit stream for SFrameBsIfo
// bool            bAnnexBFlag;            // annexeb flag, to figure it pOut the packetization mode whether need 4 bytes (0 0 0 1) of start code prefix
} SWelsEncoderOutput;

//#define MT_DEBUG_BS_WR        0       // for MT debugging if needed

typedef struct TagWelsSliceBs {
uint8_t*        pBs;                    // output bitstream, pBitStringAux not needed for slice 0 due to no dependency of pFrameBs available
uint32_t        uiBsPos;                // position of output bitstream
uint8_t*        pBsBuffer;              // overall bitstream pBuffer allocation for a coded slice, recycling use intend.
uint32_t        uiSize;                 // size of allocation pBuffer above

SBitStringAux   sBsWrite;

SWelsNalRaw     sNalList[2];            // nal list, PREFIX NAL(if applicable) + SLICE NAL
// int32_t         iCountNals;             // count number of NAL in list
int32_t         iNalLen[2];
int32_t         iNalIndex;              // coding NAL currently, 0 based

// bool            bAnnexBFlag;            // annexeb flag, to figure it pOut the packetization mode whether need 4 bytes (0 0 0 1) of start code prefix
#if MT_DEBUG_BS_WR
bool            bSliceCodedFlag;
#endif//MT_DEBUG_BS_WR
} SWelsSliceBs;

/*!
 * \brief   load an initialize NAL pRawNal pData
 */
void WelsLoadNal (SWelsEncoderOutput* pEncoderOuput, const int32_t/*EWelsNalUnitType*/ kiType,
                  const int32_t/*EWelsNalRefIdc*/ kiNalRefIdc);

/*!
 * \brief   unload pRawNal NAL
 */
void WelsUnloadNal (SWelsEncoderOutput* pEncoderOuput);

/*!
 * \brief   load an initialize NAL pRawNal pData
 */
void WelsLoadNalForSlice (SWelsSliceBs* pSliceBs, const int32_t/*EWelsNalUnitType*/ kiType,
                          const int32_t/*EWelsNalRefIdc*/ kiNalRefIdc);

/*!
 * \brief   unload pRawNal NAL
 */
void WelsUnloadNalForSlice (SWelsSliceBs* pSliceBs);

/*!
 * \brief   encode NAL with emulation forbidden three bytes checking
 * \param   pDst        pDst NAL pData
 * \param   pDstLen     length of pDst NAL output
 * \param   annexeb     annexeb flag
 * \param   pRawNal     pRawNal NAL pData
 * \return  ERR_CODE
 */
int32_t WelsEncodeNal (SWelsNalRaw* pRawNal, void* pNalHeaderExt, const int32_t kiDstBufferLen, void* pDst,
                       int32_t* pDstLen);

/*!
 * \brief   write prefix nal
 */
int32_t WelsWriteSVCPrefixNal (SBitStringAux* pBitStringAux, const int32_t keNalRefIdc, const bool kbIdrFlag);
}
#endif//WELS_NAL_UNIT_ENCAPSULATION_H__
