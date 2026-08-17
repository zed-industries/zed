// Windows Template Library - WTL version 10.0
// Copyright (C) Microsoft Corporation, WTL Team. All rights reserved.
//
// This file is a part of the Windows Template Library.
// The use and distribution terms for this software are covered by the
// Microsoft Public License (http://opensource.org/licenses/MS-PL)
// which can be found in the file MS-PL.txt at the root folder.

#ifndef __ATLFIND_H__
#define __ATLFIND_H__

#pragma once

#ifndef __ATLCTRLS_H__
	#error atlfind.h requires atlctrls.h to be included first
#endif

#ifndef __ATLDLGS_H__
	#error atlfind.h requires atldlgs.h to be included first
#endif

#ifndef __ATLSTR_H__
	#error atlfind.h requires CString
#endif


///////////////////////////////////////////////////////////////////////////////
// Classes in this file:
//
// CEditFindReplaceImplBase<T, TFindReplaceDialog>
// CEditFindReplaceImpl<T, TFindReplaceDialog>
// CRichEditFindReplaceImpl<T, TFindReplaceDialog>


namespace WTL
{

///////////////////////////////////////////////////////////////////////////////
// CEditFindReplaceImplBase - Base class for mixin classes that
// help implement Find/Replace for CEdit or CRichEditCtrl based window classes.

template <class T, class TFindReplaceDialog = CFindReplaceDialog>
class CEditFindReplaceImplBase
{
protected:
// Typedefs
	typedef CEditFindReplaceImplBase<T, TFindReplaceDialog> thisClass;

// Data members
	TFindReplaceDialog* m_pFindReplaceDialog;
	ATL::CString m_sFindNext, m_sReplaceWith;
	BOOL m_bFindOnly, m_bFirstSearch, m_bMatchCase, m_bWholeWord, m_bFindDown;
	LONG m_nInitialSearchPos;
	HCURSOR m_hOldCursor;

// Enumerations
	enum TranslationTextItem
	{
		eText_OnReplaceAllMessage   = 0,
		eText_OnReplaceAllTitle     = 1,
		eText_OnTextNotFoundMessage = 2,
		eText_OnTextNotFoundTitle   = 3
	};

public:
// Constructors
	CEditFindReplaceImplBase() :
		m_pFindReplaceDialog(NULL),
		m_bFindOnly(TRUE),
		m_bFirstSearch(TRUE),
		m_bMatchCase(FALSE),
		m_bWholeWord(FALSE),
		m_bFindDown(TRUE),
		m_nInitialSearchPos(0),
		m_hOldCursor(NULL)
	{
	}

// Message Handlers
	BEGIN_MSG_MAP(thisClass)
	ALT_MSG_MAP(1)
		MESSAGE_HANDLER(WM_DESTROY, OnDestroy)
		MESSAGE_HANDLER(TFindReplaceDialog::GetFindReplaceMsg(), OnFindReplaceCmd)
		COMMAND_ID_HANDLER(ID_EDIT_FIND, OnEditFind)
		COMMAND_ID_HANDLER(ID_EDIT_REPEAT, OnEditRepeat)
		COMMAND_ID_HANDLER(ID_EDIT_REPLACE, OnEditReplace)
	END_MSG_MAP()

	LRESULT OnDestroy(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& bHandled)
	{
		if(m_pFindReplaceDialog != NULL)
		{
			m_pFindReplaceDialog->SendMessage(WM_CLOSE);
			ATLASSERT(m_pFindReplaceDialog == NULL);
		}

		bHandled = FALSE;
		return 0;
	}

