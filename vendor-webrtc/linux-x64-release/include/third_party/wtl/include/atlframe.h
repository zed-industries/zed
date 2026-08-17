// Windows Template Library - WTL version 10.0
// Copyright (C) Microsoft Corporation, WTL Team. All rights reserved.
//
// This file is a part of the Windows Template Library.
// The use and distribution terms for this software are covered by the
// Microsoft Public License (http://opensource.org/licenses/MS-PL)
// which can be found in the file MS-PL.txt at the root folder.

#ifndef __ATLFRAME_H__
#define __ATLFRAME_H__

#pragma once

#ifndef __ATLAPP_H__
	#error atlframe.h requires atlapp.h to be included first
#endif

#ifndef __ATLWIN_H__
	#error atlframe.h requires atlwin.h to be included first
#endif


///////////////////////////////////////////////////////////////////////////////
// Classes in this file:
//
// CFrameWindowImpl<T, TBase, TWinTraits>
// CMDIWindow
// CMDIFrameWindowImpl<T, TBase, TWinTraits>
// CMDIChildWindowImpl<T, TBase, TWinTraits>
// COwnerDraw<T>
// CUpdateUIBase
// CUpdateUI<T>
// CDynamicUpdateUI<T>
// CAutoUpdateUI<T>
// CDialogResize<T>
// CDynamicDialogLayout<T>
// CDoubleBufferImpl<T>
// CDoubleBufferWindowImpl<T, TBase, TWinTraits>
//
// Global functions:
//   AtlCreateSimpleToolBar()


namespace WTL
{

///////////////////////////////////////////////////////////////////////////////
// CFrameWndClassInfo - Manages frame window Windows class information

class CFrameWndClassInfo
{
public:
	enum { cchAutoName = 5 + sizeof(void*) * 2 };   // sizeof(void*) * 2 is the number of digits %p outputs
	WNDCLASSEX m_wc;
	LPCTSTR m_lpszOrigName;
	WNDPROC pWndProc;
	LPCTSTR m_lpszCursorID;
	BOOL m_bSystemCursor;
	ATOM m_atom;
	TCHAR m_szAutoName[cchAutoName];
	UINT m_uCommonResourceID;

	ATOM Register(WNDPROC* pProc)
	{
		if (m_atom == 0)
		{
			CWindowCreateCriticalSectionLock lock;
			if(FAILED(lock.Lock()))
			{
				ATLTRACE2(atlTraceUI, 0, _T("ERROR : Unable to lock critical section in CFrameWndClassInfo::Register.\n"));
				ATLASSERT(FALSE);
				return 0;
			}

			if(m_atom == 0)
			{
				HINSTANCE hInst = ModuleHelper::GetModuleInstance();

				if (m_lpszOrigName != NULL)
				{
					ATLASSERT(pProc != NULL);
					LPCTSTR lpsz = m_wc.lpszClassName;
					WNDPROC proc = m_wc.lpfnWndProc;

					WNDCLASSEX wc = { sizeof(WNDCLASSEX) };
					// try process local class first
					if(!::GetClassInfoEx(ModuleHelper::GetModuleInstance(), m_lpszOrigName, &wc))
					{
						// try global class
						if(!::GetClassInfoEx(NULL, m_lpszOrigName, &wc))
						{
							lock.Unlock();
							return 0;
						}
					}
					m_wc = wc;
					pWndProc = m_wc.lpfnWndProc;
					m_wc.lpszClassName = lpsz;
					m_wc.lpfnWndProc = proc;
				}
				else
				{
					m_wc.hCursor = ::LoadCursor(m_bSystemCursor ? NULL : hInst, m_lpszCursorID);
				}

				m_wc.hInstance = hInst;
				m_wc.style &= ~CS_GLOBALCLASS;   // we don't register global classes
				if (m_wc.lpszClassName == NULL)
				{
					_stprintf_s(m_szAutoName, cchAutoName, _T("ATL:%p"), &m_wc);
					m_wc.lpszClassName = m_szAutoName;
				}

				WNDCLASSEX wcTemp = m_wc;
				m_atom = (ATOM)::GetClassInfoEx(m_wc.hInstance, m_wc.lpszClassName, &wcTemp);
				if (m_atom == 0)
				{
					if(m_uCommonResourceID != 0)   // use it if not zero
					{
						m_wc.hIcon = (HICON)::LoadImage(ModuleHelper::GetResourceInstance(), 
							MAKEINTRESOURCE(m_uCommonResourceID), IMAGE_ICON, 
							::GetSystemMetrics(SM_CXICON), ::GetSystemMetrics(SM_CYICON), LR_DEFAULTCOLOR);
						m_wc.hIconSm = (HICON)::LoadImage(ModuleHelper::GetResourceInstance(), 
							MAKEINTRESOURCE(m_uCommonResourceID), IMAGE_ICON, 
							::GetSystemMetrics(SM_CXSMICON), ::GetSystemMetrics(SM_CYSMICON), LR_DEFAULTCOLOR);
					}
					m_atom = ::RegisterClassEx(&m_wc);
				}
			}

			lock.Unlock();
		}

		if (m_lpszOrigName != NULL)
		{
			ATLASSERT(pProc != NULL);
			ATLASSERT(pWndProc != NULL);
			*pProc = pWndProc;
		}

		return m_atom;
	}
};


///////////////////////////////////////////////////////////////////////////////
// Macros for declaring frame window WNDCLASS

#define DECLARE_FRAME_WND_CLASS(WndClassName, uCommonResourceID) \
static WTL::CFrameWndClassInfo& GetWndClassInfo() \
{ \
	static WTL::CFrameWndClassInfo wc = \
	{ \
		{ sizeof(WNDCLASSEX), 0, StartWindowProc, \
		  0, 0, NULL, NULL, NULL, (HBRUSH)(COLOR_WINDOW + 1), NULL, WndClassName, NULL }, \
		NULL, NULL, IDC_ARROW, TRUE, 0, _T(""), uCommonResourceID \
	}; \
	return wc; \
}

#define DECLARE_FRAME_WND_CLASS_EX(WndClassName, uCommonResourceID, style, bkgnd) \
static WTL::CFrameWndClassInfo& GetWndClassInfo() \
{ \
	static WTL::CFrameWndClassInfo wc = \
	{ \
		{ sizeof(WNDCLASSEX), style, StartWindowProc, \
		  0, 0, NULL, NULL, NULL, (HBRUSH)(bkgnd + 1), NULL, WndClassName, NULL }, \
		NULL, NULL, IDC_ARROW, TRUE, 0, _T(""), uCommonResourceID \
	}; \
	return wc; \
}

#define DECLARE_FRAME_WND_SUPERCLASS(WndClassName, OrigWndClassName, uCommonResourceID) \
static WTL::CFrameWndClassInfo& GetWndClassInfo() \
{ \
	static WTL::CFrameWndClassInfo wc = \
	{ \
		{ sizeof(WNDCLASSEX), 0, StartWindowProc, \
		  0, 0, NULL, NULL, NULL, NULL, NULL, WndClassName, NULL }, \
		OrigWndClassName, NULL, NULL, TRUE, 0, _T(""), uCommonResourceID \
	}; \
	return wc; \
}

// These are for templated classes
#define DECLARE_FRAME_WND_CLASS2(WndClassName, EnclosingClass, uCommonResourceID) \
static WTL::CFrameWndClassInfo& GetWndClassInfo() \
{ \
	static WTL::CFrameWndClassInfo wc = \
	{ \
		{ sizeof(WNDCLASSEX), 0, EnclosingClass::StartWindowProc, \
		  0, 0, NULL, NULL, NULL, (HBRUSH)(COLOR_WINDOW + 1), NULL, WndClassName, NULL }, \
		NULL, NULL, IDC_ARROW, TRUE, 0, _T(""), uCommonResourceID \
	}; \
	return wc; \
}

#define DECLARE_FRAME_WND_CLASS_EX2(WndClassName, EnclosingClass, uCommonResourceID, style, bkgnd) \
static WTL::CFrameWndClassInfo& GetWndClassInfo() \
{ \
	static WTL::CFrameWndClassInfo wc = \
	{ \
		{ sizeof(WNDCLASSEX), style, EnclosingClass::StartWindowProc, \
		  0, 0, NULL, NULL, NULL, (HBRUSH)(bkgnd + 1), NULL, WndClassName, NULL }, \
		NULL, NULL, IDC_ARROW, TRUE, 0, _T(""), uCommonResourceID \
	}; \
	return wc; \
}

#define DECLARE_FRAME_WND_SUPERCLASS2(WndClassName, EnclosingClass, OrigWndClassName, uCommonResourceID) \
static WTL::CFrameWndClassInfo& GetWndClassInfo() \
{ \
	static WTL::CFrameWndClassInfo wc = \
	{ \
		{ sizeof(WNDCLASSEX), 0, EnclosingClass::StartWindowProc, \
		  0, 0, NULL, NULL, NULL, NULL, NULL, WndClassName, NULL }, \
		OrigWndClassName, NULL, NULL, TRUE, 0, _T(""), uCommonResourceID \
	}; \
	return wc; \
}


///////////////////////////////////////////////////////////////////////////////
// CFrameWindowImpl

// Client window command chaining macro (only for frame windows)
#define CHAIN_CLIENT_COMMANDS() \
	if((uMsg == WM_COMMAND) && (this->m_hWndClient != NULL)) \
		::SendMessage(this->m_hWndClient, uMsg, wParam, lParam);

// standard toolbar styles
#define ATL_SIMPLE_TOOLBAR_STYLE \
	(WS_CHILD | WS_VISIBLE | WS_CLIPCHILDREN | WS_CLIPSIBLINGS | TBSTYLE_TOOLTIPS)
// toolbar in a rebar pane
#define ATL_SIMPLE_TOOLBAR_PANE_STYLE \
	(WS_CHILD | WS_VISIBLE | WS_CLIPCHILDREN | WS_CLIPSIBLINGS | CCS_NODIVIDER | CCS_NORESIZE | CCS_NOPARENTALIGN | TBSTYLE_TOOLTIPS | TBSTYLE_FLAT)
// standard rebar styles
  #define ATL_SIMPLE_REBAR_STYLE \
	(WS_CHILD | WS_VISIBLE | WS_BORDER | WS_CLIPCHILDREN | WS_CLIPSIBLINGS | RBS_VARHEIGHT | RBS_BANDBORDERS | RBS_AUTOSIZE)
// rebar without borders
  #define ATL_SIMPLE_REBAR_NOBORDER_STYLE \
	(WS_CHILD | WS_VISIBLE | WS_CLIPCHILDREN | WS_CLIPSIBLINGS | RBS_VARHEIGHT | RBS_BANDBORDERS | RBS_AUTOSIZE | CCS_NODIVIDER)

// command bar support
#if !defined(__ATLCTRLW_H__)

#define CBRM_GETCMDBAR			(WM_USER + 301) // returns command bar HWND
#define CBRM_GETMENU			(WM_USER + 302) // returns loaded or attached menu
#define CBRM_TRACKPOPUPMENU		(WM_USER + 303) // displays a popup menu

struct _AtlFrameWnd_CmdBarPopupMenu
{
	int cbSize;
	HMENU hMenu;
	UINT uFlags;
	int x;
	int y;
	LPTPMPARAMS lptpm;
};

#define CBRPOPUPMENU _AtlFrameWnd_CmdBarPopupMenu

#endif // !defined(__ATLCTRLW_H__)


template <class TBase = ATL::CWindow, class TWinTraits = ATL::CFrameWinTraits>
class ATL_NO_VTABLE CFrameWindowImplBase : public ATL::CWindowImplBaseT< TBase, TWinTraits >
{
public:
	typedef CFrameWindowImplBase<TBase, TWinTraits >	_thisClass;
	DECLARE_FRAME_WND_CLASS2(NULL, _thisClass, 0)

	struct _ChevronMenuInfo
	{
		HMENU hMenu;
		LPNMREBARCHEVRON lpnm;
		bool bCmdBar;
	};

// Data members
	HWND m_hWndToolBar;
	HWND m_hWndStatusBar;
	HWND m_hWndClient;

	HACCEL m_hAccel;

// Constructor
	CFrameWindowImplBase() : 
		m_hWndToolBar(NULL), 
		m_hWndStatusBar(NULL), 
		m_hWndClient(NULL), 
		m_hAccel(NULL)
	{ }

// Methods
	HWND Create(HWND hWndParent, ATL::_U_RECT rect, LPCTSTR szWindowName, DWORD dwStyle, DWORD dwExStyle, ATL::_U_MENUorID MenuOrID, ATOM atom, LPVOID lpCreateParam)
	{
		ATLASSERT(this->m_hWnd == NULL);

		// Allocate the thunk structure here, where we can fail gracefully.
		BOOL bRet = this->m_thunk.Init(NULL, NULL);
		if(bRet == FALSE)
		{
			::SetLastError(ERROR_OUTOFMEMORY);
			return NULL;
		}

		if(atom == 0)
			return NULL;

		ModuleHelper::AddCreateWndData(&this->m_thunk.cd, this);

		if((MenuOrID.m_hMenu == NULL) && (dwStyle & WS_CHILD))
			MenuOrID.m_hMenu = (HMENU)(UINT_PTR)this;
		if(rect.m_lpRect == NULL)
			rect.m_lpRect = &TBase::rcDefault;

		HWND hWnd = ::CreateWindowEx(dwExStyle, MAKEINTATOM(atom), szWindowName,
			dwStyle, rect.m_lpRect->left, rect.m_lpRect->top, rect.m_lpRect->right - rect.m_lpRect->left,
			rect.m_lpRect->bottom - rect.m_lpRect->top, hWndParent, MenuOrID.m_hMenu,
			ModuleHelper::GetModuleInstance(), lpCreateParam);

		ATLASSERT((hWnd == NULL) || (this->m_hWnd == hWnd));

		return hWnd;
	}

	static HWND CreateSimpleToolBarCtrl(HWND hWndParent, UINT nResourceID, BOOL bInitialSeparator = FALSE, 
			DWORD dwStyle = ATL_SIMPLE_TOOLBAR_STYLE, UINT nID = ATL_IDW_TOOLBAR)
	{
		HINSTANCE hInst = ModuleHelper::GetResourceInstance();
		HRSRC hRsrc = ::FindResource(hInst, MAKEINTRESOURCE(nResourceID), RT_TOOLBAR);
		if (hRsrc == NULL)
			return NULL;

		HGLOBAL hGlobal = ::LoadResource(hInst, hRsrc);
		if (hGlobal == NULL)
			return NULL;

		_AtlToolBarData* pData = (_AtlToolBarData*)::LockResource(hGlobal);
		if (pData == NULL)
			return NULL;
		ATLASSERT(pData->wVersion == 1);

		WORD* pItems = pData->items();
		int nItems = pData->wItemCount + (bInitialSeparator ? 1 : 0);
		ATL::CTempBuffer<TBBUTTON, _WTL_STACK_ALLOC_THRESHOLD> buff;
		TBBUTTON* pTBBtn = buff.Allocate(nItems);
		ATLASSERT(pTBBtn != NULL);
		if(pTBBtn == NULL)
			return NULL;

		const int cxSeparator = 8;

		// set initial separator (half width)
		if(bInitialSeparator)
		{
			pTBBtn[0].iBitmap = cxSeparator / 2;
			pTBBtn[0].idCommand = 0;
			pTBBtn[0].fsState = 0;
			pTBBtn[0].fsStyle = BTNS_SEP;
			pTBBtn[0].dwData = 0;
			pTBBtn[0].iString = 0;
		}

		int nBmp = 0;
		for(int i = 0, j = bInitialSeparator ? 1 : 0; i < pData->wItemCount; i++, j++)
		{
			if(pItems[i] != 0)
			{
				pTBBtn[j].iBitmap = nBmp++;
				pTBBtn[j].idCommand = pItems[i];
				pTBBtn[j].fsState = TBSTATE_ENABLED;
				pTBBtn[j].fsStyle = BTNS_BUTTON;
				pTBBtn[j].dwData = 0;
				pTBBtn[j].iString = 0;
			}
			else
			{
				pTBBtn[j].iBitmap = cxSeparator;
				pTBBtn[j].idCommand = 0;
				pTBBtn[j].fsState = 0;
				pTBBtn[j].fsStyle = BTNS_SEP;
				pTBBtn[j].dwData = 0;
				pTBBtn[j].iString = 0;
			}
		}

		HWND hWnd = ::CreateWindowEx(0, TOOLBARCLASSNAME, NULL, dwStyle, 0, 0, 100, 100, hWndParent, (HMENU)LongToHandle(nID), ModuleHelper::GetModuleInstance(), NULL);
		if(hWnd == NULL)
		{
			ATLASSERT(FALSE);
			return NULL;
		}

		::SendMessage(hWnd, TB_BUTTONSTRUCTSIZE, sizeof(TBBUTTON), 0L);

		// check if font is taller than our bitmaps
		CFontHandle font = (HFONT)::SendMessage(hWnd, WM_GETFONT, 0, 0L);
		if(font.IsNull())
			font = (HFONT)::GetStockObject(SYSTEM_FONT);
		LOGFONT lf = {};
		font.GetLogFont(lf);
		WORD cyFontHeight = (WORD)abs(lf.lfHeight);

		WORD bitsPerPixel = AtlGetBitmapResourceBitsPerPixel(nResourceID);
		if(bitsPerPixel > 4)
		{
			COLORREF crMask = CLR_DEFAULT;
			if(bitsPerPixel == 32)
			{
				// 32-bit color bitmap with alpha channel (valid for Windows XP and later)
				crMask = CLR_NONE;
			}
			HIMAGELIST hImageList = ImageList_LoadImage(ModuleHelper::GetResourceInstance(), MAKEINTRESOURCE(nResourceID), pData->wWidth, 1, crMask, IMAGE_BITMAP, LR_CREATEDIBSECTION | LR_DEFAULTSIZE);
			ATLASSERT(hImageList != NULL);
			::SendMessage(hWnd, TB_SETIMAGELIST, 0, (LPARAM)hImageList);
		}
		else
		{
			TBADDBITMAP tbab = {};
			tbab.hInst = hInst;
			tbab.nID = nResourceID;
			::SendMessage(hWnd, TB_ADDBITMAP, nBmp, (LPARAM)&tbab);
		}

		::SendMessage(hWnd, TB_ADDBUTTONS, nItems, (LPARAM)pTBBtn);
		::SendMessage(hWnd, TB_SETBITMAPSIZE, 0, MAKELONG(pData->wWidth, __max(pData->wHeight, cyFontHeight)));
		const int cxyButtonMargin = 7;
		::SendMessage(hWnd, TB_SETBUTTONSIZE, 0, MAKELONG(pData->wWidth + cxyButtonMargin, __max(pData->wHeight, cyFontHeight) + cxyButtonMargin));

		return hWnd;
	}

