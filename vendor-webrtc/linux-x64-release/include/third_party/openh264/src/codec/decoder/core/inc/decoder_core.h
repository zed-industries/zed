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
 *  decoder_core.h
 *
 *  Abstract
 *      Encapsulative core interfaces
 *
 *  History
 *      07/10/2008 Created
 *
 *****************************************************************************/
#ifndef WELS_DECODER_CORE_H__
#define WELS_DECODER_CORE_H__

#include "typedefs.h"
#include "wels_common_basis.h"
#include "decoder_context.h"

#include "codec_def.h"

namespace WelsDec {
/*
 * InitBsBuffer
 * Memory allocation for Bitstream Buffer
 * return:
 *  0 - success; otherwise returned error_no defined in error_no.h.
 */
int32_t InitBsBuffer (PWelsDecoderContext pCtx);

/*
 * ExpandBsBuffer
 * Expand current BS buffer and copy its content
 * new buffer size will consider input size as a reference
 * return:
 *  0 - success; otherwise returned error_no defined in error_no.h.
 */
int32_t ExpandBsBuffer (PWelsDecoderContext pCtx, const int32_t kiSrcLen);

/*
 * ExpandBsLenBuffer
 * Expand current BS length buffer to double size or maximum, due to max slice number exceeding
 * Parameter:
 * kiCurrLen: current value of total nal number (including non-VCL nal)
 * return:
 *  0 - success; otherwise returned error_no defined in error_no.h.
 */
int32_t ExpandBsLenBuffer (PWelsDecoderContext pCtx, const int32_t kiCurrLen);

/*
 * CheckBsBuffer
 * Check if current buffer size is enough
 */
int32_t CheckBsBuffer (PWelsDecoderContext pCtx, const int32_t kiSrcLen);

/*
 * WelsInitStaticMemory
 * Memory request for introduced data at decoder start
 * Especially for:
 * rbsp_au_buffer, cur_dq_layer_ptr and ref_dq_layer_ptr in MB info cache.
 * return:
 *  0 - success; otherwise returned error_no defined in error_no.h.
 */
int32_t WelsInitStaticMemory (PWelsDecoderContext pCtx);

/*
 * WelsFreeStaticMemory
 * Free memory introduced in WelsInitStaticMemory at destruction of decoder.
 *
 */
void WelsFreeStaticMemory (PWelsDecoderContext pCtx);

/*!
 * \brief   request memory when maximal picture width and height are available
 */
int32_t InitialDqLayersContext (PWelsDecoderContext pCtx, const int32_t kiMaxWidth, const int32_t kiMaxHeight);

/*!
 * \brief   free dq layer context memory related
 */
void UninitialDqLayersContext (PWelsDecoderContext pCtx);

/*
 *  DecodeNalHeaderExt
 *  Trigger condition: NAL_UNIT_TYPE = NAL_UNIT_PREFIX or NAL_UNIT_CODED_SLICE_EXT
 *  Parameter:
 *  pNal:   target NALUnit ptr
 *  pSrc:   NAL Unit bitstream
 */
void DecodeNalHeaderExt (PNalUnit pNal, uint8_t* pSrc);

/*
 *  ParseSliceHeaderSyntaxs
 *  Parse slice header of bitstream
 */
int32_t ParseSliceHeaderSyntaxs (PWelsDecoderContext pCtx, PBitStringAux pBs, const bool kbExtensionFlag);
/*
 *  Copy relative syntax elements of NALUnitHeaderExt, sRefPicBaseMarking and bStoreRefBasePicFlag in prefix nal unit.
 *  pSrc:   mark as decoded prefix NAL
 *  pDst:   succeeded VCL NAL based AVC (I/P Slice)
 */
bool PrefetchNalHeaderExtSyntax (PWelsDecoderContext pCtx, PNalUnit const kpDst, PNalUnit const kpSrc);

/*
* WelsDecodeInitAccessUnitStart
* check and (re)allocate picture buffers on new sequence begin
*  bit_len:    size in bit length of data
*  buf_len:    size in byte length of data
*  coded_au:   mark an Access Unit decoding finished
* return:
*  0 - success; otherwise returned error_no defined in error_no.h
*/
int32_t WelsDecodeInitAccessUnitStart (PWelsDecoderContext pCtx, SBufferInfo* pDstInfo);
/*
* AllocPicBuffOnNewSeqBegin
* check and (re)allocate picture buffers on new sequence begin
* return:
*  0 - success; otherwise returned error_no defined in error_no.h
*/
int32_t AllocPicBuffOnNewSeqBegin (PWelsDecoderContext pCtx);

/*
* InitConstructAccessUnit
* Init before constructing an access unit for given input bitstream, maybe partial NAL Unit, one or more Units are involved to
* joint a collective access unit.
* parameter\
*  SBufferInfo:    Buffer info
* return:
*  0 - success; otherwise returned error_no defined in error_no.h
*/
int32_t InitConstructAccessUnit (PWelsDecoderContext pCtx, SBufferInfo* pDstInfo);

/*
 * ConstructAccessUnit
 * construct an access unit for given input bitstream, maybe partial NAL Unit, one or more Units are involved to
 * joint a collective access unit.
 * parameter\
 *  buf:        bitstream data buffer
 *  bit_len:    size in bit length of data
 *  buf_len:    size in byte length of data
 *  coded_au:   mark an Access Unit decoding finished
 * return:
 *  0 - success; otherwise returned error_no defined in error_no.h
 */
int32_t ConstructAccessUnit (PWelsDecoderContext pCtx, uint8_t** ppDst, SBufferInfo* pDstInfo);


/*
 * DecodeCurrentAccessUnit
 * Decode current access unit when current AU is completed.
 */
int32_t DecodeCurrentAccessUnit (PWelsDecoderContext pCtx, uint8_t** ppDst, SBufferInfo* pDstInfo);

/*
 * Check if frame is completed and EC is required
 */
bool CheckAndFinishLastPic (PWelsDecoderContext pCtx, uint8_t** pDst, SBufferInfo* pDstInfo);
/*
 *  Prepare current dq layer context initialization.
 */
void WelsDqLayerDecodeStart (PWelsDecoderContext pCtx, PNalUnit pCurNal, PSps pSps, PPps pPps);


int32_t WelsDecodeAccessUnitStart (PWelsDecoderContext pCtx);
void WelsDecodeAccessUnitEnd (PWelsDecoderContext pCtx);
void DecodeFinishUpdate (PWelsDecoderContext pCtx);

void ForceResetCurrentAccessUnit (PAccessUnit pAu);
void ForceClearCurrentNal (PAccessUnit pAu);

bool CheckRefPicturesComplete (PWelsDecoderContext pCtx); // Check whether all ref pictures are complete

void ForceResetParaSetStatusAndAUList (PWelsDecoderContext pCtx);
} // namespace WelsDec

#endif//WELS_DECODER_CORE_H__


