// Windows Template Library - WTL version 10.0
// Copyright (C) Microsoft Corporation, WTL Team. All rights reserved.
//
// This file is a part of the Windows Template Library.
// The use and distribution terms for this software are covered by the
// Microsoft Public License (http://opensource.org/licenses/MS-PL)
// which can be found in the file MS-PL.txt at the root folder.

#ifndef __ATLDLGS_H__
#define __ATLDLGS_H__

#pragma once

#ifndef __ATLAPP_H__
	#error atldlgs.h requires atlapp.h to be included first
#endif

#ifndef __ATLWIN_H__
	#error atldlgs.h requires atlwin.h to be included first
#endif

#include <shlobj.h>

#if (_WIN32_WINNT >= 0x0600)
  #include <shobjidl.h>
#endif // (_WIN32_WINNT >= 0x0600)


///////////////////////////////////////////////////////////////////////////////
// Classes in this file:
//
// CFileDialogImpl<T>
// CFileDialog
// CSimpleFileDialog
// CMultiFileDialogImpl<T>
// CMultiFileDialog
// CShellFileDialogImpl<T>
// CShellFileOpenDialogImpl<T>
// CShellFileOpenDialog
// CShellFileSaveDialogImpl<T>
// CShellFileSaveDialog
// CFolderDialogImpl<T>
// CFolderDialog
// CFontDialogImpl<T>
// CFontDialog
// CRichEditFontDialogImpl<T>
// CRichEditFontDialog
// CColorDialogImpl<T>
// CColorDialog
// CPrintDialogImpl<T>
// CPrintDialog
// CPrintDialogExImpl<T>
// CPrintDialogEx
// CPageSetupDialogImpl<T>
// CPageSetupDialog
// CFindReplaceDialogImpl<T>
// CFindReplaceDialog
//
// CDialogBaseUnits
// CMemDlgTemplate
// CIndirectDialogImpl<T, TDlgTemplate, TBase>
//
// CPropertySheetWindow
// CPropertySheetImpl<T, TBase>
// CPropertySheet
// CPropertyPageWindow
// CPropertyPageImpl<T, TBase>
// CPropertyPage<t_wDlgTemplateID>
// CAxPropertyPageImpl<T, TBase>
// CAxPropertyPage<t_wDlgTemplateID>
//
// CWizard97SheetWindow
// CWizard97SheetImpl<T, TBase>
// CWizard97Sheet
// CWizard97PageWindow
// CWizard97PageImpl<T, TBase>
// CWizard97ExteriorPageImpl<T, TBase>
// CWizard97InteriorPageImpl<T, TBase>
//
// CAeroWizardFrameWindow
// CAeroWizardFrameImpl<T, TBase>
// CAeroWizardFrame
// CAeroWizardPageWindow
// CAeroWizardPageImpl<T, TBase>
// CAeroWizardPage<t_wDlgTemplateID>
// CAeroWizardAxPageImpl<T, TBase>
// CAeroWizardAxPage<t_wDlgTemplateID>
//
// CTaskDialogConfig
// CTaskDialogImpl<T>
// CTaskDialog
//
// Global functions:
//   AtlTaskDialog()


namespace WTL
{

///////////////////////////////////////////////////////////////////////////////
// CFileDialogImpl - used for File Open or File Save As

template <class T>
class ATL_NO_VTABLE CFileDialogImpl : public ATL::CDialogImplBase
{
public:
	OPENFILENAME m_ofn;
	BOOL m_bOpenFileDialog;            // TRUE for file open, FALSE for file save
	TCHAR m_szFileTitle[_MAX_FNAME];   // contains file title after return
	TCHAR m_szFileName[_MAX_PATH];     // contains full path name after return

	CFileDialogImpl(BOOL bOpenFileDialog, // TRUE for FileOpen, FALSE for FileSaveAs
			LPCTSTR lpszDefExt = NULL,
			LPCTSTR lpszFileName = NULL,
			DWORD dwFlags = OFN_HIDEREADONLY | OFN_OVERWRITEPROMPT,
			LPCTSTR lpszFilter = NULL,
			HWND hWndParent = NULL) : m_bOpenFileDialog(bOpenFileDialog)
	{
		memset(&m_ofn, 0, sizeof(m_ofn)); // initialize structure to 0/NULL
		m_ofn.lStructSize = sizeof(m_ofn);
		m_ofn.lpstrFile = m_szFileName;
		m_ofn.nMaxFile = _MAX_PATH;
		m_ofn.lpstrDefExt = lpszDefExt;
		m_ofn.lpstrFileTitle = (LPTSTR)m_szFileTitle;
		m_ofn.nMaxFileTitle = _MAX_FNAME;
		m_ofn.Flags = dwFlags | OFN_EXPLORER | OFN_ENABLEHOOK | OFN_ENABLESIZING;
		m_ofn.lpstrFilter = lpszFilter;
		m_ofn.hInstance = ModuleHelper::GetResourceInstance();
		m_ofn.lpfnHook = (LPOFNHOOKPROC)T::StartDialogProc;
		m_ofn.hwndOwner = hWndParent;

		m_szFileName[0] = _T('\0');
		m_szFileTitle[0] = _T('\0');

		// setup initial file name
		if(lpszFileName != NULL)
			ATL::Checked::tcsncpy_s(m_szFileName, _countof(m_szFileName), lpszFileName, _TRUNCATE);
	}

	INT_PTR DoModal(HWND hWndParent = ::GetActiveWindow())
	{
		ATLASSERT((m_ofn.Flags & OFN_ENABLEHOOK) != 0);
		ATLASSERT(m_ofn.lpfnHook != NULL);   // can still be a user hook

		ATLASSERT((m_ofn.Flags & OFN_EXPLORER) != 0);

		if(m_ofn.hwndOwner == NULL)   // set only if not specified before
			m_ofn.hwndOwner = hWndParent;

		ATLASSERT(m_hWnd == NULL);

		// Allocate the thunk structure here, where we can fail gracefully.
		BOOL bRetTh = m_thunk.Init(NULL, NULL);
		if(bRetTh == FALSE)
		{
			::SetLastError(ERROR_OUTOFMEMORY);
			return -1;
		}

		ModuleHelper::AddCreateWndData(&m_thunk.cd, (ATL::CDialogImplBase*)this);

		BOOL bRet = (m_bOpenFileDialog != FALSE) ? ::GetOpenFileName(&m_ofn) : ::GetSaveFileName(&m_ofn);

		m_hWnd = NULL;

		return (bRet != FALSE) ? IDOK : IDCANCEL;
	}

// Attributes
	ATL::CWindow GetFileDialogWindow() const
	{
		ATLASSERT(::IsWindow(m_hWnd));
		return ATL::CWindow(GetParent());
	}

	int GetFilePath(LPTSTR lpstrFilePath, int nLength) const
	{
		ATLASSERT(::IsWindow(m_hWnd));
		ATLASSERT((m_ofn.Flags & OFN_EXPLORER) != 0);

		return (int)GetFileDialogWindow().SendMessage(CDM_GETFILEPATH, nLength, (LPARAM)lpstrFilePath);
	}

	int GetFolderIDList(LPVOID lpBuff, int nLength) const
	{
		ATLASSERT(::IsWindow(m_hWnd));
		ATLASSERT((m_ofn.Flags & OFN_EXPLORER) != 0);

		return (int)GetFileDialogWindow().SendMessage(CDM_GETFOLDERIDLIST, nLength, (LPARAM)lpBuff);
	}

	int GetFolderPath(LPTSTR lpstrFolderPath, int nLength) const
	{
		ATLASSERT(::IsWindow(m_hWnd));
		ATLASSERT((m_ofn.Flags & OFN_EXPLORER) != 0);

		return (int)GetFileDialogWindow().SendMessage(CDM_GETFOLDERPATH, nLength, (LPARAM)lpstrFolderPath);
	}

	int GetSpec(LPTSTR lpstrSpec, int nLength) const
	{
		ATLASSERT(::IsWindow(m_hWnd));
		ATLASSERT((m_ofn.Flags & OFN_EXPLORER) != 0);

		return (int)GetFileDialogWindow().SendMessage(CDM_GETSPEC, nLength, (LPARAM)lpstrSpec);
	}

	void SetControlText(int nCtrlID, LPCTSTR lpstrText)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		ATLASSERT((m_ofn.Flags & OFN_EXPLORER) != 0);

		GetFileDialogWindow().SendMessage(CDM_SETCONTROLTEXT, nCtrlID, (LPARAM)lpstrText);
	}

	void SetDefExt(LPCTSTR lpstrExt)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		ATLASSERT((m_ofn.Flags & OFN_EXPLORER) != 0);

		GetFileDialogWindow().SendMessage(CDM_SETDEFEXT, 0, (LPARAM)lpstrExt);
	}

	BOOL GetReadOnlyPref() const	// return TRUE if readonly checked
	{
		return ((m_ofn.Flags & OFN_READONLY) != 0) ? TRUE : FALSE;
	}

// Operations
	void HideControl(int nCtrlID)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		ATLASSERT((m_ofn.Flags & OFN_EXPLORER) != 0);

		GetFileDialogWindow().SendMessage(CDM_HIDECONTROL, nCtrlID);
	}

// Special override for common dialogs
	BOOL EndDialog(INT_PTR /*nRetCode*/ = 0)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		GetFileDialogWindow().SendMessage(WM_COMMAND, MAKEWPARAM(IDCANCEL, 0));
		return TRUE;
	}

// Message map and handlers
	BEGIN_MSG_MAP(CFileDialogImpl)
		NOTIFY_CODE_HANDLER(CDN_FILEOK, _OnFileOK)
		NOTIFY_CODE_HANDLER(CDN_FOLDERCHANGE, _OnFolderChange)
		NOTIFY_CODE_HANDLER(CDN_HELP, _OnHelp)
		NOTIFY_CODE_HANDLER(CDN_INITDONE, _OnInitDone)
		NOTIFY_CODE_HANDLER(CDN_SELCHANGE, _OnSelChange)
		NOTIFY_CODE_HANDLER(CDN_SHAREVIOLATION, _OnShareViolation)
		NOTIFY_CODE_HANDLER(CDN_TYPECHANGE, _OnTypeChange)
		NOTIFY_CODE_HANDLER(CDN_INCLUDEITEM, _OnIncludeItem)
	END_MSG_MAP()

	LRESULT _OnFileOK(int /*idCtrl*/, LPNMHDR pnmh, BOOL& /*bHandled*/)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		T* pT = static_cast<T*>(this);
		return !pT->OnFileOK((LPOFNOTIFY)pnmh);
	}

	LRESULT _OnFolderChange(int /*idCtrl*/, LPNMHDR pnmh, BOOL& /*bHandled*/)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		T* pT = static_cast<T*>(this);
		pT->OnFolderChange((LPOFNOTIFY)pnmh);
		return 0;
	}

	LRESULT _OnHelp(int /*idCtrl*/, LPNMHDR pnmh, BOOL& /*bHandled*/)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		T* pT = static_cast<T*>(this);
		pT->OnHelp((LPOFNOTIFY)pnmh);
		return 0;
	}

	LRESULT _OnInitDone(int /*idCtrl*/, LPNMHDR pnmh, BOOL& /*bHandled*/)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		T* pT = static_cast<T*>(this);
		pT->OnInitDone((LPOFNOTIFY)pnmh);
		return 0;
	}

	LRESULT _OnSelChange(int /*idCtrl*/, LPNMHDR pnmh, BOOL& /*bHandled*/)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		T* pT = static_cast<T*>(this);
		pT->OnSelChange((LPOFNOTIFY)pnmh);
		return 0;
	}

	LRESULT _OnShareViolation(int /*idCtrl*/, LPNMHDR pnmh, BOOL& /*bHandled*/)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		T* pT = static_cast<T*>(this);
		return pT->OnShareViolation((LPOFNOTIFY)pnmh);
	}

	LRESULT _OnTypeChange(int /*idCtrl*/, LPNMHDR pnmh, BOOL& /*bHandled*/)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		T* pT = static_cast<T*>(this);
		pT->OnTypeChange((LPOFNOTIFY)pnmh);
		return 0;
	}

	LRESULT _OnIncludeItem(int /*idCtrl*/, LPNMHDR pnmh, BOOL& /*bHandled*/)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		T* pT = static_cast<T*>(this);
		return pT->OnIncludeItem((LPOFNOTIFYEX)pnmh);
	}

// Overrideables
	BOOL OnFileOK(LPOFNOTIFY /*lpon*/)
	{
		return TRUE;
	}

	void OnFolderChange(LPOFNOTIFY /*lpon*/)
	{
	}

	void OnHelp(LPOFNOTIFY /*lpon*/)
	{
	}

	void OnInitDone(LPOFNOTIFY /*lpon*/)
	{
	}

	void OnSelChange(LPOFNOTIFY /*lpon*/)
	{
	}

	int OnShareViolation(LPOFNOTIFY /*lpon*/)
	{
		return 0;
	}

	void OnTypeChange(LPOFNOTIFY /*lpon*/)
	{
	}

	BOOL OnIncludeItem(LPOFNOTIFYEX /*lponex*/)
	{
		return TRUE;   // include item
	}
};

class CFileDialog : public CFileDialogImpl<CFileDialog>
{
public:
	CFileDialog(BOOL bOpenFileDialog, // TRUE for FileOpen, FALSE for FileSaveAs
		LPCTSTR lpszDefExt = NULL,
		LPCTSTR lpszFileName = NULL,
		DWORD dwFlags = OFN_HIDEREADONLY | OFN_OVERWRITEPROMPT,
		LPCTSTR lpszFilter = NULL,
		HWND hWndParent = NULL)
		: CFileDialogImpl<CFileDialog>(bOpenFileDialog, lpszDefExt, lpszFileName, dwFlags, lpszFilter, hWndParent)
	{ }

	// override base class map and references to handlers
	DECLARE_EMPTY_MSG_MAP()
};


///////////////////////////////////////////////////////////////////////////////
// CSimpleFileDialog - simple class for non-customized Open/SaveAs dialogs

class CSimpleFileDialog
{
public:
	OPENFILENAME m_ofn;
	BOOL m_bOpenFileDialog;            // TRUE for file open, FALSE for file save
	TCHAR m_szFileTitle[_MAX_FNAME];   // contains file title after return
	TCHAR m_szFileName[_MAX_PATH];     // contains full path name after return

	CSimpleFileDialog(BOOL bOpenFileDialog, // TRUE for FileOpen, FALSE for FileSaveAs
			LPCTSTR lpszDefExt = NULL,
			LPCTSTR lpszFileName = NULL,
			DWORD dwFlags = OFN_HIDEREADONLY | OFN_OVERWRITEPROMPT,
			LPCTSTR lpszFilter = NULL,
			HWND hWndParent = NULL) : m_bOpenFileDialog(bOpenFileDialog)
	{
		memset(&m_ofn, 0, sizeof(m_ofn)); // initialize structure to 0/NULL
		m_ofn.lStructSize = sizeof(m_ofn);
		m_ofn.lpstrFile = m_szFileName;
		m_ofn.nMaxFile = _MAX_PATH;
		m_ofn.lpstrDefExt = lpszDefExt;
		m_ofn.lpstrFileTitle = (LPTSTR)m_szFileTitle;
		m_ofn.nMaxFileTitle = _MAX_FNAME;
		m_ofn.Flags = dwFlags | OFN_EXPLORER | OFN_ENABLESIZING;
		m_ofn.lpstrFilter = lpszFilter;
		m_ofn.hInstance = ModuleHelper::GetResourceInstance();
		m_ofn.hwndOwner = hWndParent;

		m_szFileName[0] = _T('\0');
		m_szFileTitle[0] = _T('\0');

		// setup initial file name
		if(lpszFileName != NULL)
			ATL::Checked::tcsncpy_s(m_szFileName, _countof(m_szFileName), lpszFileName, _TRUNCATE);
	}

	INT_PTR DoModal(HWND hWndParent = ::GetActiveWindow())
	{
		ATLASSERT((m_ofn.Flags & OFN_EXPLORER) != 0);

		if(m_ofn.hwndOwner == NULL)   // set only if not specified before
			m_ofn.hwndOwner = hWndParent;

		BOOL bRet = (m_bOpenFileDialog != FALSE) ? ::GetOpenFileName(&m_ofn) : ::GetSaveFileName(&m_ofn);

		return (bRet != FALSE) ? IDOK : IDCANCEL;
	}
};


///////////////////////////////////////////////////////////////////////////////
// Multi File Dialog - Multi-select File Open dialog

// The class dynamically resizes the buffer as the file selection changes
// (as described in Knowledge Base article 131462). It also expands selected
// shortcut files to take into account the full path of the target file.
// Note that this doesn't work on Win9x for the old style dialogs, as well as
// on NT for non-Unicode builds. 

#ifndef _WTL_FIXED_OFN_BUFFER_LENGTH
  #define _WTL_FIXED_OFN_BUFFER_LENGTH 0x10000
#endif

template <class T>
class ATL_NO_VTABLE CMultiFileDialogImpl : public CFileDialogImpl< T >
{
public:
	mutable LPCTSTR m_pNextFile; 
#ifndef _UNICODE
	bool m_bIsNT;
#endif

	CMultiFileDialogImpl(
		LPCTSTR lpszDefExt = NULL,
		LPCTSTR lpszFileName = NULL,
		DWORD dwFlags = OFN_HIDEREADONLY,
		LPCTSTR lpszFilter = NULL,
		HWND hWndParent = NULL)
		: CFileDialogImpl<T>(TRUE, lpszDefExt, lpszFileName, dwFlags, lpszFilter, hWndParent), 
		  m_pNextFile(NULL)
	{
		this->m_ofn.Flags |= OFN_ALLOWMULTISELECT;   // Force multiple selection mode

#ifndef _UNICODE
#ifdef _versionhelpers_H_INCLUDED_
		OSVERSIONINFOEX ovi = { sizeof(OSVERSIONINFOEX) };
		ovi.dwPlatformId = VER_PLATFORM_WIN32_NT;
		DWORDLONG const dwlConditionMask = ::VerSetConditionMask(0, VER_PLATFORMID, VER_EQUAL);
		m_bIsNT = (::VerifyVersionInfo(&ovi, VER_PLATFORMID, dwlConditionMask) != FALSE);
#else // !_versionhelpers_H_INCLUDED_
		OSVERSIONINFO ovi = { sizeof(OSVERSIONINFO) };
		::GetVersionEx(&ovi);
		m_bIsNT = (ovi.dwPlatformId == VER_PLATFORM_WIN32_NT);
#endif // _versionhelpers_H_INCLUDED_
		if (m_bIsNT)
		{
			// On NT platforms, GetOpenFileNameA thunks to GetOpenFileNameW and there 
			// is absolutely nothing we can do except to start off with a large buffer.
			ATLVERIFY(ResizeFilenameBuffer(_WTL_FIXED_OFN_BUFFER_LENGTH));
		}
#endif
	}

	~CMultiFileDialogImpl()
	{
		if (this->m_ofn.lpstrFile != this->m_szFileName)   // Free the buffer if we allocated it
			delete[] this->m_ofn.lpstrFile;
	}

// Operations
	// Get the directory that the files were chosen from.
	// The function returns the number of characters copied, not including the terminating zero. 
	// If the buffer is NULL, the function returns the required size, in characters, including the terminating zero.
	// If the function fails, the return value is zero.
	int GetDirectory(LPTSTR pBuffer, int nBufLen) const
	{
		if (this->m_ofn.lpstrFile == NULL)
			return 0;

		LPCTSTR pStr = this->m_ofn.lpstrFile;
		int nLength = lstrlen(pStr);
		if (pStr[nLength + 1] == 0)
		{
			// The OFN buffer contains a single item so extract its path.
			LPCTSTR pSep = _tcsrchr(pStr, _T('\\'));
			if (pSep != NULL)
				nLength = (int)(DWORD_PTR)(pSep - pStr);
		}

		int nRet = 0;
		if (pBuffer == NULL)   // If the buffer is NULL, return the required length
		{
			nRet = nLength + 1;
		}
		else if (nBufLen > nLength)
		{
			ATL::Checked::tcsncpy_s(pBuffer, nBufLen, pStr, nLength);
			nRet = nLength;
		}

		return nRet;
	}

#ifdef __ATLSTR_H__
	bool GetDirectory(ATL::CString& strDir) const
	{
		bool bRet = false;

		int nLength = GetDirectory(NULL, 0);
		if (nLength > 0)
		{
			bRet = (GetDirectory(strDir.GetBuffer(nLength), nLength) > 0);
			strDir.ReleaseBuffer(nLength - 1);
		}

		return bRet;
	}
#endif // __ATLSTR_H__

	// Get the first filename as a pointer into the buffer.
	LPCTSTR GetFirstFileName() const
	{
		if (this->m_ofn.lpstrFile == NULL)
			return NULL;

		m_pNextFile = NULL;   // Reset internal buffer pointer

		LPCTSTR pStr = this->m_ofn.lpstrFile;
		int nLength = lstrlen(pStr);
		if (pStr[nLength + 1] != 0)
		{
			// Multiple items were selected. The first string is the directory,
			// so skip forwards to the second string.
			pStr += nLength + 1;

			// Set up m_pNext so it points to the second item (or null).
			m_pNextFile = pStr;
			GetNextFileName();
		}
		else
		{
			// A single item was selected. Skip forward past the path.
			LPCTSTR pSep = _tcsrchr(pStr, _T('\\'));
			if (pSep != NULL)
				pStr = pSep + 1;
		}

		return pStr;
	}

	// Get the next filename as a pointer into the buffer.
	LPCTSTR GetNextFileName() const
	{
		if (m_pNextFile == NULL)
			return NULL;

		LPCTSTR pStr = m_pNextFile;
		// Set "m_pNextFile" to point to the next file name, or null if we 
		// have reached the last file in the list.
		int nLength = lstrlen(pStr);
		m_pNextFile = (pStr[nLength + 1] != 0) ? &pStr[nLength + 1] : NULL;

		return pStr;
	}

	// Get the first filename as a full path.
	// The function returns the number of characters copied, not including the terminating zero. 
	// If the buffer is NULL, the function returns the required size, in characters, including the terminating zero.
	// If the function fails, the return value is zero.
	int GetFirstPathName(LPTSTR pBuffer, int nBufLen) const
	{
		LPCTSTR pStr = GetFirstFileName();
		int nLengthDir = GetDirectory(NULL, 0);
		if((pStr == NULL) || (nLengthDir == 0))
			return 0;

		// Figure out the required length.
		int nLengthTotal = nLengthDir + lstrlen(pStr);

		int nRet = 0;
		if(pBuffer == NULL) // If the buffer is NULL, return the required length
		{
			nRet = nLengthTotal + 1;
		}
		else if (nBufLen > nLengthTotal) // If the buffer is big enough, go ahead and construct the path
		{		
			GetDirectory(pBuffer, nBufLen);
			ATL::Checked::tcscat_s(pBuffer, nBufLen, _T("\\"));
			ATL::Checked::tcscat_s(pBuffer, nBufLen, pStr);
			nRet = nLengthTotal;
		}

		return nRet;
	}

#ifdef __ATLSTR_H__
	bool GetFirstPathName(ATL::CString& strPath) const
	{
		bool bRet = false;

		int nLength = GetFirstPathName(NULL, 0);
		if (nLength > 0)
		{
			bRet = (GetFirstPathName(strPath.GetBuffer(nLength), nLength) > 0);
			strPath.ReleaseBuffer(nLength - 1);
		}

		return bRet;
	}
#endif // __ATLSTR_H__

	// Get the next filename as a full path.
	// The function returns the number of characters copied, not including the terminating zero. 
	// If the buffer is NULL, the function returns the required size, in characters, including the terminating zero.
	// If the function fails, the return value is zero.
	// The internal position marker is moved forward only if the function succeeds and the buffer was large enough.
	int GetNextPathName(LPTSTR pBuffer, int nBufLen) const
	{
		if (m_pNextFile == NULL)
			return 0;

		int nRet = 0;
		LPCTSTR pStr = m_pNextFile;
		// Does the filename contain a backslash?
		if (_tcsrchr(pStr, _T('\\')) != NULL)
		{
			// Yes, so we'll assume it's a full path.
			int nLength = lstrlen(pStr);

			if (pBuffer == NULL) // If the buffer is NULL, return the required length
			{
				nRet = nLength + 1;
			}
			else if (nBufLen > nLength) // The buffer is big enough, so go ahead and copy the filename
			{
				ATL::Checked::tcscpy_s(pBuffer, nBufLen, GetNextFileName());
				nRet = nBufLen;
			}
		}
		else
		{
			// The filename is relative, so construct the full path.
			int nLengthDir = GetDirectory(NULL, 0);
			if (nLengthDir > 0)
			{
				// Calculate the required space.
				int nLengthTotal = nLengthDir + lstrlen(pStr);

				if(pBuffer == NULL) // If the buffer is NULL, return the required length
				{
					nRet = nLengthTotal + 1;
				}
				else if (nBufLen > nLengthTotal) // If the buffer is big enough, go ahead and construct the path
				{
					GetDirectory(pBuffer, nBufLen);
					ATL::Checked::tcscat_s(pBuffer, nBufLen, _T("\\"));
					ATL::Checked::tcscat_s(pBuffer, nBufLen, GetNextFileName());
					nRet = nLengthTotal;
				}
			}
		}

		return nRet;
	}

#ifdef __ATLSTR_H__
	bool GetNextPathName(ATL::CString& strPath) const
	{
		bool bRet = false;

		int nLength = GetNextPathName(NULL, 0);
		if (nLength > 0)
		{
			bRet = (GetNextPathName(strPath.GetBuffer(nLength), nLength) > 0);
			strPath.ReleaseBuffer(nLength - 1);
		}

		return bRet;
	}
#endif // __ATLSTR_H__

// Implementation
	bool ResizeFilenameBuffer(DWORD dwLength)
	{
		if (dwLength > this->m_ofn.nMaxFile)
		{
			// Free the old buffer.
			if (this->m_ofn.lpstrFile != this->m_szFileName)
			{
				delete[] this->m_ofn.lpstrFile;
				this->m_ofn.lpstrFile = NULL;
				this->m_ofn.nMaxFile = 0;
			}

			// Allocate the new buffer.
			LPTSTR lpstrBuff = NULL;
			ATLTRY(lpstrBuff = new TCHAR[dwLength]);
			if (lpstrBuff != NULL)
			{
				this->m_ofn.lpstrFile = lpstrBuff;
				this->m_ofn.lpstrFile[0] = 0;
				this->m_ofn.nMaxFile = dwLength;
			}
		}

		return (this->m_ofn.lpstrFile != NULL);
	}

