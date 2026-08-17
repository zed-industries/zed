// Windows Template Library - WTL version 10.0
// Copyright (C) Microsoft Corporation, WTL Team. All rights reserved.
//
// This file is a part of the Windows Template Library.
// The use and distribution terms for this software are covered by the
// Microsoft Public License (http://opensource.org/licenses/MS-PL)
// which can be found in the file MS-PL.txt at the root folder.

#ifndef __ATLPRINT_H__
#define __ATLPRINT_H__

#pragma once

#ifndef __ATLAPP_H__
	#error atlprint.h requires atlapp.h to be included first
#endif

#ifndef __ATLWIN_H__
	#error atlprint.h requires atlwin.h to be included first
#endif

#include <winspool.h>


///////////////////////////////////////////////////////////////////////////////
// Classes in this file:
//
// CPrinterInfo<t_nInfo>
// CPrinterT<t_bManaged>
// CDevModeT<t_bManaged>
// CPrinterDC
// CPrintJobInfo
// CPrintJob
// CPrintPreview
// CPrintPreviewWindowImpl<T, TBase, TWinTraits>
// CPrintPreviewWindow
// CZoomPrintPreviewWindowImpl<T, TBase, TWinTraits>
// CZoomPrintPreviewWindow

namespace WTL
{

///////////////////////////////////////////////////////////////////////////////
// CPrinterInfo - This class wraps all of the PRINTER_INFO_* structures
//                and provided by ::GetPrinter.

template <unsigned int t_nInfo>
class _printer_info
{
public:
	typedef void infotype;
};

template <> class _printer_info<1> { public: typedef PRINTER_INFO_1 infotype; };
template <> class _printer_info<2> { public: typedef PRINTER_INFO_2 infotype; };
template <> class _printer_info<3> { public: typedef PRINTER_INFO_3 infotype; };
template <> class _printer_info<4> { public: typedef PRINTER_INFO_4 infotype; };
template <> class _printer_info<5> { public: typedef PRINTER_INFO_5 infotype; };
template <> class _printer_info<6> { public: typedef PRINTER_INFO_6 infotype; };
template <> class _printer_info<7> { public: typedef PRINTER_INFO_7 infotype; };
template <> class _printer_info<8> { public: typedef PRINTER_INFO_8 infotype; };
template <> class _printer_info<9> { public: typedef PRINTER_INFO_9 infotype; };


template <unsigned int t_nInfo>
class CPrinterInfo
{
public:
// Data members
	typename _printer_info<t_nInfo>::infotype* m_pi;

// Constructor/destructor
	CPrinterInfo() : m_pi(NULL)
	{ }

	CPrinterInfo(HANDLE hPrinter) : m_pi(NULL)
	{
		GetPrinterInfo(hPrinter);
	}

	~CPrinterInfo()
	{
		Cleanup();
	}

// Operations
	bool GetPrinterInfo(HANDLE hPrinter)
	{
		Cleanup();
		return GetPrinterInfoHelper(hPrinter, (BYTE**)&m_pi, t_nInfo);
	}

// Implementation
	void Cleanup()
	{
		delete [] (BYTE*)m_pi;
		m_pi = NULL;
	}

	static bool GetPrinterInfoHelper(HANDLE hPrinter, BYTE** pi, int nIndex)
	{
		ATLASSERT(pi != NULL);
		DWORD dw = 0;
		BYTE* pb = NULL;
		::GetPrinter(hPrinter, nIndex, NULL, 0, &dw);
		if (dw > 0)
		{
			ATLTRY(pb = new BYTE[dw]);
			if (pb != NULL)
			{
				memset(pb, 0, dw);
				DWORD dwNew;
				if (!::GetPrinter(hPrinter, nIndex, pb, dw, &dwNew))
				{
					delete [] pb;
					pb = NULL;
				}
			}
		}
		*pi = pb;
		return (pb != NULL);
	}
};


///////////////////////////////////////////////////////////////////////////////
// CPrinter - Wrapper class for a HANDLE to a printer

template <bool t_bManaged>
class CPrinterT
{
public:
// Data members
	HANDLE m_hPrinter;

// Constructor/destructor
	CPrinterT(HANDLE hPrinter = NULL) : m_hPrinter(hPrinter)
	{ }

	~CPrinterT()
	{
		ClosePrinter();
	}

// Operations
	CPrinterT& operator =(HANDLE hPrinter)
	{
		if (hPrinter != m_hPrinter)
		{
			ClosePrinter();
			m_hPrinter = hPrinter;
		}
		return *this;
	}

	bool IsNull() const { return (m_hPrinter == NULL); }

	bool OpenPrinter(HANDLE hDevNames, const DEVMODE* pDevMode = NULL)
	{
		bool b = false;
		DEVNAMES* pdn = (DEVNAMES*)::GlobalLock(hDevNames);
		if (pdn != NULL)
		{
			LPTSTR lpszPrinterName = (LPTSTR)pdn + pdn->wDeviceOffset;
			b = OpenPrinter(lpszPrinterName, pDevMode);
			::GlobalUnlock(hDevNames);
		}
		return b;
	}

