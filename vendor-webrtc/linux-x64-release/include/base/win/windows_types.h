// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file contains defines and typedefs that allow popular Windows types to
// be used without the overhead of including windows.h.

#ifndef BASE_WIN_WINDOWS_TYPES_H_
#define BASE_WIN_WINDOWS_TYPES_H_

// Needed for function prototypes.
#include <sal.h>
#include <specstrings.h>

#include "base/win/win_handle_types.h"

#ifdef __cplusplus
extern "C" {
#endif

// typedef and define the most commonly used Windows integer types.

typedef unsigned long DWORD;  // NOLINT(runtime/int)
typedef long LONG;            // NOLINT(runtime/int)
typedef __int64 LONGLONG;
typedef unsigned __int64 ULONGLONG;

#define VOID void
typedef char CHAR;
typedef short SHORT;  // NOLINT(runtime/int)
typedef long LONG;    // NOLINT(runtime/int)
typedef int INT;
typedef unsigned int UINT;
typedef unsigned int* PUINT;
typedef unsigned __int64 UINT64;
typedef void* LPVOID;
typedef void* PVOID;
typedef void* HANDLE;
typedef int BOOL;
typedef unsigned char BYTE;
typedef BYTE BOOLEAN;
typedef DWORD ULONG;
typedef unsigned short WORD;  // NOLINT(runtime/int)
typedef WORD UWORD;
typedef WORD ATOM;

#if defined(_WIN64)
typedef __int64 INT_PTR, *PINT_PTR;
typedef unsigned __int64 UINT_PTR, *PUINT_PTR;

typedef __int64 LONG_PTR, *PLONG_PTR;
typedef unsigned __int64 ULONG_PTR, *PULONG_PTR;
#else
typedef __w64 int INT_PTR, *PINT_PTR;
typedef __w64 unsigned int UINT_PTR, *PUINT_PTR;

typedef __w64 long LONG_PTR, *PLONG_PTR;             // NOLINT(runtime/int)
typedef __w64 unsigned long ULONG_PTR, *PULONG_PTR;  // NOLINT(runtime/int)
#endif

typedef UINT_PTR WPARAM;
typedef LONG_PTR LPARAM;
typedef LONG_PTR LRESULT;
#define LRESULT LONG_PTR
typedef _Return_type_success_(return >= 0) long HRESULT;  // NOLINT(runtime/int)

typedef ULONG_PTR SIZE_T, *PSIZE_T;
typedef LONG_PTR SSIZE_T, *PSSIZE_T;

typedef DWORD ACCESS_MASK;
typedef ACCESS_MASK REGSAM;

typedef LONG NTSTATUS;

// As defined in guiddef.h.
#ifndef _REFGUID_DEFINED
#define _REFGUID_DEFINED
#define REFGUID const GUID&
#endif

// As defined in ncrypt.h.
#ifndef __SECSTATUS_DEFINED__
typedef LONG SECURITY_STATUS;
#define __SECSTATUS_DEFINED__
#endif

typedef LPVOID HINTERNET;
typedef HICON HCURSOR;
typedef HINSTANCE HMODULE;
typedef PVOID LSA_HANDLE;
typedef PVOID HDEVINFO;

// Forward declare some Windows struct/typedef sets.

typedef struct _OVERLAPPED OVERLAPPED;
typedef struct tagMSG MSG, *PMSG, *NPMSG, *LPMSG;
typedef struct tagTOUCHINPUT TOUCHINPUT;
typedef struct tagPOINTER_INFO POINTER_INFO;
typedef struct tagRECT RECT;

typedef struct _RTL_SRWLOCK RTL_SRWLOCK;
typedef RTL_SRWLOCK SRWLOCK, *PSRWLOCK;

typedef struct _GUID GUID;
typedef GUID CLSID;
typedef GUID IID;
typedef GUID UUID;

typedef struct tagLOGFONTW LOGFONTW, *PLOGFONTW, *NPLOGFONTW, *LPLOGFONTW;
typedef LOGFONTW LOGFONT;

typedef struct _FILETIME FILETIME;

typedef struct tagMENUITEMINFOW MENUITEMINFOW, MENUITEMINFO;

typedef struct tagNMHDR NMHDR;

typedef struct _SP_DEVINFO_DATA SP_DEVINFO_DATA;

typedef PVOID PSID;
typedef PVOID PSECURITY_DESCRIPTOR;
typedef DWORD SECURITY_INFORMATION;

typedef HANDLE HLOCAL;

typedef /* [wire_marshal] */ WORD CLIPFORMAT;
typedef struct tagDVTARGETDEVICE DVTARGETDEVICE;

typedef struct tagFORMATETC FORMATETC;

// Use WIN32_FIND_DATAW when you just need a forward declaration. Use
// CHROME_WIN32_FIND_DATA when you need a concrete declaration to reserve
// space.
typedef struct _WIN32_FIND_DATAW WIN32_FIND_DATAW;
typedef WIN32_FIND_DATAW WIN32_FIND_DATA;

typedef UINT_PTR SOCKET;
typedef struct _PROCESS_INFORMATION PROCESS_INFORMATION;
typedef struct _SECURITY_CAPABILITIES SECURITY_CAPABILITIES;
typedef struct _ACL ACL;
typedef struct _SECURITY_DESCRIPTOR SECURITY_DESCRIPTOR;
typedef struct _GENERIC_MAPPING GENERIC_MAPPING;

// Declare Chrome versions of some Windows structures. These are needed for
// when we need a concrete type but don't want to pull in Windows.h. We can't
// declare the Windows types so we declare our types and cast to the Windows
// types in a few places. The sizes must match the Windows types so we verify
// that with static asserts in win_includes_unittest.cc.
// ChromeToWindowsType functions are provided for pointer conversions.

struct CHROME_SRWLOCK {
  PVOID Ptr;
};

struct CHROME_CONDITION_VARIABLE {
  PVOID Ptr;
};

struct CHROME_LUID {
  DWORD LowPart;
  LONG HighPart;

  bool operator==(const CHROME_LUID&) const = default;
};

// _WIN32_FIND_DATAW is 592 bytes and the largest built-in type in it is a
// DWORD. The buffer declaration guarantees the correct size and alignment.
struct CHROME_WIN32_FIND_DATA {
  DWORD buffer[592 / sizeof(DWORD)];
};

struct CHROME_FORMATETC {
  CLIPFORMAT cfFormat;
  /* [unique] */ DVTARGETDEVICE* ptd;
  DWORD dwAspect;
  LONG lindex;
  DWORD tymed;
};

struct CHROME_POINT {
  LONG x;
  LONG y;
};

struct CHROME_MSG {
  HWND hwnd;
  UINT message;
  WPARAM wParam;
  LPARAM lParam;
  DWORD time;
  CHROME_POINT pt;
};

// Define some commonly used Windows constants. Note that the layout of these
// macros - including internal spacing - must be 100% consistent with windows.h.

// clang-format off

#ifndef FALSE
#define FALSE               0
#endif
#ifndef TRUE
#define TRUE                1
#endif
#ifndef INVALID_HANDLE_VALUE
// Work around there being two slightly different definitions in the SDK.
#define INVALID_HANDLE_VALUE ((HANDLE)(LONG_PTR)-1)
#endif
#define TLS_OUT_OF_INDEXES ((DWORD)0xFFFFFFFF)
#define HTNOWHERE 0
#define MAX_PATH 260
#define CS_GLOBALCLASS 0x4000

#define ERROR_SUCCESS 0L
#define ERROR_FILE_NOT_FOUND 2L
#define ERROR_ACCESS_DENIED 5L
#define ERROR_INVALID_HANDLE 6L
#define ERROR_SHARING_VIOLATION 32L
#define ERROR_LOCK_VIOLATION 33L
#define ERROR_MORE_DATA 234L
#define REG_BINARY ( 3ul )
#define REG_NONE ( 0ul )

#ifndef STATUS_PENDING
// Allow people to include ntstatus.h
#define STATUS_PENDING ((DWORD   )0x00000103L)
#endif  // STATUS_PENDING
#define STILL_ACTIVE STATUS_PENDING
#define SUCCEEDED(hr) (((HRESULT)(hr)) >= 0)
#define FAILED(hr) (((HRESULT)(hr)) < 0)

#define HKEY_CLASSES_ROOT (( HKEY ) (ULONG_PTR)((LONG)0x80000000) )
#define HKEY_LOCAL_MACHINE (( HKEY ) (ULONG_PTR)((LONG)0x80000002) )
#define HKEY_CURRENT_USER (( HKEY ) (ULONG_PTR)((LONG)0x80000001) )
#define KEY_QUERY_VALUE (0x0001)
#define KEY_SET_VALUE (0x0002)
#define KEY_CREATE_SUB_KEY (0x0004)
#define KEY_ENUMERATE_SUB_KEYS (0x0008)
#define KEY_NOTIFY (0x0010)
#define KEY_CREATE_LINK (0x0020)
#define KEY_WOW64_32KEY (0x0200)
#define KEY_WOW64_64KEY (0x0100)
#define KEY_WOW64_RES (0x0300)

#define PROCESS_QUERY_INFORMATION (0x0400)
#define READ_CONTROL (0x00020000L)
#define SYNCHRONIZE (0x00100000L)

#define STANDARD_RIGHTS_READ (READ_CONTROL)
#define STANDARD_RIGHTS_WRITE (READ_CONTROL)
#define STANDARD_RIGHTS_ALL (0x001F0000L)

#define KEY_READ                ((STANDARD_RIGHTS_READ       |\
                                  KEY_QUERY_VALUE            |\
                                  KEY_ENUMERATE_SUB_KEYS     |\
                                  KEY_NOTIFY)                 \
                                  &                           \
                                 (~SYNCHRONIZE))


