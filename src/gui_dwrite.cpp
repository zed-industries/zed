/* vi:set ts=8 sts=4 sw=4 noet: */
/*
 * Author: MURAOKA Taro <koron.kaoriya@gmail.com>
 *
 * Contributors:
 *  - Ken Takata
 *  - Yasuhiro Matsumoto
 *
 * Copyright (C) 2013 MURAOKA Taro <koron.kaoriya@gmail.com>
 * THIS FILE IS DISTRIBUTED UNDER THE VIM LICENSE.
 */

#define WIN32_LEAN_AND_MEAN

#ifndef DYNAMIC_DIRECTX
# if WINVER < 0x0600
#  error WINVER must be 0x0600 or above to use DirectWrite(DirectX)
# endif
#endif

#include <windows.h>
#include <crtdbg.h>
#include <assert.h>
#include <math.h>
#include <d2d1.h>
#include <d2d1helper.h>

// Disable these macros to compile with old VC and newer SDK (V8.1 or later).
#if defined(_MSC_VER) && (_MSC_VER < 1700)
# define _COM_Outptr_ __out
# define _In_reads_(s)
# define _In_reads_opt_(s)
# define _Maybenull_
# define _Out_writes_(s)
# define _Out_writes_opt_(s)
# define _Out_writes_to_(x, y)
# define _Out_writes_to_opt_(x, y)
# define _Outptr_
#endif

#ifdef FEAT_DIRECTX_COLOR_EMOJI
# include <dwrite_2.h>
#else
# include <dwrite.h>
#endif

#include "gui_dwrite.h"

#ifdef __MINGW32__
# define __maybenull	SAL__maybenull
# define __in		SAL__in
# define __out		SAL__out
#endif

#ifdef __MINGW32__
# define UNUSED __attribute__((unused))
#else
# define UNUSED
#endif

#if (defined(_MSC_VER) && (_MSC_VER >= 1700)) || (__cplusplus >= 201103L)
# define FINAL final
#else
# define FINAL
#endif

#ifdef DYNAMIC_DIRECTX
extern "C" HINSTANCE vimLoadLib(const char *name);

typedef int (WINAPI *PGETUSERDEFAULTLOCALENAME)(LPWSTR, int);
typedef HRESULT (WINAPI *PD2D1CREATEFACTORY)(D2D1_FACTORY_TYPE,
	REFIID, const D2D1_FACTORY_OPTIONS *, void **);
typedef HRESULT (WINAPI *PDWRITECREATEFACTORY)(DWRITE_FACTORY_TYPE,
	REFIID, IUnknown **);

static HINSTANCE hD2D1DLL = NULL;
static HINSTANCE hDWriteDLL = NULL;

static PGETUSERDEFAULTLOCALENAME pGetUserDefaultLocaleName = NULL;
static PD2D1CREATEFACTORY pD2D1CreateFactory = NULL;
static PDWRITECREATEFACTORY pDWriteCreateFactory = NULL;

#define GetUserDefaultLocaleName	(*pGetUserDefaultLocaleName)
#define D2D1CreateFactory		(*pD2D1CreateFactory)
#define DWriteCreateFactory		(*pDWriteCreateFactory)

    static void
unload(HINSTANCE &hinst)
{
    if (hinst != NULL)
    {
	FreeLibrary(hinst);
	hinst = NULL;
    }
}
#endif // DYNAMIC_DIRECTX

template <class T> inline void SafeRelease(T **ppT)
{
    if (*ppT)
    {
	(*ppT)->Release();
	*ppT = NULL;
    }
}

    static DWRITE_PIXEL_GEOMETRY
ToPixelGeometry(int value)
{
    switch (value)
    {
	default:
	case 0:
	    return DWRITE_PIXEL_GEOMETRY_FLAT;
	case 1:
	    return DWRITE_PIXEL_GEOMETRY_RGB;
	case 2:
	    return DWRITE_PIXEL_GEOMETRY_BGR;
    }
}

    static int
ToInt(DWRITE_PIXEL_GEOMETRY value)
{
    switch (value)
    {
	case DWRITE_PIXEL_GEOMETRY_FLAT:
	    return 0;
	case DWRITE_PIXEL_GEOMETRY_RGB:
	    return 1;
	case DWRITE_PIXEL_GEOMETRY_BGR:
	    return 2;
	default:
	    return -1;
    }
}

    static DWRITE_RENDERING_MODE
ToRenderingMode(int value)
{
    switch (value)
    {
	default:
	case 0:
	    return DWRITE_RENDERING_MODE_DEFAULT;
	case 1:
	    return DWRITE_RENDERING_MODE_ALIASED;
	case 2:
	    return DWRITE_RENDERING_MODE_CLEARTYPE_GDI_CLASSIC;
	case 3:
	    return DWRITE_RENDERING_MODE_CLEARTYPE_GDI_NATURAL;
	case 4:
	    return DWRITE_RENDERING_MODE_CLEARTYPE_NATURAL;
	case 5:
	    return DWRITE_RENDERING_MODE_CLEARTYPE_NATURAL_SYMMETRIC;
	case 6:
	    return DWRITE_RENDERING_MODE_OUTLINE;
    }
}

    static D2D1_TEXT_ANTIALIAS_MODE
