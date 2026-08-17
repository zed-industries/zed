// Windows Template Library - WTL version 10.0
// Copyright (C) Microsoft Corporation, WTL Team. All rights reserved.
//
// This file is a part of the Windows Template Library.
// The use and distribution terms for this software are covered by the
// Microsoft Public License (http://opensource.org/licenses/MS-PL)
// which can be found in the file MS-PL.txt at the root folder.

#ifndef __ATLRIBBON_H__
#define __ATLRIBBON_H__

#pragma once

#if (_MSC_VER < 1500)
	#error atlribbon.h requires Visual C++ 2008 compiler or higher
#endif

#ifndef _UNICODE
	#error atlribbon.h requires the Unicode character set
#endif

#if !defined(NTDDI_WIN7) || (NTDDI_VERSION < NTDDI_WIN7)
	#error atlribbon.h requires the Windows 7 SDK or higher
#endif

#ifndef __ATLAPP_H__
	#error atlribbon.h requires atlapp.h to be included first
#endif

#include <atlmisc.h>    // for RecentDocumentList classes
#include <atlframe.h>   // for Frame and UpdateUI classes
#include <atlctrls.h>   // required for atlctrlw.h
#include <atlctrlw.h>   // for CCommandBarCtrl

#ifndef __ATLSTR_H__
  #pragma warning(push)
  #pragma warning(disable: 4530)   // unwind semantics not enabled
  #include <string>
  #pragma warning(pop)
#endif

#include <dwmapi.h>
#pragma comment(lib, "dwmapi.lib")

#include "UIRibbon.h"
#include "UIRibbonPropertyHelpers.h"
#pragma comment(lib, "propsys.lib")

#include <Richedit.h>   // for CHARFORMAT2


///////////////////////////////////////////////////////////////////////////////
// Classes in this file:
//
// CRibbonUpdateUI : Automatic mapping of ribbon UI elements
//
// RibbonUI::Text
// RibbonUI::CharFormat
// RibbonUI::ICtrl
// RibbonUI::CtrlImpl
// RibbonUI::CommandCtrlImpl
// RibbonUI::ItemProperty
// RibbonUI::CollectionImplBase
// RibbonUI::CollectionImpl
// RibbonUI::TextCollectionImpl
// RibbonUI::ItemCollectionImpl
// RibbonUI::ComboCollectionImpl
// RibbonUI::CommandCollectionImpl
// RibbonUI::ToolbarCollectionImpl
// RibbonUI::SimpleCollectionImpl
// RibbonUI::CollectionCtrlImpl
// RibbonUI::ToolbarGalleryCtrlImpl
// RibbonUI::SimpleCollectionCtrlImpl
// RibbonUI::RecentItemsCtrlImpl
// RibbonUI::FontCtrlImpl
// RibbonUI::ColorCtrlImpl
// RibbonUI::SpinnerCtrlImpl
//
// RibbonUI::CRibbonImpl
//	 CRibbonImpl::CRibbonComboCtrl
//	 CRibbonImpl::CRibbonItemGalleryCtrl
//	 CRibbonImpl::CRibbonCommandGalleryCtrl
//	 CRibbonImpl::CRibbonToolbarGalleryCtrl
//	 CRibbonImpl::CRibbonSimpleComboCtrl
//	 CRibbonImpl::CRibbonSimpleGalleryCtrl
//	 CRibbonImpl::CRibbonRecentItemsCtrl
//	 CRibbonImpl::CRibbonColorCtrl
//	 CRibbonImpl::CRibbonFontCtrl
//	 CRibbonImpl::CRibbonSpinnerCtrl
//	 CRibbonImpl::CRibbonFloatSpinnerCtrl
//	 CRibbonImpl::CRibbonCommandCtrl
//
// CRibbonFrameWindowImplBase
// CRibbonFrameWindowImpl
// CRibbonMDIFrameWindowImpl
// CRibbonPersist
//
// Global functions:
//   RibbonUI::SetPropertyVal()
//   RibbonUI::GetImage()


// Constants

#ifndef RIBBONUI_MAX_TEXT
  #define RIBBONUI_MAX_TEXT 128
#endif

#define TWIPS_PER_POINT 20   // For font size


namespace WTL
{

///////////////////////////////////////////////////////////////////////////////
// CRibbonUpdateUI : Automatic mapping of ribbon UI elements

template <class T>
class CRibbonUpdateUI : public CAutoUpdateUI<T>
{
public:
	enum
	{
		UPDUI_RIBBON = 0x0080, 
		UPDUI_PERSIST = 0x0020
	};

	bool IsRibbonElement(const CUpdateUIBase::_AtlUpdateUIMap& UIMap)
	{
		return (UIMap.m_wType & UPDUI_RIBBON) != 0;
	}

	bool IsRibbonID(UINT nID)
	{
		for(int i = 0; i < this->m_arrUIMap.GetSize(); i++)
		{
			if(this->m_arrUIMap[i].m_nID == nID)
				return IsRibbonElement(this->m_arrUIMap[i]);
		}

		return false;
	}

// Element
	bool UIAddRibbonElement(UINT nID)
	{
		return this->UIAddElement<UPDUI_RIBBON>(nID);
	}

	bool UIRemoveRibbonElement(UINT nID)
	{
		return this->UIRemoveElement<UPDUI_RIBBON>(nID);
	}

	bool UIPersistElement(UINT nID, bool bPersist = true)
	{
		return bPersist ?
			this->UIAddElement<UPDUI_PERSIST>(nID) :
			this->UIRemoveElement<UPDUI_PERSIST>(nID);
	}

// methods for Ribbon elements
	BOOL UISetText(int nID, LPCWSTR sText, BOOL bForceUpdate = FALSE)
	{
		T* pT = static_cast<T*>(this);
		BOOL bRes = CUpdateUIBase::UISetText(nID, sText, bForceUpdate);
		if (pT->IsRibbonUI() && IsRibbonID(nID))
			bRes = SUCCEEDED(pT->InvalidateProperty(nID, UI_PKEY_Label));
		return bRes;
	}

	BOOL UISetText(int nID, UINT uIdResource, BOOL bForceUpdate = FALSE)
	{
		ATL::CTempBuffer<WCHAR> sText(RIBBONUI_MAX_TEXT);
		int nRet = ATL::AtlLoadString(uIdResource, sText, RIBBONUI_MAX_TEXT);
		if(nRet > 0)
			UISetText(nID, sText, bForceUpdate);
		return (nRet > 0) ? TRUE : FALSE;
	}

	LPCTSTR UIGetText(int nID)
	{
		T* pT = static_cast<T*>(this);
		LPCTSTR sUI = CAutoUpdateUI<T>::UIGetText(nID);
		
		// replace 'tab' by 'space' for RibbonUI elements
		if (sUI && pT->IsRibbonUI() && IsRibbonID(nID) && wcschr(sUI, L'\t'))
		{
			static WCHAR sText[RIBBONUI_MAX_TEXT] = {};
			wcscpy_s(sText, sUI);
			WCHAR* pch = wcschr(sText, L'\t');
			if (pch != NULL)
				*pch = L' ';
			return sText;
		}
		else
		{
			return sUI;
		}
	}

	BOOL UIEnable(int nID, BOOL bEnable, BOOL bForceUpdate = FALSE)
	{
		T* pT = static_cast<T*>(this);
		BOOL bRes = CUpdateUIBase::UIEnable(nID, bEnable, bForceUpdate);
		if (pT->IsRibbonUI() && IsRibbonID(nID))
			bRes = SUCCEEDED(pT->SetProperty((WORD)nID, UI_PKEY_Enabled, bEnable));
		return bRes;
	}

	BOOL UISetCheck(int nID, INT nCheck, BOOL bForceUpdate = FALSE)
	{
		if ((nCheck == 0) || (nCheck == 1))
			return UISetCheck(nID, nCheck != 0, bForceUpdate);
		else
			return CUpdateUIBase::UISetCheck(nID, nCheck, bForceUpdate);
	}

	BOOL UISetCheck(int nID, bool bCheck, BOOL bForceUpdate = FALSE)
	{
		T* pT = static_cast<T*>(this);
		BOOL bRes = CUpdateUIBase::UISetCheck(nID, bCheck, bForceUpdate);
		if (bRes && pT->IsRibbonUI() && IsRibbonID(nID))
			bRes = SUCCEEDED(pT->SetProperty((WORD)nID, UI_PKEY_BooleanValue, bCheck));
		return bRes;
	}
};


///////////////////////////////////////////////////////////////////////////////
// RibbonUI namespace
//

namespace RibbonUI
{

// Minimal string allocation support for various PROPERTYKEY values
#ifdef __ATLSTR_H__
  typedef ATL::CString Text;
#else
  class Text : public std::wstring
  {
  public:
	Text(std::wstring& s) : std::wstring(s)
	{ }
	Text(LPCWSTR s) : std::wstring(s)
	{ }
	Text()
	{ }
	bool IsEmpty()
	{
		return empty();
	}
	operator LPCWSTR()
	{
		return c_str();
	}
	Text& operator =(LPCWSTR s)
	{
		return static_cast<Text&>(std::wstring::operator =(s));
	}
  };
#endif // __ATLSTR_H__

// PROPERTYKEY enum and helpers
enum k_KEY
{
	// state
	k_Enabled = 1, k_BooleanValue = 200, 
	// text properties
	k_LabelDescription = 2, k_Keytip = 3, k_Label = 4, k_TooltipDescription = 5, k_TooltipTitle = 6, 
	// image properties
	k_LargeImage = 7, k_LargeHighContrastImage = 8, k_SmallImage = 9, k_SmallHighContrastImage = 10,
	// collection properties
	k_ItemsSource = 101, k_Categories = 102, k_SelectedItem = 104,
	// collection item properties
	k_CommandId = 100, k_CategoryId = 103, k_CommandType = 105, k_ItemImage = 106,
	// combo control property
	k_StringValue = 202,
	// spinner control properties
	k_DecimalValue = 201, k_MaxValue = 203, k_MinValue, k_Increment, k_DecimalPlaces, k_FormatString, k_RepresentativeString = 208,
	// font control properties
	k_FontProperties = 300, k_FontProperties_Family, k_FontProperties_Size, k_FontProperties_Bold, k_FontProperties_Italic = 304, 
	k_FontProperties_Underline = 305, k_FontProperties_Strikethrough, k_FontProperties_VerticalPositioning, k_FontProperties_ForegroundColor = 308, 
	k_FontProperties_BackgroundColor = 309, k_FontProperties_ForegroundColorType, k_FontProperties_BackgroundColorType, k_FontProperties_ChangedProperties = 312, 
	k_FontProperties_DeltaSize = 313, 
	// recent items properties
	k_RecentItems = 350, k_Pinned = 351,
	// color control properties
	k_Color = 400, k_ColorType = 401, k_ColorMode, 
	k_ThemeColorsCategoryLabel = 403, k_StandardColorsCategoryLabel, k_RecentColorsCategoryLabel = 405, k_AutomaticColorLabel = 406, 
	k_NoColorLabel = 407, k_MoreColorsLabel = 408, 
	k_ThemeColors = 409, k_StandardColors = 410, k_ThemeColorsTooltips = 411, k_StandardColorsTooltips = 412,
	// Ribbon state
	k_Viewable = 1000, k_Minimized = 1001, k_QuickAccessToolbarDock = 1002, k_ContextAvailable = 1100,
	// Ribbon UI colors
	k_GlobalBackgroundColor = 2000, k_GlobalHighlightColor, k_GlobalTextColor = 2002
};

inline k_KEY k_(REFPROPERTYKEY key)
{
	return (k_KEY)key.fmtid.Data1;
}

// PROPERTYKEY value assignment and specializations
//
template <typename V>
HRESULT SetPropertyVal(REFPROPERTYKEY key, V val, PROPVARIANT* ppv)
{
	switch (k_(key))
	{
	case k_Enabled:
	case k_BooleanValue:
		return InitPropVariantFromBoolean(val, ppv);
	default:
		return UIInitPropertyFromUInt32(key, val, ppv);
	}
}

inline HRESULT SetPropertyVal(REFPROPERTYKEY key, DOUBLE val, PROPVARIANT* ppv)
{
	return SetPropertyVal(key, (LONG)val, ppv);
}

inline HRESULT SetPropertyVal(REFPROPERTYKEY key, IUIImage* val, PROPVARIANT* ppv)
{
	HRESULT hr = UIInitPropertyFromImage(key, val, ppv);
	ATLVERIFY(val->Release() == 1);
	return hr;
}

inline HRESULT SetPropertyVal(REFPROPERTYKEY key, IUnknown* val, PROPVARIANT* ppv)
{
	return UIInitPropertyFromInterface(key, val, ppv);
}

inline HRESULT SetPropertyVal(REFPROPERTYKEY key, IPropertyStore* val, PROPVARIANT* ppv)
{
	return UIInitPropertyFromInterface(key, val, ppv);
}

inline HRESULT SetPropertyVal(REFPROPERTYKEY key, SAFEARRAY* val, PROPVARIANT* ppv)
{
	return UIInitPropertyFromIUnknownArray(key, val, ppv);
}

inline HRESULT SetPropertyVal(REFPROPERTYKEY key, DECIMAL* val, PROPVARIANT* ppv)
{
	return UIInitPropertyFromDecimal(key, *val, ppv);
}

inline HRESULT SetPropertyVal(REFPROPERTYKEY key, bool val, PROPVARIANT* ppv)
{
	return UIInitPropertyFromBoolean(key, val, ppv);
}

inline HRESULT SetPropertyVal(REFPROPERTYKEY key, LPCWSTR val, PROPVARIANT* ppv)
{
	return UIInitPropertyFromString(key, val, ppv);
}

// CharFormat helper struct for RibbonUI font control
//
struct CharFormat : CHARFORMAT2
{
	// Default constructor
	CharFormat()
	{
		cbSize = sizeof(CHARFORMAT2);
		Reset();
	}

	// Copy constructor
	CharFormat(const CharFormat& cf)
	{
		::CopyMemory(this, &cf, sizeof(CHARFORMAT2));
	}

	// Assign operator
	CharFormat& operator =(const CharFormat& cf)
	{
		::CopyMemory(this, &cf, sizeof(CHARFORMAT2));
		return (*this);
	}

	void Reset()
	{
		uValue = dwMask = dwEffects = 0;
		PropVariantInit(&propvar);
	}

	void operator <<(IPropertyStore* pStore)
	{
		if (pStore == NULL)
		{
			ATLASSERT(FALSE);
			return;
		}

		static void (CharFormat::*Getk_[])(IPropertyStore*) = 
		{
			&CharFormat::Getk_Family, 
			&CharFormat::Getk_FontProperties_Size, 
			&CharFormat::Getk_MaskEffectBold,
			&CharFormat::Getk_MaskEffectItalic,
			&CharFormat::Getk_MaskEffectUnderline,
			&CharFormat::Getk_MaskEffectStrikeout,
			&CharFormat::Getk_VerticalPositioning,
			&CharFormat::Getk_Color, 
			&CharFormat::Getk_ColorBack, 
			&CharFormat::Getk_ColorType,
			&CharFormat::Getk_ColorTypeBack,
		};

		DWORD nProps = 0;
		Reset();

		ATLVERIFY(SUCCEEDED(pStore->GetCount(&nProps)));
		for (DWORD iProp = 0; iProp < nProps; iProp++)
		{
			PROPERTYKEY key;	
			ATLVERIFY(SUCCEEDED(pStore->GetAt(iProp, &key)));
			ATLASSERT(k_(key) >= k_FontProperties_Family);

			if (k_(key) <= k_FontProperties_BackgroundColorType)
				(this->*Getk_[k_(key) - k_FontProperties_Family])(pStore);
		}
	}