	void OnSelChange(LPOFNOTIFY /*lpon*/)
	{
#ifndef _UNICODE
		// There is no point resizing the buffer in ANSI builds running on NT.
		if (m_bIsNT)
			return;
#endif

		// Get the buffer length required to hold the spec.
		int nLength = this->GetSpec(NULL, 0);
		if (nLength <= 1)
			return; // no files are selected, presumably
		
		// Add room for the directory, and an extra terminating zero.
		nLength += this->GetFolderPath(NULL, 0) + 1;

		if (!ResizeFilenameBuffer(nLength))
		{
			ATLASSERT(FALSE);
			return;
		}

		// If we are not following links then our work is done.
		if ((this->m_ofn.Flags & OFN_NODEREFERENCELINKS) != 0)
			return;

		// Get the file spec, which is the text in the edit control.
		if (this->GetSpec(this->m_ofn.lpstrFile, this->m_ofn.nMaxFile) <= 0)
			return;
		
		// Get the ID-list of the current folder.
		int nBytes = this->GetFolderIDList(NULL, 0);
#ifdef STRICT_TYPED_ITEMIDS
		ATL::CTempBuffer<ITEMIDLIST_RELATIVE> idlist;
#else
		ATL::CTempBuffer<ITEMIDLIST> idlist;
#endif
		idlist.AllocateBytes(nBytes);
		if ((nBytes <= 0) || (this->GetFolderIDList(idlist, nBytes) <= 0))
			return;

		// First bind to the desktop folder, then to the current folder.
		ATL::CComPtr<IShellFolder> pDesktop, pFolder;
		if (FAILED(::SHGetDesktopFolder(&pDesktop)))
			return;
		if (FAILED(pDesktop->BindToObject(idlist, NULL, IID_IShellFolder, (void**)&pFolder)))
			return;

		// Work through the file spec, looking for quoted filenames. If we find a shortcut file, then 
		// we need to add enough extra buffer space to hold its target path.
		DWORD nExtraChars = 0;
		bool bInsideQuotes = false;
		LPCTSTR pAnchor = this->m_ofn.lpstrFile;
		LPCTSTR pChar = this->m_ofn.lpstrFile;
		for ( ; *pChar; ++pChar)
		{
			// Look for quotation marks.
			if (*pChar == _T('\"'))
			{
				// We are either entering or leaving a passage of quoted text.
				bInsideQuotes = !bInsideQuotes;

				// Is it an opening or closing quote?
				if (bInsideQuotes)
				{
					// We found an opening quote, so set "pAnchor" to the following character.
					pAnchor = pChar + 1;
				}
				else // closing quote
				{
					// Each quoted entity should be shorter than MAX_PATH.
					if (pChar - pAnchor >= MAX_PATH)
						return;

					// Get the ID-list and attributes of the file.
					USES_CONVERSION;
					int nFileNameLength = (int)(DWORD_PTR)(pChar - pAnchor);
					TCHAR szFileName[MAX_PATH] = {};
					ATL::Checked::tcsncpy_s(szFileName, MAX_PATH, pAnchor, nFileNameLength);
#ifdef STRICT_TYPED_ITEMIDS
					PIDLIST_RELATIVE pidl = NULL;
#else
					LPITEMIDLIST pidl = NULL;
#endif
					DWORD dwAttrib = SFGAO_LINK;
					if (SUCCEEDED(pFolder->ParseDisplayName(NULL, NULL, T2W(szFileName), NULL, &pidl, &dwAttrib)))
					{
						// Is it a shortcut file?
						if (dwAttrib & SFGAO_LINK)
						{
							// Bind to its IShellLink interface.
							ATL::CComPtr<IShellLink> pLink;
							if (SUCCEEDED(pFolder->BindToObject(pidl, NULL, IID_IShellLink, (void**)&pLink)))
							{
								// Get the shortcut's target path.
								TCHAR szPath[MAX_PATH] = {};
								if (SUCCEEDED(pLink->GetPath(szPath, MAX_PATH, NULL, 0)))
								{
									// If the target path is longer than the shortcut name, then add on the number 
									// of extra characters that are required.
									int nNewLength = lstrlen(szPath);
									if (nNewLength > nFileNameLength)
										nExtraChars += nNewLength - nFileNameLength;
								}
							}
						}

						// Free the ID-list returned by ParseDisplayName.
						::CoTaskMemFree(pidl);
					}
				}
			}
		}

		// If we need more space for shortcut targets, then reallocate.
		if (nExtraChars > 0)
			ATLVERIFY(ResizeFilenameBuffer(this->m_ofn.nMaxFile + nExtraChars));
	}
};

class CMultiFileDialog : public CMultiFileDialogImpl<CMultiFileDialog>
{
public:
	CMultiFileDialog(
		LPCTSTR lpszDefExt = NULL,
		LPCTSTR lpszFileName = NULL,
		DWORD dwFlags = OFN_HIDEREADONLY,
		LPCTSTR lpszFilter = NULL,
		HWND hWndParent = NULL)
		: CMultiFileDialogImpl<CMultiFileDialog>(lpszDefExt, lpszFileName, dwFlags, lpszFilter, hWndParent)
	{ }

	BEGIN_MSG_MAP(CMultiFileDialog)
		CHAIN_MSG_MAP(CMultiFileDialogImpl<CMultiFileDialog>)
	END_MSG_MAP()
};


///////////////////////////////////////////////////////////////////////////////
// Shell File Dialog - new Shell File Open and Save dialogs in Vista

// Note: Use GetPtr() to access dialog interface methods.
// Example:
//	CShellFileOpenDialog dlg;
//	dlg.GetPtr()->SetTitle(L"MyFileOpenDialog");

#if (_WIN32_WINNT >= 0x0600)

///////////////////////////////////////////////////////////////////////////////
// CShellFileDialogImpl - base class for CShellFileOpenDialogImpl and CShellFileSaveDialogImpl

template <class T>
class ATL_NO_VTABLE CShellFileDialogImpl : public IFileDialogEvents
{
public:
// Operations
	INT_PTR DoModal(HWND hWndParent = ::GetActiveWindow())
	{
		INT_PTR nRet = -1;

		T* pT = static_cast<T*>(this);
		if(pT->m_spFileDlg == NULL)
		{
			ATLASSERT(FALSE);
			return nRet;
		}

		DWORD dwCookie = 0;
		pT->_Advise(dwCookie);

		HRESULT hRet = pT->m_spFileDlg->Show(hWndParent);
		if(SUCCEEDED(hRet))
			nRet = IDOK;
		else if(hRet == HRESULT_FROM_WIN32(ERROR_CANCELLED))
			nRet = IDCANCEL;
		else
			ATLASSERT(FALSE);   // error

		pT->_Unadvise(dwCookie);

		return nRet;
	}

	bool IsNull() const
	{
		const T* pT = static_cast<const T*>(this);
		return (pT->m_spFileDlg == NULL);
	}

// Operations - get file path after dialog returns
	HRESULT GetFilePath(LPWSTR lpstrFilePath, int cchLength)
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(pT->m_spFileDlg != NULL);

		ATL::CComPtr<IShellItem> spItem;
		HRESULT hRet = pT->m_spFileDlg->GetResult(&spItem);

		if(SUCCEEDED(hRet))
			hRet = GetFileNameFromShellItem(spItem, SIGDN_FILESYSPATH, lpstrFilePath, cchLength);

		return hRet;
	}

	HRESULT GetFileTitle(LPWSTR lpstrFileTitle, int cchLength)
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(pT->m_spFileDlg != NULL);

		ATL::CComPtr<IShellItem> spItem;
		HRESULT hRet = pT->m_spFileDlg->GetResult(&spItem);

		if(SUCCEEDED(hRet))
			hRet = GetFileNameFromShellItem(spItem, SIGDN_NORMALDISPLAY, lpstrFileTitle, cchLength);

		return hRet;
	}

#ifdef __ATLSTR_H__
	HRESULT GetFilePath(ATL::CString& strFilePath)
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(pT->m_spFileDlg != NULL);

		ATL::CComPtr<IShellItem> spItem;
		HRESULT hRet = pT->m_spFileDlg->GetResult(&spItem);

		if(SUCCEEDED(hRet))
			hRet = GetFileNameFromShellItem(spItem, SIGDN_FILESYSPATH, strFilePath);

		return hRet;
	}

	HRESULT GetFileTitle(ATL::CString& strFileTitle)
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(pT->m_spFileDlg != NULL);

		ATL::CComPtr<IShellItem> spItem;
		HRESULT hRet = pT->m_spFileDlg->GetResult(&spItem);

		if(SUCCEEDED(hRet))
			hRet = GetFileNameFromShellItem(spItem, SIGDN_NORMALDISPLAY, strFileTitle);

		return hRet;
	}
#endif // __ATLSTR_H__

// Helpers for IShellItem
	static HRESULT GetFileNameFromShellItem(IShellItem* pShellItem, SIGDN type, LPWSTR lpstr, int cchLength)
	{
		ATLASSERT(pShellItem != NULL);

		LPWSTR lpstrName = NULL;
		HRESULT hRet = pShellItem->GetDisplayName(type, &lpstrName);

		if(SUCCEEDED(hRet))
		{
			if(lstrlenW(lpstrName) < cchLength)
			{
				ATL::Checked::wcscpy_s(lpstr, cchLength, lpstrName);
			}
			else
			{
				ATLASSERT(FALSE);
				hRet = DISP_E_BUFFERTOOSMALL;
			}

			::CoTaskMemFree(lpstrName);
		}

		return hRet;
	}

#ifdef __ATLSTR_H__
	static HRESULT GetFileNameFromShellItem(IShellItem* pShellItem, SIGDN type, ATL::CString& str)
	{
		ATLASSERT(pShellItem != NULL);

		LPWSTR lpstrName = NULL;
		HRESULT hRet = pShellItem->GetDisplayName(type, &lpstrName);

		if(SUCCEEDED(hRet))
		{
			str = lpstrName;
			::CoTaskMemFree(lpstrName);
		}

		return hRet;
	}
#endif // __ATLSTR_H__

// Implementation
	void _Advise(DWORD& dwCookie)
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(pT->m_spFileDlg != NULL);
		HRESULT hRet = pT->m_spFileDlg->Advise((IFileDialogEvents*)this, &dwCookie);
		ATLVERIFY(SUCCEEDED(hRet));
	}

	void _Unadvise(DWORD dwCookie)
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(pT->m_spFileDlg != NULL);
		HRESULT hRet = pT->m_spFileDlg->Unadvise(dwCookie);
		ATLVERIFY(SUCCEEDED(hRet));
	}

	void _Init(LPCWSTR lpszFileName, DWORD dwOptions, LPCWSTR lpszDefExt, const COMDLG_FILTERSPEC* arrFilterSpec, UINT uFilterSpecCount)
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(pT->m_spFileDlg != NULL);

		HRESULT hRet = E_FAIL;

		if(lpszFileName != NULL)
		{
			hRet = pT->m_spFileDlg->SetFileName(lpszFileName);
			ATLASSERT(SUCCEEDED(hRet));
		}

		hRet = pT->m_spFileDlg->SetOptions(dwOptions);
		ATLASSERT(SUCCEEDED(hRet));

		if(lpszDefExt != NULL)
		{
			hRet = pT->m_spFileDlg->SetDefaultExtension(lpszDefExt);
			ATLASSERT(SUCCEEDED(hRet));
		}

		if((arrFilterSpec != NULL) && (uFilterSpecCount != 0U))
		{
			hRet = pT->m_spFileDlg->SetFileTypes(uFilterSpecCount, arrFilterSpec);
			ATLASSERT(SUCCEEDED(hRet));
		}
	}

// Implementation - IUnknown interface
	STDMETHOD(QueryInterface)(REFIID riid, void** ppvObject)
	{
		if(ppvObject == NULL)
			return E_POINTER;

		T* pT = static_cast<T*>(this);
		if(IsEqualGUID(riid, IID_IUnknown) || IsEqualGUID(riid, IID_IFileDialogEvents))
		{
			*ppvObject = (IFileDialogEvents*)pT;
			// AddRef() not needed
			return S_OK;
		}

		return E_NOINTERFACE;
	}

	virtual ULONG STDMETHODCALLTYPE AddRef()
	{
		return 1;
	}

	virtual ULONG STDMETHODCALLTYPE Release()
	{
		return 1;
	}

// Implementation - IFileDialogEvents interface
	virtual HRESULT STDMETHODCALLTYPE OnFileOk(IFileDialog* pfd)
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(pT->m_spFileDlg.IsEqualObject(pfd));
		(void)pfd;   // avoid level 4 warning
		return pT->OnFileOk();
	}

	virtual HRESULT STDMETHODCALLTYPE OnFolderChanging(IFileDialog* pfd, IShellItem* psiFolder)
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(pT->m_spFileDlg.IsEqualObject(pfd));
		(void)pfd;   // avoid level 4 warning
		return pT->OnFolderChanging(psiFolder);
	}

	virtual HRESULT STDMETHODCALLTYPE OnFolderChange(IFileDialog* pfd)
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(pT->m_spFileDlg.IsEqualObject(pfd));
		(void)pfd;   // avoid level 4 warning
		return pT->OnFolderChange();
	}

	virtual HRESULT STDMETHODCALLTYPE OnSelectionChange(IFileDialog* pfd)
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(pT->m_spFileDlg.IsEqualObject(pfd));
		(void)pfd;   // avoid level 4 warning
		return pT->OnSelectionChange();
	}

	virtual HRESULT STDMETHODCALLTYPE OnShareViolation(IFileDialog* pfd, IShellItem* psi, FDE_SHAREVIOLATION_RESPONSE* pResponse)
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(pT->m_spFileDlg.IsEqualObject(pfd));
		(void)pfd;   // avoid level 4 warning
		return pT->OnShareViolation(psi, pResponse);
	}

	virtual HRESULT STDMETHODCALLTYPE OnTypeChange(IFileDialog* pfd)
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(pT->m_spFileDlg.IsEqualObject(pfd));
		(void)pfd;   // avoid level 4 warning
		return pT->OnTypeChange();
	}

	virtual HRESULT STDMETHODCALLTYPE OnOverwrite(IFileDialog* pfd, IShellItem* psi, FDE_OVERWRITE_RESPONSE* pResponse)
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(pT->m_spFileDlg.IsEqualObject(pfd));
		(void)pfd;   // avoid level 4 warning
		return pT->OnOverwrite(psi, pResponse);
	}

// Overrideables - Event handlers
	HRESULT OnFileOk()
	{
		return E_NOTIMPL;
	}

	HRESULT OnFolderChanging(IShellItem* /*psiFolder*/)
	{
		return E_NOTIMPL;
	}

	HRESULT OnFolderChange()
	{
		return E_NOTIMPL;
	}

	HRESULT OnSelectionChange()
	{
		return E_NOTIMPL;
	}

	HRESULT OnShareViolation(IShellItem* /*psi*/, FDE_SHAREVIOLATION_RESPONSE* /*pResponse*/)
	{
		return E_NOTIMPL;
	}

	HRESULT OnTypeChange()
	{
		return E_NOTIMPL;
	}

	HRESULT OnOverwrite(IShellItem* /*psi*/, FDE_OVERWRITE_RESPONSE* /*pResponse*/)
	{
		return E_NOTIMPL;
	}
};


///////////////////////////////////////////////////////////////////////////////
// CShellFileOpenDialogImpl - implements new Shell File Open dialog

template <class T>
class ATL_NO_VTABLE CShellFileOpenDialogImpl : public CShellFileDialogImpl< T >
{
public:
	ATL::CComPtr<IFileOpenDialog> m_spFileDlg;

	CShellFileOpenDialogImpl(LPCWSTR lpszFileName = NULL, 
	                         DWORD dwOptions = FOS_FORCEFILESYSTEM | FOS_PATHMUSTEXIST | FOS_FILEMUSTEXIST, 
	                         LPCWSTR lpszDefExt = NULL, 
	                         const COMDLG_FILTERSPEC* arrFilterSpec = NULL, 
	                         UINT uFilterSpecCount = 0U)
	{
		HRESULT hRet = m_spFileDlg.CoCreateInstance(CLSID_FileOpenDialog);

		if(SUCCEEDED(hRet))
			this->_Init(lpszFileName, dwOptions, lpszDefExt, arrFilterSpec, uFilterSpecCount);
	}

	virtual ~CShellFileOpenDialogImpl()
	{ }

	IFileOpenDialog* GetPtr()
	{
		return m_spFileDlg;
	}
};


///////////////////////////////////////////////////////////////////////////////
// CShellFileOpenDialog - new Shell File Open dialog without events

class CShellFileOpenDialog : public CShellFileOpenDialogImpl<CShellFileOpenDialog>
{
public:
	CShellFileOpenDialog(LPCWSTR lpszFileName = NULL, 
	                     DWORD dwOptions = FOS_FORCEFILESYSTEM | FOS_PATHMUSTEXIST | FOS_FILEMUSTEXIST, 
	                     LPCWSTR lpszDefExt = NULL, 
	                     const COMDLG_FILTERSPEC* arrFilterSpec = NULL, 
	                     UINT uFilterSpecCount = 0U) : CShellFileOpenDialogImpl<CShellFileOpenDialog>(lpszFileName, dwOptions, lpszDefExt, arrFilterSpec, uFilterSpecCount)
	{ }

	virtual ~CShellFileOpenDialog()
	{ }

// Implementation (remove _Advise/_Unadvise code using template magic)
	void _Advise(DWORD& /*dwCookie*/)
	{ }

	void _Unadvise(DWORD /*dwCookie*/)
	{ }
};


///////////////////////////////////////////////////////////////////////////////
// CShellFileSaveDialogImpl - implements new Shell File Save dialog

template <class T>
class ATL_NO_VTABLE CShellFileSaveDialogImpl : public CShellFileDialogImpl< T >
{
public:
	ATL::CComPtr<IFileSaveDialog> m_spFileDlg;

	CShellFileSaveDialogImpl(LPCWSTR lpszFileName = NULL, 
	                         DWORD dwOptions = FOS_FORCEFILESYSTEM | FOS_PATHMUSTEXIST | FOS_OVERWRITEPROMPT, 
	                         LPCWSTR lpszDefExt = NULL, 
	                         const COMDLG_FILTERSPEC* arrFilterSpec = NULL, 
	                         UINT uFilterSpecCount = 0U)
	{
		HRESULT hRet = m_spFileDlg.CoCreateInstance(CLSID_FileSaveDialog);

		if(SUCCEEDED(hRet))
			this->_Init(lpszFileName, dwOptions, lpszDefExt, arrFilterSpec, uFilterSpecCount);
	}

	virtual ~CShellFileSaveDialogImpl()
	{ }

	IFileSaveDialog* GetPtr()
	{
		return m_spFileDlg;
	}
};


///////////////////////////////////////////////////////////////////////////////
// CShellFileSaveDialog - new Shell File Save dialog without events

class CShellFileSaveDialog : public CShellFileSaveDialogImpl<CShellFileSaveDialog>
{
public:
	CShellFileSaveDialog(LPCWSTR lpszFileName = NULL, 
	                     DWORD dwOptions = FOS_FORCEFILESYSTEM | FOS_PATHMUSTEXIST | FOS_OVERWRITEPROMPT, 
	                     LPCWSTR lpszDefExt = NULL, 
	                     const COMDLG_FILTERSPEC* arrFilterSpec = NULL, 
	                     UINT uFilterSpecCount = 0U) : CShellFileSaveDialogImpl<CShellFileSaveDialog>(lpszFileName, dwOptions, lpszDefExt, arrFilterSpec, uFilterSpecCount)
	{ }

	virtual ~CShellFileSaveDialog()
	{ }

// Implementation (remove _Advise/_Unadvise code using template magic)
	void _Advise(DWORD& /*dwCookie*/)
	{ }

	void _Unadvise(DWORD /*dwCookie*/)
	{ }
};

#endif // (_WIN32_WINNT >= 0x0600)


///////////////////////////////////////////////////////////////////////////////
// CFolderDialogImpl - used for browsing for a folder

template <class T>
class ATL_NO_VTABLE CFolderDialogImpl
{
public:
	BROWSEINFO m_bi;
	LPCTSTR m_lpstrInitialFolder;
	LPCITEMIDLIST m_pidlInitialSelection;
	bool m_bExpandInitialSelection;
	TCHAR m_szFolderDisplayName[MAX_PATH];
	TCHAR m_szFolderPath[MAX_PATH];
#ifdef STRICT_TYPED_ITEMIDS
	PIDLIST_ABSOLUTE m_pidlSelected;
#else
	LPITEMIDLIST m_pidlSelected;
#endif
	HWND m_hWnd;   // used only in the callback function

// Constructor
	CFolderDialogImpl(HWND hWndParent = NULL, LPCTSTR lpstrTitle = NULL, UINT uFlags = BIF_RETURNONLYFSDIRS) : 
			m_lpstrInitialFolder(NULL), m_pidlInitialSelection(NULL), m_bExpandInitialSelection(false), m_pidlSelected(NULL), m_hWnd(NULL)
	{
		memset(&m_bi, 0, sizeof(m_bi)); // initialize structure to 0/NULL

		m_bi.hwndOwner = hWndParent;
		m_bi.pidlRoot = NULL;
		m_bi.pszDisplayName = m_szFolderDisplayName;
		m_bi.lpszTitle = lpstrTitle;
		m_bi.ulFlags = uFlags;
		m_bi.lpfn = BrowseCallbackProc;
		m_bi.lParam = (LPARAM)static_cast<T*>(this);

		m_szFolderPath[0] = 0;
		m_szFolderDisplayName[0] = 0;
	}

	~CFolderDialogImpl()
	{
		::CoTaskMemFree(m_pidlSelected);
	}

// Operations
	INT_PTR DoModal(HWND hWndParent = ::GetActiveWindow())
	{
		if(m_bi.hwndOwner == NULL)   // set only if not specified before
			m_bi.hwndOwner = hWndParent;

		// Clear out any previous results
		m_szFolderPath[0] = 0;
		m_szFolderDisplayName[0] = 0;
		::CoTaskMemFree(m_pidlSelected);

		INT_PTR nRet = IDCANCEL;
		m_pidlSelected = ::SHBrowseForFolder(&m_bi);

		if(m_pidlSelected != NULL)
		{
			nRet = IDOK;

			// If BIF_RETURNONLYFSDIRS is set, we try to get the filesystem path.
			// Otherwise, the caller must handle the ID-list directly.
			if((m_bi.ulFlags & BIF_RETURNONLYFSDIRS) != 0)
			{
				if(::SHGetPathFromIDList(m_pidlSelected, m_szFolderPath) == FALSE)
					nRet = IDCANCEL;
			}
		}

		return nRet;
	}

	// Methods to call before DoModal
	void SetInitialFolder(LPCTSTR lpstrInitialFolder, bool bExpand = true)
	{
		// lpstrInitialFolder may be a file if BIF_BROWSEINCLUDEFILES is specified
		m_lpstrInitialFolder = lpstrInitialFolder;
		m_bExpandInitialSelection = bExpand;
	}

	void SetInitialSelection(LPCITEMIDLIST pidl, bool bExpand = true)
	{
		m_pidlInitialSelection = pidl;
		m_bExpandInitialSelection = bExpand;
	}

#ifdef STRICT_TYPED_ITEMIDS
	void SetRootFolder(PCIDLIST_ABSOLUTE pidl)
#else
	void SetRootFolder(LPCITEMIDLIST pidl)
#endif
	{
		m_bi.pidlRoot = pidl;
	}

	// Methods to call after DoModal
	LPITEMIDLIST GetSelectedItem(bool bDetach = false)
	{
		LPITEMIDLIST pidl = m_pidlSelected;
		if(bDetach)
			m_pidlSelected = NULL;

		return pidl;
	}

	LPCTSTR GetFolderPath() const
	{
		return m_szFolderPath;
	}

	LPCTSTR GetFolderDisplayName() const
	{
		return m_szFolderDisplayName;
	}

	int GetFolderImageIndex() const
	{
		return m_bi.iImage;
	}

// Callback function and overrideables
	static int CALLBACK BrowseCallbackProc(HWND hWnd, UINT uMsg, LPARAM lParam, LPARAM lpData)
	{
		int nRet = 0;
		T* pT = (T*)lpData;
		bool bClear = false;
		if(pT->m_hWnd == NULL)
		{
			pT->m_hWnd = hWnd;
			bClear = true;
		}
		else
		{
			ATLASSERT(pT->m_hWnd == hWnd);
		}

		switch(uMsg)
		{
		case BFFM_INITIALIZED:
			// Set initial selection
			// Note that m_pidlInitialSelection, if set, takes precedence over m_lpstrInitialFolder
			if(pT->m_pidlInitialSelection != NULL)
				pT->SetSelection(pT->m_pidlInitialSelection);
			else if(pT->m_lpstrInitialFolder != NULL)
				pT->SetSelection(pT->m_lpstrInitialFolder);

			// Expand initial selection if appropriate
			if(pT->m_bExpandInitialSelection && ((pT->m_bi.ulFlags & BIF_NEWDIALOGSTYLE) != 0))
			{
				if(pT->m_pidlInitialSelection != NULL)
					pT->SetExpanded(pT->m_pidlInitialSelection);
				else if(pT->m_lpstrInitialFolder != NULL)
					pT->SetExpanded(pT->m_lpstrInitialFolder);
			}
			pT->OnInitialized();
			break;
		case BFFM_SELCHANGED:
			pT->OnSelChanged((LPITEMIDLIST)lParam);
			break;
		case BFFM_VALIDATEFAILED:
			nRet = pT->OnValidateFailed((LPCTSTR)lParam);
			break;
		case BFFM_IUNKNOWN:
			pT->OnIUnknown((IUnknown*)lParam);
			break;
		default:
			ATLTRACE2(atlTraceUI, 0, _T("Unknown message received in CFolderDialogImpl::BrowseCallbackProc\n"));
			break;
		}

		if(bClear)
			pT->m_hWnd = NULL;
		return nRet;
	}

	void OnInitialized()
	{
	}

	void OnSelChanged(LPITEMIDLIST /*pItemIDList*/)
	{
	}

	int OnValidateFailed(LPCTSTR /*lpstrFolderPath*/)
	{
		return 1;   // 1=continue, 0=EndDialog
	}

	void OnIUnknown(IUnknown* /*pUnknown*/)
	{
	}

	// Commands - valid to call only from handlers
	void EnableOK(BOOL bEnable)
	{
		ATLASSERT(m_hWnd != NULL);
		::SendMessage(m_hWnd, BFFM_ENABLEOK, 0, bEnable);
	}

	void SetSelection(LPCITEMIDLIST pItemIDList)
	{
		ATLASSERT(m_hWnd != NULL);
		::SendMessage(m_hWnd, BFFM_SETSELECTION, FALSE, (LPARAM)pItemIDList);
	}

	void SetSelection(LPCTSTR lpstrFolderPath)
	{
		ATLASSERT(m_hWnd != NULL);
		::SendMessage(m_hWnd, BFFM_SETSELECTION, TRUE, (LPARAM)lpstrFolderPath);
	}

	void SetStatusText(LPCTSTR lpstrText)
	{
		ATLASSERT(m_hWnd != NULL);
		::SendMessage(m_hWnd, BFFM_SETSTATUSTEXT, 0, (LPARAM)lpstrText);
	}

	void SetOKText(LPCTSTR lpstrOKText)
	{
		ATLASSERT(m_hWnd != NULL);
		USES_CONVERSION;
		LPCWSTR lpstr = T2CW(lpstrOKText);
		::SendMessage(m_hWnd, BFFM_SETOKTEXT, 0, (LPARAM)lpstr);
	}

	void SetExpanded(LPCITEMIDLIST pItemIDList)
	{
		ATLASSERT(m_hWnd != NULL);
		::SendMessage(m_hWnd, BFFM_SETEXPANDED, FALSE, (LPARAM)pItemIDList);
	}

	void SetExpanded(LPCTSTR lpstrFolderPath)
	{
		ATLASSERT(m_hWnd != NULL);
		USES_CONVERSION;
		LPCWSTR lpstr = T2CW(lpstrFolderPath);
		::SendMessage(m_hWnd, BFFM_SETEXPANDED, TRUE, (LPARAM)lpstr);
	}
};

class CFolderDialog : public CFolderDialogImpl<CFolderDialog>
{
public:
	CFolderDialog(HWND hWndParent = NULL, LPCTSTR lpstrTitle = NULL, UINT uFlags = BIF_RETURNONLYFSDIRS)
		: CFolderDialogImpl<CFolderDialog>(hWndParent, lpstrTitle, uFlags)
	{ }
};


///////////////////////////////////////////////////////////////////////////////
// CCommonDialogImplBase - base class for common dialog classes

class ATL_NO_VTABLE CCommonDialogImplBase : public ATL::CWindowImplBase
{
public:
	static UINT_PTR APIENTRY HookProc(HWND hWnd, UINT uMsg, WPARAM wParam, LPARAM lParam)
	{
		if(uMsg != WM_INITDIALOG)
			return 0;
		CCommonDialogImplBase* pT = (CCommonDialogImplBase*)ModuleHelper::ExtractCreateWndData();
		ATLASSERT(pT != NULL);
		ATLASSERT(pT->m_hWnd == NULL);
		ATLASSERT(::IsWindow(hWnd));
		// subclass dialog's window
		if(!pT->SubclassWindow(hWnd))
		{
			ATLTRACE2(atlTraceUI, 0, _T("Subclassing a common dialog failed\n"));
			return 0;
		}
		// check message map for WM_INITDIALOG handler
		LRESULT lRes = 0;
		if(pT->ProcessWindowMessage(pT->m_hWnd, uMsg, wParam, lParam, lRes, 0) == FALSE)
			return 0;
		return lRes;
	}

// Special override for common dialogs
	BOOL EndDialog(INT_PTR /*nRetCode*/ = 0)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		SendMessage(WM_COMMAND, MAKEWPARAM(IDABORT, 0));
		return TRUE;
	}

// Implementation - try to override these, to prevent errors
	HWND Create(HWND, ATL::_U_RECT, LPCTSTR, DWORD, DWORD, ATL::_U_MENUorID, ATOM, LPVOID)
	{
		ATLASSERT(FALSE);   // should not be called
		return NULL;
	}

	static LRESULT CALLBACK StartWindowProc(HWND /*hWnd*/, UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/)
	{
		ATLASSERT(FALSE);   // should not be called
		return 0;
	}
};


///////////////////////////////////////////////////////////////////////////////
// CFontDialogImpl - font selection dialog

template <class T>
class ATL_NO_VTABLE CFontDialogImpl : public CCommonDialogImplBase
{
public:
	enum { _cchStyleName = 64 };

