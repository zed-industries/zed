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
 * \file    WelsTaskThread.h
 *
 * \brief   connecting task and thread
 *
 * \date    5/09/2012 Created
 *
 *************************************************************************************
 */


#ifndef _WELS_TASK_THREAD_H_
#define _WELS_TASK_THREAD_H_


#include "WelsTask.h"
#include "WelsThread.h"

namespace WelsCommon {

class CWelsTaskThread;

class IWelsTaskThreadSink {
 public:
  virtual WELS_THREAD_ERROR_CODE OnTaskStart (CWelsTaskThread* pThread, IWelsTask* pTask) = 0;
  virtual WELS_THREAD_ERROR_CODE OnTaskStop (CWelsTaskThread* pThread, IWelsTask* pTask) = 0;
};

class CWelsTaskThread : public CWelsThread {
 public:
  CWelsTaskThread (IWelsTaskThreadSink* pSink);
  virtual ~CWelsTaskThread();

  WELS_THREAD_ERROR_CODE  SetTask (IWelsTask* pTask);
  virtual void ExecuteTask();

  uintptr_t    GetID() const {
    return m_uiID;
  }

 private:
  CWelsLock   m_cLockTask;
  IWelsTaskThreadSink*   m_pSink;
  IWelsTask*    m_pTask;
  uintptr_t    m_uiID;

  DISALLOW_COPY_AND_ASSIGN (CWelsTaskThread);
};

}

#endif

