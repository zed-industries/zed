// Windows Template Library - WTL version 10.0
// Copyright (C) Microsoft Corporation, WTL Team. All rights reserved.
//
// This file is a part of the Windows Template Library.
// The use and distribution terms for this software are covered by the
// Microsoft Public License (http://opensource.org/licenses/MS-PL)
// which can be found in the file MS-PL.txt at the root folder.

#ifndef __ATLDWM_H__
#define __ATLDWM_H__

#pragma once

#ifndef __ATLAPP_H__
	#error atldwm.h requires atlapp.h to be included first
#endif

#ifndef __ATLWIN_H__
	#error atldwm.h requires atlwin.h to be included first
#endif

#if (_WIN32_WINNT < 0x0600)
	#error atldwm.h requires _WIN32_WINNT >= 0x0600
#endif

#ifndef _DWMAPI_H_
  #include <dwmapi.h>
#endif
#pragma comment(lib, "dwmapi.lib")

// Note: To create an application that also runs on older versions of Windows,
// use delay load of dwmapi.dll and ensure that no calls to the DWM API are
// Delay load is NOT AUTOMATIC for VC++ 7, you have to link to delayimp.lib, 
// and add dwmapi.dll in the Linker.Input.Delay Loaded DLLs section of the 
// project properties.


///////////////////////////////////////////////////////////////////////////////
// Classes in this file:
//
// CDwm
// CDwmImpl<T, TBase>
// CDwmWindowT<TBase> - CDwmWindow
// CDwmThumbnailT<t_bManaged, TBase>
// CDwmThumbnail
// CDwmThumbnailHandle
// CAeroControlImpl


namespace WTL
{

///////////////////////////////////////////////////////////////////////////////
// CDwm - wrapper for DWM handle

class CDwm
{
public:
// Data members
	static int m_nIsDwmSupported;

// Constructor
	CDwm()
	{
		IsDwmSupported();
	}

// Dwm support helper
	static bool IsDwmSupported()
	{
		if(m_nIsDwmSupported == -1)
		{
			CStaticDataInitCriticalSectionLock lock;
			if(FAILED(lock.Lock()))
			{
				ATLTRACE2(atlTraceUI, 0, _T("ERROR : Unable to lock critical section in CDwm::IsDwmSupported.\n"));
				ATLASSERT(FALSE);
				return false;
			}

			if(m_nIsDwmSupported == -1)
			{
				HMODULE hDwmDLL = ::LoadLibrary(_T("dwmapi.dll"));
				m_nIsDwmSupported = (hDwmDLL != NULL) ? 1 : 0;
				if(hDwmDLL != NULL)
					::FreeLibrary(hDwmDLL);
			}

			lock.Unlock();
		}

		ATLASSERT(m_nIsDwmSupported != -1);
		return (m_nIsDwmSupported == 1);
	}

// Operations
	BOOL DwmIsCompositionEnabled() const
	{
		if(!IsDwmSupported())
			return FALSE;

		BOOL bRes = FALSE;
		return (SUCCEEDED(::DwmIsCompositionEnabled(&bRes)) && bRes) ? TRUE : FALSE;
	}

	BOOL DwmEnableComposition(UINT fEnable)
	{
		if(!IsDwmSupported())
			return FALSE;

		return SUCCEEDED(::DwmEnableComposition(fEnable)) ? TRUE : FALSE;
	}

	BOOL DwmEnableMMCSS(BOOL fEnableMMCSS)
	{
		if(!IsDwmSupported())
			return FALSE;

		return SUCCEEDED(::DwmEnableMMCSS(fEnableMMCSS)) ? TRUE : FALSE;
	}

	HRESULT DwmGetColorizationColor(DWORD* pcrColorization, BOOL* pfOpaqueBlend)
	{
		if(!IsDwmSupported())
			return E_NOTIMPL;

		return ::DwmGetColorizationColor(pcrColorization, pfOpaqueBlend);
	}

