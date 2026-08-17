// Windows Template Library - WTL version 10.0
// Copyright (C) Microsoft Corporation, WTL Team. All rights reserved.
//
// This file is a part of the Windows Template Library.
// The use and distribution terms for this software are covered by the
// Microsoft Public License (http://opensource.org/licenses/MS-PL)
// which can be found in the file MS-PL.txt at the root folder.

#ifndef __ATLAPP_H__
#define __ATLAPP_H__

#pragma once

#ifndef __cplusplus
	#error WTL requires C++ compilation (use a .cpp suffix)
#endif

#ifndef __ATLBASE_H__
	#error atlapp.h requires atlbase.h to be included first
#endif

#ifdef _WIN32_WCE
	#error WTL10 doesn't support Windows CE
#endif

#ifdef _ATL_NO_COMMODULE
	#error WTL requires that _ATL_NO_COMMODULE is not defined
#endif

#ifdef _ATL_NO_WIN_SUPPORT
	#error WTL requires that _ATL_NO_WIN_SUPPORT is not defined
#endif

#if (_MSC_VER < 1400)
	#error WTL10 requires C++ compiler version 14 (Visual C++ 2005) or higher
#endif

#if (WINVER < 0x0501)
	#error WTL requires WINVER >= 0x0501
#endif

#if (_WIN32_WINNT < 0x0501)
	#error WTL requires _WIN32_WINNT >= 0x0501
#endif

#if (_WIN32_IE < 0x0600)
	#error WTL requires _WIN32_IE >= 0x0600
#endif

#if (_ATL_VER < 0x0800)
	#error WTL10 requires ATL version 8 or higher
#endif

#ifdef _ATL_MIN_CRT
	#error WTL10 doesn't support _ATL_MIN_CRT
#endif

#ifdef _ATL_NO_MSIMG
	#error WTL10 doesn't support _ATL_NO_MSIMG
#endif

#include <limits.h>
#ifdef _MT
  #include <process.h>	// for _beginthreadex
#endif

#include <commctrl.h>
#pragma comment(lib, "comctl32.lib")

#include <commdlg.h>
#include <shellapi.h>

// Check for VS2005 without newer WinSDK
#if (_MSC_VER == 1400) && !defined(RB_GETEXTENDEDSTYLE)
	#error WTL10 requires WinSDK 6.0 ot higher
#endif

#include <uxtheme.h>
#pragma comment(lib, "uxtheme.lib")

#if defined(_SYSINFOAPI_H_) && defined(NOT_BUILD_WINDOWS_DEPRECATE)
  #include <VersionHelpers.h>
#endif

#include "atlres.h"


///////////////////////////////////////////////////////////////////////////////
// WTL version number

#define _WTL_VER	0x1000   // version 10.0


///////////////////////////////////////////////////////////////////////////////
// Classes in this file:
//
// CMessageFilter
// CIdleHandler
// CMessageLoop
//
// CAppModule
// CServerAppModule
//
// Global functions:
//   AtlInitCommonControls()
//   AtlGetDefaultGuiFont()
//   AtlCreateControlFont()
//   AtlCreateBoldFont()
//   AtlGetStringPtr()


///////////////////////////////////////////////////////////////////////////////
// Miscellaneous global support

// define useful macros from winuser.h
#ifndef IS_INTRESOURCE
  #define IS_INTRESOURCE(_r) (((ULONG_PTR)(_r) >> 16) == 0)
#endif // IS_INTRESOURCE

// protect template members from windowsx.h macros
#ifdef _INC_WINDOWSX
  #undef SubclassWindow
#endif // _INC_WINDOWSX

// define useful macros from windowsx.h
#ifndef GET_X_LPARAM
  #define GET_X_LPARAM(lParam)	((int)(short)LOWORD(lParam))
#endif
#ifndef GET_Y_LPARAM
  #define GET_Y_LPARAM(lParam)	((int)(short)HIWORD(lParam))
#endif

// Dummy structs for compiling with /CLR
#ifdef _MANAGED
  __if_not_exists(_IMAGELIST::_IMAGELIST) { struct _IMAGELIST { }; }
  __if_not_exists(_TREEITEM::_TREEITEM) { struct _TREEITEM { }; }
  __if_not_exists(_PSP::_PSP) { struct _PSP { }; }
#endif

// Forward declaration for ATL11 fix
#if (_ATL_VER >= 0x0B00)
  namespace ATL { HRESULT AtlGetCommCtrlVersion(LPDWORD pdwMajor, LPDWORD pdwMinor); }
#endif

#ifndef WM_MOUSEHWHEEL
  #define WM_MOUSEHWHEEL                  0x020E
#endif

// Used for stack allocations with ATL::CTempBuffer
#ifndef _WTL_STACK_ALLOC_THRESHOLD
  #define _WTL_STACK_ALLOC_THRESHOLD   512
#endif

// Used to declare overriden virtual functions
#if (__cplusplus >= 201103L) || (defined(_MSC_VER) && _MSC_VER >= 1900)
  #define _WTL_OVERRIDE override
#else
  #define _WTL_OVERRIDE
#endif


