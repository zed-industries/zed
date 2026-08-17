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
 * \file    WelsThreadLib.h
 *
 * \brief   Interfaces introduced in thread programming
 *
 * \date    11/17/2009 Created
 *
 *************************************************************************************
 */

#ifndef   _WELS_THREAD_API_H_
#define   _WELS_THREAD_API_H_

#include "typedefs.h"

#ifdef  __cplusplus
extern "C" {
#endif

#if defined(_WIN32) || defined(__CYGWIN__)

#include <windows.h>

typedef    HANDLE                    WELS_THREAD_HANDLE;
typedef    LPTHREAD_START_ROUTINE    LPWELS_THREAD_ROUTINE;

typedef    CRITICAL_SECTION          WELS_MUTEX;
typedef    HANDLE                    WELS_EVENT;

#define    WELS_THREAD_ROUTINE_TYPE         DWORD  WINAPI
#define    WELS_THREAD_ROUTINE_RETURN(rc)   return (DWORD)rc;

#ifdef WINAPI_FAMILY
#if !WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#define WP80

#define InitializeCriticalSection(x) InitializeCriticalSectionEx(x, 0, 0)
#define GetSystemInfo(x) GetNativeSystemInfo(x)
#define CreateEvent(attr, reset, init, name) CreateEventEx(attr, name, ((reset) ? CREATE_EVENT_MANUAL_RESET : 0) | ((init) ? CREATE_EVENT_INITIAL_SET : 0), EVENT_ALL_ACCESS)
#define CreateSemaphore(a, b, c, d) CreateSemaphoreEx(a, b, c, d, 0, SEMAPHORE_ALL_ACCESS)
#define WaitForSingleObject(a, b) WaitForSingleObjectEx(a, b, FALSE)
#define WaitForMultipleObjects(a, b, c, d) WaitForMultipleObjectsEx(a, b, c, d, FALSE)
#endif
#endif

#else // NON-WINDOWS

#include <stdlib.h>
#include <unistd.h>
#include <string.h>
#include <pthread.h>
#include <semaphore.h>
#include <signal.h>
#include <errno.h>
#include <time.h>
#include <sys/time.h>

#include <sys/stat.h>
#include <fcntl.h>

typedef   pthread_t    WELS_THREAD_HANDLE;
typedef  void* (*LPWELS_THREAD_ROUTINE) (void*);

typedef   pthread_mutex_t           WELS_MUTEX;

#ifdef __APPLE__
typedef   pthread_cond_t            WELS_EVENT;
#else
typedef   sem_t*                    WELS_EVENT;
#endif

#define   WELS_THREAD_ROUTINE_TYPE         void *
#define   WELS_THREAD_ROUTINE_RETURN(rc)   return (void*)(intptr_t)rc;

#endif//_WIN32

typedef    int32_t        WELS_THREAD_ERROR_CODE;
typedef    int32_t        WELS_THREAD_ATTR;

typedef  struct _WelsLogicalProcessorInfo {
  int32_t    ProcessorCount;
} WelsLogicalProcessInfo;

#define    WELS_THREAD_ERROR_OK                                 0
#define    WELS_THREAD_ERROR_GENERAL                    ((uint32_t)(-1))
#define    WELS_THREAD_ERROR_WAIT_OBJECT_0              0
#define    WELS_THREAD_ERROR_WAIT_TIMEOUT               ((uint32_t)0x00000102L)
#define    WELS_THREAD_ERROR_WAIT_FAILED                WELS_THREAD_ERROR_GENERAL

WELS_THREAD_ERROR_CODE    WelsMutexInit (WELS_MUTEX*    mutex);
WELS_THREAD_ERROR_CODE    WelsMutexLock (WELS_MUTEX*    mutex);
WELS_THREAD_ERROR_CODE    WelsMutexUnlock (WELS_MUTEX* mutex);
WELS_THREAD_ERROR_CODE    WelsMutexDestroy (WELS_MUTEX* mutex);

WELS_THREAD_ERROR_CODE    WelsEventOpen (WELS_EVENT* p_event, const char* event_name = NULL);
WELS_THREAD_ERROR_CODE    WelsEventClose (WELS_EVENT* event, const char* event_name = NULL);

WELS_THREAD_ERROR_CODE    WelsEventSignal (WELS_EVENT* event,WELS_MUTEX *pMutex, int* iCondition);
WELS_THREAD_ERROR_CODE    WelsEventWait (WELS_EVENT* event,WELS_MUTEX *pMutex, int& iCondition);
WELS_THREAD_ERROR_CODE    WelsEventWaitWithTimeOut (WELS_EVENT* event, uint32_t dwMilliseconds,WELS_MUTEX *pMutex = NULL);
WELS_THREAD_ERROR_CODE    WelsMultipleEventsWaitSingleBlocking (uint32_t nCount, WELS_EVENT* event_list,
    WELS_EVENT* master_event = NULL,WELS_MUTEX *pMutex = NULL);

WELS_THREAD_ERROR_CODE    WelsThreadCreate (WELS_THREAD_HANDLE* thread,  LPWELS_THREAD_ROUTINE  routine,
    void* arg, WELS_THREAD_ATTR attr);

WELS_THREAD_ERROR_CODE    WelsThreadSetName (const char* thread_name);

WELS_THREAD_ERROR_CODE    WelsThreadJoin (WELS_THREAD_HANDLE  thread);

WELS_THREAD_HANDLE        WelsThreadSelf();

WELS_THREAD_ERROR_CODE    WelsQueryLogicalProcessInfo (WelsLogicalProcessInfo* pInfo);

void WelsSleep (uint32_t dwMilliSecond);

#ifdef  __cplusplus
}
#endif

#endif