	CHOOSEFONT m_cf;
	TCHAR m_szStyleName[_cchStyleName];  // contains style name after return
	LOGFONT m_lf;                        // default LOGFONT to store the info

// Constructors
	CFontDialogImpl(LPLOGFONT lplfInitial = NULL,
			DWORD dwFlags = CF_EFFECTS | CF_SCREENFONTS,
			HDC hDCPrinter = NULL,
			HWND hWndParent = NULL)
	{
		memset(&m_cf, 0, sizeof(m_cf));
		memset(&m_lf, 0, sizeof(m_lf));
		memset(&m_szStyleName, 0, sizeof(m_szStyleName));

		m_cf.lStructSize = sizeof(m_cf);
		m_cf.hwndOwner = hWndParent;
		m_cf.rgbColors = RGB(0, 0, 0);
		m_cf.lpszStyle = (LPTSTR)&m_szStyleName;
		m_cf.Flags = dwFlags | CF_ENABLEHOOK;
		m_cf.lpfnHook = (LPCFHOOKPROC)T::HookProc;

		if(lplfInitial != NULL)
		{
			m_cf.lpLogFont = lplfInitial;
			m_cf.Flags |= CF_INITTOLOGFONTSTRUCT;
			m_lf = *lplfInitial;
		}
		else
		{
			m_cf.lpLogFont = &m_lf;
		}

		if(hDCPrinter != NULL)
		{
			m_cf.hDC = hDCPrinter;
			m_cf.Flags |= CF_PRINTERFONTS;
		}
	}

// Operations
	INT_PTR DoModal(HWND hWndParent = ::GetActiveWindow())
	{
		ATLASSERT((m_cf.Flags & CF_ENABLEHOOK) != 0);
		ATLASSERT(m_cf.lpfnHook != NULL);   // can still be a user hook

		if(m_cf.hwndOwner == NULL)          // set only if not specified before
			m_cf.hwndOwner = hWndParent;

		ATLASSERT(m_hWnd == NULL);

		// Allocate the thunk structure here, where we can fail gracefully.
		BOOL bRetTh = m_thunk.Init(NULL, NULL);
		if(bRetTh == FALSE)
		{
			::SetLastError(ERROR_OUTOFMEMORY);
			return -1;
		}

		ModuleHelper::AddCreateWndData(&m_thunk.cd, (CCommonDialogImplBase*)this);

		BOOL bRet = ::ChooseFont(&m_cf);

		m_hWnd = NULL;

		if(bRet)   // copy logical font from user's initialization buffer (if needed)
			ATL::Checked::memcpy_s(&m_lf, sizeof(m_lf), m_cf.lpLogFont, sizeof(m_lf));

		return bRet ? IDOK : IDCANCEL;
	}

	// works only when the dialog is dislayed or after
	void GetCurrentFont(LPLOGFONT lplf) const
	{
		ATLASSERT(lplf != NULL);

		if(m_hWnd != NULL)
			::SendMessage(m_hWnd, WM_CHOOSEFONT_GETLOGFONT, 0, (LPARAM)lplf);
		else
			*lplf = m_lf;
	}

	// works only when the dialog is dislayed or before
	void SetLogFont(LPLOGFONT lplf)
	{
		ATLASSERT(lplf != NULL);

		if(m_hWnd != NULL)
		{
			::SendMessage(m_hWnd, WM_CHOOSEFONT_SETLOGFONT, 0, (LPARAM)lplf);
		}
		else
		{
			m_lf = *lplf;
			m_cf.Flags |= CF_INITTOLOGFONTSTRUCT;
		}
	}

	void SetFlags(DWORD dwFlags)
	{
		if(m_hWnd != NULL)
		{
			CHOOSEFONT cf = { sizeof(CHOOSEFONT) };
			cf.Flags = dwFlags;
			::SendMessage(m_hWnd, WM_CHOOSEFONT_SETFLAGS, 0, (LPARAM)&cf);
		}
		else
		{
			m_cf.Flags = dwFlags;
		}
	}

	// Helpers for parsing information after successful return
	LPCTSTR GetFaceName() const   // return the face name of the font
	{
		return (LPCTSTR)m_cf.lpLogFont->lfFaceName;
	}

	LPCTSTR GetStyleName() const  // return the style name of the font
	{
		return m_cf.lpszStyle;
	}

	int GetSize() const           // return the pt size of the font
	{
		return m_cf.iPointSize;
	}

	COLORREF GetColor() const     // return the color of the font
	{
		return m_cf.rgbColors;
	}

	int GetWeight() const         // return the chosen font weight
	{
		return (int)m_cf.lpLogFont->lfWeight;
	}

	BOOL IsStrikeOut() const      // return TRUE if strikeout
	{
		return (m_cf.lpLogFont->lfStrikeOut) ? TRUE : FALSE;
	}

	BOOL IsUnderline() const      // return TRUE if underline
	{
		return (m_cf.lpLogFont->lfUnderline) ? TRUE : FALSE;
	}

	BOOL IsBold() const           // return TRUE if bold font
	{
		return (m_cf.lpLogFont->lfWeight == FW_BOLD) ? TRUE : FALSE;
	}

	BOOL IsItalic() const         // return TRUE if italic font
	{
		return m_cf.lpLogFont->lfItalic ? TRUE : FALSE;
	}
};

class CFontDialog : public CFontDialogImpl<CFontDialog>
{
public:
	CFontDialog(LPLOGFONT lplfInitial = NULL,
		DWORD dwFlags = CF_EFFECTS | CF_SCREENFONTS,
		HDC hDCPrinter = NULL,
		HWND hWndParent = NULL)
		: CFontDialogImpl<CFontDialog>(lplfInitial, dwFlags, hDCPrinter, hWndParent)
	{ }

	DECLARE_EMPTY_MSG_MAP()
};


///////////////////////////////////////////////////////////////////////////////
// CRichEditFontDialogImpl - font selection for the Rich Edit ctrl

#ifdef _RICHEDIT_

template <class T>
class ATL_NO_VTABLE CRichEditFontDialogImpl : public CFontDialogImpl< T >
{
public:
	CRichEditFontDialogImpl(const CHARFORMAT& charformat,
			DWORD dwFlags = CF_SCREENFONTS,
			HDC hDCPrinter = NULL,
			HWND hWndParent = NULL)
			: CFontDialogImpl< T >(NULL, dwFlags, hDCPrinter, hWndParent)
	{
		this->m_cf.Flags |= CF_INITTOLOGFONTSTRUCT;
		this->m_cf.Flags |= FillInLogFont(charformat);
		this->m_cf.lpLogFont = &this->m_lf;

		if((charformat.dwMask & CFM_COLOR) != 0)
			this->m_cf.rgbColors = charformat.crTextColor;
	}

	void GetCharFormat(CHARFORMAT& cf) const
	{
		USES_CONVERSION;
		cf.dwEffects = 0;
		cf.dwMask = 0;
		if((this->m_cf.Flags & CF_NOSTYLESEL) == 0)
		{
			cf.dwMask |= CFM_BOLD | CFM_ITALIC;
			cf.dwEffects |= this->IsBold() ? CFE_BOLD : 0;
			cf.dwEffects |= this->IsItalic() ? CFE_ITALIC : 0;
		}
		if((this->m_cf.Flags & CF_NOSIZESEL) == 0)
		{
			cf.dwMask |= CFM_SIZE;
			// GetSize() returns in tenths of points so mulitply by 2 to get twips
			cf.yHeight = this->GetSize() * 2;
		}

		if((this->m_cf.Flags & CF_NOFACESEL) == 0)
		{
			cf.dwMask |= CFM_FACE;
			cf.bPitchAndFamily = this->m_cf.lpLogFont->lfPitchAndFamily;
			ATL::Checked::tcscpy_s(cf.szFaceName, _countof(cf.szFaceName), this->GetFaceName());
		}

		if((this->m_cf.Flags & CF_EFFECTS) != 0)
		{
			cf.dwMask |= CFM_UNDERLINE | CFM_STRIKEOUT | CFM_COLOR;
			cf.dwEffects |= this->IsUnderline() ? CFE_UNDERLINE : 0;
			cf.dwEffects |= this->IsStrikeOut() ? CFE_STRIKEOUT : 0;
			cf.crTextColor = this->GetColor();
		}
		if((this->m_cf.Flags & CF_NOSCRIPTSEL) == 0)
		{
			cf.bCharSet = this->m_cf.lpLogFont->lfCharSet;
			cf.dwMask |= CFM_CHARSET;
		}
		cf.yOffset = 0;
	}

	DWORD FillInLogFont(const CHARFORMAT& cf)
	{
		USES_CONVERSION;
		DWORD dwFlags = 0;
		if((cf.dwMask & CFM_SIZE) != 0)
		{
			HDC hDC = ::CreateDC(_T("DISPLAY"), NULL, NULL, NULL);
			LONG yPerInch = ::GetDeviceCaps(hDC, LOGPIXELSY);
			this->m_lf.lfHeight = -(int)((cf.yHeight * yPerInch) / 1440);
		}
		else
			this->m_lf.lfHeight = 0;

		this->m_lf.lfWidth = 0;
		this->m_lf.lfEscapement = 0;
		this->m_lf.lfOrientation = 0;

		if((cf.dwMask & (CFM_ITALIC | CFM_BOLD)) == (CFM_ITALIC | CFM_BOLD))
		{
			this->m_lf.lfWeight = ((cf.dwEffects & CFE_BOLD) != 0) ? FW_BOLD : FW_NORMAL;
			this->m_lf.lfItalic = (BYTE)(((cf.dwEffects & CFE_ITALIC) != 0) ? TRUE : FALSE);
		}
		else
		{
			dwFlags |= CF_NOSTYLESEL;
			this->m_lf.lfWeight = FW_DONTCARE;
			this->m_lf.lfItalic = FALSE;
		}

		if((cf.dwMask & (CFM_UNDERLINE | CFM_STRIKEOUT | CFM_COLOR)) == (CFM_UNDERLINE|CFM_STRIKEOUT|CFM_COLOR))
		{
			dwFlags |= CF_EFFECTS;
			this->m_lf.lfUnderline = (BYTE)(((cf.dwEffects & CFE_UNDERLINE) != 0) ? TRUE : FALSE);
			this->m_lf.lfStrikeOut = (BYTE)(((cf.dwEffects & CFE_STRIKEOUT) != 0) ? TRUE : FALSE);
		}
		else
		{
			this->m_lf.lfUnderline = (BYTE)FALSE;
			this->m_lf.lfStrikeOut = (BYTE)FALSE;
		}

		if((cf.dwMask & CFM_CHARSET) != 0)
			this->m_lf.lfCharSet = cf.bCharSet;
		else
			dwFlags |= CF_NOSCRIPTSEL;
		this->m_lf.lfOutPrecision = OUT_DEFAULT_PRECIS;
		this->m_lf.lfClipPrecision = CLIP_DEFAULT_PRECIS;
		this->m_lf.lfQuality = DEFAULT_QUALITY;
		if((cf.dwMask & CFM_FACE) != 0)
		{
			this->m_lf.lfPitchAndFamily = cf.bPitchAndFamily;
			ATL::Checked::tcscpy_s(this->m_lf.lfFaceName, _countof(this->m_lf.lfFaceName), cf.szFaceName);
		}
		else
		{
			this->m_lf.lfPitchAndFamily = DEFAULT_PITCH|FF_DONTCARE;
			this->m_lf.lfFaceName[0] = (TCHAR)0;
		}
		return dwFlags;
	}
};

class CRichEditFontDialog : public CRichEditFontDialogImpl<CRichEditFontDialog>
{
public:
	CRichEditFontDialog(const CHARFORMAT& charformat,
		DWORD dwFlags = CF_SCREENFONTS,
		HDC hDCPrinter = NULL,
		HWND hWndParent = NULL)
		: CRichEditFontDialogImpl<CRichEditFontDialog>(charformat, dwFlags, hDCPrinter, hWndParent)
	{ }

	DECLARE_EMPTY_MSG_MAP()
};

#endif // _RICHEDIT_


///////////////////////////////////////////////////////////////////////////////
// CColorDialogImpl - color selection

template <class T>
class ATL_NO_VTABLE CColorDialogImpl : public CCommonDialogImplBase
{
public:
	CHOOSECOLOR m_cc;

// Constructor
	CColorDialogImpl(COLORREF clrInit = 0, DWORD dwFlags = 0, HWND hWndParent = NULL)
	{
		memset(&m_cc, 0, sizeof(m_cc));

		m_cc.lStructSize = sizeof(m_cc);
		m_cc.lpCustColors = GetCustomColors();
		m_cc.hwndOwner = hWndParent;
		m_cc.Flags = dwFlags | CC_ENABLEHOOK;
		m_cc.lpfnHook = (LPCCHOOKPROC)T::HookProc;

		if(clrInit != 0)
		{
			m_cc.rgbResult = clrInit;
			m_cc.Flags |= CC_RGBINIT;
		}
	}

// Operations
	INT_PTR DoModal(HWND hWndParent = ::GetActiveWindow())
	{
		ATLASSERT((m_cc.Flags & CC_ENABLEHOOK) != 0);
		ATLASSERT(m_cc.lpfnHook != NULL);   // can still be a user hook

		if(m_cc.hwndOwner == NULL)          // set only if not specified before
			m_cc.hwndOwner = hWndParent;

		ATLASSERT(m_hWnd == NULL);

		// Allocate the thunk structure here, where we can fail gracefully.
		BOOL bRetTh = m_thunk.Init(NULL, NULL);
		if(bRetTh == FALSE)
		{
			::SetLastError(ERROR_OUTOFMEMORY);
			return -1;
		}

		ModuleHelper::AddCreateWndData(&m_thunk.cd, (CCommonDialogImplBase*)this);

		BOOL bRet = ::ChooseColor(&m_cc);

		m_hWnd = NULL;

		return bRet ? IDOK : IDCANCEL;
	}

	// Set the current color while dialog is displayed
	void SetCurrentColor(COLORREF clr)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		SendMessage(_GetSetRGBMessage(), 0, (LPARAM)clr);
	}

	// Get the selected color after DoModal returns, or in OnColorOK
	COLORREF GetColor() const
	{
		return m_cc.rgbResult;
	}

// Special override for the color dialog
	static UINT_PTR APIENTRY HookProc(HWND hWnd, UINT uMsg, WPARAM wParam, LPARAM lParam)
	{
		if((uMsg != WM_INITDIALOG) && (uMsg != _GetColorOKMessage()))
			return 0;

		LPCHOOSECOLOR lpCC = (LPCHOOSECOLOR)lParam;
		CCommonDialogImplBase* pT = NULL;

		if(uMsg == WM_INITDIALOG)
		{
			pT = (CCommonDialogImplBase*)ModuleHelper::ExtractCreateWndData();
			lpCC->lCustData = (LPARAM)pT;
			ATLASSERT(pT != NULL);
			ATLASSERT(pT->m_hWnd == NULL);
			ATLASSERT(::IsWindow(hWnd));
			// subclass dialog's window
			if(!pT->SubclassWindow(hWnd))
			{
				ATLTRACE2(atlTraceUI, 0, _T("Subclassing a Color common dialog failed\n"));
				return 0;
			}
		}
		else if(uMsg == _GetColorOKMessage())
		{
			pT = (CCommonDialogImplBase*)lpCC->lCustData;
			ATLASSERT(pT != NULL);
			ATLASSERT(::IsWindow(pT->m_hWnd));
		}
		else
		{
			ATLASSERT(FALSE);
			return 0;
		}

		// pass to the message map
		LRESULT lRes = 0;
		if(pT->ProcessWindowMessage(pT->m_hWnd, uMsg, wParam, lParam, lRes, 0) == FALSE)
			return 0;

		return lRes;
	}

// Helpers
	static COLORREF* GetCustomColors()
	{
		static COLORREF rgbCustomColors[16] =
		{
			RGB(255, 255, 255), RGB(255, 255, 255), 
			RGB(255, 255, 255), RGB(255, 255, 255), 
			RGB(255, 255, 255), RGB(255, 255, 255), 
			RGB(255, 255, 255), RGB(255, 255, 255), 
			RGB(255, 255, 255), RGB(255, 255, 255), 
			RGB(255, 255, 255), RGB(255, 255, 255), 
			RGB(255, 255, 255), RGB(255, 255, 255), 
			RGB(255, 255, 255), RGB(255, 255, 255), 
		};

		return rgbCustomColors;
	}

	static UINT _GetSetRGBMessage()
	{
		static UINT uSetRGBMessage = 0;
		if(uSetRGBMessage == 0)
		{
			CStaticDataInitCriticalSectionLock lock;
			if(FAILED(lock.Lock()))
			{
				ATLTRACE2(atlTraceUI, 0, _T("ERROR : Unable to lock critical section in CColorDialogImpl::_GetSetRGBMessage.\n"));
				ATLASSERT(FALSE);
				return 0;
			}

			if(uSetRGBMessage == 0)
				uSetRGBMessage = ::RegisterWindowMessage(SETRGBSTRING);

			lock.Unlock();
		}
		ATLASSERT(uSetRGBMessage != 0);
		return uSetRGBMessage;
	}

	static UINT _GetColorOKMessage()
	{
		static UINT uColorOKMessage = 0;
		if(uColorOKMessage == 0)
		{
			CStaticDataInitCriticalSectionLock lock;
			if(FAILED(lock.Lock()))
			{
				ATLTRACE2(atlTraceUI, 0, _T("ERROR : Unable to lock critical section in CColorDialogImpl::_GetColorOKMessage.\n"));
				ATLASSERT(FALSE);
				return 0;
			}

			if(uColorOKMessage == 0)
				uColorOKMessage = ::RegisterWindowMessage(COLOROKSTRING);

			lock.Unlock();
		}
		ATLASSERT(uColorOKMessage != 0);
		return uColorOKMessage;
	}

// Message map and handlers
	BEGIN_MSG_MAP(CColorDialogImpl)
		MESSAGE_HANDLER(_GetColorOKMessage(), _OnColorOK)
	END_MSG_MAP()

	LRESULT _OnColorOK(UINT, WPARAM, LPARAM, BOOL&)
	{
		T* pT = static_cast<T*>(this);
		return pT->OnColorOK();
	}

// Overrideable
	BOOL OnColorOK()        // validate color
	{
		return FALSE;
	}
};

class CColorDialog : public CColorDialogImpl<CColorDialog>
{
public:
	CColorDialog(COLORREF clrInit = 0, DWORD dwFlags = 0, HWND hWndParent = NULL)
		: CColorDialogImpl<CColorDialog>(clrInit, dwFlags, hWndParent)
	{ }

	// override base class map and references to handlers
	DECLARE_EMPTY_MSG_MAP()
};


///////////////////////////////////////////////////////////////////////////////
// CPrintDialogImpl - used for Print... and PrintSetup...

// global helper
static inline HDC _AtlCreateDC(HGLOBAL hDevNames, HGLOBAL hDevMode)
{
	if(hDevNames == NULL)
		return NULL;

	LPDEVNAMES lpDevNames = (LPDEVNAMES)::GlobalLock(hDevNames);
	LPDEVMODE  lpDevMode = (hDevMode != NULL) ? (LPDEVMODE)::GlobalLock(hDevMode) : NULL;

	if(lpDevNames == NULL)
		return NULL;

	HDC hDC = ::CreateDC((LPCTSTR)lpDevNames + lpDevNames->wDriverOffset,
					  (LPCTSTR)lpDevNames + lpDevNames->wDeviceOffset,
					  (LPCTSTR)lpDevNames + lpDevNames->wOutputOffset,
					  lpDevMode);

	::GlobalUnlock(hDevNames);
	if(hDevMode != NULL)
		::GlobalUnlock(hDevMode);
	return hDC;
}

#pragma warning(push)
#pragma warning(disable: 4512)   // assignment operator could not be generated

template <class T>
class ATL_NO_VTABLE CPrintDialogImpl : public CCommonDialogImplBase
{
public:
	// print dialog parameter block (note this is a reference)
	PRINTDLG& m_pd;

// Constructors
	CPrintDialogImpl(BOOL bPrintSetupOnly = FALSE,	// TRUE for Print Setup, FALSE for Print Dialog
			DWORD dwFlags = PD_ALLPAGES | PD_USEDEVMODECOPIES | PD_NOPAGENUMS | PD_NOSELECTION,
			HWND hWndParent = NULL)
			: m_pd(m_pdActual)
	{
		memset(&m_pdActual, 0, sizeof(m_pdActual));

		m_pd.lStructSize = sizeof(m_pdActual);
		m_pd.hwndOwner = hWndParent;
		m_pd.Flags = (dwFlags | PD_ENABLEPRINTHOOK | PD_ENABLESETUPHOOK);
		m_pd.lpfnPrintHook = (LPPRINTHOOKPROC)T::HookProc;
		m_pd.lpfnSetupHook = (LPSETUPHOOKPROC)T::HookProc;

		if(bPrintSetupOnly)
			m_pd.Flags |= PD_PRINTSETUP;
		else
			m_pd.Flags |= PD_RETURNDC;

		m_pd.Flags &= ~PD_RETURNIC; // do not support information context
	}

// Operations
	INT_PTR DoModal(HWND hWndParent = ::GetActiveWindow())
	{
		ATLASSERT((m_pd.Flags & PD_ENABLEPRINTHOOK) != 0);
		ATLASSERT((m_pd.Flags & PD_ENABLESETUPHOOK) != 0);
		ATLASSERT(m_pd.lpfnPrintHook != NULL);   // can still be a user hook
		ATLASSERT(m_pd.lpfnSetupHook != NULL);   // can still be a user hook
		ATLASSERT((m_pd.Flags & PD_RETURNDEFAULT) == 0);   // use GetDefaults for this

		if(m_pd.hwndOwner == NULL)   // set only if not specified before
			m_pd.hwndOwner = hWndParent;

		ATLASSERT(m_hWnd == NULL);

		// Allocate the thunk structure here, where we can fail gracefully.
		BOOL bRetTh = m_thunk.Init(NULL, NULL);
		if(bRetTh == FALSE)
		{
			::SetLastError(ERROR_OUTOFMEMORY);
			return -1;
		}

		ModuleHelper::AddCreateWndData(&m_thunk.cd, (CCommonDialogImplBase*)this);

		BOOL bRet = ::PrintDlg(&m_pd);

		m_hWnd = NULL;

		return bRet ? IDOK : IDCANCEL;
	}

	// GetDefaults will not display a dialog but will get device defaults
	BOOL GetDefaults()
	{
		m_pd.Flags |= PD_RETURNDEFAULT;
		ATLASSERT(m_pd.hDevMode == NULL);    // must be NULL
		ATLASSERT(m_pd.hDevNames == NULL);   // must be NULL

		return ::PrintDlg(&m_pd);
	}

	// Helpers for parsing information after successful return num. copies requested
	int GetCopies() const
	{
		if((m_pd.Flags & PD_USEDEVMODECOPIES) != 0)
		{
			LPDEVMODE lpDevMode = GetDevMode();
			return (lpDevMode != NULL) ? lpDevMode->dmCopies : -1;
		}

		return m_pd.nCopies;
	}

	BOOL PrintCollate() const       // TRUE if collate checked
	{
		return ((m_pd.Flags & PD_COLLATE) != 0) ? TRUE : FALSE;
	}

	BOOL PrintSelection() const     // TRUE if printing selection
	{
		return ((m_pd.Flags & PD_SELECTION) != 0) ? TRUE : FALSE;
	}

	BOOL PrintAll() const           // TRUE if printing all pages
	{
		return (!PrintRange() && !PrintSelection()) ? TRUE : FALSE;
	}

	BOOL PrintRange() const         // TRUE if printing page range
	{
		return ((m_pd.Flags & PD_PAGENUMS) != 0) ? TRUE : FALSE;
	}

	BOOL PrintToFile() const        // TRUE if printing to a file
	{
		return ((m_pd.Flags & PD_PRINTTOFILE) != 0) ? TRUE : FALSE;
	}

	int GetFromPage() const         // starting page if valid
	{
		return PrintRange() ? m_pd.nFromPage : -1;
	}

	int GetToPage() const           // ending page if valid
	{
		return PrintRange() ? m_pd.nToPage : -1;
	}

	LPDEVMODE GetDevMode() const    // return DEVMODE
	{
		if(m_pd.hDevMode == NULL)
			return NULL;

		return (LPDEVMODE)::GlobalLock(m_pd.hDevMode);
	}

	LPCTSTR GetDriverName() const   // return driver name
	{
		if(m_pd.hDevNames == NULL)
			return NULL;

		LPDEVNAMES lpDev = (LPDEVNAMES)::GlobalLock(m_pd.hDevNames);
		if(lpDev == NULL)
			return NULL;

		return (LPCTSTR)lpDev + lpDev->wDriverOffset;
	}

	LPCTSTR GetDeviceName() const   // return device name
	{
		if(m_pd.hDevNames == NULL)
			return NULL;

		LPDEVNAMES lpDev = (LPDEVNAMES)::GlobalLock(m_pd.hDevNames);
		if(lpDev == NULL)
			return NULL;

		return (LPCTSTR)lpDev + lpDev->wDeviceOffset;
	}

	LPCTSTR GetPortName() const     // return output port name
	{
		if(m_pd.hDevNames == NULL)
			return NULL;

		LPDEVNAMES lpDev = (LPDEVNAMES)::GlobalLock(m_pd.hDevNames);
		if(lpDev == NULL)
			return NULL;

		return (LPCTSTR)lpDev + lpDev->wOutputOffset;
	}

	HDC GetPrinterDC() const        // return HDC (caller must delete)
	{
		ATLASSERT((m_pd.Flags & PD_RETURNDC) != 0);
		return m_pd.hDC;
	}

	// This helper creates a DC based on the DEVNAMES and DEVMODE structures.
	// This DC is returned, but also stored in m_pd.hDC as though it had been
	// returned by CommDlg.  It is assumed that any previously obtained DC
	// has been/will be deleted by the user.  This may be
	// used without ever invoking the print/print setup dialogs.
	HDC CreatePrinterDC()
	{
		m_pd.hDC = _AtlCreateDC(m_pd.hDevNames, m_pd.hDevMode);
		return m_pd.hDC;
	}

// Implementation
	PRINTDLG m_pdActual; // the Print/Print Setup need to share this

	// The following handle the case of print setup... from the print dialog
	CPrintDialogImpl(PRINTDLG& pdInit) : m_pd(pdInit)
	{ }

	BEGIN_MSG_MAP(CPrintDialogImpl)
#ifdef psh1
		COMMAND_ID_HANDLER(psh1, OnPrintSetup) // print setup button when print is displayed
#else // !psh1
		COMMAND_ID_HANDLER(0x0400, OnPrintSetup) // value from dlgs.h
#endif // !psh1
	END_MSG_MAP()

	LRESULT OnPrintSetup(WORD wNotifyCode, WORD wID, HWND hWndCtl, BOOL& /*bHandled*/)
	{
		T dlgSetup(m_pd);
		ModuleHelper::AddCreateWndData(&dlgSetup.m_thunk.cd, (CCommonDialogImplBase*)&dlgSetup);
		return DefWindowProc(WM_COMMAND, MAKEWPARAM(wID, wNotifyCode), (LPARAM)hWndCtl);
	}
};

class CPrintDialog : public CPrintDialogImpl<CPrintDialog>
{
public:
	CPrintDialog(BOOL bPrintSetupOnly = FALSE,
		DWORD dwFlags = PD_ALLPAGES | PD_USEDEVMODECOPIES | PD_NOPAGENUMS | PD_NOSELECTION,
		HWND hWndParent = NULL)
		: CPrintDialogImpl<CPrintDialog>(bPrintSetupOnly, dwFlags, hWndParent)
	{ }

	CPrintDialog(PRINTDLG& pdInit) : CPrintDialogImpl<CPrintDialog>(pdInit)
	{ }
};

#pragma warning(pop)


///////////////////////////////////////////////////////////////////////////////
// CPrintDialogExImpl - new print dialog for Windows 2000

} // namespace WTL

#include <atlcom.h>

extern "C" const __declspec(selectany) IID IID_IPrintDialogCallback = {0x5852a2c3, 0x6530, 0x11d1, {0xb6, 0xa3, 0x0, 0x0, 0xf8, 0x75, 0x7b, 0xf9}};
extern "C" const __declspec(selectany) IID IID_IPrintDialogServices = {0x509aaeda, 0x5639, 0x11d1, {0xb6, 0xa1, 0x0, 0x0, 0xf8, 0x75, 0x7b, 0xf9}};

