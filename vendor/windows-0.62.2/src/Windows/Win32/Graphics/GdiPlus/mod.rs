#[inline]
pub unsafe fn GdipAddPathArc(path: *mut GpPath, x: f32, y: f32, width: f32, height: f32, startangle: f32, sweepangle: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathArc(path : *mut GpPath, x : f32, y : f32, width : f32, height : f32, startangle : f32, sweepangle : f32) -> Status);
    unsafe { GdipAddPathArc(path as _, x, y, width, height, startangle, sweepangle) }
}
#[inline]
pub unsafe fn GdipAddPathArcI(path: *mut GpPath, x: i32, y: i32, width: i32, height: i32, startangle: f32, sweepangle: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathArcI(path : *mut GpPath, x : i32, y : i32, width : i32, height : i32, startangle : f32, sweepangle : f32) -> Status);
    unsafe { GdipAddPathArcI(path as _, x, y, width, height, startangle, sweepangle) }
}
#[inline]
pub unsafe fn GdipAddPathBezier(path: *mut GpPath, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, x4: f32, y4: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathBezier(path : *mut GpPath, x1 : f32, y1 : f32, x2 : f32, y2 : f32, x3 : f32, y3 : f32, x4 : f32, y4 : f32) -> Status);
    unsafe { GdipAddPathBezier(path as _, x1, y1, x2, y2, x3, y3, x4, y4) }
}
#[inline]
pub unsafe fn GdipAddPathBezierI(path: *mut GpPath, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, x4: i32, y4: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathBezierI(path : *mut GpPath, x1 : i32, y1 : i32, x2 : i32, y2 : i32, x3 : i32, y3 : i32, x4 : i32, y4 : i32) -> Status);
    unsafe { GdipAddPathBezierI(path as _, x1, y1, x2, y2, x3, y3, x4, y4) }
}
#[inline]
pub unsafe fn GdipAddPathBeziers(path: *mut GpPath, points: *const PointF, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathBeziers(path : *mut GpPath, points : *const PointF, count : i32) -> Status);
    unsafe { GdipAddPathBeziers(path as _, points, count) }
}
#[inline]
pub unsafe fn GdipAddPathBeziersI(path: *mut GpPath, points: *const Point, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathBeziersI(path : *mut GpPath, points : *const Point, count : i32) -> Status);
    unsafe { GdipAddPathBeziersI(path as _, points, count) }
}
#[inline]
pub unsafe fn GdipAddPathClosedCurve(path: *mut GpPath, points: *const PointF, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathClosedCurve(path : *mut GpPath, points : *const PointF, count : i32) -> Status);
    unsafe { GdipAddPathClosedCurve(path as _, points, count) }
}
#[inline]
pub unsafe fn GdipAddPathClosedCurve2(path: *mut GpPath, points: *const PointF, count: i32, tension: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathClosedCurve2(path : *mut GpPath, points : *const PointF, count : i32, tension : f32) -> Status);
    unsafe { GdipAddPathClosedCurve2(path as _, points, count, tension) }
}
#[inline]
pub unsafe fn GdipAddPathClosedCurve2I(path: *mut GpPath, points: *const Point, count: i32, tension: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathClosedCurve2I(path : *mut GpPath, points : *const Point, count : i32, tension : f32) -> Status);
    unsafe { GdipAddPathClosedCurve2I(path as _, points, count, tension) }
}
#[inline]
pub unsafe fn GdipAddPathClosedCurveI(path: *mut GpPath, points: *const Point, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathClosedCurveI(path : *mut GpPath, points : *const Point, count : i32) -> Status);
    unsafe { GdipAddPathClosedCurveI(path as _, points, count) }
}
#[inline]
pub unsafe fn GdipAddPathCurve(path: *mut GpPath, points: *const PointF, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathCurve(path : *mut GpPath, points : *const PointF, count : i32) -> Status);
    unsafe { GdipAddPathCurve(path as _, points, count) }
}
#[inline]
pub unsafe fn GdipAddPathCurve2(path: *mut GpPath, points: *const PointF, count: i32, tension: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathCurve2(path : *mut GpPath, points : *const PointF, count : i32, tension : f32) -> Status);
    unsafe { GdipAddPathCurve2(path as _, points, count, tension) }
}
#[inline]
pub unsafe fn GdipAddPathCurve2I(path: *mut GpPath, points: *const Point, count: i32, tension: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathCurve2I(path : *mut GpPath, points : *const Point, count : i32, tension : f32) -> Status);
    unsafe { GdipAddPathCurve2I(path as _, points, count, tension) }
}
#[inline]
pub unsafe fn GdipAddPathCurve3(path: *mut GpPath, points: *const PointF, count: i32, offset: i32, numberofsegments: i32, tension: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathCurve3(path : *mut GpPath, points : *const PointF, count : i32, offset : i32, numberofsegments : i32, tension : f32) -> Status);
    unsafe { GdipAddPathCurve3(path as _, points, count, offset, numberofsegments, tension) }
}
#[inline]
pub unsafe fn GdipAddPathCurve3I(path: *mut GpPath, points: *const Point, count: i32, offset: i32, numberofsegments: i32, tension: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathCurve3I(path : *mut GpPath, points : *const Point, count : i32, offset : i32, numberofsegments : i32, tension : f32) -> Status);
    unsafe { GdipAddPathCurve3I(path as _, points, count, offset, numberofsegments, tension) }
}
#[inline]
pub unsafe fn GdipAddPathCurveI(path: *mut GpPath, points: *const Point, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathCurveI(path : *mut GpPath, points : *const Point, count : i32) -> Status);
    unsafe { GdipAddPathCurveI(path as _, points, count) }
}
#[inline]
pub unsafe fn GdipAddPathEllipse(path: *mut GpPath, x: f32, y: f32, width: f32, height: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathEllipse(path : *mut GpPath, x : f32, y : f32, width : f32, height : f32) -> Status);
    unsafe { GdipAddPathEllipse(path as _, x, y, width, height) }
}
#[inline]
pub unsafe fn GdipAddPathEllipseI(path: *mut GpPath, x: i32, y: i32, width: i32, height: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathEllipseI(path : *mut GpPath, x : i32, y : i32, width : i32, height : i32) -> Status);
    unsafe { GdipAddPathEllipseI(path as _, x, y, width, height) }
}
#[inline]
pub unsafe fn GdipAddPathLine(path: *mut GpPath, x1: f32, y1: f32, x2: f32, y2: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathLine(path : *mut GpPath, x1 : f32, y1 : f32, x2 : f32, y2 : f32) -> Status);
    unsafe { GdipAddPathLine(path as _, x1, y1, x2, y2) }
}
#[inline]
pub unsafe fn GdipAddPathLine2(path: *mut GpPath, points: *const PointF, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathLine2(path : *mut GpPath, points : *const PointF, count : i32) -> Status);
    unsafe { GdipAddPathLine2(path as _, points, count) }
}
#[inline]
pub unsafe fn GdipAddPathLine2I(path: *mut GpPath, points: *const Point, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathLine2I(path : *mut GpPath, points : *const Point, count : i32) -> Status);
    unsafe { GdipAddPathLine2I(path as _, points, count) }
}
#[inline]
pub unsafe fn GdipAddPathLineI(path: *mut GpPath, x1: i32, y1: i32, x2: i32, y2: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathLineI(path : *mut GpPath, x1 : i32, y1 : i32, x2 : i32, y2 : i32) -> Status);
    unsafe { GdipAddPathLineI(path as _, x1, y1, x2, y2) }
}
#[inline]
pub unsafe fn GdipAddPathPath(path: *mut GpPath, addingpath: *const GpPath, connect: bool) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathPath(path : *mut GpPath, addingpath : *const GpPath, connect : windows_core::BOOL) -> Status);
    unsafe { GdipAddPathPath(path as _, addingpath, connect.into()) }
}
#[inline]
pub unsafe fn GdipAddPathPie(path: *mut GpPath, x: f32, y: f32, width: f32, height: f32, startangle: f32, sweepangle: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathPie(path : *mut GpPath, x : f32, y : f32, width : f32, height : f32, startangle : f32, sweepangle : f32) -> Status);
    unsafe { GdipAddPathPie(path as _, x, y, width, height, startangle, sweepangle) }
}
#[inline]
pub unsafe fn GdipAddPathPieI(path: *mut GpPath, x: i32, y: i32, width: i32, height: i32, startangle: f32, sweepangle: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathPieI(path : *mut GpPath, x : i32, y : i32, width : i32, height : i32, startangle : f32, sweepangle : f32) -> Status);
    unsafe { GdipAddPathPieI(path as _, x, y, width, height, startangle, sweepangle) }
}
#[inline]
pub unsafe fn GdipAddPathPolygon(path: *mut GpPath, points: *const PointF, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathPolygon(path : *mut GpPath, points : *const PointF, count : i32) -> Status);
    unsafe { GdipAddPathPolygon(path as _, points, count) }
}
#[inline]
pub unsafe fn GdipAddPathPolygonI(path: *mut GpPath, points: *const Point, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathPolygonI(path : *mut GpPath, points : *const Point, count : i32) -> Status);
    unsafe { GdipAddPathPolygonI(path as _, points, count) }
}
#[inline]
pub unsafe fn GdipAddPathRectangle(path: *mut GpPath, x: f32, y: f32, width: f32, height: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathRectangle(path : *mut GpPath, x : f32, y : f32, width : f32, height : f32) -> Status);
    unsafe { GdipAddPathRectangle(path as _, x, y, width, height) }
}
#[inline]
pub unsafe fn GdipAddPathRectangleI(path: *mut GpPath, x: i32, y: i32, width: i32, height: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathRectangleI(path : *mut GpPath, x : i32, y : i32, width : i32, height : i32) -> Status);
    unsafe { GdipAddPathRectangleI(path as _, x, y, width, height) }
}
#[inline]
pub unsafe fn GdipAddPathRectangles(path: *mut GpPath, rects: *const RectF, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathRectangles(path : *mut GpPath, rects : *const RectF, count : i32) -> Status);
    unsafe { GdipAddPathRectangles(path as _, rects, count) }
}
#[inline]
pub unsafe fn GdipAddPathRectanglesI(path: *mut GpPath, rects: *const Rect, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathRectanglesI(path : *mut GpPath, rects : *const Rect, count : i32) -> Status);
    unsafe { GdipAddPathRectanglesI(path as _, rects, count) }
}
#[inline]
pub unsafe fn GdipAddPathString<P1>(path: *mut GpPath, string: P1, length: i32, family: *const GpFontFamily, style: i32, emsize: f32, layoutrect: *const RectF, format: *const GpStringFormat) -> Status
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathString(path : *mut GpPath, string : windows_core::PCWSTR, length : i32, family : *const GpFontFamily, style : i32, emsize : f32, layoutrect : *const RectF, format : *const GpStringFormat) -> Status);
    unsafe { GdipAddPathString(path as _, string.param().abi(), length, family, style, emsize, layoutrect, format) }
}
#[inline]
pub unsafe fn GdipAddPathStringI<P1>(path: *mut GpPath, string: P1, length: i32, family: *const GpFontFamily, style: i32, emsize: f32, layoutrect: *const Rect, format: *const GpStringFormat) -> Status
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("gdiplus.dll" "system" fn GdipAddPathStringI(path : *mut GpPath, string : windows_core::PCWSTR, length : i32, family : *const GpFontFamily, style : i32, emsize : f32, layoutrect : *const Rect, format : *const GpStringFormat) -> Status);
    unsafe { GdipAddPathStringI(path as _, string.param().abi(), length, family, style, emsize, layoutrect, format) }
}
#[inline]
pub unsafe fn GdipAlloc(size: usize) -> *mut core::ffi::c_void {
    windows_core::link!("gdiplus.dll" "system" fn GdipAlloc(size : usize) -> *mut core::ffi::c_void);
    unsafe { GdipAlloc(size) }
}
#[inline]
pub unsafe fn GdipBeginContainer(graphics: *mut GpGraphics, dstrect: *const RectF, srcrect: *const RectF, unit: Unit, state: *mut u32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipBeginContainer(graphics : *mut GpGraphics, dstrect : *const RectF, srcrect : *const RectF, unit : Unit, state : *mut u32) -> Status);
    unsafe { GdipBeginContainer(graphics as _, dstrect, srcrect, unit, state as _) }
}
#[inline]
pub unsafe fn GdipBeginContainer2(graphics: *mut GpGraphics, state: *mut u32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipBeginContainer2(graphics : *mut GpGraphics, state : *mut u32) -> Status);
    unsafe { GdipBeginContainer2(graphics as _, state as _) }
}
#[inline]
pub unsafe fn GdipBeginContainerI(graphics: *mut GpGraphics, dstrect: *const Rect, srcrect: *const Rect, unit: Unit, state: *mut u32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipBeginContainerI(graphics : *mut GpGraphics, dstrect : *const Rect, srcrect : *const Rect, unit : Unit, state : *mut u32) -> Status);
    unsafe { GdipBeginContainerI(graphics as _, dstrect, srcrect, unit, state as _) }
}
#[inline]
pub unsafe fn GdipBitmapApplyEffect(bitmap: *mut GpBitmap, effect: *mut CGpEffect, roi: *mut super::super::Foundation::RECT, useauxdata: bool, auxdata: *mut *mut core::ffi::c_void, auxdatasize: *mut i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipBitmapApplyEffect(bitmap : *mut GpBitmap, effect : *mut CGpEffect, roi : *mut super::super::Foundation:: RECT, useauxdata : windows_core::BOOL, auxdata : *mut *mut core::ffi::c_void, auxdatasize : *mut i32) -> Status);
    unsafe { GdipBitmapApplyEffect(bitmap as _, effect as _, roi as _, useauxdata.into(), auxdata as _, auxdatasize as _) }
}
#[inline]
pub unsafe fn GdipBitmapConvertFormat(pinputbitmap: *mut GpBitmap, format: i32, dithertype: DitherType, palettetype: PaletteType, palette: *mut ColorPalette, alphathresholdpercent: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipBitmapConvertFormat(pinputbitmap : *mut GpBitmap, format : i32, dithertype : DitherType, palettetype : PaletteType, palette : *mut ColorPalette, alphathresholdpercent : f32) -> Status);
    unsafe { GdipBitmapConvertFormat(pinputbitmap as _, format, dithertype, palettetype, palette as _, alphathresholdpercent) }
}
#[inline]
pub unsafe fn GdipBitmapCreateApplyEffect(inputbitmaps: *mut *mut GpBitmap, numinputs: i32, effect: *mut CGpEffect, roi: *mut super::super::Foundation::RECT, outputrect: *mut super::super::Foundation::RECT, outputbitmap: *mut *mut GpBitmap, useauxdata: bool, auxdata: *mut *mut core::ffi::c_void, auxdatasize: *mut i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipBitmapCreateApplyEffect(inputbitmaps : *mut *mut GpBitmap, numinputs : i32, effect : *mut CGpEffect, roi : *mut super::super::Foundation:: RECT, outputrect : *mut super::super::Foundation:: RECT, outputbitmap : *mut *mut GpBitmap, useauxdata : windows_core::BOOL, auxdata : *mut *mut core::ffi::c_void, auxdatasize : *mut i32) -> Status);
    unsafe { GdipBitmapCreateApplyEffect(inputbitmaps as _, numinputs, effect as _, roi as _, outputrect as _, outputbitmap as _, useauxdata.into(), auxdata as _, auxdatasize as _) }
}
#[inline]
pub unsafe fn GdipBitmapGetHistogram(bitmap: *mut GpBitmap, format: HistogramFormat, numberofentries: u32, channel0: *mut u32, channel1: *mut u32, channel2: *mut u32, channel3: *mut u32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipBitmapGetHistogram(bitmap : *mut GpBitmap, format : HistogramFormat, numberofentries : u32, channel0 : *mut u32, channel1 : *mut u32, channel2 : *mut u32, channel3 : *mut u32) -> Status);
    unsafe { GdipBitmapGetHistogram(bitmap as _, format, numberofentries, channel0 as _, channel1 as _, channel2 as _, channel3 as _) }
}
#[inline]
pub unsafe fn GdipBitmapGetHistogramSize(format: HistogramFormat, numberofentries: *mut u32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipBitmapGetHistogramSize(format : HistogramFormat, numberofentries : *mut u32) -> Status);
    unsafe { GdipBitmapGetHistogramSize(format, numberofentries as _) }
}
#[inline]
pub unsafe fn GdipBitmapGetPixel(bitmap: *mut GpBitmap, x: i32, y: i32, color: *mut u32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipBitmapGetPixel(bitmap : *mut GpBitmap, x : i32, y : i32, color : *mut u32) -> Status);
    unsafe { GdipBitmapGetPixel(bitmap as _, x, y, color as _) }
}
#[inline]
pub unsafe fn GdipBitmapLockBits(bitmap: *mut GpBitmap, rect: *const Rect, flags: u32, format: i32, lockedbitmapdata: *mut BitmapData) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipBitmapLockBits(bitmap : *mut GpBitmap, rect : *const Rect, flags : u32, format : i32, lockedbitmapdata : *mut BitmapData) -> Status);
    unsafe { GdipBitmapLockBits(bitmap as _, rect, flags, format, lockedbitmapdata as _) }
}
#[inline]
pub unsafe fn GdipBitmapSetPixel(bitmap: *mut GpBitmap, x: i32, y: i32, color: u32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipBitmapSetPixel(bitmap : *mut GpBitmap, x : i32, y : i32, color : u32) -> Status);
    unsafe { GdipBitmapSetPixel(bitmap as _, x, y, color) }
}
#[inline]
pub unsafe fn GdipBitmapSetResolution(bitmap: *mut GpBitmap, xdpi: f32, ydpi: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipBitmapSetResolution(bitmap : *mut GpBitmap, xdpi : f32, ydpi : f32) -> Status);
    unsafe { GdipBitmapSetResolution(bitmap as _, xdpi, ydpi) }
}
#[inline]
pub unsafe fn GdipBitmapUnlockBits(bitmap: *mut GpBitmap, lockedbitmapdata: *mut BitmapData) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipBitmapUnlockBits(bitmap : *mut GpBitmap, lockedbitmapdata : *mut BitmapData) -> Status);
    unsafe { GdipBitmapUnlockBits(bitmap as _, lockedbitmapdata as _) }
}
#[inline]
pub unsafe fn GdipClearPathMarkers(path: *mut GpPath) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipClearPathMarkers(path : *mut GpPath) -> Status);
    unsafe { GdipClearPathMarkers(path as _) }
}
#[inline]
pub unsafe fn GdipCloneBitmapArea(x: f32, y: f32, width: f32, height: f32, format: i32, srcbitmap: *mut GpBitmap, dstbitmap: *mut *mut GpBitmap) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCloneBitmapArea(x : f32, y : f32, width : f32, height : f32, format : i32, srcbitmap : *mut GpBitmap, dstbitmap : *mut *mut GpBitmap) -> Status);
    unsafe { GdipCloneBitmapArea(x, y, width, height, format, srcbitmap as _, dstbitmap as _) }
}
#[inline]
pub unsafe fn GdipCloneBitmapAreaI(x: i32, y: i32, width: i32, height: i32, format: i32, srcbitmap: *mut GpBitmap, dstbitmap: *mut *mut GpBitmap) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCloneBitmapAreaI(x : i32, y : i32, width : i32, height : i32, format : i32, srcbitmap : *mut GpBitmap, dstbitmap : *mut *mut GpBitmap) -> Status);
    unsafe { GdipCloneBitmapAreaI(x, y, width, height, format, srcbitmap as _, dstbitmap as _) }
}
#[inline]
pub unsafe fn GdipCloneBrush(brush: *mut GpBrush, clonebrush: *mut *mut GpBrush) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCloneBrush(brush : *mut GpBrush, clonebrush : *mut *mut GpBrush) -> Status);
    unsafe { GdipCloneBrush(brush as _, clonebrush as _) }
}
#[inline]
pub unsafe fn GdipCloneCustomLineCap(customcap: *mut GpCustomLineCap, clonedcap: *mut *mut GpCustomLineCap) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCloneCustomLineCap(customcap : *mut GpCustomLineCap, clonedcap : *mut *mut GpCustomLineCap) -> Status);
    unsafe { GdipCloneCustomLineCap(customcap as _, clonedcap as _) }
}
#[inline]
pub unsafe fn GdipCloneFont(font: *mut GpFont, clonefont: *mut *mut GpFont) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCloneFont(font : *mut GpFont, clonefont : *mut *mut GpFont) -> Status);
    unsafe { GdipCloneFont(font as _, clonefont as _) }
}
#[inline]
pub unsafe fn GdipCloneFontFamily(fontfamily: *mut GpFontFamily, clonedfontfamily: *mut *mut GpFontFamily) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCloneFontFamily(fontfamily : *mut GpFontFamily, clonedfontfamily : *mut *mut GpFontFamily) -> Status);
    unsafe { GdipCloneFontFamily(fontfamily as _, clonedfontfamily as _) }
}
#[inline]
pub unsafe fn GdipCloneImage(image: *mut GpImage, cloneimage: *mut *mut GpImage) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCloneImage(image : *mut GpImage, cloneimage : *mut *mut GpImage) -> Status);
    unsafe { GdipCloneImage(image as _, cloneimage as _) }
}
#[inline]
pub unsafe fn GdipCloneImageAttributes(imageattr: *const GpImageAttributes, cloneimageattr: *mut *mut GpImageAttributes) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCloneImageAttributes(imageattr : *const GpImageAttributes, cloneimageattr : *mut *mut GpImageAttributes) -> Status);
    unsafe { GdipCloneImageAttributes(imageattr, cloneimageattr as _) }
}
#[inline]
pub unsafe fn GdipCloneMatrix(matrix: *mut Matrix, clonematrix: *mut *mut Matrix) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCloneMatrix(matrix : *mut Matrix, clonematrix : *mut *mut Matrix) -> Status);
    unsafe { GdipCloneMatrix(matrix as _, clonematrix as _) }
}
#[inline]
pub unsafe fn GdipClonePath(path: *mut GpPath, clonepath: *mut *mut GpPath) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipClonePath(path : *mut GpPath, clonepath : *mut *mut GpPath) -> Status);
    unsafe { GdipClonePath(path as _, clonepath as _) }
}
#[inline]
pub unsafe fn GdipClonePen(pen: *mut GpPen, clonepen: *mut *mut GpPen) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipClonePen(pen : *mut GpPen, clonepen : *mut *mut GpPen) -> Status);
    unsafe { GdipClonePen(pen as _, clonepen as _) }
}
#[inline]
pub unsafe fn GdipCloneRegion(region: *mut GpRegion, cloneregion: *mut *mut GpRegion) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCloneRegion(region : *mut GpRegion, cloneregion : *mut *mut GpRegion) -> Status);
    unsafe { GdipCloneRegion(region as _, cloneregion as _) }
}
#[inline]
pub unsafe fn GdipCloneStringFormat(format: *const GpStringFormat, newformat: *mut *mut GpStringFormat) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCloneStringFormat(format : *const GpStringFormat, newformat : *mut *mut GpStringFormat) -> Status);
    unsafe { GdipCloneStringFormat(format, newformat as _) }
}
#[inline]
pub unsafe fn GdipClosePathFigure(path: *mut GpPath) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipClosePathFigure(path : *mut GpPath) -> Status);
    unsafe { GdipClosePathFigure(path as _) }
}
#[inline]
pub unsafe fn GdipClosePathFigures(path: *mut GpPath) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipClosePathFigures(path : *mut GpPath) -> Status);
    unsafe { GdipClosePathFigures(path as _) }
}
#[inline]
pub unsafe fn GdipCombineRegionPath(region: *mut GpRegion, path: *mut GpPath, combinemode: CombineMode) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCombineRegionPath(region : *mut GpRegion, path : *mut GpPath, combinemode : CombineMode) -> Status);
    unsafe { GdipCombineRegionPath(region as _, path as _, combinemode) }
}
#[inline]
pub unsafe fn GdipCombineRegionRect(region: *mut GpRegion, rect: *const RectF, combinemode: CombineMode) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCombineRegionRect(region : *mut GpRegion, rect : *const RectF, combinemode : CombineMode) -> Status);
    unsafe { GdipCombineRegionRect(region as _, rect, combinemode) }
}
#[inline]
pub unsafe fn GdipCombineRegionRectI(region: *mut GpRegion, rect: *const Rect, combinemode: CombineMode) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCombineRegionRectI(region : *mut GpRegion, rect : *const Rect, combinemode : CombineMode) -> Status);
    unsafe { GdipCombineRegionRectI(region as _, rect, combinemode) }
}
#[inline]
pub unsafe fn GdipCombineRegionRegion(region: *mut GpRegion, region2: *mut GpRegion, combinemode: CombineMode) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCombineRegionRegion(region : *mut GpRegion, region2 : *mut GpRegion, combinemode : CombineMode) -> Status);
    unsafe { GdipCombineRegionRegion(region as _, region2 as _, combinemode) }
}
#[inline]
pub unsafe fn GdipComment(graphics: *mut GpGraphics, sizedata: u32, data: *const u8) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipComment(graphics : *mut GpGraphics, sizedata : u32, data : *const u8) -> Status);
    unsafe { GdipComment(graphics as _, sizedata, data) }
}
#[inline]
pub unsafe fn GdipConvertToEmfPlus<P4>(refgraphics: *const GpGraphics, metafile: *mut GpMetafile, conversionfailureflag: *mut i32, emftype: EmfType, description: P4, out_metafile: *mut *mut GpMetafile) -> Status
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("gdiplus.dll" "system" fn GdipConvertToEmfPlus(refgraphics : *const GpGraphics, metafile : *mut GpMetafile, conversionfailureflag : *mut i32, emftype : EmfType, description : windows_core::PCWSTR, out_metafile : *mut *mut GpMetafile) -> Status);
    unsafe { GdipConvertToEmfPlus(refgraphics, metafile as _, conversionfailureflag as _, emftype, description.param().abi(), out_metafile as _) }
}
#[inline]
pub unsafe fn GdipConvertToEmfPlusToFile<P3, P5>(refgraphics: *const GpGraphics, metafile: *mut GpMetafile, conversionfailureflag: *mut i32, filename: P3, emftype: EmfType, description: P5, out_metafile: *mut *mut GpMetafile) -> Status
where
    P3: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("gdiplus.dll" "system" fn GdipConvertToEmfPlusToFile(refgraphics : *const GpGraphics, metafile : *mut GpMetafile, conversionfailureflag : *mut i32, filename : windows_core::PCWSTR, emftype : EmfType, description : windows_core::PCWSTR, out_metafile : *mut *mut GpMetafile) -> Status);
    unsafe { GdipConvertToEmfPlusToFile(refgraphics, metafile as _, conversionfailureflag as _, filename.param().abi(), emftype, description.param().abi(), out_metafile as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn GdipConvertToEmfPlusToStream<P3, P5>(refgraphics: *const GpGraphics, metafile: *mut GpMetafile, conversionfailureflag: *mut i32, stream: P3, emftype: EmfType, description: P5, out_metafile: *mut *mut GpMetafile) -> Status
where
    P3: windows_core::Param<super::super::System::Com::IStream>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("gdiplus.dll" "system" fn GdipConvertToEmfPlusToStream(refgraphics : *const GpGraphics, metafile : *mut GpMetafile, conversionfailureflag : *mut i32, stream : * mut core::ffi::c_void, emftype : EmfType, description : windows_core::PCWSTR, out_metafile : *mut *mut GpMetafile) -> Status);
    unsafe { GdipConvertToEmfPlusToStream(refgraphics, metafile as _, conversionfailureflag as _, stream.param().abi(), emftype, description.param().abi(), out_metafile as _) }
}
#[inline]
pub unsafe fn GdipCreateAdjustableArrowCap(height: f32, width: f32, isfilled: bool, cap: *mut *mut GpAdjustableArrowCap) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateAdjustableArrowCap(height : f32, width : f32, isfilled : windows_core::BOOL, cap : *mut *mut GpAdjustableArrowCap) -> Status);
    unsafe { GdipCreateAdjustableArrowCap(height, width, isfilled.into(), cap as _) }
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
#[inline]
pub unsafe fn GdipCreateBitmapFromDirectDrawSurface<P0>(surface: P0, bitmap: *mut *mut GpBitmap) -> Status
where
    P0: windows_core::Param<super::DirectDraw::IDirectDrawSurface7>,
{
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateBitmapFromDirectDrawSurface(surface : * mut core::ffi::c_void, bitmap : *mut *mut GpBitmap) -> Status);
    unsafe { GdipCreateBitmapFromDirectDrawSurface(surface.param().abi(), bitmap as _) }
}
#[inline]
pub unsafe fn GdipCreateBitmapFromFile<P0>(filename: P0, bitmap: *mut *mut GpBitmap) -> Status
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateBitmapFromFile(filename : windows_core::PCWSTR, bitmap : *mut *mut GpBitmap) -> Status);
    unsafe { GdipCreateBitmapFromFile(filename.param().abi(), bitmap as _) }
}
#[inline]
pub unsafe fn GdipCreateBitmapFromFileICM<P0>(filename: P0, bitmap: *mut *mut GpBitmap) -> Status
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateBitmapFromFileICM(filename : windows_core::PCWSTR, bitmap : *mut *mut GpBitmap) -> Status);
    unsafe { GdipCreateBitmapFromFileICM(filename.param().abi(), bitmap as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipCreateBitmapFromGdiDib(gdibitmapinfo: *const super::Gdi::BITMAPINFO, gdibitmapdata: *mut core::ffi::c_void, bitmap: *mut *mut GpBitmap) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateBitmapFromGdiDib(gdibitmapinfo : *const super::Gdi:: BITMAPINFO, gdibitmapdata : *mut core::ffi::c_void, bitmap : *mut *mut GpBitmap) -> Status);
    unsafe { GdipCreateBitmapFromGdiDib(gdibitmapinfo, gdibitmapdata as _, bitmap as _) }
}
#[inline]
pub unsafe fn GdipCreateBitmapFromGraphics(width: i32, height: i32, target: *mut GpGraphics, bitmap: *mut *mut GpBitmap) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateBitmapFromGraphics(width : i32, height : i32, target : *mut GpGraphics, bitmap : *mut *mut GpBitmap) -> Status);
    unsafe { GdipCreateBitmapFromGraphics(width, height, target as _, bitmap as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipCreateBitmapFromHBITMAP(hbm: super::Gdi::HBITMAP, hpal: super::Gdi::HPALETTE, bitmap: *mut *mut GpBitmap) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateBitmapFromHBITMAP(hbm : super::Gdi:: HBITMAP, hpal : super::Gdi:: HPALETTE, bitmap : *mut *mut GpBitmap) -> Status);
    unsafe { GdipCreateBitmapFromHBITMAP(hbm, hpal, bitmap as _) }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn GdipCreateBitmapFromHICON(hicon: super::super::UI::WindowsAndMessaging::HICON, bitmap: *mut *mut GpBitmap) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateBitmapFromHICON(hicon : super::super::UI::WindowsAndMessaging:: HICON, bitmap : *mut *mut GpBitmap) -> Status);
    unsafe { GdipCreateBitmapFromHICON(hicon, bitmap as _) }
}
#[inline]
pub unsafe fn GdipCreateBitmapFromResource<P1>(hinstance: super::super::Foundation::HINSTANCE, lpbitmapname: P1, bitmap: *mut *mut GpBitmap) -> Status
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateBitmapFromResource(hinstance : super::super::Foundation:: HINSTANCE, lpbitmapname : windows_core::PCWSTR, bitmap : *mut *mut GpBitmap) -> Status);
    unsafe { GdipCreateBitmapFromResource(hinstance, lpbitmapname.param().abi(), bitmap as _) }
}
#[inline]
pub unsafe fn GdipCreateBitmapFromScan0(width: i32, height: i32, stride: i32, format: i32, scan0: Option<*const u8>, bitmap: *mut *mut GpBitmap) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateBitmapFromScan0(width : i32, height : i32, stride : i32, format : i32, scan0 : *const u8, bitmap : *mut *mut GpBitmap) -> Status);
    unsafe { GdipCreateBitmapFromScan0(width, height, stride, format, scan0.unwrap_or(core::mem::zeroed()) as _, bitmap as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn GdipCreateBitmapFromStream<P0>(stream: P0, bitmap: *mut *mut GpBitmap) -> Status
where
    P0: windows_core::Param<super::super::System::Com::IStream>,
{
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateBitmapFromStream(stream : * mut core::ffi::c_void, bitmap : *mut *mut GpBitmap) -> Status);
    unsafe { GdipCreateBitmapFromStream(stream.param().abi(), bitmap as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn GdipCreateBitmapFromStreamICM<P0>(stream: P0, bitmap: *mut *mut GpBitmap) -> Status
where
    P0: windows_core::Param<super::super::System::Com::IStream>,
{
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateBitmapFromStreamICM(stream : * mut core::ffi::c_void, bitmap : *mut *mut GpBitmap) -> Status);
    unsafe { GdipCreateBitmapFromStreamICM(stream.param().abi(), bitmap as _) }
}
#[inline]
pub unsafe fn GdipCreateCachedBitmap(bitmap: *mut GpBitmap, graphics: *mut GpGraphics, cachedbitmap: *mut *mut GpCachedBitmap) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateCachedBitmap(bitmap : *mut GpBitmap, graphics : *mut GpGraphics, cachedbitmap : *mut *mut GpCachedBitmap) -> Status);
    unsafe { GdipCreateCachedBitmap(bitmap as _, graphics as _, cachedbitmap as _) }
}
#[inline]
pub unsafe fn GdipCreateCustomLineCap(fillpath: *mut GpPath, strokepath: *mut GpPath, basecap: LineCap, baseinset: f32, customcap: *mut *mut GpCustomLineCap) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateCustomLineCap(fillpath : *mut GpPath, strokepath : *mut GpPath, basecap : LineCap, baseinset : f32, customcap : *mut *mut GpCustomLineCap) -> Status);
    unsafe { GdipCreateCustomLineCap(fillpath as _, strokepath as _, basecap, baseinset, customcap as _) }
}
#[inline]
pub unsafe fn GdipCreateEffect(guid: windows_core::GUID, effect: *mut *mut CGpEffect) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateEffect(guid : windows_core::GUID, effect : *mut *mut CGpEffect) -> Status);
    unsafe { GdipCreateEffect(core::mem::transmute(guid), effect as _) }
}
#[inline]
pub unsafe fn GdipCreateFont(fontfamily: *const GpFontFamily, emsize: f32, style: i32, unit: Unit, font: *mut *mut GpFont) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateFont(fontfamily : *const GpFontFamily, emsize : f32, style : i32, unit : Unit, font : *mut *mut GpFont) -> Status);
    unsafe { GdipCreateFont(fontfamily, emsize, style, unit, font as _) }
}
#[inline]
pub unsafe fn GdipCreateFontFamilyFromName<P0>(name: P0, fontcollection: *mut GpFontCollection, fontfamily: *mut *mut GpFontFamily) -> Status
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateFontFamilyFromName(name : windows_core::PCWSTR, fontcollection : *mut GpFontCollection, fontfamily : *mut *mut GpFontFamily) -> Status);
    unsafe { GdipCreateFontFamilyFromName(name.param().abi(), fontcollection as _, fontfamily as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipCreateFontFromDC(hdc: super::Gdi::HDC, font: *mut *mut GpFont) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateFontFromDC(hdc : super::Gdi:: HDC, font : *mut *mut GpFont) -> Status);
    unsafe { GdipCreateFontFromDC(hdc, font as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipCreateFontFromLogfontA(hdc: super::Gdi::HDC, logfont: *const super::Gdi::LOGFONTA, font: *mut *mut GpFont) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateFontFromLogfontA(hdc : super::Gdi:: HDC, logfont : *const super::Gdi:: LOGFONTA, font : *mut *mut GpFont) -> Status);
    unsafe { GdipCreateFontFromLogfontA(hdc, logfont, font as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipCreateFontFromLogfontW(hdc: super::Gdi::HDC, logfont: *const super::Gdi::LOGFONTW, font: *mut *mut GpFont) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateFontFromLogfontW(hdc : super::Gdi:: HDC, logfont : *const super::Gdi:: LOGFONTW, font : *mut *mut GpFont) -> Status);
    unsafe { GdipCreateFontFromLogfontW(hdc, logfont, font as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipCreateFromHDC(hdc: super::Gdi::HDC, graphics: *mut *mut GpGraphics) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateFromHDC(hdc : super::Gdi:: HDC, graphics : *mut *mut GpGraphics) -> Status);
    unsafe { GdipCreateFromHDC(hdc, graphics as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipCreateFromHDC2(hdc: super::Gdi::HDC, hdevice: super::super::Foundation::HANDLE, graphics: *mut *mut GpGraphics) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateFromHDC2(hdc : super::Gdi:: HDC, hdevice : super::super::Foundation:: HANDLE, graphics : *mut *mut GpGraphics) -> Status);
    unsafe { GdipCreateFromHDC2(hdc, hdevice, graphics as _) }
}
#[inline]
pub unsafe fn GdipCreateFromHWND(hwnd: super::super::Foundation::HWND, graphics: *mut *mut GpGraphics) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateFromHWND(hwnd : super::super::Foundation:: HWND, graphics : *mut *mut GpGraphics) -> Status);
    unsafe { GdipCreateFromHWND(hwnd, graphics as _) }
}
#[inline]
pub unsafe fn GdipCreateFromHWNDICM(hwnd: super::super::Foundation::HWND, graphics: *mut *mut GpGraphics) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateFromHWNDICM(hwnd : super::super::Foundation:: HWND, graphics : *mut *mut GpGraphics) -> Status);
    unsafe { GdipCreateFromHWNDICM(hwnd, graphics as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipCreateHBITMAPFromBitmap(bitmap: *mut GpBitmap, hbmreturn: *mut super::Gdi::HBITMAP, background: u32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateHBITMAPFromBitmap(bitmap : *mut GpBitmap, hbmreturn : *mut super::Gdi:: HBITMAP, background : u32) -> Status);
    unsafe { GdipCreateHBITMAPFromBitmap(bitmap as _, hbmreturn as _, background) }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn GdipCreateHICONFromBitmap(bitmap: *mut GpBitmap, hbmreturn: *mut super::super::UI::WindowsAndMessaging::HICON) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateHICONFromBitmap(bitmap : *mut GpBitmap, hbmreturn : *mut super::super::UI::WindowsAndMessaging:: HICON) -> Status);
    unsafe { GdipCreateHICONFromBitmap(bitmap as _, hbmreturn as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipCreateHalftonePalette() -> super::Gdi::HPALETTE {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateHalftonePalette() -> super::Gdi:: HPALETTE);
    unsafe { GdipCreateHalftonePalette() }
}
#[inline]
pub unsafe fn GdipCreateHatchBrush(hatchstyle: HatchStyle, forecol: u32, backcol: u32, brush: *mut *mut GpHatch) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateHatchBrush(hatchstyle : HatchStyle, forecol : u32, backcol : u32, brush : *mut *mut GpHatch) -> Status);
    unsafe { GdipCreateHatchBrush(hatchstyle, forecol, backcol, brush as _) }
}
#[inline]
pub unsafe fn GdipCreateImageAttributes(imageattr: *mut *mut GpImageAttributes) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateImageAttributes(imageattr : *mut *mut GpImageAttributes) -> Status);
    unsafe { GdipCreateImageAttributes(imageattr as _) }
}
#[inline]
pub unsafe fn GdipCreateLineBrush(point1: *const PointF, point2: *const PointF, color1: u32, color2: u32, wrapmode: WrapMode, linegradient: *mut *mut GpLineGradient) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateLineBrush(point1 : *const PointF, point2 : *const PointF, color1 : u32, color2 : u32, wrapmode : WrapMode, linegradient : *mut *mut GpLineGradient) -> Status);
    unsafe { GdipCreateLineBrush(point1, point2, color1, color2, wrapmode, linegradient as _) }
}
#[inline]
pub unsafe fn GdipCreateLineBrushFromRect(rect: *const RectF, color1: u32, color2: u32, mode: LinearGradientMode, wrapmode: WrapMode, linegradient: *mut *mut GpLineGradient) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateLineBrushFromRect(rect : *const RectF, color1 : u32, color2 : u32, mode : LinearGradientMode, wrapmode : WrapMode, linegradient : *mut *mut GpLineGradient) -> Status);
    unsafe { GdipCreateLineBrushFromRect(rect, color1, color2, mode, wrapmode, linegradient as _) }
}
#[inline]
pub unsafe fn GdipCreateLineBrushFromRectI(rect: *const Rect, color1: u32, color2: u32, mode: LinearGradientMode, wrapmode: WrapMode, linegradient: *mut *mut GpLineGradient) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateLineBrushFromRectI(rect : *const Rect, color1 : u32, color2 : u32, mode : LinearGradientMode, wrapmode : WrapMode, linegradient : *mut *mut GpLineGradient) -> Status);
    unsafe { GdipCreateLineBrushFromRectI(rect, color1, color2, mode, wrapmode, linegradient as _) }
}
#[inline]
pub unsafe fn GdipCreateLineBrushFromRectWithAngle(rect: *const RectF, color1: u32, color2: u32, angle: f32, isanglescalable: bool, wrapmode: WrapMode, linegradient: *mut *mut GpLineGradient) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateLineBrushFromRectWithAngle(rect : *const RectF, color1 : u32, color2 : u32, angle : f32, isanglescalable : windows_core::BOOL, wrapmode : WrapMode, linegradient : *mut *mut GpLineGradient) -> Status);
    unsafe { GdipCreateLineBrushFromRectWithAngle(rect, color1, color2, angle, isanglescalable.into(), wrapmode, linegradient as _) }
}
#[inline]
pub unsafe fn GdipCreateLineBrushFromRectWithAngleI(rect: *const Rect, color1: u32, color2: u32, angle: f32, isanglescalable: bool, wrapmode: WrapMode, linegradient: *mut *mut GpLineGradient) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateLineBrushFromRectWithAngleI(rect : *const Rect, color1 : u32, color2 : u32, angle : f32, isanglescalable : windows_core::BOOL, wrapmode : WrapMode, linegradient : *mut *mut GpLineGradient) -> Status);
    unsafe { GdipCreateLineBrushFromRectWithAngleI(rect, color1, color2, angle, isanglescalable.into(), wrapmode, linegradient as _) }
}
#[inline]
pub unsafe fn GdipCreateLineBrushI(point1: *const Point, point2: *const Point, color1: u32, color2: u32, wrapmode: WrapMode, linegradient: *mut *mut GpLineGradient) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateLineBrushI(point1 : *const Point, point2 : *const Point, color1 : u32, color2 : u32, wrapmode : WrapMode, linegradient : *mut *mut GpLineGradient) -> Status);
    unsafe { GdipCreateLineBrushI(point1, point2, color1, color2, wrapmode, linegradient as _) }
}
#[inline]
pub unsafe fn GdipCreateMatrix(matrix: *mut *mut Matrix) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateMatrix(matrix : *mut *mut Matrix) -> Status);
    unsafe { GdipCreateMatrix(matrix as _) }
}
#[inline]
pub unsafe fn GdipCreateMatrix2(m11: f32, m12: f32, m21: f32, m22: f32, dx: f32, dy: f32, matrix: *mut *mut Matrix) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateMatrix2(m11 : f32, m12 : f32, m21 : f32, m22 : f32, dx : f32, dy : f32, matrix : *mut *mut Matrix) -> Status);
    unsafe { GdipCreateMatrix2(m11, m12, m21, m22, dx, dy, matrix as _) }
}
#[inline]
pub unsafe fn GdipCreateMatrix3(rect: *const RectF, dstplg: *const PointF, matrix: *mut *mut Matrix) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateMatrix3(rect : *const RectF, dstplg : *const PointF, matrix : *mut *mut Matrix) -> Status);
    unsafe { GdipCreateMatrix3(rect, dstplg, matrix as _) }
}
#[inline]
pub unsafe fn GdipCreateMatrix3I(rect: *const Rect, dstplg: *const Point, matrix: *mut *mut Matrix) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateMatrix3I(rect : *const Rect, dstplg : *const Point, matrix : *mut *mut Matrix) -> Status);
    unsafe { GdipCreateMatrix3I(rect, dstplg, matrix as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipCreateMetafileFromEmf(hemf: super::Gdi::HENHMETAFILE, deleteemf: bool, metafile: *mut *mut GpMetafile) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateMetafileFromEmf(hemf : super::Gdi:: HENHMETAFILE, deleteemf : windows_core::BOOL, metafile : *mut *mut GpMetafile) -> Status);
    unsafe { GdipCreateMetafileFromEmf(hemf, deleteemf.into(), metafile as _) }
}
#[inline]
pub unsafe fn GdipCreateMetafileFromFile<P0>(file: P0, metafile: *mut *mut GpMetafile) -> Status
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateMetafileFromFile(file : windows_core::PCWSTR, metafile : *mut *mut GpMetafile) -> Status);
    unsafe { GdipCreateMetafileFromFile(file.param().abi(), metafile as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn GdipCreateMetafileFromStream<P0>(stream: P0, metafile: *mut *mut GpMetafile) -> Status
where
    P0: windows_core::Param<super::super::System::Com::IStream>,
{
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateMetafileFromStream(stream : * mut core::ffi::c_void, metafile : *mut *mut GpMetafile) -> Status);
    unsafe { GdipCreateMetafileFromStream(stream.param().abi(), metafile as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipCreateMetafileFromWmf(hwmf: super::Gdi::HMETAFILE, deletewmf: bool, wmfplaceablefileheader: *const WmfPlaceableFileHeader, metafile: *mut *mut GpMetafile) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateMetafileFromWmf(hwmf : super::Gdi:: HMETAFILE, deletewmf : windows_core::BOOL, wmfplaceablefileheader : *const WmfPlaceableFileHeader, metafile : *mut *mut GpMetafile) -> Status);
    unsafe { GdipCreateMetafileFromWmf(hwmf, deletewmf.into(), wmfplaceablefileheader, metafile as _) }
}
#[inline]
pub unsafe fn GdipCreateMetafileFromWmfFile<P0>(file: P0, wmfplaceablefileheader: *const WmfPlaceableFileHeader, metafile: *mut *mut GpMetafile) -> Status
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateMetafileFromWmfFile(file : windows_core::PCWSTR, wmfplaceablefileheader : *const WmfPlaceableFileHeader, metafile : *mut *mut GpMetafile) -> Status);
    unsafe { GdipCreateMetafileFromWmfFile(file.param().abi(), wmfplaceablefileheader, metafile as _) }
}
#[inline]
pub unsafe fn GdipCreatePath(brushmode: FillMode, path: *mut *mut GpPath) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreatePath(brushmode : FillMode, path : *mut *mut GpPath) -> Status);
    unsafe { GdipCreatePath(brushmode, path as _) }
}
#[inline]
pub unsafe fn GdipCreatePath2(param0: *const PointF, param1: *const u8, param2: i32, param3: FillMode, path: *mut *mut GpPath) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreatePath2(param0 : *const PointF, param1 : *const u8, param2 : i32, param3 : FillMode, path : *mut *mut GpPath) -> Status);
    unsafe { GdipCreatePath2(param0, param1, param2, param3, path as _) }
}
#[inline]
pub unsafe fn GdipCreatePath2I(param0: *const Point, param1: *const u8, param2: i32, param3: FillMode, path: *mut *mut GpPath) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreatePath2I(param0 : *const Point, param1 : *const u8, param2 : i32, param3 : FillMode, path : *mut *mut GpPath) -> Status);
    unsafe { GdipCreatePath2I(param0, param1, param2, param3, path as _) }
}
#[inline]
pub unsafe fn GdipCreatePathGradient(points: *const PointF, count: i32, wrapmode: WrapMode, polygradient: *mut *mut GpPathGradient) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreatePathGradient(points : *const PointF, count : i32, wrapmode : WrapMode, polygradient : *mut *mut GpPathGradient) -> Status);
    unsafe { GdipCreatePathGradient(points, count, wrapmode, polygradient as _) }
}
#[inline]
pub unsafe fn GdipCreatePathGradientFromPath(path: *const GpPath, polygradient: *mut *mut GpPathGradient) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreatePathGradientFromPath(path : *const GpPath, polygradient : *mut *mut GpPathGradient) -> Status);
    unsafe { GdipCreatePathGradientFromPath(path, polygradient as _) }
}
#[inline]
pub unsafe fn GdipCreatePathGradientI(points: *const Point, count: i32, wrapmode: WrapMode, polygradient: *mut *mut GpPathGradient) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreatePathGradientI(points : *const Point, count : i32, wrapmode : WrapMode, polygradient : *mut *mut GpPathGradient) -> Status);
    unsafe { GdipCreatePathGradientI(points, count, wrapmode, polygradient as _) }
}
#[inline]
pub unsafe fn GdipCreatePathIter(iterator: *mut *mut GpPathIterator, path: *mut GpPath) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreatePathIter(iterator : *mut *mut GpPathIterator, path : *mut GpPath) -> Status);
    unsafe { GdipCreatePathIter(iterator as _, path as _) }
}
#[inline]
pub unsafe fn GdipCreatePen1(color: u32, width: f32, unit: Unit, pen: *mut *mut GpPen) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreatePen1(color : u32, width : f32, unit : Unit, pen : *mut *mut GpPen) -> Status);
    unsafe { GdipCreatePen1(color, width, unit, pen as _) }
}
#[inline]
pub unsafe fn GdipCreatePen2(brush: *mut GpBrush, width: f32, unit: Unit, pen: *mut *mut GpPen) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreatePen2(brush : *mut GpBrush, width : f32, unit : Unit, pen : *mut *mut GpPen) -> Status);
    unsafe { GdipCreatePen2(brush as _, width, unit, pen as _) }
}
#[inline]
pub unsafe fn GdipCreateRegion(region: *mut *mut GpRegion) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateRegion(region : *mut *mut GpRegion) -> Status);
    unsafe { GdipCreateRegion(region as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipCreateRegionHrgn(hrgn: super::Gdi::HRGN, region: *mut *mut GpRegion) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateRegionHrgn(hrgn : super::Gdi:: HRGN, region : *mut *mut GpRegion) -> Status);
    unsafe { GdipCreateRegionHrgn(hrgn, region as _) }
}
#[inline]
pub unsafe fn GdipCreateRegionPath(path: *mut GpPath, region: *mut *mut GpRegion) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateRegionPath(path : *mut GpPath, region : *mut *mut GpRegion) -> Status);
    unsafe { GdipCreateRegionPath(path as _, region as _) }
}
#[inline]
pub unsafe fn GdipCreateRegionRect(rect: *const RectF, region: *mut *mut GpRegion) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateRegionRect(rect : *const RectF, region : *mut *mut GpRegion) -> Status);
    unsafe { GdipCreateRegionRect(rect, region as _) }
}
#[inline]
pub unsafe fn GdipCreateRegionRectI(rect: *const Rect, region: *mut *mut GpRegion) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateRegionRectI(rect : *const Rect, region : *mut *mut GpRegion) -> Status);
    unsafe { GdipCreateRegionRectI(rect, region as _) }
}
#[inline]
pub unsafe fn GdipCreateRegionRgnData(regiondata: *const u8, size: i32, region: *mut *mut GpRegion) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateRegionRgnData(regiondata : *const u8, size : i32, region : *mut *mut GpRegion) -> Status);
    unsafe { GdipCreateRegionRgnData(regiondata, size, region as _) }
}
#[inline]
pub unsafe fn GdipCreateSolidFill(color: u32, brush: *mut *mut GpSolidFill) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateSolidFill(color : u32, brush : *mut *mut GpSolidFill) -> Status);
    unsafe { GdipCreateSolidFill(color, brush as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn GdipCreateStreamOnFile<P0>(filename: P0, access: u32, stream: *mut Option<super::super::System::Com::IStream>) -> Status
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateStreamOnFile(filename : windows_core::PCWSTR, access : u32, stream : *mut * mut core::ffi::c_void) -> Status);
    unsafe { GdipCreateStreamOnFile(filename.param().abi(), access, core::mem::transmute(stream)) }
}
#[inline]
pub unsafe fn GdipCreateStringFormat(formatattributes: i32, language: u16, format: *mut *mut GpStringFormat) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateStringFormat(formatattributes : i32, language : u16, format : *mut *mut GpStringFormat) -> Status);
    unsafe { GdipCreateStringFormat(formatattributes, language, format as _) }
}
#[inline]
pub unsafe fn GdipCreateTexture(image: *mut GpImage, wrapmode: WrapMode, texture: *mut *mut GpTexture) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateTexture(image : *mut GpImage, wrapmode : WrapMode, texture : *mut *mut GpTexture) -> Status);
    unsafe { GdipCreateTexture(image as _, wrapmode, texture as _) }
}
#[inline]
pub unsafe fn GdipCreateTexture2(image: *mut GpImage, wrapmode: WrapMode, x: f32, y: f32, width: f32, height: f32, texture: *mut *mut GpTexture) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateTexture2(image : *mut GpImage, wrapmode : WrapMode, x : f32, y : f32, width : f32, height : f32, texture : *mut *mut GpTexture) -> Status);
    unsafe { GdipCreateTexture2(image as _, wrapmode, x, y, width, height, texture as _) }
}
#[inline]
pub unsafe fn GdipCreateTexture2I(image: *mut GpImage, wrapmode: WrapMode, x: i32, y: i32, width: i32, height: i32, texture: *mut *mut GpTexture) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateTexture2I(image : *mut GpImage, wrapmode : WrapMode, x : i32, y : i32, width : i32, height : i32, texture : *mut *mut GpTexture) -> Status);
    unsafe { GdipCreateTexture2I(image as _, wrapmode, x, y, width, height, texture as _) }
}
#[inline]
pub unsafe fn GdipCreateTextureIA(image: *mut GpImage, imageattributes: *const GpImageAttributes, x: f32, y: f32, width: f32, height: f32, texture: *mut *mut GpTexture) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateTextureIA(image : *mut GpImage, imageattributes : *const GpImageAttributes, x : f32, y : f32, width : f32, height : f32, texture : *mut *mut GpTexture) -> Status);
    unsafe { GdipCreateTextureIA(image as _, imageattributes, x, y, width, height, texture as _) }
}
#[inline]
pub unsafe fn GdipCreateTextureIAI(image: *mut GpImage, imageattributes: *const GpImageAttributes, x: i32, y: i32, width: i32, height: i32, texture: *mut *mut GpTexture) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipCreateTextureIAI(image : *mut GpImage, imageattributes : *const GpImageAttributes, x : i32, y : i32, width : i32, height : i32, texture : *mut *mut GpTexture) -> Status);
    unsafe { GdipCreateTextureIAI(image as _, imageattributes, x, y, width, height, texture as _) }
}
#[inline]
pub unsafe fn GdipDeleteBrush(brush: *mut GpBrush) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDeleteBrush(brush : *mut GpBrush) -> Status);
    unsafe { GdipDeleteBrush(brush as _) }
}
#[inline]
pub unsafe fn GdipDeleteCachedBitmap(cachedbitmap: *mut GpCachedBitmap) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDeleteCachedBitmap(cachedbitmap : *mut GpCachedBitmap) -> Status);
    unsafe { GdipDeleteCachedBitmap(cachedbitmap as _) }
}
#[inline]
pub unsafe fn GdipDeleteCustomLineCap(customcap: *mut GpCustomLineCap) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDeleteCustomLineCap(customcap : *mut GpCustomLineCap) -> Status);
    unsafe { GdipDeleteCustomLineCap(customcap as _) }
}
#[inline]
pub unsafe fn GdipDeleteEffect(effect: *mut CGpEffect) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDeleteEffect(effect : *mut CGpEffect) -> Status);
    unsafe { GdipDeleteEffect(effect as _) }
}
#[inline]
pub unsafe fn GdipDeleteFont(font: *mut GpFont) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDeleteFont(font : *mut GpFont) -> Status);
    unsafe { GdipDeleteFont(font as _) }
}
#[inline]
pub unsafe fn GdipDeleteFontFamily(fontfamily: *mut GpFontFamily) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDeleteFontFamily(fontfamily : *mut GpFontFamily) -> Status);
    unsafe { GdipDeleteFontFamily(fontfamily as _) }
}
#[inline]
pub unsafe fn GdipDeleteGraphics(graphics: *mut GpGraphics) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDeleteGraphics(graphics : *mut GpGraphics) -> Status);
    unsafe { GdipDeleteGraphics(graphics as _) }
}
#[inline]
pub unsafe fn GdipDeleteMatrix(matrix: *mut Matrix) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDeleteMatrix(matrix : *mut Matrix) -> Status);
    unsafe { GdipDeleteMatrix(matrix as _) }
}
#[inline]
pub unsafe fn GdipDeletePath(path: *mut GpPath) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDeletePath(path : *mut GpPath) -> Status);
    unsafe { GdipDeletePath(path as _) }
}
#[inline]
pub unsafe fn GdipDeletePathIter(iterator: *mut GpPathIterator) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDeletePathIter(iterator : *mut GpPathIterator) -> Status);
    unsafe { GdipDeletePathIter(iterator as _) }
}
#[inline]
pub unsafe fn GdipDeletePen(pen: *mut GpPen) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDeletePen(pen : *mut GpPen) -> Status);
    unsafe { GdipDeletePen(pen as _) }
}
#[inline]
pub unsafe fn GdipDeletePrivateFontCollection(fontcollection: *mut *mut GpFontCollection) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDeletePrivateFontCollection(fontcollection : *mut *mut GpFontCollection) -> Status);
    unsafe { GdipDeletePrivateFontCollection(fontcollection as _) }
}
#[inline]
pub unsafe fn GdipDeleteRegion(region: *mut GpRegion) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDeleteRegion(region : *mut GpRegion) -> Status);
    unsafe { GdipDeleteRegion(region as _) }
}
#[inline]
pub unsafe fn GdipDeleteStringFormat(format: *mut GpStringFormat) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDeleteStringFormat(format : *mut GpStringFormat) -> Status);
    unsafe { GdipDeleteStringFormat(format as _) }
}
#[inline]
pub unsafe fn GdipDisposeImage(image: *mut GpImage) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDisposeImage(image : *mut GpImage) -> Status);
    unsafe { GdipDisposeImage(image as _) }
}
#[inline]
pub unsafe fn GdipDisposeImageAttributes(imageattr: *mut GpImageAttributes) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDisposeImageAttributes(imageattr : *mut GpImageAttributes) -> Status);
    unsafe { GdipDisposeImageAttributes(imageattr as _) }
}
#[inline]
pub unsafe fn GdipDrawArc(graphics: *mut GpGraphics, pen: *mut GpPen, x: f32, y: f32, width: f32, height: f32, startangle: f32, sweepangle: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawArc(graphics : *mut GpGraphics, pen : *mut GpPen, x : f32, y : f32, width : f32, height : f32, startangle : f32, sweepangle : f32) -> Status);
    unsafe { GdipDrawArc(graphics as _, pen as _, x, y, width, height, startangle, sweepangle) }
}
#[inline]
pub unsafe fn GdipDrawArcI(graphics: *mut GpGraphics, pen: *mut GpPen, x: i32, y: i32, width: i32, height: i32, startangle: f32, sweepangle: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawArcI(graphics : *mut GpGraphics, pen : *mut GpPen, x : i32, y : i32, width : i32, height : i32, startangle : f32, sweepangle : f32) -> Status);
    unsafe { GdipDrawArcI(graphics as _, pen as _, x, y, width, height, startangle, sweepangle) }
}
#[inline]
pub unsafe fn GdipDrawBezier(graphics: *mut GpGraphics, pen: *mut GpPen, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, x4: f32, y4: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawBezier(graphics : *mut GpGraphics, pen : *mut GpPen, x1 : f32, y1 : f32, x2 : f32, y2 : f32, x3 : f32, y3 : f32, x4 : f32, y4 : f32) -> Status);
    unsafe { GdipDrawBezier(graphics as _, pen as _, x1, y1, x2, y2, x3, y3, x4, y4) }
}
#[inline]
pub unsafe fn GdipDrawBezierI(graphics: *mut GpGraphics, pen: *mut GpPen, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, x4: i32, y4: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawBezierI(graphics : *mut GpGraphics, pen : *mut GpPen, x1 : i32, y1 : i32, x2 : i32, y2 : i32, x3 : i32, y3 : i32, x4 : i32, y4 : i32) -> Status);
    unsafe { GdipDrawBezierI(graphics as _, pen as _, x1, y1, x2, y2, x3, y3, x4, y4) }
}
#[inline]
pub unsafe fn GdipDrawBeziers(graphics: *mut GpGraphics, pen: *mut GpPen, points: *const PointF, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawBeziers(graphics : *mut GpGraphics, pen : *mut GpPen, points : *const PointF, count : i32) -> Status);
    unsafe { GdipDrawBeziers(graphics as _, pen as _, points, count) }
}
#[inline]
pub unsafe fn GdipDrawBeziersI(graphics: *mut GpGraphics, pen: *mut GpPen, points: *const Point, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawBeziersI(graphics : *mut GpGraphics, pen : *mut GpPen, points : *const Point, count : i32) -> Status);
    unsafe { GdipDrawBeziersI(graphics as _, pen as _, points, count) }
}
#[inline]
pub unsafe fn GdipDrawCachedBitmap(graphics: *mut GpGraphics, cachedbitmap: *mut GpCachedBitmap, x: i32, y: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawCachedBitmap(graphics : *mut GpGraphics, cachedbitmap : *mut GpCachedBitmap, x : i32, y : i32) -> Status);
    unsafe { GdipDrawCachedBitmap(graphics as _, cachedbitmap as _, x, y) }
}
#[inline]
pub unsafe fn GdipDrawClosedCurve(graphics: *mut GpGraphics, pen: *mut GpPen, points: *const PointF, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawClosedCurve(graphics : *mut GpGraphics, pen : *mut GpPen, points : *const PointF, count : i32) -> Status);
    unsafe { GdipDrawClosedCurve(graphics as _, pen as _, points, count) }
}
#[inline]
pub unsafe fn GdipDrawClosedCurve2(graphics: *mut GpGraphics, pen: *mut GpPen, points: *const PointF, count: i32, tension: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawClosedCurve2(graphics : *mut GpGraphics, pen : *mut GpPen, points : *const PointF, count : i32, tension : f32) -> Status);
    unsafe { GdipDrawClosedCurve2(graphics as _, pen as _, points, count, tension) }
}
#[inline]
pub unsafe fn GdipDrawClosedCurve2I(graphics: *mut GpGraphics, pen: *mut GpPen, points: *const Point, count: i32, tension: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawClosedCurve2I(graphics : *mut GpGraphics, pen : *mut GpPen, points : *const Point, count : i32, tension : f32) -> Status);
    unsafe { GdipDrawClosedCurve2I(graphics as _, pen as _, points, count, tension) }
}
#[inline]
pub unsafe fn GdipDrawClosedCurveI(graphics: *mut GpGraphics, pen: *mut GpPen, points: *const Point, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawClosedCurveI(graphics : *mut GpGraphics, pen : *mut GpPen, points : *const Point, count : i32) -> Status);
    unsafe { GdipDrawClosedCurveI(graphics as _, pen as _, points, count) }
}
#[inline]
pub unsafe fn GdipDrawCurve(graphics: *mut GpGraphics, pen: *mut GpPen, points: *const PointF, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawCurve(graphics : *mut GpGraphics, pen : *mut GpPen, points : *const PointF, count : i32) -> Status);
    unsafe { GdipDrawCurve(graphics as _, pen as _, points, count) }
}
#[inline]
pub unsafe fn GdipDrawCurve2(graphics: *mut GpGraphics, pen: *mut GpPen, points: *const PointF, count: i32, tension: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawCurve2(graphics : *mut GpGraphics, pen : *mut GpPen, points : *const PointF, count : i32, tension : f32) -> Status);
    unsafe { GdipDrawCurve2(graphics as _, pen as _, points, count, tension) }
}
#[inline]
pub unsafe fn GdipDrawCurve2I(graphics: *mut GpGraphics, pen: *mut GpPen, points: *const Point, count: i32, tension: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawCurve2I(graphics : *mut GpGraphics, pen : *mut GpPen, points : *const Point, count : i32, tension : f32) -> Status);
    unsafe { GdipDrawCurve2I(graphics as _, pen as _, points, count, tension) }
}
#[inline]
pub unsafe fn GdipDrawCurve3(graphics: *mut GpGraphics, pen: *mut GpPen, points: *const PointF, count: i32, offset: i32, numberofsegments: i32, tension: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawCurve3(graphics : *mut GpGraphics, pen : *mut GpPen, points : *const PointF, count : i32, offset : i32, numberofsegments : i32, tension : f32) -> Status);
    unsafe { GdipDrawCurve3(graphics as _, pen as _, points, count, offset, numberofsegments, tension) }
}
#[inline]
pub unsafe fn GdipDrawCurve3I(graphics: *mut GpGraphics, pen: *mut GpPen, points: *const Point, count: i32, offset: i32, numberofsegments: i32, tension: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawCurve3I(graphics : *mut GpGraphics, pen : *mut GpPen, points : *const Point, count : i32, offset : i32, numberofsegments : i32, tension : f32) -> Status);
    unsafe { GdipDrawCurve3I(graphics as _, pen as _, points, count, offset, numberofsegments, tension) }
}
#[inline]
pub unsafe fn GdipDrawCurveI(graphics: *mut GpGraphics, pen: *mut GpPen, points: *const Point, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawCurveI(graphics : *mut GpGraphics, pen : *mut GpPen, points : *const Point, count : i32) -> Status);
    unsafe { GdipDrawCurveI(graphics as _, pen as _, points, count) }
}
#[inline]
pub unsafe fn GdipDrawDriverString(graphics: *mut GpGraphics, text: *const u16, length: i32, font: *const GpFont, brush: *const GpBrush, positions: *const PointF, flags: i32, matrix: *const Matrix) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawDriverString(graphics : *mut GpGraphics, text : *const u16, length : i32, font : *const GpFont, brush : *const GpBrush, positions : *const PointF, flags : i32, matrix : *const Matrix) -> Status);
    unsafe { GdipDrawDriverString(graphics as _, text, length, font, brush, positions, flags, matrix) }
}
#[inline]
pub unsafe fn GdipDrawEllipse(graphics: *mut GpGraphics, pen: *mut GpPen, x: f32, y: f32, width: f32, height: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawEllipse(graphics : *mut GpGraphics, pen : *mut GpPen, x : f32, y : f32, width : f32, height : f32) -> Status);
    unsafe { GdipDrawEllipse(graphics as _, pen as _, x, y, width, height) }
}
#[inline]
pub unsafe fn GdipDrawEllipseI(graphics: *mut GpGraphics, pen: *mut GpPen, x: i32, y: i32, width: i32, height: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawEllipseI(graphics : *mut GpGraphics, pen : *mut GpPen, x : i32, y : i32, width : i32, height : i32) -> Status);
    unsafe { GdipDrawEllipseI(graphics as _, pen as _, x, y, width, height) }
}
#[inline]
pub unsafe fn GdipDrawImage(graphics: *mut GpGraphics, image: *mut GpImage, x: f32, y: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawImage(graphics : *mut GpGraphics, image : *mut GpImage, x : f32, y : f32) -> Status);
    unsafe { GdipDrawImage(graphics as _, image as _, x, y) }
}
#[inline]
pub unsafe fn GdipDrawImageFX(graphics: *mut GpGraphics, image: *mut GpImage, source: *mut RectF, xform: *mut Matrix, effect: *mut CGpEffect, imageattributes: *mut GpImageAttributes, srcunit: Unit) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawImageFX(graphics : *mut GpGraphics, image : *mut GpImage, source : *mut RectF, xform : *mut Matrix, effect : *mut CGpEffect, imageattributes : *mut GpImageAttributes, srcunit : Unit) -> Status);
    unsafe { GdipDrawImageFX(graphics as _, image as _, source as _, xform as _, effect as _, imageattributes as _, srcunit) }
}
#[inline]
pub unsafe fn GdipDrawImageI(graphics: *mut GpGraphics, image: *mut GpImage, x: i32, y: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawImageI(graphics : *mut GpGraphics, image : *mut GpImage, x : i32, y : i32) -> Status);
    unsafe { GdipDrawImageI(graphics as _, image as _, x, y) }
}
#[inline]
pub unsafe fn GdipDrawImagePointRect(graphics: *mut GpGraphics, image: *mut GpImage, x: f32, y: f32, srcx: f32, srcy: f32, srcwidth: f32, srcheight: f32, srcunit: Unit) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawImagePointRect(graphics : *mut GpGraphics, image : *mut GpImage, x : f32, y : f32, srcx : f32, srcy : f32, srcwidth : f32, srcheight : f32, srcunit : Unit) -> Status);
    unsafe { GdipDrawImagePointRect(graphics as _, image as _, x, y, srcx, srcy, srcwidth, srcheight, srcunit) }
}
#[inline]
pub unsafe fn GdipDrawImagePointRectI(graphics: *mut GpGraphics, image: *mut GpImage, x: i32, y: i32, srcx: i32, srcy: i32, srcwidth: i32, srcheight: i32, srcunit: Unit) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawImagePointRectI(graphics : *mut GpGraphics, image : *mut GpImage, x : i32, y : i32, srcx : i32, srcy : i32, srcwidth : i32, srcheight : i32, srcunit : Unit) -> Status);
    unsafe { GdipDrawImagePointRectI(graphics as _, image as _, x, y, srcx, srcy, srcwidth, srcheight, srcunit) }
}
#[inline]
pub unsafe fn GdipDrawImagePoints(graphics: *mut GpGraphics, image: *mut GpImage, dstpoints: *const PointF, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawImagePoints(graphics : *mut GpGraphics, image : *mut GpImage, dstpoints : *const PointF, count : i32) -> Status);
    unsafe { GdipDrawImagePoints(graphics as _, image as _, dstpoints, count) }
}
#[inline]
pub unsafe fn GdipDrawImagePointsI(graphics: *mut GpGraphics, image: *mut GpImage, dstpoints: *const Point, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawImagePointsI(graphics : *mut GpGraphics, image : *mut GpImage, dstpoints : *const Point, count : i32) -> Status);
    unsafe { GdipDrawImagePointsI(graphics as _, image as _, dstpoints, count) }
}
#[inline]
pub unsafe fn GdipDrawImagePointsRect(graphics: *mut GpGraphics, image: *mut GpImage, points: *const PointF, count: i32, srcx: f32, srcy: f32, srcwidth: f32, srcheight: f32, srcunit: Unit, imageattributes: *const GpImageAttributes, callback: isize, callbackdata: *mut core::ffi::c_void) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawImagePointsRect(graphics : *mut GpGraphics, image : *mut GpImage, points : *const PointF, count : i32, srcx : f32, srcy : f32, srcwidth : f32, srcheight : f32, srcunit : Unit, imageattributes : *const GpImageAttributes, callback : isize, callbackdata : *mut core::ffi::c_void) -> Status);
    unsafe { GdipDrawImagePointsRect(graphics as _, image as _, points, count, srcx, srcy, srcwidth, srcheight, srcunit, imageattributes, callback, callbackdata as _) }
}
#[inline]
pub unsafe fn GdipDrawImagePointsRectI(graphics: *mut GpGraphics, image: *mut GpImage, points: *const Point, count: i32, srcx: i32, srcy: i32, srcwidth: i32, srcheight: i32, srcunit: Unit, imageattributes: *const GpImageAttributes, callback: isize, callbackdata: *mut core::ffi::c_void) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawImagePointsRectI(graphics : *mut GpGraphics, image : *mut GpImage, points : *const Point, count : i32, srcx : i32, srcy : i32, srcwidth : i32, srcheight : i32, srcunit : Unit, imageattributes : *const GpImageAttributes, callback : isize, callbackdata : *mut core::ffi::c_void) -> Status);
    unsafe { GdipDrawImagePointsRectI(graphics as _, image as _, points, count, srcx, srcy, srcwidth, srcheight, srcunit, imageattributes, callback, callbackdata as _) }
}
#[inline]
pub unsafe fn GdipDrawImageRect(graphics: *mut GpGraphics, image: *mut GpImage, x: f32, y: f32, width: f32, height: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawImageRect(graphics : *mut GpGraphics, image : *mut GpImage, x : f32, y : f32, width : f32, height : f32) -> Status);
    unsafe { GdipDrawImageRect(graphics as _, image as _, x, y, width, height) }
}
#[inline]
pub unsafe fn GdipDrawImageRectI(graphics: *mut GpGraphics, image: *mut GpImage, x: i32, y: i32, width: i32, height: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawImageRectI(graphics : *mut GpGraphics, image : *mut GpImage, x : i32, y : i32, width : i32, height : i32) -> Status);
    unsafe { GdipDrawImageRectI(graphics as _, image as _, x, y, width, height) }
}
#[inline]
pub unsafe fn GdipDrawImageRectRect(graphics: *mut GpGraphics, image: *mut GpImage, dstx: f32, dsty: f32, dstwidth: f32, dstheight: f32, srcx: f32, srcy: f32, srcwidth: f32, srcheight: f32, srcunit: Unit, imageattributes: *const GpImageAttributes, callback: isize, callbackdata: *mut core::ffi::c_void) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawImageRectRect(graphics : *mut GpGraphics, image : *mut GpImage, dstx : f32, dsty : f32, dstwidth : f32, dstheight : f32, srcx : f32, srcy : f32, srcwidth : f32, srcheight : f32, srcunit : Unit, imageattributes : *const GpImageAttributes, callback : isize, callbackdata : *mut core::ffi::c_void) -> Status);
    unsafe { GdipDrawImageRectRect(graphics as _, image as _, dstx, dsty, dstwidth, dstheight, srcx, srcy, srcwidth, srcheight, srcunit, imageattributes, callback, callbackdata as _) }
}
#[inline]
pub unsafe fn GdipDrawImageRectRectI(graphics: *mut GpGraphics, image: *mut GpImage, dstx: i32, dsty: i32, dstwidth: i32, dstheight: i32, srcx: i32, srcy: i32, srcwidth: i32, srcheight: i32, srcunit: Unit, imageattributes: *const GpImageAttributes, callback: isize, callbackdata: *mut core::ffi::c_void) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawImageRectRectI(graphics : *mut GpGraphics, image : *mut GpImage, dstx : i32, dsty : i32, dstwidth : i32, dstheight : i32, srcx : i32, srcy : i32, srcwidth : i32, srcheight : i32, srcunit : Unit, imageattributes : *const GpImageAttributes, callback : isize, callbackdata : *mut core::ffi::c_void) -> Status);
    unsafe { GdipDrawImageRectRectI(graphics as _, image as _, dstx, dsty, dstwidth, dstheight, srcx, srcy, srcwidth, srcheight, srcunit, imageattributes, callback, callbackdata as _) }
}
#[inline]
pub unsafe fn GdipDrawLine(graphics: *mut GpGraphics, pen: *mut GpPen, x1: f32, y1: f32, x2: f32, y2: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawLine(graphics : *mut GpGraphics, pen : *mut GpPen, x1 : f32, y1 : f32, x2 : f32, y2 : f32) -> Status);
    unsafe { GdipDrawLine(graphics as _, pen as _, x1, y1, x2, y2) }
}
#[inline]
pub unsafe fn GdipDrawLineI(graphics: *mut GpGraphics, pen: *mut GpPen, x1: i32, y1: i32, x2: i32, y2: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawLineI(graphics : *mut GpGraphics, pen : *mut GpPen, x1 : i32, y1 : i32, x2 : i32, y2 : i32) -> Status);
    unsafe { GdipDrawLineI(graphics as _, pen as _, x1, y1, x2, y2) }
}
#[inline]
pub unsafe fn GdipDrawLines(graphics: *mut GpGraphics, pen: *mut GpPen, points: *const PointF, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawLines(graphics : *mut GpGraphics, pen : *mut GpPen, points : *const PointF, count : i32) -> Status);
    unsafe { GdipDrawLines(graphics as _, pen as _, points, count) }
}
#[inline]
pub unsafe fn GdipDrawLinesI(graphics: *mut GpGraphics, pen: *mut GpPen, points: *const Point, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawLinesI(graphics : *mut GpGraphics, pen : *mut GpPen, points : *const Point, count : i32) -> Status);
    unsafe { GdipDrawLinesI(graphics as _, pen as _, points, count) }
}
#[inline]
pub unsafe fn GdipDrawPath(graphics: *mut GpGraphics, pen: *mut GpPen, path: *mut GpPath) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawPath(graphics : *mut GpGraphics, pen : *mut GpPen, path : *mut GpPath) -> Status);
    unsafe { GdipDrawPath(graphics as _, pen as _, path as _) }
}
#[inline]
pub unsafe fn GdipDrawPie(graphics: *mut GpGraphics, pen: *mut GpPen, x: f32, y: f32, width: f32, height: f32, startangle: f32, sweepangle: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawPie(graphics : *mut GpGraphics, pen : *mut GpPen, x : f32, y : f32, width : f32, height : f32, startangle : f32, sweepangle : f32) -> Status);
    unsafe { GdipDrawPie(graphics as _, pen as _, x, y, width, height, startangle, sweepangle) }
}
#[inline]
pub unsafe fn GdipDrawPieI(graphics: *mut GpGraphics, pen: *mut GpPen, x: i32, y: i32, width: i32, height: i32, startangle: f32, sweepangle: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawPieI(graphics : *mut GpGraphics, pen : *mut GpPen, x : i32, y : i32, width : i32, height : i32, startangle : f32, sweepangle : f32) -> Status);
    unsafe { GdipDrawPieI(graphics as _, pen as _, x, y, width, height, startangle, sweepangle) }
}
#[inline]
pub unsafe fn GdipDrawPolygon(graphics: *mut GpGraphics, pen: *mut GpPen, points: *const PointF, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawPolygon(graphics : *mut GpGraphics, pen : *mut GpPen, points : *const PointF, count : i32) -> Status);
    unsafe { GdipDrawPolygon(graphics as _, pen as _, points, count) }
}
#[inline]
pub unsafe fn GdipDrawPolygonI(graphics: *mut GpGraphics, pen: *mut GpPen, points: *const Point, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawPolygonI(graphics : *mut GpGraphics, pen : *mut GpPen, points : *const Point, count : i32) -> Status);
    unsafe { GdipDrawPolygonI(graphics as _, pen as _, points, count) }
}
#[inline]
pub unsafe fn GdipDrawRectangle(graphics: *mut GpGraphics, pen: *mut GpPen, x: f32, y: f32, width: f32, height: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawRectangle(graphics : *mut GpGraphics, pen : *mut GpPen, x : f32, y : f32, width : f32, height : f32) -> Status);
    unsafe { GdipDrawRectangle(graphics as _, pen as _, x, y, width, height) }
}
#[inline]
pub unsafe fn GdipDrawRectangleI(graphics: *mut GpGraphics, pen: *mut GpPen, x: i32, y: i32, width: i32, height: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawRectangleI(graphics : *mut GpGraphics, pen : *mut GpPen, x : i32, y : i32, width : i32, height : i32) -> Status);
    unsafe { GdipDrawRectangleI(graphics as _, pen as _, x, y, width, height) }
}
#[inline]
pub unsafe fn GdipDrawRectangles(graphics: *mut GpGraphics, pen: *mut GpPen, rects: *const RectF, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawRectangles(graphics : *mut GpGraphics, pen : *mut GpPen, rects : *const RectF, count : i32) -> Status);
    unsafe { GdipDrawRectangles(graphics as _, pen as _, rects, count) }
}
#[inline]
pub unsafe fn GdipDrawRectanglesI(graphics: *mut GpGraphics, pen: *mut GpPen, rects: *const Rect, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawRectanglesI(graphics : *mut GpGraphics, pen : *mut GpPen, rects : *const Rect, count : i32) -> Status);
    unsafe { GdipDrawRectanglesI(graphics as _, pen as _, rects, count) }
}
#[inline]
pub unsafe fn GdipDrawString<P1>(graphics: *mut GpGraphics, string: P1, length: i32, font: *const GpFont, layoutrect: *const RectF, stringformat: *const GpStringFormat, brush: *const GpBrush) -> Status
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("gdiplus.dll" "system" fn GdipDrawString(graphics : *mut GpGraphics, string : windows_core::PCWSTR, length : i32, font : *const GpFont, layoutrect : *const RectF, stringformat : *const GpStringFormat, brush : *const GpBrush) -> Status);
    unsafe { GdipDrawString(graphics as _, string.param().abi(), length, font, layoutrect, stringformat, brush) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipEmfToWmfBits(hemf: super::Gdi::HENHMETAFILE, pdata16: Option<&mut [u8]>, imapmode: i32, eflags: i32) -> u32 {
    windows_core::link!("gdiplus.dll" "system" fn GdipEmfToWmfBits(hemf : super::Gdi:: HENHMETAFILE, cbdata16 : u32, pdata16 : *mut u8, imapmode : i32, eflags : i32) -> u32);
    unsafe { GdipEmfToWmfBits(hemf, pdata16.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pdata16.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), imapmode, eflags) }
}
#[inline]
pub unsafe fn GdipEndContainer(graphics: *mut GpGraphics, state: u32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipEndContainer(graphics : *mut GpGraphics, state : u32) -> Status);
    unsafe { GdipEndContainer(graphics as _, state) }
}
#[inline]
pub unsafe fn GdipEnumerateMetafileDestPoint(graphics: *mut GpGraphics, metafile: *const GpMetafile, destpoint: *const PointF, callback: isize, callbackdata: *mut core::ffi::c_void, imageattributes: *const GpImageAttributes) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipEnumerateMetafileDestPoint(graphics : *mut GpGraphics, metafile : *const GpMetafile, destpoint : *const PointF, callback : isize, callbackdata : *mut core::ffi::c_void, imageattributes : *const GpImageAttributes) -> Status);
    unsafe { GdipEnumerateMetafileDestPoint(graphics as _, metafile, destpoint, callback, callbackdata as _, imageattributes) }
}
#[inline]
pub unsafe fn GdipEnumerateMetafileDestPointI(graphics: *mut GpGraphics, metafile: *const GpMetafile, destpoint: *const Point, callback: isize, callbackdata: *mut core::ffi::c_void, imageattributes: *const GpImageAttributes) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipEnumerateMetafileDestPointI(graphics : *mut GpGraphics, metafile : *const GpMetafile, destpoint : *const Point, callback : isize, callbackdata : *mut core::ffi::c_void, imageattributes : *const GpImageAttributes) -> Status);
    unsafe { GdipEnumerateMetafileDestPointI(graphics as _, metafile, destpoint, callback, callbackdata as _, imageattributes) }
}
#[inline]
pub unsafe fn GdipEnumerateMetafileDestPoints(graphics: *mut GpGraphics, metafile: *const GpMetafile, destpoints: *const PointF, count: i32, callback: isize, callbackdata: *mut core::ffi::c_void, imageattributes: *const GpImageAttributes) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipEnumerateMetafileDestPoints(graphics : *mut GpGraphics, metafile : *const GpMetafile, destpoints : *const PointF, count : i32, callback : isize, callbackdata : *mut core::ffi::c_void, imageattributes : *const GpImageAttributes) -> Status);
    unsafe { GdipEnumerateMetafileDestPoints(graphics as _, metafile, destpoints, count, callback, callbackdata as _, imageattributes) }
}
#[inline]
pub unsafe fn GdipEnumerateMetafileDestPointsI(graphics: *mut GpGraphics, metafile: *const GpMetafile, destpoints: *const Point, count: i32, callback: isize, callbackdata: *mut core::ffi::c_void, imageattributes: *const GpImageAttributes) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipEnumerateMetafileDestPointsI(graphics : *mut GpGraphics, metafile : *const GpMetafile, destpoints : *const Point, count : i32, callback : isize, callbackdata : *mut core::ffi::c_void, imageattributes : *const GpImageAttributes) -> Status);
    unsafe { GdipEnumerateMetafileDestPointsI(graphics as _, metafile, destpoints, count, callback, callbackdata as _, imageattributes) }
}
#[inline]
pub unsafe fn GdipEnumerateMetafileDestRect(graphics: *mut GpGraphics, metafile: *const GpMetafile, destrect: *const RectF, callback: isize, callbackdata: *mut core::ffi::c_void, imageattributes: *const GpImageAttributes) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipEnumerateMetafileDestRect(graphics : *mut GpGraphics, metafile : *const GpMetafile, destrect : *const RectF, callback : isize, callbackdata : *mut core::ffi::c_void, imageattributes : *const GpImageAttributes) -> Status);
    unsafe { GdipEnumerateMetafileDestRect(graphics as _, metafile, destrect, callback, callbackdata as _, imageattributes) }
}
#[inline]
pub unsafe fn GdipEnumerateMetafileDestRectI(graphics: *mut GpGraphics, metafile: *const GpMetafile, destrect: *const Rect, callback: isize, callbackdata: *mut core::ffi::c_void, imageattributes: *const GpImageAttributes) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipEnumerateMetafileDestRectI(graphics : *mut GpGraphics, metafile : *const GpMetafile, destrect : *const Rect, callback : isize, callbackdata : *mut core::ffi::c_void, imageattributes : *const GpImageAttributes) -> Status);
    unsafe { GdipEnumerateMetafileDestRectI(graphics as _, metafile, destrect, callback, callbackdata as _, imageattributes) }
}
#[inline]
pub unsafe fn GdipEnumerateMetafileSrcRectDestPoint(graphics: *mut GpGraphics, metafile: *const GpMetafile, destpoint: *const PointF, srcrect: *const RectF, srcunit: Unit, callback: isize, callbackdata: *mut core::ffi::c_void, imageattributes: *const GpImageAttributes) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipEnumerateMetafileSrcRectDestPoint(graphics : *mut GpGraphics, metafile : *const GpMetafile, destpoint : *const PointF, srcrect : *const RectF, srcunit : Unit, callback : isize, callbackdata : *mut core::ffi::c_void, imageattributes : *const GpImageAttributes) -> Status);
    unsafe { GdipEnumerateMetafileSrcRectDestPoint(graphics as _, metafile, destpoint, srcrect, srcunit, callback, callbackdata as _, imageattributes) }
}
#[inline]
pub unsafe fn GdipEnumerateMetafileSrcRectDestPointI(graphics: *mut GpGraphics, metafile: *const GpMetafile, destpoint: *const Point, srcrect: *const Rect, srcunit: Unit, callback: isize, callbackdata: *mut core::ffi::c_void, imageattributes: *const GpImageAttributes) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipEnumerateMetafileSrcRectDestPointI(graphics : *mut GpGraphics, metafile : *const GpMetafile, destpoint : *const Point, srcrect : *const Rect, srcunit : Unit, callback : isize, callbackdata : *mut core::ffi::c_void, imageattributes : *const GpImageAttributes) -> Status);
    unsafe { GdipEnumerateMetafileSrcRectDestPointI(graphics as _, metafile, destpoint, srcrect, srcunit, callback, callbackdata as _, imageattributes) }
}
#[inline]
pub unsafe fn GdipEnumerateMetafileSrcRectDestPoints(graphics: *mut GpGraphics, metafile: *const GpMetafile, destpoints: *const PointF, count: i32, srcrect: *const RectF, srcunit: Unit, callback: isize, callbackdata: *mut core::ffi::c_void, imageattributes: *const GpImageAttributes) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipEnumerateMetafileSrcRectDestPoints(graphics : *mut GpGraphics, metafile : *const GpMetafile, destpoints : *const PointF, count : i32, srcrect : *const RectF, srcunit : Unit, callback : isize, callbackdata : *mut core::ffi::c_void, imageattributes : *const GpImageAttributes) -> Status);
    unsafe { GdipEnumerateMetafileSrcRectDestPoints(graphics as _, metafile, destpoints, count, srcrect, srcunit, callback, callbackdata as _, imageattributes) }
}
#[inline]
pub unsafe fn GdipEnumerateMetafileSrcRectDestPointsI(graphics: *mut GpGraphics, metafile: *const GpMetafile, destpoints: *const Point, count: i32, srcrect: *const Rect, srcunit: Unit, callback: isize, callbackdata: *mut core::ffi::c_void, imageattributes: *const GpImageAttributes) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipEnumerateMetafileSrcRectDestPointsI(graphics : *mut GpGraphics, metafile : *const GpMetafile, destpoints : *const Point, count : i32, srcrect : *const Rect, srcunit : Unit, callback : isize, callbackdata : *mut core::ffi::c_void, imageattributes : *const GpImageAttributes) -> Status);
    unsafe { GdipEnumerateMetafileSrcRectDestPointsI(graphics as _, metafile, destpoints, count, srcrect, srcunit, callback, callbackdata as _, imageattributes) }
}
#[inline]
pub unsafe fn GdipEnumerateMetafileSrcRectDestRect(graphics: *mut GpGraphics, metafile: *const GpMetafile, destrect: *const RectF, srcrect: *const RectF, srcunit: Unit, callback: isize, callbackdata: *mut core::ffi::c_void, imageattributes: *const GpImageAttributes) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipEnumerateMetafileSrcRectDestRect(graphics : *mut GpGraphics, metafile : *const GpMetafile, destrect : *const RectF, srcrect : *const RectF, srcunit : Unit, callback : isize, callbackdata : *mut core::ffi::c_void, imageattributes : *const GpImageAttributes) -> Status);
    unsafe { GdipEnumerateMetafileSrcRectDestRect(graphics as _, metafile, destrect, srcrect, srcunit, callback, callbackdata as _, imageattributes) }
}
#[inline]
pub unsafe fn GdipEnumerateMetafileSrcRectDestRectI(graphics: *mut GpGraphics, metafile: *const GpMetafile, destrect: *const Rect, srcrect: *const Rect, srcunit: Unit, callback: isize, callbackdata: *mut core::ffi::c_void, imageattributes: *const GpImageAttributes) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipEnumerateMetafileSrcRectDestRectI(graphics : *mut GpGraphics, metafile : *const GpMetafile, destrect : *const Rect, srcrect : *const Rect, srcunit : Unit, callback : isize, callbackdata : *mut core::ffi::c_void, imageattributes : *const GpImageAttributes) -> Status);
    unsafe { GdipEnumerateMetafileSrcRectDestRectI(graphics as _, metafile, destrect, srcrect, srcunit, callback, callbackdata as _, imageattributes) }
}
#[inline]
pub unsafe fn GdipFillClosedCurve(graphics: *mut GpGraphics, brush: *mut GpBrush, points: *const PointF, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipFillClosedCurve(graphics : *mut GpGraphics, brush : *mut GpBrush, points : *const PointF, count : i32) -> Status);
    unsafe { GdipFillClosedCurve(graphics as _, brush as _, points, count) }
}
#[inline]
pub unsafe fn GdipFillClosedCurve2(graphics: *mut GpGraphics, brush: *mut GpBrush, points: *const PointF, count: i32, tension: f32, fillmode: FillMode) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipFillClosedCurve2(graphics : *mut GpGraphics, brush : *mut GpBrush, points : *const PointF, count : i32, tension : f32, fillmode : FillMode) -> Status);
    unsafe { GdipFillClosedCurve2(graphics as _, brush as _, points, count, tension, fillmode) }
}
#[inline]
pub unsafe fn GdipFillClosedCurve2I(graphics: *mut GpGraphics, brush: *mut GpBrush, points: *const Point, count: i32, tension: f32, fillmode: FillMode) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipFillClosedCurve2I(graphics : *mut GpGraphics, brush : *mut GpBrush, points : *const Point, count : i32, tension : f32, fillmode : FillMode) -> Status);
    unsafe { GdipFillClosedCurve2I(graphics as _, brush as _, points, count, tension, fillmode) }
}
#[inline]
pub unsafe fn GdipFillClosedCurveI(graphics: *mut GpGraphics, brush: *mut GpBrush, points: *const Point, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipFillClosedCurveI(graphics : *mut GpGraphics, brush : *mut GpBrush, points : *const Point, count : i32) -> Status);
    unsafe { GdipFillClosedCurveI(graphics as _, brush as _, points, count) }
}
#[inline]
pub unsafe fn GdipFillEllipse(graphics: *mut GpGraphics, brush: *mut GpBrush, x: f32, y: f32, width: f32, height: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipFillEllipse(graphics : *mut GpGraphics, brush : *mut GpBrush, x : f32, y : f32, width : f32, height : f32) -> Status);
    unsafe { GdipFillEllipse(graphics as _, brush as _, x, y, width, height) }
}
#[inline]
pub unsafe fn GdipFillEllipseI(graphics: *mut GpGraphics, brush: *mut GpBrush, x: i32, y: i32, width: i32, height: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipFillEllipseI(graphics : *mut GpGraphics, brush : *mut GpBrush, x : i32, y : i32, width : i32, height : i32) -> Status);
    unsafe { GdipFillEllipseI(graphics as _, brush as _, x, y, width, height) }
}
#[inline]
pub unsafe fn GdipFillPath(graphics: *mut GpGraphics, brush: *mut GpBrush, path: *mut GpPath) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipFillPath(graphics : *mut GpGraphics, brush : *mut GpBrush, path : *mut GpPath) -> Status);
    unsafe { GdipFillPath(graphics as _, brush as _, path as _) }
}
#[inline]
pub unsafe fn GdipFillPie(graphics: *mut GpGraphics, brush: *mut GpBrush, x: f32, y: f32, width: f32, height: f32, startangle: f32, sweepangle: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipFillPie(graphics : *mut GpGraphics, brush : *mut GpBrush, x : f32, y : f32, width : f32, height : f32, startangle : f32, sweepangle : f32) -> Status);
    unsafe { GdipFillPie(graphics as _, brush as _, x, y, width, height, startangle, sweepangle) }
}
#[inline]
pub unsafe fn GdipFillPieI(graphics: *mut GpGraphics, brush: *mut GpBrush, x: i32, y: i32, width: i32, height: i32, startangle: f32, sweepangle: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipFillPieI(graphics : *mut GpGraphics, brush : *mut GpBrush, x : i32, y : i32, width : i32, height : i32, startangle : f32, sweepangle : f32) -> Status);
    unsafe { GdipFillPieI(graphics as _, brush as _, x, y, width, height, startangle, sweepangle) }
}
#[inline]
pub unsafe fn GdipFillPolygon(graphics: *mut GpGraphics, brush: *mut GpBrush, points: *const PointF, count: i32, fillmode: FillMode) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipFillPolygon(graphics : *mut GpGraphics, brush : *mut GpBrush, points : *const PointF, count : i32, fillmode : FillMode) -> Status);
    unsafe { GdipFillPolygon(graphics as _, brush as _, points, count, fillmode) }
}
#[inline]
pub unsafe fn GdipFillPolygon2(graphics: *mut GpGraphics, brush: *mut GpBrush, points: *const PointF, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipFillPolygon2(graphics : *mut GpGraphics, brush : *mut GpBrush, points : *const PointF, count : i32) -> Status);
    unsafe { GdipFillPolygon2(graphics as _, brush as _, points, count) }
}
#[inline]
pub unsafe fn GdipFillPolygon2I(graphics: *mut GpGraphics, brush: *mut GpBrush, points: *const Point, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipFillPolygon2I(graphics : *mut GpGraphics, brush : *mut GpBrush, points : *const Point, count : i32) -> Status);
    unsafe { GdipFillPolygon2I(graphics as _, brush as _, points, count) }
}
#[inline]
pub unsafe fn GdipFillPolygonI(graphics: *mut GpGraphics, brush: *mut GpBrush, points: *const Point, count: i32, fillmode: FillMode) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipFillPolygonI(graphics : *mut GpGraphics, brush : *mut GpBrush, points : *const Point, count : i32, fillmode : FillMode) -> Status);
    unsafe { GdipFillPolygonI(graphics as _, brush as _, points, count, fillmode) }
}
#[inline]
pub unsafe fn GdipFillRectangle(graphics: *mut GpGraphics, brush: *mut GpBrush, x: f32, y: f32, width: f32, height: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipFillRectangle(graphics : *mut GpGraphics, brush : *mut GpBrush, x : f32, y : f32, width : f32, height : f32) -> Status);
    unsafe { GdipFillRectangle(graphics as _, brush as _, x, y, width, height) }
}
#[inline]
pub unsafe fn GdipFillRectangleI(graphics: *mut GpGraphics, brush: *mut GpBrush, x: i32, y: i32, width: i32, height: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipFillRectangleI(graphics : *mut GpGraphics, brush : *mut GpBrush, x : i32, y : i32, width : i32, height : i32) -> Status);
    unsafe { GdipFillRectangleI(graphics as _, brush as _, x, y, width, height) }
}
#[inline]
pub unsafe fn GdipFillRectangles(graphics: *mut GpGraphics, brush: *mut GpBrush, rects: *const RectF, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipFillRectangles(graphics : *mut GpGraphics, brush : *mut GpBrush, rects : *const RectF, count : i32) -> Status);
    unsafe { GdipFillRectangles(graphics as _, brush as _, rects, count) }
}
#[inline]
pub unsafe fn GdipFillRectanglesI(graphics: *mut GpGraphics, brush: *mut GpBrush, rects: *const Rect, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipFillRectanglesI(graphics : *mut GpGraphics, brush : *mut GpBrush, rects : *const Rect, count : i32) -> Status);
    unsafe { GdipFillRectanglesI(graphics as _, brush as _, rects, count) }
}
#[inline]
pub unsafe fn GdipFillRegion(graphics: *mut GpGraphics, brush: *mut GpBrush, region: *mut GpRegion) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipFillRegion(graphics : *mut GpGraphics, brush : *mut GpBrush, region : *mut GpRegion) -> Status);
    unsafe { GdipFillRegion(graphics as _, brush as _, region as _) }
}
#[inline]
pub unsafe fn GdipFindFirstImageItem(image: *mut GpImage, item: *mut ImageItemData) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipFindFirstImageItem(image : *mut GpImage, item : *mut ImageItemData) -> Status);
    unsafe { GdipFindFirstImageItem(image as _, item as _) }
}
#[inline]
pub unsafe fn GdipFindNextImageItem(image: *mut GpImage, item: *mut ImageItemData) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipFindNextImageItem(image : *mut GpImage, item : *mut ImageItemData) -> Status);
    unsafe { GdipFindNextImageItem(image as _, item as _) }
}
#[inline]
pub unsafe fn GdipFlattenPath(path: *mut GpPath, matrix: *mut Matrix, flatness: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipFlattenPath(path : *mut GpPath, matrix : *mut Matrix, flatness : f32) -> Status);
    unsafe { GdipFlattenPath(path as _, matrix as _, flatness) }
}
#[inline]
pub unsafe fn GdipFlush(graphics: *mut GpGraphics, intention: FlushIntention) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipFlush(graphics : *mut GpGraphics, intention : FlushIntention) -> Status);
    unsafe { GdipFlush(graphics as _, intention) }
}
#[inline]
pub unsafe fn GdipFree(ptr: *mut core::ffi::c_void) {
    windows_core::link!("gdiplus.dll" "system" fn GdipFree(ptr : *mut core::ffi::c_void));
    unsafe { GdipFree(ptr as _) }
}
#[inline]
pub unsafe fn GdipGetAdjustableArrowCapFillState(cap: *mut GpAdjustableArrowCap, fillstate: *mut windows_core::BOOL) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetAdjustableArrowCapFillState(cap : *mut GpAdjustableArrowCap, fillstate : *mut windows_core::BOOL) -> Status);
    unsafe { GdipGetAdjustableArrowCapFillState(cap as _, fillstate as _) }
}
#[inline]
pub unsafe fn GdipGetAdjustableArrowCapHeight(cap: *mut GpAdjustableArrowCap, height: *mut f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetAdjustableArrowCapHeight(cap : *mut GpAdjustableArrowCap, height : *mut f32) -> Status);
    unsafe { GdipGetAdjustableArrowCapHeight(cap as _, height as _) }
}
#[inline]
pub unsafe fn GdipGetAdjustableArrowCapMiddleInset(cap: *mut GpAdjustableArrowCap, middleinset: *mut f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetAdjustableArrowCapMiddleInset(cap : *mut GpAdjustableArrowCap, middleinset : *mut f32) -> Status);
    unsafe { GdipGetAdjustableArrowCapMiddleInset(cap as _, middleinset as _) }
}
#[inline]
pub unsafe fn GdipGetAdjustableArrowCapWidth(cap: *mut GpAdjustableArrowCap, width: *mut f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetAdjustableArrowCapWidth(cap : *mut GpAdjustableArrowCap, width : *mut f32) -> Status);
    unsafe { GdipGetAdjustableArrowCapWidth(cap as _, width as _) }
}
#[inline]
pub unsafe fn GdipGetAllPropertyItems(image: *mut GpImage, totalbuffersize: u32, numproperties: u32, allitems: *mut PropertyItem) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetAllPropertyItems(image : *mut GpImage, totalbuffersize : u32, numproperties : u32, allitems : *mut PropertyItem) -> Status);
    unsafe { GdipGetAllPropertyItems(image as _, totalbuffersize, numproperties, allitems as _) }
}
#[inline]
pub unsafe fn GdipGetBrushType(brush: *mut GpBrush, r#type: *mut BrushType) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetBrushType(brush : *mut GpBrush, r#type : *mut BrushType) -> Status);
    unsafe { GdipGetBrushType(brush as _, r#type as _) }
}
#[inline]
pub unsafe fn GdipGetCellAscent(family: *const GpFontFamily, style: i32, cellascent: *mut u16) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetCellAscent(family : *const GpFontFamily, style : i32, cellascent : *mut u16) -> Status);
    unsafe { GdipGetCellAscent(family, style, cellascent as _) }
}
#[inline]
pub unsafe fn GdipGetCellDescent(family: *const GpFontFamily, style: i32, celldescent: *mut u16) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetCellDescent(family : *const GpFontFamily, style : i32, celldescent : *mut u16) -> Status);
    unsafe { GdipGetCellDescent(family, style, celldescent as _) }
}
#[inline]
pub unsafe fn GdipGetClip(graphics: *mut GpGraphics, region: *mut GpRegion) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetClip(graphics : *mut GpGraphics, region : *mut GpRegion) -> Status);
    unsafe { GdipGetClip(graphics as _, region as _) }
}
#[inline]
pub unsafe fn GdipGetClipBounds(graphics: *mut GpGraphics, rect: *mut RectF) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetClipBounds(graphics : *mut GpGraphics, rect : *mut RectF) -> Status);
    unsafe { GdipGetClipBounds(graphics as _, rect as _) }
}
#[inline]
pub unsafe fn GdipGetClipBoundsI(graphics: *mut GpGraphics, rect: *mut Rect) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetClipBoundsI(graphics : *mut GpGraphics, rect : *mut Rect) -> Status);
    unsafe { GdipGetClipBoundsI(graphics as _, rect as _) }
}
#[inline]
pub unsafe fn GdipGetCompositingMode(graphics: *mut GpGraphics, compositingmode: *mut CompositingMode) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetCompositingMode(graphics : *mut GpGraphics, compositingmode : *mut CompositingMode) -> Status);
    unsafe { GdipGetCompositingMode(graphics as _, compositingmode as _) }
}
#[inline]
pub unsafe fn GdipGetCompositingQuality(graphics: *mut GpGraphics, compositingquality: *mut CompositingQuality) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetCompositingQuality(graphics : *mut GpGraphics, compositingquality : *mut CompositingQuality) -> Status);
    unsafe { GdipGetCompositingQuality(graphics as _, compositingquality as _) }
}
#[inline]
pub unsafe fn GdipGetCustomLineCapBaseCap(customcap: *mut GpCustomLineCap, basecap: *mut LineCap) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetCustomLineCapBaseCap(customcap : *mut GpCustomLineCap, basecap : *mut LineCap) -> Status);
    unsafe { GdipGetCustomLineCapBaseCap(customcap as _, basecap as _) }
}
#[inline]
pub unsafe fn GdipGetCustomLineCapBaseInset(customcap: *mut GpCustomLineCap, inset: *mut f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetCustomLineCapBaseInset(customcap : *mut GpCustomLineCap, inset : *mut f32) -> Status);
    unsafe { GdipGetCustomLineCapBaseInset(customcap as _, inset as _) }
}
#[inline]
pub unsafe fn GdipGetCustomLineCapStrokeCaps(customcap: *mut GpCustomLineCap, startcap: *mut LineCap, endcap: *mut LineCap) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetCustomLineCapStrokeCaps(customcap : *mut GpCustomLineCap, startcap : *mut LineCap, endcap : *mut LineCap) -> Status);
    unsafe { GdipGetCustomLineCapStrokeCaps(customcap as _, startcap as _, endcap as _) }
}
#[inline]
pub unsafe fn GdipGetCustomLineCapStrokeJoin(customcap: *mut GpCustomLineCap, linejoin: *mut LineJoin) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetCustomLineCapStrokeJoin(customcap : *mut GpCustomLineCap, linejoin : *mut LineJoin) -> Status);
    unsafe { GdipGetCustomLineCapStrokeJoin(customcap as _, linejoin as _) }
}
#[inline]
pub unsafe fn GdipGetCustomLineCapType(customcap: *mut GpCustomLineCap, captype: *mut CustomLineCapType) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetCustomLineCapType(customcap : *mut GpCustomLineCap, captype : *mut CustomLineCapType) -> Status);
    unsafe { GdipGetCustomLineCapType(customcap as _, captype as _) }
}
#[inline]
pub unsafe fn GdipGetCustomLineCapWidthScale(customcap: *mut GpCustomLineCap, widthscale: *mut f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetCustomLineCapWidthScale(customcap : *mut GpCustomLineCap, widthscale : *mut f32) -> Status);
    unsafe { GdipGetCustomLineCapWidthScale(customcap as _, widthscale as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipGetDC(graphics: *mut GpGraphics, hdc: *mut super::Gdi::HDC) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetDC(graphics : *mut GpGraphics, hdc : *mut super::Gdi:: HDC) -> Status);
    unsafe { GdipGetDC(graphics as _, hdc as _) }
}
#[inline]
pub unsafe fn GdipGetDpiX(graphics: *mut GpGraphics, dpi: *mut f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetDpiX(graphics : *mut GpGraphics, dpi : *mut f32) -> Status);
    unsafe { GdipGetDpiX(graphics as _, dpi as _) }
}
#[inline]
pub unsafe fn GdipGetDpiY(graphics: *mut GpGraphics, dpi: *mut f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetDpiY(graphics : *mut GpGraphics, dpi : *mut f32) -> Status);
    unsafe { GdipGetDpiY(graphics as _, dpi as _) }
}
#[inline]
pub unsafe fn GdipGetEffectParameterSize(effect: *mut CGpEffect, size: *mut u32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetEffectParameterSize(effect : *mut CGpEffect, size : *mut u32) -> Status);
    unsafe { GdipGetEffectParameterSize(effect as _, size as _) }
}
#[inline]
pub unsafe fn GdipGetEffectParameters(effect: *mut CGpEffect, size: *mut u32, params: *mut core::ffi::c_void) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetEffectParameters(effect : *mut CGpEffect, size : *mut u32, params : *mut core::ffi::c_void) -> Status);
    unsafe { GdipGetEffectParameters(effect as _, size as _, params as _) }
}
#[inline]
pub unsafe fn GdipGetEmHeight(family: *const GpFontFamily, style: i32, emheight: *mut u16) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetEmHeight(family : *const GpFontFamily, style : i32, emheight : *mut u16) -> Status);
    unsafe { GdipGetEmHeight(family, style, emheight as _) }
}
#[inline]
pub unsafe fn GdipGetEncoderParameterList(image: *mut GpImage, clsidencoder: *const windows_core::GUID, size: u32, buffer: *mut EncoderParameters) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetEncoderParameterList(image : *mut GpImage, clsidencoder : *const windows_core::GUID, size : u32, buffer : *mut EncoderParameters) -> Status);
    unsafe { GdipGetEncoderParameterList(image as _, clsidencoder, size, buffer as _) }
}
#[inline]
pub unsafe fn GdipGetEncoderParameterListSize(image: *mut GpImage, clsidencoder: *const windows_core::GUID, size: *mut u32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetEncoderParameterListSize(image : *mut GpImage, clsidencoder : *const windows_core::GUID, size : *mut u32) -> Status);
    unsafe { GdipGetEncoderParameterListSize(image as _, clsidencoder, size as _) }
}
#[inline]
pub unsafe fn GdipGetFamily(font: *mut GpFont, family: *mut *mut GpFontFamily) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetFamily(font : *mut GpFont, family : *mut *mut GpFontFamily) -> Status);
    unsafe { GdipGetFamily(font as _, family as _) }
}
#[inline]
pub unsafe fn GdipGetFamilyName(family: *const GpFontFamily, name: &mut [u16; 32], language: u16) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetFamilyName(family : *const GpFontFamily, name : windows_core::PWSTR, language : u16) -> Status);
    unsafe { GdipGetFamilyName(family, core::mem::transmute(name.as_ptr()), language) }
}
#[inline]
pub unsafe fn GdipGetFontCollectionFamilyCount(fontcollection: *mut GpFontCollection, numfound: *mut i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetFontCollectionFamilyCount(fontcollection : *mut GpFontCollection, numfound : *mut i32) -> Status);
    unsafe { GdipGetFontCollectionFamilyCount(fontcollection as _, numfound as _) }
}
#[inline]
pub unsafe fn GdipGetFontCollectionFamilyList(fontcollection: *const GpFontCollection, gpfamilies: &mut [*mut GpFontFamily], numfound: *mut i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetFontCollectionFamilyList(fontcollection : *const GpFontCollection, numsought : i32, gpfamilies : *mut *mut GpFontFamily, numfound : *mut i32) -> Status);
    unsafe { GdipGetFontCollectionFamilyList(fontcollection, gpfamilies.len().try_into().unwrap(), core::mem::transmute(gpfamilies.as_ptr()), numfound as _) }
}
#[inline]
pub unsafe fn GdipGetFontHeight(font: *const GpFont, graphics: *const GpGraphics, height: *mut f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetFontHeight(font : *const GpFont, graphics : *const GpGraphics, height : *mut f32) -> Status);
    unsafe { GdipGetFontHeight(font, graphics, height as _) }
}
#[inline]
pub unsafe fn GdipGetFontHeightGivenDPI(font: *const GpFont, dpi: f32, height: *mut f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetFontHeightGivenDPI(font : *const GpFont, dpi : f32, height : *mut f32) -> Status);
    unsafe { GdipGetFontHeightGivenDPI(font, dpi, height as _) }
}
#[inline]
pub unsafe fn GdipGetFontSize(font: *mut GpFont, size: *mut f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetFontSize(font : *mut GpFont, size : *mut f32) -> Status);
    unsafe { GdipGetFontSize(font as _, size as _) }
}
#[inline]
pub unsafe fn GdipGetFontStyle(font: *mut GpFont, style: *mut i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetFontStyle(font : *mut GpFont, style : *mut i32) -> Status);
    unsafe { GdipGetFontStyle(font as _, style as _) }
}
#[inline]
pub unsafe fn GdipGetFontUnit(font: *mut GpFont, unit: *mut Unit) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetFontUnit(font : *mut GpFont, unit : *mut Unit) -> Status);
    unsafe { GdipGetFontUnit(font as _, unit as _) }
}
#[inline]
pub unsafe fn GdipGetGenericFontFamilyMonospace(nativefamily: *mut *mut GpFontFamily) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetGenericFontFamilyMonospace(nativefamily : *mut *mut GpFontFamily) -> Status);
    unsafe { GdipGetGenericFontFamilyMonospace(nativefamily as _) }
}
#[inline]
pub unsafe fn GdipGetGenericFontFamilySansSerif(nativefamily: *mut *mut GpFontFamily) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetGenericFontFamilySansSerif(nativefamily : *mut *mut GpFontFamily) -> Status);
    unsafe { GdipGetGenericFontFamilySansSerif(nativefamily as _) }
}
#[inline]
pub unsafe fn GdipGetGenericFontFamilySerif(nativefamily: *mut *mut GpFontFamily) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetGenericFontFamilySerif(nativefamily : *mut *mut GpFontFamily) -> Status);
    unsafe { GdipGetGenericFontFamilySerif(nativefamily as _) }
}
#[inline]
pub unsafe fn GdipGetHatchBackgroundColor(brush: *mut GpHatch, backcol: *mut u32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetHatchBackgroundColor(brush : *mut GpHatch, backcol : *mut u32) -> Status);
    unsafe { GdipGetHatchBackgroundColor(brush as _, backcol as _) }
}
#[inline]
pub unsafe fn GdipGetHatchForegroundColor(brush: *mut GpHatch, forecol: *mut u32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetHatchForegroundColor(brush : *mut GpHatch, forecol : *mut u32) -> Status);
    unsafe { GdipGetHatchForegroundColor(brush as _, forecol as _) }
}
#[inline]
pub unsafe fn GdipGetHatchStyle(brush: *mut GpHatch, hatchstyle: *mut HatchStyle) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetHatchStyle(brush : *mut GpHatch, hatchstyle : *mut HatchStyle) -> Status);
    unsafe { GdipGetHatchStyle(brush as _, hatchstyle as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipGetHemfFromMetafile(metafile: *mut GpMetafile, hemf: *mut super::Gdi::HENHMETAFILE) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetHemfFromMetafile(metafile : *mut GpMetafile, hemf : *mut super::Gdi:: HENHMETAFILE) -> Status);
    unsafe { GdipGetHemfFromMetafile(metafile as _, hemf as _) }
}
#[inline]
pub unsafe fn GdipGetImageAttributesAdjustedPalette(imageattr: *mut GpImageAttributes, colorpalette: *mut ColorPalette, coloradjusttype: ColorAdjustType) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetImageAttributesAdjustedPalette(imageattr : *mut GpImageAttributes, colorpalette : *mut ColorPalette, coloradjusttype : ColorAdjustType) -> Status);
    unsafe { GdipGetImageAttributesAdjustedPalette(imageattr as _, colorpalette as _, coloradjusttype) }
}
#[inline]
pub unsafe fn GdipGetImageBounds(image: *mut GpImage, srcrect: *mut RectF, srcunit: *mut Unit) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetImageBounds(image : *mut GpImage, srcrect : *mut RectF, srcunit : *mut Unit) -> Status);
    unsafe { GdipGetImageBounds(image as _, srcrect as _, srcunit as _) }
}
#[inline]
pub unsafe fn GdipGetImageDecoders(numdecoders: u32, size: u32, decoders: *mut ImageCodecInfo) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetImageDecoders(numdecoders : u32, size : u32, decoders : *mut ImageCodecInfo) -> Status);
    unsafe { GdipGetImageDecoders(numdecoders, size, decoders as _) }
}
#[inline]
pub unsafe fn GdipGetImageDecodersSize(numdecoders: *mut u32, size: *mut u32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetImageDecodersSize(numdecoders : *mut u32, size : *mut u32) -> Status);
    unsafe { GdipGetImageDecodersSize(numdecoders as _, size as _) }
}
#[inline]
pub unsafe fn GdipGetImageDimension(image: *mut GpImage, width: *mut f32, height: *mut f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetImageDimension(image : *mut GpImage, width : *mut f32, height : *mut f32) -> Status);
    unsafe { GdipGetImageDimension(image as _, width as _, height as _) }
}
#[inline]
pub unsafe fn GdipGetImageEncoders(numencoders: u32, size: u32, encoders: *mut ImageCodecInfo) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetImageEncoders(numencoders : u32, size : u32, encoders : *mut ImageCodecInfo) -> Status);
    unsafe { GdipGetImageEncoders(numencoders, size, encoders as _) }
}
#[inline]
pub unsafe fn GdipGetImageEncodersSize(numencoders: *mut u32, size: *mut u32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetImageEncodersSize(numencoders : *mut u32, size : *mut u32) -> Status);
    unsafe { GdipGetImageEncodersSize(numencoders as _, size as _) }
}
#[inline]
pub unsafe fn GdipGetImageFlags(image: *mut GpImage, flags: *mut u32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetImageFlags(image : *mut GpImage, flags : *mut u32) -> Status);
    unsafe { GdipGetImageFlags(image as _, flags as _) }
}
#[inline]
pub unsafe fn GdipGetImageGraphicsContext(image: *mut GpImage, graphics: *mut *mut GpGraphics) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetImageGraphicsContext(image : *mut GpImage, graphics : *mut *mut GpGraphics) -> Status);
    unsafe { GdipGetImageGraphicsContext(image as _, graphics as _) }
}
#[inline]
pub unsafe fn GdipGetImageHeight(image: *mut GpImage, height: *mut u32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetImageHeight(image : *mut GpImage, height : *mut u32) -> Status);
    unsafe { GdipGetImageHeight(image as _, height as _) }
}
#[inline]
pub unsafe fn GdipGetImageHorizontalResolution(image: *mut GpImage, resolution: *mut f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetImageHorizontalResolution(image : *mut GpImage, resolution : *mut f32) -> Status);
    unsafe { GdipGetImageHorizontalResolution(image as _, resolution as _) }
}
#[inline]
pub unsafe fn GdipGetImageItemData(image: *mut GpImage, item: *mut ImageItemData) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetImageItemData(image : *mut GpImage, item : *mut ImageItemData) -> Status);
    unsafe { GdipGetImageItemData(image as _, item as _) }
}
#[inline]
pub unsafe fn GdipGetImagePalette(image: *mut GpImage, palette: *mut ColorPalette, size: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetImagePalette(image : *mut GpImage, palette : *mut ColorPalette, size : i32) -> Status);
    unsafe { GdipGetImagePalette(image as _, palette as _, size) }
}
#[inline]
pub unsafe fn GdipGetImagePaletteSize(image: *mut GpImage, size: *mut i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetImagePaletteSize(image : *mut GpImage, size : *mut i32) -> Status);
    unsafe { GdipGetImagePaletteSize(image as _, size as _) }
}
#[inline]
pub unsafe fn GdipGetImagePixelFormat(image: *mut GpImage, format: *mut i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetImagePixelFormat(image : *mut GpImage, format : *mut i32) -> Status);
    unsafe { GdipGetImagePixelFormat(image as _, format as _) }
}
#[inline]
pub unsafe fn GdipGetImageRawFormat(image: *mut GpImage, format: *mut windows_core::GUID) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetImageRawFormat(image : *mut GpImage, format : *mut windows_core::GUID) -> Status);
    unsafe { GdipGetImageRawFormat(image as _, format as _) }
}
#[inline]
pub unsafe fn GdipGetImageThumbnail(image: *mut GpImage, thumbwidth: u32, thumbheight: u32, thumbimage: *mut *mut GpImage, callback: isize, callbackdata: *mut core::ffi::c_void) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetImageThumbnail(image : *mut GpImage, thumbwidth : u32, thumbheight : u32, thumbimage : *mut *mut GpImage, callback : isize, callbackdata : *mut core::ffi::c_void) -> Status);
    unsafe { GdipGetImageThumbnail(image as _, thumbwidth, thumbheight, thumbimage as _, callback, callbackdata as _) }
}
#[inline]
pub unsafe fn GdipGetImageType(image: *mut GpImage, r#type: *mut ImageType) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetImageType(image : *mut GpImage, r#type : *mut ImageType) -> Status);
    unsafe { GdipGetImageType(image as _, r#type as _) }
}
#[inline]
pub unsafe fn GdipGetImageVerticalResolution(image: *mut GpImage, resolution: *mut f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetImageVerticalResolution(image : *mut GpImage, resolution : *mut f32) -> Status);
    unsafe { GdipGetImageVerticalResolution(image as _, resolution as _) }
}
#[inline]
pub unsafe fn GdipGetImageWidth(image: *mut GpImage, width: *mut u32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetImageWidth(image : *mut GpImage, width : *mut u32) -> Status);
    unsafe { GdipGetImageWidth(image as _, width as _) }
}
#[inline]
pub unsafe fn GdipGetInterpolationMode(graphics: *mut GpGraphics, interpolationmode: *mut InterpolationMode) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetInterpolationMode(graphics : *mut GpGraphics, interpolationmode : *mut InterpolationMode) -> Status);
    unsafe { GdipGetInterpolationMode(graphics as _, interpolationmode as _) }
}
#[inline]
pub unsafe fn GdipGetLineBlend(brush: *mut GpLineGradient, blend: *mut f32, positions: *mut f32, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetLineBlend(brush : *mut GpLineGradient, blend : *mut f32, positions : *mut f32, count : i32) -> Status);
    unsafe { GdipGetLineBlend(brush as _, blend as _, positions as _, count) }
}
#[inline]
pub unsafe fn GdipGetLineBlendCount(brush: *mut GpLineGradient, count: *mut i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetLineBlendCount(brush : *mut GpLineGradient, count : *mut i32) -> Status);
    unsafe { GdipGetLineBlendCount(brush as _, count as _) }
}
#[inline]
pub unsafe fn GdipGetLineColors(brush: *mut GpLineGradient, colors: *mut u32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetLineColors(brush : *mut GpLineGradient, colors : *mut u32) -> Status);
    unsafe { GdipGetLineColors(brush as _, colors as _) }
}
#[inline]
pub unsafe fn GdipGetLineGammaCorrection(brush: *mut GpLineGradient, usegammacorrection: *mut windows_core::BOOL) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetLineGammaCorrection(brush : *mut GpLineGradient, usegammacorrection : *mut windows_core::BOOL) -> Status);
    unsafe { GdipGetLineGammaCorrection(brush as _, usegammacorrection as _) }
}
#[inline]
pub unsafe fn GdipGetLinePresetBlend(brush: *mut GpLineGradient, blend: *mut u32, positions: *mut f32, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetLinePresetBlend(brush : *mut GpLineGradient, blend : *mut u32, positions : *mut f32, count : i32) -> Status);
    unsafe { GdipGetLinePresetBlend(brush as _, blend as _, positions as _, count) }
}
#[inline]
pub unsafe fn GdipGetLinePresetBlendCount(brush: *mut GpLineGradient, count: *mut i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetLinePresetBlendCount(brush : *mut GpLineGradient, count : *mut i32) -> Status);
    unsafe { GdipGetLinePresetBlendCount(brush as _, count as _) }
}
#[inline]
pub unsafe fn GdipGetLineRect(brush: *mut GpLineGradient, rect: *mut RectF) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetLineRect(brush : *mut GpLineGradient, rect : *mut RectF) -> Status);
    unsafe { GdipGetLineRect(brush as _, rect as _) }
}
#[inline]
pub unsafe fn GdipGetLineRectI(brush: *mut GpLineGradient, rect: *mut Rect) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetLineRectI(brush : *mut GpLineGradient, rect : *mut Rect) -> Status);
    unsafe { GdipGetLineRectI(brush as _, rect as _) }
}
#[inline]
pub unsafe fn GdipGetLineSpacing(family: *const GpFontFamily, style: i32, linespacing: *mut u16) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetLineSpacing(family : *const GpFontFamily, style : i32, linespacing : *mut u16) -> Status);
    unsafe { GdipGetLineSpacing(family, style, linespacing as _) }
}
#[inline]
pub unsafe fn GdipGetLineTransform(brush: *mut GpLineGradient, matrix: *mut Matrix) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetLineTransform(brush : *mut GpLineGradient, matrix : *mut Matrix) -> Status);
    unsafe { GdipGetLineTransform(brush as _, matrix as _) }
}
#[inline]
pub unsafe fn GdipGetLineWrapMode(brush: *mut GpLineGradient, wrapmode: *mut WrapMode) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetLineWrapMode(brush : *mut GpLineGradient, wrapmode : *mut WrapMode) -> Status);
    unsafe { GdipGetLineWrapMode(brush as _, wrapmode as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipGetLogFontA(font: *mut GpFont, graphics: *mut GpGraphics, logfonta: *mut super::Gdi::LOGFONTA) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetLogFontA(font : *mut GpFont, graphics : *mut GpGraphics, logfonta : *mut super::Gdi:: LOGFONTA) -> Status);
    unsafe { GdipGetLogFontA(font as _, graphics as _, logfonta as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipGetLogFontW(font: *mut GpFont, graphics: *mut GpGraphics, logfontw: *mut super::Gdi::LOGFONTW) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetLogFontW(font : *mut GpFont, graphics : *mut GpGraphics, logfontw : *mut super::Gdi:: LOGFONTW) -> Status);
    unsafe { GdipGetLogFontW(font as _, graphics as _, logfontw as _) }
}
#[inline]
pub unsafe fn GdipGetMatrixElements(matrix: *const Matrix, matrixout: *mut f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetMatrixElements(matrix : *const Matrix, matrixout : *mut f32) -> Status);
    unsafe { GdipGetMatrixElements(matrix, matrixout as _) }
}
#[inline]
pub unsafe fn GdipGetMetafileDownLevelRasterizationLimit(metafile: *const GpMetafile, metafilerasterizationlimitdpi: *mut u32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetMetafileDownLevelRasterizationLimit(metafile : *const GpMetafile, metafilerasterizationlimitdpi : *mut u32) -> Status);
    unsafe { GdipGetMetafileDownLevelRasterizationLimit(metafile, metafilerasterizationlimitdpi as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipGetMetafileHeaderFromEmf(hemf: super::Gdi::HENHMETAFILE, header: *mut MetafileHeader) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetMetafileHeaderFromEmf(hemf : super::Gdi:: HENHMETAFILE, header : *mut MetafileHeader) -> Status);
    unsafe { GdipGetMetafileHeaderFromEmf(hemf, header as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipGetMetafileHeaderFromFile<P0>(filename: P0, header: *mut MetafileHeader) -> Status
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("gdiplus.dll" "system" fn GdipGetMetafileHeaderFromFile(filename : windows_core::PCWSTR, header : *mut MetafileHeader) -> Status);
    unsafe { GdipGetMetafileHeaderFromFile(filename.param().abi(), header as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipGetMetafileHeaderFromMetafile(metafile: *mut GpMetafile, header: *mut MetafileHeader) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetMetafileHeaderFromMetafile(metafile : *mut GpMetafile, header : *mut MetafileHeader) -> Status);
    unsafe { GdipGetMetafileHeaderFromMetafile(metafile as _, header as _) }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn GdipGetMetafileHeaderFromStream<P0>(stream: P0, header: *mut MetafileHeader) -> Status
where
    P0: windows_core::Param<super::super::System::Com::IStream>,
{
    windows_core::link!("gdiplus.dll" "system" fn GdipGetMetafileHeaderFromStream(stream : * mut core::ffi::c_void, header : *mut MetafileHeader) -> Status);
    unsafe { GdipGetMetafileHeaderFromStream(stream.param().abi(), header as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipGetMetafileHeaderFromWmf(hwmf: super::Gdi::HMETAFILE, wmfplaceablefileheader: *const WmfPlaceableFileHeader, header: *mut MetafileHeader) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetMetafileHeaderFromWmf(hwmf : super::Gdi:: HMETAFILE, wmfplaceablefileheader : *const WmfPlaceableFileHeader, header : *mut MetafileHeader) -> Status);
    unsafe { GdipGetMetafileHeaderFromWmf(hwmf, wmfplaceablefileheader, header as _) }
}
#[inline]
pub unsafe fn GdipGetNearestColor(graphics: *mut GpGraphics, argb: *mut u32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetNearestColor(graphics : *mut GpGraphics, argb : *mut u32) -> Status);
    unsafe { GdipGetNearestColor(graphics as _, argb as _) }
}
#[inline]
pub unsafe fn GdipGetPageScale(graphics: *mut GpGraphics, scale: *mut f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPageScale(graphics : *mut GpGraphics, scale : *mut f32) -> Status);
    unsafe { GdipGetPageScale(graphics as _, scale as _) }
}
#[inline]
pub unsafe fn GdipGetPageUnit(graphics: *mut GpGraphics, unit: *mut Unit) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPageUnit(graphics : *mut GpGraphics, unit : *mut Unit) -> Status);
    unsafe { GdipGetPageUnit(graphics as _, unit as _) }
}
#[inline]
pub unsafe fn GdipGetPathData(path: *mut GpPath, pathdata: *mut PathData) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPathData(path : *mut GpPath, pathdata : *mut PathData) -> Status);
    unsafe { GdipGetPathData(path as _, pathdata as _) }
}
#[inline]
pub unsafe fn GdipGetPathFillMode(path: *mut GpPath, fillmode: *mut FillMode) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPathFillMode(path : *mut GpPath, fillmode : *mut FillMode) -> Status);
    unsafe { GdipGetPathFillMode(path as _, fillmode as _) }
}
#[inline]
pub unsafe fn GdipGetPathGradientBlend(brush: *mut GpPathGradient, blend: *mut f32, positions: *mut f32, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPathGradientBlend(brush : *mut GpPathGradient, blend : *mut f32, positions : *mut f32, count : i32) -> Status);
    unsafe { GdipGetPathGradientBlend(brush as _, blend as _, positions as _, count) }
}
#[inline]
pub unsafe fn GdipGetPathGradientBlendCount(brush: *mut GpPathGradient, count: *mut i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPathGradientBlendCount(brush : *mut GpPathGradient, count : *mut i32) -> Status);
    unsafe { GdipGetPathGradientBlendCount(brush as _, count as _) }
}
#[inline]
pub unsafe fn GdipGetPathGradientCenterColor(brush: *mut GpPathGradient, colors: *mut u32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPathGradientCenterColor(brush : *mut GpPathGradient, colors : *mut u32) -> Status);
    unsafe { GdipGetPathGradientCenterColor(brush as _, colors as _) }
}
#[inline]
pub unsafe fn GdipGetPathGradientCenterPoint(brush: *mut GpPathGradient, points: *mut PointF) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPathGradientCenterPoint(brush : *mut GpPathGradient, points : *mut PointF) -> Status);
    unsafe { GdipGetPathGradientCenterPoint(brush as _, points as _) }
}
#[inline]
pub unsafe fn GdipGetPathGradientCenterPointI(brush: *mut GpPathGradient, points: *mut Point) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPathGradientCenterPointI(brush : *mut GpPathGradient, points : *mut Point) -> Status);
    unsafe { GdipGetPathGradientCenterPointI(brush as _, points as _) }
}
#[inline]
pub unsafe fn GdipGetPathGradientFocusScales(brush: *mut GpPathGradient, xscale: *mut f32, yscale: *mut f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPathGradientFocusScales(brush : *mut GpPathGradient, xscale : *mut f32, yscale : *mut f32) -> Status);
    unsafe { GdipGetPathGradientFocusScales(brush as _, xscale as _, yscale as _) }
}
#[inline]
pub unsafe fn GdipGetPathGradientGammaCorrection(brush: *mut GpPathGradient, usegammacorrection: *mut windows_core::BOOL) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPathGradientGammaCorrection(brush : *mut GpPathGradient, usegammacorrection : *mut windows_core::BOOL) -> Status);
    unsafe { GdipGetPathGradientGammaCorrection(brush as _, usegammacorrection as _) }
}
#[inline]
pub unsafe fn GdipGetPathGradientPath(brush: *mut GpPathGradient, path: *mut GpPath) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPathGradientPath(brush : *mut GpPathGradient, path : *mut GpPath) -> Status);
    unsafe { GdipGetPathGradientPath(brush as _, path as _) }
}
#[inline]
pub unsafe fn GdipGetPathGradientPointCount(brush: *mut GpPathGradient, count: *mut i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPathGradientPointCount(brush : *mut GpPathGradient, count : *mut i32) -> Status);
    unsafe { GdipGetPathGradientPointCount(brush as _, count as _) }
}
#[inline]
pub unsafe fn GdipGetPathGradientPresetBlend(brush: *mut GpPathGradient, blend: *mut u32, positions: *mut f32, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPathGradientPresetBlend(brush : *mut GpPathGradient, blend : *mut u32, positions : *mut f32, count : i32) -> Status);
    unsafe { GdipGetPathGradientPresetBlend(brush as _, blend as _, positions as _, count) }
}
#[inline]
pub unsafe fn GdipGetPathGradientPresetBlendCount(brush: *mut GpPathGradient, count: *mut i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPathGradientPresetBlendCount(brush : *mut GpPathGradient, count : *mut i32) -> Status);
    unsafe { GdipGetPathGradientPresetBlendCount(brush as _, count as _) }
}
#[inline]
pub unsafe fn GdipGetPathGradientRect(brush: *mut GpPathGradient, rect: *mut RectF) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPathGradientRect(brush : *mut GpPathGradient, rect : *mut RectF) -> Status);
    unsafe { GdipGetPathGradientRect(brush as _, rect as _) }
}
#[inline]
pub unsafe fn GdipGetPathGradientRectI(brush: *mut GpPathGradient, rect: *mut Rect) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPathGradientRectI(brush : *mut GpPathGradient, rect : *mut Rect) -> Status);
    unsafe { GdipGetPathGradientRectI(brush as _, rect as _) }
}
#[inline]
pub unsafe fn GdipGetPathGradientSurroundColorCount(brush: *mut GpPathGradient, count: *mut i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPathGradientSurroundColorCount(brush : *mut GpPathGradient, count : *mut i32) -> Status);
    unsafe { GdipGetPathGradientSurroundColorCount(brush as _, count as _) }
}
#[inline]
pub unsafe fn GdipGetPathGradientSurroundColorsWithCount(brush: *const GpPathGradient, color: *mut u32, count: *mut i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPathGradientSurroundColorsWithCount(brush : *const GpPathGradient, color : *mut u32, count : *mut i32) -> Status);
    unsafe { GdipGetPathGradientSurroundColorsWithCount(brush, color as _, count as _) }
}
#[inline]
pub unsafe fn GdipGetPathGradientTransform(brush: *mut GpPathGradient, matrix: *mut Matrix) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPathGradientTransform(brush : *mut GpPathGradient, matrix : *mut Matrix) -> Status);
    unsafe { GdipGetPathGradientTransform(brush as _, matrix as _) }
}
#[inline]
pub unsafe fn GdipGetPathGradientWrapMode(brush: *mut GpPathGradient, wrapmode: *mut WrapMode) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPathGradientWrapMode(brush : *mut GpPathGradient, wrapmode : *mut WrapMode) -> Status);
    unsafe { GdipGetPathGradientWrapMode(brush as _, wrapmode as _) }
}
#[inline]
pub unsafe fn GdipGetPathLastPoint(path: *mut GpPath, lastpoint: *mut PointF) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPathLastPoint(path : *mut GpPath, lastpoint : *mut PointF) -> Status);
    unsafe { GdipGetPathLastPoint(path as _, lastpoint as _) }
}
#[inline]
pub unsafe fn GdipGetPathPoints(param0: *mut GpPath, points: *mut PointF, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPathPoints(param0 : *mut GpPath, points : *mut PointF, count : i32) -> Status);
    unsafe { GdipGetPathPoints(param0 as _, points as _, count) }
}
#[inline]
pub unsafe fn GdipGetPathPointsI(param0: *mut GpPath, points: *mut Point, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPathPointsI(param0 : *mut GpPath, points : *mut Point, count : i32) -> Status);
    unsafe { GdipGetPathPointsI(param0 as _, points as _, count) }
}
#[inline]
pub unsafe fn GdipGetPathTypes(path: *const GpPath, types: &mut [u8]) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPathTypes(path : *const GpPath, types : *mut u8, count : i32) -> Status);
    unsafe { GdipGetPathTypes(path, core::mem::transmute(types.as_ptr()), types.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn GdipGetPathWorldBounds(path: *mut GpPath, bounds: *mut RectF, matrix: *const Matrix, pen: *const GpPen) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPathWorldBounds(path : *mut GpPath, bounds : *mut RectF, matrix : *const Matrix, pen : *const GpPen) -> Status);
    unsafe { GdipGetPathWorldBounds(path as _, bounds as _, matrix, pen) }
}
#[inline]
pub unsafe fn GdipGetPathWorldBoundsI(path: *mut GpPath, bounds: *mut Rect, matrix: *const Matrix, pen: *const GpPen) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPathWorldBoundsI(path : *mut GpPath, bounds : *mut Rect, matrix : *const Matrix, pen : *const GpPen) -> Status);
    unsafe { GdipGetPathWorldBoundsI(path as _, bounds as _, matrix, pen) }
}
#[inline]
pub unsafe fn GdipGetPenBrushFill(pen: *mut GpPen, brush: *mut *mut GpBrush) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPenBrushFill(pen : *mut GpPen, brush : *mut *mut GpBrush) -> Status);
    unsafe { GdipGetPenBrushFill(pen as _, brush as _) }
}
#[inline]
pub unsafe fn GdipGetPenColor(pen: *mut GpPen, argb: *mut u32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPenColor(pen : *mut GpPen, argb : *mut u32) -> Status);
    unsafe { GdipGetPenColor(pen as _, argb as _) }
}
#[inline]
pub unsafe fn GdipGetPenCompoundArray(pen: *mut GpPen, dash: *mut f32, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPenCompoundArray(pen : *mut GpPen, dash : *mut f32, count : i32) -> Status);
    unsafe { GdipGetPenCompoundArray(pen as _, dash as _, count) }
}
#[inline]
pub unsafe fn GdipGetPenCompoundCount(pen: *mut GpPen, count: *mut i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPenCompoundCount(pen : *mut GpPen, count : *mut i32) -> Status);
    unsafe { GdipGetPenCompoundCount(pen as _, count as _) }
}
#[inline]
pub unsafe fn GdipGetPenCustomEndCap(pen: *mut GpPen, customcap: *mut *mut GpCustomLineCap) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPenCustomEndCap(pen : *mut GpPen, customcap : *mut *mut GpCustomLineCap) -> Status);
    unsafe { GdipGetPenCustomEndCap(pen as _, customcap as _) }
}
#[inline]
pub unsafe fn GdipGetPenCustomStartCap(pen: *mut GpPen, customcap: *mut *mut GpCustomLineCap) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPenCustomStartCap(pen : *mut GpPen, customcap : *mut *mut GpCustomLineCap) -> Status);
    unsafe { GdipGetPenCustomStartCap(pen as _, customcap as _) }
}
#[inline]
pub unsafe fn GdipGetPenDashArray(pen: *mut GpPen, dash: *mut f32, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPenDashArray(pen : *mut GpPen, dash : *mut f32, count : i32) -> Status);
    unsafe { GdipGetPenDashArray(pen as _, dash as _, count) }
}
#[inline]
pub unsafe fn GdipGetPenDashCap197819(pen: *mut GpPen, dashcap: *mut DashCap) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPenDashCap197819(pen : *mut GpPen, dashcap : *mut DashCap) -> Status);
    unsafe { GdipGetPenDashCap197819(pen as _, dashcap as _) }
}
#[inline]
pub unsafe fn GdipGetPenDashCount(pen: *mut GpPen, count: *mut i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPenDashCount(pen : *mut GpPen, count : *mut i32) -> Status);
    unsafe { GdipGetPenDashCount(pen as _, count as _) }
}
#[inline]
pub unsafe fn GdipGetPenDashOffset(pen: *mut GpPen, offset: *mut f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPenDashOffset(pen : *mut GpPen, offset : *mut f32) -> Status);
    unsafe { GdipGetPenDashOffset(pen as _, offset as _) }
}
#[inline]
pub unsafe fn GdipGetPenDashStyle(pen: *mut GpPen, dashstyle: *mut DashStyle) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPenDashStyle(pen : *mut GpPen, dashstyle : *mut DashStyle) -> Status);
    unsafe { GdipGetPenDashStyle(pen as _, dashstyle as _) }
}
#[inline]
pub unsafe fn GdipGetPenEndCap(pen: *mut GpPen, endcap: *mut LineCap) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPenEndCap(pen : *mut GpPen, endcap : *mut LineCap) -> Status);
    unsafe { GdipGetPenEndCap(pen as _, endcap as _) }
}
#[inline]
pub unsafe fn GdipGetPenFillType(pen: *mut GpPen, r#type: *mut PenType) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPenFillType(pen : *mut GpPen, r#type : *mut PenType) -> Status);
    unsafe { GdipGetPenFillType(pen as _, r#type as _) }
}
#[inline]
pub unsafe fn GdipGetPenLineJoin(pen: *mut GpPen, linejoin: *mut LineJoin) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPenLineJoin(pen : *mut GpPen, linejoin : *mut LineJoin) -> Status);
    unsafe { GdipGetPenLineJoin(pen as _, linejoin as _) }
}
#[inline]
pub unsafe fn GdipGetPenMiterLimit(pen: *mut GpPen, miterlimit: *mut f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPenMiterLimit(pen : *mut GpPen, miterlimit : *mut f32) -> Status);
    unsafe { GdipGetPenMiterLimit(pen as _, miterlimit as _) }
}
#[inline]
pub unsafe fn GdipGetPenMode(pen: *mut GpPen, penmode: *mut PenAlignment) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPenMode(pen : *mut GpPen, penmode : *mut PenAlignment) -> Status);
    unsafe { GdipGetPenMode(pen as _, penmode as _) }
}
#[inline]
pub unsafe fn GdipGetPenStartCap(pen: *mut GpPen, startcap: *mut LineCap) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPenStartCap(pen : *mut GpPen, startcap : *mut LineCap) -> Status);
    unsafe { GdipGetPenStartCap(pen as _, startcap as _) }
}
#[inline]
pub unsafe fn GdipGetPenTransform(pen: *mut GpPen, matrix: *mut Matrix) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPenTransform(pen : *mut GpPen, matrix : *mut Matrix) -> Status);
    unsafe { GdipGetPenTransform(pen as _, matrix as _) }
}
#[inline]
pub unsafe fn GdipGetPenUnit(pen: *mut GpPen, unit: *mut Unit) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPenUnit(pen : *mut GpPen, unit : *mut Unit) -> Status);
    unsafe { GdipGetPenUnit(pen as _, unit as _) }
}
#[inline]
pub unsafe fn GdipGetPenWidth(pen: *mut GpPen, width: *mut f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPenWidth(pen : *mut GpPen, width : *mut f32) -> Status);
    unsafe { GdipGetPenWidth(pen as _, width as _) }
}
#[inline]
pub unsafe fn GdipGetPixelOffsetMode(graphics: *mut GpGraphics, pixeloffsetmode: *mut PixelOffsetMode) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPixelOffsetMode(graphics : *mut GpGraphics, pixeloffsetmode : *mut PixelOffsetMode) -> Status);
    unsafe { GdipGetPixelOffsetMode(graphics as _, pixeloffsetmode as _) }
}
#[inline]
pub unsafe fn GdipGetPointCount(path: *mut GpPath, count: *mut i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPointCount(path : *mut GpPath, count : *mut i32) -> Status);
    unsafe { GdipGetPointCount(path as _, count as _) }
}
#[inline]
pub unsafe fn GdipGetPropertyCount(image: *mut GpImage, numofproperty: *mut u32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPropertyCount(image : *mut GpImage, numofproperty : *mut u32) -> Status);
    unsafe { GdipGetPropertyCount(image as _, numofproperty as _) }
}
#[inline]
pub unsafe fn GdipGetPropertyIdList(image: *mut GpImage, numofproperty: u32, list: *mut u32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPropertyIdList(image : *mut GpImage, numofproperty : u32, list : *mut u32) -> Status);
    unsafe { GdipGetPropertyIdList(image as _, numofproperty, list as _) }
}
#[inline]
pub unsafe fn GdipGetPropertyItem(image: *mut GpImage, propid: u32, propsize: u32, buffer: *mut PropertyItem) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPropertyItem(image : *mut GpImage, propid : u32, propsize : u32, buffer : *mut PropertyItem) -> Status);
    unsafe { GdipGetPropertyItem(image as _, propid, propsize, buffer as _) }
}
#[inline]
pub unsafe fn GdipGetPropertyItemSize(image: *mut GpImage, propid: u32, size: *mut u32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPropertyItemSize(image : *mut GpImage, propid : u32, size : *mut u32) -> Status);
    unsafe { GdipGetPropertyItemSize(image as _, propid, size as _) }
}
#[inline]
pub unsafe fn GdipGetPropertySize(image: *mut GpImage, totalbuffersize: *mut u32, numproperties: *mut u32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetPropertySize(image : *mut GpImage, totalbuffersize : *mut u32, numproperties : *mut u32) -> Status);
    unsafe { GdipGetPropertySize(image as _, totalbuffersize as _, numproperties as _) }
}
#[inline]
pub unsafe fn GdipGetRegionBounds(region: *mut GpRegion, graphics: *mut GpGraphics, rect: *mut RectF) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetRegionBounds(region : *mut GpRegion, graphics : *mut GpGraphics, rect : *mut RectF) -> Status);
    unsafe { GdipGetRegionBounds(region as _, graphics as _, rect as _) }
}
#[inline]
pub unsafe fn GdipGetRegionBoundsI(region: *mut GpRegion, graphics: *mut GpGraphics, rect: *mut Rect) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetRegionBoundsI(region : *mut GpRegion, graphics : *mut GpGraphics, rect : *mut Rect) -> Status);
    unsafe { GdipGetRegionBoundsI(region as _, graphics as _, rect as _) }
}
#[inline]
pub unsafe fn GdipGetRegionData(region: *mut GpRegion, buffer: &mut [u8], sizefilled: Option<*mut u32>) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetRegionData(region : *mut GpRegion, buffer : *mut u8, buffersize : u32, sizefilled : *mut u32) -> Status);
    unsafe { GdipGetRegionData(region as _, core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap(), sizefilled.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GdipGetRegionDataSize(region: *mut GpRegion, buffersize: *mut u32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetRegionDataSize(region : *mut GpRegion, buffersize : *mut u32) -> Status);
    unsafe { GdipGetRegionDataSize(region as _, buffersize as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipGetRegionHRgn(region: *mut GpRegion, graphics: *mut GpGraphics, hrgn: *mut super::Gdi::HRGN) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetRegionHRgn(region : *mut GpRegion, graphics : *mut GpGraphics, hrgn : *mut super::Gdi:: HRGN) -> Status);
    unsafe { GdipGetRegionHRgn(region as _, graphics as _, hrgn as _) }
}
#[inline]
pub unsafe fn GdipGetRegionScans(region: *mut GpRegion, rects: *mut RectF, count: *mut i32, matrix: *mut Matrix) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetRegionScans(region : *mut GpRegion, rects : *mut RectF, count : *mut i32, matrix : *mut Matrix) -> Status);
    unsafe { GdipGetRegionScans(region as _, rects as _, count as _, matrix as _) }
}
#[inline]
pub unsafe fn GdipGetRegionScansCount(region: *mut GpRegion, count: *mut u32, matrix: *mut Matrix) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetRegionScansCount(region : *mut GpRegion, count : *mut u32, matrix : *mut Matrix) -> Status);
    unsafe { GdipGetRegionScansCount(region as _, count as _, matrix as _) }
}
#[inline]
pub unsafe fn GdipGetRegionScansI(region: *mut GpRegion, rects: *mut Rect, count: *mut i32, matrix: *mut Matrix) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetRegionScansI(region : *mut GpRegion, rects : *mut Rect, count : *mut i32, matrix : *mut Matrix) -> Status);
    unsafe { GdipGetRegionScansI(region as _, rects as _, count as _, matrix as _) }
}
#[inline]
pub unsafe fn GdipGetRenderingOrigin(graphics: *mut GpGraphics, x: *mut i32, y: *mut i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetRenderingOrigin(graphics : *mut GpGraphics, x : *mut i32, y : *mut i32) -> Status);
    unsafe { GdipGetRenderingOrigin(graphics as _, x as _, y as _) }
}
#[inline]
pub unsafe fn GdipGetSmoothingMode(graphics: *mut GpGraphics, smoothingmode: *mut SmoothingMode) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetSmoothingMode(graphics : *mut GpGraphics, smoothingmode : *mut SmoothingMode) -> Status);
    unsafe { GdipGetSmoothingMode(graphics as _, smoothingmode as _) }
}
#[inline]
pub unsafe fn GdipGetSolidFillColor(brush: *mut GpSolidFill, color: *mut u32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetSolidFillColor(brush : *mut GpSolidFill, color : *mut u32) -> Status);
    unsafe { GdipGetSolidFillColor(brush as _, color as _) }
}
#[inline]
pub unsafe fn GdipGetStringFormatAlign(format: *const GpStringFormat, align: *mut StringAlignment) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetStringFormatAlign(format : *const GpStringFormat, align : *mut StringAlignment) -> Status);
    unsafe { GdipGetStringFormatAlign(format, align as _) }
}
#[inline]
pub unsafe fn GdipGetStringFormatDigitSubstitution(format: *const GpStringFormat, language: *mut u16, substitute: *mut StringDigitSubstitute) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetStringFormatDigitSubstitution(format : *const GpStringFormat, language : *mut u16, substitute : *mut StringDigitSubstitute) -> Status);
    unsafe { GdipGetStringFormatDigitSubstitution(format, language as _, substitute as _) }
}
#[inline]
pub unsafe fn GdipGetStringFormatFlags(format: *const GpStringFormat, flags: *mut i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetStringFormatFlags(format : *const GpStringFormat, flags : *mut i32) -> Status);
    unsafe { GdipGetStringFormatFlags(format, flags as _) }
}
#[inline]
pub unsafe fn GdipGetStringFormatHotkeyPrefix(format: *const GpStringFormat, hotkeyprefix: *mut i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetStringFormatHotkeyPrefix(format : *const GpStringFormat, hotkeyprefix : *mut i32) -> Status);
    unsafe { GdipGetStringFormatHotkeyPrefix(format, hotkeyprefix as _) }
}
#[inline]
pub unsafe fn GdipGetStringFormatLineAlign(format: *const GpStringFormat, align: *mut StringAlignment) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetStringFormatLineAlign(format : *const GpStringFormat, align : *mut StringAlignment) -> Status);
    unsafe { GdipGetStringFormatLineAlign(format, align as _) }
}
#[inline]
pub unsafe fn GdipGetStringFormatMeasurableCharacterRangeCount(format: *const GpStringFormat, count: *mut i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetStringFormatMeasurableCharacterRangeCount(format : *const GpStringFormat, count : *mut i32) -> Status);
    unsafe { GdipGetStringFormatMeasurableCharacterRangeCount(format, count as _) }
}
#[inline]
pub unsafe fn GdipGetStringFormatTabStopCount(format: *const GpStringFormat, count: *mut i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetStringFormatTabStopCount(format : *const GpStringFormat, count : *mut i32) -> Status);
    unsafe { GdipGetStringFormatTabStopCount(format, count as _) }
}
#[inline]
pub unsafe fn GdipGetStringFormatTabStops(format: *const GpStringFormat, count: i32, firsttaboffset: *mut f32, tabstops: *mut f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetStringFormatTabStops(format : *const GpStringFormat, count : i32, firsttaboffset : *mut f32, tabstops : *mut f32) -> Status);
    unsafe { GdipGetStringFormatTabStops(format, count, firsttaboffset as _, tabstops as _) }
}
#[inline]
pub unsafe fn GdipGetStringFormatTrimming(format: *const GpStringFormat, trimming: *mut StringTrimming) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetStringFormatTrimming(format : *const GpStringFormat, trimming : *mut StringTrimming) -> Status);
    unsafe { GdipGetStringFormatTrimming(format, trimming as _) }
}
#[inline]
pub unsafe fn GdipGetTextContrast(graphics: *mut GpGraphics, contrast: *mut u32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetTextContrast(graphics : *mut GpGraphics, contrast : *mut u32) -> Status);
    unsafe { GdipGetTextContrast(graphics as _, contrast as _) }
}
#[inline]
pub unsafe fn GdipGetTextRenderingHint(graphics: *mut GpGraphics, mode: *mut TextRenderingHint) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetTextRenderingHint(graphics : *mut GpGraphics, mode : *mut TextRenderingHint) -> Status);
    unsafe { GdipGetTextRenderingHint(graphics as _, mode as _) }
}
#[inline]
pub unsafe fn GdipGetTextureImage(brush: *mut GpTexture, image: *mut *mut GpImage) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetTextureImage(brush : *mut GpTexture, image : *mut *mut GpImage) -> Status);
    unsafe { GdipGetTextureImage(brush as _, image as _) }
}
#[inline]
pub unsafe fn GdipGetTextureTransform(brush: *mut GpTexture, matrix: *mut Matrix) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetTextureTransform(brush : *mut GpTexture, matrix : *mut Matrix) -> Status);
    unsafe { GdipGetTextureTransform(brush as _, matrix as _) }
}
#[inline]
pub unsafe fn GdipGetTextureWrapMode(brush: *mut GpTexture, wrapmode: *mut WrapMode) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetTextureWrapMode(brush : *mut GpTexture, wrapmode : *mut WrapMode) -> Status);
    unsafe { GdipGetTextureWrapMode(brush as _, wrapmode as _) }
}
#[inline]
pub unsafe fn GdipGetVisibleClipBounds(graphics: *mut GpGraphics, rect: *mut RectF) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetVisibleClipBounds(graphics : *mut GpGraphics, rect : *mut RectF) -> Status);
    unsafe { GdipGetVisibleClipBounds(graphics as _, rect as _) }
}
#[inline]
pub unsafe fn GdipGetVisibleClipBoundsI(graphics: *mut GpGraphics, rect: *mut Rect) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetVisibleClipBoundsI(graphics : *mut GpGraphics, rect : *mut Rect) -> Status);
    unsafe { GdipGetVisibleClipBoundsI(graphics as _, rect as _) }
}
#[inline]
pub unsafe fn GdipGetWorldTransform(graphics: *mut GpGraphics, matrix: *mut Matrix) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGetWorldTransform(graphics : *mut GpGraphics, matrix : *mut Matrix) -> Status);
    unsafe { GdipGetWorldTransform(graphics as _, matrix as _) }
}
#[inline]
pub unsafe fn GdipGraphicsClear(graphics: *mut GpGraphics, color: u32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipGraphicsClear(graphics : *mut GpGraphics, color : u32) -> Status);
    unsafe { GdipGraphicsClear(graphics as _, color) }
}
#[inline]
pub unsafe fn GdipGraphicsSetAbort<P1>(pgraphics: *mut GpGraphics, piabort: P1) -> Status
where
    P1: windows_core::Param<GdiplusAbort>,
{
    windows_core::link!("gdiplus.dll" "system" fn GdipGraphicsSetAbort(pgraphics : *mut GpGraphics, piabort : * mut core::ffi::c_void) -> Status);
    unsafe { GdipGraphicsSetAbort(pgraphics as _, piabort.param().abi()) }
}
#[inline]
pub unsafe fn GdipImageForceValidation(image: *mut GpImage) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipImageForceValidation(image : *mut GpImage) -> Status);
    unsafe { GdipImageForceValidation(image as _) }
}
#[inline]
pub unsafe fn GdipImageGetFrameCount(image: *mut GpImage, dimensionid: *const windows_core::GUID, count: *mut u32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipImageGetFrameCount(image : *mut GpImage, dimensionid : *const windows_core::GUID, count : *mut u32) -> Status);
    unsafe { GdipImageGetFrameCount(image as _, dimensionid, count as _) }
}
#[inline]
pub unsafe fn GdipImageGetFrameDimensionsCount(image: *mut GpImage, count: *mut u32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipImageGetFrameDimensionsCount(image : *mut GpImage, count : *mut u32) -> Status);
    unsafe { GdipImageGetFrameDimensionsCount(image as _, count as _) }
}
#[inline]
pub unsafe fn GdipImageGetFrameDimensionsList(image: *mut GpImage, dimensionids: *mut windows_core::GUID, count: u32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipImageGetFrameDimensionsList(image : *mut GpImage, dimensionids : *mut windows_core::GUID, count : u32) -> Status);
    unsafe { GdipImageGetFrameDimensionsList(image as _, dimensionids as _, count) }
}
#[inline]
pub unsafe fn GdipImageRotateFlip(image: *mut GpImage, rftype: RotateFlipType) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipImageRotateFlip(image : *mut GpImage, rftype : RotateFlipType) -> Status);
    unsafe { GdipImageRotateFlip(image as _, rftype) }
}
#[inline]
pub unsafe fn GdipImageSelectActiveFrame(image: *mut GpImage, dimensionid: *const windows_core::GUID, frameindex: u32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipImageSelectActiveFrame(image : *mut GpImage, dimensionid : *const windows_core::GUID, frameindex : u32) -> Status);
    unsafe { GdipImageSelectActiveFrame(image as _, dimensionid, frameindex) }
}
#[inline]
pub unsafe fn GdipImageSetAbort<P1>(pimage: *mut GpImage, piabort: P1) -> Status
where
    P1: windows_core::Param<GdiplusAbort>,
{
    windows_core::link!("gdiplus.dll" "system" fn GdipImageSetAbort(pimage : *mut GpImage, piabort : * mut core::ffi::c_void) -> Status);
    unsafe { GdipImageSetAbort(pimage as _, piabort.param().abi()) }
}
#[inline]
pub unsafe fn GdipInitializePalette(palette: *mut ColorPalette, palettetype: PaletteType, optimalcolors: i32, usetransparentcolor: bool, bitmap: *mut GpBitmap) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipInitializePalette(palette : *mut ColorPalette, palettetype : PaletteType, optimalcolors : i32, usetransparentcolor : windows_core::BOOL, bitmap : *mut GpBitmap) -> Status);
    unsafe { GdipInitializePalette(palette as _, palettetype, optimalcolors, usetransparentcolor.into(), bitmap as _) }
}
#[inline]
pub unsafe fn GdipInvertMatrix(matrix: *mut Matrix) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipInvertMatrix(matrix : *mut Matrix) -> Status);
    unsafe { GdipInvertMatrix(matrix as _) }
}
#[inline]
pub unsafe fn GdipIsClipEmpty(graphics: *mut GpGraphics, result: *mut windows_core::BOOL) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipIsClipEmpty(graphics : *mut GpGraphics, result : *mut windows_core::BOOL) -> Status);
    unsafe { GdipIsClipEmpty(graphics as _, result as _) }
}
#[inline]
pub unsafe fn GdipIsEmptyRegion(region: *mut GpRegion, graphics: *mut GpGraphics, result: *mut windows_core::BOOL) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipIsEmptyRegion(region : *mut GpRegion, graphics : *mut GpGraphics, result : *mut windows_core::BOOL) -> Status);
    unsafe { GdipIsEmptyRegion(region as _, graphics as _, result as _) }
}
#[inline]
pub unsafe fn GdipIsEqualRegion(region: *mut GpRegion, region2: *mut GpRegion, graphics: *mut GpGraphics, result: *mut windows_core::BOOL) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipIsEqualRegion(region : *mut GpRegion, region2 : *mut GpRegion, graphics : *mut GpGraphics, result : *mut windows_core::BOOL) -> Status);
    unsafe { GdipIsEqualRegion(region as _, region2 as _, graphics as _, result as _) }
}
#[inline]
pub unsafe fn GdipIsInfiniteRegion(region: *mut GpRegion, graphics: *mut GpGraphics, result: *mut windows_core::BOOL) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipIsInfiniteRegion(region : *mut GpRegion, graphics : *mut GpGraphics, result : *mut windows_core::BOOL) -> Status);
    unsafe { GdipIsInfiniteRegion(region as _, graphics as _, result as _) }
}
#[inline]
pub unsafe fn GdipIsMatrixEqual(matrix: *const Matrix, matrix2: *const Matrix, result: *mut windows_core::BOOL) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipIsMatrixEqual(matrix : *const Matrix, matrix2 : *const Matrix, result : *mut windows_core::BOOL) -> Status);
    unsafe { GdipIsMatrixEqual(matrix, matrix2, result as _) }
}
#[inline]
pub unsafe fn GdipIsMatrixIdentity(matrix: *const Matrix, result: *mut windows_core::BOOL) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipIsMatrixIdentity(matrix : *const Matrix, result : *mut windows_core::BOOL) -> Status);
    unsafe { GdipIsMatrixIdentity(matrix, result as _) }
}
#[inline]
pub unsafe fn GdipIsMatrixInvertible(matrix: *const Matrix, result: *mut windows_core::BOOL) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipIsMatrixInvertible(matrix : *const Matrix, result : *mut windows_core::BOOL) -> Status);
    unsafe { GdipIsMatrixInvertible(matrix, result as _) }
}
#[inline]
pub unsafe fn GdipIsOutlineVisiblePathPoint(path: *mut GpPath, x: f32, y: f32, pen: *mut GpPen, graphics: *mut GpGraphics, result: *mut windows_core::BOOL) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipIsOutlineVisiblePathPoint(path : *mut GpPath, x : f32, y : f32, pen : *mut GpPen, graphics : *mut GpGraphics, result : *mut windows_core::BOOL) -> Status);
    unsafe { GdipIsOutlineVisiblePathPoint(path as _, x, y, pen as _, graphics as _, result as _) }
}
#[inline]
pub unsafe fn GdipIsOutlineVisiblePathPointI(path: *mut GpPath, x: i32, y: i32, pen: *mut GpPen, graphics: *mut GpGraphics, result: *mut windows_core::BOOL) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipIsOutlineVisiblePathPointI(path : *mut GpPath, x : i32, y : i32, pen : *mut GpPen, graphics : *mut GpGraphics, result : *mut windows_core::BOOL) -> Status);
    unsafe { GdipIsOutlineVisiblePathPointI(path as _, x, y, pen as _, graphics as _, result as _) }
}
#[inline]
pub unsafe fn GdipIsStyleAvailable(family: *const GpFontFamily, style: i32, isstyleavailable: *mut windows_core::BOOL) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipIsStyleAvailable(family : *const GpFontFamily, style : i32, isstyleavailable : *mut windows_core::BOOL) -> Status);
    unsafe { GdipIsStyleAvailable(family, style, isstyleavailable as _) }
}
#[inline]
pub unsafe fn GdipIsVisibleClipEmpty(graphics: *mut GpGraphics, result: *mut windows_core::BOOL) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipIsVisibleClipEmpty(graphics : *mut GpGraphics, result : *mut windows_core::BOOL) -> Status);
    unsafe { GdipIsVisibleClipEmpty(graphics as _, result as _) }
}
#[inline]
pub unsafe fn GdipIsVisiblePathPoint(path: *mut GpPath, x: f32, y: f32, graphics: *mut GpGraphics, result: *mut windows_core::BOOL) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipIsVisiblePathPoint(path : *mut GpPath, x : f32, y : f32, graphics : *mut GpGraphics, result : *mut windows_core::BOOL) -> Status);
    unsafe { GdipIsVisiblePathPoint(path as _, x, y, graphics as _, result as _) }
}
#[inline]
pub unsafe fn GdipIsVisiblePathPointI(path: *mut GpPath, x: i32, y: i32, graphics: *mut GpGraphics, result: *mut windows_core::BOOL) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipIsVisiblePathPointI(path : *mut GpPath, x : i32, y : i32, graphics : *mut GpGraphics, result : *mut windows_core::BOOL) -> Status);
    unsafe { GdipIsVisiblePathPointI(path as _, x, y, graphics as _, result as _) }
}
#[inline]
pub unsafe fn GdipIsVisiblePoint(graphics: *mut GpGraphics, x: f32, y: f32, result: *mut windows_core::BOOL) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipIsVisiblePoint(graphics : *mut GpGraphics, x : f32, y : f32, result : *mut windows_core::BOOL) -> Status);
    unsafe { GdipIsVisiblePoint(graphics as _, x, y, result as _) }
}
#[inline]
pub unsafe fn GdipIsVisiblePointI(graphics: *mut GpGraphics, x: i32, y: i32, result: *mut windows_core::BOOL) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipIsVisiblePointI(graphics : *mut GpGraphics, x : i32, y : i32, result : *mut windows_core::BOOL) -> Status);
    unsafe { GdipIsVisiblePointI(graphics as _, x, y, result as _) }
}
#[inline]
pub unsafe fn GdipIsVisibleRect(graphics: *mut GpGraphics, x: f32, y: f32, width: f32, height: f32, result: *mut windows_core::BOOL) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipIsVisibleRect(graphics : *mut GpGraphics, x : f32, y : f32, width : f32, height : f32, result : *mut windows_core::BOOL) -> Status);
    unsafe { GdipIsVisibleRect(graphics as _, x, y, width, height, result as _) }
}
#[inline]
pub unsafe fn GdipIsVisibleRectI(graphics: *mut GpGraphics, x: i32, y: i32, width: i32, height: i32, result: *mut windows_core::BOOL) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipIsVisibleRectI(graphics : *mut GpGraphics, x : i32, y : i32, width : i32, height : i32, result : *mut windows_core::BOOL) -> Status);
    unsafe { GdipIsVisibleRectI(graphics as _, x, y, width, height, result as _) }
}
#[inline]
pub unsafe fn GdipIsVisibleRegionPoint(region: *mut GpRegion, x: f32, y: f32, graphics: *mut GpGraphics, result: *mut windows_core::BOOL) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipIsVisibleRegionPoint(region : *mut GpRegion, x : f32, y : f32, graphics : *mut GpGraphics, result : *mut windows_core::BOOL) -> Status);
    unsafe { GdipIsVisibleRegionPoint(region as _, x, y, graphics as _, result as _) }
}
#[inline]
pub unsafe fn GdipIsVisibleRegionPointI(region: *mut GpRegion, x: i32, y: i32, graphics: *mut GpGraphics, result: *mut windows_core::BOOL) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipIsVisibleRegionPointI(region : *mut GpRegion, x : i32, y : i32, graphics : *mut GpGraphics, result : *mut windows_core::BOOL) -> Status);
    unsafe { GdipIsVisibleRegionPointI(region as _, x, y, graphics as _, result as _) }
}
#[inline]
pub unsafe fn GdipIsVisibleRegionRect(region: *mut GpRegion, x: f32, y: f32, width: f32, height: f32, graphics: *mut GpGraphics, result: *mut windows_core::BOOL) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipIsVisibleRegionRect(region : *mut GpRegion, x : f32, y : f32, width : f32, height : f32, graphics : *mut GpGraphics, result : *mut windows_core::BOOL) -> Status);
    unsafe { GdipIsVisibleRegionRect(region as _, x, y, width, height, graphics as _, result as _) }
}
#[inline]
pub unsafe fn GdipIsVisibleRegionRectI(region: *mut GpRegion, x: i32, y: i32, width: i32, height: i32, graphics: *mut GpGraphics, result: *mut windows_core::BOOL) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipIsVisibleRegionRectI(region : *mut GpRegion, x : i32, y : i32, width : i32, height : i32, graphics : *mut GpGraphics, result : *mut windows_core::BOOL) -> Status);
    unsafe { GdipIsVisibleRegionRectI(region as _, x, y, width, height, graphics as _, result as _) }
}
#[inline]
pub unsafe fn GdipLoadImageFromFile<P0>(filename: P0, image: *mut *mut GpImage) -> Status
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("gdiplus.dll" "system" fn GdipLoadImageFromFile(filename : windows_core::PCWSTR, image : *mut *mut GpImage) -> Status);
    unsafe { GdipLoadImageFromFile(filename.param().abi(), image as _) }
}
#[inline]
pub unsafe fn GdipLoadImageFromFileICM<P0>(filename: P0, image: *mut *mut GpImage) -> Status
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("gdiplus.dll" "system" fn GdipLoadImageFromFileICM(filename : windows_core::PCWSTR, image : *mut *mut GpImage) -> Status);
    unsafe { GdipLoadImageFromFileICM(filename.param().abi(), image as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn GdipLoadImageFromStream<P0>(stream: P0, image: *mut *mut GpImage) -> Status
where
    P0: windows_core::Param<super::super::System::Com::IStream>,
{
    windows_core::link!("gdiplus.dll" "system" fn GdipLoadImageFromStream(stream : * mut core::ffi::c_void, image : *mut *mut GpImage) -> Status);
    unsafe { GdipLoadImageFromStream(stream.param().abi(), image as _) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn GdipLoadImageFromStreamICM<P0>(stream: P0, image: *mut *mut GpImage) -> Status
where
    P0: windows_core::Param<super::super::System::Com::IStream>,
{
    windows_core::link!("gdiplus.dll" "system" fn GdipLoadImageFromStreamICM(stream : * mut core::ffi::c_void, image : *mut *mut GpImage) -> Status);
    unsafe { GdipLoadImageFromStreamICM(stream.param().abi(), image as _) }
}
#[inline]
pub unsafe fn GdipMeasureCharacterRanges<P1>(graphics: *mut GpGraphics, string: P1, length: i32, font: *const GpFont, layoutrect: *const RectF, stringformat: *const GpStringFormat, regioncount: i32, regions: *mut *mut GpRegion) -> Status
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("gdiplus.dll" "system" fn GdipMeasureCharacterRanges(graphics : *mut GpGraphics, string : windows_core::PCWSTR, length : i32, font : *const GpFont, layoutrect : *const RectF, stringformat : *const GpStringFormat, regioncount : i32, regions : *mut *mut GpRegion) -> Status);
    unsafe { GdipMeasureCharacterRanges(graphics as _, string.param().abi(), length, font, layoutrect, stringformat, regioncount, regions as _) }
}
#[inline]
pub unsafe fn GdipMeasureDriverString(graphics: *mut GpGraphics, text: *const u16, length: i32, font: *const GpFont, positions: *const PointF, flags: i32, matrix: *const Matrix, boundingbox: *mut RectF) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipMeasureDriverString(graphics : *mut GpGraphics, text : *const u16, length : i32, font : *const GpFont, positions : *const PointF, flags : i32, matrix : *const Matrix, boundingbox : *mut RectF) -> Status);
    unsafe { GdipMeasureDriverString(graphics as _, text, length, font, positions, flags, matrix, boundingbox as _) }
}
#[inline]
pub unsafe fn GdipMeasureString<P1>(graphics: *mut GpGraphics, string: P1, length: i32, font: *const GpFont, layoutrect: *const RectF, stringformat: *const GpStringFormat, boundingbox: *mut RectF, codepointsfitted: *mut i32, linesfilled: *mut i32) -> Status
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("gdiplus.dll" "system" fn GdipMeasureString(graphics : *mut GpGraphics, string : windows_core::PCWSTR, length : i32, font : *const GpFont, layoutrect : *const RectF, stringformat : *const GpStringFormat, boundingbox : *mut RectF, codepointsfitted : *mut i32, linesfilled : *mut i32) -> Status);
    unsafe { GdipMeasureString(graphics as _, string.param().abi(), length, font, layoutrect, stringformat, boundingbox as _, codepointsfitted as _, linesfilled as _) }
}
#[inline]
pub unsafe fn GdipMultiplyLineTransform(brush: *mut GpLineGradient, matrix: *const Matrix, order: MatrixOrder) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipMultiplyLineTransform(brush : *mut GpLineGradient, matrix : *const Matrix, order : MatrixOrder) -> Status);
    unsafe { GdipMultiplyLineTransform(brush as _, matrix, order) }
}
#[inline]
pub unsafe fn GdipMultiplyMatrix(matrix: *mut Matrix, matrix2: *mut Matrix, order: MatrixOrder) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipMultiplyMatrix(matrix : *mut Matrix, matrix2 : *mut Matrix, order : MatrixOrder) -> Status);
    unsafe { GdipMultiplyMatrix(matrix as _, matrix2 as _, order) }
}
#[inline]
pub unsafe fn GdipMultiplyPathGradientTransform(brush: *mut GpPathGradient, matrix: *const Matrix, order: MatrixOrder) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipMultiplyPathGradientTransform(brush : *mut GpPathGradient, matrix : *const Matrix, order : MatrixOrder) -> Status);
    unsafe { GdipMultiplyPathGradientTransform(brush as _, matrix, order) }
}
#[inline]
pub unsafe fn GdipMultiplyPenTransform(pen: *mut GpPen, matrix: *const Matrix, order: MatrixOrder) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipMultiplyPenTransform(pen : *mut GpPen, matrix : *const Matrix, order : MatrixOrder) -> Status);
    unsafe { GdipMultiplyPenTransform(pen as _, matrix, order) }
}
#[inline]
pub unsafe fn GdipMultiplyTextureTransform(brush: *mut GpTexture, matrix: *const Matrix, order: MatrixOrder) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipMultiplyTextureTransform(brush : *mut GpTexture, matrix : *const Matrix, order : MatrixOrder) -> Status);
    unsafe { GdipMultiplyTextureTransform(brush as _, matrix, order) }
}
#[inline]
pub unsafe fn GdipMultiplyWorldTransform(graphics: *mut GpGraphics, matrix: *const Matrix, order: MatrixOrder) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipMultiplyWorldTransform(graphics : *mut GpGraphics, matrix : *const Matrix, order : MatrixOrder) -> Status);
    unsafe { GdipMultiplyWorldTransform(graphics as _, matrix, order) }
}
#[inline]
pub unsafe fn GdipNewInstalledFontCollection(fontcollection: *mut *mut GpFontCollection) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipNewInstalledFontCollection(fontcollection : *mut *mut GpFontCollection) -> Status);
    unsafe { GdipNewInstalledFontCollection(fontcollection as _) }
}
#[inline]
pub unsafe fn GdipNewPrivateFontCollection(fontcollection: *mut *mut GpFontCollection) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipNewPrivateFontCollection(fontcollection : *mut *mut GpFontCollection) -> Status);
    unsafe { GdipNewPrivateFontCollection(fontcollection as _) }
}
#[inline]
pub unsafe fn GdipPathIterCopyData(iterator: *mut GpPathIterator, resultcount: *mut i32, points: *mut PointF, types: *mut u8, startindex: i32, endindex: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipPathIterCopyData(iterator : *mut GpPathIterator, resultcount : *mut i32, points : *mut PointF, types : *mut u8, startindex : i32, endindex : i32) -> Status);
    unsafe { GdipPathIterCopyData(iterator as _, resultcount as _, points as _, types as _, startindex, endindex) }
}
#[inline]
pub unsafe fn GdipPathIterEnumerate(iterator: *mut GpPathIterator, resultcount: *mut i32, points: *mut PointF, types: *mut u8, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipPathIterEnumerate(iterator : *mut GpPathIterator, resultcount : *mut i32, points : *mut PointF, types : *mut u8, count : i32) -> Status);
    unsafe { GdipPathIterEnumerate(iterator as _, resultcount as _, points as _, types as _, count) }
}
#[inline]
pub unsafe fn GdipPathIterGetCount(iterator: *mut GpPathIterator, count: *mut i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipPathIterGetCount(iterator : *mut GpPathIterator, count : *mut i32) -> Status);
    unsafe { GdipPathIterGetCount(iterator as _, count as _) }
}
#[inline]
pub unsafe fn GdipPathIterGetSubpathCount(iterator: *mut GpPathIterator, count: *mut i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipPathIterGetSubpathCount(iterator : *mut GpPathIterator, count : *mut i32) -> Status);
    unsafe { GdipPathIterGetSubpathCount(iterator as _, count as _) }
}
#[inline]
pub unsafe fn GdipPathIterHasCurve(iterator: *mut GpPathIterator, hascurve: *mut windows_core::BOOL) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipPathIterHasCurve(iterator : *mut GpPathIterator, hascurve : *mut windows_core::BOOL) -> Status);
    unsafe { GdipPathIterHasCurve(iterator as _, hascurve as _) }
}
#[inline]
pub unsafe fn GdipPathIterIsValid(iterator: *mut GpPathIterator, valid: *mut windows_core::BOOL) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipPathIterIsValid(iterator : *mut GpPathIterator, valid : *mut windows_core::BOOL) -> Status);
    unsafe { GdipPathIterIsValid(iterator as _, valid as _) }
}
#[inline]
pub unsafe fn GdipPathIterNextMarker(iterator: *mut GpPathIterator, resultcount: *mut i32, startindex: *mut i32, endindex: *mut i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipPathIterNextMarker(iterator : *mut GpPathIterator, resultcount : *mut i32, startindex : *mut i32, endindex : *mut i32) -> Status);
    unsafe { GdipPathIterNextMarker(iterator as _, resultcount as _, startindex as _, endindex as _) }
}
#[inline]
pub unsafe fn GdipPathIterNextMarkerPath(iterator: *mut GpPathIterator, resultcount: *mut i32, path: *mut GpPath) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipPathIterNextMarkerPath(iterator : *mut GpPathIterator, resultcount : *mut i32, path : *mut GpPath) -> Status);
    unsafe { GdipPathIterNextMarkerPath(iterator as _, resultcount as _, path as _) }
}
#[inline]
pub unsafe fn GdipPathIterNextPathType(iterator: *mut GpPathIterator, resultcount: *mut i32, pathtype: *mut u8, startindex: *mut i32, endindex: *mut i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipPathIterNextPathType(iterator : *mut GpPathIterator, resultcount : *mut i32, pathtype : *mut u8, startindex : *mut i32, endindex : *mut i32) -> Status);
    unsafe { GdipPathIterNextPathType(iterator as _, resultcount as _, pathtype as _, startindex as _, endindex as _) }
}
#[inline]
pub unsafe fn GdipPathIterNextSubpath(iterator: *mut GpPathIterator, resultcount: *mut i32, startindex: *mut i32, endindex: *mut i32, isclosed: *mut windows_core::BOOL) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipPathIterNextSubpath(iterator : *mut GpPathIterator, resultcount : *mut i32, startindex : *mut i32, endindex : *mut i32, isclosed : *mut windows_core::BOOL) -> Status);
    unsafe { GdipPathIterNextSubpath(iterator as _, resultcount as _, startindex as _, endindex as _, isclosed as _) }
}
#[inline]
pub unsafe fn GdipPathIterNextSubpathPath(iterator: *mut GpPathIterator, resultcount: *mut i32, path: *mut GpPath, isclosed: *mut windows_core::BOOL) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipPathIterNextSubpathPath(iterator : *mut GpPathIterator, resultcount : *mut i32, path : *mut GpPath, isclosed : *mut windows_core::BOOL) -> Status);
    unsafe { GdipPathIterNextSubpathPath(iterator as _, resultcount as _, path as _, isclosed as _) }
}
#[inline]
pub unsafe fn GdipPathIterRewind(iterator: *mut GpPathIterator) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipPathIterRewind(iterator : *mut GpPathIterator) -> Status);
    unsafe { GdipPathIterRewind(iterator as _) }
}
#[inline]
pub unsafe fn GdipPlayMetafileRecord(metafile: *const GpMetafile, recordtype: EmfPlusRecordType, flags: u32, datasize: u32, data: *const u8) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipPlayMetafileRecord(metafile : *const GpMetafile, recordtype : EmfPlusRecordType, flags : u32, datasize : u32, data : *const u8) -> Status);
    unsafe { GdipPlayMetafileRecord(metafile, recordtype, flags, datasize, data) }
}
#[inline]
pub unsafe fn GdipPrivateAddFontFile<P1>(fontcollection: *mut GpFontCollection, filename: P1) -> Status
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("gdiplus.dll" "system" fn GdipPrivateAddFontFile(fontcollection : *mut GpFontCollection, filename : windows_core::PCWSTR) -> Status);
    unsafe { GdipPrivateAddFontFile(fontcollection as _, filename.param().abi()) }
}
#[inline]
pub unsafe fn GdipPrivateAddMemoryFont(fontcollection: *mut GpFontCollection, memory: *const core::ffi::c_void, length: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipPrivateAddMemoryFont(fontcollection : *mut GpFontCollection, memory : *const core::ffi::c_void, length : i32) -> Status);
    unsafe { GdipPrivateAddMemoryFont(fontcollection as _, memory, length) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipRecordMetafile<P4>(referencehdc: super::Gdi::HDC, r#type: EmfType, framerect: *const RectF, frameunit: MetafileFrameUnit, description: P4, metafile: *mut *mut GpMetafile) -> Status
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("gdiplus.dll" "system" fn GdipRecordMetafile(referencehdc : super::Gdi:: HDC, r#type : EmfType, framerect : *const RectF, frameunit : MetafileFrameUnit, description : windows_core::PCWSTR, metafile : *mut *mut GpMetafile) -> Status);
    unsafe { GdipRecordMetafile(referencehdc, r#type, framerect, frameunit, description.param().abi(), metafile as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipRecordMetafileFileName<P0, P5>(filename: P0, referencehdc: super::Gdi::HDC, r#type: EmfType, framerect: *const RectF, frameunit: MetafileFrameUnit, description: P5, metafile: *mut *mut GpMetafile) -> Status
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("gdiplus.dll" "system" fn GdipRecordMetafileFileName(filename : windows_core::PCWSTR, referencehdc : super::Gdi:: HDC, r#type : EmfType, framerect : *const RectF, frameunit : MetafileFrameUnit, description : windows_core::PCWSTR, metafile : *mut *mut GpMetafile) -> Status);
    unsafe { GdipRecordMetafileFileName(filename.param().abi(), referencehdc, r#type, framerect, frameunit, description.param().abi(), metafile as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipRecordMetafileFileNameI<P0, P5>(filename: P0, referencehdc: super::Gdi::HDC, r#type: EmfType, framerect: *const Rect, frameunit: MetafileFrameUnit, description: P5, metafile: *mut *mut GpMetafile) -> Status
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("gdiplus.dll" "system" fn GdipRecordMetafileFileNameI(filename : windows_core::PCWSTR, referencehdc : super::Gdi:: HDC, r#type : EmfType, framerect : *const Rect, frameunit : MetafileFrameUnit, description : windows_core::PCWSTR, metafile : *mut *mut GpMetafile) -> Status);
    unsafe { GdipRecordMetafileFileNameI(filename.param().abi(), referencehdc, r#type, framerect, frameunit, description.param().abi(), metafile as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipRecordMetafileI<P4>(referencehdc: super::Gdi::HDC, r#type: EmfType, framerect: *const Rect, frameunit: MetafileFrameUnit, description: P4, metafile: *mut *mut GpMetafile) -> Status
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("gdiplus.dll" "system" fn GdipRecordMetafileI(referencehdc : super::Gdi:: HDC, r#type : EmfType, framerect : *const Rect, frameunit : MetafileFrameUnit, description : windows_core::PCWSTR, metafile : *mut *mut GpMetafile) -> Status);
    unsafe { GdipRecordMetafileI(referencehdc, r#type, framerect, frameunit, description.param().abi(), metafile as _) }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn GdipRecordMetafileStream<P0, P5>(stream: P0, referencehdc: super::Gdi::HDC, r#type: EmfType, framerect: *const RectF, frameunit: MetafileFrameUnit, description: P5, metafile: *mut *mut GpMetafile) -> Status
where
    P0: windows_core::Param<super::super::System::Com::IStream>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("gdiplus.dll" "system" fn GdipRecordMetafileStream(stream : * mut core::ffi::c_void, referencehdc : super::Gdi:: HDC, r#type : EmfType, framerect : *const RectF, frameunit : MetafileFrameUnit, description : windows_core::PCWSTR, metafile : *mut *mut GpMetafile) -> Status);
    unsafe { GdipRecordMetafileStream(stream.param().abi(), referencehdc, r#type, framerect, frameunit, description.param().abi(), metafile as _) }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn GdipRecordMetafileStreamI<P0, P5>(stream: P0, referencehdc: super::Gdi::HDC, r#type: EmfType, framerect: *const Rect, frameunit: MetafileFrameUnit, description: P5, metafile: *mut *mut GpMetafile) -> Status
where
    P0: windows_core::Param<super::super::System::Com::IStream>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("gdiplus.dll" "system" fn GdipRecordMetafileStreamI(stream : * mut core::ffi::c_void, referencehdc : super::Gdi:: HDC, r#type : EmfType, framerect : *const Rect, frameunit : MetafileFrameUnit, description : windows_core::PCWSTR, metafile : *mut *mut GpMetafile) -> Status);
    unsafe { GdipRecordMetafileStreamI(stream.param().abi(), referencehdc, r#type, framerect, frameunit, description.param().abi(), metafile as _) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipReleaseDC(graphics: *mut GpGraphics, hdc: super::Gdi::HDC) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipReleaseDC(graphics : *mut GpGraphics, hdc : super::Gdi:: HDC) -> Status);
    unsafe { GdipReleaseDC(graphics as _, hdc) }
}
#[inline]
pub unsafe fn GdipRemovePropertyItem(image: *mut GpImage, propid: u32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipRemovePropertyItem(image : *mut GpImage, propid : u32) -> Status);
    unsafe { GdipRemovePropertyItem(image as _, propid) }
}
#[inline]
pub unsafe fn GdipResetClip(graphics: *mut GpGraphics) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipResetClip(graphics : *mut GpGraphics) -> Status);
    unsafe { GdipResetClip(graphics as _) }
}
#[inline]
pub unsafe fn GdipResetImageAttributes(imageattr: *mut GpImageAttributes, r#type: ColorAdjustType) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipResetImageAttributes(imageattr : *mut GpImageAttributes, r#type : ColorAdjustType) -> Status);
    unsafe { GdipResetImageAttributes(imageattr as _, r#type) }
}
#[inline]
pub unsafe fn GdipResetLineTransform(brush: *mut GpLineGradient) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipResetLineTransform(brush : *mut GpLineGradient) -> Status);
    unsafe { GdipResetLineTransform(brush as _) }
}
#[inline]
pub unsafe fn GdipResetPageTransform(graphics: *mut GpGraphics) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipResetPageTransform(graphics : *mut GpGraphics) -> Status);
    unsafe { GdipResetPageTransform(graphics as _) }
}
#[inline]
pub unsafe fn GdipResetPath(path: *mut GpPath) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipResetPath(path : *mut GpPath) -> Status);
    unsafe { GdipResetPath(path as _) }
}
#[inline]
pub unsafe fn GdipResetPathGradientTransform(brush: *mut GpPathGradient) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipResetPathGradientTransform(brush : *mut GpPathGradient) -> Status);
    unsafe { GdipResetPathGradientTransform(brush as _) }
}
#[inline]
pub unsafe fn GdipResetPenTransform(pen: *mut GpPen) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipResetPenTransform(pen : *mut GpPen) -> Status);
    unsafe { GdipResetPenTransform(pen as _) }
}
#[inline]
pub unsafe fn GdipResetTextureTransform(brush: *mut GpTexture) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipResetTextureTransform(brush : *mut GpTexture) -> Status);
    unsafe { GdipResetTextureTransform(brush as _) }
}
#[inline]
pub unsafe fn GdipResetWorldTransform(graphics: *mut GpGraphics) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipResetWorldTransform(graphics : *mut GpGraphics) -> Status);
    unsafe { GdipResetWorldTransform(graphics as _) }
}
#[inline]
pub unsafe fn GdipRestoreGraphics(graphics: *mut GpGraphics, state: u32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipRestoreGraphics(graphics : *mut GpGraphics, state : u32) -> Status);
    unsafe { GdipRestoreGraphics(graphics as _, state) }
}
#[inline]
pub unsafe fn GdipReversePath(path: *mut GpPath) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipReversePath(path : *mut GpPath) -> Status);
    unsafe { GdipReversePath(path as _) }
}
#[inline]
pub unsafe fn GdipRotateLineTransform(brush: *mut GpLineGradient, angle: f32, order: MatrixOrder) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipRotateLineTransform(brush : *mut GpLineGradient, angle : f32, order : MatrixOrder) -> Status);
    unsafe { GdipRotateLineTransform(brush as _, angle, order) }
}
#[inline]
pub unsafe fn GdipRotateMatrix(matrix: *mut Matrix, angle: f32, order: MatrixOrder) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipRotateMatrix(matrix : *mut Matrix, angle : f32, order : MatrixOrder) -> Status);
    unsafe { GdipRotateMatrix(matrix as _, angle, order) }
}
#[inline]
pub unsafe fn GdipRotatePathGradientTransform(brush: *mut GpPathGradient, angle: f32, order: MatrixOrder) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipRotatePathGradientTransform(brush : *mut GpPathGradient, angle : f32, order : MatrixOrder) -> Status);
    unsafe { GdipRotatePathGradientTransform(brush as _, angle, order) }
}
#[inline]
pub unsafe fn GdipRotatePenTransform(pen: *mut GpPen, angle: f32, order: MatrixOrder) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipRotatePenTransform(pen : *mut GpPen, angle : f32, order : MatrixOrder) -> Status);
    unsafe { GdipRotatePenTransform(pen as _, angle, order) }
}
#[inline]
pub unsafe fn GdipRotateTextureTransform(brush: *mut GpTexture, angle: f32, order: MatrixOrder) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipRotateTextureTransform(brush : *mut GpTexture, angle : f32, order : MatrixOrder) -> Status);
    unsafe { GdipRotateTextureTransform(brush as _, angle, order) }
}
#[inline]
pub unsafe fn GdipRotateWorldTransform(graphics: *mut GpGraphics, angle: f32, order: MatrixOrder) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipRotateWorldTransform(graphics : *mut GpGraphics, angle : f32, order : MatrixOrder) -> Status);
    unsafe { GdipRotateWorldTransform(graphics as _, angle, order) }
}
#[inline]
pub unsafe fn GdipSaveAdd(image: *mut GpImage, encoderparams: *const EncoderParameters) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSaveAdd(image : *mut GpImage, encoderparams : *const EncoderParameters) -> Status);
    unsafe { GdipSaveAdd(image as _, encoderparams) }
}
#[inline]
pub unsafe fn GdipSaveAddImage(image: *mut GpImage, newimage: *mut GpImage, encoderparams: *const EncoderParameters) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSaveAddImage(image : *mut GpImage, newimage : *mut GpImage, encoderparams : *const EncoderParameters) -> Status);
    unsafe { GdipSaveAddImage(image as _, newimage as _, encoderparams) }
}
#[inline]
pub unsafe fn GdipSaveGraphics(graphics: *mut GpGraphics, state: *mut u32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSaveGraphics(graphics : *mut GpGraphics, state : *mut u32) -> Status);
    unsafe { GdipSaveGraphics(graphics as _, state as _) }
}
#[inline]
pub unsafe fn GdipSaveImageToFile<P1>(image: *mut GpImage, filename: P1, clsidencoder: *const windows_core::GUID, encoderparams: *const EncoderParameters) -> Status
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("gdiplus.dll" "system" fn GdipSaveImageToFile(image : *mut GpImage, filename : windows_core::PCWSTR, clsidencoder : *const windows_core::GUID, encoderparams : *const EncoderParameters) -> Status);
    unsafe { GdipSaveImageToFile(image as _, filename.param().abi(), clsidencoder, encoderparams) }
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn GdipSaveImageToStream<P1>(image: *mut GpImage, stream: P1, clsidencoder: *const windows_core::GUID, encoderparams: *const EncoderParameters) -> Status
where
    P1: windows_core::Param<super::super::System::Com::IStream>,
{
    windows_core::link!("gdiplus.dll" "system" fn GdipSaveImageToStream(image : *mut GpImage, stream : * mut core::ffi::c_void, clsidencoder : *const windows_core::GUID, encoderparams : *const EncoderParameters) -> Status);
    unsafe { GdipSaveImageToStream(image as _, stream.param().abi(), clsidencoder, encoderparams) }
}
#[inline]
pub unsafe fn GdipScaleLineTransform(brush: *mut GpLineGradient, sx: f32, sy: f32, order: MatrixOrder) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipScaleLineTransform(brush : *mut GpLineGradient, sx : f32, sy : f32, order : MatrixOrder) -> Status);
    unsafe { GdipScaleLineTransform(brush as _, sx, sy, order) }
}
#[inline]
pub unsafe fn GdipScaleMatrix(matrix: *mut Matrix, scalex: f32, scaley: f32, order: MatrixOrder) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipScaleMatrix(matrix : *mut Matrix, scalex : f32, scaley : f32, order : MatrixOrder) -> Status);
    unsafe { GdipScaleMatrix(matrix as _, scalex, scaley, order) }
}
#[inline]
pub unsafe fn GdipScalePathGradientTransform(brush: *mut GpPathGradient, sx: f32, sy: f32, order: MatrixOrder) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipScalePathGradientTransform(brush : *mut GpPathGradient, sx : f32, sy : f32, order : MatrixOrder) -> Status);
    unsafe { GdipScalePathGradientTransform(brush as _, sx, sy, order) }
}
#[inline]
pub unsafe fn GdipScalePenTransform(pen: *mut GpPen, sx: f32, sy: f32, order: MatrixOrder) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipScalePenTransform(pen : *mut GpPen, sx : f32, sy : f32, order : MatrixOrder) -> Status);
    unsafe { GdipScalePenTransform(pen as _, sx, sy, order) }
}
#[inline]
pub unsafe fn GdipScaleTextureTransform(brush: *mut GpTexture, sx: f32, sy: f32, order: MatrixOrder) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipScaleTextureTransform(brush : *mut GpTexture, sx : f32, sy : f32, order : MatrixOrder) -> Status);
    unsafe { GdipScaleTextureTransform(brush as _, sx, sy, order) }
}
#[inline]
pub unsafe fn GdipScaleWorldTransform(graphics: *mut GpGraphics, sx: f32, sy: f32, order: MatrixOrder) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipScaleWorldTransform(graphics : *mut GpGraphics, sx : f32, sy : f32, order : MatrixOrder) -> Status);
    unsafe { GdipScaleWorldTransform(graphics as _, sx, sy, order) }
}
#[inline]
pub unsafe fn GdipSetAdjustableArrowCapFillState(cap: *mut GpAdjustableArrowCap, fillstate: bool) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetAdjustableArrowCapFillState(cap : *mut GpAdjustableArrowCap, fillstate : windows_core::BOOL) -> Status);
    unsafe { GdipSetAdjustableArrowCapFillState(cap as _, fillstate.into()) }
}
#[inline]
pub unsafe fn GdipSetAdjustableArrowCapHeight(cap: *mut GpAdjustableArrowCap, height: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetAdjustableArrowCapHeight(cap : *mut GpAdjustableArrowCap, height : f32) -> Status);
    unsafe { GdipSetAdjustableArrowCapHeight(cap as _, height) }
}
#[inline]
pub unsafe fn GdipSetAdjustableArrowCapMiddleInset(cap: *mut GpAdjustableArrowCap, middleinset: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetAdjustableArrowCapMiddleInset(cap : *mut GpAdjustableArrowCap, middleinset : f32) -> Status);
    unsafe { GdipSetAdjustableArrowCapMiddleInset(cap as _, middleinset) }
}
#[inline]
pub unsafe fn GdipSetAdjustableArrowCapWidth(cap: *mut GpAdjustableArrowCap, width: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetAdjustableArrowCapWidth(cap : *mut GpAdjustableArrowCap, width : f32) -> Status);
    unsafe { GdipSetAdjustableArrowCapWidth(cap as _, width) }
}
#[inline]
pub unsafe fn GdipSetClipGraphics(graphics: *mut GpGraphics, srcgraphics: *mut GpGraphics, combinemode: CombineMode) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetClipGraphics(graphics : *mut GpGraphics, srcgraphics : *mut GpGraphics, combinemode : CombineMode) -> Status);
    unsafe { GdipSetClipGraphics(graphics as _, srcgraphics as _, combinemode) }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipSetClipHrgn(graphics: *mut GpGraphics, hrgn: super::Gdi::HRGN, combinemode: CombineMode) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetClipHrgn(graphics : *mut GpGraphics, hrgn : super::Gdi:: HRGN, combinemode : CombineMode) -> Status);
    unsafe { GdipSetClipHrgn(graphics as _, hrgn, combinemode) }
}
#[inline]
pub unsafe fn GdipSetClipPath(graphics: *mut GpGraphics, path: *mut GpPath, combinemode: CombineMode) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetClipPath(graphics : *mut GpGraphics, path : *mut GpPath, combinemode : CombineMode) -> Status);
    unsafe { GdipSetClipPath(graphics as _, path as _, combinemode) }
}
#[inline]
pub unsafe fn GdipSetClipRect(graphics: *mut GpGraphics, x: f32, y: f32, width: f32, height: f32, combinemode: CombineMode) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetClipRect(graphics : *mut GpGraphics, x : f32, y : f32, width : f32, height : f32, combinemode : CombineMode) -> Status);
    unsafe { GdipSetClipRect(graphics as _, x, y, width, height, combinemode) }
}
#[inline]
pub unsafe fn GdipSetClipRectI(graphics: *mut GpGraphics, x: i32, y: i32, width: i32, height: i32, combinemode: CombineMode) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetClipRectI(graphics : *mut GpGraphics, x : i32, y : i32, width : i32, height : i32, combinemode : CombineMode) -> Status);
    unsafe { GdipSetClipRectI(graphics as _, x, y, width, height, combinemode) }
}
#[inline]
pub unsafe fn GdipSetClipRegion(graphics: *mut GpGraphics, region: *mut GpRegion, combinemode: CombineMode) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetClipRegion(graphics : *mut GpGraphics, region : *mut GpRegion, combinemode : CombineMode) -> Status);
    unsafe { GdipSetClipRegion(graphics as _, region as _, combinemode) }
}
#[inline]
pub unsafe fn GdipSetCompositingMode(graphics: *mut GpGraphics, compositingmode: CompositingMode) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetCompositingMode(graphics : *mut GpGraphics, compositingmode : CompositingMode) -> Status);
    unsafe { GdipSetCompositingMode(graphics as _, compositingmode) }
}
#[inline]
pub unsafe fn GdipSetCompositingQuality(graphics: *mut GpGraphics, compositingquality: CompositingQuality) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetCompositingQuality(graphics : *mut GpGraphics, compositingquality : CompositingQuality) -> Status);
    unsafe { GdipSetCompositingQuality(graphics as _, compositingquality) }
}
#[inline]
pub unsafe fn GdipSetCustomLineCapBaseCap(customcap: *mut GpCustomLineCap, basecap: LineCap) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetCustomLineCapBaseCap(customcap : *mut GpCustomLineCap, basecap : LineCap) -> Status);
    unsafe { GdipSetCustomLineCapBaseCap(customcap as _, basecap) }
}
#[inline]
pub unsafe fn GdipSetCustomLineCapBaseInset(customcap: *mut GpCustomLineCap, inset: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetCustomLineCapBaseInset(customcap : *mut GpCustomLineCap, inset : f32) -> Status);
    unsafe { GdipSetCustomLineCapBaseInset(customcap as _, inset) }
}
#[inline]
pub unsafe fn GdipSetCustomLineCapStrokeCaps(customcap: *mut GpCustomLineCap, startcap: LineCap, endcap: LineCap) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetCustomLineCapStrokeCaps(customcap : *mut GpCustomLineCap, startcap : LineCap, endcap : LineCap) -> Status);
    unsafe { GdipSetCustomLineCapStrokeCaps(customcap as _, startcap, endcap) }
}
#[inline]
pub unsafe fn GdipSetCustomLineCapStrokeJoin(customcap: *mut GpCustomLineCap, linejoin: LineJoin) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetCustomLineCapStrokeJoin(customcap : *mut GpCustomLineCap, linejoin : LineJoin) -> Status);
    unsafe { GdipSetCustomLineCapStrokeJoin(customcap as _, linejoin) }
}
#[inline]
pub unsafe fn GdipSetCustomLineCapWidthScale(customcap: *mut GpCustomLineCap, widthscale: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetCustomLineCapWidthScale(customcap : *mut GpCustomLineCap, widthscale : f32) -> Status);
    unsafe { GdipSetCustomLineCapWidthScale(customcap as _, widthscale) }
}
#[inline]
pub unsafe fn GdipSetEffectParameters(effect: *mut CGpEffect, params: *const core::ffi::c_void, size: u32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetEffectParameters(effect : *mut CGpEffect, params : *const core::ffi::c_void, size : u32) -> Status);
    unsafe { GdipSetEffectParameters(effect as _, params, size) }
}
#[inline]
pub unsafe fn GdipSetEmpty(region: *mut GpRegion) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetEmpty(region : *mut GpRegion) -> Status);
    unsafe { GdipSetEmpty(region as _) }
}
#[inline]
pub unsafe fn GdipSetImageAttributesCachedBackground(imageattr: *mut GpImageAttributes, enableflag: bool) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetImageAttributesCachedBackground(imageattr : *mut GpImageAttributes, enableflag : windows_core::BOOL) -> Status);
    unsafe { GdipSetImageAttributesCachedBackground(imageattr as _, enableflag.into()) }
}
#[inline]
pub unsafe fn GdipSetImageAttributesColorKeys(imageattr: *mut GpImageAttributes, r#type: ColorAdjustType, enableflag: bool, colorlow: u32, colorhigh: u32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetImageAttributesColorKeys(imageattr : *mut GpImageAttributes, r#type : ColorAdjustType, enableflag : windows_core::BOOL, colorlow : u32, colorhigh : u32) -> Status);
    unsafe { GdipSetImageAttributesColorKeys(imageattr as _, r#type, enableflag.into(), colorlow, colorhigh) }
}
#[inline]
pub unsafe fn GdipSetImageAttributesColorMatrix(imageattr: *mut GpImageAttributes, r#type: ColorAdjustType, enableflag: bool, colormatrix: *const ColorMatrix, graymatrix: *const ColorMatrix, flags: ColorMatrixFlags) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetImageAttributesColorMatrix(imageattr : *mut GpImageAttributes, r#type : ColorAdjustType, enableflag : windows_core::BOOL, colormatrix : *const ColorMatrix, graymatrix : *const ColorMatrix, flags : ColorMatrixFlags) -> Status);
    unsafe { GdipSetImageAttributesColorMatrix(imageattr as _, r#type, enableflag.into(), colormatrix, graymatrix, flags) }
}
#[inline]
pub unsafe fn GdipSetImageAttributesGamma(imageattr: *mut GpImageAttributes, r#type: ColorAdjustType, enableflag: bool, gamma: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetImageAttributesGamma(imageattr : *mut GpImageAttributes, r#type : ColorAdjustType, enableflag : windows_core::BOOL, gamma : f32) -> Status);
    unsafe { GdipSetImageAttributesGamma(imageattr as _, r#type, enableflag.into(), gamma) }
}
#[inline]
pub unsafe fn GdipSetImageAttributesNoOp(imageattr: *mut GpImageAttributes, r#type: ColorAdjustType, enableflag: bool) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetImageAttributesNoOp(imageattr : *mut GpImageAttributes, r#type : ColorAdjustType, enableflag : windows_core::BOOL) -> Status);
    unsafe { GdipSetImageAttributesNoOp(imageattr as _, r#type, enableflag.into()) }
}
#[inline]
pub unsafe fn GdipSetImageAttributesOutputChannel(imageattr: *mut GpImageAttributes, r#type: ColorAdjustType, enableflag: bool, channelflags: ColorChannelFlags) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetImageAttributesOutputChannel(imageattr : *mut GpImageAttributes, r#type : ColorAdjustType, enableflag : windows_core::BOOL, channelflags : ColorChannelFlags) -> Status);
    unsafe { GdipSetImageAttributesOutputChannel(imageattr as _, r#type, enableflag.into(), channelflags) }
}
#[inline]
pub unsafe fn GdipSetImageAttributesOutputChannelColorProfile<P3>(imageattr: *mut GpImageAttributes, r#type: ColorAdjustType, enableflag: bool, colorprofilefilename: P3) -> Status
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("gdiplus.dll" "system" fn GdipSetImageAttributesOutputChannelColorProfile(imageattr : *mut GpImageAttributes, r#type : ColorAdjustType, enableflag : windows_core::BOOL, colorprofilefilename : windows_core::PCWSTR) -> Status);
    unsafe { GdipSetImageAttributesOutputChannelColorProfile(imageattr as _, r#type, enableflag.into(), colorprofilefilename.param().abi()) }
}
#[inline]
pub unsafe fn GdipSetImageAttributesRemapTable(imageattr: *mut GpImageAttributes, r#type: ColorAdjustType, enableflag: bool, mapsize: u32, map: *const ColorMap) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetImageAttributesRemapTable(imageattr : *mut GpImageAttributes, r#type : ColorAdjustType, enableflag : windows_core::BOOL, mapsize : u32, map : *const ColorMap) -> Status);
    unsafe { GdipSetImageAttributesRemapTable(imageattr as _, r#type, enableflag.into(), mapsize, map) }
}
#[inline]
pub unsafe fn GdipSetImageAttributesThreshold(imageattr: *mut GpImageAttributes, r#type: ColorAdjustType, enableflag: bool, threshold: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetImageAttributesThreshold(imageattr : *mut GpImageAttributes, r#type : ColorAdjustType, enableflag : windows_core::BOOL, threshold : f32) -> Status);
    unsafe { GdipSetImageAttributesThreshold(imageattr as _, r#type, enableflag.into(), threshold) }
}
#[inline]
pub unsafe fn GdipSetImageAttributesToIdentity(imageattr: *mut GpImageAttributes, r#type: ColorAdjustType) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetImageAttributesToIdentity(imageattr : *mut GpImageAttributes, r#type : ColorAdjustType) -> Status);
    unsafe { GdipSetImageAttributesToIdentity(imageattr as _, r#type) }
}
#[inline]
pub unsafe fn GdipSetImageAttributesWrapMode(imageattr: *mut GpImageAttributes, wrap: WrapMode, argb: u32, clamp: bool) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetImageAttributesWrapMode(imageattr : *mut GpImageAttributes, wrap : WrapMode, argb : u32, clamp : windows_core::BOOL) -> Status);
    unsafe { GdipSetImageAttributesWrapMode(imageattr as _, wrap, argb, clamp.into()) }
}
#[inline]
pub unsafe fn GdipSetImagePalette(image: *mut GpImage, palette: *const ColorPalette) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetImagePalette(image : *mut GpImage, palette : *const ColorPalette) -> Status);
    unsafe { GdipSetImagePalette(image as _, palette) }
}
#[inline]
pub unsafe fn GdipSetInfinite(region: *mut GpRegion) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetInfinite(region : *mut GpRegion) -> Status);
    unsafe { GdipSetInfinite(region as _) }
}
#[inline]
pub unsafe fn GdipSetInterpolationMode(graphics: *mut GpGraphics, interpolationmode: InterpolationMode) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetInterpolationMode(graphics : *mut GpGraphics, interpolationmode : InterpolationMode) -> Status);
    unsafe { GdipSetInterpolationMode(graphics as _, interpolationmode) }
}
#[inline]
pub unsafe fn GdipSetLineBlend(brush: *mut GpLineGradient, blend: *const f32, positions: *const f32, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetLineBlend(brush : *mut GpLineGradient, blend : *const f32, positions : *const f32, count : i32) -> Status);
    unsafe { GdipSetLineBlend(brush as _, blend, positions, count) }
}
#[inline]
pub unsafe fn GdipSetLineColors(brush: *mut GpLineGradient, color1: u32, color2: u32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetLineColors(brush : *mut GpLineGradient, color1 : u32, color2 : u32) -> Status);
    unsafe { GdipSetLineColors(brush as _, color1, color2) }
}
#[inline]
pub unsafe fn GdipSetLineGammaCorrection(brush: *mut GpLineGradient, usegammacorrection: bool) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetLineGammaCorrection(brush : *mut GpLineGradient, usegammacorrection : windows_core::BOOL) -> Status);
    unsafe { GdipSetLineGammaCorrection(brush as _, usegammacorrection.into()) }
}
#[inline]
pub unsafe fn GdipSetLineLinearBlend(brush: *mut GpLineGradient, focus: f32, scale: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetLineLinearBlend(brush : *mut GpLineGradient, focus : f32, scale : f32) -> Status);
    unsafe { GdipSetLineLinearBlend(brush as _, focus, scale) }
}
#[inline]
pub unsafe fn GdipSetLinePresetBlend(brush: *mut GpLineGradient, blend: *const u32, positions: *const f32, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetLinePresetBlend(brush : *mut GpLineGradient, blend : *const u32, positions : *const f32, count : i32) -> Status);
    unsafe { GdipSetLinePresetBlend(brush as _, blend, positions, count) }
}
#[inline]
pub unsafe fn GdipSetLineSigmaBlend(brush: *mut GpLineGradient, focus: f32, scale: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetLineSigmaBlend(brush : *mut GpLineGradient, focus : f32, scale : f32) -> Status);
    unsafe { GdipSetLineSigmaBlend(brush as _, focus, scale) }
}
#[inline]
pub unsafe fn GdipSetLineTransform(brush: *mut GpLineGradient, matrix: *const Matrix) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetLineTransform(brush : *mut GpLineGradient, matrix : *const Matrix) -> Status);
    unsafe { GdipSetLineTransform(brush as _, matrix) }
}
#[inline]
pub unsafe fn GdipSetLineWrapMode(brush: *mut GpLineGradient, wrapmode: WrapMode) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetLineWrapMode(brush : *mut GpLineGradient, wrapmode : WrapMode) -> Status);
    unsafe { GdipSetLineWrapMode(brush as _, wrapmode) }
}
#[inline]
pub unsafe fn GdipSetMatrixElements(matrix: *mut Matrix, m11: f32, m12: f32, m21: f32, m22: f32, dx: f32, dy: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetMatrixElements(matrix : *mut Matrix, m11 : f32, m12 : f32, m21 : f32, m22 : f32, dx : f32, dy : f32) -> Status);
    unsafe { GdipSetMatrixElements(matrix as _, m11, m12, m21, m22, dx, dy) }
}
#[inline]
pub unsafe fn GdipSetMetafileDownLevelRasterizationLimit(metafile: *mut GpMetafile, metafilerasterizationlimitdpi: u32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetMetafileDownLevelRasterizationLimit(metafile : *mut GpMetafile, metafilerasterizationlimitdpi : u32) -> Status);
    unsafe { GdipSetMetafileDownLevelRasterizationLimit(metafile as _, metafilerasterizationlimitdpi) }
}
#[inline]
pub unsafe fn GdipSetPageScale(graphics: *mut GpGraphics, scale: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPageScale(graphics : *mut GpGraphics, scale : f32) -> Status);
    unsafe { GdipSetPageScale(graphics as _, scale) }
}
#[inline]
pub unsafe fn GdipSetPageUnit(graphics: *mut GpGraphics, unit: Unit) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPageUnit(graphics : *mut GpGraphics, unit : Unit) -> Status);
    unsafe { GdipSetPageUnit(graphics as _, unit) }
}
#[inline]
pub unsafe fn GdipSetPathFillMode(path: *mut GpPath, fillmode: FillMode) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPathFillMode(path : *mut GpPath, fillmode : FillMode) -> Status);
    unsafe { GdipSetPathFillMode(path as _, fillmode) }
}
#[inline]
pub unsafe fn GdipSetPathGradientBlend(brush: *mut GpPathGradient, blend: *const f32, positions: *const f32, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPathGradientBlend(brush : *mut GpPathGradient, blend : *const f32, positions : *const f32, count : i32) -> Status);
    unsafe { GdipSetPathGradientBlend(brush as _, blend, positions, count) }
}
#[inline]
pub unsafe fn GdipSetPathGradientCenterColor(brush: *mut GpPathGradient, colors: u32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPathGradientCenterColor(brush : *mut GpPathGradient, colors : u32) -> Status);
    unsafe { GdipSetPathGradientCenterColor(brush as _, colors) }
}
#[inline]
pub unsafe fn GdipSetPathGradientCenterPoint(brush: *mut GpPathGradient, points: *const PointF) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPathGradientCenterPoint(brush : *mut GpPathGradient, points : *const PointF) -> Status);
    unsafe { GdipSetPathGradientCenterPoint(brush as _, points) }
}
#[inline]
pub unsafe fn GdipSetPathGradientCenterPointI(brush: *mut GpPathGradient, points: *const Point) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPathGradientCenterPointI(brush : *mut GpPathGradient, points : *const Point) -> Status);
    unsafe { GdipSetPathGradientCenterPointI(brush as _, points) }
}
#[inline]
pub unsafe fn GdipSetPathGradientFocusScales(brush: *mut GpPathGradient, xscale: f32, yscale: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPathGradientFocusScales(brush : *mut GpPathGradient, xscale : f32, yscale : f32) -> Status);
    unsafe { GdipSetPathGradientFocusScales(brush as _, xscale, yscale) }
}
#[inline]
pub unsafe fn GdipSetPathGradientGammaCorrection(brush: *mut GpPathGradient, usegammacorrection: bool) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPathGradientGammaCorrection(brush : *mut GpPathGradient, usegammacorrection : windows_core::BOOL) -> Status);
    unsafe { GdipSetPathGradientGammaCorrection(brush as _, usegammacorrection.into()) }
}
#[inline]
pub unsafe fn GdipSetPathGradientLinearBlend(brush: *mut GpPathGradient, focus: f32, scale: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPathGradientLinearBlend(brush : *mut GpPathGradient, focus : f32, scale : f32) -> Status);
    unsafe { GdipSetPathGradientLinearBlend(brush as _, focus, scale) }
}
#[inline]
pub unsafe fn GdipSetPathGradientPath(brush: *mut GpPathGradient, path: *const GpPath) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPathGradientPath(brush : *mut GpPathGradient, path : *const GpPath) -> Status);
    unsafe { GdipSetPathGradientPath(brush as _, path) }
}
#[inline]
pub unsafe fn GdipSetPathGradientPresetBlend(brush: *mut GpPathGradient, blend: *const u32, positions: *const f32, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPathGradientPresetBlend(brush : *mut GpPathGradient, blend : *const u32, positions : *const f32, count : i32) -> Status);
    unsafe { GdipSetPathGradientPresetBlend(brush as _, blend, positions, count) }
}
#[inline]
pub unsafe fn GdipSetPathGradientSigmaBlend(brush: *mut GpPathGradient, focus: f32, scale: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPathGradientSigmaBlend(brush : *mut GpPathGradient, focus : f32, scale : f32) -> Status);
    unsafe { GdipSetPathGradientSigmaBlend(brush as _, focus, scale) }
}
#[inline]
pub unsafe fn GdipSetPathGradientSurroundColorsWithCount(brush: *mut GpPathGradient, color: *const u32, count: *mut i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPathGradientSurroundColorsWithCount(brush : *mut GpPathGradient, color : *const u32, count : *mut i32) -> Status);
    unsafe { GdipSetPathGradientSurroundColorsWithCount(brush as _, color, count as _) }
}
#[inline]
pub unsafe fn GdipSetPathGradientTransform(brush: *mut GpPathGradient, matrix: *mut Matrix) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPathGradientTransform(brush : *mut GpPathGradient, matrix : *mut Matrix) -> Status);
    unsafe { GdipSetPathGradientTransform(brush as _, matrix as _) }
}
#[inline]
pub unsafe fn GdipSetPathGradientWrapMode(brush: *mut GpPathGradient, wrapmode: WrapMode) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPathGradientWrapMode(brush : *mut GpPathGradient, wrapmode : WrapMode) -> Status);
    unsafe { GdipSetPathGradientWrapMode(brush as _, wrapmode) }
}
#[inline]
pub unsafe fn GdipSetPathMarker(path: *mut GpPath) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPathMarker(path : *mut GpPath) -> Status);
    unsafe { GdipSetPathMarker(path as _) }
}
#[inline]
pub unsafe fn GdipSetPenBrushFill(pen: *mut GpPen, brush: *mut GpBrush) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPenBrushFill(pen : *mut GpPen, brush : *mut GpBrush) -> Status);
    unsafe { GdipSetPenBrushFill(pen as _, brush as _) }
}
#[inline]
pub unsafe fn GdipSetPenColor(pen: *mut GpPen, argb: u32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPenColor(pen : *mut GpPen, argb : u32) -> Status);
    unsafe { GdipSetPenColor(pen as _, argb) }
}
#[inline]
pub unsafe fn GdipSetPenCompoundArray(pen: *mut GpPen, dash: *const f32, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPenCompoundArray(pen : *mut GpPen, dash : *const f32, count : i32) -> Status);
    unsafe { GdipSetPenCompoundArray(pen as _, dash, count) }
}
#[inline]
pub unsafe fn GdipSetPenCustomEndCap(pen: *mut GpPen, customcap: *mut GpCustomLineCap) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPenCustomEndCap(pen : *mut GpPen, customcap : *mut GpCustomLineCap) -> Status);
    unsafe { GdipSetPenCustomEndCap(pen as _, customcap as _) }
}
#[inline]
pub unsafe fn GdipSetPenCustomStartCap(pen: *mut GpPen, customcap: *mut GpCustomLineCap) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPenCustomStartCap(pen : *mut GpPen, customcap : *mut GpCustomLineCap) -> Status);
    unsafe { GdipSetPenCustomStartCap(pen as _, customcap as _) }
}
#[inline]
pub unsafe fn GdipSetPenDashArray(pen: *mut GpPen, dash: *const f32, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPenDashArray(pen : *mut GpPen, dash : *const f32, count : i32) -> Status);
    unsafe { GdipSetPenDashArray(pen as _, dash, count) }
}
#[inline]
pub unsafe fn GdipSetPenDashCap197819(pen: *mut GpPen, dashcap: DashCap) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPenDashCap197819(pen : *mut GpPen, dashcap : DashCap) -> Status);
    unsafe { GdipSetPenDashCap197819(pen as _, dashcap) }
}
#[inline]
pub unsafe fn GdipSetPenDashOffset(pen: *mut GpPen, offset: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPenDashOffset(pen : *mut GpPen, offset : f32) -> Status);
    unsafe { GdipSetPenDashOffset(pen as _, offset) }
}
#[inline]
pub unsafe fn GdipSetPenDashStyle(pen: *mut GpPen, dashstyle: DashStyle) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPenDashStyle(pen : *mut GpPen, dashstyle : DashStyle) -> Status);
    unsafe { GdipSetPenDashStyle(pen as _, dashstyle) }
}
#[inline]
pub unsafe fn GdipSetPenEndCap(pen: *mut GpPen, endcap: LineCap) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPenEndCap(pen : *mut GpPen, endcap : LineCap) -> Status);
    unsafe { GdipSetPenEndCap(pen as _, endcap) }
}
#[inline]
pub unsafe fn GdipSetPenLineCap197819(pen: *mut GpPen, startcap: LineCap, endcap: LineCap, dashcap: DashCap) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPenLineCap197819(pen : *mut GpPen, startcap : LineCap, endcap : LineCap, dashcap : DashCap) -> Status);
    unsafe { GdipSetPenLineCap197819(pen as _, startcap, endcap, dashcap) }
}
#[inline]
pub unsafe fn GdipSetPenLineJoin(pen: *mut GpPen, linejoin: LineJoin) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPenLineJoin(pen : *mut GpPen, linejoin : LineJoin) -> Status);
    unsafe { GdipSetPenLineJoin(pen as _, linejoin) }
}
#[inline]
pub unsafe fn GdipSetPenMiterLimit(pen: *mut GpPen, miterlimit: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPenMiterLimit(pen : *mut GpPen, miterlimit : f32) -> Status);
    unsafe { GdipSetPenMiterLimit(pen as _, miterlimit) }
}
#[inline]
pub unsafe fn GdipSetPenMode(pen: *mut GpPen, penmode: PenAlignment) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPenMode(pen : *mut GpPen, penmode : PenAlignment) -> Status);
    unsafe { GdipSetPenMode(pen as _, penmode) }
}
#[inline]
pub unsafe fn GdipSetPenStartCap(pen: *mut GpPen, startcap: LineCap) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPenStartCap(pen : *mut GpPen, startcap : LineCap) -> Status);
    unsafe { GdipSetPenStartCap(pen as _, startcap) }
}
#[inline]
pub unsafe fn GdipSetPenTransform(pen: *mut GpPen, matrix: *mut Matrix) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPenTransform(pen : *mut GpPen, matrix : *mut Matrix) -> Status);
    unsafe { GdipSetPenTransform(pen as _, matrix as _) }
}
#[inline]
pub unsafe fn GdipSetPenUnit(pen: *mut GpPen, unit: Unit) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPenUnit(pen : *mut GpPen, unit : Unit) -> Status);
    unsafe { GdipSetPenUnit(pen as _, unit) }
}
#[inline]
pub unsafe fn GdipSetPenWidth(pen: *mut GpPen, width: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPenWidth(pen : *mut GpPen, width : f32) -> Status);
    unsafe { GdipSetPenWidth(pen as _, width) }
}
#[inline]
pub unsafe fn GdipSetPixelOffsetMode(graphics: *mut GpGraphics, pixeloffsetmode: PixelOffsetMode) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPixelOffsetMode(graphics : *mut GpGraphics, pixeloffsetmode : PixelOffsetMode) -> Status);
    unsafe { GdipSetPixelOffsetMode(graphics as _, pixeloffsetmode) }
}
#[inline]
pub unsafe fn GdipSetPropertyItem(image: *mut GpImage, item: *const PropertyItem) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetPropertyItem(image : *mut GpImage, item : *const PropertyItem) -> Status);
    unsafe { GdipSetPropertyItem(image as _, item) }
}
#[inline]
pub unsafe fn GdipSetRenderingOrigin(graphics: *mut GpGraphics, x: i32, y: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetRenderingOrigin(graphics : *mut GpGraphics, x : i32, y : i32) -> Status);
    unsafe { GdipSetRenderingOrigin(graphics as _, x, y) }
}
#[inline]
pub unsafe fn GdipSetSmoothingMode(graphics: *mut GpGraphics, smoothingmode: SmoothingMode) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetSmoothingMode(graphics : *mut GpGraphics, smoothingmode : SmoothingMode) -> Status);
    unsafe { GdipSetSmoothingMode(graphics as _, smoothingmode) }
}
#[inline]
pub unsafe fn GdipSetSolidFillColor(brush: *mut GpSolidFill, color: u32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetSolidFillColor(brush : *mut GpSolidFill, color : u32) -> Status);
    unsafe { GdipSetSolidFillColor(brush as _, color) }
}
#[inline]
pub unsafe fn GdipSetStringFormatAlign(format: *mut GpStringFormat, align: StringAlignment) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetStringFormatAlign(format : *mut GpStringFormat, align : StringAlignment) -> Status);
    unsafe { GdipSetStringFormatAlign(format as _, align) }
}
#[inline]
pub unsafe fn GdipSetStringFormatDigitSubstitution(format: *mut GpStringFormat, language: u16, substitute: StringDigitSubstitute) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetStringFormatDigitSubstitution(format : *mut GpStringFormat, language : u16, substitute : StringDigitSubstitute) -> Status);
    unsafe { GdipSetStringFormatDigitSubstitution(format as _, language, substitute) }
}
#[inline]
pub unsafe fn GdipSetStringFormatFlags(format: *mut GpStringFormat, flags: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetStringFormatFlags(format : *mut GpStringFormat, flags : i32) -> Status);
    unsafe { GdipSetStringFormatFlags(format as _, flags) }
}
#[inline]
pub unsafe fn GdipSetStringFormatHotkeyPrefix(format: *mut GpStringFormat, hotkeyprefix: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetStringFormatHotkeyPrefix(format : *mut GpStringFormat, hotkeyprefix : i32) -> Status);
    unsafe { GdipSetStringFormatHotkeyPrefix(format as _, hotkeyprefix) }
}
#[inline]
pub unsafe fn GdipSetStringFormatLineAlign(format: *mut GpStringFormat, align: StringAlignment) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetStringFormatLineAlign(format : *mut GpStringFormat, align : StringAlignment) -> Status);
    unsafe { GdipSetStringFormatLineAlign(format as _, align) }
}
#[inline]
pub unsafe fn GdipSetStringFormatMeasurableCharacterRanges(format: *mut GpStringFormat, rangecount: i32, ranges: *const CharacterRange) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetStringFormatMeasurableCharacterRanges(format : *mut GpStringFormat, rangecount : i32, ranges : *const CharacterRange) -> Status);
    unsafe { GdipSetStringFormatMeasurableCharacterRanges(format as _, rangecount, ranges) }
}
#[inline]
pub unsafe fn GdipSetStringFormatTabStops(format: *mut GpStringFormat, firsttaboffset: f32, count: i32, tabstops: *const f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetStringFormatTabStops(format : *mut GpStringFormat, firsttaboffset : f32, count : i32, tabstops : *const f32) -> Status);
    unsafe { GdipSetStringFormatTabStops(format as _, firsttaboffset, count, tabstops) }
}
#[inline]
pub unsafe fn GdipSetStringFormatTrimming(format: *mut GpStringFormat, trimming: StringTrimming) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetStringFormatTrimming(format : *mut GpStringFormat, trimming : StringTrimming) -> Status);
    unsafe { GdipSetStringFormatTrimming(format as _, trimming) }
}
#[inline]
pub unsafe fn GdipSetTextContrast(graphics: *mut GpGraphics, contrast: u32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetTextContrast(graphics : *mut GpGraphics, contrast : u32) -> Status);
    unsafe { GdipSetTextContrast(graphics as _, contrast) }
}
#[inline]
pub unsafe fn GdipSetTextRenderingHint(graphics: *mut GpGraphics, mode: TextRenderingHint) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetTextRenderingHint(graphics : *mut GpGraphics, mode : TextRenderingHint) -> Status);
    unsafe { GdipSetTextRenderingHint(graphics as _, mode) }
}
#[inline]
pub unsafe fn GdipSetTextureTransform(brush: *mut GpTexture, matrix: *const Matrix) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetTextureTransform(brush : *mut GpTexture, matrix : *const Matrix) -> Status);
    unsafe { GdipSetTextureTransform(brush as _, matrix) }
}
#[inline]
pub unsafe fn GdipSetTextureWrapMode(brush: *mut GpTexture, wrapmode: WrapMode) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetTextureWrapMode(brush : *mut GpTexture, wrapmode : WrapMode) -> Status);
    unsafe { GdipSetTextureWrapMode(brush as _, wrapmode) }
}
#[inline]
pub unsafe fn GdipSetWorldTransform(graphics: *mut GpGraphics, matrix: *mut Matrix) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipSetWorldTransform(graphics : *mut GpGraphics, matrix : *mut Matrix) -> Status);
    unsafe { GdipSetWorldTransform(graphics as _, matrix as _) }
}
#[inline]
pub unsafe fn GdipShearMatrix(matrix: *mut Matrix, shearx: f32, sheary: f32, order: MatrixOrder) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipShearMatrix(matrix : *mut Matrix, shearx : f32, sheary : f32, order : MatrixOrder) -> Status);
    unsafe { GdipShearMatrix(matrix as _, shearx, sheary, order) }
}
#[inline]
pub unsafe fn GdipStartPathFigure(path: *mut GpPath) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipStartPathFigure(path : *mut GpPath) -> Status);
    unsafe { GdipStartPathFigure(path as _) }
}
#[inline]
pub unsafe fn GdipStringFormatGetGenericDefault(format: *mut *mut GpStringFormat) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipStringFormatGetGenericDefault(format : *mut *mut GpStringFormat) -> Status);
    unsafe { GdipStringFormatGetGenericDefault(format as _) }
}
#[inline]
pub unsafe fn GdipStringFormatGetGenericTypographic(format: *mut *mut GpStringFormat) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipStringFormatGetGenericTypographic(format : *mut *mut GpStringFormat) -> Status);
    unsafe { GdipStringFormatGetGenericTypographic(format as _) }
}
#[inline]
pub unsafe fn GdipTestControl(control: GpTestControlEnum, param1: *mut core::ffi::c_void) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipTestControl(control : GpTestControlEnum, param1 : *mut core::ffi::c_void) -> Status);
    unsafe { GdipTestControl(control, param1 as _) }
}
#[inline]
pub unsafe fn GdipTransformMatrixPoints(matrix: *mut Matrix, pts: *mut PointF, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipTransformMatrixPoints(matrix : *mut Matrix, pts : *mut PointF, count : i32) -> Status);
    unsafe { GdipTransformMatrixPoints(matrix as _, pts as _, count) }
}
#[inline]
pub unsafe fn GdipTransformMatrixPointsI(matrix: *mut Matrix, pts: *mut Point, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipTransformMatrixPointsI(matrix : *mut Matrix, pts : *mut Point, count : i32) -> Status);
    unsafe { GdipTransformMatrixPointsI(matrix as _, pts as _, count) }
}
#[inline]
pub unsafe fn GdipTransformPath(path: *mut GpPath, matrix: *mut Matrix) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipTransformPath(path : *mut GpPath, matrix : *mut Matrix) -> Status);
    unsafe { GdipTransformPath(path as _, matrix as _) }
}
#[inline]
pub unsafe fn GdipTransformPoints(graphics: *mut GpGraphics, destspace: CoordinateSpace, srcspace: CoordinateSpace, points: *mut PointF, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipTransformPoints(graphics : *mut GpGraphics, destspace : CoordinateSpace, srcspace : CoordinateSpace, points : *mut PointF, count : i32) -> Status);
    unsafe { GdipTransformPoints(graphics as _, destspace, srcspace, points as _, count) }
}
#[inline]
pub unsafe fn GdipTransformPointsI(graphics: *mut GpGraphics, destspace: CoordinateSpace, srcspace: CoordinateSpace, points: *mut Point, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipTransformPointsI(graphics : *mut GpGraphics, destspace : CoordinateSpace, srcspace : CoordinateSpace, points : *mut Point, count : i32) -> Status);
    unsafe { GdipTransformPointsI(graphics as _, destspace, srcspace, points as _, count) }
}
#[inline]
pub unsafe fn GdipTransformRegion(region: *mut GpRegion, matrix: *mut Matrix) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipTransformRegion(region : *mut GpRegion, matrix : *mut Matrix) -> Status);
    unsafe { GdipTransformRegion(region as _, matrix as _) }
}
#[inline]
pub unsafe fn GdipTranslateClip(graphics: *mut GpGraphics, dx: f32, dy: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipTranslateClip(graphics : *mut GpGraphics, dx : f32, dy : f32) -> Status);
    unsafe { GdipTranslateClip(graphics as _, dx, dy) }
}
#[inline]
pub unsafe fn GdipTranslateClipI(graphics: *mut GpGraphics, dx: i32, dy: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipTranslateClipI(graphics : *mut GpGraphics, dx : i32, dy : i32) -> Status);
    unsafe { GdipTranslateClipI(graphics as _, dx, dy) }
}
#[inline]
pub unsafe fn GdipTranslateLineTransform(brush: *mut GpLineGradient, dx: f32, dy: f32, order: MatrixOrder) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipTranslateLineTransform(brush : *mut GpLineGradient, dx : f32, dy : f32, order : MatrixOrder) -> Status);
    unsafe { GdipTranslateLineTransform(brush as _, dx, dy, order) }
}
#[inline]
pub unsafe fn GdipTranslateMatrix(matrix: *mut Matrix, offsetx: f32, offsety: f32, order: MatrixOrder) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipTranslateMatrix(matrix : *mut Matrix, offsetx : f32, offsety : f32, order : MatrixOrder) -> Status);
    unsafe { GdipTranslateMatrix(matrix as _, offsetx, offsety, order) }
}
#[inline]
pub unsafe fn GdipTranslatePathGradientTransform(brush: *mut GpPathGradient, dx: f32, dy: f32, order: MatrixOrder) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipTranslatePathGradientTransform(brush : *mut GpPathGradient, dx : f32, dy : f32, order : MatrixOrder) -> Status);
    unsafe { GdipTranslatePathGradientTransform(brush as _, dx, dy, order) }
}
#[inline]
pub unsafe fn GdipTranslatePenTransform(pen: *mut GpPen, dx: f32, dy: f32, order: MatrixOrder) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipTranslatePenTransform(pen : *mut GpPen, dx : f32, dy : f32, order : MatrixOrder) -> Status);
    unsafe { GdipTranslatePenTransform(pen as _, dx, dy, order) }
}
#[inline]
pub unsafe fn GdipTranslateRegion(region: *mut GpRegion, dx: f32, dy: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipTranslateRegion(region : *mut GpRegion, dx : f32, dy : f32) -> Status);
    unsafe { GdipTranslateRegion(region as _, dx, dy) }
}
#[inline]
pub unsafe fn GdipTranslateRegionI(region: *mut GpRegion, dx: i32, dy: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipTranslateRegionI(region : *mut GpRegion, dx : i32, dy : i32) -> Status);
    unsafe { GdipTranslateRegionI(region as _, dx, dy) }
}
#[inline]
pub unsafe fn GdipTranslateTextureTransform(brush: *mut GpTexture, dx: f32, dy: f32, order: MatrixOrder) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipTranslateTextureTransform(brush : *mut GpTexture, dx : f32, dy : f32, order : MatrixOrder) -> Status);
    unsafe { GdipTranslateTextureTransform(brush as _, dx, dy, order) }
}
#[inline]
pub unsafe fn GdipTranslateWorldTransform(graphics: *mut GpGraphics, dx: f32, dy: f32, order: MatrixOrder) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipTranslateWorldTransform(graphics : *mut GpGraphics, dx : f32, dy : f32, order : MatrixOrder) -> Status);
    unsafe { GdipTranslateWorldTransform(graphics as _, dx, dy, order) }
}
#[inline]
pub unsafe fn GdipVectorTransformMatrixPoints(matrix: *mut Matrix, pts: *mut PointF, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipVectorTransformMatrixPoints(matrix : *mut Matrix, pts : *mut PointF, count : i32) -> Status);
    unsafe { GdipVectorTransformMatrixPoints(matrix as _, pts as _, count) }
}
#[inline]
pub unsafe fn GdipVectorTransformMatrixPointsI(matrix: *mut Matrix, pts: *mut Point, count: i32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipVectorTransformMatrixPointsI(matrix : *mut Matrix, pts : *mut Point, count : i32) -> Status);
    unsafe { GdipVectorTransformMatrixPointsI(matrix as _, pts as _, count) }
}
#[inline]
pub unsafe fn GdipWarpPath(path: *mut GpPath, matrix: *mut Matrix, points: *const PointF, count: i32, srcx: f32, srcy: f32, srcwidth: f32, srcheight: f32, warpmode: WarpMode, flatness: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipWarpPath(path : *mut GpPath, matrix : *mut Matrix, points : *const PointF, count : i32, srcx : f32, srcy : f32, srcwidth : f32, srcheight : f32, warpmode : WarpMode, flatness : f32) -> Status);
    unsafe { GdipWarpPath(path as _, matrix as _, points, count, srcx, srcy, srcwidth, srcheight, warpmode, flatness) }
}
#[inline]
pub unsafe fn GdipWidenPath(nativepath: *mut GpPath, pen: *mut GpPen, matrix: *mut Matrix, flatness: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipWidenPath(nativepath : *mut GpPath, pen : *mut GpPen, matrix : *mut Matrix, flatness : f32) -> Status);
    unsafe { GdipWidenPath(nativepath as _, pen as _, matrix as _, flatness) }
}
#[inline]
pub unsafe fn GdipWindingModeOutline(path: *mut GpPath, matrix: *mut Matrix, flatness: f32) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdipWindingModeOutline(path : *mut GpPath, matrix : *mut Matrix, flatness : f32) -> Status);
    unsafe { GdipWindingModeOutline(path as _, matrix as _, flatness) }
}
#[inline]
pub unsafe fn GdiplusNotificationHook(token: *mut usize) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdiplusNotificationHook(token : *mut usize) -> Status);
    unsafe { GdiplusNotificationHook(token as _) }
}
#[inline]
pub unsafe fn GdiplusNotificationUnhook(token: usize) {
    windows_core::link!("gdiplus.dll" "system" fn GdiplusNotificationUnhook(token : usize));
    unsafe { GdiplusNotificationUnhook(token) }
}
#[inline]
pub unsafe fn GdiplusShutdown(token: usize) {
    windows_core::link!("gdiplus.dll" "system" fn GdiplusShutdown(token : usize));
    unsafe { GdiplusShutdown(token) }
}
#[inline]
pub unsafe fn GdiplusStartup(token: *mut usize, input: *const GdiplusStartupInput, output: *mut GdiplusStartupOutput) -> Status {
    windows_core::link!("gdiplus.dll" "system" fn GdiplusStartup(token : *mut usize, input : *const GdiplusStartupInput, output : *mut GdiplusStartupOutput) -> Status);
    unsafe { GdiplusStartup(token as _, input, output as _) }
}
pub const ALPHA_SHIFT: u32 = 24u32;
pub const Aborted: Status = Status(9i32);
pub const AccessDenied: Status = Status(12i32);
pub const AdjustBlackSaturation: CurveAdjustments = CurveAdjustments(7i32);
pub const AdjustContrast: CurveAdjustments = CurveAdjustments(2i32);
pub const AdjustDensity: CurveAdjustments = CurveAdjustments(1i32);
pub const AdjustExposure: CurveAdjustments = CurveAdjustments(0i32);
pub const AdjustHighlight: CurveAdjustments = CurveAdjustments(3i32);
pub const AdjustMidtone: CurveAdjustments = CurveAdjustments(5i32);
pub const AdjustShadow: CurveAdjustments = CurveAdjustments(4i32);
pub const AdjustWhiteSaturation: CurveAdjustments = CurveAdjustments(6i32);
pub const BLUE_SHIFT: u32 = 0u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Bitmap(pub isize);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BitmapData {
    pub Width: u32,
    pub Height: u32,
    pub Stride: i32,
    pub PixelFormat: i32,
    pub Scan0: *mut core::ffi::c_void,
    pub Reserved: usize,
}
impl Default for BitmapData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Blur {
    pub Base: Effect,
}
pub const BlurEffectGuid: windows_core::GUID = windows_core::GUID::from_u128(0x633c80a4_1843_482b_9ef2_be2834c5fdd4);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BlurParams {
    pub radius: f32,
    pub expandEdge: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BrightnessContrast {
    pub Base: Effect,
}
pub const BrightnessContrastEffectGuid: windows_core::GUID = windows_core::GUID::from_u128(0xd3a1dbe1_8ec4_4c17_9f4c_ea97ad1c343d);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BrightnessContrastParams {
    pub brightnessLevel: i32,
    pub contrastLevel: i32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BrushType(pub i32);
pub const BrushTypeHatchFill: BrushType = BrushType(1i32);
pub const BrushTypeLinearGradient: BrushType = BrushType(4i32);
pub const BrushTypePathGradient: BrushType = BrushType(3i32);
pub const BrushTypeSolidColor: BrushType = BrushType(0i32);
pub const BrushTypeTextureFill: BrushType = BrushType(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct CGpEffect(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct CachedBitmap(pub isize);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CharacterRange {
    pub First: i32,
    pub Length: i32,
}
pub const CodecIImageBytes: windows_core::GUID = windows_core::GUID::from_u128(0x025d1823_6c7d_447b_bbdb_a3cbc3dfa2fc);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Color {
    pub Argb: u32,
}
impl Color {
    pub const AliceBlue: i32 = -984833i32;
    pub const AntiqueWhite: i32 = -332841i32;
    pub const Aqua: i32 = -16711681i32;
    pub const Aquamarine: i32 = -8388652i32;
    pub const Azure: i32 = -983041i32;
    pub const Beige: i32 = -657956i32;
    pub const Bisque: i32 = -6972i32;
    pub const Black: i32 = -16777216i32;
    pub const BlanchedAlmond: i32 = -5171i32;
    pub const Blue: i32 = -16776961i32;
    pub const BlueViolet: i32 = -7722014i32;
    pub const Brown: i32 = -5952982i32;
    pub const BurlyWood: i32 = -2180985i32;
    pub const CadetBlue: i32 = -10510688i32;
    pub const Chartreuse: i32 = -8388864i32;
    pub const Chocolate: i32 = -2987746i32;
    pub const Coral: i32 = -32944i32;
    pub const CornflowerBlue: i32 = -10185235i32;
    pub const Cornsilk: i32 = -1828i32;
    pub const Crimson: i32 = -2354116i32;
    pub const Cyan: i32 = -16711681i32;
    pub const DarkBlue: i32 = -16777077i32;
    pub const DarkCyan: i32 = -16741493i32;
    pub const DarkGoldenrod: i32 = -4684277i32;
    pub const DarkGray: i32 = -5658199i32;
    pub const DarkGreen: i32 = -16751616i32;
    pub const DarkKhaki: i32 = -4343957i32;
    pub const DarkMagenta: i32 = -7667573i32;
    pub const DarkOliveGreen: i32 = -11179217i32;
    pub const DarkOrange: i32 = -29696i32;
    pub const DarkOrchid: i32 = -6737204i32;
    pub const DarkRed: i32 = -7667712i32;
    pub const DarkSalmon: i32 = -1468806i32;
    pub const DarkSeaGreen: i32 = -7357301i32;
    pub const DarkSlateBlue: i32 = -12042869i32;
    pub const DarkSlateGray: i32 = -13676721i32;
    pub const DarkTurquoise: i32 = -16724271i32;
    pub const DarkViolet: i32 = -7077677i32;
    pub const DeepPink: i32 = -60269i32;
    pub const DeepSkyBlue: i32 = -16728065i32;
    pub const DimGray: i32 = -9868951i32;
    pub const DodgerBlue: i32 = -14774017i32;
    pub const Firebrick: i32 = -5103070i32;
    pub const FloralWhite: i32 = -1296i32;
    pub const ForestGreen: i32 = -14513374i32;
    pub const Fuchsia: i32 = -65281i32;
    pub const Gainsboro: i32 = -2302756i32;
    pub const GhostWhite: i32 = -460545i32;
    pub const Gold: i32 = -10496i32;
    pub const Goldenrod: i32 = -2448096i32;
    pub const Gray: i32 = -8355712i32;
    pub const Green: i32 = -16744448i32;
    pub const GreenYellow: i32 = -5374161i32;
    pub const Honeydew: i32 = -983056i32;
    pub const HotPink: i32 = -38476i32;
    pub const IndianRed: i32 = -3318692i32;
    pub const Indigo: i32 = -11861886i32;
    pub const Ivory: i32 = -16i32;
    pub const Khaki: i32 = -989556i32;
    pub const Lavender: i32 = -1644806i32;
    pub const LavenderBlush: i32 = -3851i32;
    pub const LawnGreen: i32 = -8586240i32;
    pub const LemonChiffon: i32 = -1331i32;
    pub const LightBlue: i32 = -5383962i32;
    pub const LightCoral: i32 = -1015680i32;
    pub const LightCyan: i32 = -2031617i32;
    pub const LightGoldenrodYellow: i32 = -329006i32;
    pub const LightGray: i32 = -2894893i32;
    pub const LightGreen: i32 = -7278960i32;
    pub const LightPink: i32 = -18751i32;
    pub const LightSalmon: i32 = -24454i32;
    pub const LightSeaGreen: i32 = -14634326i32;
    pub const LightSkyBlue: i32 = -7876870i32;
    pub const LightSlateGray: i32 = -8943463i32;
    pub const LightSteelBlue: i32 = -5192482i32;
    pub const LightYellow: i32 = -32i32;
    pub const Lime: i32 = -16711936i32;
    pub const LimeGreen: i32 = -13447886i32;
    pub const Linen: i32 = -331546i32;
    pub const Magenta: i32 = -65281i32;
    pub const Maroon: i32 = -8388608i32;
    pub const MediumAquamarine: i32 = -10039894i32;
    pub const MediumBlue: i32 = -16777011i32;
    pub const MediumOrchid: i32 = -4565549i32;
    pub const MediumPurple: i32 = -7114533i32;
    pub const MediumSeaGreen: i32 = -12799119i32;
    pub const MediumSlateBlue: i32 = -8689426i32;
    pub const MediumSpringGreen: i32 = -16713062i32;
    pub const MediumTurquoise: i32 = -12004916i32;
    pub const MediumVioletRed: i32 = -3730043i32;
    pub const MidnightBlue: i32 = -15132304i32;
    pub const MintCream: i32 = -655366i32;
    pub const MistyRose: i32 = -6943i32;
    pub const Moccasin: i32 = -6987i32;
    pub const NavajoWhite: i32 = -8531i32;
    pub const Navy: i32 = -16777088i32;
    pub const OldLace: i32 = -133658i32;
    pub const Olive: i32 = -8355840i32;
    pub const OliveDrab: i32 = -9728477i32;
    pub const Orange: i32 = -23296i32;
    pub const OrangeRed: i32 = -47872i32;
    pub const Orchid: i32 = -2461482i32;
    pub const PaleGoldenrod: i32 = -1120086i32;
    pub const PaleGreen: i32 = -6751336i32;
    pub const PaleTurquoise: i32 = -5247250i32;
    pub const PaleVioletRed: i32 = -2396013i32;
    pub const PapayaWhip: i32 = -4139i32;
    pub const PeachPuff: i32 = -9543i32;
    pub const Peru: i32 = -3308225i32;
    pub const Pink: i32 = -16181i32;
    pub const Plum: i32 = -2252579i32;
    pub const PowderBlue: i32 = -5185306i32;
    pub const Purple: i32 = -8388480i32;
    pub const Red: i32 = -65536i32;
    pub const RosyBrown: i32 = -4419697i32;
    pub const RoyalBlue: i32 = -12490271i32;
    pub const SaddleBrown: i32 = -7650029i32;
    pub const Salmon: i32 = -360334i32;
    pub const SandyBrown: i32 = -744352i32;
    pub const SeaGreen: i32 = -13726889i32;
    pub const SeaShell: i32 = -2578i32;
    pub const Sienna: i32 = -6270419i32;
    pub const Silver: i32 = -4144960i32;
    pub const SkyBlue: i32 = -7876885i32;
    pub const SlateBlue: i32 = -9807155i32;
    pub const SlateGray: i32 = -9404272i32;
    pub const Snow: i32 = -1286i32;
    pub const SpringGreen: i32 = -16711809i32;
    pub const SteelBlue: i32 = -12156236i32;
    pub const Tan: i32 = -2968436i32;
    pub const Teal: i32 = -16744320i32;
    pub const Thistle: i32 = -2572328i32;
    pub const Tomato: i32 = -40121i32;
    pub const Transparent: i32 = 16777215i32;
    pub const Turquoise: i32 = -12525360i32;
    pub const Violet: i32 = -1146130i32;
    pub const Wheat: i32 = -663885i32;
    pub const White: i32 = -1i32;
    pub const WhiteSmoke: i32 = -657931i32;
    pub const Yellow: i32 = -256i32;
    pub const YellowGreen: i32 = -6632142i32;
    pub const AlphaShift: i32 = 24i32;
    pub const RedShift: i32 = 16i32;
    pub const GreenShift: i32 = 8i32;
    pub const BlueShift: i32 = 0i32;
    pub const AlphaMask: i32 = -16777216i32;
    pub const RedMask: i32 = 16711680i32;
    pub const GreenMask: i32 = 65280i32;
    pub const BlueMask: i32 = 255i32;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ColorAdjustType(pub i32);
pub const ColorAdjustTypeAny: ColorAdjustType = ColorAdjustType(6i32);
pub const ColorAdjustTypeBitmap: ColorAdjustType = ColorAdjustType(1i32);
pub const ColorAdjustTypeBrush: ColorAdjustType = ColorAdjustType(2i32);
pub const ColorAdjustTypeCount: ColorAdjustType = ColorAdjustType(5i32);
pub const ColorAdjustTypeDefault: ColorAdjustType = ColorAdjustType(0i32);
pub const ColorAdjustTypePen: ColorAdjustType = ColorAdjustType(3i32);
pub const ColorAdjustTypeText: ColorAdjustType = ColorAdjustType(4i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ColorBalance {
    pub Base: Effect,
}
pub const ColorBalanceEffectGuid: windows_core::GUID = windows_core::GUID::from_u128(0x537e597d_251e_48da_9664_29ca496b70f8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ColorBalanceParams {
    pub cyanRed: i32,
    pub magentaGreen: i32,
    pub yellowBlue: i32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ColorChannelFlags(pub i32);
pub const ColorChannelFlagsC: ColorChannelFlags = ColorChannelFlags(0i32);
pub const ColorChannelFlagsK: ColorChannelFlags = ColorChannelFlags(3i32);
pub const ColorChannelFlagsLast: ColorChannelFlags = ColorChannelFlags(4i32);
pub const ColorChannelFlagsM: ColorChannelFlags = ColorChannelFlags(1i32);
pub const ColorChannelFlagsY: ColorChannelFlags = ColorChannelFlags(2i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ColorCurve {
    pub Base: Effect,
}
pub const ColorCurveEffectGuid: windows_core::GUID = windows_core::GUID::from_u128(0xdd6a0022_58e4_4a67_9d9b_d48eb881a53d);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ColorCurveParams {
    pub adjustment: CurveAdjustments,
    pub channel: CurveChannel,
    pub adjustValue: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ColorLUT {
    pub Base: Effect,
}
pub const ColorLUTEffectGuid: windows_core::GUID = windows_core::GUID::from_u128(0xa7ce72a9_0f7f_40d7_b3cc_d0c02d5c3212);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorLUTParams {
    pub lutB: [u8; 256],
    pub lutG: [u8; 256],
    pub lutR: [u8; 256],
    pub lutA: [u8; 256],
}
impl Default for ColorLUTParams {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ColorMap {
    pub oldColor: Color,
    pub newColor: Color,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorMatrix {
    pub m: [f32; 25],
}
impl Default for ColorMatrix {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ColorMatrixEffect {
    pub Base: Effect,
}
pub const ColorMatrixEffectGuid: windows_core::GUID = windows_core::GUID::from_u128(0x718f2615_7933_40e3_a511_5f68fe14dd74);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ColorMatrixFlags(pub i32);
pub const ColorMatrixFlagsAltGray: ColorMatrixFlags = ColorMatrixFlags(2i32);
pub const ColorMatrixFlagsDefault: ColorMatrixFlags = ColorMatrixFlags(0i32);
pub const ColorMatrixFlagsSkipGrays: ColorMatrixFlags = ColorMatrixFlags(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ColorMode(pub i32);
pub const ColorModeARGB32: ColorMode = ColorMode(0i32);
pub const ColorModeARGB64: ColorMode = ColorMode(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorPalette {
    pub Flags: u32,
    pub Count: u32,
    pub Entries: [u32; 1],
}
impl Default for ColorPalette {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CombineMode(pub i32);
pub const CombineModeComplement: CombineMode = CombineMode(5i32);
pub const CombineModeExclude: CombineMode = CombineMode(4i32);
pub const CombineModeIntersect: CombineMode = CombineMode(1i32);
pub const CombineModeReplace: CombineMode = CombineMode(0i32);
pub const CombineModeUnion: CombineMode = CombineMode(2i32);
pub const CombineModeXor: CombineMode = CombineMode(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CompositingMode(pub i32);
pub const CompositingModeSourceCopy: CompositingMode = CompositingMode(1i32);
pub const CompositingModeSourceOver: CompositingMode = CompositingMode(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CompositingQuality(pub i32);
pub const CompositingQualityAssumeLinear: CompositingQuality = CompositingQuality(4i32);
pub const CompositingQualityDefault: CompositingQuality = CompositingQuality(0i32);
pub const CompositingQualityGammaCorrected: CompositingQuality = CompositingQuality(3i32);
pub const CompositingQualityHighQuality: CompositingQuality = CompositingQuality(2i32);
pub const CompositingQualityHighSpeed: CompositingQuality = CompositingQuality(1i32);
pub const CompositingQualityInvalid: CompositingQuality = CompositingQuality(-1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ConvertToEmfPlusFlags(pub i32);
pub const ConvertToEmfPlusFlagsDefault: ConvertToEmfPlusFlags = ConvertToEmfPlusFlags(0i32);
pub const ConvertToEmfPlusFlagsInvalidRecord: ConvertToEmfPlusFlags = ConvertToEmfPlusFlags(4i32);
pub const ConvertToEmfPlusFlagsRopUsed: ConvertToEmfPlusFlags = ConvertToEmfPlusFlags(1i32);
pub const ConvertToEmfPlusFlagsText: ConvertToEmfPlusFlags = ConvertToEmfPlusFlags(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CoordinateSpace(pub i32);
pub const CoordinateSpaceDevice: CoordinateSpace = CoordinateSpace(2i32);
pub const CoordinateSpacePage: CoordinateSpace = CoordinateSpace(1i32);
pub const CoordinateSpaceWorld: CoordinateSpace = CoordinateSpace(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CurveAdjustments(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CurveChannel(pub i32);
pub const CurveChannelAll: CurveChannel = CurveChannel(0i32);
pub const CurveChannelBlue: CurveChannel = CurveChannel(3i32);
pub const CurveChannelGreen: CurveChannel = CurveChannel(2i32);
pub const CurveChannelRed: CurveChannel = CurveChannel(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct CustomLineCap(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CustomLineCapType(pub i32);
pub const CustomLineCapTypeAdjustableArrow: CustomLineCapType = CustomLineCapType(1i32);
pub const CustomLineCapTypeDefault: CustomLineCapType = CustomLineCapType(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DashCap(pub i32);
pub const DashCapFlat: DashCap = DashCap(0i32);
pub const DashCapRound: DashCap = DashCap(2i32);
pub const DashCapTriangle: DashCap = DashCap(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DashStyle(pub i32);
pub const DashStyleCustom: DashStyle = DashStyle(5i32);
pub const DashStyleDash: DashStyle = DashStyle(1i32);
pub const DashStyleDashDot: DashStyle = DashStyle(3i32);
pub const DashStyleDashDotDot: DashStyle = DashStyle(4i32);
pub const DashStyleDot: DashStyle = DashStyle(2i32);
pub const DashStyleSolid: DashStyle = DashStyle(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DebugEventLevel(pub i32);
pub const DebugEventLevelFatal: DebugEventLevel = DebugEventLevel(0i32);
pub const DebugEventLevelWarning: DebugEventLevel = DebugEventLevel(1i32);
pub type DebugEventProc = Option<unsafe extern "system" fn(level: DebugEventLevel, message: windows_core::PCSTR)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DitherType(pub i32);
pub const DitherTypeDualSpiral4x4: DitherType = DitherType(7i32);
pub const DitherTypeDualSpiral8x8: DitherType = DitherType(8i32);
pub const DitherTypeErrorDiffusion: DitherType = DitherType(9i32);
pub const DitherTypeMax: DitherType = DitherType(10i32);
pub const DitherTypeNone: DitherType = DitherType(0i32);
pub const DitherTypeOrdered16x16: DitherType = DitherType(4i32);
pub const DitherTypeOrdered4x4: DitherType = DitherType(2i32);
pub const DitherTypeOrdered8x8: DitherType = DitherType(3i32);
pub const DitherTypeSolid: DitherType = DitherType(1i32);
pub const DitherTypeSpiral4x4: DitherType = DitherType(5i32);
pub const DitherTypeSpiral8x8: DitherType = DitherType(6i32);
pub type DrawImageAbort = Option<unsafe extern "system" fn() -> windows_core::BOOL>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DriverStringOptions(pub i32);
pub const DriverStringOptionsCmapLookup: DriverStringOptions = DriverStringOptions(1i32);
pub const DriverStringOptionsLimitSubpixel: DriverStringOptions = DriverStringOptions(8i32);
pub const DriverStringOptionsRealizedAdvance: DriverStringOptions = DriverStringOptions(4i32);
pub const DriverStringOptionsVertical: DriverStringOptions = DriverStringOptions(2i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ENHMETAHEADER3 {
    pub iType: u32,
    pub nSize: u32,
    pub rclBounds: super::super::Foundation::RECTL,
    pub rclFrame: super::super::Foundation::RECTL,
    pub dSignature: u32,
    pub nVersion: u32,
    pub nBytes: u32,
    pub nRecords: u32,
    pub nHandles: u16,
    pub sReserved: u16,
    pub nDescription: u32,
    pub offDescription: u32,
    pub nPalEntries: u32,
    pub szlDevice: super::super::Foundation::SIZE,
    pub szlMillimeters: super::super::Foundation::SIZE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Effect {
    pub lpVtbl: *mut *mut core::ffi::c_void,
    pub nativeEffect: *mut CGpEffect,
    pub auxDataSize: i32,
    pub auxData: *mut core::ffi::c_void,
    pub useAuxData: windows_core::BOOL,
}
impl Default for Effect {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EmfPlusRecordTotal: EmfPlusRecordType = EmfPlusRecordType(16443i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EmfPlusRecordType(pub i32);
pub const EmfPlusRecordTypeBeginContainer: EmfPlusRecordType = EmfPlusRecordType(16423i32);
pub const EmfPlusRecordTypeBeginContainerNoParams: EmfPlusRecordType = EmfPlusRecordType(16424i32);
pub const EmfPlusRecordTypeClear: EmfPlusRecordType = EmfPlusRecordType(16393i32);
pub const EmfPlusRecordTypeComment: EmfPlusRecordType = EmfPlusRecordType(16387i32);
pub const EmfPlusRecordTypeDrawArc: EmfPlusRecordType = EmfPlusRecordType(16402i32);
pub const EmfPlusRecordTypeDrawBeziers: EmfPlusRecordType = EmfPlusRecordType(16409i32);
pub const EmfPlusRecordTypeDrawClosedCurve: EmfPlusRecordType = EmfPlusRecordType(16407i32);
pub const EmfPlusRecordTypeDrawCurve: EmfPlusRecordType = EmfPlusRecordType(16408i32);
pub const EmfPlusRecordTypeDrawDriverString: EmfPlusRecordType = EmfPlusRecordType(16438i32);
pub const EmfPlusRecordTypeDrawEllipse: EmfPlusRecordType = EmfPlusRecordType(16399i32);
pub const EmfPlusRecordTypeDrawImage: EmfPlusRecordType = EmfPlusRecordType(16410i32);
pub const EmfPlusRecordTypeDrawImagePoints: EmfPlusRecordType = EmfPlusRecordType(16411i32);
pub const EmfPlusRecordTypeDrawLines: EmfPlusRecordType = EmfPlusRecordType(16397i32);
pub const EmfPlusRecordTypeDrawPath: EmfPlusRecordType = EmfPlusRecordType(16405i32);
pub const EmfPlusRecordTypeDrawPie: EmfPlusRecordType = EmfPlusRecordType(16401i32);
pub const EmfPlusRecordTypeDrawRects: EmfPlusRecordType = EmfPlusRecordType(16395i32);
pub const EmfPlusRecordTypeDrawString: EmfPlusRecordType = EmfPlusRecordType(16412i32);
pub const EmfPlusRecordTypeEndContainer: EmfPlusRecordType = EmfPlusRecordType(16425i32);
pub const EmfPlusRecordTypeEndOfFile: EmfPlusRecordType = EmfPlusRecordType(16386i32);
pub const EmfPlusRecordTypeFillClosedCurve: EmfPlusRecordType = EmfPlusRecordType(16406i32);
pub const EmfPlusRecordTypeFillEllipse: EmfPlusRecordType = EmfPlusRecordType(16398i32);
pub const EmfPlusRecordTypeFillPath: EmfPlusRecordType = EmfPlusRecordType(16404i32);
pub const EmfPlusRecordTypeFillPie: EmfPlusRecordType = EmfPlusRecordType(16400i32);
pub const EmfPlusRecordTypeFillPolygon: EmfPlusRecordType = EmfPlusRecordType(16396i32);
pub const EmfPlusRecordTypeFillRects: EmfPlusRecordType = EmfPlusRecordType(16394i32);
pub const EmfPlusRecordTypeFillRegion: EmfPlusRecordType = EmfPlusRecordType(16403i32);
pub const EmfPlusRecordTypeGetDC: EmfPlusRecordType = EmfPlusRecordType(16388i32);
pub const EmfPlusRecordTypeHeader: EmfPlusRecordType = EmfPlusRecordType(16385i32);
pub const EmfPlusRecordTypeInvalid: EmfPlusRecordType = EmfPlusRecordType(16384i32);
pub const EmfPlusRecordTypeMax: EmfPlusRecordType = EmfPlusRecordType(16442i32);
pub const EmfPlusRecordTypeMin: EmfPlusRecordType = EmfPlusRecordType(16385i32);
pub const EmfPlusRecordTypeMultiFormatEnd: EmfPlusRecordType = EmfPlusRecordType(16391i32);
pub const EmfPlusRecordTypeMultiFormatSection: EmfPlusRecordType = EmfPlusRecordType(16390i32);
pub const EmfPlusRecordTypeMultiFormatStart: EmfPlusRecordType = EmfPlusRecordType(16389i32);
pub const EmfPlusRecordTypeMultiplyWorldTransform: EmfPlusRecordType = EmfPlusRecordType(16428i32);
pub const EmfPlusRecordTypeObject: EmfPlusRecordType = EmfPlusRecordType(16392i32);
pub const EmfPlusRecordTypeOffsetClip: EmfPlusRecordType = EmfPlusRecordType(16437i32);
pub const EmfPlusRecordTypeResetClip: EmfPlusRecordType = EmfPlusRecordType(16433i32);
pub const EmfPlusRecordTypeResetWorldTransform: EmfPlusRecordType = EmfPlusRecordType(16427i32);
pub const EmfPlusRecordTypeRestore: EmfPlusRecordType = EmfPlusRecordType(16422i32);
pub const EmfPlusRecordTypeRotateWorldTransform: EmfPlusRecordType = EmfPlusRecordType(16431i32);
pub const EmfPlusRecordTypeSave: EmfPlusRecordType = EmfPlusRecordType(16421i32);
pub const EmfPlusRecordTypeScaleWorldTransform: EmfPlusRecordType = EmfPlusRecordType(16430i32);
pub const EmfPlusRecordTypeSerializableObject: EmfPlusRecordType = EmfPlusRecordType(16440i32);
pub const EmfPlusRecordTypeSetAntiAliasMode: EmfPlusRecordType = EmfPlusRecordType(16414i32);
pub const EmfPlusRecordTypeSetClipPath: EmfPlusRecordType = EmfPlusRecordType(16435i32);
pub const EmfPlusRecordTypeSetClipRect: EmfPlusRecordType = EmfPlusRecordType(16434i32);
pub const EmfPlusRecordTypeSetClipRegion: EmfPlusRecordType = EmfPlusRecordType(16436i32);
pub const EmfPlusRecordTypeSetCompositingMode: EmfPlusRecordType = EmfPlusRecordType(16419i32);
pub const EmfPlusRecordTypeSetCompositingQuality: EmfPlusRecordType = EmfPlusRecordType(16420i32);
pub const EmfPlusRecordTypeSetInterpolationMode: EmfPlusRecordType = EmfPlusRecordType(16417i32);
pub const EmfPlusRecordTypeSetPageTransform: EmfPlusRecordType = EmfPlusRecordType(16432i32);
pub const EmfPlusRecordTypeSetPixelOffsetMode: EmfPlusRecordType = EmfPlusRecordType(16418i32);
pub const EmfPlusRecordTypeSetRenderingOrigin: EmfPlusRecordType = EmfPlusRecordType(16413i32);
pub const EmfPlusRecordTypeSetTSClip: EmfPlusRecordType = EmfPlusRecordType(16442i32);
pub const EmfPlusRecordTypeSetTSGraphics: EmfPlusRecordType = EmfPlusRecordType(16441i32);
pub const EmfPlusRecordTypeSetTextContrast: EmfPlusRecordType = EmfPlusRecordType(16416i32);
pub const EmfPlusRecordTypeSetTextRenderingHint: EmfPlusRecordType = EmfPlusRecordType(16415i32);
pub const EmfPlusRecordTypeSetWorldTransform: EmfPlusRecordType = EmfPlusRecordType(16426i32);
pub const EmfPlusRecordTypeStrokeFillPath: EmfPlusRecordType = EmfPlusRecordType(16439i32);
pub const EmfPlusRecordTypeTranslateWorldTransform: EmfPlusRecordType = EmfPlusRecordType(16429i32);
pub const EmfRecordTypeAbortPath: EmfPlusRecordType = EmfPlusRecordType(68i32);
pub const EmfRecordTypeAlphaBlend: EmfPlusRecordType = EmfPlusRecordType(114i32);
pub const EmfRecordTypeAngleArc: EmfPlusRecordType = EmfPlusRecordType(41i32);
pub const EmfRecordTypeArc: EmfPlusRecordType = EmfPlusRecordType(45i32);
pub const EmfRecordTypeArcTo: EmfPlusRecordType = EmfPlusRecordType(55i32);
pub const EmfRecordTypeBeginPath: EmfPlusRecordType = EmfPlusRecordType(59i32);
pub const EmfRecordTypeBitBlt: EmfPlusRecordType = EmfPlusRecordType(76i32);
pub const EmfRecordTypeChord: EmfPlusRecordType = EmfPlusRecordType(46i32);
pub const EmfRecordTypeCloseFigure: EmfPlusRecordType = EmfPlusRecordType(61i32);
pub const EmfRecordTypeColorCorrectPalette: EmfPlusRecordType = EmfPlusRecordType(111i32);
pub const EmfRecordTypeColorMatchToTargetW: EmfPlusRecordType = EmfPlusRecordType(121i32);
pub const EmfRecordTypeCreateBrushIndirect: EmfPlusRecordType = EmfPlusRecordType(39i32);
pub const EmfRecordTypeCreateColorSpace: EmfPlusRecordType = EmfPlusRecordType(99i32);
pub const EmfRecordTypeCreateColorSpaceW: EmfPlusRecordType = EmfPlusRecordType(122i32);
pub const EmfRecordTypeCreateDIBPatternBrushPt: EmfPlusRecordType = EmfPlusRecordType(94i32);
pub const EmfRecordTypeCreateMonoBrush: EmfPlusRecordType = EmfPlusRecordType(93i32);
pub const EmfRecordTypeCreatePalette: EmfPlusRecordType = EmfPlusRecordType(49i32);
pub const EmfRecordTypeCreatePen: EmfPlusRecordType = EmfPlusRecordType(38i32);
pub const EmfRecordTypeDeleteColorSpace: EmfPlusRecordType = EmfPlusRecordType(101i32);
pub const EmfRecordTypeDeleteObject: EmfPlusRecordType = EmfPlusRecordType(40i32);
pub const EmfRecordTypeDrawEscape: EmfPlusRecordType = EmfPlusRecordType(105i32);
pub const EmfRecordTypeEOF: EmfPlusRecordType = EmfPlusRecordType(14i32);
pub const EmfRecordTypeEllipse: EmfPlusRecordType = EmfPlusRecordType(42i32);
pub const EmfRecordTypeEndPath: EmfPlusRecordType = EmfPlusRecordType(60i32);
pub const EmfRecordTypeExcludeClipRect: EmfPlusRecordType = EmfPlusRecordType(29i32);
pub const EmfRecordTypeExtCreateFontIndirect: EmfPlusRecordType = EmfPlusRecordType(82i32);
pub const EmfRecordTypeExtCreatePen: EmfPlusRecordType = EmfPlusRecordType(95i32);
pub const EmfRecordTypeExtEscape: EmfPlusRecordType = EmfPlusRecordType(106i32);
pub const EmfRecordTypeExtFloodFill: EmfPlusRecordType = EmfPlusRecordType(53i32);
pub const EmfRecordTypeExtSelectClipRgn: EmfPlusRecordType = EmfPlusRecordType(75i32);
pub const EmfRecordTypeExtTextOutA: EmfPlusRecordType = EmfPlusRecordType(83i32);
pub const EmfRecordTypeExtTextOutW: EmfPlusRecordType = EmfPlusRecordType(84i32);
pub const EmfRecordTypeFillPath: EmfPlusRecordType = EmfPlusRecordType(62i32);
pub const EmfRecordTypeFillRgn: EmfPlusRecordType = EmfPlusRecordType(71i32);
pub const EmfRecordTypeFlattenPath: EmfPlusRecordType = EmfPlusRecordType(65i32);
pub const EmfRecordTypeForceUFIMapping: EmfPlusRecordType = EmfPlusRecordType(109i32);
pub const EmfRecordTypeFrameRgn: EmfPlusRecordType = EmfPlusRecordType(72i32);
pub const EmfRecordTypeGLSBoundedRecord: EmfPlusRecordType = EmfPlusRecordType(103i32);
pub const EmfRecordTypeGLSRecord: EmfPlusRecordType = EmfPlusRecordType(102i32);
pub const EmfRecordTypeGdiComment: EmfPlusRecordType = EmfPlusRecordType(70i32);
pub const EmfRecordTypeGradientFill: EmfPlusRecordType = EmfPlusRecordType(118i32);
pub const EmfRecordTypeHeader: EmfPlusRecordType = EmfPlusRecordType(1i32);
pub const EmfRecordTypeIntersectClipRect: EmfPlusRecordType = EmfPlusRecordType(30i32);
pub const EmfRecordTypeInvertRgn: EmfPlusRecordType = EmfPlusRecordType(73i32);
pub const EmfRecordTypeLineTo: EmfPlusRecordType = EmfPlusRecordType(54i32);
pub const EmfRecordTypeMaskBlt: EmfPlusRecordType = EmfPlusRecordType(78i32);
pub const EmfRecordTypeMax: EmfPlusRecordType = EmfPlusRecordType(122i32);
pub const EmfRecordTypeMin: EmfPlusRecordType = EmfPlusRecordType(1i32);
pub const EmfRecordTypeModifyWorldTransform: EmfPlusRecordType = EmfPlusRecordType(36i32);
pub const EmfRecordTypeMoveToEx: EmfPlusRecordType = EmfPlusRecordType(27i32);
pub const EmfRecordTypeNamedEscape: EmfPlusRecordType = EmfPlusRecordType(110i32);
pub const EmfRecordTypeOffsetClipRgn: EmfPlusRecordType = EmfPlusRecordType(26i32);
pub const EmfRecordTypePaintRgn: EmfPlusRecordType = EmfPlusRecordType(74i32);
pub const EmfRecordTypePie: EmfPlusRecordType = EmfPlusRecordType(47i32);
pub const EmfRecordTypePixelFormat: EmfPlusRecordType = EmfPlusRecordType(104i32);
pub const EmfRecordTypePlgBlt: EmfPlusRecordType = EmfPlusRecordType(79i32);
pub const EmfRecordTypePolyBezier: EmfPlusRecordType = EmfPlusRecordType(2i32);
pub const EmfRecordTypePolyBezier16: EmfPlusRecordType = EmfPlusRecordType(85i32);
pub const EmfRecordTypePolyBezierTo: EmfPlusRecordType = EmfPlusRecordType(5i32);
pub const EmfRecordTypePolyBezierTo16: EmfPlusRecordType = EmfPlusRecordType(88i32);
pub const EmfRecordTypePolyDraw: EmfPlusRecordType = EmfPlusRecordType(56i32);
pub const EmfRecordTypePolyDraw16: EmfPlusRecordType = EmfPlusRecordType(92i32);
pub const EmfRecordTypePolyLineTo: EmfPlusRecordType = EmfPlusRecordType(6i32);
pub const EmfRecordTypePolyPolygon: EmfPlusRecordType = EmfPlusRecordType(8i32);
pub const EmfRecordTypePolyPolygon16: EmfPlusRecordType = EmfPlusRecordType(91i32);
pub const EmfRecordTypePolyPolyline: EmfPlusRecordType = EmfPlusRecordType(7i32);
pub const EmfRecordTypePolyPolyline16: EmfPlusRecordType = EmfPlusRecordType(90i32);
pub const EmfRecordTypePolyTextOutA: EmfPlusRecordType = EmfPlusRecordType(96i32);
pub const EmfRecordTypePolyTextOutW: EmfPlusRecordType = EmfPlusRecordType(97i32);
pub const EmfRecordTypePolygon: EmfPlusRecordType = EmfPlusRecordType(3i32);
pub const EmfRecordTypePolygon16: EmfPlusRecordType = EmfPlusRecordType(86i32);
pub const EmfRecordTypePolyline: EmfPlusRecordType = EmfPlusRecordType(4i32);
pub const EmfRecordTypePolyline16: EmfPlusRecordType = EmfPlusRecordType(87i32);
pub const EmfRecordTypePolylineTo16: EmfPlusRecordType = EmfPlusRecordType(89i32);
pub const EmfRecordTypeRealizePalette: EmfPlusRecordType = EmfPlusRecordType(52i32);
pub const EmfRecordTypeRectangle: EmfPlusRecordType = EmfPlusRecordType(43i32);
pub const EmfRecordTypeReserved_069: EmfPlusRecordType = EmfPlusRecordType(69i32);
pub const EmfRecordTypeReserved_117: EmfPlusRecordType = EmfPlusRecordType(117i32);
pub const EmfRecordTypeResizePalette: EmfPlusRecordType = EmfPlusRecordType(51i32);
pub const EmfRecordTypeRestoreDC: EmfPlusRecordType = EmfPlusRecordType(34i32);
pub const EmfRecordTypeRoundRect: EmfPlusRecordType = EmfPlusRecordType(44i32);
pub const EmfRecordTypeSaveDC: EmfPlusRecordType = EmfPlusRecordType(33i32);
pub const EmfRecordTypeScaleViewportExtEx: EmfPlusRecordType = EmfPlusRecordType(31i32);
pub const EmfRecordTypeScaleWindowExtEx: EmfPlusRecordType = EmfPlusRecordType(32i32);
pub const EmfRecordTypeSelectClipPath: EmfPlusRecordType = EmfPlusRecordType(67i32);
pub const EmfRecordTypeSelectObject: EmfPlusRecordType = EmfPlusRecordType(37i32);
pub const EmfRecordTypeSelectPalette: EmfPlusRecordType = EmfPlusRecordType(48i32);
pub const EmfRecordTypeSetArcDirection: EmfPlusRecordType = EmfPlusRecordType(57i32);
pub const EmfRecordTypeSetBkColor: EmfPlusRecordType = EmfPlusRecordType(25i32);
pub const EmfRecordTypeSetBkMode: EmfPlusRecordType = EmfPlusRecordType(18i32);
pub const EmfRecordTypeSetBrushOrgEx: EmfPlusRecordType = EmfPlusRecordType(13i32);
pub const EmfRecordTypeSetColorAdjustment: EmfPlusRecordType = EmfPlusRecordType(23i32);
pub const EmfRecordTypeSetColorSpace: EmfPlusRecordType = EmfPlusRecordType(100i32);
pub const EmfRecordTypeSetDIBitsToDevice: EmfPlusRecordType = EmfPlusRecordType(80i32);
pub const EmfRecordTypeSetICMMode: EmfPlusRecordType = EmfPlusRecordType(98i32);
pub const EmfRecordTypeSetICMProfileA: EmfPlusRecordType = EmfPlusRecordType(112i32);
pub const EmfRecordTypeSetICMProfileW: EmfPlusRecordType = EmfPlusRecordType(113i32);
pub const EmfRecordTypeSetLayout: EmfPlusRecordType = EmfPlusRecordType(115i32);
pub const EmfRecordTypeSetLinkedUFIs: EmfPlusRecordType = EmfPlusRecordType(119i32);
pub const EmfRecordTypeSetMapMode: EmfPlusRecordType = EmfPlusRecordType(17i32);
pub const EmfRecordTypeSetMapperFlags: EmfPlusRecordType = EmfPlusRecordType(16i32);
pub const EmfRecordTypeSetMetaRgn: EmfPlusRecordType = EmfPlusRecordType(28i32);
pub const EmfRecordTypeSetMiterLimit: EmfPlusRecordType = EmfPlusRecordType(58i32);
pub const EmfRecordTypeSetPaletteEntries: EmfPlusRecordType = EmfPlusRecordType(50i32);
pub const EmfRecordTypeSetPixelV: EmfPlusRecordType = EmfPlusRecordType(15i32);
pub const EmfRecordTypeSetPolyFillMode: EmfPlusRecordType = EmfPlusRecordType(19i32);
pub const EmfRecordTypeSetROP2: EmfPlusRecordType = EmfPlusRecordType(20i32);
pub const EmfRecordTypeSetStretchBltMode: EmfPlusRecordType = EmfPlusRecordType(21i32);
pub const EmfRecordTypeSetTextAlign: EmfPlusRecordType = EmfPlusRecordType(22i32);
pub const EmfRecordTypeSetTextColor: EmfPlusRecordType = EmfPlusRecordType(24i32);
pub const EmfRecordTypeSetTextJustification: EmfPlusRecordType = EmfPlusRecordType(120i32);
pub const EmfRecordTypeSetViewportExtEx: EmfPlusRecordType = EmfPlusRecordType(11i32);
pub const EmfRecordTypeSetViewportOrgEx: EmfPlusRecordType = EmfPlusRecordType(12i32);
pub const EmfRecordTypeSetWindowExtEx: EmfPlusRecordType = EmfPlusRecordType(9i32);
pub const EmfRecordTypeSetWindowOrgEx: EmfPlusRecordType = EmfPlusRecordType(10i32);
pub const EmfRecordTypeSetWorldTransform: EmfPlusRecordType = EmfPlusRecordType(35i32);
pub const EmfRecordTypeSmallTextOut: EmfPlusRecordType = EmfPlusRecordType(108i32);
pub const EmfRecordTypeStartDoc: EmfPlusRecordType = EmfPlusRecordType(107i32);
pub const EmfRecordTypeStretchBlt: EmfPlusRecordType = EmfPlusRecordType(77i32);
pub const EmfRecordTypeStretchDIBits: EmfPlusRecordType = EmfPlusRecordType(81i32);
pub const EmfRecordTypeStrokeAndFillPath: EmfPlusRecordType = EmfPlusRecordType(63i32);
pub const EmfRecordTypeStrokePath: EmfPlusRecordType = EmfPlusRecordType(64i32);
pub const EmfRecordTypeTransparentBlt: EmfPlusRecordType = EmfPlusRecordType(116i32);
pub const EmfRecordTypeWidenPath: EmfPlusRecordType = EmfPlusRecordType(66i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EmfToWmfBitsFlags(pub i32);
pub const EmfToWmfBitsFlagsDefault: EmfToWmfBitsFlags = EmfToWmfBitsFlags(0i32);
pub const EmfToWmfBitsFlagsEmbedEmf: EmfToWmfBitsFlags = EmfToWmfBitsFlags(1i32);
pub const EmfToWmfBitsFlagsIncludePlaceable: EmfToWmfBitsFlags = EmfToWmfBitsFlags(2i32);
pub const EmfToWmfBitsFlagsNoXORClip: EmfToWmfBitsFlags = EmfToWmfBitsFlags(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EmfType(pub i32);
pub const EmfTypeEmfOnly: EmfType = EmfType(3i32);
pub const EmfTypeEmfPlusDual: EmfType = EmfType(5i32);
pub const EmfTypeEmfPlusOnly: EmfType = EmfType(4i32);
pub const EncoderChrominanceTable: windows_core::GUID = windows_core::GUID::from_u128(0xf2e455dc_09b3_4316_8260_676ada32481c);
pub const EncoderColorDepth: windows_core::GUID = windows_core::GUID::from_u128(0x66087055_ad66_4c7c_9a18_38a2310b8337);
pub const EncoderColorSpace: windows_core::GUID = windows_core::GUID::from_u128(0xae7a62a0_ee2c_49d8_9d07_1ba8a927596e);
pub const EncoderCompression: windows_core::GUID = windows_core::GUID::from_u128(0xe09d739d_ccd4_44ee_8eba_3fbf8be4fc58);
pub const EncoderImageItems: windows_core::GUID = windows_core::GUID::from_u128(0x63875e13_1f1d_45ab_9195_a29b6066a650);
pub const EncoderLuminanceTable: windows_core::GUID = windows_core::GUID::from_u128(0xedb33bce_0266_4a77_b904_27216099e717);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EncoderParameter {
    pub Guid: windows_core::GUID,
    pub NumberOfValues: u32,
    pub Type: u32,
    pub Value: *mut core::ffi::c_void,
}
impl Default for EncoderParameter {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EncoderParameterValueType(pub i32);
pub const EncoderParameterValueTypeASCII: EncoderParameterValueType = EncoderParameterValueType(2i32);
pub const EncoderParameterValueTypeByte: EncoderParameterValueType = EncoderParameterValueType(1i32);
pub const EncoderParameterValueTypeLong: EncoderParameterValueType = EncoderParameterValueType(4i32);
pub const EncoderParameterValueTypeLongRange: EncoderParameterValueType = EncoderParameterValueType(6i32);
pub const EncoderParameterValueTypePointer: EncoderParameterValueType = EncoderParameterValueType(9i32);
pub const EncoderParameterValueTypeRational: EncoderParameterValueType = EncoderParameterValueType(5i32);
pub const EncoderParameterValueTypeRationalRange: EncoderParameterValueType = EncoderParameterValueType(8i32);
pub const EncoderParameterValueTypeShort: EncoderParameterValueType = EncoderParameterValueType(3i32);
pub const EncoderParameterValueTypeUndefined: EncoderParameterValueType = EncoderParameterValueType(7i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EncoderParameters {
    pub Count: u32,
    pub Parameter: [EncoderParameter; 1],
}
impl Default for EncoderParameters {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EncoderQuality: windows_core::GUID = windows_core::GUID::from_u128(0x1d5be4b5_fa4a_452d_9cdd_5db35105e7eb);
pub const EncoderRenderMethod: windows_core::GUID = windows_core::GUID::from_u128(0x6d42c53a_229a_4825_8bb7_5c99e2b9a8b8);
pub const EncoderSaveAsCMYK: windows_core::GUID = windows_core::GUID::from_u128(0xa219bbc9_0a9d_4005_a3ee_3a421b8bb06c);
pub const EncoderSaveFlag: windows_core::GUID = windows_core::GUID::from_u128(0x292266fc_ac40_47bf_8cfc_a85b89a655de);
pub const EncoderScanMethod: windows_core::GUID = windows_core::GUID::from_u128(0x3a4e2661_3109_4e56_8536_42c156e7dcfa);
pub const EncoderTransformation: windows_core::GUID = windows_core::GUID::from_u128(0x8d0eb2d1_a58e_4ea8_aa14_108074b7b6f9);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EncoderValue(pub i32);
pub const EncoderValueColorTypeCMYK: EncoderValue = EncoderValue(0i32);
pub const EncoderValueColorTypeGray: EncoderValue = EncoderValue(24i32);
pub const EncoderValueColorTypeRGB: EncoderValue = EncoderValue(25i32);
pub const EncoderValueColorTypeYCCK: EncoderValue = EncoderValue(1i32);
pub const EncoderValueCompressionCCITT3: EncoderValue = EncoderValue(3i32);
pub const EncoderValueCompressionCCITT4: EncoderValue = EncoderValue(4i32);
pub const EncoderValueCompressionLZW: EncoderValue = EncoderValue(2i32);
pub const EncoderValueCompressionNone: EncoderValue = EncoderValue(6i32);
pub const EncoderValueCompressionRle: EncoderValue = EncoderValue(5i32);
pub const EncoderValueFlush: EncoderValue = EncoderValue(20i32);
pub const EncoderValueFrameDimensionPage: EncoderValue = EncoderValue(23i32);
pub const EncoderValueFrameDimensionResolution: EncoderValue = EncoderValue(22i32);
pub const EncoderValueFrameDimensionTime: EncoderValue = EncoderValue(21i32);
pub const EncoderValueLastFrame: EncoderValue = EncoderValue(19i32);
pub const EncoderValueMultiFrame: EncoderValue = EncoderValue(18i32);
pub const EncoderValueRenderNonProgressive: EncoderValue = EncoderValue(12i32);
pub const EncoderValueRenderProgressive: EncoderValue = EncoderValue(11i32);
pub const EncoderValueScanMethodInterlaced: EncoderValue = EncoderValue(7i32);
pub const EncoderValueScanMethodNonInterlaced: EncoderValue = EncoderValue(8i32);
pub const EncoderValueTransformFlipHorizontal: EncoderValue = EncoderValue(16i32);
pub const EncoderValueTransformFlipVertical: EncoderValue = EncoderValue(17i32);
pub const EncoderValueTransformRotate180: EncoderValue = EncoderValue(14i32);
pub const EncoderValueTransformRotate270: EncoderValue = EncoderValue(15i32);
pub const EncoderValueTransformRotate90: EncoderValue = EncoderValue(13i32);
pub const EncoderValueVersionGif87: EncoderValue = EncoderValue(9i32);
pub const EncoderValueVersionGif89: EncoderValue = EncoderValue(10i32);
pub const EncoderVersion: windows_core::GUID = windows_core::GUID::from_u128(0x24d18c76_814a_41a4_bf53_1c219cccf797);
pub type EnumerateMetafileProc = Option<unsafe extern "system" fn(param0: EmfPlusRecordType, param1: u32, param2: u32, param3: *const u8, param4: *mut core::ffi::c_void) -> windows_core::BOOL>;
pub const FileNotFound: Status = Status(10i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FillMode(pub i32);
pub const FillModeAlternate: FillMode = FillMode(0i32);
pub const FillModeWinding: FillMode = FillMode(1i32);
pub const FlatnessDefault: f32 = 0.25f32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FlushIntention(pub i32);
pub const FlushIntentionFlush: FlushIntention = FlushIntention(0i32);
pub const FlushIntentionSync: FlushIntention = FlushIntention(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Font(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct FontCollection(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct FontFamily(pub isize);
pub const FontFamilyNotFound: Status = Status(14i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FontStyle(pub i32);
pub const FontStyleBold: FontStyle = FontStyle(1i32);
pub const FontStyleBoldItalic: FontStyle = FontStyle(3i32);
pub const FontStyleItalic: FontStyle = FontStyle(2i32);
pub const FontStyleNotFound: Status = Status(15i32);
pub const FontStyleRegular: FontStyle = FontStyle(0i32);
pub const FontStyleStrikeout: FontStyle = FontStyle(8i32);
pub const FontStyleUnderline: FontStyle = FontStyle(4i32);
pub const FormatIDImageInformation: windows_core::GUID = windows_core::GUID::from_u128(0xe5836cbe_5eef_4f1d_acde_ae4c43b608ce);
pub const FormatIDJpegAppHeaders: windows_core::GUID = windows_core::GUID::from_u128(0x1c4afdcd_6177_43cf_abc7_5f51af39ee85);
pub const FrameDimensionPage: windows_core::GUID = windows_core::GUID::from_u128(0x7462dc86_6180_4c7e_8e3f_ee7333a7a483);
pub const FrameDimensionResolution: windows_core::GUID = windows_core::GUID::from_u128(0x84236f7b_3bd3_428f_8dab_4ea1439ca315);
pub const FrameDimensionTime: windows_core::GUID = windows_core::GUID::from_u128(0x6aedbd6d_3fb5_418a_83a6_7f45229dc872);
pub const GDIP_EMFPLUSFLAGS_DISPLAY: u32 = 1u32;
pub const GDIP_EMFPLUS_RECORD_BASE: u32 = 16384u32;
pub const GDIP_WMF_RECORD_BASE: u32 = 65536u32;
pub const GREEN_SHIFT: u32 = 8u32;
windows_core::imp::define_interface!(GdiplusAbort, GdiplusAbort_Vtbl);
impl GdiplusAbort {
    pub unsafe fn Abort(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Abort)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct GdiplusAbort_Vtbl {
    pub Abort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait GdiplusAbort_Impl {
    fn Abort(&self) -> windows_core::Result<()>;
}
impl GdiplusAbort_Vtbl {
    pub const fn new<Identity: GdiplusAbort_Impl>() -> Self {
        unsafe extern "system" fn Abort<Identity: GdiplusAbort_Impl>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                GdiplusAbort_Impl::Abort(this).into()
            }
        }
        Self { Abort: Abort::<Identity> }
    }
}
struct GdiplusAbort_ImplVtbl<T: GdiplusAbort_Impl>(core::marker::PhantomData<T>);
impl<T: GdiplusAbort_Impl> GdiplusAbort_ImplVtbl<T> {
    const VTABLE: GdiplusAbort_Vtbl = GdiplusAbort_Vtbl::new::<T>();
}
impl GdiplusAbort {
    pub fn new<'a, T: GdiplusAbort_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &GdiplusAbort_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::Box::new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
pub const GdiplusNotInitialized: Status = Status(18i32);
pub const GdiplusStartupDefault: GdiplusStartupParams = GdiplusStartupParams(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GdiplusStartupInput {
    pub GdiplusVersion: u32,
    pub DebugEventCallback: isize,
    pub SuppressBackgroundThread: windows_core::BOOL,
    pub SuppressExternalCodecs: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GdiplusStartupInputEx {
    pub Base: GdiplusStartupInput,
    pub StartupParameters: i32,
}
pub const GdiplusStartupNoSetRound: GdiplusStartupParams = GdiplusStartupParams(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GdiplusStartupOutput {
    pub NotificationHook: isize,
    pub NotificationUnhook: isize,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GdiplusStartupParams(pub i32);
pub const GdiplusStartupSetPSValue: GdiplusStartupParams = GdiplusStartupParams(2i32);
pub const GdiplusStartupTransparencyMask: GdiplusStartupParams = GdiplusStartupParams(-16777216i32);
pub const GenericError: Status = Status(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GenericFontFamily(pub i32);
pub const GenericFontFamilyMonospace: GenericFontFamily = GenericFontFamily(2i32);
pub const GenericFontFamilySansSerif: GenericFontFamily = GenericFontFamily(1i32);
pub const GenericFontFamilySerif: GenericFontFamily = GenericFontFamily(0i32);
pub type GetThumbnailImageAbort = Option<unsafe extern "system" fn() -> windows_core::BOOL>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GpAdjustableArrowCap(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GpBitmap(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GpBrush(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GpCachedBitmap(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GpCustomLineCap(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GpFont(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GpFontCollection(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GpFontFamily(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GpGraphics(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GpHatch(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GpImage(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GpImageAttributes(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GpInstalledFontCollection(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GpLineGradient(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GpMetafile(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GpPath(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GpPathGradient(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GpPathIterator(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GpPen(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GpPrivateFontCollection(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GpRegion(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GpSolidFill(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GpStringFormat(pub u8);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GpTestControlEnum(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GpTexture(pub u8);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HatchStyle(pub i32);
pub const HatchStyle05Percent: HatchStyle = HatchStyle(6i32);
pub const HatchStyle10Percent: HatchStyle = HatchStyle(7i32);
pub const HatchStyle20Percent: HatchStyle = HatchStyle(8i32);
pub const HatchStyle25Percent: HatchStyle = HatchStyle(9i32);
pub const HatchStyle30Percent: HatchStyle = HatchStyle(10i32);
pub const HatchStyle40Percent: HatchStyle = HatchStyle(11i32);
pub const HatchStyle50Percent: HatchStyle = HatchStyle(12i32);
pub const HatchStyle60Percent: HatchStyle = HatchStyle(13i32);
pub const HatchStyle70Percent: HatchStyle = HatchStyle(14i32);
pub const HatchStyle75Percent: HatchStyle = HatchStyle(15i32);
pub const HatchStyle80Percent: HatchStyle = HatchStyle(16i32);
pub const HatchStyle90Percent: HatchStyle = HatchStyle(17i32);
pub const HatchStyleBackwardDiagonal: HatchStyle = HatchStyle(3i32);
pub const HatchStyleCross: HatchStyle = HatchStyle(4i32);
pub const HatchStyleDarkDownwardDiagonal: HatchStyle = HatchStyle(20i32);
pub const HatchStyleDarkHorizontal: HatchStyle = HatchStyle(29i32);
pub const HatchStyleDarkUpwardDiagonal: HatchStyle = HatchStyle(21i32);
pub const HatchStyleDarkVertical: HatchStyle = HatchStyle(28i32);
pub const HatchStyleDashedDownwardDiagonal: HatchStyle = HatchStyle(30i32);
pub const HatchStyleDashedHorizontal: HatchStyle = HatchStyle(32i32);
pub const HatchStyleDashedUpwardDiagonal: HatchStyle = HatchStyle(31i32);
pub const HatchStyleDashedVertical: HatchStyle = HatchStyle(33i32);
pub const HatchStyleDiagonalBrick: HatchStyle = HatchStyle(38i32);
pub const HatchStyleDiagonalCross: HatchStyle = HatchStyle(5i32);
pub const HatchStyleDivot: HatchStyle = HatchStyle(42i32);
pub const HatchStyleDottedDiamond: HatchStyle = HatchStyle(44i32);
pub const HatchStyleDottedGrid: HatchStyle = HatchStyle(43i32);
pub const HatchStyleForwardDiagonal: HatchStyle = HatchStyle(2i32);
pub const HatchStyleHorizontal: HatchStyle = HatchStyle(0i32);
pub const HatchStyleHorizontalBrick: HatchStyle = HatchStyle(39i32);
pub const HatchStyleLargeCheckerBoard: HatchStyle = HatchStyle(50i32);
pub const HatchStyleLargeConfetti: HatchStyle = HatchStyle(35i32);
pub const HatchStyleLargeGrid: HatchStyle = HatchStyle(4i32);
pub const HatchStyleLightDownwardDiagonal: HatchStyle = HatchStyle(18i32);
pub const HatchStyleLightHorizontal: HatchStyle = HatchStyle(25i32);
pub const HatchStyleLightUpwardDiagonal: HatchStyle = HatchStyle(19i32);
pub const HatchStyleLightVertical: HatchStyle = HatchStyle(24i32);
pub const HatchStyleMax: HatchStyle = HatchStyle(52i32);
pub const HatchStyleMin: HatchStyle = HatchStyle(0i32);
pub const HatchStyleNarrowHorizontal: HatchStyle = HatchStyle(27i32);
pub const HatchStyleNarrowVertical: HatchStyle = HatchStyle(26i32);
pub const HatchStyleOutlinedDiamond: HatchStyle = HatchStyle(51i32);
pub const HatchStylePlaid: HatchStyle = HatchStyle(41i32);
pub const HatchStyleShingle: HatchStyle = HatchStyle(45i32);
pub const HatchStyleSmallCheckerBoard: HatchStyle = HatchStyle(49i32);
pub const HatchStyleSmallConfetti: HatchStyle = HatchStyle(34i32);
pub const HatchStyleSmallGrid: HatchStyle = HatchStyle(48i32);
pub const HatchStyleSolidDiamond: HatchStyle = HatchStyle(52i32);
pub const HatchStyleSphere: HatchStyle = HatchStyle(47i32);
pub const HatchStyleTotal: HatchStyle = HatchStyle(53i32);
pub const HatchStyleTrellis: HatchStyle = HatchStyle(46i32);
pub const HatchStyleVertical: HatchStyle = HatchStyle(1i32);
pub const HatchStyleWave: HatchStyle = HatchStyle(37i32);
pub const HatchStyleWeave: HatchStyle = HatchStyle(40i32);
pub const HatchStyleWideDownwardDiagonal: HatchStyle = HatchStyle(22i32);
pub const HatchStyleWideUpwardDiagonal: HatchStyle = HatchStyle(23i32);
pub const HatchStyleZigZag: HatchStyle = HatchStyle(36i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HistogramFormat(pub i32);
pub const HistogramFormatA: HistogramFormat = HistogramFormat(7i32);
pub const HistogramFormatARGB: HistogramFormat = HistogramFormat(0i32);
pub const HistogramFormatB: HistogramFormat = HistogramFormat(4i32);
pub const HistogramFormatG: HistogramFormat = HistogramFormat(5i32);
pub const HistogramFormatGray: HistogramFormat = HistogramFormat(3i32);
pub const HistogramFormatPARGB: HistogramFormat = HistogramFormat(1i32);
pub const HistogramFormatR: HistogramFormat = HistogramFormat(6i32);
pub const HistogramFormatRGB: HistogramFormat = HistogramFormat(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HotkeyPrefix(pub i32);
pub const HotkeyPrefixHide: HotkeyPrefix = HotkeyPrefix(2i32);
pub const HotkeyPrefixNone: HotkeyPrefix = HotkeyPrefix(0i32);
pub const HotkeyPrefixShow: HotkeyPrefix = HotkeyPrefix(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HueSaturationLightness {
    pub Base: Effect,
}
pub const HueSaturationLightnessEffectGuid: windows_core::GUID = windows_core::GUID::from_u128(0x8b2dd6c3_eb07_4d87_a5f0_7108e26a9c5f);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HueSaturationLightnessParams {
    pub hueLevel: i32,
    pub saturationLevel: i32,
    pub lightnessLevel: i32,
}
windows_core::imp::define_interface!(IImageBytes, IImageBytes_Vtbl, 0x025d1823_6c7d_447b_bbdb_a3cbc3dfa2fc);
windows_core::imp::interface_hierarchy!(IImageBytes, windows_core::IUnknown);
impl IImageBytes {
    pub unsafe fn CountBytes(&self, pcb: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CountBytes)(windows_core::Interface::as_raw(self), pcb as _).ok() }
    }
    pub unsafe fn LockBytes(&self, cb: u32, uloffset: u32, ppvbytes: *const *const core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).LockBytes)(windows_core::Interface::as_raw(self), cb, uloffset, ppvbytes).ok() }
    }
    pub unsafe fn UnlockBytes(&self, pvbytes: *const core::ffi::c_void, cb: u32, uloffset: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UnlockBytes)(windows_core::Interface::as_raw(self), pvbytes, cb, uloffset).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageBytes_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CountBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub LockBytes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *const core::ffi::c_void) -> windows_core::HRESULT,
    pub UnlockBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
}
pub trait IImageBytes_Impl: windows_core::IUnknownImpl {
    fn CountBytes(&self, pcb: *mut u32) -> windows_core::Result<()>;
    fn LockBytes(&self, cb: u32, uloffset: u32, ppvbytes: *const *const core::ffi::c_void) -> windows_core::Result<()>;
    fn UnlockBytes(&self, pvbytes: *const core::ffi::c_void, cb: u32, uloffset: u32) -> windows_core::Result<()>;
}
impl IImageBytes_Vtbl {
    pub const fn new<Identity: IImageBytes_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CountBytes<Identity: IImageBytes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcb: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageBytes_Impl::CountBytes(this, core::mem::transmute_copy(&pcb)).into()
            }
        }
        unsafe extern "system" fn LockBytes<Identity: IImageBytes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cb: u32, uloffset: u32, ppvbytes: *const *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageBytes_Impl::LockBytes(this, core::mem::transmute_copy(&cb), core::mem::transmute_copy(&uloffset), core::mem::transmute_copy(&ppvbytes)).into()
            }
        }
        unsafe extern "system" fn UnlockBytes<Identity: IImageBytes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvbytes: *const core::ffi::c_void, cb: u32, uloffset: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IImageBytes_Impl::UnlockBytes(this, core::mem::transmute_copy(&pvbytes), core::mem::transmute_copy(&cb), core::mem::transmute_copy(&uloffset)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CountBytes: CountBytes::<Identity, OFFSET>,
            LockBytes: LockBytes::<Identity, OFFSET>,
            UnlockBytes: UnlockBytes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IImageBytes as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IImageBytes {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Image(pub isize);
pub type ImageAbort = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void) -> windows_core::BOOL>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ImageCodecFlags(pub i32);
pub const ImageCodecFlagsBlockingDecode: ImageCodecFlags = ImageCodecFlags(32i32);
pub const ImageCodecFlagsBuiltin: ImageCodecFlags = ImageCodecFlags(65536i32);
pub const ImageCodecFlagsDecoder: ImageCodecFlags = ImageCodecFlags(2i32);
pub const ImageCodecFlagsEncoder: ImageCodecFlags = ImageCodecFlags(1i32);
pub const ImageCodecFlagsSeekableEncode: ImageCodecFlags = ImageCodecFlags(16i32);
pub const ImageCodecFlagsSupportBitmap: ImageCodecFlags = ImageCodecFlags(4i32);
pub const ImageCodecFlagsSupportVector: ImageCodecFlags = ImageCodecFlags(8i32);
pub const ImageCodecFlagsSystem: ImageCodecFlags = ImageCodecFlags(131072i32);
pub const ImageCodecFlagsUser: ImageCodecFlags = ImageCodecFlags(262144i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ImageCodecInfo {
    pub Clsid: windows_core::GUID,
    pub FormatID: windows_core::GUID,
    pub CodecName: windows_core::PCWSTR,
    pub DllName: windows_core::PCWSTR,
    pub FormatDescription: windows_core::PCWSTR,
    pub FilenameExtension: windows_core::PCWSTR,
    pub MimeType: windows_core::PCWSTR,
    pub Flags: u32,
    pub Version: u32,
    pub SigCount: u32,
    pub SigSize: u32,
    pub SigPattern: *const u8,
    pub SigMask: *const u8,
}
impl Default for ImageCodecInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ImageFlags(pub i32);
pub const ImageFlagsCaching: ImageFlags = ImageFlags(131072i32);
pub const ImageFlagsColorSpaceCMYK: ImageFlags = ImageFlags(32i32);
pub const ImageFlagsColorSpaceGRAY: ImageFlags = ImageFlags(64i32);
pub const ImageFlagsColorSpaceRGB: ImageFlags = ImageFlags(16i32);
pub const ImageFlagsColorSpaceYCBCR: ImageFlags = ImageFlags(128i32);
pub const ImageFlagsColorSpaceYCCK: ImageFlags = ImageFlags(256i32);
pub const ImageFlagsHasAlpha: ImageFlags = ImageFlags(2i32);
pub const ImageFlagsHasRealDPI: ImageFlags = ImageFlags(4096i32);
pub const ImageFlagsHasRealPixelSize: ImageFlags = ImageFlags(8192i32);
pub const ImageFlagsHasTranslucent: ImageFlags = ImageFlags(4i32);
pub const ImageFlagsNone: ImageFlags = ImageFlags(0i32);
pub const ImageFlagsPartiallyScalable: ImageFlags = ImageFlags(8i32);
pub const ImageFlagsReadOnly: ImageFlags = ImageFlags(65536i32);
pub const ImageFlagsScalable: ImageFlags = ImageFlags(1i32);
pub const ImageFormatBMP: windows_core::GUID = windows_core::GUID::from_u128(0xb96b3cab_0728_11d3_9d7b_0000f81ef32e);
pub const ImageFormatEMF: windows_core::GUID = windows_core::GUID::from_u128(0xb96b3cac_0728_11d3_9d7b_0000f81ef32e);
pub const ImageFormatEXIF: windows_core::GUID = windows_core::GUID::from_u128(0xb96b3cb2_0728_11d3_9d7b_0000f81ef32e);
pub const ImageFormatGIF: windows_core::GUID = windows_core::GUID::from_u128(0xb96b3cb0_0728_11d3_9d7b_0000f81ef32e);
pub const ImageFormatHEIF: windows_core::GUID = windows_core::GUID::from_u128(0xb96b3cb6_0728_11d3_9d7b_0000f81ef32e);
pub const ImageFormatIcon: windows_core::GUID = windows_core::GUID::from_u128(0xb96b3cb5_0728_11d3_9d7b_0000f81ef32e);
pub const ImageFormatJPEG: windows_core::GUID = windows_core::GUID::from_u128(0xb96b3cae_0728_11d3_9d7b_0000f81ef32e);
pub const ImageFormatMemoryBMP: windows_core::GUID = windows_core::GUID::from_u128(0xb96b3caa_0728_11d3_9d7b_0000f81ef32e);
pub const ImageFormatPNG: windows_core::GUID = windows_core::GUID::from_u128(0xb96b3caf_0728_11d3_9d7b_0000f81ef32e);
pub const ImageFormatTIFF: windows_core::GUID = windows_core::GUID::from_u128(0xb96b3cb1_0728_11d3_9d7b_0000f81ef32e);
pub const ImageFormatUndefined: windows_core::GUID = windows_core::GUID::from_u128(0xb96b3ca9_0728_11d3_9d7b_0000f81ef32e);
pub const ImageFormatWEBP: windows_core::GUID = windows_core::GUID::from_u128(0xb96b3cb7_0728_11d3_9d7b_0000f81ef32e);
pub const ImageFormatWMF: windows_core::GUID = windows_core::GUID::from_u128(0xb96b3cad_0728_11d3_9d7b_0000f81ef32e);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ImageItemData {
    pub Size: u32,
    pub Position: u32,
    pub Desc: *mut core::ffi::c_void,
    pub DescSize: u32,
    pub Data: *mut core::ffi::c_void,
    pub DataSize: u32,
    pub Cookie: u32,
}
impl Default for ImageItemData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ImageLockMode(pub i32);
pub const ImageLockModeRead: ImageLockMode = ImageLockMode(1i32);
pub const ImageLockModeUserInputBuf: ImageLockMode = ImageLockMode(4i32);
pub const ImageLockModeWrite: ImageLockMode = ImageLockMode(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ImageType(pub i32);
pub const ImageTypeBitmap: ImageType = ImageType(1i32);
pub const ImageTypeMetafile: ImageType = ImageType(2i32);
pub const ImageTypeUnknown: ImageType = ImageType(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct InstalledFontCollection(pub isize);
pub const InsufficientBuffer: Status = Status(5i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct InterpolationMode(pub i32);
pub const InterpolationModeBicubic: InterpolationMode = InterpolationMode(4i32);
pub const InterpolationModeBilinear: InterpolationMode = InterpolationMode(3i32);
pub const InterpolationModeDefault: InterpolationMode = InterpolationMode(0i32);
pub const InterpolationModeHighQuality: InterpolationMode = InterpolationMode(2i32);
pub const InterpolationModeHighQualityBicubic: InterpolationMode = InterpolationMode(7i32);
pub const InterpolationModeHighQualityBilinear: InterpolationMode = InterpolationMode(6i32);
pub const InterpolationModeInvalid: InterpolationMode = InterpolationMode(-1i32);
pub const InterpolationModeLowQuality: InterpolationMode = InterpolationMode(1i32);
pub const InterpolationModeNearestNeighbor: InterpolationMode = InterpolationMode(5i32);
pub const InvalidParameter: Status = Status(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ItemDataPosition(pub i32);
pub const ItemDataPositionAfterBits: ItemDataPosition = ItemDataPosition(2i32);
pub const ItemDataPositionAfterHeader: ItemDataPosition = ItemDataPosition(0i32);
pub const ItemDataPositionAfterPalette: ItemDataPosition = ItemDataPosition(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Levels {
    pub Base: Effect,
}
pub const LevelsEffectGuid: windows_core::GUID = windows_core::GUID::from_u128(0x99c354ec_2a31_4f3a_8c34_17a803b33a25);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LevelsParams {
    pub highlight: i32,
    pub midtone: i32,
    pub shadow: i32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LineCap(pub i32);
pub const LineCapAnchorMask: LineCap = LineCap(240i32);
pub const LineCapArrowAnchor: LineCap = LineCap(20i32);
pub const LineCapCustom: LineCap = LineCap(255i32);
pub const LineCapDiamondAnchor: LineCap = LineCap(19i32);
pub const LineCapFlat: LineCap = LineCap(0i32);
pub const LineCapNoAnchor: LineCap = LineCap(16i32);
pub const LineCapRound: LineCap = LineCap(2i32);
pub const LineCapRoundAnchor: LineCap = LineCap(18i32);
pub const LineCapSquare: LineCap = LineCap(1i32);
pub const LineCapSquareAnchor: LineCap = LineCap(17i32);
pub const LineCapTriangle: LineCap = LineCap(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LineJoin(pub i32);
pub const LineJoinBevel: LineJoin = LineJoin(1i32);
pub const LineJoinMiter: LineJoin = LineJoin(0i32);
pub const LineJoinMiterClipped: LineJoin = LineJoin(3i32);
pub const LineJoinRound: LineJoin = LineJoin(2i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LinearGradientMode(pub i32);
pub const LinearGradientModeBackwardDiagonal: LinearGradientMode = LinearGradientMode(3i32);
pub const LinearGradientModeForwardDiagonal: LinearGradientMode = LinearGradientMode(2i32);
pub const LinearGradientModeHorizontal: LinearGradientMode = LinearGradientMode(0i32);
pub const LinearGradientModeVertical: LinearGradientMode = LinearGradientMode(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Matrix(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MatrixOrder(pub i32);
pub const MatrixOrderAppend: MatrixOrder = MatrixOrder(1i32);
pub const MatrixOrderPrepend: MatrixOrder = MatrixOrder(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Metafile(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MetafileFrameUnit(pub i32);
pub const MetafileFrameUnitDocument: MetafileFrameUnit = MetafileFrameUnit(5i32);
pub const MetafileFrameUnitGdi: MetafileFrameUnit = MetafileFrameUnit(7i32);
pub const MetafileFrameUnitInch: MetafileFrameUnit = MetafileFrameUnit(4i32);
pub const MetafileFrameUnitMillimeter: MetafileFrameUnit = MetafileFrameUnit(6i32);
pub const MetafileFrameUnitPixel: MetafileFrameUnit = MetafileFrameUnit(2i32);
pub const MetafileFrameUnitPoint: MetafileFrameUnit = MetafileFrameUnit(3i32);
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy)]
pub struct MetafileHeader {
    pub Type: MetafileType,
    pub Size: u32,
    pub Version: u32,
    pub EmfPlusFlags: u32,
    pub DpiX: f32,
    pub DpiY: f32,
    pub X: i32,
    pub Y: i32,
    pub Width: i32,
    pub Height: i32,
    pub Anonymous: MetafileHeader_0,
    pub EmfPlusHeaderSize: i32,
    pub LogicalDpiX: i32,
    pub LogicalDpiY: i32,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for MetafileHeader {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy)]
pub union MetafileHeader_0 {
    pub WmfHeader: super::Gdi::METAHEADER,
    pub EmfHeader: ENHMETAHEADER3,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for MetafileHeader_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MetafileType(pub i32);
pub const MetafileTypeEmf: MetafileType = MetafileType(3i32);
pub const MetafileTypeEmfPlusDual: MetafileType = MetafileType(5i32);
pub const MetafileTypeEmfPlusOnly: MetafileType = MetafileType(4i32);
pub const MetafileTypeInvalid: MetafileType = MetafileType(0i32);
pub const MetafileTypeWmf: MetafileType = MetafileType(1i32);
pub const MetafileTypeWmfPlaceable: MetafileType = MetafileType(2i32);
pub const NotImplemented: Status = Status(6i32);
pub const NotTrueTypeFont: Status = Status(16i32);
pub type NotificationHookProc = Option<unsafe extern "system" fn(token: *mut usize) -> Status>;
pub type NotificationUnhookProc = Option<unsafe extern "system" fn(token: usize)>;
pub const ObjectBusy: Status = Status(4i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ObjectType(pub i32);
pub const ObjectTypeBrush: ObjectType = ObjectType(1i32);
pub const ObjectTypeCustomLineCap: ObjectType = ObjectType(9i32);
pub const ObjectTypeFont: ObjectType = ObjectType(6i32);
pub const ObjectTypeGraphics: ObjectType = ObjectType(10i32);
pub const ObjectTypeImage: ObjectType = ObjectType(5i32);
pub const ObjectTypeImageAttributes: ObjectType = ObjectType(8i32);
pub const ObjectTypeInvalid: ObjectType = ObjectType(0i32);
pub const ObjectTypeMax: ObjectType = ObjectType(10i32);
pub const ObjectTypeMin: ObjectType = ObjectType(1i32);
pub const ObjectTypePath: ObjectType = ObjectType(3i32);
pub const ObjectTypePen: ObjectType = ObjectType(2i32);
pub const ObjectTypeRegion: ObjectType = ObjectType(4i32);
pub const ObjectTypeStringFormat: ObjectType = ObjectType(7i32);
pub const Ok: Status = Status(0i32);
pub const OutOfMemory: Status = Status(3i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PWMFRect16 {
    pub Left: i16,
    pub Top: i16,
    pub Right: i16,
    pub Bottom: i16,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PaletteFlags(pub i32);
pub const PaletteFlagsGrayScale: PaletteFlags = PaletteFlags(2i32);
pub const PaletteFlagsHalftone: PaletteFlags = PaletteFlags(4i32);
pub const PaletteFlagsHasAlpha: PaletteFlags = PaletteFlags(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PaletteType(pub i32);
pub const PaletteTypeCustom: PaletteType = PaletteType(0i32);
pub const PaletteTypeFixedBW: PaletteType = PaletteType(2i32);
pub const PaletteTypeFixedHalftone125: PaletteType = PaletteType(6i32);
pub const PaletteTypeFixedHalftone216: PaletteType = PaletteType(7i32);
pub const PaletteTypeFixedHalftone252: PaletteType = PaletteType(8i32);
pub const PaletteTypeFixedHalftone256: PaletteType = PaletteType(9i32);
pub const PaletteTypeFixedHalftone27: PaletteType = PaletteType(4i32);
pub const PaletteTypeFixedHalftone64: PaletteType = PaletteType(5i32);
pub const PaletteTypeFixedHalftone8: PaletteType = PaletteType(3i32);
pub const PaletteTypeOptimal: PaletteType = PaletteType(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PathData(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PathPointType(pub i32);
pub const PathPointTypeBezier: PathPointType = PathPointType(3i32);
pub const PathPointTypeBezier3: PathPointType = PathPointType(3i32);
pub const PathPointTypeCloseSubpath: PathPointType = PathPointType(128i32);
pub const PathPointTypeDashMode: PathPointType = PathPointType(16i32);
pub const PathPointTypeLine: PathPointType = PathPointType(1i32);
pub const PathPointTypePathMarker: PathPointType = PathPointType(32i32);
pub const PathPointTypePathTypeMask: PathPointType = PathPointType(7i32);
pub const PathPointTypeStart: PathPointType = PathPointType(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PenAlignment(pub i32);
pub const PenAlignmentCenter: PenAlignment = PenAlignment(0i32);
pub const PenAlignmentInset: PenAlignment = PenAlignment(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PenType(pub i32);
pub const PenTypeHatchFill: PenType = PenType(1i32);
pub const PenTypeLinearGradient: PenType = PenType(4i32);
pub const PenTypePathGradient: PenType = PenType(3i32);
pub const PenTypeSolidColor: PenType = PenType(0i32);
pub const PenTypeTextureFill: PenType = PenType(2i32);
pub const PenTypeUnknown: PenType = PenType(-1i32);
pub const PixelFormatAlpha: u32 = 262144u32;
pub const PixelFormatCanonical: u32 = 2097152u32;
pub const PixelFormatDontCare: u32 = 0u32;
pub const PixelFormatExtended: u32 = 1048576u32;
pub const PixelFormatGDI: u32 = 131072u32;
pub const PixelFormatIndexed: u32 = 65536u32;
pub const PixelFormatMax: u32 = 16u32;
pub const PixelFormatPAlpha: u32 = 524288u32;
pub const PixelFormatUndefined: u32 = 0u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PixelOffsetMode(pub i32);
pub const PixelOffsetModeDefault: PixelOffsetMode = PixelOffsetMode(0i32);
pub const PixelOffsetModeHalf: PixelOffsetMode = PixelOffsetMode(4i32);
pub const PixelOffsetModeHighQuality: PixelOffsetMode = PixelOffsetMode(2i32);
pub const PixelOffsetModeHighSpeed: PixelOffsetMode = PixelOffsetMode(1i32);
pub const PixelOffsetModeInvalid: PixelOffsetMode = PixelOffsetMode(-1i32);
pub const PixelOffsetModeNone: PixelOffsetMode = PixelOffsetMode(3i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Point {
    pub X: i32,
    pub Y: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PointF {
    pub X: f32,
    pub Y: f32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PrivateFontCollection(pub isize);
pub const ProfileNotFound: Status = Status(21i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PropertyItem {
    pub id: u32,
    pub length: u32,
    pub r#type: u16,
    pub value: *mut core::ffi::c_void,
}
impl Default for PropertyItem {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PropertyNotFound: Status = Status(19i32);
pub const PropertyNotSupported: Status = Status(20i32);
pub const PropertyTagArtist: u32 = 315u32;
pub const PropertyTagBitsPerSample: u32 = 258u32;
pub const PropertyTagCellHeight: u32 = 265u32;
pub const PropertyTagCellWidth: u32 = 264u32;
pub const PropertyTagChrominanceTable: u32 = 20625u32;
pub const PropertyTagColorMap: u32 = 320u32;
pub const PropertyTagColorTransferFunction: u32 = 20506u32;
pub const PropertyTagCompression: u32 = 259u32;
pub const PropertyTagCopyright: u32 = 33432u32;
pub const PropertyTagDateTime: u32 = 306u32;
pub const PropertyTagDocumentName: u32 = 269u32;
pub const PropertyTagDotRange: u32 = 336u32;
pub const PropertyTagEquipMake: u32 = 271u32;
pub const PropertyTagEquipModel: u32 = 272u32;
pub const PropertyTagExifAperture: u32 = 37378u32;
pub const PropertyTagExifBrightness: u32 = 37379u32;
pub const PropertyTagExifCfaPattern: u32 = 41730u32;
pub const PropertyTagExifColorSpace: u32 = 40961u32;
pub const PropertyTagExifCompBPP: u32 = 37122u32;
pub const PropertyTagExifCompConfig: u32 = 37121u32;
pub const PropertyTagExifContrast: u32 = 41992u32;
pub const PropertyTagExifCustomRendered: u32 = 41985u32;
pub const PropertyTagExifDTDigSS: u32 = 37522u32;
pub const PropertyTagExifDTDigitized: u32 = 36868u32;
pub const PropertyTagExifDTOrig: u32 = 36867u32;
pub const PropertyTagExifDTOrigSS: u32 = 37521u32;
pub const PropertyTagExifDTSubsec: u32 = 37520u32;
pub const PropertyTagExifDeviceSettingDesc: u32 = 41995u32;
pub const PropertyTagExifDigitalZoomRatio: u32 = 41988u32;
pub const PropertyTagExifExposureBias: u32 = 37380u32;
pub const PropertyTagExifExposureIndex: u32 = 41493u32;
pub const PropertyTagExifExposureMode: u32 = 41986u32;
pub const PropertyTagExifExposureProg: u32 = 34850u32;
pub const PropertyTagExifExposureTime: u32 = 33434u32;
pub const PropertyTagExifFNumber: u32 = 33437u32;
pub const PropertyTagExifFPXVer: u32 = 40960u32;
pub const PropertyTagExifFileSource: u32 = 41728u32;
pub const PropertyTagExifFlash: u32 = 37385u32;
pub const PropertyTagExifFlashEnergy: u32 = 41483u32;
pub const PropertyTagExifFocalLength: u32 = 37386u32;
pub const PropertyTagExifFocalLengthIn35mmFilm: u32 = 41989u32;
pub const PropertyTagExifFocalResUnit: u32 = 41488u32;
pub const PropertyTagExifFocalXRes: u32 = 41486u32;
pub const PropertyTagExifFocalYRes: u32 = 41487u32;
pub const PropertyTagExifGainControl: u32 = 41991u32;
pub const PropertyTagExifIFD: u32 = 34665u32;
pub const PropertyTagExifISOSpeed: u32 = 34855u32;
pub const PropertyTagExifInterop: u32 = 40965u32;
pub const PropertyTagExifLightSource: u32 = 37384u32;
pub const PropertyTagExifMakerNote: u32 = 37500u32;
pub const PropertyTagExifMaxAperture: u32 = 37381u32;
pub const PropertyTagExifMeteringMode: u32 = 37383u32;
pub const PropertyTagExifOECF: u32 = 34856u32;
pub const PropertyTagExifPixXDim: u32 = 40962u32;
pub const PropertyTagExifPixYDim: u32 = 40963u32;
pub const PropertyTagExifRelatedWav: u32 = 40964u32;
pub const PropertyTagExifSaturation: u32 = 41993u32;
pub const PropertyTagExifSceneCaptureType: u32 = 41990u32;
pub const PropertyTagExifSceneType: u32 = 41729u32;
pub const PropertyTagExifSensingMethod: u32 = 41495u32;
pub const PropertyTagExifSharpness: u32 = 41994u32;
pub const PropertyTagExifShutterSpeed: u32 = 37377u32;
pub const PropertyTagExifSpatialFR: u32 = 41484u32;
pub const PropertyTagExifSpectralSense: u32 = 34852u32;
pub const PropertyTagExifSubjectArea: u32 = 37396u32;
pub const PropertyTagExifSubjectDist: u32 = 37382u32;
pub const PropertyTagExifSubjectDistanceRange: u32 = 41996u32;
pub const PropertyTagExifSubjectLoc: u32 = 41492u32;
pub const PropertyTagExifUniqueImageID: u32 = 42016u32;
pub const PropertyTagExifUserComment: u32 = 37510u32;
pub const PropertyTagExifVer: u32 = 36864u32;
pub const PropertyTagExifWhiteBalance: u32 = 41987u32;
pub const PropertyTagExtraSamples: u32 = 338u32;
pub const PropertyTagFillOrder: u32 = 266u32;
pub const PropertyTagFrameDelay: u32 = 20736u32;
pub const PropertyTagFreeByteCounts: u32 = 289u32;
pub const PropertyTagFreeOffset: u32 = 288u32;
pub const PropertyTagGamma: u32 = 769u32;
pub const PropertyTagGlobalPalette: u32 = 20738u32;
pub const PropertyTagGpsAltitude: u32 = 6u32;
pub const PropertyTagGpsAltitudeRef: u32 = 5u32;
pub const PropertyTagGpsAreaInformation: u32 = 28u32;
pub const PropertyTagGpsDate: u32 = 29u32;
pub const PropertyTagGpsDestBear: u32 = 24u32;
pub const PropertyTagGpsDestBearRef: u32 = 23u32;
pub const PropertyTagGpsDestDist: u32 = 26u32;
pub const PropertyTagGpsDestDistRef: u32 = 25u32;
pub const PropertyTagGpsDestLat: u32 = 20u32;
pub const PropertyTagGpsDestLatRef: u32 = 19u32;
pub const PropertyTagGpsDestLong: u32 = 22u32;
pub const PropertyTagGpsDestLongRef: u32 = 21u32;
pub const PropertyTagGpsDifferential: u32 = 30u32;
pub const PropertyTagGpsGpsDop: u32 = 11u32;
pub const PropertyTagGpsGpsMeasureMode: u32 = 10u32;
pub const PropertyTagGpsGpsSatellites: u32 = 8u32;
pub const PropertyTagGpsGpsStatus: u32 = 9u32;
pub const PropertyTagGpsGpsTime: u32 = 7u32;
pub const PropertyTagGpsIFD: u32 = 34853u32;
pub const PropertyTagGpsImgDir: u32 = 17u32;
pub const PropertyTagGpsImgDirRef: u32 = 16u32;
pub const PropertyTagGpsLatitude: u32 = 2u32;
pub const PropertyTagGpsLatitudeRef: u32 = 1u32;
pub const PropertyTagGpsLongitude: u32 = 4u32;
pub const PropertyTagGpsLongitudeRef: u32 = 3u32;
pub const PropertyTagGpsMapDatum: u32 = 18u32;
pub const PropertyTagGpsProcessingMethod: u32 = 27u32;
pub const PropertyTagGpsSpeed: u32 = 13u32;
pub const PropertyTagGpsSpeedRef: u32 = 12u32;
pub const PropertyTagGpsTrack: u32 = 15u32;
pub const PropertyTagGpsTrackRef: u32 = 14u32;
pub const PropertyTagGpsVer: u32 = 0u32;
pub const PropertyTagGrayResponseCurve: u32 = 291u32;
pub const PropertyTagGrayResponseUnit: u32 = 290u32;
pub const PropertyTagGridSize: u32 = 20497u32;
pub const PropertyTagHalftoneDegree: u32 = 20492u32;
pub const PropertyTagHalftoneHints: u32 = 321u32;
pub const PropertyTagHalftoneLPI: u32 = 20490u32;
pub const PropertyTagHalftoneLPIUnit: u32 = 20491u32;
pub const PropertyTagHalftoneMisc: u32 = 20494u32;
pub const PropertyTagHalftoneScreen: u32 = 20495u32;
pub const PropertyTagHalftoneShape: u32 = 20493u32;
pub const PropertyTagHostComputer: u32 = 316u32;
pub const PropertyTagICCProfile: u32 = 34675u32;
pub const PropertyTagICCProfileDescriptor: u32 = 770u32;
pub const PropertyTagImageDescription: u32 = 270u32;
pub const PropertyTagImageHeight: u32 = 257u32;
pub const PropertyTagImageTitle: u32 = 800u32;
pub const PropertyTagImageWidth: u32 = 256u32;
pub const PropertyTagIndexBackground: u32 = 20739u32;
pub const PropertyTagIndexTransparent: u32 = 20740u32;
pub const PropertyTagInkNames: u32 = 333u32;
pub const PropertyTagInkSet: u32 = 332u32;
pub const PropertyTagJPEGACTables: u32 = 521u32;
pub const PropertyTagJPEGDCTables: u32 = 520u32;
pub const PropertyTagJPEGInterFormat: u32 = 513u32;
pub const PropertyTagJPEGInterLength: u32 = 514u32;
pub const PropertyTagJPEGLosslessPredictors: u32 = 517u32;
pub const PropertyTagJPEGPointTransforms: u32 = 518u32;
pub const PropertyTagJPEGProc: u32 = 512u32;
pub const PropertyTagJPEGQTables: u32 = 519u32;
pub const PropertyTagJPEGQuality: u32 = 20496u32;
pub const PropertyTagJPEGRestartInterval: u32 = 515u32;
pub const PropertyTagLoopCount: u32 = 20737u32;
pub const PropertyTagLuminanceTable: u32 = 20624u32;
pub const PropertyTagMaxSampleValue: u32 = 281u32;
pub const PropertyTagMinSampleValue: u32 = 280u32;
pub const PropertyTagNewSubfileType: u32 = 254u32;
pub const PropertyTagNumberOfInks: u32 = 334u32;
pub const PropertyTagOrientation: u32 = 274u32;
pub const PropertyTagPageName: u32 = 285u32;
pub const PropertyTagPageNumber: u32 = 297u32;
pub const PropertyTagPaletteHistogram: u32 = 20755u32;
pub const PropertyTagPhotometricInterp: u32 = 262u32;
pub const PropertyTagPixelPerUnitX: u32 = 20753u32;
pub const PropertyTagPixelPerUnitY: u32 = 20754u32;
pub const PropertyTagPixelUnit: u32 = 20752u32;
pub const PropertyTagPlanarConfig: u32 = 284u32;
pub const PropertyTagPredictor: u32 = 317u32;
pub const PropertyTagPrimaryChromaticities: u32 = 319u32;
pub const PropertyTagPrintFlags: u32 = 20485u32;
pub const PropertyTagPrintFlagsBleedWidth: u32 = 20488u32;
pub const PropertyTagPrintFlagsBleedWidthScale: u32 = 20489u32;
pub const PropertyTagPrintFlagsCrop: u32 = 20487u32;
pub const PropertyTagPrintFlagsVersion: u32 = 20486u32;
pub const PropertyTagREFBlackWhite: u32 = 532u32;
pub const PropertyTagResolutionUnit: u32 = 296u32;
pub const PropertyTagResolutionXLengthUnit: u32 = 20483u32;
pub const PropertyTagResolutionXUnit: u32 = 20481u32;
pub const PropertyTagResolutionYLengthUnit: u32 = 20484u32;
pub const PropertyTagResolutionYUnit: u32 = 20482u32;
pub const PropertyTagRowsPerStrip: u32 = 278u32;
pub const PropertyTagSMaxSampleValue: u32 = 341u32;
pub const PropertyTagSMinSampleValue: u32 = 340u32;
pub const PropertyTagSRGBRenderingIntent: u32 = 771u32;
pub const PropertyTagSampleFormat: u32 = 339u32;
pub const PropertyTagSamplesPerPixel: u32 = 277u32;
pub const PropertyTagSoftwareUsed: u32 = 305u32;
pub const PropertyTagStripBytesCount: u32 = 279u32;
pub const PropertyTagStripOffsets: u32 = 273u32;
pub const PropertyTagSubfileType: u32 = 255u32;
pub const PropertyTagT4Option: u32 = 292u32;
pub const PropertyTagT6Option: u32 = 293u32;
pub const PropertyTagTargetPrinter: u32 = 337u32;
pub const PropertyTagThreshHolding: u32 = 263u32;
pub const PropertyTagThumbnailArtist: u32 = 20532u32;
pub const PropertyTagThumbnailBitsPerSample: u32 = 20514u32;
pub const PropertyTagThumbnailColorDepth: u32 = 20501u32;
pub const PropertyTagThumbnailCompressedSize: u32 = 20505u32;
pub const PropertyTagThumbnailCompression: u32 = 20515u32;
pub const PropertyTagThumbnailCopyRight: u32 = 20539u32;
pub const PropertyTagThumbnailData: u32 = 20507u32;
pub const PropertyTagThumbnailDateTime: u32 = 20531u32;
pub const PropertyTagThumbnailEquipMake: u32 = 20518u32;
pub const PropertyTagThumbnailEquipModel: u32 = 20519u32;
pub const PropertyTagThumbnailFormat: u32 = 20498u32;
pub const PropertyTagThumbnailHeight: u32 = 20500u32;
pub const PropertyTagThumbnailImageDescription: u32 = 20517u32;
pub const PropertyTagThumbnailImageHeight: u32 = 20513u32;
pub const PropertyTagThumbnailImageWidth: u32 = 20512u32;
pub const PropertyTagThumbnailOrientation: u32 = 20521u32;
pub const PropertyTagThumbnailPhotometricInterp: u32 = 20516u32;
pub const PropertyTagThumbnailPlanarConfig: u32 = 20527u32;
pub const PropertyTagThumbnailPlanes: u32 = 20502u32;
pub const PropertyTagThumbnailPrimaryChromaticities: u32 = 20534u32;
pub const PropertyTagThumbnailRawBytes: u32 = 20503u32;
pub const PropertyTagThumbnailRefBlackWhite: u32 = 20538u32;
pub const PropertyTagThumbnailResolutionUnit: u32 = 20528u32;
pub const PropertyTagThumbnailResolutionX: u32 = 20525u32;
pub const PropertyTagThumbnailResolutionY: u32 = 20526u32;
pub const PropertyTagThumbnailRowsPerStrip: u32 = 20523u32;
pub const PropertyTagThumbnailSamplesPerPixel: u32 = 20522u32;
pub const PropertyTagThumbnailSize: u32 = 20504u32;
pub const PropertyTagThumbnailSoftwareUsed: u32 = 20530u32;
pub const PropertyTagThumbnailStripBytesCount: u32 = 20524u32;
pub const PropertyTagThumbnailStripOffsets: u32 = 20520u32;
pub const PropertyTagThumbnailTransferFunction: u32 = 20529u32;
pub const PropertyTagThumbnailWhitePoint: u32 = 20533u32;
pub const PropertyTagThumbnailWidth: u32 = 20499u32;
pub const PropertyTagThumbnailYCbCrCoefficients: u32 = 20535u32;
pub const PropertyTagThumbnailYCbCrPositioning: u32 = 20537u32;
pub const PropertyTagThumbnailYCbCrSubsampling: u32 = 20536u32;
pub const PropertyTagTileByteCounts: u32 = 325u32;
pub const PropertyTagTileLength: u32 = 323u32;
pub const PropertyTagTileOffset: u32 = 324u32;
pub const PropertyTagTileWidth: u32 = 322u32;
pub const PropertyTagTransferFuncition: u32 = 301u32;
pub const PropertyTagTransferRange: u32 = 342u32;
pub const PropertyTagTypeASCII: u32 = 2u32;
pub const PropertyTagTypeByte: u32 = 1u32;
pub const PropertyTagTypeLong: u32 = 4u32;
pub const PropertyTagTypeRational: u32 = 5u32;
pub const PropertyTagTypeSLONG: u32 = 9u32;
pub const PropertyTagTypeSRational: u32 = 10u32;
pub const PropertyTagTypeShort: u32 = 3u32;
pub const PropertyTagTypeUndefined: u32 = 7u32;
pub const PropertyTagWhitePoint: u32 = 318u32;
pub const PropertyTagXPosition: u32 = 286u32;
pub const PropertyTagXResolution: u32 = 282u32;
pub const PropertyTagYCbCrCoefficients: u32 = 529u32;
pub const PropertyTagYCbCrPositioning: u32 = 531u32;
pub const PropertyTagYCbCrSubsampling: u32 = 530u32;
pub const PropertyTagYPosition: u32 = 287u32;
pub const PropertyTagYResolution: u32 = 283u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QualityMode(pub i32);
pub const QualityModeDefault: QualityMode = QualityMode(0i32);
pub const QualityModeHigh: QualityMode = QualityMode(2i32);
pub const QualityModeInvalid: QualityMode = QualityMode(-1i32);
pub const QualityModeLow: QualityMode = QualityMode(1i32);
pub const RED_SHIFT: u32 = 16u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Rect {
    pub X: i32,
    pub Y: i32,
    pub Width: i32,
    pub Height: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RectF {
    pub X: f32,
    pub Y: f32,
    pub Width: f32,
    pub Height: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RedEyeCorrection {
    pub Base: Effect,
}
pub const RedEyeCorrectionEffectGuid: windows_core::GUID = windows_core::GUID::from_u128(0x74d29d05_69a4_4266_9549_3cc52836b632);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RedEyeCorrectionParams {
    pub numberOfAreas: u32,
    pub areas: *mut super::super::Foundation::RECT,
}
impl Default for RedEyeCorrectionParams {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Region(pub isize);
pub const Rotate180FlipNone: RotateFlipType = RotateFlipType(2i32);
pub const Rotate180FlipX: RotateFlipType = RotateFlipType(6i32);
pub const Rotate180FlipXY: RotateFlipType = RotateFlipType(0i32);
pub const Rotate180FlipY: RotateFlipType = RotateFlipType(4i32);
pub const Rotate270FlipNone: RotateFlipType = RotateFlipType(3i32);
pub const Rotate270FlipX: RotateFlipType = RotateFlipType(7i32);
pub const Rotate270FlipXY: RotateFlipType = RotateFlipType(1i32);
pub const Rotate270FlipY: RotateFlipType = RotateFlipType(5i32);
pub const Rotate90FlipNone: RotateFlipType = RotateFlipType(1i32);
pub const Rotate90FlipX: RotateFlipType = RotateFlipType(5i32);
pub const Rotate90FlipXY: RotateFlipType = RotateFlipType(3i32);
pub const Rotate90FlipY: RotateFlipType = RotateFlipType(7i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RotateFlipType(pub i32);
pub const RotateNoneFlipNone: RotateFlipType = RotateFlipType(0i32);
pub const RotateNoneFlipX: RotateFlipType = RotateFlipType(4i32);
pub const RotateNoneFlipXY: RotateFlipType = RotateFlipType(2i32);
pub const RotateNoneFlipY: RotateFlipType = RotateFlipType(6i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Sharpen {
    pub Base: Effect,
}
pub const SharpenEffectGuid: windows_core::GUID = windows_core::GUID::from_u128(0x63cbf3ee_c526_402c_8f71_62c540bf5142);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SharpenParams {
    pub radius: f32,
    pub amount: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Size {
    pub Width: i32,
    pub Height: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SizeF {
    pub Width: f32,
    pub Height: f32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SmoothingMode(pub i32);
pub const SmoothingModeAntiAlias: SmoothingMode = SmoothingMode(4i32);
pub const SmoothingModeAntiAlias8x4: SmoothingMode = SmoothingMode(4i32);
pub const SmoothingModeAntiAlias8x8: SmoothingMode = SmoothingMode(5i32);
pub const SmoothingModeDefault: SmoothingMode = SmoothingMode(0i32);
pub const SmoothingModeHighQuality: SmoothingMode = SmoothingMode(2i32);
pub const SmoothingModeHighSpeed: SmoothingMode = SmoothingMode(1i32);
pub const SmoothingModeInvalid: SmoothingMode = SmoothingMode(-1i32);
pub const SmoothingModeNone: SmoothingMode = SmoothingMode(3i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Status(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct StringAlignment(pub i32);
pub const StringAlignmentCenter: StringAlignment = StringAlignment(1i32);
pub const StringAlignmentFar: StringAlignment = StringAlignment(2i32);
pub const StringAlignmentNear: StringAlignment = StringAlignment(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct StringDigitSubstitute(pub i32);
pub const StringDigitSubstituteNational: StringDigitSubstitute = StringDigitSubstitute(2i32);
pub const StringDigitSubstituteNone: StringDigitSubstitute = StringDigitSubstitute(1i32);
pub const StringDigitSubstituteTraditional: StringDigitSubstitute = StringDigitSubstitute(3i32);
pub const StringDigitSubstituteUser: StringDigitSubstitute = StringDigitSubstitute(0i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct StringFormatFlags(pub i32);
pub const StringFormatFlagsBypassGDI: StringFormatFlags = StringFormatFlags(-2147483648i32);
pub const StringFormatFlagsDirectionRightToLeft: StringFormatFlags = StringFormatFlags(1i32);
pub const StringFormatFlagsDirectionVertical: StringFormatFlags = StringFormatFlags(2i32);
pub const StringFormatFlagsDisplayFormatControl: StringFormatFlags = StringFormatFlags(32i32);
pub const StringFormatFlagsLineLimit: StringFormatFlags = StringFormatFlags(8192i32);
pub const StringFormatFlagsMeasureTrailingSpaces: StringFormatFlags = StringFormatFlags(2048i32);
pub const StringFormatFlagsNoClip: StringFormatFlags = StringFormatFlags(16384i32);
pub const StringFormatFlagsNoFitBlackBox: StringFormatFlags = StringFormatFlags(4i32);
pub const StringFormatFlagsNoFontFallback: StringFormatFlags = StringFormatFlags(1024i32);
pub const StringFormatFlagsNoWrap: StringFormatFlags = StringFormatFlags(4096i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct StringTrimming(pub i32);
pub const StringTrimmingCharacter: StringTrimming = StringTrimming(1i32);
pub const StringTrimmingEllipsisCharacter: StringTrimming = StringTrimming(3i32);
pub const StringTrimmingEllipsisPath: StringTrimming = StringTrimming(5i32);
pub const StringTrimmingEllipsisWord: StringTrimming = StringTrimming(4i32);
pub const StringTrimmingNone: StringTrimming = StringTrimming(0i32);
pub const StringTrimmingWord: StringTrimming = StringTrimming(2i32);
pub const TestControlForceBilinear: GpTestControlEnum = GpTestControlEnum(0i32);
pub const TestControlGetBuildNumber: GpTestControlEnum = GpTestControlEnum(2i32);
pub const TestControlNoICM: GpTestControlEnum = GpTestControlEnum(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TextRenderingHint(pub i32);
pub const TextRenderingHintAntiAlias: TextRenderingHint = TextRenderingHint(4i32);
pub const TextRenderingHintAntiAliasGridFit: TextRenderingHint = TextRenderingHint(3i32);
pub const TextRenderingHintClearTypeGridFit: TextRenderingHint = TextRenderingHint(5i32);
pub const TextRenderingHintSingleBitPerPixel: TextRenderingHint = TextRenderingHint(2i32);
pub const TextRenderingHintSingleBitPerPixelGridFit: TextRenderingHint = TextRenderingHint(1i32);
pub const TextRenderingHintSystemDefault: TextRenderingHint = TextRenderingHint(0i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Tint {
    pub Base: Effect,
}
pub const TintEffectGuid: windows_core::GUID = windows_core::GUID::from_u128(0x1077af00_2848_4441_9489_44ad4c2d7a2c);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TintParams {
    pub hue: i32,
    pub amount: i32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Unit(pub i32);
pub const UnitDisplay: Unit = Unit(1i32);
pub const UnitDocument: Unit = Unit(5i32);
pub const UnitInch: Unit = Unit(4i32);
pub const UnitMillimeter: Unit = Unit(6i32);
pub const UnitPixel: Unit = Unit(2i32);
pub const UnitPoint: Unit = Unit(3i32);
pub const UnitWorld: Unit = Unit(0i32);
pub const UnknownImageFormat: Status = Status(13i32);
pub const UnsupportedGdiplusVersion: Status = Status(17i32);
pub const ValueOverflow: Status = Status(11i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WarpMode(pub i32);
pub const WarpModeBilinear: WarpMode = WarpMode(1i32);
pub const WarpModePerspective: WarpMode = WarpMode(0i32);
pub const Win32Error: Status = Status(7i32);
#[repr(C, packed(2))]
#[derive(Clone, Copy, Default)]
pub struct WmfPlaceableFileHeader {
    pub Key: u32,
    pub Hmf: i16,
    pub BoundingBox: PWMFRect16,
    pub Inch: i16,
    pub Reserved: u32,
    pub Checksum: i16,
}
pub const WmfRecordTypeAbortDoc: EmfPlusRecordType = EmfPlusRecordType(65618i32);
pub const WmfRecordTypeAnimatePalette: EmfPlusRecordType = EmfPlusRecordType(66614i32);
pub const WmfRecordTypeArc: EmfPlusRecordType = EmfPlusRecordType(67607i32);
pub const WmfRecordTypeBitBlt: EmfPlusRecordType = EmfPlusRecordType(67874i32);
pub const WmfRecordTypeChord: EmfPlusRecordType = EmfPlusRecordType(67632i32);
pub const WmfRecordTypeCreateBitmap: EmfPlusRecordType = EmfPlusRecordType(67326i32);
pub const WmfRecordTypeCreateBitmapIndirect: EmfPlusRecordType = EmfPlusRecordType(66301i32);
pub const WmfRecordTypeCreateBrush: EmfPlusRecordType = EmfPlusRecordType(65784i32);
pub const WmfRecordTypeCreateBrushIndirect: EmfPlusRecordType = EmfPlusRecordType(66300i32);
pub const WmfRecordTypeCreateFontIndirect: EmfPlusRecordType = EmfPlusRecordType(66299i32);
pub const WmfRecordTypeCreatePalette: EmfPlusRecordType = EmfPlusRecordType(65783i32);
pub const WmfRecordTypeCreatePatternBrush: EmfPlusRecordType = EmfPlusRecordType(66041i32);
pub const WmfRecordTypeCreatePenIndirect: EmfPlusRecordType = EmfPlusRecordType(66298i32);
pub const WmfRecordTypeCreateRegion: EmfPlusRecordType = EmfPlusRecordType(67327i32);
pub const WmfRecordTypeDIBBitBlt: EmfPlusRecordType = EmfPlusRecordType(67904i32);
pub const WmfRecordTypeDIBCreatePatternBrush: EmfPlusRecordType = EmfPlusRecordType(65858i32);
pub const WmfRecordTypeDIBStretchBlt: EmfPlusRecordType = EmfPlusRecordType(68417i32);
pub const WmfRecordTypeDeleteObject: EmfPlusRecordType = EmfPlusRecordType(66032i32);
pub const WmfRecordTypeDrawText: EmfPlusRecordType = EmfPlusRecordType(67119i32);
pub const WmfRecordTypeEllipse: EmfPlusRecordType = EmfPlusRecordType(66584i32);
pub const WmfRecordTypeEndDoc: EmfPlusRecordType = EmfPlusRecordType(65630i32);
pub const WmfRecordTypeEndPage: EmfPlusRecordType = EmfPlusRecordType(65616i32);
pub const WmfRecordTypeEscape: EmfPlusRecordType = EmfPlusRecordType(67110i32);
pub const WmfRecordTypeExcludeClipRect: EmfPlusRecordType = EmfPlusRecordType(66581i32);
pub const WmfRecordTypeExtFloodFill: EmfPlusRecordType = EmfPlusRecordType(66888i32);
pub const WmfRecordTypeExtTextOut: EmfPlusRecordType = EmfPlusRecordType(68146i32);
pub const WmfRecordTypeFillRegion: EmfPlusRecordType = EmfPlusRecordType(66088i32);
pub const WmfRecordTypeFloodFill: EmfPlusRecordType = EmfPlusRecordType(66585i32);
pub const WmfRecordTypeFrameRegion: EmfPlusRecordType = EmfPlusRecordType(66601i32);
pub const WmfRecordTypeIntersectClipRect: EmfPlusRecordType = EmfPlusRecordType(66582i32);
pub const WmfRecordTypeInvertRegion: EmfPlusRecordType = EmfPlusRecordType(65834i32);
pub const WmfRecordTypeLineTo: EmfPlusRecordType = EmfPlusRecordType(66067i32);
pub const WmfRecordTypeMoveTo: EmfPlusRecordType = EmfPlusRecordType(66068i32);
pub const WmfRecordTypeOffsetClipRgn: EmfPlusRecordType = EmfPlusRecordType(66080i32);
pub const WmfRecordTypeOffsetViewportOrg: EmfPlusRecordType = EmfPlusRecordType(66065i32);
pub const WmfRecordTypeOffsetWindowOrg: EmfPlusRecordType = EmfPlusRecordType(66063i32);
pub const WmfRecordTypePaintRegion: EmfPlusRecordType = EmfPlusRecordType(65835i32);
pub const WmfRecordTypePatBlt: EmfPlusRecordType = EmfPlusRecordType(67101i32);
pub const WmfRecordTypePie: EmfPlusRecordType = EmfPlusRecordType(67610i32);
pub const WmfRecordTypePolyPolygon: EmfPlusRecordType = EmfPlusRecordType(66872i32);
pub const WmfRecordTypePolygon: EmfPlusRecordType = EmfPlusRecordType(66340i32);
pub const WmfRecordTypePolyline: EmfPlusRecordType = EmfPlusRecordType(66341i32);
pub const WmfRecordTypeRealizePalette: EmfPlusRecordType = EmfPlusRecordType(65589i32);
pub const WmfRecordTypeRectangle: EmfPlusRecordType = EmfPlusRecordType(66587i32);
pub const WmfRecordTypeResetDC: EmfPlusRecordType = EmfPlusRecordType(65868i32);
pub const WmfRecordTypeResizePalette: EmfPlusRecordType = EmfPlusRecordType(65849i32);
pub const WmfRecordTypeRestoreDC: EmfPlusRecordType = EmfPlusRecordType(65831i32);
pub const WmfRecordTypeRoundRect: EmfPlusRecordType = EmfPlusRecordType(67100i32);
pub const WmfRecordTypeSaveDC: EmfPlusRecordType = EmfPlusRecordType(65566i32);
pub const WmfRecordTypeScaleViewportExt: EmfPlusRecordType = EmfPlusRecordType(66578i32);
pub const WmfRecordTypeScaleWindowExt: EmfPlusRecordType = EmfPlusRecordType(66576i32);
pub const WmfRecordTypeSelectClipRegion: EmfPlusRecordType = EmfPlusRecordType(65836i32);
pub const WmfRecordTypeSelectObject: EmfPlusRecordType = EmfPlusRecordType(65837i32);
pub const WmfRecordTypeSelectPalette: EmfPlusRecordType = EmfPlusRecordType(66100i32);
pub const WmfRecordTypeSetBkColor: EmfPlusRecordType = EmfPlusRecordType(66049i32);
pub const WmfRecordTypeSetBkMode: EmfPlusRecordType = EmfPlusRecordType(65794i32);
pub const WmfRecordTypeSetDIBToDev: EmfPlusRecordType = EmfPlusRecordType(68915i32);
pub const WmfRecordTypeSetLayout: EmfPlusRecordType = EmfPlusRecordType(65865i32);
pub const WmfRecordTypeSetMapMode: EmfPlusRecordType = EmfPlusRecordType(65795i32);
pub const WmfRecordTypeSetMapperFlags: EmfPlusRecordType = EmfPlusRecordType(66097i32);
pub const WmfRecordTypeSetPalEntries: EmfPlusRecordType = EmfPlusRecordType(65591i32);
pub const WmfRecordTypeSetPixel: EmfPlusRecordType = EmfPlusRecordType(66591i32);
pub const WmfRecordTypeSetPolyFillMode: EmfPlusRecordType = EmfPlusRecordType(65798i32);
pub const WmfRecordTypeSetROP2: EmfPlusRecordType = EmfPlusRecordType(65796i32);
pub const WmfRecordTypeSetRelAbs: EmfPlusRecordType = EmfPlusRecordType(65797i32);
pub const WmfRecordTypeSetStretchBltMode: EmfPlusRecordType = EmfPlusRecordType(65799i32);
pub const WmfRecordTypeSetTextAlign: EmfPlusRecordType = EmfPlusRecordType(65838i32);
pub const WmfRecordTypeSetTextCharExtra: EmfPlusRecordType = EmfPlusRecordType(65800i32);
pub const WmfRecordTypeSetTextColor: EmfPlusRecordType = EmfPlusRecordType(66057i32);
pub const WmfRecordTypeSetTextJustification: EmfPlusRecordType = EmfPlusRecordType(66058i32);
pub const WmfRecordTypeSetViewportExt: EmfPlusRecordType = EmfPlusRecordType(66062i32);
pub const WmfRecordTypeSetViewportOrg: EmfPlusRecordType = EmfPlusRecordType(66061i32);
pub const WmfRecordTypeSetWindowExt: EmfPlusRecordType = EmfPlusRecordType(66060i32);
pub const WmfRecordTypeSetWindowOrg: EmfPlusRecordType = EmfPlusRecordType(66059i32);
pub const WmfRecordTypeStartDoc: EmfPlusRecordType = EmfPlusRecordType(65869i32);
pub const WmfRecordTypeStartPage: EmfPlusRecordType = EmfPlusRecordType(65615i32);
pub const WmfRecordTypeStretchBlt: EmfPlusRecordType = EmfPlusRecordType(68387i32);
pub const WmfRecordTypeStretchDIB: EmfPlusRecordType = EmfPlusRecordType(69443i32);
pub const WmfRecordTypeTextOut: EmfPlusRecordType = EmfPlusRecordType(66849i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WrapMode(pub i32);
pub const WrapModeClamp: WrapMode = WrapMode(4i32);
pub const WrapModeTile: WrapMode = WrapMode(0i32);
pub const WrapModeTileFlipX: WrapMode = WrapMode(1i32);
pub const WrapModeTileFlipXY: WrapMode = WrapMode(3i32);
pub const WrapModeTileFlipY: WrapMode = WrapMode(2i32);
pub const WrongState: Status = Status(8i32);