	void operator >>(IPropertyStore* pStore)
	{
		if (pStore == NULL)
		{
			ATLASSERT(FALSE);
			return;
		}

		PutFace(pStore);
		PutSize(pStore);
		PutMaskEffect(CFM_BOLD, CFE_BOLD, UI_PKEY_FontProperties_Bold, pStore);
		PutMaskEffect(CFM_ITALIC, CFE_ITALIC, UI_PKEY_FontProperties_Italic, pStore);
		PutMaskEffect(CFM_UNDERLINE, CFE_UNDERLINE, UI_PKEY_FontProperties_Underline, pStore);
		PutMaskEffect(CFM_STRIKEOUT, CFE_STRIKEOUT, UI_PKEY_FontProperties_Strikethrough, pStore);
		PutVerticalPos(pStore);
		PutColor(pStore);
		PutBackColor(pStore);
	}

private:
	PROPVARIANT propvar;
	UINT uValue;

	// Getk_ functions
	void Getk_Family(IPropertyStore* pStore)
	{
		if (SUCCEEDED(pStore->GetValue(UI_PKEY_FontProperties_Family, &propvar)))
		{
			PropVariantToString(propvar, szFaceName, LF_FACESIZE);
			if (*szFaceName)
				dwMask |= CFM_FACE;
		}
	}

	void Getk_FontProperties_Size(IPropertyStore* pStore)
	{
		if (SUCCEEDED(pStore->GetValue(UI_PKEY_FontProperties_Size, &propvar)))
		{
			DECIMAL decSize = {};
			UIPropertyToDecimal(UI_PKEY_FontProperties_Size, propvar, &decSize);
			DOUBLE dSize = 0;
			VarR8FromDec(&decSize, &dSize);
			if (dSize > 0)
			{
				dwMask |= CFM_SIZE;
				yHeight = (LONG)(dSize * TWIPS_PER_POINT);
			}
		}
	}

	void Getk_MaskEffectBold(IPropertyStore* pStore)
	{
		Getk_MaskEffectAll(pStore, CFM_BOLD, CFE_BOLD, UI_PKEY_FontProperties_Bold);
	}

	void Getk_MaskEffectItalic(IPropertyStore* pStore)
	{
		Getk_MaskEffectAll(pStore, CFM_ITALIC, CFE_ITALIC, UI_PKEY_FontProperties_Italic);
	}

	void Getk_MaskEffectUnderline(IPropertyStore* pStore)
	{
		Getk_MaskEffectAll(pStore, CFM_UNDERLINE, CFE_UNDERLINE, UI_PKEY_FontProperties_Underline);
	}

	void Getk_MaskEffectStrikeout(IPropertyStore* pStore)
	{
		Getk_MaskEffectAll(pStore, CFM_STRIKEOUT, CFE_STRIKEOUT, UI_PKEY_FontProperties_Strikethrough);
	}

	void Getk_MaskEffectAll(IPropertyStore* pStore, DWORD _dwMask, DWORD _dwEffects, REFPROPERTYKEY key)
	{
		if (SUCCEEDED(pStore->GetValue(key, &propvar)))
		{
			UIPropertyToUInt32(key, propvar, &uValue);
			if ((UI_FONTPROPERTIES)uValue != UI_FONTPROPERTIES_NOTAVAILABLE)
			{
				dwMask |= _dwMask;
				dwEffects |= ((UI_FONTPROPERTIES)uValue == UI_FONTPROPERTIES_SET) ? _dwEffects : 0;
			}
		}
	}

	void Getk_VerticalPositioning(IPropertyStore* pStore)
	{
		if (SUCCEEDED(pStore->GetValue(UI_PKEY_FontProperties_VerticalPositioning, &propvar)))
		{
			UIPropertyToUInt32(UI_PKEY_FontProperties_VerticalPositioning, propvar, &uValue);
			UI_FONTVERTICALPOSITION uVerticalPosition = (UI_FONTVERTICALPOSITION) uValue;
			if ((uVerticalPosition != UI_FONTVERTICALPOSITION_NOTAVAILABLE))
			{
				dwMask |= (CFM_SUPERSCRIPT | CFM_SUBSCRIPT);
				if (uVerticalPosition != UI_FONTVERTICALPOSITION_NOTSET)
				{
					dwEffects |= (uVerticalPosition == UI_FONTVERTICALPOSITION_SUPERSCRIPT) ? CFE_SUPERSCRIPT : CFE_SUBSCRIPT;
				}
			}
		}
	}

	void Getk_Color(IPropertyStore* pStore)
	{
		Getk_ColorAll(pStore, CFM_COLOR, UI_PKEY_FontProperties_ForegroundColor);
	}

	void Getk_ColorBack(IPropertyStore* pStore)
	{
		Getk_ColorAll(pStore, CFM_BACKCOLOR, UI_PKEY_FontProperties_BackgroundColor);
	}
		
	void Getk_ColorAll(IPropertyStore* pStore, DWORD _dwMask, REFPROPERTYKEY key)
	{
		UINT32 color = 0;
		if (SUCCEEDED(pStore->GetValue(key, &propvar)))
		{
			UIPropertyToUInt32(key, propvar, &color);
			dwMask |= _dwMask;

			if (_dwMask == CFM_COLOR)
				crTextColor = color;
			else
				crBackColor = color;
		}
	}

	void Getk_ColorType(IPropertyStore* pStore)
	{
		Getk_ColorTypeAll(pStore, CFM_COLOR, CFE_AUTOCOLOR, UI_SWATCHCOLORTYPE_AUTOMATIC, UI_PKEY_FontProperties_ForegroundColor);

	}

	void Getk_ColorTypeBack(IPropertyStore* pStore)
	{
		Getk_ColorTypeAll(pStore, CFM_BACKCOLOR, CFE_AUTOBACKCOLOR, UI_SWATCHCOLORTYPE_NOCOLOR, UI_PKEY_FontProperties_BackgroundColor);
	}

	void Getk_ColorTypeAll(IPropertyStore* pStore, DWORD _dwMask, DWORD _dwEffects, UI_SWATCHCOLORTYPE _type, REFPROPERTYKEY key)
	{
		if (SUCCEEDED(pStore->GetValue(key, &propvar)))
		{
			UIPropertyToUInt32(key, propvar, &uValue);
			if (_type == (UI_SWATCHCOLORTYPE)uValue)
			{
				dwMask |= _dwMask;
				dwEffects |= _dwEffects;
			}
		}
	}

	// Put functions
	void PutMaskEffect(WORD dwMaskVal, WORD dwEffectVal, REFPROPERTYKEY key, IPropertyStore* pStore)
	{
		PROPVARIANT var;
		UI_FONTPROPERTIES uProp = UI_FONTPROPERTIES_NOTAVAILABLE;
		if ((dwMask & dwMaskVal) != 0)
			uProp = dwEffects & dwEffectVal ? UI_FONTPROPERTIES_SET : UI_FONTPROPERTIES_NOTSET;
		SetPropertyVal(key, uProp, &var);
		pStore->SetValue(key, var);
	}

	void PutVerticalPos(IPropertyStore* pStore)
	{
		PROPVARIANT var;
		UI_FONTVERTICALPOSITION uProp = UI_FONTVERTICALPOSITION_NOTAVAILABLE;

		if ((dwMask & CFE_SUBSCRIPT) != 0)
		{
			if ((dwMask & CFM_SUBSCRIPT) && (dwEffects & CFE_SUBSCRIPT))
				uProp = UI_FONTVERTICALPOSITION_SUBSCRIPT;
			else
				uProp = UI_FONTVERTICALPOSITION_SUPERSCRIPT;
		}
		else if ((dwMask & CFM_OFFSET) != 0)
		{
			if (yOffset > 0)
				uProp = UI_FONTVERTICALPOSITION_SUPERSCRIPT;
			else if (yOffset < 0)
				uProp = UI_FONTVERTICALPOSITION_SUBSCRIPT;
		}

		SetPropertyVal(UI_PKEY_FontProperties_VerticalPositioning, uProp, &var);
		pStore->SetValue(UI_PKEY_FontProperties_VerticalPositioning, var);
	}

	void PutFace(IPropertyStore* pStore)
	{
		PROPVARIANT var;
		SetPropertyVal(UI_PKEY_FontProperties_Family, 
			dwMask & CFM_FACE ? szFaceName : L"", &var);
		pStore->SetValue(UI_PKEY_FontProperties_Family, var);
	}

	void PutSize(IPropertyStore* pStore)
	{
		PROPVARIANT var;
		DECIMAL decVal;

		if ((dwMask & CFM_SIZE) != 0)
			VarDecFromR8((DOUBLE)yHeight / TWIPS_PER_POINT, &decVal);
		else
			VarDecFromI4(0, &decVal);

		SetPropertyVal(UI_PKEY_FontProperties_Size, &decVal, &var);
		pStore->SetValue(UI_PKEY_FontProperties_Size, var);
	}

	void PutColor(IPropertyStore* pStore)
	{
		if ((dwMask & CFM_COLOR) != 0)
		{
			if ((dwEffects & CFE_AUTOCOLOR) == 0)
			{
				SetPropertyVal(UI_PKEY_FontProperties_ForegroundColorType, UI_SWATCHCOLORTYPE_RGB, &propvar);
				pStore->SetValue(UI_PKEY_FontProperties_ForegroundColorType, propvar);
				
				SetPropertyVal(UI_PKEY_FontProperties_ForegroundColor, crTextColor, &propvar);
				pStore->SetValue(UI_PKEY_FontProperties_ForegroundColor, propvar);
			}
			else
			{
				SetPropertyVal(UI_PKEY_FontProperties_ForegroundColorType, UI_SWATCHCOLORTYPE_AUTOMATIC, &propvar);
				pStore->SetValue(UI_PKEY_FontProperties_ForegroundColorType, propvar);
			}
		}
	}

	void PutBackColor(IPropertyStore* pStore)
	{
		if (((dwMask & CFM_BACKCOLOR) != 0) && ((dwEffects & CFE_AUTOBACKCOLOR) == 0))
		{
			SetPropertyVal(UI_PKEY_FontProperties_BackgroundColorType, UI_SWATCHCOLORTYPE_RGB, &propvar);
			pStore->SetValue(UI_PKEY_FontProperties_BackgroundColorType, propvar);
				
			SetPropertyVal(UI_PKEY_FontProperties_BackgroundColor, crBackColor, &propvar);
			pStore->SetValue(UI_PKEY_FontProperties_BackgroundColor, propvar);
		}
		else
		{
			SetPropertyVal(UI_PKEY_FontProperties_BackgroundColorType, UI_SWATCHCOLORTYPE_NOCOLOR, &propvar);
			pStore->SetValue(UI_PKEY_FontProperties_BackgroundColorType, propvar);
		}
	}
};

// IUIImage helper
//
inline IUIImage* GetImage(HBITMAP hbm, UI_OWNERSHIP owner)
{
	ATLASSERT(hbm);
	IUIImage* pIUII = NULL;
	ATL::CComPtr<IUIImageFromBitmap> pIFB;

	if SUCCEEDED(pIFB.CoCreateInstance(CLSID_UIRibbonImageFromBitmapFactory))
		ATLVERIFY(SUCCEEDED(pIFB->CreateImage(hbm, owner, &pIUII)));

	return pIUII;
}


///////////////////////////////////////////////////////////////////////////////
// Ribbon control classes

// RibbonUI::ICtrl abstract interface of RibbonUI::CRibbonImpl and all RibbonUI control classes
//
struct ICtrl
{
	virtual HRESULT DoExecute(UINT nCmdID, UI_EXECUTIONVERB verb, 
	                          const PROPERTYKEY* key, const PROPVARIANT* ppropvarValue,
	                          IUISimplePropertySet* pCommandExecutionProperties) = 0;

	virtual HRESULT DoUpdateProperty(UINT nCmdID, REFPROPERTYKEY key, 
	                                 const PROPVARIANT* ppropvarCurrentValue, PROPVARIANT* ppropvarNewValue) = 0;
};

// RibbonUI::CtrlImpl base class for all ribbon controls
//
template <class T, UINT t_ID>
class ATL_NO_VTABLE CtrlImpl : public ICtrl
{
protected:
	T* m_pWndRibbon;

public:
	typedef T WndRibbon;

	CtrlImpl() : m_pWndRibbon(T::pWndRibbon)
	{ }

	virtual ~CtrlImpl()
	{ }

	WndRibbon& GetWndRibbon()
	{
		return *m_pWndRibbon;
	}

	static WORD GetID()
	{
		return t_ID;
	}

	Text m_sTxt[5];

	// Operations
	HRESULT Invalidate()
	{
		return GetWndRibbon().InvalidateCtrl(GetID());
	}

	HRESULT Invalidate(REFPROPERTYKEY key, UI_INVALIDATIONS flags = UI_INVALIDATIONS_PROPERTY)
	{
		return GetWndRibbon().InvalidateProperty(GetID(), key, flags);
	}

	HRESULT SetText(REFPROPERTYKEY key, LPCWSTR sTxt, bool bUpdate = false)
	{
		ATLASSERT((k_(key) <= k_TooltipTitle) && (k_(key) >= k_LabelDescription));

		m_sTxt[k_(key) - k_LabelDescription] = sTxt;

		return bUpdate ?
			GetWndRibbon().InvalidateProperty(GetID(), key) :
			S_OK;
	}

	// Implementation
	template <typename V>
	HRESULT SetProperty(REFPROPERTYKEY key, V val)
	{
		return GetWndRibbon().SetProperty(GetID(), key, val);
	}

	HRESULT OnGetText(REFPROPERTYKEY key, PROPVARIANT* ppv)
	{
		ATLASSERT((k_(key) <= k_TooltipTitle) && (k_(key) >= k_LabelDescription));

		const INT iText = k_(key) - k_LabelDescription;
		if (m_sTxt[iText].IsEmpty())
			if (LPCWSTR sText = GetWndRibbon().OnRibbonQueryText(GetID(), key))
				m_sTxt[iText] = sText;

		return !m_sTxt[iText].IsEmpty() ?
			SetPropertyVal(key, (LPCWSTR)m_sTxt[iText], ppv) :
			S_OK;
	}

	virtual HRESULT DoExecute(UINT nCmdID, UI_EXECUTIONVERB verb, 
	                          const PROPERTYKEY* key, const PROPVARIANT* ppropvarValue,
	                          IUISimplePropertySet* pCommandExecutionProperties)
	{
		ATLASSERT(nCmdID == t_ID);
		return GetWndRibbon().DoExecute(nCmdID, verb, key, ppropvarValue, pCommandExecutionProperties);
	}

	virtual HRESULT DoUpdateProperty(UINT nCmdID, REFPROPERTYKEY key, 
	                                 const PROPVARIANT* ppropvarCurrentValue, PROPVARIANT* ppropvarNewValue)
	{
		ATLASSERT(nCmdID == t_ID);

		const INT iMax = k_TooltipTitle - k_LabelDescription;
		const INT iVal = k_(key) - k_LabelDescription;

		return (iVal <= iMax) && (iVal >= 0) ?
			OnGetText(key, ppropvarNewValue) :
			GetWndRibbon().DoUpdateProperty(nCmdID, key, ppropvarCurrentValue, ppropvarNewValue);
	}
};

// CommandCtrlImpl base class for most ribbon controls
//
template <class T, UINT t_ID>
class CommandCtrlImpl : public CtrlImpl<T, t_ID>
{
public:
	CBitmap m_hbm[4];

	HRESULT SetImage(REFPROPERTYKEY key, HBITMAP hbm, bool bUpdate = false)
	{
		ATLASSERT((k_(key) <= k_SmallHighContrastImage) && (k_(key) >= k_LargeImage));
			
		m_hbm[k_(key) - k_LargeImage].Attach(hbm);

		return bUpdate ?
			this->GetWndRibbon().InvalidateProperty(this->GetID(), key) :
			S_OK;
	}