	bool OpenPrinter(LPCTSTR lpszPrinterName, const DEVMODE* pDevMode = NULL)
	{
		ClosePrinter();
		PRINTER_DEFAULTS pdefs = { NULL, (DEVMODE*)pDevMode, PRINTER_ACCESS_USE };
		::OpenPrinter((LPTSTR) lpszPrinterName, &m_hPrinter, (pDevMode == NULL) ? NULL : &pdefs);

		return (m_hPrinter != NULL);
	}

	bool OpenPrinter(LPCTSTR lpszPrinterName, PRINTER_DEFAULTS* pprintdefs)
	{
		ClosePrinter();
		::OpenPrinter((LPTSTR) lpszPrinterName, &m_hPrinter, pprintdefs);
		return (m_hPrinter != NULL);
	}

	bool OpenDefaultPrinter(const DEVMODE* pDevMode = NULL)
	{
		ClosePrinter();

		DWORD cchBuff = 0;
		::GetDefaultPrinter(NULL, &cchBuff);
		TCHAR* pszBuff = new TCHAR[cchBuff];
		BOOL bRet = ::GetDefaultPrinter(pszBuff, &cchBuff);
		if(bRet != FALSE)
		{
			PRINTER_DEFAULTS pdefs = { NULL, (DEVMODE*)pDevMode, PRINTER_ACCESS_USE };
			::OpenPrinter(pszBuff, &m_hPrinter, (pDevMode == NULL) ? NULL : &pdefs);
		}
		delete [] pszBuff;

		return m_hPrinter != NULL;
	}

	void ClosePrinter()
	{
		if (m_hPrinter != NULL)
		{
			if (t_bManaged)
				::ClosePrinter(m_hPrinter);
			m_hPrinter = NULL;
		}
	}

	bool PrinterProperties(HWND hWnd = NULL)
	{
		if (hWnd == NULL)
			hWnd = ::GetActiveWindow();
		return !!::PrinterProperties(hWnd, m_hPrinter);
	}

	HANDLE CopyToHDEVNAMES() const
	{
		HANDLE hDevNames = NULL;
		CPrinterInfo<5> pinfon5;
		CPrinterInfo<2> pinfon2;
		LPTSTR lpszPrinterName = NULL;
		LPTSTR lpszPortName = NULL;
		// Some printers fail for PRINTER_INFO_5 in some situations
		if(pinfon5.GetPrinterInfo(m_hPrinter))
		{
			lpszPrinterName = pinfon5.m_pi->pPrinterName;
			lpszPortName = pinfon5.m_pi->pPortName;
		}
		else if(pinfon2.GetPrinterInfo(m_hPrinter))
		{
			lpszPrinterName = pinfon2.m_pi->pPrinterName;
			lpszPortName = pinfon2.m_pi->pPortName;
		}

		if(lpszPrinterName != NULL)
		{
			int nLen = sizeof(DEVNAMES) + (lstrlen(lpszPrinterName) + 1 + lstrlen(lpszPortName) + 1) * sizeof(TCHAR);
			hDevNames = ::GlobalAlloc(GMEM_MOVEABLE, nLen);
			BYTE* pv = (BYTE*)::GlobalLock(hDevNames);
			DEVNAMES* pdev = (DEVNAMES*)pv;
			if(pv != NULL)
			{
				memset(pv, 0, nLen);
				pdev->wDeviceOffset = sizeof(DEVNAMES);
				pv = pv + sizeof(DEVNAMES); // now points to end
				ATL::Checked::tcscpy_s((LPTSTR)pv, lstrlen(lpszPrinterName) + 1, lpszPrinterName);
				pdev->wOutputOffset = (WORD)(sizeof(DEVNAMES) + (lstrlen(lpszPrinterName) + 1) * sizeof(TCHAR));
				pv = pv + (lstrlen(lpszPrinterName) + 1) * sizeof(TCHAR);
				ATL::Checked::tcscpy_s((LPTSTR)pv, lstrlen(lpszPortName) + 1, lpszPortName);
				::GlobalUnlock(hDevNames);
			}
		}

		return hDevNames;
	}

	HDC CreatePrinterDC(const DEVMODE* pdm = NULL) const
	{
		CPrinterInfo<5> pinfo5;
		CPrinterInfo<2> pinfo2;
		HDC hDC = NULL;
		LPTSTR lpszPrinterName = NULL;
		// Some printers fail for PRINTER_INFO_5 in some situations
		if (pinfo5.GetPrinterInfo(m_hPrinter))
			lpszPrinterName = pinfo5.m_pi->pPrinterName;
		else if (pinfo2.GetPrinterInfo(m_hPrinter))
			lpszPrinterName = pinfo2.m_pi->pPrinterName;
		if (lpszPrinterName != NULL)
			hDC = ::CreateDC(NULL, lpszPrinterName, NULL, pdm);
		return hDC;
	}

