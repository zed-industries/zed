// Windows Template Library - WTL version 10.0
// Copyright (C) Microsoft Corporation, WTL Team. All rights reserved.
//
// This file is a part of the Windows Template Library.
// The use and distribution terms for this software are covered by the
// Microsoft Public License (http://opensource.org/licenses/MS-PL)
// which can be found in the file MS-PL.txt at the root folder.

#ifndef __ATLUSER_H__
#define __ATLUSER_H__

#pragma once

#ifndef __ATLAPP_H__
	#error atluser.h requires atlapp.h to be included first
#endif


///////////////////////////////////////////////////////////////////////////////
// Classes in this file:
//
// CMenuItemInfo
// CMenuT<t_bManaged>
// CAcceleratorT<t_bManaged>
// CIconT<t_bManaged>
// CCursorT<t_bManaged>
// CResource
//
// Global functions:
//   AtlMessageBox()
//
//   AtlLoadAccelerators()
//   AtlLoadMenu()
//   AtlLoadBitmap()
//   AtlLoadSysBitmap()
//   AtlLoadCursor()
//   AtlLoadSysCursor()
//   AtlLoadIcon()
//   AtlLoadSysIcon()
//   AtlLoadBitmapImage()
//   AtlLoadCursorImage()
//   AtlLoadIconImage()
//   AtlLoadSysBitmapImage()
//   AtlLoadSysCursorImage()
//   AtlLoadSysIconImage()
//   AtlLoadString()


namespace WTL
{

///////////////////////////////////////////////////////////////////////////////
// AtlMessageBox - accepts both memory and resource based strings

inline int AtlMessageBox(HWND hWndOwner, ATL::_U_STRINGorID message, ATL::_U_STRINGorID title = (LPCTSTR)NULL, UINT uType = MB_OK | MB_ICONINFORMATION)
{
	ATLASSERT((hWndOwner == NULL) || ::IsWindow(hWndOwner));

	LPTSTR lpstrMessage = NULL;
	if(IS_INTRESOURCE(message.m_lpstr))
	{
		for(int nLen = 256; ; nLen *= 2)
		{
			ATLTRY(lpstrMessage = new TCHAR[nLen]);
			if(lpstrMessage == NULL)
			{
				ATLASSERT(FALSE);
				return 0;
			}
			int nRes = ::LoadString(ModuleHelper::GetResourceInstance(), LOWORD(message.m_lpstr), lpstrMessage, nLen);
			if(nRes < nLen - 1)
				break;
			delete [] lpstrMessage;
			lpstrMessage = NULL;
		}

		message.m_lpstr = lpstrMessage;
	}

	LPTSTR lpstrTitle = NULL;
	if(IS_INTRESOURCE(title.m_lpstr) && (LOWORD(title.m_lpstr) != 0))
	{
		for(int nLen = 256; ; nLen *= 2)
		{
			ATLTRY(lpstrTitle = new TCHAR[nLen]);
			if(lpstrTitle == NULL)
			{
				ATLASSERT(FALSE);
				return 0;
			}
			int nRes = ::LoadString(ModuleHelper::GetResourceInstance(), LOWORD(title.m_lpstr), lpstrTitle, nLen);
			if(nRes < nLen - 1)
				break;
			delete [] lpstrTitle;
			lpstrTitle = NULL;
		}

		title.m_lpstr = lpstrTitle;
	}

	int nRet = ::MessageBox(hWndOwner, message.m_lpstr, title.m_lpstr, uType);

	delete [] lpstrMessage;
	delete [] lpstrTitle;

	return nRet;
}


///////////////////////////////////////////////////////////////////////////////
// CMenu

class CMenuItemInfo : public MENUITEMINFO
{
public:
	CMenuItemInfo()
	{
		memset(this, 0, sizeof(MENUITEMINFO));
		cbSize = sizeof(MENUITEMINFO);
	}
};


// forward declarations
template <bool t_bManaged> class CMenuT;
typedef CMenuT<false>   CMenuHandle;
typedef CMenuT<true>    CMenu;


template <bool t_bManaged>
class CMenuT
{
public:
// Data members
	HMENU m_hMenu;

// Constructor/destructor/operators
	CMenuT(HMENU hMenu = NULL) : m_hMenu(hMenu)
	{ }

	~CMenuT()
	{
		if(t_bManaged && (m_hMenu != NULL))
			DestroyMenu();
	}

	CMenuT<t_bManaged>& operator =(HMENU hMenu)
	{
		Attach(hMenu);
		return *this;
	}

	void Attach(HMENU hMenuNew)
	{
		ATLASSERT(::IsMenu(hMenuNew));
		if(t_bManaged && (m_hMenu != NULL) && (m_hMenu != hMenuNew))
			::DestroyMenu(m_hMenu);
		m_hMenu = hMenuNew;
	}

	HMENU Detach()
	{
		HMENU hMenu = m_hMenu;
		m_hMenu = NULL;
		return hMenu;
	}

	operator HMENU() const { return m_hMenu; }

	bool IsNull() const { return (m_hMenu == NULL); }

	BOOL IsMenu() const
	{
		return ::IsMenu(m_hMenu);
	}

// Create/destroy methods
	BOOL CreateMenu()
	{
		ATLASSERT(m_hMenu == NULL);
		m_hMenu = ::CreateMenu();
		return (m_hMenu != NULL) ? TRUE : FALSE;
	}

	BOOL CreatePopupMenu()
	{
		ATLASSERT(m_hMenu == NULL);
		m_hMenu = ::CreatePopupMenu();
		return (m_hMenu != NULL) ? TRUE : FALSE;
	}

	BOOL LoadMenu(ATL::_U_STRINGorID menu)
	{
		ATLASSERT(m_hMenu == NULL);
		m_hMenu = ::LoadMenu(ModuleHelper::GetResourceInstance(), menu.m_lpstr);
		return (m_hMenu != NULL) ? TRUE : FALSE;
	}

	BOOL LoadMenuIndirect(const void* lpMenuTemplate)
	{
		ATLASSERT(m_hMenu == NULL);
		m_hMenu = ::LoadMenuIndirect(lpMenuTemplate);
		return (m_hMenu != NULL) ? TRUE : FALSE;
	}