	HRESULT OnGetImage(REFPROPERTYKEY key, PROPVARIANT* ppv)
	{
		ATLASSERT((k_(key) <= k_SmallHighContrastImage) && (k_(key) >= k_LargeImage));

		const INT iImage = k_(key) - k_LargeImage;

		if (m_hbm[iImage].IsNull())
			m_hbm[iImage] = this->GetWndRibbon().OnRibbonQueryImage(this->GetID(), key);

		return m_hbm[iImage].IsNull() ?
			E_NOTIMPL :
			SetPropertyVal(key, GetImage(m_hbm[iImage], UI_OWNERSHIP_COPY), ppv);
	}

	virtual HRESULT DoUpdateProperty(UINT nCmdID, REFPROPERTYKEY key, 
	                                 const PROPVARIANT* ppropvarCurrentValue, PROPVARIANT* ppropvarNewValue)
	{
		ATLASSERT (nCmdID == this->GetID());

		return (k_(key) <= k_SmallHighContrastImage) && (k_(key) >= k_LargeImage) ?
			OnGetImage(key, ppropvarNewValue) :
			CtrlImpl<T, t_ID>::DoUpdateProperty(nCmdID, key, ppropvarCurrentValue, ppropvarNewValue);
	}
};


///////////////////////////////////////////////////////////////////////////////
// Ribbon collection base classes

// ItemProperty class: ribbon callback for each item in a collection
//

#pragma warning(push)
#pragma warning(disable: 4512)   // assignment operator could not be generated

template <class TCollection>
class ItemProperty : public IUISimplePropertySet
{
public:
	ItemProperty(UINT i, TCollection* pCollection) : m_Index(i), m_pCollection(pCollection)
	{ }

	const UINT m_Index;
	TCollection* m_pCollection;

	// IUISimplePropertySet method.
	STDMETHODIMP GetValue(REFPROPERTYKEY key, PROPVARIANT *value)
	{
		return m_pCollection->OnGetItem(m_Index, key, value);
	}

	// IUnknown methods.
	STDMETHODIMP_(ULONG) AddRef()
	{
		return 1;
	}

	STDMETHODIMP_(ULONG) Release()
	{
		return 1;
	}

	STDMETHODIMP QueryInterface(REFIID iid, void** ppv)
	{
		if ((iid == __uuidof(IUnknown)) || (iid == __uuidof(IUISimplePropertySet)))
		{
			*ppv = this;
			return S_OK;
		}
		else
		{
			return E_NOINTERFACE;
		}
	}
};

#pragma warning(pop)


// CollectionImplBase: base class for all RibbonUI collections
//
template <class TCollection, size_t t_size>
class CollectionImplBase
{
	typedef CollectionImplBase<TCollection, t_size> thisClass;

public:
	CollectionImplBase()
	{
		for (int i = 0; i < t_size; i++)
			m_apItems[i] = new ItemProperty<TCollection>(i, static_cast<TCollection*>(this));
	}

	~CollectionImplBase()
	{
		for (int i = 0; i < t_size; i++)
			delete m_apItems[i];
	}

// Data members
	ItemProperty<TCollection>* m_apItems[t_size];
};

// CollectionImpl: handles categories and collecton resizing
//
template <class TCtrl, size_t t_items, size_t t_categories>
class CollectionImpl : public CollectionImplBase<CollectionImpl<TCtrl, t_items, t_categories>, t_items + t_categories>
{
	typedef CollectionImpl<TCtrl, t_items, t_categories> thisClass;
public:
	typedef thisClass Collection;

	CollectionImpl() : m_size(t_items)
	{
		::FillMemory(m_auItemCat, sizeof(m_auItemCat), 0xff); // UI_COLLECTION_INVALIDINDEX
	}

	UINT32 m_auItemCat[t_items];
	Text m_asCatName[__max(t_categories, 1)];
	size_t m_size;

// Operations
	HRESULT SetItemCategory(UINT uItem, UINT uCat, bool bUpdate = false)
	{
		ATLASSERT((uItem < t_items) && (uCat < t_categories));

		m_auItemCat[uItem] = uCat;

		return bUpdate ? InvalidateItems() : S_OK;
	}

	HRESULT SetCategoryText(UINT uCat, LPCWSTR sText, bool bUpdate = false)
	{
		ATLASSERT(uCat < t_categories);

		m_asCatName[uCat] = sText;

		return bUpdate ? InvalidateCategories() : S_OK;
	}

	HRESULT Resize(size_t size, bool bUpdate = false)
	{
		ATLASSERT(size <= t_items);

		m_size = size;

		return bUpdate ? InvalidateItems() : S_OK;
	}

// Implementation
	HRESULT OnGetItem(UINT uIndex, REFPROPERTYKEY key, PROPVARIANT *value)
	{
		ATLASSERT(uIndex < t_items + t_categories);
		TCtrl* pCtrl = static_cast<TCtrl*>(this);

		return uIndex < t_items ?
			pCtrl->DoGetItem(uIndex, key, value) :
			pCtrl->DoGetCategory(uIndex - t_items, key, value);
	}

	HRESULT DoGetItem(UINT uItem, REFPROPERTYKEY key, PROPVARIANT *value)
	{
		ATLASSERT(k_(key) == k_CategoryId);
		UINT32 uCat = UI_COLLECTION_INVALIDINDEX;

		if (t_categories != 0)
		{
			if (m_auItemCat[uItem] == UI_COLLECTION_INVALIDINDEX)
			{
				typename TCtrl::WndRibbon& ribbon = static_cast<TCtrl*>(this)->GetWndRibbon();
				m_auItemCat[uItem] = ribbon.OnRibbonQueryItemCategory(TCtrl::GetID(), uItem);
			}
			uCat = m_auItemCat[uItem];
		}

		return SetPropertyVal(key, uCat, value);
	}

	HRESULT DoGetCategory(UINT uCat, REFPROPERTYKEY key, PROPVARIANT *value)
	{
		HRESULT hr = S_OK;

		switch (k_(key))
		{
		case k_Label:
			if (m_asCatName[uCat].IsEmpty())
			{
				typename TCtrl::WndRibbon& ribbon = static_cast<TCtrl*>(this)->GetWndRibbon();
				m_asCatName[uCat] = ribbon.OnRibbonQueryCategoryText(TCtrl::GetID(), uCat);
			}
			hr = SetPropertyVal(key, (LPCWSTR)m_asCatName[uCat], value);
			break;
		case k_CategoryId:
			hr = SetPropertyVal(key, uCat, value);
			break;
		default:
			ATLASSERT(FALSE);
			break;
		}

		return hr;
	}

	HRESULT InvalidateItems()
	{
		return static_cast<TCtrl*>(this)->Invalidate(UI_PKEY_ItemsSource);
	}

	HRESULT InvalidateCategories()
	{
		return static_cast<TCtrl*>(this)->Invalidate(UI_PKEY_Categories);
	}

	HRESULT DoUpdateProperty(UINT nCmdID, REFPROPERTYKEY key, 
	                         const PROPVARIANT* ppropvarCurrentValue, PROPVARIANT* /*ppropvarNewValue*/)
	{
		ATLASSERT(nCmdID == TCtrl::GetID());
		(void)nCmdID;   // avoid level 4 warning

		HRESULT hr = E_NOTIMPL;
		switch (k_(key))
		{
		case k_ItemsSource:
			{
				ATL::CComQIPtr<IUICollection> pIUICollection(ppropvarCurrentValue->punkVal);
				ATLASSERT(pIUICollection);
				hr = pIUICollection->Clear();
				for (UINT i = 0; i < m_size; i++)
				{
					if FAILED(hr = pIUICollection->Add(this->m_apItems[i]))
						break;
				}
				ATLASSERT(SUCCEEDED(hr));
			}
			break;
		case k_Categories:
			if (t_categories != 0)
			{
				ATL::CComQIPtr<IUICollection> pIUICategory(ppropvarCurrentValue->punkVal);
				ATLASSERT(pIUICategory.p);
				hr = pIUICategory->Clear();
				for (UINT i = t_items; i < (t_items + t_categories); i++)
				{
					if FAILED(hr = pIUICategory->Add(this->m_apItems[i]))
						break;
				}
				ATLASSERT(SUCCEEDED(hr));
			}
			break;
		}

		return hr;
	}
};

// TextCollectionImpl: handles item labels and selection
//
template <class TCtrl, size_t t_items, size_t t_categories = 0>
class TextCollectionImpl : public CollectionImpl<TCtrl, t_items, t_categories>
{
	typedef TextCollectionImpl<TCtrl, t_items, t_categories> thisClass;
public:
	typedef thisClass TextCollection;

	TextCollectionImpl() : m_uSelected(UI_COLLECTION_INVALIDINDEX)
	{ }

	Text m_asText[t_items];
	UINT m_uSelected;

	// Operations
	HRESULT SetItemText(UINT uItem, LPCWSTR sText, bool bUpdate = false)
	{
		ATLASSERT(uItem < t_items);

		m_asText[uItem] = sText;

		return bUpdate ? this->InvalidateItems() : S_OK;
	}

	UINT GetSelected()
	{
		return m_uSelected;
	}

	HRESULT Select(UINT uItem, bool bUpdate = false)
	{
		ATLASSERT((uItem < t_items) || (uItem == UI_COLLECTION_INVALIDINDEX));

		m_uSelected = uItem;

		typename TCtrl::WndRibbon& ribbon = static_cast<TCtrl*>(this)->GetWndRibbon();
		return bUpdate ?
			ribbon.SetProperty(TCtrl::GetID(), UI_PKEY_SelectedItem, uItem) : 
			S_OK;
	}

// Implementation
 	HRESULT DoGetItem(UINT uItem, REFPROPERTYKEY key, PROPVARIANT *value)
	{
		ATLASSERT(uItem < t_items);

		if (k_(key) == k_Label)
		{
			if (m_asText[uItem].IsEmpty())
			{
				typename TCtrl::WndRibbon& ribbon = static_cast<TCtrl*>(this)->GetWndRibbon();
				m_asText[uItem] = ribbon.OnRibbonQueryItemText(TCtrl::GetID(), uItem);
			}
			return SetPropertyVal(key, (LPCWSTR)m_asText[uItem], value);
		}
		else
		{
			return CollectionImpl<TCtrl, t_items, t_categories>::Collection::DoGetItem(uItem, key, value);
		}
	}

	HRESULT DoUpdateProperty(UINT nCmdID, REFPROPERTYKEY key, 
	                         const PROPVARIANT* ppropvarCurrentValue, PROPVARIANT* ppropvarNewValue)
	{
		ATLASSERT(nCmdID == TCtrl::GetID());

		if (k_(key) == k_SelectedItem)
		{
			typename TCtrl::WndRibbon& ribbon = static_cast<TCtrl*>(this)->GetWndRibbon();
			UINT uSel = UI_COLLECTION_INVALIDINDEX;
			if ((m_uSelected == UI_COLLECTION_INVALIDINDEX) &&
			    ribbon.OnRibbonQuerySelectedItem(TCtrl::GetID(), uSel))
				m_uSelected = uSel;

			return SetPropertyVal(key, m_uSelected, ppropvarNewValue);
		}
		else
		{
			return CollectionImpl<TCtrl, t_items, t_categories>::Collection::DoUpdateProperty(nCmdID, key, ppropvarCurrentValue, ppropvarNewValue);
		}
	}
};

// ItemCollectionImpl: handles item image
//
template <class TCtrl, size_t t_items, size_t t_categories = 0>
class ItemCollectionImpl : public TextCollectionImpl<TCtrl, t_items, t_categories>
{
	typedef ItemCollectionImpl<TCtrl, t_items, t_categories> thisClass;
public:
	typedef thisClass ItemCollection;
	
	ItemCollectionImpl()
	{
		::ZeroMemory(m_aBitmap, sizeof(m_aBitmap));
	}

	CBitmap m_aBitmap[t_items];

	// Operations
	HRESULT SetItemImage(UINT uIndex, HBITMAP hbm, bool bUpdate = false)
	{
		ATLASSERT(uIndex < t_items);

		m_aBitmap[uIndex] = hbm;

		return bUpdate ? this->InvalidateItems() : S_OK;
	}

// Implementation
	HRESULT DoGetItem(UINT uItem, REFPROPERTYKEY key, PROPVARIANT *value)
	{
		ATLASSERT(uItem < t_items);

		if (k_(key) == k_ItemImage)
		{
			if (m_aBitmap[uItem].IsNull())
			{
				typename TCtrl::WndRibbon& ribbon = static_cast<TCtrl*>(this)->GetWndRibbon();
				m_aBitmap[uItem] = ribbon.OnRibbonQueryItemImage(TCtrl::GetID(), uItem);
			}
			return m_aBitmap[uItem].IsNull() ?
				E_NOTIMPL :
				SetPropertyVal(key, GetImage(m_aBitmap[uItem], UI_OWNERSHIP_COPY), value);
		}
		else
		{
			return TextCollectionImpl<TCtrl, t_items, t_categories>::TextCollection::DoGetItem(uItem, key, value);
		}
	}
};

// ComboCollectionImpl: handles combo text
//
template <class TCtrl, size_t t_items, size_t t_categories = 0>
class ComboCollectionImpl : public ItemCollectionImpl<TCtrl, t_items, t_categories>
{
	typedef ComboCollectionImpl<TCtrl, t_items, t_categories> thisClass;
public:
	typedef thisClass ComboCollection;

	// Operations
	HRESULT SetComboText(LPCWSTR sText)
	{
		typename TCtrl::WndRibbon& ribbon = static_cast<TCtrl*>(this)->GetWndRibbon();
		return ribbon.IsRibbonUI() ? 
			ribbon.SetProperty(TCtrl::GetID(), UI_PKEY_StringValue, sText) : 
			S_OK;
	}

	LPCWSTR GetComboText()
	{
		static WCHAR sCombo[RIBBONUI_MAX_TEXT] = {};
		typename TCtrl::WndRibbon& ribbon = static_cast<TCtrl*>(this)->GetWndRibbon();
		PROPVARIANT var;
		if (ribbon.IsRibbonUI())
		{
			HRESULT hr = ribbon.GetIUIFrameworkPtr()->GetUICommandProperty(TCtrl::GetID(), UI_PKEY_StringValue, &var);
			hr = PropVariantToString(var, sCombo, RIBBONUI_MAX_TEXT);
			return sCombo;
		}
		return NULL;
	}
};

// CommandCollectionImpl: handles RibbonUI command collection controls
//
template <class TCtrl, size_t t_items, size_t t_categories = 0>
class CommandCollectionImpl : public CollectionImpl<TCtrl, t_items, t_categories>
{
	typedef CommandCollectionImpl<TCtrl, t_items, t_categories> thisClass;
public:
	typedef thisClass CommandCollection;

	CommandCollectionImpl()
	{
		::ZeroMemory(m_auCmd, sizeof(m_auCmd));
		::ZeroMemory(m_aCmdType, sizeof(m_aCmdType));
	}

	UINT32 m_auCmd[t_items];
	BYTE m_aCmdType[t_items];

	// Operations
	HRESULT SetItemCommand(UINT uItem, UINT32 uCommandID, bool bUpdate = false)
	{
		ATLASSERT(uItem < t_items);

		if (uCommandID == m_auCmd[uItem])
			return S_OK;

		typename TCtrl::WndRibbon& ribbon = static_cast<TCtrl*>(this)->GetWndRibbon();

		m_auCmd[uItem] = uCommandID;
		if (uCommandID != 0)
			ribbon.UIAddRibbonElement(uCommandID);

		return bUpdate ? this->InvalidateItems() : S_OK;
	}