	HDC CreatePrinterIC(const DEVMODE* pdm = NULL) const
	{
		CPrinterInfo<5> pinfo5;
		CPrinterInfo<2> pinfo2;
		HDC hDC = NULL;
		LPTSTR lpszPrinterName = NULL;
		// Some printers fail for PRINTER_INFO_5 in some situations
		if (pinfo5.GetPrinterInfo(m_hPrinter))
			lpszPrinterName = pinfo5.m_pi->pPrinterName;
		else if (pinfo2.GetPrinterInfo(m_hPrinter))
			lpszPrinterName = pinfo2.m_pi->pPrinterName;
		if (lpszPrinterName != NULL)
			hDC = ::CreateIC(NULL, lpszPrinterName, NULL, pdm);
		return hDC;
	}

	void Attach(HANDLE hPrinter)
	{
		ClosePrinter();
		m_hPrinter = hPrinter;
	}

	HANDLE Detach()
	{
		HANDLE hPrinter = m_hPrinter;
		m_hPrinter = NULL;
		return hPrinter;
	}

	operator HANDLE() const { return m_hPrinter; }
};

typedef CPrinterT<false>   CPrinterHandle;
typedef CPrinterT<true>    CPrinter;


///////////////////////////////////////////////////////////////////////////////
// CDevMode - Wrapper class for DEVMODE

template <bool t_bManaged>
class CDevModeT
{
public:
// Data members
	HANDLE m_hDevMode;
	DEVMODE* m_pDevMode;

// Constructor/destructor
	CDevModeT(HANDLE hDevMode = NULL) : m_hDevMode(hDevMode)
	{
		m_pDevMode = (m_hDevMode != NULL) ? (DEVMODE*)::GlobalLock(m_hDevMode) : NULL;
	}

	~CDevModeT()
	{
		Cleanup();
	}

// Operations
	CDevModeT<t_bManaged>& operator =(HANDLE hDevMode)
	{
		Attach(hDevMode);
		return *this;
	}

	void Attach(HANDLE hDevModeNew)
	{
		Cleanup();
		m_hDevMode = hDevModeNew;
		m_pDevMode = (m_hDevMode != NULL) ? (DEVMODE*)::GlobalLock(m_hDevMode) : NULL;
	}

	HANDLE Detach()
	{
		if (m_hDevMode != NULL)
			::GlobalUnlock(m_hDevMode);
		HANDLE hDevMode = m_hDevMode;
		m_hDevMode = NULL;
		return hDevMode;
	}

	bool IsNull() const { return (m_hDevMode == NULL); }

	bool CopyFromPrinter(HANDLE hPrinter)
	{
		CPrinterInfo<2> pinfo;
		bool b = pinfo.GetPrinterInfo(hPrinter);
		if (b)
		 b = CopyFromDEVMODE(pinfo.m_pi->pDevMode);
		return b;
	}

	bool CopyFromDEVMODE(const DEVMODE* pdm)
	{
		if (pdm == NULL)
			return false;
		int nSize = pdm->dmSize + pdm->dmDriverExtra;
		HANDLE h = ::GlobalAlloc(GMEM_MOVEABLE, nSize);
		if (h != NULL)
		{
			void* p = ::GlobalLock(h);
			ATL::Checked::memcpy_s(p, nSize, pdm, nSize);
			::GlobalUnlock(h);
		}
		Attach(h);
		return (h != NULL);
	}

	bool CopyFromHDEVMODE(HANDLE hdm)
	{
		bool b = false;
		if (hdm != NULL)
		{
			DEVMODE* pdm = (DEVMODE*)::GlobalLock(hdm);
			b = CopyFromDEVMODE(pdm);
			::GlobalUnlock(hdm);
		}
		return b;
	}

	HANDLE CopyToHDEVMODE()
	{
		if ((m_hDevMode == NULL) || (m_pDevMode == NULL))
			return NULL;
		int nSize = m_pDevMode->dmSize + m_pDevMode->dmDriverExtra;
		HANDLE h = ::GlobalAlloc(GMEM_MOVEABLE, nSize);
		if (h != NULL)
		{
			void* p = ::GlobalLock(h);
			ATL::Checked::memcpy_s(p, nSize, m_pDevMode, nSize);
			::GlobalUnlock(h);
		}
		return h;
	}

	// If this devmode was for another printer, this will create a new devmode
	// based on the existing devmode, but retargeted at the new printer
	bool UpdateForNewPrinter(HANDLE hPrinter)
	{
		bool bRet = false;
		LONG nLen = ::DocumentProperties(NULL, hPrinter, NULL, NULL, NULL, 0);
		ATL::CTempBuffer<DEVMODE, _WTL_STACK_ALLOC_THRESHOLD> buff;
		DEVMODE* pdm = buff.AllocateBytes(nLen);
		if(pdm != NULL)
		{
			memset(pdm, 0, nLen);
			LONG l = ::DocumentProperties(NULL, hPrinter, NULL, pdm, m_pDevMode, DM_IN_BUFFER | DM_OUT_BUFFER);
			if (l == IDOK)
				bRet = CopyFromDEVMODE(pdm);
		}

		return bRet;
	}