	LRESULT OnFindReplaceCmd(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM lParam, BOOL& /*bHandled*/)
	{
		T* pT = static_cast<T*>(this);

		TFindReplaceDialog* pDialog = TFindReplaceDialog::GetNotifier(lParam);
		if(pDialog == NULL)
		{
			ATLASSERT(FALSE);
			::MessageBeep(MB_ICONERROR);
			return 1;
		}
		ATLASSERT(pDialog == m_pFindReplaceDialog);

		LPFINDREPLACE findReplace = (LPFINDREPLACE)lParam;
		if((m_pFindReplaceDialog != NULL) && (findReplace != NULL))
		{
			if(pDialog->FindNext())
			{
				pT->OnFindNext(pDialog->GetFindString(), pDialog->SearchDown(),
					pDialog->MatchCase(), pDialog->MatchWholeWord());
			}
			else if(pDialog->ReplaceCurrent())
			{
				pT->OnReplaceSel(pDialog->GetFindString(),
					pDialog->SearchDown(), pDialog->MatchCase(), pDialog->MatchWholeWord(),
					pDialog->GetReplaceString());
			}
			else if(pDialog->ReplaceAll())
			{
				pT->OnReplaceAll(pDialog->GetFindString(), pDialog->GetReplaceString(),
					pDialog->MatchCase(), pDialog->MatchWholeWord());
			}
			else if(pDialog->IsTerminating())
			{
				// Dialog is going away (but hasn't gone away yet)
				// OnFinalMessage will "delete this"
				pT->OnTerminatingFindReplaceDialog(m_pFindReplaceDialog);
				m_pFindReplaceDialog = NULL;
			}
		}

		return 0;
	}

	LRESULT OnEditFind(WORD /*wNotifyCode*/, WORD /*wID*/, HWND /*hWndCtl*/, BOOL& /*bHandled*/)
	{
		T* pT = static_cast<T*>(this);
		pT->FindReplace(TRUE);

		return 0;
	}

	LRESULT OnEditRepeat(WORD /*wNotifyCode*/, WORD /*wID*/, HWND /*hWndCtl*/, BOOL& /*bHandled*/)
	{
		T* pT = static_cast<T*>(this);

		// If the user is holding down SHIFT when hitting F3, we'll
		// search in reverse. Otherwise, we'll search forward.
		// (be sure to have an accelerator mapped to ID_EDIT_REPEAT
		// for both F3 and Shift+F3)
		m_bFindDown = !((::GetKeyState(VK_SHIFT) & 0x8000) == 0x8000);

		if(m_sFindNext.IsEmpty())
		{
			pT->FindReplace(TRUE);
		}
		else
		{
			if(!pT->FindTextSimple(m_sFindNext, m_bMatchCase, m_bWholeWord, m_bFindDown))
				pT->TextNotFound(m_sFindNext);
		}

		return 0;
	}

	LRESULT OnEditReplace(WORD /*wNotifyCode*/, WORD /*wID*/, HWND /*hWndCtl*/, BOOL& bHandled)
	{
		T* pT = static_cast<T*>(this);

		DWORD style = pT->GetStyle();
		if((style & ES_READONLY) != ES_READONLY)
		{
			pT->FindReplace(FALSE);
		}
		else
		{
			// Don't allow replace when the edit control is read only
			bHandled = FALSE;
		}

		return 0;
	}

// Operations (overrideable)
	TFindReplaceDialog* CreateFindReplaceDialog(BOOL bFindOnly, // TRUE for Find, FALSE for FindReplace
			LPCTSTR lpszFindWhat,
			LPCTSTR lpszReplaceWith = NULL,
			DWORD dwFlags = FR_DOWN,
			HWND hWndParent = NULL)
	{
		// You can override all of this in a derived class

		TFindReplaceDialog* findReplaceDialog = new TFindReplaceDialog();
		if(findReplaceDialog == NULL)
		{
			::MessageBeep(MB_ICONHAND);
		}
		else
		{
			HWND hWndFindReplace = findReplaceDialog->Create(bFindOnly,
				lpszFindWhat, lpszReplaceWith, dwFlags, hWndParent);
			if(hWndFindReplace == NULL)
			{
				delete findReplaceDialog;
				findReplaceDialog = NULL;
			}
			else
			{
				findReplaceDialog->SetActiveWindow();
				findReplaceDialog->ShowWindow(SW_SHOW);
			}
		}

		return findReplaceDialog;
	}

