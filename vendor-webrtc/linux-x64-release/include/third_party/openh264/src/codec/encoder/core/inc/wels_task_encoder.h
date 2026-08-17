/*!
 * \copy
 *     Copyright (c)  2009-2015, Cisco Systems
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
 * \file    wels_task_encoder.h
 *
 * \brief   interface for encoder tasks
 *
 * \date    07/06/2015 Created
 *
 *************************************************************************************
 */

#ifndef _WELS_ENCODER_TASK_H_
#define _WELS_ENCODER_TASK_H_

#include "wels_task_base.h"
#include "encoder_context.h"

namespace WelsEnc {

extern int32_t WriteSliceToFrameBs (sWelsEncCtx* pCtx, SLayerBSInfo* pLbi, uint8_t* pFrameBsBuffer,
                                    const int32_t iSliceIdx,
                                    int32_t& iSliceSize);
extern int32_t WriteSliceBs (sWelsEncCtx* pCtx,SWelsSliceBs* pSliceBs,const int32_t iSliceIdx, int32_t& iSliceSize);

class CWelsSliceEncodingTask : public CWelsBaseTask {
 public:
  CWelsSliceEncodingTask (WelsCommon::IWelsTaskSink* pSink, sWelsEncCtx* pCtx, const int32_t iSliceIdx);
  virtual ~CWelsSliceEncodingTask();

  CWelsSliceEncodingTask* CreateSliceEncodingTask (sWelsEncCtx* pCtx, const int32_t iSliceIdx);
  WelsErrorType SetBoundary (int32_t iStartMbIdx, int32_t iEndMbIdx);

  virtual WelsErrorType Execute();
  virtual WelsErrorType InitTask();
  virtual WelsErrorType ExecuteTask();
  virtual void FinishTask();

  virtual uint32_t        GetTaskType() const {
    return WELS_ENC_TASK_ENCODE_FIXED_SLICE;
  }
 protected:
  WelsErrorType m_eTaskResult;

  int32_t QueryEmptyThread (bool* pThreadBsBufferUsage);

  sWelsEncCtx* m_pCtx;
  SSliceThreadPrivateData* m_pPrivateData;
  SLayerBSInfo* m_pLbi;
  int32_t m_iStartMbIdx;
  int32_t m_iEndMbIdx;

  EWelsNalUnitType m_eNalType;
  EWelsNalRefIdc m_eNalRefIdc;
  bool m_bNeedPrefix;
  uint32_t m_uiDependencyId;

  SSlice* m_pSlice;
  SWelsSliceBs* m_pSliceBs;
  int32_t m_iSliceIdx;
  int32_t m_iSliceSize;
  int32_t m_iThreadIdx;
};

class CWelsLoadBalancingSlicingEncodingTask : public CWelsSliceEncodingTask {
 public:
  CWelsLoadBalancingSlicingEncodingTask (WelsCommon::IWelsTaskSink* pSink, sWelsEncCtx* pCtx, const int32_t iSliceIdx) : CWelsSliceEncodingTask (pSink, pCtx,
        iSliceIdx) {
  };

  virtual WelsErrorType InitTask();
  virtual void FinishTask();

  virtual uint32_t        GetTaskType() const {
    return WELS_ENC_TASK_ENCODE_SLICE_LOADBALANCING;
  }
 private:
  int64_t m_iSliceStart;
};


class CWelsConstrainedSizeSlicingEncodingTask : public CWelsLoadBalancingSlicingEncodingTask {
 public:
  CWelsConstrainedSizeSlicingEncodingTask (WelsCommon::IWelsTaskSink* pSink, sWelsEncCtx* pCtx,
      const int32_t iSliceIdx) : CWelsLoadBalancingSlicingEncodingTask (pSink, pCtx, iSliceIdx) {
  };

  virtual WelsErrorType ExecuteTask();

  virtual uint32_t        GetTaskType() const {
    return WELS_ENC_TASK_ENCODE_SLICE_SIZECONSTRAINED;
  }

};


class CWelsUpdateMbMapTask : public CWelsBaseTask {
 public:
  CWelsUpdateMbMapTask (WelsCommon::IWelsTaskSink* pSink, sWelsEncCtx* pCtx, const int32_t iSliceIdx);
  virtual ~CWelsUpdateMbMapTask();

  virtual WelsErrorType Execute();

  virtual uint32_t        GetTaskType() const {
    return WELS_ENC_TASK_UPDATEMBMAP;
  }
 protected:
  sWelsEncCtx* m_pCtx;
  int32_t m_iSliceIdx;
};

}       //namespace
#endif  //header guard