	bool DocumentProperties(HANDLE hPrinter, HWND hWnd = NULL)
	{
		CPrinterInfo<1> pi;
		pi.GetPrinterInfo(hPrinter);
		if (hWnd == NULL)
			hWnd = ::GetActiveWindow();

		bool bRet = false;
		LONG nLen = ::DocumentProperties(hWnd, hPrinter, pi.m_pi->pName, NULL, NULL, 0);
		ATL::CTempBuffer<DEVMODE, _WTL_STACK_ALLOC_THRESHOLD> buff;
		DEVMODE* pdm = buff.AllocateBytes(nLen);
		if(pdm != NULL)
		{
			memset(pdm, 0, nLen);
			LONG l = ::DocumentProperties(hWnd, hPrinter, pi.m_pi->pName, pdm, m_pDevMode, DM_IN_BUFFER | DM_OUT_BUFFER | DM_PROMPT);
			if (l == IDOK)
				bRet = CopyFromDEVMODE(pdm);
		}

		return bRet;
	}

	operator HANDLE() const { return m_hDevMode; }

	operator DEVMODE*() const { return m_pDevMode; }

// Implementation
	void Cleanup()
	{
		if (m_hDevMode != NULL)
		{
			::GlobalUnlock(m_hDevMode);
			if(t_bManaged)
				::GlobalFree(m_hDevMode);
			m_hDevMode = NULL;
		}
	}
};

typedef CDevModeT<false>   CDevModeHandle;
typedef CDevModeT<true>    CDevMode;


///////////////////////////////////////////////////////////////////////////////
// CPrinterDC

class CPrinterDC : public CDC
{
public:
// Constructors/destructor
	CPrinterDC()
	{
		CPrinter printer;
		printer.OpenDefaultPrinter();
		Attach(printer.CreatePrinterDC());
		ATLASSERT(m_hDC != NULL);
	}

	CPrinterDC(HANDLE hPrinter, const DEVMODE* pdm = NULL)
	{
		CPrinterHandle p;
		p.Attach(hPrinter);
		Attach(p.CreatePrinterDC(pdm));
		ATLASSERT(m_hDC != NULL);
	}

	~CPrinterDC()
	{
		DeleteDC();
	}
};


///////////////////////////////////////////////////////////////////////////////
// CPrintJob - Wraps a set of tasks for a specific printer (StartDoc/EndDoc)
//             Handles aborting, background printing

// Defines callbacks used by CPrintJob (not a COM interface)
class ATL_NO_VTABLE IPrintJobInfo
{
public:
	virtual void BeginPrintJob(HDC hDC) = 0;                // allocate handles needed, etc.
	virtual void EndPrintJob(HDC hDC, bool bAborted) = 0;   // free handles, etc.
	virtual void PrePrintPage(UINT nPage, HDC hDC) = 0;
	virtual bool PrintPage(UINT nPage, HDC hDC) = 0;
	virtual void PostPrintPage(UINT nPage, HDC hDC) = 0;
	// If you want per page devmodes, return the DEVMODE* to use for nPage.
	// You can optimize by only returning a new DEVMODE* when it is different
	// from the one for nLastPage, otherwise return NULL.
	// When nLastPage==0, the current DEVMODE* will be the default passed to
	// StartPrintJob.
	// Note: During print preview, nLastPage will always be "0".
	virtual DEVMODE* GetNewDevModeForPage(UINT nLastPage, UINT nPage) = 0;
	virtual bool IsValidPage(UINT nPage) = 0;
};

// Provides a default implementatin for IPrintJobInfo
// Typically, MI'd into a document or view class
class ATL_NO_VTABLE CPrintJobInfo : public IPrintJobInfo
{
public:
	virtual void BeginPrintJob(HDC /*hDC*/)   // allocate handles needed, etc
	{
	}

	virtual void EndPrintJob(HDC /*hDC*/, bool /*bAborted*/)   // free handles, etc
	{
	}

	virtual void PrePrintPage(UINT /*nPage*/, HDC hDC)
	{
		m_nPJState = ::SaveDC(hDC);
	}

	virtual bool PrintPage(UINT /*nPage*/, HDC /*hDC*/) = 0;

	virtual void PostPrintPage(UINT /*nPage*/, HDC hDC)
	{
		RestoreDC(hDC, m_nPJState);
	}

	virtual DEVMODE* GetNewDevModeForPage(UINT /*nLastPage*/, UINT /*nPage*/)
	{
		return NULL;
	}

	virtual bool IsValidPage(UINT /*nPage*/)
	{
		return true;
	}

// Implementation - data
	int m_nPJState;
};


class CPrintJob
{
public:
// Data members
	CPrinterHandle m_printer;
	IPrintJobInfo* m_pInfo;
	DEVMODE* m_pDefDevMode;
	DOCINFO m_docinfo;
	int m_nJobID;
	bool m_bCancel;
	bool m_bComplete;
	unsigned long m_nStartPage;
	unsigned long m_nEndPage;

// Constructor/destructor
	CPrintJob() : m_nJobID(0), m_bCancel(false), m_bComplete(true)
	{ }

