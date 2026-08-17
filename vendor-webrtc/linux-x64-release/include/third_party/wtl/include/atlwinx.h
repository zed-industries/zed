// Windows Template Library - WTL version 10.0
// Copyright (C) Microsoft Corporation, WTL Team. All rights reserved.
//
// This file is a part of the Windows Template Library.
// The use and distribution terms for this software are covered by the
// Microsoft Public License (http://opensource.org/licenses/MS-PL)
// which can be found in the file MS-PL.txt at the root folder.

#ifndef __ATLWINX_H__
#define __ATLWINX_H__

#pragma once

#ifndef __ATLAPP_H__
	#error atlwinx.h requires atlapp.h to be included first
#endif

#include <atlwin.h>


///////////////////////////////////////////////////////////////////////////////
// Classes in this file:
//
// CWindowEx


/////////////////////////////////////////////////////////////////////////////
// Additional macros needed for template classes

#ifndef DECLARE_WND_CLASS_EX2
  #define DECLARE_WND_CLASS_EX2(WndClassName, EnclosingClass, style, bkgnd) \
  static ATL::CWndClassInfo& GetWndClassInfo() \
  { \
	static ATL::CWndClassInfo wc = \
	{ \
		{ sizeof(WNDCLASSEX), style, EnclosingClass::StartWindowProc, \
		  0, 0, NULL, NULL, NULL, (HBRUSH)(bkgnd + 1), NULL, WndClassName, NULL }, \
		  NULL, NULL, IDC_ARROW, TRUE, 0, _T("") \
	}; \
	return wc; \
  }
#endif // DECLARE_WND_CLASS_EX2

#ifndef DECLARE_WND_SUPERCLASS2
  #define DECLARE_WND_SUPERCLASS2(WndClassName, EnclosingClass, OrigWndClassName) \
  static ATL::CWndClassInfo& GetWndClassInfo() \
  { \
	static ATL::CWndClassInfo wc = \
	{ \
		{ sizeof(WNDCLASSEX), 0, EnclosingClass::StartWindowProc, \
		  0, 0, NULL, NULL, NULL, NULL, NULL, WndClassName, NULL }, \
		  OrigWndClassName, NULL, NULL, TRUE, 0, _T("") \
	}; \
	return wc; \
  }
#endif // DECLARE_WND_SUPERCLASS2


///////////////////////////////////////////////////////////////////////////////
// Command Chaining Macros

#define CHAIN_COMMANDS(theChainClass) \
	if(uMsg == WM_COMMAND) \
		CHAIN_MSG_MAP(theChainClass)

#define CHAIN_COMMANDS_ALT(theChainClass, msgMapID) \
	if(uMsg == WM_COMMAND) \
		CHAIN_MSG_MAP_ALT(theChainClass, msgMapID)

#define CHAIN_COMMANDS_MEMBER(theChainMember) \
	if(uMsg == WM_COMMAND) \
		CHAIN_MSG_MAP_MEMBER(theChainMember)

#define CHAIN_COMMANDS_ALT_MEMBER(theChainMember, msgMapID) \
	if(uMsg == WM_COMMAND) \
		CHAIN_MSG_MAP_ALT_MEMBER(theChainMember, msgMapID)


///////////////////////////////////////////////////////////////////////////////
// Macros for parent message map to selectively reflect control messages

// NOTE: ReflectNotifications is a member of ATL's CWindowImplRoot
//  (and overridden in 2 cases - CContainedWindowT and CAxHostWindow)
//  Since we can't modify ATL, we'll provide the needed additions
//  in a separate function (that is not a member of CWindowImplRoot)