namespace WTL
{

template <class T>
class ATL_NO_VTABLE CPrintDialogExImpl : 
				public ATL::CWindow,
				public ATL::CMessageMap,
				public IPrintDialogCallback,
				public ATL::IObjectWithSiteImpl< T >
{
public:
	PRINTDLGEX m_pdex;

// Constructor
	CPrintDialogExImpl(DWORD dwFlags = PD_ALLPAGES | PD_USEDEVMODECOPIES | PD_NOPAGENUMS | PD_NOSELECTION | PD_NOCURRENTPAGE,
				HWND hWndParent = NULL)
	{
		memset(&m_pdex, 0, sizeof(m_pdex));

		m_pdex.lStructSize = sizeof(PRINTDLGEX);
		m_pdex.hwndOwner = hWndParent;
		m_pdex.Flags = dwFlags;
		m_pdex.nStartPage = START_PAGE_GENERAL;
		// callback object will be set in DoModal

		m_pdex.Flags &= ~PD_RETURNIC; // do not support information context
	}

// Operations
	HRESULT DoModal(HWND hWndParent = ::GetActiveWindow())
	{
		ATLASSERT(m_hWnd == NULL);
		ATLASSERT((m_pdex.Flags & PD_RETURNDEFAULT) == 0);   // use GetDefaults for this

		if(m_pdex.hwndOwner == NULL)   // set only if not specified before
			m_pdex.hwndOwner = hWndParent;

		T* pT = static_cast<T*>(this);
		m_pdex.lpCallback = (IUnknown*)(IPrintDialogCallback*)pT;

		HRESULT hResult = ::PrintDlgEx(&m_pdex);

		m_hWnd = NULL;

		return hResult;
	}

	BOOL EndDialog(INT_PTR /*nRetCode*/ = 0)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		SendMessage(WM_COMMAND, MAKEWPARAM(IDABORT, 0));
		return TRUE;
	}

	// GetDefaults will not display a dialog but will get device defaults
	HRESULT GetDefaults()
	{
		ATLASSERT(m_pdex.hDevMode == NULL);    // must be NULL
		ATLASSERT(m_pdex.hDevNames == NULL);   // must be NULL

		if(m_pdex.hwndOwner == NULL)   // set only if not specified before
			m_pdex.hwndOwner = ::GetActiveWindow();

		m_pdex.Flags |= PD_RETURNDEFAULT;
		HRESULT hRet = ::PrintDlgEx(&m_pdex);
		m_pdex.Flags &= ~PD_RETURNDEFAULT;

		return hRet;
	}

	// Helpers for parsing information after successful return num. copies requested
	int GetCopies() const
	{
		if((m_pdex.Flags & PD_USEDEVMODECOPIES) != 0)
		{
			LPDEVMODE lpDevMode = GetDevMode();
			return (lpDevMode != NULL) ? lpDevMode->dmCopies : -1;
		}

		return m_pdex.nCopies;
	}

	BOOL PrintCollate() const       // TRUE if collate checked
	{
		return ((m_pdex.Flags & PD_COLLATE) != 0) ? TRUE : FALSE;
	}

	BOOL PrintSelection() const     // TRUE if printing selection
	{
		return ((m_pdex.Flags & PD_SELECTION) != 0) ? TRUE : FALSE;
	}

	BOOL PrintAll() const           // TRUE if printing all pages
	{
		return (!PrintRange() && !PrintSelection()) ? TRUE : FALSE;
	}

	BOOL PrintRange() const         // TRUE if printing page range
	{
		return ((m_pdex.Flags & PD_PAGENUMS) != 0) ? TRUE : FALSE;
	}

	BOOL PrintToFile() const        // TRUE if printing to a file
	{
		return ((m_pdex.Flags & PD_PRINTTOFILE) != 0) ? TRUE : FALSE;
	}

	LPDEVMODE GetDevMode() const    // return DEVMODE
	{
		if(m_pdex.hDevMode == NULL)
			return NULL;

		return (LPDEVMODE)::GlobalLock(m_pdex.hDevMode);
	}

	LPCTSTR GetDriverName() const   // return driver name
	{
		if(m_pdex.hDevNames == NULL)
			return NULL;

		LPDEVNAMES lpDev = (LPDEVNAMES)::GlobalLock(m_pdex.hDevNames);
		if(lpDev == NULL)
			return NULL;

		return (LPCTSTR)lpDev + lpDev->wDriverOffset;
	}

	LPCTSTR GetDeviceName() const   // return device name
	{
		if(m_pdex.hDevNames == NULL)
			return NULL;

		LPDEVNAMES lpDev = (LPDEVNAMES)::GlobalLock(m_pdex.hDevNames);
		if(lpDev == NULL)
			return NULL;

		return (LPCTSTR)lpDev + lpDev->wDeviceOffset;
	}

	LPCTSTR GetPortName() const     // return output port name
	{
		if(m_pdex.hDevNames == NULL)
			return NULL;

		LPDEVNAMES lpDev = (LPDEVNAMES)::GlobalLock(m_pdex.hDevNames);
		if(lpDev == NULL)
			return NULL;

		return (LPCTSTR)lpDev + lpDev->wOutputOffset;
	}

	HDC GetPrinterDC() const        // return HDC (caller must delete)
	{
		ATLASSERT((m_pdex.Flags & PD_RETURNDC) != 0);
		return m_pdex.hDC;
	}

	// This helper creates a DC based on the DEVNAMES and DEVMODE structures.
	// This DC is returned, but also stored in m_pdex.hDC as though it had been
	// returned by CommDlg.  It is assumed that any previously obtained DC
	// has been/will be deleted by the user.  This may be
	// used without ever invoking the print/print setup dialogs.
	HDC CreatePrinterDC()
	{
		m_pdex.hDC = _AtlCreateDC(m_pdex.hDevNames, m_pdex.hDevMode);
		return m_pdex.hDC;
	}

// Implementation - interfaces

// IUnknown
	STDMETHOD(QueryInterface)(REFIID riid, void** ppvObject)
	{
		if(ppvObject == NULL)
			return E_POINTER;

		T* pT = static_cast<T*>(this);
		if(IsEqualGUID(riid, IID_IUnknown) || IsEqualGUID(riid, IID_IPrintDialogCallback))
		{
			*ppvObject = (IPrintDialogCallback*)pT;
			// AddRef() not needed
			return S_OK;
		}
		else if(IsEqualGUID(riid, IID_IObjectWithSite))
		{
			*ppvObject = (IObjectWithSite*)pT;
			// AddRef() not needed
			return S_OK;
		}

		return E_NOINTERFACE;
	}

	virtual ULONG STDMETHODCALLTYPE AddRef()
	{
		return 1;
	}

	virtual ULONG STDMETHODCALLTYPE Release()
	{
		return 1;
	}

// IPrintDialogCallback
	STDMETHOD(InitDone)()
	{
		return S_FALSE;
	}

	STDMETHOD(SelectionChange)()
	{
		return S_FALSE;
	}

	STDMETHOD(HandleMessage)(HWND hWnd, UINT uMsg, WPARAM wParam, LPARAM lParam, LRESULT* plResult)
	{
		// set up m_hWnd the first time
		if(m_hWnd == NULL)
			Attach(hWnd);

		// call message map
		HRESULT hRet = ProcessWindowMessage(hWnd, uMsg, wParam, lParam, *plResult, 0) ? S_OK : S_FALSE;
		if((hRet == S_OK) && (uMsg == WM_NOTIFY))   // return in DWLP_MSGRESULT
			::SetWindowLongPtr(GetParent(), DWLP_MSGRESULT, (LONG_PTR)*plResult);

		if((uMsg == WM_INITDIALOG) && (hRet == S_OK) && ((BOOL)*plResult != FALSE))
			hRet = S_FALSE;

		return hRet;
	}
};

class CPrintDialogEx : public CPrintDialogExImpl<CPrintDialogEx>
{
public:
	CPrintDialogEx(
		DWORD dwFlags = PD_ALLPAGES | PD_USEDEVMODECOPIES | PD_NOPAGENUMS | PD_NOSELECTION | PD_NOCURRENTPAGE,
		HWND hWndParent = NULL)
		: CPrintDialogExImpl<CPrintDialogEx>(dwFlags, hWndParent)
	{ }

	DECLARE_EMPTY_MSG_MAP()
};


///////////////////////////////////////////////////////////////////////////////
// CPageSetupDialogImpl - Page Setup dialog

template <class T>
class ATL_NO_VTABLE CPageSetupDialogImpl : public CCommonDialogImplBase
{
public:
	PAGESETUPDLG m_psd;
	ATL::CWndProcThunk m_thunkPaint;

// Constructors
	CPageSetupDialogImpl(DWORD dwFlags = PSD_MARGINS | PSD_INWININIINTLMEASURE, HWND hWndParent = NULL)
	{
		memset(&m_psd, 0, sizeof(m_psd));

		m_psd.lStructSize = sizeof(m_psd);
		m_psd.hwndOwner = hWndParent;
		m_psd.Flags = (dwFlags | PSD_ENABLEPAGESETUPHOOK | PSD_ENABLEPAGEPAINTHOOK);
		m_psd.lpfnPageSetupHook = (LPPAGESETUPHOOK)T::HookProc;
		m_thunkPaint.Init((WNDPROC)T::PaintHookProc, this);
		m_psd.lpfnPagePaintHook = (LPPAGEPAINTHOOK)m_thunkPaint.GetWNDPROC();
	}

	DECLARE_EMPTY_MSG_MAP()

// Attributes
	LPDEVMODE GetDevMode() const    // return DEVMODE
	{
		if(m_psd.hDevMode == NULL)
			return NULL;

		return (LPDEVMODE)::GlobalLock(m_psd.hDevMode);
	}

	LPCTSTR GetDriverName() const   // return driver name
	{
		if(m_psd.hDevNames == NULL)
			return NULL;

		LPDEVNAMES lpDev = (LPDEVNAMES)::GlobalLock(m_psd.hDevNames);
		return (LPCTSTR)lpDev + lpDev->wDriverOffset;
	}

	LPCTSTR GetDeviceName() const   // return device name
	{
		if(m_psd.hDevNames == NULL)
			return NULL;

		LPDEVNAMES lpDev = (LPDEVNAMES)::GlobalLock(m_psd.hDevNames);
		return (LPCTSTR)lpDev + lpDev->wDeviceOffset;
	}

	LPCTSTR GetPortName() const     // return output port name
	{
		if(m_psd.hDevNames == NULL)
			return NULL;

		LPDEVNAMES lpDev = (LPDEVNAMES)::GlobalLock(m_psd.hDevNames);
		return (LPCTSTR)lpDev + lpDev->wOutputOffset;
	}

	HDC CreatePrinterDC()
	{
		return _AtlCreateDC(m_psd.hDevNames, m_psd.hDevMode);
	}

	SIZE GetPaperSize() const
	{
		SIZE size = { m_psd.ptPaperSize.x, m_psd.ptPaperSize.y };
		return size;
	}

	void GetMargins(LPRECT lpRectMargins, LPRECT lpRectMinMargins) const
	{
		if(lpRectMargins != NULL)
			*lpRectMargins = m_psd.rtMargin;
		if(lpRectMinMargins != NULL)
			*lpRectMinMargins = m_psd.rtMinMargin;
	}

// Operations
	INT_PTR DoModal(HWND hWndParent = ::GetActiveWindow())
	{
		ATLASSERT((m_psd.Flags & PSD_ENABLEPAGESETUPHOOK) != 0);
		ATLASSERT((m_psd.Flags & PSD_ENABLEPAGEPAINTHOOK) != 0);
		ATLASSERT(m_psd.lpfnPageSetupHook != NULL);   // can still be a user hook
		ATLASSERT(m_psd.lpfnPagePaintHook != NULL);   // can still be a user hook

		if(m_psd.hwndOwner == NULL)   // set only if not specified before
			m_psd.hwndOwner = hWndParent;

		ATLASSERT(m_hWnd == NULL);

		// Allocate the thunk structure here, where we can fail gracefully.
		BOOL bRetTh = m_thunk.Init(NULL, NULL);
		if(bRetTh == FALSE)
		{
			::SetLastError(ERROR_OUTOFMEMORY);
			return -1;
		}

		ModuleHelper::AddCreateWndData(&m_thunk.cd, (CCommonDialogImplBase*)this);

		BOOL bRet = ::PageSetupDlg(&m_psd);

		m_hWnd = NULL;

		return bRet ? IDOK : IDCANCEL;
	}

// Implementation
	static UINT_PTR CALLBACK PaintHookProc(HWND hWnd, UINT uMsg, WPARAM wParam, LPARAM lParam)
	{
		T* pT = (T*)hWnd;
		UINT_PTR uRet = 0;
		switch(uMsg)
		{
		case WM_PSD_PAGESETUPDLG:
			uRet = pT->PreDrawPage(LOWORD(wParam), HIWORD(wParam), (LPPAGESETUPDLG)lParam);
			break;
		case WM_PSD_FULLPAGERECT:
		case WM_PSD_MINMARGINRECT:
		case WM_PSD_MARGINRECT:
		case WM_PSD_GREEKTEXTRECT:
		case WM_PSD_ENVSTAMPRECT:
		case WM_PSD_YAFULLPAGERECT:
			uRet = pT->OnDrawPage(uMsg, (HDC)wParam, (LPRECT)lParam);
			break;
		default:
			ATLTRACE2(atlTraceUI, 0, _T("CPageSetupDialogImpl::PaintHookProc - unknown message received\n"));
			break;
		}
		return uRet;
	}

// Overridables
	UINT_PTR PreDrawPage(WORD /*wPaper*/, WORD /*wFlags*/, LPPAGESETUPDLG /*pPSD*/)
	{
		// return 1 to prevent any more drawing
		return 0;
	}

	UINT_PTR OnDrawPage(UINT /*uMsg*/, HDC /*hDC*/, LPRECT /*lpRect*/)
	{
		return 0; // do the default
	}
};

class CPageSetupDialog : public CPageSetupDialogImpl<CPageSetupDialog>
{
public:
	CPageSetupDialog(DWORD dwFlags = PSD_MARGINS | PSD_INWININIINTLMEASURE, HWND hWndParent = NULL)
		: CPageSetupDialogImpl<CPageSetupDialog>(dwFlags, hWndParent)
	{ }

	// override PaintHookProc and references to handlers
	static UINT_PTR CALLBACK PaintHookProc(HWND, UINT, WPARAM, LPARAM)
	{
		return 0;
	}
};


///////////////////////////////////////////////////////////////////////////////
// CFindReplaceDialogImpl - Find/FindReplace modeless dialogs

template <class T>
class ATL_NO_VTABLE CFindReplaceDialogImpl : public CCommonDialogImplBase
{
public:
	enum { _cchFindReplaceBuffer = 128 };

	FINDREPLACE m_fr;
	TCHAR m_szFindWhat[_cchFindReplaceBuffer];
	TCHAR m_szReplaceWith[_cchFindReplaceBuffer];

// Constructors
	CFindReplaceDialogImpl()
	{
		memset(&m_fr, 0, sizeof(m_fr));
		m_szFindWhat[0] = _T('\0');
		m_szReplaceWith[0] = _T('\0');

		m_fr.lStructSize = sizeof(m_fr);
		m_fr.Flags = FR_ENABLEHOOK;
		m_fr.lpfnHook = (LPFRHOOKPROC)T::HookProc;
		m_fr.lpstrFindWhat = (LPTSTR)m_szFindWhat;
		m_fr.wFindWhatLen = _cchFindReplaceBuffer;
		m_fr.lpstrReplaceWith = (LPTSTR)m_szReplaceWith;
		m_fr.wReplaceWithLen = _cchFindReplaceBuffer;
	}

	// Note: You must allocate the object on the heap.
	//       If you do not, you must override OnFinalMessage()
	virtual void OnFinalMessage(HWND /*hWnd*/)
	{
		delete this;
	}

	HWND Create(BOOL bFindDialogOnly, // TRUE for Find, FALSE for FindReplace
			LPCTSTR lpszFindWhat,
			LPCTSTR lpszReplaceWith = NULL,
			DWORD dwFlags = FR_DOWN,
			HWND hWndParent = NULL)
	{
		ATLASSERT((m_fr.Flags & FR_ENABLEHOOK) != 0);
		ATLASSERT(m_fr.lpfnHook != NULL);

		m_fr.Flags |= dwFlags;

		if(hWndParent == NULL)
			m_fr.hwndOwner = ::GetActiveWindow();
		else
			m_fr.hwndOwner = hWndParent;
		ATLASSERT(m_fr.hwndOwner != NULL); // must have an owner for modeless dialog

		if(lpszFindWhat != NULL)
			ATL::Checked::tcsncpy_s(m_szFindWhat, _countof(m_szFindWhat), lpszFindWhat, _TRUNCATE);

		if(lpszReplaceWith != NULL)
			ATL::Checked::tcsncpy_s(m_szReplaceWith, _countof(m_szReplaceWith), lpszReplaceWith, _TRUNCATE);

		ATLASSERT(m_hWnd == NULL);

		// Allocate the thunk structure here, where we can fail gracefully.
		BOOL bRet = m_thunk.Init(NULL, NULL);
		if(bRet == FALSE)
		{
			::SetLastError(ERROR_OUTOFMEMORY);
			return NULL;
		}

		ModuleHelper::AddCreateWndData(&m_thunk.cd, (CCommonDialogImplBase*)this);

		HWND hWnd = NULL;
		if(bFindDialogOnly)
			hWnd = ::FindText(&m_fr);
		else
			hWnd = ::ReplaceText(&m_fr);

		ATLASSERT(m_hWnd == hWnd);
		return hWnd;
	}

	static UINT GetFindReplaceMsg()
	{
		static const UINT nMsgFindReplace = ::RegisterWindowMessage(FINDMSGSTRING);
		return nMsgFindReplace;
	}
	// call while handling FINDMSGSTRING registered message
	// to retreive the object
	static T* PASCAL GetNotifier(LPARAM lParam)
	{
		ATLASSERT(lParam != NULL);
		T* pDlg = (T*)(lParam - offsetof(T, m_fr));
		return pDlg;
	}

// Operations
	// Helpers for parsing information after successful return
	LPCTSTR GetFindString() const    // get find string
	{
		return (LPCTSTR)m_fr.lpstrFindWhat;
	}

	LPCTSTR GetReplaceString() const // get replacement string
	{
		return (LPCTSTR)m_fr.lpstrReplaceWith;
	}

	BOOL SearchDown() const          // TRUE if search down, FALSE is up
	{
		return ((m_fr.Flags & FR_DOWN) != 0) ? TRUE : FALSE;
	}

	BOOL FindNext() const            // TRUE if command is find next
	{
		return ((m_fr.Flags & FR_FINDNEXT) != 0) ? TRUE : FALSE;
	}

	BOOL MatchCase() const           // TRUE if matching case
	{
		return ((m_fr.Flags & FR_MATCHCASE) != 0) ? TRUE : FALSE;
	}

	BOOL MatchWholeWord() const      // TRUE if matching whole words only
	{
		return ((m_fr.Flags & FR_WHOLEWORD) != 0) ? TRUE : FALSE;
	}

	BOOL ReplaceCurrent() const      // TRUE if replacing current string
	{
		return ((m_fr. Flags & FR_REPLACE) != 0) ? TRUE : FALSE;
	}

	BOOL ReplaceAll() const          // TRUE if replacing all occurrences
	{
		return ((m_fr.Flags & FR_REPLACEALL) != 0) ? TRUE : FALSE;
	}

	BOOL IsTerminating() const       // TRUE if terminating dialog
	{
		return ((m_fr.Flags & FR_DIALOGTERM) != 0) ? TRUE : FALSE ;
	}
};

class CFindReplaceDialog : public CFindReplaceDialogImpl<CFindReplaceDialog>
{
public:
	DECLARE_EMPTY_MSG_MAP()
};


/////////////////////////////////////////////////////////////////////////
// CDialogBaseUnits - Dialog Units helper
//

class CDialogBaseUnits
{
public:
	SIZE m_sizeUnits;

// Constructors
	CDialogBaseUnits()
	{
		// The base units of the out-dated System Font
		LONG nDlgBaseUnits = ::GetDialogBaseUnits();
		m_sizeUnits.cx = LOWORD(nDlgBaseUnits);
		m_sizeUnits.cy = HIWORD(nDlgBaseUnits);
	}

	CDialogBaseUnits(HWND hWnd)
	{
		if(!InitDialogBaseUnits(hWnd)) {
			LONG nDlgBaseUnits = ::GetDialogBaseUnits();
			m_sizeUnits.cx = LOWORD(nDlgBaseUnits);
			m_sizeUnits.cy = HIWORD(nDlgBaseUnits);
		}
	}

	CDialogBaseUnits(HFONT hFont, HWND hWnd = NULL)
	{
		if(!InitDialogBaseUnits(hFont, hWnd)) {
			LONG nDlgBaseUnits = ::GetDialogBaseUnits();
			m_sizeUnits.cx = LOWORD(nDlgBaseUnits);
			m_sizeUnits.cy = HIWORD(nDlgBaseUnits);
		}
	}

	CDialogBaseUnits(const LOGFONT& lf, HWND hWnd = NULL)
	{
		if(!InitDialogBaseUnits(lf, hWnd)) {
			LONG nDlgBaseUnits = ::GetDialogBaseUnits();
			m_sizeUnits.cx = LOWORD(nDlgBaseUnits);
			m_sizeUnits.cy = HIWORD(nDlgBaseUnits);
		}
	}

// Operations
	BOOL InitDialogBaseUnits(HWND hWnd)
	{
		ATLASSERT(::IsWindow(hWnd));
		RECT rc = { 0, 0, 4, 8 };
		if(!::MapDialogRect(hWnd, &rc)) return FALSE;
		m_sizeUnits.cx = rc.right;
		m_sizeUnits.cy = rc.bottom;
		return TRUE;
	}

	BOOL InitDialogBaseUnits(const LOGFONT& lf, HWND hWnd = NULL)
	{
		CFont font;
		font.CreateFontIndirect(&lf);
		if(font.IsNull()) return FALSE;
		return InitDialogBaseUnits(font, hWnd);
	}

	BOOL InitDialogBaseUnits(HFONT hFont, HWND hWnd = NULL)
	{
		ATLASSERT(hFont != NULL);
		CWindowDC dc = hWnd;
		TEXTMETRIC tmText = {};
		SIZE sizeText = {};
		HFONT hFontOld = dc.SelectFont(hFont);
		dc.GetTextMetrics(&tmText);
		m_sizeUnits.cy = tmText.tmHeight + tmText.tmExternalLeading;
		dc.GetTextExtent(_T("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"), 52, &sizeText);
		m_sizeUnits.cx = (sizeText.cx + 26) / 52;
		dc.SelectFont(hFontOld);
		return TRUE;
	}

	SIZE GetDialogBaseUnits() const
	{
		return m_sizeUnits;
	}

	INT MapDialogPixelsX(INT x) const
	{
		return ::MulDiv(x, 4, m_sizeUnits.cx);  // Pixels X to DLU
	}

	INT MapDialogPixelsY(INT y) const
	{
		return ::MulDiv(y, 8, m_sizeUnits.cy);  // Pixels Y to DLU
	}

	POINT MapDialogPixels(POINT pt) const
	{
		POINT out = { MapDialogPixelsX(pt.x), MapDialogPixelsY(pt.y) };
		return out;
	}

	SIZE MapDialogPixels(SIZE input) const
	{
		SIZE out = { MapDialogPixelsX(input.cx), MapDialogPixelsY(input.cy) };
		return out;
	}

	RECT MapDialogPixels(const RECT& input) const
	{
		RECT out = { MapDialogPixelsX(input.left), MapDialogPixelsY(input.top), MapDialogPixelsX(input.right), MapDialogPixelsY(input.bottom) };
		return out;
	}

	INT MapDialogUnitsX(INT x) const
	{
		return ::MulDiv(x, m_sizeUnits.cx, 4);  // DLU to Pixels X
	}

	INT MapDialogUnitsY(INT y) const
	{
		return ::MulDiv(y, m_sizeUnits.cy, 8);  // DLU to Pixels Y
	}

	POINT MapDialogUnits(POINT pt) const
	{
		POINT out = { MapDialogUnitsX(pt.x), MapDialogUnitsY(pt.y) };
		return out;
	}

	SIZE MapDialogUnits(SIZE input) const
	{
		SIZE out = { MapDialogUnitsX(input.cx), MapDialogUnitsY(input.cy) };
		return out;
	}

	RECT MapDialogUnits(const RECT& input) const
	{
		RECT out = { MapDialogUnitsX(input.left), MapDialogUnitsY(input.top), MapDialogUnitsX(input.right), MapDialogUnitsY(input.bottom) };
		return out;
	}
};


///////////////////////////////////////////////////////////////////////////////
// CMemDlgTemplate - in-memory dialog template - DLGTEMPLATE or DLGTEMPLATEEX

// traits suitable for dialog controls
typedef ATL::CWinTraits<WS_CHILD | WS_VISIBLE, 0>	CDlgControlWinTraits;

template <class TWinTraits>
class CMemDlgTemplateT
{
public:
	typedef ATL::_DialogSplitHelper::DLGTEMPLATEEX DLGTEMPLATEEX;
	typedef ATL::_DialogSplitHelper::DLGITEMTEMPLATEEX DLGITEMTEMPLATEEX;

	enum StdCtrlType
	{
		CTRL_BUTTON    = 0x0080,
		CTRL_EDIT      = 0x0081,
		CTRL_STATIC    = 0x0082,
		CTRL_LISTBOX   = 0x0083,
		CTRL_SCROLLBAR = 0x0084,
		CTRL_COMBOBOX  = 0x0085
	};

	HANDLE m_hData;
	LPBYTE m_pData;
	LPBYTE m_pPtr;
	SIZE_T m_cAllocated;

	CMemDlgTemplateT() : m_hData(NULL), m_pData(NULL), m_pPtr(NULL), m_cAllocated(0)
	{ }

	~CMemDlgTemplateT()
	{
		Reset();
	}

	bool IsValid() const
	{
		return (m_pData != NULL);
	}

	bool IsTemplateEx() const
	{
		return (IsValid() && ((DLGTEMPLATEEX*)m_pData)->signature == 0xFFFF);
	}

	LPDLGTEMPLATE GetTemplatePtr()
	{
		return reinterpret_cast<LPDLGTEMPLATE>(m_pData);
	}

	DLGTEMPLATEEX* GetTemplateExPtr()
	{
		return reinterpret_cast<DLGTEMPLATEEX*>(m_pData);
	}

	void Reset()
	{
		if (IsValid())
		{
			::GlobalUnlock(m_pData);
			ATLVERIFY(::GlobalFree(m_hData) == NULL);
		}

		m_hData = NULL;
		m_pData = NULL;
		m_pPtr = NULL;
		m_cAllocated = 0;
	}

	void Create(bool bDlgEx, LPCTSTR lpszCaption, const RECT& rc, DWORD dwStyle = 0, DWORD dwExStyle = 0,
	            LPCTSTR lpstrFontName = NULL, WORD wFontSize = 0, WORD wWeight = 0, BYTE bItalic = 0, BYTE bCharset = 0, DWORD dwHelpID = 0,
	            ATL::_U_STRINGorID ClassName = 0U, ATL::_U_STRINGorID Menu = 0U)
	{
		Create(bDlgEx, lpszCaption, (short) rc.left, (short) rc.top, (short) (rc.right - rc.left), (short) (rc.bottom - rc.top), dwStyle, dwExStyle,
			lpstrFontName, wFontSize, wWeight, bItalic, bCharset, dwHelpID, ClassName.m_lpstr, Menu.m_lpstr);
	}

	void Create(bool bDlgEx, LPCTSTR lpszCaption, short nX, short nY, short nWidth, short nHeight, DWORD dwStyle = 0, DWORD dwExStyle = 0,
	            LPCTSTR lpstrFontName = NULL, WORD wFontSize = 0, WORD wWeight = 0, BYTE bItalic = 0, BYTE bCharset = 0, DWORD dwHelpID = 0,
	            ATL::_U_STRINGorID ClassName = 0U, ATL::_U_STRINGorID Menu = 0U)
	{
		// Should have DS_SETFONT style to set the dialog font name and size
		if (lpstrFontName != NULL)
		{
			dwStyle |= DS_SETFONT;
		}
		else
		{
			dwStyle &= ~DS_SETFONT;
		}

		if (bDlgEx)
		{
			DLGTEMPLATEEX dlg = {1, 0xFFFF, dwHelpID, dwExStyle, dwStyle, 0, nX, nY, nWidth, nHeight};
			AddData(&dlg, sizeof(dlg));
		}
		else
		{
			DLGTEMPLATE dlg = {dwStyle, dwExStyle, 0, nX, nY, nWidth, nHeight};
			AddData(&dlg, sizeof(dlg));
		}

		if (Menu.m_lpstr == NULL)
		{
			WORD menuData = 0;
			AddData(&menuData, sizeof(WORD));
		}
		else if (IS_INTRESOURCE(Menu.m_lpstr))
		{
			WORD menuData[] = { 0xFFFF, LOWORD(Menu.m_lpstr) };
			AddData(menuData, sizeof(menuData));
		}
		else
		{
			AddString(Menu.m_lpstr);
		}

		if (ClassName.m_lpstr == NULL)
		{
			WORD classData = 0;
			AddData(&classData, sizeof(WORD));
		}
		else if (IS_INTRESOURCE(ClassName.m_lpstr))
		{
			WORD classData[] = { 0xFFFF, LOWORD(ClassName.m_lpstr) };
			AddData(classData, sizeof(classData));
		}
		else
		{
			AddString(ClassName.m_lpstr);
		}

		// Set dialog caption
		AddString(lpszCaption);

		if (lpstrFontName != NULL)
		{
			AddData(&wFontSize, sizeof(wFontSize));

			if (bDlgEx)
			{
				AddData(&wWeight, sizeof(wWeight));
				AddData(&bItalic, sizeof(bItalic));
				AddData(&bCharset, sizeof(bCharset));
			}

			AddString(lpstrFontName);
		}
	}