	~CPrintJob()
	{
		ATLASSERT(IsJobComplete()); // premature destruction?
	}

// Operations
	bool IsJobComplete() const
	{
		return m_bComplete;
	}

	bool StartPrintJob(bool bBackground, HANDLE hPrinter, DEVMODE* pDefaultDevMode,
			IPrintJobInfo* pInfo, LPCTSTR lpszDocName, 
			unsigned long nStartPage, unsigned long nEndPage,
			bool bPrintToFile = false, LPCTSTR lpstrOutputFile = NULL)
	{
		ATLASSERT(m_bComplete); // previous job not done yet?
		if (pInfo == NULL)
			return false;

		memset(&m_docinfo, 0, sizeof(m_docinfo));
		m_docinfo.cbSize = sizeof(m_docinfo);
		m_docinfo.lpszDocName = lpszDocName;
		m_pInfo = pInfo;
		m_nStartPage = nStartPage;
		m_nEndPage = nEndPage;
		m_printer.Attach(hPrinter);
		m_pDefDevMode = pDefaultDevMode;
		m_bComplete = false;

		if(bPrintToFile)
			m_docinfo.lpszOutput = (lpstrOutputFile != NULL) ? lpstrOutputFile : _T("FILE:");

		if (!bBackground)
		{
			m_bComplete = true;
			return StartHelper();
		}

		// Create a thread and return
		DWORD dwThreadID = 0;
#ifdef _MT
		HANDLE hThread = (HANDLE)_beginthreadex(NULL, 0, (UINT (WINAPI*)(void*))StartProc, this, 0, (UINT*)&dwThreadID);
#else
		HANDLE hThread = ::CreateThread(NULL, 0, StartProc, (void*)this, 0, &dwThreadID);
#endif
		if (hThread == NULL)
			return false;

		::CloseHandle(hThread);

		return true;
	}

// Implementation
	static DWORD WINAPI StartProc(void* p)
	{
		CPrintJob* pThis = (CPrintJob*)p;
		pThis->StartHelper();
		pThis->m_bComplete = true;
		return 0;
	}

	bool StartHelper()
	{
		CDC dcPrinter;
		dcPrinter.Attach(m_printer.CreatePrinterDC(m_pDefDevMode));
		if (dcPrinter.IsNull())
			return false;
			
		m_nJobID = ::StartDoc(dcPrinter, &m_docinfo);
		if (m_nJobID <= 0)
			return false;

		m_pInfo->BeginPrintJob(dcPrinter);

		// print all the pages now
		unsigned long nLastPage = 0;
		for (unsigned long nPage = m_nStartPage; nPage <= m_nEndPage; nPage++)
		{
			if (!m_pInfo->IsValidPage(nPage))
				break;
			DEVMODE* pdm = m_pInfo->GetNewDevModeForPage(nLastPage, nPage);
			if (pdm != NULL)
				dcPrinter.ResetDC(pdm);
			dcPrinter.StartPage();
			m_pInfo->PrePrintPage(nPage, dcPrinter);
			if (!m_pInfo->PrintPage(nPage, dcPrinter))
				m_bCancel = true;
			m_pInfo->PostPrintPage(nPage, dcPrinter);
			dcPrinter.EndPage();
			if (m_bCancel)
				break;
			nLastPage = nPage;
		}

		m_pInfo->EndPrintJob(dcPrinter, m_bCancel);
		if (m_bCancel)
			::AbortDoc(dcPrinter);
		else
			::EndDoc(dcPrinter);
		m_nJobID = 0;
		return true;
	}

	// Cancels a print job. Can be called asynchronously.
	void CancelPrintJob()
	{
		m_bCancel = true;
	}
};


///////////////////////////////////////////////////////////////////////////////
// CPrintPreview - Adds print preview support to an existing window

class CPrintPreview
{
public:
// Data members
	IPrintJobInfo* m_pInfo;
	CPrinterHandle m_printer;
	CEnhMetaFile m_meta;
	DEVMODE* m_pDefDevMode;
	DEVMODE* m_pCurDevMode;
	SIZE m_sizeCurPhysOffset;

// Constructor
	CPrintPreview() : m_pInfo(NULL), m_pDefDevMode(NULL), m_pCurDevMode(NULL)
	{
		m_sizeCurPhysOffset.cx = 0;
		m_sizeCurPhysOffset.cy = 0;
	}

// Operations
	void SetPrintPreviewInfo(HANDLE hPrinter, DEVMODE* pDefaultDevMode, IPrintJobInfo* pji)
	{
		m_printer.Attach(hPrinter);
		m_pDefDevMode = pDefaultDevMode;
		m_pInfo = pji;
		m_nCurPage = 0;
		m_pCurDevMode = NULL;
	}

	void SetEnhMetaFile(HENHMETAFILE hEMF)
	{
		m_meta = hEMF;
	}