ToTextAntialiasMode(int value)
{
    switch (value)
    {
	default:
	case 0:
	    return D2D1_TEXT_ANTIALIAS_MODE_DEFAULT;
	case 1:
	    return D2D1_TEXT_ANTIALIAS_MODE_CLEARTYPE;
	case 2:
	    return D2D1_TEXT_ANTIALIAS_MODE_GRAYSCALE;
	case 3:
	    return D2D1_TEXT_ANTIALIAS_MODE_ALIASED;
    }
}

    static int
ToInt(DWRITE_RENDERING_MODE value)
{
    switch (value)
    {
	case DWRITE_RENDERING_MODE_DEFAULT:
	    return 0;
	case DWRITE_RENDERING_MODE_ALIASED:
	    return 1;
	case DWRITE_RENDERING_MODE_CLEARTYPE_GDI_CLASSIC:
	    return 2;
	case DWRITE_RENDERING_MODE_CLEARTYPE_GDI_NATURAL:
	    return 3;
	case DWRITE_RENDERING_MODE_CLEARTYPE_NATURAL:
	    return 4;
	case DWRITE_RENDERING_MODE_CLEARTYPE_NATURAL_SYMMETRIC:
	    return 5;
	case DWRITE_RENDERING_MODE_OUTLINE:
	    return 6;
	default:
	    return -1;
    }
}

class FontCache {
public:
    struct Item {
	HFONT              hFont;
	IDWriteTextFormat* pTextFormat;
	DWRITE_FONT_WEIGHT fontWeight;
	DWRITE_FONT_STYLE  fontStyle;
	Item() : hFont(NULL), pTextFormat(NULL) {}
    };

private:
    int mSize;
    Item *mItems;

public:
    FontCache(int size = 2) :
	mSize(size),
	mItems(new Item[size])
    {
    }

    ~FontCache()
    {
	for (int i = 0; i < mSize; ++i)
	    SafeRelease(&mItems[i].pTextFormat);
	delete[] mItems;
    }

    bool get(HFONT hFont, Item &item)
    {
	int n = find(hFont);
	if (n < 0)
	    return false;
	item = mItems[n];
	slide(n);
	return true;
    }

    void put(const Item& item)
    {
	int n = find(item.hFont);
	if (n < 0)
	    n = mSize - 1;
	if (mItems[n].pTextFormat != item.pTextFormat)
	{
	    SafeRelease(&mItems[n].pTextFormat);
	    if (item.pTextFormat != NULL)
		item.pTextFormat->AddRef();
	}
	mItems[n] = item;
	slide(n);
    }

private:
    int find(HFONT hFont)
    {
	for (int i = 0; i < mSize; ++i)
	{
	    if (mItems[i].hFont == hFont)
		return i;
	}
	return -1;
    }

    void slide(int nextTop)
    {
	if (nextTop == 0)
	    return;
	Item tmp = mItems[nextTop];
	for (int i = nextTop - 1; i >= 0; --i)
	    mItems[i + 1] = mItems[i];
	mItems[0] = tmp;
    }
};

enum DrawingMode {
    DM_GDI = 0,
    DM_DIRECTX = 1,
    DM_INTEROP = 2,
};

struct DWriteContext {
    HDC mHDC;
    RECT mBindRect;
    DrawingMode mDMode;
    HDC mInteropHDC;
    bool mDrawing;
    bool mFallbackDC;

    ID2D1Factory *mD2D1Factory;

    ID2D1DCRenderTarget *mRT;
    ID2D1GdiInteropRenderTarget *mGDIRT;
    ID2D1SolidColorBrush *mBrush;
    ID2D1Bitmap *mBitmap;

    IDWriteFactory *mDWriteFactory;
#ifdef FEAT_DIRECTX_COLOR_EMOJI
    IDWriteFactory2 *mDWriteFactory2;
#endif

    IDWriteGdiInterop *mGdiInterop;
    IDWriteRenderingParams *mRenderingParams;

    FontCache mFontCache;
    IDWriteTextFormat *mTextFormat;
    DWRITE_FONT_WEIGHT mFontWeight;
    DWRITE_FONT_STYLE mFontStyle;

    D2D1_TEXT_ANTIALIAS_MODE mTextAntialiasMode;

    // METHODS

    DWriteContext();

    virtual ~DWriteContext();

    HRESULT CreateDeviceResources();

    void DiscardDeviceResources();

    HRESULT CreateTextFormatFromLOGFONT(const LOGFONTW &logFont,
	    IDWriteTextFormat **ppTextFormat);

    HRESULT SetFontByLOGFONT(const LOGFONTW &logFont);

    void SetFont(HFONT hFont);

    void Rebind();

    void BindDC(HDC hdc, const RECT *rect);

    HRESULT SetDrawingMode(DrawingMode mode);

    ID2D1Brush* SolidBrush(COLORREF color);

    void DrawText(const WCHAR *text, int len,
	int x, int y, int w, int h, int cellWidth, COLORREF color,
	UINT fuOptions, const RECT *lprc, const INT *lpDx);

    void FillRect(const RECT *rc, COLORREF color);

    void DrawLine(int x1, int y1, int x2, int y2, COLORREF color);

    void SetPixel(int x, int y, COLORREF color);

    void Scroll(int x, int y, const RECT *rc);

    void Flush();

    void SetRenderingParams(
	    const DWriteRenderingParams *params);

    DWriteRenderingParams *GetRenderingParams(
	    DWriteRenderingParams *params);
};

