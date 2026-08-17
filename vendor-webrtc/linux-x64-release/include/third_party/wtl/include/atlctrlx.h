// Windows Template Library - WTL version 10.0
// Copyright (C) Microsoft Corporation, WTL Team. All rights reserved.
//
// This file is a part of the Windows Template Library.
// The use and distribution terms for this software are covered by the
// Microsoft Public License (http://opensource.org/licenses/MS-PL)
// which can be found in the file MS-PL.txt at the root folder.

#ifndef __ATLCTRLX_H__
#define __ATLCTRLX_H__

#pragma once

#ifndef __ATLAPP_H__
	#error atlctrlx.h requires atlapp.h to be included first
#endif

#ifndef __ATLCTRLS_H__
	#error atlctrlx.h requires atlctrls.h to be included first
#endif


///////////////////////////////////////////////////////////////////////////////
// Classes in this file:
//
// CBitmapButtonImpl<T, TBase, TWinTraits>
// CBitmapButton
// CCheckListViewCtrlImpl<T, TBase, TWinTraits>
// CCheckListViewCtrl
// CHyperLinkImpl<T, TBase, TWinTraits>
// CHyperLink
// CWaitCursor
// CCustomWaitCursor
// CMultiPaneStatusBarCtrlImpl<T, TBase>
// CMultiPaneStatusBarCtrl
// CPaneContainerImpl<T, TBase, TWinTraits>
// CPaneContainer
// CSortListViewImpl<T>
// CSortListViewCtrlImpl<T, TBase, TWinTraits>
// CSortListViewCtrl
// CTabViewImpl<T, TBase, TWinTraits>
// CTabView

namespace WTL
{

///////////////////////////////////////////////////////////////////////////////
// CBitmapButton - bitmap button implementation

// bitmap button extended styles
#define BMPBTN_HOVER            0x00000001
#define BMPBTN_AUTO3D_SINGLE    0x00000002
#define BMPBTN_AUTO3D_DOUBLE    0x00000004
#define BMPBTN_AUTOSIZE         0x00000008
#define BMPBTN_SHAREIMAGELISTS  0x00000010
#define BMPBTN_AUTOFIRE         0x00000020
#define BMPBTN_CHECK            0x00000040
#define BMPBTN_AUTOCHECK        0x00000080

// Note: BMPBTN_CHECK/BMPBTN_AUTOCHECK disables BN_DOUBLECLICKED,
// BMPBTN_AUTOFIRE doesn't work with BMPBTN_CHECK/BMPBTN_AUTOCHECK

template <class T, class TBase = CButton, class TWinTraits = ATL::CControlWinTraits>
class ATL_NO_VTABLE CBitmapButtonImpl : public ATL::CWindowImpl< T, TBase, TWinTraits >
{
public:
	DECLARE_WND_SUPERCLASS2(NULL, T, TBase::GetWndClassName())

	enum
	{
		_nImageNormal = 0,
		_nImagePushed,
		_nImageFocusOrHover,
		_nImageDisabled,

		_nImageCount = 4,
	};

	enum
	{
		ID_TIMER_FIRST = 1000,
		ID_TIMER_REPEAT = 1001
	};

	// Bitmap button specific extended styles
	DWORD m_dwExtendedStyle;

	CImageList m_ImageList;
	int m_nImage[_nImageCount];

	CToolTipCtrl m_tip;
	LPTSTR m_lpstrToolTipText;

	// Internal states
	unsigned m_fMouseOver:1;
	unsigned m_fFocus:1;
	unsigned m_fPressed:1;
	unsigned m_fChecked:1;


// Constructor/Destructor
	CBitmapButtonImpl(DWORD dwExtendedStyle = BMPBTN_AUTOSIZE, HIMAGELIST hImageList = NULL) : 
	                  m_dwExtendedStyle(dwExtendedStyle), m_ImageList(hImageList), 
	                  m_lpstrToolTipText(NULL),
	                  m_fMouseOver(0), m_fFocus(0), m_fPressed(0), m_fChecked(0)
	{
		m_nImage[_nImageNormal] = -1;
		m_nImage[_nImagePushed] = -1;
		m_nImage[_nImageFocusOrHover] = -1;
		m_nImage[_nImageDisabled] = -1;

#ifdef _DEBUG
		if(((m_dwExtendedStyle & BMPBTN_AUTOFIRE) != 0) && IsCheckMode())
			ATLTRACE2(atlTraceUI, 0, _T("CBitmapButtonImpl - Check mode and BMPBTN_AUTOFIRE cannot be used together, BMPBTN_AUTOFIRE will be ignored.\n"));
#endif // _DEBUG
	}

	~CBitmapButtonImpl()
	{
		if((m_dwExtendedStyle & BMPBTN_SHAREIMAGELISTS) == 0)
			m_ImageList.Destroy();
		delete [] m_lpstrToolTipText;
	}

	// overridden to provide proper initialization
	BOOL SubclassWindow(HWND hWnd)
	{
		BOOL bRet = ATL::CWindowImpl< T, TBase, TWinTraits >::SubclassWindow(hWnd);
		if(bRet != FALSE)
		{
			T* pT = static_cast<T*>(this);
			pT->Init();
		}

		return bRet;
	}

// Attributes
	DWORD GetBitmapButtonExtendedStyle() const
	{
		return m_dwExtendedStyle;
	}

	DWORD SetBitmapButtonExtendedStyle(DWORD dwExtendedStyle, DWORD dwMask = 0)
	{
		DWORD dwPrevStyle = m_dwExtendedStyle;
		if(dwMask == 0)
			m_dwExtendedStyle = dwExtendedStyle;
		else
			m_dwExtendedStyle = (m_dwExtendedStyle & ~dwMask) | (dwExtendedStyle & dwMask);

#ifdef _DEBUG
		if(((m_dwExtendedStyle & BMPBTN_AUTOFIRE) != 0) && IsCheckMode())
			ATLTRACE2(atlTraceUI, 0, _T("CBitmapButtonImpl - Check mode and BMPBTN_AUTOFIRE cannot be used together, BMPBTN_AUTOFIRE will be ignored.\n"));
#endif // _DEBUG

		return dwPrevStyle;
	}

	HIMAGELIST GetImageList() const
	{
		return m_ImageList;
	}

	HIMAGELIST SetImageList(HIMAGELIST hImageList)
	{
		HIMAGELIST hImageListPrev = m_ImageList;
		m_ImageList = hImageList;
		if(((m_dwExtendedStyle & BMPBTN_AUTOSIZE) != 0) && ::IsWindow(this->m_hWnd))
			SizeToImage();

		return hImageListPrev;
	}

	int GetToolTipTextLength() const
	{
		return (m_lpstrToolTipText == NULL) ? -1 : lstrlen(m_lpstrToolTipText);
	}

	bool GetToolTipText(LPTSTR lpstrText, int nLength) const
	{
		ATLASSERT(lpstrText != NULL);
		if(m_lpstrToolTipText == NULL)
			return false;

		errno_t nRet = ATL::Checked::tcsncpy_s(lpstrText, nLength, m_lpstrToolTipText, _TRUNCATE);

		return ((nRet == 0) || (nRet == STRUNCATE));
	}

	bool SetToolTipText(LPCTSTR lpstrText)
	{
		if(m_lpstrToolTipText != NULL)
		{
			delete [] m_lpstrToolTipText;
			m_lpstrToolTipText = NULL;
		}

		if(lpstrText == NULL)
		{
			if(m_tip.IsWindow())
				m_tip.Activate(FALSE);
			return true;
		}

		int cchLen = lstrlen(lpstrText) + 1;
		ATLTRY(m_lpstrToolTipText = new TCHAR[cchLen]);
		if(m_lpstrToolTipText == NULL)
			return false;

		ATL::Checked::tcscpy_s(m_lpstrToolTipText, cchLen, lpstrText);
		if(m_tip.IsWindow())
		{
			m_tip.Activate(TRUE);
			m_tip.AddTool(this->m_hWnd, m_lpstrToolTipText);
		}

		return true;
	}

	bool GetCheck() const
	{
		return (m_fChecked == 1);
	}

	void SetCheck(bool bCheck, bool bUpdate = true)
	{
		m_fChecked = bCheck ? 1 : 0;

		if(bUpdate)
		{
			this->Invalidate();
			this->UpdateWindow();
		}
	}

// Operations
	void SetImages(int nNormal, int nPushed = -1, int nFocusOrHover = -1, int nDisabled = -1)
	{
		if(nNormal != -1)
			m_nImage[_nImageNormal] = nNormal;
		if(nPushed != -1)
			m_nImage[_nImagePushed] = nPushed;
		if(nFocusOrHover != -1)
			m_nImage[_nImageFocusOrHover] = nFocusOrHover;
		if(nDisabled != -1)
			m_nImage[_nImageDisabled] = nDisabled;
	}

	BOOL SizeToImage()
	{
		ATLASSERT(::IsWindow(this->m_hWnd) && (m_ImageList.m_hImageList != NULL));
		int cx = 0;
		int cy = 0;
		if(!m_ImageList.GetIconSize(cx, cy))
			return FALSE;
		return this->ResizeClient(cx, cy);
	}

// Overrideables
	void DoPaint(CDCHandle dc)
	{
		ATLASSERT(m_ImageList.m_hImageList != NULL);   // image list must be set
		ATLASSERT(m_nImage[0] != -1);                  // main bitmap must be set

		// set bitmap according to the current button state
		bool bHover = IsHoverMode();
		bool bPressed = (m_fPressed == 1) || (IsCheckMode() && (m_fChecked == 1));
		int nImage = -1;
		if(!this->IsWindowEnabled())
			nImage = m_nImage[_nImageDisabled];
		else if(bPressed)
			nImage = m_nImage[_nImagePushed];
		else if((!bHover && (m_fFocus == 1)) || (bHover && (m_fMouseOver == 1)))
			nImage = m_nImage[_nImageFocusOrHover];

		// if none is set, use default one
		if(nImage == -1)
			nImage = m_nImage[_nImageNormal];

		// draw the button image
		bool bAuto3D = (m_dwExtendedStyle & (BMPBTN_AUTO3D_SINGLE | BMPBTN_AUTO3D_DOUBLE)) != 0;
		int xyPos = (bPressed && bAuto3D && (m_nImage[_nImagePushed] == -1)) ? 1 : 0;
		m_ImageList.Draw(dc, nImage, xyPos, xyPos, ILD_NORMAL);

		// draw 3D border if required
		if(bAuto3D)
		{
			RECT rect = {};
			this->GetClientRect(&rect);

			if(bPressed)
				dc.DrawEdge(&rect, ((m_dwExtendedStyle & BMPBTN_AUTO3D_SINGLE) != 0) ? BDR_SUNKENOUTER : EDGE_SUNKEN, BF_RECT);
			else if(!bHover || (m_fMouseOver == 1))
				dc.DrawEdge(&rect, ((m_dwExtendedStyle & BMPBTN_AUTO3D_SINGLE) != 0) ? BDR_RAISEDINNER : EDGE_RAISED, BF_RECT);

			if(!bHover && (m_fFocus == 1))
			{
				::InflateRect(&rect, -2 * ::GetSystemMetrics(SM_CXEDGE), -2 * ::GetSystemMetrics(SM_CYEDGE));
				dc.DrawFocusRect(&rect);
			}
		}
	}

// Message map and handlers
	BEGIN_MSG_MAP(CBitmapButtonImpl)
		MESSAGE_HANDLER(WM_CREATE, OnCreate)
		MESSAGE_HANDLER(WM_DESTROY, OnDestroy)
		MESSAGE_RANGE_HANDLER(WM_MOUSEFIRST, WM_MOUSELAST, OnMouseMessage)
		MESSAGE_HANDLER(WM_ERASEBKGND, OnEraseBackground)
		MESSAGE_HANDLER(WM_PAINT, OnPaint)
		MESSAGE_HANDLER(WM_PRINTCLIENT, OnPaint)
		MESSAGE_HANDLER(WM_SETFOCUS, OnFocus)
		MESSAGE_HANDLER(WM_KILLFOCUS, OnFocus)
		MESSAGE_HANDLER(WM_LBUTTONDOWN, OnLButtonDown)
		MESSAGE_HANDLER(WM_LBUTTONDBLCLK, OnLButtonDblClk)
		MESSAGE_HANDLER(WM_LBUTTONUP, OnLButtonUp)
		MESSAGE_HANDLER(WM_CAPTURECHANGED, OnCaptureChanged)
		MESSAGE_HANDLER(WM_ENABLE, OnEnable)
		MESSAGE_HANDLER(WM_MOUSEMOVE, OnMouseMove)
		MESSAGE_HANDLER(WM_MOUSELEAVE, OnMouseLeave)
		MESSAGE_HANDLER(WM_KEYDOWN, OnKeyDown)
		MESSAGE_HANDLER(WM_KEYUP, OnKeyUp)
		MESSAGE_HANDLER(WM_TIMER, OnTimer)
		MESSAGE_HANDLER(WM_UPDATEUISTATE, OnUpdateUiState)
	END_MSG_MAP()

	LRESULT OnCreate(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& bHandled)
	{
		T* pT = static_cast<T*>(this);
		pT->Init();

		bHandled = FALSE;
		return 1;
	}

	LRESULT OnDestroy(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& bHandled)
	{
		if(m_tip.IsWindow())
		{
			m_tip.DestroyWindow();
			m_tip.m_hWnd = NULL;
		}
		bHandled = FALSE;
		return 1;
	}

	LRESULT OnMouseMessage(UINT uMsg, WPARAM wParam, LPARAM lParam, BOOL& bHandled)
	{
		MSG msg = { this->m_hWnd, uMsg, wParam, lParam };
		if(m_tip.IsWindow())
			m_tip.RelayEvent(&msg);
		bHandled = FALSE;
		return 1;
	}

	LRESULT OnEraseBackground(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		return 1;   // no background needed
	}

	LRESULT OnPaint(UINT /*uMsg*/, WPARAM wParam, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		T* pT = static_cast<T*>(this);
		if(wParam != NULL)
		{
			pT->DoPaint((HDC)wParam);
		}
		else
		{
			CPaintDC dc(this->m_hWnd);
			pT->DoPaint(dc.m_hDC);
		}
		return 0;
	}

	LRESULT OnFocus(UINT uMsg, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& bHandled)
	{
		m_fFocus = (uMsg == WM_SETFOCUS) ? 1 : 0;
		this->Invalidate();
		this->UpdateWindow();
		bHandled = FALSE;
		return 1;
	}

	LRESULT OnLButtonDown(UINT uMsg, WPARAM wParam, LPARAM lParam, BOOL& /*bHandled*/)
	{
		LRESULT lRet = 0;
		if(IsHoverMode())
			this->SetCapture();
		else
			lRet = this->DefWindowProc(uMsg, wParam, lParam);
		if(::GetCapture() == this->m_hWnd)
		{
			m_fPressed = 1;
			this->Invalidate();
			this->UpdateWindow();
		}
		if(((m_dwExtendedStyle & BMPBTN_AUTOFIRE) != 0) && !IsCheckMode())
		{
			int nElapse = 250;
			int nDelay = 0;
			if(::SystemParametersInfo(SPI_GETKEYBOARDDELAY, 0, &nDelay, 0))
				nElapse += nDelay * 250;   // all milli-seconds
			this->SetTimer(ID_TIMER_FIRST, nElapse);
		}
		return lRet;
	}

	LRESULT OnLButtonDblClk(UINT uMsg, WPARAM wParam, LPARAM lParam, BOOL& /*bHandled*/)
	{
		LRESULT lRet = 0;
		if(!IsHoverMode() && !IsCheckMode())
			lRet = this->DefWindowProc(uMsg, wParam, lParam);
		if(::GetCapture() != this->m_hWnd)
			this->SetCapture();
		if(m_fPressed == 0)
		{
			m_fPressed = 1;
			this->Invalidate();
			this->UpdateWindow();
		}
		return lRet;
	}

	LRESULT OnLButtonUp(UINT uMsg, WPARAM wParam, LPARAM lParam, BOOL& /*bHandled*/)
	{
		if(((m_dwExtendedStyle & BMPBTN_AUTOCHECK) != 0) && (m_fPressed == 1))
			SetCheck(!GetCheck(), false);

		LRESULT lRet = 0;
		if(!IsHoverMode() && !IsCheckMode())
			lRet = this->DefWindowProc(uMsg, wParam, lParam);
		if(::GetCapture() == this->m_hWnd)
		{
			if((IsHoverMode() || IsCheckMode()) && (m_fPressed == 1))
				this->GetParent().SendMessage(WM_COMMAND, MAKEWPARAM(this->GetDlgCtrlID(), BN_CLICKED), (LPARAM)this->m_hWnd);
			::ReleaseCapture();
		}
		return lRet;
	}

	LRESULT OnCaptureChanged(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& bHandled)
	{
		if(m_fPressed == 1)
		{
			m_fPressed = 0;
			this->Invalidate();
			this->UpdateWindow();
		}
		bHandled = FALSE;
		return 1;
	}

	LRESULT OnEnable(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& bHandled)
	{
		this->Invalidate();
		this->UpdateWindow();
		bHandled = FALSE;
		return 1;
	}

	LRESULT OnMouseMove(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM lParam, BOOL& bHandled)
	{
		if(::GetCapture() == this->m_hWnd)
		{
			POINT ptCursor = { GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam) };
			this->ClientToScreen(&ptCursor);
			RECT rect = {};
			this->GetWindowRect(&rect);
			unsigned int uPressed = ::PtInRect(&rect, ptCursor) ? 1 : 0;
			if(m_fPressed != uPressed)
			{
				m_fPressed = uPressed;
				this->Invalidate();
				this->UpdateWindow();
			}
		}
		else if(IsHoverMode() && m_fMouseOver == 0)
		{
			m_fMouseOver = 1;
			this->Invalidate();
			this->UpdateWindow();
			StartTrackMouseLeave();
		}
		bHandled = FALSE;
		return 1;
	}

	LRESULT OnMouseLeave(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		if(m_fMouseOver == 1)
		{
			m_fMouseOver = 0;
			this->Invalidate();
			this->UpdateWindow();
		}
		return 0;
	}

	LRESULT OnKeyDown(UINT /*uMsg*/, WPARAM wParam, LPARAM /*lParam*/, BOOL& bHandled)
	{
		if((wParam == VK_SPACE) && IsHoverMode())
			return 0;   // ignore if in hover mode
		if((wParam == VK_SPACE) && (m_fPressed == 0))
		{
			m_fPressed = 1;
			this->Invalidate();
			this->UpdateWindow();
		}
		bHandled = FALSE;
		return 1;
	}

	LRESULT OnKeyUp(UINT /*uMsg*/, WPARAM wParam, LPARAM /*lParam*/, BOOL& bHandled)
	{
		if((wParam == VK_SPACE) && IsHoverMode())
			return 0;   // ignore if in hover mode
		if((wParam == VK_SPACE) && (m_fPressed == 1))
		{
			m_fPressed = 0;
			if((m_dwExtendedStyle & BMPBTN_AUTOCHECK) != 0)
				SetCheck(!GetCheck(), false);
			this->Invalidate();
			this->UpdateWindow();
		}
		bHandled = FALSE;
		return 1;
	}

	LRESULT OnTimer(UINT /*uMsg*/, WPARAM wParam, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		ATLASSERT((m_dwExtendedStyle & BMPBTN_AUTOFIRE) != 0);
		switch(wParam)   // timer ID
		{
		case ID_TIMER_FIRST:
			this->KillTimer(ID_TIMER_FIRST);
			if(m_fPressed == 1)
			{
				this->GetParent().SendMessage(WM_COMMAND, MAKEWPARAM(this->GetDlgCtrlID(), BN_CLICKED), (LPARAM)this->m_hWnd);
				int nElapse = 250;
				int nRepeat = 40;
				if(::SystemParametersInfo(SPI_GETKEYBOARDSPEED, 0, &nRepeat, 0))
					nElapse = 10000 / (10 * nRepeat + 25);   // milli-seconds, approximated
				this->SetTimer(ID_TIMER_REPEAT, nElapse);
			}
			break;
		case ID_TIMER_REPEAT:
			if(m_fPressed == 1)
				this->GetParent().SendMessage(WM_COMMAND, MAKEWPARAM(this->GetDlgCtrlID(), BN_CLICKED), (LPARAM)this->m_hWnd);
			else if(::GetCapture() != this->m_hWnd)
				this->KillTimer(ID_TIMER_REPEAT);
			break;
		default:	// not our timer
			break;
		}
		return 0;
	}

	LRESULT OnUpdateUiState(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		// If the control is subclassed or superclassed, this message can cause
		// repainting without WM_PAINT. We don't use this state, so just do nothing.
		return 0;
	}

// Implementation
	void Init()
	{
		// We need this style to prevent Windows from painting the button
		this->ModifyStyle(0, BS_OWNERDRAW);

		// create a tool tip
		m_tip.Create(this->m_hWnd);
		ATLASSERT(m_tip.IsWindow());
		if(m_tip.IsWindow() && (m_lpstrToolTipText != NULL))
		{
			m_tip.Activate(TRUE);
			m_tip.AddTool(this->m_hWnd, m_lpstrToolTipText);
		}

		if((m_ImageList.m_hImageList != NULL) && ((m_dwExtendedStyle & BMPBTN_AUTOSIZE) != 0))
			SizeToImage();
	}

	BOOL StartTrackMouseLeave()
	{
		TRACKMOUSEEVENT tme = {};
		tme.cbSize = sizeof(tme);
		tme.dwFlags = TME_LEAVE;
		tme.hwndTrack = this->m_hWnd;
		return ::TrackMouseEvent(&tme);
	}

	bool IsHoverMode() const
	{
		return ((m_dwExtendedStyle & BMPBTN_HOVER) != 0);
	}

	bool IsCheckMode() const
	{
		return ((m_dwExtendedStyle & (BMPBTN_CHECK | BMPBTN_AUTOCHECK)) != 0);
	}
};