	void AdjustDialogPosition(HWND hWndDialog)
	{
		ATLASSERT((hWndDialog != NULL) && ::IsWindow(hWndDialog));

		T* pT = static_cast<T*>(this);
		LONG nStartChar = 0, nEndChar = 0;
		// Send EM_GETSEL so we can use both Edit and RichEdit
		// (CEdit::GetSel uses int&, and CRichEditCtrlT::GetSel uses LONG&)
		::SendMessage(pT->m_hWnd, EM_GETSEL, (WPARAM)&nStartChar, (LPARAM)&nEndChar);
		POINT point = pT->PosFromChar(nStartChar);
		::ClientToScreen(pT->GetParent(), &point);
		RECT rect = {};
		::GetWindowRect(hWndDialog, &rect);
		if(::PtInRect(&rect, point) != FALSE)
		{
			if(point.y > (rect.bottom - rect.top))
			{
				::OffsetRect(&rect, 0, point.y - rect.bottom - 20);
			}
			else
			{
				int nVertExt = GetSystemMetrics(SM_CYSCREEN);
				if((point.y + (rect.bottom - rect.top)) < nVertExt)
					::OffsetRect(&rect, 0, 40 + point.y - rect.top);
			}

			::MoveWindow(hWndDialog, rect.left, rect.top, rect.right - rect.left, rect.bottom - rect.top, TRUE);
		}
	}

	DWORD GetFindReplaceDialogFlags() const
	{
		DWORD dwFlags = 0;
		if(m_bFindDown)
			dwFlags |= FR_DOWN;
		if(m_bMatchCase)
			dwFlags |= FR_MATCHCASE;
		if(m_bWholeWord)
			dwFlags |= FR_WHOLEWORD;

		return dwFlags;
	}

	void FindReplace(BOOL bFindOnly)
	{
		T* pT = static_cast<T*>(this);
		m_bFirstSearch = TRUE;
		if(m_pFindReplaceDialog != NULL)
		{
			if(m_bFindOnly == bFindOnly)
			{
				m_pFindReplaceDialog->SetActiveWindow();
				m_pFindReplaceDialog->ShowWindow(SW_SHOW);
				return;
			}
			else
			{
				m_pFindReplaceDialog->SendMessage(WM_CLOSE);
				ATLASSERT(m_pFindReplaceDialog == NULL);
			}
		}

		ATLASSERT(m_pFindReplaceDialog == NULL);

		ATL::CString findNext;
		pT->GetSelText(findNext);
		// if selection is empty or spans multiple lines use old find text
		if(findNext.IsEmpty() || (findNext.FindOneOf(_T("\n\r")) != -1))
			findNext = m_sFindNext;
		ATL::CString replaceWith = m_sReplaceWith;
		DWORD dwFlags = pT->GetFindReplaceDialogFlags();

		m_pFindReplaceDialog = pT->CreateFindReplaceDialog(bFindOnly,
			findNext, replaceWith, dwFlags, pT->operator HWND());
		ATLASSERT(m_pFindReplaceDialog != NULL);
		if(m_pFindReplaceDialog != NULL)
			m_bFindOnly = bFindOnly;
	}

	BOOL SameAsSelected(LPCTSTR lpszCompare, BOOL bMatchCase, BOOL /*bWholeWord*/)
	{
		T* pT = static_cast<T*>(this);

		// check length first
		size_t nLen = lstrlen(lpszCompare);
		LONG nStartChar = 0, nEndChar = 0;
		// Send EM_GETSEL so we can use both Edit and RichEdit
		// (CEdit::GetSel uses int&, and CRichEditCtrlT::GetSel uses LONG&)
		::SendMessage(pT->m_hWnd, EM_GETSEL, (WPARAM)&nStartChar, (LPARAM)&nEndChar);
		if(nLen != (size_t)(nEndChar - nStartChar))
			return FALSE;

		// length is the same, check contents
		ATL::CString selectedText;
		pT->GetSelText(selectedText);

		return (bMatchCase && (selectedText.Compare(lpszCompare) == 0)) ||
			(!bMatchCase && (selectedText.CompareNoCase(lpszCompare) == 0));
	}