namespace WTL
{

inline LRESULT WtlReflectNotificationsFiltered(HWND hWndParent, UINT uMsg, WPARAM wParam, LPARAM lParam, BOOL& bHandled,
                                               UINT uMsgFilter = WM_NULL, UINT_PTR idFromFilter = 0, HWND hWndChildFilter = NULL)
{
	if((uMsgFilter != WM_NULL) && (uMsgFilter != uMsg))
	{
		// The notification message doesn't match the filter.
		bHandled = FALSE;
		return 1;
	}

	HWND hWndChild = NULL;
	UINT_PTR idFrom = 0;

	switch(uMsg)
	{
	case WM_COMMAND:
		if(lParam != NULL)	// not from a menu
		{
			hWndChild = (HWND)lParam;
			idFrom = (UINT_PTR)LOWORD(wParam);
		}
		break;
	case WM_NOTIFY:
		hWndChild = ((LPNMHDR)lParam)->hwndFrom;
		idFrom = ((LPNMHDR)lParam)->idFrom;
		break;
	case WM_PARENTNOTIFY:
		switch(LOWORD(wParam))
		{
		case WM_CREATE:
		case WM_DESTROY:
			hWndChild = (HWND)lParam;
			idFrom = (UINT_PTR)HIWORD(wParam);
			break;
		default:
			hWndChild = ::GetDlgItem(hWndParent, HIWORD(wParam));
			idFrom = (UINT_PTR)::GetDlgCtrlID(hWndChild);
			break;
		}
		break;
	case WM_DRAWITEM:
		if(wParam)	// not from a menu
		{
			hWndChild = ((LPDRAWITEMSTRUCT)lParam)->hwndItem;
			idFrom = (UINT_PTR)wParam;
		}
		break;
	case WM_MEASUREITEM:
		if(wParam)	// not from a menu
		{
			hWndChild = ::GetDlgItem(hWndParent, ((LPMEASUREITEMSTRUCT)lParam)->CtlID);
			idFrom = (UINT_PTR)wParam;
		}
		break;
	case WM_COMPAREITEM:
		if(wParam)	// not from a menu
		{
			hWndChild = ((LPCOMPAREITEMSTRUCT)lParam)->hwndItem;
			idFrom = (UINT_PTR)wParam;
		}
		break;
	case WM_DELETEITEM:
		if(wParam)	// not from a menu
		{
			hWndChild = ((LPDELETEITEMSTRUCT)lParam)->hwndItem;
			idFrom = (UINT_PTR)wParam;
		}
		break;
	case WM_VKEYTOITEM:
	case WM_CHARTOITEM:
	case WM_HSCROLL:
	case WM_VSCROLL:
		hWndChild = (HWND)lParam;
		idFrom = (UINT_PTR)::GetDlgCtrlID(hWndChild);
		break;
	case WM_CTLCOLORBTN:
	case WM_CTLCOLORDLG:
	case WM_CTLCOLOREDIT:
	case WM_CTLCOLORLISTBOX:
	case WM_CTLCOLORMSGBOX:
	case WM_CTLCOLORSCROLLBAR:
	case WM_CTLCOLORSTATIC:
		hWndChild = (HWND)lParam;
		idFrom = (UINT_PTR)::GetDlgCtrlID(hWndChild);
		break;
	default:
		break;
	}

	if((hWndChild == NULL) ||
		((hWndChildFilter != NULL) && (hWndChildFilter != hWndChild)))
	{
		// Either hWndChild isn't valid, or
		// hWndChild doesn't match the filter.
		bHandled = FALSE;
		return 1;
	}

	if((idFromFilter != 0) && (idFromFilter != idFrom))
	{
		// The dialog control id doesn't match the filter.
		bHandled = FALSE;
		return 1;
	}

	ATLASSERT(::IsWindow(hWndChild));
	LRESULT lResult = ::SendMessage(hWndChild, OCM__BASE + uMsg, wParam, lParam);
	if((lResult == 0) && (uMsg >= WM_CTLCOLORMSGBOX) && (uMsg <= WM_CTLCOLORSTATIC))
	{
		// Try to prevent problems with WM_CTLCOLOR* messages when
		// the message wasn't really handled
		bHandled = FALSE;
	}

	return lResult;
}

} // namespace WTL

// Try to prevent problems with WM_CTLCOLOR* messages when
// the message wasn't really handled
#define REFLECT_NOTIFICATIONS_EX() \
{ \
	bHandled = TRUE; \
	lResult = this->ReflectNotifications(uMsg, wParam, lParam, bHandled); \
	if((lResult == 0) && (uMsg >= WM_CTLCOLORMSGBOX) && (uMsg <= WM_CTLCOLORSTATIC)) \
		bHandled = FALSE; \
	if(bHandled) \
		return TRUE; \
}

#define REFLECT_NOTIFICATIONS_MSG_FILTERED(uMsgFilter) \
	{ \
		bHandled = TRUE; \
		lResult = WTL::WtlReflectNotificationsFiltered(this->m_hWnd, uMsg, wParam, lParam, bHandled, uMsgFilter, 0, NULL); \
		if(bHandled) \
			return TRUE; \
	}

#define REFLECT_NOTIFICATIONS_ID_FILTERED(idFromFilter) \
	{ \
		bHandled = TRUE; \
		lResult = WTL::WtlReflectNotificationsFiltered(this->m_hWnd, uMsg, wParam, lParam, bHandled, WM_NULL, idFromFilter, NULL); \
		if(bHandled) \
			return TRUE; \
	}