	static HWND CreateSimpleReBarCtrl(HWND hWndParent, DWORD dwStyle = ATL_SIMPLE_REBAR_STYLE, UINT nID = ATL_IDW_TOOLBAR)
	{
		// Ensure style combinations for proper rebar painting
		if(dwStyle & CCS_NODIVIDER && (dwStyle & WS_BORDER))
			dwStyle &= ~WS_BORDER;
		else if(!(dwStyle & WS_BORDER) && !(dwStyle & CCS_NODIVIDER))
			dwStyle |= CCS_NODIVIDER;

		// Create rebar window
		HWND hWndReBar = ::CreateWindowEx(0, REBARCLASSNAME, NULL, dwStyle, 0, 0, 100, 100, hWndParent, (HMENU)LongToHandle(nID), ModuleHelper::GetModuleInstance(), NULL);
		if(hWndReBar == NULL)
		{
			ATLTRACE2(atlTraceUI, 0, _T("Failed to create rebar.\n"));
			return NULL;
		}

		// Initialize and send the REBARINFO structure
		REBARINFO rbi = { sizeof(REBARINFO), 0 };
		if(::SendMessage(hWndReBar, RB_SETBARINFO, 0, (LPARAM)&rbi) == 0)
		{
			ATLTRACE2(atlTraceUI, 0, _T("Failed to initialize rebar.\n"));
			::DestroyWindow(hWndReBar);
			return NULL;
		}

		return hWndReBar;
	}

	BOOL CreateSimpleReBar(DWORD dwStyle = ATL_SIMPLE_REBAR_STYLE, UINT nID = ATL_IDW_TOOLBAR)
	{
		ATLASSERT(!::IsWindow(m_hWndToolBar));
		m_hWndToolBar = CreateSimpleReBarCtrl(this->m_hWnd, dwStyle, nID);
		return (m_hWndToolBar != NULL);
	}

	static BOOL AddSimpleReBarBandCtrl(HWND hWndReBar, HWND hWndBand, int nID = 0, LPCTSTR lpstrTitle = NULL, BOOL bNewRow = FALSE, int cxWidth = 0, BOOL bFullWidthAlways = FALSE)
	{
		ATLASSERT(::IsWindow(hWndReBar));   // must be already created
#ifdef _DEBUG
		// block - check if this is really a rebar
		{
			TCHAR lpszClassName[sizeof(REBARCLASSNAME)] = {};
			::GetClassName(hWndReBar, lpszClassName, sizeof(REBARCLASSNAME));
			ATLASSERT(lstrcmp(lpszClassName, REBARCLASSNAME) == 0);
		}
#endif // _DEBUG
		ATLASSERT(::IsWindow(hWndBand));   // must be already created

		// Get number of buttons on the toolbar
		int nBtnCount = (int)::SendMessage(hWndBand, TB_BUTTONCOUNT, 0, 0L);

		// Set band info structure
		REBARBANDINFO rbBand = { RunTimeHelper::SizeOf_REBARBANDINFO() };
		rbBand.fMask = RBBIM_CHILD | RBBIM_CHILDSIZE | RBBIM_STYLE | RBBIM_ID | RBBIM_SIZE | RBBIM_IDEALSIZE;
		if(lpstrTitle != NULL)
			rbBand.fMask |= RBBIM_TEXT;
		rbBand.fStyle = RBBS_CHILDEDGE;
		if(nBtnCount > 0)   // add chevron style for toolbar with buttons
			rbBand.fStyle |= RBBS_USECHEVRON;
		if(bNewRow)
			rbBand.fStyle |= RBBS_BREAK;

		rbBand.lpText = (LPTSTR)lpstrTitle;
		rbBand.hwndChild = hWndBand;
		if(nID == 0)   // calc band ID
			nID = ATL_IDW_BAND_FIRST + (int)::SendMessage(hWndReBar, RB_GETBANDCOUNT, 0, 0L);
		rbBand.wID = nID;

		// Calculate the size of the band
		BOOL bRet = FALSE;
		RECT rcTmp = {};
		if(nBtnCount > 0)
		{
			bRet = (BOOL)::SendMessage(hWndBand, TB_GETITEMRECT, nBtnCount - 1, (LPARAM)&rcTmp);
			ATLASSERT(bRet);
			rbBand.cx = (cxWidth != 0) ? cxWidth : rcTmp.right;
			rbBand.cyMinChild = rcTmp.bottom - rcTmp.top;
			if(bFullWidthAlways)
			{
				rbBand.cxMinChild = rbBand.cx;
			}
			else if(lpstrTitle == NULL)
			{
				bRet = (BOOL)::SendMessage(hWndBand, TB_GETITEMRECT, 0, (LPARAM)&rcTmp);
				ATLASSERT(bRet);
				rbBand.cxMinChild = rcTmp.right;
			}
			else
			{
				rbBand.cxMinChild = 0;
			}
		}
		else	// no buttons, either not a toolbar or really has no buttons
		{
			bRet = ::GetWindowRect(hWndBand, &rcTmp);
			ATLASSERT(bRet);
			rbBand.cx = (cxWidth != 0) ? cxWidth : (rcTmp.right - rcTmp.left);
			rbBand.cxMinChild = bFullWidthAlways ? rbBand.cx : 0;
			rbBand.cyMinChild = rcTmp.bottom - rcTmp.top;
		}

		rbBand.cxIdeal = rbBand.cx;

		// Add the band
		LRESULT lRes = ::SendMessage(hWndReBar, RB_INSERTBAND, (WPARAM)-1, (LPARAM)&rbBand);
		if(lRes == 0)
		{
			ATLTRACE2(atlTraceUI, 0, _T("Failed to add a band to the rebar.\n"));
			return FALSE;
		}

		DWORD dwExStyle = (DWORD)::SendMessage(hWndBand, TB_GETEXTENDEDSTYLE, 0, 0L);
		::SendMessage(hWndBand, TB_SETEXTENDEDSTYLE, 0, dwExStyle | TBSTYLE_EX_HIDECLIPPEDBUTTONS);

		return TRUE;
	}

	BOOL AddSimpleReBarBand(HWND hWndBand, LPCTSTR lpstrTitle = NULL, BOOL bNewRow = FALSE, int cxWidth = 0, BOOL bFullWidthAlways = FALSE)
	{
		ATLASSERT(::IsWindow(m_hWndToolBar));   // must be an existing rebar
		ATLASSERT(::IsWindow(hWndBand));        // must be created
		return AddSimpleReBarBandCtrl(m_hWndToolBar, hWndBand, 0, lpstrTitle, bNewRow, cxWidth, bFullWidthAlways);
	}

	void SizeSimpleReBarBands()
	{
		ATLASSERT(::IsWindow(m_hWndToolBar));   // must be an existing rebar

		int nCount = (int)::SendMessage(m_hWndToolBar, RB_GETBANDCOUNT, 0, 0L);

		for(int i = 0; i < nCount; i++)
		{
			REBARBANDINFO rbBand = { RunTimeHelper::SizeOf_REBARBANDINFO() };
			rbBand.fMask = RBBIM_SIZE;
			BOOL bRet = (BOOL)::SendMessage(m_hWndToolBar, RB_GETBANDINFO, i, (LPARAM)&rbBand);
			ATLASSERT(bRet);
			RECT rect = {};
			::SendMessage(m_hWndToolBar, RB_GETBANDBORDERS, i, (LPARAM)&rect);
			rbBand.cx += rect.left + rect.right;
			bRet = (BOOL)::SendMessage(m_hWndToolBar, RB_SETBANDINFO, i, (LPARAM)&rbBand);
			ATLASSERT(bRet);
		}
	}

	BOOL CreateSimpleStatusBar(LPCTSTR lpstrText, DWORD dwStyle = WS_CHILD | WS_VISIBLE | WS_CLIPCHILDREN | WS_CLIPSIBLINGS | SBARS_SIZEGRIP, UINT nID = ATL_IDW_STATUS_BAR)
	{
		ATLASSERT(!::IsWindow(m_hWndStatusBar));
		m_hWndStatusBar = ::CreateStatusWindow(dwStyle, lpstrText, this->m_hWnd, nID);
		return (m_hWndStatusBar != NULL);
	}

	BOOL CreateSimpleStatusBar(UINT nTextID = ATL_IDS_IDLEMESSAGE, DWORD dwStyle = WS_CHILD | WS_VISIBLE | WS_CLIPCHILDREN | WS_CLIPSIBLINGS | SBARS_SIZEGRIP, UINT nID = ATL_IDW_STATUS_BAR)
	{
		const int cchMax = 128;   // max text length is 127 for status bars (+1 for null)
		TCHAR szText[cchMax] = {};
		::LoadString(ModuleHelper::GetResourceInstance(), nTextID, szText, cchMax);
		return CreateSimpleStatusBar(szText, dwStyle, nID);
	}

	void UpdateLayout(BOOL bResizeBars = TRUE)
	{
		RECT rect = {};
		this->GetClientRect(&rect);

		// position bars and offset their dimensions
		UpdateBarsPosition(rect, bResizeBars);

		// resize client window
		if(m_hWndClient != NULL)
			::SetWindowPos(m_hWndClient, NULL, rect.left, rect.top,
				rect.right - rect.left, rect.bottom - rect.top,
				SWP_NOZORDER | SWP_NOACTIVATE);
	}

	void UpdateBarsPosition(RECT& rect, BOOL bResizeBars = TRUE)
	{
		// resize toolbar
		if((m_hWndToolBar != NULL) && ((DWORD)::GetWindowLong(m_hWndToolBar, GWL_STYLE) & WS_VISIBLE))
		{
			if(bResizeBars != FALSE)
			{
				::SendMessage(m_hWndToolBar, WM_SIZE, 0, 0);
				::InvalidateRect(m_hWndToolBar, NULL, TRUE);
			}
			RECT rectTB = {};
			::GetWindowRect(m_hWndToolBar, &rectTB);
			rect.top += rectTB.bottom - rectTB.top;
		}

		// resize status bar
		if((m_hWndStatusBar != NULL) && ((DWORD)::GetWindowLong(m_hWndStatusBar, GWL_STYLE) & WS_VISIBLE))
		{
			if(bResizeBars != FALSE)
				::SendMessage(m_hWndStatusBar, WM_SIZE, 0, 0);
			RECT rectSB = {};
			::GetWindowRect(m_hWndStatusBar, &rectSB);
			rect.bottom -= rectSB.bottom - rectSB.top;
		}
	}

	BOOL PreTranslateMessage(MSG* pMsg)
	{
		if((m_hAccel != NULL) && ::TranslateAccelerator(this->m_hWnd, m_hAccel, pMsg))
			return TRUE;
		return FALSE;
	}

	BEGIN_MSG_MAP(CFrameWindowImplBase)
		MESSAGE_HANDLER(WM_ERASEBKGND, OnEraseBackground)
		MESSAGE_HANDLER(WM_MENUSELECT, OnMenuSelect)
		MESSAGE_HANDLER(WM_SETFOCUS, OnSetFocus)
		MESSAGE_HANDLER(WM_DESTROY, OnDestroy)
		NOTIFY_CODE_HANDLER(TTN_GETDISPINFOA, OnToolTipTextA)
		NOTIFY_CODE_HANDLER(TTN_GETDISPINFOW, OnToolTipTextW)
	END_MSG_MAP()

	LRESULT OnEraseBackground(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& bHandled)
	{
		if(m_hWndClient != NULL)   // view will paint itself instead
			return 1;

		bHandled = FALSE;
		return 0;
	}

	LRESULT OnMenuSelect(UINT /*uMsg*/, WPARAM wParam, LPARAM lParam, BOOL& bHandled)
	{
		bHandled = FALSE;

		if(m_hWndStatusBar == NULL)
			return 1;

		WORD wFlags = HIWORD(wParam);
		if((wFlags == 0xFFFF) && (lParam == NULL))   // menu closing
		{
			::SendMessage(m_hWndStatusBar, SB_SIMPLE, FALSE, 0L);
		}
		else
		{
			const int cchBuff = 256;
			TCHAR szBuff[cchBuff] = {};
			if(!(wFlags & MF_POPUP))
			{
				WORD wID = LOWORD(wParam);
				// check for special cases
				if((wID >= 0xF000) && (wID < 0xF1F0))   // system menu IDs
					wID = (WORD)(((wID - 0xF000) >> 4) + ATL_IDS_SCFIRST);
				else if((wID >= ID_FILE_MRU_FIRST) && (wID <= ID_FILE_MRU_LAST))   // MRU items
					wID = ATL_IDS_MRU_FILE;
				else if((wID >= ATL_IDM_FIRST_MDICHILD) && (wID <= ATL_IDM_LAST_MDICHILD))   // MDI child windows
					wID = ATL_IDS_MDICHILD;

				int nRet = ::LoadString(ModuleHelper::GetResourceInstance(), wID, szBuff, cchBuff);
				for(int i = 0; i < nRet; i++)
				{
					if(szBuff[i] == _T('\n'))
					{
						szBuff[i] = 0;
						break;
					}
				}
			}
			::SendMessage(m_hWndStatusBar, SB_SIMPLE, TRUE, 0L);
			::SendMessage(m_hWndStatusBar, SB_SETTEXT, (255 | SBT_NOBORDERS), (LPARAM)szBuff);
		}

		return 1;
	}

	LRESULT OnSetFocus(UINT, WPARAM, LPARAM, BOOL& bHandled)
	{
		if(m_hWndClient != NULL)
			::SetFocus(m_hWndClient);

		bHandled = FALSE;
		return 1;
	}

	LRESULT OnDestroy(UINT, WPARAM, LPARAM, BOOL& bHandled)
	{
		if((this->GetStyle() & (WS_CHILD | WS_POPUP)) == 0)
			::PostQuitMessage(1);

		bHandled = FALSE;
		return 1;
	}

	LRESULT OnToolTipTextA(int idCtrl, LPNMHDR pnmh, BOOL& /*bHandled*/)
	{
		LPNMTTDISPINFOA pDispInfo = (LPNMTTDISPINFOA)pnmh;
		if((idCtrl != 0) && !(pDispInfo->uFlags & TTF_IDISHWND))
		{
			const int cchBuff = 256;
			char szBuff[cchBuff] = {};
			int nRet = ::LoadStringA(ModuleHelper::GetResourceInstance(), idCtrl, szBuff, cchBuff);
			for(int i = 0; i < nRet; i++)
			{
				if(szBuff[i] == '\n')
				{
					ATL::Checked::strncpy_s(pDispInfo->szText, _countof(pDispInfo->szText), &szBuff[i + 1], _TRUNCATE);
					break;
				}
			}
			if(nRet > 0)   // string was loaded, save it
				pDispInfo->uFlags |= TTF_DI_SETITEM;
		}

		return 0;
	}

	LRESULT OnToolTipTextW(int idCtrl, LPNMHDR pnmh, BOOL& /*bHandled*/)
	{
		LPNMTTDISPINFOW pDispInfo = (LPNMTTDISPINFOW)pnmh;
		if((idCtrl != 0) && !(pDispInfo->uFlags & TTF_IDISHWND))
		{
			const int cchBuff = 256;
			wchar_t szBuff[cchBuff] = {};
			int nRet = ::LoadStringW(ModuleHelper::GetResourceInstance(), idCtrl, szBuff, cchBuff);
			for(int i = 0; i < nRet; i++)
			{
				if(szBuff[i] == L'\n')
				{
					ATL::Checked::wcsncpy_s(pDispInfo->szText, _countof(pDispInfo->szText), &szBuff[i + 1], _TRUNCATE);
					break;
				}
			}
			if(nRet > 0)   // string was loaded, save it
				pDispInfo->uFlags |= TTF_DI_SETITEM;
		}

		return 0;
	}

// Implementation - chevron menu support
	bool PrepareChevronMenu(_ChevronMenuInfo& cmi)
	{
		// get rebar and toolbar
		REBARBANDINFO rbbi = { RunTimeHelper::SizeOf_REBARBANDINFO() };
		rbbi.fMask = RBBIM_CHILD;
		BOOL bRet = (BOOL)::SendMessage(cmi.lpnm->hdr.hwndFrom, RB_GETBANDINFO, cmi.lpnm->uBand, (LPARAM)&rbbi);
		ATLASSERT(bRet);

		// assume the band is a toolbar
		ATL::CWindow wnd = rbbi.hwndChild;
		int nCount = (int)wnd.SendMessage(TB_BUTTONCOUNT);
		if(nCount <= 0)   // probably not a toolbar
			return false;

		// check if it's a command bar
		CMenuHandle menuCmdBar = (HMENU)wnd.SendMessage(CBRM_GETMENU);
		cmi.bCmdBar = (menuCmdBar.m_hMenu != NULL);

		// build a menu from hidden items
		CMenuHandle menu;
		bRet = menu.CreatePopupMenu();
		ATLASSERT(bRet);
		RECT rcClient = {};
		bRet = wnd.GetClientRect(&rcClient);
		ATLASSERT(bRet);
		for(int i = 0; i < nCount; i++)
		{
			TBBUTTON tbb = {};
			bRet = (BOOL)wnd.SendMessage(TB_GETBUTTON, i, (LPARAM)&tbb);
			ATLASSERT(bRet);
			// skip hidden buttons
			if((tbb.fsState & TBSTATE_HIDDEN) != 0)
				continue;
			RECT rcButton = {};
			bRet = (BOOL)wnd.SendMessage(TB_GETITEMRECT, i, (LPARAM)&rcButton);
			ATLASSERT(bRet);
			bool bEnabled = ((tbb.fsState & TBSTATE_ENABLED) != 0);
			if((rcButton.right > rcClient.right) || (rcButton.bottom > rcClient.bottom))
			{
				if(tbb.fsStyle & BTNS_SEP)
				{
					if(menu.GetMenuItemCount() > 0)
						menu.AppendMenu(MF_SEPARATOR);
				}
				else if(cmi.bCmdBar)
				{
					const int cchBuff = 200;
					TCHAR szBuff[cchBuff] = {};
					CMenuItemInfo mii;
					mii.fMask = MIIM_TYPE | MIIM_SUBMENU;
					mii.dwTypeData = szBuff;
					mii.cch = cchBuff;
					bRet = menuCmdBar.GetMenuItemInfo(i, TRUE, &mii);
					ATLASSERT(bRet);
					// Note: CmdBar currently supports only drop-down items
					ATLASSERT(::IsMenu(mii.hSubMenu));
					bRet = menu.AppendMenu(MF_STRING | MF_POPUP | (bEnabled ? MF_ENABLED : MF_GRAYED), (UINT_PTR)mii.hSubMenu, mii.dwTypeData);
					ATLASSERT(bRet);
				}
				else
				{
					// get button's text
					const int cchBuff = 200;
					TCHAR szBuff[cchBuff] = {};
					LPCTSTR lpstrText = szBuff;
					TBBUTTONINFO tbbi = {};
					tbbi.cbSize = sizeof(TBBUTTONINFO);
					tbbi.dwMask = TBIF_TEXT;
					tbbi.pszText = szBuff;
					tbbi.cchText = cchBuff;
					if((wnd.SendMessage(TB_GETBUTTONINFO, tbb.idCommand, (LPARAM)&tbbi) == -1) || (szBuff[0] == 0))
					{
						// no text for this button, try a resource string
						lpstrText = _T("");
						int nRet = ::LoadString(ModuleHelper::GetResourceInstance(), tbb.idCommand, szBuff, cchBuff);
						for(int n = 0; n < nRet; n++)
						{
							if(szBuff[n] == _T('\n'))
							{
								lpstrText = &szBuff[n + 1];
								break;
							}
						}
					}
					bRet = menu.AppendMenu(MF_STRING | (bEnabled ? MF_ENABLED : MF_GRAYED), tbb.idCommand, lpstrText);
					ATLASSERT(bRet);
				}
			}
		}

		if(menu.GetMenuItemCount() == 0)   // no hidden buttons after all
		{
			menu.DestroyMenu();
			::MessageBeep((UINT)-1);
			return false;
		}

		cmi.hMenu = menu;
		return true;
	}