	void TextNotFound(LPCTSTR lpszFind)
	{
		T* pT = static_cast<T*>(this);
		m_bFirstSearch = TRUE;
		pT->OnTextNotFound(lpszFind);
	}

	ATL::CString GetTranslationText(enum TranslationTextItem eItem) const
	{
		ATL::CString text;
		switch(eItem)
		{
		case eText_OnReplaceAllMessage:
			text = _T("Replaced %d occurances of \"%s\" with \"%s\"");
			break;
		case eText_OnReplaceAllTitle:
			text = _T("Replace All");
			break;
		case eText_OnTextNotFoundMessage:
			text = _T("Unable to find the text \"%s\"");
			break;
		case eText_OnTextNotFoundTitle:
			text = _T("Text not found");
			break;
		}

		return text;
	}

// Overrideable Handlers
	void OnFindNext(LPCTSTR lpszFind, BOOL bFindDown, BOOL bMatchCase, BOOL bWholeWord)
	{
		T* pT = static_cast<T*>(this);

		m_sFindNext = lpszFind;
		m_bMatchCase = bMatchCase;
		m_bWholeWord = bWholeWord;
		m_bFindDown = bFindDown;

		if(!pT->FindTextSimple(m_sFindNext, m_bMatchCase, m_bWholeWord, m_bFindDown))
			pT->TextNotFound(m_sFindNext);
		else
			pT->AdjustDialogPosition(m_pFindReplaceDialog->operator HWND());
	}

	void OnReplaceSel(LPCTSTR lpszFind, BOOL bFindDown, BOOL bMatchCase, BOOL bWholeWord, LPCTSTR lpszReplace)
	{
		T* pT = static_cast<T*>(this);

		m_sFindNext = lpszFind;
		m_sReplaceWith = lpszReplace;
		m_bMatchCase = bMatchCase;
		m_bWholeWord = bWholeWord;
		m_bFindDown = bFindDown;

		if(pT->SameAsSelected(m_sFindNext, m_bMatchCase, m_bWholeWord))
			pT->ReplaceSel(m_sReplaceWith);

		if(!pT->FindTextSimple(m_sFindNext, m_bMatchCase, m_bWholeWord, m_bFindDown))
			pT->TextNotFound(m_sFindNext);
		else
			pT->AdjustDialogPosition(m_pFindReplaceDialog->operator HWND());
	}

	void OnReplaceAll(LPCTSTR lpszFind, LPCTSTR lpszReplace, BOOL bMatchCase, BOOL bWholeWord)
	{
		T* pT = static_cast<T*>(this);

		m_sFindNext = lpszFind;
		m_sReplaceWith = lpszReplace;
		m_bMatchCase = bMatchCase;
		m_bWholeWord = bWholeWord;
		m_bFindDown = TRUE;

		// no selection or different than what looking for
		if(!pT->SameAsSelected(m_sFindNext, m_bMatchCase, m_bWholeWord))
		{
			if(!pT->FindTextSimple(m_sFindNext, m_bMatchCase, m_bWholeWord, m_bFindDown))
			{
				pT->TextNotFound(m_sFindNext);
				return;
			}
		}

		pT->OnReplaceAllCoreBegin();

		int replaceCount=0;
		do
		{
			++replaceCount;
			pT->ReplaceSel(m_sReplaceWith);
		} while(pT->FindTextSimple(m_sFindNext, m_bMatchCase, m_bWholeWord, m_bFindDown));

		pT->OnReplaceAllCoreEnd(replaceCount);
	}

	void OnReplaceAllCoreBegin()
	{
		T* pT = static_cast<T*>(this);

		m_hOldCursor = ::SetCursor(::LoadCursor(NULL, IDC_WAIT));

		pT->HideSelection(TRUE, FALSE);

	}