	void AddControl(ATL::_U_STRINGorID ClassName, WORD wId, const RECT& rc, DWORD dwStyle, DWORD dwExStyle,
	                ATL::_U_STRINGorID Text, const WORD* pCreationData = NULL, WORD nCreationData = 0, DWORD dwHelpID = 0)
	{
		AddControl(ClassName.m_lpstr, wId, (short) rc.left, (short) rc.top, (short) (rc.right - rc.left), (short) (rc.bottom - rc.top), dwStyle, dwExStyle,
			Text.m_lpstr, pCreationData, nCreationData, dwHelpID);
	}

	void AddControl(ATL::_U_STRINGorID ClassName, WORD wId, short nX, short nY, short nWidth, short nHeight, DWORD dwStyle, DWORD dwExStyle,
	                ATL::_U_STRINGorID Text, const WORD* pCreationData = NULL, WORD nCreationData = 0, DWORD dwHelpID = 0)
	{
		ATLASSERT(IsValid());

		// DWORD align data
		const DWORD_PTR dwDwordAlignBits = sizeof(DWORD) - 1;
		m_pPtr = (LPBYTE)(((DWORD_PTR)m_pPtr + dwDwordAlignBits) & (~dwDwordAlignBits));

		if (IsTemplateEx())
		{
			DLGTEMPLATEEX* dlg = (DLGTEMPLATEEX*)m_pData;
			dlg->cDlgItems++;

			DLGITEMTEMPLATEEX item = {dwHelpID, TWinTraits::GetWndExStyle(0) | dwExStyle, TWinTraits::GetWndStyle(0) | dwStyle, nX, nY, nWidth, nHeight, wId};
			AddData(&item, sizeof(item));
		}
		else
		{
			LPDLGTEMPLATE dlg = (LPDLGTEMPLATE)m_pData;
			dlg->cdit++;

			DLGITEMTEMPLATE item = {TWinTraits::GetWndStyle(0) | dwStyle, TWinTraits::GetWndExStyle(0) | dwExStyle, nX, nY, nWidth, nHeight, wId};
			AddData(&item, sizeof(item));
		}

		ATLASSERT(ClassName.m_lpstr != NULL);
		if (IS_INTRESOURCE(ClassName.m_lpstr))
		{
			WORD wData[] = { 0xFFFF, LOWORD(ClassName.m_lpstr) };
			AddData(wData, sizeof(wData));
		}
		else
		{
			AddString(ClassName.m_lpstr);
		}

		if (Text.m_lpstr == NULL)
		{
			WORD classData = 0;
			AddData(&classData, sizeof(WORD));
		}
		else if (IS_INTRESOURCE(Text.m_lpstr))
		{
			WORD wData[] = { 0xFFFF, LOWORD(Text.m_lpstr) };
			AddData(wData, sizeof(wData));
		}
		else
		{
			AddString(Text.m_lpstr);
		}

		AddData(&nCreationData, sizeof(nCreationData));

		if ((nCreationData != 0))
		{
			ATLASSERT(pCreationData != NULL);
			AddData(pCreationData, nCreationData * sizeof(WORD));
		}
	}

	void AddStdControl(StdCtrlType CtrlType, WORD wId, short nX, short nY, short nWidth, short nHeight,
	                   DWORD dwStyle, DWORD dwExStyle, ATL::_U_STRINGorID Text, const WORD* pCreationData = NULL, WORD nCreationData = 0, DWORD dwHelpID = 0)
	{
		AddControl(CtrlType, wId, nX, nY, nWidth, nHeight, dwStyle, dwExStyle, Text, pCreationData, nCreationData, dwHelpID);
	}

	void AddData(LPCVOID pData, size_t nData)
	{
		ATLASSERT(pData != NULL);

		const SIZE_T ALLOCATION_INCREMENT = 1024;

		if (m_pData == NULL)
		{
			m_cAllocated = ((nData / ALLOCATION_INCREMENT) + 1) * ALLOCATION_INCREMENT;
			m_hData = ::GlobalAlloc(GMEM_MOVEABLE | GMEM_ZEROINIT, m_cAllocated);
			ATLASSERT(m_hData != NULL);
			m_pPtr = m_pData = static_cast<LPBYTE>(::GlobalLock(m_hData));
			ATLASSERT(m_pData != NULL);
		}
		else if (((m_pPtr - m_pData) + nData) > m_cAllocated)
		{
			SIZE_T ptrPos = (m_pPtr - m_pData);
			m_cAllocated += ((nData / ALLOCATION_INCREMENT) + 1) * ALLOCATION_INCREMENT;
			::GlobalUnlock(m_pData);
			m_hData = ::GlobalReAlloc(m_hData, m_cAllocated, GMEM_MOVEABLE | GMEM_ZEROINIT);
			ATLASSERT(m_hData != NULL);
			m_pData = static_cast<LPBYTE>(::GlobalLock(m_hData));
			ATLASSERT(m_pData != NULL);
			m_pPtr = m_pData + ptrPos;
		}

		ATL::Checked::memcpy_s(m_pPtr, m_cAllocated - (m_pPtr - m_pData), pData, nData);

		m_pPtr += nData;
	}

	void AddString(LPCTSTR lpszStr)
	{
		if (lpszStr == NULL)
		{
			WCHAR szEmpty = 0;
			AddData(&szEmpty, sizeof(szEmpty));
		}
		else
		{
			USES_CONVERSION;
			LPCWSTR lpstr = T2CW(lpszStr);
			int nSize = lstrlenW(lpstr) + 1;
			AddData(lpstr, nSize * sizeof(WCHAR));
		}
	}
};

typedef CMemDlgTemplateT<CDlgControlWinTraits>	CMemDlgTemplate;


///////////////////////////////////////////////////////////////////////////////
// Dialog and control macros for indirect dialogs

// for DLGTEMPLATE
#define BEGIN_DIALOG(x, y, width, height) \
	void DoInitTemplate() \
	{ \
		bool bExTemplate = false; \
		short nX = x, nY = y, nWidth = width, nHeight = height; \
		LPCTSTR szCaption = NULL; \
		DWORD dwStyle = WS_POPUP | WS_BORDER | WS_SYSMENU; \
		DWORD dwExStyle = 0; \
		LPCTSTR szFontName = NULL; \
		WORD wFontSize = 0; \
		WORD wWeight = 0; \
		BYTE bItalic = 0; \
		BYTE bCharset = 0; \
		DWORD dwHelpID = 0; \
		ATL::_U_STRINGorID Menu = 0U; \
		ATL::_U_STRINGorID ClassName = 0U;

// for DLGTEMPLATEEX
#define BEGIN_DIALOG_EX(x, y, width, height, helpID) \
	void DoInitTemplate() \
	{ \
		bool bExTemplate = true; \
		short nX = x, nY = y, nWidth = width, nHeight = height; \
		LPCTSTR szCaption = NULL; \
		DWORD dwStyle = WS_POPUP | WS_BORDER | WS_SYSMENU; \
		DWORD dwExStyle = 0; \
		LPCTSTR szFontName = NULL; \
		WORD wFontSize = 0; \
		WORD wWeight = 0; \
		BYTE bItalic = 0; \
		BYTE bCharset = 0; \
		DWORD dwHelpID = helpID; \
		ATL::_U_STRINGorID Menu = 0U; \
		ATL::_U_STRINGorID ClassName = 0U;

#define END_DIALOG() \
		m_Template.Create(bExTemplate, szCaption, nX, nY, nWidth, nHeight, dwStyle, dwExStyle, szFontName, wFontSize, wWeight, bItalic, bCharset, dwHelpID, ClassName, Menu); \
	}

#define DIALOG_CAPTION(caption) \
		szCaption = caption;
#define DIALOG_STYLE(style) \
		dwStyle = style;
#define DIALOG_EXSTYLE(exStyle) \
		dwExStyle = exStyle;
#define DIALOG_FONT(pointSize, typeFace) \
		wFontSize = pointSize; \
		szFontName = typeFace;
#define DIALOG_FONT_EX(pointsize, typeface, weight, italic, charset) \
		ATLASSERT(bExTemplate); \
		wFontSize = pointsize; \
		szFontName = typeface; \
		wWeight = weight; \
		bItalic = italic; \
		bCharset = charset;
#define DIALOG_MENU(menuName) \
		Menu = menuName;
#define DIALOG_CLASS(className) \
		ClassName = className;

#define BEGIN_CONTROLS_MAP() \
	void DoInitControls() \
	{

#define END_CONTROLS_MAP() \
	}


#define CONTROL_LTEXT(text, id, x, y, width, height, style, exStyle) \
	m_Template.AddStdControl(m_Template.CTRL_STATIC, (WORD)id, x, y, width, height, style | SS_LEFT | WS_GROUP, exStyle, text, NULL, 0);
#define CONTROL_CTEXT(text, id, x, y, width, height, style, exStyle) \
	m_Template.AddStdControl(m_Template.CTRL_STATIC, (WORD)id, x, y, width, height, style | SS_CENTER | WS_GROUP, exStyle, text, NULL, 0);
#define CONTROL_RTEXT(text, id, x, y, width, height, style, exStyle) \
	m_Template.AddStdControl(m_Template.CTRL_STATIC, (WORD)id, x, y, width, height, style | SS_RIGHT | WS_GROUP, exStyle, text, NULL, 0);
#define CONTROL_PUSHBUTTON(text, id, x, y, width, height, style, exStyle) \
	m_Template.AddStdControl(m_Template.CTRL_BUTTON, (WORD)id, x, y, width, height, style | BS_PUSHBUTTON | WS_TABSTOP, exStyle, text, NULL, 0);
#define CONTROL_DEFPUSHBUTTON(text, id, x, y, width, height, style, exStyle) \
	m_Template.AddStdControl(m_Template.CTRL_BUTTON, (WORD)id, x, y, width, height, style | BS_DEFPUSHBUTTON | WS_TABSTOP, exStyle, text, NULL, 0);
#define CONTROL_PUSHBOX(text, id, x, y, width, height, style, exStyle) \
	m_Template.AddStdControl(m_Template.CTRL_BUTTON, (WORD)id, x, y, width, height, style | BS_PUSHBOX | WS_TABSTOP, exStyle, text, NULL, 0);
#define CONTROL_STATE3(text, id, x, y, width, height, style, exStyle) \
	m_Template.AddStdControl(m_Template.CTRL_BUTTON, (WORD)id, x, y, width, height, style | BS_3STATE | WS_TABSTOP, exStyle, text, NULL, 0);
#define CONTROL_AUTO3STATE(text, id, x, y, width, height, style, exStyle) \
	m_Template.AddStdControl(m_Template.CTRL_BUTTON, (WORD)id, x, y, width, height, style | BS_AUTO3STATE | WS_TABSTOP, exStyle, text, NULL, 0);
#define CONTROL_CHECKBOX(text, id, x, y, width, height, style, exStyle) \
	m_Template.AddStdControl(m_Template.CTRL_BUTTON, (WORD)id, x, y, width, height, style | BS_CHECKBOX | WS_TABSTOP, exStyle, text, NULL, 0);
#define CONTROL_AUTOCHECKBOX(text, id, x, y, width, height, style, exStyle) \
	m_Template.AddStdControl(m_Template.CTRL_BUTTON, (WORD)id, x, y, width, height, style | BS_AUTOCHECKBOX | WS_TABSTOP, exStyle, text, NULL, 0);
#define CONTROL_RADIOBUTTON(text, id, x, y, width, height, style, exStyle) \
	m_Template.AddStdControl(m_Template.CTRL_BUTTON, (WORD)id, x, y, width, height, style | BS_RADIOBUTTON | WS_TABSTOP, exStyle, text, NULL, 0);
#define CONTROL_AUTORADIOBUTTON(text, id, x, y, width, height, style, exStyle) \
	m_Template.AddStdControl(m_Template.CTRL_BUTTON, (WORD)id, x, y, width, height, style | BS_AUTORADIOBUTTON | WS_TABSTOP, exStyle, text, NULL, 0);
#define CONTROL_COMBOBOX(id, x, y, width, height, style, exStyle) \
	m_Template.AddStdControl(m_Template.CTRL_COMBOBOX, (WORD)id, x, y, width, height, style | CBS_DROPDOWN | WS_TABSTOP, exStyle, (LPCTSTR)NULL, NULL, 0);
#define CONTROL_EDITTEXT(id, x, y, width, height, style, exStyle) \
	m_Template.AddStdControl(m_Template.CTRL_EDIT, (WORD)id, x, y, width, height, style | ES_LEFT | WS_BORDER | WS_TABSTOP, exStyle, (LPCTSTR)NULL, NULL, 0);
#define CONTROL_GROUPBOX(text, id, x, y, width, height, style, exStyle) \
	m_Template.AddStdControl(m_Template.CTRL_BUTTON, (WORD)id, x, y, width, height, style | BS_GROUPBOX, exStyle, text, NULL, 0);
#define CONTROL_LISTBOX(id, x, y, width, height, style, exStyle) \
	m_Template.AddStdControl(m_Template.CTRL_LISTBOX, (WORD)id, x, y, width, height, style | LBS_NOTIFY | WS_BORDER, exStyle, (LPCTSTR)NULL, NULL, 0);
#define CONTROL_SCROLLBAR(id, x, y, width, height, style, exStyle) \
	m_Template.AddStdControl(m_Template.CTRL_SCROLLBAR, (WORD)id, x, y, width, height, style | SBS_HORZ, exStyle, (LPCTSTR)NULL, NULL, 0);
#define CONTROL_ICON(text, id, x, y, width, height, style, exStyle) \
	m_Template.AddStdControl(m_Template.CTRL_STATIC, (WORD)id, x, y, width, height, style | SS_ICON, exStyle, text, NULL, 0);
#define CONTROL_CONTROL(text, id, className, style, x, y, width, height, exStyle) \
	m_Template.AddControl(className, (WORD)id, x, y, width, height, style, exStyle, text, NULL, 0);


///////////////////////////////////////////////////////////////////////////////
// CIndirectDialogImpl - dialogs with template in memory

template <class T, class TDlgTemplate = CMemDlgTemplate, class TBase = ATL::CWindow>
class ATL_NO_VTABLE CIndirectDialogImpl : public ATL::CDialogImpl< T, TBase >
{
public:
	enum { IDD = 0 };   // no dialog template resource

	TDlgTemplate m_Template;

	void CreateTemplate()
	{
		T* pT = static_cast<T*>(this);
		pT->DoInitTemplate();
		pT->DoInitControls();
	}

	INT_PTR DoModal(HWND hWndParent = ::GetActiveWindow(), LPARAM dwInitParam = NULL)
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(pT->m_hWnd == NULL);

		if(!m_Template.IsValid())
			CreateTemplate();

		// Allocate the thunk structure here, where we can fail gracefully.
		BOOL bRet = this->m_thunk.Init(NULL, NULL);
		if(bRet == FALSE)
		{
			::SetLastError(ERROR_OUTOFMEMORY);
			return -1;
		}

		ModuleHelper::AddCreateWndData(&this->m_thunk.cd, (ATL::CDialogImplBaseT< TBase >*)pT);

#ifdef _DEBUG
		this->m_bModal = true;
#endif // _DEBUG

		return ::DialogBoxIndirectParam(ModuleHelper::GetResourceInstance(), m_Template.GetTemplatePtr(), hWndParent, (DLGPROC)T::StartDialogProc, dwInitParam);
	}

	HWND Create(HWND hWndParent, LPARAM dwInitParam = NULL)
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(pT->m_hWnd == NULL);

		if(!m_Template.IsValid())
			CreateTemplate();

		// Allocate the thunk structure here, where we can fail gracefully.
		BOOL bRet = this->m_thunk.Init(NULL, NULL);
		if(bRet == FALSE) 
		{
			::SetLastError(ERROR_OUTOFMEMORY);
			return NULL;
		}

		ModuleHelper::AddCreateWndData(&this->m_thunk.cd, (ATL::CDialogImplBaseT< TBase >*)pT);

#ifdef _DEBUG
		this->m_bModal = false;
#endif // _DEBUG

		HWND hWnd = ::CreateDialogIndirectParam(ModuleHelper::GetResourceInstance(), (LPCDLGTEMPLATE)m_Template.GetTemplatePtr(), hWndParent, (DLGPROC)T::StartDialogProc, dwInitParam);
		ATLASSERT(this->m_hWnd == hWnd);

		return hWnd;
	}

	// for CComControl
	HWND Create(HWND hWndParent, RECT&, LPARAM dwInitParam = NULL)
	{
		return Create(hWndParent, dwInitParam);
	}

	void DoInitTemplate() 
	{
		ATLASSERT(FALSE);   // MUST be defined in derived class
	}

	void DoInitControls() 
	{
		ATLASSERT(FALSE);   // MUST be defined in derived class
	}
};


///////////////////////////////////////////////////////////////////////////////
// CPropertySheetWindow - client side for a property sheet

class CPropertySheetWindow : public ATL::CWindow
{
public:
// Constructors
	CPropertySheetWindow(HWND hWnd = NULL) : ATL::CWindow(hWnd)
	{ }

	CPropertySheetWindow& operator =(HWND hWnd)
	{
		m_hWnd = hWnd;
		return *this;
	}

// Attributes
	int GetPageCount() const
	{
		ATLASSERT(::IsWindow(m_hWnd));
		HWND hWndTabCtrl = GetTabControl();
		ATLASSERT(hWndTabCtrl != NULL);
		return (int)::SendMessage(hWndTabCtrl, TCM_GETITEMCOUNT, 0, 0L);
	}

	HWND GetActivePage() const
	{
		ATLASSERT(::IsWindow(m_hWnd));
		return (HWND)::SendMessage(m_hWnd, PSM_GETCURRENTPAGEHWND, 0, 0L);
	}

	int GetActiveIndex() const
	{
		ATLASSERT(::IsWindow(m_hWnd));
		HWND hWndTabCtrl = GetTabControl();
		ATLASSERT(hWndTabCtrl != NULL);
		return (int)::SendMessage(hWndTabCtrl, TCM_GETCURSEL, 0, 0L);
	}

	BOOL SetActivePage(int nPageIndex)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		return (BOOL)::SendMessage(m_hWnd, PSM_SETCURSEL, nPageIndex, 0L);
	}

	BOOL SetActivePage(HPROPSHEETPAGE hPage)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		ATLASSERT(hPage != NULL);
		return (BOOL)::SendMessage(m_hWnd, PSM_SETCURSEL, 0, (LPARAM)hPage);
	}

	BOOL SetActivePageByID(int nPageID)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		return (BOOL)::SendMessage(m_hWnd, PSM_SETCURSELID, 0, nPageID);
	}

	void SetTitle(LPCTSTR lpszText, UINT nStyle = 0)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		ATLASSERT((nStyle & ~PSH_PROPTITLE) == 0); // only PSH_PROPTITLE is valid
		ATLASSERT(lpszText != NULL);
		::SendMessage(m_hWnd, PSM_SETTITLE, nStyle, (LPARAM)lpszText);
	}

	HWND GetTabControl() const
	{
		ATLASSERT(::IsWindow(m_hWnd));
		return (HWND)::SendMessage(m_hWnd, PSM_GETTABCONTROL, 0, 0L);
	}

	void SetFinishText(LPCTSTR lpszText)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		::SendMessage(m_hWnd, PSM_SETFINISHTEXT, 0, (LPARAM)lpszText);
	}

	void SetWizardButtons(DWORD dwFlags)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		::PostMessage(m_hWnd, PSM_SETWIZBUTTONS, 0, dwFlags);
	}

// Operations
	BOOL AddPage(HPROPSHEETPAGE hPage)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		ATLASSERT(hPage != NULL);
		return (BOOL)::SendMessage(m_hWnd, PSM_ADDPAGE, 0, (LPARAM)hPage);
	}

	BOOL AddPage(LPCPROPSHEETPAGE pPage)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		ATLASSERT(pPage != NULL);
		HPROPSHEETPAGE hPage = ::CreatePropertySheetPage(pPage);
		if(hPage == NULL)
			return FALSE;
		return (BOOL)::SendMessage(m_hWnd, PSM_ADDPAGE, 0, (LPARAM)hPage);
	}

	BOOL InsertPage(int nNewPageIndex, HPROPSHEETPAGE hPage)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		ATLASSERT(hPage != NULL);
		return (BOOL)::SendMessage(m_hWnd, PSM_INSERTPAGE, nNewPageIndex, (LPARAM)hPage);
	}

	BOOL InsertPage(int nNewPageIndex, LPCPROPSHEETPAGE pPage)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		ATLASSERT(pPage != NULL);
		HPROPSHEETPAGE hPage = ::CreatePropertySheetPage(pPage);
		if(hPage == NULL)
			return FALSE;
		return (BOOL)::SendMessage(m_hWnd, PSM_INSERTPAGE, nNewPageIndex, (LPARAM)hPage);
	}

	BOOL InsertPage(HPROPSHEETPAGE hPageInsertAfter, HPROPSHEETPAGE hPage)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		ATLASSERT(hPage != NULL);
		return (BOOL)::SendMessage(m_hWnd, PSM_INSERTPAGE, (WPARAM)hPageInsertAfter, (LPARAM)hPage);
	}

	BOOL InsertPage(HPROPSHEETPAGE hPageInsertAfter, LPCPROPSHEETPAGE pPage)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		ATLASSERT(pPage != NULL);
		HPROPSHEETPAGE hPage = ::CreatePropertySheetPage(pPage);
		if(hPage == NULL)
			return FALSE;
		return (BOOL)::SendMessage(m_hWnd, PSM_INSERTPAGE, (WPARAM)hPageInsertAfter, (LPARAM)hPage);
	}

	void RemovePage(int nPageIndex)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		::SendMessage(m_hWnd, PSM_REMOVEPAGE, nPageIndex, 0L);
	}

	void RemovePage(HPROPSHEETPAGE hPage)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		ATLASSERT(hPage != NULL);
		::SendMessage(m_hWnd, PSM_REMOVEPAGE, 0, (LPARAM)hPage);
	}

	BOOL PressButton(int nButton)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		return (BOOL)::SendMessage(m_hWnd, PSM_PRESSBUTTON, nButton, 0L);
	}

	BOOL Apply()
	{
		ATLASSERT(::IsWindow(m_hWnd));
		return (BOOL)::SendMessage(m_hWnd, PSM_APPLY, 0, 0L);
	}

	void CancelToClose()
	{
		ATLASSERT(::IsWindow(m_hWnd));
		::SendMessage(m_hWnd, PSM_CANCELTOCLOSE, 0, 0L);
	}

	void SetModified(HWND hWndPage, BOOL bChanged = TRUE)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		ATLASSERT(::IsWindow(hWndPage));
		UINT uMsg = bChanged ? PSM_CHANGED : PSM_UNCHANGED;
		::SendMessage(m_hWnd, uMsg, (WPARAM)hWndPage, 0L);
	}

	LRESULT QuerySiblings(WPARAM wParam, LPARAM lParam)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		return ::SendMessage(m_hWnd, PSM_QUERYSIBLINGS, wParam, lParam);
	}

	void RebootSystem()
	{
		ATLASSERT(::IsWindow(m_hWnd));
		::SendMessage(m_hWnd, PSM_REBOOTSYSTEM, 0, 0L);
	}

	void RestartWindows()
	{
		ATLASSERT(::IsWindow(m_hWnd));
		::SendMessage(m_hWnd, PSM_RESTARTWINDOWS, 0, 0L);
	}

	BOOL IsDialogMessage(LPMSG lpMsg)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		return (BOOL)::SendMessage(m_hWnd, PSM_ISDIALOGMESSAGE, 0, (LPARAM)lpMsg);
	}

	int HwndToIndex(HWND hWnd) const
	{
		ATLASSERT(::IsWindow(m_hWnd));
		return (int)::SendMessage(m_hWnd, PSM_HWNDTOINDEX, (WPARAM)hWnd, 0L);
	}

	HWND IndexToHwnd(int nIndex) const
	{
		ATLASSERT(::IsWindow(m_hWnd));
		return (HWND)::SendMessage(m_hWnd, PSM_INDEXTOHWND, nIndex, 0L);
	}

	int PageToIndex(HPROPSHEETPAGE hPage) const
	{
		ATLASSERT(::IsWindow(m_hWnd));
		return (int)::SendMessage(m_hWnd, PSM_PAGETOINDEX, 0, (LPARAM)hPage);
	}

	HPROPSHEETPAGE IndexToPage(int nIndex) const
	{
		ATLASSERT(::IsWindow(m_hWnd));
		return (HPROPSHEETPAGE)::SendMessage(m_hWnd, PSM_INDEXTOPAGE, nIndex, 0L);
	}

	int IdToIndex(int nID) const
	{
		ATLASSERT(::IsWindow(m_hWnd));
		return (int)::SendMessage(m_hWnd, PSM_IDTOINDEX, 0, nID);
	}

	int IndexToId(int nIndex) const
	{
		ATLASSERT(::IsWindow(m_hWnd));
		return (int)::SendMessage(m_hWnd, PSM_INDEXTOID, nIndex, 0L);
	}

	int GetResult() const
	{
		ATLASSERT(::IsWindow(m_hWnd));
		return (int)::SendMessage(m_hWnd, PSM_GETRESULT, 0, 0L);
	}

	BOOL RecalcPageSizes()
	{
		ATLASSERT(::IsWindow(m_hWnd));
		return (BOOL)::SendMessage(m_hWnd, PSM_RECALCPAGESIZES, 0, 0L);
	}

	void SetHeaderTitle(int nIndex, LPCTSTR lpstrHeaderTitle)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		::SendMessage(m_hWnd, PSM_SETHEADERTITLE, nIndex, (LPARAM)lpstrHeaderTitle);
	}

	void SetHeaderSubTitle(int nIndex, LPCTSTR lpstrHeaderSubTitle)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		::SendMessage(m_hWnd, PSM_SETHEADERSUBTITLE, nIndex, (LPARAM)lpstrHeaderSubTitle);
	}

// Implementation - override to prevent usage
	HWND Create(LPCTSTR, HWND, ATL::_U_RECT = NULL, LPCTSTR = NULL, DWORD = 0, DWORD = 0, ATL::_U_MENUorID = 0U, LPVOID = NULL)
	{
		ATLASSERT(FALSE);
		return NULL;
	}
};

///////////////////////////////////////////////////////////////////////////////
// CPropertySheetImpl - implements a property sheet

template <class T, class TBase = CPropertySheetWindow>
class ATL_NO_VTABLE CPropertySheetImpl : public ATL::CWindowImplBaseT< TBase >
{
public:
	PROPSHEETHEADER m_psh;
	ATL::CSimpleArray<HPROPSHEETPAGE> m_arrPages;

// Construction/Destruction
	CPropertySheetImpl(ATL::_U_STRINGorID title = (LPCTSTR)NULL, UINT uStartPage = 0, HWND hWndParent = NULL)
	{
		memset(&m_psh, 0, sizeof(PROPSHEETHEADER));
		m_psh.dwSize = sizeof(PROPSHEETHEADER);
		m_psh.dwFlags = PSH_USECALLBACK;
		m_psh.hInstance = ModuleHelper::GetResourceInstance();
		m_psh.phpage = NULL;   // will be set later
		m_psh.nPages = 0;      // will be set later
		m_psh.pszCaption = title.m_lpstr;
		m_psh.nStartPage = uStartPage;
		m_psh.hwndParent = hWndParent;   // if NULL, will be set in DoModal/Create
		m_psh.pfnCallback = T::PropSheetCallback;
	}