namespace WTL
{

DECLARE_TRACE_CATEGORY(atlTraceUI)
#ifdef _DEBUG
  __declspec(selectany) ATL::CTraceCategory atlTraceUI(_T("atlTraceUI"));
#endif // _DEBUG

// Common Controls initialization helper
inline BOOL AtlInitCommonControls(DWORD dwFlags)
{
	INITCOMMONCONTROLSEX iccx = { sizeof(INITCOMMONCONTROLSEX), dwFlags };
	BOOL bRet = ::InitCommonControlsEx(&iccx);
	ATLASSERT(bRet);
	return bRet;
}

// Default GUI font helper - "MS Shell Dlg" stock font
inline HFONT AtlGetDefaultGuiFont()
{
	return (HFONT)::GetStockObject(DEFAULT_GUI_FONT);
}

// Control font helper - default font for controls not in a dialog
// (NOTE: Caller owns the font, and should destroy it when it's no longer needed)
inline HFONT AtlCreateControlFont()
{
	LOGFONT lf = {};
	ATLVERIFY(::SystemParametersInfo(SPI_GETICONTITLELOGFONT, sizeof(LOGFONT), &lf, 0));
	HFONT hFont = ::CreateFontIndirect(&lf);
	ATLASSERT(hFont != NULL);
	return hFont;
}

// Bold font helper
// (NOTE: Caller owns the font, and should destroy it when it's no longer needed)
inline HFONT AtlCreateBoldFont(HFONT hFont = NULL)
{
	LOGFONT lf = {};
	if(hFont == NULL)
		ATLVERIFY(::SystemParametersInfo(SPI_GETICONTITLELOGFONT, sizeof(LOGFONT), &lf, 0));
	else
		(void)(ATLVERIFY(::GetObject(hFont, sizeof(LOGFONT), &lf) == sizeof(LOGFONT)));
	lf.lfWeight = FW_BOLD;
	HFONT hFontBold =  ::CreateFontIndirect(&lf);
	ATLASSERT(hFontBold != NULL);
	return hFontBold;
}

// Resource string pointer
inline LPCWSTR AtlGetStringPtr(UINT uID, int* pch = NULL)
{
	LPCWSTR lpstr = NULL;
	int nRet = ::LoadStringW(ATL::_AtlBaseModule.GetResourceInstance(), uID, (LPWSTR)&lpstr, 0);
	if(pch != NULL)
		*pch = nRet;
	return lpstr;
}


///////////////////////////////////////////////////////////////////////////////
// RunTimeHelper - helper functions for Windows version and structure sizes

#ifndef _WTL_NO_RUNTIME_STRUCT_SIZE

#ifndef _SIZEOF_STRUCT
  #define _SIZEOF_STRUCT(structname, member)  (((int)((LPBYTE)(&((structname*)0)->member) - ((LPBYTE)((structname*)0)))) + sizeof(((structname*)0)->member))
#endif

#if (_WIN32_WINNT >= 0x0600) && !defined(REBARBANDINFO_V6_SIZE)
  #define REBARBANDINFO_V6_SIZE   _SIZEOF_STRUCT(REBARBANDINFO, cxHeader)
#endif // (_WIN32_WINNT >= 0x0600) && !defined(REBARBANDINFO_V6_SIZE)

#if (_WIN32_WINNT >= 0x0600) && !defined(LVGROUP_V5_SIZE)
  #define LVGROUP_V5_SIZE   _SIZEOF_STRUCT(LVGROUP, uAlign)
#endif // (_WIN32_WINNT >= 0x0600) && !defined(LVGROUP_V5_SIZE)

#if (_WIN32_WINNT >= 0x0600) && !defined(LVTILEINFO_V5_SIZE)
  #define LVTILEINFO_V5_SIZE   _SIZEOF_STRUCT(LVTILEINFO, puColumns)
#endif // (_WIN32_WINNT >= 0x0600) && !defined(LVTILEINFO_V5_SIZE)

#if defined(NTDDI_VERSION) && (NTDDI_VERSION >= NTDDI_LONGHORN) && !defined(MCHITTESTINFO_V1_SIZE)
  #define MCHITTESTINFO_V1_SIZE   _SIZEOF_STRUCT(MCHITTESTINFO, st)
#endif // defined(NTDDI_VERSION) && (NTDDI_VERSION >= NTDDI_LONGHORN) && !defined(MCHITTESTINFO_V1_SIZE)

#if (WINVER >= 0x0600) && !defined(NONCLIENTMETRICS_V1_SIZE)
  #define NONCLIENTMETRICS_V1_SIZE   _SIZEOF_STRUCT(NONCLIENTMETRICS, lfMessageFont)
#endif // (WINVER >= 0x0600) && !defined(NONCLIENTMETRICS_V1_SIZE)

#ifndef TTTOOLINFO_V2_SIZE
  #define TTTOOLINFO_V2_SIZE   _SIZEOF_STRUCT(TTTOOLINFO, lParam)
#endif

#endif // !_WTL_NO_RUNTIME_STRUCT_SIZE

namespace RunTimeHelper
{
	inline bool IsCommCtrl6()
	{
		DWORD dwMajor = 0, dwMinor = 0;
		HRESULT hRet = ATL::AtlGetCommCtrlVersion(&dwMajor, &dwMinor);
		return (SUCCEEDED(hRet) && (dwMajor >= 6));
	}

	inline bool IsVista()
	{
#ifdef _versionhelpers_H_INCLUDED_
		return ::IsWindowsVistaOrGreater();
#else // !_versionhelpers_H_INCLUDED_
		OSVERSIONINFO ovi = { sizeof(OSVERSIONINFO) };
		BOOL bRet = ::GetVersionEx(&ovi);
		return ((bRet != FALSE) && (ovi.dwMajorVersion >= 6));
#endif // _versionhelpers_H_INCLUDED_
	}

