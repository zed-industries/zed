// Windows Template Library - WTL version 10.0
// Copyright (C) Microsoft Corporation, WTL Team. All rights reserved.
//
// This file is a part of the Windows Template Library.
// The use and distribution terms for this software are covered by the
// Microsoft Public License (http://opensource.org/licenses/MS-PL)
// which can be found in the file MS-PL.txt at the root folder.

#ifndef __ATLSCRL_H__
#define __ATLSCRL_H__

#pragma once

#ifndef __ATLAPP_H__
	#error atlscrl.h requires atlapp.h to be included first
#endif

#ifndef __ATLWIN_H__
	#error atlscrl.h requires atlwin.h to be included first
#endif


///////////////////////////////////////////////////////////////////////////////
// Classes in this file:
//
// CScrollImpl<T>
// CScrollWindowImpl<T, TBase, TWinTraits>
// CMapScrollImpl<T>
// CMapScrollWindowImpl<T, TBase, TWinTraits>
// CFSBWindowT<TBase>
// CZoomScrollImpl<T>
// CZoomScrollWindowImpl<T, TBase, TWinTraits>
// CScrollContainerImpl<T, TBase, TWinTraits>
// CScrollContainer

namespace WTL
{

///////////////////////////////////////////////////////////////////////////////
// CScrollImpl - Provides scrolling support to any window

// Scroll extended styles
#define SCRL_SCROLLCHILDREN	0x00000001
#define SCRL_ERASEBACKGROUND	0x00000002
#define SCRL_NOTHUMBTRACKING	0x00000004
#define SCRL_SMOOTHSCROLL	0x00000008
#define SCRL_DISABLENOSCROLLV	0x00000010
#define SCRL_DISABLENOSCROLLH	0x00000020
#define SCRL_DISABLENOSCROLL	(SCRL_DISABLENOSCROLLV | SCRL_DISABLENOSCROLLH)


template <class T>
class CScrollImpl
{
public:
	enum { uSCROLL_FLAGS = SW_INVALIDATE };

	POINT m_ptOffset;
	SIZE m_sizeAll;
	SIZE m_sizeLine;
	SIZE m_sizePage;
	SIZE m_sizeClient;
	int m_zDelta;              // current wheel value
	int m_nWheelLines;         // number of lines to scroll on wheel
	int m_zHDelta;              // current horizontal wheel value
	int m_nHWheelChars;         // number of chars to scroll on horizontal wheel
	UINT m_uScrollFlags;
	DWORD m_dwExtendedStyle;   // scroll specific extended styles

// Constructor
	CScrollImpl() : m_zDelta(0), m_nWheelLines(3), 
			m_zHDelta(0), m_nHWheelChars(3), 
			m_uScrollFlags(0U), m_dwExtendedStyle(0)
	{
		m_ptOffset.x = 0;
		m_ptOffset.y = 0;
		m_sizeAll.cx = 0;
		m_sizeAll.cy = 0;
		m_sizePage.cx = 0;
		m_sizePage.cy = 0;
		m_sizeLine.cx = 0;
		m_sizeLine.cy = 0;
		m_sizeClient.cx = 0;
		m_sizeClient.cy = 0;

		SetScrollExtendedStyle(SCRL_SCROLLCHILDREN | SCRL_ERASEBACKGROUND);
	}

// Attributes & Operations
	DWORD GetScrollExtendedStyle() const
	{
		return m_dwExtendedStyle;
	}

	DWORD SetScrollExtendedStyle(DWORD dwExtendedStyle, DWORD dwMask = 0)
	{
		DWORD dwPrevStyle = m_dwExtendedStyle;
		if(dwMask == 0)
			m_dwExtendedStyle = dwExtendedStyle;
		else
			m_dwExtendedStyle = (m_dwExtendedStyle & ~dwMask) | (dwExtendedStyle & dwMask);
		// cache scroll flags
		T* pT = static_cast<T*>(this);
		(void)pT;   // avoid level 4 warning
		m_uScrollFlags = pT->uSCROLL_FLAGS | (IsScrollingChildren() ? SW_SCROLLCHILDREN : 0) | (IsErasingBackground() ? SW_ERASE : 0);
		m_uScrollFlags |= (IsSmoothScroll() ? SW_SMOOTHSCROLL : 0);
		return dwPrevStyle;
	}