class AdjustedGlyphRun : public DWRITE_GLYPH_RUN
{
private:
    FLOAT &mAccum;
    FLOAT mDelta;
    FLOAT *mAdjustedAdvances;

public:
    AdjustedGlyphRun(
	    const DWRITE_GLYPH_RUN *glyphRun,
	    FLOAT cellWidth,
	    FLOAT &accum) :
	DWRITE_GLYPH_RUN(*glyphRun),
	mAccum(accum),
	mDelta(0.0f),
	mAdjustedAdvances(new FLOAT[glyphRun->glyphCount])
    {
	assert(cellWidth != 0.0f);
	for (UINT32 i = 0; i < glyphRun->glyphCount; ++i)
	{
	    FLOAT orig = glyphRun->glyphAdvances[i];
	    FLOAT adjusted = adjustToCell(orig, cellWidth);
	    mAdjustedAdvances[i] = adjusted;
	    mDelta += adjusted - orig;
	}
	glyphAdvances = mAdjustedAdvances;
    }

    ~AdjustedGlyphRun()
    {
	mAccum += mDelta;
	delete[] mAdjustedAdvances;
    }

    static FLOAT adjustToCell(FLOAT value, FLOAT cellWidth)
    {
	int cellCount = int(floor(value / cellWidth + 0.5f));
	if (cellCount < 1)
	    cellCount = 1;
	return cellCount * cellWidth;
    }
};

struct TextRendererContext {
    // const fields.
    COLORREF color;
    FLOAT cellWidth;

    // working fields.
    FLOAT offsetX;
};

class TextRenderer FINAL : public IDWriteTextRenderer
{
public:
    TextRenderer(
	    DWriteContext* pDWC) :
	cRefCount_(0),
	pDWC_(pDWC)
    {
	AddRef();
    }

    // add "virtual" to avoid a compiler warning
    virtual ~TextRenderer()
    {
    }

    IFACEMETHOD(IsPixelSnappingDisabled)(
	__maybenull void* clientDrawingContext UNUSED,
	__out BOOL* isDisabled)
    {
	*isDisabled = FALSE;
	return S_OK;
    }

    IFACEMETHOD(GetCurrentTransform)(
	__maybenull void* clientDrawingContext UNUSED,
	__out DWRITE_MATRIX* transform)
    {
	// forward the render target's transform
	pDWC_->mRT->GetTransform(
		reinterpret_cast<D2D1_MATRIX_3X2_F*>(transform));
	return S_OK;
    }

    IFACEMETHOD(GetPixelsPerDip)(
	__maybenull void* clientDrawingContext UNUSED,
	__out FLOAT* pixelsPerDip)
    {
	float dpiX, unused;
	pDWC_->mRT->GetDpi(&dpiX, &unused);
	*pixelsPerDip = dpiX / 96.0f;
	return S_OK;
    }

    IFACEMETHOD(DrawUnderline)(
	__maybenull void* clientDrawingContext UNUSED,
	FLOAT baselineOriginX UNUSED,
	FLOAT baselineOriginY UNUSED,
	__in DWRITE_UNDERLINE const* underline UNUSED,
	IUnknown* clientDrawingEffect UNUSED)
    {
	return E_NOTIMPL;
    }

    IFACEMETHOD(DrawStrikethrough)(
	__maybenull void* clientDrawingContext UNUSED,
	FLOAT baselineOriginX UNUSED,
	FLOAT baselineOriginY UNUSED,
	__in DWRITE_STRIKETHROUGH const* strikethrough UNUSED,
	IUnknown* clientDrawingEffect UNUSED)
    {
	return E_NOTIMPL;
    }

    IFACEMETHOD(DrawInlineObject)(
	__maybenull void* clientDrawingContext UNUSED,
	FLOAT originX UNUSED,
	FLOAT originY UNUSED,
	IDWriteInlineObject* inlineObject UNUSED,
	BOOL isSideways UNUSED,
	BOOL isRightToLeft UNUSED,
	IUnknown* clientDrawingEffect UNUSED)
    {
	return E_NOTIMPL;
    }

    IFACEMETHOD(DrawGlyphRun)(
	__maybenull void* clientDrawingContext,
	FLOAT baselineOriginX,
	FLOAT baselineOriginY,
	DWRITE_MEASURING_MODE measuringMode UNUSED,
	__in DWRITE_GLYPH_RUN const* glyphRun,
	__in DWRITE_GLYPH_RUN_DESCRIPTION const* glyphRunDescription UNUSED,
	IUnknown* clientDrawingEffect UNUSED)
    {
	TextRendererContext *context =
	    reinterpret_cast<TextRendererContext*>(clientDrawingContext);

	AdjustedGlyphRun adjustedGlyphRun(glyphRun, context->cellWidth,
		context->offsetX);

#ifdef FEAT_DIRECTX_COLOR_EMOJI
	if (pDWC_->mDWriteFactory2 != NULL)
	{
	    IDWriteColorGlyphRunEnumerator *enumerator = NULL;
	    HRESULT hr = pDWC_->mDWriteFactory2->TranslateColorGlyphRun(
		baselineOriginX + context->offsetX,
		baselineOriginY,
		&adjustedGlyphRun,
		NULL,
		DWRITE_MEASURING_MODE_GDI_NATURAL,
		NULL,
		0,
		&enumerator);
	    if (SUCCEEDED(hr))
	    {
		// Draw by IDWriteFactory2 for color emoji
		BOOL hasRun = TRUE;
		enumerator->MoveNext(&hasRun);
		while (hasRun)
		{
		    const DWRITE_COLOR_GLYPH_RUN* colorGlyphRun;
		    enumerator->GetCurrentRun(&colorGlyphRun);

		    pDWC_->mBrush->SetColor(colorGlyphRun->runColor);
		    pDWC_->mRT->DrawGlyphRun(
			    D2D1::Point2F(
				colorGlyphRun->baselineOriginX,
				colorGlyphRun->baselineOriginY),
			    &colorGlyphRun->glyphRun,
			    pDWC_->mBrush,
			    DWRITE_MEASURING_MODE_NATURAL);
		    enumerator->MoveNext(&hasRun);
		}
		SafeRelease(&enumerator);
		return S_OK;
	    }
	}
#endif

	// Draw by IDWriteFactory (without color emoji)
	pDWC_->mRT->DrawGlyphRun(
		D2D1::Point2F(
		    baselineOriginX + context->offsetX,
		    baselineOriginY),
		&adjustedGlyphRun,
		pDWC_->SolidBrush(context->color),
		DWRITE_MEASURING_MODE_NATURAL);
	return S_OK;
    }

public:
    IFACEMETHOD_(unsigned long, AddRef) ()
    {
	return InterlockedIncrement(&cRefCount_);
    }