	~CPropertySheetImpl()
	{
		if(m_arrPages.GetSize() > 0)   // sheet never created, destroy all pages
		{
			for(int i = 0; i < m_arrPages.GetSize(); i++)
				::DestroyPropertySheetPage((HPROPSHEETPAGE)m_arrPages[i]);
		}
	}

// Callback function and overrideables
	static int CALLBACK PropSheetCallback(HWND hWnd, UINT uMsg, LPARAM lParam)
	{
		(void)lParam;   // avoid level 4 warning
		int nRet = 0;

		if(uMsg == PSCB_INITIALIZED)
		{
			ATLASSERT(hWnd != NULL);
			T* pT = (T*)ModuleHelper::ExtractCreateWndData();
			// subclass the sheet window
			pT->SubclassWindow(hWnd);
			// remove page handles array
			pT->_CleanUpPages();

			pT->OnSheetInitialized();
		}

		return nRet;
	}

	void OnSheetInitialized()
	{
	}

// Create method
	HWND Create(HWND hWndParent = NULL)
	{
		ATLASSERT(this->m_hWnd == NULL);

		m_psh.dwFlags |= PSH_MODELESS;
		if(m_psh.hwndParent == NULL)
			m_psh.hwndParent = hWndParent;
		m_psh.phpage = (HPROPSHEETPAGE*)m_arrPages.GetData();
		m_psh.nPages = m_arrPages.GetSize();

		T* pT = static_cast<T*>(this);

		// Allocate the thunk structure here, where we can fail gracefully.
		BOOL bRet = pT->m_thunk.Init(NULL, NULL);
		if(bRet == FALSE)
		{
			::SetLastError(ERROR_OUTOFMEMORY);
			return NULL;
		}

		ModuleHelper::AddCreateWndData(&pT->m_thunk.cd, pT);

		HWND hWnd = (HWND)::PropertySheet(&m_psh);
		_CleanUpPages();   // ensure clean-up, required if call failed

		ATLASSERT(this->m_hWnd == hWnd);

		return hWnd;
	}

	INT_PTR DoModal(HWND hWndParent = ::GetActiveWindow())
	{
		ATLASSERT(this->m_hWnd == NULL);

		m_psh.dwFlags &= ~PSH_MODELESS;
		if(m_psh.hwndParent == NULL)
			m_psh.hwndParent = hWndParent;
		m_psh.phpage = (HPROPSHEETPAGE*)m_arrPages.GetData();
		m_psh.nPages = m_arrPages.GetSize();

		T* pT = static_cast<T*>(this);

		// Allocate the thunk structure here, where we can fail gracefully.
		BOOL bRet = pT->m_thunk.Init(NULL, NULL);
		if(bRet == FALSE)
		{
			::SetLastError(ERROR_OUTOFMEMORY);
			return -1;
		}

		ModuleHelper::AddCreateWndData(&pT->m_thunk.cd, pT);

		INT_PTR nRet = ::PropertySheet(&m_psh);
		_CleanUpPages();   // ensure clean-up, required if call failed

		return nRet;
	}

	// implementation helper - clean up pages array
	void _CleanUpPages()
	{
		m_psh.nPages = 0;
		m_psh.phpage = NULL;
		m_arrPages.RemoveAll();
	}

// Attributes (extended overrides of client class methods)
// These now can be called before the sheet is created
// Note: Calling these after the sheet is created gives unpredictable results
	int GetPageCount() const
	{
		if(this->m_hWnd == NULL)   // not created yet
			return m_arrPages.GetSize();
		return TBase::GetPageCount();
	}

	int GetActiveIndex() const
	{
		if(this->m_hWnd == NULL)   // not created yet
			return m_psh.nStartPage;
		return TBase::GetActiveIndex();
	}

	HPROPSHEETPAGE GetPage(int nPageIndex) const
	{
		ATLASSERT(this->m_hWnd == NULL);   // can't do this after it's created
		return (HPROPSHEETPAGE)m_arrPages[nPageIndex];
	}

	int GetPageIndex(HPROPSHEETPAGE hPage) const
	{
		ATLASSERT(this->m_hWnd == NULL);   // can't do this after it's created
		return m_arrPages.Find((HPROPSHEETPAGE&)hPage);
	}

	BOOL SetActivePage(int nPageIndex)
	{
		if(this->m_hWnd == NULL)   // not created yet
		{
			ATLASSERT((nPageIndex >= 0) && (nPageIndex < m_arrPages.GetSize()));
			m_psh.nStartPage = nPageIndex;
			return TRUE;
		}
		return TBase::SetActivePage(nPageIndex);
	}

	BOOL SetActivePage(HPROPSHEETPAGE hPage)
	{
		ATLASSERT(hPage != NULL);
		if(this->m_hWnd == NULL)   // not created yet
		{
			int nPageIndex = GetPageIndex(hPage);
			if(nPageIndex == -1)
				return FALSE;

			return SetActivePage(nPageIndex);
		}
		return TBase::SetActivePage(hPage);

	}

	void SetTitle(LPCTSTR lpszText, UINT nStyle = 0)
	{
		ATLASSERT((nStyle & ~PSH_PROPTITLE) == 0);   // only PSH_PROPTITLE is valid
		ATLASSERT(lpszText != NULL);

		if(this->m_hWnd == NULL)
		{
			// set internal state
			m_psh.pszCaption = lpszText;   // must exist until sheet is created
			m_psh.dwFlags &= ~PSH_PROPTITLE;
			m_psh.dwFlags |= nStyle;
		}
		else
		{
			// set external state
			TBase::SetTitle(lpszText, nStyle);
		}
	}

	void SetWizardMode()
	{
		m_psh.dwFlags |= PSH_WIZARD;
	}

	void EnableHelp()
	{
		m_psh.dwFlags |= PSH_HASHELP;
	}

// Operations
	BOOL AddPage(HPROPSHEETPAGE hPage)
	{
		ATLASSERT(hPage != NULL);
		BOOL bRet = FALSE;
		if(this->m_hWnd != NULL)
			bRet = TBase::AddPage(hPage);
		else	// sheet not created yet, use internal data
			bRet = m_arrPages.Add((HPROPSHEETPAGE&)hPage);
		return bRet;
	}

	BOOL AddPage(LPCPROPSHEETPAGE pPage)
	{
		ATLASSERT(pPage != NULL);
		HPROPSHEETPAGE hPage = ::CreatePropertySheetPage(pPage);
		if(hPage == NULL)
			return FALSE;
		BOOL bRet = AddPage(hPage);
		if(!bRet)
			::DestroyPropertySheetPage(hPage);
		return bRet;
	}

	BOOL RemovePage(HPROPSHEETPAGE hPage)
	{
		ATLASSERT(hPage != NULL);
		if(this->m_hWnd == NULL)   // not created yet
		{
			int nPage = GetPageIndex(hPage);
			if(nPage == -1)
				return FALSE;
			return RemovePage(nPage);
		}
		TBase::RemovePage(hPage);
		return TRUE;

	}

	BOOL RemovePage(int nPageIndex)
	{
		BOOL bRet = TRUE;
		if(this->m_hWnd != NULL)
			TBase::RemovePage(nPageIndex);
		else	// sheet not created yet, use internal data
			bRet = m_arrPages.RemoveAt(nPageIndex);
		return bRet;
	}

	void SetHeader(LPCTSTR szbmHeader)
	{
		ATLASSERT(this->m_hWnd == NULL);   // can't do this after it's created

		m_psh.dwFlags &= ~PSH_WIZARD;
		m_psh.dwFlags |= (PSH_HEADER | PSH_WIZARD97);
		m_psh.pszbmHeader = szbmHeader;
	}

	void SetHeader(HBITMAP hbmHeader)
	{
		ATLASSERT(this->m_hWnd == NULL);   // can't do this after it's created

		m_psh.dwFlags &= ~PSH_WIZARD;
		m_psh.dwFlags |= (PSH_HEADER | PSH_USEHBMHEADER | PSH_WIZARD97);
		m_psh.hbmHeader = hbmHeader;
	}

	void SetWatermark(LPCTSTR szbmWatermark, HPALETTE hplWatermark = NULL)
	{
		ATLASSERT(this->m_hWnd == NULL);   // can't do this after it's created

		m_psh.dwFlags &= ~PSH_WIZARD;
		m_psh.dwFlags |= PSH_WATERMARK | PSH_WIZARD97;
		m_psh.pszbmWatermark = szbmWatermark;

		if(hplWatermark != NULL)
		{
			m_psh.dwFlags |= PSH_USEHPLWATERMARK;
			m_psh.hplWatermark = hplWatermark;
		}
	}

	void SetWatermark(HBITMAP hbmWatermark, HPALETTE hplWatermark = NULL)
	{
		ATLASSERT(this->m_hWnd == NULL);   // can't do this after it's created

		m_psh.dwFlags &= ~PSH_WIZARD;
		m_psh.dwFlags |= (PSH_WATERMARK | PSH_USEHBMWATERMARK | PSH_WIZARD97);
		m_psh.hbmWatermark = hbmWatermark;

		if(hplWatermark != NULL)
		{
			m_psh.dwFlags |= PSH_USEHPLWATERMARK;
			m_psh.hplWatermark = hplWatermark;
		}
	}

	void StretchWatermark(bool bStretchWatermark)
	{
		ATLASSERT(this->m_hWnd == NULL);   // can't do this after it's created
		if(bStretchWatermark)
			m_psh.dwFlags |= PSH_STRETCHWATERMARK;
		else
			m_psh.dwFlags &= ~PSH_STRETCHWATERMARK;
	}

// Message map and handlers
	BEGIN_MSG_MAP(CPropertySheetImpl)
		MESSAGE_HANDLER(WM_COMMAND, OnCommand)
		MESSAGE_HANDLER(WM_SYSCOMMAND, OnSysCommand)
	END_MSG_MAP()

	LRESULT OnCommand(UINT uMsg, WPARAM wParam, LPARAM lParam, BOOL& /*bHandled*/)
	{
		LRESULT lRet = this->DefWindowProc(uMsg, wParam, lParam);
		if((HIWORD(wParam) == BN_CLICKED) && ((LOWORD(wParam) == IDOK) || (LOWORD(wParam) == IDCANCEL)) &&
		   ((m_psh.dwFlags & PSH_MODELESS) != 0) && (this->GetActivePage() == NULL))
			this->DestroyWindow();
		return lRet;
	}

	LRESULT OnSysCommand(UINT /*uMsg*/, WPARAM wParam, LPARAM /*lParam*/, BOOL& bHandled)
	{
		if(((m_psh.dwFlags & PSH_MODELESS) == PSH_MODELESS) && ((wParam & 0xFFF0) == SC_CLOSE))
			this->SendMessage(WM_CLOSE);
		else
			bHandled = FALSE;
		return 0;
	}
};

// for non-customized sheets
class CPropertySheet : public CPropertySheetImpl<CPropertySheet>
{
public:
	CPropertySheet(ATL::_U_STRINGorID title = (LPCTSTR)NULL, UINT uStartPage = 0, HWND hWndParent = NULL)
		: CPropertySheetImpl<CPropertySheet>(title, uStartPage, hWndParent)
	{ }
};


///////////////////////////////////////////////////////////////////////////////
// CPropertyPageWindow - client side for a property page

class CPropertyPageWindow : public ATL::CWindow
{
public:
// Constructors
	CPropertyPageWindow(HWND hWnd = NULL) : ATL::CWindow(hWnd)
	{ }

	CPropertyPageWindow& operator =(HWND hWnd)
	{
		m_hWnd = hWnd;
		return *this;
	}

// Attributes
	CPropertySheetWindow GetPropertySheet() const
	{
		ATLASSERT(::IsWindow(m_hWnd));
		return CPropertySheetWindow(GetParent());
	}

// Operations
	BOOL Apply()
	{
		ATLASSERT(::IsWindow(m_hWnd));
		ATLASSERT(GetParent() != NULL);
		return GetPropertySheet().Apply();
	}

	void CancelToClose()
	{
		ATLASSERT(::IsWindow(m_hWnd));
		ATLASSERT(GetParent() != NULL);
		GetPropertySheet().CancelToClose();
	}

	void SetModified(BOOL bChanged = TRUE)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		ATLASSERT(GetParent() != NULL);
		GetPropertySheet().SetModified(m_hWnd, bChanged);
	}

	LRESULT QuerySiblings(WPARAM wParam, LPARAM lParam)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		ATLASSERT(GetParent() != NULL);
		return GetPropertySheet().QuerySiblings(wParam, lParam);
	}

	void RebootSystem()
	{
		ATLASSERT(::IsWindow(m_hWnd));
		ATLASSERT(GetParent() != NULL);
		GetPropertySheet().RebootSystem();
	}

	void RestartWindows()
	{
		ATLASSERT(::IsWindow(m_hWnd));
		ATLASSERT(GetParent() != NULL);
		GetPropertySheet().RestartWindows();
	}

	void SetWizardButtons(DWORD dwFlags)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		ATLASSERT(GetParent() != NULL);
		GetPropertySheet().SetWizardButtons(dwFlags);
	}

// Implementation - overrides to prevent usage
	HWND Create(LPCTSTR, HWND, ATL::_U_RECT = NULL, LPCTSTR = NULL, DWORD = 0, DWORD = 0, ATL::_U_MENUorID = 0U, LPVOID = NULL)
	{
		ATLASSERT(FALSE);
		return NULL;
	}
};

///////////////////////////////////////////////////////////////////////////////
// CPropertyPageImpl - implements a property page

#if defined(_WTL_FORCE_OLD_PAGE_NOTIFY_HANDLERS) && defined(_WTL_NEW_PAGE_NOTIFY_HANDLERS)
	#error _WTL_FORCE_OLD_PAGE_NOTIFY_HANDLERS and _WTL_NEW_PAGE_NOTIFY_HANDLERS cannot be both defined
#endif

#if !defined(_WTL_FORCE_OLD_PAGE_NOTIFY_HANDLERS) && !defined(_WTL_NEW_PAGE_NOTIFY_HANDLERS)
  #define _WTL_NEW_PAGE_NOTIFY_HANDLERS
#endif

// NOTE: _WTL_NEW_PAGE_NOTIFY_HANDLERS is now defined by default.
// It enables use of new notification handlers that 
// return direct values without any restrictions.
// Define _WTL_FORCE_OLD_PAGE_NOTIFY_HANDLERS to use old handlers.

template <class T, class TBase = CPropertyPageWindow>
class ATL_NO_VTABLE CPropertyPageImpl : public ATL::CDialogImplBaseT< TBase >
{
public:
	PROPSHEETPAGE m_psp;

	operator PROPSHEETPAGE*() { return &m_psp; }

// Construction
	CPropertyPageImpl(ATL::_U_STRINGorID title = (LPCTSTR)NULL)
	{
		// initialize PROPSHEETPAGE struct
		memset(&m_psp, 0, sizeof(PROPSHEETPAGE));
		m_psp.dwSize = sizeof(PROPSHEETPAGE);
		m_psp.dwFlags = PSP_USECALLBACK;
		m_psp.hInstance = ModuleHelper::GetResourceInstance();
		T* pT = static_cast<T*>(this);
		m_psp.pszTemplate = MAKEINTRESOURCE(pT->IDD);
		m_psp.pfnDlgProc = (DLGPROC)T::StartDialogProc;
		m_psp.pfnCallback = T::PropPageCallback;
		m_psp.lParam = (LPARAM)pT;

		if(title.m_lpstr != NULL)
			SetTitle(title);
	}

// Callback function and overrideables
	static UINT CALLBACK PropPageCallback(HWND hWnd, UINT uMsg, LPPROPSHEETPAGE ppsp)
	{
		(void)hWnd;   // avoid level 4 warning
		ATLASSERT(hWnd == NULL);
		T* pT = (T*)ppsp->lParam;
		UINT uRet = 0;

		switch(uMsg)
		{
		case PSPCB_CREATE:
			{
				ATL::CDialogImplBaseT< TBase >* pPage = (ATL::CDialogImplBaseT< TBase >*)pT;
				ModuleHelper::AddCreateWndData(&pPage->m_thunk.cd, pPage);
				uRet = pT->OnPageCreate() ? 1 : 0;
			}
			break;
		case PSPCB_ADDREF:
			pT->OnPageAddRef();
			break;
		case PSPCB_RELEASE:
			pT->OnPageRelease();
			break;
		default:
			break;
		}

		return uRet;
	}

	bool OnPageCreate()
	{
		return true;   // true - allow page to be created, false - prevent creation
	}

	void OnPageAddRef()
	{
	}

	void OnPageRelease()
	{
	}

// Create method
	HPROPSHEETPAGE Create()
	{
		return ::CreatePropertySheetPage(&m_psp);
	}

// Attributes
	void SetTitle(ATL::_U_STRINGorID title)
	{
		m_psp.pszTitle = title.m_lpstr;
		m_psp.dwFlags |= PSP_USETITLE;
	}

	void SetHeaderTitle(LPCTSTR lpstrHeaderTitle)
	{
		ATLASSERT(this->m_hWnd == NULL);   // can't do this after it's created
		m_psp.dwFlags |= PSP_USEHEADERTITLE;
		m_psp.pszHeaderTitle = lpstrHeaderTitle;
	}

	void SetHeaderSubTitle(LPCTSTR lpstrHeaderSubTitle)
	{
		ATLASSERT(this->m_hWnd == NULL);   // can't do this after it's created
		m_psp.dwFlags |= PSP_USEHEADERSUBTITLE;
		m_psp.pszHeaderSubTitle = lpstrHeaderSubTitle;
	}

// Operations
	void EnableHelp()
	{
		m_psp.dwFlags |= PSP_HASHELP;
	}

// Message map and handlers
	BEGIN_MSG_MAP(CPropertyPageImpl)
		MESSAGE_HANDLER(WM_NOTIFY, OnNotify)
	END_MSG_MAP()

	LRESULT OnNotify(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM lParam, BOOL& bHandled)
	{
		ATLASSERT(::IsWindow(this->m_hWnd));
		NMHDR* pNMHDR = (NMHDR*)lParam;

		// don't handle messages not from the page/sheet itself
		if((pNMHDR->hwndFrom != this->m_hWnd) && (pNMHDR->hwndFrom != ::GetParent(this->m_hWnd)))
		{
			bHandled = FALSE;
			return 1;
		}

		T* pT = static_cast<T*>(this);
		LRESULT lResult = 0;
		switch(pNMHDR->code)
		{
#ifdef _WTL_NEW_PAGE_NOTIFY_HANDLERS
		case PSN_SETACTIVE:
			lResult = pT->OnSetActive();
			break;
		case PSN_KILLACTIVE:
			lResult = pT->OnKillActive();
			break;
		case PSN_APPLY:
			lResult = pT->OnApply();
			break;
		case PSN_RESET:
			pT->OnReset();
			break;
		case PSN_QUERYCANCEL:
			lResult = pT->OnQueryCancel();
			break;
		case PSN_WIZNEXT:
			lResult = pT->OnWizardNext();
			break;
		case PSN_WIZBACK:
			lResult = pT->OnWizardBack();
			break;
		case PSN_WIZFINISH:
			lResult = pT->OnWizardFinish();
			break;
		case PSN_HELP:
			pT->OnHelp();
			break;
		case PSN_GETOBJECT:
			if(!pT->OnGetObject((LPNMOBJECTNOTIFY)lParam))
				bHandled = FALSE;
			break;
		case PSN_TRANSLATEACCELERATOR:
			{
				LPPSHNOTIFY lpPSHNotify = (LPPSHNOTIFY)lParam;
				lResult = pT->OnTranslateAccelerator((LPMSG)lpPSHNotify->lParam);
			}
			break;
		case PSN_QUERYINITIALFOCUS:
			{
				LPPSHNOTIFY lpPSHNotify = (LPPSHNOTIFY)lParam;
				lResult = (LRESULT)pT->OnQueryInitialFocus((HWND)lpPSHNotify->lParam);
			}
			break;

#else // !_WTL_NEW_PAGE_NOTIFY_HANDLERS
		case PSN_SETACTIVE:
			lResult = pT->OnSetActive() ? 0 : -1;
			break;
		case PSN_KILLACTIVE:
			lResult = !pT->OnKillActive();
			break;
		case PSN_APPLY:
			lResult = pT->OnApply() ? PSNRET_NOERROR : PSNRET_INVALID_NOCHANGEPAGE;
			break;
		case PSN_RESET:
			pT->OnReset();
			break;
		case PSN_QUERYCANCEL:
			lResult = !pT->OnQueryCancel();
			break;
		case PSN_WIZNEXT:
			lResult = pT->OnWizardNext();
			break;
		case PSN_WIZBACK:
			lResult = pT->OnWizardBack();
			break;
		case PSN_WIZFINISH:
			lResult = !pT->OnWizardFinish();
			break;
		case PSN_HELP:
			pT->OnHelp();
			break;
		case PSN_GETOBJECT:
			if(!pT->OnGetObject((LPNMOBJECTNOTIFY)lParam))
				bHandled = FALSE;
			break;
		case PSN_TRANSLATEACCELERATOR:
			{
				LPPSHNOTIFY lpPSHNotify = (LPPSHNOTIFY)lParam;
				lResult = pT->OnTranslateAccelerator((LPMSG)lpPSHNotify->lParam) ? PSNRET_MESSAGEHANDLED : PSNRET_NOERROR;
			}
			break;
		case PSN_QUERYINITIALFOCUS:
			{
				LPPSHNOTIFY lpPSHNotify = (LPPSHNOTIFY)lParam;
				lResult = (LRESULT)pT->OnQueryInitialFocus((HWND)lpPSHNotify->lParam);
			}
			break;

#endif // !_WTL_NEW_PAGE_NOTIFY_HANDLERS
		default:
			bHandled = FALSE;   // not handled
		}

		return lResult;
	}

// Overridables
#ifdef _WTL_NEW_PAGE_NOTIFY_HANDLERS
	int OnSetActive()
	{
		// 0 = allow activate
		// -1 = go back that was active
		// page ID = jump to page
		return 0;
	}

	BOOL OnKillActive()
	{
		// FALSE = allow deactivate
		// TRUE = prevent deactivation
		return FALSE;
	}

	int OnApply()
	{
		// PSNRET_NOERROR = apply OK
		// PSNRET_INVALID = apply not OK, return to this page
		// PSNRET_INVALID_NOCHANGEPAGE = apply not OK, don't change focus
		return PSNRET_NOERROR;
	}

	void OnReset()
	{
	}

	BOOL OnQueryCancel()
	{
		// FALSE = allow cancel
		// TRUE = prevent cancel
		return FALSE;
	}

	int OnWizardBack()
	{
		// 0  = goto previous page
		// -1 = prevent page change
		// >0 = jump to page by dlg ID
		return 0;
	}

	int OnWizardNext()
	{
		// 0  = goto next page
		// -1 = prevent page change
		// >0 = jump to page by dlg ID
		return 0;
	}

	INT_PTR OnWizardFinish()
	{
		// FALSE = allow finish
		// TRUE = prevent finish
		// HWND = prevent finish and set focus to HWND (CommCtrl 5.80 only)
		return FALSE;
	}

	void OnHelp()
	{
	}

	BOOL OnGetObject(LPNMOBJECTNOTIFY /*lpObjectNotify*/)
	{
		return FALSE;   // not processed
	}

	int OnTranslateAccelerator(LPMSG /*lpMsg*/)
	{
		// PSNRET_NOERROR - message not handled
		// PSNRET_MESSAGEHANDLED - message handled
		return PSNRET_NOERROR;
	}

	HWND OnQueryInitialFocus(HWND /*hWndFocus*/)
	{
		// NULL = set focus to default control
		// HWND = set focus to HWND
		return NULL;
	}

#else // !_WTL_NEW_PAGE_NOTIFY_HANDLERS
	BOOL OnSetActive()
	{
		return TRUE;
	}

	BOOL OnKillActive()
	{
		return TRUE;
	}

	BOOL OnApply()
	{
		return TRUE;
	}

	void OnReset()
	{
	}

	BOOL OnQueryCancel()
	{
		return TRUE;    // ok to cancel
	}

	int OnWizardBack()
	{
		// 0  = goto previous page
		// -1 = prevent page change
		// >0 = jump to page by dlg ID
		return 0;
	}

	int OnWizardNext()
	{
		// 0  = goto next page
		// -1 = prevent page change
		// >0 = jump to page by dlg ID
		return 0;
	}

	BOOL OnWizardFinish()
	{
		return TRUE;
	}

	void OnHelp()
	{
	}

	BOOL OnGetObject(LPNMOBJECTNOTIFY /*lpObjectNotify*/)
	{
		return FALSE;   // not processed
	}

	BOOL OnTranslateAccelerator(LPMSG /*lpMsg*/)
	{
		return FALSE;   // not translated
	}

	HWND OnQueryInitialFocus(HWND /*hWndFocus*/)
	{
		return NULL;   // default
	}

#endif // !_WTL_NEW_PAGE_NOTIFY_HANDLERS
};

// for non-customized pages
template <WORD t_wDlgTemplateID>
class CPropertyPage : public CPropertyPageImpl<CPropertyPage<t_wDlgTemplateID> >
{
public:
	enum { IDD = t_wDlgTemplateID };

	CPropertyPage(ATL::_U_STRINGorID title = (LPCTSTR)NULL) : CPropertyPageImpl<CPropertyPage>(title)
	{ }

	DECLARE_EMPTY_MSG_MAP()
};

///////////////////////////////////////////////////////////////////////////////
// CAxPropertyPageImpl - property page that hosts ActiveX controls

#ifndef _ATL_NO_HOSTING

// Note: You must #include <atlhost.h> to use these classes

template <class T, class TBase = CPropertyPageWindow>
class ATL_NO_VTABLE CAxPropertyPageImpl : public CPropertyPageImpl< T, TBase >
{
public:
// Data members
	HGLOBAL m_hInitData;
	HGLOBAL m_hDlgRes;
	HGLOBAL m_hDlgResSplit;

// Constructor/destructor
	CAxPropertyPageImpl(ATL::_U_STRINGorID title = (LPCTSTR)NULL) : 
			CPropertyPageImpl< T, TBase >(title),
			m_hInitData(NULL), m_hDlgRes(NULL), m_hDlgResSplit(NULL)
	{
		T* pT = static_cast<T*>(this);
		(void)pT;   // avoid level 4 warning

		// initialize ActiveX hosting and modify dialog template
		ATL::AtlAxWinInit();

		HINSTANCE hInstance = ModuleHelper::GetResourceInstance();
		LPCTSTR lpTemplateName = MAKEINTRESOURCE(pT->IDD);
		HRSRC hDlg = ::FindResource(hInstance, lpTemplateName, (LPTSTR)RT_DIALOG);
		if(hDlg != NULL)
		{
			HRSRC hDlgInit = ::FindResource(hInstance, lpTemplateName, (LPTSTR)_ATL_RT_DLGINIT);

			BYTE* pInitData = NULL;
			if(hDlgInit != NULL)
			{
				m_hInitData = ::LoadResource(hInstance, hDlgInit);
				pInitData = (BYTE*)::LockResource(m_hInitData);
			}

			m_hDlgRes = ::LoadResource(hInstance, hDlg);
			DLGTEMPLATE* pDlg = (DLGTEMPLATE*)::LockResource(m_hDlgRes);
			LPCDLGTEMPLATE lpDialogTemplate = ATL::_DialogSplitHelper::SplitDialogTemplate(pDlg, pInitData);
			if(lpDialogTemplate != pDlg)
				m_hDlgResSplit = GlobalHandle(lpDialogTemplate);

			// set up property page to use in-memory dialog template
			if(lpDialogTemplate != NULL)
			{
				this->m_psp.dwFlags |= PSP_DLGINDIRECT;
				this->m_psp.pResource = lpDialogTemplate;
			}
			else
			{
				ATLASSERT(FALSE && _T("CAxPropertyPageImpl - ActiveX initializtion failed!"));
			}
		}
		else
		{
			ATLASSERT(FALSE && _T("CAxPropertyPageImpl - Cannot find dialog template!"));
		}
	}