	// offset operations
	void SetScrollOffset(int x, int y, BOOL bRedraw = TRUE)
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));

		pT->AdjustScrollOffset(x, y);

		int dx = m_ptOffset.x - x;
		int dy = m_ptOffset.y - y;
		m_ptOffset.x = x;
		m_ptOffset.y = y;

		// block: set horizontal scroll bar
		{
			SCROLLINFO si = { sizeof(SCROLLINFO) };
			si.fMask = SIF_POS;
			if((m_dwExtendedStyle & SCRL_DISABLENOSCROLLH) != 0)
				si.fMask |= SIF_DISABLENOSCROLL;
			si.nPos = m_ptOffset.x;
			pT->SetScrollInfo(SB_HORZ, &si, bRedraw);
		}

		// block: set vertical scroll bar
		{
			SCROLLINFO si = { sizeof(SCROLLINFO) };
			si.fMask = SIF_POS;
			if((m_dwExtendedStyle & SCRL_DISABLENOSCROLLV) != 0)
				si.fMask |= SIF_DISABLENOSCROLL;
			si.nPos = m_ptOffset.y;
			pT->SetScrollInfo(SB_VERT, &si, bRedraw);
		}

		// Move all children if needed
		if(IsScrollingChildren() && ((dx != 0) || (dy != 0)))
		{
			for(HWND hWndChild = ::GetWindow(pT->m_hWnd, GW_CHILD); hWndChild != NULL; hWndChild = ::GetWindow(hWndChild, GW_HWNDNEXT))
			{
				RECT rect = {};
				::GetWindowRect(hWndChild, &rect);
				::MapWindowPoints(NULL, pT->m_hWnd, (LPPOINT)&rect, 1);
				::SetWindowPos(hWndChild, NULL, rect.left + dx, rect.top + dy, 0, 0, SWP_NOZORDER | SWP_NOSIZE | SWP_NOACTIVATE);
			}
		}

		if(bRedraw)
			pT->Invalidate();
	}

	void SetScrollOffset(POINT ptOffset, BOOL bRedraw = TRUE)
	{
		SetScrollOffset(ptOffset.x, ptOffset.y, bRedraw);
	}

	void GetScrollOffset(POINT& ptOffset) const
	{
		ptOffset = m_ptOffset;
	}

	// size operations
	void SetScrollSize(int cx, int cy, BOOL bRedraw = TRUE, bool bResetOffset = true)
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));

		m_sizeAll.cx = cx;
		m_sizeAll.cy = cy;

		int x = 0;
		int y = 0;
		if(!bResetOffset)
		{
			x = m_ptOffset.x;
			y = m_ptOffset.y;
			pT->AdjustScrollOffset(x, y);
		}

		int dx = m_ptOffset.x - x;
		int dy = m_ptOffset.y - y;
		m_ptOffset.x = x;
		m_ptOffset.y = y;

		// block: set horizontal scroll bar
		{
			SCROLLINFO si = { sizeof(SCROLLINFO) };
			si.fMask = SIF_PAGE | SIF_RANGE | SIF_POS;
			if((m_dwExtendedStyle & SCRL_DISABLENOSCROLLH) != 0)
				si.fMask |= SIF_DISABLENOSCROLL;
			si.nMin = 0;
			si.nMax = m_sizeAll.cx - 1;
			si.nPage = m_sizeClient.cx;
			si.nPos = m_ptOffset.x;
			pT->SetScrollInfo(SB_HORZ, &si, bRedraw);
		}

		// block: set vertical scroll bar
		{
			SCROLLINFO si = { sizeof(SCROLLINFO) };
			si.fMask = SIF_PAGE | SIF_RANGE | SIF_POS;
			if((m_dwExtendedStyle & SCRL_DISABLENOSCROLLV) != 0)
				si.fMask |= SIF_DISABLENOSCROLL;
			si.nMin = 0;
			si.nMax = m_sizeAll.cy - 1;
			si.nPage = m_sizeClient.cy;
			si.nPos = m_ptOffset.y;
			pT->SetScrollInfo(SB_VERT, &si, bRedraw);
		}

		// Move all children if needed
		if(IsScrollingChildren() && ((dx != 0) || (dy != 0)))
		{
			for(HWND hWndChild = ::GetWindow(pT->m_hWnd, GW_CHILD); hWndChild != NULL; hWndChild = ::GetWindow(hWndChild, GW_HWNDNEXT))
			{
				RECT rect = {};
				::GetWindowRect(hWndChild, &rect);
				::MapWindowPoints(NULL, pT->m_hWnd, (LPPOINT)&rect, 1);
				::SetWindowPos(hWndChild, NULL, rect.left + dx, rect.top + dy, 0, 0, SWP_NOZORDER | SWP_NOSIZE | SWP_NOACTIVATE);
			}
		}

		SetScrollLine(0, 0);
		SetScrollPage(0, 0);

		if(bRedraw)
			pT->Invalidate();
	}

	void SetScrollSize(SIZE size, BOOL bRedraw = TRUE, bool bResetOffset = true)
	{
		SetScrollSize(size.cx, size.cy, bRedraw, bResetOffset);
	}

	void GetScrollSize(SIZE& sizeWnd) const
	{
		sizeWnd = m_sizeAll;
	}

	// line operations
	void SetScrollLine(int cxLine, int cyLine)
	{
		ATLASSERT((cxLine >= 0) && (cyLine >= 0));
		ATLASSERT((m_sizeAll.cx != 0) && (m_sizeAll.cy != 0));

		m_sizeLine.cx = T::CalcLineOrPage(cxLine, m_sizeAll.cx, 100);
		m_sizeLine.cy = T::CalcLineOrPage(cyLine, m_sizeAll.cy, 100);
	}

	void SetScrollLine(SIZE sizeLine)
	{
		SetScrollLine(sizeLine.cx, sizeLine.cy);
	}

	void GetScrollLine(SIZE& sizeLine) const
	{
		sizeLine = m_sizeLine;
	}

	// page operations
	void SetScrollPage(int cxPage, int cyPage)
	{
		ATLASSERT((cxPage >= 0) && (cyPage >= 0));
		ATLASSERT((m_sizeAll.cx != 0) && (m_sizeAll.cy != 0));

		m_sizePage.cx = T::CalcLineOrPage(cxPage, m_sizeAll.cx, 10);
		m_sizePage.cy = T::CalcLineOrPage(cyPage, m_sizeAll.cy, 10);
	}

	void SetScrollPage(SIZE sizePage)
	{
		SetScrollPage(sizePage.cx, sizePage.cy);
	}

	void GetScrollPage(SIZE& sizePage) const
	{
		sizePage = m_sizePage;
	}

	// commands
	void ScrollLineDown()
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));
		pT->DoScroll(SB_VERT, SB_LINEDOWN, (int&)m_ptOffset.y, m_sizeAll.cy, m_sizePage.cy, m_sizeLine.cy);
	}

	void ScrollLineUp()
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));
		pT->DoScroll(SB_VERT, SB_LINEUP, (int&)m_ptOffset.y, m_sizeAll.cy, m_sizePage.cy, m_sizeLine.cy);
	}

	void ScrollPageDown()
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));
		pT->DoScroll(SB_VERT, SB_PAGEDOWN, (int&)m_ptOffset.y, m_sizeAll.cy, m_sizePage.cy, m_sizeLine.cy);
	}

	void ScrollPageUp()
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));
		pT->DoScroll(SB_VERT, SB_PAGEUP, (int&)m_ptOffset.y, m_sizeAll.cy, m_sizePage.cy, m_sizeLine.cy);
	}

	void ScrollTop()
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));
		pT->DoScroll(SB_VERT, SB_TOP, (int&)m_ptOffset.y, m_sizeAll.cy, m_sizePage.cy, m_sizeLine.cy);
	}

	void ScrollBottom()
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));
		pT->DoScroll(SB_VERT, SB_BOTTOM, (int&)m_ptOffset.y, m_sizeAll.cy, m_sizePage.cy, m_sizeLine.cy);
	}

	void ScrollLineRight()
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));
		pT->DoScroll(SB_HORZ, SB_LINEDOWN, (int&)m_ptOffset.x, m_sizeAll.cx, m_sizePage.cx, m_sizeLine.cx);
	}

	void ScrollLineLeft()
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));
		pT->DoScroll(SB_HORZ, SB_LINEUP, (int&)m_ptOffset.x, m_sizeAll.cx, m_sizePage.cx, m_sizeLine.cx);
	}

	void ScrollPageRight()
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));
		pT->DoScroll(SB_HORZ, SB_PAGEDOWN, (int&)m_ptOffset.x, m_sizeAll.cx, m_sizePage.cx, m_sizeLine.cx);
	}

	void ScrollPageLeft()
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));
		pT->DoScroll(SB_HORZ, SB_PAGEUP, (int&)m_ptOffset.x, m_sizeAll.cx, m_sizePage.cx, m_sizeLine.cx);
	}

	void ScrollAllLeft()
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));
		pT->DoScroll(SB_HORZ, SB_TOP, (int&)m_ptOffset.x, m_sizeAll.cx, m_sizePage.cx, m_sizeLine.cx);
	}

	void ScrollAllRight()
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));
		pT->DoScroll(SB_HORZ, SB_BOTTOM, (int&)m_ptOffset.x, m_sizeAll.cx, m_sizePage.cx, m_sizeLine.cx);
	}

	// scroll to make point/view/window visible
	void ScrollToView(POINT pt)
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));
		RECT rect = { pt.x, pt.y, pt.x, pt.y };
		pT->ScrollToView(rect);
	}

	void ScrollToView(RECT& rect)
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));

		RECT rcClient = {};
		pT->GetClientRect(&rcClient);

		int x = m_ptOffset.x;
		if(rect.left < m_ptOffset.x)
			x = rect.left;
		else if(rect.right > (m_ptOffset.x + rcClient.right))
			x = rect.right - rcClient.right;

		int y = m_ptOffset.y;
		if(rect.top < m_ptOffset.y)
			y = rect.top;
		else if(rect.bottom > (m_ptOffset.y + rcClient.bottom))
			y = rect.bottom - rcClient.bottom;

		SetScrollOffset(x, y);
	}

	void ScrollToView(HWND hWnd)
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));

		RECT rect = {};
		::GetWindowRect(hWnd, &rect);
		::OffsetRect(&rect, m_ptOffset.x, m_ptOffset.y);
		::MapWindowPoints(NULL, pT->m_hWnd, (LPPOINT)&rect, 2);
		ScrollToView(rect);
	}

	BEGIN_MSG_MAP(CScrollImpl)
		MESSAGE_HANDLER(WM_CREATE, OnCreate)
		MESSAGE_HANDLER(WM_VSCROLL, OnVScroll)
		MESSAGE_HANDLER(WM_HSCROLL, OnHScroll)
		MESSAGE_HANDLER(WM_MOUSEWHEEL, OnMouseWheel)
		MESSAGE_HANDLER(WM_MOUSEHWHEEL, OnMouseHWheel)
		MESSAGE_HANDLER(WM_SETTINGCHANGE, OnSettingChange)
		MESSAGE_HANDLER(WM_SIZE, OnSize)
		MESSAGE_HANDLER(WM_PAINT, OnPaint)
		MESSAGE_HANDLER(WM_PRINTCLIENT, OnPaint)
	// standard scroll commands
	ALT_MSG_MAP(1)
		COMMAND_ID_HANDLER(ID_SCROLL_UP, OnScrollUp)
		COMMAND_ID_HANDLER(ID_SCROLL_DOWN, OnScrollDown)
		COMMAND_ID_HANDLER(ID_SCROLL_PAGE_UP, OnScrollPageUp)
		COMMAND_ID_HANDLER(ID_SCROLL_PAGE_DOWN, OnScrollPageDown)
		COMMAND_ID_HANDLER(ID_SCROLL_TOP, OnScrollTop)
		COMMAND_ID_HANDLER(ID_SCROLL_BOTTOM, OnScrollBottom)
		COMMAND_ID_HANDLER(ID_SCROLL_LEFT, OnScrollLeft)
		COMMAND_ID_HANDLER(ID_SCROLL_RIGHT, OnScrollRight)
		COMMAND_ID_HANDLER(ID_SCROLL_PAGE_LEFT, OnScrollPageLeft)
		COMMAND_ID_HANDLER(ID_SCROLL_PAGE_RIGHT, OnScrollPageRight)
		COMMAND_ID_HANDLER(ID_SCROLL_ALL_LEFT, OnScrollAllLeft)
		COMMAND_ID_HANDLER(ID_SCROLL_ALL_RIGHT, OnScrollAllRight)
	END_MSG_MAP()

	LRESULT OnCreate(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& bHandled)
	{
		T* pT = static_cast<T*>(this);
		pT->GetSystemSettings();

		bHandled = FALSE;
		return 1;
	}

	LRESULT OnVScroll(UINT /*uMsg*/, WPARAM wParam, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));
		pT->DoScroll(SB_VERT, (int)(short)LOWORD(wParam), (int&)m_ptOffset.y, m_sizeAll.cy, m_sizePage.cy, m_sizeLine.cy);
		return 0;
	}

	LRESULT OnHScroll(UINT /*uMsg*/, WPARAM wParam, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));
		pT->DoScroll(SB_HORZ, (int)(short)LOWORD(wParam), (int&)m_ptOffset.x, m_sizeAll.cx, m_sizePage.cx, m_sizeLine.cx);
		return 0;
	}

	LRESULT OnMouseWheel(UINT /*uMsg*/, WPARAM wParam, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));

		int zDelta = (int)GET_WHEEL_DELTA_WPARAM(wParam);
		int nScrollCode = (m_nWheelLines == WHEEL_PAGESCROLL) ? ((zDelta > 0) ? SB_PAGEUP : SB_PAGEDOWN) : ((zDelta > 0) ? SB_LINEUP : SB_LINEDOWN);
		m_zDelta += zDelta;   // cumulative
		int zTotal = (m_nWheelLines == WHEEL_PAGESCROLL) ? abs(m_zDelta) : abs(m_zDelta) * m_nWheelLines;
		if(m_sizeAll.cy > m_sizeClient.cy)
		{
			for(int i = 0; i < zTotal; i += WHEEL_DELTA)
			{
				pT->DoScroll(SB_VERT, nScrollCode, (int&)m_ptOffset.y, m_sizeAll.cy, m_sizePage.cy, m_sizeLine.cy);
				pT->UpdateWindow();
			}
		}
		else if(m_sizeAll.cx > m_sizeClient.cx)   // can't scroll vertically, scroll horizontally
		{
			for(int i = 0; i < zTotal; i += WHEEL_DELTA)
			{
				pT->DoScroll(SB_HORZ, nScrollCode, (int&)m_ptOffset.x, m_sizeAll.cx, m_sizePage.cx, m_sizeLine.cx);
				pT->UpdateWindow();
			}
		}
		m_zDelta %= WHEEL_DELTA;

		return 0;
	}

	LRESULT OnMouseHWheel(UINT /*uMsg*/, WPARAM wParam, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));

		int zDelta = (int)GET_WHEEL_DELTA_WPARAM(wParam);
		int nScrollCode = (m_nHWheelChars == WHEEL_PAGESCROLL) ? ((zDelta > 0) ? SB_PAGERIGHT : SB_PAGELEFT) : ((zDelta > 0) ? SB_LINERIGHT : SB_LINELEFT);
		m_zHDelta += zDelta;   // cumulative
		int zTotal = (m_nHWheelChars == WHEEL_PAGESCROLL) ? abs(m_zHDelta) : abs(m_zHDelta) * m_nHWheelChars;
		if(m_sizeAll.cx > m_sizeClient.cx)
		{
			for(int i = 0; i < zTotal; i += WHEEL_DELTA)
			{
				pT->DoScroll(SB_HORZ, nScrollCode, (int&)m_ptOffset.x, m_sizeAll.cx, m_sizePage.cx, m_sizeLine.cx);
				pT->UpdateWindow();
			}
		}
		m_zHDelta %= WHEEL_DELTA;

		return 0;
	}

	LRESULT OnSettingChange(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		GetSystemSettings();
		return 0;
	}

	LRESULT OnSize(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM lParam, BOOL& bHandled)
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));

		pT->DoSize(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam));

		bHandled = FALSE;
		return 1;
	}

	LRESULT OnPaint(UINT /*uMsg*/, WPARAM wParam, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));
		if(wParam != NULL)
		{
			CDCHandle dc = (HDC)wParam;
			POINT ptViewportOrg = { 0, 0 };
			dc.SetViewportOrg(-m_ptOffset.x, -m_ptOffset.y, &ptViewportOrg);
			pT->DoPaint(dc);
			dc.SetViewportOrg(ptViewportOrg);
		}
		else
		{
			CPaintDC dc(pT->m_hWnd);
			dc.SetViewportOrg(-m_ptOffset.x, -m_ptOffset.y);
			pT->DoPaint(dc.m_hDC);
		}
		return 0;
	}

	// scrolling handlers
	LRESULT OnScrollUp(WORD /*wNotifyCode*/, WORD /*wID*/, HWND /*hWndCtl*/, BOOL& /*bHandled*/)
	{
		ScrollLineUp();
		return 0;
	}

	LRESULT OnScrollDown(WORD /*wNotifyCode*/, WORD /*wID*/, HWND /*hWndCtl*/, BOOL& /*bHandled*/)
	{
		ScrollLineDown();
		return 0;
	}

	LRESULT OnScrollPageUp(WORD /*wNotifyCode*/, WORD /*wID*/, HWND /*hWndCtl*/, BOOL& /*bHandled*/)
	{
		ScrollPageUp();
		return 0;
	}

	LRESULT OnScrollPageDown(WORD /*wNotifyCode*/, WORD /*wID*/, HWND /*hWndCtl*/, BOOL& /*bHandled*/)
	{
		ScrollPageDown();
		return 0;
	}

	LRESULT OnScrollTop(WORD /*wNotifyCode*/, WORD /*wID*/, HWND /*hWndCtl*/, BOOL& /*bHandled*/)
	{
		ScrollTop();
		return 0;
	}

	LRESULT OnScrollBottom(WORD /*wNotifyCode*/, WORD /*wID*/, HWND /*hWndCtl*/, BOOL& /*bHandled*/)
	{
		ScrollBottom();
		return 0;
	}

	LRESULT OnScrollLeft(WORD /*wNotifyCode*/, WORD /*wID*/, HWND /*hWndCtl*/, BOOL& /*bHandled*/)
	{
		ScrollLineLeft();
		return 0;
	}

	LRESULT OnScrollRight(WORD /*wNotifyCode*/, WORD /*wID*/, HWND /*hWndCtl*/, BOOL& /*bHandled*/)
	{
		ScrollLineRight();
		return 0;
	}

	LRESULT OnScrollPageLeft(WORD /*wNotifyCode*/, WORD /*wID*/, HWND /*hWndCtl*/, BOOL& /*bHandled*/)
	{
		ScrollPageLeft();
		return 0;
	}

	LRESULT OnScrollPageRight(WORD /*wNotifyCode*/, WORD /*wID*/, HWND /*hWndCtl*/, BOOL& /*bHandled*/)
	{
		ScrollPageRight();
		return 0;
	}

	LRESULT OnScrollAllLeft(WORD /*wNotifyCode*/, WORD /*wID*/, HWND /*hWndCtl*/, BOOL& /*bHandled*/)
	{
		ScrollAllLeft();
		return 0;
	}

	LRESULT OnScrollAllRight(WORD /*wNotifyCode*/, WORD /*wID*/, HWND /*hWndCtl*/, BOOL& /*bHandled*/)
	{
		ScrollAllRight();
		return 0;
	}