	BOOL DestroyMenu()
	{
		if (m_hMenu == NULL)
			return FALSE;
		BOOL bRet = ::DestroyMenu(m_hMenu);
		if(bRet)
			m_hMenu = NULL;
		return bRet;
	}

// Menu Operations
	BOOL DeleteMenu(UINT nPosition, UINT nFlags)
	{
		ATLASSERT(::IsMenu(m_hMenu));
		return ::DeleteMenu(m_hMenu, nPosition, nFlags);
	}

	BOOL TrackPopupMenu(UINT nFlags, int x, int y, HWND hWnd, LPCRECT lpRect = NULL)
	{
		ATLASSERT(::IsMenu(m_hMenu));
		x = _FixTrackMenuPopupX(x, y);
		return ::TrackPopupMenu(m_hMenu, nFlags, x, y, 0, hWnd, lpRect);
	}

	BOOL TrackPopupMenuEx(UINT uFlags, int x, int y, HWND hWnd, LPTPMPARAMS lptpm = NULL)
	{
		ATLASSERT(::IsMenu(m_hMenu));
		x = _FixTrackMenuPopupX(x, y);
		return ::TrackPopupMenuEx(m_hMenu, uFlags, x, y, hWnd, lptpm);
	}

	// helper that fixes popup menu X position when it's off-screen
	static int _FixTrackMenuPopupX(int x, int y)
	{
		POINT pt = { x, y };
		HMONITOR hMonitor = ::MonitorFromPoint(pt, MONITOR_DEFAULTTONULL);
		if(hMonitor == NULL)
		{
			HMONITOR hMonitorNear = ::MonitorFromPoint(pt, MONITOR_DEFAULTTONEAREST);
			if(hMonitorNear != NULL)
			{
				MONITORINFO mi = { sizeof(MONITORINFO) };
				if(::GetMonitorInfo(hMonitorNear, &mi) != FALSE)
				{
					if(x < mi.rcWork.left)
						x = mi.rcWork.left;
					else if(x > mi.rcWork.right)
						x = mi.rcWork.right;
				}
			}
		}

		return x;
	}

	BOOL GetMenuInfo(LPMENUINFO lpMenuInfo) const
	{
		ATLASSERT(::IsMenu(m_hMenu));
		return ::GetMenuInfo(m_hMenu, lpMenuInfo);
	}

	BOOL SetMenuInfo(LPCMENUINFO lpMenuInfo)
	{
		ATLASSERT(::IsMenu(m_hMenu));
		return ::SetMenuInfo(m_hMenu, lpMenuInfo);
	}

// Menu Item Operations
	BOOL AppendMenu(UINT nFlags, UINT_PTR nIDNewItem = 0, LPCTSTR lpszNewItem = NULL)
	{
		ATLASSERT(::IsMenu(m_hMenu));
		return ::AppendMenu(m_hMenu, nFlags, nIDNewItem, lpszNewItem);
	}

	BOOL AppendMenu(UINT nFlags, HMENU hSubMenu, LPCTSTR lpszNewItem)
	{
		ATLASSERT(::IsMenu(m_hMenu));
		ATLASSERT(::IsMenu(hSubMenu));
		return ::AppendMenu(m_hMenu, nFlags | MF_POPUP, (UINT_PTR)hSubMenu, lpszNewItem);
	}

	BOOL AppendMenu(UINT nFlags, UINT_PTR nIDNewItem, HBITMAP hBmp)
	{
		ATLASSERT(::IsMenu(m_hMenu));
		return ::AppendMenu(m_hMenu, nFlags | MF_BITMAP, nIDNewItem, (LPCTSTR)hBmp);
	}

	BOOL AppendMenu(UINT nFlags, HMENU hSubMenu, HBITMAP hBmp)
	{
		ATLASSERT(::IsMenu(m_hMenu));
		ATLASSERT(::IsMenu(hSubMenu));
		return ::AppendMenu(m_hMenu, nFlags | (MF_BITMAP | MF_POPUP), (UINT_PTR)hSubMenu, (LPCTSTR)hBmp);
	}

	UINT CheckMenuItem(UINT nIDCheckItem, UINT nCheck)
	{
		ATLASSERT(::IsMenu(m_hMenu));
		return (UINT)::CheckMenuItem(m_hMenu, nIDCheckItem, nCheck);
	}

	UINT EnableMenuItem(UINT nIDEnableItem, UINT nEnable)
	{
		ATLASSERT(::IsMenu(m_hMenu));
		return ::EnableMenuItem(m_hMenu, nIDEnableItem, nEnable);
	}

	BOOL HiliteMenuItem(HWND hWnd, UINT uIDHiliteItem, UINT uHilite)
	{
		ATLASSERT(::IsMenu(m_hMenu));
		return ::HiliteMenuItem(hWnd, m_hMenu, uIDHiliteItem, uHilite);
	}

	int GetMenuItemCount() const
	{
		ATLASSERT(::IsMenu(m_hMenu));
		return ::GetMenuItemCount(m_hMenu);
	}

	UINT GetMenuItemID(int nPos) const
	{
		ATLASSERT(::IsMenu(m_hMenu));
		return ::GetMenuItemID(m_hMenu, nPos);
	}

	UINT GetMenuState(UINT nID, UINT nFlags) const
	{
		ATLASSERT(::IsMenu(m_hMenu));
		return ::GetMenuState(m_hMenu, nID, nFlags);
	}

	int GetMenuString(UINT nIDItem, LPTSTR lpString, int nMaxCount, UINT nFlags) const
	{
		ATLASSERT(::IsMenu(m_hMenu));
		return ::GetMenuString(m_hMenu, nIDItem, lpString, nMaxCount, nFlags);
	}

	int GetMenuStringLen(UINT nIDItem, UINT nFlags) const
	{
		ATLASSERT(::IsMenu(m_hMenu));
		return ::GetMenuString(m_hMenu, nIDItem, NULL, 0, nFlags);
	}