	HRESULT SetItemCommandType(UINT uItem, UI_COMMANDTYPE type, bool bUpdate = false)
	{
		ATLASSERT(uItem < t_items);

		m_aCmdType[uItem] = (BYTE)type;

		return bUpdate ? this->InvalidateItems() : S_OK;
	}

// Implementation
 	HRESULT DoGetItem(UINT uItem, REFPROPERTYKEY key, PROPVARIANT *value)
	{
		ATLASSERT(uItem < t_items);
		typename TCtrl::WndRibbon& ribbon = static_cast<TCtrl*>(this)->GetWndRibbon();

		HRESULT hr = E_FAIL;
		switch (k_(key))
		{
		case k_CommandId:
			if (m_auCmd[uItem] == 0)
				SetItemCommand(uItem, ribbon.OnRibbonQueryItemCommand(TCtrl::GetID(), uItem));
			hr = SetPropertyVal(key, m_auCmd[uItem], value);
			break;
		case k_CommandType:
			if (m_aCmdType[uItem] == UI_COMMANDTYPE_UNKNOWN)
				SetItemCommandType(uItem, ribbon.OnRibbonQueryItemCommandType(TCtrl::GetID(), uItem));
			hr = SetPropertyVal(key, UINT32(m_aCmdType[uItem]), value);
			break;
		case k_CategoryId:
		default:
			hr = CollectionImpl<TCtrl, t_items, t_categories>::Collection::DoGetItem(uItem, key, value);
			break;
		}

		return hr;
	}

	HRESULT Select(UINT /*uItem*/, bool /*bUpdate*/ = false)
	{
		ATLASSERT(FALSE);
		return S_OK;
	}
};

// SimpleCollectionImpl: collection class for ribbon simple collection controls
//
template <class TCtrl, size_t t_size, UI_COMMANDTYPE t_CommandType = UI_COMMANDTYPE_ACTION>
class SimpleCollectionImpl : public CollectionImplBase<SimpleCollectionImpl<TCtrl, t_size>, t_size>
{
	typedef SimpleCollectionImpl<TCtrl, t_size, t_CommandType> thisClass;
public:
	typedef CollectionImplBase<thisClass, t_size> CollectionBase;
	typedef thisClass SimpleCollection;

// Implementation
	HRESULT OnGetItem(UINT uItem, REFPROPERTYKEY key, PROPVARIANT *value)
	{
		ATLASSERT(uItem < t_size);
		typename TCtrl::WndRibbon& ribbon = static_cast<TCtrl*>(this)->GetWndRibbon();

		HRESULT hr = E_NOTIMPL;
		switch (k_(key))
		{
		case k_ItemImage:
			if (HBITMAP hbm = ribbon.DefRibbonQueryItemImage(TCtrl::GetID(), uItem))
				hr = SetPropertyVal(key, GetImage(hbm, UI_OWNERSHIP_TRANSFER), value);
			break;
		case k_Label:
			if (LPCWSTR sText = ribbon.DefRibbonQueryItemText(TCtrl::GetID(), uItem))
				hr = SetPropertyVal(key, (LPCWSTR)sText, value);
			break;
		case k_CommandType:
			hr = SetPropertyVal(key, t_CommandType, value);
			break;
		case k_CommandId:
			hr = SetPropertyVal(key, ribbon.DefRibbonQueryItemCommand(TCtrl::GetID(), uItem), value);
			break;
		case k_CategoryId:
			hr = SetPropertyVal(key, UI_COLLECTION_INVALIDINDEX, value);
			break;
		default:
			ATLASSERT(FALSE);
			break;
		}

		return hr;
	}
};


///////////////////////////////////////////////////////////////////////////////
// Ribbon collection control classes

// CollectionCtrlImpl: specializable class for ribbon collection controls
//
template <class T, UINT t_ID, class TCollection>
class CollectionCtrlImpl : public CommandCtrlImpl<T, t_ID>, public TCollection
{
	typedef CollectionCtrlImpl<T, t_ID, TCollection> thisClass;
public:
	typedef CommandCtrlImpl<T, t_ID> CommandCtrl;
	typedef TCollection Collection;

	// Implementation
	virtual HRESULT DoUpdateProperty(UINT nCmdID, REFPROPERTYKEY key, 
	                                 const PROPVARIANT* ppropvarCurrentValue, PROPVARIANT* ppropvarNewValue)
	{
		ATLASSERT(nCmdID == this->GetID());
		ATLASSERT(ppropvarNewValue);

		HRESULT hr = Collection::DoUpdateProperty(nCmdID, key, ppropvarCurrentValue, ppropvarNewValue);
		if FAILED(hr)
			hr = CommandCtrl::DoUpdateProperty(nCmdID, key, ppropvarCurrentValue, ppropvarNewValue);

		return hr;
	}

	virtual HRESULT DoExecute(UINT nCmdID, UI_EXECUTIONVERB verb, 
	                          const PROPERTYKEY* key, const PROPVARIANT* ppropvarValue,
	                          IUISimplePropertySet* /*pCommandExecutionProperties*/)
	{
		ATLASSERT (nCmdID == this->GetID());
		(void)nCmdID; // avoid level4 warning

		if (key == NULL) // gallery button pressed
		{
			this->GetWndRibbon().OnRibbonItemSelected(this->GetID(), UI_EXECUTIONVERB_EXECUTE, UI_COLLECTION_INVALIDINDEX);
			return S_OK;
		}

		ATLASSERT(k_(*key) == k_SelectedItem);
		ATLASSERT(ppropvarValue);

		HRESULT hr = S_OK;
		UINT32 uSel = 0xffff;
		hr = UIPropertyToUInt32(*key, *ppropvarValue, &uSel);

		if (SUCCEEDED(hr))
		{
			if (this->GetWndRibbon().OnRibbonItemSelected(this->GetID(), verb, uSel))
				TCollection::Select(uSel);
		}

		return hr;
	}
};

// ToolbarGalleryCtrlImpl: base class for ribbon toolbar gallery controls
//
template <class T, UINT t_ID, UINT t_idTB, size_t t_size>
class ToolbarGalleryCtrlImpl : public CollectionCtrlImpl<T, t_ID, CommandCollectionImpl<ToolbarGalleryCtrlImpl<T, t_ID, t_idTB, t_size>, t_size>>
{
public:
	ToolbarGalleryCtrlImpl()
	{
		CResource tbres;
		ATLVERIFY(tbres.Load(RT_TOOLBAR, t_idTB));
		_AtlToolBarData* pData = (_AtlToolBarData*)tbres.Lock();
		ATLASSERT(pData);
		ATLASSERT(pData->wVersion == 1);

		WORD* pItems = pData->items();
		INT j = 0;
		for (int i = 0; (i < pData->wItemCount) && (j < t_size); i++)
		{
			if (pItems[i] != 0)
			{
				this->m_aCmdType[j] = UI_COMMANDTYPE_ACTION;
				this->m_auCmd[j++] = pItems[i];
			}
		}

		if (j < t_size)
			this->Resize(j);
	}

 	HRESULT DoGetItem(UINT uItem, REFPROPERTYKEY key, PROPVARIANT *value)
	{
		ATLASSERT(uItem < this->m_size);
		ATLASSERT(this->m_auCmd[uItem]);

		HRESULT hr = E_FAIL;
		switch (k_(key))
		{
		case k_CommandId:
			hr = SetPropertyVal(key, this->m_auCmd[uItem], value);
			break;
		case k_CommandType:
			hr = SetPropertyVal(key, UINT32(this->m_aCmdType[uItem]), value);
			break;
		case k_CategoryId:
			hr = SetPropertyVal(key, UI_COLLECTION_INVALIDINDEX, value);
			break;
		default:
			ATLASSERT(FALSE);
			break;
		}

		return hr;
	}
};


// SimpleCollectionCtrlImpl: base class for simple gallery and listbox controls
//
template <class T, UINT t_ID, size_t t_size, UI_COMMANDTYPE t_CommandType = UI_COMMANDTYPE_ACTION>
class SimpleCollectionCtrlImpl : 
		public CommandCtrlImpl<T, t_ID>,
		public SimpleCollectionImpl<SimpleCollectionCtrlImpl<T, t_ID, t_size, t_CommandType>, t_size, t_CommandType>
{
	typedef SimpleCollectionCtrlImpl<T, t_ID, t_size, t_CommandType> thisClass;
public:
	typedef thisClass SimpleCollection;

	SimpleCollectionCtrlImpl() : m_uSelected(0)
	{ }

	UINT m_uSelected;

	HRESULT Select(UINT uItem, bool bUpdate = false)
	{
		ATLASSERT((uItem < t_size) || (uItem == UI_COLLECTION_INVALIDINDEX));

		m_uSelected = uItem;

		return bUpdate ? 
			this->GetWndRibbon().SetProperty(this->GetID(), UI_PKEY_SelectedItem, uItem) :
			S_OK;
	}

	// Implementation
	virtual HRESULT DoUpdateProperty(UINT nCmdID, REFPROPERTYKEY key, 
	                                 const PROPVARIANT* ppropvarCurrentValue, PROPVARIANT* ppropvarNewValue)
	{
		ATLASSERT(nCmdID == this->GetID());
		ATLASSERT(ppropvarNewValue != NULL);

		HRESULT hr = S_OK;
		switch (k_(key))
		{
		case k_ItemsSource:
			{
				ATL::CComQIPtr<IUICollection> pIUICollection(ppropvarCurrentValue->punkVal);
				ATLASSERT(pIUICollection.p);
				hr = pIUICollection->Clear();
				for (UINT i = 0; i < t_size; i++)
				{
					if FAILED(hr = pIUICollection->Add(this->m_apItems[i]))
						break;
				}
				ATLASSERT(SUCCEEDED(hr));
			}
			break;
		case k_SelectedItem:
			hr = SetPropertyVal(UI_PKEY_SelectedItem, m_uSelected, ppropvarNewValue);
			break;
		default:
			hr = CommandCtrlImpl<T, t_ID>::DoUpdateProperty(nCmdID, key, ppropvarCurrentValue, ppropvarNewValue);
			break;
		}

		return hr;
	}

	virtual HRESULT DoExecute(UINT nCmdID, UI_EXECUTIONVERB verb, 
	                          const PROPERTYKEY* key, const PROPVARIANT* ppropvarValue,
	                          IUISimplePropertySet* /*pCommandExecutionProperties*/)
	{
		ATLASSERT (nCmdID == this->GetID());
		(void)nCmdID;   // avoid level 4 warning
		
		HRESULT hr = S_OK;
		if (key == NULL) // gallery button pressed
		{
			this->GetWndRibbon().OnRibbonItemSelected(this->GetID(), UI_EXECUTIONVERB_EXECUTE, UI_COLLECTION_INVALIDINDEX);
			return hr;
		}
		ATLASSERT(k_(*key) == k_SelectedItem);
		ATLASSERT(ppropvarValue);

		if SUCCEEDED(hr = UIPropertyToUInt32(*key, *ppropvarValue, &m_uSelected))
			this->GetWndRibbon().OnRibbonItemSelected(this->GetID(), verb, m_uSelected);

		return hr;
	}
};

// RecentItemsCtrlImpl
//
template <class T, UINT t_ID, class TDocList = CRecentDocumentList>
class RecentItemsCtrlImpl : 
		public CtrlImpl<T, t_ID>,
		public CollectionImplBase<RecentItemsCtrlImpl<T, t_ID, TDocList>, TDocList::m_nMaxEntries_Max>,
		public TDocList
{
	typedef RecentItemsCtrlImpl<T, t_ID, TDocList> thisClass;
public:
	typedef thisClass RecentItems;

	// Implementation
	HRESULT OnGetItem(UINT uItem, REFPROPERTYKEY key, PROPVARIANT *value)
	{
		ATLASSERT((INT)uItem < this->GetMaxEntries());

		LPCWSTR sPath = this->m_arrDocs[uItem].szDocName;
		HRESULT hr = E_NOTIMPL;
		switch (k_(key))
		{
		case k_Label:
			hr = SetPropertyVal(key, this->GetWndRibbon().OnRibbonQueryRecentItemName(sPath), value);
			break;
		case k_LabelDescription:
			hr = SetPropertyVal(key, sPath, value);
			break;
		default:
			ATLASSERT(FALSE);
			break;
		}

		return hr;
	}

	virtual HRESULT DoUpdateProperty(UINT nCmdID, REFPROPERTYKEY key, 
	                                 const PROPVARIANT* ppropvarCurrentValue, PROPVARIANT* ppropvarNewValue)
	{
		ATLASSERT(nCmdID == this->GetID());
		ATLASSERT(ppropvarNewValue);

		HRESULT hr = S_OK;
		switch (k_(key))
		{
		case k_RecentItems:
			if (SAFEARRAY* psa = SafeArrayCreateVector(VT_UNKNOWN, 0, this->m_arrDocs.GetSize()))
			{
				const int iLastIndex = this->m_arrDocs.GetSize() - 1;
				for (LONG i = 0; i <= iLastIndex; i++)
					SafeArrayPutElement(psa, &i, this->m_apItems[iLastIndex - i]); // reverse order

				hr = SetPropertyVal(key, psa, ppropvarNewValue);
				SafeArrayDestroy(psa);
			}
			break;
		default:
			hr = CtrlImpl<T, t_ID>::DoUpdateProperty(nCmdID, key, ppropvarCurrentValue, ppropvarNewValue);
			break;
		}

		return hr;
	}

	virtual HRESULT DoExecute(UINT nCmdID, UI_EXECUTIONVERB verb, 
	                          const PROPERTYKEY* key, const PROPVARIANT* ppropvarValue,
	                          IUISimplePropertySet* /*pCommandExecutionProperties*/)
	{
		ATLASSERT(nCmdID == this->GetID());
		(void)nCmdID;   // avoid level 4 warning
		ATLASSERT(verb == UI_EXECUTIONVERB_EXECUTE);
		(void)verb;   // avoid level 4 warning
		ATLASSERT((key) && (k_(*key) == k_SelectedItem));
		ATLASSERT(ppropvarValue);

		UINT32 uSel = 0xffff;
		HRESULT hr = UIPropertyToUInt32(*key, *ppropvarValue, &uSel);
		if SUCCEEDED(hr)
		{
			ATLASSERT(uSel < (UINT)this->GetMaxEntries());
			this->GetWndRibbon().DefCommandExecute(ID_FILE_MRU_FIRST + uSel);
		}

		return hr;
	}
};


///////////////////////////////////////////////////////////////////////////////
// Ribbon stand-alone control classes

// FontCtrlImpl
//
template <class T, UINT t_ID>
class FontCtrlImpl : public CtrlImpl<T, t_ID>
{
public:

	CharFormat m_cf;

// Implementation
	virtual HRESULT DoExecute(UINT nCmdID, UI_EXECUTIONVERB verb, 
	                          const PROPERTYKEY* key, const PROPVARIANT* ppropvarValue,
	                          IUISimplePropertySet* pCommandExecutionProperties)
	{
		ATLASSERT (nCmdID == this->GetID());
		(void)nCmdID;   // avoid level 4 warning
		ATLASSERT ((key) && (k_(*key) == k_FontProperties));
		(void)key;   // avoid level 4 warning

		HRESULT hr = E_INVALIDARG;
		switch (verb)
		{
			case UI_EXECUTIONVERB_PREVIEW:
			case UI_EXECUTIONVERB_EXECUTE:
				ATLASSERT(pCommandExecutionProperties);
				PROPVARIANT propvar;

				if (SUCCEEDED(hr = pCommandExecutionProperties->GetValue(UI_PKEY_FontProperties_ChangedProperties, &propvar)))
					m_cf << ATL::CComQIPtr<IPropertyStore>(propvar.punkVal);
				break;

			case UI_EXECUTIONVERB_CANCELPREVIEW:
				ATLASSERT(ppropvarValue);
				ATL::CComPtr<IPropertyStore> pStore;

				if (SUCCEEDED(hr = UIPropertyToInterface(UI_PKEY_FontProperties, *ppropvarValue, &pStore)))
					m_cf << pStore;
				break;
		}

		if (SUCCEEDED(hr))
			this->GetWndRibbon().OnRibbonFontCtrlExecute(this->GetID(), verb, &m_cf);
		else
			ATLASSERT(FALSE);

		return hr;
	}