// Overrideables
	void DoPaint(CDCHandle /*dc*/)
	{
		// must be implemented in a derived class
		ATLASSERT(FALSE);
	}

// Implementation
	void DoSize(int cx, int cy)
	{
		m_sizeClient.cx = cx;
		m_sizeClient.cy = cy;

		T* pT = static_cast<T*>(this);

		// block: set horizontal scroll bar
		{
			SCROLLINFO si = { sizeof(SCROLLINFO) };
			si.fMask = SIF_PAGE | SIF_RANGE | SIF_POS;
			si.nMin = 0;
			si.nMax = m_sizeAll.cx - 1;
			if((m_dwExtendedStyle & SCRL_DISABLENOSCROLLH) != 0)
				si.fMask |= SIF_DISABLENOSCROLL;
			si.nPage = m_sizeClient.cx;
			si.nPos = m_ptOffset.x;
			pT->SetScrollInfo(SB_HORZ, &si, TRUE);
		}

		// block: set vertical scroll bar
		{
			SCROLLINFO si = { sizeof(SCROLLINFO) };
			si.fMask = SIF_PAGE | SIF_RANGE | SIF_POS;
			si.nMin = 0;
			si.nMax = m_sizeAll.cy - 1;
			if((m_dwExtendedStyle & SCRL_DISABLENOSCROLLV) != 0)
				si.fMask |= SIF_DISABLENOSCROLL;
			si.nPage = m_sizeClient.cy;
			si.nPos = m_ptOffset.y;
			pT->SetScrollInfo(SB_VERT, &si, TRUE);
		}

		int x = m_ptOffset.x;
		int y = m_ptOffset.y;
		if(pT->AdjustScrollOffset(x, y))
		{
			// Children will be moved in SetScrollOffset, if needed
			pT->ScrollWindowEx(m_ptOffset.x - x, m_ptOffset.y - y, (m_uScrollFlags & ~SCRL_SCROLLCHILDREN));
			SetScrollOffset(x, y, FALSE);
		}
	}

	void DoScroll(int nType, int nScrollCode, int& cxyOffset, int cxySizeAll, int cxySizePage, int cxySizeLine)
	{
		T* pT = static_cast<T*>(this);
		RECT rect = {};
		pT->GetClientRect(&rect);
		int cxyClient = (nType == SB_VERT) ? rect.bottom : rect.right;
		int cxyMax = cxySizeAll - cxyClient;

		if(cxyMax < 0)   // can't scroll, client area is bigger
			return;

		bool bUpdate = true;
		int cxyScroll = 0;

		switch(nScrollCode)
		{
		case SB_TOP:		// top or all left
			cxyScroll = cxyOffset;
			cxyOffset = 0;
			break;
		case SB_BOTTOM:		// bottom or all right
			cxyScroll = cxyOffset - cxyMax;
			cxyOffset = cxyMax;
			break;
		case SB_LINEUP:		// line up or line left
			if(cxyOffset >= cxySizeLine)
			{
				cxyScroll = cxySizeLine;
				cxyOffset -= cxySizeLine;
			}
			else
			{
				cxyScroll = cxyOffset;
				cxyOffset = 0;
			}
			break;
		case SB_LINEDOWN:	// line down or line right
			if(cxyOffset < cxyMax - cxySizeLine)
			{
				cxyScroll = -cxySizeLine;
				cxyOffset += cxySizeLine;
			}
			else
			{
				cxyScroll = cxyOffset - cxyMax;
				cxyOffset = cxyMax;
			}
			break;
		case SB_PAGEUP:		// page up or page left
			if(cxyOffset >= cxySizePage)
			{
				cxyScroll = cxySizePage;
				cxyOffset -= cxySizePage;
			}
			else
			{
				cxyScroll = cxyOffset;
				cxyOffset = 0;
			}
			break;
		case SB_PAGEDOWN:	// page down or page right
			if(cxyOffset < cxyMax - cxySizePage)
			{
				cxyScroll = -cxySizePage;
				cxyOffset += cxySizePage;
			}
			else
			{
				cxyScroll = cxyOffset - cxyMax;
				cxyOffset = cxyMax;
			}
			break;
		case SB_THUMBTRACK:
			if(IsNoThumbTracking())
				break;
			// else fall through
		case SB_THUMBPOSITION:
			{
				SCROLLINFO si = { sizeof(SCROLLINFO), SIF_TRACKPOS };
				if(pT->GetScrollInfo(nType, &si))
				{
					cxyScroll = cxyOffset - si.nTrackPos;
					cxyOffset = si.nTrackPos;
				}
			}
			break;
		case SB_ENDSCROLL:
		default:
			bUpdate = false;
			break;
		}

		if(bUpdate && (cxyScroll != 0))
		{
			pT->SetScrollPos(nType, cxyOffset, TRUE);
			if(nType == SB_VERT)
				pT->ScrollWindowEx(0, cxyScroll, m_uScrollFlags);
			else
				pT->ScrollWindowEx(cxyScroll, 0, m_uScrollFlags);
		}
	}

	static int CalcLineOrPage(int nVal, int nMax, int nDiv)
	{
		if(nVal == 0)
		{
			nVal = nMax / nDiv;
			if(nVal < 1)
				nVal = 1;
		}
		else if(nVal > nMax)
		{
			nVal = nMax;
		}

		return nVal;
	}

	bool AdjustScrollOffset(int& x, int& y)
	{
		int xOld = x;
		int yOld = y;

		int cxMax = m_sizeAll.cx - m_sizeClient.cx;
		if(x > cxMax)
			x = (cxMax >= 0) ? cxMax : 0;
		else if(x < 0)
			x = 0;

		int cyMax = m_sizeAll.cy - m_sizeClient.cy;
		if(y > cyMax)
			y = (cyMax >= 0) ? cyMax : 0;
		else if(y < 0)
			y = 0;

		return ((x != xOld) || (y != yOld));
	}

	void GetSystemSettings()
	{
		::SystemParametersInfo(SPI_GETWHEELSCROLLLINES, 0, &m_nWheelLines, 0);

#ifndef SPI_GETWHEELSCROLLCHARS
		const UINT SPI_GETWHEELSCROLLCHARS = 0x006C;
#endif
		::SystemParametersInfo(SPI_GETWHEELSCROLLCHARS, 0, &m_nHWheelChars, 0);
	}

	bool IsScrollingChildren() const
	{
		return (m_dwExtendedStyle & SCRL_SCROLLCHILDREN) != 0;
	}

	bool IsErasingBackground() const
	{
		return (m_dwExtendedStyle & SCRL_ERASEBACKGROUND) != 0;
	}

	bool IsNoThumbTracking() const
	{
		return (m_dwExtendedStyle & SCRL_NOTHUMBTRACKING) != 0;
	}

	bool IsSmoothScroll() const
	{
		return (m_dwExtendedStyle & SCRL_SMOOTHSCROLL) != 0;
	}
};


///////////////////////////////////////////////////////////////////////////////
// CScrollWindowImpl - Implements a scrollable window

template <class T, class TBase = ATL::CWindow, class TWinTraits = ATL::CControlWinTraits>
class ATL_NO_VTABLE CScrollWindowImpl : public ATL::CWindowImpl<T, TBase, TWinTraits>, public CScrollImpl< T >
{
public:
	BOOL SubclassWindow(HWND hWnd)
	{
		BOOL bRet = ATL::CWindowImpl< T, TBase, TWinTraits >::SubclassWindow(hWnd);
		if(bRet != FALSE)
		{
			T* pT = static_cast<T*>(this);
			pT->GetSystemSettings();

			RECT rect = {};
			this->GetClientRect(&rect);
			pT->DoSize(rect.right, rect.bottom);
		}

		return bRet;
	}

