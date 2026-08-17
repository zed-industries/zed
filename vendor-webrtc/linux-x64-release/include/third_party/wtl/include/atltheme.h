// Windows Template Library - WTL version 10.0
// Copyright (C) Microsoft Corporation, WTL Team. All rights reserved.
//
// This file is a part of the Windows Template Library.
// The use and distribution terms for this software are covered by the
// Microsoft Public License (http://opensource.org/licenses/MS-PL)
// which can be found in the file MS-PL.txt at the root folder.

#ifndef __ATLTHEME_H__
#define __ATLTHEME_H__

#pragma once

#ifndef __ATLAPP_H__
	#error atltheme.h requires atlapp.h to be included first
#endif

#ifndef __ATLWIN_H__
	#error atltheme.h requires atlwin.h to be included first
#endif

#include <vssym32.h>

// Note: To create an application that also runs on older versions of Windows,
// use delay load of uxtheme.dll and ensure that no calls to the Theme API are
// made if theming is not supported. It is enough to check if m_hTheme is NULL.
// Example:
//	if(m_hTheme != NULL)
//	{
//		DrawThemeBackground(dc, BP_PUSHBUTTON, PBS_NORMAL, &rect, NULL);
//		DrawThemeText(dc, BP_PUSHBUTTON, PBS_NORMAL, L"Button", -1, DT_SINGLELINE | DT_CENTER | DT_VCENTER, 0, &rect);
//	}
//	else
//	{
//		dc.DrawFrameControl(&rect, DFC_BUTTON, DFCS_BUTTONPUSH);
//		dc.DrawText(_T("Button"), -1, &rect, DT_SINGLELINE | DT_CENTER | DT_VCENTER);
//	}
//
// Delay load is NOT AUTOMATIC for VC++ 7, you have to link to delayimp.lib, 
// and add uxtheme.dll in the Linker.Input.Delay Loaded DLLs section of the 
// project properties.


///////////////////////////////////////////////////////////////////////////////
// Classes in this file:
//
// CTheme
// CThemeImpl<T, TBase>
//
// CBufferedPaint
// CBufferedPaintImpl<T>
// CBufferedPaintWindowImpl<T, TBase, TWinTraits>
// CBufferedAnimation
// CBufferedAnimationImpl<T, TState>
// CBufferedAnimationWindowImpl<T, TState, TBase, TWinTraits>
//
// Global functions:
//   AtlDrawThemeClientEdge()


namespace WTL
{

///////////////////////////////////////////////////////////////////////////////
// CTheme - wrapper for theme handle

class CTheme
{
public:
// Data members
	HTHEME m_hTheme;
	static int m_nIsThemingSupported;

// Constructor
	CTheme(HTHEME hTheme = NULL) : m_hTheme(hTheme)
	{
		IsThemingSupported();
	}

// Operators and helpers
	bool IsThemeNull() const
	{
		return (m_hTheme == NULL);
	}

	CTheme& operator =(HTHEME hTheme)
	{
		m_hTheme = hTheme;
		return *this;
	}

	operator HTHEME() const
	{
		return m_hTheme;
	}

	void Attach(HTHEME hTheme)
	{
		m_hTheme = hTheme;
	}

	HTHEME Detach()
	{
		HTHEME hTheme = m_hTheme;
		m_hTheme = NULL;
		return hTheme;
	}

// Theme support helper
	static bool IsThemingSupported()
	{
		if(m_nIsThemingSupported == -1)
		{
			CStaticDataInitCriticalSectionLock lock;
			if(FAILED(lock.Lock()))
			{
				ATLTRACE2(atlTraceUI, 0, _T("ERROR : Unable to lock critical section in CTheme::IsThemingSupported.\n"));
				ATLASSERT(FALSE);
				return false;
			}

			if(m_nIsThemingSupported == -1)
			{
				HMODULE hThemeDLL = ::LoadLibrary(_T("uxtheme.dll"));
				m_nIsThemingSupported = (hThemeDLL != NULL) ? 1 : 0;
				if(hThemeDLL != NULL)
					::FreeLibrary(hThemeDLL);
			}

			lock.Unlock();
		}

		ATLASSERT(m_nIsThemingSupported != -1);
		return (m_nIsThemingSupported == 1);
	}

// Operations and theme properties
	HTHEME OpenThemeData(HWND hWnd, LPCWSTR pszClassList)
	{
		if(!IsThemingSupported())
			return NULL;

		ATLASSERT(m_hTheme == NULL);
		m_hTheme = ::OpenThemeData(hWnd, pszClassList);
		return m_hTheme;
	}

	HRESULT CloseThemeData()
	{
		HRESULT hRet = S_FALSE;
		if(m_hTheme != NULL)
		{
			hRet = ::CloseThemeData(m_hTheme);
			if(SUCCEEDED(hRet))
				m_hTheme = NULL;
		}
		return hRet;
	}

	HRESULT DrawThemeBackground(HDC hDC, int nPartID, int nStateID, LPCRECT pRect, LPCRECT pClipRect = NULL)
	{
		ATLASSERT(m_hTheme != NULL);
		return ::DrawThemeBackground(m_hTheme, hDC, nPartID, nStateID, pRect, pClipRect);
	}

// Missing in original uxtheme.h
#ifdef DTBG_CLIPRECT
	HRESULT DrawThemeBackgroundEx(HDC hDC, int nPartID, int nStateID, LPCRECT pRect, const DTBGOPTS* pOptions = NULL)
	{
		ATLASSERT(m_hTheme != NULL);
		return ::DrawThemeBackgroundEx(m_hTheme, hDC, nPartID, nStateID, pRect, pOptions);
	}
#endif // DTBG_CLIPRECT