	void DisplayChevronMenu(_ChevronMenuInfo& cmi)
	{
		// convert chevron rect to screen coordinates
		ATL::CWindow wndFrom = cmi.lpnm->hdr.hwndFrom;
		POINT pt = { cmi.lpnm->rc.left, cmi.lpnm->rc.bottom };
		wndFrom.MapWindowPoints(NULL, &pt, 1);
		RECT rc = cmi.lpnm->rc;
		wndFrom.MapWindowPoints(NULL, &rc);
		// set up flags and rect
		UINT uMenuFlags = TPM_LEFTBUTTON | TPM_VERTICAL | TPM_LEFTALIGN | TPM_TOPALIGN | TPM_VERPOSANIMATION;
		TPMPARAMS TPMParams = {};
		TPMParams.cbSize = sizeof(TPMPARAMS);
		TPMParams.rcExclude = rc;
		// check if this window has a command bar
		HWND hWndCmdBar = (HWND)::SendMessage(this->m_hWnd, CBRM_GETCMDBAR, 0, 0L);
		if(::IsWindow(hWndCmdBar))
		{
			CBRPOPUPMENU CBRPopupMenu = { sizeof(CBRPOPUPMENU), cmi.hMenu, uMenuFlags, pt.x, pt.y, &TPMParams };
			::SendMessage(hWndCmdBar, CBRM_TRACKPOPUPMENU, 0, (LPARAM)&CBRPopupMenu);
		}
		else
		{
			CMenuHandle menu = cmi.hMenu;
			menu.TrackPopupMenuEx(uMenuFlags, pt.x, pt.y, this->m_hWnd, &TPMParams);
		}
	}

	void CleanupChevronMenu(_ChevronMenuInfo& cmi)
	{
		CMenuHandle menu = cmi.hMenu;
		// if menu is from a command bar, detach submenus so they are not destroyed
		if(cmi.bCmdBar)
		{
			for(int i = menu.GetMenuItemCount() - 1; i >=0; i--)
				menu.RemoveMenu(i, MF_BYPOSITION);
		}
		// destroy menu
		menu.DestroyMenu();
		// convert chevron rect to screen coordinates
		ATL::CWindow wndFrom = cmi.lpnm->hdr.hwndFrom;
		RECT rc = cmi.lpnm->rc;
		wndFrom.MapWindowPoints(NULL, &rc);
		// eat next message if click is on the same button
		MSG msg = {};
		if(::PeekMessage(&msg, this->m_hWnd, WM_LBUTTONDOWN, WM_LBUTTONDOWN, PM_NOREMOVE) && ::PtInRect(&rc, msg.pt))
			::PeekMessage(&msg, this->m_hWnd, WM_LBUTTONDOWN, WM_LBUTTONDOWN, PM_REMOVE);
	}
};


template <class T, class TBase = ATL::CWindow, class TWinTraits = ATL::CFrameWinTraits>
class ATL_NO_VTABLE CFrameWindowImpl : public CFrameWindowImplBase< TBase, TWinTraits >
{
public:
	HWND Create(HWND hWndParent = NULL, ATL::_U_RECT rect = NULL, LPCTSTR szWindowName = NULL,
			DWORD dwStyle = 0, DWORD dwExStyle = 0,
			HMENU hMenu = NULL, LPVOID lpCreateParam = NULL)
	{
		ATOM atom = T::GetWndClassInfo().Register(&this->m_pfnSuperWindowProc);

		dwStyle = T::GetWndStyle(dwStyle);
		dwExStyle = T::GetWndExStyle(dwExStyle);

		if(rect.m_lpRect == NULL)
			rect.m_lpRect = &TBase::rcDefault;

		return CFrameWindowImplBase< TBase, TWinTraits >::Create(hWndParent, rect.m_lpRect, szWindowName, dwStyle, dwExStyle, hMenu, atom, lpCreateParam);
	}

	HWND CreateEx(HWND hWndParent = NULL, ATL::_U_RECT rect = NULL, DWORD dwStyle = 0, DWORD dwExStyle = 0, LPVOID lpCreateParam = NULL)
	{
		const int cchName = 256;
		TCHAR szWindowName[cchName] = {};
		::LoadString(ModuleHelper::GetResourceInstance(), T::GetWndClassInfo().m_uCommonResourceID, szWindowName, cchName);
		HMENU hMenu = ::LoadMenu(ModuleHelper::GetResourceInstance(), MAKEINTRESOURCE(T::GetWndClassInfo().m_uCommonResourceID));

		T* pT = static_cast<T*>(this);
		HWND hWnd = pT->Create(hWndParent, rect, szWindowName, dwStyle, dwExStyle, hMenu, lpCreateParam);

		if(hWnd != NULL)
			this->m_hAccel = ::LoadAccelerators(ModuleHelper::GetResourceInstance(), MAKEINTRESOURCE(T::GetWndClassInfo().m_uCommonResourceID));

		return hWnd;
	}

	BOOL CreateSimpleToolBar(UINT nResourceID = 0, DWORD dwStyle = ATL_SIMPLE_TOOLBAR_STYLE, UINT nID = ATL_IDW_TOOLBAR)
	{
		if(nResourceID == 0)
			nResourceID = T::GetWndClassInfo().m_uCommonResourceID;
		ATLASSERT(!::IsWindow(this->m_hWndToolBar));
		this->m_hWndToolBar = T::CreateSimpleToolBarCtrl(this->m_hWnd, nResourceID, TRUE, dwStyle, nID);
		return (this->m_hWndToolBar != NULL);
	}

// message map and handlers
	typedef CFrameWindowImplBase< TBase, TWinTraits >   _baseClass;

	BEGIN_MSG_MAP(CFrameWindowImpl)
		MESSAGE_HANDLER(WM_SIZE, OnSize)
#ifndef _ATL_NO_REBAR_SUPPORT
		NOTIFY_CODE_HANDLER(RBN_AUTOSIZE, OnReBarAutoSize)
		NOTIFY_CODE_HANDLER(RBN_CHEVRONPUSHED, OnChevronPushed)
#endif // !_ATL_NO_REBAR_SUPPORT
		CHAIN_MSG_MAP(_baseClass)
	END_MSG_MAP()

	LRESULT OnSize(UINT /*uMsg*/, WPARAM wParam, LPARAM /*lParam*/, BOOL& bHandled)
	{
		if(wParam != SIZE_MINIMIZED)
		{
			T* pT = static_cast<T*>(this);
			pT->UpdateLayout();
		}
		bHandled = FALSE;
		return 1;
	}

#ifndef _ATL_NO_REBAR_SUPPORT
	LRESULT OnReBarAutoSize(int /*idCtrl*/, LPNMHDR /*pnmh*/, BOOL& /*bHandled*/)
	{
		T* pT = static_cast<T*>(this);
		pT->UpdateLayout(FALSE);
		return 0;
	}

	LRESULT OnChevronPushed(int /*idCtrl*/, LPNMHDR pnmh, BOOL& bHandled)
	{
		T* pT = static_cast<T*>(this);
		typename CFrameWindowImplBase< TBase, TWinTraits >::_ChevronMenuInfo cmi = { NULL, (LPNMREBARCHEVRON)pnmh, false };
		if(!pT->PrepareChevronMenu(cmi))
		{
			bHandled = FALSE;
			return 1;
		}
		// display a popup menu with hidden items
		pT->DisplayChevronMenu(cmi);
		// cleanup
		pT->CleanupChevronMenu(cmi);
		return 0;
	}
#endif // !_ATL_NO_REBAR_SUPPORT
};


///////////////////////////////////////////////////////////////////////////////
// AtlCreateSimpleToolBar - helper for creating simple toolbars

inline HWND AtlCreateSimpleToolBar(HWND hWndParent, UINT nResourceID, BOOL bInitialSeparator = FALSE, 
		DWORD dwStyle = ATL_SIMPLE_TOOLBAR_STYLE, UINT nID = ATL_IDW_TOOLBAR)
{
	return CFrameWindowImplBase<>::CreateSimpleToolBarCtrl(hWndParent, nResourceID, bInitialSeparator, dwStyle, nID);
}


///////////////////////////////////////////////////////////////////////////////
// CMDIWindow

#ifndef _WTL_MDIWINDOWMENU_TEXT
  #define _WTL_MDIWINDOWMENU_TEXT	_T("&Window")
#endif

class CMDIWindow : public ATL::CWindow
{
public:
// Data members
	HWND m_hWndMDIClient;
	HMENU m_hMenu;

// Constructors
	CMDIWindow(HWND hWnd = NULL) : ATL::CWindow(hWnd), m_hWndMDIClient(NULL), m_hMenu(NULL)
	{ }

	CMDIWindow& operator =(HWND hWnd)
	{
		m_hWnd = hWnd;
		return *this;
	}

// Operations
	HWND MDIGetActive(BOOL* lpbMaximized = NULL)
	{
		ATLASSERT(::IsWindow(m_hWndMDIClient));
		return (HWND)::SendMessage(m_hWndMDIClient, WM_MDIGETACTIVE, 0, (LPARAM)lpbMaximized);
	}

	void MDIActivate(HWND hWndChildToActivate)
	{
		ATLASSERT(::IsWindow(m_hWndMDIClient));
		ATLASSERT(::IsWindow(hWndChildToActivate));
		::SendMessage(m_hWndMDIClient, WM_MDIACTIVATE, (WPARAM)hWndChildToActivate, 0);
	}

	void MDINext(HWND hWndChild, BOOL bPrevious = FALSE)
	{
		ATLASSERT(::IsWindow(m_hWndMDIClient));
		ATLASSERT((hWndChild == NULL) || ::IsWindow(hWndChild));
		::SendMessage(m_hWndMDIClient, WM_MDINEXT, (WPARAM)hWndChild, (LPARAM)bPrevious);
	}

	void MDIMaximize(HWND hWndChildToMaximize)
	{
		ATLASSERT(::IsWindow(m_hWndMDIClient));
		ATLASSERT(::IsWindow(hWndChildToMaximize));
		::SendMessage(m_hWndMDIClient, WM_MDIMAXIMIZE, (WPARAM)hWndChildToMaximize, 0);
	}

	void MDIRestore(HWND hWndChildToRestore)
	{
		ATLASSERT(::IsWindow(m_hWndMDIClient));
		ATLASSERT(::IsWindow(hWndChildToRestore));
		::SendMessage(m_hWndMDIClient, WM_MDIRESTORE, (WPARAM)hWndChildToRestore, 0);
	}

	void MDIDestroy(HWND hWndChildToDestroy)
	{
		ATLASSERT(::IsWindow(m_hWndMDIClient));
		ATLASSERT(::IsWindow(hWndChildToDestroy));
		::SendMessage(m_hWndMDIClient, WM_MDIDESTROY, (WPARAM)hWndChildToDestroy, 0);
	}

	BOOL MDICascade(UINT uFlags = 0)
	{
		ATLASSERT(::IsWindow(m_hWndMDIClient));
		return (BOOL)::SendMessage(m_hWndMDIClient, WM_MDICASCADE, (WPARAM)uFlags, 0);
	}

	BOOL MDITile(UINT uFlags = MDITILE_HORIZONTAL)
	{
		ATLASSERT(::IsWindow(m_hWndMDIClient));
		return (BOOL)::SendMessage(m_hWndMDIClient, WM_MDITILE, (WPARAM)uFlags, 0);
	}

	void MDIIconArrange()
	{
		ATLASSERT(::IsWindow(m_hWndMDIClient));
		::SendMessage(m_hWndMDIClient, WM_MDIICONARRANGE, 0, 0);
	}

	HMENU MDISetMenu(HMENU hMenuFrame, HMENU hMenuWindow)
	{
		ATLASSERT(::IsWindow(m_hWndMDIClient));
		return (HMENU)::SendMessage(m_hWndMDIClient, WM_MDISETMENU, (WPARAM)hMenuFrame, (LPARAM)hMenuWindow);
	}

	HMENU MDIRefreshMenu()
	{
		ATLASSERT(::IsWindow(m_hWndMDIClient));
		return (HMENU)::SendMessage(m_hWndMDIClient, WM_MDIREFRESHMENU, 0, 0);
	}

// Additional operations
	static HMENU GetStandardWindowMenu(HMENU hMenu)
	{
		int nCount = ::GetMenuItemCount(hMenu);
		if(nCount == -1)
			return NULL;
		int nLen = ::GetMenuString(hMenu, nCount - 2, NULL, 0, MF_BYPOSITION);
		if(nLen == 0)
			return NULL;
		ATL::CTempBuffer<TCHAR, _WTL_STACK_ALLOC_THRESHOLD> buff;
		LPTSTR lpszText = buff.Allocate(nLen + 1);
		if(lpszText == NULL)
			return NULL;
		if(::GetMenuString(hMenu, nCount - 2, lpszText, nLen + 1, MF_BYPOSITION) != nLen)
			return NULL;
		if(lstrcmp(lpszText, _WTL_MDIWINDOWMENU_TEXT) != 0)
			return NULL;
		return ::GetSubMenu(hMenu, nCount - 2);
	}

	void SetMDIFrameMenu()
	{
		HMENU hWindowMenu = GetStandardWindowMenu(m_hMenu);
		MDISetMenu(m_hMenu, hWindowMenu);
		MDIRefreshMenu();
		::DrawMenuBar(GetMDIFrame());
	}

	HWND GetMDIFrame() const
	{
		return ::GetParent(m_hWndMDIClient);
	}
};


///////////////////////////////////////////////////////////////////////////////
// CMDIFrameWindowImpl

// MDI child command chaining macro (only for MDI frame windows)
#define CHAIN_MDI_CHILD_COMMANDS() \
	if(uMsg == WM_COMMAND) \
	{ \
		HWND hWndChild = this->MDIGetActive(); \
		if(hWndChild != NULL) \
			::SendMessage(hWndChild, uMsg, wParam, lParam); \
	}

template <class T, class TBase = CMDIWindow, class TWinTraits = ATL::CFrameWinTraits>
class ATL_NO_VTABLE CMDIFrameWindowImpl : public CFrameWindowImplBase<TBase, TWinTraits >
{
public:
	HWND Create(HWND hWndParent = NULL, ATL::_U_RECT rect = NULL, LPCTSTR szWindowName = NULL,
			DWORD dwStyle = 0, DWORD dwExStyle = 0,
			HMENU hMenu = NULL, LPVOID lpCreateParam = NULL)
	{
		this->m_hMenu = hMenu;
		ATOM atom = T::GetWndClassInfo().Register(&this->m_pfnSuperWindowProc);

		dwStyle = T::GetWndStyle(dwStyle);
		dwExStyle = T::GetWndExStyle(dwExStyle);

		if(rect.m_lpRect == NULL)
			rect.m_lpRect = &TBase::rcDefault;

		return CFrameWindowImplBase<TBase, TWinTraits >::Create(hWndParent, rect.m_lpRect, szWindowName, dwStyle, dwExStyle, hMenu, atom, lpCreateParam);
	}

	HWND CreateEx(HWND hWndParent = NULL, ATL::_U_RECT rect = NULL, DWORD dwStyle = 0, DWORD dwExStyle = 0, LPVOID lpCreateParam = NULL)
	{
		const int cchName = 256;
		TCHAR szWindowName[cchName] = {};
		::LoadString(ModuleHelper::GetResourceInstance(), T::GetWndClassInfo().m_uCommonResourceID, szWindowName, cchName);
		HMENU hMenu = ::LoadMenu(ModuleHelper::GetResourceInstance(), MAKEINTRESOURCE(T::GetWndClassInfo().m_uCommonResourceID));

		T* pT = static_cast<T*>(this);
		HWND hWnd = pT->Create(hWndParent, rect, szWindowName, dwStyle, dwExStyle, hMenu, lpCreateParam);

		if(hWnd != NULL)
			this->m_hAccel = ::LoadAccelerators(ModuleHelper::GetResourceInstance(), MAKEINTRESOURCE(T::GetWndClassInfo().m_uCommonResourceID));

		return hWnd;
	}

	BOOL CreateSimpleToolBar(UINT nResourceID = 0, DWORD dwStyle = ATL_SIMPLE_TOOLBAR_STYLE, UINT nID = ATL_IDW_TOOLBAR)
	{
		ATLASSERT(!::IsWindow(this->m_hWndToolBar));
		if(nResourceID == 0)
			nResourceID = T::GetWndClassInfo().m_uCommonResourceID;
		this->m_hWndToolBar = T::CreateSimpleToolBarCtrl(this->m_hWnd, nResourceID, TRUE, dwStyle, nID);
		return (this->m_hWndToolBar != NULL);
	}

	virtual WNDPROC GetWindowProc()
	{
		return MDIFrameWindowProc;
	}