	BOOL GetMenuString(UINT nIDItem, BSTR& bstrText, UINT nFlags) const
	{
		USES_CONVERSION;
		ATLASSERT(::IsMenu(m_hMenu));
		ATLASSERT(bstrText == NULL);

		int nLen = GetMenuStringLen(nIDItem, nFlags);
		if(nLen == 0)
		{
			bstrText = ::SysAllocString(OLESTR(""));
			return (bstrText != NULL) ? TRUE : FALSE;
		}

		nLen++;   // increment to include terminating NULL char
		ATL::CTempBuffer<TCHAR, _WTL_STACK_ALLOC_THRESHOLD> buff;
		LPTSTR lpszText = buff.Allocate(nLen);
		if(lpszText == NULL)
			return FALSE;

		if(!GetMenuString(nIDItem, lpszText, nLen, nFlags))
			return FALSE;

		bstrText = ::SysAllocString(T2OLE(lpszText));
		return (bstrText != NULL) ? TRUE : FALSE;
	}

#ifdef __ATLSTR_H__
	int GetMenuString(UINT nIDItem, ATL::CString& strText, UINT nFlags) const
	{
		ATLASSERT(::IsMenu(m_hMenu));

		int nLen = GetMenuStringLen(nIDItem, nFlags);
		if(nLen == 0)
			return 0;

		nLen++;   // increment to include terminating NULL char
		LPTSTR lpstr = strText.GetBufferSetLength(nLen);
		if(lpstr == NULL)
			return 0;
		int nRet = GetMenuString(nIDItem, lpstr, nLen, nFlags);
		strText.ReleaseBuffer();
		return nRet;
	}
#endif // __ATLSTR_H__

	CMenuHandle GetSubMenu(int nPos) const
	{
		ATLASSERT(::IsMenu(m_hMenu));
		return CMenuHandle(::GetSubMenu(m_hMenu, nPos));
	}

	BOOL InsertMenu(UINT nPosition, UINT nFlags, UINT_PTR nIDNewItem = 0, LPCTSTR lpszNewItem = NULL)
	{
		ATLASSERT(::IsMenu(m_hMenu));
		return ::InsertMenu(m_hMenu, nPosition, nFlags, nIDNewItem, lpszNewItem);
	}

	BOOL InsertMenu(UINT nPosition, UINT nFlags, HMENU hSubMenu, LPCTSTR lpszNewItem)
	{
		ATLASSERT(::IsMenu(m_hMenu));
		ATLASSERT(::IsMenu(hSubMenu));
		return ::InsertMenu(m_hMenu, nPosition, nFlags | MF_POPUP, (UINT_PTR)hSubMenu, lpszNewItem);
	}

	BOOL InsertMenu(UINT nPosition, UINT nFlags, UINT_PTR nIDNewItem, HBITMAP hBmp)
	{
		ATLASSERT(::IsMenu(m_hMenu));
		return ::InsertMenu(m_hMenu, nPosition, nFlags | MF_BITMAP, nIDNewItem, (LPCTSTR)hBmp);
	}

	BOOL InsertMenu(UINT nPosition, UINT nFlags, HMENU hSubMenu, HBITMAP hBmp)
	{
		ATLASSERT(::IsMenu(m_hMenu));
		ATLASSERT(::IsMenu(hSubMenu));
		return ::InsertMenu(m_hMenu, nPosition, nFlags | (MF_BITMAP | MF_POPUP), (UINT_PTR)hSubMenu, (LPCTSTR)hBmp);
	}

	BOOL ModifyMenu(UINT nPosition, UINT nFlags, UINT_PTR nIDNewItem = 0, LPCTSTR lpszNewItem = NULL)
	{
		ATLASSERT(::IsMenu(m_hMenu));
		return ::ModifyMenu(m_hMenu, nPosition, nFlags, nIDNewItem, lpszNewItem);
	}

	BOOL ModifyMenu(UINT nPosition, UINT nFlags, HMENU hSubMenu, LPCTSTR lpszNewItem)
	{
		ATLASSERT(::IsMenu(m_hMenu));
		ATLASSERT(::IsMenu(hSubMenu));
		return ::ModifyMenu(m_hMenu, nPosition, nFlags | MF_POPUP, (UINT_PTR)hSubMenu, lpszNewItem);
	}

	BOOL ModifyMenu(UINT nPosition, UINT nFlags, UINT_PTR nIDNewItem, HBITMAP hBmp)
	{
		ATLASSERT(::IsMenu(m_hMenu));
		return ::ModifyMenu(m_hMenu, nPosition, nFlags | MF_BITMAP, nIDNewItem, (LPCTSTR)hBmp);
	}

	BOOL ModifyMenu(UINT nPosition, UINT nFlags, HMENU hSubMenu, HBITMAP hBmp)
	{
		ATLASSERT(::IsMenu(m_hMenu));
		ATLASSERT(::IsMenu(hSubMenu));
		return ::ModifyMenu(m_hMenu, nPosition, nFlags | (MF_BITMAP | MF_POPUP), (UINT_PTR)hSubMenu, (LPCTSTR)hBmp);
	}

	BOOL RemoveMenu(UINT nPosition, UINT nFlags)
	{
		ATLASSERT(::IsMenu(m_hMenu));
		return ::RemoveMenu(m_hMenu, nPosition, nFlags);
	}

	BOOL SetMenuItemBitmaps(UINT nPosition, UINT nFlags, HBITMAP hBmpUnchecked, HBITMAP hBmpChecked)
	{
		ATLASSERT(::IsMenu(m_hMenu));
		return ::SetMenuItemBitmaps(m_hMenu, nPosition, nFlags, hBmpUnchecked, hBmpChecked);
	}

	BOOL CheckMenuRadioItem(UINT nIDFirst, UINT nIDLast, UINT nIDItem, UINT nFlags)
	{
		ATLASSERT(::IsMenu(m_hMenu));
		return ::CheckMenuRadioItem(m_hMenu, nIDFirst, nIDLast, nIDItem, nFlags);
	}

	BOOL GetMenuItemInfo(UINT uItem, BOOL bByPosition, LPMENUITEMINFO lpmii) const
	{
		ATLASSERT(::IsMenu(m_hMenu));
		return (BOOL)::GetMenuItemInfo(m_hMenu, uItem, bByPosition, lpmii);
	}