	void OnReplaceAllCoreEnd(int replaceCount)
	{
		T* pT = static_cast<T*>(this);
		pT->HideSelection(FALSE, FALSE);

		::SetCursor(m_hOldCursor);

		ATL::CString message = pT->GetTranslationText(eText_OnReplaceAllMessage);
		if(message.GetLength() > 0)
		{
			ATL::CString formattedMessage;
			formattedMessage.Format(message, replaceCount, (LPCTSTR)m_sFindNext, (LPCTSTR)m_sReplaceWith);
			if(m_pFindReplaceDialog != NULL)
			{
				m_pFindReplaceDialog->MessageBox(formattedMessage,
					pT->GetTranslationText(eText_OnReplaceAllTitle),
					MB_OK | MB_ICONINFORMATION | MB_APPLMODAL);
			}
			else
			{
				pT->MessageBox(formattedMessage,
					pT->GetTranslationText(eText_OnReplaceAllTitle),
					MB_OK | MB_ICONINFORMATION | MB_APPLMODAL);
			}
		}
	}

	void OnTextNotFound(LPCTSTR lpszFind)
	{
		T* pT = static_cast<T*>(this);
		ATL::CString message = pT->GetTranslationText(eText_OnTextNotFoundMessage);
		if(message.GetLength() > 0)
		{
			ATL::CString formattedMessage;
			formattedMessage.Format(message, lpszFind);
			if(m_pFindReplaceDialog != NULL)
			{
				m_pFindReplaceDialog->MessageBox(formattedMessage,
					pT->GetTranslationText(eText_OnTextNotFoundTitle),
					MB_OK | MB_ICONINFORMATION | MB_APPLMODAL);
			}
			else
			{
				pT->MessageBox(formattedMessage,
					pT->GetTranslationText(eText_OnTextNotFoundTitle),
					MB_OK | MB_ICONINFORMATION | MB_APPLMODAL);
			}
		}
		else
		{
			::MessageBeep(MB_ICONHAND);
		}
	}