	inline bool IsThemeAvailable()
	{
		return IsCommCtrl6() && (::IsThemeActive() != FALSE) && (::IsAppThemed() != FALSE);
	}

	inline bool IsWin7()
	{
#ifdef _versionhelpers_H_INCLUDED_
		return ::IsWindows7OrGreater();
#else // !_versionhelpers_H_INCLUDED_
		OSVERSIONINFO ovi = { sizeof(OSVERSIONINFO) };
		BOOL bRet = ::GetVersionEx(&ovi);
		return ((bRet != FALSE) && (ovi.dwMajorVersion == 6) && (ovi.dwMinorVersion >= 1));
#endif // _versionhelpers_H_INCLUDED_
	}

	inline bool IsRibbonUIAvailable()
	{
		static INT iRibbonUI = -1;

#if defined(NTDDI_WIN7) && (NTDDI_VERSION >= NTDDI_WIN7)
		if (iRibbonUI == -1)
		{
			HMODULE hRibbonDLL = ::LoadLibrary(_T("propsys.dll"));
			if (hRibbonDLL != NULL)
			{
				const GUID CLSID_UIRibbonFramework = { 0x926749fa, 0x2615, 0x4987, { 0x88, 0x45, 0xc3, 0x3e, 0x65, 0xf2, 0xb9, 0x57 } };
				// block - create instance
				{
					ATL::CComPtr<IUnknown> pIUIFramework;
					iRibbonUI = SUCCEEDED(pIUIFramework.CoCreateInstance(CLSID_UIRibbonFramework)) ? 1 : 0;
				}
				::FreeLibrary(hRibbonDLL);
			}
			else
			{
				iRibbonUI = 0;
			}
		}
#endif // defined(NTDDI_WIN7) && (NTDDI_VERSION >= NTDDI_WIN7)

		return (iRibbonUI == 1);
	}

	inline UINT SizeOf_REBARBANDINFO()
	{
		UINT uSize = sizeof(REBARBANDINFO);
#if !defined(_WTL_NO_RUNTIME_STRUCT_SIZE) && (_WIN32_WINNT >= 0x0600)
		if(!(IsVista() && IsCommCtrl6()))
			uSize = REBARBANDINFO_V6_SIZE;
#endif // !defined(_WTL_NO_RUNTIME_STRUCT_SIZE) && (_WIN32_WINNT >= 0x0600)
		return uSize;
	}

  	inline UINT SizeOf_LVGROUP()
	{
		UINT uSize = sizeof(LVGROUP);
#if !defined(_WTL_NO_RUNTIME_STRUCT_SIZE) && (_WIN32_WINNT >= 0x0600)
		if(!IsVista())
			uSize = LVGROUP_V5_SIZE;
#endif // !defined(_WTL_NO_RUNTIME_STRUCT_SIZE) && (_WIN32_WINNT >= 0x0600)
		return uSize;
	}

	inline UINT SizeOf_LVTILEINFO()
	{
		UINT uSize = sizeof(LVTILEINFO);
#if !defined(_WTL_NO_RUNTIME_STRUCT_SIZE) && (_WIN32_WINNT >= 0x0600)
		if(!IsVista())
			uSize = LVTILEINFO_V5_SIZE;
#endif // !defined(_WTL_NO_RUNTIME_STRUCT_SIZE) && (_WIN32_WINNT >= 0x0600)
		return uSize;
	}

	inline UINT SizeOf_MCHITTESTINFO()
	{
		UINT uSize = sizeof(MCHITTESTINFO);
#if !defined(_WTL_NO_RUNTIME_STRUCT_SIZE) && defined(NTDDI_VERSION) && (NTDDI_VERSION >= NTDDI_LONGHORN)
		if(!(IsVista() && IsCommCtrl6()))
			uSize = MCHITTESTINFO_V1_SIZE;
#endif // !defined(_WTL_NO_RUNTIME_STRUCT_SIZE) && defined(NTDDI_VERSION) && (NTDDI_VERSION >= NTDDI_LONGHORN)
		return uSize;
	}

	inline UINT SizeOf_NONCLIENTMETRICS()
	{
		UINT uSize = sizeof(NONCLIENTMETRICS);
#if !defined(_WTL_NO_RUNTIME_STRUCT_SIZE) && (WINVER >= 0x0600)
		if(!IsVista())
			uSize = NONCLIENTMETRICS_V1_SIZE;
#endif // !defined(_WTL_NO_RUNTIME_STRUCT_SIZE) && (WINVER >= 0x0600)
		return uSize;
	}

	inline UINT SizeOf_TOOLINFO()
	{
		UINT uSize = sizeof(TOOLINFO);
#ifndef _WTL_NO_RUNTIME_STRUCT_SIZE
		if(!IsVista())
			uSize = TTTOOLINFO_V2_SIZE;
#endif
		return uSize;
	}
} // namespace RunTimeHelper


///////////////////////////////////////////////////////////////////////////////
// ModuleHelper - helper functions for ATL (deprecated)

namespace ModuleHelper
{
	inline HINSTANCE GetModuleInstance()
	{
		return ATL::_AtlBaseModule.GetModuleInstance();
	}

	inline HINSTANCE GetResourceInstance()
	{
		return ATL::_AtlBaseModule.GetResourceInstance();
	}

	inline void AddCreateWndData(ATL::_AtlCreateWndData* pData, void* pObject)
	{
		ATL::_AtlWinModule.AddCreateWndData(pData, pObject);
	}