    IFACEMETHOD_(unsigned long, Release) ()
    {
	long newCount = InterlockedDecrement(&cRefCount_);

	if (newCount == 0)
	{
	    delete this;
	    return 0;
	}
	return newCount;
    }

    IFACEMETHOD(QueryInterface)(
	IID const& riid,
	void** ppvObject)
    {
	if (__uuidof(IDWriteTextRenderer) == riid)
	{
	    *ppvObject = this;
	}
	else if (__uuidof(IDWritePixelSnapping) == riid)
	{
	    *ppvObject = this;
	}
	else if (__uuidof(IUnknown) == riid)
	{
	    *ppvObject = this;
	}
	else
	{
	    *ppvObject = NULL;
	    return E_FAIL;
	}

	return S_OK;
    }

private:
    long cRefCount_;
    DWriteContext* pDWC_;
};

DWriteContext::DWriteContext() :
    mHDC(NULL),
    mBindRect(),
    mDMode(DM_GDI),
    mInteropHDC(NULL),
    mDrawing(false),
    mFallbackDC(false),
    mD2D1Factory(NULL),
    mRT(NULL),
    mGDIRT(NULL),
    mBrush(NULL),
    mBitmap(NULL),
    mDWriteFactory(NULL),
#ifdef FEAT_DIRECTX_COLOR_EMOJI
    mDWriteFactory2(NULL),
#endif
    mGdiInterop(NULL),
    mRenderingParams(NULL),
    mFontCache(8),
    mTextFormat(NULL),
    mFontWeight(DWRITE_FONT_WEIGHT_NORMAL),
    mFontStyle(DWRITE_FONT_STYLE_NORMAL),
    mTextAntialiasMode(D2D1_TEXT_ANTIALIAS_MODE_DEFAULT)
{
    HRESULT hr;

    hr = D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED,
	    __uuidof(ID2D1Factory), NULL,
	    reinterpret_cast<void**>(&mD2D1Factory));
    _RPT2(_CRT_WARN, "D2D1CreateFactory: hr=%p p=%p\n", hr, mD2D1Factory);

    if (SUCCEEDED(hr))
    {
	hr = DWriteCreateFactory(
		DWRITE_FACTORY_TYPE_SHARED,
		__uuidof(IDWriteFactory),
		reinterpret_cast<IUnknown**>(&mDWriteFactory));
	_RPT2(_CRT_WARN, "DWriteCreateFactory: hr=%p p=%p\n", hr,
		mDWriteFactory);
    }

#ifdef FEAT_DIRECTX_COLOR_EMOJI
    if (SUCCEEDED(hr))
    {
	DWriteCreateFactory(
		DWRITE_FACTORY_TYPE_SHARED,
		__uuidof(IDWriteFactory2),
		reinterpret_cast<IUnknown**>(&mDWriteFactory2));
	_RPT1(_CRT_WARN, "IDWriteFactory2: %s\n", SUCCEEDED(hr) ? "available" : "not available");
    }
#endif

    if (SUCCEEDED(hr))
    {
	hr = mDWriteFactory->GetGdiInterop(&mGdiInterop);
	_RPT2(_CRT_WARN, "GetGdiInterop: hr=%p p=%p\n", hr, mGdiInterop);
    }

    if (SUCCEEDED(hr))
    {
	hr = mDWriteFactory->CreateRenderingParams(&mRenderingParams);
	_RPT2(_CRT_WARN, "CreateRenderingParams: hr=%p p=%p\n", hr,
		mRenderingParams);
    }
}

DWriteContext::~DWriteContext()
{
    SafeRelease(&mTextFormat);
    SafeRelease(&mRenderingParams);
    SafeRelease(&mGdiInterop);
    SafeRelease(&mDWriteFactory);
#ifdef FEAT_DIRECTX_COLOR_EMOJI
    SafeRelease(&mDWriteFactory2);
#endif
    SafeRelease(&mBitmap);
    SafeRelease(&mBrush);
    SafeRelease(&mGDIRT);
    SafeRelease(&mRT);
    SafeRelease(&mD2D1Factory);
}

    HRESULT