	void OnTerminatingFindReplaceDialog(TFindReplaceDialog*& /*findReplaceDialog*/)
	{
	}
};


///////////////////////////////////////////////////////////////////////////////
// CEditFindReplaceImpl - Mixin class for implementing Find/Replace for CEdit
// based window classes.

// Chain to CEditFindReplaceImpl message map. Your class must also derive from CEdit.
// Example:
// class CMyEdit : public CWindowImpl<CMyEdit, CEdit>,
//                 public CEditFindReplaceImpl<CMyEdit>
// {
// public:
//      BEGIN_MSG_MAP(CMyEdit)
//              // your handlers...
//              CHAIN_MSG_MAP_ALT(CEditFindReplaceImpl<CMyEdit>, 1)
//      END_MSG_MAP()
//      // other stuff...
// };

template <class T, class TFindReplaceDialog = CFindReplaceDialog>
class CEditFindReplaceImpl : public CEditFindReplaceImplBase<T, TFindReplaceDialog>
{
protected:
	typedef CEditFindReplaceImpl<T, TFindReplaceDialog> thisClass;
	typedef CEditFindReplaceImplBase<T, TFindReplaceDialog> baseClass;

public:
// Message Handlers
	BEGIN_MSG_MAP(thisClass)
	ALT_MSG_MAP(1)
		CHAIN_MSG_MAP_ALT(baseClass, 1)
	END_MSG_MAP()

// Operations
	// Supported only for RichEdit, so this does nothing for Edit
	void HideSelection(BOOL /*bHide*/ = TRUE, BOOL /*bChangeStyle*/ = FALSE)
	{
	}

// Operations (overrideable)
	BOOL FindTextSimple(LPCTSTR lpszFind, BOOL bMatchCase, BOOL bWholeWord, BOOL bFindDown = TRUE)
	{
		T* pT = static_cast<T*>(this);

		ATLASSERT(lpszFind != NULL);
		ATLASSERT(*lpszFind != _T('\0'));

		UINT nLen = pT->GetBufferLength();
		int nStartChar = 0, nEndChar = 0;
		pT->GetSel(nStartChar, nEndChar);
		UINT nStart = nStartChar;
		int iDir = bFindDown ? +1 : -1;

		// can't find a match before the first character
		if((nStart == 0) && (iDir < 0))
			return FALSE;

		LPCTSTR lpszText = pT->LockBuffer();

		bool isDBCS = false;
#ifdef _MBCS
		CPINFO info = {};
		::GetCPInfo(::GetOEMCP(), &info);
		isDBCS = (info.MaxCharSize > 1);
#endif

		if(iDir < 0)
		{
			// always go back one for search backwards
			nStart -= int((lpszText + nStart) - ::CharPrev(lpszText, lpszText + nStart));
		}
		else if((nStartChar != nEndChar) && (pT->SameAsSelected(lpszFind, bMatchCase, bWholeWord)))
		{
			// easy to go backward/forward with SBCS
#ifndef _UNICODE
			if(::IsDBCSLeadByte(lpszText[nStart]))
				nStart++;
#endif
			nStart += iDir;
		}

		// handle search with nStart past end of buffer
		UINT nLenFind = ::lstrlen(lpszFind);
		if((nStart + nLenFind - 1) >= nLen)
		{
			if((iDir < 0) && (nLen >= nLenFind))
			{
				if(isDBCS)
				{
					// walk back to previous character n times
					nStart = nLen;
					int n = nLenFind;
					while(n--)
					{
						nStart -= int((lpszText + nStart) - ::CharPrev(lpszText, lpszText + nStart));
					}
				}
				else
				{
					// single-byte character set is easy and fast
					nStart = nLen - nLenFind;
				}
				ATLASSERT((nStart + nLenFind - 1) <= nLen);
			}
			else
			{
				pT->UnlockBuffer();
				return FALSE;
			}
		}

		// start the search at nStart
		LPCTSTR lpsz = lpszText + nStart;
		typedef int (WINAPI* CompareProc)(LPCTSTR str1, LPCTSTR str2);
		CompareProc pfnCompare = bMatchCase ? lstrcmp : lstrcmpi;

		if(isDBCS)
		{
			// double-byte string search
			LPCTSTR lpszStop = NULL;
			if(iDir > 0)
			{
				// start at current and find _first_ occurrance
				lpszStop = lpszText + nLen - nLenFind + 1;
			}
			else
			{
				// start at top and find _last_ occurrance
				lpszStop = lpsz;
				lpsz = lpszText;
			}

			LPCTSTR lpszFound = NULL;
			while(lpsz <= lpszStop)
			{
#ifndef _UNICODE
				if(!bMatchCase || ((*lpsz == *lpszFind) && (!::IsDBCSLeadByte(*lpsz) || (lpsz[1] == lpszFind[1]))))
#else
				if(!bMatchCase || ((*lpsz == *lpszFind) && (lpsz[1] == lpszFind[1])))
#endif
				{
					LPTSTR lpch = (LPTSTR)(lpsz + nLenFind);
					TCHAR chSave = *lpch;
					*lpch = _T('\0');
					int nResult = (*pfnCompare)(lpsz, lpszFind);
					*lpch = chSave;
					if(nResult == 0)
					{
						lpszFound = lpsz;
						if(iDir > 0)
							break;
					}
				}
				lpsz = ::CharNext(lpsz);
			}
			pT->UnlockBuffer();

			if(lpszFound != NULL)
			{
				int n = (int)(lpszFound - lpszText);
				pT->SetSel(n, n + nLenFind);
				return TRUE;
			}
		}
		else
		{
			// single-byte string search
			UINT nCompare = 0;
			if(iDir < 0)
				nCompare = (UINT)(lpsz - lpszText) + 1;
			else
				nCompare = nLen - (UINT)(lpsz - lpszText) - nLenFind + 1;

			while(nCompare > 0)
			{
				ATLASSERT(lpsz >= lpszText);
				ATLASSERT((lpsz + nLenFind - 1) <= (lpszText + nLen - 1));

				LPSTR lpch = (LPSTR)(lpsz + nLenFind);
				char chSave = *lpch;
				*lpch = '\0';
				int nResult = (*pfnCompare)(lpsz, lpszFind);
				*lpch = chSave;
				if(nResult == 0)
				{
					pT->UnlockBuffer();
					int n = (int)(lpsz - lpszText);
					pT->SetSel(n, n + nLenFind);
					return TRUE;
				}

				// restore character at end of search
				*lpch = chSave;

				// move on to next substring
				nCompare--;
				lpsz += iDir;
			}
			pT->UnlockBuffer();
		}

		return FALSE;
	}