	static LRESULT CALLBACK MDIFrameWindowProc(HWND hWnd, UINT uMsg, WPARAM wParam, LPARAM lParam)
	{
		CMDIFrameWindowImpl< T, TBase, TWinTraits >* pThis = (CMDIFrameWindowImpl< T, TBase, TWinTraits >*)hWnd;
		// set a ptr to this message and save the old value
		ATL::_ATL_MSG msg(pThis->m_hWnd, uMsg, wParam, lParam);
		const ATL::_ATL_MSG* pOldMsg = pThis->m_pCurrentMsg;
		pThis->m_pCurrentMsg = &msg;
		// pass to the message map to process
		LRESULT lRes = 0;
		BOOL bRet = pThis->ProcessWindowMessage(pThis->m_hWnd, uMsg, wParam, lParam, lRes, 0);
		// restore saved value for the current message
		ATLASSERT(pThis->m_pCurrentMsg == &msg);
		pThis->m_pCurrentMsg = pOldMsg;
		// do the default processing if message was not handled
		if(!bRet)
		{
			if(uMsg != WM_NCDESTROY)
			{
				lRes = pThis->DefWindowProc(uMsg, wParam, lParam);
			}
			else
			{
				// unsubclass, if needed
				LONG_PTR pfnWndProc = ::GetWindowLongPtr(pThis->m_hWnd, GWLP_WNDPROC);
				lRes = pThis->DefWindowProc(uMsg, wParam, lParam);
				if((pThis->m_pfnSuperWindowProc != ::DefWindowProc) && (::GetWindowLongPtr(pThis->m_hWnd, GWLP_WNDPROC) == pfnWndProc))
					::SetWindowLongPtr(pThis->m_hWnd, GWLP_WNDPROC, (LONG_PTR)pThis->m_pfnSuperWindowProc);
				// mark window as destryed
				pThis->m_dwState |= ATL::CWindowImplRoot< TBase >::WINSTATE_DESTROYED;
			}
		}
		if((pThis->m_dwState & ATL::CWindowImplRoot< TBase >::WINSTATE_DESTROYED) && (pThis->m_pCurrentMsg == NULL))
		{
			// clear out window handle
			HWND hWndThis = pThis->m_hWnd;
			pThis->m_hWnd = NULL;
			pThis->m_dwState &= ~ATL::CWindowImplRoot< TBase >::WINSTATE_DESTROYED;
			// clean up after window is destroyed
			pThis->OnFinalMessage(hWndThis);
		}
		return lRes;
	}

	// Overriden to call DefWindowProc which uses DefFrameProc
	LRESULT DefWindowProc()
	{
		const ATL::_ATL_MSG* pMsg = this->m_pCurrentMsg;
		LRESULT lRes = 0;
		if (pMsg != NULL)
			lRes = DefWindowProc(pMsg->message, pMsg->wParam, pMsg->lParam);
		return lRes;
	}

	LRESULT DefWindowProc(UINT uMsg, WPARAM wParam, LPARAM lParam)
	{
		return ::DefFrameProc(this->m_hWnd, this->m_hWndMDIClient, uMsg, wParam, lParam);
	}

	BOOL PreTranslateMessage(MSG* pMsg)
	{
		if(CFrameWindowImplBase<TBase, TWinTraits>::PreTranslateMessage(pMsg))
			return TRUE;
		return ::TranslateMDISysAccel(this->m_hWndMDIClient, pMsg);
	}

	HWND CreateMDIClient(HMENU hWindowMenu = NULL, UINT nID = ATL_IDW_CLIENT, UINT nFirstChildID = ATL_IDM_FIRST_MDICHILD)
	{
		DWORD dwStyle = WS_VISIBLE | WS_CHILD | WS_CLIPCHILDREN | WS_CLIPSIBLINGS | MDIS_ALLCHILDSTYLES;
		DWORD dwExStyle = WS_EX_CLIENTEDGE;

		CLIENTCREATESTRUCT ccs = {};
		ccs.hWindowMenu = hWindowMenu;
		ccs.idFirstChild = nFirstChildID;

		if((this->GetStyle() & (WS_HSCROLL | WS_VSCROLL)) != 0)
		{
			// parent MDI frame's scroll styles move to the MDICLIENT
			dwStyle |= (this->GetStyle() & (WS_HSCROLL | WS_VSCROLL));

			// fast way to turn off the scrollbar bits (without a resize)
			this->ModifyStyle(WS_HSCROLL | WS_VSCROLL, 0, SWP_NOREDRAW | SWP_FRAMECHANGED);
		}

		// Create MDICLIENT window
		this->m_hWndClient = ::CreateWindowEx(dwExStyle, _T("MDIClient"), NULL,
			dwStyle, 0, 0, 1, 1, this->m_hWnd, (HMENU)LongToHandle(nID),
			ModuleHelper::GetModuleInstance(), (LPVOID)&ccs);
		if (this->m_hWndClient == NULL)
		{
			ATLTRACE2(atlTraceUI, 0, _T("MDI Frame failed to create MDICLIENT.\n"));
			return NULL;
		}

		// Move it to the top of z-order
		::BringWindowToTop(this->m_hWndClient);

		// set as MDI client window
		this->m_hWndMDIClient = this->m_hWndClient;

		// update to proper size
		T* pT = static_cast<T*>(this);
		pT->UpdateLayout();

		return this->m_hWndClient;
	}

	typedef CFrameWindowImplBase<TBase, TWinTraits >   _baseClass;

	BEGIN_MSG_MAP(CMDIFrameWindowImpl)
		MESSAGE_HANDLER(WM_SIZE, OnSize)
		MESSAGE_HANDLER(WM_SETFOCUS, OnSetFocus)
		MESSAGE_HANDLER(WM_MDISETMENU, OnMDISetMenu)
#ifndef _ATL_NO_REBAR_SUPPORT
		NOTIFY_CODE_HANDLER(RBN_AUTOSIZE, OnReBarAutoSize)
		NOTIFY_CODE_HANDLER(RBN_CHEVRONPUSHED, OnChevronPushed)
#endif // !_ATL_NO_REBAR_SUPPORT
		CHAIN_MSG_MAP(_baseClass)
	END_MSG_MAP()

	LRESULT OnSize(UINT /*uMsg*/, WPARAM wParam, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		if(wParam != SIZE_MINIMIZED)
		{
			T* pT = static_cast<T*>(this);
			pT->UpdateLayout();
		}
		// message must be handled, otherwise DefFrameProc would resize the client again
		return 0;
	}

	LRESULT OnSetFocus(UINT uMsg, WPARAM wParam, LPARAM lParam, BOOL& /*bHandled*/)
	{
		// don't allow CFrameWindowImplBase to handle this one
		return DefWindowProc(uMsg, wParam, lParam);
	}

	LRESULT OnMDISetMenu(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		this->SetMDIFrameMenu();
		return 0;
	}

#ifndef _ATL_NO_REBAR_SUPPORT
	LRESULT OnReBarAutoSize(int /*idCtrl*/, LPNMHDR /*pnmh*/, BOOL& /*bHandled*/)
	{
		T* pT = static_cast<T*>(this);
		pT->UpdateLayout(FALSE);
		return 0;
	}

	LRESULT OnChevronPushed(int /*idCtrl*/, LPNMHDR pnmh, BOOL& bHandled)
	{
		T* pT = static_cast<T*>(this);
		typename CFrameWindowImplBase<TBase, TWinTraits >::_ChevronMenuInfo cmi = { NULL, (LPNMREBARCHEVRON)pnmh, false };
		if(!pT->PrepareChevronMenu(cmi))
		{
			bHandled = FALSE;
			return 1;
		}
		// display a popup menu with hidden items
		pT->DisplayChevronMenu(cmi);
		// cleanup
		pT->CleanupChevronMenu(cmi);
		return 0;
	}
#endif // !_ATL_NO_REBAR_SUPPORT
};


///////////////////////////////////////////////////////////////////////////////
// CMDIChildWindowImpl

template <class T, class TBase = CMDIWindow, class TWinTraits = ATL::CMDIChildWinTraits>
class ATL_NO_VTABLE CMDIChildWindowImpl : public CFrameWindowImplBase<TBase, TWinTraits >
{
public:
	HWND Create(HWND hWndParent, ATL::_U_RECT rect = NULL, LPCTSTR szWindowName = NULL,
			DWORD dwStyle = 0, DWORD dwExStyle = 0,
			UINT nMenuID = 0, LPVOID lpCreateParam = NULL)
	{
		ATOM atom = T::GetWndClassInfo().Register(&this->m_pfnSuperWindowProc);

		if(nMenuID != 0)
			this->m_hMenu = ::LoadMenu(ModuleHelper::GetResourceInstance(), MAKEINTRESOURCE(nMenuID));

		dwStyle = T::GetWndStyle(dwStyle);
		dwExStyle = T::GetWndExStyle(dwExStyle);

		dwExStyle |= WS_EX_MDICHILD;   // force this one
		this->m_pfnSuperWindowProc = ::DefMDIChildProc;
		this->m_hWndMDIClient = hWndParent;
		ATLASSERT(::IsWindow(this->m_hWndMDIClient));

		if(rect.m_lpRect == NULL)
			rect.m_lpRect = &TBase::rcDefault;

		// If the currently active MDI child is maximized, we want to create this one maximized too
		ATL::CWindow wndParent = hWndParent;
		BOOL bMaximized = FALSE;
		wndParent.SendMessage(WM_MDIGETACTIVE, 0, (LPARAM)&bMaximized);
		if(bMaximized)
			wndParent.SetRedraw(FALSE);

		HWND hWnd = CFrameWindowImplBase<TBase, TWinTraits >::Create(hWndParent, rect.m_lpRect, szWindowName, dwStyle, dwExStyle, (UINT)0U, atom, lpCreateParam);

		if(bMaximized)
		{
			// Maximize and redraw everything
			if(hWnd != NULL)
				this->MDIMaximize(hWnd);
			wndParent.SetRedraw(TRUE);
			wndParent.RedrawWindow(NULL, NULL, RDW_INVALIDATE | RDW_ALLCHILDREN);
			::SetFocus(this->GetMDIFrame());   // focus will be set back to this window
		}
		else if((hWnd != NULL) && ::IsWindowVisible(this->m_hWnd) && !::IsChild(hWnd, ::GetFocus()))
		{
			::SetFocus(hWnd);
		}

		return hWnd;
	}

	HWND CreateEx(HWND hWndParent, ATL::_U_RECT rect = NULL, LPCTSTR lpcstrWindowName = NULL, DWORD dwStyle = 0, DWORD dwExStyle = 0, LPVOID lpCreateParam = NULL)
	{
		const int cchName = 256;
		TCHAR szWindowName[cchName] = {};
		if(lpcstrWindowName == NULL)
		{
			::LoadString(ModuleHelper::GetResourceInstance(), T::GetWndClassInfo().m_uCommonResourceID, szWindowName, cchName);
			lpcstrWindowName = szWindowName;
		}

		T* pT = static_cast<T*>(this);
		HWND hWnd = pT->Create(hWndParent, rect, lpcstrWindowName, dwStyle, dwExStyle, T::GetWndClassInfo().m_uCommonResourceID, lpCreateParam);

		if(hWnd != NULL)
			this->m_hAccel = ::LoadAccelerators(ModuleHelper::GetResourceInstance(), MAKEINTRESOURCE(T::GetWndClassInfo().m_uCommonResourceID));

		return hWnd;
	}

	BOOL CreateSimpleToolBar(UINT nResourceID = 0, DWORD dwStyle = ATL_SIMPLE_TOOLBAR_STYLE, UINT nID = ATL_IDW_TOOLBAR)
	{
		ATLASSERT(!::IsWindow(this->m_hWndToolBar));
		if(nResourceID == 0)
			nResourceID = T::GetWndClassInfo().m_uCommonResourceID;
		this->m_hWndToolBar = T::CreateSimpleToolBarCtrl(this->m_hWnd, nResourceID, TRUE, dwStyle, nID);
		return (this->m_hWndToolBar != NULL);
	}

	BOOL UpdateClientEdge(LPRECT lpRect = NULL)
	{
		// only adjust for active MDI child window
		HWND hWndChild = this->MDIGetActive();
		if((hWndChild != NULL) && (hWndChild != this->m_hWnd))
			return FALSE;

		// need to adjust the client edge style as max/restore happens
		DWORD dwStyle = ::GetWindowLong(this->m_hWndMDIClient, GWL_EXSTYLE);
		DWORD dwNewStyle = dwStyle;
		if((hWndChild != NULL) && ((this->GetExStyle() & WS_EX_CLIENTEDGE) == 0) && ((this->GetStyle() & WS_MAXIMIZE) != 0))
			dwNewStyle &= ~(WS_EX_CLIENTEDGE);
		else
			dwNewStyle |= WS_EX_CLIENTEDGE;

		if(dwStyle != dwNewStyle)
		{
			// SetWindowPos will not move invalid bits
			::RedrawWindow(this->m_hWndMDIClient, NULL, NULL,
				RDW_INVALIDATE | RDW_ALLCHILDREN);
			// remove/add WS_EX_CLIENTEDGE to MDI client area
			::SetWindowLong(this->m_hWndMDIClient, GWL_EXSTYLE, dwNewStyle);
			::SetWindowPos(this->m_hWndMDIClient, NULL, 0, 0, 0, 0,
				SWP_FRAMECHANGED | SWP_NOACTIVATE | SWP_NOMOVE | SWP_NOSIZE |
				SWP_NOZORDER | SWP_NOCOPYBITS);

			// return new client area
			if (lpRect != NULL)
				::GetClientRect(this->m_hWndMDIClient, lpRect);

			return TRUE;
		}

		return FALSE;
	}

	typedef CFrameWindowImplBase<TBase, TWinTraits >   _baseClass;
	BEGIN_MSG_MAP(CMDIChildWindowImpl)
		MESSAGE_HANDLER(WM_SIZE, OnSize)
		MESSAGE_HANDLER(WM_WINDOWPOSCHANGED, OnWindowPosChanged)
		MESSAGE_HANDLER(WM_MOUSEACTIVATE, OnMouseActivate)
		MESSAGE_HANDLER(WM_MENUSELECT, OnMenuSelect)
		MESSAGE_HANDLER(WM_MDIACTIVATE, OnMDIActivate)
		MESSAGE_HANDLER(WM_DESTROY, OnDestroy)
#ifndef _ATL_NO_REBAR_SUPPORT
		NOTIFY_CODE_HANDLER(RBN_AUTOSIZE, OnReBarAutoSize)
		NOTIFY_CODE_HANDLER(RBN_CHEVRONPUSHED, OnChevronPushed)
#endif // !_ATL_NO_REBAR_SUPPORT
		CHAIN_MSG_MAP(_baseClass)
	END_MSG_MAP()

	LRESULT OnSize(UINT uMsg, WPARAM wParam, LPARAM lParam, BOOL& /*bHandled*/)
	{
		this->DefWindowProc(uMsg, wParam, lParam);   // needed for MDI children
		if(wParam != SIZE_MINIMIZED)
		{
			T* pT = static_cast<T*>(this);
			pT->UpdateLayout();
		}
		return 0;
	}

	LRESULT OnWindowPosChanged(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM lParam, BOOL& bHandled)
	{
		// update MDI client edge and adjust MDI child rect
		LPWINDOWPOS lpWndPos = (LPWINDOWPOS)lParam;

		if(!(lpWndPos->flags & SWP_NOSIZE))
		{
			RECT rectClient = {};
			if(UpdateClientEdge(&rectClient) && ((this->GetStyle() & WS_MAXIMIZE) != 0))
			{
				::AdjustWindowRectEx(&rectClient, this->GetStyle(), FALSE, this->GetExStyle());
				lpWndPos->x = rectClient.left;
				lpWndPos->y = rectClient.top;
				lpWndPos->cx = rectClient.right - rectClient.left;
				lpWndPos->cy = rectClient.bottom - rectClient.top;
			}
		}

		bHandled = FALSE;
		return 1;
	}

	LRESULT OnMouseActivate(UINT uMsg, WPARAM wParam, LPARAM lParam, BOOL& /*bHandled*/)
	{
		LRESULT lRes = this->DefWindowProc(uMsg, wParam, lParam);

		// Activate this MDI window if needed
		if((lRes == MA_ACTIVATE) || (lRes == MA_ACTIVATEANDEAT))
		{
			if(this->MDIGetActive() != this->m_hWnd)
				this->MDIActivate(this->m_hWnd);
		}

		return lRes;
	}

	LRESULT OnMenuSelect(UINT uMsg, WPARAM wParam, LPARAM lParam, BOOL& /*bHandled*/)
	{
		return ::SendMessage(this->GetMDIFrame(), uMsg, wParam, lParam);
	}

	LRESULT OnMDIActivate(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM lParam, BOOL& bHandled)
	{
		if(((HWND)lParam == this->m_hWnd) && (this->m_hMenu != NULL))
			this->SetMDIFrameMenu();
		else if((HWND)lParam == NULL)
			::SendMessage(this->GetMDIFrame(), WM_MDISETMENU, 0, 0);

		bHandled = FALSE;
		return 1;
	}

	LRESULT OnDestroy(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& bHandled)
	{
		if(this->m_hMenu != NULL)
		{
			::DestroyMenu(this->m_hMenu);
			this->m_hMenu = NULL;
		}
		UpdateClientEdge();
		bHandled = FALSE;
		return 1;
	}

#ifndef _ATL_NO_REBAR_SUPPORT
	LRESULT OnReBarAutoSize(int /*idCtrl*/, LPNMHDR /*pnmh*/, BOOL& /*bHandled*/)
	{
		T* pT = static_cast<T*>(this);
		pT->UpdateLayout(FALSE);
		return 0;
	}