DWriteContext::CreateDeviceResources()
{
    HRESULT hr;

    if (mRT != NULL)
	return S_OK;

    D2D1_RENDER_TARGET_PROPERTIES props = {
	D2D1_RENDER_TARGET_TYPE_DEFAULT,
	{ DXGI_FORMAT_B8G8R8A8_UNORM, D2D1_ALPHA_MODE_IGNORE },
	0, 0,
	D2D1_RENDER_TARGET_USAGE_GDI_COMPATIBLE,
	D2D1_FEATURE_LEVEL_DEFAULT
    };
    hr = mD2D1Factory->CreateDCRenderTarget(&props, &mRT);
    _RPT2(_CRT_WARN, "CreateDCRenderTarget: hr=%p p=%p\n", hr, mRT);

    if (SUCCEEDED(hr))
    {
	// This always succeeds.
	mRT->QueryInterface(
		__uuidof(ID2D1GdiInteropRenderTarget),
		reinterpret_cast<void**>(&mGDIRT));
	_RPT1(_CRT_WARN, "GdiInteropRenderTarget: p=%p\n", mGDIRT);
    }

    if (SUCCEEDED(hr))
    {
	hr = mRT->CreateSolidColorBrush(
		D2D1::ColorF(D2D1::ColorF::Black),
		&mBrush);
	_RPT2(_CRT_WARN, "CreateSolidColorBrush: hr=%p p=%p\n", hr, mBrush);
    }

    if (SUCCEEDED(hr))
	Rebind();

    return hr;
}

    void
DWriteContext::DiscardDeviceResources()
{
    SafeRelease(&mBitmap);
    SafeRelease(&mBrush);
    SafeRelease(&mGDIRT);
    SafeRelease(&mRT);
}

    HRESULT
DWriteContext::CreateTextFormatFromLOGFONT(const LOGFONTW &logFont,
	IDWriteTextFormat **ppTextFormat)
{
    // Most of this function is copied from: https://github.com/Microsoft/Windows-classic-samples/blob/master/Samples/Win7Samples/multimedia/DirectWrite/RenderTest/TextHelpers.cpp
    HRESULT hr = S_OK;
    IDWriteTextFormat *pTextFormat = NULL;

    IDWriteFont *font = NULL;
    IDWriteFontFamily *fontFamily = NULL;
    IDWriteLocalizedStrings *localizedFamilyNames = NULL;
    float fontSize = 0;

    if (SUCCEEDED(hr))
    {
	hr = mGdiInterop->CreateFontFromLOGFONT(&logFont, &font);
    }

    // Get the font family to which this font belongs.
    if (SUCCEEDED(hr))
    {
	hr = font->GetFontFamily(&fontFamily);
    }

    // Get the family names. This returns an object that encapsulates one or
    // more names with the same meaning but in different languages.
    if (SUCCEEDED(hr))
    {
	hr = fontFamily->GetFamilyNames(&localizedFamilyNames);
    }

    // Get the family name at index zero. If we were going to display the name
    // we'd want to try to find one that matched the use locale, but for
    // purposes of creating a text format object any language will do.

    wchar_t familyName[100];
    if (SUCCEEDED(hr))
    {
	hr = localizedFamilyNames->GetString(0, familyName,
		ARRAYSIZE(familyName));
    }

    if (SUCCEEDED(hr))
    {
	// Use lfHeight of the LOGFONT as font size.
	fontSize = float(logFont.lfHeight);

	if (fontSize < 0)
	{
	    // Negative lfHeight represents the size of the em unit.
	    fontSize = -fontSize;
	}
	else
	{
	    // Positive lfHeight represents the cell height (ascent +
	    // descent).
	    DWRITE_FONT_METRICS fontMetrics;
	    font->GetMetrics(&fontMetrics);

	    // Convert the cell height (ascent + descent) from design units
	    // to ems.
	    float cellHeight = static_cast<float>(
		    fontMetrics.ascent + fontMetrics.descent)
		/ fontMetrics.designUnitsPerEm;

	    // Divide the font size by the cell height to get the font em
	    // size.
	    fontSize /= cellHeight;
	}
    }

    // The text format includes a locale name. Ideally, this would be the
    // language of the text, which may or may not be the same as the primary
    // language of the user. However, for our purposes the user locale will do.
    wchar_t localeName[LOCALE_NAME_MAX_LENGTH];
    if (SUCCEEDED(hr))
    {
	if (GetUserDefaultLocaleName(localeName, LOCALE_NAME_MAX_LENGTH) == 0)
	    hr = HRESULT_FROM_WIN32(GetLastError());
    }

    if (SUCCEEDED(hr))
    {
	// Create the text format object.
	hr = mDWriteFactory->CreateTextFormat(
		familyName,
		NULL, // no custom font collection
		font->GetWeight(),
		font->GetStyle(),
		font->GetStretch(),
		fontSize,
		localeName,
		&pTextFormat);
    }

    if (SUCCEEDED(hr))
	hr = pTextFormat->SetTextAlignment(DWRITE_TEXT_ALIGNMENT_LEADING);

    if (SUCCEEDED(hr))
	hr = pTextFormat->SetParagraphAlignment(
		DWRITE_PARAGRAPH_ALIGNMENT_FAR);

    if (SUCCEEDED(hr))
	hr = pTextFormat->SetWordWrapping(DWRITE_WORD_WRAPPING_NO_WRAP);

    SafeRelease(&localizedFamilyNames);
    SafeRelease(&fontFamily);
    SafeRelease(&font);

    if (SUCCEEDED(hr))
	*ppTextFormat = pTextFormat;
    else
	SafeRelease(&pTextFormat);

    return hr;
}

    HRESULT