	LPCTSTR LockBuffer() const
	{
		const T* pT = static_cast<const T*>(this);
		ATLASSERT(pT->m_hWnd != NULL);

#ifndef _UNICODE
		// If common controls version 6 is in use (such as via theming), then EM_GETHANDLE 
		// will always return a UNICODE string. You're really not supposed to superclass 
		// or subclass common controls with an ANSI windows procedure.
		DWORD dwMajor = 0, dwMinor = 0;
		HRESULT hRet = ATL::AtlGetCommCtrlVersion(&dwMajor, &dwMinor);
		if(SUCCEEDED(hRet) && (dwMajor >= 6))
		{
			ATLTRACE2(atlTraceUI, 0, _T("AtlFind Warning: You have compiled for MBCS/ANSI but are using common controls version 6 or later which are always Unicode.\r\n"));
			ATLASSERT(FALSE);
		}
#endif // !_UNICODE

		HLOCAL hLocal = pT->GetHandle();
		ATLASSERT(hLocal != NULL);
		LPCTSTR lpszText = (LPCTSTR)::LocalLock(hLocal);
		ATLASSERT(lpszText != NULL);

		return lpszText;
	}

	void UnlockBuffer() const
	{
		const T* pT = static_cast<const T*>(this);
		ATLASSERT(pT->m_hWnd != NULL);

		HLOCAL hLocal = pT->GetHandle();
		ATLASSERT(hLocal != NULL);
		::LocalUnlock(hLocal);
	}

	UINT GetBufferLength() const
	{
		const T* pT = static_cast<const T*>(this);
		ATLASSERT(pT->m_hWnd != NULL);

		UINT nLen = 0;
		LPCTSTR lpszText = pT->LockBuffer();
		if(lpszText != NULL)
			nLen = ::lstrlen(lpszText);
		pT->UnlockBuffer();

		return nLen;
	}

	LONG EndOfLine(LPCTSTR lpszText, UINT nLen, UINT nIndex) const
	{
		LPCTSTR lpsz = lpszText + nIndex;
		LPCTSTR lpszStop = lpszText + nLen;
		while((lpsz < lpszStop) && (*lpsz != _T('\r')))
			++lpsz;
		return LONG(lpsz - lpszText);
	}