#define REFLECT_NOTIFICATIONS_HWND_FILTERED(hWndChildFilter) \
	{ \
		bHandled = TRUE; \
		lResult = WTL::WtlReflectNotificationsFiltered(this->m_hWnd, uMsg, wParam, lParam, bHandled, WM_NULL, 0, hWndChildFilter); \
		if(bHandled) \
			return TRUE; \
	}

#define REFLECT_NOTIFICATIONS_MSG_ID_FILTERED(uMsgFilter, idFromFilter) \
	{ \
		bHandled = TRUE; \
		lResult = WTL::WtlReflectNotificationsFiltered(this->m_hWnd, uMsg, wParam, lParam, bHandled, uMsgFilter, idFromFilter, NULL); \
		if(bHandled) \
			return TRUE; \
	}

#define REFLECT_NOTIFICATIONS_MSG_HWND_FILTERED(uMsgFilter, hWndChildFilter) \
	{ \
		bHandled = TRUE; \
		lResult = WTL::WtlReflectNotificationsFiltered(this->m_hWnd, uMsg, wParam, lParam, bHandled, uMsgFilter, 0, hWndChildFilter); \
		if(bHandled) \
			return TRUE; \
	}

#define REFLECT_COMMAND(id, code) \
	if((uMsg == WM_COMMAND) && (id == LOWORD(wParam)) && (code == HIWORD(wParam))) \
	{ \
		bHandled = TRUE; \
		lResult = this->ReflectNotifications(uMsg, wParam, lParam, bHandled); \
		if(bHandled) \
			return TRUE; \
	}

#define REFLECT_COMMAND_ID(id) \
	if((uMsg == WM_COMMAND) && (id == LOWORD(wParam))) \
	{ \
		bHandled = TRUE; \
		lResult = this->ReflectNotifications(uMsg, wParam, lParam, bHandled); \
		if(bHandled) \
			return TRUE; \
	}

#define REFLECT_COMMAND_CODE(code) \
	if((uMsg == WM_COMMAND) && (code == HIWORD(wParam))) \
	{ \
		bHandled = TRUE; \
		lResult = this->ReflectNotifications(uMsg, wParam, lParam, bHandled); \
		if(bHandled) \
			return TRUE; \
	}

#define REFLECT_COMMAND_RANGE(idFirst, idLast) \
	if((uMsg == WM_COMMAND) && (LOWORD(wParam) >= idFirst) && (LOWORD(wParam) <= idLast)) \
	{ \
		bHandled = TRUE; \
		lResult = this->ReflectNotifications(uMsg, wParam, lParam, bHandled); \
		if(bHandled) \
			return TRUE; \
	}

#define REFLECT_COMMAND_RANGE_CODE(idFirst, idLast, code) \
	if((uMsg == WM_COMMAND) && (code == HIWORD(wParam)) && (LOWORD(wParam) >= idFirst) && (LOWORD(wParam) <= idLast)) \
	{ \
		bHandled = TRUE; \
		lResult = this->ReflectNotifications(uMsg, wParam, lParam, bHandled); \
		if(bHandled) \
			return TRUE; \
	}

#define REFLECT_NOTIFY(id, cd) \
	if((uMsg == WM_NOTIFY) && (id == ((LPNMHDR)lParam)->idFrom) && (cd == ((LPNMHDR)lParam)->code)) \
	{ \
		bHandled = TRUE; \
		lResult = this->ReflectNotifications(uMsg, wParam, lParam, bHandled); \
		if(bHandled) \
			return TRUE; \
	}

#define REFLECT_NOTIFY_ID(id) \
	if((uMsg == WM_NOTIFY) && (id == ((LPNMHDR)lParam)->idFrom)) \
	{ \
		bHandled = TRUE; \
		lResult = this->ReflectNotifications(uMsg, wParam, lParam, bHandled); \
		if(bHandled) \
			return TRUE; \
	}

#define REFLECT_NOTIFY_CODE(cd) \
	if((uMsg == WM_NOTIFY) && (cd == ((LPNMHDR)lParam)->code)) \
	{ \
		bHandled = TRUE; \
		lResult = this->ReflectNotifications(uMsg, wParam, lParam, bHandled); \
		if(bHandled) \
			return TRUE; \
	}