	BOOL SetMenuItemInfo(UINT uItem, BOOL bByPosition, LPMENUITEMINFO lpmii)
	{
		ATLASSERT(::IsMenu(m_hMenu));
		return (BOOL)::SetMenuItemInfo(m_hMenu, uItem, bByPosition, lpmii);
	}

	BOOL InsertMenuItem(UINT uItem, BOOL bByPosition, LPMENUITEMINFO lpmii)
	{
		ATLASSERT(::IsMenu(m_hMenu));
		return (BOOL)::InsertMenuItem(m_hMenu, uItem, bByPosition, lpmii);
	}

	UINT GetMenuDefaultItem(BOOL bByPosition = FALSE, UINT uFlags = 0U) const
	{
		ATLASSERT(::IsMenu(m_hMenu));
		return ::GetMenuDefaultItem(m_hMenu, (UINT)bByPosition, uFlags);
	}

	BOOL SetMenuDefaultItem(UINT uItem = (UINT)-1,  BOOL bByPosition = FALSE)
	{
		ATLASSERT(::IsMenu(m_hMenu));
		return ::SetMenuDefaultItem(m_hMenu, uItem, (UINT)bByPosition);
	}

	BOOL GetMenuItemRect(HWND hWnd, UINT uItem, LPRECT lprcItem) const
	{
		ATLASSERT(::IsMenu(m_hMenu));
		return ::GetMenuItemRect(hWnd, m_hMenu, uItem, lprcItem);
	}

	int MenuItemFromPoint(HWND hWnd, POINT point) const
	{
		ATLASSERT(::IsMenu(m_hMenu));
		return ::MenuItemFromPoint(hWnd, m_hMenu, point);
	}

// Context Help Functions
	BOOL SetMenuContextHelpId(DWORD dwContextHelpId)
	{
		ATLASSERT(::IsMenu(m_hMenu));
		return ::SetMenuContextHelpId(m_hMenu, dwContextHelpId);
	}

	DWORD GetMenuContextHelpId() const
	{
		ATLASSERT(::IsMenu(m_hMenu));
		return ::GetMenuContextHelpId(m_hMenu);
	}
};


///////////////////////////////////////////////////////////////////////////////
// CAccelerator

template <bool t_bManaged>
class CAcceleratorT
{
public:
	HACCEL m_hAccel;

// Constructor/destructor/operators
	CAcceleratorT(HACCEL hAccel = NULL) : m_hAccel(hAccel)
	{ }

	~CAcceleratorT()
	{
		if(t_bManaged && (m_hAccel != NULL))
			::DestroyAcceleratorTable(m_hAccel);
	}

	CAcceleratorT<t_bManaged>& operator =(HACCEL hAccel)
	{
		Attach(hAccel);
		return *this;
	}

	void Attach(HACCEL hAccel)
	{
		if(t_bManaged && (m_hAccel != NULL))
			::DestroyAcceleratorTable(m_hAccel);
		m_hAccel = hAccel;
	}

	HACCEL Detach()
	{
		HACCEL hAccel = m_hAccel;
		m_hAccel = NULL;
		return hAccel;
	}

	operator HACCEL() const { return m_hAccel; }

	bool IsNull() const { return m_hAccel == NULL; }

// Create/destroy methods
	HACCEL LoadAccelerators(ATL::_U_STRINGorID accel)
	{
		ATLASSERT(m_hAccel == NULL);
		m_hAccel = ::LoadAccelerators(ModuleHelper::GetResourceInstance(), accel.m_lpstr);
		return m_hAccel;
	}

	HACCEL CreateAcceleratorTable(LPACCEL pAccel, int cEntries)
	{
		ATLASSERT(m_hAccel == NULL);
		ATLASSERT(pAccel != NULL);
		m_hAccel = ::CreateAcceleratorTable(pAccel, cEntries);
		return m_hAccel;
	}

	void DestroyObject()
	{
		if(m_hAccel != NULL)
		{
			::DestroyAcceleratorTable(m_hAccel);
			m_hAccel = NULL;
		}
	}

// Operations
	int CopyAcceleratorTable(LPACCEL lpAccelDst, int cEntries)
	{
		ATLASSERT(m_hAccel != NULL);
		ATLASSERT(lpAccelDst != NULL);
		return ::CopyAcceleratorTable(m_hAccel, lpAccelDst, cEntries);
	}

	int GetEntriesCount() const
	{
		ATLASSERT(m_hAccel != NULL);
		return ::CopyAcceleratorTable(m_hAccel, NULL, 0);
	}

	BOOL TranslateAccelerator(HWND hWnd, LPMSG pMsg)
	{
		ATLASSERT(m_hAccel != NULL);
		ATLASSERT(::IsWindow(hWnd));
		ATLASSERT(pMsg != NULL);
		return ::TranslateAccelerator(hWnd, m_hAccel, pMsg);
	}
};

typedef CAcceleratorT<false>   CAcceleratorHandle;
typedef CAcceleratorT<true>    CAccelerator;


///////////////////////////////////////////////////////////////////////////////
// CIcon

template <bool t_bManaged>
class CIconT
{
public:
	HICON m_hIcon;

// Constructor/destructor/operators
	CIconT(HICON hIcon = NULL) : m_hIcon(hIcon)
	{ }

	~CIconT()
	{
		if(t_bManaged && (m_hIcon != NULL))
			::DestroyIcon(m_hIcon);
	}

	CIconT<t_bManaged>& operator =(HICON hIcon)
	{
		Attach(hIcon);
		return *this;
	}

	void Attach(HICON hIcon)
	{
		if(t_bManaged && (m_hIcon != NULL))
			::DestroyIcon(m_hIcon);
		m_hIcon = hIcon;
	}

	HICON Detach()
	{
		HICON hIcon = m_hIcon;
		m_hIcon = NULL;
		return hIcon;
	}

	operator HICON() const { return m_hIcon; }

	bool IsNull() const { return m_hIcon == NULL; }

// Create/destroy methods
	HICON LoadIcon(ATL::_U_STRINGorID icon)
	{
		ATLASSERT(m_hIcon == NULL);
		m_hIcon = ::LoadIcon(ModuleHelper::GetResourceInstance(), icon.m_lpstr);
		return m_hIcon;
	}