	~CAxPropertyPageImpl()
	{
		if(m_hInitData != NULL)
		{
			UnlockResource(m_hInitData);
			FreeResource(m_hInitData);
		}
		if(m_hDlgRes != NULL)
		{
			UnlockResource(m_hDlgRes);
			FreeResource(m_hDlgRes);
		}
		if(m_hDlgResSplit != NULL)
		{
			::GlobalFree(m_hDlgResSplit);
		}
	}

// Methods
	// call this one to handle keyboard message for ActiveX controls
	BOOL PreTranslateMessage(LPMSG pMsg)
	{
		if (((pMsg->message < WM_KEYFIRST) || (pMsg->message > WM_KEYLAST)) &&
		   ((pMsg->message < WM_MOUSEFIRST) || (pMsg->message > WM_MOUSELAST)))
			return FALSE;
		// find a direct child of the dialog from the window that has focus
		HWND hWndCtl = ::GetFocus();
		if (this->IsChild(hWndCtl) && (::GetParent(hWndCtl) != this->m_hWnd))
		{
			do
			{
				hWndCtl = ::GetParent(hWndCtl);
			}
			while (::GetParent(hWndCtl) != this->m_hWnd);
		}
		// give controls a chance to translate this message
		return (BOOL)::SendMessage(hWndCtl, WM_FORWARDMSG, 0, (LPARAM)pMsg);
	}

// Overridables
	// new default implementation for ActiveX hosting pages
#ifdef _WTL_NEW_PAGE_NOTIFY_HANDLERS
	int OnTranslateAccelerator(LPMSG lpMsg)
	{
		T* pT = static_cast<T*>(this);
		return (pT->PreTranslateMessage(lpMsg) != FALSE) ? PSNRET_MESSAGEHANDLED : PSNRET_NOERROR;
	}
#else // !_WTL_NEW_PAGE_NOTIFY_HANDLERS
	BOOL OnTranslateAccelerator(LPMSG lpMsg)
	{
		T* pT = static_cast<T*>(this);
		return pT->PreTranslateMessage(lpMsg);
	}
#endif // !_WTL_NEW_PAGE_NOTIFY_HANDLERS

	int GetIDD()
	{
		return( static_cast<T*>(this)->IDD );
	}

	virtual DLGPROC GetDialogProc()
	{
		return DialogProc;
	}

	static INT_PTR CALLBACK DialogProc(HWND hWnd, UINT uMsg, WPARAM wParam, LPARAM lParam)
	{
		CAxPropertyPageImpl< T, TBase >* pThis = (CAxPropertyPageImpl< T, TBase >*)hWnd;
		if (uMsg == WM_INITDIALOG)
		{
			HRESULT hr;
			if (FAILED(hr = pThis->CreateActiveXControls(pThis->GetIDD())))
			{
				ATLASSERT(FALSE);
				return FALSE;
			}
		}
		return CPropertyPageImpl< T, TBase >::DialogProc(hWnd, uMsg, wParam, lParam);
	}

// ActiveX controls creation
	virtual HRESULT CreateActiveXControls(UINT nID)
	{
		// Load dialog template and InitData
		HRSRC hDlgInit = ::FindResource(ATL::_AtlBaseModule.GetResourceInstance(), MAKEINTRESOURCE(nID), (LPTSTR)_ATL_RT_DLGINIT);
		BYTE* pInitData = NULL;
		HGLOBAL hData = NULL;
		HRESULT hr = S_OK;
		if (hDlgInit != NULL)
		{
			hData = ::LoadResource(ATL::_AtlBaseModule.GetResourceInstance(), hDlgInit);
			if (hData != NULL)
				pInitData = (BYTE*) ::LockResource(hData);
		}

		HRSRC hDlg = ::FindResource(ATL::_AtlBaseModule.GetResourceInstance(), MAKEINTRESOURCE(nID), (LPTSTR)RT_DIALOG);
		if (hDlg != NULL)
		{
			HGLOBAL hResource = ::LoadResource(ATL::_AtlBaseModule.GetResourceInstance(), hDlg);
			DLGTEMPLATE* pDlg = NULL;
			if (hResource != NULL)
			{
				pDlg = (DLGTEMPLATE*) ::LockResource(hResource);
				if (pDlg != NULL)
				{
					// Get first control on the template
					BOOL bDialogEx = ATL::_DialogSplitHelper::IsDialogEx(pDlg);
					WORD nItems = ATL::_DialogSplitHelper::DlgTemplateItemCount(pDlg);

					// Get first control on the dialog
					DLGITEMTEMPLATE* pItem = ATL::_DialogSplitHelper::FindFirstDlgItem(pDlg);
					HWND hWndPrev = this->GetWindow(GW_CHILD);

					// Create all ActiveX cotnrols in the dialog template and place them in the correct tab order (z-order)
					for (WORD nItem = 0; nItem < nItems; nItem++)
					{
						DWORD wID = bDialogEx ? ((ATL::_DialogSplitHelper::DLGITEMTEMPLATEEX*)pItem)->id : pItem->id;
						if (ATL::_DialogSplitHelper::IsActiveXControl(pItem, bDialogEx))
						{
							BYTE* pData = NULL;
							DWORD dwLen = ATL::_DialogSplitHelper::FindCreateData(wID, pInitData, &pData);
							ATL::CComPtr<IStream> spStream;
							if (dwLen != 0)
							{
								HGLOBAL h = GlobalAlloc(GHND, dwLen);
								if (h != NULL)
								{
									BYTE* pBytes = (BYTE*) GlobalLock(h);
									BYTE* pSource = pData; 
									ATL::Checked::memcpy_s(pBytes, dwLen, pSource, dwLen);
									GlobalUnlock(h);
									CreateStreamOnHGlobal(h, TRUE, &spStream);
								}
								else
								{
									hr = E_OUTOFMEMORY;
									break;
								}
							}

							ATL::CComBSTR bstrLicKey;
							hr = ATL::_DialogSplitHelper::ParseInitData(spStream, &bstrLicKey.m_str);
							if (SUCCEEDED(hr))
							{
								ATL::CAxWindow2 wnd;
								// Get control caption.
								LPWSTR pszClassName = 
									bDialogEx ? 
										(LPWSTR)(((ATL::_DialogSplitHelper::DLGITEMTEMPLATEEX*)pItem) + 1) :
										(LPWSTR)(pItem + 1);
								// Get control rect.
								RECT rect = {};
								rect.left = bDialogEx ? ((ATL::_DialogSplitHelper::DLGITEMTEMPLATEEX*)pItem)->x : pItem->x;
								rect.top = bDialogEx ? ((ATL::_DialogSplitHelper::DLGITEMTEMPLATEEX*)pItem)->y : pItem->y;
								rect.right = rect.left + (bDialogEx ? ((ATL::_DialogSplitHelper::DLGITEMTEMPLATEEX*)pItem)->cx : pItem->cx);
								rect.bottom = rect.top + (bDialogEx ? ((ATL::_DialogSplitHelper::DLGITEMTEMPLATEEX*)pItem)->cy : pItem->cy);

								// Convert from dialog units to screen units
								this->MapDialogRect(&rect);

								// Create AxWindow with a NULL caption.
								wnd.Create(this->m_hWnd,
									&rect, 
									NULL, 
									(bDialogEx ? 
										((ATL::_DialogSplitHelper::DLGITEMTEMPLATEEX*)pItem)->style : 
										pItem->style) | WS_TABSTOP, 
									bDialogEx ? 
										((ATL::_DialogSplitHelper::DLGITEMTEMPLATEEX*)pItem)->exStyle : 
										0,
									bDialogEx ? 
										((ATL::_DialogSplitHelper::DLGITEMTEMPLATEEX*)pItem)->id : 
										pItem->id,
									NULL);

								if (wnd != NULL)
								{
									// Set the Help ID
									if (bDialogEx && ((ATL::_DialogSplitHelper::DLGITEMTEMPLATEEX*)pItem)->helpID != 0)
										wnd.SetWindowContextHelpId(((ATL::_DialogSplitHelper::DLGITEMTEMPLATEEX*)pItem)->helpID);
									// Try to create the ActiveX control.
									hr = wnd.CreateControlLic(pszClassName, spStream, NULL, bstrLicKey);
									if (FAILED(hr))
										break;
									// Set the correct tab position.
									if (nItem == 0)
										hWndPrev = HWND_TOP;
									wnd.SetWindowPos(hWndPrev, 0,0,0,0,SWP_NOACTIVATE | SWP_NOMOVE | SWP_NOSIZE);
									hWndPrev = wnd;
								}
								else
								{
									hr = ATL::AtlHresultFromLastError();
								}
							}
						}
						else
						{
							if (nItem != 0)
								hWndPrev = ::GetWindow(hWndPrev, GW_HWNDNEXT);
						}
						pItem = ATL::_DialogSplitHelper::FindNextDlgItem(pItem, bDialogEx);
					}
				}
				else
					hr = ATL::AtlHresultFromLastError();
			}
			else
				hr = ATL::AtlHresultFromLastError();
		}
		return hr;
	}

// Event handling support
	HRESULT AdviseSinkMap(bool bAdvise)
	{
		if(!bAdvise && (this->m_hWnd == NULL))
		{
			// window is gone, controls are already unadvised
			ATLTRACE2(atlTraceUI, 0, _T("CAxPropertyPageImpl::AdviseSinkMap called after the window was destroyed\n"));
			return S_OK;
		}
		HRESULT hRet = E_NOTIMPL;
		__if_exists(T::_GetSinkMapFinder)
		{
			T* pT = static_cast<T*>(this);
			hRet = AtlAdviseSinkMap(pT, bAdvise);
		}
		return hRet;
	}

// Message map and handlers
	typedef CPropertyPageImpl< T, TBase>   _baseClass;
	BEGIN_MSG_MAP(CAxPropertyPageImpl)
		MESSAGE_HANDLER(WM_INITDIALOG, OnInitDialog)
		MESSAGE_HANDLER(WM_DESTROY, OnDestroy)
		CHAIN_MSG_MAP(_baseClass)
	END_MSG_MAP()

	LRESULT OnInitDialog(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& bHandled)
	{
		// initialize controls in dialog with DLGINIT resource section
		this->ExecuteDlgInit(static_cast<T*>(this)->IDD);
		AdviseSinkMap(true);
		bHandled = FALSE;
		return 1;
	}

	LRESULT OnDestroy(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& bHandled)
	{
		AdviseSinkMap(false);
		bHandled = FALSE;
		return 1;
	}
};

// for non-customized pages
template <WORD t_wDlgTemplateID>
class CAxPropertyPage : public CAxPropertyPageImpl<CAxPropertyPage<t_wDlgTemplateID> >
{
public:
	enum { IDD = t_wDlgTemplateID };

	CAxPropertyPage(ATL::_U_STRINGorID title = (LPCTSTR)NULL) : CAxPropertyPageImpl<CAxPropertyPage>(title)
	{ }

	BEGIN_MSG_MAP(CAxPropertyPage)
		CHAIN_MSG_MAP(CAxPropertyPageImpl<CAxPropertyPage<t_wDlgTemplateID> >)
	END_MSG_MAP()
};

#endif // _ATL_NO_HOSTING


///////////////////////////////////////////////////////////////////////////////
// Wizard97 Support

// Sample wizard dialog resources:
//
// IDD_WIZ97_INTERIOR_BLANK DIALOG  0, 0, 317, 143
// STYLE DS_SETFONT | WS_CHILD | WS_DISABLED | WS_CAPTION
// CAPTION "Wizard97 Property Page - Interior"
// FONT 8, "MS Shell Dlg"
// BEGIN
// END
//
// IDD_WIZ97_EXTERIOR_BLANK DIALOGEX 0, 0, 317, 193
// STYLE DS_SETFONT | DS_FIXEDSYS | WS_CHILD | WS_DISABLED | WS_CAPTION
// CAPTION "Wizard97 Property Page - Welcome/Complete"
// FONT 8, "MS Shell Dlg", 0, 0, 0x0
// BEGIN
//    LTEXT           "Welcome to the X Wizard",IDC_WIZ97_EXTERIOR_TITLE,115,8,
//                    195,24
//    LTEXT           "Wizard Explanation\r\n(The height of the static text should be in multiples of 8 dlus)",
//                    IDC_STATIC,115,40,195,16
//    LTEXT           "h",IDC_WIZ97_BULLET1,118,64,8,8
//    LTEXT           "List Item 1 (the h is turned into a bullet)",IDC_STATIC,
//                    127,63,122,8
//    LTEXT           "h",IDC_WIZ97_BULLET2,118,79,8,8
//    LTEXT           "List Item 2. Keep 7 dlus between paragraphs",IDC_STATIC,
//                    127,78,33,8
//    CONTROL         "&Do not show this Welcome page again",
//                    IDC_WIZ97_WELCOME_NOTAGAIN,"Button",BS_AUTOCHECKBOX | 
//                    WS_TABSTOP,115,169,138,10
// END
//
// GUIDELINES DESIGNINFO 
// BEGIN
//    IDD_WIZ97_INTERIOR_BLANK, DIALOG
//    BEGIN
//        LEFTMARGIN, 7
//        RIGHTMARGIN, 310
//        VERTGUIDE, 21
//        VERTGUIDE, 31
//        VERTGUIDE, 286
//        VERTGUIDE, 296
//        TOPMARGIN, 7
//        BOTTOMMARGIN, 136
//        HORZGUIDE, 8
//    END
//
//    IDD_WIZ97_EXTERIOR_BLANK, DIALOG
//    BEGIN
//        RIGHTMARGIN, 310
//        VERTGUIDE, 115
//        VERTGUIDE, 118
//        VERTGUIDE, 127
//        TOPMARGIN, 7
//        BOTTOMMARGIN, 186
//        HORZGUIDE, 8
//        HORZGUIDE, 32
//        HORZGUIDE, 40
//        HORZGUIDE, 169
//    END
// END

///////////////////////////////////////////////////////////////////////////////
// CWizard97SheetWindow - client side for a Wizard 97 style wizard sheet

class CWizard97SheetWindow : public CPropertySheetWindow
{
public:
// Constructors
	CWizard97SheetWindow(HWND hWnd = NULL) : CPropertySheetWindow(hWnd)
	{ }

	CWizard97SheetWindow& operator =(HWND hWnd)
	{
		m_hWnd = hWnd;
		return *this;
	}

// Operations
	HFONT GetExteriorPageTitleFont(void)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		return (HFONT)::SendMessage(m_hWnd, GetMessage_GetExteriorPageTitleFont(), 0, 0L);
	}

	HFONT GetBulletFont(void)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		return (HFONT)::SendMessage(m_hWnd, GetMessage_GetBulletFont(), 0, 0L);
	}

// Helpers
	static UINT GetMessage_GetExteriorPageTitleFont()
	{
		static UINT uGetExteriorPageTitleFont = 0;
		if(uGetExteriorPageTitleFont == 0)
		{
			CStaticDataInitCriticalSectionLock lock;
			if(FAILED(lock.Lock()))
			{
				ATLTRACE2(atlTraceUI, 0, _T("ERROR : Unable to lock critical section in CWizard97SheetWindow::GetMessage_GetExteriorPageTitleFont().\n"));
				ATLASSERT(FALSE);
				return 0;
			}

			if(uGetExteriorPageTitleFont == 0)
				uGetExteriorPageTitleFont = ::RegisterWindowMessage(_T("GetExteriorPageTitleFont_531AF056-B8BE-4c4c-B786-AC608DF0DF12"));

			lock.Unlock();
		}
		ATLASSERT(uGetExteriorPageTitleFont != 0);
		return uGetExteriorPageTitleFont;
	}

	static UINT GetMessage_GetBulletFont()
	{
		static UINT uGetBulletFont = 0;
		if(uGetBulletFont == 0)
		{
			CStaticDataInitCriticalSectionLock lock;
			if(FAILED(lock.Lock()))
			{
				ATLTRACE2(atlTraceUI, 0, _T("ERROR : Unable to lock critical section in CWizard97SheetWindow::GetMessage_GetBulletFont().\n"));
				ATLASSERT(FALSE);
				return 0;
			}

			if(uGetBulletFont == 0)
				uGetBulletFont = ::RegisterWindowMessage(_T("GetBulletFont_AD347D08-8F65-45ef-982E-6352E8218AD5"));

			lock.Unlock();
		}
		ATLASSERT(uGetBulletFont != 0);
		return uGetBulletFont;
	}

// Implementation - override to prevent usage
	HWND Create(LPCTSTR, HWND, ATL::_U_RECT = NULL, LPCTSTR = NULL, DWORD = 0, DWORD = 0, ATL::_U_MENUorID = 0U, LPVOID = NULL)
	{
		ATLASSERT(FALSE);
		return NULL;
	}
};


///////////////////////////////////////////////////////////////////////////////
// CWizard97SheetImpl - implements a Wizard 97 style wizard sheet

template <class T, class TBase = CWizard97SheetWindow>
class ATL_NO_VTABLE CWizard97SheetImpl : public CPropertySheetImpl< T, TBase >
{
protected:
// Typedefs
	typedef CWizard97SheetImpl< T, TBase > thisClass;
	typedef CPropertySheetImpl< T, TBase > baseClass;

// Member variables
	CFont m_fontExteriorPageTitle;   // Welcome and Completion page title font
	CFont m_fontBullet;              // Bullet font (used on static text 'h' to produce a small bullet)
	bool m_bReceivedFirstSizeMessage;   

public:
	CWizard97SheetImpl(ATL::_U_STRINGorID title, ATL::_U_STRINGorID headerBitmap, ATL::_U_STRINGorID watermarkBitmap, UINT uStartPage = 0, HWND hWndParent = NULL) :
			baseClass(title, uStartPage, hWndParent),
			m_bReceivedFirstSizeMessage(false)
	{
		this->m_psh.dwFlags &= ~(PSH_NOCONTEXTHELP);
		this->m_psh.dwFlags &= ~(PSH_WIZARD | PSH_WIZARD_LITE);

		this->m_psh.dwFlags |= (PSH_HASHELP | PSH_WIZARDCONTEXTHELP);
		this->m_psh.dwFlags |= PSH_WIZARD97;

		baseClass::SetHeader(headerBitmap.m_lpstr);
		baseClass::SetWatermark(watermarkBitmap.m_lpstr);
	}

// Overrides from base class
	void OnSheetInitialized()
	{
		T* pT = static_cast<T*>(this);
		pT->_InitializeFonts();

		// We'd like to center the wizard here, but its too early.
		// Instead, we'll do CenterWindow upon our first WM_SIZE message
	}

// Initialization
	void _InitializeFonts()
	{
		// Setup the Title and Bullet Font
		// (Property pages can send the "get external page title font" and "get bullet font" messages)
		// The derived class needs to do the actual SetFont for the dialog items)

		CFontHandle fontThisDialog = this->GetFont();
		CClientDC dcScreen(NULL);

		LOGFONT titleLogFont = {};
		LOGFONT bulletLogFont = {};
		fontThisDialog.GetLogFont(&titleLogFont);
		fontThisDialog.GetLogFont(&bulletLogFont);

		// The Wizard 97 Spec recommends to do the Title Font
		// as Verdana Bold, 12pt.
		titleLogFont.lfCharSet = DEFAULT_CHARSET;
		titleLogFont.lfWeight = FW_BOLD;
		ATL::Checked::tcscpy_s(titleLogFont.lfFaceName, _countof(titleLogFont.lfFaceName), _T("Verdana Bold"));
		INT titleFontPointSize = 12;
		titleLogFont.lfHeight = -::MulDiv(titleFontPointSize, dcScreen.GetDeviceCaps(LOGPIXELSY), 72);
		m_fontExteriorPageTitle.CreateFontIndirect(&titleLogFont);

		// The Wizard 97 Spec recommends to do Bullets by having
		// static text of "h" in the Marlett font.
		bulletLogFont.lfCharSet = DEFAULT_CHARSET;
		bulletLogFont.lfWeight = FW_NORMAL;
		ATL::Checked::tcscpy_s(bulletLogFont.lfFaceName, _countof(bulletLogFont.lfFaceName), _T("Marlett"));
		INT bulletFontSize = 8;
		bulletLogFont.lfHeight = -::MulDiv(bulletFontSize, dcScreen.GetDeviceCaps(LOGPIXELSY), 72);
		m_fontBullet.CreateFontIndirect(&bulletLogFont);
	}

// Message Handling
	BEGIN_MSG_MAP(thisClass)
		MESSAGE_HANDLER(CWizard97SheetWindow::GetMessage_GetExteriorPageTitleFont(), OnGetExteriorPageTitleFont)
		MESSAGE_HANDLER(CWizard97SheetWindow::GetMessage_GetBulletFont(), OnGetBulletFont)
		MESSAGE_HANDLER(WM_SIZE, OnSize)
		CHAIN_MSG_MAP(baseClass)
	END_MSG_MAP()

	LRESULT OnGetExteriorPageTitleFont(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		return (LRESULT)(HFONT)m_fontExteriorPageTitle;
	}

	LRESULT OnGetBulletFont(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		return (LRESULT)(HFONT)m_fontBullet;
	}

	LRESULT OnSize(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& bHandled)
	{
		if(!m_bReceivedFirstSizeMessage)
		{
			m_bReceivedFirstSizeMessage = true;
			this->CenterWindow();
		}

		bHandled = FALSE;
		return 0;
	}
};

// for non-customized sheets
class CWizard97Sheet : public CWizard97SheetImpl<CWizard97Sheet>
{
protected:
// Typedefs
	typedef CWizard97Sheet thisClass;
	typedef CWizard97SheetImpl<CWizard97Sheet> baseClass;

public:
	CWizard97Sheet(ATL::_U_STRINGorID title, ATL::_U_STRINGorID headerBitmap, ATL::_U_STRINGorID watermarkBitmap, UINT uStartPage = 0, HWND hWndParent = NULL) :
		baseClass(title, headerBitmap, watermarkBitmap, uStartPage, hWndParent)
	{ }

	BEGIN_MSG_MAP(thisClass)
		CHAIN_MSG_MAP(baseClass)
	END_MSG_MAP()
};


///////////////////////////////////////////////////////////////////////////////
// CWizard97PageWindow - client side for a Wizard 97 style wizard page

#define WIZARD97_EXTERIOR_CXDLG 317
#define WIZARD97_EXTERIOR_CYDLG 193

#define WIZARD97_INTERIOR_CXDLG 317
#define WIZARD97_INTERIOR_CYDLG 143

class CWizard97PageWindow : public CPropertyPageWindow
{
public:
// Constructors
	CWizard97PageWindow(HWND hWnd = NULL) : CPropertyPageWindow(hWnd)
	{ }

	CWizard97PageWindow& operator =(HWND hWnd)
	{
		m_hWnd = hWnd;
		return *this;
	}

// Attributes
	CWizard97SheetWindow GetPropertySheet() const
	{
		ATLASSERT(::IsWindow(m_hWnd));
		return CWizard97SheetWindow(GetParent());
	}

// Operations
	HFONT GetExteriorPageTitleFont(void)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		return GetPropertySheet().GetExteriorPageTitleFont();
	}

	HFONT GetBulletFont(void)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		return GetPropertySheet().GetBulletFont();
	}

// Implementation - overrides to prevent usage
	HWND Create(LPCTSTR, HWND, ATL::_U_RECT = NULL, LPCTSTR = NULL, DWORD = 0, DWORD = 0, ATL::_U_MENUorID = 0U, LPVOID = NULL)
	{
		ATLASSERT(FALSE);
		return NULL;
	}

};


///////////////////////////////////////////////////////////////////////////////
// CWizard97PageImpl - implements a Wizard 97 style wizard page

template <class T, class TBase = CWizard97PageWindow>
class ATL_NO_VTABLE CWizard97PageImpl : public CPropertyPageImpl< T, TBase >
{
protected:
// Typedefs
	typedef CWizard97PageImpl< T, TBase > thisClass;
	typedef CPropertyPageImpl< T, TBase > baseClass;

public:
	CWizard97PageImpl(ATL::_U_STRINGorID title = (LPCTSTR)NULL) : baseClass(title)
	{ }

// Message Handling
	BEGIN_MSG_MAP(thisClass)
		CHAIN_MSG_MAP(baseClass)
	END_MSG_MAP()
};


///////////////////////////////////////////////////////////////////////////////
// CWizard97ExteriorPageImpl - implements a Wizard 97 style exterior wizard page

template <class T, class TBase = CWizard97PageWindow>
class ATL_NO_VTABLE CWizard97ExteriorPageImpl : public CPropertyPageImpl< T, TBase >
{
protected:
// Typedefs
	typedef CWizard97ExteriorPageImpl< T, TBase > thisClass;
	typedef CPropertyPageImpl< T, TBase > baseClass;

public:
// Constructors
	CWizard97ExteriorPageImpl(ATL::_U_STRINGorID title = (LPCTSTR)NULL) : baseClass(title)
	{
		this->m_psp.dwFlags |= PSP_HASHELP;
		this->m_psp.dwFlags |= PSP_HIDEHEADER;
	}

// Message Handling
	BEGIN_MSG_MAP(thisClass)
		CHAIN_MSG_MAP(baseClass)
	END_MSG_MAP()
};


///////////////////////////////////////////////////////////////////////////////
// CWizard97InteriorPageImpl - implements a Wizard 97 style interior wizard page

template <class T, class TBase = CWizard97PageWindow>
class ATL_NO_VTABLE CWizard97InteriorPageImpl : public CPropertyPageImpl< T, TBase >
{
protected:
// Typedefs
	typedef CWizard97InteriorPageImpl< T, TBase > thisClass;
	typedef CPropertyPageImpl< T, TBase > baseClass;

public:
// Constructors
	CWizard97InteriorPageImpl(ATL::_U_STRINGorID title = (LPCTSTR)NULL) : baseClass(title)
	{
		this->m_psp.dwFlags |= PSP_HASHELP;
		this->m_psp.dwFlags &= ~PSP_HIDEHEADER;
		this->m_psp.dwFlags |= PSP_USEHEADERTITLE | PSP_USEHEADERSUBTITLE;

		// Be sure to have the derived class define this in the constructor.
		// We'll default it to something obvious in case its forgotten.
		baseClass::SetHeaderTitle(_T("Call SetHeaderTitle in Derived Class"));
		baseClass::SetHeaderSubTitle(_T("Call SetHeaderSubTitle in the constructor of the Derived Class."));
	}

// Message Handling
	BEGIN_MSG_MAP(thisClass)
		CHAIN_MSG_MAP(baseClass)
	END_MSG_MAP()
};


///////////////////////////////////////////////////////////////////////////////
// Aero Wizard support

#if (_WIN32_WINNT >= 0x0600)

///////////////////////////////////////////////////////////////////////////////
// CAeroWizardFrameWindow - client side for an Aero Wizard frame window

class CAeroWizardFrameWindow : public CPropertySheetWindow
{
public:
// Constructors
	CAeroWizardFrameWindow(HWND hWnd = NULL) : CPropertySheetWindow(hWnd)
	{ }

	CAeroWizardFrameWindow& operator =(HWND hWnd)
	{
		m_hWnd = hWnd;
		return *this;
	}

// Operations - new, Aero Wizard only
	void SetNextText(LPCWSTR lpszText)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		::SendMessage(m_hWnd, PSM_SETNEXTTEXT, 0, (LPARAM)lpszText);
	}

	void ShowWizardButtons(DWORD dwButtons, DWORD dwStates)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		::PostMessage(m_hWnd, PSM_SHOWWIZBUTTONS, (WPARAM)dwStates, (LPARAM)dwButtons);
	}

	void EnableWizardButtons(DWORD dwButtons, DWORD dwStates)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		::PostMessage(m_hWnd, PSM_ENABLEWIZBUTTONS, (WPARAM)dwStates, (LPARAM)dwButtons);
	}

	void SetButtonText(DWORD dwButton, LPCWSTR lpszText)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		::SendMessage(m_hWnd, PSM_SETBUTTONTEXT, (WPARAM)dwButton, (LPARAM)lpszText);
	}
};


///////////////////////////////////////////////////////////////////////////////
// CAeroWizardFrameImpl - implements an Aero Wizard frame

template <class T, class TBase = CAeroWizardFrameWindow>
class ATL_NO_VTABLE CAeroWizardFrameImpl : public CPropertySheetImpl<T, TBase >
{
public:
// Constructor
	CAeroWizardFrameImpl(ATL::_U_STRINGorID title = (LPCTSTR)NULL, UINT uStartPage = 0, HWND hWndParent = NULL) :
		CPropertySheetImpl<T, TBase >(title, uStartPage, hWndParent)
	{
		this->m_psh.dwFlags |= PSH_WIZARD | PSH_AEROWIZARD;
	}

// Operations
	void EnableResizing()
	{
		ATLASSERT(this->m_hWnd == NULL);   // can't do this after it's created
		this->m_psh.dwFlags |= PSH_RESIZABLE;
	}

	void UseHeaderBitmap()
	{
		ATLASSERT(this->m_hWnd == NULL);   // can't do this after it's created
		this->m_psh.dwFlags |= PSH_HEADERBITMAP;
	}

	void SetNoMargin()
	{
		ATLASSERT(this->m_hWnd == NULL);   // can't do this after it's created
		this->m_psh.dwFlags |= PSH_NOMARGIN;
	}

// Override to prevent use
	HWND Create(HWND /*hWndParent*/ = NULL)
	{
		ATLASSERT(FALSE);   // not supported for Aero Wizard
		return NULL;
	}
};


///////////////////////////////////////////////////////////////////////////////
// CAeroWizardFrame - for non-customized frames