#define KEY_WRITE               ((STANDARD_RIGHTS_WRITE      |\
                                  KEY_SET_VALUE              |\
                                  KEY_CREATE_SUB_KEY)         \
                                  &                           \
                                 (~SYNCHRONIZE))

#define KEY_ALL_ACCESS          ((STANDARD_RIGHTS_ALL        |\
                                  KEY_QUERY_VALUE            |\
                                  KEY_SET_VALUE              |\
                                  KEY_CREATE_SUB_KEY         |\
                                  KEY_ENUMERATE_SUB_KEYS     |\
                                  KEY_NOTIFY                 |\
                                  KEY_CREATE_LINK)            \
                                  &                           \
                                 (~SYNCHRONIZE))

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
#define WINUSERAPI DECLSPEC_IMPORT
#define WINAPI __stdcall
#define APIENTRY WINAPI
#define CALLBACK __stdcall
#define NTAPI __stdcall

typedef INT_PTR(WINAPI* FARPROC)();

// Needed for LockImpl.
WINBASEAPI _Releases_exclusive_lock_(*SRWLock) VOID WINAPI
    ReleaseSRWLockExclusive(_Inout_ PSRWLOCK SRWLock);
WINBASEAPI BOOLEAN WINAPI TryAcquireSRWLockExclusive(_Inout_ PSRWLOCK SRWLock);