DWriteContext::SetFontByLOGFONT(const LOGFONTW &logFont)
{
    HRESULT hr = S_OK;
    IDWriteTextFormat *pTextFormat = NULL;

    hr = CreateTextFormatFromLOGFONT(logFont, &pTextFormat);

    if (SUCCEEDED(hr))
    {
	SafeRelease(&mTextFormat);
	mTextFormat = pTextFormat;
	mFontWeight = static_cast<DWRITE_FONT_WEIGHT>(logFont.lfWeight);
	mFontStyle = logFont.lfItalic ? DWRITE_FONT_STYLE_ITALIC
	    : DWRITE_FONT_STYLE_NORMAL;
    }

    return hr;
}

    void
DWriteContext::SetFont(HFONT hFont)
{
    FontCache::Item item;
    if (mFontCache.get(hFont, item))
    {
	if (item.pTextFormat != NULL)
	{
	    item.pTextFormat->AddRef();
	    SafeRelease(&mTextFormat);
	    mTextFormat = item.pTextFormat;
	    mFontWeight = item.fontWeight;
	    mFontStyle = item.fontStyle;
	    mFallbackDC = false;
	}
	else
	    mFallbackDC = true;
	return;
    }

    HRESULT hr = E_FAIL;
    LOGFONTW lf;
    if (GetObjectW(hFont, sizeof(lf), &lf))
	hr = SetFontByLOGFONT(lf);

    item.hFont = hFont;
    if (SUCCEEDED(hr))
    {
	item.pTextFormat = mTextFormat;
	item.fontWeight = mFontWeight;
	item.fontStyle = mFontStyle;
	mFallbackDC = false;
    }
    else
	mFallbackDC = true;
    mFontCache.put(item);
}

    void
DWriteContext::Rebind()
{
    SafeRelease(&mBitmap);

    mRT->BindDC(mHDC, &mBindRect);
    mRT->SetTransform(D2D1::IdentityMatrix());

    D2D1_BITMAP_PROPERTIES props = {
	{DXGI_FORMAT_B8G8R8A8_UNORM, D2D1_ALPHA_MODE_IGNORE},
	96.0f, 96.0f
    };
    mRT->CreateBitmap(
	    D2D1::SizeU(mBindRect.right - mBindRect.left,
		mBindRect.bottom - mBindRect.top),
	    props, &mBitmap);
}

    void
DWriteContext::BindDC(HDC hdc, const RECT *rect)
{
    mHDC = hdc;
    mBindRect = *rect;

    if (mRT == NULL)
	CreateDeviceResources();
    else
    {
	Flush();
	Rebind();
    }
}

extern "C" void redraw_later_clear(void);

    HRESULT
DWriteContext::SetDrawingMode(DrawingMode mode)
{
    HRESULT hr = S_OK;

    switch (mode)
    {
	default:
	case DM_GDI:
	    if (mInteropHDC != NULL)
	    {
		mGDIRT->ReleaseDC(NULL);
		mInteropHDC = NULL;
	    }
	    if (mDrawing)
	    {
		hr = mRT->EndDraw();
		if (hr == (HRESULT)D2DERR_RECREATE_TARGET)
		{
		    hr = S_OK;
		    DiscardDeviceResources();
		    CreateDeviceResources();
		    redraw_later_clear();
		}
		mDrawing = false;
	    }
	    break;

	case DM_DIRECTX:
	    if (mInteropHDC != NULL)
	    {
		mGDIRT->ReleaseDC(NULL);
		mInteropHDC = NULL;
	    }
	    else if (mDrawing == false)
	    {
		CreateDeviceResources();
		mRT->BeginDraw();
		mDrawing = true;
	    }
	    break;

	case DM_INTEROP:
	    if (mDrawing == false)
	    {
		CreateDeviceResources();
		mRT->BeginDraw();
		mDrawing = true;
	    }
	    if (mInteropHDC == NULL)
		hr = mGDIRT->GetDC(D2D1_DC_INITIALIZE_MODE_COPY, &mInteropHDC);
	    break;
    }
    mDMode = mode;
    return hr;
}

    ID2D1Brush*
DWriteContext::SolidBrush(COLORREF color)
{
    mBrush->SetColor(D2D1::ColorF(UINT32(GetRValue(color)) << 16 |
		UINT32(GetGValue(color)) << 8 | UINT32(GetBValue(color))));
    return mBrush;
}

    void