	BEGIN_MSG_MAP(CScrollWindowImpl)
		MESSAGE_HANDLER(WM_VSCROLL, CScrollImpl< T >::OnVScroll)
		MESSAGE_HANDLER(WM_HSCROLL, CScrollImpl< T >::OnHScroll)
		MESSAGE_HANDLER(WM_MOUSEWHEEL, CScrollImpl< T >::OnMouseWheel)
		MESSAGE_HANDLER(WM_MOUSEHWHEEL, CScrollImpl< T >::OnMouseHWheel)
		MESSAGE_HANDLER(WM_SETTINGCHANGE, CScrollImpl< T >::OnSettingChange)
		MESSAGE_HANDLER(WM_SIZE, CScrollImpl< T >::OnSize)
		MESSAGE_HANDLER(WM_PAINT, CScrollImpl< T >::OnPaint)
		MESSAGE_HANDLER(WM_PRINTCLIENT, CScrollImpl< T >::OnPaint)
	ALT_MSG_MAP(1)
		COMMAND_ID_HANDLER(ID_SCROLL_UP, CScrollImpl< T >::OnScrollUp)
		COMMAND_ID_HANDLER(ID_SCROLL_DOWN, CScrollImpl< T >::OnScrollDown)
		COMMAND_ID_HANDLER(ID_SCROLL_PAGE_UP, CScrollImpl< T >::OnScrollPageUp)
		COMMAND_ID_HANDLER(ID_SCROLL_PAGE_DOWN, CScrollImpl< T >::OnScrollPageDown)
		COMMAND_ID_HANDLER(ID_SCROLL_TOP, CScrollImpl< T >::OnScrollTop)
		COMMAND_ID_HANDLER(ID_SCROLL_BOTTOM, CScrollImpl< T >::OnScrollBottom)
		COMMAND_ID_HANDLER(ID_SCROLL_LEFT, CScrollImpl< T >::OnScrollLeft)
		COMMAND_ID_HANDLER(ID_SCROLL_RIGHT, CScrollImpl< T >::OnScrollRight)
		COMMAND_ID_HANDLER(ID_SCROLL_PAGE_LEFT, CScrollImpl< T >::OnScrollPageLeft)
		COMMAND_ID_HANDLER(ID_SCROLL_PAGE_RIGHT, CScrollImpl< T >::OnScrollPageRight)
		COMMAND_ID_HANDLER(ID_SCROLL_ALL_LEFT, CScrollImpl< T >::OnScrollAllLeft)
		COMMAND_ID_HANDLER(ID_SCROLL_ALL_RIGHT, CScrollImpl< T >::OnScrollAllRight)
	END_MSG_MAP()
};


///////////////////////////////////////////////////////////////////////////////
// CMapScrollImpl - Provides mapping and scrolling support to any window

template <class T>
class CMapScrollImpl : public CScrollImpl< T >
{
public:
	int m_nMapMode;
	RECT m_rectLogAll;
	SIZE m_sizeLogLine;
	SIZE m_sizeLogPage;

// Constructor
	CMapScrollImpl() : m_nMapMode(MM_TEXT)
	{
		::SetRectEmpty(&m_rectLogAll);
		m_sizeLogPage.cx = 0;
		m_sizeLogPage.cy = 0;
		m_sizeLogLine.cx = 0;
		m_sizeLogLine.cy = 0;
	}

// Attributes & Operations
	// mapping mode operations
	void SetScrollMapMode(int nMapMode)
	{
		ATLASSERT((nMapMode >= MM_MIN) && (nMapMode <= MM_MAX_FIXEDSCALE));
		m_nMapMode = nMapMode;
	}

	int GetScrollMapMode() const
	{
		ATLASSERT((m_nMapMode >= MM_MIN) && (m_nMapMode <= MM_MAX_FIXEDSCALE));
		return m_nMapMode;
	}

	// offset operations
	void SetScrollOffset(int x, int y, BOOL bRedraw = TRUE)
	{
		ATLASSERT((m_nMapMode >= MM_MIN) && (m_nMapMode <= MM_MAX_FIXEDSCALE));
		POINT ptOff = { x, y };
		// block: convert logical to device units
		{
			CWindowDC dc(NULL);
			dc.SetMapMode(m_nMapMode);
			dc.LPtoDP(&ptOff);
		}
		CScrollImpl< T >::SetScrollOffset(ptOff, bRedraw);
	}

	void SetScrollOffset(POINT ptOffset, BOOL bRedraw = TRUE)
	{
		SetScrollOffset(ptOffset.x, ptOffset.y, bRedraw);
	}

	void GetScrollOffset(POINT& ptOffset) const
	{
		ATLASSERT((m_nMapMode >= MM_MIN) && (m_nMapMode <= MM_MAX_FIXEDSCALE));
		ptOffset = this->m_ptOffset;
		// block: convert device to logical units
		{
			CWindowDC dc(NULL);
			dc.SetMapMode(m_nMapMode);
			dc.DPtoLP(&ptOffset);
		}
	}

	// size operations
	void SetScrollSize(int xMin, int yMin, int xMax, int yMax, BOOL bRedraw = TRUE, bool bResetOffset = true)
	{
		ATLASSERT((xMax > xMin) && (yMax > yMin));
		ATLASSERT((m_nMapMode >= MM_MIN) && (m_nMapMode <= MM_MAX_FIXEDSCALE));

		::SetRect(&m_rectLogAll, xMin, yMin, xMax, yMax);

		SIZE sizeAll = {};
		sizeAll.cx = xMax - xMin + 1;
		sizeAll.cy = yMax - yMin + 1;
		// block: convert logical to device units
		{
			CWindowDC dc(NULL);
			dc.SetMapMode(m_nMapMode);
			dc.LPtoDP(&sizeAll);
		}
		CScrollImpl< T >::SetScrollSize(sizeAll, bRedraw, bResetOffset);
		SetScrollLine(0, 0);
		SetScrollPage(0, 0);
	}

	void SetScrollSize(RECT& rcScroll, BOOL bRedraw = TRUE, bool bResetOffset = true)
	{
		SetScrollSize(rcScroll.left, rcScroll.top, rcScroll.right, rcScroll.bottom, bRedraw, bResetOffset);
	}

	void SetScrollSize(int cx, int cy, BOOL bRedraw = TRUE, bool bResetOffset = true)
	{
		SetScrollSize(0, 0, cx, cy, bRedraw, bResetOffset);
	}

	void SetScrollSize(SIZE size, BOOL bRedraw = TRUE, bool bResetOffset = true)
	{
		SetScrollSize(0, 0, size.cx, size.cy, bRedraw, bResetOffset);
	}

	void GetScrollSize(RECT& rcScroll) const
	{
		ATLASSERT((m_nMapMode >= MM_MIN) && (m_nMapMode <= MM_MAX_FIXEDSCALE));
		rcScroll = m_rectLogAll;
	}

	// line operations
	void SetScrollLine(int cxLine, int cyLine)
	{
		ATLASSERT((cxLine >= 0) && (cyLine >= 0));
		ATLASSERT((m_nMapMode >= MM_MIN) && (m_nMapMode <= MM_MAX_FIXEDSCALE));

		m_sizeLogLine.cx = cxLine;
		m_sizeLogLine.cy = cyLine;
		SIZE sizeLine = m_sizeLogLine;
		// block: convert logical to device units
		{
			CWindowDC dc(NULL);
			dc.SetMapMode(m_nMapMode);
			dc.LPtoDP(&sizeLine);
		}
		CScrollImpl< T >::SetScrollLine(sizeLine);
	}

	void SetScrollLine(SIZE sizeLine)
	{
		SetScrollLine(sizeLine.cx, sizeLine.cy);
	}

	void GetScrollLine(SIZE& sizeLine) const
	{
		ATLASSERT((m_nMapMode >= MM_MIN) && (m_nMapMode <= MM_MAX_FIXEDSCALE));
		sizeLine = m_sizeLogLine;
	}

	// page operations
	void SetScrollPage(int cxPage, int cyPage)
	{
		ATLASSERT((cxPage >= 0) && (cyPage >= 0));
		ATLASSERT((m_nMapMode >= MM_MIN) && (m_nMapMode <= MM_MAX_FIXEDSCALE));

		m_sizeLogPage.cx = cxPage;
		m_sizeLogPage.cy = cyPage;
		SIZE sizePage = m_sizeLogPage;
		// block: convert logical to device units
		{
			CWindowDC dc(NULL);
			dc.SetMapMode(m_nMapMode);
			dc.LPtoDP(&sizePage);
		}
		CScrollImpl< T >::SetScrollPage(sizePage);
	}

	void SetScrollPage(SIZE sizePage)
	{
		SetScrollPage(sizePage.cx, sizePage.cy);
	}

	void GetScrollPage(SIZE& sizePage) const
	{
		ATLASSERT((m_nMapMode >= MM_MIN) && (m_nMapMode <= MM_MAX_FIXEDSCALE));
		sizePage = m_sizeLogPage;
	}

	BEGIN_MSG_MAP(CMapScrollImpl)
		MESSAGE_HANDLER(WM_VSCROLL, CScrollImpl< T >::OnVScroll)
		MESSAGE_HANDLER(WM_HSCROLL, CScrollImpl< T >::OnHScroll)
		MESSAGE_HANDLER(WM_MOUSEWHEEL, CScrollImpl< T >::OnMouseWheel)
		MESSAGE_HANDLER(WM_MOUSEHWHEEL, CScrollImpl< T >::OnMouseHWheel)
		MESSAGE_HANDLER(WM_SETTINGCHANGE, CScrollImpl< T >::OnSettingChange)
		MESSAGE_HANDLER(WM_SIZE, CScrollImpl< T >::OnSize)
		MESSAGE_HANDLER(WM_PAINT, OnPaint)
		MESSAGE_HANDLER(WM_PRINTCLIENT, OnPaint)
	ALT_MSG_MAP(1)
		COMMAND_ID_HANDLER(ID_SCROLL_UP, CScrollImpl< T >::OnScrollUp)
		COMMAND_ID_HANDLER(ID_SCROLL_DOWN, CScrollImpl< T >::OnScrollDown)
		COMMAND_ID_HANDLER(ID_SCROLL_PAGE_UP, CScrollImpl< T >::OnScrollPageUp)
		COMMAND_ID_HANDLER(ID_SCROLL_PAGE_DOWN, CScrollImpl< T >::OnScrollPageDown)
		COMMAND_ID_HANDLER(ID_SCROLL_TOP, CScrollImpl< T >::OnScrollTop)
		COMMAND_ID_HANDLER(ID_SCROLL_BOTTOM, CScrollImpl< T >::OnScrollBottom)
		COMMAND_ID_HANDLER(ID_SCROLL_LEFT, CScrollImpl< T >::OnScrollLeft)
		COMMAND_ID_HANDLER(ID_SCROLL_RIGHT, CScrollImpl< T >::OnScrollRight)
		COMMAND_ID_HANDLER(ID_SCROLL_PAGE_LEFT, CScrollImpl< T >::OnScrollPageLeft)
		COMMAND_ID_HANDLER(ID_SCROLL_PAGE_RIGHT, CScrollImpl< T >::OnScrollPageRight)
		COMMAND_ID_HANDLER(ID_SCROLL_ALL_LEFT, CScrollImpl< T >::OnScrollAllLeft)
		COMMAND_ID_HANDLER(ID_SCROLL_ALL_RIGHT, CScrollImpl< T >::OnScrollAllRight)
	END_MSG_MAP()