	LRESULT OnChevronPushed(int /*idCtrl*/, LPNMHDR pnmh, BOOL& bHandled)
	{
		T* pT = static_cast<T*>(this);
		typename CFrameWindowImplBase<TBase, TWinTraits >::_ChevronMenuInfo cmi = { NULL, (LPNMREBARCHEVRON)pnmh, false };
		if(!pT->PrepareChevronMenu(cmi))
		{
			bHandled = FALSE;
			return 1;
		}
		// display a popup menu with hidden items
		pT->DisplayChevronMenu(cmi);
		// cleanup
		pT->CleanupChevronMenu(cmi);
		return 0;
	}
#endif // !_ATL_NO_REBAR_SUPPORT
};


///////////////////////////////////////////////////////////////////////////////
// COwnerDraw - MI class for owner-draw support

template <class T>
class COwnerDraw
{
public:
// Message map and handlers
	BEGIN_MSG_MAP(COwnerDraw< T >)
		MESSAGE_HANDLER(WM_DRAWITEM, OnDrawItem)
		MESSAGE_HANDLER(WM_MEASUREITEM, OnMeasureItem)
		MESSAGE_HANDLER(WM_COMPAREITEM, OnCompareItem)
		MESSAGE_HANDLER(WM_DELETEITEM, OnDeleteItem)
	ALT_MSG_MAP(1)
		MESSAGE_HANDLER(OCM_DRAWITEM, OnDrawItem)
		MESSAGE_HANDLER(OCM_MEASUREITEM, OnMeasureItem)
		MESSAGE_HANDLER(OCM_COMPAREITEM, OnCompareItem)
		MESSAGE_HANDLER(OCM_DELETEITEM, OnDeleteItem)
	END_MSG_MAP()

	LRESULT OnDrawItem(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM lParam, BOOL& bHandled)
	{
		T* pT = static_cast<T*>(this);
		pT->SetMsgHandled(TRUE);
		pT->DrawItem((LPDRAWITEMSTRUCT)lParam);
		bHandled = pT->IsMsgHandled();
		return (LRESULT)TRUE;
	}

	LRESULT OnMeasureItem(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM lParam, BOOL& bHandled)
	{
		T* pT = static_cast<T*>(this);
		pT->SetMsgHandled(TRUE);
		pT->MeasureItem((LPMEASUREITEMSTRUCT)lParam);
		bHandled = pT->IsMsgHandled();
		return (LRESULT)TRUE;
	}

	LRESULT OnCompareItem(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM lParam, BOOL& bHandled)
	{
		T* pT = static_cast<T*>(this);
		pT->SetMsgHandled(TRUE);
		bHandled = pT->IsMsgHandled();
		return (LRESULT)pT->CompareItem((LPCOMPAREITEMSTRUCT)lParam);
	}

	LRESULT OnDeleteItem(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM lParam, BOOL& bHandled)
	{
		T* pT = static_cast<T*>(this);
		pT->SetMsgHandled(TRUE);
		pT->DeleteItem((LPDELETEITEMSTRUCT)lParam);
		bHandled = pT->IsMsgHandled();
		return (LRESULT)TRUE;
	}

// Overrideables
	void DrawItem(LPDRAWITEMSTRUCT /*lpDrawItemStruct*/)
	{
		// must be implemented
		ATLASSERT(FALSE);
	}

	void MeasureItem(LPMEASUREITEMSTRUCT lpMeasureItemStruct)
	{
		if(lpMeasureItemStruct->CtlType != ODT_MENU)
		{
			// return default height for a system font
			T* pT = static_cast<T*>(this);
			HWND hWnd = pT->GetDlgItem(lpMeasureItemStruct->CtlID);
			CClientDC dc(hWnd);
			TEXTMETRIC tm = {};
			dc.GetTextMetrics(&tm);

			lpMeasureItemStruct->itemHeight = tm.tmHeight;
		}
		else
			lpMeasureItemStruct->itemHeight = ::GetSystemMetrics(SM_CYMENU);
	}

	int CompareItem(LPCOMPAREITEMSTRUCT /*lpCompareItemStruct*/)
	{
		// all items are equal
		return 0;
	}

	void DeleteItem(LPDELETEITEMSTRUCT /*lpDeleteItemStruct*/)
	{
		// default - nothing
	}
};


///////////////////////////////////////////////////////////////////////////////
// Update UI macros

// these build the Update UI map inside a class definition
#define BEGIN_UPDATE_UI_MAP(thisClass) \
	static const CUpdateUIBase::_AtlUpdateUIMap* GetUpdateUIMap() \
	{ \
		static const CUpdateUIBase::_AtlUpdateUIMap theMap[] = \
		{

#define UPDATE_ELEMENT(nID, wType) \
			{ nID,  wType },

#define END_UPDATE_UI_MAP() \
			{ (WORD)-1, 0 } \
		}; \
		return theMap; \
	}

///////////////////////////////////////////////////////////////////////////////
// CUpdateUI - manages UI elements updating

class CUpdateUIBase
{
public:
	// constants
	enum
	{
		// UI element type
		UPDUI_MENUPOPUP		= 0x0001,
		UPDUI_MENUBAR		= 0x0002,
		UPDUI_CHILDWINDOW	= 0x0004,
		UPDUI_TOOLBAR		= 0x0008,
		UPDUI_STATUSBAR		= 0x0010,
		// state
		UPDUI_ENABLED		= 0x0000,
		UPDUI_DISABLED		= 0x0100,
		UPDUI_CHECKED		= 0x0200,
		UPDUI_CHECKED2		= 0x0400,
		UPDUI_RADIO		= 0x0800,
		UPDUI_DEFAULT		= 0x1000,
		UPDUI_TEXT		= 0x2000,
		// internal state
		UPDUI_CLEARDEFAULT	= 0x4000,
	};

	// element data
	struct _AtlUpdateUIElement
	{
		HWND m_hWnd;
		WORD m_wType;

		bool operator ==(const _AtlUpdateUIElement& e) const
		{ return ((m_hWnd == e.m_hWnd) && (m_wType == e.m_wType)); }
	};

	// map data
	struct _AtlUpdateUIMap
	{
		WORD m_nID;
		WORD m_wType;

		bool operator ==(const _AtlUpdateUIMap& e) const
		{ return ((m_nID == e.m_nID) && (m_wType == e.m_wType)); }
	};

	// instance data
#pragma warning(push)
#pragma warning(disable: 4201)   // nameless unions are part of C++

	struct _AtlUpdateUIData
	{
		WORD m_wState;
		union
		{
			void* m_lpData;
			LPTSTR m_lpstrText;
			struct
			{
				WORD m_nIDFirst;
				WORD m_nIDLast;
			};
		};

		bool operator ==(const _AtlUpdateUIData& e) const
		{ return ((m_wState == e.m_wState) && (m_lpData == e.m_lpData)); }
	};

#pragma warning(pop)

	ATL::CSimpleArray<_AtlUpdateUIElement> m_UIElements;   // elements data
	const _AtlUpdateUIMap* m_pUIMap;                       // static UI data
	_AtlUpdateUIData* m_pUIData;                           // instance UI data
	WORD m_wDirtyType;                                     // global dirty flag

	bool m_bBlockAccelerators;


// Constructor, destructor
	CUpdateUIBase() : m_pUIMap(NULL), m_pUIData(NULL), m_wDirtyType(0), m_bBlockAccelerators(false)
	{ }

	~CUpdateUIBase()
	{
		if((m_pUIMap != NULL) && (m_pUIData != NULL))
		{
			const _AtlUpdateUIMap* pUIMap = m_pUIMap;
			_AtlUpdateUIData* pUIData = m_pUIData;
			while(pUIMap->m_nID != (WORD)-1)
			{
				if(pUIData->m_wState & UPDUI_TEXT)
					delete [] pUIData->m_lpstrText;
				pUIMap++;
				pUIData++;
			}
			delete [] m_pUIData;
		}
	}

// Check for disabled commands
	bool UIGetBlockAccelerators() const
	{
		return m_bBlockAccelerators;
	}

	bool UISetBlockAccelerators(bool bBlock)
	{
		bool bOld = m_bBlockAccelerators;
		m_bBlockAccelerators = bBlock;
		return bOld;
	}

// Add elements
	BOOL UIAddMenuBar(HWND hWnd)                // menu bar (main menu)
	{
		if(hWnd == NULL)
			return FALSE;
		_AtlUpdateUIElement e;
		e.m_hWnd = hWnd;
		e.m_wType = UPDUI_MENUBAR;
		return m_UIElements.Add(e);
	}

	BOOL UIAddToolBar(HWND hWnd)                // toolbar
	{
		if(hWnd == NULL)
			return FALSE;
		_AtlUpdateUIElement e;
		e.m_hWnd = hWnd;
		e.m_wType = UPDUI_TOOLBAR;
		return m_UIElements.Add(e);
	}

	BOOL UIAddStatusBar(HWND hWnd)              // status bar
	{
		if(hWnd == NULL)
			return FALSE;
		_AtlUpdateUIElement e;
		e.m_hWnd = hWnd;
		e.m_wType = UPDUI_STATUSBAR;
		return m_UIElements.Add(e);
	}

	BOOL UIAddChildWindowContainer(HWND hWnd)   // child window
	{
		if(hWnd == NULL)
			return FALSE;
		_AtlUpdateUIElement e;
		e.m_hWnd = hWnd;
		e.m_wType = UPDUI_CHILDWINDOW;
		return m_UIElements.Add(e);
	}

// Message map for popup menu updates and accelerator blocking
	BEGIN_MSG_MAP(CUpdateUIBase)
		MESSAGE_HANDLER(WM_INITMENUPOPUP, OnInitMenuPopup)
		MESSAGE_HANDLER(WM_COMMAND, OnCommand)
	END_MSG_MAP()

	LRESULT OnInitMenuPopup(UINT /*uMsg*/, WPARAM wParam, LPARAM /*lParam*/, BOOL& bHandled)
	{
		bHandled = FALSE;
		HMENU hMenu = (HMENU)wParam;
		if(hMenu == NULL)
			return 1;
		_AtlUpdateUIData* pUIData = m_pUIData;
		if(pUIData == NULL)
			return 1;
		const _AtlUpdateUIMap* pMap = m_pUIMap;
		while(pMap->m_nID != (WORD)-1)
		{
			if(pMap->m_wType & UPDUI_MENUPOPUP)
			{
				UIUpdateMenuBarElement(pMap->m_nID, pUIData, hMenu);

				if((pUIData->m_wState & UPDUI_RADIO) != 0)
					::CheckMenuRadioItem(hMenu, pUIData->m_nIDFirst, pUIData->m_nIDLast, pMap->m_nID, MF_BYCOMMAND);
			}
			pMap++;
			pUIData++;
		}
		return 0;
	}

	LRESULT OnCommand(UINT /*uMsg*/, WPARAM wParam, LPARAM /*lParam*/, BOOL& bHandled)
	{
		bHandled = FALSE;
		if(m_bBlockAccelerators && (HIWORD(wParam) == 1))   // accelerators only
		{
			int nID = LOWORD(wParam);
			if((UIGetState(nID) & UPDUI_DISABLED) == UPDUI_DISABLED)
			{
				ATLTRACE2(atlTraceUI, 0, _T("CUpdateUIBase::OnCommand - blocked disabled command 0x%4.4X\n"), nID);
				bHandled = TRUE;   // eat the command, UI item is disabled
			}
		}
		return 0;
	}

// methods for setting UI element state
	BOOL UIEnable(int nID, BOOL bEnable, BOOL bForceUpdate = FALSE)
	{
		const _AtlUpdateUIMap* pMap = m_pUIMap;
		_AtlUpdateUIData* pUIData = m_pUIData;
		if(pUIData == NULL)
			return FALSE;

		for( ; pMap->m_nID != (WORD)-1; pMap++, pUIData++)
		{
			if(nID == (int)pMap->m_nID)
			{
				if(bEnable)
				{
					if(pUIData->m_wState & UPDUI_DISABLED)
					{
						pUIData->m_wState |= pMap->m_wType;
						pUIData->m_wState &= ~UPDUI_DISABLED;
					}
				}
				else
				{
					if(!(pUIData->m_wState & UPDUI_DISABLED))
					{
						pUIData->m_wState |= pMap->m_wType;
						pUIData->m_wState |= UPDUI_DISABLED;
					}
				}

				if(bForceUpdate)
					pUIData->m_wState |= pMap->m_wType;
				if(pUIData->m_wState & pMap->m_wType)
					m_wDirtyType |= pMap->m_wType;

				break;   // found
			}
		}

		return TRUE;
	}

	BOOL UISetCheck(int nID, int nCheck, BOOL bForceUpdate = FALSE)
	{
		const _AtlUpdateUIMap* pMap = m_pUIMap;
		_AtlUpdateUIData* pUIData = m_pUIData;
		if(pUIData == NULL)
			return FALSE;

		for( ; pMap->m_nID != (WORD)-1; pMap++, pUIData++)
		{
			if(nID == (int)pMap->m_nID)
			{
				switch(nCheck)
				{
				case 0:
					if((pUIData->m_wState & UPDUI_CHECKED) || (pUIData->m_wState & UPDUI_CHECKED2))
					{
						pUIData->m_wState |= pMap->m_wType;
						pUIData->m_wState &= ~(UPDUI_CHECKED | UPDUI_CHECKED2);
					}
					break;
				case 1:
					if(!(pUIData->m_wState & UPDUI_CHECKED))
					{
						pUIData->m_wState |= pMap->m_wType;
						pUIData->m_wState &= ~UPDUI_CHECKED2;
						pUIData->m_wState |= UPDUI_CHECKED;
					}
					break;
				case 2:
					if(!(pUIData->m_wState & UPDUI_CHECKED2))
					{
						pUIData->m_wState |= pMap->m_wType;
						pUIData->m_wState &= ~UPDUI_CHECKED;
						pUIData->m_wState |= UPDUI_CHECKED2;
					}
					break;
				}

				if(bForceUpdate)
					pUIData->m_wState |= pMap->m_wType;
				if(pUIData->m_wState & pMap->m_wType)
					m_wDirtyType |= pMap->m_wType;

				break;   // found
			}
		}

		return TRUE;
	}

	// variant that supports bool (checked/not-checked, no intermediate state)
	BOOL UISetCheck(int nID, bool bCheck, BOOL bForceUpdate = FALSE)
	{
		return UISetCheck(nID, bCheck ? 1 : 0, bForceUpdate);
	}

	BOOL UISetRadio(int nID, BOOL bRadio, BOOL bForceUpdate = FALSE)
	{
		const _AtlUpdateUIMap* pMap = m_pUIMap;
		_AtlUpdateUIData* pUIData = m_pUIData;
		if(pUIData == NULL)
			return FALSE;

		for( ; pMap->m_nID != (WORD)-1; pMap++, pUIData++)
		{
			if(nID == (int)pMap->m_nID)
			{
				if(bRadio)
				{
					if(!(pUIData->m_wState & UPDUI_RADIO))
					{
						pUIData->m_wState |= pMap->m_wType;
						pUIData->m_wState |= UPDUI_RADIO;
					}
				}
				else
				{
					if(pUIData->m_wState & UPDUI_RADIO)
					{
						pUIData->m_wState |= pMap->m_wType;
						pUIData->m_wState &= ~UPDUI_RADIO;
					}
				}

				if(bForceUpdate)
					pUIData->m_wState |= pMap->m_wType;
				if(pUIData->m_wState & pMap->m_wType)
					m_wDirtyType |= pMap->m_wType;

				break;   // found
			}
		}

		return TRUE;
	}

	// for menu items
	BOOL UISetRadioMenuItem(int nID, int nIDFirst, int nIDLast, BOOL bForceUpdate = FALSE)
	{
		const _AtlUpdateUIMap* pMap = m_pUIMap;
		_AtlUpdateUIData* pUIData = m_pUIData;
		if(pUIData == NULL)
			return FALSE;

		for( ; pMap->m_nID != (WORD)-1; pMap++, pUIData++)
		{
			if(nID == (int)pMap->m_nID)
			{
				pUIData->m_wState |= pMap->m_wType;
				pUIData->m_wState |= UPDUI_RADIO;
				pUIData->m_nIDFirst = (WORD)nIDFirst;
				pUIData->m_nIDLast = (WORD)nIDLast;

				if(bForceUpdate)
					pUIData->m_wState |= pMap->m_wType;
				if(pUIData->m_wState & pMap->m_wType)
					m_wDirtyType |= pMap->m_wType;
			}
			else if((pMap->m_nID >= nIDFirst) && (pMap->m_nID <= nIDLast))
			{
				if(pUIData->m_wState & UPDUI_RADIO)
				{
					pUIData->m_wState &= ~pMap->m_wType;
					pUIData->m_wState &= ~UPDUI_RADIO;
					pUIData->m_nIDFirst = 0;
					pUIData->m_nIDLast = 0;
				}
			}

			if(pMap->m_nID == nIDLast)
				break;
		}

		return TRUE;
	}

	BOOL UISetText(int nID, LPCTSTR lpstrText, BOOL bForceUpdate = FALSE)
	{
		const _AtlUpdateUIMap* pMap = m_pUIMap;
		_AtlUpdateUIData* pUIData = m_pUIData;
		if(pUIData == NULL)
			return FALSE;
		if(lpstrText == NULL)
			lpstrText = _T("");

		for( ; pMap->m_nID != (WORD)-1; pMap++, pUIData++)
		{
			if(nID == (int)pMap->m_nID)
			{
				if((pUIData->m_lpstrText == NULL) || lstrcmp(pUIData->m_lpstrText, lpstrText))
				{
					delete [] pUIData->m_lpstrText;
					pUIData->m_lpstrText = NULL;
					int nStrLen = lstrlen(lpstrText);
					ATLTRY(pUIData->m_lpstrText = new TCHAR[nStrLen + 1]);
					if(pUIData->m_lpstrText == NULL)
					{
						ATLTRACE2(atlTraceUI, 0, _T("UISetText - memory allocation failed\n"));
						break;
					}
					ATL::Checked::tcscpy_s(pUIData->m_lpstrText, nStrLen + 1, lpstrText);
					pUIData->m_wState |= (UPDUI_TEXT | pMap->m_wType);
				}

				if(bForceUpdate)
					pUIData->m_wState |= (UPDUI_TEXT | pMap->m_wType);
				if(pUIData->m_wState & pMap->m_wType)
					m_wDirtyType |= pMap->m_wType;

				break;   // found
			}
		}

		return TRUE;
	}