	inline void* ExtractCreateWndData()
	{
		return ATL::_AtlWinModule.ExtractCreateWndData();
	}
} // namespace ModuleHelper


///////////////////////////////////////////////////////////////////////////////
// SecureHelper - WTL10 requires use of secure functions
// these are here only for compatibility with existing projects

namespace SecureHelper
{
	inline void strcpyA_x(char* lpstrDest, size_t cchDest, const char* lpstrSrc)
	{
		ATL::Checked::strcpy_s(lpstrDest, cchDest, lpstrSrc);
	}

	inline void strcpyW_x(wchar_t* lpstrDest, size_t cchDest, const wchar_t* lpstrSrc)
	{
		ATL::Checked::wcscpy_s(lpstrDest, cchDest, lpstrSrc);
	}

	inline void strcpy_x(LPTSTR lpstrDest, size_t cchDest, LPCTSTR lpstrSrc)
	{
#ifdef _UNICODE
		strcpyW_x(lpstrDest, cchDest, lpstrSrc);
#else
		strcpyA_x(lpstrDest, cchDest, lpstrSrc);
#endif
	}

	inline errno_t strncpyA_x(char* lpstrDest, size_t cchDest, const char* lpstrSrc, size_t cchCount)
	{
		return ATL::Checked::strncpy_s(lpstrDest, cchDest, lpstrSrc, cchCount);
	}

	inline errno_t strncpyW_x(wchar_t* lpstrDest, size_t cchDest, const wchar_t* lpstrSrc, size_t cchCount)
	{
		return ATL::Checked::wcsncpy_s(lpstrDest, cchDest, lpstrSrc, cchCount);
	}

	inline errno_t strncpy_x(LPTSTR lpstrDest, size_t cchDest, LPCTSTR lpstrSrc, size_t cchCount)
	{
#ifdef _UNICODE
		return strncpyW_x(lpstrDest, cchDest, lpstrSrc, cchCount);
#else
		return strncpyA_x(lpstrDest, cchDest, lpstrSrc, cchCount);
#endif
	}

	inline void strcatA_x(char* lpstrDest, size_t cchDest, const char* lpstrSrc)
	{
		ATL::Checked::strcat_s(lpstrDest, cchDest, lpstrSrc);
	}

	inline void strcatW_x(wchar_t* lpstrDest, size_t cchDest, const wchar_t* lpstrSrc)
	{
		ATL::Checked::wcscat_s(lpstrDest, cchDest, lpstrSrc);
	}

	inline void strcat_x(LPTSTR lpstrDest, size_t cchDest, LPCTSTR lpstrSrc)
	{
#ifdef _UNICODE
		strcatW_x(lpstrDest, cchDest, lpstrSrc);
#else
		strcatA_x(lpstrDest, cchDest, lpstrSrc);
#endif
	}

	inline void memcpy_x(void* pDest, size_t cbDest, const void* pSrc, size_t cbSrc)
	{
		ATL::Checked::memcpy_s(pDest, cbDest, pSrc, cbSrc);
	}

	inline void memmove_x(void* pDest, size_t cbDest, const void* pSrc, size_t cbSrc)
	{
		ATL::Checked::memmove_s(pDest, cbDest, pSrc, cbSrc);
	}

	inline int vsprintf_x(LPTSTR lpstrBuff, size_t cchBuff, LPCTSTR lpstrFormat, va_list args)
	{
		return _vstprintf_s(lpstrBuff, cchBuff, lpstrFormat, args);
	}

	inline int wvsprintf_x(LPTSTR lpstrBuff, size_t cchBuff, LPCTSTR lpstrFormat, va_list args)
	{
		return _vstprintf_s(lpstrBuff, cchBuff, lpstrFormat, args);
	}

	inline int sprintf_x(LPTSTR lpstrBuff, size_t cchBuff, LPCTSTR lpstrFormat, ...)
	{
		va_list args;
		va_start(args, lpstrFormat);
		int nRes = vsprintf_x(lpstrBuff, cchBuff, lpstrFormat, args);
		va_end(args);
		return nRes;
	}

	inline int wsprintf_x(LPTSTR lpstrBuff, size_t cchBuff, LPCTSTR lpstrFormat, ...)
	{
		va_list args;
		va_start(args, lpstrFormat);
		int nRes = wvsprintf_x(lpstrBuff, cchBuff, lpstrFormat, args);
		va_end(args);
		return nRes;
	}
} // namespace SecureHelper


///////////////////////////////////////////////////////////////////////////////
// MinCrtHelper - WTL10 doesn't support _ATL_MIN_CRT,
// these are here only for compatibility with existing projects

namespace MinCrtHelper
{
	inline int _isspace(TCHAR ch)
	{
		return _istspace(ch);
	}

	inline int _isdigit(TCHAR ch)
	{
		return _istdigit(ch);
	}

	inline int _atoi(LPCTSTR str)
	{
		return _ttoi(str);
	}

	inline LPCTSTR _strrchr(LPCTSTR str, TCHAR ch)
	{
		return _tcsrchr(str, ch);
	}

	inline LPTSTR _strrchr(LPTSTR str, TCHAR ch)
	{
		return _tcsrchr(str, ch);
	}
} // namespace MinCrtHelper


///////////////////////////////////////////////////////////////////////////////
// GenericWndClass - generic window class usable for subclassing

// Use in dialog templates to specify a placeholder to be subclassed
// Specify as a custom control with class name WTL_GenericWindow
// Call Rregister() before creating dialog (for example, in WinMain)
namespace GenericWndClass
{
	inline LPCTSTR GetName()
	{
		return _T("WTL_GenericWindow");
	}