	LONG GetSelText(ATL::CString& strText) const
	{
		const T* pT = static_cast<const T*>(this);

		int nStartChar = 0, nEndChar = 0;
		pT->GetSel(nStartChar, nEndChar);
		ATLASSERT((UINT)nEndChar <= pT->GetBufferLength());
		LPCTSTR lpszText = pT->LockBuffer();
		LONG nLen = pT->EndOfLine(lpszText, nEndChar, nStartChar) - nStartChar;
		ATL::Checked::memcpy_s(strText.GetBuffer(nLen), nLen * sizeof(TCHAR), lpszText + nStartChar, nLen * sizeof(TCHAR));
		strText.ReleaseBuffer(nLen);
		pT->UnlockBuffer();

		return nLen;
	}
};


///////////////////////////////////////////////////////////////////////////////
// CRichEditFindReplaceImpl - Mixin class for implementing Find/Replace for CRichEditCtrl
// based window classes.

// Chain to CRichEditFindReplaceImpl message map. Your class must also derive from CRichEditCtrl.
// Example:
// class CMyRichEdit : public CWindowImpl<CMyRichEdit, CRichEditCtrl>,
//                     public CRichEditFindReplaceImpl<CMyRichEdit>
// {
// public:
//      BEGIN_MSG_MAP(CMyRichEdit)
//              // your handlers...
//              CHAIN_MSG_MAP_ALT(CRichEditFindReplaceImpl<CMyRichEdit>, 1)
//      END_MSG_MAP()
//      // other stuff...
// };

template <class T, class TFindReplaceDialog = CFindReplaceDialog>
class CRichEditFindReplaceImpl : public CEditFindReplaceImplBase<T, TFindReplaceDialog>
{
protected:
	typedef CRichEditFindReplaceImpl<T, TFindReplaceDialog> thisClass;
	typedef CEditFindReplaceImplBase<T, TFindReplaceDialog> baseClass;

public:
	BEGIN_MSG_MAP(thisClass)
	ALT_MSG_MAP(1)
		CHAIN_MSG_MAP_ALT(baseClass, 1)
	END_MSG_MAP()

// Operations (overrideable)
	BOOL FindTextSimple(LPCTSTR lpszFind, BOOL bMatchCase, BOOL bWholeWord, BOOL bFindDown = TRUE)
	{
		T* pT = static_cast<T*>(this);

		ATLASSERT(lpszFind != NULL);
		FINDTEXTEX ft = {};

		pT->GetSel(ft.chrg);
		if(this->m_bFirstSearch)
		{
			if(bFindDown)
				this->m_nInitialSearchPos = ft.chrg.cpMin;
			else
				this->m_nInitialSearchPos = ft.chrg.cpMax;
			this->m_bFirstSearch = FALSE;
		}

		ft.lpstrText = (LPTSTR)lpszFind;

		if(ft.chrg.cpMin != ft.chrg.cpMax) // i.e. there is a selection
		{
			if(bFindDown)
			{
				ft.chrg.cpMin++;
			}
			else
			{
				// won't wraparound backwards
				ft.chrg.cpMin = __max(ft.chrg.cpMin, 0);
			}
		}

		DWORD dwFlags = bMatchCase ? FR_MATCHCASE : 0;
		dwFlags |= bWholeWord ? FR_WHOLEWORD : 0;

		ft.chrg.cpMax = pT->GetTextLength() + this->m_nInitialSearchPos;

		if(bFindDown)
		{
			if(this->m_nInitialSearchPos >= 0)
				ft.chrg.cpMax = pT->GetTextLength();

			dwFlags |= FR_DOWN;
			ATLASSERT(ft.chrg.cpMax >= ft.chrg.cpMin);
		}
		else
		{
			if(this->m_nInitialSearchPos >= 0)
				ft.chrg.cpMax = 0;

			dwFlags &= ~FR_DOWN;
			ATLASSERT(ft.chrg.cpMax <= ft.chrg.cpMin);
		}

		BOOL bRet = FALSE;
		if(pT->FindAndSelect(dwFlags, ft) != -1)
		{
			bRet = TRUE;   // we found the text
		}
		else if(this->m_nInitialSearchPos > 0)
		{
			// if the original starting point was not the beginning
			// of the buffer and we haven't already been here
			if(bFindDown)
			{
				ft.chrg.cpMin = 0;
				ft.chrg.cpMax = this->m_nInitialSearchPos;
			}
			else
			{
				ft.chrg.cpMin = pT->GetTextLength();
				ft.chrg.cpMax = this->m_nInitialSearchPos;
			}
			this->m_nInitialSearchPos = this->m_nInitialSearchPos - pT->GetTextLength();

			bRet = (pT->FindAndSelect(dwFlags, ft) != -1) ? TRUE : FALSE;
		}

		return bRet;
	}

	long FindAndSelect(DWORD dwFlags, FINDTEXTEX& ft)
	{
		T* pT = static_cast<T*>(this);
		LONG index = pT->FindText(dwFlags, ft);
		if(index != -1) // i.e. we found something
			pT->SetSel(ft.chrgText);

		return index;
	}
};

} // namespace WTL

#endif // __ATLFIND_H__