	HRESULT DrawThemeText(HDC hDC, int nPartID, int nStateID, LPCWSTR pszText, int nCharCount, DWORD dwTextFlags, DWORD dwTextFlags2, LPCRECT pRect)
	{
		ATLASSERT(m_hTheme != NULL);
		return ::DrawThemeText(m_hTheme, hDC, nPartID, nStateID, pszText, nCharCount, dwTextFlags, dwTextFlags2, pRect);
	}

	HRESULT GetThemeBackgroundContentRect(HDC hDC, int nPartID, int nStateID,  LPCRECT pBoundingRect, LPRECT pContentRect) const
	{
		ATLASSERT(m_hTheme != NULL);
		return ::GetThemeBackgroundContentRect(m_hTheme, hDC, nPartID, nStateID,  pBoundingRect, pContentRect);
	}

	HRESULT GetThemeBackgroundExtent(HDC hDC, int nPartID, int nStateID, LPCRECT pContentRect, LPRECT pExtentRect) const
	{
		ATLASSERT(m_hTheme != NULL);
		return ::GetThemeBackgroundExtent(m_hTheme, hDC, nPartID, nStateID, pContentRect, pExtentRect);
	}

	HRESULT GetThemePartSize(HDC hDC, int nPartID, int nStateID, LPCRECT pRect, enum THEMESIZE eSize, LPSIZE pSize) const
	{
		ATLASSERT(m_hTheme != NULL);
		return ::GetThemePartSize(m_hTheme, hDC, nPartID, nStateID, pRect, eSize, pSize);
	}

	HRESULT GetThemeTextExtent(HDC hDC, int nPartID, int nStateID, LPCWSTR pszText, int nCharCount, DWORD dwTextFlags, LPCRECT  pBoundingRect, LPRECT pExtentRect) const
	{
		ATLASSERT(m_hTheme != NULL);
		return ::GetThemeTextExtent(m_hTheme, hDC, nPartID, nStateID, pszText, nCharCount, dwTextFlags, pBoundingRect, pExtentRect);
	}

	HRESULT GetThemeTextMetrics(HDC hDC, int nPartID, int nStateID, PTEXTMETRICW pTextMetric) const
	{
		ATLASSERT(m_hTheme != NULL);
		return ::GetThemeTextMetrics(m_hTheme, hDC, nPartID, nStateID, pTextMetric);
	}

	HRESULT GetThemeBackgroundRegion(HDC hDC, int nPartID, int nStateID, LPCRECT pRect, HRGN* pRegion) const
	{
		ATLASSERT(m_hTheme != NULL);
		return ::GetThemeBackgroundRegion(m_hTheme, hDC, nPartID, nStateID, pRect, pRegion);
	}

	HRESULT HitTestThemeBackground(HDC hDC, int nPartID, int nStateID, DWORD dwOptions, LPCRECT pRect, HRGN hrgn, POINT ptTest, WORD* pwHitTestCode) const
	{
		ATLASSERT(m_hTheme != NULL);
		return ::HitTestThemeBackground(m_hTheme, hDC, nPartID, nStateID, dwOptions, pRect, hrgn, ptTest, pwHitTestCode);
	}

	HRESULT DrawThemeEdge(HDC hDC, int nPartID, int nStateID, LPCRECT pDestRect, UINT uEdge, UINT uFlags, LPRECT pContentRect = NULL)
	{
		ATLASSERT(m_hTheme != NULL);
		return ::DrawThemeEdge(m_hTheme, hDC, nPartID, nStateID, pDestRect, uEdge, uFlags, pContentRect);
	}

	HRESULT DrawThemeIcon(HDC hDC, int nPartID, int nStateID, LPCRECT pRect, HIMAGELIST himl, int nImageIndex)
	{
		ATLASSERT(m_hTheme != NULL);
		return ::DrawThemeIcon(m_hTheme, hDC, nPartID, nStateID, pRect, himl, nImageIndex);
	}

	BOOL IsThemePartDefined(int nPartID, int nStateID) const
	{
		ATLASSERT(m_hTheme != NULL);
		return ::IsThemePartDefined(m_hTheme, nPartID, nStateID);
	}

	BOOL IsThemeBackgroundPartiallyTransparent(int nPartID, int nStateID) const
	{
		ATLASSERT(m_hTheme != NULL);
		return ::IsThemeBackgroundPartiallyTransparent(m_hTheme, nPartID, nStateID);
	}

	HRESULT GetThemeColor(int nPartID, int nStateID, int nPropID, COLORREF* pColor) const
	{
		ATLASSERT(m_hTheme != NULL);
		return ::GetThemeColor(m_hTheme, nPartID, nStateID, nPropID, pColor);
	}

	HRESULT GetThemeMetric(HDC hDC, int nPartID, int nStateID, int nPropID, int* pnVal) const
	{
		ATLASSERT(m_hTheme != NULL);
		return ::GetThemeMetric(m_hTheme, hDC, nPartID, nStateID, nPropID, pnVal);
	}

	HRESULT GetThemeString(int nPartID, int nStateID, int nPropID, LPWSTR pszBuff, int cchMaxBuffChars) const
	{
		ATLASSERT(m_hTheme != NULL);
		return ::GetThemeString(m_hTheme, nPartID, nStateID, nPropID, pszBuff, cchMaxBuffChars);
	}

	HRESULT GetThemeBool(int nPartID, int nStateID, int nPropID, BOOL* pfVal) const
	{
		ATLASSERT(m_hTheme != NULL);
		return ::GetThemeBool(m_hTheme, nPartID, nStateID, nPropID, pfVal);
	}