	BOOL UISetDefault(int nID, BOOL bDefault, BOOL bForceUpdate = FALSE)
	{
		const _AtlUpdateUIMap* pMap = m_pUIMap;
		_AtlUpdateUIData* pUIData = m_pUIData;
		if(pUIData == NULL)
			return FALSE;

		for( ; pMap->m_nID != (WORD)-1; pMap++, pUIData++)
		{
			if(nID == (int)pMap->m_nID)
			{
				if(bDefault)
				{
					if((pUIData->m_wState & UPDUI_DEFAULT) == 0)
					{
						pUIData->m_wState |= pMap->m_wType;
						pUIData->m_wState |= UPDUI_DEFAULT;
					}
				}
				else
				{
					if((pUIData->m_wState & UPDUI_DEFAULT) != 0)
					{
						pUIData->m_wState |= pMap->m_wType;
						pUIData->m_wState &= ~UPDUI_DEFAULT;
						pUIData->m_wState |= UPDUI_CLEARDEFAULT;
					}
				}

				if(bForceUpdate)
					pUIData->m_wState |= pMap->m_wType;
				if(pUIData->m_wState & pMap->m_wType)
					m_wDirtyType |= pMap->m_wType;

				break;   // found
			}
		}

		return TRUE;
	}

// methods for complete state set/get
	BOOL UISetState(int nID, DWORD dwState)
	{
		const _AtlUpdateUIMap* pMap = m_pUIMap;
		_AtlUpdateUIData* pUIData = m_pUIData;
		if(pUIData == NULL)
			return FALSE;
		for( ; pMap->m_nID != (WORD)-1; pMap++, pUIData++)
		{
			if(nID == (int)pMap->m_nID)
			{		
				pUIData->m_wState = (WORD)(dwState | pMap->m_wType);
				m_wDirtyType |= pMap->m_wType;
				break;   // found
			}
		}
		return TRUE;
	}

	DWORD UIGetState(int nID)
	{
		const _AtlUpdateUIMap* pMap = m_pUIMap;
		_AtlUpdateUIData* pUIData = m_pUIData;
		if(pUIData == NULL)
			return 0;
		for( ; pMap->m_nID != (WORD)-1; pMap++, pUIData++)
		{
			if(nID == (int)pMap->m_nID)
				return pUIData->m_wState;
		}
		return 0;
	}

// methods for updating UI
	BOOL UIUpdateMenuBar(BOOL bForceUpdate = FALSE, BOOL bMainMenu = FALSE)
	{
		if(!(m_wDirtyType & UPDUI_MENUBAR) && !bForceUpdate)
			return TRUE;

		const _AtlUpdateUIMap* pMap = m_pUIMap;
		_AtlUpdateUIData* pUIData = m_pUIData;
		if(pUIData == NULL)
			return FALSE;

		while(pMap->m_nID != (WORD)-1)
		{
			for(int i = 0; i < m_UIElements.GetSize(); i++)
			{
				if(m_UIElements[i].m_wType == UPDUI_MENUBAR)
				{
					HMENU hMenu = ::GetMenu(m_UIElements[i].m_hWnd);
					if((hMenu != NULL) && (pUIData->m_wState & UPDUI_MENUBAR) && (pMap->m_wType & UPDUI_MENUBAR))
						UIUpdateMenuBarElement(pMap->m_nID, pUIData, hMenu);
				}
				if(bMainMenu)
					::DrawMenuBar(m_UIElements[i].m_hWnd);
			}
			pMap++;
			pUIData->m_wState &= ~UPDUI_MENUBAR;
			if(pUIData->m_wState & UPDUI_TEXT)
			{
				delete [] pUIData->m_lpstrText;
				pUIData->m_lpstrText = NULL;
				pUIData->m_wState &= ~UPDUI_TEXT;
			}
			pUIData++;
		}

		m_wDirtyType &= ~UPDUI_MENUBAR;
		return TRUE;
	}

	BOOL UIUpdateToolBar(BOOL bForceUpdate = FALSE)
	{
		if(!(m_wDirtyType & UPDUI_TOOLBAR) && !bForceUpdate)
			return TRUE;

		const _AtlUpdateUIMap* pMap = m_pUIMap;
		_AtlUpdateUIData* pUIData = m_pUIData;
		if(pUIData == NULL)
			return FALSE;

		while(pMap->m_nID != (WORD)-1)
		{
			for(int i = 0; i < m_UIElements.GetSize(); i++)
			{
				if(m_UIElements[i].m_wType == UPDUI_TOOLBAR)
				{
					if((pUIData->m_wState & UPDUI_TOOLBAR) && (pMap->m_wType & UPDUI_TOOLBAR))
						UIUpdateToolBarElement(pMap->m_nID, pUIData, m_UIElements[i].m_hWnd);
				}
			}
			pMap++;
			pUIData->m_wState &= ~UPDUI_TOOLBAR;
			pUIData++;
		}

		m_wDirtyType &= ~UPDUI_TOOLBAR;
		return TRUE;
	}

	BOOL UIUpdateStatusBar(BOOL bForceUpdate = FALSE)
	{
		if(!(m_wDirtyType & UPDUI_STATUSBAR) && !bForceUpdate)
			return TRUE;

		const _AtlUpdateUIMap* pMap = m_pUIMap;
		_AtlUpdateUIData* pUIData = m_pUIData;
		if(pUIData == NULL)
			return FALSE;

		while(pMap->m_nID != (WORD)-1)
		{
			for(int i = 0; i < m_UIElements.GetSize(); i++)
			{
				if(m_UIElements[i].m_wType == UPDUI_STATUSBAR)
				{
					if((pUIData->m_wState & UPDUI_STATUSBAR) && (pMap->m_wType & UPDUI_STATUSBAR))
						UIUpdateStatusBarElement(pMap->m_nID, pUIData, m_UIElements[i].m_hWnd);
				}
			}
			pMap++;
			pUIData->m_wState &= ~UPDUI_STATUSBAR;
			if(pUIData->m_wState & UPDUI_TEXT)
			{
				delete [] pUIData->m_lpstrText;
				pUIData->m_lpstrText = NULL;
				pUIData->m_wState &= ~UPDUI_TEXT;
			}
			pUIData++;
		}

		m_wDirtyType &= ~UPDUI_STATUSBAR;
		return TRUE;
	}

	BOOL UIUpdateChildWindows(BOOL bForceUpdate = FALSE)
	{
		if(!(m_wDirtyType & UPDUI_CHILDWINDOW) && !bForceUpdate)
			return TRUE;

		const _AtlUpdateUIMap* pMap = m_pUIMap;
		_AtlUpdateUIData* pUIData = m_pUIData;
		if(pUIData == NULL)
			return FALSE;

		while(pMap->m_nID != (WORD)-1)
		{
			for(int i = 0; i < m_UIElements.GetSize(); i++)
			{
				if(m_UIElements[i].m_wType == UPDUI_CHILDWINDOW)
				{
					if((pUIData->m_wState & UPDUI_CHILDWINDOW) && (pMap->m_wType & UPDUI_CHILDWINDOW))
						UIUpdateChildWindow(pMap->m_nID, pUIData, m_UIElements[i].m_hWnd);
				}
			}
			pMap++;
			pUIData->m_wState &= ~UPDUI_CHILDWINDOW;
			if(pUIData->m_wState & UPDUI_TEXT)
			{
				delete [] pUIData->m_lpstrText;
				pUIData->m_lpstrText = NULL;
				pUIData->m_wState &= ~UPDUI_TEXT;
			}
			pUIData++;
		}

		m_wDirtyType &= ~UPDUI_CHILDWINDOW;
		return TRUE;
	}

// internal element specific methods
	static void UIUpdateMenuBarElement(int nID, _AtlUpdateUIData* pUIData, HMENU hMenu)
	{
		if((pUIData->m_wState & UPDUI_CLEARDEFAULT) != 0)
		{
			::SetMenuDefaultItem(hMenu, (UINT)-1, 0);
			pUIData->m_wState &= ~UPDUI_CLEARDEFAULT;
		}

		CMenuItemInfo mii;
		mii.fMask = MIIM_STATE;
		mii.wID = nID;

		if((pUIData->m_wState & UPDUI_DISABLED) != 0)
			mii.fState |= MFS_DISABLED | MFS_GRAYED;
		else
			mii.fState |= MFS_ENABLED;

		if((pUIData->m_wState & UPDUI_CHECKED) != 0)
			mii.fState |= MFS_CHECKED;
		else
			mii.fState |= MFS_UNCHECKED;

		if((pUIData->m_wState & UPDUI_DEFAULT) != 0)
			mii.fState |= MFS_DEFAULT;

		if((pUIData->m_wState & UPDUI_TEXT) != 0)
		{
			CMenuItemInfo miiNow;
			miiNow.fMask = MIIM_TYPE;
			miiNow.wID = nID;
			if(::GetMenuItemInfo(hMenu, nID, FALSE, &miiNow))
			{
				mii.fMask |= MIIM_TYPE;
				// MFT_BITMAP and MFT_SEPARATOR don't go together with MFT_STRING
				mii.fType |= (miiNow.fType & ~(MFT_BITMAP | MFT_SEPARATOR)) | MFT_STRING;
				mii.dwTypeData = pUIData->m_lpstrText;
			}
		}

		::SetMenuItemInfo(hMenu, nID, FALSE, &mii);
	}

	static void UIUpdateToolBarElement(int nID, _AtlUpdateUIData* pUIData, HWND hWndToolBar)
	{
		// Note: only handles enabled/disabled, checked state, and radio (press)
		::SendMessage(hWndToolBar, TB_ENABLEBUTTON, nID, (LPARAM)(pUIData->m_wState & UPDUI_DISABLED) ? FALSE : TRUE);
		::SendMessage(hWndToolBar, TB_CHECKBUTTON, nID, (LPARAM)(pUIData->m_wState & UPDUI_CHECKED) ? TRUE : FALSE);
		::SendMessage(hWndToolBar, TB_INDETERMINATE, nID, (LPARAM)(pUIData->m_wState & UPDUI_CHECKED2) ? TRUE : FALSE);
		::SendMessage(hWndToolBar, TB_PRESSBUTTON, nID, (LPARAM)(pUIData->m_wState & UPDUI_RADIO) ? TRUE : FALSE);
	}

	static void UIUpdateStatusBarElement(int nID, _AtlUpdateUIData* pUIData, HWND hWndStatusBar)
	{
		// Note: only handles text
		if(pUIData->m_wState & UPDUI_TEXT)
			::SendMessage(hWndStatusBar, SB_SETTEXT, nID, (LPARAM)pUIData->m_lpstrText);
	}

	static void UIUpdateChildWindow(int nID, _AtlUpdateUIData* pUIData, HWND hWnd)
	{
		HWND hChild = ::GetDlgItem(hWnd, nID);

		::EnableWindow(hChild, (pUIData->m_wState & UPDUI_DISABLED) ? FALSE : TRUE);
		// for check and radio, assume that window is a button
		int nCheck = BST_UNCHECKED;
		if((pUIData->m_wState & UPDUI_CHECKED) || (pUIData->m_wState & UPDUI_RADIO))
			nCheck = BST_CHECKED;
		else if(pUIData->m_wState & UPDUI_CHECKED2)
			nCheck = BST_INDETERMINATE;
		::SendMessage(hChild, BM_SETCHECK, nCheck, 0L);
		if(pUIData->m_wState & UPDUI_DEFAULT)
		{
			DWORD dwRet = (DWORD)::SendMessage(hWnd, DM_GETDEFID, 0, 0L);
			if(HIWORD(dwRet) == DC_HASDEFID)
			{
				HWND hOldDef = ::GetDlgItem(hWnd, (int)(short)LOWORD(dwRet));
				// remove BS_DEFPUSHBUTTON
				::SendMessage(hOldDef, BM_SETSTYLE, BS_PUSHBUTTON, MAKELPARAM(TRUE, 0));
			}
			::SendMessage(hWnd, DM_SETDEFID, nID, 0L);
		}
		if(pUIData->m_wState & UPDUI_TEXT)
			::SetWindowText(hChild, pUIData->m_lpstrText);
	}
};

template <class T>
class CUpdateUI : public CUpdateUIBase
{
public:
	CUpdateUI()
	{
		T* pT = static_cast<T*>(this);
		(void)pT;   // avoid level 4 warning
		const _AtlUpdateUIMap* pMap = pT->GetUpdateUIMap();
		m_pUIMap = pMap;
		ATLASSERT(m_pUIMap != NULL);
		int nCount = 1;
		for( ; pMap->m_nID != (WORD)-1; nCount++)
			pMap++;

		// check for duplicates (debug only)
#ifdef _DEBUG
		for(int i = 0; i < nCount; i++)
		{
			for(int j = 0; j < nCount; j++)
			{
				// shouldn't have duplicates in the update UI map
				if(i != j)
					ATLASSERT(m_pUIMap[j].m_nID != m_pUIMap[i].m_nID);
			}
		}
#endif // _DEBUG

		ATLTRY(m_pUIData = new _AtlUpdateUIData[nCount]);
		ATLASSERT(m_pUIData != NULL);

		if(m_pUIData != NULL)
			memset(m_pUIData, 0, sizeof(_AtlUpdateUIData) * nCount);
	}
};


///////////////////////////////////////////////////////////////////////////////
// CDynamicUpdateUI - allows update elements to dynamically added and removed
//                    in addition to a static update UI map

template <class T>
class CDynamicUpdateUI : public CUpdateUIBase
{
public:
// Data members
	ATL::CSimpleArray<_AtlUpdateUIMap> m_arrUIMap;     // copy of the static UI data
	ATL::CSimpleArray<_AtlUpdateUIData> m_arrUIData;   // instance UI data

// Constructor/destructor
	CDynamicUpdateUI()
	{
		T* pT = static_cast<T*>(this);
		(void)pT;   // avoid level 4 warning
		const _AtlUpdateUIMap* pMap = pT->GetUpdateUIMap();
		ATLASSERT(pMap != NULL);

		for(;;)
		{
			BOOL bRet = m_arrUIMap.Add(*(_AtlUpdateUIMap*)pMap);
			ATLASSERT(bRet);

			if(bRet != FALSE)
			{
				_AtlUpdateUIData data = { 0, NULL };
				bRet = m_arrUIData.Add(data);
				ATLASSERT(bRet);
			}

			if(pMap->m_nID == (WORD)-1)
				break;

			pMap++;
		}

		ATLASSERT(m_arrUIMap.GetSize() == m_arrUIData.GetSize());

#ifdef _DEBUG
		// check for duplicates (debug only)
		for(int i = 0; i < m_arrUIMap.GetSize(); i++)
		{
			for(int j = 0; j < m_arrUIMap.GetSize(); j++)
			{
				// shouldn't have duplicates in the update UI map
				if(i != j)
					ATLASSERT(m_arrUIMap[j].m_nID != m_arrUIMap[i].m_nID);
			}
		}
#endif // _DEBUG

		// Set internal data pointers to point to the new data arrays
		m_pUIMap = m_arrUIMap.m_aT;
		m_pUIData = m_arrUIData.m_aT;
	}

	~CDynamicUpdateUI()
	{
		for(int i = 0; i < m_arrUIData.GetSize(); i++)
		{
			if((m_arrUIData[i].m_wState & UPDUI_TEXT) != 0)
				delete [] m_arrUIData[i].m_lpstrText;
		}

		// Reset internal data pointers (memory will be released by CSimpleArray d-tor)
		m_pUIMap = NULL;
		m_pUIData = NULL;
	}

// Methods for dynamically adding and removing update elements
	bool UIAddUpdateElement(WORD nID, WORD wType)
	{
		// check for duplicates
		for(int i = 0; i < m_arrUIMap.GetSize(); i++)
		{
			// shouldn't have duplicates in the update UI map
			ATLASSERT(m_arrUIMap[i].m_nID != nID);
			if(m_arrUIMap[i].m_nID == nID)
				return false;
		}

		bool bRetVal = false;

		// Add new end element
		_AtlUpdateUIMap uumEnd = { (WORD)-1, 0 };
		BOOL bRet = m_arrUIMap.Add(uumEnd);
		ATLASSERT(bRet);

		if(bRet != FALSE)
		{
			_AtlUpdateUIData uud = { 0, NULL };
			bRet = m_arrUIData.Add(uud);
			ATLASSERT(bRet);

			// Set new data to the previous end element
			if(bRet != FALSE)
			{
				int nSize = m_arrUIMap.GetSize();
				_AtlUpdateUIMap uum = { nID, wType };
				m_arrUIMap.SetAtIndex(nSize - 2, uum);
				m_arrUIData.SetAtIndex(nSize - 2, uud);

				// Set internal data pointers again, just in case that memory moved
				m_pUIMap = m_arrUIMap.m_aT;
				m_pUIData = m_arrUIData.m_aT;

				bRetVal = true;
			}
		}

		return bRetVal;
	}

	bool UIRemoveUpdateElement(WORD nID)
	{
		bool bRetVal = false;

		for(int i = 0; i < m_arrUIMap.GetSize(); i++)
		{
			if(m_arrUIMap[i].m_nID == nID)
			{
				if((m_arrUIData[i].m_wState & UPDUI_TEXT) != 0)
					delete [] m_arrUIData[i].m_lpstrText;

				BOOL bRet = m_arrUIMap.RemoveAt(i);
				ATLASSERT(bRet);
				bRet = m_arrUIData.RemoveAt(i);
				ATLASSERT(bRet);

				bRetVal = true;
				break;
			}
		}

		return bRetVal;
	}
};


///////////////////////////////////////////////////////////////////////////////
// CAutoUpdateUI : Automatic mapping of UI elements

template <class T>
class CAutoUpdateUI : public CDynamicUpdateUI<T>
{
public:
	LPCTSTR UIGetText(int nID)
	{
		for(int i = 0; i < this->m_arrUIMap.GetSize(); i++)
		{
			if(this->m_arrUIMap[i].m_nID == nID)
 				return this->m_arrUIData[i].m_lpstrText;
		}

		return NULL;
	}

// Element
	template <WORD t_wType>
	bool UIAddElement(UINT nID)
	{
		// check for existing UI map element
		for(int i = 0; i < this->m_arrUIMap.GetSize(); i++)
		{
			if(this->m_arrUIMap[i].m_nID == nID)
			{
				// set requested type
				this->m_arrUIMap[i].m_wType |= t_wType;
				return true;
			}
		}

		// Add element to UI map with requested type
		return this->UIAddUpdateElement((WORD)nID, t_wType);
	}