	inline ATOM Register()
	{
		WNDCLASSEX wc = { sizeof(WNDCLASSEX) };
		wc.lpfnWndProc = ::DefWindowProc;
		wc.hInstance = ModuleHelper::GetModuleInstance();
		wc.hCursor = ::LoadCursor(NULL, IDC_ARROW);
		wc.hbrBackground = (HBRUSH)(COLOR_WINDOW + 1);
		wc.lpszClassName = GetName();
		ATOM atom = ::RegisterClassEx(&wc);
		ATLASSERT(atom != 0);
		return atom;
	}

	inline BOOL Unregister()   // only needed for DLLs or tmp use
	{
		return ::UnregisterClass(GetName(), ModuleHelper::GetModuleInstance());
	}
} // namespace GenericWndClass


///////////////////////////////////////////////////////////////////////////////
// CMessageFilter - Interface for message filter support

class CMessageFilter
{
public:
	virtual BOOL PreTranslateMessage(MSG* pMsg) = 0;
};


///////////////////////////////////////////////////////////////////////////////
// CIdleHandler - Interface for idle processing

class CIdleHandler
{
public:
	virtual BOOL OnIdle() = 0;
};


///////////////////////////////////////////////////////////////////////////////
// CMessageLoop - message loop implementation

class CMessageLoop
{
public:
	ATL::CSimpleArray<CMessageFilter*> m_aMsgFilter;
	ATL::CSimpleArray<CIdleHandler*> m_aIdleHandler;
	MSG m_msg;

	CMessageLoop()
	{ }

	virtual ~CMessageLoop()
	{ }

// Message filter operations
	BOOL AddMessageFilter(CMessageFilter* pMessageFilter)
	{
		return m_aMsgFilter.Add(pMessageFilter);
	}

	BOOL RemoveMessageFilter(CMessageFilter* pMessageFilter)
	{
		return m_aMsgFilter.Remove(pMessageFilter);
	}

// Idle handler operations
	BOOL AddIdleHandler(CIdleHandler* pIdleHandler)
	{
		return m_aIdleHandler.Add(pIdleHandler);
	}

	BOOL RemoveIdleHandler(CIdleHandler* pIdleHandler)
	{
		return m_aIdleHandler.Remove(pIdleHandler);
	}

// message loop
	int Run()
	{
		BOOL bDoIdle = TRUE;
		int nIdleCount = 0;
		BOOL bRet;

		for(;;)
		{
			while(bDoIdle && !::PeekMessage(&m_msg, NULL, 0, 0, PM_NOREMOVE))
			{
				if(!OnIdle(nIdleCount++))
					bDoIdle = FALSE;
			}

			bRet = ::GetMessage(&m_msg, NULL, 0, 0);

			if(bRet == -1)
			{
				ATLTRACE2(atlTraceUI, 0, _T("::GetMessage returned -1 (error)\n"));
				continue;   // error, don't process
			}
			else if(!bRet)
			{
				ATLTRACE2(atlTraceUI, 0, _T("CMessageLoop::Run - exiting\n"));
				break;   // WM_QUIT, exit message loop
			}

			if(!PreTranslateMessage(&m_msg))
			{
				::TranslateMessage(&m_msg);
				::DispatchMessage(&m_msg);
			}

			if(IsIdleMessage(&m_msg))
			{
				bDoIdle = TRUE;
				nIdleCount = 0;
			}
		}

		return (int)m_msg.wParam;
	}

	static BOOL IsIdleMessage(MSG* pMsg)
	{
		// These messages should NOT cause idle processing
		switch(pMsg->message)
		{
		case WM_MOUSEMOVE:
		case WM_NCMOUSEMOVE:
		case WM_PAINT:
		case 0x0118:	// WM_SYSTIMER (caret blink)
			return FALSE;
		}

		return TRUE;
	}

// Overrideables
	// Override to change message filtering
	virtual BOOL PreTranslateMessage(MSG* pMsg)
	{
		// loop backwards
		for(int i = m_aMsgFilter.GetSize() - 1; i >= 0; i--)
		{
			CMessageFilter* pMessageFilter = m_aMsgFilter[i];
			if((pMessageFilter != NULL) && pMessageFilter->PreTranslateMessage(pMsg))
				return TRUE;
		}
		return FALSE;   // not translated
	}

	// override to change idle processing
	virtual BOOL OnIdle(int /*nIdleCount*/)
	{
		for(int i = 0; i < m_aIdleHandler.GetSize(); i++)
		{
			CIdleHandler* pIdleHandler = m_aIdleHandler[i];
			if(pIdleHandler != NULL)
				pIdleHandler->OnIdle();
		}
		return FALSE;   // don't continue
	}
};


///////////////////////////////////////////////////////////////////////////////
// CStaticDataInitCriticalSectionLock and CWindowCreateCriticalSectionLock
// internal classes to manage critical sections for ATL (deprecated)

class CStaticDataInitCriticalSectionLock
{
public:
	ATL::CComCritSecLock<ATL::CComCriticalSection> m_cslock;

	CStaticDataInitCriticalSectionLock() : m_cslock(ATL::_pAtlModule->m_csStaticDataInitAndTypeInfo, false)
	{ }

	HRESULT Lock()
	{
		return m_cslock.Lock();
	}

	void Unlock()
	{
		m_cslock.Unlock();
	}
};


class CWindowCreateCriticalSectionLock
{
public:
	ATL::CComCritSecLock<ATL::CComCriticalSection> m_cslock;

