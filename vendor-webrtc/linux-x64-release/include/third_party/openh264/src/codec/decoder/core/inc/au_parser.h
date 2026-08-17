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
 * \file    au_parser.h
 *
 * \brief   Interfaces introduced in Access Unit level based parser
 *
 * \date    03/10/2009 Created
 *
 *************************************************************************************
 */
#ifndef WELS_ACCESS_UNIT_PARSER_H__
#define WELS_ACCESS_UNIT_PARSER_H__
#include "typedefs.h"
#include "wels_common_basis.h"
#include "nal_prefix.h"
#include "dec_frame.h"
#include "bit_stream.h"
#include "parameter_sets.h"
#include "decoder_context.h"

namespace WelsDec {

/*!
 *************************************************************************************
 * \brief   Start Code Prefix (0x 00 00 00 01) detection
 *
 * \param   pBuf        bitstream payload buffer
 * \param   pOffset     offset between NAL rbsp and original bitsteam that
 *                      start code prefix is seperated from.
 * \param   iBufSize    count size of buffer
 *
 * \return  RBSP buffer of start code prefix exclusive
 *
 * \note    N/A
 *************************************************************************************
 */
uint8_t* DetectStartCodePrefix (const uint8_t* kpBuf, int32_t* pOffset, int32_t iBufSize);

/*!
 *************************************************************************************
 * \brief   to parse network abstraction layer unit,
 *          escape emulation_prevention_three_byte within it
    former name is parse_nal
 *
 * \param   pCtx            decoder context
 * \param   pNalUnitHeader  parsed result of NAL Unit Header to output
 * \param   pSrcRbsp        bitstream buffer to input
 * \param   iSrcRbspLen     length size of bitstream buffer payload
 * \param   pSrcNal
 * \param   iSrcNalLen
 * \param   pConsumedBytes  consumed bytes during parsing
 *
 * \return  decoded bytes payload, might be (pSrcRbsp+1) if no escapes
 *
 * \note    N/A
 *************************************************************************************
 */
uint8_t* ParseNalHeader (PWelsDecoderContext pCtx, SNalUnitHeader* pNalUnitHeader, uint8_t* pSrcRbsp,
                         int32_t iSrcRbspLen, uint8_t* pSrcNal, int32_t iSrcNalLen, int32_t* pConsumedBytes);

int32_t ParseNonVclNal (PWelsDecoderContext pCtx, uint8_t* pRbsp, const int32_t kiSrcLen, uint8_t* pSrcNal,
                        const int32_t kSrcNalLen);

int32_t ParseRefBasePicMarking (PBitStringAux pBs, PRefBasePicMarking pRefBasePicMarking);

int32_t ParsePrefixNalUnit (PWelsDecoderContext pCtx, PBitStringAux pBs);

bool CheckAccessUnitBoundary (PWelsDecoderContext pCtx, const PNalUnit kpCurNal, const PNalUnit kpLastNal,
                              const PSps kpSps);
bool CheckAccessUnitBoundaryExt (PNalUnitHeaderExt pLastNalHdrExt, PNalUnitHeaderExt pCurNalHeaderExt,
                                 PSliceHeader pLastSliceHeader, PSliceHeader pCurSliceHeader);
bool CheckNextAuNewSeq (PWelsDecoderContext pCtx, const PNalUnit kpCurNal, const PSps kpSps);

/*!
 *************************************************************************************
 * \brief   to parse Sequence Parameter Set (SPS)
 *
 * \param   pCtx        Decoder context
 * \param   pBsAux      bitstream reader auxiliary
 * \param   pPicWidth   picture width current Sps represented
 * \param   pPicHeight  picture height current Sps represented
 *
 * \return  0 - successed
 *          1 - failed
 *
 * \note    Call it in case eNalUnitType is SPS.
 *************************************************************************************
 */
int32_t ParseSps (PWelsDecoderContext pCtx, PBitStringAux pBsAux, int32_t* pPicWidth, int32_t* pPicHeight,
                  uint8_t* pSrcNal, const int32_t kSrcNalLen);

/*!
 *************************************************************************************
 * \brief   to parse Picture Parameter Set (PPS)
 *
 * \param   pCtx        Decoder context
 * \param   pPpsList    pps list
 * \param   pBsAux      bitstream reader auxiliary
 *
 * \return  0 - successed
 *          1 - failed
 *
 * \note    Call it in case eNalUnitType is PPS.
 *************************************************************************************
 */
int32_t ParsePps (PWelsDecoderContext pCtx, PPps pPpsList, PBitStringAux pBsAux, uint8_t* pSrcNal,
                  const int32_t kSrcNalLen);

/*!
*************************************************************************************
* \brief   to parse Video Usability Information (VUI) parameter of the SPS
*
* \param   pCtx        Decoder context
* \param   pSps        the sps which current Vui parameter belongs to
* \param   pBsAux      bitstream reader auxiliary
*
* \return  0 - successed
*          1 - failed
*
* \note    Call it in case the flag "vui_parameters_present_flag" in sps is true.
*************************************************************************************
*/
int32_t ParseVui (PWelsDecoderContext pCtx, PSps pSps, PBitStringAux pBsAux);

/*!
 *************************************************************************************
 * \brief to parse scaling list message payload
 *
 * \param  PPS SPS scaling list matrix     message to be parsed output
 * \param pBsAux    bitstream reader auxiliary
 *
 * \return  0 - successed
 *    1 - failed
 *
 * \note  Call it in case scaling matrix present at sps or pps
 *************************************************************************************
*/
int32_t SetScalingListValue (uint8_t* pScalingList, int iScalingListNum, bool* bUseDefaultScalingMatrixFlag,
                             PBitStringAux pBsAux);
int32_t ParseScalingList (PSps pSps, PBitStringAux pBs, bool bPPS, const bool kbTrans8x8ModeFlag,
                          bool* bScalingListPresentFlag, uint8_t (*iScalingList4x4)[16], uint8_t (*iScalingList8x8)[64]);
/*!
 *************************************************************************************
 * \brief   to parse SEI message payload
 *
 * \param   pSei        sei message to be parsed output
 * \param   pBsAux      bitstream reader auxiliary
 *
 * \return  0 - successed
 *          1 - failed
 *
 * \note    Call it in case eNalUnitType is NAL_UNIT_SEI.
 *************************************************************************************
 */
int32_t ParseSei (void* pSei, PBitStringAux pBsAux); // reserved Sei_Msg type

/*!
 *************************************************************************************
 * \brief   reset fmo list due to got Sps now
 *
 * \param   pCtx    decoder context
 *
 * \return  count number of fmo context units are reset
 *************************************************************************************
 */
int32_t ResetFmoList (PWelsDecoderContext pCtx);

} // namespace WelsDec

#endif//WELS_ACCESS_UNIT_PARSER_H__