	void SetPage(int nPage)
	{
		if (!m_pInfo->IsValidPage(nPage))
			return;
		m_nCurPage = nPage;
		m_pCurDevMode = m_pInfo->GetNewDevModeForPage(0, nPage);
		if (m_pCurDevMode == NULL)
			m_pCurDevMode = m_pDefDevMode;
		CDC dcPrinter = m_printer.CreatePrinterDC(m_pCurDevMode);

		int iWidth = dcPrinter.GetDeviceCaps(PHYSICALWIDTH); 
		int iHeight = dcPrinter.GetDeviceCaps(PHYSICALHEIGHT); 
		int nLogx = dcPrinter.GetDeviceCaps(LOGPIXELSX);
		int nLogy = dcPrinter.GetDeviceCaps(LOGPIXELSY);

		RECT rcMM = { 0, 0, ::MulDiv(iWidth, 2540, nLogx), ::MulDiv(iHeight, 2540, nLogy) };

		m_sizeCurPhysOffset.cx = dcPrinter.GetDeviceCaps(PHYSICALOFFSETX);
		m_sizeCurPhysOffset.cy = dcPrinter.GetDeviceCaps(PHYSICALOFFSETY);
		
		CEnhMetaFileDC dcMeta(dcPrinter, &rcMM);
		m_pInfo->PrePrintPage(nPage, dcMeta);
		m_pInfo->PrintPage(nPage, dcMeta);
		m_pInfo->PostPrintPage(nPage, dcMeta);
		m_meta.Attach(dcMeta.Close());
	}

	void GetPageRect(RECT& rc, LPRECT prc)
	{
		int x1 = rc.right-rc.left;
		int y1 = rc.bottom - rc.top;
		if ((x1 < 0) || (y1 < 0))
			return;

		CEnhMetaFileInfo emfinfo(m_meta);
		ENHMETAHEADER* pmh = emfinfo.GetEnhMetaFileHeader();

		// Compute whether we are OK vertically or horizontally
		int x2 = pmh->szlDevice.cx;
		int y2 = pmh->szlDevice.cy;
		int y1p = MulDiv(x1, y2, x2);
		int x1p = MulDiv(y1, x2, y2);
		ATLASSERT((x1p <= x1) || (y1p <= y1));
		if (x1p <= x1)
		{
			prc->left = rc.left + (x1 - x1p) / 2;
			prc->right = prc->left + x1p;
			prc->top = rc.top;
			prc->bottom = rc.bottom;
		}
		else
		{
			prc->left = rc.left;
			prc->right = rc.right;
			prc->top = rc.top + (y1 - y1p) / 2;
			prc->bottom = prc->top + y1p;
		}
	}

// Painting helpers
	void DoPaint(CDCHandle dc)
	{
		// this one is not used
	}

	void DoPaint(CDCHandle dc, RECT& rc)
	{
		CEnhMetaFileInfo emfinfo(m_meta);
		ENHMETAHEADER* pmh = emfinfo.GetEnhMetaFileHeader();
		int nOffsetX = MulDiv(m_sizeCurPhysOffset.cx, rc.right-rc.left, pmh->szlDevice.cx);
		int nOffsetY = MulDiv(m_sizeCurPhysOffset.cy, rc.bottom-rc.top, pmh->szlDevice.cy);

		dc.OffsetWindowOrg(-nOffsetX, -nOffsetY);
		dc.PlayMetaFile(m_meta, &rc);
	}

// Implementation - data
	int m_nCurPage;
};


///////////////////////////////////////////////////////////////////////////////
// CPrintPreviewWindow - Implements a print preview window

template <class T, class TBase = ATL::CWindow, class TWinTraits = ATL::CControlWinTraits>
class ATL_NO_VTABLE CPrintPreviewWindowImpl : public ATL::CWindowImpl<T, TBase, TWinTraits>, public CPrintPreview
{
public:
	DECLARE_WND_CLASS_EX2(NULL, T, CS_VREDRAW | CS_HREDRAW, -1)

	enum { m_cxOffset = 10, m_cyOffset = 10 };

// Constructor
	CPrintPreviewWindowImpl() : m_nMinPage(0), m_nMaxPage(0)
	{ }

// Operations
	void SetPrintPreviewInfo(HANDLE hPrinter, DEVMODE* pDefaultDevMode, 
		IPrintJobInfo* pji, int nMinPage, int nMaxPage)
	{
		CPrintPreview::SetPrintPreviewInfo(hPrinter, pDefaultDevMode, pji);
		m_nMinPage = nMinPage;
		m_nMaxPage = nMaxPage;
	}

	bool NextPage()
	{
		if (m_nCurPage == m_nMaxPage)
			return false;
		SetPage(m_nCurPage + 1);
		this->Invalidate();
		return true;
	}

	bool PrevPage()
	{
		if (m_nCurPage == m_nMinPage)
			return false;
		if (m_nCurPage == 0)
			return false;
		SetPage(m_nCurPage - 1);
		this->Invalidate();
		return true;
	}

// Message map and handlers
	BEGIN_MSG_MAP(CPrintPreviewWindowImpl)
		MESSAGE_HANDLER(WM_ERASEBKGND, OnEraseBkgnd)
		MESSAGE_HANDLER(WM_PAINT, OnPaint)
		MESSAGE_HANDLER(WM_PRINTCLIENT, OnPaint)
	END_MSG_MAP()

