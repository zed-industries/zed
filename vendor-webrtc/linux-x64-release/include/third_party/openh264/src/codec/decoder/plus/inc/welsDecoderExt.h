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
 *  WelsDecoderExt.h
 *
 *  Abstract
 *      Cisco OpenH264 decoder extension utilization interface
 *
 *  History
 *      3/12/2009 Created
 *
 *
 *************************************************************************/
#if !defined(WELS_PLUS_WELSDECODEREXT_H)
#define WELS_PLUS_WELSDECODEREXT_H

#include "codec_api.h"
#include "codec_app_def.h"
#include "decoder_context.h"
#include "welsCodecTrace.h"
#include "cpu.h"

class ISVCDecoder;

namespace WelsDec {

//#define OUTPUT_BIT_STREAM  ////for test to output bitstream

class CWelsDecoder : public ISVCDecoder {
 public:
  CWelsDecoder (void);
  virtual ~CWelsDecoder();

  virtual long EXTAPI Initialize (const SDecodingParam* pParam);
  virtual long EXTAPI Uninitialize();

  /***************************************************************************
  *   Description:
  *       Decompress one frame, and output I420 or RGB24(in the future) decoded stream and its length.
  *   Input parameters:
  *       Parameter       TYPE                   Description
  *       pSrc            unsigned char*         the h264 stream to decode
  *       srcLength       int                    the length of h264 steam
  *       pDst            unsigned char*         buffer pointer of decoded data
  *       pDstInfo        SBufferInfo&           information provided to API including width, height, SW/HW option, etc
  *
  *   return: if decode frame success return 0, otherwise corresponding error returned.
  ***************************************************************************/
  virtual DECODING_STATE EXTAPI DecodeFrame (const unsigned char* kpSrc,
      const int kiSrcLen,
      unsigned char** ppDst,
      int* pStride,
      int& iWidth,
      int& iHeight);

  virtual DECODING_STATE EXTAPI DecodeFrameNoDelay (const unsigned char* kpSrc,
      const int kiSrcLen,
      unsigned char** ppDst,
      SBufferInfo* pDstInfo);

  virtual DECODING_STATE EXTAPI DecodeFrame2 (const unsigned char* kpSrc,
      const int kiSrcLen,
      unsigned char** ppDst,
      SBufferInfo* pDstInfo);

  virtual DECODING_STATE EXTAPI FlushFrame (unsigned char** ppDst,
      SBufferInfo* pDstInfo);

  virtual DECODING_STATE EXTAPI DecodeParser (const unsigned char* kpSrc,
      const int kiSrcLen,
      SParserBsInfo* pDstInfo);
  virtual DECODING_STATE EXTAPI DecodeFrameEx (const unsigned char* kpSrc,
      const int kiSrcLen,
      unsigned char* pDst,
      int iDstStride,
      int& iDstLen,
      int& iWidth,
      int& iHeight,
      int& color_format);

  virtual long EXTAPI SetOption (DECODER_OPTION eOptID, void* pOption);
  virtual long EXTAPI GetOption (DECODER_OPTION eOptID, void* pOption);

 public:
  DECODING_STATE DecodeFrame2WithCtx (PWelsDecoderContext pCtx, const unsigned char* kpSrc, const int kiSrcLen,
                                      unsigned char** ppDst, SBufferInfo* pDstInfo);
  DECODING_STATE ParseAccessUnit (SWelsDecoderThreadCTX& sThreadCtx);

 private:
  welsCodecTrace*         m_pWelsTrace;
  uint32_t                m_uiDecodeTimeStamp;
  bool                    m_bIsBaseline;
  int32_t                 m_iCpuCount;
  int32_t                 m_iThreadCount;
  int32_t                 m_iCtxCount;
  PPicBuff                m_pPicBuff;
  bool                    m_bParamSetsLostFlag;
  bool                    m_bFreezeOutput;
  int32_t                 m_DecCtxActiveCount;
  PWelsDecoderThreadCTX   m_pDecThrCtx;
  PWelsDecoderThreadCTX   m_pLastDecThrCtx;
  int32_t                 m_iLastBufferedIdx;
  WELS_MUTEX              m_csDecoder;
  SWelsDecEvent           m_sBufferingEvent;
  SWelsDecEvent           m_sReleaseBufferEvent;
  SWelsDecSemphore        m_sIsBusy;
  SPictInfo               m_sPictInfoList[16];
  SPictReoderingStatus    m_sReoderingStatus;
  PWelsDecoderThreadCTX   m_pDecThrCtxActive[WELS_DEC_MAX_NUM_CPU];
  SVlcTable               m_sVlcTable;
  SWelsLastDecPicInfo     m_sLastDecPicInfo;
  SDecoderStatistics      m_sDecoderStatistics;// For real time debugging
  int32_t                 m_iStreamSeqNum;

 private:
  int32_t InitDecoder (const SDecodingParam* pParam);
  void UninitDecoder (void);
  int32_t InitDecoderCtx (PWelsDecoderContext& pCtx, const SDecodingParam* pParam);
  void UninitDecoderCtx (PWelsDecoderContext& pCtx);
  int32_t ResetDecoder (PWelsDecoderContext& pCtx);
  int32_t ThreadResetDecoder (PWelsDecoderContext& pCtx);

  void OutputStatisticsLog (SDecoderStatistics& sDecoderStatistics);
  DECODING_STATE ReorderPicturesInDisplay (PWelsDecoderContext pCtx, unsigned char** ppDst, SBufferInfo* pDstInfo);
  int ThreadDecodeFrameInternal (const unsigned char* kpSrc, const int kiSrcLen, unsigned char** ppDst,
                                 SBufferInfo* pDstInfo);
  void BufferingReadyPicture (PWelsDecoderContext pCtx, unsigned char** ppDst, SBufferInfo* pDstInfo);
  void ReleaseBufferedReadyPictureReorder (PWelsDecoderContext pCtx, unsigned char** ppDst, SBufferInfo* pDstInfo, bool isFlush = false);
  void ReleaseBufferedReadyPictureNoReorder (PWelsDecoderContext pCtx, unsigned char** ppDst, SBufferInfo* pDstInfo);

  void OpenDecoderThreads();
  void CloseDecoderThreads();
#ifdef OUTPUT_BIT_STREAM
  WelsFileHandle* m_pFBS;
  WelsFileHandle* m_pFBSSize;
#endif//OUTPUT_BIT_STREAM

};

} // namespace WelsDec

#endif // !defined(WELS_PLUS_WELSDECODEREXT_H)