	virtual HRESULT DoUpdateProperty(UINT nCmdID, REFPROPERTYKEY key, 
	                                 const PROPVARIANT* ppropvarCurrentValue, PROPVARIANT* ppropvarNewValue)
	{
		if ((k_(key) == k_FontProperties) && (this->GetWndRibbon().OnRibbonQueryFont(t_ID, m_cf)))
		{
			ATL::CComQIPtr<IPropertyStore> pStore(ppropvarCurrentValue->punkVal);
			m_cf >> pStore;
			return SetPropertyVal(key, pStore.p, ppropvarNewValue);
		}
		else
		{
			return CtrlImpl<T, t_ID>::DoUpdateProperty(nCmdID, key, ppropvarCurrentValue, ppropvarNewValue);
		}
	}
};

// ColorCtrlImpl
//
template <class T, UINT t_ID>
class ColorCtrlImpl : public CommandCtrlImpl<T, t_ID>
{
public:
	ColorCtrlImpl() : m_colorType(UI_SWATCHCOLORTYPE_NOCOLOR), m_color(0x800080) /*MAGENTA*/
	{ }

	UINT32 m_colorType; // value in UI_SWATCHCOLORTYPE
	COLORREF m_color;
	Text m_sLabels[6]; // k_MoreColorsLabel to k_ThemeColorsCategoryLabel
	ATL::CSimpleArray<COLORREF> m_aColors[2];
	ATL::CSimpleArray<LPCWSTR> m_aTooltips[2];

	// Operations
	HRESULT SetColor(COLORREF color, bool bUpdate = false)
	{
		if (m_colorType != UI_SWATCHCOLORTYPE_RGB)
			SetColorType(UI_SWATCHCOLORTYPE_RGB, bUpdate);
		m_color = color;
		return bUpdate ? this->SetProperty(UI_PKEY_Color, color) : S_OK;
	}

	HRESULT SetColorType(UI_SWATCHCOLORTYPE type, bool bUpdate = false)
	{
		m_colorType = type;
		return bUpdate ? this->SetProperty(UI_PKEY_ColorType, type) : S_OK;
	}

	HRESULT SetColorLabel(REFPROPERTYKEY key, LPCWSTR sLabel, bool bUpdate = false)
	{
		ATLASSERT((k_(key) >= k_ThemeColorsCategoryLabel) && (k_(key) <= k_MoreColorsLabel));
		m_sLabels[k_(key) - k_ThemeColorsCategoryLabel] = sLabel;
		return bUpdate ? this->SetProperty(key, sLabel) : S_OK;
	}

	HRESULT SetColorArray(REFPROPERTYKEY key, COLORREF* pColor, bool bUpdate = false)
	{
		ATLASSERT((k_(key) == k_ThemeColors) || (k_(key) == k_StandardColors));

		const INT ic = k_(key) - k_ThemeColors;
		m_aColors[ic].RemoveAll();
		while (*pColor != 0x800080) /*MAGENTA*/
			m_aColors[ic].Add(*pColor++);

		if (bUpdate)
		{
			PROPVARIANT var;
			if SUCCEEDED(InitPropVariantFromUInt32Vector(m_aColors[ic].GetData(), m_aColors[ic].GetSize(), &var))
				return this->SetProperty(key, var);
			else
				return E_INVALIDARG;
		}
		else
		{
			return S_OK;
		}
	}

	HRESULT SetColorTooltips(REFPROPERTYKEY key, LPCWSTR* ppsTT, bool bUpdate = false)
	{
		ATLASSERT((k_(key) == k_ThemeColorsTooltips) || (k_(key) == k_StandardColorsTooltips));

		const INT ic = k_(key) - k_ThemeColorsTooltips;
		m_aTooltips[ic].RemoveAll();
		while (*ppsTT)
			m_aTooltips[ic].Add(*ppsTT++);

		if (bUpdate)
		{
			PROPVARIANT var;
			if SUCCEEDED(InitPropVariantFromStringVector(m_aTooltips[ic].GetData(), m_aTooltips[ic].GetSize(), &var))
				return this->SetProperty(key, var);
			else
				return E_INVALIDARG;
		}
		else
		{
			return S_OK;
		}
	}

	// Implementation
	virtual HRESULT DoExecute(UINT nCmdID, UI_EXECUTIONVERB verb, 
	                          const PROPERTYKEY* key, const PROPVARIANT* ppropvarValue,
	                          IUISimplePropertySet* pCommandExecutionProperties)
	{
		ATLASSERT (nCmdID == this->GetID());
		(void)nCmdID;   // avoid level 4 warning
		ATLASSERT (key && (k_(*key) == k_ColorType));
		(void)key;   // avoid level 4 warning
		ATLASSERT (ppropvarValue);

		HRESULT hr = PropVariantToUInt32(*ppropvarValue, &m_colorType);
		ATLASSERT(SUCCEEDED(hr));

		if (SUCCEEDED(hr) && (m_colorType == UI_SWATCHCOLORTYPE_RGB))
		{
			ATLASSERT(pCommandExecutionProperties);
			PROPVARIANT var;
			if SUCCEEDED(hr = pCommandExecutionProperties->GetValue(UI_PKEY_Color, &var))
				hr = PropVariantToUInt32(var, &m_color);
		}

		if SUCCEEDED(hr)
			this->GetWndRibbon().OnRibbonColorCtrlExecute(this->GetID(), verb, (UI_SWATCHCOLORTYPE)m_colorType/*uType*/, m_color);
		else
			ATLASSERT(FALSE); // something was wrong

		return hr;
	}

	virtual HRESULT DoUpdateProperty(UINT nCmdID, REFPROPERTYKEY key, 
	                                 const PROPVARIANT* ppropvarCurrentValue, PROPVARIANT* ppropvarNewValue)
	{
		ATLASSERT (nCmdID == this->GetID());

		HRESULT hr = E_NOTIMPL;

		switch (k_(key))
		{
		case k_ColorType:
			hr = SetPropertyVal(key, m_colorType, ppropvarNewValue);
			break;
		case k_Color:
			if (m_color == 0x800080) /*MAGENTA*/
				m_color = this->GetWndRibbon().OnRibbonQueryColor(this->GetID());
			hr = SetPropertyVal(key, m_color, ppropvarNewValue);
			break;
		case k_ColorMode:
			break;
		case k_ThemeColorsCategoryLabel:
		case k_StandardColorsCategoryLabel:
		case k_RecentColorsCategoryLabel:
		case k_AutomaticColorLabel:
		case k_NoColorLabel:
		case k_MoreColorsLabel:
			{
				const UINT iLabel = k_(key) - k_ThemeColorsCategoryLabel;
				if (m_sLabels[iLabel].IsEmpty())
					if (LPCWSTR psLabel = this->GetWndRibbon().OnRibbonQueryColorLabel(this->GetID(), key))
						m_sLabels[iLabel] = psLabel;
				if (!m_sLabels[iLabel].IsEmpty())
					hr = SetPropertyVal(key, (LPCWSTR)m_sLabels[iLabel], ppropvarNewValue);
			}
			break;
		case k_ThemeColors:
		case k_StandardColors:
			{
				const INT ic = k_(key) - k_ThemeColors;
				if (!m_aColors[ic].GetSize())
					if (COLORREF* pColor = this->GetWndRibbon().OnRibbonQueryColorArray(this->GetID(), key))
						SetColorArray(key, pColor);
				if (INT iMax = m_aColors[ic].GetSize())
					hr = InitPropVariantFromUInt32Vector(m_aColors[ic].GetData(), iMax, ppropvarNewValue);
			}
			break;
		case k_ThemeColorsTooltips:
		case k_StandardColorsTooltips:
			{
				const INT ic = k_(key) - k_ThemeColorsTooltips;
				if (m_aTooltips[ic].GetSize() == 0)
					if (LPCWSTR* ppsTT = this->GetWndRibbon().OnRibbonQueryColorTooltips(this->GetID(), key))
						SetColorTooltips(key, ppsTT);
				if (INT iMax = m_aTooltips[ic].GetSize())
					hr = InitPropVariantFromStringVector(m_aTooltips[ic].GetData(), iMax, ppropvarNewValue);
			}
			break;
		default:
			hr = CommandCtrlImpl<T, t_ID>::DoUpdateProperty(nCmdID, key, ppropvarCurrentValue, ppropvarNewValue);
			break;
		}

		return hr;
	}
};

// SpinnerCtrlImpl
//
template <class T, UINT t_ID, typename V = LONG>
class SpinnerCtrlImpl : public CtrlImpl<T, t_ID>
{
public:
	SpinnerCtrlImpl()
	{
		m_Values[0] = m_Values[2] = m_Values[4] = 0;
		m_Values[1] = 100;
		m_Values[3] = 1;
	}

	V m_Values[5];
		// k_DecimalValue = 201, k_MaxValue = 203, k_MinValue, k_Increment, k_DecimalPlaces
		
	Text m_FormatString;
	Text m_RepresentativeString;

	// Operations
	HRESULT SetDecimalPlaces(V vPlaces, bool bUpdate = false)
	{
		return SetValue(UI_PKEY_DecimalPlaces, vPlaces, bUpdate);
	}

	HRESULT SetMin(V vMin, bool bUpdate = false)
	{
		return SetValue(UI_PKEY_MinValue, vMin, bUpdate);
	}

	HRESULT SetMax(V vMax, bool bUpdate = false)
	{
		return SetValue(UI_PKEY_MaxValue, vMax, bUpdate);
	}

	HRESULT SetVal(V vVal, bool bUpdate = false)
	{
		return SetValue(UI_PKEY_DecimalValue, vVal, bUpdate);
	}

	HRESULT SetIncrement(V vIncrement, bool bUpdate = false)
	{
		return SetValue(UI_PKEY_Increment, vIncrement, bUpdate);
	}

	HRESULT SetFormatString(LPCWSTR sFormat, bool bUpdate = false)
	{
		return SetText(UI_PKEY_FormatString, sFormat, bUpdate);
	}

	HRESULT SetRepresentativeString(LPCWSTR sRepresentative, bool bUpdate = false)
	{
		return SetText(UI_PKEY_RepresentativeString, sRepresentative, bUpdate);
	}

	// Implementation
	HRESULT SetText(REFPROPERTYKEY key, LPCWSTR sText, bool bUpdate = false)
	{
		switch (k_(key))
		{
		case k_FormatString:
			m_FormatString = sText;
			break;
		case k_RepresentativeString:
			m_RepresentativeString = sText;
			break;
		default:
			return CtrlImpl::SetText(key, sText, bUpdate);
		}

		return bUpdate ?
			this->GetWndRibbon().InvalidateProperty(this->GetID(), key) :
			S_OK;
	}

	HRESULT SetValue(REFPROPERTYKEY key, V val, bool bUpdate = false)
	{
		ATLASSERT((k_(key) <= k_DecimalPlaces) && (k_(key) >= k_DecimalValue));

		const INT iVal = k_(key) == k_DecimalValue ? 0 : k_(key) - k_StringValue;
		m_Values[iVal] = val;

		if (bUpdate)
		{
			if(k_(key) == k_DecimalValue)
			{
				DECIMAL decVal;
				InitDecimal(val, &decVal);
				return this->SetProperty(key, &decVal);
			}
			else
			{
				return this->GetWndRibbon().InvalidateProperty(this->GetID(), key);
			}
		}
		else
		{
			return S_OK;
		}
	}

	HRESULT QueryValue(REFPROPERTYKEY key, LONG* plVal)
	{
		return this->GetWndRibbon().OnRibbonQuerySpinnerValue(this->GetID(), key, plVal) ? S_OK : S_FALSE;
	}

	HRESULT QueryValue(REFPROPERTYKEY key, DOUBLE* pdVal)
	{
		return this->GetWndRibbon().OnRibbonQueryFloatSpinnerValue(this->GetID(), key, pdVal) ? S_OK : S_FALSE;
	}

	HRESULT OnGetValue(REFPROPERTYKEY key, PROPVARIANT* ppv)
	{
		ATLASSERT((k_(key) <= k_DecimalPlaces) && (k_(key) >= k_DecimalValue));

		const INT iVal = k_(key) == k_DecimalValue ? 0 : k_(key) - k_StringValue;

		QueryValue(key, m_Values + iVal);

		if (k_(key) == k_DecimalPlaces)
		{
			return SetPropertyVal(key, m_Values[iVal], ppv);
		}
		else
		{
			DECIMAL decVal;
			InitDecimal(m_Values[iVal], &decVal);
			return SetPropertyVal(key, &decVal, ppv);
		}
	}

	HRESULT OnGetText(REFPROPERTYKEY key, Text& sVal, PROPVARIANT* ppv)
	{
		if (LPCWSTR sNew = this->GetWndRibbon().OnRibbonQueryText(this->GetID(), key))
			sVal = sNew;
		return SetPropertyVal(key, (LPCWSTR)sVal, ppv);
	}

	virtual HRESULT DoExecute(UINT nCmdID, UI_EXECUTIONVERB verb, 
	                          const PROPERTYKEY* key, const PROPVARIANT* ppropvarValue,
	                          IUISimplePropertySet* /*pCommandExecutionProperties*/)
	{
		ATLASSERT (nCmdID == this->GetID());
		(void)nCmdID;   // avoid level 4 warning
		ATLASSERT (key && (k_(*key) == k_DecimalValue));
		(void)key;   // avoid level 4 warning
		ATLASSERT (verb == UI_EXECUTIONVERB_EXECUTE);
		(void)verb;   // avoid level 4 warning

		DECIMAL decVal;

		HRESULT hr = UIPropertyToDecimal(UI_PKEY_DecimalValue, *ppropvarValue, &decVal);
		hr = InitVal(m_Values[0], &decVal);

		this->GetWndRibbon().OnRibbonSpinnerCtrlExecute(this->GetID(), &m_Values[0]);

		return hr;
	}

	virtual HRESULT DoUpdateProperty(UINT nCmdID, REFPROPERTYKEY key, 
	                                 const PROPVARIANT* ppropvarCurrentValue, PROPVARIANT* ppropvarNewValue)
	{
		ATLASSERT (nCmdID == this->GetID());

		HRESULT hr = E_NOTIMPL;
		switch (k_(key))
		{
		case k_DecimalPlaces:
		case k_DecimalValue:
		case k_Increment:
		case k_MaxValue:
		case k_MinValue:
			hr = OnGetValue(key, ppropvarNewValue);
			break;
		case k_FormatString:
			if (m_FormatString.IsEmpty())
				return OnGetText(key, m_FormatString, ppropvarNewValue);
			break;
		case k_RepresentativeString:
			if (m_RepresentativeString.IsEmpty())
				return OnGetText(key, m_RepresentativeString, ppropvarNewValue);
			break;
		default:
			hr = CtrlImpl<T, t_ID>::DoUpdateProperty(nCmdID, key, ppropvarCurrentValue, ppropvarNewValue);
			break;
		}

		return hr;
	}

	// decimal conversion helpers
	static HRESULT InitDecimal(LONG& val, DECIMAL* pDecimal)
	{
		return ::VarDecFromI4(val, pDecimal);
	}

	static HRESULT InitDecimal(DOUBLE& val, DECIMAL* pDecimal)
	{
		return ::VarDecFromR8(val, pDecimal);
	}

	static HRESULT InitVal(LONG& val, const DECIMAL* pDecimal)
	{
		return ::VarI4FromDec(pDecimal, &val);
	}

