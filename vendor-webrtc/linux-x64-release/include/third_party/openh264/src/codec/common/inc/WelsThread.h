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
 * \file    WelsThread.h
 *
 * \brief   Interfaces introduced in threads
 *
 * \date    5/09/2012 Created
 *
 *************************************************************************************
 */


#ifndef _WELS_THREAD_H_
#define _WELS_THREAD_H_


#include "macros.h"
#include "WelsLock.h"
#include "WelsThreadLib.h"

namespace WelsCommon {

class CWelsThread {
 public:
  CWelsThread();
  virtual ~CWelsThread();

  virtual void Thread();
  virtual void ExecuteTask() = 0;
  virtual WELS_THREAD_ERROR_CODE Start();
  virtual void Kill();
  WELS_MUTEX          m_hMutex;
 protected:
  static WELS_THREAD_ROUTINE_TYPE  TheThread (void* pParam);

  void SetRunning (bool bRunning) {
    CWelsAutoLock  cLock (m_cLockStatus);

    m_bRunning = bRunning;
  }
  void SetEndFlag (bool bEndFlag) {
    CWelsAutoLock  cLock (m_cLockStatus);

    m_bEndFlag = bEndFlag;
  }

  bool GetRunning() const {
    return m_bRunning;
  }

  bool GetEndFlag() const {
    return m_bEndFlag;
  }

  void SignalThread() {
    WelsEventSignal (&m_hEvent, &m_hMutex, &m_iConVar);
  }

 private:
  WELS_THREAD_HANDLE  m_hThread;
  WELS_EVENT          m_hEvent;
  CWelsLock           m_cLockStatus;
  bool                m_bRunning;
  bool                m_bEndFlag;
  int                 m_iConVar;

  DISALLOW_COPY_AND_ASSIGN (CWelsThread);
};


}



#endif