class CBitmapButton : public CBitmapButtonImpl<CBitmapButton>
{
public:
	DECLARE_WND_SUPERCLASS(_T("WTL_BitmapButton"), GetWndClassName())

	CBitmapButton(DWORD dwExtendedStyle = BMPBTN_AUTOSIZE, HIMAGELIST hImageList = NULL) : 
		CBitmapButtonImpl<CBitmapButton>(dwExtendedStyle, hImageList)
	{ }
};


///////////////////////////////////////////////////////////////////////////////
// CCheckListCtrlView - list view control with check boxes

template <DWORD t_dwStyle, DWORD t_dwExStyle, DWORD t_dwExListViewStyle>
class CCheckListViewCtrlImplTraits
{
public:
	static DWORD GetWndStyle(DWORD dwStyle)
	{
		return (dwStyle == 0) ? t_dwStyle : dwStyle;
	}

	static DWORD GetWndExStyle(DWORD dwExStyle)
	{
		return (dwExStyle == 0) ? t_dwExStyle : dwExStyle;
	}

	static DWORD GetExtendedLVStyle()
	{
		return t_dwExListViewStyle;
	}
};

typedef CCheckListViewCtrlImplTraits<WS_CHILD | WS_VISIBLE | LVS_REPORT | LVS_SHOWSELALWAYS, WS_EX_CLIENTEDGE, LVS_EX_CHECKBOXES | LVS_EX_FULLROWSELECT>   CCheckListViewCtrlTraits;

template <class T, class TBase = CListViewCtrl, class TWinTraits = CCheckListViewCtrlTraits>
class ATL_NO_VTABLE CCheckListViewCtrlImpl : public ATL::CWindowImpl<T, TBase, TWinTraits >
{
public:
	DECLARE_WND_SUPERCLASS2(NULL, T, TBase::GetWndClassName())

// Attributes
	static DWORD GetExtendedLVStyle()
	{
		return TWinTraits::GetExtendedLVStyle();
	}

// Operations
	BOOL SubclassWindow(HWND hWnd)
	{
		BOOL bRet = ATL::CWindowImpl< T, TBase, TWinTraits >::SubclassWindow(hWnd);
		if(bRet != FALSE)
		{
			T* pT = static_cast<T*>(this);
			pT->Init();
		}

		return bRet;
	}

	void CheckSelectedItems(int nCurrItem)
	{
		// first check if this item is selected
		LVITEM lvi = {};
		lvi.iItem = nCurrItem;
		lvi.iSubItem = 0;
		lvi.mask = LVIF_STATE;
		lvi.stateMask = LVIS_SELECTED;
		this->GetItem(&lvi);
		// if item is not selected, don't do anything
		if(!(lvi.state & LVIS_SELECTED))
			return;
		// new check state will be reverse of the current state,
		BOOL bCheck = !this->GetCheckState(nCurrItem);
		int nItem = -1;
		int nOldItem = -1;
		while((nItem = this->GetNextItem(nOldItem, LVNI_SELECTED)) != -1)
		{
			if(nItem != nCurrItem)
				this->SetCheckState(nItem, bCheck);
			nOldItem = nItem;
		}
	}

// Implementation
	void Init()
	{
		T* pT = static_cast<T*>(this);
		(void)pT;   // avoid level 4 warning
		ATLASSERT((pT->GetExtendedLVStyle() & LVS_EX_CHECKBOXES) != 0);
		this->SetExtendedListViewStyle(pT->GetExtendedLVStyle());
	}

// Message map and handlers
	BEGIN_MSG_MAP(CCheckListViewCtrlImpl)
		MESSAGE_HANDLER(WM_CREATE, OnCreate)
		MESSAGE_HANDLER(WM_LBUTTONDOWN, OnLButtonDown)
		MESSAGE_HANDLER(WM_LBUTTONDBLCLK, OnLButtonDown)
		MESSAGE_HANDLER(WM_KEYDOWN, OnKeyDown)
	END_MSG_MAP()

	LRESULT OnCreate(UINT uMsg, WPARAM wParam, LPARAM lParam, BOOL& /*bHandled*/)
	{
		// first let list view control initialize everything
		LRESULT lRet = this->DefWindowProc(uMsg, wParam, lParam);
		if(lRet == 0)
		{
			T* pT = static_cast<T*>(this);
			pT->Init();
		}

		return lRet;
	}

	LRESULT OnLButtonDown(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM lParam, BOOL& bHandled)
	{
		POINT ptMsg = { GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam) };
		LVHITTESTINFO lvh = {};
		lvh.pt = ptMsg;
		if((this->HitTest(&lvh) != -1) && (lvh.flags == LVHT_ONITEMSTATEICON) && (::GetKeyState(VK_CONTROL) >= 0))
		{
			T* pT = static_cast<T*>(this);
			pT->CheckSelectedItems(lvh.iItem);
		}
		bHandled = FALSE;
		return 1;
	}

	LRESULT OnKeyDown(UINT /*uMsg*/, WPARAM wParam, LPARAM /*lParam*/, BOOL& bHandled)
	{
		if(wParam == VK_SPACE)
		{
			int nCurrItem = this->GetNextItem(-1, LVNI_FOCUSED);
			if((nCurrItem != -1)  && (::GetKeyState(VK_CONTROL) >= 0))
			{
				T* pT = static_cast<T*>(this);
				pT->CheckSelectedItems(nCurrItem);
			}
		}
		bHandled = FALSE;
		return 1;
	}
};

class CCheckListViewCtrl : public CCheckListViewCtrlImpl<CCheckListViewCtrl>
{
public:
	DECLARE_WND_SUPERCLASS(_T("WTL_CheckListView"), GetWndClassName())
};


///////////////////////////////////////////////////////////////////////////////
// CHyperLink - hyper link control implementation

#define HLINK_UNDERLINED           0x00000000
#define HLINK_NOTUNDERLINED        0x00000001
#define HLINK_UNDERLINEHOVER       0x00000002
#define HLINK_COMMANDBUTTON        0x00000004
#define HLINK_NOTIFYBUTTON         0x0000000C
#define HLINK_USETAGS              0x00000010
#define HLINK_USETAGSBOLD          0x00000030
#define HLINK_NOTOOLTIP            0x00000040
#define HLINK_AUTOCREATELINKFONT   0x00000080
#define HLINK_SINGLELINE           0x00000100

// Notes:
// - HLINK_USETAGS and HLINK_USETAGSBOLD are always left-aligned
// - When HLINK_USETAGSBOLD is used, the underlined styles will be ignored

template <class T, class TBase = ATL::CWindow, class TWinTraits = ATL::CControlWinTraits>
class ATL_NO_VTABLE CHyperLinkImpl : public ATL::CWindowImpl< T, TBase, TWinTraits >
{
public:
	LPTSTR m_lpstrLabel;
	LPTSTR m_lpstrHyperLink;

	HCURSOR m_hCursor;
	HFONT m_hFontLink;
	HFONT m_hFontNormal;

	RECT m_rcLink;
	CToolTipCtrl m_tip;

	COLORREF m_clrLink;
	COLORREF m_clrVisited;

	DWORD m_dwExtendedStyle;   // Hyper Link specific extended styles

	bool m_bPaintLabel:1;
	bool m_bVisited:1;
	bool m_bHover:1;
	bool m_bInternalLinkFont:1;
	bool m_bInternalNormalFont:1;


// Constructor/Destructor
	CHyperLinkImpl(DWORD dwExtendedStyle = HLINK_UNDERLINED) : 
			m_lpstrLabel(NULL), m_lpstrHyperLink(NULL),
			m_hCursor(NULL), m_hFontLink(NULL), m_hFontNormal(NULL),
			m_clrLink(RGB(0, 0, 255)), m_clrVisited(RGB(128, 0, 128)),
			m_dwExtendedStyle(dwExtendedStyle),
			m_bPaintLabel(true), m_bVisited(false),
			m_bHover(false), m_bInternalLinkFont(false), m_bInternalNormalFont(false)
	{
		::SetRectEmpty(&m_rcLink);
	}

	~CHyperLinkImpl()
	{
		delete [] m_lpstrLabel;
		delete [] m_lpstrHyperLink;
	}

// Attributes
	DWORD GetHyperLinkExtendedStyle() const
	{
		return m_dwExtendedStyle;
	}

	DWORD SetHyperLinkExtendedStyle(DWORD dwExtendedStyle, DWORD dwMask = 0)
	{
		DWORD dwPrevStyle = m_dwExtendedStyle;
		if(dwMask == 0)
			m_dwExtendedStyle = dwExtendedStyle;
		else
			m_dwExtendedStyle = (m_dwExtendedStyle & ~dwMask) | (dwExtendedStyle & dwMask);
		return dwPrevStyle;
	}

	bool GetLabel(LPTSTR lpstrBuffer, int nLength) const
	{
		if(m_lpstrLabel == NULL)
			return false;
		ATLASSERT(lpstrBuffer != NULL);
		if(nLength <= lstrlen(m_lpstrLabel))
			return false;

		ATL::Checked::tcscpy_s(lpstrBuffer, nLength, m_lpstrLabel);

		return true;
	}

	bool SetLabel(LPCTSTR lpstrLabel)
	{
		delete [] m_lpstrLabel;
		m_lpstrLabel = NULL;
		int cchLen = lstrlen(lpstrLabel) + 1;
		ATLTRY(m_lpstrLabel = new TCHAR[cchLen]);
		if(m_lpstrLabel == NULL)
			return false;

		ATL::Checked::tcscpy_s(m_lpstrLabel, cchLen, lpstrLabel);
		T* pT = static_cast<T*>(this);
		pT->CalcLabelRect();

		if(this->m_hWnd != NULL)
			this->SetWindowText(lpstrLabel);   // Set this for accessibility

		return true;
	}

	bool GetHyperLink(LPTSTR lpstrBuffer, int nLength) const
	{
		if(m_lpstrHyperLink == NULL)
			return false;
		ATLASSERT(lpstrBuffer != NULL);
		if(nLength <= lstrlen(m_lpstrHyperLink))
			return false;

		ATL::Checked::tcscpy_s(lpstrBuffer, nLength, m_lpstrHyperLink);

		return true;
	}

	bool SetHyperLink(LPCTSTR lpstrLink)
	{
		delete [] m_lpstrHyperLink;
		m_lpstrHyperLink = NULL;
		int cchLen = lstrlen(lpstrLink) + 1;
		ATLTRY(m_lpstrHyperLink = new TCHAR[cchLen]);
		if(m_lpstrHyperLink == NULL)
			return false;

		ATL::Checked::tcscpy_s(m_lpstrHyperLink, cchLen, lpstrLink);
		if(m_lpstrLabel == NULL)
		{
			T* pT = static_cast<T*>(this);
			pT->CalcLabelRect();
		}

		if(m_tip.IsWindow())
		{
			m_tip.Activate(TRUE);
			m_tip.AddTool(this->m_hWnd, m_lpstrHyperLink, &m_rcLink, 1);
		}

		return true;
	}

	HFONT GetLinkFont() const
	{
		return m_hFontLink;
	}

	void SetLinkFont(HFONT hFont)
	{
		if(m_bInternalLinkFont)
		{
			::DeleteObject(m_hFontLink);
			m_bInternalLinkFont = false;
		}

		m_hFontLink = hFont;

		T* pT = static_cast<T*>(this);
		pT->CalcLabelRect();
	}

	int GetIdealHeight() const
	{
		ATLASSERT(::IsWindow(this->m_hWnd));
		if((m_lpstrLabel == NULL) && (m_lpstrHyperLink == NULL))
			return -1;
		if(!m_bPaintLabel)
			return -1;

		UINT uFormat = IsSingleLine() ? DT_SINGLELINE : DT_WORDBREAK;

		CClientDC dc(this->m_hWnd);
		RECT rect = {};
		this->GetClientRect(&rect);
		HFONT hFontOld = dc.SelectFont(m_hFontNormal);
		RECT rcText = rect;
		dc.DrawText(_T("NS"), -1, &rcText, DT_LEFT | uFormat | DT_CALCRECT);
		dc.SelectFont(m_hFontLink);
		RECT rcLink = rect;
		dc.DrawText(_T("NS"), -1, &rcLink, DT_LEFT | uFormat | DT_CALCRECT);
		dc.SelectFont(hFontOld);
		return __max(rcText.bottom - rcText.top, rcLink.bottom - rcLink.top);
	}

	bool GetIdealSize(SIZE& size) const
	{
		int cx = 0, cy = 0;
		bool bRet = GetIdealSize(cx, cy);
		if(bRet)
		{
			size.cx = cx;
			size.cy = cy;
		}
		return bRet;
	}

	bool GetIdealSize(int& cx, int& cy) const
	{
		ATLASSERT(::IsWindow(this->m_hWnd));
		if((m_lpstrLabel == NULL) && (m_lpstrHyperLink == NULL))
			return false;
		if(!m_bPaintLabel)
			return false;

		CClientDC dc(this->m_hWnd);
		RECT rcClient = {};
		this->GetClientRect(&rcClient);
		RECT rcAll = rcClient;

		if(IsUsingTags())
		{
			// find tags and label parts
			LPTSTR lpstrLeft = NULL;
			int cchLeft = 0;
			LPTSTR lpstrLink = NULL;
			int cchLink = 0;
			LPTSTR lpstrRight = NULL;
			int cchRight = 0;

			const T* pT = static_cast<const T*>(this);
			pT->CalcLabelParts(lpstrLeft, cchLeft, lpstrLink, cchLink, lpstrRight, cchRight);

			// get label part rects
			UINT uFormat = IsSingleLine() ? DT_SINGLELINE : DT_WORDBREAK;

			HFONT hFontOld = dc.SelectFont(m_hFontNormal);
			RECT rcLeft = rcClient;
			dc.DrawText(lpstrLeft, cchLeft, &rcLeft, DT_LEFT | uFormat | DT_CALCRECT);

			dc.SelectFont(m_hFontLink);
			RECT rcLink = { rcLeft.right, rcLeft.top, rcClient.right, rcClient.bottom };
			dc.DrawText(lpstrLink, cchLink, &rcLink, DT_LEFT | uFormat | DT_CALCRECT);

			dc.SelectFont(m_hFontNormal);
			RECT rcRight = { rcLink.right, rcLink.top, rcClient.right, rcClient.bottom };
			dc.DrawText(lpstrRight, cchRight, &rcRight, DT_LEFT | uFormat | DT_CALCRECT);

			dc.SelectFont(hFontOld);

			int cyMax = __max(rcLeft.bottom, __max(rcLink.bottom, rcRight.bottom));
			::SetRect(&rcAll, rcLeft.left, rcLeft.top, rcRight.right, cyMax);
		}
		else
		{
			HFONT hOldFont = NULL;
			if(m_hFontLink != NULL)
				hOldFont = dc.SelectFont(m_hFontLink);
			LPTSTR lpstrText = (m_lpstrLabel != NULL) ? m_lpstrLabel : m_lpstrHyperLink;
			DWORD dwStyle = this->GetStyle();
			UINT uFormat = DT_LEFT;
			if (dwStyle & SS_CENTER)
				uFormat = DT_CENTER;
			else if (dwStyle & SS_RIGHT)
				uFormat = DT_RIGHT;
			uFormat |= IsSingleLine() ? DT_SINGLELINE : DT_WORDBREAK;
			dc.DrawText(lpstrText, -1, &rcAll, uFormat | DT_CALCRECT);
			if(m_hFontLink != NULL)
				dc.SelectFont(hOldFont);
			if (dwStyle & SS_CENTER)
			{
				int dx = (rcClient.right - rcAll.right) / 2;
				::OffsetRect(&rcAll, dx, 0);
			}
			else if (dwStyle & SS_RIGHT)
			{
				int dx = rcClient.right - rcAll.right;
				::OffsetRect(&rcAll, dx, 0);
			}
		}

		cx = rcAll.right - rcAll.left;
		cy = rcAll.bottom - rcAll.top;

		return true;
	}

	// for command buttons only
	bool GetToolTipText(LPTSTR lpstrBuffer, int nLength) const
	{
		ATLASSERT(IsCommandButton());
		return GetHyperLink(lpstrBuffer, nLength);
	}

	bool SetToolTipText(LPCTSTR lpstrToolTipText)
	{
		ATLASSERT(IsCommandButton());
		return SetHyperLink(lpstrToolTipText);
	}

// Operations
	BOOL SubclassWindow(HWND hWnd)
	{
		ATLASSERT(this->m_hWnd == NULL);
		ATLASSERT(::IsWindow(hWnd));
		if(m_hFontNormal == NULL)
			m_hFontNormal = (HFONT)::SendMessage(hWnd, WM_GETFONT, 0, 0L);

		BOOL bRet = ATL::CWindowImpl< T, TBase, TWinTraits >::SubclassWindow(hWnd);
		if(bRet != FALSE)
		{
			T* pT = static_cast<T*>(this);
			pT->Init();
		}

		return bRet;
	}

	bool Navigate()
	{
		ATLASSERT(::IsWindow(this->m_hWnd));
		bool bRet = true;
		if(IsNotifyButton())
		{
			NMHDR nmhdr = { this->m_hWnd, (UINT_PTR)this->GetDlgCtrlID(), NM_CLICK };
			this->GetParent().SendMessage(WM_NOTIFY, this->GetDlgCtrlID(), (LPARAM)&nmhdr);
		}
		else if(IsCommandButton())
		{
			this->GetParent().SendMessage(WM_COMMAND, MAKEWPARAM(this->GetDlgCtrlID(), BN_CLICKED), (LPARAM)this->m_hWnd);
		}
		else
		{
			ATLASSERT(m_lpstrHyperLink != NULL);
			DWORD_PTR dwRet = (DWORD_PTR)::ShellExecute(0, _T("open"), m_lpstrHyperLink, 0, 0, SW_SHOWNORMAL);
			bRet = (dwRet > 32);
			ATLASSERT(bRet);
			if(bRet)
			{
				m_bVisited = true;
				this->Invalidate();
			}
		}
		return bRet;
	}

	void CreateLinkFontFromNormal()
	{
		if(m_bInternalLinkFont)
		{
			::DeleteObject(m_hFontLink);
			m_bInternalLinkFont = false;
		}

		CFontHandle font = (m_hFontNormal != NULL) ? m_hFontNormal : (HFONT)::GetStockObject(SYSTEM_FONT);
		LOGFONT lf = {};
		font.GetLogFont(&lf);

		if(IsUsingTagsBold())
			lf.lfWeight = FW_BOLD;
		else if(!IsNotUnderlined())
			lf.lfUnderline = TRUE;

		m_hFontLink = ::CreateFontIndirect(&lf);
		m_bInternalLinkFont = true;
		ATLASSERT(m_hFontLink != NULL);
	}

// Message map and handlers
	BEGIN_MSG_MAP(CHyperLinkImpl)
		MESSAGE_HANDLER(WM_CREATE, OnCreate)
		MESSAGE_HANDLER(WM_DESTROY, OnDestroy)
		MESSAGE_RANGE_HANDLER(WM_MOUSEFIRST, WM_MOUSELAST, OnMouseMessage)
		MESSAGE_HANDLER(WM_ERASEBKGND, OnEraseBackground)
		MESSAGE_HANDLER(WM_PAINT, OnPaint)
		MESSAGE_HANDLER(WM_PRINTCLIENT, OnPaint)
		MESSAGE_HANDLER(WM_SETFOCUS, OnFocus)
		MESSAGE_HANDLER(WM_KILLFOCUS, OnFocus)
		MESSAGE_HANDLER(WM_MOUSEMOVE, OnMouseMove)
		MESSAGE_HANDLER(WM_MOUSELEAVE, OnMouseLeave)
		MESSAGE_HANDLER(WM_LBUTTONDOWN, OnLButtonDown)
		MESSAGE_HANDLER(WM_LBUTTONUP, OnLButtonUp)
		MESSAGE_HANDLER(WM_CHAR, OnChar)
		MESSAGE_HANDLER(WM_GETDLGCODE, OnGetDlgCode)
		MESSAGE_HANDLER(WM_SETCURSOR, OnSetCursor)
		MESSAGE_HANDLER(WM_ENABLE, OnEnable)
		MESSAGE_HANDLER(WM_GETFONT, OnGetFont)
		MESSAGE_HANDLER(WM_SETFONT, OnSetFont)
		MESSAGE_HANDLER(WM_UPDATEUISTATE, OnUpdateUiState)
		MESSAGE_HANDLER(WM_SIZE, OnSize)
	END_MSG_MAP()

	LRESULT OnCreate(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		T* pT = static_cast<T*>(this);
		pT->Init();
		return 0;
	}

	LRESULT OnDestroy(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& bHandled)
	{
		if(m_tip.IsWindow())
		{
			m_tip.DestroyWindow();
			m_tip.m_hWnd = NULL;
		}

		if(m_bInternalLinkFont)
		{
			::DeleteObject(m_hFontLink);
			m_hFontLink = NULL;
			m_bInternalLinkFont = false;
		}

		if(m_bInternalNormalFont)
		{
			::DeleteObject(m_hFontNormal);
			m_hFontNormal = NULL;
			m_bInternalNormalFont = false;
		}

		bHandled = FALSE;
		return 1;
	}

	LRESULT OnMouseMessage(UINT uMsg, WPARAM wParam, LPARAM lParam, BOOL& bHandled)
	{
		MSG msg = { this->m_hWnd, uMsg, wParam, lParam };
		if(m_tip.IsWindow() && IsUsingToolTip())
			m_tip.RelayEvent(&msg);
		bHandled = FALSE;
		return 1;
	}

	LRESULT OnEraseBackground(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		return 1;   // no background painting needed (we do it all during WM_PAINT)
	}