	LRESULT OnEraseBkgnd(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		return 1;   // no need for the background
	}

	LRESULT OnPaint(UINT /*uMsg*/, WPARAM wParam, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		T* pT = static_cast<T*>(this);
		RECT rc = {};

		if(wParam != NULL)
		{
			pT->DoPrePaint((HDC)wParam, rc);
			pT->DoPaint((HDC)wParam, rc);
		}
		else
		{
			CPaintDC dc(this->m_hWnd);
			pT->DoPrePaint(dc.m_hDC, rc);
			pT->DoPaint(dc.m_hDC, rc);
		}

		return 0;
	}

// Painting helper
	void DoPrePaint(CDCHandle dc, RECT& rc)
	{
		RECT rcClient = {};
		this->GetClientRect(&rcClient);
		RECT rcArea = rcClient;
		T* pT = static_cast<T*>(this);
		(void)pT;   // avoid level 4 warning
		::InflateRect(&rcArea, -pT->m_cxOffset, -pT->m_cyOffset);
		if (rcArea.left > rcArea.right)
			rcArea.right = rcArea.left;
		if (rcArea.top > rcArea.bottom)
			rcArea.bottom = rcArea.top;
		GetPageRect(rcArea, &rc);
		CRgn rgn1, rgn2;
		rgn1.CreateRectRgnIndirect(&rc);
		rgn2.CreateRectRgnIndirect(&rcClient);
		rgn2.CombineRgn(rgn1, RGN_DIFF);
		dc.SelectClipRgn(rgn2);
		dc.FillRect(&rcClient, COLOR_BTNSHADOW);
		dc.SelectClipRgn(NULL);
		dc.FillRect(&rc, (HBRUSH)::GetStockObject(WHITE_BRUSH));
	}

// Implementation - data
	int m_nMinPage;
	int m_nMaxPage;
};


class CPrintPreviewWindow : public CPrintPreviewWindowImpl<CPrintPreviewWindow>
{
public:
	DECLARE_WND_CLASS_EX(_T("WTL_PrintPreview"), CS_VREDRAW | CS_HREDRAW, -1)
};


///////////////////////////////////////////////////////////////////////////////
// CZoomPrintPreviewWindowImpl - Implements print preview window with zooming

#ifdef __ATLSCRL_H__

template <class T, class TBase = ATL::CWindow, class TWinTraits = ATL::CControlWinTraits>
class ATL_NO_VTABLE CZoomPrintPreviewWindowImpl : public CPrintPreviewWindowImpl< T, TBase, TWinTraits >, public CZoomScrollImpl< T >
{
public:
	bool m_bSized;

	CZoomPrintPreviewWindowImpl()  
	{
		this->SetScrollExtendedStyle(SCRL_DISABLENOSCROLL);
		InitZoom();
	}

	// should be called to reset data members before recreating window 
	void InitZoom()
	{
		m_bSized = false;	
		this->m_nZoomMode = ZOOMMODE_OFF;
		this->m_fZoomScaleMin = 1.0;
		this->m_fZoomScale = 1.0;
	}

	BEGIN_MSG_MAP(CZoomPrintPreviewWindowImpl)
		MESSAGE_HANDLER(WM_SETCURSOR, CZoomScrollImpl< T >::OnSetCursor)
		MESSAGE_HANDLER(WM_VSCROLL, CScrollImpl< T >::OnVScroll)
		MESSAGE_HANDLER(WM_HSCROLL, CScrollImpl< T >::OnHScroll)
		MESSAGE_HANDLER(WM_MOUSEWHEEL, CScrollImpl< T >::OnMouseWheel)
		MESSAGE_HANDLER(WM_MOUSEHWHEEL, CScrollImpl< T >::OnMouseHWheel)
		MESSAGE_HANDLER(WM_SETTINGCHANGE, CScrollImpl< T >::OnSettingChange)
		MESSAGE_HANDLER(WM_LBUTTONDOWN, CZoomScrollImpl< T >::OnLButtonDown)
		MESSAGE_HANDLER(WM_MOUSEMOVE, CZoomScrollImpl< T >::OnMouseMove)
		MESSAGE_HANDLER(WM_LBUTTONUP, CZoomScrollImpl< T >::OnLButtonUp)
		MESSAGE_HANDLER(WM_CAPTURECHANGED, CZoomScrollImpl< T >::OnCaptureChanged)
		MESSAGE_HANDLER(WM_SIZE, OnSize)
		MESSAGE_HANDLER(WM_ERASEBKGND, OnEraseBkgnd)
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
	