	LRESULT OnPaint(UINT /*uMsg*/, WPARAM wParam, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));
		if(wParam != NULL)
		{
			CDCHandle dc = (HDC)wParam;
			int nMapModeSav = dc.GetMapMode();
			dc.SetMapMode(m_nMapMode);
			POINT ptViewportOrg = { 0, 0 };
			if(m_nMapMode == MM_TEXT)
				dc.SetViewportOrg(-this->m_ptOffset.x, -this->m_ptOffset.y, &ptViewportOrg);
			else
				dc.SetViewportOrg(-this->m_ptOffset.x, -this->m_ptOffset.y + this->m_sizeAll.cy, &ptViewportOrg);
			POINT ptWindowOrg = { 0, 0 };
			dc.SetWindowOrg(m_rectLogAll.left, m_rectLogAll.top, &ptWindowOrg);

			pT->DoPaint(dc);

			dc.SetMapMode(nMapModeSav);
			dc.SetViewportOrg(ptViewportOrg);
			dc.SetWindowOrg(ptWindowOrg);
		}
		else
		{
			CPaintDC dc(pT->m_hWnd);
			dc.SetMapMode(m_nMapMode);
			if(m_nMapMode == MM_TEXT)
				dc.SetViewportOrg(-this->m_ptOffset.x, -this->m_ptOffset.y);
			else
				dc.SetViewportOrg(-this->m_ptOffset.x, -this->m_ptOffset.y + this->m_sizeAll.cy);
			dc.SetWindowOrg(m_rectLogAll.left, m_rectLogAll.top);
			pT->DoPaint(dc.m_hDC);
		}
		return 0;
	}
};


///////////////////////////////////////////////////////////////////////////////
// CMapScrollWindowImpl - Implements scrolling window with mapping

template <class T, class TBase = ATL::CWindow, class TWinTraits = ATL::CControlWinTraits>
class ATL_NO_VTABLE CMapScrollWindowImpl : public ATL::CWindowImpl< T, TBase, TWinTraits >, public CMapScrollImpl< T >
{
public:
	BOOL SubclassWindow(HWND hWnd)
	{
		BOOL bRet = ATL::CWindowImpl< T, TBase, TWinTraits >::SubclassWindow(hWnd);
		if(bRet != FALSE)
		{
			T* pT = static_cast<T*>(this);
			pT->GetSystemSettings();

			RECT rect = {};
			this->GetClientRect(&rect);
			pT->DoSize(rect.right, rect.bottom);
		}

		return bRet;
	}

	BEGIN_MSG_MAP(CMapScrollWindowImpl)
		MESSAGE_HANDLER(WM_VSCROLL, CScrollImpl< T >::OnVScroll)
		MESSAGE_HANDLER(WM_HSCROLL, CScrollImpl< T >::OnHScroll)
		MESSAGE_HANDLER(WM_MOUSEWHEEL, CScrollImpl< T >::OnMouseWheel)
		MESSAGE_HANDLER(WM_MOUSEHWHEEL, CScrollImpl< T >::OnMouseHWheel)
		MESSAGE_HANDLER(WM_SETTINGCHANGE, CScrollImpl< T >::OnSettingChange)
		MESSAGE_HANDLER(WM_SIZE, CScrollImpl< T >::OnSize)
		MESSAGE_HANDLER(WM_PAINT, CMapScrollImpl< T >::OnPaint)
		MESSAGE_HANDLER(WM_PRINTCLIENT, CMapScrollImpl< T >::OnPaint)
	ALT_MSG_MAP(1)
		COMMAND_ID_HANDLER(ID_SCROLL_UP, CScrollImpl< T >::OnScrollUp)
		COMMAND_ID_HANDLER(ID_SCROLL_DOWN, CScrollImpl< T >::OnScrollDown)
		COMMAND_ID_HANDLER(ID_SCROLL_PAGE_UP, CScrollImpl< T >::OnScrollPageUp)
		COMMAND_ID_HANDLER(ID_SCROLL_PAGE_DOWN, CScrollImpl< T >::OnScrollPageDown)
		COMMAND_ID_HANDLER(ID_SCROLL_TOP, CScrollImpl< T >::OnScrollTop)
		COMMAND_ID_HANDLER(ID_SCROLL_BOTTOM, CScrollImpl< T >::OnScrollBottom)
		COMMAND_ID_HANDLER(ID_SCROLL_LEFT, CScrollImpl< T >::OnScrollLeft)
		COMMAND_ID_HANDLER(ID_SCROLL_RIGHT, CScrollImpl< T >::OnScrollRight)
		COMMAND_ID_HANDLER(ID_SCROLL_PAGE_LEFT, CScrollImpl< T >::OnScrollPageLeft)
		COMMAND_ID_HANDLER(ID_SCROLL_PAGE_RIGHT, CScrollImpl< T >::OnScrollPageRight)
		COMMAND_ID_HANDLER(ID_SCROLL_ALL_LEFT, CScrollImpl< T >::OnScrollAllLeft)
		COMMAND_ID_HANDLER(ID_SCROLL_ALL_RIGHT, CScrollImpl< T >::OnScrollAllRight)
	END_MSG_MAP()
};


///////////////////////////////////////////////////////////////////////////////
// CFSBWindow - Use as a base instead of CWindow to get flat scroll bar support

#ifdef __ATLCTRLS_H__

template <class TBase = ATL::CWindow>
class CFSBWindowT : public TBase, public CFlatScrollBarImpl<CFSBWindowT< TBase > >
{
public:
// Constructors
	CFSBWindowT(HWND hWnd = NULL) : TBase(hWnd)
	{ }

	CFSBWindowT< TBase >& operator =(HWND hWnd)
	{
		this->m_hWnd = hWnd;
		return *this;
	}

// CWindow overrides that use flat scroll bar API
// (only those methods that are used by scroll window classes)
	int SetScrollPos(int nBar, int nPos, BOOL bRedraw = TRUE)
	{
		ATLASSERT(::IsWindow(this->m_hWnd));
		return this->FlatSB_SetScrollPos(nBar, nPos, bRedraw);
	}

	BOOL GetScrollInfo(int nBar, LPSCROLLINFO lpScrollInfo)
	{
		ATLASSERT(::IsWindow(this->m_hWnd));
		return this->FlatSB_GetScrollInfo(nBar, lpScrollInfo);
	}

	BOOL SetScrollInfo(int nBar, LPSCROLLINFO lpScrollInfo, BOOL bRedraw = TRUE)
	{
		ATLASSERT(::IsWindow(this->m_hWnd));
		return this->FlatSB_SetScrollInfo(nBar, lpScrollInfo, bRedraw);
	}
};

typedef CFSBWindowT<ATL::CWindow>   CFSBWindow;

#endif // __ATLCTRLS_H__


///////////////////////////////////////////////////////////////////////////////
// CZoomScrollImpl - Provides zooming and scrolling support to any window

// The zoom modes that can be set with the SetZoomMode method
enum
{
	ZOOMMODE_OFF, 
	ZOOMMODE_IN,   // If left mouse button is clicked or dragged, zoom in on point clicked or rectangle dragged.
	ZOOMMODE_OUT   // If left mouse button clicked, zoom out on point clicked.
};

// Notification to parent that zoom scale changed as a result of user mouse action.
#define ZSN_ZOOMCHANGED	(NM_FIRST - 50) 

template <class T>
class CZoomScrollImpl : public CScrollImpl< T >
{
public:
	enum { m_cxyMinZoomRect = 12 };   // min rect size to zoom in on rect.

	struct _ChildPlacement
	{
		HWND hWnd;
		int x;
		int y;
		int cx;
		int cy;

		bool operator ==(const _ChildPlacement& cp) const { return (memcmp(this, &cp, sizeof(_ChildPlacement)) == 0); }
	};

// Data members
	SIZE m_sizeLogAll;		
	SIZE m_sizeLogLine;	
	SIZE m_sizeLogPage;
	float m_fZoomScale;
	float m_fZoomScaleMin;
	float m_fZoomScaleMax;
	float m_fZoomDelta;   // Used in ZOOMMODE_IN and ZOOMMODE_OUT on left-button click.
	int m_nZoomMode;		
	RECT m_rcTrack;
	bool m_bTracking;

	bool m_bZoomChildren;
	ATL::CSimpleArray<_ChildPlacement> m_arrChildren;

// Constructor
	CZoomScrollImpl(): m_fZoomScale(1.0f), m_fZoomScaleMin(0.1f), m_fZoomScaleMax(100.0f), m_fZoomDelta(0.5f), 
	                   m_nZoomMode(ZOOMMODE_OFF), m_bTracking(false), m_bZoomChildren(false)
	{
		m_sizeLogAll.cx = 0;
		m_sizeLogAll.cy = 0;
		m_sizeLogPage.cx = 0;
		m_sizeLogPage.cy = 0;
		m_sizeLogLine.cx = 0;
		m_sizeLogLine.cy = 0;
		::SetRectEmpty(&m_rcTrack);
	}

// Attributes & Operations
	// size operations
	void SetScrollSize(int cxLog, int cyLog, BOOL bRedraw = TRUE, bool bResetOffset = true)
	{
		ATLASSERT((cxLog >= 0) && (cyLog >= 0));

		// Set up the defaults
		if((cxLog == 0) && (cyLog == 0))
		{
			cxLog = 1;
			cyLog = 1;
		}

		m_sizeLogAll.cx = cxLog;
		m_sizeLogAll.cy = cyLog;
		SIZE sizeAll = {};
		sizeAll.cx = (int)((float)m_sizeLogAll.cx * m_fZoomScale);
		sizeAll.cy = (int)((float)m_sizeLogAll.cy * m_fZoomScale);

		CScrollImpl< T >::SetScrollSize(sizeAll, bRedraw, bResetOffset);
	}