	HICON LoadIcon(ATL::_U_STRINGorID icon, int cxDesired, int cyDesired, UINT fuLoad = 0)
	{
		ATLASSERT(m_hIcon == NULL);
		m_hIcon = (HICON) ::LoadImage(ModuleHelper::GetResourceInstance(), icon.m_lpstr, IMAGE_ICON, cxDesired, cyDesired, fuLoad);
		return m_hIcon;
	}

	HICON LoadOEMIcon(LPCTSTR lpstrIconName)
	{
		ATLASSERT(m_hIcon == NULL);
		ATLASSERT(IsOEMIcon(lpstrIconName));
		m_hIcon = ::LoadIcon(NULL, lpstrIconName);
		return m_hIcon;
	}

	HICON CreateIcon(int nWidth, int nHeight, BYTE cPlanes, BYTE cBitsPixel, CONST BYTE* lpbANDbits, CONST BYTE *lpbXORbits)
	{
		ATLASSERT(m_hIcon == NULL);
		ATLASSERT(lpbANDbits != NULL);
		ATLASSERT(lpbXORbits != NULL);
		m_hIcon = ::CreateIcon(ModuleHelper::GetResourceInstance(), nWidth, nHeight, cPlanes, cBitsPixel, lpbANDbits, lpbXORbits);
		return m_hIcon;
	}

	HICON CreateIconFromResource(PBYTE pBits, DWORD dwResSize, DWORD dwVersion = 0x00030000)
	{
		ATLASSERT(m_hIcon == NULL);
		ATLASSERT(pBits != NULL);
		m_hIcon = ::CreateIconFromResource(pBits, dwResSize, TRUE, dwVersion);
		return m_hIcon;
	}

	HICON CreateIconFromResourceEx(PBYTE pbBits, DWORD cbBits, DWORD dwVersion = 0x00030000, int cxDesired = 0, int cyDesired = 0, UINT uFlags = LR_DEFAULTCOLOR)
	{
		ATLASSERT(m_hIcon == NULL);
		ATLASSERT(pbBits != NULL);
		ATLASSERT(cbBits > 0);
		m_hIcon = ::CreateIconFromResourceEx(pbBits, cbBits, TRUE, dwVersion, cxDesired, cyDesired, uFlags);
		return m_hIcon;
	}

	HICON CreateIconIndirect(PICONINFO pIconInfo)
	{
		ATLASSERT(m_hIcon == NULL);
		ATLASSERT(pIconInfo != NULL);
		m_hIcon = ::CreateIconIndirect(pIconInfo);
		return m_hIcon;
	}

	HICON ExtractIcon(LPCTSTR lpszExeFileName, UINT nIconIndex)
	{
		ATLASSERT(m_hIcon == NULL);
		ATLASSERT(lpszExeFileName != NULL);
		m_hIcon = ::ExtractIcon(ModuleHelper::GetModuleInstance(), lpszExeFileName, nIconIndex);
		return m_hIcon;
	}

	HICON ExtractAssociatedIcon(HINSTANCE hInst, LPTSTR lpIconPath, LPWORD lpiIcon)
	{
		ATLASSERT(m_hIcon == NULL);
		ATLASSERT(lpIconPath != NULL);
		ATLASSERT(lpiIcon != NULL);
		m_hIcon = ::ExtractAssociatedIcon(hInst, lpIconPath, lpiIcon);
		return m_hIcon;
	}

	BOOL DestroyIcon()
	{
		ATLASSERT(m_hIcon != NULL);
		BOOL bRet = ::DestroyIcon(m_hIcon);
		if(bRet != FALSE)
			m_hIcon = NULL;
		return bRet;
	}

// Operations
	HICON CopyIcon()
	{
		ATLASSERT(m_hIcon != NULL);
		return ::CopyIcon(m_hIcon);
	}

	HICON DuplicateIcon()
	{
		ATLASSERT(m_hIcon != NULL);
		return ::DuplicateIcon(NULL, m_hIcon);
	}

	BOOL DrawIcon(HDC hDC, int x, int y)
	{
		ATLASSERT(m_hIcon != NULL);
		return ::DrawIcon(hDC, x, y, m_hIcon);
	}

	BOOL DrawIcon(HDC hDC, POINT pt)
	{
		ATLASSERT(m_hIcon != NULL);
		return ::DrawIcon(hDC, pt.x, pt.y, m_hIcon);
	}

	BOOL DrawIconEx(HDC hDC, int x, int y, int cxWidth, int cyWidth, UINT uStepIfAniCur = 0, HBRUSH hbrFlickerFreeDraw = NULL, UINT uFlags = DI_NORMAL)
	{
		ATLASSERT(m_hIcon != NULL);
		return ::DrawIconEx(hDC, x, y, m_hIcon, cxWidth, cyWidth, uStepIfAniCur, hbrFlickerFreeDraw, uFlags);
	}

	BOOL DrawIconEx(HDC hDC, POINT pt, SIZE size, UINT uStepIfAniCur = 0, HBRUSH hbrFlickerFreeDraw = NULL, UINT uFlags = DI_NORMAL)
	{
		ATLASSERT(m_hIcon != NULL);
		return ::DrawIconEx(hDC, pt.x, pt.y, m_hIcon, size.cx, size.cy, uStepIfAniCur, hbrFlickerFreeDraw, uFlags);
	}

	BOOL GetIconInfo(PICONINFO pIconInfo) const
	{
		ATLASSERT(m_hIcon != NULL);
		ATLASSERT(pIconInfo != NULL);
		return ::GetIconInfo(m_hIcon, pIconInfo);
	}

#if (_WIN32_WINNT >= 0x0600)
	BOOL GetIconInfoEx(PICONINFOEX pIconInfo) const
	{
		ATLASSERT(m_hIcon != NULL);
		ATLASSERT(pIconInfo != NULL);
		return ::GetIconInfoEx(m_hIcon, pIconInfo);
	}
#endif // (_WIN32_WINNT >= 0x0600)

#if defined(NTDDI_VERSION) && (NTDDI_VERSION >= NTDDI_LONGHORN)
	HRESULT LoadIconMetric(ATL::_U_STRINGorID icon, int lims)
	{
		ATLASSERT(m_hIcon == NULL);
		USES_CONVERSION;
		return ::LoadIconMetric(ModuleHelper::GetResourceInstance(), T2CW(icon.m_lpstr), lims, &m_hIcon);
	}