DWriteContext::DrawText(const WCHAR *text, int len,
	int x, int y, int w, int h, int cellWidth, COLORREF color,
	UINT fuOptions, const RECT *lprc, const INT *lpDx)
{
    if (mFallbackDC)
    {
	// Fall back to GDI rendering.
	HRESULT hr = SetDrawingMode(DM_INTEROP);
	if (SUCCEEDED(hr))
	{
	    HGDIOBJ hFont = ::GetCurrentObject(mHDC, OBJ_FONT);
	    HGDIOBJ hOldFont = ::SelectObject(mInteropHDC, hFont);
	    ::SetTextColor(mInteropHDC, color);
	    ::SetBkMode(mInteropHDC, ::GetBkMode(mHDC));
	    ::ExtTextOutW(mInteropHDC, x, y, fuOptions, lprc, text, len, lpDx);
	    ::SelectObject(mInteropHDC, hOldFont);
	}
	return;
    }

    HRESULT hr;
    IDWriteTextLayout *textLayout = NULL;

    SetDrawingMode(DM_DIRECTX);

    hr = mDWriteFactory->CreateTextLayout(text, len, mTextFormat,
	    FLOAT(w), FLOAT(h), &textLayout);

    if (SUCCEEDED(hr))
    {
	DWRITE_TEXT_RANGE textRange = { 0, UINT32(len) };
	textLayout->SetFontWeight(mFontWeight, textRange);
	textLayout->SetFontStyle(mFontStyle, textRange);

	TextRenderer renderer(this);
	TextRendererContext context = { color, FLOAT(cellWidth), 0.0f };
	textLayout->Draw(&context, &renderer, FLOAT(x), FLOAT(y));
    }

    SafeRelease(&textLayout);
}

    void
DWriteContext::FillRect(const RECT *rc, COLORREF color)
{
    if (mDMode == DM_INTEROP)
    {
	// GDI functions are used before this call.  Keep using GDI.
	// (Switching to Direct2D causes terrible slowdown.)
	HBRUSH hbr = ::CreateSolidBrush(color);
	::FillRect(mInteropHDC, rc, hbr);
	::DeleteObject(HGDIOBJ(hbr));
    }
    else
    {
	SetDrawingMode(DM_DIRECTX);
	mRT->FillRectangle(
		D2D1::RectF(FLOAT(rc->left), FLOAT(rc->top),
		    FLOAT(rc->right), FLOAT(rc->bottom)),
		SolidBrush(color));
    }
}

    void
DWriteContext::DrawLine(int x1, int y1, int x2, int y2, COLORREF color)
{
    if (mDMode == DM_INTEROP)
    {
	// GDI functions are used before this call.  Keep using GDI.
	// (Switching to Direct2D causes terrible slowdown.)
	HPEN hpen = ::CreatePen(PS_SOLID, 1, color);
	HGDIOBJ old_pen = ::SelectObject(mInteropHDC, HGDIOBJ(hpen));
	::MoveToEx(mInteropHDC, x1, y1, NULL);
	::LineTo(mInteropHDC, x2, y2);
	::SelectObject(mInteropHDC, old_pen);
	::DeleteObject(HGDIOBJ(hpen));
    }
    else
    {
	SetDrawingMode(DM_DIRECTX);
	mRT->DrawLine(
		D2D1::Point2F(FLOAT(x1), FLOAT(y1) + 0.5f),
		D2D1::Point2F(FLOAT(x2), FLOAT(y2) + 0.5f),
		SolidBrush(color));
    }
}

    void
DWriteContext::SetPixel(int x, int y, COLORREF color)
{
    if (mDMode == DM_INTEROP)
    {
	// GDI functions are used before this call.  Keep using GDI.
	// (Switching to Direct2D causes terrible slowdown.)
	::SetPixel(mInteropHDC, x, y, color);
    }
    else
    {
	SetDrawingMode(DM_DIRECTX);
	// Direct2D doesn't have SetPixel API.  Use DrawLine instead.
	mRT->DrawLine(
		D2D1::Point2F(FLOAT(x), FLOAT(y) + 0.5f),
		D2D1::Point2F(FLOAT(x+1), FLOAT(y) + 0.5f),
		SolidBrush(color));
    }
}

    void
DWriteContext::Scroll(int x, int y, const RECT *rc)
{
    SetDrawingMode(DM_DIRECTX);
    mRT->Flush();

    D2D1_RECT_U srcRect;
    D2D1_POINT_2U destPoint;
    if (x >= 0)
    {
	srcRect.left = rc->left;
	srcRect.right = rc->right - x;
	destPoint.x = rc->left + x;
    }
    else
    {
	srcRect.left = rc->left - x;
	srcRect.right = rc->right;
	destPoint.x = rc->left;
    }
    if (y >= 0)
    {
	srcRect.top = rc->top;
	srcRect.bottom = rc->bottom - y;
	destPoint.y = rc->top + y;
    }
    else
    {
	srcRect.top = rc->top - y;
	srcRect.bottom = rc->bottom;
	destPoint.y = rc->top;
    }
    mBitmap->CopyFromRenderTarget(&destPoint, mRT, &srcRect);

    D2D1_RECT_F destRect = {
	    FLOAT(destPoint.x), FLOAT(destPoint.y),
	    FLOAT(destPoint.x + srcRect.right - srcRect.left),
	    FLOAT(destPoint.y + srcRect.bottom - srcRect.top)
    };
    mRT->DrawBitmap(mBitmap, destRect, 1.0F,
	    D2D1_BITMAP_INTERPOLATION_MODE_NEAREST_NEIGHBOR, destRect);
}

    void
DWriteContext::Flush()
{
    SetDrawingMode(DM_GDI);
}

    void