	void SetScrollSize(SIZE sizeLog, BOOL bRedraw = TRUE, bool bResetOffset = true)
	{
		SetScrollSize(sizeLog.cx, sizeLog.cy, bRedraw, bResetOffset);
	}

	void GetScrollSize(SIZE& sizeLog) const
	{
		sizeLog = m_sizeLogAll;
	}

	// line operations
	void SetScrollLine(int cxLogLine, int cyLogLine)
	{
		ATLASSERT((cxLogLine >= 0) && (cyLogLine >= 0));

		m_sizeLogLine.cx = cxLogLine;
		m_sizeLogLine.cy = cyLogLine;

		SIZE sizeLine = {};
		sizeLine.cx = (int)((float)m_sizeLogLine.cx * m_fZoomScale);
		sizeLine.cy = (int)((float)m_sizeLogLine.cy * m_fZoomScale);
		CScrollImpl< T >::SetScrollLine(sizeLine);
	}

	void SetScrollLine(SIZE sizeLogLine)
	{
		SetScrollLine(sizeLogLine.cx, sizeLogLine.cy);
	}

	void GetScrollLine(SIZE& sizeLogLine) const
	{
		sizeLogLine = m_sizeLogLine;
	}

	// page operations
	void SetScrollPage(int cxLogPage, int cyLogPage)
	{
		ATLASSERT((cxLogPage >= 0) && (cyLogPage >= 0));

		m_sizeLogPage.cx = cxLogPage;
		m_sizeLogPage.cy = cyLogPage;

		SIZE sizePage = {};
		sizePage.cx = (int)((float)m_sizeLogPage.cx * m_fZoomScale);
		sizePage.cy = (int)((float)m_sizeLogPage.cy * m_fZoomScale);

		CScrollImpl< T >::SetScrollPage(sizePage);
	}

	void SetScrollPage(SIZE sizeLogPage)
	{
		SetScrollPage(sizeLogPage.cx, sizeLogPage.cy);
	}

	void GetScrollPage(SIZE& sizeLogPage) const
	{
		sizeLogPage = m_sizeLogPage;
	}

	void SetZoomScale(float fZoomScale)
	{
		ATLASSERT(fZoomScale > 0.0f);
		if(fZoomScale <= 0.0f)
			return;

		m_fZoomScale = fZoomScale;
		if(m_fZoomScale < m_fZoomScaleMin)
			m_fZoomScale = m_fZoomScaleMin;
		else if(m_fZoomScale > m_fZoomScaleMax)
			m_fZoomScale = m_fZoomScaleMax;
	}

	float GetZoomScale() const
	{
		return m_fZoomScale;
	}

	void SetZoomScaleMin(float fZoomScaleMin)
	{
		ATLASSERT(fZoomScaleMin > 0.0f);
		ATLASSERT(fZoomScaleMin <= m_fZoomScaleMax);

		m_fZoomScaleMin = fZoomScaleMin;
	}

	float GetZoomScaleMin() const
	{
		return m_fZoomScaleMin;
	}

	void SetZoomScaleMax(float fZoomScaleMax)
	{
		ATLASSERT(fZoomScaleMax > 0.0f);
		ATLASSERT(m_fZoomScaleMin <= fZoomScaleMax);

		m_fZoomScaleMax = fZoomScaleMax;
	}

	float GetZoomScaleMax() const
	{
		return m_fZoomScaleMax;
	}

	void SetZoomDelta(float fZoomDelta)
	{
		ATLASSERT(fZoomDelta >= 0.0f);

		if(fZoomDelta >= 0.0f)
			m_fZoomDelta = fZoomDelta;
	}

	float GetZoomDelta() const
	{
		return m_fZoomDelta;
	}

	void SetZoomMode(int nZoomMode)
	{
		m_nZoomMode = nZoomMode;
	}

	int GetZoomMode() const
	{
		return m_nZoomMode;
	}