	template <WORD t_wType>
	bool UIRemoveElement(UINT nID)
	{
		for(int i = 0; i < this->m_arrUIMap.GetSize(); i++)
		{
			if(this->m_arrUIMap[i].m_nID == nID) // matching UI map element
			{
				WORD wType = this->m_arrUIMap[i].m_wType & ~t_wType;
				if (wType != 0)   // has other types 
				{
					this->m_arrUIMap[i].m_wType = wType; // keep other types
					return true;
				}
				else
				{
					return this->UIRemoveUpdateElement((WORD)nID);
				}
			}
		}

		return false;
	}

// Menu
	bool UIAddMenu(HMENU hMenu, bool bSetText = false)
	{
		ATLASSERT(::IsMenu(hMenu));
		MENUITEMINFO mii = {sizeof(MENUITEMINFO), MIIM_TYPE | MIIM_ID | MIIM_SUBMENU};

		// Complete the UI map
		for (INT uItem = 0; CMenuHandle(hMenu).GetMenuItemInfo(uItem, TRUE, &mii); uItem++)
		{
			if(mii.hSubMenu)
			{
				// Add submenu to UI map
				UIAddMenu(mii.hSubMenu, bSetText);
			}
			else if (mii.wID != 0)
			{
				// Add element to UI map
				UIAddElement<CDynamicUpdateUI<T>::UPDUI_MENUPOPUP>(mii.wID);
				if (bSetText)
				{
					TCHAR sText[64] = {};
					if (GetMenuString(hMenu, uItem, sText, 64, MF_BYPOSITION))
						this->UISetText(mii.wID, sText);
				}
			}
		}

		return true;
	}

	bool UIAddMenu(UINT uID, bool bSetText = false)
	{
		CMenu menu;
		ATLVERIFY(menu.LoadMenu(uID));
		return UIAddMenu(menu, bSetText);
	}

// ToolBar
	bool UIAddToolBar(HWND hWndToolBar)
	{
		ATLASSERT(::IsWindow(hWndToolBar));
		TBBUTTONINFO tbbi = { sizeof(TBBUTTONINFO), TBIF_COMMAND | TBIF_STYLE | TBIF_BYINDEX };

		// Add toolbar buttons
		for (int uItem = 0; ::SendMessage(hWndToolBar, TB_GETBUTTONINFO, uItem, (LPARAM)&tbbi) != -1; uItem++)
		{
			if (tbbi.fsStyle ^ BTNS_SEP)
				UIAddElement<CDynamicUpdateUI<T>::UPDUI_TOOLBAR>(tbbi.idCommand);
		}

		// Add embedded controls if any
		if (::GetWindow(hWndToolBar, GW_CHILD))
			UIAddChildWindowContainer(hWndToolBar);

		return (CUpdateUIBase::UIAddToolBar(hWndToolBar) != FALSE);
	}

// Container
	bool UIAddChildWindowContainer(HWND hWnd)
	{
		ATLASSERT(::IsWindow(hWnd));

		// Add children controls if any
		for (ATL::CWindow wCtl = ::GetWindow(hWnd, GW_CHILD); wCtl.IsWindow(); wCtl = wCtl.GetWindow(GW_HWNDNEXT))
		{
			int id = wCtl.GetDlgCtrlID();
			if(id != 0)
				UIAddElement<CDynamicUpdateUI<T>::UPDUI_CHILDWINDOW>(id);
		}

		return (CUpdateUIBase::UIAddChildWindowContainer(hWnd) != FALSE);
	}

// StatusBar
	BOOL UIUpdateStatusBar(BOOL bForceUpdate = FALSE)
	{
		if(!(this->m_wDirtyType & CDynamicUpdateUI<T>::UPDUI_STATUSBAR) && !bForceUpdate)
			return TRUE;

		for(int i = 0; i < this->m_arrUIMap.GetSize(); i++)
		{
			for(int e = 0; e < this->m_UIElements.GetSize(); e++)
			{
				if((this->m_UIElements[e].m_wType == CDynamicUpdateUI<T>::UPDUI_STATUSBAR) && 
				   (this->m_arrUIMap[i].m_wType & CDynamicUpdateUI<T>::UPDUI_STATUSBAR) && 
				   (this->m_arrUIData[i].m_wState & CDynamicUpdateUI<T>::UPDUI_STATUSBAR))
				{
					this->UIUpdateStatusBarElement(this->m_arrUIMap[i].m_nID, &this->m_arrUIData[i], this->m_UIElements[e].m_hWnd);
					this->m_arrUIData[i].m_wState &= ~CDynamicUpdateUI<T>::UPDUI_STATUSBAR;
					if(this->m_arrUIData[i].m_wState & CDynamicUpdateUI<T>::UPDUI_TEXT)
						this->m_arrUIData[i].m_wState &= ~CDynamicUpdateUI<T>::UPDUI_TEXT;
				}
			}
		}

		this->m_wDirtyType &= ~CDynamicUpdateUI<T>::UPDUI_STATUSBAR;
		return TRUE;
	}

	bool UIAddStatusBar(HWND hWndStatusBar, INT nPanes = 1)
	{
		ATLASSERT(::IsWindow(hWndStatusBar));

		// Add StatusBar panes
		for (int iPane = 0; iPane < nPanes; iPane++)
			UIAddElement<CDynamicUpdateUI<T>::UPDUI_STATUSBAR>(ID_DEFAULT_PANE + iPane);

		return (CUpdateUIBase::UIAddStatusBar(hWndStatusBar) != FALSE);
	}

// UI Map used if derived class has none
	BEGIN_UPDATE_UI_MAP(CAutoUpdateUI)
	END_UPDATE_UI_MAP()
};


///////////////////////////////////////////////////////////////////////////////
// CDialogResize - provides support for resizing dialog controls
//                 (works for any window that has child controls)

// Put CDialogResize in the list of base classes for a dialog (or even plain window),
// then implement DLGRESIZE map by specifying controls and groups of control
// and using DLSZ_* values to specify how are they supposed to be resized.
//
// Notes:
// - Resizeable border (WS_THICKFRAME style) should be set in the dialog template
//   for top level dialogs (popup or overlapped), so that users can resize the dialog.
// - Some flags cannot be combined; for instance DLSZ_CENTER_X overrides DLSZ_SIZE_X,
//   DLSZ_SIZE_X overrides DLSZ_MOVE_X. X and Y flags can be combined.
// - Order of controls is important - group controls are resized and moved based
//   on the position of the previous control in a group.

// dialog resize map macros
struct _AtlDlgResizeMap
{
	int m_nCtlID;
	DWORD m_dwResizeFlags;
};

#define BEGIN_DLGRESIZE_MAP(thisClass) \
	static const WTL::_AtlDlgResizeMap* GetDlgResizeMap() \
	{ \
		static const WTL::_AtlDlgResizeMap theMap[] = \
		{

#define END_DLGRESIZE_MAP() \
			{ -1, 0 }, \
		}; \
		return theMap; \
	}

#define DLGRESIZE_CONTROL(id, flags) \
		{ id, flags },

#define BEGIN_DLGRESIZE_GROUP() \
		{ -1, _DLSZ_BEGIN_GROUP },

#define END_DLGRESIZE_GROUP() \
		{ -1, _DLSZ_END_GROUP },


template <class T>
class CDialogResize
{
public:
// Data declarations and members
	enum
	{
		DLSZ_SIZE_X		= 0x00000001,
		DLSZ_SIZE_Y		= 0x00000002,
		DLSZ_MOVE_X		= 0x00000004,
		DLSZ_MOVE_Y		= 0x00000008,
		DLSZ_REPAINT		= 0x00000010,
		DLSZ_CENTER_X		= 0x00000020,
		DLSZ_CENTER_Y		= 0x00000040,

		// internal use only
		_DLSZ_BEGIN_GROUP	= 0x00001000,
		_DLSZ_END_GROUP		= 0x00002000,
		_DLSZ_GRIPPER		= 0x00004000
	};

	struct _AtlDlgResizeData
	{
		int m_nCtlID;
		DWORD m_dwResizeFlags;
		RECT m_rect;

		int GetGroupCount() const
		{
			return (int)LOBYTE(HIWORD(m_dwResizeFlags));
		}

		void SetGroupCount(int nCount)
		{
			ATLASSERT((nCount > 0) && (nCount < 256));
			DWORD dwCount = (DWORD)MAKELONG(0, MAKEWORD(nCount, 0));
			m_dwResizeFlags &= 0xFF00FFFF;
			m_dwResizeFlags |= dwCount;
		}

		bool operator ==(const _AtlDlgResizeData& r) const
		{ return ((m_nCtlID == r.m_nCtlID) && (m_dwResizeFlags == r.m_dwResizeFlags)); }
	};

	ATL::CSimpleArray<_AtlDlgResizeData> m_arrData;
	SIZE m_sizeDialog;
	POINT m_ptMinTrackSize;
	bool m_bGripper;


// Constructor
	CDialogResize() : m_bGripper(false)
	{
		m_sizeDialog.cx = 0;
		m_sizeDialog.cy = 0;
		m_ptMinTrackSize.x = -1;
		m_ptMinTrackSize.y = -1;
	}

// Operations
	void DlgResize_Init(bool bAddGripper = true, bool bUseMinTrackSize = true, DWORD dwForceStyle = WS_CLIPCHILDREN)
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));

		DWORD dwStyle = pT->GetStyle();

#ifdef _DEBUG
		// Debug only: Check if top level dialogs have a resizeable border.
		if(((dwStyle & WS_CHILD) == 0) && ((dwStyle & WS_THICKFRAME) == 0))
			ATLTRACE2(atlTraceUI, 0, _T("DlgResize_Init - warning: top level dialog without the WS_THICKFRAME style - user cannot resize it\n"));
#endif // _DEBUG

		// Force specified styles (default WS_CLIPCHILDREN reduces flicker)
		if((dwStyle & dwForceStyle) != dwForceStyle)
			pT->ModifyStyle(0, dwForceStyle);

		// Adding this style removes an empty icon that dialogs with WS_THICKFRAME have.
		// Setting icon to NULL is required when XP themes are active.
		// Note: This will not prevent adding an icon for the dialog using SetIcon()
		if((dwStyle & WS_CHILD) == 0)
		{
			pT->ModifyStyleEx(0, WS_EX_DLGMODALFRAME);
			if(pT->GetIcon(FALSE) == NULL)
				pT->SetIcon(NULL, FALSE);
		}

		// Cleanup in case of multiple initialization
		// block: first check for the gripper control, destroy it if needed
		{
			ATL::CWindow wndGripper = pT->GetDlgItem(ATL_IDW_STATUS_BAR);
			if(wndGripper.IsWindow() && (m_arrData.GetSize() > 0) && (m_arrData[0].m_dwResizeFlags & _DLSZ_GRIPPER) != 0)
				wndGripper.DestroyWindow();
		}
		// clear out everything else
		m_arrData.RemoveAll();
		m_sizeDialog.cx = 0;
		m_sizeDialog.cy = 0;
		m_ptMinTrackSize.x = -1;
		m_ptMinTrackSize.y = -1;

		// Get initial dialog client size
		RECT rectDlg = {};
		pT->GetClientRect(&rectDlg);
		m_sizeDialog.cx = rectDlg.right;
		m_sizeDialog.cy = rectDlg.bottom;

		// Create gripper if requested
		m_bGripper = false;
		if(bAddGripper)
		{
			// shouldn't exist already
			ATLASSERT(!pT->GetDlgItem(ATL_IDW_STATUS_BAR).IsWindow());
			if(!pT->GetDlgItem(ATL_IDW_STATUS_BAR).IsWindow())
			{
				ATL::CWindow wndGripper;
				wndGripper.Create(_T("SCROLLBAR"), pT->m_hWnd, rectDlg, NULL, WS_CHILD | WS_VISIBLE | WS_CLIPSIBLINGS | SBS_SIZEBOX | SBS_SIZEGRIP | SBS_SIZEBOXBOTTOMRIGHTALIGN, 0, ATL_IDW_STATUS_BAR);
				ATLASSERT(wndGripper.IsWindow());
				if(wndGripper.IsWindow())
				{
					m_bGripper = true;
					RECT rectCtl = {};
					wndGripper.GetWindowRect(&rectCtl);
					::MapWindowPoints(NULL, pT->m_hWnd, (LPPOINT)&rectCtl, 2);
					_AtlDlgResizeData data = { ATL_IDW_STATUS_BAR, DLSZ_MOVE_X | DLSZ_MOVE_Y | DLSZ_REPAINT | _DLSZ_GRIPPER, { rectCtl.left, rectCtl.top, rectCtl.right, rectCtl.bottom } };
					m_arrData.Add(data);
				}
			}
		}

		// Get min track position if requested
		if(bUseMinTrackSize)
		{
			if((dwStyle & WS_CHILD) != 0)
			{
				RECT rect = {};
				pT->GetClientRect(&rect);
				m_ptMinTrackSize.x = rect.right - rect.left;
				m_ptMinTrackSize.y = rect.bottom - rect.top;
			}
			else
			{
				RECT rect = {};
				pT->GetWindowRect(&rect);
				m_ptMinTrackSize.x = rect.right - rect.left;
				m_ptMinTrackSize.y = rect.bottom - rect.top;
			}
		}

		// Walk the map and initialize data
		const _AtlDlgResizeMap* pMap = pT->GetDlgResizeMap();
		ATLASSERT(pMap != NULL);
		int nGroupStart = -1;
		for(int nCount = 1; !((pMap->m_nCtlID == -1) && (pMap->m_dwResizeFlags == 0)); nCount++, pMap++)
		{
			if(pMap->m_nCtlID == -1)
			{
				switch(pMap->m_dwResizeFlags)
				{
				case _DLSZ_BEGIN_GROUP:
					ATLASSERT(nGroupStart == -1);
					nGroupStart = m_arrData.GetSize();
					break;
				case _DLSZ_END_GROUP:
					{
						ATLASSERT(nGroupStart != -1);
						int nGroupCount = m_arrData.GetSize() - nGroupStart;
						m_arrData[nGroupStart].SetGroupCount(nGroupCount);
						nGroupStart = -1;
					}
					break;
				default:
					ATLASSERT(FALSE && _T("Invalid DLGRESIZE Map Entry"));
					break;
				}
			}
			else
			{
				// this ID conflicts with the default gripper one
				ATLASSERT(m_bGripper ? (pMap->m_nCtlID != ATL_IDW_STATUS_BAR) : TRUE);

				ATL::CWindow ctl = pT->GetDlgItem(pMap->m_nCtlID);
				ATLASSERT(ctl.IsWindow());
				RECT rectCtl = {};
				ctl.GetWindowRect(&rectCtl);
				::MapWindowPoints(NULL, pT->m_hWnd, (LPPOINT)&rectCtl, 2);

				DWORD dwGroupFlag = ((nGroupStart != -1) && (m_arrData.GetSize() == nGroupStart)) ? _DLSZ_BEGIN_GROUP : 0;
				_AtlDlgResizeData data = { pMap->m_nCtlID, pMap->m_dwResizeFlags | dwGroupFlag, { rectCtl.left, rectCtl.top, rectCtl.right, rectCtl.bottom } };
				m_arrData.Add(data);
			}
		}
		ATLASSERT((nGroupStart == -1) && _T("No End Group Entry in the DLGRESIZE Map"));
	}

	void DlgResize_UpdateLayout(int cxWidth, int cyHeight)
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));

		// Restrict minimum size if requested
		if(((pT->GetStyle() & WS_CHILD) != 0) && (m_ptMinTrackSize.x != -1) && (m_ptMinTrackSize.y != -1))
		{
			if(cxWidth < m_ptMinTrackSize.x)
				cxWidth = m_ptMinTrackSize.x;
			if(cyHeight < m_ptMinTrackSize.y)
				cyHeight = m_ptMinTrackSize.y;
		}

		BOOL bVisible = pT->IsWindowVisible();
		if(bVisible)
			pT->SetRedraw(FALSE);

		for(int i = 0; i < m_arrData.GetSize(); i++)
		{
			if((m_arrData[i].m_dwResizeFlags & _DLSZ_BEGIN_GROUP) != 0)   // start of a group
			{
				int nGroupCount = m_arrData[i].GetGroupCount();
				ATLASSERT((nGroupCount > 0) && ((i + nGroupCount - 1) < m_arrData.GetSize()));
				RECT rectGroup = m_arrData[i].m_rect;

				int j = 1;
				for(j = 1; j < nGroupCount; j++)
				{
					rectGroup.left = __min(rectGroup.left, m_arrData[i + j].m_rect.left);
					rectGroup.top = __min(rectGroup.top, m_arrData[i + j].m_rect.top);
					rectGroup.right = __max(rectGroup.right, m_arrData[i + j].m_rect.right);
					rectGroup.bottom = __max(rectGroup.bottom, m_arrData[i + j].m_rect.bottom);
				}

				for(j = 0; j < nGroupCount; j++)
				{
					_AtlDlgResizeData* pDataPrev = NULL;
					if(j > 0)
						pDataPrev = &(m_arrData[i + j - 1]);
					pT->DlgResize_PositionControl(cxWidth, cyHeight, rectGroup, m_arrData[i + j], true, pDataPrev);
				}

				i += nGroupCount - 1;   // increment to skip all group controls
			}
			else // one control entry
			{
				RECT rectGroup = {};
				pT->DlgResize_PositionControl(cxWidth, cyHeight, rectGroup, m_arrData[i], false);
			}
		}

		if(bVisible)
			pT->SetRedraw(TRUE);

		pT->RedrawWindow(NULL, NULL, RDW_ERASE | RDW_INVALIDATE | RDW_UPDATENOW | RDW_ALLCHILDREN);
	}

// Message map and handlers
	BEGIN_MSG_MAP(CDialogResize)
		MESSAGE_HANDLER(WM_SIZE, OnSize)
		MESSAGE_HANDLER(WM_GETMINMAXINFO, OnGetMinMaxInfo)
	END_MSG_MAP()

	LRESULT OnSize(UINT /*uMsg*/, WPARAM wParam, LPARAM lParam, BOOL& /*bHandled*/)
	{
		T* pT = static_cast<T*>(this);
		if(m_bGripper)
		{
			ATL::CWindow wndGripper = pT->GetDlgItem(ATL_IDW_STATUS_BAR);
			if(wParam == SIZE_MAXIMIZED)
				wndGripper.ShowWindow(SW_HIDE);
			else if(wParam == SIZE_RESTORED)
				wndGripper.ShowWindow(SW_SHOW);
		}
		if(wParam != SIZE_MINIMIZED)
		{
			ATLASSERT(::IsWindow(pT->m_hWnd));
			pT->DlgResize_UpdateLayout(GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam));
		}
		return 0;
	}

	LRESULT OnGetMinMaxInfo(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM lParam, BOOL& /*bHandled*/)
	{
		if((m_ptMinTrackSize.x != -1) && (m_ptMinTrackSize.y != -1))
		{
			LPMINMAXINFO lpMMI = (LPMINMAXINFO)lParam;
			lpMMI->ptMinTrackSize =  m_ptMinTrackSize;
		}
		return 0;
	}

