// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file contains defines and typedefs that allow popular Windows types to
// be used without the overhead of including windows.h.

#ifndef PARTITION_ALLOC_PARTITION_ALLOC_BASE_WIN_WINDOWS_TYPES_H_
#define PARTITION_ALLOC_PARTITION_ALLOC_BASE_WIN_WINDOWS_TYPES_H_

// Needed for function prototypes.
#include <specstrings.h>

#include <cstdint>

#ifdef __cplusplus
extern "C" {
#endif

// typedef and define the most commonly used Windows integer types.

typedef unsigned long DWORD;
typedef long LONG;
typedef int64_t LONGLONG;
typedef uint64_t ULONGLONG;

#define VOID void
typedef char CHAR;
typedef short SHORT;
typedef long LONG;
typedef int INT;
typedef unsigned int UINT;
typedef unsigned int* PUINT;
typedef uint64_t UINT64;
typedef void* LPVOID;
typedef void* PVOID;
typedef void* HANDLE;
typedef int BOOL;
typedef unsigned char BYTE;
typedef BYTE BOOLEAN;
typedef DWORD ULONG;
typedef unsigned short WORD;
typedef WORD UWORD;
typedef WORD ATOM;
#if defined(_WIN64)
typedef int64_t PA_LONG_PTR, *PA_PLONG_PTR;
#else
typedef int32_t PA_LONG_PTR, *PA_PLONG_PTR;
#endif

// Forward declare some Windows struct/typedef sets.

typedef struct _RTL_SRWLOCK RTL_SRWLOCK;
typedef RTL_SRWLOCK SRWLOCK, *PSRWLOCK;

typedef struct _FILETIME FILETIME;

struct PA_CHROME_SRWLOCK {
  PVOID Ptr;
};

// Define some commonly used Windows constants. Note that the layout of these
// macros - including internal spacing - must be 100% consistent with windows.h.

// clang-format off

#ifndef INVALID_HANDLE_VALUE
// Work around there being two slightly different definitions in the SDK.
#define INVALID_HANDLE_VALUE ((HANDLE)(PA_LONG_PTR)-1)
#endif

// The trailing white-spaces after this macro are required, for compatibility
// with the definition in winnt.h.
// clang-format off
#define RTL_SRWLOCK_INIT {0}                            // NOLINT
// clang-format on
#define SRWLOCK_INIT RTL_SRWLOCK_INIT

// clang-format on

// Define some macros needed when prototyping Windows functions.

#define DECLSPEC_IMPORT __declspec(dllimport)
#define WINBASEAPI DECLSPEC_IMPORT
#define WINAPI __stdcall

// Needed for LockImpl.
WINBASEAPI _Releases_exclusive_lock_(*SRWLock) VOID WINAPI
    ReleaseSRWLockExclusive(_Inout_ PSRWLOCK SRWLock);
WINBASEAPI BOOLEAN WINAPI TryAcquireSRWLockExclusive(_Inout_ PSRWLOCK SRWLock);

// Needed for thread_local_storage.h
WINBASEAPI LPVOID WINAPI TlsGetValue(_In_ DWORD dwTlsIndex);

WINBASEAPI BOOL WINAPI TlsSetValue(_In_ DWORD dwTlsIndex,
                                   _In_opt_ LPVOID lpTlsValue);

WINBASEAPI _Check_return_ _Post_equals_last_error_ DWORD WINAPI
    GetLastError(VOID);

WINBASEAPI VOID WINAPI SetLastError(_In_ DWORD dwErrCode);

#ifdef __cplusplus
}
#endif

#endif  // PARTITION_ALLOC_PARTITION_ALLOC_BASE_WIN_WINDOWS_TYPES_H_