	HRESULT LoadIconWithScaleDown(ATL::_U_STRINGorID icon, int cx, int cy)
	{
		ATLASSERT(m_hIcon == NULL);
		USES_CONVERSION;
		return ::LoadIconWithScaleDown(ModuleHelper::GetResourceInstance(), T2CW(icon.m_lpstr), cx, cy, &m_hIcon);
	}

	HRESULT LoadOEMIconMetric(LPCTSTR lpstrIconName, int lims)
	{
		ATLASSERT(m_hIcon == NULL);
		ATLASSERT(IsOEMIcon(lpstrIconName));
		return ::LoadIconMetric(NULL, (LPCWSTR)lpstrIconName, lims, &m_hIcon);
	}

	HRESULT LoadOEMIconWithScaleDown(LPCTSTR lpstrIconName, int cx, int cy)
	{
		ATLASSERT(m_hIcon == NULL);
		ATLASSERT(IsOEMIcon(lpstrIconName));
		USES_CONVERSION;
		return ::LoadIconWithScaleDown(NULL, (LPCWSTR)lpstrIconName, cx, cy, &m_hIcon);
	}
#endif // defined(NTDDI_VERSION) && (NTDDI_VERSION >= NTDDI_LONGHORN)

	// Helper
	static bool IsOEMIcon(LPCTSTR lpstrIconName)
	{
#if (WINVER >= 0x0600)
		return ((lpstrIconName == IDI_APPLICATION) || (lpstrIconName == IDI_ASTERISK) || (lpstrIconName == IDI_EXCLAMATION) ||
		          (lpstrIconName == IDI_HAND) || (lpstrIconName == IDI_QUESTION) || (lpstrIconName == IDI_WINLOGO) ||
		          (lpstrIconName == IDI_SHIELD));
#else // !(WINVER >= 0x0600)
		return ((lpstrIconName == IDI_APPLICATION) || (lpstrIconName == IDI_ASTERISK) || (lpstrIconName == IDI_EXCLAMATION) ||
		          (lpstrIconName == IDI_HAND) || (lpstrIconName == IDI_QUESTION) || (lpstrIconName == IDI_WINLOGO));
#endif // !(WINVER >= 0x0600)
	}
};

typedef CIconT<false>   CIconHandle;
typedef CIconT<true>    CIcon;


///////////////////////////////////////////////////////////////////////////////
// CCursor

// protect template member from a winuser.h macro
#ifdef CopyCursor
  #undef CopyCursor
#endif

template <bool t_bManaged>
class CCursorT
{
public:
	HCURSOR m_hCursor;

// Constructor/destructor/operators
	CCursorT(HCURSOR hCursor = NULL) : m_hCursor(hCursor)
	{ }

	~CCursorT()
	{
		if(t_bManaged && (m_hCursor != NULL))
			DestroyCursor();
	}

	CCursorT<t_bManaged>& operator =(HCURSOR hCursor)
	{
		Attach(hCursor);
		return *this;
	}

	void Attach(HCURSOR hCursor)
	{
		if(t_bManaged && (m_hCursor != NULL))
			DestroyCursor();
		m_hCursor = hCursor;
	}

	HCURSOR Detach()
	{
		HCURSOR hCursor = m_hCursor;
		m_hCursor = NULL;
		return hCursor;
	}

	operator HCURSOR() const { return m_hCursor; }

	bool IsNull() const { return m_hCursor == NULL; }

// Create/destroy methods
	HCURSOR LoadCursor(ATL::_U_STRINGorID cursor)
	{
		ATLASSERT(m_hCursor == NULL);
		m_hCursor = ::LoadCursor(ModuleHelper::GetResourceInstance(), cursor.m_lpstr);
		return m_hCursor;
	}

	HCURSOR LoadSysCursor(LPCTSTR lpstrCursorName)
	{
		ATLASSERT(m_hCursor == NULL);
		ATLASSERT((lpstrCursorName == IDC_ARROW) || (lpstrCursorName == IDC_IBEAM) || (lpstrCursorName == IDC_WAIT) ||
			(lpstrCursorName == IDC_CROSS) || (lpstrCursorName == IDC_UPARROW) || (lpstrCursorName == IDC_SIZE) ||
			(lpstrCursorName == IDC_ICON) || (lpstrCursorName == IDC_SIZENWSE) || (lpstrCursorName == IDC_SIZENESW) ||
			(lpstrCursorName == IDC_SIZEWE) || (lpstrCursorName == IDC_SIZENS) || (lpstrCursorName == IDC_SIZEALL) ||
			(lpstrCursorName == IDC_NO) || (lpstrCursorName == IDC_APPSTARTING) || (lpstrCursorName == IDC_HELP) ||
			(lpstrCursorName == IDC_HAND));
		m_hCursor = ::LoadCursor(NULL, lpstrCursorName);
		return m_hCursor;
	}

	// deprecated
	HCURSOR LoadOEMCursor(LPCTSTR lpstrCursorName)
	{
		return LoadSysCursor(lpstrCursorName);
	}

	HCURSOR LoadCursor(ATL::_U_STRINGorID cursor, int cxDesired, int cyDesired, UINT fuLoad = 0)
	{
		ATLASSERT(m_hCursor == NULL);
		m_hCursor = (HCURSOR) ::LoadImage(ModuleHelper::GetResourceInstance(), cursor.m_lpstr, IMAGE_CURSOR, cxDesired, cyDesired, fuLoad);
		return m_hCursor;
	}

	HCURSOR LoadCursorFromFile(LPCTSTR pstrFilename)
	{
		ATLASSERT(m_hCursor == NULL);
		ATLASSERT(pstrFilename != NULL);
		m_hCursor = ::LoadCursorFromFile(pstrFilename);
		return m_hCursor;
	}

	HCURSOR CreateCursor(int xHotSpot, int yHotSpot, int nWidth, int nHeight, CONST VOID *pvANDPlane, CONST VOID *pvXORPlane)
	{
		ATLASSERT(m_hCursor == NULL);
		m_hCursor = ::CreateCursor(ModuleHelper::GetResourceInstance(), xHotSpot, yHotSpot, nWidth, nHeight, pvANDPlane, pvXORPlane);
		return m_hCursor;
	}