// Needed to support protobuf's GetMessage macro magic.
WINUSERAPI BOOL WINAPI GetMessageW(_Out_ LPMSG lpMsg,
                                   _In_opt_ HWND hWnd,
                                   _In_ UINT wMsgFilterMin,
                                   _In_ UINT wMsgFilterMax);

// Needed for thread_local_storage.h
WINBASEAPI LPVOID WINAPI TlsGetValue(_In_ DWORD dwTlsIndex);

WINBASEAPI BOOL WINAPI TlsSetValue(_In_ DWORD dwTlsIndex,
                                   _In_opt_ LPVOID lpTlsValue);

// Needed for scoped_handle.h
WINBASEAPI _Check_return_ _Post_equals_last_error_ DWORD WINAPI
    GetLastError(VOID);

WINBASEAPI VOID WINAPI SetLastError(_In_ DWORD dwErrCode);

WINBASEAPI BOOL WINAPI TerminateProcess(_In_ HANDLE hProcess,
                                        _In_ UINT uExitCode);

// Support for a deleter for LocalAlloc memory.
WINBASEAPI HLOCAL WINAPI LocalFree(_In_ HLOCAL hMem);

#ifdef __cplusplus
}

// Helper functions for converting between Chrome and Windows native versions of
// type pointers.
// Overloaded functions must be declared outside of the extern "C" block.

inline WIN32_FIND_DATA* ChromeToWindowsType(CHROME_WIN32_FIND_DATA* p) {
  return reinterpret_cast<WIN32_FIND_DATA*>(p);
}

inline const WIN32_FIND_DATA* ChromeToWindowsType(
    const CHROME_WIN32_FIND_DATA* p) {
  return reinterpret_cast<const WIN32_FIND_DATA*>(p);
}

inline FORMATETC* ChromeToWindowsType(CHROME_FORMATETC* p) {
  return reinterpret_cast<FORMATETC*>(p);
}

inline const FORMATETC* ChromeToWindowsType(const CHROME_FORMATETC* p) {
  return reinterpret_cast<const FORMATETC*>(p);
}

inline MSG* ChromeToWindowsType(CHROME_MSG* p) {
  return reinterpret_cast<MSG*>(p);
}

#endif

// These macros are all defined by windows.h and are also used as the names of
// functions in the Chromium code base. Having these macros consistently defined
// or undefined can be critical to avoid mismatches between the functions
// defined and functions called. Macros need to be added to this list in those
// cases where it is easier to have the macro defined everywhere rather than
// undefined everywhere. As windows.h is removed from more source files we may
// be able to shorten this list.

#define CopyFile CopyFileW
#define CreateDirectory CreateDirectoryW
#define CreateFile CreateFileW
#define CreateService CreateServiceW
#define DeleteFile DeleteFileW
#define DispatchMessage DispatchMessageW
#define DrawText DrawTextW
#define FindFirstFile FindFirstFileW
#define FindNextFile FindNextFileW
#define GetClassName GetClassNameW
#define GetCurrentDirectory GetCurrentDirectoryW
#define GetCurrentTime() GetTickCount()
#define GetFileAttributes GetFileAttributesW
#define GetMessage GetMessageW
#define LoadIcon LoadIconW
#define PostMessage PostMessageW
#define ReplaceFile ReplaceFileW
#define SendMessage SendMessageW
#define SendMessageCallback SendMessageCallbackW
#define SetCurrentDirectory SetCurrentDirectoryW

#endif  // BASE_WIN_WINDOWS_TYPES_H_