	static HRESULT InitVal(DOUBLE& val, const DECIMAL* pDecimal)
	{
		return ::VarR8FromDec(pDecimal, &val);
	}
};

// CRibbonImpl Ribbon implementation class
//
template <class T>
class CRibbonImpl : 
		public CRibbonUpdateUI<T>,
		public ICtrl,
		public IUIApplication,
		public IUICommandHandler
{
	typedef CRibbonImpl<T> thisClass;
public:
	typedef thisClass Ribbon;
	typedef T WndRibbon;

	CRibbonImpl() : m_bRibbonUI(false), m_hgRibbonSettings(NULL)
	{
#ifdef _DEBUG
		m_cRef = 1;
#endif
		pWndRibbon = static_cast<T*>(this);
		HRESULT hr = ::CoInitialize(NULL);
		if(SUCCEEDED(hr))
			if (RunTimeHelper::IsRibbonUIAvailable())
				hr = m_pIUIFramework.CoCreateInstance(CLSID_UIRibbonFramework);
			else
				ATLTRACE2(atlTraceUI, 0, _T("Ribbon UI not available\n"));

		if FAILED(hr)
			ATLTRACE2(atlTraceUI, 0, _T("Ribbon construction failed\n"));

		ATLASSERT(SUCCEEDED(hr));
	}

	virtual ~CRibbonImpl()
	{
		::GlobalFree(m_hgRibbonSettings);
		m_pIUIFramework.Release();
		::CoUninitialize();
	}

	ICtrl& GetRibbonCtrl(UINT)
	{
		return static_cast<ICtrl&>(*this);
	}

	ATL::CComPtr<IUIFramework> m_pIUIFramework;
	bool m_bRibbonUI;
	HGLOBAL m_hgRibbonSettings;

	bool IsRibbonUI()
	{
		return m_bRibbonUI;
	}

	IUIFramework* GetIUIFrameworkPtr()
	{
		return m_pIUIFramework;
	}

	template <typename I>
	I* GetRibbonViewPtr(UINT32 uID)
	{
		ATLASSERT(m_pIUIFramework);
		ATL::CComPtr<I> pI;
		return m_pIUIFramework->GetView(uID, __uuidof(I), (void**) &pI) == S_OK ?
			pI : 
			NULL;
	}

	IUIRibbon* GetRibbonPtr()
	{
		return GetRibbonViewPtr<IUIRibbon>(0);
	}

	IUIContextualUI* GetMenuPtr(UINT32 uID)
	{
		ATLASSERT(uID);
		return GetRibbonViewPtr<IUIContextualUI>(uID);
	}

	UINT GetRibbonHeight()
	{
		ATLASSERT(IsRibbonUI());

		UINT32 cy = 0;
		if (ATL::CComPtr<IUIRibbon> pIUIRibbon = GetRibbonPtr())
			pIUIRibbon->GetHeight(&cy);
		return cy;
	}

	HRESULT CreateRibbon(LPCWSTR sResName = L"APPLICATION_RIBBON")
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(GetIUIFrameworkPtr() && !IsRibbonUI());
		ATLASSERT(pT->IsWindow());

		HRESULT hr = m_pIUIFramework->Initialize(pT->m_hWnd, this);

		if (hr == S_OK)
			hr = m_pIUIFramework->LoadUI(ModuleHelper::GetResourceInstance(), sResName);
			
		return hr;
	}

	HRESULT DestroyRibbon()
	{
		T* pT = static_cast<T*>(this);
		ATLASSERT(GetIUIFrameworkPtr() && IsRibbonUI());
		ATLASSERT(pT->IsWindow());

		HRESULT hRes = m_pIUIFramework->Destroy();
		if (!RunTimeHelper::IsWin7())
			pT->SetWindowRgn(NULL, TRUE); // Vista Basic bug workaround
		return hRes;
	}

// Ribbon persistency
	HRESULT operator >>(IStream* pIStream)
	{
		ATLASSERT(GetIUIFrameworkPtr());
		ATLASSERT(pIStream);

		HRESULT hr = E_FAIL;
		if (ATL::CComPtr<IUIRibbon> pIUIRibbon = GetRibbonPtr())
		{
			const LARGE_INTEGER li0 = {};
			pIStream->Seek(li0, STREAM_SEEK_SET, NULL);
			hr = pIUIRibbon->SaveSettingsToStream(pIStream);
			pIStream->Commit(STGC_DEFAULT);
		}

		return hr;
	}

	HRESULT operator <<(IStream* pIStream)
	{
		ATLASSERT(GetIUIFrameworkPtr());
		ATLASSERT(pIStream);

		HRESULT hr = E_FAIL;
		if (ATL::CComPtr<IUIRibbon> pIUIRibbon = GetRibbonPtr())
		{
			const LARGE_INTEGER li0 = {};
			pIStream->Seek(li0, STREAM_SEEK_SET, NULL);
			hr = pIUIRibbon->LoadSettingsFromStream(pIStream);
		}

		return hr;
	}

	void ResetRibbonSettings()
	{
		if (m_hgRibbonSettings != NULL)
		{
			::GlobalFree(m_hgRibbonSettings);
			m_hgRibbonSettings = NULL;
		}
	}

	HRESULT SaveRibbonSettings()
	{
		ATLASSERT(GetIUIFrameworkPtr());
		ATLASSERT(static_cast<T*>(this)->IsWindow());

		HRESULT hr = E_FAIL;
		ATL::CComPtr<IStream> pIStream;

		if SUCCEEDED(hr = ::CreateStreamOnHGlobal(m_hgRibbonSettings, FALSE, &pIStream))
			hr = *this >> pIStream;

		if (SUCCEEDED(hr) && (m_hgRibbonSettings == NULL))
			hr = ::GetHGlobalFromStream(pIStream, &m_hgRibbonSettings);

		if FAILED(hr)
			ResetRibbonSettings();

		return hr;
	}

	HRESULT RestoreRibbonSettings()
	{
		ATLASSERT(GetIUIFrameworkPtr());
		ATLASSERT(m_hgRibbonSettings);
		ATLASSERT(static_cast<T*>(this)->IsWindow());

		HRESULT hr = E_FAIL;
		ATL::CComPtr<IStream> pIStream;

		if SUCCEEDED(hr = ::CreateStreamOnHGlobal(m_hgRibbonSettings, FALSE, &pIStream))
			hr = *this << pIStream;

		if FAILED(hr)
			ResetRibbonSettings();

		return hr;
	}

// QAT dock states
	UI_CONTROLDOCK GetQATDock()
	{
		ATLASSERT(GetIUIFrameworkPtr());
		ATLASSERT(IsRibbonUI());

		UINT32 uDock = 0;
		PROPVARIANT propvar;
		ATL::CComQIPtr<IPropertyStore>pIPS(GetRibbonPtr());

		if ((pIPS != NULL) && SUCCEEDED(pIPS->GetValue(UI_PKEY_QuickAccessToolbarDock, &propvar)) &&
			SUCCEEDED(UIPropertyToUInt32(UI_PKEY_QuickAccessToolbarDock, propvar, &uDock)))
				return (UI_CONTROLDOCK)uDock;

		ATLASSERT(FALSE); // something was wrong
		return (UI_CONTROLDOCK)0;
	}

	bool SetQATDock(UI_CONTROLDOCK dockState)
	{
		ATLASSERT(GetIUIFrameworkPtr());
		ATLASSERT(IsRibbonUI());

		PROPVARIANT propvar;
		ATLVERIFY(SUCCEEDED(SetPropertyVal(UI_PKEY_QuickAccessToolbarDock, dockState, &propvar)));

		ATL::CComQIPtr<IPropertyStore>pIPS(GetRibbonPtr());
		if ((pIPS != NULL) && SUCCEEDED(pIPS->SetValue(UI_PKEY_QuickAccessToolbarDock, propvar)))
		{
			pIPS->Commit();
			return true;
		}

		ATLASSERT(FALSE); // something was wrong
		return false;
	}

// Ribbon display states
	bool GetRibbonDisplayState(REFPROPERTYKEY key)
	{
		ATLASSERT(GetIUIFrameworkPtr());
		ATLASSERT(IsRibbonUI());
		ATLASSERT((k_(key) == k_Viewable) || (k_(key) == k_Minimized));

		PROPVARIANT propvar;
		ATL::CComQIPtr<IPropertyStore>pIPS(GetRibbonPtr());

		if ((pIPS != NULL) && SUCCEEDED(pIPS->GetValue(key, &propvar)))
		{
			BOOL bState = FALSE;
			if SUCCEEDED(UIPropertyToBoolean(key, propvar, &bState))
				return (bState != FALSE);
		}

		ATLASSERT(FALSE); // something was wrong
		return false;
	}

	bool SetRibbonDisplayState(REFPROPERTYKEY key, bool bState = true)
	{
		ATLASSERT(GetIUIFrameworkPtr());
		ATLASSERT(IsRibbonUI());
		ATLASSERT((k_(key) == k_Viewable) || (k_(key) == k_Minimized));

		PROPVARIANT propvar;
		ATLVERIFY(SUCCEEDED(SetPropertyVal(key, bState, &propvar)));

		ATL::CComQIPtr<IPropertyStore>pIPS(GetRibbonPtr());

		if ((pIPS != NULL) && SUCCEEDED(pIPS->SetValue(key, propvar)))
		{
			pIPS->Commit();
			return true;
		}

		ATLASSERT(FALSE); // something was wrong
		return false;
	}

	bool IsRibbonMinimized()
	{
		return GetRibbonDisplayState(UI_PKEY_Minimized);
	}

	bool MinimizeRibbon(bool bMinimize = true)
	{
		return SetRibbonDisplayState(UI_PKEY_Minimized, bMinimize);
	}

	bool IsRibbonHidden()
	{
		return !GetRibbonDisplayState(UI_PKEY_Viewable);
	}

	bool HideRibbon(bool bHide = true)
	{
		return SetRibbonDisplayState(UI_PKEY_Viewable, !bHide);
	}

// Ribbon colors
	UI_HSBCOLOR GetRibbonColor(REFPROPERTYKEY key)
	{
		ATLASSERT(GetIUIFrameworkPtr());
		ATLASSERT(IsRibbonUI());
		ATLASSERT((k_(key) >= k_GlobalBackgroundColor) && (k_(key) <= k_GlobalTextColor));

		PROPVARIANT propvar;
		ATL::CComQIPtr<IPropertyStore>pIPS(GetIUIFrameworkPtr());

		if ((pIPS != NULL) && SUCCEEDED(pIPS->GetValue(key, &propvar)))
		{
			UINT32 color = 0;
			if SUCCEEDED(UIPropertyToUInt32(key, propvar, &color))
				return color;
		}

		ATLASSERT(FALSE); // something was wrong
		return 0;
	}

	bool SetRibbonColor(REFPROPERTYKEY key, UI_HSBCOLOR color)
	{
		ATLASSERT(GetIUIFrameworkPtr());
		ATLASSERT(IsRibbonUI());
		ATLASSERT((k_(key) >= k_GlobalBackgroundColor) && (k_(key) <= k_GlobalTextColor));

		PROPVARIANT propvar;
		ATLVERIFY(SUCCEEDED(SetPropertyVal(key, color, &propvar)));

		ATL::CComQIPtr<IPropertyStore>pIPS(GetIUIFrameworkPtr());

		if ((pIPS != NULL) && SUCCEEDED(pIPS->SetValue(key, propvar)))
		{
			pIPS->Commit();
			return true;
		}

		ATLASSERT(FALSE); // something was wrong
		return false;
	}

// Ribbon modes
	HRESULT SetRibbonModes(INT32 iModes)
	{
		ATLASSERT(IsRibbonUI());
		return GetIUIFrameworkPtr()->SetModes(iModes);
	}

// Ribbon contextual tab
	UI_CONTEXTAVAILABILITY GetRibbonContextAvail(UINT32 uID)
	{
		ATLASSERT(GetIUIFrameworkPtr());

		PROPVARIANT propvar;
		if (IsRibbonUI() && 
		    SUCCEEDED(GetIUIFrameworkPtr()->GetUICommandProperty(uID, UI_PKEY_ContextAvailable, &propvar)))
		{
			UINT uav;
			if (SUCCEEDED(PropVariantToUInt32(propvar, &uav)))
			{
				CUpdateUIBase::UIEnable(uID, uav != UI_CONTEXTAVAILABILITY_NOTAVAILABLE);
				CUpdateUIBase::UISetCheck(uID, uav == UI_CONTEXTAVAILABILITY_ACTIVE);
				return (UI_CONTEXTAVAILABILITY)uav;
			}
		}

		return UI_CONTEXTAVAILABILITY_NOTAVAILABLE;
	}

	HRESULT SetRibbonContextAvail(UINT32 uID, UI_CONTEXTAVAILABILITY cav)
	{
		CUpdateUIBase::UIEnable(uID, cav != UI_CONTEXTAVAILABILITY_NOTAVAILABLE);
		CUpdateUIBase::UISetCheck(uID, cav == UI_CONTEXTAVAILABILITY_ACTIVE);
		
		return SetProperty((WORD)uID, UI_PKEY_ContextAvailable, UINT32(cav));
	}

// Ribbon context menu
	bool HasRibbonMenu(UINT32 uID)
	{
		ATL::CComPtr<IUIContextualUI> pI = GetMenuPtr(uID);
		return pI != NULL;
	}

	HRESULT TrackRibbonMenu(UINT32 uID, INT32 x, INT32 y)
	{
		ATLASSERT(HasRibbonMenu(uID));

		return IsRibbonUI() ?
			ATL::CComPtr<IUIContextualUI>(GetMenuPtr(uID))->ShowAtLocation(x, y) :
			E_FAIL;
	}

	HRESULT TrackRibbonMenu(UINT32 uID, LPARAM lParam)
	{
		return TrackRibbonMenu(uID, GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam));
	}

// Overrideables
	HBITMAP OnRibbonQueryImage(UINT nCmdID, REFPROPERTYKEY /*key*/)
	{
		return DefRibbonQueryImage(nCmdID);
	}

	LPCWSTR OnRibbonQueryText(UINT nCmdID, REFPROPERTYKEY key)
	{
		return DefRibbonQueryText(nCmdID, key);
	}

	bool OnRibbonQueryState(UINT nCmdID, REFPROPERTYKEY key)
	{
		return DefRibbonQueryState(nCmdID, key);
	}

	UI_CONTEXTAVAILABILITY OnRibbonQueryTabAvail(UINT nCmdID)
	{
		DWORD dwState = this->UIGetState(nCmdID);
		return ((dwState & CUpdateUIBase::UPDUI_DISABLED) == CUpdateUIBase::UPDUI_DISABLED) ?
			UI_CONTEXTAVAILABILITY_NOTAVAILABLE :
			(((dwState & CUpdateUIBase::UPDUI_CHECKED) == CUpdateUIBase::UPDUI_CHECKED) ?
				UI_CONTEXTAVAILABILITY_ACTIVE : 
				UI_CONTEXTAVAILABILITY_AVAILABLE);
	}

	LPCWSTR OnRibbonQueryComboText(UINT32 /*uCtrlID*/)
	{
		return NULL;
	}

	LPCWSTR OnRibbonQueryCategoryText(UINT32 /*uCtrlID*/, UINT32 /*uCat*/)
	{
		return L"Category";
	}

	UINT32 OnRibbonQueryItemCategory(UINT32 /*uCtrlID*/, UINT32 /*uItem*/)
	{
		return 0;
	}

	LPCWSTR OnRibbonQueryItemText(UINT32 uCtrlID, UINT32 uItem)
	{
		return DefRibbonQueryItemText(uCtrlID, uItem);
	}

	bool OnRibbonQuerySelectedItem(UINT32 /*uCtrlID*/, UINT32& /*uSel*/)
	{
		return false;
	}

	HBITMAP OnRibbonQueryItemImage(UINT32 uCtrlID, UINT32 uItem)
	{
		return DefRibbonQueryItemImage(uCtrlID, uItem);
	}

	UINT32 OnRibbonQueryItemCommand(UINT32 uCtrlID, UINT32 uItem)
	{
		return DefRibbonQueryItemCommand(uCtrlID, uItem);
	}

	UI_COMMANDTYPE OnRibbonQueryItemCommandType(UINT32 /*uCtrlID*/, UINT32 /*uItem*/)
	{
		return UI_COMMANDTYPE_ACTION;
	}

	LPCWSTR OnRibbonQueryRecentItemName(LPCWSTR sPath)
	{
		return ::PathFindFileName(sPath);
	}

	bool OnRibbonQueryFont(UINT /*nId*/, CHARFORMAT2& /*cf*/)
	{
		return false;
	}

	bool OnRibbonQuerySpinnerValue(UINT /*nCmdID*/, REFPROPERTYKEY /*key*/, LONG* /*pVal*/)
	{
		return false;
	}