	HCURSOR CreateCursorFromResource(PBYTE pBits, DWORD dwResSize, DWORD dwVersion = 0x00030000)
	{
		ATLASSERT(m_hCursor == NULL);
		ATLASSERT(pBits != NULL);
		m_hCursor = (HCURSOR)::CreateIconFromResource(pBits, dwResSize, FALSE, dwVersion);
		return m_hCursor;
	}

	HCURSOR CreateCursorFromResourceEx(PBYTE pbBits, DWORD cbBits, DWORD dwVersion = 0x00030000, int cxDesired = 0, int cyDesired = 0, UINT uFlags = LR_DEFAULTCOLOR)
	{
		ATLASSERT(m_hCursor == NULL);
		ATLASSERT(pbBits != NULL);
		ATLASSERT(cbBits > 0);
		m_hCursor = (HCURSOR)::CreateIconFromResourceEx(pbBits, cbBits, FALSE, dwVersion, cxDesired, cyDesired, uFlags);
		return m_hCursor;
	}

	BOOL DestroyCursor()
	{
		ATLASSERT(m_hCursor != NULL);
		BOOL bRet = ::DestroyCursor(m_hCursor);
		if(bRet != FALSE)
			m_hCursor = NULL;
		return bRet;
	}

// Operations
	HCURSOR CopyCursor()
	{
		ATLASSERT(m_hCursor != NULL);
		return (HCURSOR)::CopyIcon((HICON)m_hCursor);
	}

	BOOL GetCursorInfo(LPCURSORINFO pCursorInfo)
	{
		ATLASSERT(m_hCursor != NULL);
		ATLASSERT(pCursorInfo != NULL);
		return ::GetCursorInfo(pCursorInfo);
	}
};

typedef CCursorT<false>   CCursorHandle;
typedef CCursorT<true>    CCursor;


///////////////////////////////////////////////////////////////////////////////
// CResource - Wraps a generic Windows resource.
//             Use it with custom resource types other than the
//             standard RT_CURSOR, RT_BITMAP, etc.

class CResource
{
public:
	HGLOBAL m_hGlobal;
	HRSRC m_hResource;

// Constructor/destructor
	CResource() : m_hGlobal(NULL), m_hResource(NULL)
	{ }

	~CResource()
	{
		Release();
	}

// Load methods
	bool Load(ATL::_U_STRINGorID Type, ATL::_U_STRINGorID ID)
	{
		ATLASSERT(m_hResource == NULL);
		ATLASSERT(m_hGlobal == NULL);

		m_hResource = ::FindResource(ModuleHelper::GetResourceInstance(), ID.m_lpstr, Type.m_lpstr);
		if(m_hResource == NULL)
			return false;

		m_hGlobal = ::LoadResource(ModuleHelper::GetResourceInstance(), m_hResource);
		if(m_hGlobal == NULL)
		{
			m_hResource = NULL;
			return false;
		}

		return true;
	}

	bool LoadEx(ATL::_U_STRINGorID ID, ATL::_U_STRINGorID Type, WORD wLanguage)
	{
		ATLASSERT(m_hResource == NULL);
		ATLASSERT(m_hGlobal == NULL);

		m_hResource = ::FindResourceEx(ModuleHelper::GetResourceInstance(), Type.m_lpstr, ID.m_lpstr, wLanguage);
		if(m_hResource == NULL)
			return false;

		m_hGlobal = ::LoadResource(ModuleHelper::GetResourceInstance(), m_hResource);
		if(m_hGlobal == NULL)
		{
			m_hResource = NULL;
			return false;
		}

		return true;
	}

// Misc. operations
	DWORD GetSize() const
	{
		ATLASSERT(m_hResource != NULL);
		return ::SizeofResource(ModuleHelper::GetResourceInstance(), m_hResource);
	}

	LPVOID Lock()
	{
		ATLASSERT(m_hResource != NULL);
		ATLASSERT(m_hGlobal != NULL);
		LPVOID pVoid = ::LockResource(m_hGlobal);
		ATLASSERT(pVoid != NULL);
		return pVoid;
	}

	void Release()
	{
		if(m_hGlobal != NULL)
		{
			FreeResource(m_hGlobal);
			m_hGlobal = NULL;
			m_hResource = NULL;
		}
	}
};


///////////////////////////////////////////////////////////////////////////////
// Toolbar resource descriptor

struct _AtlToolBarData
{
	WORD wVersion;
	WORD wWidth;
	WORD wHeight;
	WORD wItemCount;