	LRESULT OnPaint(UINT /*uMsg*/, WPARAM wParam, LPARAM /*lParam*/, BOOL& bHandled)
	{
		if(!m_bPaintLabel)
		{
			bHandled = FALSE;
			return 1;
		}

		T* pT = static_cast<T*>(this);
		if(wParam != NULL)
		{
			pT->DoEraseBackground((HDC)wParam);
			pT->DoPaint((HDC)wParam);
		}
		else
		{
			CPaintDC dc(this->m_hWnd);
			pT->DoEraseBackground(dc.m_hDC);
			pT->DoPaint(dc.m_hDC);
		}

		return 0;
	}

	LRESULT OnFocus(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& bHandled)
	{
		if(m_bPaintLabel)
			this->Invalidate();
		else
			bHandled = FALSE;
		return 0;
	}

	LRESULT OnMouseMove(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM lParam, BOOL& bHandled)
	{
		POINT pt = { GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam) };
		if(((m_lpstrHyperLink != NULL) || IsCommandButton()) && ::PtInRect(&m_rcLink, pt))
		{
			::SetCursor(m_hCursor);
			if(IsUnderlineHover())
			{
				if(!m_bHover)
				{
					m_bHover = true;
					this->InvalidateRect(&m_rcLink);
					this->UpdateWindow();
					StartTrackMouseLeave();
				}
			}
		}
		else
		{
			if(IsUnderlineHover())
			{
				if(m_bHover)
				{
					m_bHover = false;
					this->InvalidateRect(&m_rcLink);
					this->UpdateWindow();
				}
			}
			bHandled = FALSE;
		}
		return 0;
	}

	LRESULT OnMouseLeave(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		if(IsUnderlineHover() && m_bHover)
		{
			m_bHover = false;
			this->InvalidateRect(&m_rcLink);
			this->UpdateWindow();
		}
		return 0;
	}

	LRESULT OnLButtonDown(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM lParam, BOOL& /*bHandled*/)
	{
		POINT pt = { GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam) };
		if(::PtInRect(&m_rcLink, pt))
		{
			this->SetFocus();
			this->SetCapture();
		}
		return 0;
	}

	LRESULT OnLButtonUp(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM lParam, BOOL& /*bHandled*/)
	{
		if(GetCapture() == this->m_hWnd)
		{
			ReleaseCapture();
			POINT pt = { GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam) };
			if(::PtInRect(&m_rcLink, pt))
			{
				T* pT = static_cast<T*>(this);
				pT->Navigate();
			}
		}
		return 0;
	}

	LRESULT OnChar(UINT /*uMsg*/, WPARAM wParam, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		if((wParam == VK_RETURN) || (wParam == VK_SPACE))
		{
			T* pT = static_cast<T*>(this);
			pT->Navigate();
		}
		return 0;
	}

	LRESULT OnGetDlgCode(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		return DLGC_WANTCHARS;
	}

	LRESULT OnSetCursor(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& bHandled)
	{
		POINT pt = {};
		GetCursorPos(&pt);
		this->ScreenToClient(&pt);
		if(((m_lpstrHyperLink != NULL)  || IsCommandButton()) && ::PtInRect(&m_rcLink, pt))
		{
			return TRUE;
		}
		bHandled = FALSE;
		return FALSE;
	}

	LRESULT OnEnable(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		this->Invalidate();
		this->UpdateWindow();
		return 0;
	}

	LRESULT OnGetFont(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		return (LRESULT)m_hFontNormal;
	}

	LRESULT OnSetFont(UINT /*uMsg*/, WPARAM wParam, LPARAM lParam, BOOL& /*bHandled*/)
	{
		if(m_bInternalNormalFont)
		{
			::DeleteObject(m_hFontNormal);
			m_bInternalNormalFont = false;
		}

		bool bCreateLinkFont = m_bInternalLinkFont;

		m_hFontNormal = (HFONT)wParam;

		if(bCreateLinkFont || IsAutoCreateLinkFont())
			CreateLinkFontFromNormal();

		T* pT = static_cast<T*>(this);
		pT->CalcLabelRect();

		if((BOOL)lParam)
		{
			this->Invalidate();
			this->UpdateWindow();
		}

		return 0;
	}

	LRESULT OnUpdateUiState(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		// If the control is subclassed or superclassed, this message can cause
		// repainting without WM_PAINT. We don't use this state, so just do nothing.
		return 0;
	}

	LRESULT OnSize(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		T* pT = static_cast<T*>(this);
		pT->CalcLabelRect();
		pT->Invalidate();
		return 0;
	}

// Implementation
	void Init()
	{
		ATLASSERT(::IsWindow(this->m_hWnd));

		// Check if we should paint a label
		const int cchBuff = 8;
		TCHAR szBuffer[cchBuff] = {};
		if(::GetClassName(this->m_hWnd, szBuffer, cchBuff))
		{
			if(lstrcmpi(szBuffer, _T("static")) == 0)
			{
				this->ModifyStyle(0, SS_NOTIFY);   // we need this
				DWORD dwStyle = this->GetStyle() & 0x000000FF;
				if((dwStyle == SS_ICON) || (dwStyle == SS_BLACKRECT) || (dwStyle == SS_GRAYRECT) || 
						(dwStyle == SS_WHITERECT) || (dwStyle == SS_BLACKFRAME) || (dwStyle == SS_GRAYFRAME) || 
						(dwStyle == SS_WHITEFRAME) || (dwStyle == SS_OWNERDRAW) || 
						(dwStyle == SS_BITMAP) || (dwStyle == SS_ENHMETAFILE))
					m_bPaintLabel = false;
			}
		}

		// create or load a cursor
		m_hCursor = ::LoadCursor(NULL, IDC_HAND);
		ATLASSERT(m_hCursor != NULL);

		// set fonts
		if(m_bPaintLabel)
		{
			if(m_hFontNormal == NULL)
			{
				m_hFontNormal = AtlCreateControlFont();
				m_bInternalNormalFont = true;
			}

			if(m_hFontLink == NULL)
				CreateLinkFontFromNormal();
		}

		// create a tool tip
		m_tip.Create(this->m_hWnd);
		ATLASSERT(m_tip.IsWindow());

		// set label (defaults to window text)
		if(m_lpstrLabel == NULL)
		{
			int nLen = this->GetWindowTextLength();
			if(nLen > 0)
			{
				ATLTRY(m_lpstrLabel = new TCHAR[nLen + 1]);
				if(m_lpstrLabel != NULL)
					ATLVERIFY(this->GetWindowText(m_lpstrLabel, nLen + 1) > 0);
			}
		}

		T* pT = static_cast<T*>(this);
		pT->CalcLabelRect();

		// set hyperlink (defaults to label), or just activate tool tip if already set
		if((m_lpstrHyperLink == NULL) && !IsCommandButton())
		{
			if(m_lpstrLabel != NULL)
				SetHyperLink(m_lpstrLabel);
		}
		else
		{
			m_tip.Activate(TRUE);
			m_tip.AddTool(this->m_hWnd, m_lpstrHyperLink, &m_rcLink, 1);
		}

		// set link colors
		if(m_bPaintLabel)
		{
			ATL::CRegKey rk;
			LONG lRet = rk.Open(HKEY_CURRENT_USER, _T("Software\\Microsoft\\Internet Explorer\\Settings"));
			if(lRet == ERROR_SUCCESS)
			{
				const int cchValue = 12;
				TCHAR szValue[cchValue] = {};
				ULONG ulCount = cchValue;
				lRet = rk.QueryStringValue(_T("Anchor Color"), szValue, &ulCount);
				if(lRet == ERROR_SUCCESS)
				{
					COLORREF clr = pT->_ParseColorString(szValue);
					ATLASSERT(clr != CLR_INVALID);
					if(clr != CLR_INVALID)
						m_clrLink = clr;
				}

				ulCount = cchValue;
				lRet = rk.QueryStringValue(_T("Anchor Color Visited"), szValue, &ulCount);
				if(lRet == ERROR_SUCCESS)
				{
					COLORREF clr = pT->_ParseColorString(szValue);
					ATLASSERT(clr != CLR_INVALID);
					if(clr != CLR_INVALID)
						m_clrVisited = clr;
				}
			}
		}
	}

	static COLORREF _ParseColorString(LPTSTR lpstr)
	{
		int c[3] = { -1, -1, -1 };
		LPTSTR p = NULL;
		for(int i = 0; i < 2; i++)
		{
			for(p = lpstr; *p != _T('\0'); p = ::CharNext(p))
			{
				if(*p == _T(','))
				{
					*p = _T('\0');
					c[i] = _ttoi(lpstr);
					lpstr = &p[1];
					break;
				}
			}
			if(c[i] == -1)
				return CLR_INVALID;
		}
		if(*lpstr == _T('\0'))
			return CLR_INVALID;
		c[2] = _ttoi(lpstr);

		return RGB(c[0], c[1], c[2]);
	}

	bool CalcLabelRect()
	{
		if(!::IsWindow(this->m_hWnd))
			return false;
		if((m_lpstrLabel == NULL) && (m_lpstrHyperLink == NULL))
			return false;

		CClientDC dc(this->m_hWnd);
		RECT rcClient = {};
		this->GetClientRect(&rcClient);
		m_rcLink = rcClient;
		if(!m_bPaintLabel)
			return true;

		if(IsUsingTags())
		{
			// find tags and label parts
			LPTSTR lpstrLeft = NULL;
			int cchLeft = 0;
			LPTSTR lpstrLink = NULL;
			int cchLink = 0;
			LPTSTR lpstrRight = NULL;
			int cchRight = 0;

			T* pT = static_cast<T*>(this);
			pT->CalcLabelParts(lpstrLeft, cchLeft, lpstrLink, cchLink, lpstrRight, cchRight);
			ATLASSERT(lpstrLink != NULL);
			ATLASSERT(cchLink > 0);

			// get label part rects
			HFONT hFontOld = dc.SelectFont(m_hFontNormal);

			UINT uFormat = IsSingleLine() ? DT_SINGLELINE : DT_WORDBREAK;

			RECT rcLeft = rcClient;
			if(lpstrLeft != NULL)
				dc.DrawText(lpstrLeft, cchLeft, &rcLeft, DT_LEFT | uFormat | DT_CALCRECT);

			dc.SelectFont(m_hFontLink);
			RECT rcLink = rcClient;
			if(lpstrLeft != NULL)
				rcLink.left = rcLeft.right;
			dc.DrawText(lpstrLink, cchLink, &rcLink, DT_LEFT | uFormat | DT_CALCRECT);

			dc.SelectFont(hFontOld);

			m_rcLink = rcLink;
		}
		else
		{
			HFONT hOldFont = NULL;
			if(m_hFontLink != NULL)
				hOldFont = dc.SelectFont(m_hFontLink);
			LPTSTR lpstrText = (m_lpstrLabel != NULL) ? m_lpstrLabel : m_lpstrHyperLink;
			DWORD dwStyle = this->GetStyle();
			UINT uFormat = DT_LEFT;
			if (dwStyle & SS_CENTER)
				uFormat = DT_CENTER;
			else if (dwStyle & SS_RIGHT)
				uFormat = DT_RIGHT;
			uFormat |= IsSingleLine() ? DT_SINGLELINE : DT_WORDBREAK;
			dc.DrawText(lpstrText, -1, &m_rcLink, uFormat | DT_CALCRECT);
			if(m_hFontLink != NULL)
				dc.SelectFont(hOldFont);
			if (dwStyle & SS_CENTER)
			{
				int dx = (rcClient.right - m_rcLink.right) / 2;
				::OffsetRect(&m_rcLink, dx, 0);
			}
			else if (dwStyle & SS_RIGHT)
			{
				int dx = rcClient.right - m_rcLink.right;
				::OffsetRect(&m_rcLink, dx, 0);
			}
		}

		return true;
	}

	void CalcLabelParts(LPTSTR& lpstrLeft, int& cchLeft, LPTSTR& lpstrLink, int& cchLink, LPTSTR& lpstrRight, int& cchRight) const
	{
		lpstrLeft = NULL;
		cchLeft = 0;
		lpstrLink = NULL;
		cchLink = 0;
		lpstrRight = NULL;
		cchRight = 0;

		LPTSTR lpstrText = (m_lpstrLabel != NULL) ? m_lpstrLabel : m_lpstrHyperLink;
		int cchText = lstrlen(lpstrText);
		bool bOutsideLink = true;
		for(int i = 0; i < cchText; i++)
		{
			if(lpstrText[i] != _T('<'))
				continue;

			if(bOutsideLink)
			{
				if(::CompareString(LOCALE_USER_DEFAULT, NORM_IGNORECASE, &lpstrText[i], 3, _T("<A>"), 3) == CSTR_EQUAL)
				{
					if(i > 0)
					{
						lpstrLeft = lpstrText;
						cchLeft = i;
					}
					lpstrLink = &lpstrText[i + 3];
					bOutsideLink = false;
				}
			}
			else
			{
				if(::CompareString(LOCALE_USER_DEFAULT, NORM_IGNORECASE, &lpstrText[i], 4, _T("</A>"), 4) == CSTR_EQUAL)
				{
					cchLink = i - 3 - cchLeft;
					if(lpstrText[i + 4] != 0)
					{
						lpstrRight = &lpstrText[i + 4];
						cchRight = cchText - (i + 4);
						break;
					}
				}
			}
		}

	}

	void DoEraseBackground(CDCHandle dc)
	{
		HBRUSH hBrush = (HBRUSH)this->GetParent().SendMessage(WM_CTLCOLORSTATIC, (WPARAM)dc.m_hDC, (LPARAM)this->m_hWnd);
		if(hBrush != NULL)
		{
			RECT rect = {};
			this->GetClientRect(&rect);
			dc.FillRect(&rect, hBrush);
		}
	}

	void DoPaint(CDCHandle dc)
	{
		if(IsUsingTags())
		{
			// find tags and label parts
			LPTSTR lpstrLeft = NULL;
			int cchLeft = 0;
			LPTSTR lpstrLink = NULL;
			int cchLink = 0;
			LPTSTR lpstrRight = NULL;
			int cchRight = 0;

			T* pT = static_cast<T*>(this);
			pT->CalcLabelParts(lpstrLeft, cchLeft, lpstrLink, cchLink, lpstrRight, cchRight);

			// get label part rects
			RECT rcClient = {};
			this->GetClientRect(&rcClient);

			dc.SetBkMode(TRANSPARENT);
			HFONT hFontOld = dc.SelectFont(m_hFontNormal);

			UINT uFormat = IsSingleLine() ? DT_SINGLELINE : DT_WORDBREAK;

			if(lpstrLeft != NULL)
				dc.DrawText(lpstrLeft, cchLeft, &rcClient, DT_LEFT | uFormat);

			COLORREF clrOld = dc.SetTextColor(this->IsWindowEnabled() ? (m_bVisited ? m_clrVisited : m_clrLink) : (::GetSysColor(COLOR_GRAYTEXT)));
			if((m_hFontLink != NULL) && (!IsUnderlineHover() || (IsUnderlineHover() && m_bHover)))
				dc.SelectFont(m_hFontLink);
			else
				dc.SelectFont(m_hFontNormal);

			dc.DrawText(lpstrLink, cchLink, &m_rcLink, DT_LEFT | uFormat);

			dc.SetTextColor(clrOld);
			dc.SelectFont(m_hFontNormal);
			if(lpstrRight != NULL)
			{
				RECT rcRight = { m_rcLink.right, m_rcLink.top, rcClient.right, rcClient.bottom };
				dc.DrawText(lpstrRight, cchRight, &rcRight, DT_LEFT | uFormat);
			}

			if(GetFocus() == this->m_hWnd)
				dc.DrawFocusRect(&m_rcLink);

			dc.SelectFont(hFontOld);
		}
		else
		{
			dc.SetBkMode(TRANSPARENT);
			COLORREF clrOld = dc.SetTextColor(this->IsWindowEnabled() ? (m_bVisited ? m_clrVisited : m_clrLink) : (::GetSysColor(COLOR_GRAYTEXT)));

			HFONT hFontOld = NULL;
			if((m_hFontLink != NULL) && (!IsUnderlineHover() || (IsUnderlineHover() && m_bHover)))
				hFontOld = dc.SelectFont(m_hFontLink);
			else
				hFontOld = dc.SelectFont(m_hFontNormal);

			LPTSTR lpstrText = (m_lpstrLabel != NULL) ? m_lpstrLabel : m_lpstrHyperLink;

			DWORD dwStyle = this->GetStyle();
			UINT uFormat = DT_LEFT;
			if (dwStyle & SS_CENTER)
				uFormat = DT_CENTER;
			else if (dwStyle & SS_RIGHT)
				uFormat = DT_RIGHT;
			uFormat |= IsSingleLine() ? DT_SINGLELINE : DT_WORDBREAK;

			dc.DrawText(lpstrText, -1, &m_rcLink, uFormat);

			if(GetFocus() == this->m_hWnd)
				dc.DrawFocusRect(&m_rcLink);

			dc.SetTextColor(clrOld);
			dc.SelectFont(hFontOld);
		}
	}

	BOOL StartTrackMouseLeave()
	{
		TRACKMOUSEEVENT tme = {};
		tme.cbSize = sizeof(tme);
		tme.dwFlags = TME_LEAVE;
		tme.hwndTrack = this->m_hWnd;
		return ::TrackMouseEvent(&tme);
	}

// Implementation helpers
	bool IsUnderlined() const
	{
		return ((m_dwExtendedStyle & (HLINK_NOTUNDERLINED | HLINK_UNDERLINEHOVER)) == 0);
	}

	bool IsNotUnderlined() const
	{
		return ((m_dwExtendedStyle & HLINK_NOTUNDERLINED) != 0);
	}

	bool IsUnderlineHover() const
	{
		return ((m_dwExtendedStyle & HLINK_UNDERLINEHOVER) != 0);
	}

	bool IsCommandButton() const
	{
		return ((m_dwExtendedStyle & HLINK_COMMANDBUTTON) != 0);
	}

	bool IsNotifyButton() const
	{
		return ((m_dwExtendedStyle & HLINK_NOTIFYBUTTON) == HLINK_NOTIFYBUTTON);
	}

	bool IsUsingTags() const
	{
		return ((m_dwExtendedStyle & HLINK_USETAGS) != 0);
	}

	bool IsUsingTagsBold() const
	{
		return ((m_dwExtendedStyle & HLINK_USETAGSBOLD) == HLINK_USETAGSBOLD);
	}

	bool IsUsingToolTip() const
	{
		return ((m_dwExtendedStyle & HLINK_NOTOOLTIP) == 0);
	}

	bool IsAutoCreateLinkFont() const
	{
		return ((m_dwExtendedStyle & HLINK_AUTOCREATELINKFONT) == HLINK_AUTOCREATELINKFONT);
	}

	bool IsSingleLine() const
	{
		return ((m_dwExtendedStyle & HLINK_SINGLELINE) == HLINK_SINGLELINE);
	}
};

class CHyperLink : public CHyperLinkImpl<CHyperLink>
{
public:
	DECLARE_WND_CLASS(_T("WTL_HyperLink"))
};


///////////////////////////////////////////////////////////////////////////////
// CWaitCursor - displays a wait cursor

class CWaitCursor
{
public:
// Data
	HCURSOR m_hWaitCursor;
	HCURSOR m_hOldCursor;
	bool m_bInUse;

// Constructor/destructor
	CWaitCursor(bool bSet = true, LPCTSTR lpstrCursor = IDC_WAIT, bool bSys = true) : m_hOldCursor(NULL), m_bInUse(false)
	{
		HINSTANCE hInstance = bSys ? NULL : ModuleHelper::GetResourceInstance();
		m_hWaitCursor = ::LoadCursor(hInstance, lpstrCursor);
		ATLASSERT(m_hWaitCursor != NULL);

		if(bSet)
			Set();
	}

	~CWaitCursor()
	{
		Restore();
	}

// Methods
	bool Set()
	{
		if(m_bInUse)
			return false;
		m_hOldCursor = ::SetCursor(m_hWaitCursor);
		m_bInUse = true;
		return true;
	}

	bool Restore()
	{
		if(!m_bInUse)
			return false;
		::SetCursor(m_hOldCursor);
		m_bInUse = false;
		return true;
	}
};


///////////////////////////////////////////////////////////////////////////////
// CCustomWaitCursor - for custom and animated cursors

class CCustomWaitCursor : public CWaitCursor
{
public:
// Constructor/destructor
	CCustomWaitCursor(ATL::_U_STRINGorID cursor, bool bSet = true, HINSTANCE hInstance = NULL) : 
			CWaitCursor(false, IDC_WAIT, true)
	{
		if(hInstance == NULL)
			hInstance = ModuleHelper::GetResourceInstance();
		m_hWaitCursor = (HCURSOR)::LoadImage(hInstance, cursor.m_lpstr, IMAGE_CURSOR, 0, 0, LR_DEFAULTSIZE);

		if(bSet)
			Set();
	}

	~CCustomWaitCursor()
	{
		Restore();
		::DestroyCursor(m_hWaitCursor);
	}
};


///////////////////////////////////////////////////////////////////////////////
// CMultiPaneStatusBarCtrl - Status Bar with multiple panes

template <class T, class TBase = CStatusBarCtrl>
class ATL_NO_VTABLE CMultiPaneStatusBarCtrlImpl : public ATL::CWindowImpl< T, TBase >
{
public:
	DECLARE_WND_SUPERCLASS2(NULL, T, TBase::GetWndClassName())

// Data
	enum { m_cxPaneMargin = 3 };

	int m_nPanes;
	int* m_pPane;

// Constructor/destructor
	CMultiPaneStatusBarCtrlImpl() : m_nPanes(0), m_pPane(NULL)
	{ }

	~CMultiPaneStatusBarCtrlImpl()
	{
		delete [] m_pPane;
	}

// Methods
	HWND Create(HWND hWndParent, LPCTSTR lpstrText, DWORD dwStyle = WS_CHILD | WS_VISIBLE | WS_CLIPCHILDREN | WS_CLIPSIBLINGS | SBARS_SIZEGRIP, UINT nID = ATL_IDW_STATUS_BAR)
	{
		return ATL::CWindowImpl< T, TBase >::Create(hWndParent, this->rcDefault, lpstrText, dwStyle, 0, nID);
	}