	void SetZoomChildren(bool bEnable = true)
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));

		m_bZoomChildren = bEnable;

		m_arrChildren.RemoveAll();
		if(m_bZoomChildren)
		{
			for(HWND hWndChild = ::GetWindow(pT->m_hWnd, GW_CHILD); hWndChild != NULL; hWndChild = ::GetWindow(hWndChild, GW_HWNDNEXT))
			{
				RECT rect = {};
				::GetWindowRect(hWndChild, &rect);
				::MapWindowPoints(NULL, pT->m_hWnd, (LPPOINT)&rect, 2);

				_ChildPlacement cp = {};
				cp.hWnd = hWndChild;
				cp.x = rect.left;
				cp.y = rect.top;
				cp.cx = rect.right - rect.left;
				cp.cy = rect.bottom - rect.top;
				m_arrChildren.Add(cp);
			}
		}
	}

	bool GetZoomChildren() const
	{
		return m_bZoomChildren;
	}

	void Zoom(int x, int y, float fZoomScale)
	{
		if(fZoomScale <= 0.0f)
			return;

		if(fZoomScale < m_fZoomScaleMin)
			fZoomScale = m_fZoomScaleMin;
		else if(fZoomScale > m_fZoomScaleMax)
			fZoomScale = m_fZoomScaleMax;

		T* pT = static_cast<T*>(this);
		POINT pt = { x, y };
		if(!pT->PtInDevRect(pt))
			return;

		pT->ViewDPtoLP(&pt);
		pT->Zoom(fZoomScale, false);
		pT->CenterOnLogicalPoint(pt);
	}

	void Zoom(POINT pt, float fZoomScale)
	{
		T* pT = static_cast<T*>(this);
		pT->Zoom(pt.x, pt.y, fZoomScale);
	}

	void Zoom(RECT& rc)
	{
		T* pT = static_cast<T*>(this);
		RECT rcZoom = rc;
		pT->NormalizeRect(rcZoom);
		SIZE size = { rcZoom.right - rcZoom.left, rcZoom.bottom - rcZoom.top };
		POINT pt = { rcZoom.left + size.cx / 2, rcZoom.top + size.cy / 2 };
		if((size.cx < m_cxyMinZoomRect) || (size.cy < m_cxyMinZoomRect))
		{
			pT->Zoom(pt, m_fZoomScale + m_fZoomDelta);
			return;
		}

		ATLASSERT((size.cx > 0) && (size.cy > 0));
		
		float fScaleH = (float)(this->m_sizeClient.cx  + 1) / (float)size.cx;
		float fScaleV = (float)(this->m_sizeClient.cy + 1) / (float)size.cy;
		float fZoomScale = __min(fScaleH, fScaleV) * m_fZoomScale;
		pT->Zoom(pt, fZoomScale);		
	}

	void Zoom(float fZoomScale, bool bCenter = true)
	{
		if(fZoomScale <= 0.0f)
			return;

		if(fZoomScale < m_fZoomScaleMin)
			fZoomScale = m_fZoomScaleMin;
		else if(fZoomScale > m_fZoomScaleMax)
			fZoomScale = m_fZoomScaleMax;

		T* pT = static_cast<T*>(this);
		POINT pt = { 0, 0 };
		if(bCenter)
		{
			RECT rcClient = {};
			::GetClientRect(pT->m_hWnd, &rcClient);
			pt.x = rcClient.right / 2;
			pt.y = rcClient.bottom / 2;
			pT->ViewDPtoLP(&pt);
		}

		// Modify the Viewport extent
		SIZE sizeAll = {};
		sizeAll.cx = (int)((float)m_sizeLogAll.cx * fZoomScale);
		sizeAll.cy = (int)((float)m_sizeLogAll.cy * fZoomScale);
		
		// Update scroll bars and window
		CScrollImpl< T >::SetScrollSize(sizeAll);

		// Zoom all children if needed
		if(m_bZoomChildren && (m_fZoomScale != fZoomScale))
		{
			for(int i = 0; i < m_arrChildren.GetSize(); i++)
			{
				ATLASSERT(::IsWindow(m_arrChildren[i].hWnd));

				::SetWindowPos(m_arrChildren[i].hWnd, NULL, 
					(int)((float)m_arrChildren[i].x * fZoomScale + 0.5f), 
					(int)((float)m_arrChildren[i].y * fZoomScale + 0.5f), 
					(int)((float)m_arrChildren[i].cx * fZoomScale + 0.5f), 
					(int)((float)m_arrChildren[i].cy * fZoomScale + 0.5f), 
					SWP_NOZORDER | SWP_NOACTIVATE);
			}
		}

		// Set new zoom scale
		m_fZoomScale = fZoomScale;

		if(bCenter)
			pT->CenterOnLogicalPoint(pt);
	}

	void ZoomIn(bool bCenter = true)
	{
		T* pT = static_cast<T*>(this);
		pT->Zoom(m_fZoomScale + m_fZoomDelta, bCenter);
	}

	void ZoomOut(bool bCenter = true)
	{
		T* pT = static_cast<T*>(this);
		pT->Zoom(m_fZoomScale - m_fZoomDelta, bCenter);
	}

	void ZoomDefault(bool bCenter = true)
	{
		T* pT = static_cast<T*>(this);
		pT->Zoom(1.0f, bCenter);
	}

	// Helper functions
	void PrepareDC(CDCHandle dc)
	{
		ATLASSERT((this->m_sizeAll.cx >= 0) && (this->m_sizeAll.cy >= 0));
		dc.SetMapMode(MM_ANISOTROPIC);
		dc.SetWindowExt(this->m_sizeLogAll);
		dc.SetViewportExt(this->m_sizeAll);
		dc.SetViewportOrg(-this->m_ptOffset.x, -this->m_ptOffset.y);
	}

	void ViewDPtoLP(LPPOINT lpPoints, int nCount = 1)
	{
		ATLASSERT(lpPoints);
		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));

		CWindowDC dc(pT->m_hWnd);
		pT->PrepareDC(dc.m_hDC);
		dc.DPtoLP(lpPoints, nCount);
	}

	void ViewLPtoDP(LPPOINT lpPoints, int nCount = 1)
	{
		ATLASSERT(lpPoints);
		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));
	
		CWindowDC dc(pT->m_hWnd);
		pT->PrepareDC(dc.m_hDC);
		dc.LPtoDP(lpPoints, nCount);
	}

	void ClientToDevice(POINT &pt)
	{
		pt.x += this->m_ptOffset.x;
		pt.y += this->m_ptOffset.y;
	}	 

	void DeviceToClient(POINT &pt)
	{
		pt.x -= this->m_ptOffset.x;
		pt.y -= this->m_ptOffset.y;
	}

	void CenterOnPoint(POINT pt)
	{
		T* pT = static_cast<T*>(this);
		RECT rect = {};
		pT->GetClientRect(&rect);

		int xOfs = pt.x - (rect.right / 2) + this->m_ptOffset.x;
		if(xOfs < 0)
		{
			xOfs = 0;
		}
		else 
		{
			int xMax = __max((int)(this->m_sizeAll.cx - rect.right), 0);
			if(xOfs > xMax)
				xOfs = xMax;
		}
		
		int yOfs = pt.y - (rect.bottom / 2) + this->m_ptOffset.y;
		if(yOfs < 0)
		{
			yOfs = 0;
		}
		else 
		{
			int yMax = __max((int)(this->m_sizeAll.cy - rect.bottom), 0);
			if(yOfs > yMax)
				yOfs = yMax;
		}

		CScrollImpl< T >::SetScrollOffset(xOfs, yOfs);
	}

	void CenterOnLogicalPoint(POINT ptLog)
	{
		T* pT = static_cast<T*>(this);
		pT->ViewLPtoDP(&ptLog);
		pT->DeviceToClient(ptLog);
		pT->CenterOnPoint(ptLog);
	}

	BOOL PtInDevRect(POINT pt)
	{
		RECT rc = { 0, 0, this->m_sizeAll.cx, this->m_sizeAll.cy };
		::OffsetRect(&rc, -this->m_ptOffset.x, -this->m_ptOffset.y);
		return ::PtInRect(&rc, pt);
	}

	void NormalizeRect(RECT& rc)
	{
		if(rc.left > rc.right) 
		{
			int r = rc.right;
			rc.right = rc.left;
			rc.left = r;
		}

		if(rc.top > rc.bottom)
		{
			int b = rc.bottom;
			rc.bottom = rc.top;
			rc.top = b;
		}
	}

	void DrawTrackRect()
	{
		T* pT = static_cast<T*>(this);
		const SIZE sizeLines = { 2, 2 };
		RECT rc = m_rcTrack;
		pT->NormalizeRect(rc);
		if(!::IsRectEmpty(&rc))
		{
			CClientDC dc(pT->m_hWnd);
			dc.DrawDragRect(&rc, sizeLines, NULL, sizeLines);
		}
	}

	void NotifyParentZoomChanged()
	{
		T* pT = static_cast<T*>(this);
		int nId = pT->GetDlgCtrlID();
		NMHDR nmhdr = { pT->m_hWnd, (UINT_PTR)nId, ZSN_ZOOMCHANGED };
		::SendMessage(pT->GetParent(), WM_NOTIFY, (WPARAM)nId, (LPARAM)&nmhdr);
	}

	void DoWheelZoom(int zDelta)
	{
		float fZoomScale = m_fZoomScale + ((zDelta > 0) ? m_fZoomDelta : -m_fZoomDelta);
		T* pT = static_cast<T*>(this);
		pT->Zoom(fZoomScale);
		pT->NotifyParentZoomChanged();
	}

	BEGIN_MSG_MAP(CZoomScrollImpl)
		MESSAGE_HANDLER(WM_SETCURSOR, OnSetCursor)
		MESSAGE_HANDLER(WM_VSCROLL, CScrollImpl< T >::OnVScroll)
		MESSAGE_HANDLER(WM_HSCROLL, CScrollImpl< T >::OnHScroll)
		MESSAGE_HANDLER(WM_MOUSEWHEEL, OnMouseWheel)
		MESSAGE_HANDLER(WM_MOUSEHWHEEL, CScrollImpl< T >::OnMouseHWheel)
		MESSAGE_HANDLER(WM_SETTINGCHANGE, CScrollImpl< T >::OnSettingChange)
		MESSAGE_HANDLER(WM_SIZE, CScrollImpl< T >::OnSize)
		MESSAGE_HANDLER(WM_PAINT, OnPaint)
		MESSAGE_HANDLER(WM_PRINTCLIENT, OnPaint)
		MESSAGE_HANDLER(WM_LBUTTONDOWN, OnLButtonDown)
		MESSAGE_HANDLER(WM_MOUSEMOVE, OnMouseMove)
		MESSAGE_HANDLER(WM_LBUTTONUP, OnLButtonUp)
		MESSAGE_HANDLER(WM_CAPTURECHANGED, OnCaptureChanged)
	ALT_MSG_MAP(1)
		COMMAND_ID_HANDLER(ID_SCROLL_UP, CScrollImpl< T >::OnScrollUp)
		COMMAND_ID_HANDLER(ID_SCROLL_DOWN, CScrollImpl< T >::OnScrollDown)
		COMMAND_ID_HANDLER(ID_SCROLL_PAGE_UP, CScrollImpl< T >::OnScrollPageUp)
		COMMAND_ID_HANDLER(ID_SCROLL_PAGE_DOWN, CScrollImpl< T >::OnScrollPageDown)
		COMMAND_ID_HANDLER(ID_SCROLL_TOP, CScrollImpl< T >::OnScrollTop)
		COMMAND_ID_HANDLER(ID_SCROLL_BOTTOM, CScrollImpl< T >::OnScrollBottom)
		COMMAND_ID_HANDLER(ID_SCROLL_LEFT, CScrollImpl< T >::OnScrollLeft)
		COMMAND_ID_HANDLER(ID_SCROLL_RIGHT, CScrollImpl< T >::OnScrollRight)
		COMMAND_ID_HANDLER(ID_SCROLL_PAGE_LEFT, CScrollImpl< T >::OnScrollPageLeft)
		COMMAND_ID_HANDLER(ID_SCROLL_PAGE_RIGHT, CScrollImpl< T >::OnScrollPageRight)
		COMMAND_ID_HANDLER(ID_SCROLL_ALL_LEFT, CScrollImpl< T >::OnScrollAllLeft)
		COMMAND_ID_HANDLER(ID_SCROLL_ALL_RIGHT, CScrollImpl< T >::OnScrollAllRight)
	END_MSG_MAP()

	LRESULT OnSetCursor(UINT /*uMsg*/, WPARAM wParam, LPARAM lParam, BOOL& bHandled)
	{
		if((LOWORD(lParam) == HTCLIENT) && (m_nZoomMode != ZOOMMODE_OFF))
		{
			T* pT = static_cast<T*>(this);
			if((HWND)wParam == pT->m_hWnd)
			{
				DWORD dwPos = ::GetMessagePos();
				POINT pt = { GET_X_LPARAM(dwPos), GET_Y_LPARAM(dwPos) };
				pT->ScreenToClient(&pt);
				if(pT->PtInDevRect(pt))
				{
					::SetCursor(::LoadCursor(NULL, IDC_CROSS));
					return 1;
				}
			}
		}

		bHandled = FALSE;
		return 0;
	}

	LRESULT OnMouseWheel(UINT uMsg, WPARAM wParam, LPARAM lParam, BOOL& bHandled)
	{
		if((GET_KEYSTATE_WPARAM(wParam) & MK_CONTROL) != 0)   // handle zoom if Ctrl is pressed
		{
			int zDelta = (int)GET_WHEEL_DELTA_WPARAM(wParam);
			T* pT = static_cast<T*>(this);
			pT->DoWheelZoom(zDelta);
		}
		else
		{
			CScrollImpl< T >::OnMouseWheel(uMsg, wParam, lParam, bHandled);
		}

		return 0;
	}

	LRESULT OnPaint(UINT /*uMsg*/, WPARAM wParam, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));
		ATLASSERT((m_sizeLogAll.cx >= 0) && (m_sizeLogAll.cy >= 0));
		ATLASSERT((this->m_sizeAll.cx >= 0) && (this->m_sizeAll.cy >= 0));

		if(wParam != NULL)
		{
			CDCHandle dc = (HDC)wParam;
			int nMapModeSav = dc.GetMapMode();
			dc.SetMapMode(MM_ANISOTROPIC);
			SIZE szWindowExt = { 0, 0 };
			dc.SetWindowExt(m_sizeLogAll, &szWindowExt);
			SIZE szViewportExt = { 0, 0 };
			dc.SetViewportExt(this->m_sizeAll, &szViewportExt);
			POINT ptViewportOrg = { 0, 0 };
			dc.SetViewportOrg(-this->m_ptOffset.x, -this->m_ptOffset.y, &ptViewportOrg);

			pT->DoPaint(dc);

			dc.SetMapMode(nMapModeSav);
			dc.SetWindowExt(szWindowExt);
			dc.SetViewportExt(szViewportExt);
			dc.SetViewportOrg(ptViewportOrg);
		}
		else
		{
			CPaintDC dc(pT->m_hWnd);
			pT->PrepareDC(dc.m_hDC);
			pT->DoPaint(dc.m_hDC);
		}

		return 0;
	}

	LRESULT OnLButtonDown(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM lParam, BOOL& bHandled)
	{
		if((m_nZoomMode == ZOOMMODE_IN) && !m_bTracking)
		{
			T* pT = static_cast<T*>(this);
			POINT pt = { GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam) };
			if(pT->PtInDevRect(pt))
			{
				pT->SetCapture();
				m_bTracking = true;
				::SetRect(&m_rcTrack, pt.x, pt.y, pt.x, pt.y);

				RECT rcClip;
				pT->GetClientRect(&rcClip);
				if((this->m_ptOffset.x == 0) && (this->m_ptOffset.y == 0))
				{
					if(rcClip.right > this->m_sizeAll.cx)
						rcClip.right = this->m_sizeAll.cx;
					if(rcClip.bottom > this->m_sizeAll.cy)
						rcClip.bottom = this->m_sizeAll.cy;
				}
				::MapWindowPoints(pT->m_hWnd, NULL, (LPPOINT)&rcClip, 2);
				::ClipCursor(&rcClip);
			}	
		}

		bHandled = FALSE;
		return 0;
	}

	LRESULT OnMouseMove(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM lParam, BOOL& bHandled)
	{
		if(m_bTracking)
		{
			T* pT = static_cast<T*>(this);
			POINT pt = { GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam) };
			if(pT->PtInDevRect(pt))
			{
				pT->DrawTrackRect();
				m_rcTrack.right = pt.x + 1;
				m_rcTrack.bottom = pt.y + 1;
				pT->DrawTrackRect();
			}
		}

		bHandled = FALSE;
		return 0;
	}

	LRESULT OnLButtonUp(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM lParam, BOOL& bHandled)
	{
		::ReleaseCapture();
		if(m_nZoomMode == ZOOMMODE_OUT)
		{
			T* pT = static_cast<T*>(this);
			pT->Zoom(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam), m_fZoomScale - m_fZoomDelta);
			pT->NotifyParentZoomChanged();
		}

		bHandled = FALSE;
		return 0;
	}

	LRESULT OnCaptureChanged(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& bHandled)
	{
		if(m_bTracking)
		{
			m_bTracking = false;
			T* pT = static_cast<T*>(this);
			pT->DrawTrackRect();
			pT->Zoom(m_rcTrack);
			pT->NotifyParentZoomChanged();
			::SetRectEmpty(&m_rcTrack);
			::ClipCursor(NULL);
		}

		bHandled = FALSE;
		return 0;
	}	
};