#define REFLECT_NOTIFY_RANGE(idFirst, idLast) \
	if((uMsg == WM_NOTIFY) && (((LPNMHDR)lParam)->idFrom >= idFirst) && (((LPNMHDR)lParam)->idFrom <= idLast)) \
	{ \
		bHandled = TRUE; \
		lResult = this->ReflectNotifications(uMsg, wParam, lParam, bHandled); \
		if(bHandled) \
			return TRUE; \
	}

#define REFLECT_NOTIFY_RANGE_CODE(idFirst, idLast, cd) \
	if((uMsg == WM_NOTIFY) && (cd == ((LPNMHDR)lParam)->code) && (((LPNMHDR)lParam)->idFrom >= idFirst) && (((LPNMHDR)lParam)->idFrom <= idLast)) \
	{ \
		bHandled = TRUE; \
		lResult = this->ReflectNotifications(uMsg, wParam, lParam, bHandled); \
		if(bHandled) \
			return TRUE; \
	}


///////////////////////////////////////////////////////////////////////////////
// GetClassLong/SetClassLong redefinition to avoid problems with class members

#ifdef SetClassLongPtrA
  #undef SetClassLongPtrA
  inline LONG_PTR SetClassLongPtrA(HWND hWnd, int nIndex, LONG_PTR dwNewLong)
  {
	return ::SetClassLongA(hWnd, nIndex, LONG(dwNewLong));
  }
#endif

#ifdef SetClassLongPtrW
  #undef SetClassLongPtrW
  inline LONG_PTR SetClassLongPtrW(HWND hWnd, int nIndex, LONG_PTR dwNewLong)
  {
	return ::SetClassLongW(hWnd, nIndex, LONG(dwNewLong));
  }
#endif

#ifdef GetClassLongPtrA
  #undef GetClassLongPtrA
  inline LONG_PTR GetClassLongPtrA(HWND hWnd, int nIndex)
  {
	return ::GetClassLongA(hWnd, nIndex);
  }
#endif

#ifdef GetClassLongPtrW
  #undef GetClassLongPtrW
  inline LONG_PTR GetClassLongPtrW(HWND hWnd, int nIndex)
  {
	return ::GetClassLongW(hWnd, nIndex);
  }
#endif


///////////////////////////////////////////////////////////////////////////////
// CWindowEx - extension of ATL::CWindow

namespace WTL
{

class CWindowEx : public ATL::CWindow
{
public:
	CWindowEx(HWND hWnd = NULL) : ATL::CWindow(hWnd)
	{ }

	CWindowEx& operator =(HWND hWnd)
	{
		m_hWnd = hWnd;
		return *this;
	}

	operator HWND() const
	{
		return m_hWnd;
	}

// Methods
	BOOL PrintWindow(HDC hDC, UINT uFlags = 0)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		return ::PrintWindow(m_hWnd, hDC, uFlags);
	}

	BOOL DragDetect(POINT pt)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		return ::DragDetect(m_hWnd, pt);
	}

	BOOL DragDetect()
	{
		ATLASSERT(::IsWindow(m_hWnd));

		POINT pt = {};
		::GetCursorPos(&pt);
		return ::DragDetect(m_hWnd, pt);
	}

	CWindowEx GetAncestor(UINT uFlags) const
	{
		ATLASSERT(::IsWindow(m_hWnd));
		return CWindowEx(::GetAncestor(m_hWnd, uFlags));
	}

	// Note: Does not work properly on Vista Aero and above
	BOOL AnimateWindow(DWORD dwFlags, DWORD dwTime = 200)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		return ::AnimateWindow(m_hWnd, dwTime, dwFlags);
	}

	BOOL FlashWindowEx(DWORD dwFlags, UINT uCount, DWORD dwTimeout = 0)
	{
		ATLASSERT(::IsWindow(m_hWnd));

		FLASHWINFO fi = { sizeof(FLASHWINFO) };
		fi.hwnd = m_hWnd;
		fi.dwFlags = dwFlags;
		fi.uCount = uCount;
		fi.dwTimeout = dwTimeout;
		return ::FlashWindowEx(&fi);
	}

	BOOL StopFlashWindowEx()
	{
		ATLASSERT(::IsWindow(m_hWnd));

		FLASHWINFO fi = { sizeof(FLASHWINFO) };
		fi.hwnd = m_hWnd;
		fi.dwFlags = FLASHW_STOP;
		return ::FlashWindowEx(&fi);
	}