	HWND Create(HWND hWndParent, UINT nTextID = ATL_IDS_IDLEMESSAGE, DWORD dwStyle = WS_CHILD | WS_VISIBLE | WS_CLIPCHILDREN | WS_CLIPSIBLINGS | SBARS_SIZEGRIP, UINT nID = ATL_IDW_STATUS_BAR)
	{
		const int cchMax = 128;   // max text length is 127 for status bars (+1 for null)
		TCHAR szText[cchMax] = {};
		::LoadString(ModuleHelper::GetResourceInstance(), nTextID, szText, cchMax);
		return Create(hWndParent, szText, dwStyle, nID);
	}

	BOOL SetPanes(int* pPanes, int nPanes, bool bSetText = true)
	{
		ATLASSERT(::IsWindow(this->m_hWnd));
		ATLASSERT(nPanes > 0);

		m_nPanes = nPanes;
		delete [] m_pPane;
		m_pPane = NULL;

		ATLTRY(m_pPane = new int[nPanes]);
		ATLASSERT(m_pPane != NULL);
		if(m_pPane == NULL)
			return FALSE;

		ATL::CTempBuffer<int, _WTL_STACK_ALLOC_THRESHOLD> buff;
		int* pPanesPos = buff.Allocate(nPanes);
		ATLASSERT(pPanesPos != NULL);
		if(pPanesPos == NULL)
			return FALSE;

		ATL::Checked::memcpy_s(m_pPane, nPanes * sizeof(int), pPanes, nPanes * sizeof(int));

		// get status bar DC and set font
		CClientDC dc(this->m_hWnd);
		HFONT hOldFont = dc.SelectFont(this->GetFont());

		// get status bar borders
		int arrBorders[3] = {};
		this->GetBorders(arrBorders);

		const int cchBuff = 128;
		TCHAR szBuff[cchBuff] = {};
		int cxLeft = arrBorders[0];

		// calculate right edge of each part
		for(int i = 0; i < nPanes; i++)
		{
			if(pPanes[i] == ID_DEFAULT_PANE)
			{
				// make very large, will be resized later
				pPanesPos[i] = INT_MAX / 2;
			}
			else
			{
				::LoadString(ModuleHelper::GetResourceInstance(), pPanes[i], szBuff, cchBuff);
				SIZE size = {};
				dc.GetTextExtent(szBuff, lstrlen(szBuff), &size);
				T* pT = static_cast<T*>(this);
				(void)pT;   // avoid level 4 warning
				pPanesPos[i] = cxLeft + size.cx + arrBorders[2] + 2 * pT->m_cxPaneMargin;
			}
			cxLeft = pPanesPos[i];
		}

		BOOL bRet = this->SetParts(nPanes, pPanesPos);

		if(bRet && bSetText)
		{
			for(int i = 0; i < nPanes; i++)
			{
				if(pPanes[i] != ID_DEFAULT_PANE)
				{
					::LoadString(ModuleHelper::GetResourceInstance(), pPanes[i], szBuff, cchBuff);
					SetPaneText(m_pPane[i], szBuff);
				}
			}
		}

		dc.SelectFont(hOldFont);
		return bRet;
	}

	bool GetPaneTextLength(int nPaneID, int* pcchLength = NULL, int* pnType = NULL) const
	{
		ATLASSERT(::IsWindow(this->m_hWnd));
		int nIndex  = GetPaneIndexFromID(nPaneID);
		if(nIndex == -1)
			return false;

		int nLength = this->GetTextLength(nIndex, pnType);
		if(pcchLength != NULL)
			*pcchLength = nLength;

		return true;
	}

	BOOL GetPaneText(int nPaneID, LPTSTR lpstrText, int* pcchLength = NULL, int* pnType = NULL) const
	{
		ATLASSERT(::IsWindow(this->m_hWnd));
		int nIndex  = GetPaneIndexFromID(nPaneID);
		if(nIndex == -1)
			return FALSE;

		int nLength = this->GetText(nIndex, lpstrText, pnType);
		if(pcchLength != NULL)
			*pcchLength = nLength;

		return TRUE;
	}

#ifdef __ATLSTR_H__
	BOOL GetPaneText(int nPaneID, ATL::CString& strText, int* pcchLength = NULL, int* pnType = NULL) const
	{
		ATLASSERT(::IsWindow(this->m_hWnd));
		int nIndex  = GetPaneIndexFromID(nPaneID);
		if(nIndex == -1)
			return FALSE;

		int nLength = this->GetText(nIndex, strText, pnType);
		if(pcchLength != NULL)
			*pcchLength = nLength;

		return TRUE;
	}
#endif // __ATLSTR_H__

	BOOL SetPaneText(int nPaneID, LPCTSTR lpstrText, int nType = 0)
	{
		ATLASSERT(::IsWindow(this->m_hWnd));
		int nIndex  = GetPaneIndexFromID(nPaneID);
		if(nIndex == -1)
			return FALSE;

		return this->SetText(nIndex, lpstrText, nType);
	}

	BOOL GetPaneRect(int nPaneID, LPRECT lpRect) const
	{
		ATLASSERT(::IsWindow(this->m_hWnd));
		int nIndex  = GetPaneIndexFromID(nPaneID);
		if(nIndex == -1)
			return FALSE;

		return this->GetRect(nIndex, lpRect);
	}

	BOOL SetPaneWidth(int nPaneID, int cxWidth)
	{
		ATLASSERT(::IsWindow(this->m_hWnd));
		ATLASSERT(nPaneID != ID_DEFAULT_PANE);   // Can't resize this one
		int nIndex  = GetPaneIndexFromID(nPaneID);
		if(nIndex == -1)
			return FALSE;

		// get pane positions
		ATL::CTempBuffer<int, _WTL_STACK_ALLOC_THRESHOLD> buff;
		int* pPanesPos = buff.Allocate(m_nPanes);
		if(pPanesPos == NULL)
			return FALSE;
		this->GetParts(m_nPanes, pPanesPos);
		// calculate offset
		int cxPaneWidth = pPanesPos[nIndex] - ((nIndex == 0) ? 0 : pPanesPos[nIndex - 1]);
		int cxOff = cxWidth - cxPaneWidth;
		// find variable width pane
		int nDef = m_nPanes;
		for(int i = 0; i < m_nPanes; i++)
		{
			if(m_pPane[i] == ID_DEFAULT_PANE)
			{
				nDef = i;
				break;
			}
		}
		// resize
		if(nIndex < nDef)   // before default pane
		{
			for(int i = nIndex; i < nDef; i++)
				pPanesPos[i] += cxOff;
				
		}
		else			// after default one
		{
			for(int i = nDef; i < nIndex; i++)
				pPanesPos[i] -= cxOff;
		}
		// set pane postions
		return this->SetParts(m_nPanes, pPanesPos);
	}

	BOOL GetPaneTipText(int nPaneID, LPTSTR lpstrText, int nSize) const
	{
		ATLASSERT(::IsWindow(this->m_hWnd));
		int nIndex  = GetPaneIndexFromID(nPaneID);
		if(nIndex == -1)
			return FALSE;

		this->GetTipText(nIndex, lpstrText, nSize);
		return TRUE;
	}

	BOOL SetPaneTipText(int nPaneID, LPCTSTR lpstrText)
	{
		ATLASSERT(::IsWindow(this->m_hWnd));
		int nIndex  = GetPaneIndexFromID(nPaneID);
		if(nIndex == -1)
			return FALSE;

		this->SetTipText(nIndex, lpstrText);
		return TRUE;
	}

	BOOL GetPaneIcon(int nPaneID, HICON& hIcon) const
	{
		ATLASSERT(::IsWindow(this->m_hWnd));
		int nIndex  = GetPaneIndexFromID(nPaneID);
		if(nIndex == -1)
			return FALSE;

		hIcon = this->GetIcon(nIndex);
		return TRUE;
	}

	BOOL SetPaneIcon(int nPaneID, HICON hIcon)
	{
		ATLASSERT(::IsWindow(this->m_hWnd));
		int nIndex  = GetPaneIndexFromID(nPaneID);
		if(nIndex == -1)
			return FALSE;

		return this->SetIcon(nIndex, hIcon);
	}

// Message map and handlers
	BEGIN_MSG_MAP(CMultiPaneStatusBarCtrlImpl< T >)
		MESSAGE_HANDLER(WM_SIZE, OnSize)
	END_MSG_MAP()

	LRESULT OnSize(UINT uMsg, WPARAM wParam, LPARAM lParam, BOOL& /*bHandled*/)
	{
		LRESULT lRet = this->DefWindowProc(uMsg, wParam, lParam);
		if((wParam != SIZE_MINIMIZED) && (m_nPanes > 0))
		{
			T* pT = static_cast<T*>(this);
			pT->UpdatePanesLayout();
		}
		return lRet;
	}

// Implementation
	BOOL UpdatePanesLayout()
	{
		// get pane positions
		ATL::CTempBuffer<int, _WTL_STACK_ALLOC_THRESHOLD> buff;
		int* pPanesPos = buff.Allocate(m_nPanes);
		ATLASSERT(pPanesPos != NULL);
		if(pPanesPos == NULL)
			return FALSE;
		int nRet = this->GetParts(m_nPanes, pPanesPos);
		ATLASSERT(nRet == m_nPanes);
		if(nRet != m_nPanes)
			return FALSE;
		// calculate offset
		RECT rcClient = {};
		this->GetClientRect(&rcClient);
		int cxOff = rcClient.right - pPanesPos[m_nPanes - 1];
		// Move panes left if size grip box is present
		if((this->GetStyle() & SBARS_SIZEGRIP) != 0)
			cxOff -= ::GetSystemMetrics(SM_CXVSCROLL) + ::GetSystemMetrics(SM_CXEDGE);
		// find variable width pane
		int i;
		for(i = 0; i < m_nPanes; i++)
		{
			if(m_pPane[i] == ID_DEFAULT_PANE)
				break;
		}
		// resize all panes from the variable one to the right
		if((i < m_nPanes) && (pPanesPos[i] + cxOff) > ((i == 0) ? 0 : pPanesPos[i - 1]))
		{
			for(; i < m_nPanes; i++)
				pPanesPos[i] += cxOff;
		}
		// set pane postions
		return this->SetParts(m_nPanes, pPanesPos);
	}

	int GetPaneIndexFromID(int nPaneID) const
	{
		for(int i = 0; i < m_nPanes; i++)
		{
			if(m_pPane[i] == nPaneID)
				return i;
		}

		return -1;   // not found
	}
};

class CMultiPaneStatusBarCtrl : public CMultiPaneStatusBarCtrlImpl<CMultiPaneStatusBarCtrl>
{
public:
	DECLARE_WND_SUPERCLASS(_T("WTL_MultiPaneStatusBar"), GetWndClassName())
};


///////////////////////////////////////////////////////////////////////////////
// CPaneContainer - provides header with title and close button for panes

// pane container extended styles
#define PANECNT_NOCLOSEBUTTON   0x00000001
#define PANECNT_VERTICAL        0x00000002
#define PANECNT_FLATBORDER      0x00000004
#define PANECNT_NOBORDER        0x00000008
#define PANECNT_DIVIDER         0x00000010
#define PANECNT_GRADIENT        0x00000020

template <class T, class TBase = ATL::CWindow, class TWinTraits = ATL::CControlWinTraits>
class ATL_NO_VTABLE CPaneContainerImpl : public ATL::CWindowImpl< T, TBase, TWinTraits >, public CCustomDraw< T >
{
public:
	DECLARE_WND_CLASS_EX2(NULL, T, 0, -1)

// Constants
	enum
	{
		m_cxyBorder = 2,
		m_cxyTextOffset = 4,
		m_cxyBtnOffset = 1,

		m_cchTitle = 80,

		m_cxImageTB = 13,
		m_cyImageTB = 11,
		m_cxyBtnAddTB = 7,

		m_cxToolBar = m_cxImageTB + m_cxyBtnAddTB + m_cxyBorder + m_cxyBtnOffset,

		m_xBtnImageLeft = 6,
		m_yBtnImageTop = 5,
		m_xBtnImageRight = 12,
		m_yBtnImageBottom = 11,

		m_nCloseBtnID = ID_PANE_CLOSE
	};

// Data members
	CToolBarCtrl m_tb;
	ATL::CWindow m_wndClient;
	int m_cxyHeader;
	TCHAR m_szTitle[m_cchTitle];
	DWORD m_dwExtendedStyle;   // Pane container specific extended styles
	HFONT m_hFont;
	bool m_bInternalFont;


// Constructor
	CPaneContainerImpl() : m_cxyHeader(0), m_dwExtendedStyle(0), m_hFont(NULL), m_bInternalFont(false)
	{
		m_szTitle[0] = 0;
	}

// Attributes
	DWORD GetPaneContainerExtendedStyle() const
	{
		return m_dwExtendedStyle;
	}

	DWORD SetPaneContainerExtendedStyle(DWORD dwExtendedStyle, DWORD dwMask = 0)
	{
		DWORD dwPrevStyle = m_dwExtendedStyle;
		if(dwMask == 0)
			m_dwExtendedStyle = dwExtendedStyle;
		else
			m_dwExtendedStyle = (m_dwExtendedStyle & ~dwMask) | (dwExtendedStyle & dwMask);
		if(this->m_hWnd != NULL)
		{
			T* pT = static_cast<T*>(this);
			bool bUpdate = false;

			if(((dwPrevStyle & PANECNT_NOCLOSEBUTTON) != 0) && ((m_dwExtendedStyle & PANECNT_NOCLOSEBUTTON) == 0))   // add close button
			{
				pT->CreateCloseButton();
				bUpdate = true;
			}
			else if(((dwPrevStyle & PANECNT_NOCLOSEBUTTON) == 0) && ((m_dwExtendedStyle & PANECNT_NOCLOSEBUTTON) != 0))   // remove close button
			{
				pT->DestroyCloseButton();
				bUpdate = true;
			}

			if((dwPrevStyle & PANECNT_VERTICAL) != (m_dwExtendedStyle & PANECNT_VERTICAL))   // change orientation
			{
				pT->CalcSize();
				bUpdate = true;
			}

			if((dwPrevStyle & (PANECNT_FLATBORDER | PANECNT_NOBORDER)) != 
			   (m_dwExtendedStyle & (PANECNT_FLATBORDER | PANECNT_NOBORDER)))   // change border
			{
				bUpdate = true;
			}

			if((dwPrevStyle & PANECNT_GRADIENT) != (m_dwExtendedStyle & PANECNT_GRADIENT))   // change background
			{
				bUpdate = true;
			}

			if(bUpdate)
				pT->UpdateLayout();
		}
		return dwPrevStyle;
	}

	HWND GetClient() const
	{
		return m_wndClient;
	}

	HWND SetClient(HWND hWndClient)
	{
		HWND hWndOldClient = m_wndClient;
		m_wndClient = hWndClient;
		if(this->m_hWnd != NULL)
		{
			T* pT = static_cast<T*>(this);
			pT->UpdateLayout();
		}
		return hWndOldClient;
	}

	BOOL GetTitle(LPTSTR lpstrTitle, int cchLength) const
	{
		ATLASSERT(lpstrTitle != NULL);

		errno_t nRet = ATL::Checked::tcsncpy_s(lpstrTitle, cchLength, m_szTitle, _TRUNCATE);

		return ((nRet == 0) || (nRet == STRUNCATE));
	}

	BOOL SetTitle(LPCTSTR lpstrTitle)
	{
		ATLASSERT(lpstrTitle != NULL);

		errno_t nRet = ATL::Checked::tcsncpy_s(m_szTitle, m_cchTitle, lpstrTitle, _TRUNCATE);
		bool bRet = ((nRet == 0) || (nRet == STRUNCATE));
		if(bRet && (this->m_hWnd != NULL))
		{
			T* pT = static_cast<T*>(this);
			pT->UpdateLayout();
		}

		return bRet;
	}

	int GetTitleLength() const
	{
		return lstrlen(m_szTitle);
	}

// Methods
	HWND Create(HWND hWndParent, LPCTSTR lpstrTitle = NULL, DWORD dwStyle = WS_CHILD | WS_VISIBLE | WS_CLIPSIBLINGS | WS_CLIPCHILDREN,
			DWORD dwExStyle = 0, UINT nID = 0, LPVOID lpCreateParam = NULL)
	{
		if(lpstrTitle != NULL)
			ATL::Checked::tcsncpy_s(m_szTitle, m_cchTitle, lpstrTitle, _TRUNCATE);
		return ATL::CWindowImpl< T, TBase, TWinTraits >::Create(hWndParent, this->rcDefault, NULL, dwStyle, dwExStyle, nID, lpCreateParam);
	}

	HWND Create(HWND hWndParent, UINT uTitleID, DWORD dwStyle = WS_CHILD | WS_VISIBLE | WS_CLIPSIBLINGS | WS_CLIPCHILDREN,
			DWORD dwExStyle = 0, UINT nID = 0, LPVOID lpCreateParam = NULL)
	{
		if(uTitleID != 0U)
			::LoadString(ModuleHelper::GetResourceInstance(), uTitleID, m_szTitle, m_cchTitle);
		return ATL::CWindowImpl< T, TBase, TWinTraits >::Create(hWndParent, this->rcDefault, NULL, dwStyle, dwExStyle, nID, lpCreateParam);
	}

	BOOL SubclassWindow(HWND hWnd)
	{
		BOOL bRet = ATL::CWindowImpl< T, TBase, TWinTraits >::SubclassWindow(hWnd);
		if(bRet != FALSE)
		{
			T* pT = static_cast<T*>(this);
			pT->Init();

			RECT rect = {};
			this->GetClientRect(&rect);
			pT->UpdateLayout(rect.right, rect.bottom);
		}

		return bRet;
	}

	BOOL EnableCloseButton(BOOL bEnable)
	{
		ATLASSERT(::IsWindow(this->m_hWnd));
		T* pT = static_cast<T*>(this);
		(void)pT;   // avoid level 4 warning
		return (m_tb.m_hWnd != NULL) ? m_tb.EnableButton(pT->m_nCloseBtnID, bEnable) : FALSE;
	}

	void UpdateLayout()
	{
		RECT rcClient = {};
		this->GetClientRect(&rcClient);
		T* pT = static_cast<T*>(this);
		pT->UpdateLayout(rcClient.right, rcClient.bottom);
	}

// Message map and handlers
	BEGIN_MSG_MAP(CPaneContainerImpl)
		MESSAGE_HANDLER(WM_CREATE, OnCreate)
		MESSAGE_HANDLER(WM_DESTROY, OnDestroy)
		MESSAGE_HANDLER(WM_SIZE, OnSize)
		MESSAGE_HANDLER(WM_SETFOCUS, OnSetFocus)
		MESSAGE_HANDLER(WM_GETFONT, OnGetFont)
		MESSAGE_HANDLER(WM_SETFONT, OnSetFont)
		MESSAGE_HANDLER(WM_ERASEBKGND, OnEraseBackground)
		MESSAGE_HANDLER(WM_PAINT, OnPaint)
		MESSAGE_HANDLER(WM_PRINTCLIENT, OnPaint)
		MESSAGE_HANDLER(WM_NOTIFY, OnNotify)
		MESSAGE_HANDLER(WM_COMMAND, OnCommand)
		FORWARD_NOTIFICATIONS()
	END_MSG_MAP()

	LRESULT OnCreate(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		T* pT = static_cast<T*>(this);
		pT->Init();

		return 0;
	}

	LRESULT OnDestroy(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		if(m_bInternalFont)
		{
			::DeleteObject(m_hFont);
			m_hFont = NULL;
			m_bInternalFont = false;
		}

		return 0;
	}

	LRESULT OnSize(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM lParam, BOOL& /*bHandled*/)
	{
		T* pT = static_cast<T*>(this);
		pT->UpdateLayout(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam));
		return 0;
	}

	LRESULT OnSetFocus(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		if(m_wndClient.m_hWnd != NULL)
			m_wndClient.SetFocus();
		return 0;
	}

	LRESULT OnGetFont(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		return (LRESULT)m_hFont;
	}

	LRESULT OnSetFont(UINT /*uMsg*/, WPARAM wParam, LPARAM lParam, BOOL& /*bHandled*/)
	{
		if(m_bInternalFont)
		{
			::DeleteObject(m_hFont);
			m_bInternalFont = false;
		}

		m_hFont = (HFONT)wParam;

		T* pT = static_cast<T*>(this);
		pT->CalcSize();

		if((BOOL)lParam != FALSE)
			pT->UpdateLayout();

		return 0;
	}

	LRESULT OnEraseBackground(UINT /*uMsg*/, WPARAM wParam, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		T* pT = static_cast<T*>(this);
		pT->DrawPaneTitleBackground((HDC)wParam);

		return 1;
	}

	LRESULT OnPaint(UINT /*uMsg*/, WPARAM wParam, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		T* pT = static_cast<T*>(this);
		if(wParam != NULL)
		{
			pT->DrawPaneTitle((HDC)wParam);

			if(m_wndClient.m_hWnd == NULL)   // no client window
				pT->DrawPane((HDC)wParam);
		}
		else
		{
			CPaintDC dc(this->m_hWnd);
			pT->DrawPaneTitle(dc.m_hDC);

			if(m_wndClient.m_hWnd == NULL)   // no client window
				pT->DrawPane(dc.m_hDC);
		}

		return 0;
	}

	LRESULT OnNotify(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM lParam, BOOL& bHandled)
	{
		if(m_tb.m_hWnd == NULL)
		{
			bHandled = FALSE;
			return 1;
		}

		T* pT = static_cast<T*>(this);
		(void)pT;   // avoid level 4 warning
		LPNMHDR lpnmh = (LPNMHDR)lParam;
		LRESULT lRet = 0;

		// pass toolbar custom draw notifications to the base class
		if((lpnmh->code == NM_CUSTOMDRAW) && (lpnmh->hwndFrom == m_tb.m_hWnd))
			lRet = CCustomDraw< T >::OnCustomDraw(0, lpnmh, bHandled);
		// tooltip notifications come with the tooltip window handle and button ID,
		// pass them to the parent if we don't handle them
		else if((lpnmh->code == TTN_GETDISPINFO) && (lpnmh->idFrom == pT->m_nCloseBtnID))
			bHandled = pT->GetToolTipText(lpnmh);
		// only let notifications not from the toolbar go to the parent
		else if((lpnmh->hwndFrom != m_tb.m_hWnd) && (lpnmh->idFrom != pT->m_nCloseBtnID))
			bHandled = FALSE;

		return lRet;
	}

	LRESULT OnCommand(UINT /*uMsg*/, WPARAM wParam, LPARAM lParam, BOOL& bHandled)
	{
		// if command comes from the close button, substitute HWND of the pane container instead
		if((m_tb.m_hWnd != NULL) && ((HWND)lParam == m_tb.m_hWnd))
			return this->GetParent().SendMessage(WM_COMMAND, wParam, (LPARAM)this->m_hWnd);

		bHandled = FALSE;
		return 1;
	}