	HRESULT DwmFlush()
	{
		if(!IsDwmSupported())
			return E_NOTIMPL;

		return ::DwmFlush();
	}
};

__declspec(selectany) int CDwm::m_nIsDwmSupported = -1;


///////////////////////////////////////////////////////////////////////////////
// CDwmImpl - DWM window support

template <class T, class TBase = CDwm>
class CDwmImpl : public TBase
{
public:
	HRESULT DwmEnableBlurBehindWindow(const DWM_BLURBEHIND* pBB)
	{
		if(!this->IsDwmSupported())
			return E_NOTIMPL;

		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));
		return ::DwmEnableBlurBehindWindow(pT->m_hWnd, pBB);
	}

	HRESULT DwmExtendFrameIntoClientArea(const MARGINS* pMargins)
	{
		if(!this->IsDwmSupported())
			return E_NOTIMPL;

		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));
		return ::DwmExtendFrameIntoClientArea(pT->m_hWnd, pMargins);
	}

	HRESULT DwmExtendFrameIntoEntireClientArea()
	{
		MARGINS margins = { -1 };
		return DwmExtendFrameIntoClientArea(&margins);
	}

	HRESULT DwmGetCompositionTimingInfo(DWM_TIMING_INFO* pTimingInfo)
	{
		if(!this->IsDwmSupported())
			return E_NOTIMPL;

		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));
		return ::DwmGetCompositionTimingInfo(pT->m_hWnd, pTimingInfo);
	}

	HRESULT DwmGetWindowAttribute(DWORD dwAttribute, PVOID pvAttribute, DWORD cbAttribute)
	{
		if(!this->IsDwmSupported())
			return E_NOTIMPL;

		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));
		return ::DwmGetWindowAttribute(pT->m_hWnd, dwAttribute, pvAttribute, cbAttribute);
	}

	HRESULT DwmModifyPreviousDxFrameDuration(INT cRefreshes, BOOL fRelative)
	{
		if(!this->IsDwmSupported())
			return E_NOTIMPL;

		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));
		return ::DwmModifyPreviousDxFrameDuration(pT->m_hWnd, cRefreshes, fRelative);
	}

	HRESULT DwmSetDxFrameDuration(INT cRefreshes)
	{
		if(!this->IsDwmSupported())
			return E_NOTIMPL;

		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));
		return ::DwmSetDxFrameDuration(pT->m_hWnd, cRefreshes);
	}

	HRESULT DwmSetPresentParameters(DWM_PRESENT_PARAMETERS* pPresentParams)
	{
		if(!this->IsDwmSupported())
			return E_NOTIMPL;

		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));
		return ::DwmSetPresentParameters(pT->m_hWnd, pPresentParams);
	}

	HRESULT DwmSetWindowAttribute(DWORD dwAttribute, LPCVOID pvAttribute, DWORD cbAttribute)
	{
		if(!this->IsDwmSupported())
			return E_NOTIMPL;

		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));
		return ::DwmSetWindowAttribute(pT->m_hWnd, dwAttribute, pvAttribute, cbAttribute);
	}

	HRESULT DwmAttachMilContent()
	{
		if(!this->IsDwmSupported())
			return E_NOTIMPL;

		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));
		return ::DwmAttachMilContent(pT->m_hWnd);
	}

	HRESULT DwmDetachMilContent()
	{
		if(!this->IsDwmSupported())
			return E_NOTIMPL;

		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));
		return ::DwmDetachMilContent(pT->m_hWnd);
	}
};

template <class TBase>
class CDwmWindowT : public TBase, public CDwmImpl<CDwmWindowT< TBase > >
{
public:
	CDwmWindowT(HWND hWnd = NULL) : TBase(hWnd)
	{ }

	CDwmWindowT< TBase >& operator =(HWND hWnd)
	{
		this->m_hWnd = hWnd;
		return *this;
	}
};

typedef CDwmWindowT<ATL::CWindow>	CDwmWindow;


///////////////////////////////////////////////////////////////////////////////
// CDwmThumbnail - provides DWM thumbnail support

template <bool t_bManaged, class TBase = CDwm>
class CDwmThumbnailT : public TBase
{
public:
// Data members
	HTHUMBNAIL m_hThumbnail;

// Constructor
	CDwmThumbnailT(HTHUMBNAIL hThumbnail = NULL) : m_hThumbnail(hThumbnail)
	{ }

	~CDwmThumbnailT()
	{
		if(t_bManaged && (m_hThumbnail != NULL))
			Unregister();
	}

// Operations
	CDwmThumbnailT<t_bManaged, TBase>& operator =(HTHUMBNAIL hThumbnail)
	{
		Attach(hThumbnail);
		return *this;
	}

	void Attach(HTHUMBNAIL hThumbnailNew)
	{
		if(t_bManaged && (m_hThumbnail != NULL) && (m_hThumbnail != hThumbnailNew))
			Unregister();
		m_hThumbnail = hThumbnailNew;
	}

	HTHUMBNAIL Detach()
	{
		HTHUMBNAIL hThumbnail = m_hThumbnail;
		m_hThumbnail = NULL;
		return hThumbnail;
	}

	HRESULT Register(HWND hwndDestination, HWND hwndSource)
	{
		ATLASSERT(::IsWindow(hwndDestination));
		ATLASSERT(::IsWindow(hwndSource));
		ATLASSERT(m_hThumbnail==NULL);

		if(!this->IsDwmSupported())
			return E_NOTIMPL;

		return ::DwmRegisterThumbnail(hwndDestination, hwndSource, &m_hThumbnail);
	}

	HRESULT Unregister()
	{
		if(!this->IsDwmSupported())
			return E_NOTIMPL;
		if(m_hThumbnail == NULL)
			return S_FALSE;

		HRESULT Hr = ::DwmUnregisterThumbnail(m_hThumbnail);
		if(SUCCEEDED(Hr))
			m_hThumbnail = NULL;

		return Hr;
	}

	operator HTHUMBNAIL() const { return m_hThumbnail; }

	bool IsNull() const { return (m_hThumbnail == NULL); }