	HRESULT GetThemeInt(int nPartID, int nStateID, int nPropID, int* pnVal) const
	{
		ATLASSERT(m_hTheme != NULL);
		return ::GetThemeInt(m_hTheme, nPartID, nStateID, nPropID, pnVal);
	}

	HRESULT GetThemeEnumValue(int nPartID, int nStateID, int nPropID, int* pnVal) const
	{
		ATLASSERT(m_hTheme != NULL);
		return ::GetThemeEnumValue(m_hTheme, nPartID, nStateID, nPropID, pnVal);
	}

	HRESULT GetThemePosition(int nPartID, int nStateID, int nPropID, LPPOINT pPoint) const
	{
		ATLASSERT(m_hTheme != NULL);
		return ::GetThemePosition(m_hTheme, nPartID, nStateID, nPropID, pPoint);
	}

	// deprecated
	HRESULT GetThemeFont(int nPartID, HDC hDC, int nStateID, int nPropID, LOGFONTW* pFont) const
	{
		ATLASSERT(m_hTheme != NULL);
		return ::GetThemeFont(m_hTheme, hDC, nPartID, nStateID, nPropID, pFont);
	}

	HRESULT GetThemeFont(HDC hDC, int nPartID, int nStateID, int nPropID, LOGFONTW* pFont) const
	{
		ATLASSERT(m_hTheme != NULL);
		return ::GetThemeFont(m_hTheme, hDC, nPartID, nStateID, nPropID, pFont);
	}

	HRESULT GetThemeRect(int nPartID, int nStateID, int nPropID, LPRECT pRect) const
	{
		ATLASSERT(m_hTheme != NULL);
		return ::GetThemeRect(m_hTheme, nPartID, nStateID, nPropID, pRect);
	}

	HRESULT GetThemeMargins(HDC hDC, int nPartID, int nStateID, int nPropID, LPRECT pRect, PMARGINS pMargins) const
	{
		ATLASSERT(m_hTheme != NULL);
		return ::GetThemeMargins(m_hTheme, hDC, nPartID, nStateID, nPropID, pRect, pMargins);
	}

	HRESULT GetThemeIntList(int nPartID, int nStateID, int nPropID, INTLIST* pIntList) const
	{
		ATLASSERT(m_hTheme != NULL);
		return ::GetThemeIntList(m_hTheme, nPartID, nStateID, nPropID, pIntList);
	}

	HRESULT GetThemePropertyOrigin(int nPartID, int nStateID, int nPropID, enum PROPERTYORIGIN* pOrigin) const
	{
		ATLASSERT(m_hTheme != NULL);
		return ::GetThemePropertyOrigin(m_hTheme, nPartID, nStateID, nPropID, pOrigin);
	}

	HRESULT GetThemeFilename(int nPartID, int nStateID, int nPropID, LPWSTR pszThemeFileName, int cchMaxBuffChars) const
	{
		ATLASSERT(m_hTheme != NULL);
		return ::GetThemeFilename(m_hTheme, nPartID, nStateID, nPropID, pszThemeFileName, cchMaxBuffChars);
	}

	COLORREF GetThemeSysColor(int nColorID) const
	{
		ATLASSERT(m_hTheme != NULL);
		return ::GetThemeSysColor(m_hTheme, nColorID);
	}

	HBRUSH GetThemeSysColorBrush(int nColorID) const
	{
		ATLASSERT(m_hTheme != NULL);
		return ::GetThemeSysColorBrush(m_hTheme, nColorID);
	}

	int GetThemeSysSize(int nSizeID) const
	{
		ATLASSERT(m_hTheme != NULL);
		return ::GetThemeSysSize(m_hTheme, nSizeID);
	}

	BOOL GetThemeSysBool(int nBoolID) const
	{
		ATLASSERT(m_hTheme != NULL);
		return ::GetThemeSysBool(m_hTheme, nBoolID);
	}

	HRESULT GetThemeSysFont(int nFontID, LOGFONTW* plf) const
	{
		ATLASSERT(m_hTheme != NULL);
		return ::GetThemeSysFont(m_hTheme, nFontID, plf);
	}

	HRESULT GetThemeSysString(int nStringID, LPWSTR pszStringBuff, int cchMaxStringChars) const
	{
		ATLASSERT(m_hTheme != NULL);
		return ::GetThemeSysString(m_hTheme, nStringID, pszStringBuff, cchMaxStringChars);
	}

	HRESULT GetThemeSysInt(int nIntID, int* pnValue) const
	{
		ATLASSERT(m_hTheme != NULL);
		return ::GetThemeSysInt(m_hTheme, nIntID, pnValue);
	}

	HTHEME OpenThemeDataEx(HWND hWnd, LPCWSTR pszClassList, DWORD dwFlags)
	{
		if(!IsThemingSupported())
			return NULL;

		ATLASSERT(m_hTheme == NULL);
		m_hTheme = ::OpenThemeDataEx(hWnd, pszClassList, dwFlags);
		return m_hTheme;
	}

#if (_WIN32_WINNT >= 0x0600)
	HRESULT DrawThemeTextEx(HDC hDC, int nPartID, int nStateID, LPCWSTR pszText, int cchText, DWORD dwTextFlags, LPRECT lpRect, const DTTOPTS* pOptions)
	{
		ATLASSERT(m_hTheme != NULL);
		return ::DrawThemeTextEx(m_hTheme, hDC, nPartID, nStateID, pszText, cchText, dwTextFlags, lpRect, pOptions);
	}