	WORD* items()
		{ return (WORD*)(this+1); }
};


///////////////////////////////////////////////////////////////////////////////
// Global functions for loading resources

inline HACCEL AtlLoadAccelerators(ATL::_U_STRINGorID table)
{
	return ::LoadAccelerators(ModuleHelper::GetResourceInstance(), table.m_lpstr);
}

inline HMENU AtlLoadMenu(ATL::_U_STRINGorID menu)
{
	return ::LoadMenu(ModuleHelper::GetResourceInstance(), menu.m_lpstr);
}

inline HBITMAP AtlLoadBitmap(ATL::_U_STRINGorID bitmap)
{
	return ::LoadBitmap(ModuleHelper::GetResourceInstance(), bitmap.m_lpstr);
}

#ifdef OEMRESOURCE
inline HBITMAP AtlLoadSysBitmap(ATL::_U_STRINGorID bitmap)
{
#ifdef _DEBUG
	WORD wID = LOWORD(bitmap.m_lpstr);
	ATLASSERT((wID >= 32734) && (wID <= 32767));
#endif // _DEBUG
	return ::LoadBitmap(NULL, bitmap.m_lpstr);
}
#endif // OEMRESOURCE

inline HCURSOR AtlLoadCursor(ATL::_U_STRINGorID cursor)
{
	return ::LoadCursor(ModuleHelper::GetResourceInstance(), cursor.m_lpstr);
}

inline HCURSOR AtlLoadSysCursor(LPCTSTR lpCursorName)
{
	ATLASSERT((lpCursorName == IDC_ARROW) || (lpCursorName == IDC_IBEAM) || (lpCursorName == IDC_WAIT) ||
		(lpCursorName == IDC_CROSS) || (lpCursorName == IDC_UPARROW) || (lpCursorName == IDC_SIZE) ||
		(lpCursorName == IDC_ICON) || (lpCursorName == IDC_SIZENWSE) || (lpCursorName == IDC_SIZENESW) ||
		(lpCursorName == IDC_SIZEWE) || (lpCursorName == IDC_SIZENS) || (lpCursorName == IDC_SIZEALL) ||
		(lpCursorName == IDC_NO) || (lpCursorName == IDC_APPSTARTING) || (lpCursorName == IDC_HELP) ||
		(lpCursorName == IDC_HAND));
	return ::LoadCursor(NULL, lpCursorName);
}

inline HICON AtlLoadIcon(ATL::_U_STRINGorID icon)
{
	return ::LoadIcon(ModuleHelper::GetResourceInstance(), icon.m_lpstr);
}

inline HICON AtlLoadSysIcon(LPCTSTR lpIconName)
{
#if (WINVER >= 0x0600)
	ATLASSERT((lpIconName == IDI_APPLICATION) || (lpIconName == IDI_ASTERISK) || (lpIconName == IDI_EXCLAMATION) ||
	          (lpIconName == IDI_HAND) || (lpIconName == IDI_QUESTION) || (lpIconName == IDI_WINLOGO) ||
	          (lpIconName == IDI_SHIELD));
#else // !(WINVER >= 0x0600)
	ATLASSERT((lpIconName == IDI_APPLICATION) || (lpIconName == IDI_ASTERISK) || (lpIconName == IDI_EXCLAMATION) ||
	          (lpIconName == IDI_HAND) || (lpIconName == IDI_QUESTION) || (lpIconName == IDI_WINLOGO));
#endif // !(WINVER >= 0x0600)
	return ::LoadIcon(NULL, lpIconName);
}

inline HBITMAP AtlLoadBitmapImage(ATL::_U_STRINGorID bitmap, UINT fuLoad = LR_DEFAULTCOLOR)
{
	return (HBITMAP)::LoadImage(ModuleHelper::GetResourceInstance(), bitmap.m_lpstr, IMAGE_BITMAP, 0, 0, fuLoad);
}

inline HCURSOR AtlLoadCursorImage(ATL::_U_STRINGorID cursor, UINT fuLoad = LR_DEFAULTCOLOR | LR_DEFAULTSIZE, int cxDesired = 0, int cyDesired = 0)
{
	return (HCURSOR)::LoadImage(ModuleHelper::GetResourceInstance(), cursor.m_lpstr, IMAGE_CURSOR, cxDesired, cyDesired, fuLoad);
}

inline HICON AtlLoadIconImage(ATL::_U_STRINGorID icon, UINT fuLoad = LR_DEFAULTCOLOR | LR_DEFAULTSIZE, int cxDesired = 0, int cyDesired = 0)
{
	return (HICON)::LoadImage(ModuleHelper::GetResourceInstance(), icon.m_lpstr, IMAGE_ICON, cxDesired, cyDesired, fuLoad);
}

#ifdef OEMRESOURCE
inline HBITMAP AtlLoadSysBitmapImage(WORD wBitmapID, UINT fuLoad = LR_DEFAULTCOLOR)
{
	ATLASSERT((wBitmapID >= 32734) && (wBitmapID <= 32767));
	ATLASSERT((fuLoad & LR_LOADFROMFILE) == 0);   // this one doesn't load from a file
	return (HBITMAP)::LoadImage(NULL, MAKEINTRESOURCE(wBitmapID), IMAGE_BITMAP, 0, 0, fuLoad);
}
#endif // OEMRESOURCE

inline HCURSOR AtlLoadSysCursorImage(ATL::_U_STRINGorID cursor, UINT fuLoad = LR_DEFAULTCOLOR | LR_DEFAULTSIZE, int cxDesired = 0, int cyDesired = 0)
{
#ifdef _DEBUG
	WORD wID = LOWORD(cursor.m_lpstr);
	ATLASSERT(((wID >= 32512) && (wID <= 32516)) || ((wID >= 32640) && (wID <= 32648)) || (wID == 32650) || (wID == 32651));
	ATLASSERT((fuLoad & LR_LOADFROMFILE) == 0);   // this one doesn't load from a file
#endif // _DEBUG
	return (HCURSOR)::LoadImage(NULL, cursor.m_lpstr, IMAGE_CURSOR, cxDesired, cyDesired, fuLoad);
}

inline HICON AtlLoadSysIconImage(ATL::_U_STRINGorID icon, UINT fuLoad = LR_DEFAULTCOLOR | LR_DEFAULTSIZE, int cxDesired = 0, int cyDesired = 0)
{
#ifdef _DEBUG
	WORD wID = LOWORD(icon.m_lpstr);
	ATLASSERT((wID >= 32512) && (wID <= 32517));
	ATLASSERT((fuLoad & LR_LOADFROMFILE) == 0);   // this one doesn't load from a file
#endif // _DEBUG
	return (HICON)::LoadImage(NULL, icon.m_lpstr, IMAGE_ICON, cxDesired, cyDesired, fuLoad);
}

inline bool AtlLoadString(UINT uID, BSTR& bstrText)
{
	USES_CONVERSION;
	ATLASSERT(bstrText == NULL);

	LPTSTR lpstrText = NULL;
	int nRes = 0;
	for(int nLen = 256; ; nLen *= 2)
	{
		ATLTRY(lpstrText = new TCHAR[nLen]);
		if(lpstrText == NULL)
			break;
		nRes = ::LoadString(ModuleHelper::GetResourceInstance(), uID, lpstrText, nLen);
		if(nRes < nLen - 1)
			break;
		delete [] lpstrText;
		lpstrText = NULL;
	}

	if(lpstrText != NULL)
	{
		if(nRes != 0)
			bstrText = ::SysAllocString(T2OLE(lpstrText));
		delete [] lpstrText;
	}

	return (bstrText != NULL) ? true : false;
}

} // namespace WTL

#endif // __ATLUSER_H__