// Implementation
	bool DlgResize_PositionControl(int cxWidth, int cyHeight, RECT& rectGroup, _AtlDlgResizeData& data, bool bGroup, 
	                               _AtlDlgResizeData* pDataPrev = NULL)
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));
		ATL::CWindow ctl;
		RECT rectCtl = {};

		ctl = pT->GetDlgItem(data.m_nCtlID);
		if(!ctl.GetWindowRect(&rectCtl))
			return false;
		::MapWindowPoints(NULL, pT->m_hWnd, (LPPOINT)&rectCtl, 2);

		if(bGroup)
		{
			if((data.m_dwResizeFlags & DLSZ_CENTER_X) != 0)
			{
				int cxRight = rectGroup.right + cxWidth - m_sizeDialog.cx;
				int cxCtl = data.m_rect.right - data.m_rect.left;
				rectCtl.left = rectGroup.left + (cxRight - rectGroup.left - cxCtl) / 2;
				rectCtl.right = rectCtl.left + cxCtl;
			}
			else if((data.m_dwResizeFlags & (DLSZ_SIZE_X | DLSZ_MOVE_X)) != 0)
			{
				rectCtl.left = rectGroup.left + ::MulDiv(data.m_rect.left - rectGroup.left, rectGroup.right - rectGroup.left + (cxWidth - m_sizeDialog.cx), rectGroup.right - rectGroup.left);

				if((data.m_dwResizeFlags & DLSZ_SIZE_X) != 0)
				{
					rectCtl.right = rectGroup.left + ::MulDiv(data.m_rect.right - rectGroup.left, rectGroup.right - rectGroup.left + (cxWidth - m_sizeDialog.cx), rectGroup.right - rectGroup.left);

					if(pDataPrev != NULL)
					{
						ATL::CWindow ctlPrev = pT->GetDlgItem(pDataPrev->m_nCtlID);
						RECT rcPrev = {};
						ctlPrev.GetWindowRect(&rcPrev);
						::MapWindowPoints(NULL, pT->m_hWnd, (LPPOINT)&rcPrev, 2);
						int dxAdjust = (rectCtl.left - rcPrev.right) - (data.m_rect.left - pDataPrev->m_rect.right);
						rcPrev.right += dxAdjust;
						ctlPrev.SetWindowPos(NULL, &rcPrev, SWP_NOZORDER | SWP_NOACTIVATE | SWP_NOMOVE);
					}
				}
				else
				{
					rectCtl.right = rectCtl.left + (data.m_rect.right - data.m_rect.left);
				}
			}

			if((data.m_dwResizeFlags & DLSZ_CENTER_Y) != 0)
			{
				int cyBottom = rectGroup.bottom + cyHeight - m_sizeDialog.cy;
				int cyCtl = data.m_rect.bottom - data.m_rect.top;
				rectCtl.top = rectGroup.top + (cyBottom - rectGroup.top - cyCtl) / 2;
				rectCtl.bottom = rectCtl.top + cyCtl;
			}
			else if((data.m_dwResizeFlags & (DLSZ_SIZE_Y | DLSZ_MOVE_Y)) != 0)
			{
				rectCtl.top = rectGroup.top + ::MulDiv(data.m_rect.top - rectGroup.top, rectGroup.bottom - rectGroup.top + (cyHeight - m_sizeDialog.cy), rectGroup.bottom - rectGroup.top);

				if((data.m_dwResizeFlags & DLSZ_SIZE_Y) != 0)
				{
					rectCtl.bottom = rectGroup.top + ::MulDiv(data.m_rect.bottom - rectGroup.top, rectGroup.bottom - rectGroup.top + (cyHeight - m_sizeDialog.cy), rectGroup.bottom - rectGroup.top);

					if(pDataPrev != NULL)
					{
						ATL::CWindow ctlPrev = pT->GetDlgItem(pDataPrev->m_nCtlID);
						RECT rcPrev = {};
						ctlPrev.GetWindowRect(&rcPrev);
						::MapWindowPoints(NULL, pT->m_hWnd, (LPPOINT)&rcPrev, 2);
						int dxAdjust = (rectCtl.top - rcPrev.bottom) - (data.m_rect.top - pDataPrev->m_rect.bottom);
						rcPrev.bottom += dxAdjust;
						ctlPrev.SetWindowPos(NULL, &rcPrev, SWP_NOZORDER | SWP_NOACTIVATE | SWP_NOMOVE);
					}
				}
				else
				{
					rectCtl.bottom = rectCtl.top + (data.m_rect.bottom - data.m_rect.top);
				}
			}
		}
		else // no group
		{
			if((data.m_dwResizeFlags & DLSZ_CENTER_X) != 0)
			{
				int cxCtl = data.m_rect.right - data.m_rect.left;
				rectCtl.left = (cxWidth - cxCtl) / 2;
				rectCtl.right = rectCtl.left + cxCtl;
			}
			else if((data.m_dwResizeFlags & (DLSZ_SIZE_X | DLSZ_MOVE_X)) != 0)
			{
				rectCtl.right = data.m_rect.right + (cxWidth - m_sizeDialog.cx);

				if((data.m_dwResizeFlags & DLSZ_MOVE_X) != 0)
					rectCtl.left = rectCtl.right - (data.m_rect.right - data.m_rect.left);
			}

			if((data.m_dwResizeFlags & DLSZ_CENTER_Y) != 0)
			{
				int cyCtl = data.m_rect.bottom - data.m_rect.top;
				rectCtl.top = (cyHeight - cyCtl) / 2;
				rectCtl.bottom = rectCtl.top + cyCtl;
			}
			else if((data.m_dwResizeFlags & (DLSZ_SIZE_Y | DLSZ_MOVE_Y)) != 0)
			{
				rectCtl.bottom = data.m_rect.bottom + (cyHeight - m_sizeDialog.cy);

				if((data.m_dwResizeFlags & DLSZ_MOVE_Y) != 0)
					rectCtl.top = rectCtl.bottom - (data.m_rect.bottom - data.m_rect.top);
			}
		}

		if((data.m_dwResizeFlags & DLSZ_REPAINT) != 0)
			ctl.Invalidate();

		if((data.m_dwResizeFlags & (DLSZ_SIZE_X | DLSZ_SIZE_Y | DLSZ_MOVE_X | DLSZ_MOVE_Y | DLSZ_REPAINT | DLSZ_CENTER_X | DLSZ_CENTER_Y)) != 0)
			ctl.SetWindowPos(NULL, &rectCtl, SWP_NOZORDER | SWP_NOACTIVATE);

		return true;
	}
};


///////////////////////////////////////////////////////////////////////////////
// CDynamicDialogLayout - support for dialog dynamic layout resource info
//                        (AFX_DIALOG_LAYOUT) in VS2015 and higher

#if (_MSC_VER >= 1900)

template<class T>
class CDynamicDialogLayout
{
public:
// Data declarations
	struct _AtlDynamicLayoutData
	{
		HWND m_hWnd;
		char m_nMoveRatioX;
		char m_nMoveRatioY;
		char m_nSizeRatioX;
		char m_nSizeRatioY;
		RECT m_rcInit;
	};

// Data members
	ATL::CSimpleArray<_AtlDynamicLayoutData> m_arrLayoutData;
	SIZE m_szParentInit;
	POINT m_ptMinTrackSize;
	bool m_bGripper;

// Constructor
	CDynamicDialogLayout() : m_bGripper(false)
	{
		m_szParentInit.cx = 0;
		m_szParentInit.cy = 0;
		m_ptMinTrackSize.x = -1;
		m_ptMinTrackSize.y = -1;
	}

// Methods
	void InitDynamicLayout(bool bAddGripper = true, bool bMinTrackSize = true)
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));

		// Cleanup in case of multiple initialization
		// block: first check for the gripper control, destroy it if needed
		{
			ATL::CWindow wndGripper = pT->GetDlgItem(ATL_IDW_STATUS_BAR);
			if(wndGripper.IsWindow() != FALSE)
				wndGripper.DestroyWindow();
		}
		// clear out everything else
		m_arrLayoutData.RemoveAll();
		m_ptMinTrackSize.x = -1;
		m_ptMinTrackSize.y = -1;
		m_szParentInit.cx = 0;
		m_szParentInit.cy = 0;
		m_bGripper = false;

		CResource rcLayout;
		if(rcLayout.Load(_T("AFX_DIALOG_LAYOUT"), pT->IDD))
		{
			int nCount = rcLayout.GetSize() / sizeof(WORD);
			if(nCount > 1)
			{
				RECT rcParent = {};
				pT->GetWindowRect(&rcParent);
				m_szParentInit.cx = rcParent.right - rcParent.left;
				m_szParentInit.cy = rcParent.bottom - rcParent.top;

				WORD* pnData = (WORD*)rcLayout.Lock();
				WORD wVersion = *pnData;   // AFX_DIALOG_LAYOUT version
				ATLASSERT(wVersion == 0);
				if(wVersion == 0)
				{
					pnData++;   // skip version
					ATL::CWindow wndChild = pT->GetWindow(GW_CHILD);
					for(int i = 0; (i < nCount) && (wndChild.m_hWnd != NULL); i += 4)
					{
						char nMoveRatioX = _GetDataPct(pnData[i]);
						char nMoveRatioY = _GetDataPct(pnData[i + 1]);
						char nSizeRatioX = _GetDataPct(pnData[i + 2]);
						char nSizeRatioY = _GetDataPct(pnData[i + 3]);

						bool bValid = ((nMoveRatioX != -1) && (nMoveRatioY != -1) && (nSizeRatioX != -1) && (nSizeRatioY != -1));
						bool bAction = ((nMoveRatioX != 0) || (nMoveRatioY != 0) || (nSizeRatioX != 0) || (nSizeRatioY != 0));
						if(bValid && bAction)
						{
							_AtlDynamicLayoutData LayoutData = { wndChild.m_hWnd, nMoveRatioX, nMoveRatioY, nSizeRatioX, nSizeRatioY };
							wndChild.GetWindowRect(&LayoutData.m_rcInit);
							pT->ScreenToClient(&LayoutData.m_rcInit);
							m_arrLayoutData.Add(LayoutData);
						}

						wndChild = wndChild.GetWindow(GW_HWNDNEXT);
					}
				}

				rcLayout.Release();
			}
		}

		if(bAddGripper)
		{
			RECT rcDialog = {};
			pT->GetClientRect(&rcDialog);

			ATL::CWindow wndGripper = pT->GetDlgItem(ATL_IDW_STATUS_BAR);
			if(wndGripper.m_hWnd != NULL)
				wndGripper.DestroyWindow();

			wndGripper.Create(_T("SCROLLBAR"), pT->m_hWnd, rcDialog, NULL, WS_CHILD | WS_VISIBLE | WS_CLIPSIBLINGS | SBS_SIZEBOX | SBS_SIZEGRIP | SBS_SIZEBOXBOTTOMRIGHTALIGN, 0, ATL_IDW_STATUS_BAR);
			ATLASSERT(wndGripper.m_hWnd != NULL);
			if(wndGripper.m_hWnd != NULL)
			{
				m_bGripper = true;
				_AtlDynamicLayoutData LayoutData = { wndGripper.m_hWnd, 100, 100, 0, 0 };
				wndGripper.GetWindowRect(&LayoutData.m_rcInit);
				pT->ScreenToClient(&LayoutData.m_rcInit);
				m_arrLayoutData.Add(LayoutData);
			}
		}

		if(bMinTrackSize)
		{
			RECT rcMinTrack = {};
			if((pT->GetStyle() & WS_CHILD) != 0)
				pT->GetClientRect(&rcMinTrack);
			else
				pT->GetWindowRect(&rcMinTrack);

			m_ptMinTrackSize.x = rcMinTrack.right - rcMinTrack.left;
			m_ptMinTrackSize.y = rcMinTrack.bottom - rcMinTrack.top;
		}
	}

	void UpdateDynamicLayout()
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));

		int nCount = m_arrLayoutData.GetSize();
		if(nCount == 0)
			return;

		RECT rcParent = {};
		pT->GetWindowRect(&rcParent);

		int nDeltaWidth = rcParent.right - rcParent.left - m_szParentInit.cx;
		int nDeltaHeight = rcParent.bottom - rcParent.top - m_szParentInit.cy;

		HDWP hDwp = ::BeginDeferWindowPos(nCount);

		for(int i = 0; i < nCount; i++)
		{
			_AtlDynamicLayoutData& LayoutData = m_arrLayoutData[i];

			ATLASSERT(::IsWindow(LayoutData.m_hWnd) != FALSE);

			int nID = ::GetDlgCtrlID(LayoutData.m_hWnd);
			UINT nFlags = (SWP_NOMOVE | SWP_NOSIZE);
			RECT rcItem = LayoutData.m_rcInit;

			if(((nID == ATL_IDW_STATUS_BAR) || (nDeltaWidth >= 0)) && (LayoutData.m_nMoveRatioX != 0))
			{
				rcItem.left += ::MulDiv(nDeltaWidth, LayoutData.m_nMoveRatioX, 100);
				rcItem.right += ::MulDiv(nDeltaWidth, LayoutData.m_nMoveRatioX, 100);
				nFlags &= ~SWP_NOMOVE;
			}

			if(((nID == ATL_IDW_STATUS_BAR) || (nDeltaHeight >= 0)) && (LayoutData.m_nMoveRatioY != 0))
			{
				rcItem.top += ::MulDiv(nDeltaHeight, LayoutData.m_nMoveRatioY, 100);
				rcItem.bottom += ::MulDiv(nDeltaHeight, LayoutData.m_nMoveRatioY, 100);
				nFlags &= ~SWP_NOMOVE;
			}

			if((nDeltaWidth >= 0) && (LayoutData.m_nSizeRatioX != 0))
			{
				rcItem.right += ::MulDiv(nDeltaWidth, LayoutData.m_nSizeRatioX, 100);
				nFlags &= ~SWP_NOSIZE;
			}

			if((nDeltaHeight >= 0) && (LayoutData.m_nSizeRatioY != 0))
			{
				rcItem.bottom += ::MulDiv(nDeltaHeight, LayoutData.m_nSizeRatioY, 100);
				nFlags &= ~SWP_NOSIZE;
			}

			if(nFlags != (SWP_NOMOVE | SWP_NOSIZE))
				::DeferWindowPos(hDwp, LayoutData.m_hWnd, NULL, rcItem.left, rcItem.top, rcItem.right - rcItem.left, rcItem.bottom - rcItem.top, nFlags | SWP_NOZORDER | SWP_NOREPOSITION | SWP_NOACTIVATE | SWP_NOCOPYBITS);
		}

		::EndDeferWindowPos(hDwp);
	}

// Message map and handlers
	BEGIN_MSG_MAP(CDynamicDialogLayout)
		MESSAGE_HANDLER(WM_SIZE, OnSize)
		MESSAGE_HANDLER(WM_GETMINMAXINFO, OnGetMinMaxInfo)
	END_MSG_MAP()

	LRESULT OnSize(UINT /*uMsg*/, WPARAM wParam, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		T* pT = static_cast<T*>(this);

		if(m_bGripper)
		{
			ATL::CWindow wndGripper = pT->GetDlgItem(ATL_IDW_STATUS_BAR);
			if(wndGripper.m_hWnd != NULL)
			{
				if(wParam == SIZE_MAXIMIZED)
					wndGripper.ShowWindow(SW_HIDE);
				else if(wParam == SIZE_RESTORED)
					wndGripper.ShowWindow(SW_SHOW);
			}
		}

		if(wParam != SIZE_MINIMIZED)
			pT->UpdateDynamicLayout();

		return 0;
	}

	LRESULT OnGetMinMaxInfo(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM lParam, BOOL& /*bHandled*/)
	{
		if((m_ptMinTrackSize.x != -1) && (m_ptMinTrackSize.y != -1))
		{
			LPMINMAXINFO lpMMI = (LPMINMAXINFO)lParam;
			lpMMI->ptMinTrackSize =  m_ptMinTrackSize;
		}

		return 0;
	}

// Implementation
	char _GetDataPct(WORD wResData)
	{
		ATLASSERT((wResData >= 0) && (wResData <= 100));
		char nPct = (char)LOBYTE(wResData);
		if((nPct < 0) || (nPct > 100))
			nPct = -1;

		return nPct;
	}
};

#endif // (_MSC_VER >= 1900)


///////////////////////////////////////////////////////////////////////////////
// CDoubleBufferImpl - Provides double-buffer painting support to any window

template <class T>
class CDoubleBufferImpl
{
public:
// Overrideables
	void DoPaint(CDCHandle /*dc*/)
	{
		// must be implemented in a derived class
		ATLASSERT(FALSE);
	}

// Message map and handlers
	BEGIN_MSG_MAP(CDoubleBufferImpl)
		MESSAGE_HANDLER(WM_ERASEBKGND, OnEraseBackground)
		MESSAGE_HANDLER(WM_PAINT, OnPaint)
		MESSAGE_HANDLER(WM_PRINTCLIENT, OnPaint)
	END_MSG_MAP()

	LRESULT OnEraseBackground(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		return 1;   // no background painting needed
	}

	LRESULT OnPaint(UINT /*uMsg*/, WPARAM wParam, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(::IsWindow(pT->m_hWnd));

		if(wParam != NULL)
		{
			RECT rect = {};
			pT->GetClientRect(&rect);
			CMemoryDC dcMem((HDC)wParam, rect);
			pT->DoPaint(dcMem.m_hDC);
		}
		else
		{
			CPaintDC dc(pT->m_hWnd);
			CMemoryDC dcMem(dc.m_hDC, dc.m_ps.rcPaint);
			pT->DoPaint(dcMem.m_hDC);
		}

		return 0;
	}
};


///////////////////////////////////////////////////////////////////////////////
// CDoubleBufferWindowImpl - Implements a double-buffer painting window

template <class T, class TBase = ATL::CWindow, class TWinTraits = ATL::CControlWinTraits>
class ATL_NO_VTABLE CDoubleBufferWindowImpl : public ATL::CWindowImpl< T, TBase, TWinTraits >, public CDoubleBufferImpl< T >
{
public:
	BEGIN_MSG_MAP(CDoubleBufferWindowImpl)
		CHAIN_MSG_MAP(CDoubleBufferImpl< T >)
	END_MSG_MAP()
};


// command bar support
#if !defined(__ATLCTRLW_H__)
  #undef CBRM_GETMENU
  #undef CBRM_TRACKPOPUPMENU
  #undef CBRM_GETCMDBAR
  #undef CBRPOPUPMENU
#endif // !defined(__ATLCTRLW_H__)

} // namespace WTL

#endif // __ATLFRAME_H__