// Custom draw overrides
	DWORD OnPrePaint(int /*idCtrl*/, LPNMCUSTOMDRAW /*lpNMCustomDraw*/)
	{
		return CDRF_NOTIFYITEMDRAW;   // we need per-item notifications
	}

	DWORD OnItemPrePaint(int /*idCtrl*/, LPNMCUSTOMDRAW /*lpNMCustomDraw*/)
	{
		return CDRF_NOTIFYPOSTPAINT;
	}

	DWORD OnItemPostPaint(int /*idCtrl*/, LPNMCUSTOMDRAW lpNMCustomDraw)
	{
		CDCHandle dc = lpNMCustomDraw->hdc;
		RECT& rc = lpNMCustomDraw->rc;

		RECT rcImage = { m_xBtnImageLeft, m_yBtnImageTop, m_xBtnImageRight + 1, m_yBtnImageBottom + 1 };
		::OffsetRect(&rcImage, rc.left, rc.top);
		T* pT = static_cast<T*>(this);

		if((lpNMCustomDraw->uItemState & CDIS_DISABLED) != 0)
		{
			RECT rcShadow = rcImage;
			::OffsetRect(&rcShadow, 1, 1);
			CPen pen1;
			pen1.CreatePen(PS_SOLID, 0, ::GetSysColor(COLOR_3DHILIGHT));
			pT->DrawButtonImage(dc, rcShadow, pen1);
			CPen pen2;
			pen2.CreatePen(PS_SOLID, 0, ::GetSysColor(COLOR_3DSHADOW));
			pT->DrawButtonImage(dc, rcImage, pen2);
		}
		else
		{
			if((lpNMCustomDraw->uItemState & CDIS_SELECTED) != 0)
				::OffsetRect(&rcImage, 1, 1);
			CPen pen;
			pen.CreatePen(PS_SOLID, 0, ::GetSysColor(COLOR_BTNTEXT));
			pT->DrawButtonImage(dc, rcImage, pen);
		}

		return CDRF_DODEFAULT;   // continue with the default item painting
	}

// Implementation - overrideable methods
	void Init()
	{
		if(m_hFont == NULL)
		{
			// The same as AtlCreateControlFont() for horizontal pane
			LOGFONT lf = {};
			ATLVERIFY(::SystemParametersInfo(SPI_GETICONTITLELOGFONT, sizeof(LOGFONT), &lf, 0) != FALSE);
			if(IsVertical())
				lf.lfEscapement = 900;   // 90 degrees
			m_hFont = ::CreateFontIndirect(&lf);
			m_bInternalFont = true;
		}

		T* pT = static_cast<T*>(this);
		pT->CalcSize();

		if((m_dwExtendedStyle & PANECNT_NOCLOSEBUTTON) == 0)
			pT->CreateCloseButton();
	}

	void UpdateLayout(int cxWidth, int cyHeight)
	{
		ATLASSERT(::IsWindow(this->m_hWnd));
		RECT rect = {};

		if(IsVertical())
		{
			::SetRect(&rect, 0, 0, m_cxyHeader, cyHeight);
			if(m_tb.m_hWnd != NULL)
				m_tb.SetWindowPos(NULL, m_cxyBorder, m_cxyBorder + m_cxyBtnOffset, 0, 0, SWP_NOZORDER | SWP_NOSIZE | SWP_NOACTIVATE);

			if(m_wndClient.m_hWnd != NULL)
				m_wndClient.SetWindowPos(NULL, m_cxyHeader, 0, cxWidth - m_cxyHeader, cyHeight, SWP_NOZORDER);
			else
				rect.right = cxWidth;
		}
		else
		{
			::SetRect(&rect, 0, 0, cxWidth, m_cxyHeader);
			if(m_tb.m_hWnd != NULL)
				m_tb.SetWindowPos(NULL, rect.right - m_cxToolBar, m_cxyBorder + m_cxyBtnOffset, 0, 0, SWP_NOZORDER | SWP_NOSIZE | SWP_NOACTIVATE);

			if(m_wndClient.m_hWnd != NULL)
				m_wndClient.SetWindowPos(NULL, 0, m_cxyHeader, cxWidth, cyHeight - m_cxyHeader, SWP_NOZORDER);
			else
				rect.bottom = cyHeight;
		}

		this->InvalidateRect(&rect);
	}

	void CreateCloseButton()
	{
		ATLASSERT(m_tb.m_hWnd == NULL);
		// create toolbar for the "x" button
		m_tb.Create(this->m_hWnd, this->rcDefault, NULL, WS_CHILD | WS_VISIBLE | WS_CLIPCHILDREN | WS_CLIPSIBLINGS | CCS_NODIVIDER | CCS_NORESIZE | CCS_NOPARENTALIGN | CCS_NOMOVEY | TBSTYLE_TOOLTIPS | TBSTYLE_FLAT, 0);
		ATLASSERT(m_tb.IsWindow());

		if(m_tb.m_hWnd != NULL)
		{
			T* pT = static_cast<T*>(this);
			(void)pT;   // avoid level 4 warning

			m_tb.SetButtonStructSize();

			TBBUTTON tbbtn = {};
			tbbtn.idCommand = pT->m_nCloseBtnID;
			tbbtn.fsState = TBSTATE_ENABLED;
			tbbtn.fsStyle = BTNS_BUTTON;
			m_tb.AddButtons(1, &tbbtn);

			m_tb.SetBitmapSize(m_cxImageTB, m_cyImageTB);
			m_tb.SetButtonSize(m_cxImageTB + m_cxyBtnAddTB, m_cyImageTB + m_cxyBtnAddTB);

			if(IsVertical())
				m_tb.SetWindowPos(NULL, m_cxyBorder + m_cxyBtnOffset, m_cxyBorder + m_cxyBtnOffset, m_cxImageTB + m_cxyBtnAddTB, m_cyImageTB + m_cxyBtnAddTB + 1, SWP_NOZORDER | SWP_NOACTIVATE);
			else
				m_tb.SetWindowPos(NULL, 0, 0, m_cxImageTB + m_cxyBtnAddTB, m_cyImageTB + m_cxyBtnAddTB + 1, SWP_NOZORDER | SWP_NOMOVE | SWP_NOACTIVATE);
		}
	}

	void DestroyCloseButton()
	{
		if(m_tb.m_hWnd != NULL)
			m_tb.DestroyWindow();
	}

	void CalcSize()
	{
		T* pT = static_cast<T*>(this);
		CFontHandle font = pT->GetTitleFont();
		if(font.IsNull())
			font = (HFONT)::GetStockObject(SYSTEM_FONT);
		LOGFONT lf = {};
		font.GetLogFont(lf);
		if(IsVertical())
		{
			m_cxyHeader = m_cxImageTB + m_cxyBtnAddTB + m_cxyBorder + 1;
		}
		else
		{
			int cyFont = abs(lf.lfHeight) + m_cxyBorder + 2 * m_cxyTextOffset;
			int cyBtn = m_cyImageTB + m_cxyBtnAddTB + m_cxyBorder + 2 * m_cxyBtnOffset + 1;
			m_cxyHeader = __max(cyFont, cyBtn);
		}
	}

	HFONT GetTitleFont() const
	{
		return m_hFont;
	}

	BOOL GetToolTipText(LPNMHDR /*lpnmh*/)
	{
		return FALSE;
	}

	void DrawPaneTitle(CDCHandle dc)
	{
		RECT rect = {};
		this->GetClientRect(&rect);

		UINT uBorder = BF_LEFT | BF_TOP | BF_ADJUST;
		if(IsVertical())
		{
			rect.right = rect.left + m_cxyHeader;
			uBorder |= BF_BOTTOM;
		}
		else
		{
			rect.bottom = rect.top + m_cxyHeader;
			uBorder |= BF_RIGHT;
		}

		if((m_dwExtendedStyle & PANECNT_NOBORDER) == 0)
		{
			if((m_dwExtendedStyle & PANECNT_FLATBORDER) != 0)
				uBorder |= BF_FLAT;
			dc.DrawEdge(&rect, EDGE_ETCHED, uBorder);
		}

		if((m_dwExtendedStyle & PANECNT_DIVIDER) != 0)
		{
			uBorder = BF_FLAT | BF_ADJUST | (IsVertical() ? BF_RIGHT : BF_BOTTOM);
			dc.DrawEdge(&rect, BDR_SUNKENOUTER, uBorder);
		}

		// draw title text
		dc.SetTextColor(::GetSysColor(COLOR_WINDOWTEXT));
		dc.SetBkMode(TRANSPARENT);
		T* pT = static_cast<T*>(this);
		HFONT hFontOld = dc.SelectFont(pT->GetTitleFont());

		if(IsVertical())
		{
			rect.top += m_cxyTextOffset;
			rect.bottom -= m_cxyTextOffset;
			if(m_tb.m_hWnd != NULL)
				rect.top += m_cxToolBar;;

			RECT rcCalc = { rect.left, rect.bottom, rect.right, rect.top };
			int cxFont = dc.DrawText(m_szTitle, -1, &rcCalc, DT_TOP | DT_SINGLELINE | DT_END_ELLIPSIS | DT_CALCRECT);
			RECT rcText = {};
			rcText.left = (rect.right - rect.left - cxFont) / 2;
			rcText.right = rcText.left + (rect.bottom - rect.top);
			rcText.top = rect.bottom;
			rcText.bottom = rect.top;
			dc.DrawText(m_szTitle, -1, &rcText, DT_TOP | DT_SINGLELINE | DT_END_ELLIPSIS);
		}
		else
		{
			rect.left += m_cxyTextOffset;
			rect.right -= m_cxyTextOffset;
			if(m_tb.m_hWnd != NULL)
				rect.right -= m_cxToolBar;;

			dc.DrawText(m_szTitle, -1, &rect, DT_LEFT | DT_SINGLELINE | DT_VCENTER | DT_END_ELLIPSIS);
		}

		dc.SelectFont(hFontOld);
	}

	void DrawPaneTitleBackground(CDCHandle dc)
	{
		RECT rect = {};
		this->GetClientRect(&rect);
		if(IsVertical())
			rect.right = m_cxyHeader;
		else
			rect.bottom = m_cxyHeader;

		if((m_dwExtendedStyle & PANECNT_GRADIENT) != 0)
			dc.GradientFillRect(rect, ::GetSysColor(COLOR_WINDOW), ::GetSysColor(COLOR_3DFACE), IsVertical());
		else
			dc.FillRect(&rect, COLOR_3DFACE);
	}

	// called only if pane is empty
	void DrawPane(CDCHandle dc)
	{
		RECT rect = {};
		this->GetClientRect(&rect);
		if(IsVertical())
			rect.left += m_cxyHeader;
		else
			rect.top += m_cxyHeader;
		if((this->GetExStyle() & WS_EX_CLIENTEDGE) == 0)
			dc.DrawEdge(&rect, EDGE_SUNKEN, BF_RECT | BF_ADJUST);
		dc.FillRect(&rect, COLOR_APPWORKSPACE);
	}

	// drawing helper - draws "x" button image
	void DrawButtonImage(CDCHandle dc, RECT& rcImage, HPEN hPen)
	{
		HPEN hPenOld = dc.SelectPen(hPen);

		dc.MoveTo(rcImage.left, rcImage.top);
		dc.LineTo(rcImage.right, rcImage.bottom);
		dc.MoveTo(rcImage.left + 1, rcImage.top);
		dc.LineTo(rcImage.right + 1, rcImage.bottom);

		dc.MoveTo(rcImage.left, rcImage.bottom - 1);
		dc.LineTo(rcImage.right, rcImage.top - 1);
		dc.MoveTo(rcImage.left + 1, rcImage.bottom - 1);
		dc.LineTo(rcImage.right + 1, rcImage.top - 1);

		dc.SelectPen(hPenOld);
	}

	bool IsVertical() const
	{
		return ((m_dwExtendedStyle & PANECNT_VERTICAL) != 0);
	}
};

class CPaneContainer : public CPaneContainerImpl<CPaneContainer>
{
public:
	DECLARE_WND_CLASS_EX(_T("WTL_PaneContainer"), 0, -1)
};


///////////////////////////////////////////////////////////////////////////////
// CSortListViewCtrl - implements sorting for a listview control

// sort listview extended styles
#define SORTLV_USESHELLBITMAPS	0x00000001

// Notification sent to parent when sort column is changed by user clicking header.  
#define SLVN_SORTCHANGED	LVN_LAST

// A LPNMSORTLISTVIEW is sent with the SLVN_SORTCHANGED notification
typedef struct tagNMSORTLISTVIEW
{
    NMHDR hdr;
    int iNewSortColumn;
    int iOldSortColumn;
} NMSORTLISTVIEW, *LPNMSORTLISTVIEW;

// Column sort types. Can be set on a per-column basis with the SetColumnSortType method.
enum
{
	LVCOLSORT_NONE,
	LVCOLSORT_TEXT,   // default
	LVCOLSORT_TEXTNOCASE,
	LVCOLSORT_LONG,
	LVCOLSORT_DOUBLE,
	LVCOLSORT_DECIMAL,
	LVCOLSORT_DATETIME,
	LVCOLSORT_DATE,
	LVCOLSORT_TIME,
	LVCOLSORT_CUSTOM,
	LVCOLSORT_LAST = LVCOLSORT_CUSTOM
};


template <class T>
class CSortListViewImpl
{
public:
	enum
	{
		m_cchCmpTextMax = 32, // overrideable
		m_cxSortImage = 16,
		m_cySortImage = 15,
		m_cxSortArrow = 11,
		m_cySortArrow = 6,
		m_iSortUp = 0,        // index of sort bitmaps
		m_iSortDown = 1,
		m_nShellSortUpID = 133
	};

	// passed to LVCompare functions as lParam1 and lParam2 
	struct LVCompareParam
	{
		int iItem;
		DWORD_PTR dwItemData;
		union
		{
			long lValue;
			double dblValue;
			DECIMAL decValue;
			LPCTSTR pszValue;
		};
	};
	
	// passed to LVCompare functions as the lParamSort parameter
	struct LVSortInfo
	{
		T* pT;
		int iSortCol;
		bool bDescending;
	};

	bool m_bSortDescending;
	bool m_bCommCtrl6;
	int m_iSortColumn;
	CBitmap m_bmSort[2];
	int m_fmtOldSortCol;
	HBITMAP m_hbmOldSortCol;
	DWORD m_dwSortLVExtendedStyle;
	ATL::CSimpleArray<WORD> m_arrColSortType;
	bool m_bUseWaitCursor;
	
	CSortListViewImpl() :
			m_bSortDescending(false),
			m_bCommCtrl6(false),
			m_iSortColumn(-1), 
			m_fmtOldSortCol(0),
			m_hbmOldSortCol(NULL),
			m_dwSortLVExtendedStyle(SORTLV_USESHELLBITMAPS),
			m_bUseWaitCursor(true)
	{
		DWORD dwMajor = 0;
		DWORD dwMinor = 0;
		HRESULT hRet = ATL::AtlGetCommCtrlVersion(&dwMajor, &dwMinor);
		m_bCommCtrl6 = SUCCEEDED(hRet) && (dwMajor >= 6);
	}
	
// Attributes
	void SetSortColumn(int iCol)
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));
		CHeaderCtrl header = pT->GetHeader();
		ATLASSERT(header.m_hWnd != NULL);
		ATLASSERT((iCol >= -1) && (iCol < m_arrColSortType.GetSize()));

		int iOldSortCol = m_iSortColumn;
		m_iSortColumn = iCol;
		if(m_bCommCtrl6)
		{
			const int nMask = HDF_SORTUP | HDF_SORTDOWN;
			HDITEM hditem = { HDI_FORMAT };
			if((iOldSortCol != iCol) && (iOldSortCol >= 0) && header.GetItem(iOldSortCol, &hditem))
			{
				hditem.fmt &= ~nMask;
				header.SetItem(iOldSortCol, &hditem);
			}
			if((iCol >= 0) && header.GetItem(iCol, &hditem))
			{
				hditem.fmt &= ~nMask;
				hditem.fmt |= m_bSortDescending ? HDF_SORTDOWN : HDF_SORTUP;
				header.SetItem(iCol, &hditem);
			}
			return;
		}

		if(m_bmSort[m_iSortUp].IsNull())
			pT->CreateSortBitmaps();

		// restore previous sort column's bitmap, if any, and format
		HDITEM hditem = { HDI_BITMAP | HDI_FORMAT };
		if((iOldSortCol != iCol) && (iOldSortCol >= 0))
		{
			hditem.hbm = m_hbmOldSortCol;
			hditem.fmt = m_fmtOldSortCol;
			header.SetItem(iOldSortCol, &hditem);
		}

		// save new sort column's bitmap and format, and add our sort bitmap
		if((iCol >= 0) && header.GetItem(iCol, &hditem))
		{
			if(iOldSortCol != iCol)
			{
				m_fmtOldSortCol = hditem.fmt;
				m_hbmOldSortCol = hditem.hbm;
			}
			hditem.fmt &= ~HDF_IMAGE;
			hditem.fmt |= HDF_BITMAP | HDF_BITMAP_ON_RIGHT;
			int i = m_bSortDescending ? m_iSortDown : m_iSortUp;
			hditem.hbm = m_bmSort[i];
			header.SetItem(iCol, &hditem);
		}
	}

	int GetSortColumn() const
	{
		return m_iSortColumn;
	}

	void SetColumnSortType(int iCol, WORD wType)
	{
		ATLASSERT((iCol >= 0) && (iCol < m_arrColSortType.GetSize()));
		ATLASSERT((wType >= LVCOLSORT_NONE) && (wType <= LVCOLSORT_LAST));
		m_arrColSortType[iCol] = wType;
	}

	WORD GetColumnSortType(int iCol) const
	{
		ATLASSERT((iCol >= 0) && (iCol < m_arrColSortType.GetSize()));
		return m_arrColSortType[iCol];
	}

	int GetColumnCount() const
	{
		const T* pT = static_cast<const T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));
		CHeaderCtrl header = pT->GetHeader();
		return header.m_hWnd != NULL ? header.GetItemCount() : 0;
	}

	bool IsSortDescending() const
	{
		return m_bSortDescending;
	}

	DWORD GetSortListViewExtendedStyle() const
	{
		return m_dwSortLVExtendedStyle;
	}

	DWORD SetSortListViewExtendedStyle(DWORD dwExtendedStyle, DWORD dwMask = 0)
	{
		DWORD dwPrevStyle = m_dwSortLVExtendedStyle;
		if(dwMask == 0)
			m_dwSortLVExtendedStyle = dwExtendedStyle;
		else
			m_dwSortLVExtendedStyle = (m_dwSortLVExtendedStyle & ~dwMask) | (dwExtendedStyle & dwMask);
		return dwPrevStyle;
	}

