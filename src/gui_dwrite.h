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

#ifndef GUI_DWRITE_H
#define GUI_DWRITE_H

#ifdef __cplusplus
extern "C" {
#endif

typedef struct DWriteContext DWriteContext;

typedef struct DWriteRenderingParams {
    float gamma;
    float enhancedContrast;
    float clearTypeLevel;
    /*
     * pixelGeometry:
     *	0 - DWRITE_PIXEL_GEOMETRY_FLAT
     *	1 - DWRITE_PIXEL_GEOMETRY_RGB
     *	2 - DWRITE_PIXEL_GEOMETRY_BGR
     */
    int pixelGeometry;
    /*
     * renderingMode:
     *	0 - DWRITE_RENDERING_MODE_DEFAULT
     *	1 - DWRITE_RENDERING_MODE_ALIASED
     *	2 - DWRITE_RENDERING_MODE_CLEARTYPE_GDI_CLASSIC
     *	3 - DWRITE_RENDERING_MODE_CLEARTYPE_GDI_NATURAL
     *	4 - DWRITE_RENDERING_MODE_CLEARTYPE_NATURAL
     *	5 - DWRITE_RENDERING_MODE_CLEARTYPE_NATURAL_SYMMETRIC
     *	6 - DWRITE_RENDERING_MODE_OUTLINE
     */
    int renderingMode;
    /*
     * antialiasMode:
     *	0 - D2D1_TEXT_ANTIALIAS_MODE_DEFAULT
     *	1 - D2D1_TEXT_ANTIALIAS_MODE_CLEARTYPE
     *	2 - D2D1_TEXT_ANTIALIAS_MODE_GRAYSCALE
     *	3 - D2D1_TEXT_ANTIALIAS_MODE_ALIASED
     */
    int textAntialiasMode;
} DWriteRenderingParams;

void DWrite_Init(void);
void DWrite_Final(void);

DWriteContext *DWriteContext_Open(void);
void DWriteContext_BindDC(DWriteContext *ctx, HDC hdc, const RECT *rect);
void DWriteContext_SetFont(DWriteContext *ctx, HFONT hFont);
void DWriteContext_DrawText(
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
	const INT *lpDx);
void DWriteContext_FillRect(DWriteContext *ctx, const RECT *rc, COLORREF color);
void DWriteContext_DrawLine(DWriteContext *ctx, int x1, int y1, int x2, int y2,
	COLORREF color);
void DWriteContext_SetPixel(DWriteContext *ctx, int x, int y, COLORREF color);
void DWriteContext_Scroll(DWriteContext *ctx, int x, int y, const RECT *rc);
void DWriteContext_Flush(DWriteContext *ctx);
void DWriteContext_Close(DWriteContext *ctx);

void DWriteContext_SetRenderingParams(
	DWriteContext *ctx,
	const DWriteRenderingParams *params);

DWriteRenderingParams *DWriteContext_GetRenderingParams(
	DWriteContext *ctx,
	DWriteRenderingParams *params);

#ifdef __cplusplus
}
#endif
#endif/*GUI_DWRITE_H*/