	HRESULT GetThemeTransitionDuration(int nPartID, int nFromStateID, int nToStateID, int nPropID, DWORD& dwDuration)
	{
		ATLASSERT(m_hTheme != NULL);
		return ::GetThemeTransitionDuration(m_hTheme, nPartID, nFromStateID, nToStateID, nPropID, &dwDuration);
	}
#endif // (_WIN32_WINNT >= 0x0600)

#if (_WIN32_WINNT >= 0x0600)
	HRESULT GetThemeBitmap(int nPartID, int nStateID, int nPropID, ULONG uFlags, HBITMAP& hBitmap)
	{
		ATLASSERT(m_hTheme != NULL);
		return ::GetThemeBitmap(m_hTheme, nPartID, nStateID, nPropID, uFlags, &hBitmap);
	}

	HRESULT GetThemeStream(int nPartID, int nStateID, int nPropID, VOID** ppvStream, DWORD* pcbStream, HINSTANCE hInstance)
	{
		ATLASSERT(m_hTheme != NULL);
		return ::GetThemeStream(m_hTheme, nPartID, nStateID, nPropID, ppvStream, pcbStream, hInstance);
	}
#endif // (_WIN32_WINNT >= 0x0600)

#if (_WIN32_WINNT >= 0x0602)
	HRESULT GetThemeAnimationProperty(int iStoryboardId, int iTargetId, TA_PROPERTY eProperty, VOID* pvProperty, DWORD cbSize, DWORD* pcbSizeOut)
	{
		ATLASSERT(m_hTheme != NULL);
		return ::GetThemeAnimationProperty(m_hTheme, iStoryboardId, iTargetId, eProperty, pvProperty, cbSize, pcbSizeOut);
	}

	HRESULT GetThemeAnimationTransform(int iStoryboardId, int iTargetId, DWORD dwTransformIndex, TA_TRANSFORM* pTransform, DWORD cbSize, DWORD* pcbSizeOut)
	{
		ATLASSERT(m_hTheme != NULL);
		return ::GetThemeAnimationTransform(m_hTheme, iStoryboardId, iTargetId, dwTransformIndex, pTransform, cbSize, pcbSizeOut);
	}

	HRESULT GetThemeTimingFunction(int iTimingFunctionId, TA_TIMINGFUNCTION* pTimingFunction, DWORD cbSize, DWORD* pcbSizeOut)
	{
		ATLASSERT(m_hTheme != NULL);
		return ::GetThemeTimingFunction(m_hTheme, iTimingFunctionId, pTimingFunction, cbSize, pcbSizeOut);
	}
#endif // (_WIN32_WINNT >= 0x0602)
};

__declspec(selectany) int CTheme::m_nIsThemingSupported = -1;


///////////////////////////////////////////////////////////////////////////////
// CThemeImpl - theme support implementation

// Derive from this class to implement window with theme support.
// Example:
//	class CMyThemeWindow : public CWindowImpl<CMyThemeWindow>, public CThemeImpl<CMyThemeWindow>
//	{
//	...
//		BEGIN_MSG_MAP(CMyThemeWindow)
//			CHAIN_MSG_MAP(CThemeImpl<CMyThemeWindow>)
//			...
//		END_MSG_MAP()
//	...
//	};
//
// If you set theme class list, the class will automaticaly open/close/reopen theme data.


// Helper for drawing theme client edge
inline bool AtlDrawThemeClientEdge(HTHEME hTheme, HWND hWnd, HRGN hRgnUpdate = NULL, HBRUSH hBrush = NULL, int nPartID = 0, int nStateID = 0)
{
	ATLASSERT(hTheme != NULL);
	ATLASSERT(::IsWindow(hWnd));

	CWindowDC dc(hWnd);
	if(dc.IsNull())
		return false;

	// Get border size
	int cxBorder = ::GetSystemMetrics(SM_CXBORDER);
	int cyBorder = ::GetSystemMetrics(SM_CYBORDER);
	if(SUCCEEDED(::GetThemeInt(hTheme, nPartID, nStateID, TMT_SIZINGBORDERWIDTH, &cxBorder)))
		cyBorder = cxBorder;

	RECT rect = {};
	::GetWindowRect(hWnd, &rect);            

	// Remove the client edge from the update region
	int cxEdge = ::GetSystemMetrics(SM_CXEDGE);
	int cyEdge = ::GetSystemMetrics(SM_CYEDGE);
	::InflateRect(&rect, -cxEdge, -cyEdge);
	CRgn rgn;
	rgn.CreateRectRgnIndirect(&rect);
	if(rgn.IsNull())
		return false;

	if(hRgnUpdate != NULL)
		rgn.CombineRgn(hRgnUpdate, rgn, RGN_AND);

	::OffsetRect(&rect, -rect.left, -rect.top);

	::OffsetRect(&rect, cxEdge, cyEdge);
	dc.ExcludeClipRect(&rect);
	::InflateRect(&rect, cxEdge, cyEdge);

	::DrawThemeBackground(hTheme, dc, nPartID, nStateID, &rect, NULL);

	// Use background brush too, since theme border might not cover everything
	if((cxBorder < cxEdge) && (cyBorder < cyEdge))
	{
		if(hBrush == NULL)
			hBrush = (HBRUSH)::GetClassLongPtr(hWnd, GCLP_HBRBACKGROUND);

		::InflateRect(&rect, cxBorder - cxEdge, cyBorder - cyEdge);
		dc.FillRect(&rect, hBrush);
	}

	::DefWindowProc(hWnd, WM_NCPAINT, (WPARAM)rgn.m_hRgn, 0L);

	return true;
}


// Theme extended styles
#define THEME_EX_3DCLIENTEDGE		0x00000001
#define THEME_EX_THEMECLIENTEDGE	0x00000002

template <class T, class TBase = CTheme>
class CThemeImpl : public TBase
{
public:
// Data members
	LPWSTR m_lpstrThemeClassList;
	DWORD m_dwExtendedStyle;   // theme specific extended styles

// Constructor & destructor
	CThemeImpl() : m_lpstrThemeClassList(NULL), m_dwExtendedStyle(0)
	{ }