// Operations
	bool DoSortItems(int iCol, bool bDescending = false)
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));
		ATLASSERT((iCol >= 0) && (iCol < m_arrColSortType.GetSize()));

		WORD wType = m_arrColSortType[iCol];
		if(wType == LVCOLSORT_NONE)
			return false;

		int nCount = pT->GetItemCount();
		if(nCount < 2)
		{
			m_bSortDescending = bDescending;
			SetSortColumn(iCol);
			return true;
		}

		CWaitCursor waitCursor(false);
		if(m_bUseWaitCursor)
			waitCursor.Set();

		LVCompareParam* pParam = NULL;
		ATLTRY(pParam = new LVCompareParam[nCount]);
		PFNLVCOMPARE pFunc = NULL;
		TCHAR pszTemp[pT->m_cchCmpTextMax] = {};
		bool bStrValue = false;

		switch(wType)
		{
		case LVCOLSORT_TEXT:
			pFunc = (PFNLVCOMPARE)pT->LVCompareText;
		case LVCOLSORT_TEXTNOCASE:
			if(pFunc == NULL)
				pFunc = (PFNLVCOMPARE)pT->LVCompareTextNoCase;
		case LVCOLSORT_CUSTOM:
			{
				if(pFunc == NULL)
					pFunc = (PFNLVCOMPARE)pT->LVCompareCustom;

				for(int i = 0; i < nCount; i++)
				{
					pParam[i].iItem = i;
					pParam[i].dwItemData = pT->GetItemData(i);
					pParam[i].pszValue = new TCHAR[pT->m_cchCmpTextMax];
					pT->GetItemText(i, iCol, (LPTSTR)pParam[i].pszValue, pT->m_cchCmpTextMax);
					pT->SetItemData(i, (DWORD_PTR)&pParam[i]);
				}
				bStrValue = true;
			}
			break;
		case LVCOLSORT_LONG:
			{
				pFunc = (PFNLVCOMPARE)pT->LVCompareLong;
				for(int i = 0; i < nCount; i++)
				{
					pParam[i].iItem = i;
					pParam[i].dwItemData = pT->GetItemData(i);
					pT->GetItemText(i, iCol, pszTemp, pT->m_cchCmpTextMax);
					pParam[i].lValue = pT->StrToLong(pszTemp);
					pT->SetItemData(i, (DWORD_PTR)&pParam[i]);
				}
			}
			break;
		case LVCOLSORT_DOUBLE:
			{
				pFunc = (PFNLVCOMPARE)pT->LVCompareDouble;
				for(int i = 0; i < nCount; i++)
				{
					pParam[i].iItem = i;
					pParam[i].dwItemData = pT->GetItemData(i);
					pT->GetItemText(i, iCol, pszTemp, pT->m_cchCmpTextMax);
					pParam[i].dblValue = pT->StrToDouble(pszTemp);
					pT->SetItemData(i, (DWORD_PTR)&pParam[i]);
				}
			}
			break;
		case LVCOLSORT_DECIMAL:
			{
				pFunc = (PFNLVCOMPARE)pT->LVCompareDecimal;
				for(int i = 0; i < nCount; i++)
				{
					pParam[i].iItem = i;
					pParam[i].dwItemData = pT->GetItemData(i);
					pT->GetItemText(i, iCol, pszTemp, pT->m_cchCmpTextMax);
					pT->StrToDecimal(pszTemp, &pParam[i].decValue);
					pT->SetItemData(i, (DWORD_PTR)&pParam[i]);
				}
			}
			break;
		case LVCOLSORT_DATETIME:
		case LVCOLSORT_DATE:
		case LVCOLSORT_TIME:
			{
				pFunc = (PFNLVCOMPARE)pT->LVCompareDouble;
				DWORD dwFlags = LOCALE_NOUSEROVERRIDE;
				if(wType == LVCOLSORT_DATE)
					dwFlags |= VAR_DATEVALUEONLY;
				else if(wType == LVCOLSORT_TIME)
					dwFlags |= VAR_TIMEVALUEONLY;
				for(int i = 0; i < nCount; i++)
				{
					pParam[i].iItem = i;
					pParam[i].dwItemData = pT->GetItemData(i);
					pT->GetItemText(i, iCol, pszTemp, pT->m_cchCmpTextMax);
					pParam[i].dblValue = pT->DateStrToDouble(pszTemp, dwFlags);
					pT->SetItemData(i, (DWORD_PTR)&pParam[i]);
				}
			}
			break;
		default:
			ATLTRACE2(atlTraceUI, 0, _T("Unknown value for sort type in CSortListViewImpl::DoSortItems()\n"));
			break;
		} // switch(wType)

		ATLASSERT(pFunc != NULL);
		LVSortInfo lvsi = { pT, iCol, bDescending };
		bool bRet = ((BOOL)pT->DefWindowProc(LVM_SORTITEMS, (WPARAM)&lvsi, (LPARAM)pFunc) != FALSE);
		for(int i = 0; i < nCount; i++)
		{
			DWORD_PTR dwItemData = pT->GetItemData(i);
			LVCompareParam* p = (LVCompareParam*)dwItemData;
			ATLASSERT(p != NULL);
			if(bStrValue)
				delete [] (TCHAR*)p->pszValue;
			pT->SetItemData(i, p->dwItemData);
		}
		delete [] pParam;

		if(bRet)
		{
			m_bSortDescending = bDescending;
			SetSortColumn(iCol);
		}

		if(m_bUseWaitCursor)
			waitCursor.Restore();

		return bRet;
	}

	void CreateSortBitmaps()
	{
		if((m_dwSortLVExtendedStyle & SORTLV_USESHELLBITMAPS) != 0)
		{
			bool bFree = false;
			LPCTSTR pszModule = _T("shell32.dll"); 
			HINSTANCE hShell = ::GetModuleHandle(pszModule);

			if (hShell == NULL)		
			{
				hShell = ::LoadLibrary(pszModule);
				bFree = true;
			}
 
			if (hShell != NULL)
			{
				bool bSuccess = true;
				for(int i = m_iSortUp; i <= m_iSortDown; i++)
				{
					if(!m_bmSort[i].IsNull())
						m_bmSort[i].DeleteObject();
					m_bmSort[i] = (HBITMAP)::LoadImage(hShell, MAKEINTRESOURCE(m_nShellSortUpID + i), 
						IMAGE_BITMAP, 0, 0, LR_LOADMAP3DCOLORS);
					if(m_bmSort[i].IsNull())
					{
						bSuccess = false;
						break;
					}
				}
				if(bFree)
					::FreeLibrary(hShell);
				if(bSuccess)
					return;
			}
		}

		T* pT = static_cast<T*>(this);
		for(int i = m_iSortUp; i <= m_iSortDown; i++)
		{
			if(!m_bmSort[i].IsNull())
				m_bmSort[i].DeleteObject();

			CDC dcMem;
			CClientDC dc(::GetDesktopWindow());
			dcMem.CreateCompatibleDC(dc.m_hDC);
			m_bmSort[i].CreateCompatibleBitmap(dc.m_hDC, m_cxSortImage, m_cySortImage);
			HBITMAP hbmOld = dcMem.SelectBitmap(m_bmSort[i]);
			RECT rc = { 0, 0, m_cxSortImage, m_cySortImage };
			pT->DrawSortBitmap(dcMem.m_hDC, i, &rc);
			dcMem.SelectBitmap(hbmOld);
			dcMem.DeleteDC();
		}
	}

	void NotifyParentSortChanged(int iNewSortCol, int iOldSortCol)
	{
		T* pT = static_cast<T*>(this);
		int nID = pT->GetDlgCtrlID();
		NMSORTLISTVIEW nm = { { pT->m_hWnd, (UINT_PTR)nID, SLVN_SORTCHANGED }, iNewSortCol, iOldSortCol };
		::SendMessage(pT->GetParent(), WM_NOTIFY, (WPARAM)nID, (LPARAM)&nm);
	}

// Overrideables
	int CompareItemsCustom(LVCompareParam* /*pItem1*/, LVCompareParam* /*pItem2*/, int /*iSortCol*/)
	{
		// pItem1 and pItem2 contain valid iItem, dwItemData, and pszValue members.
		// If item1 > item2 return 1, if item1 < item2 return -1, else return 0.
		return 0;
	}

	void DrawSortBitmap(CDCHandle dc, int iBitmap, LPRECT prc)
	{
		dc.FillRect(prc, ::GetSysColorBrush(COLOR_BTNFACE));	
		HBRUSH hbrOld = dc.SelectBrush(::GetSysColorBrush(COLOR_BTNSHADOW));
		CPen pen;
		pen.CreatePen(PS_SOLID, 0, ::GetSysColor(COLOR_BTNSHADOW));
		HPEN hpenOld = dc.SelectPen(pen);
		POINT ptOrg = { (m_cxSortImage - m_cxSortArrow) / 2, (m_cySortImage - m_cySortArrow) / 2 };
		if(iBitmap == m_iSortUp)
		{
			POINT pts[3] = 
			{
				{ ptOrg.x + m_cxSortArrow / 2, ptOrg.y },
				{ ptOrg.x, ptOrg.y + m_cySortArrow - 1 }, 
				{ ptOrg.x + m_cxSortArrow - 1, ptOrg.y + m_cySortArrow - 1 }
			};
			dc.Polygon(pts, 3);
		}
		else
		{
			POINT pts[3] = 
			{
				{ ptOrg.x, ptOrg.y },
				{ ptOrg.x + m_cxSortArrow / 2, ptOrg.y + m_cySortArrow - 1 },
				{ ptOrg.x + m_cxSortArrow - 1, ptOrg.y }
			};
			dc.Polygon(pts, 3);
		}
		dc.SelectBrush(hbrOld);
		dc.SelectPen(hpenOld);
	}

	double DateStrToDouble(LPCTSTR lpstr, DWORD dwFlags)
	{
		ATLASSERT(lpstr != NULL);
		if((lpstr == NULL) || (lpstr[0] == _T('\0')))
			return 0;

		USES_CONVERSION;
		HRESULT hRet = E_FAIL;
		DATE dRet = 0;
		if (FAILED(hRet = ::VarDateFromStr((LPOLESTR)T2COLE(lpstr), LANG_USER_DEFAULT, dwFlags, &dRet)))
		{
			ATLTRACE2(atlTraceUI, 0, _T("VarDateFromStr failed with result of 0x%8.8X\n"), hRet);
			dRet = 0;
		}
		return dRet;
	}

	long StrToLong(LPCTSTR lpstr)
	{
		ATLASSERT(lpstr != NULL);
		if((lpstr == NULL) || (lpstr[0] == _T('\0')))
			return 0;
		
		USES_CONVERSION;
		HRESULT hRet = E_FAIL;
		long lRet = 0;
		if (FAILED(hRet = ::VarI4FromStr((LPOLESTR)T2COLE(lpstr), LANG_USER_DEFAULT, LOCALE_NOUSEROVERRIDE, &lRet)))
		{
			ATLTRACE2(atlTraceUI, 0, _T("VarI4FromStr failed with result of 0x%8.8X\n"), hRet);
			lRet = 0;
		}
		return lRet;
	}

	double StrToDouble(LPCTSTR lpstr)
	{
		ATLASSERT(lpstr != NULL);
		if((lpstr == NULL) || (lpstr[0] == _T('\0')))
			return 0;

		USES_CONVERSION;
		HRESULT hRet = E_FAIL;
		double dblRet = 0;
		if (FAILED(hRet = ::VarR8FromStr((LPOLESTR)T2COLE(lpstr), LANG_USER_DEFAULT, LOCALE_NOUSEROVERRIDE, &dblRet)))
		{
			ATLTRACE2(atlTraceUI, 0, _T("VarR8FromStr failed with result of 0x%8.8X\n"), hRet);
			dblRet = 0;
		}
		return dblRet;
	}

	bool StrToDecimal(LPCTSTR lpstr, DECIMAL* pDecimal)
	{
		ATLASSERT(lpstr != NULL);
		ATLASSERT(pDecimal != NULL);
		if((lpstr == NULL) || (pDecimal == NULL))
			return false;

		USES_CONVERSION;
		HRESULT hRet = E_FAIL;
		if (FAILED(hRet = ::VarDecFromStr((LPOLESTR)T2COLE(lpstr), LANG_USER_DEFAULT, LOCALE_NOUSEROVERRIDE, pDecimal)))
		{
			ATLTRACE2(atlTraceUI, 0, _T("VarDecFromStr failed with result of 0x%8.8X\n"), hRet);
			pDecimal->Lo64 = 0;
			pDecimal->Hi32 = 0;
			pDecimal->signscale = 0;
			return false;
		}
		return true;
	}

// Overrideable PFNLVCOMPARE functions
	static int CALLBACK LVCompareText(LPARAM lParam1, LPARAM lParam2, LPARAM lParamSort)
	{
		ATLASSERT((lParam1 != NULL) && (lParam2 != NULL) && (lParamSort != NULL));

		LVCompareParam* pParam1 = (LVCompareParam*)lParam1;
		LVCompareParam* pParam2 = (LVCompareParam*)lParam2;
		LVSortInfo* pInfo = (LVSortInfo*)lParamSort;
		
		int nRet = lstrcmp(pParam1->pszValue, pParam2->pszValue);
		return pInfo->bDescending ? -nRet : nRet;
	}

	static int CALLBACK LVCompareTextNoCase(LPARAM lParam1, LPARAM lParam2, LPARAM lParamSort)
	{
		ATLASSERT((lParam1 != NULL) && (lParam2 != NULL) && (lParamSort != NULL));

		LVCompareParam* pParam1 = (LVCompareParam*)lParam1;
		LVCompareParam* pParam2 = (LVCompareParam*)lParam2;
		LVSortInfo* pInfo = (LVSortInfo*)lParamSort;
		
		int nRet = lstrcmpi(pParam1->pszValue, pParam2->pszValue);
		return pInfo->bDescending ? -nRet : nRet;
	}

	static int CALLBACK LVCompareLong(LPARAM lParam1, LPARAM lParam2, LPARAM lParamSort)
	{
		ATLASSERT((lParam1 != NULL) && (lParam2 != NULL) && (lParamSort != NULL));

		LVCompareParam* pParam1 = (LVCompareParam*)lParam1;
		LVCompareParam* pParam2 = (LVCompareParam*)lParam2;
		LVSortInfo* pInfo = (LVSortInfo*)lParamSort;
		
		int nRet = 0;
		if(pParam1->lValue > pParam2->lValue)
			nRet = 1;
		else if(pParam1->lValue < pParam2->lValue)
			nRet = -1;
		return pInfo->bDescending ? -nRet : nRet;
	}

	static int CALLBACK LVCompareDouble(LPARAM lParam1, LPARAM lParam2, LPARAM lParamSort)
	{
		ATLASSERT((lParam1 != NULL) && (lParam2 != NULL) && (lParamSort != NULL));

		LVCompareParam* pParam1 = (LVCompareParam*)lParam1;
		LVCompareParam* pParam2 = (LVCompareParam*)lParam2;
		LVSortInfo* pInfo = (LVSortInfo*)lParamSort;
		
		int nRet = 0;
		if(pParam1->dblValue > pParam2->dblValue)
			nRet = 1;
		else if(pParam1->dblValue < pParam2->dblValue)
			nRet = -1;
		return pInfo->bDescending ? -nRet : nRet;
	}

	static int CALLBACK LVCompareCustom(LPARAM lParam1, LPARAM lParam2, LPARAM lParamSort)
	{
		ATLASSERT((lParam1 != NULL) && (lParam2 != NULL) && (lParamSort != NULL));

		LVCompareParam* pParam1 = (LVCompareParam*)lParam1;
		LVCompareParam* pParam2 = (LVCompareParam*)lParam2;
		LVSortInfo* pInfo = (LVSortInfo*)lParamSort;
		
		int nRet = pInfo->pT->CompareItemsCustom(pParam1, pParam2, pInfo->iSortCol);
		return pInfo->bDescending ? -nRet : nRet;
	}

	static int CALLBACK LVCompareDecimal(LPARAM lParam1, LPARAM lParam2, LPARAM lParamSort)
	{
		ATLASSERT((lParam1 != NULL) && (lParam2 != NULL) && (lParamSort != NULL));

		LVCompareParam* pParam1 = (LVCompareParam*)lParam1;
		LVCompareParam* pParam2 = (LVCompareParam*)lParam2;
		LVSortInfo* pInfo = (LVSortInfo*)lParamSort;
		
		int nRet = (int)::VarDecCmp(&pParam1->decValue, &pParam2->decValue);
		nRet--;
		return pInfo->bDescending ? -nRet : nRet;
	}

	BEGIN_MSG_MAP(CSortListViewImpl)
		MESSAGE_HANDLER(LVM_INSERTCOLUMN, OnInsertColumn)
		MESSAGE_HANDLER(LVM_DELETECOLUMN, OnDeleteColumn)
		NOTIFY_CODE_HANDLER(HDN_ITEMCLICKA, OnHeaderItemClick)
		NOTIFY_CODE_HANDLER(HDN_ITEMCLICKW, OnHeaderItemClick)
		MESSAGE_HANDLER(WM_SETTINGCHANGE, OnSettingChange)
	END_MSG_MAP()

	LRESULT OnInsertColumn(UINT uMsg, WPARAM wParam, LPARAM lParam, BOOL& /*bHandled*/)	
	{
		T* pT = static_cast<T*>(this);
		LRESULT lRet = pT->DefWindowProc(uMsg, wParam, lParam);
		if(lRet == -1)
			return -1;

		WORD wType = 0;
		m_arrColSortType.Add(wType);
		int nCount = m_arrColSortType.GetSize();
		ATLASSERT(nCount == GetColumnCount());

		for(int i = nCount - 1; i > lRet; i--)
			m_arrColSortType[i] = m_arrColSortType[i - 1];
		m_arrColSortType[(int)lRet] = LVCOLSORT_TEXT;

		if(lRet <= m_iSortColumn)
			m_iSortColumn++;

		return lRet;
	}

	LRESULT OnDeleteColumn(UINT uMsg, WPARAM wParam, LPARAM lParam, BOOL& /*bHandled*/)	
	{
		T* pT = static_cast<T*>(this);
		LRESULT lRet = pT->DefWindowProc(uMsg, wParam, lParam);
		if(lRet == 0)
			return 0;

		int iCol = (int)wParam; 
		if(m_iSortColumn == iCol)
			m_iSortColumn = -1;
		else if(m_iSortColumn > iCol)
			m_iSortColumn--;
		m_arrColSortType.RemoveAt(iCol);

		return lRet;
	}

	LRESULT OnHeaderItemClick(int /*idCtrl*/, LPNMHDR pnmh, BOOL& bHandled)
	{
		LPNMHEADER p = (LPNMHEADER)pnmh;
		if(p->iButton == 0)
		{
			int iOld = m_iSortColumn;
			bool bDescending = (m_iSortColumn == p->iItem) ? !m_bSortDescending : false;
			if(DoSortItems(p->iItem, bDescending))
				NotifyParentSortChanged(p->iItem, iOld);				
		}
		bHandled = FALSE;
		return 0;
	}

	LRESULT OnSettingChange(UINT /*uMsg*/, WPARAM wParam, LPARAM /*lParam*/, BOOL& bHandled)
	{
		if(wParam == SPI_SETNONCLIENTMETRICS)
			GetSystemSettings();
		bHandled = FALSE;
		return 0;
	}

	void GetSystemSettings()
	{
		if(!m_bCommCtrl6 && !m_bmSort[m_iSortUp].IsNull())
		{
			T* pT = static_cast<T*>(this);
			pT->CreateSortBitmaps();
			if(m_iSortColumn != -1)
				SetSortColumn(m_iSortColumn);
		}
	}

};


typedef ATL::CWinTraits<WS_CHILD | WS_VISIBLE | WS_CLIPCHILDREN | WS_CLIPSIBLINGS | LVS_REPORT | LVS_SHOWSELALWAYS , WS_EX_CLIENTEDGE>   CSortListViewCtrlTraits;

template <class T, class TBase = CListViewCtrl, class TWinTraits = CSortListViewCtrlTraits>
class ATL_NO_VTABLE CSortListViewCtrlImpl: public ATL::CWindowImpl<T, TBase, TWinTraits>, public CSortListViewImpl<T>
{
public:
	DECLARE_WND_SUPERCLASS2(NULL, T, TBase::GetWndClassName())

	bool SortItems(int iCol, bool bDescending = false)
	{
		return this->DoSortItems(iCol, bDescending);
	}
		
	BEGIN_MSG_MAP(CSortListViewCtrlImpl)
		MESSAGE_HANDLER(LVM_INSERTCOLUMN, CSortListViewImpl<T>::OnInsertColumn)
		MESSAGE_HANDLER(LVM_DELETECOLUMN, CSortListViewImpl<T>::OnDeleteColumn)
		NOTIFY_CODE_HANDLER(HDN_ITEMCLICKA, CSortListViewImpl<T>::OnHeaderItemClick)
		NOTIFY_CODE_HANDLER(HDN_ITEMCLICKW, CSortListViewImpl<T>::OnHeaderItemClick)
		MESSAGE_HANDLER(WM_SETTINGCHANGE, CSortListViewImpl<T>::OnSettingChange)
	END_MSG_MAP()
};

class CSortListViewCtrl : public CSortListViewCtrlImpl<CSortListViewCtrl>
{
public:
	DECLARE_WND_SUPERCLASS(_T("WTL_SortListViewCtrl"), GetWndClassName())
};


///////////////////////////////////////////////////////////////////////////////
// CTabView - implements tab view window

// TabView Notifications
#define TBVN_PAGEACTIVATED   (0U-741)
#define TBVN_CONTEXTMENU     (0U-742)

// Notification data for TBVN_CONTEXTMENU
struct TBVCONTEXTMENUINFO
{
	NMHDR hdr;
	POINT pt;
};

typedef TBVCONTEXTMENUINFO* LPTBVCONTEXTMENUINFO;


template <class T, class TBase = ATL::CWindow, class TWinTraits = ATL::CControlWinTraits>
class ATL_NO_VTABLE CTabViewImpl : public ATL::CWindowImpl< T, TBase, TWinTraits >
{
public:
	DECLARE_WND_CLASS_EX2(NULL, T, 0, COLOR_APPWORKSPACE)

// Declarations and enums
	struct TABVIEWPAGE
	{
		HWND hWnd;
		LPTSTR lpstrTitle;
		LPVOID pData;
	};

	struct TCITEMEXTRA
	{
		TCITEMHEADER tciheader;
		TABVIEWPAGE tvpage;

		operator LPTCITEM() { return (LPTCITEM)this; }
	};

	enum
	{
		m_nTabID = 1313,
		m_cxMoveMark = 6,
		m_cyMoveMark = 3,
		m_nMenuItemsMax = (ID_WINDOW_TABLAST - ID_WINDOW_TABFIRST + 1)
	};

	enum { _nAutoScrollTimerID = 4321 };

	enum AutoScroll
	{
		_AUTOSCROLL_NONE = 0,
		_AUTOSCROLL_LEFT = -1,
		_AUTOSCROLL_RIGHT = 1
	};

// Data members
	ATL::CContainedWindowT<CTabCtrl> m_tab;
	int m_cyTabHeight;

	int m_nActivePage;

	int m_nInsertItem;
	POINT m_ptStartDrag;

	CMenuHandle m_menu;

	int m_cchTabTextLength;

	int m_nMenuItemsCount;

	ATL::CWindow m_wndTitleBar;
	LPTSTR m_lpstrTitleBarBase;
	int m_cchTitleBarLength;

	CImageList m_ilDrag;

	AutoScroll m_AutoScroll;
	CUpDownCtrl m_ud;