class CAeroWizardFrame : public CAeroWizardFrameImpl<CAeroWizardFrame>
{
public:
	CAeroWizardFrame(ATL::_U_STRINGorID title = (LPCTSTR)NULL, UINT uStartPage = 0, HWND hWndParent = NULL)
		: CAeroWizardFrameImpl<CAeroWizardFrame>(title, uStartPage, hWndParent)
	{ }

	BEGIN_MSG_MAP(CAeroWizardFrame)
		MESSAGE_HANDLER(WM_COMMAND, CAeroWizardFrameImpl<CAeroWizardFrame>::OnCommand)
	END_MSG_MAP()
};


///////////////////////////////////////////////////////////////////////////////
// CAeroWizardPageWindow - client side for an Aero Wizard page

class CAeroWizardPageWindow : public CPropertyPageWindow
{
public:
// Constructors
	CAeroWizardPageWindow(HWND hWnd = NULL) : CPropertyPageWindow(hWnd)
	{ }

	CAeroWizardPageWindow& operator =(HWND hWnd)
	{
		m_hWnd = hWnd;
		return *this;
	}

// Attributes
	CAeroWizardFrameWindow GetAeroWizardFrame() const
	{
		ATLASSERT(::IsWindow(m_hWnd));
		// This is not really top-level frame window, but it processes all frame messages
		return CAeroWizardFrameWindow(GetParent());
	}

// Operations - new, Aero Wizard only
	void SetNextText(LPCWSTR lpszText)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		ATLASSERT(GetParent() != NULL);
		GetAeroWizardFrame().SetNextText(lpszText);
	}

	void ShowWizardButtons(DWORD dwButtons, DWORD dwStates)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		ATLASSERT(GetParent() != NULL);
		GetAeroWizardFrame().ShowWizardButtons(dwButtons, dwStates);
	}

	void EnableWizardButtons(DWORD dwButtons, DWORD dwStates)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		ATLASSERT(GetParent() != NULL);
		GetAeroWizardFrame().EnableWizardButtons(dwButtons, dwStates);
	}

	void SetButtonText(DWORD dwButton, LPCWSTR lpszText)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		ATLASSERT(GetParent() != NULL);
		GetAeroWizardFrame().SetButtonText(dwButton, lpszText);
	}
};


///////////////////////////////////////////////////////////////////////////////
// CAeroWizardPageImpl - implements an Aero Wizard page

template <class T, class TBase = CAeroWizardPageWindow>
class ATL_NO_VTABLE CAeroWizardPageImpl : public CPropertyPageImpl<T, TBase >
{
public:
	CAeroWizardPageImpl(ATL::_U_STRINGorID title = (LPCTSTR)NULL) : CPropertyPageImpl<T, TBase >(title)
	{ }
};


///////////////////////////////////////////////////////////////////////////////
// CAeroWizardPage - for non-customized pages

template <WORD t_wDlgTemplateID>
class CAeroWizardPage : public CAeroWizardPageImpl<CAeroWizardPage<t_wDlgTemplateID> >
{
public:
	enum { IDD = t_wDlgTemplateID };

	CAeroWizardPage(ATL::_U_STRINGorID title = (LPCTSTR)NULL) : CAeroWizardPageImpl<CAeroWizardPage>(title)
	{ }

	DECLARE_EMPTY_MSG_MAP()
};


#ifndef _ATL_NO_HOSTING

// Note: You must #include <atlhost.h> to use these classes

///////////////////////////////////////////////////////////////////////////////
// CAeroWizardAxPageImpl - Aero Wizard page that hosts ActiveX controls

template <class T, class TBase = CAeroWizardPageWindow>
class ATL_NO_VTABLE CAeroWizardAxPageImpl : public CAxPropertyPageImpl< T, TBase >
{
public:
	CAeroWizardAxPageImpl(ATL::_U_STRINGorID title = (LPCTSTR)NULL) : CAxPropertyPageImpl< T, TBase >(title)
	{ }
};


///////////////////////////////////////////////////////////////////////////////
// CAeroWizardAxPage - for non-customized pages

template <WORD t_wDlgTemplateID>
class CAeroWizardAxPage : public CAeroWizardAxPageImpl<CAeroWizardAxPage<t_wDlgTemplateID> >
{
public:
	enum { IDD = t_wDlgTemplateID };

	CAeroWizardAxPage(ATL::_U_STRINGorID title = (LPCTSTR)NULL) : CAeroWizardAxPageImpl<CAeroWizardAxPage>(title)
	{ }

	BEGIN_MSG_MAP(CAeroWizardAxPage)
		CHAIN_MSG_MAP(CAeroWizardAxPageImpl<CAeroWizardAxPage<t_wDlgTemplateID> >)
	END_MSG_MAP()
};

#endif // _ATL_NO_HOSTING

#endif // (_WIN32_WINNT >= 0x0600)


///////////////////////////////////////////////////////////////////////////////
// TaskDialog support

#if (_WIN32_WINNT >= 0x0600) || defined(_WTL_TASKDIALOG)

///////////////////////////////////////////////////////////////////////////////
// AtlTaskDialog - support for TaskDialog() function

inline int AtlTaskDialog(HWND hWndParent, 
                         ATL::_U_STRINGorID WindowTitle, ATL::_U_STRINGorID MainInstructionText, ATL::_U_STRINGorID ContentText, 
                         TASKDIALOG_COMMON_BUTTON_FLAGS dwCommonButtons = 0U, ATL::_U_STRINGorID Icon = (LPCTSTR)NULL)
{
	int nRet = -1;

#ifdef _WTL_TASKDIALOG_DIRECT
	USES_CONVERSION;
	HRESULT hRet = ::TaskDialog(hWndParent, ModuleHelper::GetResourceInstance(), 
		IS_INTRESOURCE(WindowTitle.m_lpstr) ? (LPCWSTR) WindowTitle.m_lpstr : T2CW(WindowTitle.m_lpstr), 
		IS_INTRESOURCE(MainInstructionText.m_lpstr) ? (LPCWSTR) MainInstructionText.m_lpstr : T2CW(MainInstructionText.m_lpstr), 
		IS_INTRESOURCE(ContentText.m_lpstr) ?  (LPCWSTR) ContentText.m_lpstr : T2CW(ContentText.m_lpstr), 
		dwCommonButtons, 
		IS_INTRESOURCE(Icon.m_lpstr) ? (LPCWSTR) Icon.m_lpstr : T2CW(Icon.m_lpstr),
		&nRet);
	ATLVERIFY(SUCCEEDED(hRet));
#else
	// This allows apps to run on older versions of Windows
	typedef HRESULT (STDAPICALLTYPE *PFN_TaskDialog)(HWND hwndParent, HINSTANCE hInstance, PCWSTR pszWindowTitle, PCWSTR pszMainInstruction, PCWSTR pszContent, TASKDIALOG_COMMON_BUTTON_FLAGS dwCommonButtons, PCWSTR pszIcon, int* pnButton);

	HMODULE m_hCommCtrlDLL = ::LoadLibrary(_T("comctl32.dll"));
	if(m_hCommCtrlDLL != NULL)
	{
		PFN_TaskDialog pfnTaskDialog = (PFN_TaskDialog)::GetProcAddress(m_hCommCtrlDLL, "TaskDialog");
		if(pfnTaskDialog != NULL)
		{
			USES_CONVERSION;
			HRESULT hRet = pfnTaskDialog(hWndParent, ModuleHelper::GetResourceInstance(), 
				IS_INTRESOURCE(WindowTitle.m_lpstr) ? (LPCWSTR) WindowTitle.m_lpstr : T2CW(WindowTitle.m_lpstr), 
				IS_INTRESOURCE(MainInstructionText.m_lpstr) ? (LPCWSTR) MainInstructionText.m_lpstr : T2CW(MainInstructionText.m_lpstr), 
				IS_INTRESOURCE(ContentText.m_lpstr) ?  (LPCWSTR) ContentText.m_lpstr : T2CW(ContentText.m_lpstr), 
				dwCommonButtons, 
				IS_INTRESOURCE(Icon.m_lpstr) ? (LPCWSTR) Icon.m_lpstr : T2CW(Icon.m_lpstr),
				&nRet);
			ATLVERIFY(SUCCEEDED(hRet));
		}

		::FreeLibrary(m_hCommCtrlDLL);
	}
#endif

	return nRet;
}


///////////////////////////////////////////////////////////////////////////////
// CTaskDialogConfig - TASKDIALOGCONFIG wrapper

class CTaskDialogConfig : public TASKDIALOGCONFIG
{
public:
// Constructor
	CTaskDialogConfig()
	{
		Init();
	}

	void Init()
	{
		memset(this, 0, sizeof(TASKDIALOGCONFIG));   // initialize structure to 0/NULL
		this->cbSize = sizeof(TASKDIALOGCONFIG);
		this->hInstance = ModuleHelper::GetResourceInstance();
	}

// Operations - setting values
	// common buttons
	void SetCommonButtons(TASKDIALOG_COMMON_BUTTON_FLAGS dwCommonButtonsArg)
	{
		this->dwCommonButtons = dwCommonButtonsArg;
	}

	// window title text
	void SetWindowTitle(UINT nID)
	{
		this->pszWindowTitle = MAKEINTRESOURCEW(nID);
	}

	void SetWindowTitle(LPCWSTR lpstrWindowTitle)
	{
		this->pszWindowTitle = lpstrWindowTitle;
	}

	// main icon
	void SetMainIcon(HICON hIcon)
	{
		this->dwFlags |= TDF_USE_HICON_MAIN;
		this->hMainIcon = hIcon;
	}

	void SetMainIcon(UINT nID)
	{
		this->dwFlags &= ~TDF_USE_HICON_MAIN;
		this->pszMainIcon = MAKEINTRESOURCEW(nID);
	}

	void SetMainIcon(LPCWSTR lpstrMainIcon)
	{
		this->dwFlags &= ~TDF_USE_HICON_MAIN;
		this->pszMainIcon = lpstrMainIcon;
	}

	// main instruction text
	void SetMainInstructionText(UINT nID)
	{
		this->pszMainInstruction = MAKEINTRESOURCEW(nID);
	}

	void SetMainInstructionText(LPCWSTR lpstrMainInstruction)
	{
		this->pszMainInstruction = lpstrMainInstruction;
	}

	// content text
	void SetContentText(UINT nID)
	{
		this->pszContent = MAKEINTRESOURCEW(nID);
	}

	void SetContentText(LPCWSTR lpstrContent)
	{
		this->pszContent = lpstrContent;
	}

	// buttons
	void SetButtons(const TASKDIALOG_BUTTON* pButtonsArg, UINT cButtonsArg, int nDefaultButtonArg = 0)
	{
		this->pButtons = pButtonsArg;
		this->cButtons = cButtonsArg;
		if(nDefaultButtonArg != 0)
			this->nDefaultButton = nDefaultButtonArg;
	}

	void SetDefaultButton(int nDefaultButtonArg)
	{
		this->nDefaultButton = nDefaultButtonArg;
	}

	// radio buttons
	void SetRadioButtons(const TASKDIALOG_BUTTON* pRadioButtonsArg, UINT cRadioButtonsArg, int nDefaultRadioButtonArg = 0)
	{
		this->pRadioButtons = pRadioButtonsArg;
		this->cRadioButtons = cRadioButtonsArg;
		if(nDefaultRadioButtonArg != 0)
			this->nDefaultRadioButton = nDefaultRadioButtonArg;
	}

	void SetDefaultRadioButton(int nDefaultRadioButtonArg)
	{
		this->nDefaultRadioButton = nDefaultRadioButtonArg;
	}

	// verification text
	void SetVerificationText(UINT nID)
	{
		this->pszVerificationText = MAKEINTRESOURCEW(nID);
	}

	void SetVerificationText(LPCWSTR lpstrVerificationText)
	{
		this->pszVerificationText = lpstrVerificationText;
	}

	// expanded information text
	void SetExpandedInformationText(UINT nID)
	{
		this->pszExpandedInformation = MAKEINTRESOURCEW(nID);
	}

	void SetExpandedInformationText(LPCWSTR lpstrExpandedInformation)
	{
		this->pszExpandedInformation = lpstrExpandedInformation;
	}

	// expanded control text
	void SetExpandedControlText(UINT nID)
	{
		this->pszExpandedControlText = MAKEINTRESOURCEW(nID);
	}

	void SetExpandedControlText(LPCWSTR lpstrExpandedControlText)
	{
		this->pszExpandedControlText = lpstrExpandedControlText;
	}

	// collapsed control text
	void SetCollapsedControlText(UINT nID)
	{
		this->pszCollapsedControlText = MAKEINTRESOURCEW(nID);
	}

	void SetCollapsedControlText(LPCWSTR lpstrCollapsedControlText)
	{
		this->pszCollapsedControlText = lpstrCollapsedControlText;
	}

	// footer icon
	void SetFooterIcon(HICON hIcon)
	{
		this->dwFlags |= TDF_USE_HICON_FOOTER;
		this->hFooterIcon = hIcon;
	}

	void SetFooterIcon(UINT nID)
	{
		this->dwFlags &= ~TDF_USE_HICON_FOOTER;
		this->pszFooterIcon = MAKEINTRESOURCEW(nID);
	}

	void SetFooterIcon(LPCWSTR lpstrFooterIcon)
	{
		this->dwFlags &= ~TDF_USE_HICON_FOOTER;
		this->pszFooterIcon = lpstrFooterIcon;
	}

	// footer text
	void SetFooterText(UINT nID)
	{
		this->pszFooter = MAKEINTRESOURCEW(nID);
	}

	void SetFooterText(LPCWSTR lpstrFooterText)
	{
		this->pszFooter = lpstrFooterText;
	}

	// width (in DLUs)
	void SetWidth(UINT cxWidthArg)
	{
		this->cxWidth = cxWidthArg;
	}

	// modify flags
	void ModifyFlags(DWORD dwRemove, DWORD dwAdd)
	{
		this->dwFlags = (this->dwFlags & ~dwRemove) | dwAdd;
	}
};


///////////////////////////////////////////////////////////////////////////////
// CTaskDialogImpl - implements a Task Dialog

template <class T>
class ATL_NO_VTABLE CTaskDialogImpl
{
public:
	CTaskDialogConfig m_tdc;
	HWND m_hWnd;   // used only in callback functions

// Constructor
	CTaskDialogImpl(HWND hWndParent = NULL) : m_hWnd(NULL)
	{
		m_tdc.hwndParent = hWndParent;
		m_tdc.pfCallback = T::TaskDialogCallback;
		m_tdc.lpCallbackData = (LONG_PTR)static_cast<T*>(this);
	}

// Operations
	HRESULT DoModal(HWND hWndParent = ::GetActiveWindow(), int* pnButton = NULL, int* pnRadioButton = NULL, BOOL* pfVerificationFlagChecked = NULL)
	{
		if(m_tdc.hwndParent == NULL)
			m_tdc.hwndParent = hWndParent;

#ifdef _WTL_TASKDIALOG_DIRECT
		return ::TaskDialogIndirect(&m_tdc, pnButton, pnRadioButton, pfVerificationFlagChecked);
#else

		// This allows apps to run on older versions of Windows
		typedef HRESULT (STDAPICALLTYPE *PFN_TaskDialogIndirect)(const TASKDIALOGCONFIG* pTaskConfig, int* pnButton, int* pnRadioButton, BOOL* pfVerificationFlagChecked);

		HRESULT hRet = E_UNEXPECTED;
		HMODULE m_hCommCtrlDLL = ::LoadLibrary(_T("comctl32.dll"));
		if(m_hCommCtrlDLL != NULL)
		{
			PFN_TaskDialogIndirect pfnTaskDialogIndirect = (PFN_TaskDialogIndirect)::GetProcAddress(m_hCommCtrlDLL, "TaskDialogIndirect");
			if(pfnTaskDialogIndirect != NULL)
				hRet = pfnTaskDialogIndirect(&m_tdc, pnButton, pnRadioButton, pfVerificationFlagChecked);

			::FreeLibrary(m_hCommCtrlDLL);
		}

		return hRet;
#endif
	}

// Operations - setting values of TASKDIALOGCONFIG
	// common buttons
	void SetCommonButtons(TASKDIALOG_COMMON_BUTTON_FLAGS dwCommonButtons)
	{	m_tdc.SetCommonButtons(dwCommonButtons); }
	// window title text
	void SetWindowTitle(UINT nID)
	{	m_tdc.SetWindowTitle(nID); }
	void SetWindowTitle(LPCWSTR lpstrWindowTitle)
	{	m_tdc.SetWindowTitle(lpstrWindowTitle); }
	// main icon
	void SetMainIcon(HICON hIcon)
	{	m_tdc.SetMainIcon(hIcon); }
	void SetMainIcon(UINT nID)
	{	m_tdc.SetMainIcon(nID); }
	void SetMainIcon(LPCWSTR lpstrMainIcon)
	{	m_tdc.SetMainIcon(lpstrMainIcon); }
	// main instruction text
	void SetMainInstructionText(UINT nID)
	{	m_tdc.SetMainInstructionText(nID); }
	void SetMainInstructionText(LPCWSTR lpstrMainInstruction)
	{	m_tdc.SetMainInstructionText(lpstrMainInstruction); }
	// content text
	void SetContentText(UINT nID)
	{	m_tdc.SetContentText(nID); }
	void SetContentText(LPCWSTR lpstrContent)
	{	m_tdc.SetContentText(lpstrContent); }
	// buttons
	void SetButtons(const TASKDIALOG_BUTTON* pButtons, UINT cButtons, int nDefaultButton = 0)
	{	m_tdc.SetButtons(pButtons, cButtons, nDefaultButton); }
	void SetDefaultButton(int nDefaultButton)
	{	m_tdc.SetDefaultButton(nDefaultButton); }
	// radio buttons
	void SetRadioButtons(const TASKDIALOG_BUTTON* pRadioButtons, UINT cRadioButtons, int nDefaultRadioButton = 0)
	{	m_tdc.SetRadioButtons(pRadioButtons, cRadioButtons, nDefaultRadioButton); }
	void SetDefaultRadioButton(int nDefaultRadioButton)
	{	m_tdc.SetDefaultRadioButton(nDefaultRadioButton); }
	// verification text
	void SetVerificationText(UINT nID)
	{	m_tdc.SetVerificationText(nID); }
	void SetVerificationText(LPCWSTR lpstrVerificationText)
	{	m_tdc.SetVerificationText(lpstrVerificationText); }
	// expanded information text
	void SetExpandedInformationText(UINT nID)
	{	m_tdc.SetExpandedInformationText(nID); }
	void SetExpandedInformationText(LPCWSTR lpstrExpandedInformation)
	{	m_tdc.SetExpandedInformationText(lpstrExpandedInformation); }
	// expanded control text
	void SetExpandedControlText(UINT nID)
	{	m_tdc.SetExpandedControlText(nID); }
	void SetExpandedControlText(LPCWSTR lpstrExpandedControlText)
	{	m_tdc.SetExpandedControlText(lpstrExpandedControlText); }
	// collapsed control text
	void SetCollapsedControlText(UINT nID)
	{	m_tdc.SetCollapsedControlText(nID); }
	void SetCollapsedControlText(LPCWSTR lpstrCollapsedControlText)
	{	m_tdc.SetCollapsedControlText(lpstrCollapsedControlText); }
	// footer icon
	void SetFooterIcon(HICON hIcon)
	{	m_tdc.SetFooterIcon(hIcon); }
	void SetFooterIcon(UINT nID)
	{	m_tdc.SetFooterIcon(nID); }
	void SetFooterIcon(LPCWSTR lpstrFooterIcon)
	{	m_tdc.SetFooterIcon(lpstrFooterIcon); }
	// footer text
	void SetFooterText(UINT nID)
	{	m_tdc.SetFooterText(nID); }
	void SetFooterText(LPCWSTR lpstrFooterText)
	{	m_tdc.SetFooterText(lpstrFooterText); }
	// width (in DLUs)
	void SetWidth(UINT cxWidth)
	{	m_tdc.SetWidth(cxWidth); }
	// modify flags
	void ModifyFlags(DWORD dwRemove, DWORD dwAdd)
	{	m_tdc.ModifyFlags(dwRemove, dwAdd); }

// Implementation
	static HRESULT CALLBACK TaskDialogCallback(HWND hWnd, UINT uMsg, WPARAM wParam, LPARAM lParam, LONG_PTR lpRefData)
	{
		T* pT = (T*)lpRefData;
		ATLASSERT((pT->m_hWnd == NULL) || (pT->m_hWnd == hWnd));

		BOOL bRet = FALSE;
		switch(uMsg)
		{
		case TDN_DIALOG_CONSTRUCTED:
			pT->m_hWnd = hWnd;
			pT->OnDialogConstructed();
			break;
		case TDN_CREATED:
			pT->OnCreated();
			break;
		case TDN_BUTTON_CLICKED:
			bRet = pT->OnButtonClicked((int)wParam);
			break;
		case TDN_RADIO_BUTTON_CLICKED:
			pT->OnRadioButtonClicked((int)wParam);
			break;
		case TDN_HYPERLINK_CLICKED:
			pT->OnHyperlinkClicked((LPCWSTR)lParam);
			break;
		case TDN_EXPANDO_BUTTON_CLICKED:
			pT->OnExpandoButtonClicked((wParam != 0));
			break;
		case TDN_VERIFICATION_CLICKED:
			pT->OnVerificationClicked((wParam != 0));
			break;
		case TDN_HELP:
			pT->OnHelp();
			break;
		case TDN_TIMER:
			bRet = pT->OnTimer((DWORD)wParam);
			break;
		case TDN_NAVIGATED:
			pT->OnNavigated();
			break;
		case TDN_DESTROYED:
			pT->OnDestroyed();
			pT->m_hWnd = NULL;
			break;
		default:
			ATLTRACE2(atlTraceUI, 0, _T("Unknown notification received in CTaskDialogImpl::TaskDialogCallback\n"));
			break;
		}

		return (bRet != FALSE) ? S_OK : S_FALSE;
	}

// Overrideables - notification handlers
	void OnDialogConstructed()
	{
	}

	void OnCreated()
	{
	}

	BOOL OnButtonClicked(int /*nButton*/)
	{
		return FALSE;   // don't prevent dialog to close
	}

	void OnRadioButtonClicked(int /*nRadioButton*/)
	{
	}

	void OnHyperlinkClicked(LPCWSTR /*pszHREF*/)
	{
	}

	void OnExpandoButtonClicked(bool /*bExpanded*/)
	{
	}

	void OnVerificationClicked(bool /*bChecked*/)
	{
	}

	void OnHelp()
	{
	}

	BOOL OnTimer(DWORD /*dwTickCount*/)
	{
		return FALSE;   // don't reset counter
	}

	void OnNavigated()
	{
	}

	void OnDestroyed()
	{
	}

// Commands - valid to call only from handlers
	void NavigatePage(TASKDIALOGCONFIG& tdc)
	{
		ATLASSERT(m_hWnd != NULL);

		tdc.cbSize = sizeof(TASKDIALOGCONFIG);
		if(tdc.hwndParent == NULL)
			tdc.hwndParent = m_tdc.hwndParent;
		tdc.pfCallback = m_tdc.pfCallback;
		tdc.lpCallbackData = m_tdc.lpCallbackData;
		(TASKDIALOGCONFIG)m_tdc = tdc;

		::SendMessage(m_hWnd, TDM_NAVIGATE_PAGE, 0, (LPARAM)&tdc);
	}

	// modify TASKDIALOGCONFIG values, then call this to update task dialog
	void NavigatePage()
	{
		ATLASSERT(m_hWnd != NULL);
		::SendMessage(m_hWnd, TDM_NAVIGATE_PAGE, 0, (LPARAM)&m_tdc);
	}

	void ClickButton(int nButton)
	{
		ATLASSERT(m_hWnd != NULL);
		::SendMessage(m_hWnd, TDM_CLICK_BUTTON, nButton, 0L);
	}

	void SetMarqueeProgressBar(BOOL bMarquee)
	{
		ATLASSERT(m_hWnd != NULL);
		::SendMessage(m_hWnd, TDM_SET_MARQUEE_PROGRESS_BAR, bMarquee, 0L);
	}

	BOOL SetProgressBarState(int nNewState)
	{
		ATLASSERT(m_hWnd != NULL);
		return (BOOL)::SendMessage(m_hWnd, TDM_SET_PROGRESS_BAR_STATE, nNewState, 0L);
	}

	DWORD SetProgressBarRange(int nMinRange, int nMaxRange)
	{
		ATLASSERT(m_hWnd != NULL);
		return (DWORD)::SendMessage(m_hWnd, TDM_SET_PROGRESS_BAR_RANGE, 0, MAKELPARAM(nMinRange, nMaxRange));
	}

	int SetProgressBarPos(int nNewPos)
	{
		ATLASSERT(m_hWnd != NULL);
		return (int)::SendMessage(m_hWnd, TDM_SET_PROGRESS_BAR_POS, nNewPos, 0L);
	}

	BOOL SetProgressBarMarquee(BOOL bMarquee, UINT uSpeed)
	{
		ATLASSERT(m_hWnd != NULL);
		return (BOOL)::SendMessage(m_hWnd, TDM_SET_PROGRESS_BAR_MARQUEE, bMarquee, uSpeed);
	}

	void SetElementText(TASKDIALOG_ELEMENTS element, LPCWSTR lpstrText)
	{
		ATLASSERT(m_hWnd != NULL);
		::SendMessage(m_hWnd, TDM_SET_ELEMENT_TEXT, element, (LPARAM)lpstrText);
	}

	void ClickRadioButton(int nRadioButton)
	{
		ATLASSERT(m_hWnd != NULL);
		::SendMessage(m_hWnd, TDM_CLICK_RADIO_BUTTON, nRadioButton, 0L);
	}

	void EnableButton(int nButton, BOOL bEnable)
	{
		ATLASSERT(m_hWnd != NULL);
		::SendMessage(m_hWnd, TDM_ENABLE_BUTTON, nButton, bEnable);
	}

	void EnableRadioButton(int nButton, BOOL bEnable)
	{
		ATLASSERT(m_hWnd != NULL);
		::SendMessage(m_hWnd, TDM_ENABLE_RADIO_BUTTON, nButton, bEnable);
	}

	void ClickVerification(BOOL bCheck, BOOL bFocus)
	{
		ATLASSERT(m_hWnd != NULL);
		::SendMessage(m_hWnd, TDM_CLICK_VERIFICATION, bCheck, bFocus);
	}

	void UpdateElementText(TASKDIALOG_ELEMENTS element, LPCWSTR lpstrText)
	{
		ATLASSERT(m_hWnd != NULL);
		::SendMessage(m_hWnd, TDM_UPDATE_ELEMENT_TEXT, element, (LPARAM)lpstrText);
	}

	void SetButtonElevationRequiredState(int nButton, BOOL bElevation)
	{
		ATLASSERT(m_hWnd != NULL);
		::SendMessage(m_hWnd, TDM_SET_BUTTON_ELEVATION_REQUIRED_STATE, nButton, bElevation);
	}

	void UpdateIcon(TASKDIALOG_ICON_ELEMENTS element, HICON hIcon)
	{
		ATLASSERT(m_hWnd != NULL);
#ifdef _DEBUG
		if(element == TDIE_ICON_MAIN)
			ATLASSERT((m_tdc.dwFlags & TDF_USE_HICON_MAIN) != 0);
		else if(element == TDIE_ICON_FOOTER)
			ATLASSERT((m_tdc.dwFlags & TDF_USE_HICON_FOOTER) != 0);
#endif // _DEBUG
		::SendMessage(m_hWnd, TDM_UPDATE_ICON, element, (LPARAM)hIcon);
	}

	void UpdateIcon(TASKDIALOG_ICON_ELEMENTS element, LPCWSTR lpstrIcon)
	{
		ATLASSERT(m_hWnd != NULL);
#ifdef _DEBUG
		if(element == TDIE_ICON_MAIN)
			ATLASSERT((m_tdc.dwFlags & TDF_USE_HICON_MAIN) == 0);
		else if(element == TDIE_ICON_FOOTER)
			ATLASSERT((m_tdc.dwFlags & TDF_USE_HICON_FOOTER) == 0);
#endif // _DEBUG
		::SendMessage(m_hWnd, TDM_UPDATE_ICON, element, (LPARAM)lpstrIcon);
	}
};


///////////////////////////////////////////////////////////////////////////////
// CTaskDialog - for non-customized task dialogs

class CTaskDialog : public CTaskDialogImpl<CTaskDialog>
{
public:
	CTaskDialog(HWND hWndParent = NULL) : CTaskDialogImpl<CTaskDialog>(hWndParent)
	{
		m_tdc.pfCallback = NULL;
	}
};

#endif // (_WIN32_WINNT >= 0x0600) || defined(_WTL_TASKDIALOG)

} // namespace WTL

#endif // __ATLDLGS_H__
