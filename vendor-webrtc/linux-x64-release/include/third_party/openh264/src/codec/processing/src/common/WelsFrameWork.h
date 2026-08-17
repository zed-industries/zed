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
 * \file        :  WelsFrameWork.h
 *
 * \brief       :  framework of wels video processor class
 *
 * \date        :  2011/01/04
 *
 * \description :
 *
 *************************************************************************************
 */

#ifndef WELSVP_WELSFRAMEWORK_H
#define WELSVP_WELSFRAMEWORK_H

#include "IWelsVP.h"
#include "util.h"
#include "WelsThreadLib.h"

WELSVP_NAMESPACE_BEGIN

EResult CreateSpecificVpInterface (IWelsVP** ppCtx);
EResult DestroySpecificVpInterface (IWelsVP* pCtx);

EResult CreateSpecificVpInterface (IWelsVPc** ppCtx);
EResult DestroySpecificVpInterface (IWelsVPc* pCtx);

#define MAX_STRATEGY_NUM (METHOD_MASK - 1)

class IStrategy : public IWelsVP {
 public:
  IStrategy() {
    m_eMethod  = METHOD_NULL;
    m_eFormat  = VIDEO_FORMAT_I420;
    m_iIndex   = 0;
    m_bInit    = false;
  }

  virtual ~IStrategy() {}

 public:
  virtual EResult Init (int32_t iType, void* pCfg)  {
    return RET_SUCCESS;
  }
  virtual EResult Uninit (int32_t iType)              {
    return RET_SUCCESS;
  }
  virtual EResult Flush (int32_t iType)               {
    return RET_SUCCESS;
  }
  virtual EResult Get (int32_t iType, void* pParam) {
    return RET_SUCCESS;
  }
  virtual EResult Set (int32_t iType, void* pParam) {
    return RET_SUCCESS;
  }
  virtual EResult SpecialFeature (int32_t iType, void* pIn, void* pOut) {
    return RET_SUCCESS;
  }
  virtual EResult Process (int32_t iType, SPixMap* pSrc, SPixMap* pDst) = 0;

 public:
  EMethods       m_eMethod;
  EVideoFormat m_eFormat;
  int32_t           m_iIndex;
  bool            m_bInit;
};

class CVpFrameWork : public IWelsVP {
 public:
  CVpFrameWork (uint32_t uiThreadsNum, EResult& ret);
  ~CVpFrameWork();

 public:
  EResult Init (int32_t iType, void* pCfg);

  EResult Uninit (int32_t iType);

  EResult Flush (int32_t iType);

  EResult Process (int32_t iType, SPixMap* pSrc, SPixMap* pDst);

  EResult Get (int32_t iType, void* pParam);

  EResult Set (int32_t iType, void* pParam);

  EResult SpecialFeature (int32_t iType, void* pIn, void* pOut);

 private:
  bool  CheckValid (EMethods eMethod, SPixMap& sSrc, SPixMap& sDst);
  IStrategy* CreateStrategy (EMethods eMethod, int32_t iCpuFlag);

 private:
  IStrategy* m_pStgChain[MAX_STRATEGY_NUM];

  WELS_MUTEX m_mutes;
};

WELSVP_NAMESPACE_END

#endif
