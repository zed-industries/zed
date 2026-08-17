/*!
 * \copy
 *     Copyright (c)  2009-2019, Cisco Systems
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
 * \file    wels_decoder_thread.h
 *
 * \brief   Interfaces introduced in thread programming
 *
 * \date    08/06/2018 Created
 *
 *************************************************************************************
 */

#ifndef   _WELS_DECODER_THREAD_H_
#define   _WELS_DECODER_THREAD_H_

#include "WelsThreadLib.h"

#ifdef  __cplusplus
extern "C" {
#endif

#define WELS_DEC_MAX_NUM_CPU 16
#define WELS_DEC_MAX_THREAD_STACK_SIZE   4096
#define WELS_DEC_THREAD_COMMAND_RUN 0
#define WELS_DEC_THREAD_COMMAND_ABORT 1

#if defined(_WIN32) || defined(__CYGWIN__)
typedef struct tagWelsDecSemphore {
  WELS_THREAD_HANDLE h;
} SWelsDecSemphore;

typedef struct tagWelsDecEvent {
  WELS_THREAD_HANDLE h;
  int isSignaled;
} SWelsDecEvent;

typedef struct tagWelsDecThread {
  WELS_THREAD_HANDLE h;
} SWelsDecThread;

#define WelsDecThreadFunc(fn,a) DWORD WINAPI fn(LPVOID a)
#define WelsDecThreadFuncArg(a) LPWELS_THREAD_ROUTINE a
#define WELS_DEC_THREAD_WAIT_TIMEDOUT    WAIT_TIMEOUT
#define WELS_DEC_THREAD_WAIT_SIGNALED    WAIT_OBJECT_0
#define WELS_DEC_THREAD_WAIT_INFINITE    INFINITE

#else // NON-WINDOWS

typedef   pthread_mutexattr_t       WELS_MUTEX_ATTR;

typedef struct tagWelsDecSemphore {
  long max;
  long v;
  WELS_EVENT  e;
  WELS_MUTEX  m;
} SWelsDecSemphore;

typedef struct tagWelsDecEvent {
  int manualReset;
  int isSignaled;
  pthread_cond_t c;
  WELS_MUTEX m;
} SWelsDecEvent;

typedef struct tagWelsDecThread {
  WELS_THREAD_HANDLE h;
} SWelsDecThread;

#define WelsDecThreadFunc(fn,a) void* fn(void* a)
#define WelsDecThreadFuncArg(a) void* (*a)(void*)

#define WELS_DEC_THREAD_WAIT_TIMEDOUT    ETIMEDOUT
#define WELS_DEC_THREAD_WAIT_SIGNALED    EINTR
#define WELS_DEC_THREAD_WAIT_INFINITE    -1

#endif//_WIN32

#define WelsDecThreadReturn   WELS_THREAD_ROUTINE_RETURN(0);

int32_t GetCPUCount();

// Event
int EventCreate (SWelsDecEvent* e, int manualReset, int initialState);
void EventPost (SWelsDecEvent* e);
int EventWait (SWelsDecEvent* e, int32_t timeout);
void EventReset (SWelsDecEvent* e);
void EventDestroy (SWelsDecEvent* e);

// Semaphore
int SemCreate (SWelsDecSemphore* s, long value, long max);
int SemWait (SWelsDecSemphore* s, int32_t timeout);
void SemRelease (SWelsDecSemphore* s, long* prev_count);
void SemDestroy (SWelsDecSemphore* s);

// Thread
int ThreadCreate (SWelsDecThread* t, LPWELS_THREAD_ROUTINE tf, void* ta);
int ThreadWait (SWelsDecThread* t);

#define DECLARE_PROCTHREAD(name, argument) \
  WelsDecThreadFunc(name,argument)

#define DECLARE_PROCTHREAD_PTR(name) \
  LPWELS_THREAD_ROUTINE name

#define CREATE_THREAD(ph, threadproc,argument) \
  ThreadCreate(ph, threadproc, (void*)argument)

#define CREATE_EVENT(ph, manualreset,initial_state,name) \
  EventCreate(ph,(int)(manualreset),(int)(initial_state))

#define CREATE_SEMAPHORE(ph, initial_count,max_count, name) \
  SemCreate(ph, (long)initial_count,(long)(max_count))

#define CLOSE_EVENT(ph) \
  EventDestroy(ph)

#define CLOSE_SEMAPHORE(ph) \
  SemDestroy(ph)

#define SET_EVENT(ph) \
  EventPost(ph)

#define RESET_EVENT(ph) \
  EventReset(ph)

#define RELEASE_SEMAPHORE(ph) \
  SemRelease(ph,NULL)

#define WAIT_EVENT(ph,timeout) \
  EventWait(ph, (int32_t)timeout)

#define WAIT_THREAD(ph) \
  ThreadWait(ph)

#define WAIT_SEMAPHORE(ph,timeout) \
  SemWait(ph,(int32_t)timeout)

#ifdef  __cplusplus
}
#endif

#endif