	~CThemeImpl()
	{
		delete [] m_lpstrThemeClassList;
	}

// Attributes
	bool SetThemeClassList(LPCWSTR lpstrThemeClassList)
	{
		if(m_lpstrThemeClassList != NULL)
		{
			delete [] m_lpstrThemeClassList;
			m_lpstrThemeClassList = NULL;
		}

		if(lpstrThemeClassList == NULL)
			return true;

		int cchLen = lstrlenW(lpstrThemeClassList) + 1;
		ATLTRY(m_lpstrThemeClassList = new WCHAR[cchLen]);
		if(m_lpstrThemeClassList == NULL)
			return false;

		ATL::Checked::wcscpy_s(m_lpstrThemeClassList, cchLen, lpstrThemeClassList);

		return true;
	}

	bool GetThemeClassList(LPWSTR lpstrThemeClassList, int cchListBuffer) const
	{
		int cchLen = lstrlenW(m_lpstrThemeClassList) + 1;
		if(cchListBuffer < cchLen)
			return false;

		ATL::Checked::wcscpy_s(lpstrThemeClassList, cchListBuffer, m_lpstrThemeClassList);

		return true;
	}

	LPCWSTR GetThemeClassList() const
	{
		return m_lpstrThemeClassList;
	}

	DWORD SetThemeExtendedStyle(DWORD dwExtendedStyle, DWORD dwMask = 0)
	{
		DWORD dwPrevStyle = m_dwExtendedStyle;
		if(dwMask == 0)
			m_dwExtendedStyle = dwExtendedStyle;
		else
			m_dwExtendedStyle = (m_dwExtendedStyle & ~dwMask) | (dwExtendedStyle & dwMask);

		return dwPrevStyle;
	}

	DWORD GetThemeExtendedStyle() const
	{
		return m_dwExtendedStyle;
	}

// Operations
	HTHEME OpenThemeData()
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));
		ATLASSERT(m_lpstrThemeClassList != NULL);
		if(m_lpstrThemeClassList == NULL)
			return NULL;
		this->CloseThemeData();

		return TBase::OpenThemeData(pT->m_hWnd, m_lpstrThemeClassList);
	}

	HTHEME OpenThemeData(LPCWSTR pszClassList)
	{
		if(!SetThemeClassList(pszClassList))
			return NULL;

		return OpenThemeData();
	}

	HRESULT SetWindowTheme(LPCWSTR pszSubAppName, LPCWSTR pszSubIDList)
	{
		if(!this->IsThemingSupported())
			return S_FALSE;

		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));
		return ::SetWindowTheme(pT->m_hWnd, pszSubAppName, pszSubIDList);
	}

	HTHEME GetWindowTheme() const
	{
		if(!this->IsThemingSupported())
			return NULL;

		const T* pT = static_cast<const T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));
		return ::GetWindowTheme(pT->m_hWnd);
	}

	HRESULT EnableThemeDialogTexture(DWORD dwFlags)
	{
		if(!this->IsThemingSupported())
			return S_FALSE;

		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));
		return ::EnableThemeDialogTexture(pT->m_hWnd, dwFlags);
	}

	BOOL IsThemeDialogTextureEnabled() const
	{
		if(!this->IsThemingSupported())
			return FALSE;

		const T* pT = static_cast<const T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));
		return ::IsThemeDialogTextureEnabled(pT->m_hWnd);
	}

	HRESULT DrawThemeParentBackground(HDC hDC, const RECT* pRect = NULL)
	{
		if(!this->IsThemingSupported())
			return S_FALSE;

		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));
		return ::DrawThemeParentBackground(pT->m_hWnd, hDC, pRect);
	}

#if (_WIN32_WINNT >= 0x0600)
	HRESULT SetWindowThemeAttribute(WINDOWTHEMEATTRIBUTETYPE type, PVOID pvAttribute, DWORD cbAttribute)
	{
		if(!this->IsThemingSupported())
			return S_FALSE;

		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));
		return ::SetWindowThemeAttribute(pT->m_hWnd, type, pvAttribute, cbAttribute);
	}

	HRESULT SetWindowThemeNonClientAttributes(DWORD dwAttributes, DWORD dwMask)
	{
		if(!this->IsThemingSupported())
			return S_FALSE;

		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));
		WTA_OPTIONS opt = { dwAttributes, dwMask };
		return ::SetWindowThemeAttribute(pT->m_hWnd, WTA_NONCLIENT, (PVOID)&opt, sizeof(opt));
	}

	HRESULT DrawThemeParentBackgroundEx(HDC hDC, DWORD dwFlags, const RECT* lpRect = NULL)
	{
		if(!this->IsThemingSupported())
			return S_FALSE;

		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));
		return ::DrawThemeParentBackgroundEx(pT->m_hWnd, hDC, dwFlags, lpRect);
	}
#endif // (_WIN32_WINNT >= 0x0600)