///////////////////////////////////////////////////////////////////////////////
// CZoomScrollWindowImpl - Implements scrolling window with zooming

template <class T, class TBase = ATL::CWindow, class TWinTraits = ATL::CControlWinTraits>
class ATL_NO_VTABLE CZoomScrollWindowImpl : public ATL::CWindowImpl< T, TBase, TWinTraits >, public CZoomScrollImpl< T >
{
public:
	BOOL SubclassWindow(HWND hWnd)
	{
		BOOL bRet = ATL::CWindowImpl< T, TBase, TWinTraits >::SubclassWindow(hWnd);
		if(bRet != FALSE)
		{
			T* pT = static_cast<T*>(this);
			pT->GetSystemSettings();

			RECT rect = {};
			this->GetClientRect(&rect);
			pT->DoSize(rect.right, rect.bottom);
		}

		return bRet;
	}

	BEGIN_MSG_MAP(CZoomScrollWindowImpl)
		MESSAGE_HANDLER(WM_SETCURSOR, CZoomScrollImpl< T >::OnSetCursor)
		MESSAGE_HANDLER(WM_VSCROLL, CScrollImpl< T >::OnVScroll)
		MESSAGE_HANDLER(WM_HSCROLL, CScrollImpl< T >::OnHScroll)
		MESSAGE_HANDLER(WM_MOUSEWHEEL, CZoomScrollImpl< T >::OnMouseWheel)
		MESSAGE_HANDLER(WM_MOUSEHWHEEL, CScrollImpl< T >::OnMouseHWheel)
		MESSAGE_HANDLER(WM_SETTINGCHANGE, CScrollImpl< T >::OnSettingChange)
		MESSAGE_HANDLER(WM_SIZE, CScrollImpl< T >::OnSize)
		MESSAGE_HANDLER(WM_PAINT, CZoomScrollImpl< T >::OnPaint)
		MESSAGE_HANDLER(WM_PRINTCLIENT, CZoomScrollImpl< T >::OnPaint)
		MESSAGE_HANDLER(WM_LBUTTONDOWN, CZoomScrollImpl< T >::OnLButtonDown)
		MESSAGE_HANDLER(WM_MOUSEMOVE, CZoomScrollImpl< T >::OnMouseMove)
		MESSAGE_HANDLER(WM_LBUTTONUP, CZoomScrollImpl< T >::OnLButtonUp)
		MESSAGE_HANDLER(WM_CAPTURECHANGED, CZoomScrollImpl< T >::OnCaptureChanged)
	ALT_MSG_MAP(1)
		COMMAND_ID_HANDLER(ID_SCROLL_UP, CScrollImpl< T >::OnScrollUp)
		COMMAND_ID_HANDLER(ID_SCROLL_DOWN, CScrollImpl< T >::OnScrollDown)
		COMMAND_ID_HANDLER(ID_SCROLL_PAGE_UP, CScrollImpl< T >::OnScrollPageUp)
		COMMAND_ID_HANDLER(ID_SCROLL_PAGE_DOWN, CScrollImpl< T >::OnScrollPageDown)
		COMMAND_ID_HANDLER(ID_SCROLL_TOP, CScrollImpl< T >::OnScrollTop)
		COMMAND_ID_HANDLER(ID_SCROLL_BOTTOM, CScrollImpl< T >::OnScrollBottom)
		COMMAND_ID_HANDLER(ID_SCROLL_LEFT, CScrollImpl< T >::OnScrollLeft)
		COMMAND_ID_HANDLER(ID_SCROLL_RIGHT, CScrollImpl< T >::OnScrollRight)
		COMMAND_ID_HANDLER(ID_SCROLL_PAGE_LEFT, CScrollImpl< T >::OnScrollPageLeft)
		COMMAND_ID_HANDLER(ID_SCROLL_PAGE_RIGHT, CScrollImpl< T >::OnScrollPageRight)
		COMMAND_ID_HANDLER(ID_SCROLL_ALL_LEFT, CScrollImpl< T >::OnScrollAllLeft)
		COMMAND_ID_HANDLER(ID_SCROLL_ALL_RIGHT, CScrollImpl< T >::OnScrollAllRight)
	END_MSG_MAP()
};


///////////////////////////////////////////////////////////////////////////////
// CScrollContainer

template <class T, class TBase = ATL::CWindow, class TWinTraits = ATL::CControlWinTraits>
class ATL_NO_VTABLE CScrollContainerImpl : public CScrollWindowImpl< T, TBase, TWinTraits >
{
public:
	DECLARE_WND_CLASS_EX2(NULL, T, 0, -1)

	typedef CScrollWindowImpl< T, TBase, TWinTraits >   _baseClass;

// Data members
	ATL::CWindow m_wndClient;
	bool m_bAutoSizeClient;
	bool m_bDrawEdgeIfEmpty;

// Constructor
	CScrollContainerImpl() : m_bAutoSizeClient(true), m_bDrawEdgeIfEmpty(false)
	{
		// Set CScrollWindowImpl extended style
		this->SetScrollExtendedStyle(SCRL_SCROLLCHILDREN);
	}

// Attributes
	HWND GetClient() const
	{
		return m_wndClient;
	}

	HWND SetClient(HWND hWndClient, bool bClientSizeAsMin = true)
	{
		ATLASSERT(::IsWindow(this->m_hWnd));

		HWND hWndOldClient = m_wndClient;
		m_wndClient = hWndClient;

		this->SetRedraw(FALSE);
		this->SetScrollSize(1, 1, FALSE);

		if(m_wndClient.m_hWnd != NULL)
		{
			m_wndClient.SetWindowPos(NULL, 0, 0, 0, 0, SWP_NOZORDER | SWP_NOSIZE);

			if(bClientSizeAsMin)
			{
				RECT rect = {};
				m_wndClient.GetWindowRect(&rect);
				if(((rect.right - rect.left) > 0) && ((rect.bottom - rect.top) > 0))
					this->SetScrollSize(rect.right - rect.left, rect.bottom - rect.top, FALSE);
			}

			T* pT = static_cast<T*>(this);
			pT->UpdateLayout();
		}

		this->SetRedraw(TRUE);
		this->RedrawWindow(NULL, NULL, RDW_INVALIDATE | RDW_FRAME | RDW_UPDATENOW | RDW_ALLCHILDREN);

		return hWndOldClient;
	}

// Message map and handlers
	BEGIN_MSG_MAP(CScrollContainerImpl)
		MESSAGE_HANDLER(WM_SETFOCUS, OnSetFocus)
		MESSAGE_HANDLER(WM_ERASEBKGND, OnEraseBackground)
		CHAIN_MSG_MAP(_baseClass)
		FORWARD_NOTIFICATIONS()
	ALT_MSG_MAP(1)
		CHAIN_MSG_MAP_ALT(_baseClass, 1)
	END_MSG_MAP()

	LRESULT OnSetFocus(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		if(m_wndClient.m_hWnd != NULL)
			m_wndClient.SetFocus();

		return 0;
	}

	LRESULT OnEraseBackground(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		return 1;   // no background needed
	}

// Overrides for CScrollWindowImpl
	void DoSize(int cx, int cy)
	{
		_baseClass::DoSize(cx, cy);

		T* pT = static_cast<T*>(this);
		pT->UpdateLayout();
	}

	void DoPaint(CDCHandle dc)
	{
		if(!m_bAutoSizeClient || (m_wndClient.m_hWnd == NULL))
		{
			T* pT = static_cast<T*>(this);
			RECT rect = {};
			pT->GetContainerRect(rect);

			if(m_bDrawEdgeIfEmpty && (m_wndClient.m_hWnd == NULL))
				dc.DrawEdge(&rect, EDGE_SUNKEN, BF_RECT | BF_ADJUST);

			dc.FillRect(&rect, COLOR_APPWORKSPACE);
		}
	}

	void ScrollToView(POINT pt)
	{
		CScrollWindowImpl< T, TBase, TWinTraits >::ScrollToView(pt);
	}

	void ScrollToView(RECT& rect)
	{
		CScrollWindowImpl< T, TBase, TWinTraits >::ScrollToView(rect);
	}

	void ScrollToView(HWND hWnd)   // client window coordinates
	{
		T* pT = static_cast<T*>(this);
		(void)pT;   // avoid level 4 warning
		ATLASSERT(::IsWindow(pT->m_hWnd));
		ATLASSERT(m_wndClient.IsWindow());

		RECT rect = {};
		::GetWindowRect(hWnd, &rect);
		::MapWindowPoints(NULL, m_wndClient.m_hWnd, (LPPOINT)&rect, 2);
		ScrollToView(rect);
	}

// Implementation - overrideable methods
	void UpdateLayout()
	{
		ATLASSERT(::IsWindow(this->m_hWnd));

		if(m_bAutoSizeClient && (m_wndClient.m_hWnd != NULL))
		{
			T* pT = static_cast<T*>(this);
			RECT rect = {};
			pT->GetContainerRect(rect);

			m_wndClient.SetWindowPos(NULL, &rect, SWP_NOZORDER | SWP_NOMOVE);
		}
		else
		{
			this->Invalidate();
		}
	}

	void GetContainerRect(RECT& rect)
	{
		this->GetClientRect(&rect);

		if(rect.right < this->m_sizeAll.cx)
			rect.right = this->m_sizeAll.cx;

		if(rect.bottom < this->m_sizeAll.cy)
			rect.bottom = this->m_sizeAll.cy;
	}
};

class CScrollContainer : public CScrollContainerImpl<CScrollContainer>
{
public:
	DECLARE_WND_CLASS_EX(_T("WTL_ScrollContainer"), 0, -1)
};

} // namespace WTL

#endif // __ATLSCRL_H__