	bool OnRibbonQueryFloatSpinnerValue(UINT /*nCmdID*/, REFPROPERTYKEY /*key*/, DOUBLE* /*pVal*/)
	{
		return false;
	}

	COLORREF OnRibbonQueryColor(UINT /*nCmdID*/)
	{
		return 0x800080; /*MAGENTA*/
	}

	LPCWSTR OnRibbonQueryColorLabel(UINT /*nCmdID*/, REFPROPERTYKEY /*key*/)
	{
		return NULL;
	}

	COLORREF* OnRibbonQueryColorArray(UINT /*nCmdID*/, REFPROPERTYKEY /*key*/)
	{
		return NULL;
	}

	LPCWSTR* OnRibbonQueryColorTooltips(UINT /*nCmdID*/, REFPROPERTYKEY /*key*/)
	{
		return NULL;
	}

	bool OnRibbonItemSelected(UINT32 uCtrlID, UI_EXECUTIONVERB verb, UINT32 uItem)
	{
		DefCommandExecute(MAKELONG(uCtrlID, verb), uItem);
		return true;
	}

	void OnRibbonColorCtrlExecute(UINT32 uCtrlID, UI_EXECUTIONVERB verb, UI_SWATCHCOLORTYPE uType, COLORREF color)
	{
		DefRibbonColorCtrlExecute(uCtrlID, verb, uType, color);
	}

	void OnRibbonFontCtrlExecute(UINT32 uCtrlID, UI_EXECUTIONVERB verb, CHARFORMAT2* pcf)
	{
		DefCommandExecute(MAKELONG(uCtrlID, verb), (LPARAM)pcf);
	}

	void OnRibbonSpinnerCtrlExecute(UINT32 uCtrlID, LONG* pVal)
	{
		DefCommandExecute(uCtrlID, *pVal);
	}

	void OnRibbonSpinnerCtrlExecute(UINT32 uCtrlID, DOUBLE* pVal)
	{
		DefCommandExecute(uCtrlID, (LPARAM)pVal);
	}

	void OnRibbonCommandExecute(UINT32 uCmdID)
	{
		DefCommandExecute(uCmdID);
	}

// Default implementations
	HBITMAP DefRibbonQueryImage(UINT nCmdID)
	{
		return AtlLoadBitmapImage(nCmdID, LR_CREATEDIBSECTION);
	}

	bool DefRibbonQueryState(UINT nCmdID, REFPROPERTYKEY key)
	{
		DWORD dwState = this->UIGetState(nCmdID);
		bool bRet = false;
		switch (k_(key))
		{
		case k_BooleanValue:
			bRet = (dwState & CUpdateUIBase::UPDUI_CHECKED) == CUpdateUIBase::UPDUI_CHECKED;
			break;
		case k_Enabled:
			bRet = (dwState & CUpdateUIBase::UPDUI_DISABLED) != CUpdateUIBase::UPDUI_DISABLED;
			break;
		default:
			ATLASSERT(FALSE);
			break;
		}

		return bRet;
	}

	LPCTSTR DefRibbonQueryText(UINT nCmdID, REFPROPERTYKEY key)
	{
		static WCHAR sText[RIBBONUI_MAX_TEXT] = {};

		if (k_(key) == k_Label)
			 return this->UIGetText(nCmdID);

		if (ATL::AtlLoadString(nCmdID, sText, RIBBONUI_MAX_TEXT))
		{
			PWCHAR pTitle = wcschr(sText, L'\n');
			switch (k_(key))
			{
			case k_Keytip:
				if (PWCHAR pAmp = wcschr(sText, L'&'))
					pTitle = pAmp;
				if (pTitle != NULL)
					*(pTitle + 2) = NULL; // fall through
			case k_TooltipTitle:
				return pTitle ? ++pTitle : NULL;
			case k_TooltipDescription:
			case k_LabelDescription:
				if (pTitle != NULL)
					*pTitle = NULL;
				return sText;
			}
		}

		return NULL;
	}

	LPCWSTR DefRibbonQueryItemText(UINT32 uCtrlID, UINT32 uItem)
	{
		return DefRibbonQueryText(uCtrlID + 1 + uItem, UI_PKEY_LabelDescription);
	}

	HBITMAP DefRibbonQueryItemImage(UINT32 uCtrlID, UINT32 uItem)
	{
		return DefRibbonQueryImage(uCtrlID + 1 + uItem);
	}

	UINT32 DefRibbonQueryItemCommand(UINT32 uCtrlID, UINT32 uItem)
	{
		return uCtrlID + 1 + uItem;
	}

	void DefRibbonColorCtrlExecute(UINT32 uCtrlID, UI_EXECUTIONVERB verb, UI_SWATCHCOLORTYPE uType, COLORREF color)
	{
		switch(uType)
		{
		case UI_SWATCHCOLORTYPE_RGB:
			break;
		case UI_SWATCHCOLORTYPE_AUTOMATIC:
			color = ::GetSysColor(COLOR_WINDOWTEXT);
			break;
		case UI_SWATCHCOLORTYPE_NOCOLOR:
			color = ::GetSysColor(COLOR_WINDOW);
			break;
		default:
			ATLASSERT(FALSE);
			break;
		}

		DefCommandExecute(MAKELONG(uCtrlID, verb), color);
	}

	void DefCommandExecute(UINT32 uCmd, LPARAM lParam = 0)
	{
		static_cast<T*>(this)->PostMessage(WM_COMMAND, uCmd, lParam);
	}

// Elements setting helpers
	HRESULT InvalidateCtrl(UINT32 nID)
	{
		return IsRibbonUI() ?
			GetIUIFrameworkPtr()->InvalidateUICommand(nID, UI_INVALIDATIONS_ALLPROPERTIES, NULL) :
			E_FAIL;
	}

	HRESULT InvalidateProperty(UINT32 nID, REFPROPERTYKEY key, UI_INVALIDATIONS flags = UI_INVALIDATIONS_PROPERTY)
	{
		return IsRibbonUI() ?
			GetIUIFrameworkPtr()->InvalidateUICommand(nID, flags, &key) :
			E_FAIL;
	}

	template <typename V>
	HRESULT SetProperty(WORD wID, REFPROPERTYKEY key, V val)
	{
		if (IsRibbonUI())
		{
			PROPVARIANT var;
			if (SUCCEEDED(RibbonUI::SetPropertyVal(key, val, &var)))
			{
				return SetProperty(wID, key, var);
			}
			return E_INVALIDARG;
		}
		else
		{
			return E_FAIL;
		}
	}

	template <>
	HRESULT SetProperty(WORD nID, REFPROPERTYKEY key, PROPVARIANT var)
	{
		return IsRibbonUI() ?
			GetIUIFrameworkPtr()->SetUICommandProperty(nID, key, var) :
			E_FAIL;
	}

// Interfaces
	// IUIApplication
	STDMETHODIMP OnViewChanged(UINT32, UI_VIEWTYPE, IUnknown*, UI_VIEWVERB verb, INT32)
	{
		switch (verb)
		{			
		case UI_VIEWVERB_CREATE:
			m_bRibbonUI = true;
			if (m_hgRibbonSettings != NULL)
				RestoreRibbonSettings();
			break;
		case UI_VIEWVERB_SIZE:
			static_cast<T*>(this)->UpdateLayout(FALSE);
			break;
		case UI_VIEWVERB_DESTROY:
			SaveRibbonSettings();
			m_bRibbonUI = false;
			break;
		}

		return S_OK;
	}

	STDMETHODIMP OnCreateUICommand(UINT32 nCmdID, UI_COMMANDTYPE typeID, IUICommandHandler** ppCommandHandler)
	{
		this->UIAddRibbonElement(nCmdID);
		if (typeID == UI_COMMANDTYPE_CONTEXT)
			CUpdateUIBase::UIEnable(nCmdID, false);
		*ppCommandHandler = this;
		return S_OK;
	}

	STDMETHODIMP OnDestroyUICommand(UINT32 nCmdID, UI_COMMANDTYPE, IUICommandHandler*)
	{
		this->UIRemoveRibbonElement(nCmdID);
		return S_OK;
	}

	// IUICommandHandler
	STDMETHODIMP Execute(UINT nCmdID,
		UI_EXECUTIONVERB verb, 
		const PROPERTYKEY* key,
		const PROPVARIANT* ppropvarValue,
		IUISimplePropertySet* pCommandExecutionProperties)
	{
		T* pT =static_cast<T*>(this);
		return pT->GetRibbonCtrl(nCmdID).DoExecute(nCmdID, verb, key, ppropvarValue, pCommandExecutionProperties);	
	}

	STDMETHODIMP UpdateProperty(UINT nCmdID, REFPROPERTYKEY key, 
	                            const PROPVARIANT* ppropvarCurrentValue, PROPVARIANT* ppropvarNewValue)
	{
		T* pT =static_cast<T*>(this);
		return pT->GetRibbonCtrl(nCmdID).DoUpdateProperty(nCmdID, key, ppropvarCurrentValue, ppropvarNewValue);	
	}

#ifdef _DEBUG
	// IUnknown methods (heavyweight)
	STDMETHODIMP_(ULONG) AddRef()
	{
		return InterlockedIncrement(&m_cRef);
	}

	STDMETHODIMP_(ULONG) Release()
	{
		LONG cRef = InterlockedDecrement(&m_cRef);
		if (cRef == 0) // NoOp for breakpoint
		{
			cRef = 0;
		}

		return cRef;
	}

	STDMETHODIMP QueryInterface(REFIID iid, void** ppv)
	{
		if (ppv == NULL)
		{
			return E_POINTER;
		}
		else if ((iid == __uuidof(IUnknown)) ||
		         (iid == __uuidof(IUICommandHandler)) ||
		         (iid == __uuidof(IUIApplication)))
		{
			*ppv = this;
			AddRef();
			return S_OK;
		}
		else
		{
			return E_NOINTERFACE;
		}
	}

	LONG m_cRef;
#else
	// IUnknown methods (lightweight)
	STDMETHODIMP QueryInterface(REFIID iid, void** ppv)
	{
		if ((iid == __uuidof(IUnknown)) ||
		    (iid == __uuidof(IUICommandHandler)) ||
		    (iid == __uuidof(IUIApplication)))
		{
			*ppv = this;
			return S_OK;
		}
		return E_NOINTERFACE;
	}
	ULONG STDMETHODCALLTYPE AddRef()
	{
		return 1;
	}
	ULONG STDMETHODCALLTYPE Release()
	{
		return 1;
	}
#endif

// CRibbonImpl ICtrl implementation
	virtual HRESULT DoExecute(UINT nCmdID, UI_EXECUTIONVERB verb, 
	                          const PROPERTYKEY* key, const PROPVARIANT* ppropvarValue,
	                          IUISimplePropertySet* /*pCommandExecutionProperties*/)
	{
		if (key != NULL)
		{
			if(k_(*key) != k_BooleanValue)
			{
				ATLTRACE2(atlTraceUI, 0, _T("Control ID %d is not handled\n"), nCmdID);
				return E_NOTIMPL;
			}
			BOOL bChecked = FALSE;
			ATLVERIFY(SUCCEEDED(PropVariantToBoolean(*ppropvarValue, &bChecked)));
			CUpdateUIBase::UISetCheck(nCmdID, bChecked);
		}

		ATLASSERT(verb == UI_EXECUTIONVERB_EXECUTE);
		(void)verb;   // avoid level 4 warning

		static_cast<T*>(this)->OnRibbonCommandExecute(nCmdID);
		
		return S_OK;
	}

	virtual HRESULT DoUpdateProperty(UINT nCmdID, REFPROPERTYKEY key, 
		const PROPVARIANT* /*ppropvarCurrentValue*/, PROPVARIANT* ppropvarNewValue)
	{
		T* pT = static_cast<T*>(this);
		HRESULT hr = E_NOTIMPL;
		switch (k_(key))
		{
		case k_LargeImage:
		case k_LargeHighContrastImage:
		case k_SmallImage:
		case k_SmallHighContrastImage:
			if (HBITMAP hbm = pT->OnRibbonQueryImage(nCmdID, key))
				hr = SetPropertyVal(key, GetImage(hbm, UI_OWNERSHIP_TRANSFER), ppropvarNewValue);
			break;
		case k_Label:
		case k_Keytip:
		case k_TooltipTitle:
		case k_TooltipDescription:
		case k_LabelDescription:
			if (LPCWSTR sText = pT->OnRibbonQueryText(nCmdID, key))
				hr = SetPropertyVal(key, sText, ppropvarNewValue);
			break;
		case k_BooleanValue:
		case k_Enabled:
			hr = SetPropertyVal(key, pT->OnRibbonQueryState(nCmdID, key), ppropvarNewValue);
			break;
		case k_ContextAvailable:
			hr = SetPropertyVal(key, pT->OnRibbonQueryTabAvail(nCmdID), ppropvarNewValue);
			break;
		}

		return hr;
	}

// CRibbonImpl::CRibbonXXXCtrl specialized classes
	 //CRibbonComboCtrl
	template <UINT t_ID, size_t t_items, size_t t_categories = 0>
	class CRibbonComboCtrl : public CollectionCtrlImpl<T, t_ID, ComboCollectionImpl<CRibbonComboCtrl<t_ID, t_items, t_categories>, t_items, t_categories>>
	{
	public:
		CRibbonComboCtrl()
		{ }
	};

	// CRibbonItemGalleryCtrl
	template <UINT t_ID, size_t t_items, size_t t_categories = 0>
	class CRibbonItemGalleryCtrl : public CollectionCtrlImpl<T, t_ID, ItemCollectionImpl<CRibbonItemGalleryCtrl<t_ID, t_items, t_categories>, t_items, t_categories>>
	{
	public:
		CRibbonItemGalleryCtrl()
		{ }
	};

	// CRibbonCommandGalleryCtrl
	template <UINT t_ID, size_t t_items, size_t t_categories = 0>
	class CRibbonCommandGalleryCtrl : public CollectionCtrlImpl<T, t_ID, CommandCollectionImpl<CRibbonCommandGalleryCtrl<t_ID, t_items, t_categories>, t_items, t_categories>>
	{
	public:
		CRibbonCommandGalleryCtrl()
		{ }
	};

	// CRibbonToolbarGalleryCtrl
	template <UINT t_ID, UINT t_idTB, size_t t_size>
	class CRibbonToolbarGalleryCtrl : public ToolbarGalleryCtrlImpl<T, t_ID, t_idTB, t_size>
	{ };

	// CRibbonSimpleComboCtrl
	template <UINT t_ID, size_t t_size>
	class CRibbonSimpleComboCtrl : public SimpleCollectionCtrlImpl<T, t_ID, t_size>
	{ };

	// CRibbonSimpleGalleryCtrl
	template <UINT t_ID, size_t t_size, UI_COMMANDTYPE t_CommandType = UI_COMMANDTYPE_ACTION>
	class CRibbonSimpleGalleryCtrl : public SimpleCollectionCtrlImpl<T, t_ID, t_size, t_CommandType>
	{ };

	//CRibbonRecentItemsCtrl
	template <UINT t_ID, class TDocList = CRecentDocumentList>
	class CRibbonRecentItemsCtrl : public RecentItemsCtrlImpl<T, t_ID, TDocList>
	{
	public:
		CRibbonRecentItemsCtrl()
		{ }
	};

	// CRibbonColorCtrl
	template <UINT t_ID>
	class CRibbonColorCtrl : public ColorCtrlImpl<T, t_ID>
	{
	public:
		CRibbonColorCtrl()
		{ }
	};

	 //CRibbonFontCtrl
	template <UINT t_ID>
	class CRibbonFontCtrl : public FontCtrlImpl<T, t_ID>
	{
	public:
		CRibbonFontCtrl()
		{ }
	};

	// CRibbonSpinnerCtrl
	template <UINT t_ID>
	class CRibbonSpinnerCtrl : public SpinnerCtrlImpl<T, t_ID, LONG>
	{
	public:
		CRibbonSpinnerCtrl()
		{ }
	};