// Message map and handlers
	// Note: If you handle any of these messages in your derived class,
	// it is better to put CHAIN_MSG_MAP at the start of your message map.
	BEGIN_MSG_MAP(CThemeImpl)
		MESSAGE_HANDLER(WM_CREATE, OnCreate)
		MESSAGE_HANDLER(WM_DESTROY, OnDestroy)
		MESSAGE_HANDLER(WM_THEMECHANGED, OnThemeChanged)
		MESSAGE_HANDLER(WM_NCPAINT, OnNcPaint)
	END_MSG_MAP()

	LRESULT OnCreate(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& bHandled)
	{
		if(m_lpstrThemeClassList != NULL)
			OpenThemeData();

		bHandled = FALSE;
		return 1;
	}

	LRESULT OnDestroy(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& bHandled)
	{
		this->CloseThemeData();

		bHandled = FALSE;
		return 1;
	}

	LRESULT OnThemeChanged(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& bHandled)
	{
		this->CloseThemeData();
		if(m_lpstrThemeClassList != NULL)
			this->OpenThemeData();

		bHandled = FALSE;
		return 1;
	}

	LRESULT OnNcPaint(UINT uMsg, WPARAM wParam, LPARAM lParam, BOOL& bHandled)
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));
		LRESULT lRet = 0;
		bHandled = FALSE;
		if(this->IsThemingSupported() && ((pT->GetExStyle() & WS_EX_CLIENTEDGE) != 0))
		{
			if((m_dwExtendedStyle & THEME_EX_3DCLIENTEDGE) != 0)
			{
				lRet = ::DefWindowProc(pT->m_hWnd, uMsg, wParam, lParam);
				bHandled = TRUE;
			}
			else if((this->m_hTheme != NULL) && ((m_dwExtendedStyle & THEME_EX_THEMECLIENTEDGE) != 0))
			{
				HRGN hRgn = (wParam != 1) ? (HRGN)wParam : NULL;
				if(pT->DrawThemeClientEdge(hRgn))
					bHandled = TRUE;
			}
		}

		return lRet;
	}

// Drawing helper
	bool DrawThemeClientEdge(HRGN hRgnUpdate)
	{
		T* pT = static_cast<T*>(this);
		return AtlDrawThemeClientEdge(this->m_hTheme, pT->m_hWnd, hRgnUpdate, NULL, 0, 0);
	}
};

///////////////////////////////////////////////////////////////////////////////
// Buffered Paint and Animation

#if (_WIN32_WINNT >= 0x0600)

///////////////////////////////////////////////////////////////////////////////
// CBufferedPaintBase - Buffered Paint support for othe classes

class CBufferedPaintBase
{
public:
	static int m_nIsBufferedPaintSupported;

	CBufferedPaintBase()
	{
		if(IsBufferedPaintSupported())
			ATLVERIFY(SUCCEEDED(::BufferedPaintInit()));
	}

	~CBufferedPaintBase()
	{
		if(IsBufferedPaintSupported())
			ATLVERIFY(SUCCEEDED(::BufferedPaintUnInit()));
	}

	static bool IsBufferedPaintSupported()
	{
		if(m_nIsBufferedPaintSupported == -1)
		{
			CStaticDataInitCriticalSectionLock lock;
			if(FAILED(lock.Lock()))
			{
				ATLTRACE2(atlTraceUI, 0, _T("ERROR : Unable to lock critical section in CBufferedPaintBase::IsBufferedPaintSupported.\n"));
				ATLASSERT(FALSE);
				return false;
			}

			if(m_nIsBufferedPaintSupported == -1)
				m_nIsBufferedPaintSupported = RunTimeHelper::IsVista() ? 1 : 0;

			lock.Unlock();
		}

		ATLASSERT(m_nIsBufferedPaintSupported != -1);
		return (m_nIsBufferedPaintSupported == 1);
	}
};

__declspec(selectany) int CBufferedPaintBase::m_nIsBufferedPaintSupported = -1;


///////////////////////////////////////////////////////////////////////////////
// CBufferedPaint - support for buffered paint functions

class CBufferedPaint
{
public:
	HPAINTBUFFER m_hPaintBuffer;

	CBufferedPaint() : m_hPaintBuffer(NULL)
	{ }

	~CBufferedPaint()
	{
		ATLVERIFY(SUCCEEDED(End()));
	}

	bool IsNull() const
	{
		return (m_hPaintBuffer == NULL);
	}

	HPAINTBUFFER Begin(HDC hdcTarget, const RECT* prcTarget, BP_BUFFERFORMAT dwFormat, BP_PAINTPARAMS* pPaintParams, HDC* phdcPaint)
	{
		ATLASSERT(m_hPaintBuffer == NULL);
		m_hPaintBuffer = ::BeginBufferedPaint(hdcTarget, prcTarget, dwFormat, pPaintParams, phdcPaint);
		return m_hPaintBuffer;
	}

	HRESULT End(BOOL bUpdate = TRUE)
	{
		HRESULT hRet = S_FALSE;
		if(m_hPaintBuffer != NULL)
		{
			hRet = ::EndBufferedPaint(m_hPaintBuffer, bUpdate);
			m_hPaintBuffer = NULL;
		}
		return hRet;
	}

	HRESULT GetTargetRect(LPRECT pRect) const
	{
		ATLASSERT(m_hPaintBuffer != NULL);
		return ::GetBufferedPaintTargetRect(m_hPaintBuffer, pRect);
	}

	HDC GetTargetDC() const
	{
		ATLASSERT(m_hPaintBuffer != NULL);
		return ::GetBufferedPaintTargetDC(m_hPaintBuffer);
	}

	HDC GetPaintDC() const
	{
		ATLASSERT(m_hPaintBuffer != NULL);
		return ::GetBufferedPaintDC(m_hPaintBuffer);
	}