	HRESULT UpdateProperties(const DWM_THUMBNAIL_PROPERTIES* ptnProperties)
	{
		if(!this->IsDwmSupported())
			return E_NOTIMPL;

		ATLASSERT(m_hThumbnail != NULL);
		return ::DwmUpdateThumbnailProperties(m_hThumbnail, ptnProperties);
	}

// Attributes
	HRESULT QuerySourceSize(PSIZE pSize)
	{
		if(!this->IsDwmSupported())
			return E_NOTIMPL;

		ATLASSERT(m_hThumbnail != NULL);
		return ::DwmQueryThumbnailSourceSize(m_hThumbnail, pSize);
	}
};

typedef CDwmThumbnailT<true, CDwm> CDwmThumbnail;
typedef CDwmThumbnailT<false, CDwm> CDwmThumbnailHandle;


#ifdef __ATLTHEME_H__

///////////////////////////////////////////////////////////////////////////////
// CAeroControlImpl - Base class for controls on Glass

template <class T, class TBase = ATL::CWindow, class TWinTraits = ATL::CControlWinTraits>
class CAeroControlImpl : public CThemeImpl<T>,
                         public CBufferedPaintImpl<T>,
                         public ATL::CWindowImpl<T, TBase, TWinTraits>
{
public:
	typedef CThemeImpl<T> _themeClass;
	typedef CBufferedPaintImpl<T> _baseClass;
	typedef ATL::CWindowImpl<T, TBase, TWinTraits> _windowClass;

	CAeroControlImpl()
	{
		this->m_PaintParams.dwFlags = BPPF_ERASE;
	}

	static LPCWSTR GetThemeName()
	{
#ifdef _UNICODE
		return TBase::GetWndClassName();
#else
		ATLASSERT(!_T("Return UNICODE string of window classname / theme class"));
		return NULL;
#endif // _UNICODE
	}

// Message map and handlers
	BEGIN_MSG_MAP(CAeroControlImpl)
		MESSAGE_HANDLER(WM_CREATE, OnCreate)
		MESSAGE_HANDLER(WM_ACTIVATE, OnActivate)
		CHAIN_MSG_MAP(_themeClass)
		CHAIN_MSG_MAP(_baseClass)
	END_MSG_MAP()

	LRESULT OnCreate(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& bHandled)
	{
		T* pT = static_cast<T*>(this);
		pT->Init();

		bHandled = FALSE;
		return 0;
	}

	LRESULT OnActivate(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& bHandled)
	{
		if(this->IsThemingSupported())
			this->Invalidate(FALSE);

		bHandled = FALSE;
		return 0;
	}

// Operations
	BOOL SubclassWindow(HWND hWnd)
	{
		ATLASSERT(this->m_hWnd == NULL);
		ATLASSERT(::IsWindow(hWnd));
		BOOL bRet = _windowClass::SubclassWindow(hWnd);
		if(bRet)
		{
			T* pT = static_cast<T*>(this);
			pT->Init();
		}

		return bRet;
	}

// Implementation
	LRESULT DefWindowProc()
	{
		const ATL::_ATL_MSG* pMsg = this->m_pCurrentMsg;
		LRESULT lRes = 0;
		if(pMsg != NULL)
			lRes = DefWindowProc(pMsg->message, pMsg->wParam, pMsg->lParam);

		return lRes;
	}

	LRESULT DefWindowProc(UINT uMsg, WPARAM wParam, LPARAM lParam)
	{
		T* pT = static_cast<T*>(this);
		LRESULT lRes = 0;
		if(::DwmDefWindowProc(pT->m_hWnd, uMsg, wParam, lParam, &lRes) != FALSE)
			return lRes;

		return _windowClass::DefWindowProc(uMsg, wParam, lParam);
	}

	void DoBufferedPaint(HDC hDC, RECT& rcPaint)
	{
		T* pT = static_cast<T*>(this);
		HDC hDCPaint = NULL;
		RECT rcClient = {};
		this->GetClientRect(&rcClient);
		this->m_BufferedPaint.Begin(hDC, &rcClient, this->m_dwFormat, &this->m_PaintParams, &hDCPaint);
		ATLASSERT(hDCPaint != NULL);
		pT->DoAeroPaint(hDCPaint, rcClient, rcPaint);
		this->m_BufferedPaint.End();
	}

	void DoPaint(HDC /*hdc*/, RECT& /*rcClient*/)
	{
		DefWindowProc();
	}

// Overridables
	void Init()
	{
		T* pT = static_cast<T*>(this);
		(void)pT;   // avoid level 4 warning
		this->SetThemeClassList(pT->GetThemeName());
		if(this->m_lpstrThemeClassList != NULL)
			this->OpenThemeData();
	}

	void DoAeroPaint(HDC hDC, RECT& /*rcClient*/, RECT& rcPaint)
	{
		DefWindowProc(WM_PAINT, (WPARAM) hDC, 0L);
		this->m_BufferedPaint.MakeOpaque(&rcPaint);
	}
};

#endif // __ATLTHEME_H__

} // namespace WTL

#endif // __ATLDWM_H__