DWriteContext::SetRenderingParams(
	const DWriteRenderingParams *params)
{
    if (mDWriteFactory == NULL)
	return;

    IDWriteRenderingParams *renderingParams = NULL;
    D2D1_TEXT_ANTIALIAS_MODE textAntialiasMode =
	D2D1_TEXT_ANTIALIAS_MODE_DEFAULT;
    HRESULT hr;
    if (params != NULL)
    {
	hr = mDWriteFactory->CreateCustomRenderingParams(params->gamma,
		params->enhancedContrast, params->clearTypeLevel,
		ToPixelGeometry(params->pixelGeometry),
		ToRenderingMode(params->renderingMode), &renderingParams);
	textAntialiasMode = ToTextAntialiasMode(params->textAntialiasMode);
    }
    else
	hr = mDWriteFactory->CreateRenderingParams(&renderingParams);
    if (SUCCEEDED(hr) && renderingParams != NULL)
    {
	SafeRelease(&mRenderingParams);
	mRenderingParams = renderingParams;
	mTextAntialiasMode = textAntialiasMode;

	Flush();
	mRT->SetTextRenderingParams(mRenderingParams);
	mRT->SetTextAntialiasMode(mTextAntialiasMode);
    }
}

    DWriteRenderingParams *
DWriteContext::GetRenderingParams(
	DWriteRenderingParams *params)
{
    if (params != NULL && mRenderingParams != NULL)
    {
	params->gamma = mRenderingParams->GetGamma();
	params->enhancedContrast = mRenderingParams->GetEnhancedContrast();
	params->clearTypeLevel = mRenderingParams->GetClearTypeLevel();
	params->pixelGeometry = ToInt(mRenderingParams->GetPixelGeometry());
	params->renderingMode = ToInt(mRenderingParams->GetRenderingMode());
	params->textAntialiasMode = mTextAntialiasMode;
    }
    return params;
}

////////////////////////////////////////////////////////////////////////////
// PUBLIC C INTERFACES

    void
DWrite_Init(void)
{
#ifdef DYNAMIC_DIRECTX
    // Load libraries.
    hD2D1DLL = vimLoadLib("d2d1.dll");
    hDWriteDLL = vimLoadLib("dwrite.dll");
    if (hD2D1DLL == NULL || hDWriteDLL == NULL)
    {
	DWrite_Final();
	return;
    }
    // Get address of procedures.
    pGetUserDefaultLocaleName = (PGETUSERDEFAULTLOCALENAME)GetProcAddress(
	    GetModuleHandle("kernel32.dll"), "GetUserDefaultLocaleName");
    pD2D1CreateFactory = (PD2D1CREATEFACTORY)GetProcAddress(hD2D1DLL,
	    "D2D1CreateFactory");
    pDWriteCreateFactory = (PDWRITECREATEFACTORY)GetProcAddress(hDWriteDLL,
	    "DWriteCreateFactory");
#endif
}

    void
DWrite_Final(void)
{
#ifdef DYNAMIC_DIRECTX
    pGetUserDefaultLocaleName = NULL;
    pD2D1CreateFactory = NULL;
    pDWriteCreateFactory = NULL;
    unload(hDWriteDLL);
    unload(hD2D1DLL);
#endif
}

    DWriteContext *
DWriteContext_Open(void)
{
#ifdef DYNAMIC_DIRECTX
    if (pGetUserDefaultLocaleName == NULL || pD2D1CreateFactory == NULL
	    || pDWriteCreateFactory == NULL)
	return NULL;
#endif
    return new DWriteContext();
}

    void
DWriteContext_BindDC(DWriteContext *ctx, HDC hdc, const RECT *rect)
{
    if (ctx != NULL)
	ctx->BindDC(hdc, rect);
}

    void
DWriteContext_SetFont(DWriteContext *ctx, HFONT hFont)
{
    if (ctx != NULL)
	ctx->SetFont(hFont);
}

    void
DWriteContext_DrawText(
	DWriteContext *ctx,
	const WCHAR *text,
	int len,
	int x,
	int y,
	int w,
	int h,
	int cellWidth,
	COLORREF color,
	UINT fuOptions,
	const RECT *lprc,
	const INT *lpDx)
{
    if (ctx != NULL)
	ctx->DrawText(text, len, x, y, w, h, cellWidth, color,
		fuOptions, lprc, lpDx);
}

    void
DWriteContext_FillRect(DWriteContext *ctx, const RECT *rc, COLORREF color)
{
    if (ctx != NULL)
	ctx->FillRect(rc, color);
}

    void
DWriteContext_DrawLine(DWriteContext *ctx, int x1, int y1, int x2, int y2,
	COLORREF color)
{
    if (ctx != NULL)
	ctx->DrawLine(x1, y1, x2, y2, color);
}

    void
DWriteContext_SetPixel(DWriteContext *ctx, int x, int y, COLORREF color)
{
    if (ctx != NULL)
	ctx->SetPixel(x, y, color);
}

    void
DWriteContext_Scroll(DWriteContext *ctx, int x, int y, const RECT *rc)
{
    if (ctx != NULL)
	ctx->Scroll(x, y, rc);
}

    void
DWriteContext_Flush(DWriteContext *ctx)
{
    if (ctx != NULL)
	ctx->Flush();
}

    void
DWriteContext_Close(DWriteContext *ctx)
{
    delete ctx;
}

    void
DWriteContext_SetRenderingParams(
	DWriteContext *ctx,
	const DWriteRenderingParams *params)
{
    if (ctx != NULL)
	ctx->SetRenderingParams(params);
}

    DWriteRenderingParams *
DWriteContext_GetRenderingParams(
	DWriteContext *ctx,
	DWriteRenderingParams *params)
{
    if (ctx != NULL)
	return ctx->GetRenderingParams(params);
    else
	return NULL;
}
