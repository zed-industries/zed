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
 * \file    WelsLock.h
 *
 * \brief   class wrapping for locks
 *
 * \date    5/09/2012 Created
 *
 *************************************************************************************
 */

#ifndef _WELS_LOCK_H_
#define _WELS_LOCK_H_

#include "macros.h"
#include "typedefs.h"
#include "WelsThreadLib.h"

namespace WelsCommon {

class CWelsLock {
  DISALLOW_COPY_AND_ASSIGN (CWelsLock);
 public:
  CWelsLock() {
    WelsMutexInit (&m_cMutex);
  }

  virtual ~CWelsLock() {
    WelsMutexDestroy (&m_cMutex);
  }

  WELS_THREAD_ERROR_CODE  Lock() {
    return WelsMutexLock (&m_cMutex);
  }

  WELS_THREAD_ERROR_CODE Unlock() {
    return WelsMutexUnlock (&m_cMutex);
  }

 private:
  WELS_MUTEX   m_cMutex;
};

class CWelsAutoLock {
  DISALLOW_COPY_AND_ASSIGN (CWelsAutoLock);
 public:
  CWelsAutoLock (CWelsLock& cLock) : m_cLock (cLock) {
    m_cLock.Lock();
  }

  virtual ~CWelsAutoLock() {
    m_cLock.Unlock();
  }

 private:
  CWelsLock&    m_cLock;
};

}

#endif