	bool m_bDestroyPageOnRemove:1;
	bool m_bDestroyImageList:1;
	bool m_bActivePageMenuItem:1;
	bool m_bActiveAsDefaultMenuItem:1;
	bool m_bEmptyMenuItem:1;
	bool m_bWindowsMenuItem:1;
	bool m_bNoTabDrag:1;
	bool m_bNoTabDragAutoScroll:1;
	// internal
	bool m_bTabCapture:1;
	bool m_bTabDrag:1;
	bool m_bInternalFont:1;

// Constructor/destructor
	CTabViewImpl() :
			m_tab(this, 1),
			m_cyTabHeight(0),
			m_nActivePage(-1),
			m_nInsertItem(-1), 
			m_cchTabTextLength(30), 
			m_nMenuItemsCount(10), 
			m_lpstrTitleBarBase(NULL), 
			m_cchTitleBarLength(100), 
	                m_AutoScroll(_AUTOSCROLL_NONE), 
			m_bDestroyPageOnRemove(true), 
			m_bDestroyImageList(true), 
			m_bActivePageMenuItem(true), 
			m_bActiveAsDefaultMenuItem(false), 
			m_bEmptyMenuItem(false), 
			m_bWindowsMenuItem(false), 
			m_bNoTabDrag(false), 
	                m_bNoTabDragAutoScroll(false), 
			m_bTabCapture(false), 
			m_bTabDrag(false), 
			m_bInternalFont(false)
	{
		m_ptStartDrag.x = 0;
		m_ptStartDrag.y = 0;
	}

	~CTabViewImpl()
	{
		delete [] m_lpstrTitleBarBase;
	}

// Message filter function - to be called from PreTranslateMessage of the main window
	BOOL PreTranslateMessage(MSG* pMsg)
	{
		if(this->IsWindow() == FALSE)
			return FALSE;

		BOOL bRet = FALSE;

		// Check for TabView built-in accelerators (Ctrl+Tab/Ctrl+Shift+Tab - next/previous page)
		int nCount = GetPageCount();
		if(nCount > 0)
		{
			bool bControl = (::GetKeyState(VK_CONTROL) < 0);
			if((pMsg->message == WM_KEYDOWN) && (pMsg->wParam == VK_TAB) && bControl)
			{
				if(nCount > 1)
				{
					int nPage = m_nActivePage;
					bool bShift = (::GetKeyState(VK_SHIFT) < 0);
					if(bShift)
						nPage = (nPage > 0) ? (nPage - 1) : (nCount - 1);
					else
						nPage = ((nPage >= 0) && (nPage < (nCount - 1))) ? (nPage + 1) : 0;

					SetActivePage(nPage);
					T* pT = static_cast<T*>(this);
					pT->OnPageActivated(m_nActivePage);
				}

				bRet = TRUE;
			}
		}

		// If we are doing drag-drop, check for Escape key that cancels it
		if(bRet == FALSE)
		{
			if(m_bTabCapture && (pMsg->message == WM_KEYDOWN) && (pMsg->wParam == VK_ESCAPE))
			{
				::ReleaseCapture();
				bRet = TRUE;
			}
		}

		// Pass the message to the active page
		if(bRet == FALSE)
		{
			if(m_nActivePage != -1)
				bRet = (BOOL)::SendMessage(GetPageHWND(m_nActivePage), WM_FORWARDMSG, 0, (LPARAM)pMsg);
		}

		return bRet;
	}

// Attributes
	int GetPageCount() const
	{
		ATLASSERT(::IsWindow(this->m_hWnd));
		return m_tab.GetItemCount();
	}

	int GetActivePage() const
	{
		return m_nActivePage;
	}

	void SetActivePage(int nPage)
	{
		ATLASSERT(::IsWindow(this->m_hWnd));
		ATLASSERT(IsValidPageIndex(nPage));

		T* pT = static_cast<T*>(this);

		this->SetRedraw(FALSE);

		if(m_nActivePage != -1)
			::ShowWindow(GetPageHWND(m_nActivePage), SW_HIDE);
		m_nActivePage = nPage;
		m_tab.SetCurSel(m_nActivePage);
		::ShowWindow(GetPageHWND(m_nActivePage), SW_SHOW);

		pT->UpdateLayout();

		this->SetRedraw(TRUE);
		this->RedrawWindow(NULL, NULL, RDW_FRAME | RDW_INVALIDATE | RDW_UPDATENOW | RDW_ALLCHILDREN);

		if(::GetFocus() != m_tab.m_hWnd)
			::SetFocus(GetPageHWND(m_nActivePage));

		pT->UpdateTitleBar();
		pT->UpdateMenu();
	}

	HIMAGELIST GetImageList() const
	{
		ATLASSERT(::IsWindow(this->m_hWnd));
		return m_tab.GetImageList();
	}

	HIMAGELIST SetImageList(HIMAGELIST hImageList)
	{
		ATLASSERT(::IsWindow(this->m_hWnd));
		return m_tab.SetImageList(hImageList);
	}

	void SetWindowMenu(HMENU hMenu)
	{
		ATLASSERT(::IsWindow(this->m_hWnd));

		m_menu = hMenu;

		T* pT = static_cast<T*>(this);
		pT->UpdateMenu();
	}

	void SetTitleBarWindow(HWND hWnd)
	{
		ATLASSERT(::IsWindow(this->m_hWnd));

		delete [] m_lpstrTitleBarBase;
		m_lpstrTitleBarBase = NULL;

		m_wndTitleBar = hWnd;
		if(hWnd == NULL)
			return;

		int cchLen = m_wndTitleBar.GetWindowTextLength() + 1;
		ATLTRY(m_lpstrTitleBarBase = new TCHAR[cchLen]);
		if(m_lpstrTitleBarBase != NULL)
		{
			m_wndTitleBar.GetWindowText(m_lpstrTitleBarBase, cchLen);
			T* pT = static_cast<T*>(this);
			pT->UpdateTitleBar();
		}
	}

// Page attributes
	HWND GetPageHWND(int nPage) const
	{
		ATLASSERT(::IsWindow(this->m_hWnd));
		ATLASSERT(IsValidPageIndex(nPage));

		TCITEMEXTRA tcix = {};
		tcix.tciheader.mask = TCIF_PARAM;
		m_tab.GetItem(nPage, tcix);

		return tcix.tvpage.hWnd;
	}

	LPCTSTR GetPageTitle(int nPage) const
	{
		ATLASSERT(::IsWindow(this->m_hWnd));
		ATLASSERT(IsValidPageIndex(nPage));

		TCITEMEXTRA tcix = {};
		tcix.tciheader.mask = TCIF_PARAM;
		if(m_tab.GetItem(nPage, tcix) == FALSE)
			return NULL;

		return tcix.tvpage.lpstrTitle;
	}

	bool SetPageTitle(int nPage, LPCTSTR lpstrTitle)
	{
		ATLASSERT(::IsWindow(this->m_hWnd));
		ATLASSERT(IsValidPageIndex(nPage));

		T* pT = static_cast<T*>(this);

		int cchBuff = lstrlen(lpstrTitle) + 1;
		LPTSTR lpstrBuff = NULL;
		ATLTRY(lpstrBuff = new TCHAR[cchBuff]);
		if(lpstrBuff == NULL)
			return false;

		ATL::Checked::tcscpy_s(lpstrBuff, cchBuff, lpstrTitle);
		TCITEMEXTRA tcix = {};
		tcix.tciheader.mask = TCIF_PARAM;
		if(m_tab.GetItem(nPage, tcix) == FALSE)
			return false;

		ATL::CTempBuffer<TCHAR, _WTL_STACK_ALLOC_THRESHOLD> buff;
		LPTSTR lpstrTabText = buff.Allocate(m_cchTabTextLength + 1);
		if(lpstrTabText == NULL)
			return false;

		delete [] tcix.tvpage.lpstrTitle;

		pT->ShortenTitle(lpstrTitle, lpstrTabText, m_cchTabTextLength + 1);

		tcix.tciheader.mask = TCIF_TEXT | TCIF_PARAM;
		tcix.tciheader.pszText = lpstrTabText;
		tcix.tvpage.lpstrTitle = lpstrBuff;
		if(m_tab.SetItem(nPage, tcix) == FALSE)
			return false;

		pT->UpdateTitleBar();
		pT->UpdateMenu();

		return true;
	}

	LPVOID GetPageData(int nPage) const
	{
		ATLASSERT(::IsWindow(this->m_hWnd));
		ATLASSERT(IsValidPageIndex(nPage));

		TCITEMEXTRA tcix = {};
		tcix.tciheader.mask = TCIF_PARAM;
		m_tab.GetItem(nPage, tcix);

		return tcix.tvpage.pData;
	}

	LPVOID SetPageData(int nPage, LPVOID pData)
	{
		ATLASSERT(::IsWindow(this->m_hWnd));
		ATLASSERT(IsValidPageIndex(nPage));

		TCITEMEXTRA tcix = {};
		tcix.tciheader.mask = TCIF_PARAM;
		m_tab.GetItem(nPage, tcix);
		LPVOID pDataOld = tcix.tvpage.pData;

		tcix.tvpage.pData = pData;
		m_tab.SetItem(nPage, tcix);

		return pDataOld;
	}

	int GetPageImage(int nPage) const
	{
		ATLASSERT(::IsWindow(this->m_hWnd));
		ATLASSERT(IsValidPageIndex(nPage));

		TCITEMEXTRA tcix = {};
		tcix.tciheader.mask = TCIF_IMAGE;
		m_tab.GetItem(nPage, tcix);

		return tcix.tciheader.iImage;
	}

	int SetPageImage(int nPage, int nImage)
	{
		ATLASSERT(::IsWindow(this->m_hWnd));
		ATLASSERT(IsValidPageIndex(nPage));

		TCITEMEXTRA tcix = {};
		tcix.tciheader.mask = TCIF_IMAGE;
		m_tab.GetItem(nPage, tcix);
		int nImageOld = tcix.tciheader.iImage;

		tcix.tciheader.iImage = nImage;
		m_tab.SetItem(nPage, tcix);

		return nImageOld;
	}

// Operations
	bool AddPage(HWND hWndView, LPCTSTR lpstrTitle, int nImage = -1, LPVOID pData = NULL)
	{
		return InsertPage(GetPageCount(), hWndView, lpstrTitle, nImage, pData);
	}

	bool InsertPage(int nPage, HWND hWndView, LPCTSTR lpstrTitle, int nImage = -1, LPVOID pData = NULL)
	{
		ATLASSERT(::IsWindow(this->m_hWnd));
		ATLASSERT((nPage == GetPageCount()) || IsValidPageIndex(nPage));

		T* pT = static_cast<T*>(this);

		int cchBuff = lstrlen(lpstrTitle) + 1;
		LPTSTR lpstrBuff = NULL;
		ATLTRY(lpstrBuff = new TCHAR[cchBuff]);
		if(lpstrBuff == NULL)
			return false;

		ATL::Checked::tcscpy_s(lpstrBuff, cchBuff, lpstrTitle);

		ATL::CTempBuffer<TCHAR, _WTL_STACK_ALLOC_THRESHOLD> buff;
		LPTSTR lpstrTabText = buff.Allocate(m_cchTabTextLength + 1);
		if(lpstrTabText == NULL)
			return false;

		pT->ShortenTitle(lpstrTitle, lpstrTabText, m_cchTabTextLength + 1);

		this->SetRedraw(FALSE);

		TCITEMEXTRA tcix = {};
		tcix.tciheader.mask = TCIF_TEXT | TCIF_IMAGE | TCIF_PARAM;
		tcix.tciheader.pszText = lpstrTabText;
		tcix.tciheader.iImage = nImage;
		tcix.tvpage.hWnd = hWndView;
		tcix.tvpage.lpstrTitle = lpstrBuff;
		tcix.tvpage.pData = pData;
		int nItem = m_tab.InsertItem(nPage, tcix);
		if(nItem == -1)
		{
			delete [] lpstrBuff;
			this->SetRedraw(TRUE);
			return false;
		}

		// adjust active page index, if inserted before it
		if(nPage <= m_nActivePage)
			m_nActivePage++;

		SetActivePage(nItem);
		pT->OnPageActivated(m_nActivePage);

		if(GetPageCount() == 1)
			pT->ShowTabControl(true);

		pT->UpdateLayout();

		this->SetRedraw(TRUE);
		this->RedrawWindow(NULL, NULL, RDW_FRAME | RDW_INVALIDATE | RDW_UPDATENOW | RDW_ALLCHILDREN);

		return true;
	}

	void RemovePage(int nPage)
	{
		ATLASSERT(::IsWindow(this->m_hWnd));
		ATLASSERT(IsValidPageIndex(nPage));

		T* pT = static_cast<T*>(this);

		this->SetRedraw(FALSE);

		if(GetPageCount() == 1)
			pT->ShowTabControl(false);

		if(m_bDestroyPageOnRemove)
			::DestroyWindow(GetPageHWND(nPage));
		else
			::ShowWindow(GetPageHWND(nPage), SW_HIDE);
		LPTSTR lpstrTitle = (LPTSTR)GetPageTitle(nPage);
		delete [] lpstrTitle;

		ATLVERIFY(m_tab.DeleteItem(nPage) != FALSE);

		if(m_nActivePage == nPage)
		{
			m_nActivePage = -1;

			if(nPage > 0)
			{
				SetActivePage(nPage - 1);
			}
			else if(GetPageCount() > 0)
			{
				SetActivePage(nPage);
			}
			else
			{
				this->SetRedraw(TRUE);
				this->Invalidate();
				this->UpdateWindow();
				pT->UpdateTitleBar();
				pT->UpdateMenu();
			}
		}
		else
		{
			nPage = (nPage < m_nActivePage) ? (m_nActivePage - 1) : m_nActivePage;
			m_nActivePage = -1;
			SetActivePage(nPage);
		}

		pT->OnPageActivated(m_nActivePage);
	}

	void RemoveAllPages()
	{
		ATLASSERT(::IsWindow(this->m_hWnd));

		if(GetPageCount() == 0)
			return;

		T* pT = static_cast<T*>(this);

		this->SetRedraw(FALSE);

		pT->ShowTabControl(false);

		for(int i = 0; i < GetPageCount(); i++)
		{
			if(m_bDestroyPageOnRemove)
				::DestroyWindow(GetPageHWND(i));
			else
				::ShowWindow(GetPageHWND(i), SW_HIDE);
			LPTSTR lpstrTitle = (LPTSTR)GetPageTitle(i);
			delete [] lpstrTitle;
		}
		m_tab.DeleteAllItems();

		m_nActivePage = -1;
		pT->OnPageActivated(m_nActivePage);

		this->SetRedraw(TRUE);
		this->Invalidate();
		this->UpdateWindow();

		pT->UpdateTitleBar();
		pT->UpdateMenu();
	}

	int PageIndexFromHwnd(HWND hWnd) const
	{
		int nIndex = -1;

		for(int i = 0; i < GetPageCount(); i++)
		{
			if(GetPageHWND(i) == hWnd)
			{
				nIndex = i;
				break;
			}
		}

		return nIndex;
	}

	void BuildWindowMenu(HMENU hMenu, int nMenuItemsCount = 10, bool bEmptyMenuItem = true, bool bWindowsMenuItem = true, bool bActivePageMenuItem = true, bool bActiveAsDefaultMenuItem = false)
	{
		ATLASSERT(::IsWindow(this->m_hWnd));

		CMenuHandle menu = hMenu;
		T* pT = static_cast<T*>(this);
		(void)pT;   // avoid level 4 warning
		int nFirstPos = 0;

		// Find first menu item in our range
		for(nFirstPos = 0; nFirstPos < menu.GetMenuItemCount(); nFirstPos++)
		{
			UINT nID = menu.GetMenuItemID(nFirstPos);
			if(((nID >= ID_WINDOW_TABFIRST) && (nID <= ID_WINDOW_TABLAST)) || (nID == ID_WINDOW_SHOWTABLIST))
				break;
		}

		// Remove all menu items for tab pages
		BOOL bRet = TRUE;
		while(bRet != FALSE)
			bRet = menu.DeleteMenu(nFirstPos, MF_BYPOSITION);

		// Add separator if it's not already there
		int nPageCount = GetPageCount();
		if((bWindowsMenuItem || (nPageCount > 0)) && (nFirstPos > 0))
		{
			CMenuItemInfo mii;
			mii.fMask = MIIM_TYPE;
			menu.GetMenuItemInfo(nFirstPos - 1, TRUE, &mii);
			if((nFirstPos <= 0) || ((mii.fType & MFT_SEPARATOR) == 0))
			{
				menu.AppendMenu(MF_SEPARATOR);
				nFirstPos++;
			}
		}

		// Add menu items for all pages
		if(nPageCount > 0)
		{
			// Append menu items for all pages
			const int cchPrefix = 3;   // 2 digits + space
			nMenuItemsCount = __min(__min(nPageCount, nMenuItemsCount), (int)m_nMenuItemsMax);
			ATLASSERT(nMenuItemsCount < 100);   // 2 digits only
			if(nMenuItemsCount >= 100)
				nMenuItemsCount = 99;

			for(int i = 0; i < nMenuItemsCount; i++)
			{
				LPCTSTR lpstrTitle = GetPageTitle(i);
				int nLen = lstrlen(lpstrTitle);
				ATL::CTempBuffer<TCHAR, _WTL_STACK_ALLOC_THRESHOLD> buff;
				LPTSTR lpstrText = buff.Allocate(cchPrefix + nLen + 1);
				ATLASSERT(lpstrText != NULL);
				if(lpstrText != NULL)
				{
					LPCTSTR lpstrFormat = (i < 9) ? _T("&%i %s") : _T("%i %s");
					_stprintf_s(lpstrText, cchPrefix + nLen + 1, lpstrFormat, i + 1, lpstrTitle);
					menu.AppendMenu(MF_STRING, ID_WINDOW_TABFIRST + i, lpstrText);
				}
			}

			// Mark active page
			if(bActivePageMenuItem && (m_nActivePage != -1))
			{
				if(bActiveAsDefaultMenuItem)
				{
					menu.SetMenuDefaultItem((UINT)-1,  TRUE);
					menu.SetMenuDefaultItem(nFirstPos + m_nActivePage,  TRUE);
				}
				else
				{
					menu.CheckMenuRadioItem(nFirstPos, nFirstPos + nMenuItemsCount, nFirstPos + m_nActivePage, MF_BYPOSITION);
				}
			}
		}
		else
		{
			if(bEmptyMenuItem)
			{
				menu.AppendMenu(MF_BYPOSITION | MF_STRING, ID_WINDOW_TABFIRST, pT->GetEmptyListText());
				menu.EnableMenuItem(ID_WINDOW_TABFIRST, MF_GRAYED);
			}

			// Remove separator if nothing else is there
			if(!bEmptyMenuItem && !bWindowsMenuItem && (nFirstPos > 0))
			{
				CMenuItemInfo mii;
				mii.fMask = MIIM_TYPE;
				menu.GetMenuItemInfo(nFirstPos - 1, TRUE, &mii);
				if((mii.fType & MFT_SEPARATOR) != 0)
					menu.DeleteMenu(nFirstPos - 1, MF_BYPOSITION);
			}
		}

		// Add "Windows..." menu item
		if(bWindowsMenuItem)
			menu.AppendMenu(MF_BYPOSITION | MF_STRING, ID_WINDOW_SHOWTABLIST, pT->GetWindowsMenuItemText());
	}

	BOOL SubclassWindow(HWND hWnd)
	{
		BOOL bRet = ATL::CWindowImpl< T, TBase, TWinTraits >::SubclassWindow(hWnd);
		if(bRet != FALSE)
		{
			T* pT = static_cast<T*>(this);
			pT->CreateTabControl();
			pT->UpdateLayout();
		}

		return bRet;
	}

// Message map and handlers
	BEGIN_MSG_MAP(CTabViewImpl)
		MESSAGE_HANDLER(WM_CREATE, OnCreate)
		MESSAGE_HANDLER(WM_DESTROY, OnDestroy)
		MESSAGE_HANDLER(WM_SIZE, OnSize)
		MESSAGE_HANDLER(WM_SETFOCUS, OnSetFocus)
		MESSAGE_HANDLER(WM_GETFONT, OnGetFont)
		MESSAGE_HANDLER(WM_SETFONT, OnSetFont)
		MESSAGE_HANDLER(WM_TIMER, OnTimer)
		MESSAGE_HANDLER(WM_CONTEXTMENU, OnTabContextMenu)
		NOTIFY_HANDLER(m_nTabID, TCN_SELCHANGE, OnTabChanged)
		NOTIFY_ID_HANDLER(m_nTabID, OnTabNotification)
		NOTIFY_CODE_HANDLER(TTN_GETDISPINFO, OnTabGetDispInfo)
		FORWARD_NOTIFICATIONS()
	ALT_MSG_MAP(1)   // tab control
		MESSAGE_HANDLER(WM_LBUTTONDOWN, OnTabLButtonDown)
		MESSAGE_HANDLER(WM_LBUTTONUP, OnTabLButtonUp)
		MESSAGE_HANDLER(WM_CAPTURECHANGED, OnTabCaptureChanged)
		MESSAGE_HANDLER(WM_MOUSEMOVE, OnTabMouseMove)
	END_MSG_MAP()

	LRESULT OnCreate(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		T* pT = static_cast<T*>(this);
		pT->CreateTabControl();

		return 0;
	}

	LRESULT OnDestroy(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		RemoveAllPages();

		if(m_bDestroyImageList)
		{
			CImageList il = m_tab.SetImageList(NULL);
			if(il.m_hImageList != NULL)
				il.Destroy();
		}

		if(m_bInternalFont)
		{
			HFONT hFont = m_tab.GetFont();
			m_tab.SetFont(NULL, FALSE);
			::DeleteObject(hFont);
			m_bInternalFont = false;
		}

		m_ud.m_hWnd = NULL;

		return 0;
	}

	LRESULT OnSize(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		T* pT = static_cast<T*>(this);
		pT->UpdateLayout();
		return 0;
	}

	LRESULT OnSetFocus(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		if(m_nActivePage != -1)
			::SetFocus(GetPageHWND(m_nActivePage));
		return 0;
	}