	LRESULT OnSize(UINT uMsg, WPARAM wParam, LPARAM lParam, BOOL& bHandled)
	{
		SIZE sizeClient = {GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam)};
		POINT ptOffset = this->m_ptOffset;
		SIZE sizeAll = this->m_sizeAll;
		this->SetScrollSize(sizeClient);
		if(sizeAll.cx > 0)
			ptOffset.x = ::MulDiv(ptOffset.x, this->m_sizeAll.cx, sizeAll.cx);
		if(sizeAll.cy > 0)
			ptOffset.y = ::MulDiv(ptOffset.y, this->m_sizeAll.cy, sizeAll.cy);
		this->SetScrollOffset(ptOffset);
		CScrollImpl< T >::OnSize(uMsg, wParam, lParam, bHandled);
		if(!m_bSized)
		{
			m_bSized = true;
			T* pT = static_cast<T*>(this);
			pT->ShowScrollBar(SB_HORZ, TRUE);
			pT->ShowScrollBar(SB_VERT, TRUE);
		}
		return 0;
	}

	LRESULT OnEraseBkgnd(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		return 1;
	}

	LRESULT OnPaint(UINT /*uMsg*/, WPARAM wParam, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		T* pT = static_cast<T*>(this);
		RECT rc = {};

		if(wParam != NULL)
		{
			CDCHandle dc = (HDC)wParam;
			int nMapModeSav = dc.GetMapMode();
			dc.SetMapMode(MM_ANISOTROPIC);
			SIZE szWindowExt = { 0, 0 };
			dc.SetWindowExt(this->m_sizeLogAll, &szWindowExt);
			SIZE szViewportExt = { 0, 0 };
			dc.SetViewportExt(this->m_sizeAll, &szViewportExt);
			POINT ptViewportOrg = { 0, 0 };
			dc.SetViewportOrg(-this->m_ptOffset.x, -this->m_ptOffset.y, &ptViewportOrg);

			pT->DoPrePaint(dc, rc);
			pT->DoPaint(dc, rc);

			dc.SetMapMode(nMapModeSav);
			dc.SetWindowExt(szWindowExt);
			dc.SetViewportExt(szViewportExt);
			dc.SetViewportOrg(ptViewportOrg);
		}
		else
		{
			CPaintDC dc(pT->m_hWnd);
			pT->PrepareDC(dc.m_hDC);
			pT->DoPrePaint(dc.m_hDC, rc);
			pT->DoPaint(dc.m_hDC, rc);
		}

		return 0;
	}

	// Painting helpers
	void DoPaint(CDCHandle dc)
	{
		// this one is not used
	}

	void DoPrePaint(CDCHandle dc, RECT& rc)
	{
		RECT rcClient = {};
		this->GetClientRect(&rcClient);
		RECT rcArea = rcClient;
		T* pT = static_cast<T*>(this);
		(void)pT;   // avoid level 4 warning
		::InflateRect(&rcArea, -pT->m_cxOffset, -pT->m_cyOffset);
		if (rcArea.left > rcArea.right)
			rcArea.right = rcArea.left;
		if (rcArea.top > rcArea.bottom)
			rcArea.bottom = rcArea.top;
		this->GetPageRect(rcArea, &rc);
		HBRUSH hbrOld = dc.SelectBrush(::GetSysColorBrush(COLOR_BTNSHADOW));
		dc.PatBlt(rcClient.left, rcClient.top, rc.left - rcClient.left, rcClient.bottom - rcClient.top, PATCOPY);
		dc.PatBlt(rc.left, rcClient.top, rc.right - rc.left, rc.top - rcClient.top, PATCOPY);
		dc.PatBlt(rc.right, rcClient.top, rcClient.right - rc.right, rcClient.bottom - rcClient.top, PATCOPY);
		dc.PatBlt(rc.left, rc.bottom, rc.right - rc.left, rcClient.bottom - rc.bottom, PATCOPY);
		dc.SelectBrush((HBRUSH)::GetStockObject(WHITE_BRUSH));
		dc.PatBlt(rc.left, rc.top, rc.right - rc.left, rc.bottom - rc.top, PATCOPY);
		dc.SelectBrush(::GetSysColorBrush(COLOR_3DDKSHADOW));
		dc.PatBlt(rc.right, rc.top + 4, 4, rc.bottom - rc.top, PATCOPY);
		dc.PatBlt(rc.left + 4, rc.bottom, rc.right - rc.left, 4, PATCOPY);
		dc.SelectBrush(hbrOld);
	}

	void DoPaint(CDCHandle dc, RECT& rc)
	{
		CEnhMetaFileInfo emfinfo(this->m_meta);
		ENHMETAHEADER* pmh = emfinfo.GetEnhMetaFileHeader();
		int nOffsetX = MulDiv(this->m_sizeCurPhysOffset.cx, rc.right-rc.left, pmh->szlDevice.cx);
		int nOffsetY = MulDiv(this->m_sizeCurPhysOffset.cy, rc.bottom-rc.top, pmh->szlDevice.cy);

		dc.OffsetWindowOrg(-nOffsetX, -nOffsetY);
		dc.PlayMetaFile(this->m_meta, &rc);
	}
};

class CZoomPrintPreviewWindow : public CZoomPrintPreviewWindowImpl<CZoomPrintPreviewWindow>
{
public:
	DECLARE_WND_CLASS_EX(_T("WTL_ZoomPrintPreview"), CS_VREDRAW | CS_HREDRAW, -1)
};

#endif // __ATLSCRL_H__

} // namespace WTL

#endif // __ATLPRINT_H__