	HRESULT GetBits(RGBQUAD** ppbBuffer, int* pcxRow) const
	{
		ATLASSERT(m_hPaintBuffer != NULL);
		return ::GetBufferedPaintBits(m_hPaintBuffer, ppbBuffer, pcxRow);
	}

	HRESULT Clear(const RECT* pRect = NULL)
	{
		ATLASSERT(m_hPaintBuffer != NULL);
		return ::BufferedPaintClear(m_hPaintBuffer, pRect);
	}

	HRESULT SetAlpha(BYTE alpha, const RECT* pRect = NULL)
	{
		ATLASSERT(m_hPaintBuffer != NULL);
		return ::BufferedPaintSetAlpha(m_hPaintBuffer, pRect, alpha);
	}

	HRESULT MakeOpaque(const RECT* pRect = NULL)
	{
		ATLASSERT(m_hPaintBuffer != NULL);
		return ::BufferedPaintSetAlpha(m_hPaintBuffer, pRect, 255);
	}
};


///////////////////////////////////////////////////////////////////////////////
// CBufferedPaintImpl - provides buffered paint for any window

template <class T>
class ATL_NO_VTABLE CBufferedPaintImpl : public CBufferedPaintBase
{
public:
	CBufferedPaint m_BufferedPaint;
	BP_BUFFERFORMAT m_dwFormat;
	BP_PAINTPARAMS m_PaintParams;

	CBufferedPaintImpl() : m_dwFormat(BPBF_TOPDOWNDIB)
	{
		memset(&m_PaintParams, 0, sizeof(BP_PAINTPARAMS));
		m_PaintParams.cbSize = sizeof(BP_PAINTPARAMS);
	}

// Message map and handlers
	BEGIN_MSG_MAP(CBufferedPaintImpl)
		MESSAGE_HANDLER(WM_ERASEBKGND, OnEraseBackground)
		MESSAGE_HANDLER(WM_PAINT, OnPaint)
		MESSAGE_HANDLER(WM_PRINTCLIENT, OnPaint)
	END_MSG_MAP()

	LRESULT OnEraseBackground(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		return 1;   // no background needed
	}

	LRESULT OnPaint(UINT /*uMsg*/, WPARAM wParam, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		T* pT = static_cast<T*>(this);
		if(wParam != NULL)
		{
			RECT rect = {};
			pT->GetClientRect(&rect);
			pT->DoPaint((HDC)wParam, rect);
		}
		else
		{
			CPaintDC dc(pT->m_hWnd);
			pT->DoBufferedPaint(dc.m_hDC, dc.m_ps.rcPaint);
		}

		return 0;
	}

// Overrideables
	void DoBufferedPaint(CDCHandle dc, RECT& rect)
	{
		HDC hDCPaint = NULL;
		if(IsBufferedPaintSupported())
			m_BufferedPaint.Begin(dc, &rect, m_dwFormat, &m_PaintParams, &hDCPaint);

		T* pT = static_cast<T*>(this);
		if(hDCPaint != NULL)
			pT->DoPaint(hDCPaint, rect);
		else
			pT->DoPaint(dc.m_hDC, rect);

		if(IsBufferedPaintSupported())
			m_BufferedPaint.End();
	}

	void DoPaint(CDCHandle /*dc*/, RECT& /*rect*/)
	{
		// must be implemented in a derived class
		ATLASSERT(FALSE);
	}
};


///////////////////////////////////////////////////////////////////////////////
// CBufferedPaintWindowImpl - implements a window that uses buffered paint

template <class T, class TBase = ATL::CWindow, class TWinTraits = ATL::CControlWinTraits>
class ATL_NO_VTABLE CBufferedPaintWindowImpl : 
		public ATL::CWindowImpl<T, TBase, TWinTraits>, 
		public CBufferedPaintImpl< T >
{
public:
	BEGIN_MSG_MAP(CBufferedPaintWindowImpl)
		CHAIN_MSG_MAP(CBufferedPaintImpl< T >)
	END_MSG_MAP()
};


///////////////////////////////////////////////////////////////////////////////
// CBufferedAnimation - support for buffered animation

class CBufferedAnimation
{
public:
	HANIMATIONBUFFER m_hAnimationBuffer;

	CBufferedAnimation() : m_hAnimationBuffer(NULL)
	{ }

	~CBufferedAnimation()
	{
		ATLVERIFY(SUCCEEDED(End()));
	}

	bool IsNull() const
	{
		return (m_hAnimationBuffer == NULL);
	}

	HANIMATIONBUFFER Begin(HWND hWnd, HDC hDCTarget, const RECT* pRectTarget, BP_BUFFERFORMAT dwFormat, BP_PAINTPARAMS* pPaintParams, BP_ANIMATIONPARAMS* pAnimationParams, HDC* phdcFrom, HDC* phdcTo)
	{
		ATLASSERT(m_hAnimationBuffer == NULL);
		m_hAnimationBuffer = ::BeginBufferedAnimation(hWnd, hDCTarget, pRectTarget, dwFormat, pPaintParams, pAnimationParams, phdcFrom, phdcTo);
		return m_hAnimationBuffer;
	}

	HRESULT End(BOOL bUpdate = TRUE)
	{
		HRESULT hRet = S_FALSE;
		if(m_hAnimationBuffer != NULL)
		{
			hRet = ::EndBufferedAnimation(m_hAnimationBuffer, bUpdate);
			m_hAnimationBuffer = NULL;
		}
		return hRet;
	}

	static bool IsRendering(HWND hWnd, HDC hDC)
	{
		return (::BufferedPaintRenderAnimation(hWnd, hDC) != FALSE);
	}