	CWindowCreateCriticalSectionLock() : m_cslock(ATL::_AtlWinModule.m_csWindowCreate, false)
	{ }

	HRESULT Lock()
	{
		return m_cslock.Lock();
	}

	void Unlock()
	{
		m_cslock.Unlock();
	}
};


///////////////////////////////////////////////////////////////////////////////
// CAppModule - module class for an application

#if (_MSC_VER == 1400)   // VS2005
  #pragma warning(push)
  #pragma warning(disable : 4244)
  #pragma warning(disable : 4312)
#endif

class CAppModule : public ATL::CComModule
{
public:
	DWORD m_dwMainThreadID;
	ATL::CSimpleMap<DWORD, CMessageLoop*>* m_pMsgLoopMap;
	ATL::CSimpleArray<HWND>* m_pSettingChangeNotify;

// Overrides of CComModule::Init and Term
	HRESULT Init(ATL::_ATL_OBJMAP_ENTRY* pObjMap, HINSTANCE hInstance, const GUID* pLibID = NULL)
	{
		HRESULT hRet = CComModule::Init(pObjMap, hInstance, pLibID);
		if(FAILED(hRet))
			return hRet;

		m_dwMainThreadID = ::GetCurrentThreadId();
		typedef ATL::CSimpleMap<DWORD, CMessageLoop*>   _mapClass;
		m_pMsgLoopMap = NULL;
		ATLTRY(m_pMsgLoopMap = new _mapClass);
		if(m_pMsgLoopMap == NULL)
			return E_OUTOFMEMORY;
		m_pSettingChangeNotify = NULL;

		return hRet;
	}

	void Term()
	{
		TermSettingChangeNotify();
		delete m_pMsgLoopMap;
		CComModule::Term();
	}

// Message loop map methods
	BOOL AddMessageLoop(CMessageLoop* pMsgLoop)
	{
		CStaticDataInitCriticalSectionLock lock;
		if(FAILED(lock.Lock()))
		{
			ATLTRACE2(atlTraceUI, 0, _T("ERROR : Unable to lock critical section in CAppModule::AddMessageLoop.\n"));
			ATLASSERT(FALSE);
			return FALSE;
		}

		ATLASSERT(pMsgLoop != NULL);
		ATLASSERT(m_pMsgLoopMap->Lookup(::GetCurrentThreadId()) == NULL);   // not in map yet

		BOOL bRet = m_pMsgLoopMap->Add(::GetCurrentThreadId(), pMsgLoop);

		lock.Unlock();

		return bRet;
	}

	BOOL RemoveMessageLoop()
	{
		CStaticDataInitCriticalSectionLock lock;
		if(FAILED(lock.Lock()))
		{
			ATLTRACE2(atlTraceUI, 0, _T("ERROR : Unable to lock critical section in CAppModule::RemoveMessageLoop.\n"));
			ATLASSERT(FALSE);
			return FALSE;
		}

		BOOL bRet = m_pMsgLoopMap->Remove(::GetCurrentThreadId());

		lock.Unlock();

		return bRet;
	}

	CMessageLoop* GetMessageLoop(DWORD dwThreadID = ::GetCurrentThreadId()) const
	{
		CStaticDataInitCriticalSectionLock lock;
		if(FAILED(lock.Lock()))
		{
			ATLTRACE2(atlTraceUI, 0, _T("ERROR : Unable to lock critical section in CAppModule::GetMessageLoop.\n"));
			ATLASSERT(FALSE);
			return NULL;
		}

		CMessageLoop* pLoop =  m_pMsgLoopMap->Lookup(dwThreadID);

		lock.Unlock();

		return pLoop;
	}

// Setting change notify methods
	// Note: Call this from the main thread for MSDI apps
	BOOL InitSettingChangeNotify(DLGPROC pfnDlgProc = _SettingChangeDlgProc)
	{
		CStaticDataInitCriticalSectionLock lock;
		if(FAILED(lock.Lock()))
		{
			ATLTRACE2(atlTraceUI, 0, _T("ERROR : Unable to lock critical section in CAppModule::InitSettingChangeNotify.\n"));
			ATLASSERT(FALSE);
			return FALSE;
		}

		if(m_pSettingChangeNotify == NULL)
		{
			typedef ATL::CSimpleArray<HWND>   _notifyClass;
			ATLTRY(m_pSettingChangeNotify = new _notifyClass);
			ATLASSERT(m_pSettingChangeNotify != NULL);
		}

		BOOL bRet = (m_pSettingChangeNotify != NULL);
		if(bRet && (m_pSettingChangeNotify->GetSize() == 0))
		{
			// init everything
			_ATL_EMPTY_DLGTEMPLATE templ;
			HWND hNtfWnd = ::CreateDialogIndirect(GetModuleInstance(), &templ, NULL, pfnDlgProc);
			ATLASSERT(::IsWindow(hNtfWnd));
			if(::IsWindow(hNtfWnd))
			{
				::SetWindowLongPtr(hNtfWnd, GWLP_USERDATA, (LONG_PTR)this);
				bRet = m_pSettingChangeNotify->Add(hNtfWnd);
			}
			else
			{
				bRet = FALSE;
			}
		}

		lock.Unlock();

		return bRet;
	}