	LRESULT OnGetFont(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		return m_tab.SendMessage(WM_GETFONT);
	}

	LRESULT OnSetFont(UINT /*uMsg*/, WPARAM wParam, LPARAM lParam, BOOL& /*bHandled*/)
	{
		if(m_bInternalFont)
		{
			HFONT hFont = m_tab.GetFont();
			m_tab.SetFont(NULL, FALSE);
			::DeleteObject(hFont);
			m_bInternalFont = false;
		}

		m_tab.SendMessage(WM_SETFONT, wParam, lParam);

		T* pT = static_cast<T*>(this);
		m_cyTabHeight = pT->CalcTabHeight();

		if((BOOL)lParam != FALSE)
			pT->UpdateLayout();

		return 0;
	}

	LRESULT OnTimer(UINT /*uMsg*/, WPARAM wParam, LPARAM /*lParam*/, BOOL& bHandled)
	{
		if(wParam == _nAutoScrollTimerID)
		{
			T* pT = static_cast<T*>(this);
			pT->DoAutoScroll();
		}
		else
		{
			bHandled = FALSE;
		}

		return 0;
	}

	LRESULT OnTabContextMenu(UINT /*uMsg*/, WPARAM wParam, LPARAM lParam, BOOL& bHandled)
	{
		POINT pt = { GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam) };
		int nPage = m_nActivePage;
		bool bAction = false;
		if((HWND)wParam == m_tab.m_hWnd)
		{
			if((pt.x == -1) && (pt.y == -1))   // keyboard
			{
				RECT rect = {};
				m_tab.GetItemRect(m_nActivePage, &rect);
				pt.x = rect.left;
				pt.y = rect.bottom;
				m_tab.ClientToScreen(&pt);
				bAction = true;
			}
			else if(::WindowFromPoint(pt) == m_tab.m_hWnd)
			{
				TCHITTESTINFO hti = {};
				hti.pt = pt;
				this->ScreenToClient(&hti.pt);
				nPage = m_tab.HitTest(&hti);

				bAction = true;
			}
		}

		if(bAction)
		{
			T* pT = static_cast<T*>(this);
			pT->OnContextMenu(nPage, pt);
		}
		else
		{
			bHandled = FALSE;
		}

		return 0;
	}

	LRESULT OnTabChanged(int /*idCtrl*/, LPNMHDR /*pnmh*/, BOOL& /*bHandled*/)
	{
		SetActivePage(m_tab.GetCurSel());
		T* pT = static_cast<T*>(this);
		pT->OnPageActivated(m_nActivePage);

		return 0;
	}

	LRESULT OnTabNotification(int /*idCtrl*/, LPNMHDR /*pnmh*/, BOOL& /*bHandled*/)
	{
		// nothing to do - this just blocks all tab control
		// notifications from being propagated further
		return 0;
	}

	LRESULT OnTabGetDispInfo(int /*idCtrl*/, LPNMHDR pnmh, BOOL& bHandled)
	{
		LPNMTTDISPINFO pTTDI = (LPNMTTDISPINFO)pnmh;
		if(pTTDI->hdr.hwndFrom == m_tab.GetTooltips())
		{
			T* pT = static_cast<T*>(this);
			pT->UpdateTooltipText(pTTDI);
		}
		else
		{
			bHandled = FALSE;
		}

		return 0;
	}

// Tab control message handlers
	LRESULT OnTabLButtonDown(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM lParam, BOOL& bHandled)
	{
		if(!m_bNoTabDrag && (m_tab.GetItemCount() > 1))
		{
			m_bTabCapture = true;
			m_tab.SetCapture();

			m_ptStartDrag.x = GET_X_LPARAM(lParam);
			m_ptStartDrag.y = GET_Y_LPARAM(lParam);
		}

		bHandled = FALSE;
		return 0;
	}

	LRESULT OnTabLButtonUp(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM lParam, BOOL& bHandled)
	{
		if(m_bTabCapture)
		{
			if(m_bTabDrag)
			{
				T* pT = static_cast<T*>(this);
				POINT pt = { GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam) };
				int nItem = pT->DragHitTest(pt);
				if(nItem != -1)
					MovePage(m_nActivePage, nItem);
			}

			::ReleaseCapture();
		}

		bHandled = FALSE;
		return 0;
	}

	LRESULT OnTabCaptureChanged(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& bHandled)
	{
		if(m_bTabCapture)
		{
			m_bTabCapture = false;

			if(m_bTabDrag)
			{
				m_bTabDrag = false;

				T* pT = static_cast<T*>(this);
				if(!m_bNoTabDragAutoScroll)
					pT->StartStopAutoScroll(-1);

				pT->DrawMoveMark(-1);

				m_ilDrag.DragLeave(GetDesktopWindow());
				m_ilDrag.EndDrag();

				m_ilDrag.Destroy();
				m_ilDrag.m_hImageList = NULL;
			}
		}

		bHandled = FALSE;
		return 0;
	}

	LRESULT OnTabMouseMove(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM lParam, BOOL& bHandled)
	{
		bHandled = FALSE;

		if(m_bTabCapture)
		{
			POINT pt = { GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam) };

			if(!m_bTabDrag)
			{
				if((abs(m_ptStartDrag.x - GET_X_LPARAM(lParam)) >= ::GetSystemMetrics(SM_CXDRAG)) ||
				   (abs(m_ptStartDrag.y - GET_Y_LPARAM(lParam)) >= ::GetSystemMetrics(SM_CYDRAG)))
				{
					T* pT = static_cast<T*>(this);
					pT->GenerateDragImage(m_nActivePage);

					int cxCursor = ::GetSystemMetrics(SM_CXCURSOR);
					int cyCursor = ::GetSystemMetrics(SM_CYCURSOR);
					m_ilDrag.BeginDrag(0, -(cxCursor / 2), -(cyCursor / 2));
					POINT ptEnter = m_ptStartDrag;
					m_tab.ClientToScreen(&ptEnter);
					m_ilDrag.DragEnter(GetDesktopWindow(), ptEnter);

					m_bTabDrag = true;
				}
			}

			if(m_bTabDrag)
			{
				T* pT = static_cast<T*>(this);
				int nItem = pT->DragHitTest(pt);

				pT->SetMoveCursor(nItem != -1);

				if(m_nInsertItem != nItem)
					pT->DrawMoveMark(nItem);

				if(!m_bNoTabDragAutoScroll)
					pT->StartStopAutoScroll(pt.x);

				m_ilDrag.DragShowNolock((nItem != -1) ? TRUE : FALSE);
				m_tab.ClientToScreen(&pt);
				m_ilDrag.DragMove(pt);

				bHandled = TRUE;
			}
		}

		return 0;
	}

// Implementation helpers
	bool IsValidPageIndex(int nPage) const
	{
		return ((nPage >= 0) && (nPage < GetPageCount()));
	}

	bool MovePage(int nMovePage, int nInsertBeforePage)
	{
		ATLASSERT(IsValidPageIndex(nMovePage));
		ATLASSERT(IsValidPageIndex(nInsertBeforePage));

		if(!IsValidPageIndex(nMovePage) || !IsValidPageIndex(nInsertBeforePage))
			return false;

		if(nMovePage == nInsertBeforePage)
			return true;   // nothing to do

		ATL::CTempBuffer<TCHAR, _WTL_STACK_ALLOC_THRESHOLD> buff;
		LPTSTR lpstrTabText = buff.Allocate(m_cchTabTextLength + 1);
		if(lpstrTabText == NULL)
			return false;
		TCITEMEXTRA tcix = {};
		tcix.tciheader.mask = TCIF_TEXT | TCIF_IMAGE | TCIF_PARAM;
		tcix.tciheader.pszText = lpstrTabText;
		tcix.tciheader.cchTextMax = m_cchTabTextLength + 1;
		BOOL bRet = m_tab.GetItem(nMovePage, tcix);
		ATLASSERT(bRet != FALSE);
		if(bRet == FALSE)
			return false;

		int nInsertItem = (nInsertBeforePage > nMovePage) ? nInsertBeforePage + 1 : nInsertBeforePage;
		int nNewItem = m_tab.InsertItem(nInsertItem, tcix);
		ATLASSERT(nNewItem == nInsertItem);
		if(nNewItem != nInsertItem)
		{
			ATLVERIFY(m_tab.DeleteItem(nNewItem));
			return false;
		}

		if(nMovePage > nInsertBeforePage)
			ATLVERIFY(m_tab.DeleteItem(nMovePage + 1) != FALSE);
		else if(nMovePage < nInsertBeforePage)
			ATLVERIFY(m_tab.DeleteItem(nMovePage) != FALSE);

		SetActivePage(nInsertBeforePage);
		T* pT = static_cast<T*>(this);
		pT->OnPageActivated(m_nActivePage);

		return true;
	}

// Implementation overrideables
	bool CreateTabControl()
	{
		m_tab.Create(this->m_hWnd, this->rcDefault, NULL, WS_CHILD | WS_CLIPSIBLINGS | WS_CLIPCHILDREN | TCS_TOOLTIPS, 0, m_nTabID);
		ATLASSERT(m_tab.m_hWnd != NULL);
		if(m_tab.m_hWnd == NULL)
			return false;

		m_tab.SetFont(AtlCreateControlFont());
		m_bInternalFont = true;

		m_tab.SetItemExtra(sizeof(TABVIEWPAGE));

		T* pT = static_cast<T*>(this);
		m_cyTabHeight = pT->CalcTabHeight();

		return true;
	}

	int CalcTabHeight()
	{
		int nCount = m_tab.GetItemCount();
		TCHAR szText[] = _T("NS");
		TCITEMEXTRA tcix = {};
		tcix.tciheader.mask = TCIF_TEXT;
		tcix.tciheader.pszText = szText;
		int nIndex = m_tab.InsertItem(nCount, tcix);

		RECT rect = { 0, 0, 1000, 1000 };
		m_tab.AdjustRect(FALSE, &rect);

		RECT rcWnd = { 0, 0, 1000, rect.top };
		::AdjustWindowRectEx(&rcWnd, m_tab.GetStyle(), FALSE, m_tab.GetExStyle());

		int nHeight = rcWnd.bottom - rcWnd.top;

		m_tab.DeleteItem(nIndex);

		return nHeight;
	}

	void ShowTabControl(bool bShow)
	{
		m_tab.ShowWindow(bShow ? SW_SHOWNOACTIVATE : SW_HIDE);
		T* pT = static_cast<T*>(this);
		pT->UpdateLayout();
	}

	void UpdateLayout()
	{
		RECT rect = {};
		this->GetClientRect(&rect);

		int cyOffset = 0;
		if(m_tab.IsWindow() && ((m_tab.GetStyle() & WS_VISIBLE) != 0))
		{
			m_tab.SetWindowPos(NULL, 0, 0, rect.right - rect.left, m_cyTabHeight, SWP_NOZORDER);
			cyOffset = m_cyTabHeight;
		}

		if(m_nActivePage != -1)
			::SetWindowPos(GetPageHWND(m_nActivePage), NULL, 0, cyOffset, rect.right - rect.left, rect.bottom - rect.top - cyOffset, SWP_NOZORDER);
	}

	void UpdateMenu()
	{
		if(m_menu.m_hMenu != NULL)
			BuildWindowMenu(m_menu, m_nMenuItemsCount, m_bEmptyMenuItem, m_bWindowsMenuItem, m_bActivePageMenuItem, m_bActiveAsDefaultMenuItem);
	}

	void UpdateTitleBar()
	{
		if(!m_wndTitleBar.IsWindow() || (m_lpstrTitleBarBase == NULL))
			return;   // nothing to do

		if(m_nActivePage != -1)
		{
			T* pT = static_cast<T*>(this);
			LPCTSTR lpstrTitle = pT->GetPageTitle(m_nActivePage);
			LPCTSTR lpstrDivider = pT->GetTitleDividerText();
			int cchBuffer = m_cchTitleBarLength + lstrlen(lpstrDivider) + lstrlen(m_lpstrTitleBarBase) + 1;
			ATL::CTempBuffer<TCHAR, _WTL_STACK_ALLOC_THRESHOLD> buff;
			LPTSTR lpstrPageTitle = buff.Allocate(cchBuffer);
			ATLASSERT(lpstrPageTitle != NULL);
			if(lpstrPageTitle != NULL)
			{
				pT->ShortenTitle(lpstrTitle, lpstrPageTitle, m_cchTitleBarLength + 1);
				ATL::Checked::tcscat_s(lpstrPageTitle, cchBuffer, lpstrDivider);
				ATL::Checked::tcscat_s(lpstrPageTitle, cchBuffer, m_lpstrTitleBarBase);
			}
			else
			{
				lpstrPageTitle = m_lpstrTitleBarBase;
			}

			m_wndTitleBar.SetWindowText(lpstrPageTitle);
		}
		else
		{
			m_wndTitleBar.SetWindowText(m_lpstrTitleBarBase);
		}
	}

	void DrawMoveMark(int nItem)
	{
		T* pT = static_cast<T*>(this);

		if(m_nInsertItem != -1)
		{
			RECT rect = {};
			pT->GetMoveMarkRect(rect);
			m_tab.InvalidateRect(&rect);
		}

		m_nInsertItem = nItem;

		if(m_nInsertItem != -1)
		{
			CClientDC dc(m_tab.m_hWnd);

			RECT rect = {};
			pT->GetMoveMarkRect(rect);

			CPen pen;
			pen.CreatePen(PS_SOLID, 1, ::GetSysColor(COLOR_WINDOWTEXT));
			CBrush brush;
			brush.CreateSolidBrush(::GetSysColor(COLOR_WINDOWTEXT));

			HPEN hPenOld = dc.SelectPen(pen);
			HBRUSH hBrushOld = dc.SelectBrush(brush);

			int x = rect.left;
			int y = rect.top;
			POINT ptsTop[3] = { { x, y }, { x + m_cxMoveMark, y }, { x + (m_cxMoveMark / 2), y + m_cyMoveMark } };
			dc.Polygon(ptsTop, 3);

			y = rect.bottom - 1;
			POINT ptsBottom[3] = { { x, y }, { x + m_cxMoveMark, y }, { x + (m_cxMoveMark / 2), y - m_cyMoveMark } };
			dc.Polygon(ptsBottom, 3);

			dc.SelectPen(hPenOld);
			dc.SelectBrush(hBrushOld);
		}
	}

	void GetMoveMarkRect(RECT& rect) const
	{
		m_tab.GetClientRect(&rect);

		RECT rcItem = {};
		m_tab.GetItemRect(m_nInsertItem, &rcItem);

		if(m_nInsertItem <= m_nActivePage)
		{
			rect.left = rcItem.left - m_cxMoveMark / 2 - 1;
			rect.right = rcItem.left + m_cxMoveMark / 2;
		}
		else
		{
			rect.left = rcItem.right - m_cxMoveMark / 2 - 1;
			rect.right = rcItem.right + m_cxMoveMark / 2;
		}
	}

	void SetMoveCursor(bool bCanMove)
	{
		::SetCursor(::LoadCursor(NULL, bCanMove ? IDC_ARROW : IDC_NO));
	}

	void GenerateDragImage(int nItem)
	{
		ATLASSERT(IsValidPageIndex(nItem));

		RECT rcItem = {};
		m_tab.GetItemRect(nItem, &rcItem);
		::InflateRect(&rcItem, 2, 2);   // make bigger to cover selected item

		ATLASSERT(m_ilDrag.m_hImageList == NULL);
		m_ilDrag.Create(rcItem.right - rcItem.left, rcItem.bottom - rcItem.top, ILC_COLORDDB | ILC_MASK, 1, 1);

		CClientDC dc(this->m_hWnd);
		CDC dcMem;
		dcMem.CreateCompatibleDC(dc);
		ATLASSERT(dcMem.m_hDC != NULL);
		dcMem.SetViewportOrg(-rcItem.left, -rcItem.top);

		CBitmap bmp;
		bmp.CreateCompatibleBitmap(dc, rcItem.right - rcItem.left, rcItem.bottom - rcItem.top);
		ATLASSERT(bmp.m_hBitmap != NULL);

		HBITMAP hBmpOld = dcMem.SelectBitmap(bmp);
		m_tab.SendMessage(WM_PRINTCLIENT, (WPARAM)dcMem.m_hDC);
		dcMem.SelectBitmap(hBmpOld);

		ATLVERIFY(m_ilDrag.Add(bmp.m_hBitmap, RGB(255, 0, 255)) != -1);
	}

	void ShortenTitle(LPCTSTR lpstrTitle, LPTSTR lpstrShortTitle, int cchShortTitle)
	{
		if(lstrlen(lpstrTitle) >= cchShortTitle)
		{
			LPCTSTR lpstrEllipsis = _T("...");
			int cchEllipsis = lstrlen(lpstrEllipsis);
			ATL::Checked::tcsncpy_s(lpstrShortTitle, cchShortTitle, lpstrTitle, cchShortTitle - cchEllipsis - 1);
			ATL::Checked::tcscat_s(lpstrShortTitle, cchShortTitle, lpstrEllipsis);
		}
		else
		{
			ATL::Checked::tcscpy_s(lpstrShortTitle, cchShortTitle, lpstrTitle);
		}
	}

	void UpdateTooltipText(LPNMTTDISPINFO pTTDI)
	{
		ATLASSERT(pTTDI != NULL);
		pTTDI->lpszText = (LPTSTR)GetPageTitle((int)pTTDI->hdr.idFrom);
	}

	int DragHitTest(POINT pt) const
	{
		RECT rect = {};
		this->GetClientRect(&rect);
		if(::PtInRect(&rect, pt) == FALSE)
			return -1;

		m_tab.GetClientRect(&rect);
		TCHITTESTINFO hti = {};
		hti.pt.x = pt.x;
		hti.pt.y = rect.bottom / 2;   // use middle to ignore
		int nItem = m_tab.HitTest(&hti);
		if(nItem == -1)
		{
			int nLast = m_tab.GetItemCount() - 1;
			RECT rcItem = {};
			m_tab.GetItemRect(nLast, &rcItem);
			if(pt.x >= rcItem.right)
				nItem = nLast;
		}

		return nItem;
	}

	void StartStopAutoScroll(int x)
	{
		AutoScroll scroll = _AUTOSCROLL_NONE;
		if(x != -1)
		{
			RECT rect = {};
			m_tab.GetClientRect(&rect);
			int dx = ::GetSystemMetrics(SM_CXVSCROLL);
			if((x >= 0) && (x < dx))
			{
				RECT rcItem = {};
				m_tab.GetItemRect(0, &rcItem);
				if(rcItem.left < rect.left)
					scroll = _AUTOSCROLL_LEFT;
			}
			else if((x >= (rect.right - dx)) && (x < rect.right))
			{
				RECT rcItem = {};
				m_tab.GetItemRect(m_tab.GetItemCount() - 1, &rcItem);
				if(rcItem.right > rect.right)
					scroll = _AUTOSCROLL_RIGHT;
			}
		}

		if(scroll != _AUTOSCROLL_NONE)
		{
			if(m_ud.m_hWnd == NULL)
				m_ud = m_tab.GetWindow(GW_CHILD);

			if(m_AutoScroll != scroll)
			{
				m_AutoScroll = scroll;
				this->SetTimer(_nAutoScrollTimerID, 300);
			}
		}
		else
		{
			this->KillTimer(_nAutoScrollTimerID);
			m_AutoScroll = _AUTOSCROLL_NONE;
		}
	}

	void DoAutoScroll()
	{
		ATLASSERT(m_AutoScroll != _AUTOSCROLL_NONE);

		int nMin = -1, nMax = -1;
		m_ud.GetRange(nMin, nMax);
		int nPos = m_ud.GetPos();

		int nNewPos = -1;
		if((m_AutoScroll == _AUTOSCROLL_LEFT) && (nPos > nMin))
			nNewPos = nPos - 1;
		else if((m_AutoScroll == _AUTOSCROLL_RIGHT) && (nPos < nMax))
			nNewPos = nPos + 1;
		if(nNewPos != -1)
		{
			m_tab.SendMessage(WM_HSCROLL, MAKEWPARAM(SB_THUMBPOSITION, nNewPos));
			m_tab.SendMessage(WM_HSCROLL, MAKEWPARAM(SB_ENDSCROLL, 0));

			POINT pt = {};
			::GetCursorPos(&pt);
			m_tab.ScreenToClient(&pt);
			m_tab.SendMessage(WM_MOUSEMOVE, NULL, MAKELPARAM(pt.x, pt.y));
		}
	}

// Text for menu items and title bar - override to provide different strings
	static LPCTSTR GetEmptyListText()
	{
		return _T("(Empty)");
	}

	static LPCTSTR GetWindowsMenuItemText()
	{
		return _T("&Windows...");
	}

	static LPCTSTR GetTitleDividerText()
	{
		return _T(" - ");
	}

// Notifications - override to provide different behavior
	void OnPageActivated(int nPage)
	{
		NMHDR nmhdr = {};
		nmhdr.hwndFrom = this->m_hWnd;
		nmhdr.idFrom = nPage;
		nmhdr.code = TBVN_PAGEACTIVATED;
		this->GetParent().SendMessage(WM_NOTIFY, this->GetDlgCtrlID(), (LPARAM)&nmhdr);
	}

	void OnContextMenu(int nPage, POINT pt)
	{
		TBVCONTEXTMENUINFO cmi = {};
		cmi.hdr.hwndFrom = this->m_hWnd;
		cmi.hdr.idFrom = nPage;
		cmi.hdr.code = TBVN_CONTEXTMENU;
		cmi.pt = pt;
		this->GetParent().SendMessage(WM_NOTIFY, this->GetDlgCtrlID(), (LPARAM)&cmi);
	}
};

class CTabView : public CTabViewImpl<CTabView>
{
public:
	DECLARE_WND_CLASS_EX(_T("WTL_TabView"), 0, COLOR_APPWORKSPACE)
};

} // namespace WTL

#endif // __ATLCTRLX_H__