	static HRESULT StopAllAnimations(HWND hWnd)
	{
		return ::BufferedPaintStopAllAnimations(hWnd);
	}
};


///////////////////////////////////////////////////////////////////////////////
// CBufferedAnimationImpl - provides buffered animation support for any window

// Note: You can either use m_State and m_NewState to store the state information
// for the animation change, or map your state to those data members. DoPaint()
// should only rely on the state information that is passed to it.

template <class T, class TState = DWORD_PTR>
class ATL_NO_VTABLE CBufferedAnimationImpl : public CBufferedPaintBase
{
public:
	BP_BUFFERFORMAT m_dwFormat;
	BP_PAINTPARAMS m_PaintParams;
	BP_ANIMATIONPARAMS m_AnimationParams;

	TState m_State;
	TState m_NewState;

	CBufferedAnimationImpl(TState InitialState) : m_dwFormat(BPBF_TOPDOWNDIB)
	{
		memset(&m_PaintParams, 0, sizeof(BP_PAINTPARAMS));
		m_PaintParams.cbSize = sizeof(BP_PAINTPARAMS);

		memset(&m_AnimationParams, 0, sizeof(BP_ANIMATIONPARAMS));
		m_AnimationParams.cbSize = sizeof(BP_ANIMATIONPARAMS);
		m_AnimationParams.style = BPAS_LINEAR;
		m_AnimationParams.dwDuration = 500;

		T* pT = static_cast<T*>(this);
		pT->SetState(InitialState);
		pT->SetNewState(InitialState);
	}

	DWORD GetDuration() const
	{
		return m_AnimationParams.dwDuration;
	}

	void SetDuration(DWORD dwDuration)
	{
		m_AnimationParams.dwDuration = dwDuration;
	}

	void DoAnimation(TState NewState, const RECT* pRect = NULL)
	{
		T* pT = static_cast<T*>(this);
		pT->SetNewState(NewState);

		pT->InvalidateRect(pRect, FALSE);
		pT->UpdateWindow();

		pT->SetState(NewState);
	}

// Message map and handlers
	BEGIN_MSG_MAP(CBufferedAnimationImpl)
		MESSAGE_HANDLER(WM_ERASEBKGND, OnEraseBackground)
		MESSAGE_HANDLER(WM_PAINT, OnPaint)
		MESSAGE_HANDLER(WM_PRINTCLIENT, OnPaint)
	END_MSG_MAP()

	LRESULT OnEraseBackground(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		return 1;   // no background needed
	}

	LRESULT OnPaint(UINT /*uMsg*/, WPARAM wParam, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		T* pT = static_cast<T*>(this);
		if(wParam != NULL)
		{
			RECT rect = {};
			pT->GetClientRect(&rect);
			pT->DoPaint((HDC)wParam, rect, m_NewState);
		}
		else
		{
			CPaintDC dc(pT->m_hWnd);
			pT->DoAnimationPaint(dc.m_hDC, dc.m_ps.rcPaint);
		}

		return 0;
	}

// Overrideables
	void SetState(TState State)
	{
		m_State = State;
	}

	void SetNewState(TState State)
	{
		m_NewState = State;
	}

	bool AreStatesEqual() const
	{
		return (m_State == m_NewState);
	}

	void DoAnimationPaint(CDCHandle dc, RECT& rect)
	{
		T* pT = static_cast<T*>(this);
		if(IsBufferedPaintSupported() && CBufferedAnimation::IsRendering(pT->m_hWnd, dc))
			return;

		DWORD dwDurationSave = m_AnimationParams.dwDuration;
		if(pT->AreStatesEqual())
			m_AnimationParams.dwDuration = 0;

		HDC hdcFrom = NULL, hdcTo = NULL;
		CBufferedAnimation ba;
		if(IsBufferedPaintSupported())
			ba.Begin(pT->m_hWnd, dc, &rect, m_dwFormat, &m_PaintParams, &m_AnimationParams, &hdcFrom, &hdcTo);

		if(!ba.IsNull())
		{
			if(hdcFrom != NULL)
				pT->DoPaint(hdcFrom, rect, m_State);

			if (hdcTo != NULL)
				pT->DoPaint(hdcTo, rect, m_NewState);
		}
		else
		{
			pT->DoPaint(dc.m_hDC, rect, m_NewState);
		}

		m_AnimationParams.dwDuration = dwDurationSave;
	}

	void DoPaint(CDCHandle /*dc*/, RECT& /*rect*/, TState /*State*/)
	{
		// must be implemented in a derived class
		ATLASSERT(FALSE);
	}
};


///////////////////////////////////////////////////////////////////////////////
// CBufferedAnimationWindowImpl - implements a window that uses buffered animation

template <class T, class TState = DWORD_PTR, class TBase = ATL::CWindow, class TWinTraits = ATL::CControlWinTraits>
class ATL_NO_VTABLE CBufferedAnimationWindowImpl : 
		public ATL::CWindowImpl<T, TBase, TWinTraits>, 
		public CBufferedAnimationImpl< T, TState >
{
public:
	CBufferedAnimationWindowImpl(TState InitialState) : CBufferedAnimationImpl< T, TState >(InitialState)
	{ }

	typedef CBufferedAnimationImpl< T, TState >   _baseBufferedAnimation;
	BEGIN_MSG_MAP(CBufferedAnimationWindowImpl)
		CHAIN_MSG_MAP(_baseBufferedAnimation)
	END_MSG_MAP()
};

#endif // (_WIN32_WINNT >= 0x0600)

} // namespace WTL

#endif // __ATLTHEME_H__