	void TermSettingChangeNotify()
	{
		CStaticDataInitCriticalSectionLock lock;
		if(FAILED(lock.Lock()))
		{
			ATLTRACE2(atlTraceUI, 0, _T("ERROR : Unable to lock critical section in CAppModule::TermSettingChangeNotify.\n"));
			ATLASSERT(FALSE);
			return;
		}

		if((m_pSettingChangeNotify != NULL) && (m_pSettingChangeNotify->GetSize() > 0))
			::DestroyWindow((*m_pSettingChangeNotify)[0]);
		delete m_pSettingChangeNotify;
		m_pSettingChangeNotify = NULL;

		lock.Unlock();
	}

	BOOL AddSettingChangeNotify(HWND hWnd)
	{
		CStaticDataInitCriticalSectionLock lock;
		if(FAILED(lock.Lock()))
		{
			ATLTRACE2(atlTraceUI, 0, _T("ERROR : Unable to lock critical section in CAppModule::AddSettingChangeNotify.\n"));
			ATLASSERT(FALSE);
			return FALSE;
		}

		ATLASSERT(::IsWindow(hWnd));
		BOOL bRet = FALSE;
		if(InitSettingChangeNotify() != FALSE)
			bRet = m_pSettingChangeNotify->Add(hWnd);

		lock.Unlock();

		return bRet;
	}

	BOOL RemoveSettingChangeNotify(HWND hWnd)
	{
		CStaticDataInitCriticalSectionLock lock;
		if(FAILED(lock.Lock()))
		{
			ATLTRACE2(atlTraceUI, 0, _T("ERROR : Unable to lock critical section in CAppModule::RemoveSettingChangeNotify.\n"));
			ATLASSERT(FALSE);
			return FALSE;
		}

		BOOL bRet = FALSE;
		if(m_pSettingChangeNotify != NULL)
			bRet = m_pSettingChangeNotify->Remove(hWnd);

		lock.Unlock();

		return bRet;
	}

// Implementation - setting change notify dialog template and dialog procedure
	struct _ATL_EMPTY_DLGTEMPLATE : DLGTEMPLATE
	{
		_ATL_EMPTY_DLGTEMPLATE()
		{
			memset(this, 0, sizeof(_ATL_EMPTY_DLGTEMPLATE));
			style = WS_POPUP;
		}
		WORD wMenu, wClass, wTitle;
	};

	static INT_PTR CALLBACK _SettingChangeDlgProc(HWND hWnd, UINT uMsg, WPARAM wParam, LPARAM lParam)
	{
		if(uMsg == WM_SETTINGCHANGE)
		{
			CAppModule* pModule = (CAppModule*)::GetWindowLongPtr(hWnd, GWLP_USERDATA);
			ATLASSERT(pModule != NULL);
			ATLASSERT(pModule->m_pSettingChangeNotify != NULL);
			const UINT uTimeout = 1500;   // ms
			for(int i = 1; i < pModule->m_pSettingChangeNotify->GetSize(); i++)
				::SendMessageTimeout((*pModule->m_pSettingChangeNotify)[i], uMsg, wParam, lParam, SMTO_ABORTIFHUNG, uTimeout, NULL);

			return TRUE;
		}

		return FALSE;
	}
};

#if (_MSC_VER == 1400)   // VS2005
  #pragma warning(pop)
#endif


///////////////////////////////////////////////////////////////////////////////
// CServerAppModule - module class for a COM server application

class CServerAppModule : public CAppModule
{
public:
	HANDLE m_hEventShutdown;
	bool m_bActivity;
	DWORD m_dwTimeOut;
	DWORD m_dwPause;

// Override of CAppModule::Init
	HRESULT Init(ATL::_ATL_OBJMAP_ENTRY* pObjMap, HINSTANCE hInstance, const GUID* pLibID = NULL)
	{
		m_dwTimeOut = 5000;
		m_dwPause = 1000;
		return CAppModule::Init(pObjMap, hInstance, pLibID);
	}

	void Term()
	{
		if((m_hEventShutdown != NULL) && ::CloseHandle(m_hEventShutdown))
			m_hEventShutdown = NULL;
		CAppModule::Term();
	}

// COM Server methods
	LONG Unlock() throw()
	{
		LONG lRet = CComModule::Unlock();
		if(lRet == 0)
		{
			m_bActivity = true;
			::SetEvent(m_hEventShutdown); // tell monitor that we transitioned to zero
		}
		return lRet;
	}

	void MonitorShutdown()
	{
		for(;;)
		{
			::WaitForSingleObject(m_hEventShutdown, INFINITE);
			DWORD dwWait = 0;
			do
			{
				m_bActivity = false;
				dwWait = ::WaitForSingleObject(m_hEventShutdown, m_dwTimeOut);
			}
			while(dwWait == WAIT_OBJECT_0);
			// timed out
			if(!m_bActivity && (m_nLockCnt == 0)) // if no activity let's really bail
			{
#if defined(_WIN32_DCOM) && defined(_ATL_FREE_THREADED)
				::CoSuspendClassObjects();
				if(!m_bActivity && (m_nLockCnt == 0))
#endif
					break;
			}
		}
		// This handle should be valid now. If it isn't, 
		// check if _Module.Term was called first (it shouldn't)
		if(::CloseHandle(m_hEventShutdown))
			m_hEventShutdown = NULL;
		::PostThreadMessage(m_dwMainThreadID, WM_QUIT, 0, 0);
	}