	// CRibbonFloatSpinnerCtrl
	template <UINT t_ID>
	class CRibbonFloatSpinnerCtrl : public SpinnerCtrlImpl<T, t_ID, DOUBLE>
	{
	public:
		CRibbonFloatSpinnerCtrl()
		{
			this->m_Values[4] = 1; // 1 decimal
		}
	};

	// CRibbonCommandCtrl
	template <UINT t_ID>
	class CRibbonCommandCtrl : public CommandCtrlImpl<T, t_ID>
	{
	public:
		CRibbonCommandCtrl()
		{ }
	};

// Control classes access to T instance (re-initialized in constructor)
	static T* pWndRibbon;
};

template <class T>
__declspec(selectany) T* CRibbonImpl<T>::pWndRibbon;

// Control map element
#pragma warning(push)
#pragma warning(disable: 4510 610 4512)   // missing default constructor, can't be instatiated, assignment operator could not be generated
typedef struct
{
	UINT uID;
	ICtrl& ctrl;
} _ribbonCtrl;
#pragma warning(pop)

} // namespace RibbonUI


///////////////////////////////////////////////////////////////////////////////
// RibbonUI Control map

// Control map macros
#define BEGIN_RIBBON_CONTROL_MAP(theClass) \
	WTL::RibbonUI::ICtrl& GetRibbonCtrl(UINT id) \
	{ \
		WTL::RibbonUI::_ribbonCtrl _ctrls[] = \
		{

#define RIBBON_CONTROL(member) {member.GetID(), static_cast<WTL::RibbonUI::ICtrl&>(member)},

#define END_RIBBON_CONTROL_MAP() \
		{0, *this} \
	}; \
	int i = 0; \
	for(; i < _countof(_ctrls) - 1; i++) \
		if (_ctrls[i].uID == id) \
			break; \
	return _ctrls[i].ctrl; \
}

// Control message map macros
#define RIBBON_GALLERY_CONTROL_HANDLER(id, func) \
	if((uMsg == WM_COMMAND) && (id == LOWORD(wParam))) \
	{ \
		bHandled = TRUE; \
		lResult = func((UI_EXECUTIONVERB)HIWORD(wParam), LOWORD(wParam), (UINT)lParam, bHandled); \
		if(bHandled) \
			return TRUE; \
	}

#define RIBBON_COMBO_CONTROL_HANDLER(id, func) \
	RIBBON_GALLERY_CONTROL_HANDLER(id, func)	

#define RIBBON_FONT_CONTROL_HANDLER(id, func) \
	if((uMsg == WM_COMMAND) && (id == LOWORD(wParam))) \
	{ \
		bHandled = TRUE; \
		lResult = func((UI_EXECUTIONVERB)HIWORD(wParam), LOWORD(wParam), (CHARFORMAT2*)lParam, bHandled); \
		if(bHandled) \
			return TRUE; \
	}

#define RIBBON_COLOR_CONTROL_HANDLER(id, func) \
	if((uMsg == WM_COMMAND) && (id == LOWORD(wParam))) \
	{ \
		bHandled = TRUE; \
		lResult = func((UI_EXECUTIONVERB)HIWORD(wParam), LOWORD(wParam), (COLORREF)lParam, bHandled); \
		if(bHandled) \
			return TRUE; \
	}

#define RIBBON_SPINNER_CONTROL_HANDLER(id, func) \
	if((uMsg == WM_COMMAND) && (id == wParam)) \
	{ \
		bHandled = TRUE; \
		lResult = func((WORD)wParam, (LONG)lParam, bHandled); \
		if(bHandled) \
			return TRUE; \
	}

#define RIBBON_FLOATSPINNER_CONTROL_HANDLER(id, func) \
	if((uMsg == WM_COMMAND) && (id == wParam)) \
	{ \
		bHandled = TRUE; \
		lResult = func((WORD)wParam, (DOUBLE*)lParam, bHandled); \
		if(bHandled) \
			return TRUE; \
	}

// Handler prototypes
/*
	LRESULT OnRibbonGalleryCtrl(UI_EXECUTIONVERB verb, WORD wID, UINT uSel, BOOL& bHandled);
	LRESULT OnRibbonComboCtrl(UI_EXECUTIONVERB verb, WORD wID, UINT uSel, BOOL& bHandled);
	LRESULT OnRibbonFontCtrl(UI_EXECUTIONVERB verb, WORD wID, CHARFORMAT2* pcf, BOOL& bHandled);
	LRESULT OnRibbonColorCtrl(UI_EXECUTIONVERB verb, WORD wID, COLORREF color, BOOL& bHandled);
	LRESULT OnRibbonSpinnerCtrl(WORD wID, LONG lVal, BOOL& bHandled);
	LRESULT OnRibbonFloatSpinnerCtrl(WORD wID, DOUBLE* pdVal, BOOL& bHandled);
*/


///////////////////////////////////////////////////////////////////////////////
// Ribbon frame classes

// CRibbonFrameWindowImplBase
//
template <class T, class TFrameImpl>
class ATL_NO_VTABLE CRibbonFrameWindowImplBase : public TFrameImpl, public RibbonUI::CRibbonImpl<T>
{
	typedef TFrameImpl baseFrame;
	bool m_bUseCommandBarBitmaps;
	bool m_bWin7Fix;

public:
// Construction
	CRibbonFrameWindowImplBase(bool bUseCommandBarBitmaps = true) : 
			m_bUseCommandBarBitmaps(bUseCommandBarBitmaps), m_bWin7Fix(false)
	{
		__if_not_exists(T::m_CmdBar)
		{
			m_bUseCommandBarBitmaps = false;
		}
	}

// Win7 Aero fix helpers
	void ResetFrame()
	{
		const MARGINS margins = { 0, 0, 0, 0 };
		::DwmExtendFrameIntoClientArea(this->m_hWnd, &margins);
	}

	INT CalcWin7Fix()
	{
		ResetFrame();
		RECT rc = {};
		::AdjustWindowRectEx(&rc, T::GetWndStyle(0), this->GetMenu() != NULL, T::GetWndExStyle(0));
		return -rc.top;
	}

	bool NeedWin7Fix()
	{
		BOOL bComp = FALSE;
		return m_bWin7Fix && RunTimeHelper::IsWin7() && SUCCEEDED(DwmIsCompositionEnabled(&bComp)) && bComp;
	}

// Operations
	bool UseCommandBarBitmaps(bool bUse)
	{
		__if_exists(T::m_CmdBar)
		{
			return m_bUseCommandBarBitmaps = bUse;
		}
		__if_not_exists(T::m_CmdBar)
		{
			(void)bUse;   // avoid level 4 warning
			return false;
		}
	}

	bool ShowRibbonUI(bool bShow, INT32 imodes = UI_MAKEAPPMODE(0), LPCWSTR sResName = L"APPLICATION_RIBBON")
	{
		if (!RunTimeHelper::IsRibbonUIAvailable())
			return false;

		ATLASSERT(this->GetIUIFrameworkPtr());

		if (this->IsRibbonUI() == bShow)
			return bShow;

		bool bVisible = (this->IsWindowVisible() != FALSE);
		if(bVisible && !bShow)
			this->SetRedraw(FALSE);

		if (bShow && ::IsWindow(this->m_hWndToolBar))
		{
			::ShowWindow(this->m_hWndToolBar, SW_HIDE);
			UpdateLayout();
		}

		m_bWin7Fix = !bShow;

		HRESULT hr = bShow ? this->CreateRibbon(sResName) : this->DestroyRibbon();

		m_bWin7Fix = SUCCEEDED(hr) && !bShow;

		if (SUCCEEDED(hr))
		{
			if(::IsWindow(this->m_hWndToolBar) && !bShow)
			{
				::ShowWindow(this->m_hWndToolBar, SW_SHOWNA);
				UpdateLayout(); 
			}
			else if (bShow)
			{
				this->PostMessage(WM_SIZE);
				this->SetRibbonModes(imodes);
			}
		}

		if(bVisible && !bShow)
		{
			this->SetRedraw(TRUE);
			this->RedrawWindow(NULL, NULL, RDW_FRAME | RDW_ERASE | RDW_INVALIDATE | RDW_UPDATENOW | RDW_ALLCHILDREN);
		}

		return SUCCEEDED(hr) ? bShow : !bShow;
	}

// Overrideables
	HBITMAP OnRibbonQueryImage(UINT nCmdID, REFPROPERTYKEY key)
	{
		if ((key == UI_PKEY_SmallImage) && m_bUseCommandBarBitmaps)
		{
			if (HBITMAP hbm = GetCommandBarBitmap(nCmdID))
				return (HBITMAP)::CopyImage(hbm, IMAGE_BITMAP, 0, 0, LR_CREATEDIBSECTION);
		}

		return this->DefRibbonQueryImage(nCmdID);
	}

	BEGIN_MSG_MAP(CRibbonFrameWindowImplBase)
		if (!this->IsRibbonUI() && NeedWin7Fix())
		{
			MESSAGE_HANDLER(WM_SIZING, OnSizing)
			MESSAGE_HANDLER(WM_SIZE, OnSize)
			MESSAGE_HANDLER(WM_ACTIVATE, OnActivate)
			MESSAGE_HANDLER(WM_NCCALCSIZE, OnNCCalcSize)
		}
		CHAIN_MSG_MAP(CRibbonUpdateUI<T>)
		CHAIN_MSG_MAP(baseFrame)
	END_MSG_MAP()

// Message handlers for Win7 Aero
	LRESULT OnSizing(UINT /*uMsg*/, WPARAM wParam, LPARAM lParam, BOOL& /*bHandled*/)
	{
		switch (wParam)
		{		
		case WMSZ_TOP:
		case WMSZ_TOPLEFT:
		case WMSZ_TOPRIGHT:
			this->SetWindowPos(NULL, (LPRECT)lParam, SWP_NOMOVE | SWP_NOZORDER | SWP_FRAMECHANGED);
			break;
		default:
			this->DefWindowProc();
			break;
		}

		return 1; // handled
	}

	LRESULT OnSize(UINT /*uMsg*/, WPARAM wParam, LPARAM /*lParam*/, BOOL& bHandled)
	{
		if (wParam != SIZE_MINIMIZED)
			this->SetWindowPos(NULL, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_FRAMECHANGED);

		bHandled = FALSE;
		return 1;
	}

	LRESULT OnActivate(UINT /*uMsg*/, WPARAM wParam, LPARAM /*lParam*/, BOOL& bHandled)
	{
		if(wParam != WA_INACTIVE)
			this->SetWindowPos(NULL, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_FRAMECHANGED);

		bHandled = FALSE;
		return 1;
	}

	LRESULT OnNCCalcSize(UINT /*uMsg*/, WPARAM wParam, LPARAM lParam, BOOL& /*bHandled*/)
	{
		ATLASSERT(!this->IsRibbonUI() && NeedWin7Fix());

		LRESULT lRet = this->DefWindowProc();

		if(wParam)
		{
			LPNCCALCSIZE_PARAMS pParams = (LPNCCALCSIZE_PARAMS)lParam;
			pParams->rgrc[0].top = pParams->rgrc[1].top + CalcWin7Fix();
		}

		return lRet;
	}

// Overrides
	void UpdateLayout(BOOL bResizeBars = TRUE)
	{
		RECT rect = {};
		this->GetClientRect(&rect);

		if (this->IsRibbonUI() && !this->IsRibbonHidden())
		{
			rect.top += this->GetRibbonHeight();
		}
		else if (!this->IsRibbonUI() && NeedWin7Fix())
		{
			ResetFrame();
		}

		// position bars and offset their dimensions
		this->UpdateBarsPosition(rect, bResizeBars);

		// resize client window
		if(this->m_hWndClient != NULL)
			::SetWindowPos(this->m_hWndClient, NULL, rect.left, rect.top,
				rect.right - rect.left, rect.bottom - rect.top,
				SWP_NOZORDER | SWP_NOACTIVATE);
	}

	// Implementation
	HBITMAP GetCommandBarBitmap(UINT nCmdID)
	{
		__if_exists (T::m_CmdBar)
		{
			ATLASSERT(RunTimeHelper::IsVista());
			T* pT =static_cast<T*>(this);
			int nIndex = pT->m_CmdBar.m_arrCommand.Find((WORD&)nCmdID);
			return (nIndex == -1) ? NULL : pT->m_CmdBar.m_arrVistaBitmap[nIndex];
		}
		__if_not_exists (T::m_CmdBar)
		{
			(void)nCmdID;   // avoid level 4 warning
			return NULL;
		}
	}
};

// CRibbonFrameWindowImpl
//
template <class T, class TBase = ATL::CWindow, class TWinTraits = ATL::CFrameWinTraits>
class ATL_NO_VTABLE CRibbonFrameWindowImpl : public CRibbonFrameWindowImplBase<T, CFrameWindowImpl<T, TBase, TWinTraits>>
{ };

// CRibbonMDIFrameWindowImpl
//
template <class T, class TBase = CMDIWindow, class TWinTraits = ATL::CFrameWinTraits>
class ATL_NO_VTABLE CRibbonMDIFrameWindowImpl : public CRibbonFrameWindowImplBase<T, CMDIFrameWindowImpl<T, TBase, TWinTraits>>
{ };


///////////////////////////////////////////////////////////////////////////////
// CRibbonPersist helper for RibbonUI persistency

class CRibbonPersist
{
public:
	CRibbonPersist(LPCWSTR sAppKey)
	{
		ATLASSERT(sAppKey && *sAppKey);
		m_Key.Create(HKEY_CURRENT_USER, sAppKey);
		ATLASSERT(m_Key.m_hKey);
	}

	ATL::CRegKey m_Key;

	LONG Save(bool bRibbonUI, HGLOBAL hgSettings = NULL)
	{
		ATL::CRegKey key;
		const DWORD dwUI = bRibbonUI;

		LONG lRet = key.Create(m_Key, L"Ribbon");
		if(lRet != ERROR_SUCCESS)
			return lRet;
		
		lRet = key.SetDWORDValue(L"UI", dwUI);
		if(lRet != ERROR_SUCCESS)
			return lRet;

		if (hgSettings != NULL)
		{
			LPBYTE pVal = (LPBYTE)::GlobalLock(hgSettings);
			if (pVal != NULL)
			{
				lRet = key.SetBinaryValue(L"Settings", pVal, (ULONG)::GlobalSize(hgSettings));
				::GlobalUnlock(hgSettings);
			}
			else
			{
				lRet = GetLastError();
			}
		}

		return lRet;
	}

	LONG Restore(bool& bRibbonUI, HGLOBAL& hgSettings)
	{
		ATLASSERT(hgSettings == NULL);

		ATL::CRegKey key;

		LONG lRet = key.Open(m_Key, L"Ribbon");
		if(lRet != ERROR_SUCCESS)
			return lRet;
		
		DWORD dwUI = 0xffff;
		lRet = key.QueryDWORDValue(L"UI", dwUI);
		if(lRet == ERROR_SUCCESS)
			bRibbonUI = dwUI == 1;
		else
			return lRet;

		ULONG ulSize = 0;
		lRet = key.QueryBinaryValue(L"Settings", NULL, &ulSize);
		if (lRet == ERROR_SUCCESS)
		{
			ATLASSERT(ulSize != 0);
			
			hgSettings = ::GlobalAlloc(GHND, ulSize);
			if (hgSettings != NULL)
			{
				LPBYTE pData = (LPBYTE)::GlobalLock(hgSettings);
				if (pData != NULL)
				{
					lRet = key.QueryBinaryValue(L"Settings", pData, &ulSize);
				}
				else
				{
					lRet = GetLastError();
					::GlobalFree(hgSettings);
					hgSettings = NULL;
				}
			}
			else
			{
				lRet = GetLastError();
			}
		}
		return lRet;
	}

	LONG Delete()
	{
		return m_Key.DeleteSubKey(L"Ribbon");
	}
};

} // namespace WTL

#endif // __ATLRIBBON_H__