// Class long properties
	DWORD GetClassLong(int nIndex) const
	{
		ATLASSERT(::IsWindow(m_hWnd));
		return ::GetClassLong(m_hWnd, nIndex);
	}

	DWORD SetClassLong(int nIndex, LONG dwNewLong)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		return ::SetClassLong(m_hWnd, nIndex, dwNewLong);
	}

	ULONG_PTR GetClassLongPtr(int nIndex) const
	{
		ATLASSERT(::IsWindow(m_hWnd));
		return ::GetClassLongPtr(m_hWnd, nIndex);
	}

	ULONG_PTR SetClassLongPtr(int nIndex, LONG_PTR dwNewLong)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		return ::SetClassLongPtr(m_hWnd, nIndex, dwNewLong);
	}

// Layered windows
	BOOL SetLayeredWindowAttributes(COLORREF crlKey, BYTE byteAlpha, DWORD dwFlags)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		ATLASSERT((GetExStyle() & WS_EX_LAYERED) != 0);

		return ::SetLayeredWindowAttributes(m_hWnd, crlKey, byteAlpha, dwFlags);
	}

	BOOL UpdateLayeredWindow(HDC hdcDst, LPPOINT pptDst, LPSIZE psize, HDC hdcSrc, LPPOINT pptSrc, COLORREF crlKey, BLENDFUNCTION* pblend, DWORD dwFlags)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		ATLASSERT((GetExStyle() & WS_EX_LAYERED) != 0);

		return ::UpdateLayeredWindow(m_hWnd, hdcDst, pptDst, psize, hdcSrc, pptSrc, crlKey, pblend, dwFlags);
	}

	BOOL UpdateLayeredWindow(LPPOINT pptDst = NULL)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		ATLASSERT((GetExStyle() & WS_EX_LAYERED) != 0);

		return ::UpdateLayeredWindow(m_hWnd, NULL, pptDst, NULL, NULL, NULL, CLR_NONE, NULL, 0);
	}

	BOOL GetLayeredWindowAttributes(COLORREF* pcrlKey, BYTE* pbyteAlpha, DWORD* pdwFlags) const
	{
		ATLASSERT(::IsWindow(m_hWnd));
		ATLASSERT((GetExStyle() & WS_EX_LAYERED) != 0);

		return ::GetLayeredWindowAttributes(m_hWnd, pcrlKey, pbyteAlpha, pdwFlags);
	}

// Mouse tracking
	BOOL StartTrackMouseLeave()
	{
		ATLASSERT(::IsWindow(m_hWnd));

		TRACKMOUSEEVENT tme = {};
		tme.cbSize = sizeof(TRACKMOUSEEVENT);
		tme.dwFlags = TME_LEAVE;
		tme.hwndTrack = m_hWnd;
		return ::TrackMouseEvent(&tme);
	}

	BOOL StartTrackMouse(DWORD dwFlags, DWORD dwHoverTime = HOVER_DEFAULT)
	{
		ATLASSERT(::IsWindow(m_hWnd));

		TRACKMOUSEEVENT tme = {};
		tme.cbSize = sizeof(TRACKMOUSEEVENT);
		tme.dwFlags = dwFlags;
		tme.hwndTrack = m_hWnd;
		tme.dwHoverTime = dwHoverTime;
		return ::TrackMouseEvent(&tme);
	}

	BOOL CancelTrackMouse(DWORD dwType)
	{
		ATLASSERT(::IsWindow(m_hWnd));

		TRACKMOUSEEVENT tme = {};
		tme.cbSize = sizeof(TRACKMOUSEEVENT);
		tme.dwFlags = TME_CANCEL | dwType;
		tme.hwndTrack = m_hWnd;
		return ::TrackMouseEvent(&tme);
	}

// CString support
#ifdef __ATLSTR_H__
	int GetWindowText(ATL::CString& strText) const
	{
		int nLength = GetWindowTextLength();
		LPTSTR pszText = strText.GetBuffer(nLength + 1);
		nLength = ::GetWindowText(m_hWnd, pszText, nLength + 1);
		strText.ReleaseBuffer(nLength);

		return nLength;
	}

	UINT GetDlgItemText(int nID, ATL::CString& strText) const
	{
		ATLASSERT(::IsWindow(m_hWnd));

		HWND hItem = GetDlgItem(nID);
		if(hItem != NULL)
		{
			int nLength = ::GetWindowTextLength(hItem);
			LPTSTR pszText = strText.GetBuffer(nLength + 1);
			nLength = ::GetWindowText(hItem, pszText, nLength + 1);
			strText.ReleaseBuffer(nLength);

			return nLength;
		}
		else
		{
			strText.Empty();

			return 0;
		}
	}
#endif // __ATLSTR_H__
};

} // namespace WTL

#endif // __ATLWINX_H__