	bool StartMonitor()
	{
		m_hEventShutdown = ::CreateEvent(NULL, false, false, NULL);
		if(m_hEventShutdown == NULL)
			return false;
		DWORD dwThreadID = 0;
#ifdef _MT
		HANDLE hThread = (HANDLE)_beginthreadex(NULL, 0, (UINT (WINAPI*)(void*))MonitorProc, this, 0, (UINT*)&dwThreadID);
#else
		HANDLE hThread = ::CreateThread(NULL, 0, MonitorProc, this, 0, &dwThreadID);
#endif
		bool bRet = (hThread != NULL);
		if(bRet)
			::CloseHandle(hThread);
		return bRet;
	}

	static DWORD WINAPI MonitorProc(void* pv)
	{
		CServerAppModule* p = (CServerAppModule*)pv;
		p->MonitorShutdown();
		return 0;
	}
};


///////////////////////////////////////////////////////////////////////////////
// CRegKeyEx - not used any more, here only for compatibility with old projects

typedef ATL::CRegKey CRegKeyEx;

} // namespace WTL


///////////////////////////////////////////////////////////////////////////////
// CString forward reference (enables CString use in atluser.h and atlgdi.h)

#if (defined(_WTL_USE_CSTRING) || defined(_WTL_FORWARD_DECLARE_CSTRING)) && !defined(__ATLSTR_H__)
  #include <atlstr.h>
#endif

// CString namespace
#define _CSTRING_NS	ATL

// Type classes namespace
#define _WTYPES_NS


///////////////////////////////////////////////////////////////////////////////
// General DLL version helpers (removed in ATL11)

#if (_ATL_VER >= 0x0B00)

namespace ATL
{

inline HRESULT AtlGetDllVersion(HINSTANCE hInstDLL, DLLVERSIONINFO* pDllVersionInfo)
{
	ATLASSERT(pDllVersionInfo != NULL);
	if(pDllVersionInfo == NULL)
		return E_INVALIDARG;

	// We must get this function explicitly because some DLLs don't implement it.
	DLLGETVERSIONPROC pfnDllGetVersion = (DLLGETVERSIONPROC)::GetProcAddress(hInstDLL, "DllGetVersion");
	if(pfnDllGetVersion == NULL)
		return E_NOTIMPL;

	return (*pfnDllGetVersion)(pDllVersionInfo);
}

inline HRESULT AtlGetDllVersion(LPCTSTR lpstrDllName, DLLVERSIONINFO* pDllVersionInfo)
{
	HINSTANCE hInstDLL = ::LoadLibrary(lpstrDllName);
	if(hInstDLL == NULL)
		return E_FAIL;
	HRESULT hRet = AtlGetDllVersion(hInstDLL, pDllVersionInfo);
	::FreeLibrary(hInstDLL);
	return hRet;
}

// Common Control Versions:
//   Win95/WinNT 4.0    maj=4 min=00
//   IE 3.x     maj=4 min=70
//   IE 4.0     maj=4 min=71
inline HRESULT AtlGetCommCtrlVersion(LPDWORD pdwMajor, LPDWORD pdwMinor)
{
	ATLASSERT((pdwMajor != NULL) && (pdwMinor != NULL));
	if((pdwMajor == NULL) || (pdwMinor == NULL))
		return E_INVALIDARG;

	DLLVERSIONINFO dvi;
	::ZeroMemory(&dvi, sizeof(dvi));
	dvi.cbSize = sizeof(dvi);
	HRESULT hRet = AtlGetDllVersion(_T("comctl32.dll"), &dvi);

	if(SUCCEEDED(hRet))
	{
		*pdwMajor = dvi.dwMajorVersion;
		*pdwMinor = dvi.dwMinorVersion;
	}
	else if(hRet == E_NOTIMPL)
	{
		// If DllGetVersion is not there, then the DLL is a version
		// previous to the one shipped with IE 3.x
		*pdwMajor = 4;
		*pdwMinor = 0;
		hRet = S_OK;
	}

	return hRet;
}

// Shell Versions:
//   Win95/WinNT 4.0                    maj=4 min=00
//   IE 3.x, IE 4.0 without Web Integrated Desktop  maj=4 min=00
//   IE 4.0 with Web Integrated Desktop         maj=4 min=71
//   IE 4.01 with Web Integrated Desktop        maj=4 min=72
inline HRESULT AtlGetShellVersion(LPDWORD pdwMajor, LPDWORD pdwMinor)
{
	ATLASSERT((pdwMajor != NULL) && (pdwMinor != NULL));
	if((pdwMajor == NULL) || (pdwMinor == NULL))
		return E_INVALIDARG;

	DLLVERSIONINFO dvi;
	::ZeroMemory(&dvi, sizeof(dvi));
	dvi.cbSize = sizeof(dvi);
	HRESULT hRet = AtlGetDllVersion(_T("shell32.dll"), &dvi);

	if(SUCCEEDED(hRet))
	{
		*pdwMajor = dvi.dwMajorVersion;
		*pdwMinor = dvi.dwMinorVersion;
	}
	else if(hRet == E_NOTIMPL)
	{
		// If DllGetVersion is not there, then the DLL is a version
		// previous to the one shipped with IE 4.x
		*pdwMajor = 4;
		*pdwMinor = 0;
		hRet = S_OK;
	}

	return hRet;
}

} // namespace ATL

#endif // (_ATL_VER >= 0x0B00)


// These are always included
#include "atlwinx.h"
#include "atluser.h"
#include "atlgdi.h"

#ifndef _WTL_NO_AUTOMATIC_NAMESPACE
using namespace WTL;
#endif // !_WTL_NO_AUTOMATIC_NAMESPACE

#endif // __ATLAPP_H__
