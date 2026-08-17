#[inline]
pub unsafe fn GdipAddPathArc(path: *mut GpPath, x: f32, y: f32, width: f32, height: f32, startangle: f32, sweepangle: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipAddPathArc(path : *mut GpPath, x : f32, y : f32, width : f32, height : f32, startangle : f32, sweepangle : f32) -> Status);
    GdipAddPathArc(path, x, y, width, height, startangle, sweepangle)
}
#[inline]
pub unsafe fn GdipAddPathArcI(path: *mut GpPath, x: i32, y: i32, width: i32, height: i32, startangle: f32, sweepangle: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipAddPathArcI(path : *mut GpPath, x : i32, y : i32, width : i32, height : i32, startangle : f32, sweepangle : f32) -> Status);
    GdipAddPathArcI(path, x, y, width, height, startangle, sweepangle)
}
#[inline]
pub unsafe fn GdipAddPathBezier(path: *mut GpPath, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, x4: f32, y4: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipAddPathBezier(path : *mut GpPath, x1 : f32, y1 : f32, x2 : f32, y2 : f32, x3 : f32, y3 : f32, x4 : f32, y4 : f32) -> Status);
    GdipAddPathBezier(path, x1, y1, x2, y2, x3, y3, x4, y4)
}
#[inline]
pub unsafe fn GdipAddPathBezierI(path: *mut GpPath, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, x4: i32, y4: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipAddPathBezierI(path : *mut GpPath, x1 : i32, y1 : i32, x2 : i32, y2 : i32, x3 : i32, y3 : i32, x4 : i32, y4 : i32) -> Status);
    GdipAddPathBezierI(path, x1, y1, x2, y2, x3, y3, x4, y4)
}
#[inline]
pub unsafe fn GdipAddPathBeziers(path: *mut GpPath, points: *const PointF, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipAddPathBeziers(path : *mut GpPath, points : *const PointF, count : i32) -> Status);
    GdipAddPathBeziers(path, points, count)
}
#[inline]
pub unsafe fn GdipAddPathBeziersI(path: *mut GpPath, points: *const Point, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipAddPathBeziersI(path : *mut GpPath, points : *const Point, count : i32) -> Status);
    GdipAddPathBeziersI(path, points, count)
}
#[inline]
pub unsafe fn GdipAddPathClosedCurve(path: *mut GpPath, points: *const PointF, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipAddPathClosedCurve(path : *mut GpPath, points : *const PointF, count : i32) -> Status);
    GdipAddPathClosedCurve(path, points, count)
}
#[inline]
pub unsafe fn GdipAddPathClosedCurve2(path: *mut GpPath, points: *const PointF, count: i32, tension: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipAddPathClosedCurve2(path : *mut GpPath, points : *const PointF, count : i32, tension : f32) -> Status);
    GdipAddPathClosedCurve2(path, points, count, tension)
}
#[inline]
pub unsafe fn GdipAddPathClosedCurve2I(path: *mut GpPath, points: *const Point, count: i32, tension: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipAddPathClosedCurve2I(path : *mut GpPath, points : *const Point, count : i32, tension : f32) -> Status);
    GdipAddPathClosedCurve2I(path, points, count, tension)
}
#[inline]
pub unsafe fn GdipAddPathClosedCurveI(path: *mut GpPath, points: *const Point, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipAddPathClosedCurveI(path : *mut GpPath, points : *const Point, count : i32) -> Status);
    GdipAddPathClosedCurveI(path, points, count)
}
#[inline]
pub unsafe fn GdipAddPathCurve(path: *mut GpPath, points: *const PointF, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipAddPathCurve(path : *mut GpPath, points : *const PointF, count : i32) -> Status);
    GdipAddPathCurve(path, points, count)
}
#[inline]
pub unsafe fn GdipAddPathCurve2(path: *mut GpPath, points: *const PointF, count: i32, tension: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipAddPathCurve2(path : *mut GpPath, points : *const PointF, count : i32, tension : f32) -> Status);
    GdipAddPathCurve2(path, points, count, tension)
}
#[inline]
pub unsafe fn GdipAddPathCurve2I(path: *mut GpPath, points: *const Point, count: i32, tension: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipAddPathCurve2I(path : *mut GpPath, points : *const Point, count : i32, tension : f32) -> Status);
    GdipAddPathCurve2I(path, points, count, tension)
}
#[inline]
pub unsafe fn GdipAddPathCurve3(path: *mut GpPath, points: *const PointF, count: i32, offset: i32, numberofsegments: i32, tension: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipAddPathCurve3(path : *mut GpPath, points : *const PointF, count : i32, offset : i32, numberofsegments : i32, tension : f32) -> Status);
    GdipAddPathCurve3(path, points, count, offset, numberofsegments, tension)
}
#[inline]
pub unsafe fn GdipAddPathCurve3I(path: *mut GpPath, points: *const Point, count: i32, offset: i32, numberofsegments: i32, tension: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipAddPathCurve3I(path : *mut GpPath, points : *const Point, count : i32, offset : i32, numberofsegments : i32, tension : f32) -> Status);
    GdipAddPathCurve3I(path, points, count, offset, numberofsegments, tension)
}
#[inline]
pub unsafe fn GdipAddPathCurveI(path: *mut GpPath, points: *const Point, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipAddPathCurveI(path : *mut GpPath, points : *const Point, count : i32) -> Status);
    GdipAddPathCurveI(path, points, count)
}
#[inline]
pub unsafe fn GdipAddPathEllipse(path: *mut GpPath, x: f32, y: f32, width: f32, height: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipAddPathEllipse(path : *mut GpPath, x : f32, y : f32, width : f32, height : f32) -> Status);
    GdipAddPathEllipse(path, x, y, width, height)
}
#[inline]
pub unsafe fn GdipAddPathEllipseI(path: *mut GpPath, x: i32, y: i32, width: i32, height: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipAddPathEllipseI(path : *mut GpPath, x : i32, y : i32, width : i32, height : i32) -> Status);
    GdipAddPathEllipseI(path, x, y, width, height)
}
#[inline]
pub unsafe fn GdipAddPathLine(path: *mut GpPath, x1: f32, y1: f32, x2: f32, y2: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipAddPathLine(path : *mut GpPath, x1 : f32, y1 : f32, x2 : f32, y2 : f32) -> Status);
    GdipAddPathLine(path, x1, y1, x2, y2)
}
#[inline]
pub unsafe fn GdipAddPathLine2(path: *mut GpPath, points: *const PointF, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipAddPathLine2(path : *mut GpPath, points : *const PointF, count : i32) -> Status);
    GdipAddPathLine2(path, points, count)
}
#[inline]
pub unsafe fn GdipAddPathLine2I(path: *mut GpPath, points: *const Point, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipAddPathLine2I(path : *mut GpPath, points : *const Point, count : i32) -> Status);
    GdipAddPathLine2I(path, points, count)
}
#[inline]
pub unsafe fn GdipAddPathLineI(path: *mut GpPath, x1: i32, y1: i32, x2: i32, y2: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipAddPathLineI(path : *mut GpPath, x1 : i32, y1 : i32, x2 : i32, y2 : i32) -> Status);
    GdipAddPathLineI(path, x1, y1, x2, y2)
}
#[inline]
pub unsafe fn GdipAddPathPath<P0>(path: *mut GpPath, addingpath: *const GpPath, connect: P0) -> Status
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipAddPathPath(path : *mut GpPath, addingpath : *const GpPath, connect : super::super::Foundation:: BOOL) -> Status);
    GdipAddPathPath(path, addingpath, connect.param().abi())
}
#[inline]
pub unsafe fn GdipAddPathPie(path: *mut GpPath, x: f32, y: f32, width: f32, height: f32, startangle: f32, sweepangle: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipAddPathPie(path : *mut GpPath, x : f32, y : f32, width : f32, height : f32, startangle : f32, sweepangle : f32) -> Status);
    GdipAddPathPie(path, x, y, width, height, startangle, sweepangle)
}
#[inline]
pub unsafe fn GdipAddPathPieI(path: *mut GpPath, x: i32, y: i32, width: i32, height: i32, startangle: f32, sweepangle: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipAddPathPieI(path : *mut GpPath, x : i32, y : i32, width : i32, height : i32, startangle : f32, sweepangle : f32) -> Status);
    GdipAddPathPieI(path, x, y, width, height, startangle, sweepangle)
}
#[inline]
pub unsafe fn GdipAddPathPolygon(path: *mut GpPath, points: *const PointF, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipAddPathPolygon(path : *mut GpPath, points : *const PointF, count : i32) -> Status);
    GdipAddPathPolygon(path, points, count)
}
#[inline]
pub unsafe fn GdipAddPathPolygonI(path: *mut GpPath, points: *const Point, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipAddPathPolygonI(path : *mut GpPath, points : *const Point, count : i32) -> Status);
    GdipAddPathPolygonI(path, points, count)
}
#[inline]
pub unsafe fn GdipAddPathRectangle(path: *mut GpPath, x: f32, y: f32, width: f32, height: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipAddPathRectangle(path : *mut GpPath, x : f32, y : f32, width : f32, height : f32) -> Status);
    GdipAddPathRectangle(path, x, y, width, height)
}
#[inline]
pub unsafe fn GdipAddPathRectangleI(path: *mut GpPath, x: i32, y: i32, width: i32, height: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipAddPathRectangleI(path : *mut GpPath, x : i32, y : i32, width : i32, height : i32) -> Status);
    GdipAddPathRectangleI(path, x, y, width, height)
}
#[inline]
pub unsafe fn GdipAddPathRectangles(path: *mut GpPath, rects: *const RectF, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipAddPathRectangles(path : *mut GpPath, rects : *const RectF, count : i32) -> Status);
    GdipAddPathRectangles(path, rects, count)
}
#[inline]
pub unsafe fn GdipAddPathRectanglesI(path: *mut GpPath, rects: *const Rect, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipAddPathRectanglesI(path : *mut GpPath, rects : *const Rect, count : i32) -> Status);
    GdipAddPathRectanglesI(path, rects, count)
}
#[inline]
pub unsafe fn GdipAddPathString<P0>(path: *mut GpPath, string: P0, length: i32, family: *const GpFontFamily, style: i32, emsize: f32, layoutrect: *const RectF, format: *const GpStringFormat) -> Status
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipAddPathString(path : *mut GpPath, string : windows_core::PCWSTR, length : i32, family : *const GpFontFamily, style : i32, emsize : f32, layoutrect : *const RectF, format : *const GpStringFormat) -> Status);
    GdipAddPathString(path, string.param().abi(), length, family, style, emsize, layoutrect, format)
}
#[inline]
pub unsafe fn GdipAddPathStringI<P0>(path: *mut GpPath, string: P0, length: i32, family: *const GpFontFamily, style: i32, emsize: f32, layoutrect: *const Rect, format: *const GpStringFormat) -> Status
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipAddPathStringI(path : *mut GpPath, string : windows_core::PCWSTR, length : i32, family : *const GpFontFamily, style : i32, emsize : f32, layoutrect : *const Rect, format : *const GpStringFormat) -> Status);
    GdipAddPathStringI(path, string.param().abi(), length, family, style, emsize, layoutrect, format)
}
#[inline]
pub unsafe fn GdipAlloc(size: usize) -> *mut core::ffi::c_void {
    windows_targets::link!("gdiplus.dll" "system" fn GdipAlloc(size : usize) -> *mut core::ffi::c_void);
    GdipAlloc(size)
}
#[inline]
pub unsafe fn GdipBeginContainer(graphics: *mut GpGraphics, dstrect: *const RectF, srcrect: *const RectF, unit: Unit, state: *mut u32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipBeginContainer(graphics : *mut GpGraphics, dstrect : *const RectF, srcrect : *const RectF, unit : Unit, state : *mut u32) -> Status);
    GdipBeginContainer(graphics, dstrect, srcrect, unit, state)
}
#[inline]
pub unsafe fn GdipBeginContainer2(graphics: *mut GpGraphics, state: *mut u32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipBeginContainer2(graphics : *mut GpGraphics, state : *mut u32) -> Status);
    GdipBeginContainer2(graphics, state)
}
#[inline]
pub unsafe fn GdipBeginContainerI(graphics: *mut GpGraphics, dstrect: *const Rect, srcrect: *const Rect, unit: Unit, state: *mut u32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipBeginContainerI(graphics : *mut GpGraphics, dstrect : *const Rect, srcrect : *const Rect, unit : Unit, state : *mut u32) -> Status);
    GdipBeginContainerI(graphics, dstrect, srcrect, unit, state)
}
#[inline]
pub unsafe fn GdipBitmapApplyEffect<P0>(bitmap: *mut GpBitmap, effect: *mut CGpEffect, roi: *mut super::super::Foundation::RECT, useauxdata: P0, auxdata: *mut *mut core::ffi::c_void, auxdatasize: *mut i32) -> Status
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipBitmapApplyEffect(bitmap : *mut GpBitmap, effect : *mut CGpEffect, roi : *mut super::super::Foundation:: RECT, useauxdata : super::super::Foundation:: BOOL, auxdata : *mut *mut core::ffi::c_void, auxdatasize : *mut i32) -> Status);
    GdipBitmapApplyEffect(bitmap, effect, roi, useauxdata.param().abi(), auxdata, auxdatasize)
}
#[inline]
pub unsafe fn GdipBitmapConvertFormat(pinputbitmap: *mut GpBitmap, format: i32, dithertype: DitherType, palettetype: PaletteType, palette: *mut ColorPalette, alphathresholdpercent: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipBitmapConvertFormat(pinputbitmap : *mut GpBitmap, format : i32, dithertype : DitherType, palettetype : PaletteType, palette : *mut ColorPalette, alphathresholdpercent : f32) -> Status);
    GdipBitmapConvertFormat(pinputbitmap, format, dithertype, palettetype, palette, alphathresholdpercent)
}
#[inline]
pub unsafe fn GdipBitmapCreateApplyEffect<P0>(inputbitmaps: *mut *mut GpBitmap, numinputs: i32, effect: *mut CGpEffect, roi: *mut super::super::Foundation::RECT, outputrect: *mut super::super::Foundation::RECT, outputbitmap: *mut *mut GpBitmap, useauxdata: P0, auxdata: *mut *mut core::ffi::c_void, auxdatasize: *mut i32) -> Status
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipBitmapCreateApplyEffect(inputbitmaps : *mut *mut GpBitmap, numinputs : i32, effect : *mut CGpEffect, roi : *mut super::super::Foundation:: RECT, outputrect : *mut super::super::Foundation:: RECT, outputbitmap : *mut *mut GpBitmap, useauxdata : super::super::Foundation:: BOOL, auxdata : *mut *mut core::ffi::c_void, auxdatasize : *mut i32) -> Status);
    GdipBitmapCreateApplyEffect(inputbitmaps, numinputs, effect, roi, outputrect, outputbitmap, useauxdata.param().abi(), auxdata, auxdatasize)
}
#[inline]
pub unsafe fn GdipBitmapGetHistogram(bitmap: *mut GpBitmap, format: HistogramFormat, numberofentries: u32, channel0: *mut u32, channel1: *mut u32, channel2: *mut u32, channel3: *mut u32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipBitmapGetHistogram(bitmap : *mut GpBitmap, format : HistogramFormat, numberofentries : u32, channel0 : *mut u32, channel1 : *mut u32, channel2 : *mut u32, channel3 : *mut u32) -> Status);
    GdipBitmapGetHistogram(bitmap, format, numberofentries, channel0, channel1, channel2, channel3)
}
#[inline]
pub unsafe fn GdipBitmapGetHistogramSize(format: HistogramFormat, numberofentries: *mut u32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipBitmapGetHistogramSize(format : HistogramFormat, numberofentries : *mut u32) -> Status);
    GdipBitmapGetHistogramSize(format, numberofentries)
}
#[inline]
pub unsafe fn GdipBitmapGetPixel(bitmap: *mut GpBitmap, x: i32, y: i32, color: *mut u32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipBitmapGetPixel(bitmap : *mut GpBitmap, x : i32, y : i32, color : *mut u32) -> Status);
    GdipBitmapGetPixel(bitmap, x, y, color)
}
#[inline]
pub unsafe fn GdipBitmapLockBits(bitmap: *mut GpBitmap, rect: *const Rect, flags: u32, format: i32, lockedbitmapdata: *mut BitmapData) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipBitmapLockBits(bitmap : *mut GpBitmap, rect : *const Rect, flags : u32, format : i32, lockedbitmapdata : *mut BitmapData) -> Status);
    GdipBitmapLockBits(bitmap, rect, flags, format, lockedbitmapdata)
}
#[inline]
pub unsafe fn GdipBitmapSetPixel(bitmap: *mut GpBitmap, x: i32, y: i32, color: u32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipBitmapSetPixel(bitmap : *mut GpBitmap, x : i32, y : i32, color : u32) -> Status);
    GdipBitmapSetPixel(bitmap, x, y, color)
}
#[inline]
pub unsafe fn GdipBitmapSetResolution(bitmap: *mut GpBitmap, xdpi: f32, ydpi: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipBitmapSetResolution(bitmap : *mut GpBitmap, xdpi : f32, ydpi : f32) -> Status);
    GdipBitmapSetResolution(bitmap, xdpi, ydpi)
}
#[inline]
pub unsafe fn GdipBitmapUnlockBits(bitmap: *mut GpBitmap, lockedbitmapdata: *mut BitmapData) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipBitmapUnlockBits(bitmap : *mut GpBitmap, lockedbitmapdata : *mut BitmapData) -> Status);
    GdipBitmapUnlockBits(bitmap, lockedbitmapdata)
}
#[inline]
pub unsafe fn GdipClearPathMarkers(path: *mut GpPath) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipClearPathMarkers(path : *mut GpPath) -> Status);
    GdipClearPathMarkers(path)
}
#[inline]
pub unsafe fn GdipCloneBitmapArea(x: f32, y: f32, width: f32, height: f32, format: i32, srcbitmap: *mut GpBitmap, dstbitmap: *mut *mut GpBitmap) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCloneBitmapArea(x : f32, y : f32, width : f32, height : f32, format : i32, srcbitmap : *mut GpBitmap, dstbitmap : *mut *mut GpBitmap) -> Status);
    GdipCloneBitmapArea(x, y, width, height, format, srcbitmap, dstbitmap)
}
#[inline]
pub unsafe fn GdipCloneBitmapAreaI(x: i32, y: i32, width: i32, height: i32, format: i32, srcbitmap: *mut GpBitmap, dstbitmap: *mut *mut GpBitmap) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCloneBitmapAreaI(x : i32, y : i32, width : i32, height : i32, format : i32, srcbitmap : *mut GpBitmap, dstbitmap : *mut *mut GpBitmap) -> Status);
    GdipCloneBitmapAreaI(x, y, width, height, format, srcbitmap, dstbitmap)
}
#[inline]
pub unsafe fn GdipCloneBrush(brush: *mut GpBrush, clonebrush: *mut *mut GpBrush) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCloneBrush(brush : *mut GpBrush, clonebrush : *mut *mut GpBrush) -> Status);
    GdipCloneBrush(brush, clonebrush)
}
#[inline]
pub unsafe fn GdipCloneCustomLineCap(customcap: *mut GpCustomLineCap, clonedcap: *mut *mut GpCustomLineCap) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCloneCustomLineCap(customcap : *mut GpCustomLineCap, clonedcap : *mut *mut GpCustomLineCap) -> Status);
    GdipCloneCustomLineCap(customcap, clonedcap)
}
#[inline]
pub unsafe fn GdipCloneFont(font: *mut GpFont, clonefont: *mut *mut GpFont) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCloneFont(font : *mut GpFont, clonefont : *mut *mut GpFont) -> Status);
    GdipCloneFont(font, clonefont)
}
#[inline]
pub unsafe fn GdipCloneFontFamily(fontfamily: *mut GpFontFamily, clonedfontfamily: *mut *mut GpFontFamily) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCloneFontFamily(fontfamily : *mut GpFontFamily, clonedfontfamily : *mut *mut GpFontFamily) -> Status);
    GdipCloneFontFamily(fontfamily, clonedfontfamily)
}
#[inline]
pub unsafe fn GdipCloneImage(image: *mut GpImage, cloneimage: *mut *mut GpImage) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCloneImage(image : *mut GpImage, cloneimage : *mut *mut GpImage) -> Status);
    GdipCloneImage(image, cloneimage)
}
#[inline]
pub unsafe fn GdipCloneImageAttributes(imageattr: *const GpImageAttributes, cloneimageattr: *mut *mut GpImageAttributes) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCloneImageAttributes(imageattr : *const GpImageAttributes, cloneimageattr : *mut *mut GpImageAttributes) -> Status);
    GdipCloneImageAttributes(imageattr, cloneimageattr)
}
#[inline]
pub unsafe fn GdipCloneMatrix(matrix: *mut Matrix, clonematrix: *mut *mut Matrix) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCloneMatrix(matrix : *mut Matrix, clonematrix : *mut *mut Matrix) -> Status);
    GdipCloneMatrix(matrix, clonematrix)
}
#[inline]
pub unsafe fn GdipClonePath(path: *mut GpPath, clonepath: *mut *mut GpPath) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipClonePath(path : *mut GpPath, clonepath : *mut *mut GpPath) -> Status);
    GdipClonePath(path, clonepath)
}
#[inline]
pub unsafe fn GdipClonePen(pen: *mut GpPen, clonepen: *mut *mut GpPen) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipClonePen(pen : *mut GpPen, clonepen : *mut *mut GpPen) -> Status);
    GdipClonePen(pen, clonepen)
}
#[inline]
pub unsafe fn GdipCloneRegion(region: *mut GpRegion, cloneregion: *mut *mut GpRegion) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCloneRegion(region : *mut GpRegion, cloneregion : *mut *mut GpRegion) -> Status);
    GdipCloneRegion(region, cloneregion)
}
#[inline]
pub unsafe fn GdipCloneStringFormat(format: *const GpStringFormat, newformat: *mut *mut GpStringFormat) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCloneStringFormat(format : *const GpStringFormat, newformat : *mut *mut GpStringFormat) -> Status);
    GdipCloneStringFormat(format, newformat)
}
#[inline]
pub unsafe fn GdipClosePathFigure(path: *mut GpPath) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipClosePathFigure(path : *mut GpPath) -> Status);
    GdipClosePathFigure(path)
}
#[inline]
pub unsafe fn GdipClosePathFigures(path: *mut GpPath) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipClosePathFigures(path : *mut GpPath) -> Status);
    GdipClosePathFigures(path)
}
#[inline]
pub unsafe fn GdipCombineRegionPath(region: *mut GpRegion, path: *mut GpPath, combinemode: CombineMode) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCombineRegionPath(region : *mut GpRegion, path : *mut GpPath, combinemode : CombineMode) -> Status);
    GdipCombineRegionPath(region, path, combinemode)
}
#[inline]
pub unsafe fn GdipCombineRegionRect(region: *mut GpRegion, rect: *const RectF, combinemode: CombineMode) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCombineRegionRect(region : *mut GpRegion, rect : *const RectF, combinemode : CombineMode) -> Status);
    GdipCombineRegionRect(region, rect, combinemode)
}
#[inline]
pub unsafe fn GdipCombineRegionRectI(region: *mut GpRegion, rect: *const Rect, combinemode: CombineMode) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCombineRegionRectI(region : *mut GpRegion, rect : *const Rect, combinemode : CombineMode) -> Status);
    GdipCombineRegionRectI(region, rect, combinemode)
}
#[inline]
pub unsafe fn GdipCombineRegionRegion(region: *mut GpRegion, region2: *mut GpRegion, combinemode: CombineMode) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCombineRegionRegion(region : *mut GpRegion, region2 : *mut GpRegion, combinemode : CombineMode) -> Status);
    GdipCombineRegionRegion(region, region2, combinemode)
}
#[inline]
pub unsafe fn GdipComment(graphics: *mut GpGraphics, sizedata: u32, data: *const u8) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipComment(graphics : *mut GpGraphics, sizedata : u32, data : *const u8) -> Status);
    GdipComment(graphics, sizedata, data)
}
#[inline]
pub unsafe fn GdipConvertToEmfPlus<P0>(refgraphics: *const GpGraphics, metafile: *mut GpMetafile, conversionfailureflag: *mut i32, emftype: EmfType, description: P0, out_metafile: *mut *mut GpMetafile) -> Status
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipConvertToEmfPlus(refgraphics : *const GpGraphics, metafile : *mut GpMetafile, conversionfailureflag : *mut i32, emftype : EmfType, description : windows_core::PCWSTR, out_metafile : *mut *mut GpMetafile) -> Status);
    GdipConvertToEmfPlus(refgraphics, metafile, conversionfailureflag, emftype, description.param().abi(), out_metafile)
}
#[inline]
pub unsafe fn GdipConvertToEmfPlusToFile<P0, P1>(refgraphics: *const GpGraphics, metafile: *mut GpMetafile, conversionfailureflag: *mut i32, filename: P0, emftype: EmfType, description: P1, out_metafile: *mut *mut GpMetafile) -> Status
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipConvertToEmfPlusToFile(refgraphics : *const GpGraphics, metafile : *mut GpMetafile, conversionfailureflag : *mut i32, filename : windows_core::PCWSTR, emftype : EmfType, description : windows_core::PCWSTR, out_metafile : *mut *mut GpMetafile) -> Status);
    GdipConvertToEmfPlusToFile(refgraphics, metafile, conversionfailureflag, filename.param().abi(), emftype, description.param().abi(), out_metafile)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn GdipConvertToEmfPlusToStream<P0, P1>(refgraphics: *const GpGraphics, metafile: *mut GpMetafile, conversionfailureflag: *mut i32, stream: P0, emftype: EmfType, description: P1, out_metafile: *mut *mut GpMetafile) -> Status
where
    P0: windows_core::Param<super::super::System::Com::IStream>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipConvertToEmfPlusToStream(refgraphics : *const GpGraphics, metafile : *mut GpMetafile, conversionfailureflag : *mut i32, stream : * mut core::ffi::c_void, emftype : EmfType, description : windows_core::PCWSTR, out_metafile : *mut *mut GpMetafile) -> Status);
    GdipConvertToEmfPlusToStream(refgraphics, metafile, conversionfailureflag, stream.param().abi(), emftype, description.param().abi(), out_metafile)
}
#[inline]
pub unsafe fn GdipCreateAdjustableArrowCap<P0>(height: f32, width: f32, isfilled: P0, cap: *mut *mut GpAdjustableArrowCap) -> Status
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateAdjustableArrowCap(height : f32, width : f32, isfilled : super::super::Foundation:: BOOL, cap : *mut *mut GpAdjustableArrowCap) -> Status);
    GdipCreateAdjustableArrowCap(height, width, isfilled.param().abi(), cap)
}
#[cfg(feature = "Win32_Graphics_DirectDraw")]
#[inline]
pub unsafe fn GdipCreateBitmapFromDirectDrawSurface<P0>(surface: P0, bitmap: *mut *mut GpBitmap) -> Status
where
    P0: windows_core::Param<super::DirectDraw::IDirectDrawSurface7>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateBitmapFromDirectDrawSurface(surface : * mut core::ffi::c_void, bitmap : *mut *mut GpBitmap) -> Status);
    GdipCreateBitmapFromDirectDrawSurface(surface.param().abi(), bitmap)
}
#[inline]
pub unsafe fn GdipCreateBitmapFromFile<P0>(filename: P0, bitmap: *mut *mut GpBitmap) -> Status
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateBitmapFromFile(filename : windows_core::PCWSTR, bitmap : *mut *mut GpBitmap) -> Status);
    GdipCreateBitmapFromFile(filename.param().abi(), bitmap)
}
#[inline]
pub unsafe fn GdipCreateBitmapFromFileICM<P0>(filename: P0, bitmap: *mut *mut GpBitmap) -> Status
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateBitmapFromFileICM(filename : windows_core::PCWSTR, bitmap : *mut *mut GpBitmap) -> Status);
    GdipCreateBitmapFromFileICM(filename.param().abi(), bitmap)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipCreateBitmapFromGdiDib(gdibitmapinfo: *const super::Gdi::BITMAPINFO, gdibitmapdata: *mut core::ffi::c_void, bitmap: *mut *mut GpBitmap) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateBitmapFromGdiDib(gdibitmapinfo : *const super::Gdi:: BITMAPINFO, gdibitmapdata : *mut core::ffi::c_void, bitmap : *mut *mut GpBitmap) -> Status);
    GdipCreateBitmapFromGdiDib(gdibitmapinfo, gdibitmapdata, bitmap)
}
#[inline]
pub unsafe fn GdipCreateBitmapFromGraphics(width: i32, height: i32, target: *mut GpGraphics, bitmap: *mut *mut GpBitmap) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateBitmapFromGraphics(width : i32, height : i32, target : *mut GpGraphics, bitmap : *mut *mut GpBitmap) -> Status);
    GdipCreateBitmapFromGraphics(width, height, target, bitmap)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipCreateBitmapFromHBITMAP<P0, P1>(hbm: P0, hpal: P1, bitmap: *mut *mut GpBitmap) -> Status
where
    P0: windows_core::Param<super::Gdi::HBITMAP>,
    P1: windows_core::Param<super::Gdi::HPALETTE>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateBitmapFromHBITMAP(hbm : super::Gdi:: HBITMAP, hpal : super::Gdi:: HPALETTE, bitmap : *mut *mut GpBitmap) -> Status);
    GdipCreateBitmapFromHBITMAP(hbm.param().abi(), hpal.param().abi(), bitmap)
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn GdipCreateBitmapFromHICON<P0>(hicon: P0, bitmap: *mut *mut GpBitmap) -> Status
where
    P0: windows_core::Param<super::super::UI::WindowsAndMessaging::HICON>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateBitmapFromHICON(hicon : super::super::UI::WindowsAndMessaging:: HICON, bitmap : *mut *mut GpBitmap) -> Status);
    GdipCreateBitmapFromHICON(hicon.param().abi(), bitmap)
}
#[inline]
pub unsafe fn GdipCreateBitmapFromResource<P0, P1>(hinstance: P0, lpbitmapname: P1, bitmap: *mut *mut GpBitmap) -> Status
where
    P0: windows_core::Param<super::super::Foundation::HINSTANCE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateBitmapFromResource(hinstance : super::super::Foundation:: HINSTANCE, lpbitmapname : windows_core::PCWSTR, bitmap : *mut *mut GpBitmap) -> Status);
    GdipCreateBitmapFromResource(hinstance.param().abi(), lpbitmapname.param().abi(), bitmap)
}
#[inline]
pub unsafe fn GdipCreateBitmapFromScan0(width: i32, height: i32, stride: i32, format: i32, scan0: Option<*const u8>, bitmap: *mut *mut GpBitmap) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateBitmapFromScan0(width : i32, height : i32, stride : i32, format : i32, scan0 : *const u8, bitmap : *mut *mut GpBitmap) -> Status);
    GdipCreateBitmapFromScan0(width, height, stride, format, core::mem::transmute(scan0.unwrap_or(std::ptr::null())), bitmap)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn GdipCreateBitmapFromStream<P0>(stream: P0, bitmap: *mut *mut GpBitmap) -> Status
where
    P0: windows_core::Param<super::super::System::Com::IStream>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateBitmapFromStream(stream : * mut core::ffi::c_void, bitmap : *mut *mut GpBitmap) -> Status);
    GdipCreateBitmapFromStream(stream.param().abi(), bitmap)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn GdipCreateBitmapFromStreamICM<P0>(stream: P0, bitmap: *mut *mut GpBitmap) -> Status
where
    P0: windows_core::Param<super::super::System::Com::IStream>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateBitmapFromStreamICM(stream : * mut core::ffi::c_void, bitmap : *mut *mut GpBitmap) -> Status);
    GdipCreateBitmapFromStreamICM(stream.param().abi(), bitmap)
}
#[inline]
pub unsafe fn GdipCreateCachedBitmap(bitmap: *mut GpBitmap, graphics: *mut GpGraphics, cachedbitmap: *mut *mut GpCachedBitmap) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateCachedBitmap(bitmap : *mut GpBitmap, graphics : *mut GpGraphics, cachedbitmap : *mut *mut GpCachedBitmap) -> Status);
    GdipCreateCachedBitmap(bitmap, graphics, cachedbitmap)
}
#[inline]
pub unsafe fn GdipCreateCustomLineCap(fillpath: *mut GpPath, strokepath: *mut GpPath, basecap: LineCap, baseinset: f32, customcap: *mut *mut GpCustomLineCap) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateCustomLineCap(fillpath : *mut GpPath, strokepath : *mut GpPath, basecap : LineCap, baseinset : f32, customcap : *mut *mut GpCustomLineCap) -> Status);
    GdipCreateCustomLineCap(fillpath, strokepath, basecap, baseinset, customcap)
}
#[inline]
pub unsafe fn GdipCreateEffect(guid: windows_core::GUID, effect: *mut *mut CGpEffect) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateEffect(guid : windows_core::GUID, effect : *mut *mut CGpEffect) -> Status);
    GdipCreateEffect(core::mem::transmute(guid), effect)
}
#[inline]
pub unsafe fn GdipCreateFont(fontfamily: *const GpFontFamily, emsize: f32, style: i32, unit: Unit, font: *mut *mut GpFont) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateFont(fontfamily : *const GpFontFamily, emsize : f32, style : i32, unit : Unit, font : *mut *mut GpFont) -> Status);
    GdipCreateFont(fontfamily, emsize, style, unit, font)
}
#[inline]
pub unsafe fn GdipCreateFontFamilyFromName<P0>(name: P0, fontcollection: *mut GpFontCollection, fontfamily: *mut *mut GpFontFamily) -> Status
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateFontFamilyFromName(name : windows_core::PCWSTR, fontcollection : *mut GpFontCollection, fontfamily : *mut *mut GpFontFamily) -> Status);
    GdipCreateFontFamilyFromName(name.param().abi(), fontcollection, fontfamily)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipCreateFontFromDC<P0>(hdc: P0, font: *mut *mut GpFont) -> Status
where
    P0: windows_core::Param<super::Gdi::HDC>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateFontFromDC(hdc : super::Gdi:: HDC, font : *mut *mut GpFont) -> Status);
    GdipCreateFontFromDC(hdc.param().abi(), font)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipCreateFontFromLogfontA<P0>(hdc: P0, logfont: *const super::Gdi::LOGFONTA, font: *mut *mut GpFont) -> Status
where
    P0: windows_core::Param<super::Gdi::HDC>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateFontFromLogfontA(hdc : super::Gdi:: HDC, logfont : *const super::Gdi:: LOGFONTA, font : *mut *mut GpFont) -> Status);
    GdipCreateFontFromLogfontA(hdc.param().abi(), logfont, font)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipCreateFontFromLogfontW<P0>(hdc: P0, logfont: *const super::Gdi::LOGFONTW, font: *mut *mut GpFont) -> Status
where
    P0: windows_core::Param<super::Gdi::HDC>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateFontFromLogfontW(hdc : super::Gdi:: HDC, logfont : *const super::Gdi:: LOGFONTW, font : *mut *mut GpFont) -> Status);
    GdipCreateFontFromLogfontW(hdc.param().abi(), logfont, font)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipCreateFromHDC<P0>(hdc: P0, graphics: *mut *mut GpGraphics) -> Status
where
    P0: windows_core::Param<super::Gdi::HDC>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateFromHDC(hdc : super::Gdi:: HDC, graphics : *mut *mut GpGraphics) -> Status);
    GdipCreateFromHDC(hdc.param().abi(), graphics)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipCreateFromHDC2<P0, P1>(hdc: P0, hdevice: P1, graphics: *mut *mut GpGraphics) -> Status
where
    P0: windows_core::Param<super::Gdi::HDC>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateFromHDC2(hdc : super::Gdi:: HDC, hdevice : super::super::Foundation:: HANDLE, graphics : *mut *mut GpGraphics) -> Status);
    GdipCreateFromHDC2(hdc.param().abi(), hdevice.param().abi(), graphics)
}
#[inline]
pub unsafe fn GdipCreateFromHWND<P0>(hwnd: P0, graphics: *mut *mut GpGraphics) -> Status
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateFromHWND(hwnd : super::super::Foundation:: HWND, graphics : *mut *mut GpGraphics) -> Status);
    GdipCreateFromHWND(hwnd.param().abi(), graphics)
}
#[inline]
pub unsafe fn GdipCreateFromHWNDICM<P0>(hwnd: P0, graphics: *mut *mut GpGraphics) -> Status
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateFromHWNDICM(hwnd : super::super::Foundation:: HWND, graphics : *mut *mut GpGraphics) -> Status);
    GdipCreateFromHWNDICM(hwnd.param().abi(), graphics)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipCreateHBITMAPFromBitmap(bitmap: *mut GpBitmap, hbmreturn: *mut super::Gdi::HBITMAP, background: u32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateHBITMAPFromBitmap(bitmap : *mut GpBitmap, hbmreturn : *mut super::Gdi:: HBITMAP, background : u32) -> Status);
    GdipCreateHBITMAPFromBitmap(bitmap, hbmreturn, background)
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn GdipCreateHICONFromBitmap(bitmap: *mut GpBitmap, hbmreturn: *mut super::super::UI::WindowsAndMessaging::HICON) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateHICONFromBitmap(bitmap : *mut GpBitmap, hbmreturn : *mut super::super::UI::WindowsAndMessaging:: HICON) -> Status);
    GdipCreateHICONFromBitmap(bitmap, hbmreturn)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipCreateHalftonePalette() -> super::Gdi::HPALETTE {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateHalftonePalette() -> super::Gdi:: HPALETTE);
    GdipCreateHalftonePalette()
}
#[inline]
pub unsafe fn GdipCreateHatchBrush(hatchstyle: HatchStyle, forecol: u32, backcol: u32, brush: *mut *mut GpHatch) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateHatchBrush(hatchstyle : HatchStyle, forecol : u32, backcol : u32, brush : *mut *mut GpHatch) -> Status);
    GdipCreateHatchBrush(hatchstyle, forecol, backcol, brush)
}
#[inline]
pub unsafe fn GdipCreateImageAttributes(imageattr: *mut *mut GpImageAttributes) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateImageAttributes(imageattr : *mut *mut GpImageAttributes) -> Status);
    GdipCreateImageAttributes(imageattr)
}
#[inline]
pub unsafe fn GdipCreateLineBrush(point1: *const PointF, point2: *const PointF, color1: u32, color2: u32, wrapmode: WrapMode, linegradient: *mut *mut GpLineGradient) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateLineBrush(point1 : *const PointF, point2 : *const PointF, color1 : u32, color2 : u32, wrapmode : WrapMode, linegradient : *mut *mut GpLineGradient) -> Status);
    GdipCreateLineBrush(point1, point2, color1, color2, wrapmode, linegradient)
}
#[inline]
pub unsafe fn GdipCreateLineBrushFromRect(rect: *const RectF, color1: u32, color2: u32, mode: LinearGradientMode, wrapmode: WrapMode, linegradient: *mut *mut GpLineGradient) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateLineBrushFromRect(rect : *const RectF, color1 : u32, color2 : u32, mode : LinearGradientMode, wrapmode : WrapMode, linegradient : *mut *mut GpLineGradient) -> Status);
    GdipCreateLineBrushFromRect(rect, color1, color2, mode, wrapmode, linegradient)
}
#[inline]
pub unsafe fn GdipCreateLineBrushFromRectI(rect: *const Rect, color1: u32, color2: u32, mode: LinearGradientMode, wrapmode: WrapMode, linegradient: *mut *mut GpLineGradient) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateLineBrushFromRectI(rect : *const Rect, color1 : u32, color2 : u32, mode : LinearGradientMode, wrapmode : WrapMode, linegradient : *mut *mut GpLineGradient) -> Status);
    GdipCreateLineBrushFromRectI(rect, color1, color2, mode, wrapmode, linegradient)
}
#[inline]
pub unsafe fn GdipCreateLineBrushFromRectWithAngle<P0>(rect: *const RectF, color1: u32, color2: u32, angle: f32, isanglescalable: P0, wrapmode: WrapMode, linegradient: *mut *mut GpLineGradient) -> Status
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateLineBrushFromRectWithAngle(rect : *const RectF, color1 : u32, color2 : u32, angle : f32, isanglescalable : super::super::Foundation:: BOOL, wrapmode : WrapMode, linegradient : *mut *mut GpLineGradient) -> Status);
    GdipCreateLineBrushFromRectWithAngle(rect, color1, color2, angle, isanglescalable.param().abi(), wrapmode, linegradient)
}
#[inline]
pub unsafe fn GdipCreateLineBrushFromRectWithAngleI<P0>(rect: *const Rect, color1: u32, color2: u32, angle: f32, isanglescalable: P0, wrapmode: WrapMode, linegradient: *mut *mut GpLineGradient) -> Status
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateLineBrushFromRectWithAngleI(rect : *const Rect, color1 : u32, color2 : u32, angle : f32, isanglescalable : super::super::Foundation:: BOOL, wrapmode : WrapMode, linegradient : *mut *mut GpLineGradient) -> Status);
    GdipCreateLineBrushFromRectWithAngleI(rect, color1, color2, angle, isanglescalable.param().abi(), wrapmode, linegradient)
}
#[inline]
pub unsafe fn GdipCreateLineBrushI(point1: *const Point, point2: *const Point, color1: u32, color2: u32, wrapmode: WrapMode, linegradient: *mut *mut GpLineGradient) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateLineBrushI(point1 : *const Point, point2 : *const Point, color1 : u32, color2 : u32, wrapmode : WrapMode, linegradient : *mut *mut GpLineGradient) -> Status);
    GdipCreateLineBrushI(point1, point2, color1, color2, wrapmode, linegradient)
}
#[inline]
pub unsafe fn GdipCreateMatrix(matrix: *mut *mut Matrix) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateMatrix(matrix : *mut *mut Matrix) -> Status);
    GdipCreateMatrix(matrix)
}
#[inline]
pub unsafe fn GdipCreateMatrix2(m11: f32, m12: f32, m21: f32, m22: f32, dx: f32, dy: f32, matrix: *mut *mut Matrix) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateMatrix2(m11 : f32, m12 : f32, m21 : f32, m22 : f32, dx : f32, dy : f32, matrix : *mut *mut Matrix) -> Status);
    GdipCreateMatrix2(m11, m12, m21, m22, dx, dy, matrix)
}
#[inline]
pub unsafe fn GdipCreateMatrix3(rect: *const RectF, dstplg: *const PointF, matrix: *mut *mut Matrix) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateMatrix3(rect : *const RectF, dstplg : *const PointF, matrix : *mut *mut Matrix) -> Status);
    GdipCreateMatrix3(rect, dstplg, matrix)
}
#[inline]
pub unsafe fn GdipCreateMatrix3I(rect: *const Rect, dstplg: *const Point, matrix: *mut *mut Matrix) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateMatrix3I(rect : *const Rect, dstplg : *const Point, matrix : *mut *mut Matrix) -> Status);
    GdipCreateMatrix3I(rect, dstplg, matrix)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipCreateMetafileFromEmf<P0, P1>(hemf: P0, deleteemf: P1, metafile: *mut *mut GpMetafile) -> Status
where
    P0: windows_core::Param<super::Gdi::HENHMETAFILE>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateMetafileFromEmf(hemf : super::Gdi:: HENHMETAFILE, deleteemf : super::super::Foundation:: BOOL, metafile : *mut *mut GpMetafile) -> Status);
    GdipCreateMetafileFromEmf(hemf.param().abi(), deleteemf.param().abi(), metafile)
}
#[inline]
pub unsafe fn GdipCreateMetafileFromFile<P0>(file: P0, metafile: *mut *mut GpMetafile) -> Status
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateMetafileFromFile(file : windows_core::PCWSTR, metafile : *mut *mut GpMetafile) -> Status);
    GdipCreateMetafileFromFile(file.param().abi(), metafile)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn GdipCreateMetafileFromStream<P0>(stream: P0, metafile: *mut *mut GpMetafile) -> Status
where
    P0: windows_core::Param<super::super::System::Com::IStream>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateMetafileFromStream(stream : * mut core::ffi::c_void, metafile : *mut *mut GpMetafile) -> Status);
    GdipCreateMetafileFromStream(stream.param().abi(), metafile)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipCreateMetafileFromWmf<P0, P1>(hwmf: P0, deletewmf: P1, wmfplaceablefileheader: *const WmfPlaceableFileHeader, metafile: *mut *mut GpMetafile) -> Status
where
    P0: windows_core::Param<super::Gdi::HMETAFILE>,
    P1: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateMetafileFromWmf(hwmf : super::Gdi:: HMETAFILE, deletewmf : super::super::Foundation:: BOOL, wmfplaceablefileheader : *const WmfPlaceableFileHeader, metafile : *mut *mut GpMetafile) -> Status);
    GdipCreateMetafileFromWmf(hwmf.param().abi(), deletewmf.param().abi(), wmfplaceablefileheader, metafile)
}
#[inline]
pub unsafe fn GdipCreateMetafileFromWmfFile<P0>(file: P0, wmfplaceablefileheader: *const WmfPlaceableFileHeader, metafile: *mut *mut GpMetafile) -> Status
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateMetafileFromWmfFile(file : windows_core::PCWSTR, wmfplaceablefileheader : *const WmfPlaceableFileHeader, metafile : *mut *mut GpMetafile) -> Status);
    GdipCreateMetafileFromWmfFile(file.param().abi(), wmfplaceablefileheader, metafile)
}
#[inline]
pub unsafe fn GdipCreatePath(brushmode: FillMode, path: *mut *mut GpPath) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreatePath(brushmode : FillMode, path : *mut *mut GpPath) -> Status);
    GdipCreatePath(brushmode, path)
}
#[inline]
pub unsafe fn GdipCreatePath2(param0: *const PointF, param1: *const u8, param2: i32, param3: FillMode, path: *mut *mut GpPath) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreatePath2(param0 : *const PointF, param1 : *const u8, param2 : i32, param3 : FillMode, path : *mut *mut GpPath) -> Status);
    GdipCreatePath2(param0, param1, param2, param3, path)
}
#[inline]
pub unsafe fn GdipCreatePath2I(param0: *const Point, param1: *const u8, param2: i32, param3: FillMode, path: *mut *mut GpPath) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreatePath2I(param0 : *const Point, param1 : *const u8, param2 : i32, param3 : FillMode, path : *mut *mut GpPath) -> Status);
    GdipCreatePath2I(param0, param1, param2, param3, path)
}
#[inline]
pub unsafe fn GdipCreatePathGradient(points: *const PointF, count: i32, wrapmode: WrapMode, polygradient: *mut *mut GpPathGradient) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreatePathGradient(points : *const PointF, count : i32, wrapmode : WrapMode, polygradient : *mut *mut GpPathGradient) -> Status);
    GdipCreatePathGradient(points, count, wrapmode, polygradient)
}
#[inline]
pub unsafe fn GdipCreatePathGradientFromPath(path: *const GpPath, polygradient: *mut *mut GpPathGradient) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreatePathGradientFromPath(path : *const GpPath, polygradient : *mut *mut GpPathGradient) -> Status);
    GdipCreatePathGradientFromPath(path, polygradient)
}
#[inline]
pub unsafe fn GdipCreatePathGradientI(points: *const Point, count: i32, wrapmode: WrapMode, polygradient: *mut *mut GpPathGradient) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreatePathGradientI(points : *const Point, count : i32, wrapmode : WrapMode, polygradient : *mut *mut GpPathGradient) -> Status);
    GdipCreatePathGradientI(points, count, wrapmode, polygradient)
}
#[inline]
pub unsafe fn GdipCreatePathIter(iterator: *mut *mut GpPathIterator, path: *mut GpPath) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreatePathIter(iterator : *mut *mut GpPathIterator, path : *mut GpPath) -> Status);
    GdipCreatePathIter(iterator, path)
}
#[inline]
pub unsafe fn GdipCreatePen1(color: u32, width: f32, unit: Unit, pen: *mut *mut GpPen) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreatePen1(color : u32, width : f32, unit : Unit, pen : *mut *mut GpPen) -> Status);
    GdipCreatePen1(color, width, unit, pen)
}
#[inline]
pub unsafe fn GdipCreatePen2(brush: *mut GpBrush, width: f32, unit: Unit, pen: *mut *mut GpPen) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreatePen2(brush : *mut GpBrush, width : f32, unit : Unit, pen : *mut *mut GpPen) -> Status);
    GdipCreatePen2(brush, width, unit, pen)
}
#[inline]
pub unsafe fn GdipCreateRegion(region: *mut *mut GpRegion) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateRegion(region : *mut *mut GpRegion) -> Status);
    GdipCreateRegion(region)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipCreateRegionHrgn<P0>(hrgn: P0, region: *mut *mut GpRegion) -> Status
where
    P0: windows_core::Param<super::Gdi::HRGN>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateRegionHrgn(hrgn : super::Gdi:: HRGN, region : *mut *mut GpRegion) -> Status);
    GdipCreateRegionHrgn(hrgn.param().abi(), region)
}
#[inline]
pub unsafe fn GdipCreateRegionPath(path: *mut GpPath, region: *mut *mut GpRegion) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateRegionPath(path : *mut GpPath, region : *mut *mut GpRegion) -> Status);
    GdipCreateRegionPath(path, region)
}
#[inline]
pub unsafe fn GdipCreateRegionRect(rect: *const RectF, region: *mut *mut GpRegion) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateRegionRect(rect : *const RectF, region : *mut *mut GpRegion) -> Status);
    GdipCreateRegionRect(rect, region)
}
#[inline]
pub unsafe fn GdipCreateRegionRectI(rect: *const Rect, region: *mut *mut GpRegion) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateRegionRectI(rect : *const Rect, region : *mut *mut GpRegion) -> Status);
    GdipCreateRegionRectI(rect, region)
}
#[inline]
pub unsafe fn GdipCreateRegionRgnData(regiondata: *const u8, size: i32, region: *mut *mut GpRegion) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateRegionRgnData(regiondata : *const u8, size : i32, region : *mut *mut GpRegion) -> Status);
    GdipCreateRegionRgnData(regiondata, size, region)
}
#[inline]
pub unsafe fn GdipCreateSolidFill(color: u32, brush: *mut *mut GpSolidFill) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateSolidFill(color : u32, brush : *mut *mut GpSolidFill) -> Status);
    GdipCreateSolidFill(color, brush)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn GdipCreateStreamOnFile<P0>(filename: P0, access: u32, stream: *mut Option<super::super::System::Com::IStream>) -> Status
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateStreamOnFile(filename : windows_core::PCWSTR, access : u32, stream : *mut * mut core::ffi::c_void) -> Status);
    GdipCreateStreamOnFile(filename.param().abi(), access, core::mem::transmute(stream))
}
#[inline]
pub unsafe fn GdipCreateStringFormat(formatattributes: i32, language: u16, format: *mut *mut GpStringFormat) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateStringFormat(formatattributes : i32, language : u16, format : *mut *mut GpStringFormat) -> Status);
    GdipCreateStringFormat(formatattributes, language, format)
}
#[inline]
pub unsafe fn GdipCreateTexture(image: *mut GpImage, wrapmode: WrapMode, texture: *mut *mut GpTexture) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateTexture(image : *mut GpImage, wrapmode : WrapMode, texture : *mut *mut GpTexture) -> Status);
    GdipCreateTexture(image, wrapmode, texture)
}
#[inline]
pub unsafe fn GdipCreateTexture2(image: *mut GpImage, wrapmode: WrapMode, x: f32, y: f32, width: f32, height: f32, texture: *mut *mut GpTexture) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateTexture2(image : *mut GpImage, wrapmode : WrapMode, x : f32, y : f32, width : f32, height : f32, texture : *mut *mut GpTexture) -> Status);
    GdipCreateTexture2(image, wrapmode, x, y, width, height, texture)
}
#[inline]
pub unsafe fn GdipCreateTexture2I(image: *mut GpImage, wrapmode: WrapMode, x: i32, y: i32, width: i32, height: i32, texture: *mut *mut GpTexture) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateTexture2I(image : *mut GpImage, wrapmode : WrapMode, x : i32, y : i32, width : i32, height : i32, texture : *mut *mut GpTexture) -> Status);
    GdipCreateTexture2I(image, wrapmode, x, y, width, height, texture)
}
#[inline]
pub unsafe fn GdipCreateTextureIA(image: *mut GpImage, imageattributes: *const GpImageAttributes, x: f32, y: f32, width: f32, height: f32, texture: *mut *mut GpTexture) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateTextureIA(image : *mut GpImage, imageattributes : *const GpImageAttributes, x : f32, y : f32, width : f32, height : f32, texture : *mut *mut GpTexture) -> Status);
    GdipCreateTextureIA(image, imageattributes, x, y, width, height, texture)
}
#[inline]
pub unsafe fn GdipCreateTextureIAI(image: *mut GpImage, imageattributes: *const GpImageAttributes, x: i32, y: i32, width: i32, height: i32, texture: *mut *mut GpTexture) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipCreateTextureIAI(image : *mut GpImage, imageattributes : *const GpImageAttributes, x : i32, y : i32, width : i32, height : i32, texture : *mut *mut GpTexture) -> Status);
    GdipCreateTextureIAI(image, imageattributes, x, y, width, height, texture)
}
#[inline]
pub unsafe fn GdipDeleteBrush(brush: *mut GpBrush) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDeleteBrush(brush : *mut GpBrush) -> Status);
    GdipDeleteBrush(brush)
}
#[inline]
pub unsafe fn GdipDeleteCachedBitmap(cachedbitmap: *mut GpCachedBitmap) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDeleteCachedBitmap(cachedbitmap : *mut GpCachedBitmap) -> Status);
    GdipDeleteCachedBitmap(cachedbitmap)
}
#[inline]
pub unsafe fn GdipDeleteCustomLineCap(customcap: *mut GpCustomLineCap) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDeleteCustomLineCap(customcap : *mut GpCustomLineCap) -> Status);
    GdipDeleteCustomLineCap(customcap)
}
#[inline]
pub unsafe fn GdipDeleteEffect(effect: *mut CGpEffect) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDeleteEffect(effect : *mut CGpEffect) -> Status);
    GdipDeleteEffect(effect)
}
#[inline]
pub unsafe fn GdipDeleteFont(font: *mut GpFont) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDeleteFont(font : *mut GpFont) -> Status);
    GdipDeleteFont(font)
}
#[inline]
pub unsafe fn GdipDeleteFontFamily(fontfamily: *mut GpFontFamily) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDeleteFontFamily(fontfamily : *mut GpFontFamily) -> Status);
    GdipDeleteFontFamily(fontfamily)
}
#[inline]
pub unsafe fn GdipDeleteGraphics(graphics: *mut GpGraphics) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDeleteGraphics(graphics : *mut GpGraphics) -> Status);
    GdipDeleteGraphics(graphics)
}
#[inline]
pub unsafe fn GdipDeleteMatrix(matrix: *mut Matrix) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDeleteMatrix(matrix : *mut Matrix) -> Status);
    GdipDeleteMatrix(matrix)
}
#[inline]
pub unsafe fn GdipDeletePath(path: *mut GpPath) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDeletePath(path : *mut GpPath) -> Status);
    GdipDeletePath(path)
}
#[inline]
pub unsafe fn GdipDeletePathIter(iterator: *mut GpPathIterator) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDeletePathIter(iterator : *mut GpPathIterator) -> Status);
    GdipDeletePathIter(iterator)
}
#[inline]
pub unsafe fn GdipDeletePen(pen: *mut GpPen) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDeletePen(pen : *mut GpPen) -> Status);
    GdipDeletePen(pen)
}
#[inline]
pub unsafe fn GdipDeletePrivateFontCollection(fontcollection: *mut *mut GpFontCollection) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDeletePrivateFontCollection(fontcollection : *mut *mut GpFontCollection) -> Status);
    GdipDeletePrivateFontCollection(fontcollection)
}
#[inline]
pub unsafe fn GdipDeleteRegion(region: *mut GpRegion) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDeleteRegion(region : *mut GpRegion) -> Status);
    GdipDeleteRegion(region)
}
#[inline]
pub unsafe fn GdipDeleteStringFormat(format: *mut GpStringFormat) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDeleteStringFormat(format : *mut GpStringFormat) -> Status);
    GdipDeleteStringFormat(format)
}
#[inline]
pub unsafe fn GdipDisposeImage(image: *mut GpImage) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDisposeImage(image : *mut GpImage) -> Status);
    GdipDisposeImage(image)
}
#[inline]
pub unsafe fn GdipDisposeImageAttributes(imageattr: *mut GpImageAttributes) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDisposeImageAttributes(imageattr : *mut GpImageAttributes) -> Status);
    GdipDisposeImageAttributes(imageattr)
}
#[inline]
pub unsafe fn GdipDrawArc(graphics: *mut GpGraphics, pen: *mut GpPen, x: f32, y: f32, width: f32, height: f32, startangle: f32, sweepangle: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDrawArc(graphics : *mut GpGraphics, pen : *mut GpPen, x : f32, y : f32, width : f32, height : f32, startangle : f32, sweepangle : f32) -> Status);
    GdipDrawArc(graphics, pen, x, y, width, height, startangle, sweepangle)
}
#[inline]
pub unsafe fn GdipDrawArcI(graphics: *mut GpGraphics, pen: *mut GpPen, x: i32, y: i32, width: i32, height: i32, startangle: f32, sweepangle: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDrawArcI(graphics : *mut GpGraphics, pen : *mut GpPen, x : i32, y : i32, width : i32, height : i32, startangle : f32, sweepangle : f32) -> Status);
    GdipDrawArcI(graphics, pen, x, y, width, height, startangle, sweepangle)
}
#[inline]
pub unsafe fn GdipDrawBezier(graphics: *mut GpGraphics, pen: *mut GpPen, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, x4: f32, y4: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDrawBezier(graphics : *mut GpGraphics, pen : *mut GpPen, x1 : f32, y1 : f32, x2 : f32, y2 : f32, x3 : f32, y3 : f32, x4 : f32, y4 : f32) -> Status);
    GdipDrawBezier(graphics, pen, x1, y1, x2, y2, x3, y3, x4, y4)
}
#[inline]
pub unsafe fn GdipDrawBezierI(graphics: *mut GpGraphics, pen: *mut GpPen, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, x4: i32, y4: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDrawBezierI(graphics : *mut GpGraphics, pen : *mut GpPen, x1 : i32, y1 : i32, x2 : i32, y2 : i32, x3 : i32, y3 : i32, x4 : i32, y4 : i32) -> Status);
    GdipDrawBezierI(graphics, pen, x1, y1, x2, y2, x3, y3, x4, y4)
}
#[inline]
pub unsafe fn GdipDrawBeziers(graphics: *mut GpGraphics, pen: *mut GpPen, points: *const PointF, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDrawBeziers(graphics : *mut GpGraphics, pen : *mut GpPen, points : *const PointF, count : i32) -> Status);
    GdipDrawBeziers(graphics, pen, points, count)
}
#[inline]
pub unsafe fn GdipDrawBeziersI(graphics: *mut GpGraphics, pen: *mut GpPen, points: *const Point, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDrawBeziersI(graphics : *mut GpGraphics, pen : *mut GpPen, points : *const Point, count : i32) -> Status);
    GdipDrawBeziersI(graphics, pen, points, count)
}
#[inline]
pub unsafe fn GdipDrawCachedBitmap(graphics: *mut GpGraphics, cachedbitmap: *mut GpCachedBitmap, x: i32, y: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDrawCachedBitmap(graphics : *mut GpGraphics, cachedbitmap : *mut GpCachedBitmap, x : i32, y : i32) -> Status);
    GdipDrawCachedBitmap(graphics, cachedbitmap, x, y)
}
#[inline]
pub unsafe fn GdipDrawClosedCurve(graphics: *mut GpGraphics, pen: *mut GpPen, points: *const PointF, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDrawClosedCurve(graphics : *mut GpGraphics, pen : *mut GpPen, points : *const PointF, count : i32) -> Status);
    GdipDrawClosedCurve(graphics, pen, points, count)
}
#[inline]
pub unsafe fn GdipDrawClosedCurve2(graphics: *mut GpGraphics, pen: *mut GpPen, points: *const PointF, count: i32, tension: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDrawClosedCurve2(graphics : *mut GpGraphics, pen : *mut GpPen, points : *const PointF, count : i32, tension : f32) -> Status);
    GdipDrawClosedCurve2(graphics, pen, points, count, tension)
}
#[inline]
pub unsafe fn GdipDrawClosedCurve2I(graphics: *mut GpGraphics, pen: *mut GpPen, points: *const Point, count: i32, tension: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDrawClosedCurve2I(graphics : *mut GpGraphics, pen : *mut GpPen, points : *const Point, count : i32, tension : f32) -> Status);
    GdipDrawClosedCurve2I(graphics, pen, points, count, tension)
}
#[inline]
pub unsafe fn GdipDrawClosedCurveI(graphics: *mut GpGraphics, pen: *mut GpPen, points: *const Point, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDrawClosedCurveI(graphics : *mut GpGraphics, pen : *mut GpPen, points : *const Point, count : i32) -> Status);
    GdipDrawClosedCurveI(graphics, pen, points, count)
}
#[inline]
pub unsafe fn GdipDrawCurve(graphics: *mut GpGraphics, pen: *mut GpPen, points: *const PointF, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDrawCurve(graphics : *mut GpGraphics, pen : *mut GpPen, points : *const PointF, count : i32) -> Status);
    GdipDrawCurve(graphics, pen, points, count)
}
#[inline]
pub unsafe fn GdipDrawCurve2(graphics: *mut GpGraphics, pen: *mut GpPen, points: *const PointF, count: i32, tension: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDrawCurve2(graphics : *mut GpGraphics, pen : *mut GpPen, points : *const PointF, count : i32, tension : f32) -> Status);
    GdipDrawCurve2(graphics, pen, points, count, tension)
}
#[inline]
pub unsafe fn GdipDrawCurve2I(graphics: *mut GpGraphics, pen: *mut GpPen, points: *const Point, count: i32, tension: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDrawCurve2I(graphics : *mut GpGraphics, pen : *mut GpPen, points : *const Point, count : i32, tension : f32) -> Status);
    GdipDrawCurve2I(graphics, pen, points, count, tension)
}
#[inline]
pub unsafe fn GdipDrawCurve3(graphics: *mut GpGraphics, pen: *mut GpPen, points: *const PointF, count: i32, offset: i32, numberofsegments: i32, tension: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDrawCurve3(graphics : *mut GpGraphics, pen : *mut GpPen, points : *const PointF, count : i32, offset : i32, numberofsegments : i32, tension : f32) -> Status);
    GdipDrawCurve3(graphics, pen, points, count, offset, numberofsegments, tension)
}
#[inline]
pub unsafe fn GdipDrawCurve3I(graphics: *mut GpGraphics, pen: *mut GpPen, points: *const Point, count: i32, offset: i32, numberofsegments: i32, tension: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDrawCurve3I(graphics : *mut GpGraphics, pen : *mut GpPen, points : *const Point, count : i32, offset : i32, numberofsegments : i32, tension : f32) -> Status);
    GdipDrawCurve3I(graphics, pen, points, count, offset, numberofsegments, tension)
}
#[inline]
pub unsafe fn GdipDrawCurveI(graphics: *mut GpGraphics, pen: *mut GpPen, points: *const Point, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDrawCurveI(graphics : *mut GpGraphics, pen : *mut GpPen, points : *const Point, count : i32) -> Status);
    GdipDrawCurveI(graphics, pen, points, count)
}
#[inline]
pub unsafe fn GdipDrawDriverString(graphics: *mut GpGraphics, text: *const u16, length: i32, font: *const GpFont, brush: *const GpBrush, positions: *const PointF, flags: i32, matrix: *const Matrix) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDrawDriverString(graphics : *mut GpGraphics, text : *const u16, length : i32, font : *const GpFont, brush : *const GpBrush, positions : *const PointF, flags : i32, matrix : *const Matrix) -> Status);
    GdipDrawDriverString(graphics, text, length, font, brush, positions, flags, matrix)
}
#[inline]
pub unsafe fn GdipDrawEllipse(graphics: *mut GpGraphics, pen: *mut GpPen, x: f32, y: f32, width: f32, height: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDrawEllipse(graphics : *mut GpGraphics, pen : *mut GpPen, x : f32, y : f32, width : f32, height : f32) -> Status);
    GdipDrawEllipse(graphics, pen, x, y, width, height)
}
#[inline]
pub unsafe fn GdipDrawEllipseI(graphics: *mut GpGraphics, pen: *mut GpPen, x: i32, y: i32, width: i32, height: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDrawEllipseI(graphics : *mut GpGraphics, pen : *mut GpPen, x : i32, y : i32, width : i32, height : i32) -> Status);
    GdipDrawEllipseI(graphics, pen, x, y, width, height)
}
#[inline]
pub unsafe fn GdipDrawImage(graphics: *mut GpGraphics, image: *mut GpImage, x: f32, y: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDrawImage(graphics : *mut GpGraphics, image : *mut GpImage, x : f32, y : f32) -> Status);
    GdipDrawImage(graphics, image, x, y)
}
#[inline]
pub unsafe fn GdipDrawImageFX(graphics: *mut GpGraphics, image: *mut GpImage, source: *mut RectF, xform: *mut Matrix, effect: *mut CGpEffect, imageattributes: *mut GpImageAttributes, srcunit: Unit) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDrawImageFX(graphics : *mut GpGraphics, image : *mut GpImage, source : *mut RectF, xform : *mut Matrix, effect : *mut CGpEffect, imageattributes : *mut GpImageAttributes, srcunit : Unit) -> Status);
    GdipDrawImageFX(graphics, image, source, xform, effect, imageattributes, srcunit)
}
#[inline]
pub unsafe fn GdipDrawImageI(graphics: *mut GpGraphics, image: *mut GpImage, x: i32, y: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDrawImageI(graphics : *mut GpGraphics, image : *mut GpImage, x : i32, y : i32) -> Status);
    GdipDrawImageI(graphics, image, x, y)
}
#[inline]
pub unsafe fn GdipDrawImagePointRect(graphics: *mut GpGraphics, image: *mut GpImage, x: f32, y: f32, srcx: f32, srcy: f32, srcwidth: f32, srcheight: f32, srcunit: Unit) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDrawImagePointRect(graphics : *mut GpGraphics, image : *mut GpImage, x : f32, y : f32, srcx : f32, srcy : f32, srcwidth : f32, srcheight : f32, srcunit : Unit) -> Status);
    GdipDrawImagePointRect(graphics, image, x, y, srcx, srcy, srcwidth, srcheight, srcunit)
}
#[inline]
pub unsafe fn GdipDrawImagePointRectI(graphics: *mut GpGraphics, image: *mut GpImage, x: i32, y: i32, srcx: i32, srcy: i32, srcwidth: i32, srcheight: i32, srcunit: Unit) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDrawImagePointRectI(graphics : *mut GpGraphics, image : *mut GpImage, x : i32, y : i32, srcx : i32, srcy : i32, srcwidth : i32, srcheight : i32, srcunit : Unit) -> Status);
    GdipDrawImagePointRectI(graphics, image, x, y, srcx, srcy, srcwidth, srcheight, srcunit)
}
#[inline]
pub unsafe fn GdipDrawImagePoints(graphics: *mut GpGraphics, image: *mut GpImage, dstpoints: *const PointF, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDrawImagePoints(graphics : *mut GpGraphics, image : *mut GpImage, dstpoints : *const PointF, count : i32) -> Status);
    GdipDrawImagePoints(graphics, image, dstpoints, count)
}
#[inline]
pub unsafe fn GdipDrawImagePointsI(graphics: *mut GpGraphics, image: *mut GpImage, dstpoints: *const Point, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDrawImagePointsI(graphics : *mut GpGraphics, image : *mut GpImage, dstpoints : *const Point, count : i32) -> Status);
    GdipDrawImagePointsI(graphics, image, dstpoints, count)
}
#[inline]
pub unsafe fn GdipDrawImagePointsRect(graphics: *mut GpGraphics, image: *mut GpImage, points: *const PointF, count: i32, srcx: f32, srcy: f32, srcwidth: f32, srcheight: f32, srcunit: Unit, imageattributes: *const GpImageAttributes, callback: isize, callbackdata: *mut core::ffi::c_void) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDrawImagePointsRect(graphics : *mut GpGraphics, image : *mut GpImage, points : *const PointF, count : i32, srcx : f32, srcy : f32, srcwidth : f32, srcheight : f32, srcunit : Unit, imageattributes : *const GpImageAttributes, callback : isize, callbackdata : *mut core::ffi::c_void) -> Status);
    GdipDrawImagePointsRect(graphics, image, points, count, srcx, srcy, srcwidth, srcheight, srcunit, imageattributes, callback, callbackdata)
}
#[inline]
pub unsafe fn GdipDrawImagePointsRectI(graphics: *mut GpGraphics, image: *mut GpImage, points: *const Point, count: i32, srcx: i32, srcy: i32, srcwidth: i32, srcheight: i32, srcunit: Unit, imageattributes: *const GpImageAttributes, callback: isize, callbackdata: *mut core::ffi::c_void) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDrawImagePointsRectI(graphics : *mut GpGraphics, image : *mut GpImage, points : *const Point, count : i32, srcx : i32, srcy : i32, srcwidth : i32, srcheight : i32, srcunit : Unit, imageattributes : *const GpImageAttributes, callback : isize, callbackdata : *mut core::ffi::c_void) -> Status);
    GdipDrawImagePointsRectI(graphics, image, points, count, srcx, srcy, srcwidth, srcheight, srcunit, imageattributes, callback, callbackdata)
}
#[inline]
pub unsafe fn GdipDrawImageRect(graphics: *mut GpGraphics, image: *mut GpImage, x: f32, y: f32, width: f32, height: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDrawImageRect(graphics : *mut GpGraphics, image : *mut GpImage, x : f32, y : f32, width : f32, height : f32) -> Status);
    GdipDrawImageRect(graphics, image, x, y, width, height)
}
#[inline]
pub unsafe fn GdipDrawImageRectI(graphics: *mut GpGraphics, image: *mut GpImage, x: i32, y: i32, width: i32, height: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDrawImageRectI(graphics : *mut GpGraphics, image : *mut GpImage, x : i32, y : i32, width : i32, height : i32) -> Status);
    GdipDrawImageRectI(graphics, image, x, y, width, height)
}
#[inline]
pub unsafe fn GdipDrawImageRectRect(graphics: *mut GpGraphics, image: *mut GpImage, dstx: f32, dsty: f32, dstwidth: f32, dstheight: f32, srcx: f32, srcy: f32, srcwidth: f32, srcheight: f32, srcunit: Unit, imageattributes: *const GpImageAttributes, callback: isize, callbackdata: *mut core::ffi::c_void) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDrawImageRectRect(graphics : *mut GpGraphics, image : *mut GpImage, dstx : f32, dsty : f32, dstwidth : f32, dstheight : f32, srcx : f32, srcy : f32, srcwidth : f32, srcheight : f32, srcunit : Unit, imageattributes : *const GpImageAttributes, callback : isize, callbackdata : *mut core::ffi::c_void) -> Status);
    GdipDrawImageRectRect(graphics, image, dstx, dsty, dstwidth, dstheight, srcx, srcy, srcwidth, srcheight, srcunit, imageattributes, callback, callbackdata)
}
#[inline]
pub unsafe fn GdipDrawImageRectRectI(graphics: *mut GpGraphics, image: *mut GpImage, dstx: i32, dsty: i32, dstwidth: i32, dstheight: i32, srcx: i32, srcy: i32, srcwidth: i32, srcheight: i32, srcunit: Unit, imageattributes: *const GpImageAttributes, callback: isize, callbackdata: *mut core::ffi::c_void) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDrawImageRectRectI(graphics : *mut GpGraphics, image : *mut GpImage, dstx : i32, dsty : i32, dstwidth : i32, dstheight : i32, srcx : i32, srcy : i32, srcwidth : i32, srcheight : i32, srcunit : Unit, imageattributes : *const GpImageAttributes, callback : isize, callbackdata : *mut core::ffi::c_void) -> Status);
    GdipDrawImageRectRectI(graphics, image, dstx, dsty, dstwidth, dstheight, srcx, srcy, srcwidth, srcheight, srcunit, imageattributes, callback, callbackdata)
}
#[inline]
pub unsafe fn GdipDrawLine(graphics: *mut GpGraphics, pen: *mut GpPen, x1: f32, y1: f32, x2: f32, y2: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDrawLine(graphics : *mut GpGraphics, pen : *mut GpPen, x1 : f32, y1 : f32, x2 : f32, y2 : f32) -> Status);
    GdipDrawLine(graphics, pen, x1, y1, x2, y2)
}
#[inline]
pub unsafe fn GdipDrawLineI(graphics: *mut GpGraphics, pen: *mut GpPen, x1: i32, y1: i32, x2: i32, y2: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDrawLineI(graphics : *mut GpGraphics, pen : *mut GpPen, x1 : i32, y1 : i32, x2 : i32, y2 : i32) -> Status);
    GdipDrawLineI(graphics, pen, x1, y1, x2, y2)
}
#[inline]
pub unsafe fn GdipDrawLines(graphics: *mut GpGraphics, pen: *mut GpPen, points: *const PointF, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDrawLines(graphics : *mut GpGraphics, pen : *mut GpPen, points : *const PointF, count : i32) -> Status);
    GdipDrawLines(graphics, pen, points, count)
}
#[inline]
pub unsafe fn GdipDrawLinesI(graphics: *mut GpGraphics, pen: *mut GpPen, points: *const Point, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDrawLinesI(graphics : *mut GpGraphics, pen : *mut GpPen, points : *const Point, count : i32) -> Status);
    GdipDrawLinesI(graphics, pen, points, count)
}
#[inline]
pub unsafe fn GdipDrawPath(graphics: *mut GpGraphics, pen: *mut GpPen, path: *mut GpPath) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDrawPath(graphics : *mut GpGraphics, pen : *mut GpPen, path : *mut GpPath) -> Status);
    GdipDrawPath(graphics, pen, path)
}
#[inline]
pub unsafe fn GdipDrawPie(graphics: *mut GpGraphics, pen: *mut GpPen, x: f32, y: f32, width: f32, height: f32, startangle: f32, sweepangle: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDrawPie(graphics : *mut GpGraphics, pen : *mut GpPen, x : f32, y : f32, width : f32, height : f32, startangle : f32, sweepangle : f32) -> Status);
    GdipDrawPie(graphics, pen, x, y, width, height, startangle, sweepangle)
}
#[inline]
pub unsafe fn GdipDrawPieI(graphics: *mut GpGraphics, pen: *mut GpPen, x: i32, y: i32, width: i32, height: i32, startangle: f32, sweepangle: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDrawPieI(graphics : *mut GpGraphics, pen : *mut GpPen, x : i32, y : i32, width : i32, height : i32, startangle : f32, sweepangle : f32) -> Status);
    GdipDrawPieI(graphics, pen, x, y, width, height, startangle, sweepangle)
}
#[inline]
pub unsafe fn GdipDrawPolygon(graphics: *mut GpGraphics, pen: *mut GpPen, points: *const PointF, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDrawPolygon(graphics : *mut GpGraphics, pen : *mut GpPen, points : *const PointF, count : i32) -> Status);
    GdipDrawPolygon(graphics, pen, points, count)
}
#[inline]
pub unsafe fn GdipDrawPolygonI(graphics: *mut GpGraphics, pen: *mut GpPen, points: *const Point, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDrawPolygonI(graphics : *mut GpGraphics, pen : *mut GpPen, points : *const Point, count : i32) -> Status);
    GdipDrawPolygonI(graphics, pen, points, count)
}
#[inline]
pub unsafe fn GdipDrawRectangle(graphics: *mut GpGraphics, pen: *mut GpPen, x: f32, y: f32, width: f32, height: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDrawRectangle(graphics : *mut GpGraphics, pen : *mut GpPen, x : f32, y : f32, width : f32, height : f32) -> Status);
    GdipDrawRectangle(graphics, pen, x, y, width, height)
}
#[inline]
pub unsafe fn GdipDrawRectangleI(graphics: *mut GpGraphics, pen: *mut GpPen, x: i32, y: i32, width: i32, height: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDrawRectangleI(graphics : *mut GpGraphics, pen : *mut GpPen, x : i32, y : i32, width : i32, height : i32) -> Status);
    GdipDrawRectangleI(graphics, pen, x, y, width, height)
}
#[inline]
pub unsafe fn GdipDrawRectangles(graphics: *mut GpGraphics, pen: *mut GpPen, rects: *const RectF, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDrawRectangles(graphics : *mut GpGraphics, pen : *mut GpPen, rects : *const RectF, count : i32) -> Status);
    GdipDrawRectangles(graphics, pen, rects, count)
}
#[inline]
pub unsafe fn GdipDrawRectanglesI(graphics: *mut GpGraphics, pen: *mut GpPen, rects: *const Rect, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipDrawRectanglesI(graphics : *mut GpGraphics, pen : *mut GpPen, rects : *const Rect, count : i32) -> Status);
    GdipDrawRectanglesI(graphics, pen, rects, count)
}
#[inline]
pub unsafe fn GdipDrawString<P0>(graphics: *mut GpGraphics, string: P0, length: i32, font: *const GpFont, layoutrect: *const RectF, stringformat: *const GpStringFormat, brush: *const GpBrush) -> Status
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipDrawString(graphics : *mut GpGraphics, string : windows_core::PCWSTR, length : i32, font : *const GpFont, layoutrect : *const RectF, stringformat : *const GpStringFormat, brush : *const GpBrush) -> Status);
    GdipDrawString(graphics, string.param().abi(), length, font, layoutrect, stringformat, brush)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipEmfToWmfBits<P0>(hemf: P0, pdata16: Option<&mut [u8]>, imapmode: i32, eflags: i32) -> u32
where
    P0: windows_core::Param<super::Gdi::HENHMETAFILE>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipEmfToWmfBits(hemf : super::Gdi:: HENHMETAFILE, cbdata16 : u32, pdata16 : *mut u8, imapmode : i32, eflags : i32) -> u32);
    GdipEmfToWmfBits(hemf.param().abi(), pdata16.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pdata16.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), imapmode, eflags)
}
#[inline]
pub unsafe fn GdipEndContainer(graphics: *mut GpGraphics, state: u32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipEndContainer(graphics : *mut GpGraphics, state : u32) -> Status);
    GdipEndContainer(graphics, state)
}
#[inline]
pub unsafe fn GdipEnumerateMetafileDestPoint(graphics: *mut GpGraphics, metafile: *const GpMetafile, destpoint: *const PointF, callback: isize, callbackdata: *mut core::ffi::c_void, imageattributes: *const GpImageAttributes) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipEnumerateMetafileDestPoint(graphics : *mut GpGraphics, metafile : *const GpMetafile, destpoint : *const PointF, callback : isize, callbackdata : *mut core::ffi::c_void, imageattributes : *const GpImageAttributes) -> Status);
    GdipEnumerateMetafileDestPoint(graphics, metafile, destpoint, callback, callbackdata, imageattributes)
}
#[inline]
pub unsafe fn GdipEnumerateMetafileDestPointI(graphics: *mut GpGraphics, metafile: *const GpMetafile, destpoint: *const Point, callback: isize, callbackdata: *mut core::ffi::c_void, imageattributes: *const GpImageAttributes) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipEnumerateMetafileDestPointI(graphics : *mut GpGraphics, metafile : *const GpMetafile, destpoint : *const Point, callback : isize, callbackdata : *mut core::ffi::c_void, imageattributes : *const GpImageAttributes) -> Status);
    GdipEnumerateMetafileDestPointI(graphics, metafile, destpoint, callback, callbackdata, imageattributes)
}
#[inline]
pub unsafe fn GdipEnumerateMetafileDestPoints(graphics: *mut GpGraphics, metafile: *const GpMetafile, destpoints: *const PointF, count: i32, callback: isize, callbackdata: *mut core::ffi::c_void, imageattributes: *const GpImageAttributes) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipEnumerateMetafileDestPoints(graphics : *mut GpGraphics, metafile : *const GpMetafile, destpoints : *const PointF, count : i32, callback : isize, callbackdata : *mut core::ffi::c_void, imageattributes : *const GpImageAttributes) -> Status);
    GdipEnumerateMetafileDestPoints(graphics, metafile, destpoints, count, callback, callbackdata, imageattributes)
}
#[inline]
pub unsafe fn GdipEnumerateMetafileDestPointsI(graphics: *mut GpGraphics, metafile: *const GpMetafile, destpoints: *const Point, count: i32, callback: isize, callbackdata: *mut core::ffi::c_void, imageattributes: *const GpImageAttributes) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipEnumerateMetafileDestPointsI(graphics : *mut GpGraphics, metafile : *const GpMetafile, destpoints : *const Point, count : i32, callback : isize, callbackdata : *mut core::ffi::c_void, imageattributes : *const GpImageAttributes) -> Status);
    GdipEnumerateMetafileDestPointsI(graphics, metafile, destpoints, count, callback, callbackdata, imageattributes)
}
#[inline]
pub unsafe fn GdipEnumerateMetafileDestRect(graphics: *mut GpGraphics, metafile: *const GpMetafile, destrect: *const RectF, callback: isize, callbackdata: *mut core::ffi::c_void, imageattributes: *const GpImageAttributes) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipEnumerateMetafileDestRect(graphics : *mut GpGraphics, metafile : *const GpMetafile, destrect : *const RectF, callback : isize, callbackdata : *mut core::ffi::c_void, imageattributes : *const GpImageAttributes) -> Status);
    GdipEnumerateMetafileDestRect(graphics, metafile, destrect, callback, callbackdata, imageattributes)
}
#[inline]
pub unsafe fn GdipEnumerateMetafileDestRectI(graphics: *mut GpGraphics, metafile: *const GpMetafile, destrect: *const Rect, callback: isize, callbackdata: *mut core::ffi::c_void, imageattributes: *const GpImageAttributes) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipEnumerateMetafileDestRectI(graphics : *mut GpGraphics, metafile : *const GpMetafile, destrect : *const Rect, callback : isize, callbackdata : *mut core::ffi::c_void, imageattributes : *const GpImageAttributes) -> Status);
    GdipEnumerateMetafileDestRectI(graphics, metafile, destrect, callback, callbackdata, imageattributes)
}
#[inline]
pub unsafe fn GdipEnumerateMetafileSrcRectDestPoint(graphics: *mut GpGraphics, metafile: *const GpMetafile, destpoint: *const PointF, srcrect: *const RectF, srcunit: Unit, callback: isize, callbackdata: *mut core::ffi::c_void, imageattributes: *const GpImageAttributes) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipEnumerateMetafileSrcRectDestPoint(graphics : *mut GpGraphics, metafile : *const GpMetafile, destpoint : *const PointF, srcrect : *const RectF, srcunit : Unit, callback : isize, callbackdata : *mut core::ffi::c_void, imageattributes : *const GpImageAttributes) -> Status);
    GdipEnumerateMetafileSrcRectDestPoint(graphics, metafile, destpoint, srcrect, srcunit, callback, callbackdata, imageattributes)
}
#[inline]
pub unsafe fn GdipEnumerateMetafileSrcRectDestPointI(graphics: *mut GpGraphics, metafile: *const GpMetafile, destpoint: *const Point, srcrect: *const Rect, srcunit: Unit, callback: isize, callbackdata: *mut core::ffi::c_void, imageattributes: *const GpImageAttributes) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipEnumerateMetafileSrcRectDestPointI(graphics : *mut GpGraphics, metafile : *const GpMetafile, destpoint : *const Point, srcrect : *const Rect, srcunit : Unit, callback : isize, callbackdata : *mut core::ffi::c_void, imageattributes : *const GpImageAttributes) -> Status);
    GdipEnumerateMetafileSrcRectDestPointI(graphics, metafile, destpoint, srcrect, srcunit, callback, callbackdata, imageattributes)
}
#[inline]
pub unsafe fn GdipEnumerateMetafileSrcRectDestPoints(graphics: *mut GpGraphics, metafile: *const GpMetafile, destpoints: *const PointF, count: i32, srcrect: *const RectF, srcunit: Unit, callback: isize, callbackdata: *mut core::ffi::c_void, imageattributes: *const GpImageAttributes) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipEnumerateMetafileSrcRectDestPoints(graphics : *mut GpGraphics, metafile : *const GpMetafile, destpoints : *const PointF, count : i32, srcrect : *const RectF, srcunit : Unit, callback : isize, callbackdata : *mut core::ffi::c_void, imageattributes : *const GpImageAttributes) -> Status);
    GdipEnumerateMetafileSrcRectDestPoints(graphics, metafile, destpoints, count, srcrect, srcunit, callback, callbackdata, imageattributes)
}
#[inline]
pub unsafe fn GdipEnumerateMetafileSrcRectDestPointsI(graphics: *mut GpGraphics, metafile: *const GpMetafile, destpoints: *const Point, count: i32, srcrect: *const Rect, srcunit: Unit, callback: isize, callbackdata: *mut core::ffi::c_void, imageattributes: *const GpImageAttributes) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipEnumerateMetafileSrcRectDestPointsI(graphics : *mut GpGraphics, metafile : *const GpMetafile, destpoints : *const Point, count : i32, srcrect : *const Rect, srcunit : Unit, callback : isize, callbackdata : *mut core::ffi::c_void, imageattributes : *const GpImageAttributes) -> Status);
    GdipEnumerateMetafileSrcRectDestPointsI(graphics, metafile, destpoints, count, srcrect, srcunit, callback, callbackdata, imageattributes)
}
#[inline]
pub unsafe fn GdipEnumerateMetafileSrcRectDestRect(graphics: *mut GpGraphics, metafile: *const GpMetafile, destrect: *const RectF, srcrect: *const RectF, srcunit: Unit, callback: isize, callbackdata: *mut core::ffi::c_void, imageattributes: *const GpImageAttributes) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipEnumerateMetafileSrcRectDestRect(graphics : *mut GpGraphics, metafile : *const GpMetafile, destrect : *const RectF, srcrect : *const RectF, srcunit : Unit, callback : isize, callbackdata : *mut core::ffi::c_void, imageattributes : *const GpImageAttributes) -> Status);
    GdipEnumerateMetafileSrcRectDestRect(graphics, metafile, destrect, srcrect, srcunit, callback, callbackdata, imageattributes)
}
#[inline]
pub unsafe fn GdipEnumerateMetafileSrcRectDestRectI(graphics: *mut GpGraphics, metafile: *const GpMetafile, destrect: *const Rect, srcrect: *const Rect, srcunit: Unit, callback: isize, callbackdata: *mut core::ffi::c_void, imageattributes: *const GpImageAttributes) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipEnumerateMetafileSrcRectDestRectI(graphics : *mut GpGraphics, metafile : *const GpMetafile, destrect : *const Rect, srcrect : *const Rect, srcunit : Unit, callback : isize, callbackdata : *mut core::ffi::c_void, imageattributes : *const GpImageAttributes) -> Status);
    GdipEnumerateMetafileSrcRectDestRectI(graphics, metafile, destrect, srcrect, srcunit, callback, callbackdata, imageattributes)
}
#[inline]
pub unsafe fn GdipFillClosedCurve(graphics: *mut GpGraphics, brush: *mut GpBrush, points: *const PointF, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipFillClosedCurve(graphics : *mut GpGraphics, brush : *mut GpBrush, points : *const PointF, count : i32) -> Status);
    GdipFillClosedCurve(graphics, brush, points, count)
}
#[inline]
pub unsafe fn GdipFillClosedCurve2(graphics: *mut GpGraphics, brush: *mut GpBrush, points: *const PointF, count: i32, tension: f32, fillmode: FillMode) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipFillClosedCurve2(graphics : *mut GpGraphics, brush : *mut GpBrush, points : *const PointF, count : i32, tension : f32, fillmode : FillMode) -> Status);
    GdipFillClosedCurve2(graphics, brush, points, count, tension, fillmode)
}
#[inline]
pub unsafe fn GdipFillClosedCurve2I(graphics: *mut GpGraphics, brush: *mut GpBrush, points: *const Point, count: i32, tension: f32, fillmode: FillMode) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipFillClosedCurve2I(graphics : *mut GpGraphics, brush : *mut GpBrush, points : *const Point, count : i32, tension : f32, fillmode : FillMode) -> Status);
    GdipFillClosedCurve2I(graphics, brush, points, count, tension, fillmode)
}
#[inline]
pub unsafe fn GdipFillClosedCurveI(graphics: *mut GpGraphics, brush: *mut GpBrush, points: *const Point, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipFillClosedCurveI(graphics : *mut GpGraphics, brush : *mut GpBrush, points : *const Point, count : i32) -> Status);
    GdipFillClosedCurveI(graphics, brush, points, count)
}
#[inline]
pub unsafe fn GdipFillEllipse(graphics: *mut GpGraphics, brush: *mut GpBrush, x: f32, y: f32, width: f32, height: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipFillEllipse(graphics : *mut GpGraphics, brush : *mut GpBrush, x : f32, y : f32, width : f32, height : f32) -> Status);
    GdipFillEllipse(graphics, brush, x, y, width, height)
}
#[inline]
pub unsafe fn GdipFillEllipseI(graphics: *mut GpGraphics, brush: *mut GpBrush, x: i32, y: i32, width: i32, height: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipFillEllipseI(graphics : *mut GpGraphics, brush : *mut GpBrush, x : i32, y : i32, width : i32, height : i32) -> Status);
    GdipFillEllipseI(graphics, brush, x, y, width, height)
}
#[inline]
pub unsafe fn GdipFillPath(graphics: *mut GpGraphics, brush: *mut GpBrush, path: *mut GpPath) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipFillPath(graphics : *mut GpGraphics, brush : *mut GpBrush, path : *mut GpPath) -> Status);
    GdipFillPath(graphics, brush, path)
}
#[inline]
pub unsafe fn GdipFillPie(graphics: *mut GpGraphics, brush: *mut GpBrush, x: f32, y: f32, width: f32, height: f32, startangle: f32, sweepangle: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipFillPie(graphics : *mut GpGraphics, brush : *mut GpBrush, x : f32, y : f32, width : f32, height : f32, startangle : f32, sweepangle : f32) -> Status);
    GdipFillPie(graphics, brush, x, y, width, height, startangle, sweepangle)
}
#[inline]
pub unsafe fn GdipFillPieI(graphics: *mut GpGraphics, brush: *mut GpBrush, x: i32, y: i32, width: i32, height: i32, startangle: f32, sweepangle: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipFillPieI(graphics : *mut GpGraphics, brush : *mut GpBrush, x : i32, y : i32, width : i32, height : i32, startangle : f32, sweepangle : f32) -> Status);
    GdipFillPieI(graphics, brush, x, y, width, height, startangle, sweepangle)
}
#[inline]
pub unsafe fn GdipFillPolygon(graphics: *mut GpGraphics, brush: *mut GpBrush, points: *const PointF, count: i32, fillmode: FillMode) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipFillPolygon(graphics : *mut GpGraphics, brush : *mut GpBrush, points : *const PointF, count : i32, fillmode : FillMode) -> Status);
    GdipFillPolygon(graphics, brush, points, count, fillmode)
}
#[inline]
pub unsafe fn GdipFillPolygon2(graphics: *mut GpGraphics, brush: *mut GpBrush, points: *const PointF, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipFillPolygon2(graphics : *mut GpGraphics, brush : *mut GpBrush, points : *const PointF, count : i32) -> Status);
    GdipFillPolygon2(graphics, brush, points, count)
}
#[inline]
pub unsafe fn GdipFillPolygon2I(graphics: *mut GpGraphics, brush: *mut GpBrush, points: *const Point, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipFillPolygon2I(graphics : *mut GpGraphics, brush : *mut GpBrush, points : *const Point, count : i32) -> Status);
    GdipFillPolygon2I(graphics, brush, points, count)
}
#[inline]
pub unsafe fn GdipFillPolygonI(graphics: *mut GpGraphics, brush: *mut GpBrush, points: *const Point, count: i32, fillmode: FillMode) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipFillPolygonI(graphics : *mut GpGraphics, brush : *mut GpBrush, points : *const Point, count : i32, fillmode : FillMode) -> Status);
    GdipFillPolygonI(graphics, brush, points, count, fillmode)
}
#[inline]
pub unsafe fn GdipFillRectangle(graphics: *mut GpGraphics, brush: *mut GpBrush, x: f32, y: f32, width: f32, height: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipFillRectangle(graphics : *mut GpGraphics, brush : *mut GpBrush, x : f32, y : f32, width : f32, height : f32) -> Status);
    GdipFillRectangle(graphics, brush, x, y, width, height)
}
#[inline]
pub unsafe fn GdipFillRectangleI(graphics: *mut GpGraphics, brush: *mut GpBrush, x: i32, y: i32, width: i32, height: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipFillRectangleI(graphics : *mut GpGraphics, brush : *mut GpBrush, x : i32, y : i32, width : i32, height : i32) -> Status);
    GdipFillRectangleI(graphics, brush, x, y, width, height)
}
#[inline]
pub unsafe fn GdipFillRectangles(graphics: *mut GpGraphics, brush: *mut GpBrush, rects: *const RectF, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipFillRectangles(graphics : *mut GpGraphics, brush : *mut GpBrush, rects : *const RectF, count : i32) -> Status);
    GdipFillRectangles(graphics, brush, rects, count)
}
#[inline]
pub unsafe fn GdipFillRectanglesI(graphics: *mut GpGraphics, brush: *mut GpBrush, rects: *const Rect, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipFillRectanglesI(graphics : *mut GpGraphics, brush : *mut GpBrush, rects : *const Rect, count : i32) -> Status);
    GdipFillRectanglesI(graphics, brush, rects, count)
}
#[inline]
pub unsafe fn GdipFillRegion(graphics: *mut GpGraphics, brush: *mut GpBrush, region: *mut GpRegion) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipFillRegion(graphics : *mut GpGraphics, brush : *mut GpBrush, region : *mut GpRegion) -> Status);
    GdipFillRegion(graphics, brush, region)
}
#[inline]
pub unsafe fn GdipFindFirstImageItem(image: *mut GpImage, item: *mut ImageItemData) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipFindFirstImageItem(image : *mut GpImage, item : *mut ImageItemData) -> Status);
    GdipFindFirstImageItem(image, item)
}
#[inline]
pub unsafe fn GdipFindNextImageItem(image: *mut GpImage, item: *mut ImageItemData) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipFindNextImageItem(image : *mut GpImage, item : *mut ImageItemData) -> Status);
    GdipFindNextImageItem(image, item)
}
#[inline]
pub unsafe fn GdipFlattenPath(path: *mut GpPath, matrix: *mut Matrix, flatness: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipFlattenPath(path : *mut GpPath, matrix : *mut Matrix, flatness : f32) -> Status);
    GdipFlattenPath(path, matrix, flatness)
}
#[inline]
pub unsafe fn GdipFlush(graphics: *mut GpGraphics, intention: FlushIntention) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipFlush(graphics : *mut GpGraphics, intention : FlushIntention) -> Status);
    GdipFlush(graphics, intention)
}
#[inline]
pub unsafe fn GdipFree(ptr: *mut core::ffi::c_void) {
    windows_targets::link!("gdiplus.dll" "system" fn GdipFree(ptr : *mut core::ffi::c_void));
    GdipFree(ptr)
}
#[inline]
pub unsafe fn GdipGetAdjustableArrowCapFillState(cap: *mut GpAdjustableArrowCap, fillstate: *mut super::super::Foundation::BOOL) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetAdjustableArrowCapFillState(cap : *mut GpAdjustableArrowCap, fillstate : *mut super::super::Foundation:: BOOL) -> Status);
    GdipGetAdjustableArrowCapFillState(cap, fillstate)
}
#[inline]
pub unsafe fn GdipGetAdjustableArrowCapHeight(cap: *mut GpAdjustableArrowCap, height: *mut f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetAdjustableArrowCapHeight(cap : *mut GpAdjustableArrowCap, height : *mut f32) -> Status);
    GdipGetAdjustableArrowCapHeight(cap, height)
}
#[inline]
pub unsafe fn GdipGetAdjustableArrowCapMiddleInset(cap: *mut GpAdjustableArrowCap, middleinset: *mut f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetAdjustableArrowCapMiddleInset(cap : *mut GpAdjustableArrowCap, middleinset : *mut f32) -> Status);
    GdipGetAdjustableArrowCapMiddleInset(cap, middleinset)
}
#[inline]
pub unsafe fn GdipGetAdjustableArrowCapWidth(cap: *mut GpAdjustableArrowCap, width: *mut f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetAdjustableArrowCapWidth(cap : *mut GpAdjustableArrowCap, width : *mut f32) -> Status);
    GdipGetAdjustableArrowCapWidth(cap, width)
}
#[inline]
pub unsafe fn GdipGetAllPropertyItems(image: *mut GpImage, totalbuffersize: u32, numproperties: u32, allitems: *mut PropertyItem) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetAllPropertyItems(image : *mut GpImage, totalbuffersize : u32, numproperties : u32, allitems : *mut PropertyItem) -> Status);
    GdipGetAllPropertyItems(image, totalbuffersize, numproperties, allitems)
}
#[inline]
pub unsafe fn GdipGetBrushType(brush: *mut GpBrush, r#type: *mut BrushType) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetBrushType(brush : *mut GpBrush, r#type : *mut BrushType) -> Status);
    GdipGetBrushType(brush, r#type)
}
#[inline]
pub unsafe fn GdipGetCellAscent(family: *const GpFontFamily, style: i32, cellascent: *mut u16) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetCellAscent(family : *const GpFontFamily, style : i32, cellascent : *mut u16) -> Status);
    GdipGetCellAscent(family, style, cellascent)
}
#[inline]
pub unsafe fn GdipGetCellDescent(family: *const GpFontFamily, style: i32, celldescent: *mut u16) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetCellDescent(family : *const GpFontFamily, style : i32, celldescent : *mut u16) -> Status);
    GdipGetCellDescent(family, style, celldescent)
}
#[inline]
pub unsafe fn GdipGetClip(graphics: *mut GpGraphics, region: *mut GpRegion) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetClip(graphics : *mut GpGraphics, region : *mut GpRegion) -> Status);
    GdipGetClip(graphics, region)
}
#[inline]
pub unsafe fn GdipGetClipBounds(graphics: *mut GpGraphics, rect: *mut RectF) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetClipBounds(graphics : *mut GpGraphics, rect : *mut RectF) -> Status);
    GdipGetClipBounds(graphics, rect)
}
#[inline]
pub unsafe fn GdipGetClipBoundsI(graphics: *mut GpGraphics, rect: *mut Rect) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetClipBoundsI(graphics : *mut GpGraphics, rect : *mut Rect) -> Status);
    GdipGetClipBoundsI(graphics, rect)
}
#[inline]
pub unsafe fn GdipGetCompositingMode(graphics: *mut GpGraphics, compositingmode: *mut CompositingMode) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetCompositingMode(graphics : *mut GpGraphics, compositingmode : *mut CompositingMode) -> Status);
    GdipGetCompositingMode(graphics, compositingmode)
}
#[inline]
pub unsafe fn GdipGetCompositingQuality(graphics: *mut GpGraphics, compositingquality: *mut CompositingQuality) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetCompositingQuality(graphics : *mut GpGraphics, compositingquality : *mut CompositingQuality) -> Status);
    GdipGetCompositingQuality(graphics, compositingquality)
}
#[inline]
pub unsafe fn GdipGetCustomLineCapBaseCap(customcap: *mut GpCustomLineCap, basecap: *mut LineCap) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetCustomLineCapBaseCap(customcap : *mut GpCustomLineCap, basecap : *mut LineCap) -> Status);
    GdipGetCustomLineCapBaseCap(customcap, basecap)
}
#[inline]
pub unsafe fn GdipGetCustomLineCapBaseInset(customcap: *mut GpCustomLineCap, inset: *mut f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetCustomLineCapBaseInset(customcap : *mut GpCustomLineCap, inset : *mut f32) -> Status);
    GdipGetCustomLineCapBaseInset(customcap, inset)
}
#[inline]
pub unsafe fn GdipGetCustomLineCapStrokeCaps(customcap: *mut GpCustomLineCap, startcap: *mut LineCap, endcap: *mut LineCap) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetCustomLineCapStrokeCaps(customcap : *mut GpCustomLineCap, startcap : *mut LineCap, endcap : *mut LineCap) -> Status);
    GdipGetCustomLineCapStrokeCaps(customcap, startcap, endcap)
}
#[inline]
pub unsafe fn GdipGetCustomLineCapStrokeJoin(customcap: *mut GpCustomLineCap, linejoin: *mut LineJoin) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetCustomLineCapStrokeJoin(customcap : *mut GpCustomLineCap, linejoin : *mut LineJoin) -> Status);
    GdipGetCustomLineCapStrokeJoin(customcap, linejoin)
}
#[inline]
pub unsafe fn GdipGetCustomLineCapType(customcap: *mut GpCustomLineCap, captype: *mut CustomLineCapType) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetCustomLineCapType(customcap : *mut GpCustomLineCap, captype : *mut CustomLineCapType) -> Status);
    GdipGetCustomLineCapType(customcap, captype)
}
#[inline]
pub unsafe fn GdipGetCustomLineCapWidthScale(customcap: *mut GpCustomLineCap, widthscale: *mut f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetCustomLineCapWidthScale(customcap : *mut GpCustomLineCap, widthscale : *mut f32) -> Status);
    GdipGetCustomLineCapWidthScale(customcap, widthscale)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipGetDC(graphics: *mut GpGraphics, hdc: *mut super::Gdi::HDC) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetDC(graphics : *mut GpGraphics, hdc : *mut super::Gdi:: HDC) -> Status);
    GdipGetDC(graphics, hdc)
}
#[inline]
pub unsafe fn GdipGetDpiX(graphics: *mut GpGraphics, dpi: *mut f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetDpiX(graphics : *mut GpGraphics, dpi : *mut f32) -> Status);
    GdipGetDpiX(graphics, dpi)
}
#[inline]
pub unsafe fn GdipGetDpiY(graphics: *mut GpGraphics, dpi: *mut f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetDpiY(graphics : *mut GpGraphics, dpi : *mut f32) -> Status);
    GdipGetDpiY(graphics, dpi)
}
#[inline]
pub unsafe fn GdipGetEffectParameterSize(effect: *mut CGpEffect, size: *mut u32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetEffectParameterSize(effect : *mut CGpEffect, size : *mut u32) -> Status);
    GdipGetEffectParameterSize(effect, size)
}
#[inline]
pub unsafe fn GdipGetEffectParameters(effect: *mut CGpEffect, size: *mut u32, params: *mut core::ffi::c_void) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetEffectParameters(effect : *mut CGpEffect, size : *mut u32, params : *mut core::ffi::c_void) -> Status);
    GdipGetEffectParameters(effect, size, params)
}
#[inline]
pub unsafe fn GdipGetEmHeight(family: *const GpFontFamily, style: i32, emheight: *mut u16) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetEmHeight(family : *const GpFontFamily, style : i32, emheight : *mut u16) -> Status);
    GdipGetEmHeight(family, style, emheight)
}
#[inline]
pub unsafe fn GdipGetEncoderParameterList(image: *mut GpImage, clsidencoder: *const windows_core::GUID, size: u32, buffer: *mut EncoderParameters) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetEncoderParameterList(image : *mut GpImage, clsidencoder : *const windows_core::GUID, size : u32, buffer : *mut EncoderParameters) -> Status);
    GdipGetEncoderParameterList(image, clsidencoder, size, buffer)
}
#[inline]
pub unsafe fn GdipGetEncoderParameterListSize(image: *mut GpImage, clsidencoder: *const windows_core::GUID, size: *mut u32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetEncoderParameterListSize(image : *mut GpImage, clsidencoder : *const windows_core::GUID, size : *mut u32) -> Status);
    GdipGetEncoderParameterListSize(image, clsidencoder, size)
}
#[inline]
pub unsafe fn GdipGetFamily(font: *mut GpFont, family: *mut *mut GpFontFamily) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetFamily(font : *mut GpFont, family : *mut *mut GpFontFamily) -> Status);
    GdipGetFamily(font, family)
}
#[inline]
pub unsafe fn GdipGetFamilyName(family: *const GpFontFamily, name: &mut [u16; 32], language: u16) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetFamilyName(family : *const GpFontFamily, name : windows_core::PWSTR, language : u16) -> Status);
    GdipGetFamilyName(family, core::mem::transmute(name.as_ptr()), language)
}
#[inline]
pub unsafe fn GdipGetFontCollectionFamilyCount(fontcollection: *mut GpFontCollection, numfound: *mut i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetFontCollectionFamilyCount(fontcollection : *mut GpFontCollection, numfound : *mut i32) -> Status);
    GdipGetFontCollectionFamilyCount(fontcollection, numfound)
}
#[inline]
pub unsafe fn GdipGetFontCollectionFamilyList(fontcollection: *const GpFontCollection, gpfamilies: &mut [*mut GpFontFamily], numfound: *mut i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetFontCollectionFamilyList(fontcollection : *const GpFontCollection, numsought : i32, gpfamilies : *mut *mut GpFontFamily, numfound : *mut i32) -> Status);
    GdipGetFontCollectionFamilyList(fontcollection, gpfamilies.len().try_into().unwrap(), core::mem::transmute(gpfamilies.as_ptr()), numfound)
}
#[inline]
pub unsafe fn GdipGetFontHeight(font: *const GpFont, graphics: *const GpGraphics, height: *mut f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetFontHeight(font : *const GpFont, graphics : *const GpGraphics, height : *mut f32) -> Status);
    GdipGetFontHeight(font, graphics, height)
}
#[inline]
pub unsafe fn GdipGetFontHeightGivenDPI(font: *const GpFont, dpi: f32, height: *mut f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetFontHeightGivenDPI(font : *const GpFont, dpi : f32, height : *mut f32) -> Status);
    GdipGetFontHeightGivenDPI(font, dpi, height)
}
#[inline]
pub unsafe fn GdipGetFontSize(font: *mut GpFont, size: *mut f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetFontSize(font : *mut GpFont, size : *mut f32) -> Status);
    GdipGetFontSize(font, size)
}
#[inline]
pub unsafe fn GdipGetFontStyle(font: *mut GpFont, style: *mut i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetFontStyle(font : *mut GpFont, style : *mut i32) -> Status);
    GdipGetFontStyle(font, style)
}
#[inline]
pub unsafe fn GdipGetFontUnit(font: *mut GpFont, unit: *mut Unit) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetFontUnit(font : *mut GpFont, unit : *mut Unit) -> Status);
    GdipGetFontUnit(font, unit)
}
#[inline]
pub unsafe fn GdipGetGenericFontFamilyMonospace(nativefamily: *mut *mut GpFontFamily) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetGenericFontFamilyMonospace(nativefamily : *mut *mut GpFontFamily) -> Status);
    GdipGetGenericFontFamilyMonospace(nativefamily)
}
#[inline]
pub unsafe fn GdipGetGenericFontFamilySansSerif(nativefamily: *mut *mut GpFontFamily) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetGenericFontFamilySansSerif(nativefamily : *mut *mut GpFontFamily) -> Status);
    GdipGetGenericFontFamilySansSerif(nativefamily)
}
#[inline]
pub unsafe fn GdipGetGenericFontFamilySerif(nativefamily: *mut *mut GpFontFamily) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetGenericFontFamilySerif(nativefamily : *mut *mut GpFontFamily) -> Status);
    GdipGetGenericFontFamilySerif(nativefamily)
}
#[inline]
pub unsafe fn GdipGetHatchBackgroundColor(brush: *mut GpHatch, backcol: *mut u32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetHatchBackgroundColor(brush : *mut GpHatch, backcol : *mut u32) -> Status);
    GdipGetHatchBackgroundColor(brush, backcol)
}
#[inline]
pub unsafe fn GdipGetHatchForegroundColor(brush: *mut GpHatch, forecol: *mut u32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetHatchForegroundColor(brush : *mut GpHatch, forecol : *mut u32) -> Status);
    GdipGetHatchForegroundColor(brush, forecol)
}
#[inline]
pub unsafe fn GdipGetHatchStyle(brush: *mut GpHatch, hatchstyle: *mut HatchStyle) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetHatchStyle(brush : *mut GpHatch, hatchstyle : *mut HatchStyle) -> Status);
    GdipGetHatchStyle(brush, hatchstyle)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipGetHemfFromMetafile(metafile: *mut GpMetafile, hemf: *mut super::Gdi::HENHMETAFILE) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetHemfFromMetafile(metafile : *mut GpMetafile, hemf : *mut super::Gdi:: HENHMETAFILE) -> Status);
    GdipGetHemfFromMetafile(metafile, hemf)
}
#[inline]
pub unsafe fn GdipGetImageAttributesAdjustedPalette(imageattr: *mut GpImageAttributes, colorpalette: *mut ColorPalette, coloradjusttype: ColorAdjustType) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetImageAttributesAdjustedPalette(imageattr : *mut GpImageAttributes, colorpalette : *mut ColorPalette, coloradjusttype : ColorAdjustType) -> Status);
    GdipGetImageAttributesAdjustedPalette(imageattr, colorpalette, coloradjusttype)
}
#[inline]
pub unsafe fn GdipGetImageBounds(image: *mut GpImage, srcrect: *mut RectF, srcunit: *mut Unit) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetImageBounds(image : *mut GpImage, srcrect : *mut RectF, srcunit : *mut Unit) -> Status);
    GdipGetImageBounds(image, srcrect, srcunit)
}
#[inline]
pub unsafe fn GdipGetImageDecoders(numdecoders: u32, size: u32, decoders: *mut ImageCodecInfo) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetImageDecoders(numdecoders : u32, size : u32, decoders : *mut ImageCodecInfo) -> Status);
    GdipGetImageDecoders(numdecoders, size, decoders)
}
#[inline]
pub unsafe fn GdipGetImageDecodersSize(numdecoders: *mut u32, size: *mut u32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetImageDecodersSize(numdecoders : *mut u32, size : *mut u32) -> Status);
    GdipGetImageDecodersSize(numdecoders, size)
}
#[inline]
pub unsafe fn GdipGetImageDimension(image: *mut GpImage, width: *mut f32, height: *mut f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetImageDimension(image : *mut GpImage, width : *mut f32, height : *mut f32) -> Status);
    GdipGetImageDimension(image, width, height)
}
#[inline]
pub unsafe fn GdipGetImageEncoders(numencoders: u32, size: u32, encoders: *mut ImageCodecInfo) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetImageEncoders(numencoders : u32, size : u32, encoders : *mut ImageCodecInfo) -> Status);
    GdipGetImageEncoders(numencoders, size, encoders)
}
#[inline]
pub unsafe fn GdipGetImageEncodersSize(numencoders: *mut u32, size: *mut u32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetImageEncodersSize(numencoders : *mut u32, size : *mut u32) -> Status);
    GdipGetImageEncodersSize(numencoders, size)
}
#[inline]
pub unsafe fn GdipGetImageFlags(image: *mut GpImage, flags: *mut u32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetImageFlags(image : *mut GpImage, flags : *mut u32) -> Status);
    GdipGetImageFlags(image, flags)
}
#[inline]
pub unsafe fn GdipGetImageGraphicsContext(image: *mut GpImage, graphics: *mut *mut GpGraphics) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetImageGraphicsContext(image : *mut GpImage, graphics : *mut *mut GpGraphics) -> Status);
    GdipGetImageGraphicsContext(image, graphics)
}
#[inline]
pub unsafe fn GdipGetImageHeight(image: *mut GpImage, height: *mut u32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetImageHeight(image : *mut GpImage, height : *mut u32) -> Status);
    GdipGetImageHeight(image, height)
}
#[inline]
pub unsafe fn GdipGetImageHorizontalResolution(image: *mut GpImage, resolution: *mut f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetImageHorizontalResolution(image : *mut GpImage, resolution : *mut f32) -> Status);
    GdipGetImageHorizontalResolution(image, resolution)
}
#[inline]
pub unsafe fn GdipGetImageItemData(image: *mut GpImage, item: *mut ImageItemData) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetImageItemData(image : *mut GpImage, item : *mut ImageItemData) -> Status);
    GdipGetImageItemData(image, item)
}
#[inline]
pub unsafe fn GdipGetImagePalette(image: *mut GpImage, palette: *mut ColorPalette, size: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetImagePalette(image : *mut GpImage, palette : *mut ColorPalette, size : i32) -> Status);
    GdipGetImagePalette(image, palette, size)
}
#[inline]
pub unsafe fn GdipGetImagePaletteSize(image: *mut GpImage, size: *mut i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetImagePaletteSize(image : *mut GpImage, size : *mut i32) -> Status);
    GdipGetImagePaletteSize(image, size)
}
#[inline]
pub unsafe fn GdipGetImagePixelFormat(image: *mut GpImage, format: *mut i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetImagePixelFormat(image : *mut GpImage, format : *mut i32) -> Status);
    GdipGetImagePixelFormat(image, format)
}
#[inline]
pub unsafe fn GdipGetImageRawFormat(image: *mut GpImage, format: *mut windows_core::GUID) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetImageRawFormat(image : *mut GpImage, format : *mut windows_core::GUID) -> Status);
    GdipGetImageRawFormat(image, format)
}
#[inline]
pub unsafe fn GdipGetImageThumbnail(image: *mut GpImage, thumbwidth: u32, thumbheight: u32, thumbimage: *mut *mut GpImage, callback: isize, callbackdata: *mut core::ffi::c_void) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetImageThumbnail(image : *mut GpImage, thumbwidth : u32, thumbheight : u32, thumbimage : *mut *mut GpImage, callback : isize, callbackdata : *mut core::ffi::c_void) -> Status);
    GdipGetImageThumbnail(image, thumbwidth, thumbheight, thumbimage, callback, callbackdata)
}
#[inline]
pub unsafe fn GdipGetImageType(image: *mut GpImage, r#type: *mut ImageType) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetImageType(image : *mut GpImage, r#type : *mut ImageType) -> Status);
    GdipGetImageType(image, r#type)
}
#[inline]
pub unsafe fn GdipGetImageVerticalResolution(image: *mut GpImage, resolution: *mut f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetImageVerticalResolution(image : *mut GpImage, resolution : *mut f32) -> Status);
    GdipGetImageVerticalResolution(image, resolution)
}
#[inline]
pub unsafe fn GdipGetImageWidth(image: *mut GpImage, width: *mut u32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetImageWidth(image : *mut GpImage, width : *mut u32) -> Status);
    GdipGetImageWidth(image, width)
}
#[inline]
pub unsafe fn GdipGetInterpolationMode(graphics: *mut GpGraphics, interpolationmode: *mut InterpolationMode) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetInterpolationMode(graphics : *mut GpGraphics, interpolationmode : *mut InterpolationMode) -> Status);
    GdipGetInterpolationMode(graphics, interpolationmode)
}
#[inline]
pub unsafe fn GdipGetLineBlend(brush: *mut GpLineGradient, blend: *mut f32, positions: *mut f32, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetLineBlend(brush : *mut GpLineGradient, blend : *mut f32, positions : *mut f32, count : i32) -> Status);
    GdipGetLineBlend(brush, blend, positions, count)
}
#[inline]
pub unsafe fn GdipGetLineBlendCount(brush: *mut GpLineGradient, count: *mut i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetLineBlendCount(brush : *mut GpLineGradient, count : *mut i32) -> Status);
    GdipGetLineBlendCount(brush, count)
}
#[inline]
pub unsafe fn GdipGetLineColors(brush: *mut GpLineGradient, colors: *mut u32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetLineColors(brush : *mut GpLineGradient, colors : *mut u32) -> Status);
    GdipGetLineColors(brush, colors)
}
#[inline]
pub unsafe fn GdipGetLineGammaCorrection(brush: *mut GpLineGradient, usegammacorrection: *mut super::super::Foundation::BOOL) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetLineGammaCorrection(brush : *mut GpLineGradient, usegammacorrection : *mut super::super::Foundation:: BOOL) -> Status);
    GdipGetLineGammaCorrection(brush, usegammacorrection)
}
#[inline]
pub unsafe fn GdipGetLinePresetBlend(brush: *mut GpLineGradient, blend: *mut u32, positions: *mut f32, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetLinePresetBlend(brush : *mut GpLineGradient, blend : *mut u32, positions : *mut f32, count : i32) -> Status);
    GdipGetLinePresetBlend(brush, blend, positions, count)
}
#[inline]
pub unsafe fn GdipGetLinePresetBlendCount(brush: *mut GpLineGradient, count: *mut i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetLinePresetBlendCount(brush : *mut GpLineGradient, count : *mut i32) -> Status);
    GdipGetLinePresetBlendCount(brush, count)
}
#[inline]
pub unsafe fn GdipGetLineRect(brush: *mut GpLineGradient, rect: *mut RectF) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetLineRect(brush : *mut GpLineGradient, rect : *mut RectF) -> Status);
    GdipGetLineRect(brush, rect)
}
#[inline]
pub unsafe fn GdipGetLineRectI(brush: *mut GpLineGradient, rect: *mut Rect) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetLineRectI(brush : *mut GpLineGradient, rect : *mut Rect) -> Status);
    GdipGetLineRectI(brush, rect)
}
#[inline]
pub unsafe fn GdipGetLineSpacing(family: *const GpFontFamily, style: i32, linespacing: *mut u16) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetLineSpacing(family : *const GpFontFamily, style : i32, linespacing : *mut u16) -> Status);
    GdipGetLineSpacing(family, style, linespacing)
}
#[inline]
pub unsafe fn GdipGetLineTransform(brush: *mut GpLineGradient, matrix: *mut Matrix) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetLineTransform(brush : *mut GpLineGradient, matrix : *mut Matrix) -> Status);
    GdipGetLineTransform(brush, matrix)
}
#[inline]
pub unsafe fn GdipGetLineWrapMode(brush: *mut GpLineGradient, wrapmode: *mut WrapMode) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetLineWrapMode(brush : *mut GpLineGradient, wrapmode : *mut WrapMode) -> Status);
    GdipGetLineWrapMode(brush, wrapmode)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipGetLogFontA(font: *mut GpFont, graphics: *mut GpGraphics, logfonta: *mut super::Gdi::LOGFONTA) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetLogFontA(font : *mut GpFont, graphics : *mut GpGraphics, logfonta : *mut super::Gdi:: LOGFONTA) -> Status);
    GdipGetLogFontA(font, graphics, logfonta)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipGetLogFontW(font: *mut GpFont, graphics: *mut GpGraphics, logfontw: *mut super::Gdi::LOGFONTW) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetLogFontW(font : *mut GpFont, graphics : *mut GpGraphics, logfontw : *mut super::Gdi:: LOGFONTW) -> Status);
    GdipGetLogFontW(font, graphics, logfontw)
}
#[inline]
pub unsafe fn GdipGetMatrixElements(matrix: *const Matrix, matrixout: *mut f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetMatrixElements(matrix : *const Matrix, matrixout : *mut f32) -> Status);
    GdipGetMatrixElements(matrix, matrixout)
}
#[inline]
pub unsafe fn GdipGetMetafileDownLevelRasterizationLimit(metafile: *const GpMetafile, metafilerasterizationlimitdpi: *mut u32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetMetafileDownLevelRasterizationLimit(metafile : *const GpMetafile, metafilerasterizationlimitdpi : *mut u32) -> Status);
    GdipGetMetafileDownLevelRasterizationLimit(metafile, metafilerasterizationlimitdpi)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipGetMetafileHeaderFromEmf<P0>(hemf: P0, header: *mut MetafileHeader) -> Status
where
    P0: windows_core::Param<super::Gdi::HENHMETAFILE>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetMetafileHeaderFromEmf(hemf : super::Gdi:: HENHMETAFILE, header : *mut MetafileHeader) -> Status);
    GdipGetMetafileHeaderFromEmf(hemf.param().abi(), header)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipGetMetafileHeaderFromFile<P0>(filename: P0, header: *mut MetafileHeader) -> Status
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetMetafileHeaderFromFile(filename : windows_core::PCWSTR, header : *mut MetafileHeader) -> Status);
    GdipGetMetafileHeaderFromFile(filename.param().abi(), header)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipGetMetafileHeaderFromMetafile(metafile: *mut GpMetafile, header: *mut MetafileHeader) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetMetafileHeaderFromMetafile(metafile : *mut GpMetafile, header : *mut MetafileHeader) -> Status);
    GdipGetMetafileHeaderFromMetafile(metafile, header)
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn GdipGetMetafileHeaderFromStream<P0>(stream: P0, header: *mut MetafileHeader) -> Status
where
    P0: windows_core::Param<super::super::System::Com::IStream>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetMetafileHeaderFromStream(stream : * mut core::ffi::c_void, header : *mut MetafileHeader) -> Status);
    GdipGetMetafileHeaderFromStream(stream.param().abi(), header)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipGetMetafileHeaderFromWmf<P0>(hwmf: P0, wmfplaceablefileheader: *const WmfPlaceableFileHeader, header: *mut MetafileHeader) -> Status
where
    P0: windows_core::Param<super::Gdi::HMETAFILE>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetMetafileHeaderFromWmf(hwmf : super::Gdi:: HMETAFILE, wmfplaceablefileheader : *const WmfPlaceableFileHeader, header : *mut MetafileHeader) -> Status);
    GdipGetMetafileHeaderFromWmf(hwmf.param().abi(), wmfplaceablefileheader, header)
}
#[inline]
pub unsafe fn GdipGetNearestColor(graphics: *mut GpGraphics, argb: *mut u32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetNearestColor(graphics : *mut GpGraphics, argb : *mut u32) -> Status);
    GdipGetNearestColor(graphics, argb)
}
#[inline]
pub unsafe fn GdipGetPageScale(graphics: *mut GpGraphics, scale: *mut f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPageScale(graphics : *mut GpGraphics, scale : *mut f32) -> Status);
    GdipGetPageScale(graphics, scale)
}
#[inline]
pub unsafe fn GdipGetPageUnit(graphics: *mut GpGraphics, unit: *mut Unit) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPageUnit(graphics : *mut GpGraphics, unit : *mut Unit) -> Status);
    GdipGetPageUnit(graphics, unit)
}
#[inline]
pub unsafe fn GdipGetPathData(path: *mut GpPath, pathdata: *mut PathData) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPathData(path : *mut GpPath, pathdata : *mut PathData) -> Status);
    GdipGetPathData(path, pathdata)
}
#[inline]
pub unsafe fn GdipGetPathFillMode(path: *mut GpPath, fillmode: *mut FillMode) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPathFillMode(path : *mut GpPath, fillmode : *mut FillMode) -> Status);
    GdipGetPathFillMode(path, fillmode)
}
#[inline]
pub unsafe fn GdipGetPathGradientBlend(brush: *mut GpPathGradient, blend: *mut f32, positions: *mut f32, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPathGradientBlend(brush : *mut GpPathGradient, blend : *mut f32, positions : *mut f32, count : i32) -> Status);
    GdipGetPathGradientBlend(brush, blend, positions, count)
}
#[inline]
pub unsafe fn GdipGetPathGradientBlendCount(brush: *mut GpPathGradient, count: *mut i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPathGradientBlendCount(brush : *mut GpPathGradient, count : *mut i32) -> Status);
    GdipGetPathGradientBlendCount(brush, count)
}
#[inline]
pub unsafe fn GdipGetPathGradientCenterColor(brush: *mut GpPathGradient, colors: *mut u32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPathGradientCenterColor(brush : *mut GpPathGradient, colors : *mut u32) -> Status);
    GdipGetPathGradientCenterColor(brush, colors)
}
#[inline]
pub unsafe fn GdipGetPathGradientCenterPoint(brush: *mut GpPathGradient, points: *mut PointF) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPathGradientCenterPoint(brush : *mut GpPathGradient, points : *mut PointF) -> Status);
    GdipGetPathGradientCenterPoint(brush, points)
}
#[inline]
pub unsafe fn GdipGetPathGradientCenterPointI(brush: *mut GpPathGradient, points: *mut Point) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPathGradientCenterPointI(brush : *mut GpPathGradient, points : *mut Point) -> Status);
    GdipGetPathGradientCenterPointI(brush, points)
}
#[inline]
pub unsafe fn GdipGetPathGradientFocusScales(brush: *mut GpPathGradient, xscale: *mut f32, yscale: *mut f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPathGradientFocusScales(brush : *mut GpPathGradient, xscale : *mut f32, yscale : *mut f32) -> Status);
    GdipGetPathGradientFocusScales(brush, xscale, yscale)
}
#[inline]
pub unsafe fn GdipGetPathGradientGammaCorrection(brush: *mut GpPathGradient, usegammacorrection: *mut super::super::Foundation::BOOL) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPathGradientGammaCorrection(brush : *mut GpPathGradient, usegammacorrection : *mut super::super::Foundation:: BOOL) -> Status);
    GdipGetPathGradientGammaCorrection(brush, usegammacorrection)
}
#[inline]
pub unsafe fn GdipGetPathGradientPath(brush: *mut GpPathGradient, path: *mut GpPath) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPathGradientPath(brush : *mut GpPathGradient, path : *mut GpPath) -> Status);
    GdipGetPathGradientPath(brush, path)
}
#[inline]
pub unsafe fn GdipGetPathGradientPointCount(brush: *mut GpPathGradient, count: *mut i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPathGradientPointCount(brush : *mut GpPathGradient, count : *mut i32) -> Status);
    GdipGetPathGradientPointCount(brush, count)
}
#[inline]
pub unsafe fn GdipGetPathGradientPresetBlend(brush: *mut GpPathGradient, blend: *mut u32, positions: *mut f32, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPathGradientPresetBlend(brush : *mut GpPathGradient, blend : *mut u32, positions : *mut f32, count : i32) -> Status);
    GdipGetPathGradientPresetBlend(brush, blend, positions, count)
}
#[inline]
pub unsafe fn GdipGetPathGradientPresetBlendCount(brush: *mut GpPathGradient, count: *mut i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPathGradientPresetBlendCount(brush : *mut GpPathGradient, count : *mut i32) -> Status);
    GdipGetPathGradientPresetBlendCount(brush, count)
}
#[inline]
pub unsafe fn GdipGetPathGradientRect(brush: *mut GpPathGradient, rect: *mut RectF) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPathGradientRect(brush : *mut GpPathGradient, rect : *mut RectF) -> Status);
    GdipGetPathGradientRect(brush, rect)
}
#[inline]
pub unsafe fn GdipGetPathGradientRectI(brush: *mut GpPathGradient, rect: *mut Rect) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPathGradientRectI(brush : *mut GpPathGradient, rect : *mut Rect) -> Status);
    GdipGetPathGradientRectI(brush, rect)
}
#[inline]
pub unsafe fn GdipGetPathGradientSurroundColorCount(brush: *mut GpPathGradient, count: *mut i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPathGradientSurroundColorCount(brush : *mut GpPathGradient, count : *mut i32) -> Status);
    GdipGetPathGradientSurroundColorCount(brush, count)
}
#[inline]
pub unsafe fn GdipGetPathGradientSurroundColorsWithCount(brush: *const GpPathGradient, color: *mut u32, count: *mut i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPathGradientSurroundColorsWithCount(brush : *const GpPathGradient, color : *mut u32, count : *mut i32) -> Status);
    GdipGetPathGradientSurroundColorsWithCount(brush, color, count)
}
#[inline]
pub unsafe fn GdipGetPathGradientTransform(brush: *mut GpPathGradient, matrix: *mut Matrix) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPathGradientTransform(brush : *mut GpPathGradient, matrix : *mut Matrix) -> Status);
    GdipGetPathGradientTransform(brush, matrix)
}
#[inline]
pub unsafe fn GdipGetPathGradientWrapMode(brush: *mut GpPathGradient, wrapmode: *mut WrapMode) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPathGradientWrapMode(brush : *mut GpPathGradient, wrapmode : *mut WrapMode) -> Status);
    GdipGetPathGradientWrapMode(brush, wrapmode)
}
#[inline]
pub unsafe fn GdipGetPathLastPoint(path: *mut GpPath, lastpoint: *mut PointF) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPathLastPoint(path : *mut GpPath, lastpoint : *mut PointF) -> Status);
    GdipGetPathLastPoint(path, lastpoint)
}
#[inline]
pub unsafe fn GdipGetPathPoints(param0: *mut GpPath, points: *mut PointF, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPathPoints(param0 : *mut GpPath, points : *mut PointF, count : i32) -> Status);
    GdipGetPathPoints(param0, points, count)
}
#[inline]
pub unsafe fn GdipGetPathPointsI(param0: *mut GpPath, points: *mut Point, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPathPointsI(param0 : *mut GpPath, points : *mut Point, count : i32) -> Status);
    GdipGetPathPointsI(param0, points, count)
}
#[inline]
pub unsafe fn GdipGetPathTypes(path: *const GpPath, types: &mut [u8]) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPathTypes(path : *const GpPath, types : *mut u8, count : i32) -> Status);
    GdipGetPathTypes(path, core::mem::transmute(types.as_ptr()), types.len().try_into().unwrap())
}
#[inline]
pub unsafe fn GdipGetPathWorldBounds(path: *mut GpPath, bounds: *mut RectF, matrix: *const Matrix, pen: *const GpPen) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPathWorldBounds(path : *mut GpPath, bounds : *mut RectF, matrix : *const Matrix, pen : *const GpPen) -> Status);
    GdipGetPathWorldBounds(path, bounds, matrix, pen)
}
#[inline]
pub unsafe fn GdipGetPathWorldBoundsI(path: *mut GpPath, bounds: *mut Rect, matrix: *const Matrix, pen: *const GpPen) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPathWorldBoundsI(path : *mut GpPath, bounds : *mut Rect, matrix : *const Matrix, pen : *const GpPen) -> Status);
    GdipGetPathWorldBoundsI(path, bounds, matrix, pen)
}
#[inline]
pub unsafe fn GdipGetPenBrushFill(pen: *mut GpPen, brush: *mut *mut GpBrush) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPenBrushFill(pen : *mut GpPen, brush : *mut *mut GpBrush) -> Status);
    GdipGetPenBrushFill(pen, brush)
}
#[inline]
pub unsafe fn GdipGetPenColor(pen: *mut GpPen, argb: *mut u32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPenColor(pen : *mut GpPen, argb : *mut u32) -> Status);
    GdipGetPenColor(pen, argb)
}
#[inline]
pub unsafe fn GdipGetPenCompoundArray(pen: *mut GpPen, dash: *mut f32, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPenCompoundArray(pen : *mut GpPen, dash : *mut f32, count : i32) -> Status);
    GdipGetPenCompoundArray(pen, dash, count)
}
#[inline]
pub unsafe fn GdipGetPenCompoundCount(pen: *mut GpPen, count: *mut i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPenCompoundCount(pen : *mut GpPen, count : *mut i32) -> Status);
    GdipGetPenCompoundCount(pen, count)
}
#[inline]
pub unsafe fn GdipGetPenCustomEndCap(pen: *mut GpPen, customcap: *mut *mut GpCustomLineCap) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPenCustomEndCap(pen : *mut GpPen, customcap : *mut *mut GpCustomLineCap) -> Status);
    GdipGetPenCustomEndCap(pen, customcap)
}
#[inline]
pub unsafe fn GdipGetPenCustomStartCap(pen: *mut GpPen, customcap: *mut *mut GpCustomLineCap) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPenCustomStartCap(pen : *mut GpPen, customcap : *mut *mut GpCustomLineCap) -> Status);
    GdipGetPenCustomStartCap(pen, customcap)
}
#[inline]
pub unsafe fn GdipGetPenDashArray(pen: *mut GpPen, dash: *mut f32, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPenDashArray(pen : *mut GpPen, dash : *mut f32, count : i32) -> Status);
    GdipGetPenDashArray(pen, dash, count)
}
#[inline]
pub unsafe fn GdipGetPenDashCap197819(pen: *mut GpPen, dashcap: *mut DashCap) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPenDashCap197819(pen : *mut GpPen, dashcap : *mut DashCap) -> Status);
    GdipGetPenDashCap197819(pen, dashcap)
}
#[inline]
pub unsafe fn GdipGetPenDashCount(pen: *mut GpPen, count: *mut i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPenDashCount(pen : *mut GpPen, count : *mut i32) -> Status);
    GdipGetPenDashCount(pen, count)
}
#[inline]
pub unsafe fn GdipGetPenDashOffset(pen: *mut GpPen, offset: *mut f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPenDashOffset(pen : *mut GpPen, offset : *mut f32) -> Status);
    GdipGetPenDashOffset(pen, offset)
}
#[inline]
pub unsafe fn GdipGetPenDashStyle(pen: *mut GpPen, dashstyle: *mut DashStyle) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPenDashStyle(pen : *mut GpPen, dashstyle : *mut DashStyle) -> Status);
    GdipGetPenDashStyle(pen, dashstyle)
}
#[inline]
pub unsafe fn GdipGetPenEndCap(pen: *mut GpPen, endcap: *mut LineCap) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPenEndCap(pen : *mut GpPen, endcap : *mut LineCap) -> Status);
    GdipGetPenEndCap(pen, endcap)
}
#[inline]
pub unsafe fn GdipGetPenFillType(pen: *mut GpPen, r#type: *mut PenType) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPenFillType(pen : *mut GpPen, r#type : *mut PenType) -> Status);
    GdipGetPenFillType(pen, r#type)
}
#[inline]
pub unsafe fn GdipGetPenLineJoin(pen: *mut GpPen, linejoin: *mut LineJoin) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPenLineJoin(pen : *mut GpPen, linejoin : *mut LineJoin) -> Status);
    GdipGetPenLineJoin(pen, linejoin)
}
#[inline]
pub unsafe fn GdipGetPenMiterLimit(pen: *mut GpPen, miterlimit: *mut f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPenMiterLimit(pen : *mut GpPen, miterlimit : *mut f32) -> Status);
    GdipGetPenMiterLimit(pen, miterlimit)
}
#[inline]
pub unsafe fn GdipGetPenMode(pen: *mut GpPen, penmode: *mut PenAlignment) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPenMode(pen : *mut GpPen, penmode : *mut PenAlignment) -> Status);
    GdipGetPenMode(pen, penmode)
}
#[inline]
pub unsafe fn GdipGetPenStartCap(pen: *mut GpPen, startcap: *mut LineCap) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPenStartCap(pen : *mut GpPen, startcap : *mut LineCap) -> Status);
    GdipGetPenStartCap(pen, startcap)
}
#[inline]
pub unsafe fn GdipGetPenTransform(pen: *mut GpPen, matrix: *mut Matrix) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPenTransform(pen : *mut GpPen, matrix : *mut Matrix) -> Status);
    GdipGetPenTransform(pen, matrix)
}
#[inline]
pub unsafe fn GdipGetPenUnit(pen: *mut GpPen, unit: *mut Unit) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPenUnit(pen : *mut GpPen, unit : *mut Unit) -> Status);
    GdipGetPenUnit(pen, unit)
}
#[inline]
pub unsafe fn GdipGetPenWidth(pen: *mut GpPen, width: *mut f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPenWidth(pen : *mut GpPen, width : *mut f32) -> Status);
    GdipGetPenWidth(pen, width)
}
#[inline]
pub unsafe fn GdipGetPixelOffsetMode(graphics: *mut GpGraphics, pixeloffsetmode: *mut PixelOffsetMode) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPixelOffsetMode(graphics : *mut GpGraphics, pixeloffsetmode : *mut PixelOffsetMode) -> Status);
    GdipGetPixelOffsetMode(graphics, pixeloffsetmode)
}
#[inline]
pub unsafe fn GdipGetPointCount(path: *mut GpPath, count: *mut i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPointCount(path : *mut GpPath, count : *mut i32) -> Status);
    GdipGetPointCount(path, count)
}
#[inline]
pub unsafe fn GdipGetPropertyCount(image: *mut GpImage, numofproperty: *mut u32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPropertyCount(image : *mut GpImage, numofproperty : *mut u32) -> Status);
    GdipGetPropertyCount(image, numofproperty)
}
#[inline]
pub unsafe fn GdipGetPropertyIdList(image: *mut GpImage, numofproperty: u32, list: *mut u32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPropertyIdList(image : *mut GpImage, numofproperty : u32, list : *mut u32) -> Status);
    GdipGetPropertyIdList(image, numofproperty, list)
}
#[inline]
pub unsafe fn GdipGetPropertyItem(image: *mut GpImage, propid: u32, propsize: u32, buffer: *mut PropertyItem) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPropertyItem(image : *mut GpImage, propid : u32, propsize : u32, buffer : *mut PropertyItem) -> Status);
    GdipGetPropertyItem(image, propid, propsize, buffer)
}
#[inline]
pub unsafe fn GdipGetPropertyItemSize(image: *mut GpImage, propid: u32, size: *mut u32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPropertyItemSize(image : *mut GpImage, propid : u32, size : *mut u32) -> Status);
    GdipGetPropertyItemSize(image, propid, size)
}
#[inline]
pub unsafe fn GdipGetPropertySize(image: *mut GpImage, totalbuffersize: *mut u32, numproperties: *mut u32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetPropertySize(image : *mut GpImage, totalbuffersize : *mut u32, numproperties : *mut u32) -> Status);
    GdipGetPropertySize(image, totalbuffersize, numproperties)
}
#[inline]
pub unsafe fn GdipGetRegionBounds(region: *mut GpRegion, graphics: *mut GpGraphics, rect: *mut RectF) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetRegionBounds(region : *mut GpRegion, graphics : *mut GpGraphics, rect : *mut RectF) -> Status);
    GdipGetRegionBounds(region, graphics, rect)
}
#[inline]
pub unsafe fn GdipGetRegionBoundsI(region: *mut GpRegion, graphics: *mut GpGraphics, rect: *mut Rect) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetRegionBoundsI(region : *mut GpRegion, graphics : *mut GpGraphics, rect : *mut Rect) -> Status);
    GdipGetRegionBoundsI(region, graphics, rect)
}
#[inline]
pub unsafe fn GdipGetRegionData(region: *mut GpRegion, buffer: &mut [u8], sizefilled: Option<*mut u32>) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetRegionData(region : *mut GpRegion, buffer : *mut u8, buffersize : u32, sizefilled : *mut u32) -> Status);
    GdipGetRegionData(region, core::mem::transmute(buffer.as_ptr()), buffer.len().try_into().unwrap(), core::mem::transmute(sizefilled.unwrap_or(std::ptr::null_mut())))
}
#[inline]
pub unsafe fn GdipGetRegionDataSize(region: *mut GpRegion, buffersize: *mut u32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetRegionDataSize(region : *mut GpRegion, buffersize : *mut u32) -> Status);
    GdipGetRegionDataSize(region, buffersize)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipGetRegionHRgn(region: *mut GpRegion, graphics: *mut GpGraphics, hrgn: *mut super::Gdi::HRGN) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetRegionHRgn(region : *mut GpRegion, graphics : *mut GpGraphics, hrgn : *mut super::Gdi:: HRGN) -> Status);
    GdipGetRegionHRgn(region, graphics, hrgn)
}
#[inline]
pub unsafe fn GdipGetRegionScans(region: *mut GpRegion, rects: *mut RectF, count: *mut i32, matrix: *mut Matrix) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetRegionScans(region : *mut GpRegion, rects : *mut RectF, count : *mut i32, matrix : *mut Matrix) -> Status);
    GdipGetRegionScans(region, rects, count, matrix)
}
#[inline]
pub unsafe fn GdipGetRegionScansCount(region: *mut GpRegion, count: *mut u32, matrix: *mut Matrix) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetRegionScansCount(region : *mut GpRegion, count : *mut u32, matrix : *mut Matrix) -> Status);
    GdipGetRegionScansCount(region, count, matrix)
}
#[inline]
pub unsafe fn GdipGetRegionScansI(region: *mut GpRegion, rects: *mut Rect, count: *mut i32, matrix: *mut Matrix) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetRegionScansI(region : *mut GpRegion, rects : *mut Rect, count : *mut i32, matrix : *mut Matrix) -> Status);
    GdipGetRegionScansI(region, rects, count, matrix)
}
#[inline]
pub unsafe fn GdipGetRenderingOrigin(graphics: *mut GpGraphics, x: *mut i32, y: *mut i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetRenderingOrigin(graphics : *mut GpGraphics, x : *mut i32, y : *mut i32) -> Status);
    GdipGetRenderingOrigin(graphics, x, y)
}
#[inline]
pub unsafe fn GdipGetSmoothingMode(graphics: *mut GpGraphics, smoothingmode: *mut SmoothingMode) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetSmoothingMode(graphics : *mut GpGraphics, smoothingmode : *mut SmoothingMode) -> Status);
    GdipGetSmoothingMode(graphics, smoothingmode)
}
#[inline]
pub unsafe fn GdipGetSolidFillColor(brush: *mut GpSolidFill, color: *mut u32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetSolidFillColor(brush : *mut GpSolidFill, color : *mut u32) -> Status);
    GdipGetSolidFillColor(brush, color)
}
#[inline]
pub unsafe fn GdipGetStringFormatAlign(format: *const GpStringFormat, align: *mut StringAlignment) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetStringFormatAlign(format : *const GpStringFormat, align : *mut StringAlignment) -> Status);
    GdipGetStringFormatAlign(format, align)
}
#[inline]
pub unsafe fn GdipGetStringFormatDigitSubstitution(format: *const GpStringFormat, language: *mut u16, substitute: *mut StringDigitSubstitute) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetStringFormatDigitSubstitution(format : *const GpStringFormat, language : *mut u16, substitute : *mut StringDigitSubstitute) -> Status);
    GdipGetStringFormatDigitSubstitution(format, language, substitute)
}
#[inline]
pub unsafe fn GdipGetStringFormatFlags(format: *const GpStringFormat, flags: *mut i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetStringFormatFlags(format : *const GpStringFormat, flags : *mut i32) -> Status);
    GdipGetStringFormatFlags(format, flags)
}
#[inline]
pub unsafe fn GdipGetStringFormatHotkeyPrefix(format: *const GpStringFormat, hotkeyprefix: *mut i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetStringFormatHotkeyPrefix(format : *const GpStringFormat, hotkeyprefix : *mut i32) -> Status);
    GdipGetStringFormatHotkeyPrefix(format, hotkeyprefix)
}
#[inline]
pub unsafe fn GdipGetStringFormatLineAlign(format: *const GpStringFormat, align: *mut StringAlignment) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetStringFormatLineAlign(format : *const GpStringFormat, align : *mut StringAlignment) -> Status);
    GdipGetStringFormatLineAlign(format, align)
}
#[inline]
pub unsafe fn GdipGetStringFormatMeasurableCharacterRangeCount(format: *const GpStringFormat, count: *mut i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetStringFormatMeasurableCharacterRangeCount(format : *const GpStringFormat, count : *mut i32) -> Status);
    GdipGetStringFormatMeasurableCharacterRangeCount(format, count)
}
#[inline]
pub unsafe fn GdipGetStringFormatTabStopCount(format: *const GpStringFormat, count: *mut i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetStringFormatTabStopCount(format : *const GpStringFormat, count : *mut i32) -> Status);
    GdipGetStringFormatTabStopCount(format, count)
}
#[inline]
pub unsafe fn GdipGetStringFormatTabStops(format: *const GpStringFormat, count: i32, firsttaboffset: *mut f32, tabstops: *mut f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetStringFormatTabStops(format : *const GpStringFormat, count : i32, firsttaboffset : *mut f32, tabstops : *mut f32) -> Status);
    GdipGetStringFormatTabStops(format, count, firsttaboffset, tabstops)
}
#[inline]
pub unsafe fn GdipGetStringFormatTrimming(format: *const GpStringFormat, trimming: *mut StringTrimming) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetStringFormatTrimming(format : *const GpStringFormat, trimming : *mut StringTrimming) -> Status);
    GdipGetStringFormatTrimming(format, trimming)
}
#[inline]
pub unsafe fn GdipGetTextContrast(graphics: *mut GpGraphics, contrast: *mut u32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetTextContrast(graphics : *mut GpGraphics, contrast : *mut u32) -> Status);
    GdipGetTextContrast(graphics, contrast)
}
#[inline]
pub unsafe fn GdipGetTextRenderingHint(graphics: *mut GpGraphics, mode: *mut TextRenderingHint) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetTextRenderingHint(graphics : *mut GpGraphics, mode : *mut TextRenderingHint) -> Status);
    GdipGetTextRenderingHint(graphics, mode)
}
#[inline]
pub unsafe fn GdipGetTextureImage(brush: *mut GpTexture, image: *mut *mut GpImage) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetTextureImage(brush : *mut GpTexture, image : *mut *mut GpImage) -> Status);
    GdipGetTextureImage(brush, image)
}
#[inline]
pub unsafe fn GdipGetTextureTransform(brush: *mut GpTexture, matrix: *mut Matrix) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetTextureTransform(brush : *mut GpTexture, matrix : *mut Matrix) -> Status);
    GdipGetTextureTransform(brush, matrix)
}
#[inline]
pub unsafe fn GdipGetTextureWrapMode(brush: *mut GpTexture, wrapmode: *mut WrapMode) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetTextureWrapMode(brush : *mut GpTexture, wrapmode : *mut WrapMode) -> Status);
    GdipGetTextureWrapMode(brush, wrapmode)
}
#[inline]
pub unsafe fn GdipGetVisibleClipBounds(graphics: *mut GpGraphics, rect: *mut RectF) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetVisibleClipBounds(graphics : *mut GpGraphics, rect : *mut RectF) -> Status);
    GdipGetVisibleClipBounds(graphics, rect)
}
#[inline]
pub unsafe fn GdipGetVisibleClipBoundsI(graphics: *mut GpGraphics, rect: *mut Rect) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetVisibleClipBoundsI(graphics : *mut GpGraphics, rect : *mut Rect) -> Status);
    GdipGetVisibleClipBoundsI(graphics, rect)
}
#[inline]
pub unsafe fn GdipGetWorldTransform(graphics: *mut GpGraphics, matrix: *mut Matrix) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGetWorldTransform(graphics : *mut GpGraphics, matrix : *mut Matrix) -> Status);
    GdipGetWorldTransform(graphics, matrix)
}
#[inline]
pub unsafe fn GdipGraphicsClear(graphics: *mut GpGraphics, color: u32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipGraphicsClear(graphics : *mut GpGraphics, color : u32) -> Status);
    GdipGraphicsClear(graphics, color)
}
#[inline]
pub unsafe fn GdipGraphicsSetAbort<P0>(pgraphics: *mut GpGraphics, piabort: P0) -> Status
where
    P0: windows_core::Param<GdiplusAbort>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipGraphicsSetAbort(pgraphics : *mut GpGraphics, piabort : * mut core::ffi::c_void) -> Status);
    GdipGraphicsSetAbort(pgraphics, piabort.param().abi())
}
#[inline]
pub unsafe fn GdipImageForceValidation(image: *mut GpImage) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipImageForceValidation(image : *mut GpImage) -> Status);
    GdipImageForceValidation(image)
}
#[inline]
pub unsafe fn GdipImageGetFrameCount(image: *mut GpImage, dimensionid: *const windows_core::GUID, count: *mut u32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipImageGetFrameCount(image : *mut GpImage, dimensionid : *const windows_core::GUID, count : *mut u32) -> Status);
    GdipImageGetFrameCount(image, dimensionid, count)
}
#[inline]
pub unsafe fn GdipImageGetFrameDimensionsCount(image: *mut GpImage, count: *mut u32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipImageGetFrameDimensionsCount(image : *mut GpImage, count : *mut u32) -> Status);
    GdipImageGetFrameDimensionsCount(image, count)
}
#[inline]
pub unsafe fn GdipImageGetFrameDimensionsList(image: *mut GpImage, dimensionids: *mut windows_core::GUID, count: u32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipImageGetFrameDimensionsList(image : *mut GpImage, dimensionids : *mut windows_core::GUID, count : u32) -> Status);
    GdipImageGetFrameDimensionsList(image, dimensionids, count)
}
#[inline]
pub unsafe fn GdipImageRotateFlip(image: *mut GpImage, rftype: RotateFlipType) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipImageRotateFlip(image : *mut GpImage, rftype : RotateFlipType) -> Status);
    GdipImageRotateFlip(image, rftype)
}
#[inline]
pub unsafe fn GdipImageSelectActiveFrame(image: *mut GpImage, dimensionid: *const windows_core::GUID, frameindex: u32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipImageSelectActiveFrame(image : *mut GpImage, dimensionid : *const windows_core::GUID, frameindex : u32) -> Status);
    GdipImageSelectActiveFrame(image, dimensionid, frameindex)
}
#[inline]
pub unsafe fn GdipImageSetAbort<P0>(pimage: *mut GpImage, piabort: P0) -> Status
where
    P0: windows_core::Param<GdiplusAbort>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipImageSetAbort(pimage : *mut GpImage, piabort : * mut core::ffi::c_void) -> Status);
    GdipImageSetAbort(pimage, piabort.param().abi())
}
#[inline]
pub unsafe fn GdipInitializePalette<P0>(palette: *mut ColorPalette, palettetype: PaletteType, optimalcolors: i32, usetransparentcolor: P0, bitmap: *mut GpBitmap) -> Status
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipInitializePalette(palette : *mut ColorPalette, palettetype : PaletteType, optimalcolors : i32, usetransparentcolor : super::super::Foundation:: BOOL, bitmap : *mut GpBitmap) -> Status);
    GdipInitializePalette(palette, palettetype, optimalcolors, usetransparentcolor.param().abi(), bitmap)
}
#[inline]
pub unsafe fn GdipInvertMatrix(matrix: *mut Matrix) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipInvertMatrix(matrix : *mut Matrix) -> Status);
    GdipInvertMatrix(matrix)
}
#[inline]
pub unsafe fn GdipIsClipEmpty(graphics: *mut GpGraphics, result: *mut super::super::Foundation::BOOL) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipIsClipEmpty(graphics : *mut GpGraphics, result : *mut super::super::Foundation:: BOOL) -> Status);
    GdipIsClipEmpty(graphics, result)
}
#[inline]
pub unsafe fn GdipIsEmptyRegion(region: *mut GpRegion, graphics: *mut GpGraphics, result: *mut super::super::Foundation::BOOL) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipIsEmptyRegion(region : *mut GpRegion, graphics : *mut GpGraphics, result : *mut super::super::Foundation:: BOOL) -> Status);
    GdipIsEmptyRegion(region, graphics, result)
}
#[inline]
pub unsafe fn GdipIsEqualRegion(region: *mut GpRegion, region2: *mut GpRegion, graphics: *mut GpGraphics, result: *mut super::super::Foundation::BOOL) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipIsEqualRegion(region : *mut GpRegion, region2 : *mut GpRegion, graphics : *mut GpGraphics, result : *mut super::super::Foundation:: BOOL) -> Status);
    GdipIsEqualRegion(region, region2, graphics, result)
}
#[inline]
pub unsafe fn GdipIsInfiniteRegion(region: *mut GpRegion, graphics: *mut GpGraphics, result: *mut super::super::Foundation::BOOL) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipIsInfiniteRegion(region : *mut GpRegion, graphics : *mut GpGraphics, result : *mut super::super::Foundation:: BOOL) -> Status);
    GdipIsInfiniteRegion(region, graphics, result)
}
#[inline]
pub unsafe fn GdipIsMatrixEqual(matrix: *const Matrix, matrix2: *const Matrix, result: *mut super::super::Foundation::BOOL) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipIsMatrixEqual(matrix : *const Matrix, matrix2 : *const Matrix, result : *mut super::super::Foundation:: BOOL) -> Status);
    GdipIsMatrixEqual(matrix, matrix2, result)
}
#[inline]
pub unsafe fn GdipIsMatrixIdentity(matrix: *const Matrix, result: *mut super::super::Foundation::BOOL) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipIsMatrixIdentity(matrix : *const Matrix, result : *mut super::super::Foundation:: BOOL) -> Status);
    GdipIsMatrixIdentity(matrix, result)
}
#[inline]
pub unsafe fn GdipIsMatrixInvertible(matrix: *const Matrix, result: *mut super::super::Foundation::BOOL) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipIsMatrixInvertible(matrix : *const Matrix, result : *mut super::super::Foundation:: BOOL) -> Status);
    GdipIsMatrixInvertible(matrix, result)
}
#[inline]
pub unsafe fn GdipIsOutlineVisiblePathPoint(path: *mut GpPath, x: f32, y: f32, pen: *mut GpPen, graphics: *mut GpGraphics, result: *mut super::super::Foundation::BOOL) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipIsOutlineVisiblePathPoint(path : *mut GpPath, x : f32, y : f32, pen : *mut GpPen, graphics : *mut GpGraphics, result : *mut super::super::Foundation:: BOOL) -> Status);
    GdipIsOutlineVisiblePathPoint(path, x, y, pen, graphics, result)
}
#[inline]
pub unsafe fn GdipIsOutlineVisiblePathPointI(path: *mut GpPath, x: i32, y: i32, pen: *mut GpPen, graphics: *mut GpGraphics, result: *mut super::super::Foundation::BOOL) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipIsOutlineVisiblePathPointI(path : *mut GpPath, x : i32, y : i32, pen : *mut GpPen, graphics : *mut GpGraphics, result : *mut super::super::Foundation:: BOOL) -> Status);
    GdipIsOutlineVisiblePathPointI(path, x, y, pen, graphics, result)
}
#[inline]
pub unsafe fn GdipIsStyleAvailable(family: *const GpFontFamily, style: i32, isstyleavailable: *mut super::super::Foundation::BOOL) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipIsStyleAvailable(family : *const GpFontFamily, style : i32, isstyleavailable : *mut super::super::Foundation:: BOOL) -> Status);
    GdipIsStyleAvailable(family, style, isstyleavailable)
}
#[inline]
pub unsafe fn GdipIsVisibleClipEmpty(graphics: *mut GpGraphics, result: *mut super::super::Foundation::BOOL) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipIsVisibleClipEmpty(graphics : *mut GpGraphics, result : *mut super::super::Foundation:: BOOL) -> Status);
    GdipIsVisibleClipEmpty(graphics, result)
}
#[inline]
pub unsafe fn GdipIsVisiblePathPoint(path: *mut GpPath, x: f32, y: f32, graphics: *mut GpGraphics, result: *mut super::super::Foundation::BOOL) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipIsVisiblePathPoint(path : *mut GpPath, x : f32, y : f32, graphics : *mut GpGraphics, result : *mut super::super::Foundation:: BOOL) -> Status);
    GdipIsVisiblePathPoint(path, x, y, graphics, result)
}
#[inline]
pub unsafe fn GdipIsVisiblePathPointI(path: *mut GpPath, x: i32, y: i32, graphics: *mut GpGraphics, result: *mut super::super::Foundation::BOOL) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipIsVisiblePathPointI(path : *mut GpPath, x : i32, y : i32, graphics : *mut GpGraphics, result : *mut super::super::Foundation:: BOOL) -> Status);
    GdipIsVisiblePathPointI(path, x, y, graphics, result)
}
#[inline]
pub unsafe fn GdipIsVisiblePoint(graphics: *mut GpGraphics, x: f32, y: f32, result: *mut super::super::Foundation::BOOL) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipIsVisiblePoint(graphics : *mut GpGraphics, x : f32, y : f32, result : *mut super::super::Foundation:: BOOL) -> Status);
    GdipIsVisiblePoint(graphics, x, y, result)
}
#[inline]
pub unsafe fn GdipIsVisiblePointI(graphics: *mut GpGraphics, x: i32, y: i32, result: *mut super::super::Foundation::BOOL) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipIsVisiblePointI(graphics : *mut GpGraphics, x : i32, y : i32, result : *mut super::super::Foundation:: BOOL) -> Status);
    GdipIsVisiblePointI(graphics, x, y, result)
}
#[inline]
pub unsafe fn GdipIsVisibleRect(graphics: *mut GpGraphics, x: f32, y: f32, width: f32, height: f32, result: *mut super::super::Foundation::BOOL) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipIsVisibleRect(graphics : *mut GpGraphics, x : f32, y : f32, width : f32, height : f32, result : *mut super::super::Foundation:: BOOL) -> Status);
    GdipIsVisibleRect(graphics, x, y, width, height, result)
}
#[inline]
pub unsafe fn GdipIsVisibleRectI(graphics: *mut GpGraphics, x: i32, y: i32, width: i32, height: i32, result: *mut super::super::Foundation::BOOL) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipIsVisibleRectI(graphics : *mut GpGraphics, x : i32, y : i32, width : i32, height : i32, result : *mut super::super::Foundation:: BOOL) -> Status);
    GdipIsVisibleRectI(graphics, x, y, width, height, result)
}
#[inline]
pub unsafe fn GdipIsVisibleRegionPoint(region: *mut GpRegion, x: f32, y: f32, graphics: *mut GpGraphics, result: *mut super::super::Foundation::BOOL) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipIsVisibleRegionPoint(region : *mut GpRegion, x : f32, y : f32, graphics : *mut GpGraphics, result : *mut super::super::Foundation:: BOOL) -> Status);
    GdipIsVisibleRegionPoint(region, x, y, graphics, result)
}
#[inline]
pub unsafe fn GdipIsVisibleRegionPointI(region: *mut GpRegion, x: i32, y: i32, graphics: *mut GpGraphics, result: *mut super::super::Foundation::BOOL) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipIsVisibleRegionPointI(region : *mut GpRegion, x : i32, y : i32, graphics : *mut GpGraphics, result : *mut super::super::Foundation:: BOOL) -> Status);
    GdipIsVisibleRegionPointI(region, x, y, graphics, result)
}
#[inline]
pub unsafe fn GdipIsVisibleRegionRect(region: *mut GpRegion, x: f32, y: f32, width: f32, height: f32, graphics: *mut GpGraphics, result: *mut super::super::Foundation::BOOL) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipIsVisibleRegionRect(region : *mut GpRegion, x : f32, y : f32, width : f32, height : f32, graphics : *mut GpGraphics, result : *mut super::super::Foundation:: BOOL) -> Status);
    GdipIsVisibleRegionRect(region, x, y, width, height, graphics, result)
}
#[inline]
pub unsafe fn GdipIsVisibleRegionRectI(region: *mut GpRegion, x: i32, y: i32, width: i32, height: i32, graphics: *mut GpGraphics, result: *mut super::super::Foundation::BOOL) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipIsVisibleRegionRectI(region : *mut GpRegion, x : i32, y : i32, width : i32, height : i32, graphics : *mut GpGraphics, result : *mut super::super::Foundation:: BOOL) -> Status);
    GdipIsVisibleRegionRectI(region, x, y, width, height, graphics, result)
}
#[inline]
pub unsafe fn GdipLoadImageFromFile<P0>(filename: P0, image: *mut *mut GpImage) -> Status
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipLoadImageFromFile(filename : windows_core::PCWSTR, image : *mut *mut GpImage) -> Status);
    GdipLoadImageFromFile(filename.param().abi(), image)
}
#[inline]
pub unsafe fn GdipLoadImageFromFileICM<P0>(filename: P0, image: *mut *mut GpImage) -> Status
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipLoadImageFromFileICM(filename : windows_core::PCWSTR, image : *mut *mut GpImage) -> Status);
    GdipLoadImageFromFileICM(filename.param().abi(), image)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn GdipLoadImageFromStream<P0>(stream: P0, image: *mut *mut GpImage) -> Status
where
    P0: windows_core::Param<super::super::System::Com::IStream>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipLoadImageFromStream(stream : * mut core::ffi::c_void, image : *mut *mut GpImage) -> Status);
    GdipLoadImageFromStream(stream.param().abi(), image)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn GdipLoadImageFromStreamICM<P0>(stream: P0, image: *mut *mut GpImage) -> Status
where
    P0: windows_core::Param<super::super::System::Com::IStream>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipLoadImageFromStreamICM(stream : * mut core::ffi::c_void, image : *mut *mut GpImage) -> Status);
    GdipLoadImageFromStreamICM(stream.param().abi(), image)
}
#[inline]
pub unsafe fn GdipMeasureCharacterRanges<P0>(graphics: *mut GpGraphics, string: P0, length: i32, font: *const GpFont, layoutrect: *const RectF, stringformat: *const GpStringFormat, regioncount: i32, regions: *mut *mut GpRegion) -> Status
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipMeasureCharacterRanges(graphics : *mut GpGraphics, string : windows_core::PCWSTR, length : i32, font : *const GpFont, layoutrect : *const RectF, stringformat : *const GpStringFormat, regioncount : i32, regions : *mut *mut GpRegion) -> Status);
    GdipMeasureCharacterRanges(graphics, string.param().abi(), length, font, layoutrect, stringformat, regioncount, regions)
}
#[inline]
pub unsafe fn GdipMeasureDriverString(graphics: *mut GpGraphics, text: *const u16, length: i32, font: *const GpFont, positions: *const PointF, flags: i32, matrix: *const Matrix, boundingbox: *mut RectF) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipMeasureDriverString(graphics : *mut GpGraphics, text : *const u16, length : i32, font : *const GpFont, positions : *const PointF, flags : i32, matrix : *const Matrix, boundingbox : *mut RectF) -> Status);
    GdipMeasureDriverString(graphics, text, length, font, positions, flags, matrix, boundingbox)
}
#[inline]
pub unsafe fn GdipMeasureString<P0>(graphics: *mut GpGraphics, string: P0, length: i32, font: *const GpFont, layoutrect: *const RectF, stringformat: *const GpStringFormat, boundingbox: *mut RectF, codepointsfitted: *mut i32, linesfilled: *mut i32) -> Status
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipMeasureString(graphics : *mut GpGraphics, string : windows_core::PCWSTR, length : i32, font : *const GpFont, layoutrect : *const RectF, stringformat : *const GpStringFormat, boundingbox : *mut RectF, codepointsfitted : *mut i32, linesfilled : *mut i32) -> Status);
    GdipMeasureString(graphics, string.param().abi(), length, font, layoutrect, stringformat, boundingbox, codepointsfitted, linesfilled)
}
#[inline]
pub unsafe fn GdipMultiplyLineTransform(brush: *mut GpLineGradient, matrix: *const Matrix, order: MatrixOrder) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipMultiplyLineTransform(brush : *mut GpLineGradient, matrix : *const Matrix, order : MatrixOrder) -> Status);
    GdipMultiplyLineTransform(brush, matrix, order)
}
#[inline]
pub unsafe fn GdipMultiplyMatrix(matrix: *mut Matrix, matrix2: *mut Matrix, order: MatrixOrder) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipMultiplyMatrix(matrix : *mut Matrix, matrix2 : *mut Matrix, order : MatrixOrder) -> Status);
    GdipMultiplyMatrix(matrix, matrix2, order)
}
#[inline]
pub unsafe fn GdipMultiplyPathGradientTransform(brush: *mut GpPathGradient, matrix: *const Matrix, order: MatrixOrder) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipMultiplyPathGradientTransform(brush : *mut GpPathGradient, matrix : *const Matrix, order : MatrixOrder) -> Status);
    GdipMultiplyPathGradientTransform(brush, matrix, order)
}
#[inline]
pub unsafe fn GdipMultiplyPenTransform(pen: *mut GpPen, matrix: *const Matrix, order: MatrixOrder) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipMultiplyPenTransform(pen : *mut GpPen, matrix : *const Matrix, order : MatrixOrder) -> Status);
    GdipMultiplyPenTransform(pen, matrix, order)
}
#[inline]
pub unsafe fn GdipMultiplyTextureTransform(brush: *mut GpTexture, matrix: *const Matrix, order: MatrixOrder) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipMultiplyTextureTransform(brush : *mut GpTexture, matrix : *const Matrix, order : MatrixOrder) -> Status);
    GdipMultiplyTextureTransform(brush, matrix, order)
}
#[inline]
pub unsafe fn GdipMultiplyWorldTransform(graphics: *mut GpGraphics, matrix: *const Matrix, order: MatrixOrder) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipMultiplyWorldTransform(graphics : *mut GpGraphics, matrix : *const Matrix, order : MatrixOrder) -> Status);
    GdipMultiplyWorldTransform(graphics, matrix, order)
}
#[inline]
pub unsafe fn GdipNewInstalledFontCollection(fontcollection: *mut *mut GpFontCollection) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipNewInstalledFontCollection(fontcollection : *mut *mut GpFontCollection) -> Status);
    GdipNewInstalledFontCollection(fontcollection)
}
#[inline]
pub unsafe fn GdipNewPrivateFontCollection(fontcollection: *mut *mut GpFontCollection) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipNewPrivateFontCollection(fontcollection : *mut *mut GpFontCollection) -> Status);
    GdipNewPrivateFontCollection(fontcollection)
}
#[inline]
pub unsafe fn GdipPathIterCopyData(iterator: *mut GpPathIterator, resultcount: *mut i32, points: *mut PointF, types: *mut u8, startindex: i32, endindex: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipPathIterCopyData(iterator : *mut GpPathIterator, resultcount : *mut i32, points : *mut PointF, types : *mut u8, startindex : i32, endindex : i32) -> Status);
    GdipPathIterCopyData(iterator, resultcount, points, types, startindex, endindex)
}
#[inline]
pub unsafe fn GdipPathIterEnumerate(iterator: *mut GpPathIterator, resultcount: *mut i32, points: *mut PointF, types: *mut u8, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipPathIterEnumerate(iterator : *mut GpPathIterator, resultcount : *mut i32, points : *mut PointF, types : *mut u8, count : i32) -> Status);
    GdipPathIterEnumerate(iterator, resultcount, points, types, count)
}
#[inline]
pub unsafe fn GdipPathIterGetCount(iterator: *mut GpPathIterator, count: *mut i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipPathIterGetCount(iterator : *mut GpPathIterator, count : *mut i32) -> Status);
    GdipPathIterGetCount(iterator, count)
}
#[inline]
pub unsafe fn GdipPathIterGetSubpathCount(iterator: *mut GpPathIterator, count: *mut i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipPathIterGetSubpathCount(iterator : *mut GpPathIterator, count : *mut i32) -> Status);
    GdipPathIterGetSubpathCount(iterator, count)
}
#[inline]
pub unsafe fn GdipPathIterHasCurve(iterator: *mut GpPathIterator, hascurve: *mut super::super::Foundation::BOOL) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipPathIterHasCurve(iterator : *mut GpPathIterator, hascurve : *mut super::super::Foundation:: BOOL) -> Status);
    GdipPathIterHasCurve(iterator, hascurve)
}
#[inline]
pub unsafe fn GdipPathIterIsValid(iterator: *mut GpPathIterator, valid: *mut super::super::Foundation::BOOL) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipPathIterIsValid(iterator : *mut GpPathIterator, valid : *mut super::super::Foundation:: BOOL) -> Status);
    GdipPathIterIsValid(iterator, valid)
}
#[inline]
pub unsafe fn GdipPathIterNextMarker(iterator: *mut GpPathIterator, resultcount: *mut i32, startindex: *mut i32, endindex: *mut i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipPathIterNextMarker(iterator : *mut GpPathIterator, resultcount : *mut i32, startindex : *mut i32, endindex : *mut i32) -> Status);
    GdipPathIterNextMarker(iterator, resultcount, startindex, endindex)
}
#[inline]
pub unsafe fn GdipPathIterNextMarkerPath(iterator: *mut GpPathIterator, resultcount: *mut i32, path: *mut GpPath) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipPathIterNextMarkerPath(iterator : *mut GpPathIterator, resultcount : *mut i32, path : *mut GpPath) -> Status);
    GdipPathIterNextMarkerPath(iterator, resultcount, path)
}
#[inline]
pub unsafe fn GdipPathIterNextPathType(iterator: *mut GpPathIterator, resultcount: *mut i32, pathtype: *mut u8, startindex: *mut i32, endindex: *mut i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipPathIterNextPathType(iterator : *mut GpPathIterator, resultcount : *mut i32, pathtype : *mut u8, startindex : *mut i32, endindex : *mut i32) -> Status);
    GdipPathIterNextPathType(iterator, resultcount, pathtype, startindex, endindex)
}
#[inline]
pub unsafe fn GdipPathIterNextSubpath(iterator: *mut GpPathIterator, resultcount: *mut i32, startindex: *mut i32, endindex: *mut i32, isclosed: *mut super::super::Foundation::BOOL) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipPathIterNextSubpath(iterator : *mut GpPathIterator, resultcount : *mut i32, startindex : *mut i32, endindex : *mut i32, isclosed : *mut super::super::Foundation:: BOOL) -> Status);
    GdipPathIterNextSubpath(iterator, resultcount, startindex, endindex, isclosed)
}
#[inline]
pub unsafe fn GdipPathIterNextSubpathPath(iterator: *mut GpPathIterator, resultcount: *mut i32, path: *mut GpPath, isclosed: *mut super::super::Foundation::BOOL) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipPathIterNextSubpathPath(iterator : *mut GpPathIterator, resultcount : *mut i32, path : *mut GpPath, isclosed : *mut super::super::Foundation:: BOOL) -> Status);
    GdipPathIterNextSubpathPath(iterator, resultcount, path, isclosed)
}
#[inline]
pub unsafe fn GdipPathIterRewind(iterator: *mut GpPathIterator) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipPathIterRewind(iterator : *mut GpPathIterator) -> Status);
    GdipPathIterRewind(iterator)
}
#[inline]
pub unsafe fn GdipPlayMetafileRecord(metafile: *const GpMetafile, recordtype: EmfPlusRecordType, flags: u32, datasize: u32, data: *const u8) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipPlayMetafileRecord(metafile : *const GpMetafile, recordtype : EmfPlusRecordType, flags : u32, datasize : u32, data : *const u8) -> Status);
    GdipPlayMetafileRecord(metafile, recordtype, flags, datasize, data)
}
#[inline]
pub unsafe fn GdipPrivateAddFontFile<P0>(fontcollection: *mut GpFontCollection, filename: P0) -> Status
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipPrivateAddFontFile(fontcollection : *mut GpFontCollection, filename : windows_core::PCWSTR) -> Status);
    GdipPrivateAddFontFile(fontcollection, filename.param().abi())
}
#[inline]
pub unsafe fn GdipPrivateAddMemoryFont(fontcollection: *mut GpFontCollection, memory: *const core::ffi::c_void, length: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipPrivateAddMemoryFont(fontcollection : *mut GpFontCollection, memory : *const core::ffi::c_void, length : i32) -> Status);
    GdipPrivateAddMemoryFont(fontcollection, memory, length)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipRecordMetafile<P0, P1>(referencehdc: P0, r#type: EmfType, framerect: *const RectF, frameunit: MetafileFrameUnit, description: P1, metafile: *mut *mut GpMetafile) -> Status
where
    P0: windows_core::Param<super::Gdi::HDC>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipRecordMetafile(referencehdc : super::Gdi:: HDC, r#type : EmfType, framerect : *const RectF, frameunit : MetafileFrameUnit, description : windows_core::PCWSTR, metafile : *mut *mut GpMetafile) -> Status);
    GdipRecordMetafile(referencehdc.param().abi(), r#type, framerect, frameunit, description.param().abi(), metafile)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipRecordMetafileFileName<P0, P1, P2>(filename: P0, referencehdc: P1, r#type: EmfType, framerect: *const RectF, frameunit: MetafileFrameUnit, description: P2, metafile: *mut *mut GpMetafile) -> Status
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::Gdi::HDC>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipRecordMetafileFileName(filename : windows_core::PCWSTR, referencehdc : super::Gdi:: HDC, r#type : EmfType, framerect : *const RectF, frameunit : MetafileFrameUnit, description : windows_core::PCWSTR, metafile : *mut *mut GpMetafile) -> Status);
    GdipRecordMetafileFileName(filename.param().abi(), referencehdc.param().abi(), r#type, framerect, frameunit, description.param().abi(), metafile)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipRecordMetafileFileNameI<P0, P1, P2>(filename: P0, referencehdc: P1, r#type: EmfType, framerect: *const Rect, frameunit: MetafileFrameUnit, description: P2, metafile: *mut *mut GpMetafile) -> Status
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::Gdi::HDC>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipRecordMetafileFileNameI(filename : windows_core::PCWSTR, referencehdc : super::Gdi:: HDC, r#type : EmfType, framerect : *const Rect, frameunit : MetafileFrameUnit, description : windows_core::PCWSTR, metafile : *mut *mut GpMetafile) -> Status);
    GdipRecordMetafileFileNameI(filename.param().abi(), referencehdc.param().abi(), r#type, framerect, frameunit, description.param().abi(), metafile)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipRecordMetafileI<P0, P1>(referencehdc: P0, r#type: EmfType, framerect: *const Rect, frameunit: MetafileFrameUnit, description: P1, metafile: *mut *mut GpMetafile) -> Status
where
    P0: windows_core::Param<super::Gdi::HDC>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipRecordMetafileI(referencehdc : super::Gdi:: HDC, r#type : EmfType, framerect : *const Rect, frameunit : MetafileFrameUnit, description : windows_core::PCWSTR, metafile : *mut *mut GpMetafile) -> Status);
    GdipRecordMetafileI(referencehdc.param().abi(), r#type, framerect, frameunit, description.param().abi(), metafile)
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn GdipRecordMetafileStream<P0, P1, P2>(stream: P0, referencehdc: P1, r#type: EmfType, framerect: *const RectF, frameunit: MetafileFrameUnit, description: P2, metafile: *mut *mut GpMetafile) -> Status
where
    P0: windows_core::Param<super::super::System::Com::IStream>,
    P1: windows_core::Param<super::Gdi::HDC>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipRecordMetafileStream(stream : * mut core::ffi::c_void, referencehdc : super::Gdi:: HDC, r#type : EmfType, framerect : *const RectF, frameunit : MetafileFrameUnit, description : windows_core::PCWSTR, metafile : *mut *mut GpMetafile) -> Status);
    GdipRecordMetafileStream(stream.param().abi(), referencehdc.param().abi(), r#type, framerect, frameunit, description.param().abi(), metafile)
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn GdipRecordMetafileStreamI<P0, P1, P2>(stream: P0, referencehdc: P1, r#type: EmfType, framerect: *const Rect, frameunit: MetafileFrameUnit, description: P2, metafile: *mut *mut GpMetafile) -> Status
where
    P0: windows_core::Param<super::super::System::Com::IStream>,
    P1: windows_core::Param<super::Gdi::HDC>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipRecordMetafileStreamI(stream : * mut core::ffi::c_void, referencehdc : super::Gdi:: HDC, r#type : EmfType, framerect : *const Rect, frameunit : MetafileFrameUnit, description : windows_core::PCWSTR, metafile : *mut *mut GpMetafile) -> Status);
    GdipRecordMetafileStreamI(stream.param().abi(), referencehdc.param().abi(), r#type, framerect, frameunit, description.param().abi(), metafile)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipReleaseDC<P0>(graphics: *mut GpGraphics, hdc: P0) -> Status
where
    P0: windows_core::Param<super::Gdi::HDC>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipReleaseDC(graphics : *mut GpGraphics, hdc : super::Gdi:: HDC) -> Status);
    GdipReleaseDC(graphics, hdc.param().abi())
}
#[inline]
pub unsafe fn GdipRemovePropertyItem(image: *mut GpImage, propid: u32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipRemovePropertyItem(image : *mut GpImage, propid : u32) -> Status);
    GdipRemovePropertyItem(image, propid)
}
#[inline]
pub unsafe fn GdipResetClip(graphics: *mut GpGraphics) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipResetClip(graphics : *mut GpGraphics) -> Status);
    GdipResetClip(graphics)
}
#[inline]
pub unsafe fn GdipResetImageAttributes(imageattr: *mut GpImageAttributes, r#type: ColorAdjustType) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipResetImageAttributes(imageattr : *mut GpImageAttributes, r#type : ColorAdjustType) -> Status);
    GdipResetImageAttributes(imageattr, r#type)
}
#[inline]
pub unsafe fn GdipResetLineTransform(brush: *mut GpLineGradient) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipResetLineTransform(brush : *mut GpLineGradient) -> Status);
    GdipResetLineTransform(brush)
}
#[inline]
pub unsafe fn GdipResetPageTransform(graphics: *mut GpGraphics) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipResetPageTransform(graphics : *mut GpGraphics) -> Status);
    GdipResetPageTransform(graphics)
}
#[inline]
pub unsafe fn GdipResetPath(path: *mut GpPath) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipResetPath(path : *mut GpPath) -> Status);
    GdipResetPath(path)
}
#[inline]
pub unsafe fn GdipResetPathGradientTransform(brush: *mut GpPathGradient) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipResetPathGradientTransform(brush : *mut GpPathGradient) -> Status);
    GdipResetPathGradientTransform(brush)
}
#[inline]
pub unsafe fn GdipResetPenTransform(pen: *mut GpPen) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipResetPenTransform(pen : *mut GpPen) -> Status);
    GdipResetPenTransform(pen)
}
#[inline]
pub unsafe fn GdipResetTextureTransform(brush: *mut GpTexture) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipResetTextureTransform(brush : *mut GpTexture) -> Status);
    GdipResetTextureTransform(brush)
}
#[inline]
pub unsafe fn GdipResetWorldTransform(graphics: *mut GpGraphics) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipResetWorldTransform(graphics : *mut GpGraphics) -> Status);
    GdipResetWorldTransform(graphics)
}
#[inline]
pub unsafe fn GdipRestoreGraphics(graphics: *mut GpGraphics, state: u32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipRestoreGraphics(graphics : *mut GpGraphics, state : u32) -> Status);
    GdipRestoreGraphics(graphics, state)
}
#[inline]
pub unsafe fn GdipReversePath(path: *mut GpPath) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipReversePath(path : *mut GpPath) -> Status);
    GdipReversePath(path)
}
#[inline]
pub unsafe fn GdipRotateLineTransform(brush: *mut GpLineGradient, angle: f32, order: MatrixOrder) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipRotateLineTransform(brush : *mut GpLineGradient, angle : f32, order : MatrixOrder) -> Status);
    GdipRotateLineTransform(brush, angle, order)
}
#[inline]
pub unsafe fn GdipRotateMatrix(matrix: *mut Matrix, angle: f32, order: MatrixOrder) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipRotateMatrix(matrix : *mut Matrix, angle : f32, order : MatrixOrder) -> Status);
    GdipRotateMatrix(matrix, angle, order)
}
#[inline]
pub unsafe fn GdipRotatePathGradientTransform(brush: *mut GpPathGradient, angle: f32, order: MatrixOrder) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipRotatePathGradientTransform(brush : *mut GpPathGradient, angle : f32, order : MatrixOrder) -> Status);
    GdipRotatePathGradientTransform(brush, angle, order)
}
#[inline]
pub unsafe fn GdipRotatePenTransform(pen: *mut GpPen, angle: f32, order: MatrixOrder) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipRotatePenTransform(pen : *mut GpPen, angle : f32, order : MatrixOrder) -> Status);
    GdipRotatePenTransform(pen, angle, order)
}
#[inline]
pub unsafe fn GdipRotateTextureTransform(brush: *mut GpTexture, angle: f32, order: MatrixOrder) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipRotateTextureTransform(brush : *mut GpTexture, angle : f32, order : MatrixOrder) -> Status);
    GdipRotateTextureTransform(brush, angle, order)
}
#[inline]
pub unsafe fn GdipRotateWorldTransform(graphics: *mut GpGraphics, angle: f32, order: MatrixOrder) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipRotateWorldTransform(graphics : *mut GpGraphics, angle : f32, order : MatrixOrder) -> Status);
    GdipRotateWorldTransform(graphics, angle, order)
}
#[inline]
pub unsafe fn GdipSaveAdd(image: *mut GpImage, encoderparams: *const EncoderParameters) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSaveAdd(image : *mut GpImage, encoderparams : *const EncoderParameters) -> Status);
    GdipSaveAdd(image, encoderparams)
}
#[inline]
pub unsafe fn GdipSaveAddImage(image: *mut GpImage, newimage: *mut GpImage, encoderparams: *const EncoderParameters) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSaveAddImage(image : *mut GpImage, newimage : *mut GpImage, encoderparams : *const EncoderParameters) -> Status);
    GdipSaveAddImage(image, newimage, encoderparams)
}
#[inline]
pub unsafe fn GdipSaveGraphics(graphics: *mut GpGraphics, state: *mut u32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSaveGraphics(graphics : *mut GpGraphics, state : *mut u32) -> Status);
    GdipSaveGraphics(graphics, state)
}
#[inline]
pub unsafe fn GdipSaveImageToFile<P0>(image: *mut GpImage, filename: P0, clsidencoder: *const windows_core::GUID, encoderparams: *const EncoderParameters) -> Status
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipSaveImageToFile(image : *mut GpImage, filename : windows_core::PCWSTR, clsidencoder : *const windows_core::GUID, encoderparams : *const EncoderParameters) -> Status);
    GdipSaveImageToFile(image, filename.param().abi(), clsidencoder, encoderparams)
}
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn GdipSaveImageToStream<P0>(image: *mut GpImage, stream: P0, clsidencoder: *const windows_core::GUID, encoderparams: *const EncoderParameters) -> Status
where
    P0: windows_core::Param<super::super::System::Com::IStream>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipSaveImageToStream(image : *mut GpImage, stream : * mut core::ffi::c_void, clsidencoder : *const windows_core::GUID, encoderparams : *const EncoderParameters) -> Status);
    GdipSaveImageToStream(image, stream.param().abi(), clsidencoder, encoderparams)
}
#[inline]
pub unsafe fn GdipScaleLineTransform(brush: *mut GpLineGradient, sx: f32, sy: f32, order: MatrixOrder) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipScaleLineTransform(brush : *mut GpLineGradient, sx : f32, sy : f32, order : MatrixOrder) -> Status);
    GdipScaleLineTransform(brush, sx, sy, order)
}
#[inline]
pub unsafe fn GdipScaleMatrix(matrix: *mut Matrix, scalex: f32, scaley: f32, order: MatrixOrder) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipScaleMatrix(matrix : *mut Matrix, scalex : f32, scaley : f32, order : MatrixOrder) -> Status);
    GdipScaleMatrix(matrix, scalex, scaley, order)
}
#[inline]
pub unsafe fn GdipScalePathGradientTransform(brush: *mut GpPathGradient, sx: f32, sy: f32, order: MatrixOrder) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipScalePathGradientTransform(brush : *mut GpPathGradient, sx : f32, sy : f32, order : MatrixOrder) -> Status);
    GdipScalePathGradientTransform(brush, sx, sy, order)
}
#[inline]
pub unsafe fn GdipScalePenTransform(pen: *mut GpPen, sx: f32, sy: f32, order: MatrixOrder) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipScalePenTransform(pen : *mut GpPen, sx : f32, sy : f32, order : MatrixOrder) -> Status);
    GdipScalePenTransform(pen, sx, sy, order)
}
#[inline]
pub unsafe fn GdipScaleTextureTransform(brush: *mut GpTexture, sx: f32, sy: f32, order: MatrixOrder) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipScaleTextureTransform(brush : *mut GpTexture, sx : f32, sy : f32, order : MatrixOrder) -> Status);
    GdipScaleTextureTransform(brush, sx, sy, order)
}
#[inline]
pub unsafe fn GdipScaleWorldTransform(graphics: *mut GpGraphics, sx: f32, sy: f32, order: MatrixOrder) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipScaleWorldTransform(graphics : *mut GpGraphics, sx : f32, sy : f32, order : MatrixOrder) -> Status);
    GdipScaleWorldTransform(graphics, sx, sy, order)
}
#[inline]
pub unsafe fn GdipSetAdjustableArrowCapFillState<P0>(cap: *mut GpAdjustableArrowCap, fillstate: P0) -> Status
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetAdjustableArrowCapFillState(cap : *mut GpAdjustableArrowCap, fillstate : super::super::Foundation:: BOOL) -> Status);
    GdipSetAdjustableArrowCapFillState(cap, fillstate.param().abi())
}
#[inline]
pub unsafe fn GdipSetAdjustableArrowCapHeight(cap: *mut GpAdjustableArrowCap, height: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetAdjustableArrowCapHeight(cap : *mut GpAdjustableArrowCap, height : f32) -> Status);
    GdipSetAdjustableArrowCapHeight(cap, height)
}
#[inline]
pub unsafe fn GdipSetAdjustableArrowCapMiddleInset(cap: *mut GpAdjustableArrowCap, middleinset: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetAdjustableArrowCapMiddleInset(cap : *mut GpAdjustableArrowCap, middleinset : f32) -> Status);
    GdipSetAdjustableArrowCapMiddleInset(cap, middleinset)
}
#[inline]
pub unsafe fn GdipSetAdjustableArrowCapWidth(cap: *mut GpAdjustableArrowCap, width: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetAdjustableArrowCapWidth(cap : *mut GpAdjustableArrowCap, width : f32) -> Status);
    GdipSetAdjustableArrowCapWidth(cap, width)
}
#[inline]
pub unsafe fn GdipSetClipGraphics(graphics: *mut GpGraphics, srcgraphics: *mut GpGraphics, combinemode: CombineMode) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetClipGraphics(graphics : *mut GpGraphics, srcgraphics : *mut GpGraphics, combinemode : CombineMode) -> Status);
    GdipSetClipGraphics(graphics, srcgraphics, combinemode)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GdipSetClipHrgn<P0>(graphics: *mut GpGraphics, hrgn: P0, combinemode: CombineMode) -> Status
where
    P0: windows_core::Param<super::Gdi::HRGN>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetClipHrgn(graphics : *mut GpGraphics, hrgn : super::Gdi:: HRGN, combinemode : CombineMode) -> Status);
    GdipSetClipHrgn(graphics, hrgn.param().abi(), combinemode)
}
#[inline]
pub unsafe fn GdipSetClipPath(graphics: *mut GpGraphics, path: *mut GpPath, combinemode: CombineMode) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetClipPath(graphics : *mut GpGraphics, path : *mut GpPath, combinemode : CombineMode) -> Status);
    GdipSetClipPath(graphics, path, combinemode)
}
#[inline]
pub unsafe fn GdipSetClipRect(graphics: *mut GpGraphics, x: f32, y: f32, width: f32, height: f32, combinemode: CombineMode) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetClipRect(graphics : *mut GpGraphics, x : f32, y : f32, width : f32, height : f32, combinemode : CombineMode) -> Status);
    GdipSetClipRect(graphics, x, y, width, height, combinemode)
}
#[inline]
pub unsafe fn GdipSetClipRectI(graphics: *mut GpGraphics, x: i32, y: i32, width: i32, height: i32, combinemode: CombineMode) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetClipRectI(graphics : *mut GpGraphics, x : i32, y : i32, width : i32, height : i32, combinemode : CombineMode) -> Status);
    GdipSetClipRectI(graphics, x, y, width, height, combinemode)
}
#[inline]
pub unsafe fn GdipSetClipRegion(graphics: *mut GpGraphics, region: *mut GpRegion, combinemode: CombineMode) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetClipRegion(graphics : *mut GpGraphics, region : *mut GpRegion, combinemode : CombineMode) -> Status);
    GdipSetClipRegion(graphics, region, combinemode)
}
#[inline]
pub unsafe fn GdipSetCompositingMode(graphics: *mut GpGraphics, compositingmode: CompositingMode) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetCompositingMode(graphics : *mut GpGraphics, compositingmode : CompositingMode) -> Status);
    GdipSetCompositingMode(graphics, compositingmode)
}
#[inline]
pub unsafe fn GdipSetCompositingQuality(graphics: *mut GpGraphics, compositingquality: CompositingQuality) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetCompositingQuality(graphics : *mut GpGraphics, compositingquality : CompositingQuality) -> Status);
    GdipSetCompositingQuality(graphics, compositingquality)
}
#[inline]
pub unsafe fn GdipSetCustomLineCapBaseCap(customcap: *mut GpCustomLineCap, basecap: LineCap) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetCustomLineCapBaseCap(customcap : *mut GpCustomLineCap, basecap : LineCap) -> Status);
    GdipSetCustomLineCapBaseCap(customcap, basecap)
}
#[inline]
pub unsafe fn GdipSetCustomLineCapBaseInset(customcap: *mut GpCustomLineCap, inset: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetCustomLineCapBaseInset(customcap : *mut GpCustomLineCap, inset : f32) -> Status);
    GdipSetCustomLineCapBaseInset(customcap, inset)
}
#[inline]
pub unsafe fn GdipSetCustomLineCapStrokeCaps(customcap: *mut GpCustomLineCap, startcap: LineCap, endcap: LineCap) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetCustomLineCapStrokeCaps(customcap : *mut GpCustomLineCap, startcap : LineCap, endcap : LineCap) -> Status);
    GdipSetCustomLineCapStrokeCaps(customcap, startcap, endcap)
}
#[inline]
pub unsafe fn GdipSetCustomLineCapStrokeJoin(customcap: *mut GpCustomLineCap, linejoin: LineJoin) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetCustomLineCapStrokeJoin(customcap : *mut GpCustomLineCap, linejoin : LineJoin) -> Status);
    GdipSetCustomLineCapStrokeJoin(customcap, linejoin)
}
#[inline]
pub unsafe fn GdipSetCustomLineCapWidthScale(customcap: *mut GpCustomLineCap, widthscale: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetCustomLineCapWidthScale(customcap : *mut GpCustomLineCap, widthscale : f32) -> Status);
    GdipSetCustomLineCapWidthScale(customcap, widthscale)
}
#[inline]
pub unsafe fn GdipSetEffectParameters(effect: *mut CGpEffect, params: *const core::ffi::c_void, size: u32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetEffectParameters(effect : *mut CGpEffect, params : *const core::ffi::c_void, size : u32) -> Status);
    GdipSetEffectParameters(effect, params, size)
}
#[inline]
pub unsafe fn GdipSetEmpty(region: *mut GpRegion) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetEmpty(region : *mut GpRegion) -> Status);
    GdipSetEmpty(region)
}
#[inline]
pub unsafe fn GdipSetImageAttributesCachedBackground<P0>(imageattr: *mut GpImageAttributes, enableflag: P0) -> Status
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetImageAttributesCachedBackground(imageattr : *mut GpImageAttributes, enableflag : super::super::Foundation:: BOOL) -> Status);
    GdipSetImageAttributesCachedBackground(imageattr, enableflag.param().abi())
}
#[inline]
pub unsafe fn GdipSetImageAttributesColorKeys<P0>(imageattr: *mut GpImageAttributes, r#type: ColorAdjustType, enableflag: P0, colorlow: u32, colorhigh: u32) -> Status
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetImageAttributesColorKeys(imageattr : *mut GpImageAttributes, r#type : ColorAdjustType, enableflag : super::super::Foundation:: BOOL, colorlow : u32, colorhigh : u32) -> Status);
    GdipSetImageAttributesColorKeys(imageattr, r#type, enableflag.param().abi(), colorlow, colorhigh)
}
#[inline]
pub unsafe fn GdipSetImageAttributesColorMatrix<P0>(imageattr: *mut GpImageAttributes, r#type: ColorAdjustType, enableflag: P0, colormatrix: *const ColorMatrix, graymatrix: *const ColorMatrix, flags: ColorMatrixFlags) -> Status
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetImageAttributesColorMatrix(imageattr : *mut GpImageAttributes, r#type : ColorAdjustType, enableflag : super::super::Foundation:: BOOL, colormatrix : *const ColorMatrix, graymatrix : *const ColorMatrix, flags : ColorMatrixFlags) -> Status);
    GdipSetImageAttributesColorMatrix(imageattr, r#type, enableflag.param().abi(), colormatrix, graymatrix, flags)
}
#[inline]
pub unsafe fn GdipSetImageAttributesGamma<P0>(imageattr: *mut GpImageAttributes, r#type: ColorAdjustType, enableflag: P0, gamma: f32) -> Status
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetImageAttributesGamma(imageattr : *mut GpImageAttributes, r#type : ColorAdjustType, enableflag : super::super::Foundation:: BOOL, gamma : f32) -> Status);
    GdipSetImageAttributesGamma(imageattr, r#type, enableflag.param().abi(), gamma)
}
#[inline]
pub unsafe fn GdipSetImageAttributesNoOp<P0>(imageattr: *mut GpImageAttributes, r#type: ColorAdjustType, enableflag: P0) -> Status
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetImageAttributesNoOp(imageattr : *mut GpImageAttributes, r#type : ColorAdjustType, enableflag : super::super::Foundation:: BOOL) -> Status);
    GdipSetImageAttributesNoOp(imageattr, r#type, enableflag.param().abi())
}
#[inline]
pub unsafe fn GdipSetImageAttributesOutputChannel<P0>(imageattr: *mut GpImageAttributes, r#type: ColorAdjustType, enableflag: P0, channelflags: ColorChannelFlags) -> Status
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetImageAttributesOutputChannel(imageattr : *mut GpImageAttributes, r#type : ColorAdjustType, enableflag : super::super::Foundation:: BOOL, channelflags : ColorChannelFlags) -> Status);
    GdipSetImageAttributesOutputChannel(imageattr, r#type, enableflag.param().abi(), channelflags)
}
#[inline]
pub unsafe fn GdipSetImageAttributesOutputChannelColorProfile<P0, P1>(imageattr: *mut GpImageAttributes, r#type: ColorAdjustType, enableflag: P0, colorprofilefilename: P1) -> Status
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetImageAttributesOutputChannelColorProfile(imageattr : *mut GpImageAttributes, r#type : ColorAdjustType, enableflag : super::super::Foundation:: BOOL, colorprofilefilename : windows_core::PCWSTR) -> Status);
    GdipSetImageAttributesOutputChannelColorProfile(imageattr, r#type, enableflag.param().abi(), colorprofilefilename.param().abi())
}
#[inline]
pub unsafe fn GdipSetImageAttributesRemapTable<P0>(imageattr: *mut GpImageAttributes, r#type: ColorAdjustType, enableflag: P0, mapsize: u32, map: *const ColorMap) -> Status
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetImageAttributesRemapTable(imageattr : *mut GpImageAttributes, r#type : ColorAdjustType, enableflag : super::super::Foundation:: BOOL, mapsize : u32, map : *const ColorMap) -> Status);
    GdipSetImageAttributesRemapTable(imageattr, r#type, enableflag.param().abi(), mapsize, map)
}
#[inline]
pub unsafe fn GdipSetImageAttributesThreshold<P0>(imageattr: *mut GpImageAttributes, r#type: ColorAdjustType, enableflag: P0, threshold: f32) -> Status
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetImageAttributesThreshold(imageattr : *mut GpImageAttributes, r#type : ColorAdjustType, enableflag : super::super::Foundation:: BOOL, threshold : f32) -> Status);
    GdipSetImageAttributesThreshold(imageattr, r#type, enableflag.param().abi(), threshold)
}
#[inline]
pub unsafe fn GdipSetImageAttributesToIdentity(imageattr: *mut GpImageAttributes, r#type: ColorAdjustType) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetImageAttributesToIdentity(imageattr : *mut GpImageAttributes, r#type : ColorAdjustType) -> Status);
    GdipSetImageAttributesToIdentity(imageattr, r#type)
}
#[inline]
pub unsafe fn GdipSetImageAttributesWrapMode<P0>(imageattr: *mut GpImageAttributes, wrap: WrapMode, argb: u32, clamp: P0) -> Status
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetImageAttributesWrapMode(imageattr : *mut GpImageAttributes, wrap : WrapMode, argb : u32, clamp : super::super::Foundation:: BOOL) -> Status);
    GdipSetImageAttributesWrapMode(imageattr, wrap, argb, clamp.param().abi())
}
#[inline]
pub unsafe fn GdipSetImagePalette(image: *mut GpImage, palette: *const ColorPalette) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetImagePalette(image : *mut GpImage, palette : *const ColorPalette) -> Status);
    GdipSetImagePalette(image, palette)
}
#[inline]
pub unsafe fn GdipSetInfinite(region: *mut GpRegion) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetInfinite(region : *mut GpRegion) -> Status);
    GdipSetInfinite(region)
}
#[inline]
pub unsafe fn GdipSetInterpolationMode(graphics: *mut GpGraphics, interpolationmode: InterpolationMode) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetInterpolationMode(graphics : *mut GpGraphics, interpolationmode : InterpolationMode) -> Status);
    GdipSetInterpolationMode(graphics, interpolationmode)
}
#[inline]
pub unsafe fn GdipSetLineBlend(brush: *mut GpLineGradient, blend: *const f32, positions: *const f32, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetLineBlend(brush : *mut GpLineGradient, blend : *const f32, positions : *const f32, count : i32) -> Status);
    GdipSetLineBlend(brush, blend, positions, count)
}
#[inline]
pub unsafe fn GdipSetLineColors(brush: *mut GpLineGradient, color1: u32, color2: u32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetLineColors(brush : *mut GpLineGradient, color1 : u32, color2 : u32) -> Status);
    GdipSetLineColors(brush, color1, color2)
}
#[inline]
pub unsafe fn GdipSetLineGammaCorrection<P0>(brush: *mut GpLineGradient, usegammacorrection: P0) -> Status
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetLineGammaCorrection(brush : *mut GpLineGradient, usegammacorrection : super::super::Foundation:: BOOL) -> Status);
    GdipSetLineGammaCorrection(brush, usegammacorrection.param().abi())
}
#[inline]
pub unsafe fn GdipSetLineLinearBlend(brush: *mut GpLineGradient, focus: f32, scale: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetLineLinearBlend(brush : *mut GpLineGradient, focus : f32, scale : f32) -> Status);
    GdipSetLineLinearBlend(brush, focus, scale)
}
#[inline]
pub unsafe fn GdipSetLinePresetBlend(brush: *mut GpLineGradient, blend: *const u32, positions: *const f32, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetLinePresetBlend(brush : *mut GpLineGradient, blend : *const u32, positions : *const f32, count : i32) -> Status);
    GdipSetLinePresetBlend(brush, blend, positions, count)
}
#[inline]
pub unsafe fn GdipSetLineSigmaBlend(brush: *mut GpLineGradient, focus: f32, scale: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetLineSigmaBlend(brush : *mut GpLineGradient, focus : f32, scale : f32) -> Status);
    GdipSetLineSigmaBlend(brush, focus, scale)
}
#[inline]
pub unsafe fn GdipSetLineTransform(brush: *mut GpLineGradient, matrix: *const Matrix) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetLineTransform(brush : *mut GpLineGradient, matrix : *const Matrix) -> Status);
    GdipSetLineTransform(brush, matrix)
}
#[inline]
pub unsafe fn GdipSetLineWrapMode(brush: *mut GpLineGradient, wrapmode: WrapMode) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetLineWrapMode(brush : *mut GpLineGradient, wrapmode : WrapMode) -> Status);
    GdipSetLineWrapMode(brush, wrapmode)
}
#[inline]
pub unsafe fn GdipSetMatrixElements(matrix: *mut Matrix, m11: f32, m12: f32, m21: f32, m22: f32, dx: f32, dy: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetMatrixElements(matrix : *mut Matrix, m11 : f32, m12 : f32, m21 : f32, m22 : f32, dx : f32, dy : f32) -> Status);
    GdipSetMatrixElements(matrix, m11, m12, m21, m22, dx, dy)
}
#[inline]
pub unsafe fn GdipSetMetafileDownLevelRasterizationLimit(metafile: *mut GpMetafile, metafilerasterizationlimitdpi: u32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetMetafileDownLevelRasterizationLimit(metafile : *mut GpMetafile, metafilerasterizationlimitdpi : u32) -> Status);
    GdipSetMetafileDownLevelRasterizationLimit(metafile, metafilerasterizationlimitdpi)
}
#[inline]
pub unsafe fn GdipSetPageScale(graphics: *mut GpGraphics, scale: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetPageScale(graphics : *mut GpGraphics, scale : f32) -> Status);
    GdipSetPageScale(graphics, scale)
}
#[inline]
pub unsafe fn GdipSetPageUnit(graphics: *mut GpGraphics, unit: Unit) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetPageUnit(graphics : *mut GpGraphics, unit : Unit) -> Status);
    GdipSetPageUnit(graphics, unit)
}
#[inline]
pub unsafe fn GdipSetPathFillMode(path: *mut GpPath, fillmode: FillMode) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetPathFillMode(path : *mut GpPath, fillmode : FillMode) -> Status);
    GdipSetPathFillMode(path, fillmode)
}
#[inline]
pub unsafe fn GdipSetPathGradientBlend(brush: *mut GpPathGradient, blend: *const f32, positions: *const f32, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetPathGradientBlend(brush : *mut GpPathGradient, blend : *const f32, positions : *const f32, count : i32) -> Status);
    GdipSetPathGradientBlend(brush, blend, positions, count)
}
#[inline]
pub unsafe fn GdipSetPathGradientCenterColor(brush: *mut GpPathGradient, colors: u32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetPathGradientCenterColor(brush : *mut GpPathGradient, colors : u32) -> Status);
    GdipSetPathGradientCenterColor(brush, colors)
}
#[inline]
pub unsafe fn GdipSetPathGradientCenterPoint(brush: *mut GpPathGradient, points: *const PointF) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetPathGradientCenterPoint(brush : *mut GpPathGradient, points : *const PointF) -> Status);
    GdipSetPathGradientCenterPoint(brush, points)
}
#[inline]
pub unsafe fn GdipSetPathGradientCenterPointI(brush: *mut GpPathGradient, points: *const Point) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetPathGradientCenterPointI(brush : *mut GpPathGradient, points : *const Point) -> Status);
    GdipSetPathGradientCenterPointI(brush, points)
}
#[inline]
pub unsafe fn GdipSetPathGradientFocusScales(brush: *mut GpPathGradient, xscale: f32, yscale: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetPathGradientFocusScales(brush : *mut GpPathGradient, xscale : f32, yscale : f32) -> Status);
    GdipSetPathGradientFocusScales(brush, xscale, yscale)
}
#[inline]
pub unsafe fn GdipSetPathGradientGammaCorrection<P0>(brush: *mut GpPathGradient, usegammacorrection: P0) -> Status
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetPathGradientGammaCorrection(brush : *mut GpPathGradient, usegammacorrection : super::super::Foundation:: BOOL) -> Status);
    GdipSetPathGradientGammaCorrection(brush, usegammacorrection.param().abi())
}
#[inline]
pub unsafe fn GdipSetPathGradientLinearBlend(brush: *mut GpPathGradient, focus: f32, scale: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetPathGradientLinearBlend(brush : *mut GpPathGradient, focus : f32, scale : f32) -> Status);
    GdipSetPathGradientLinearBlend(brush, focus, scale)
}
#[inline]
pub unsafe fn GdipSetPathGradientPath(brush: *mut GpPathGradient, path: *const GpPath) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetPathGradientPath(brush : *mut GpPathGradient, path : *const GpPath) -> Status);
    GdipSetPathGradientPath(brush, path)
}
#[inline]
pub unsafe fn GdipSetPathGradientPresetBlend(brush: *mut GpPathGradient, blend: *const u32, positions: *const f32, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetPathGradientPresetBlend(brush : *mut GpPathGradient, blend : *const u32, positions : *const f32, count : i32) -> Status);
    GdipSetPathGradientPresetBlend(brush, blend, positions, count)
}
#[inline]
pub unsafe fn GdipSetPathGradientSigmaBlend(brush: *mut GpPathGradient, focus: f32, scale: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetPathGradientSigmaBlend(brush : *mut GpPathGradient, focus : f32, scale : f32) -> Status);
    GdipSetPathGradientSigmaBlend(brush, focus, scale)
}
#[inline]
pub unsafe fn GdipSetPathGradientSurroundColorsWithCount(brush: *mut GpPathGradient, color: *const u32, count: *mut i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetPathGradientSurroundColorsWithCount(brush : *mut GpPathGradient, color : *const u32, count : *mut i32) -> Status);
    GdipSetPathGradientSurroundColorsWithCount(brush, color, count)
}
#[inline]
pub unsafe fn GdipSetPathGradientTransform(brush: *mut GpPathGradient, matrix: *mut Matrix) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetPathGradientTransform(brush : *mut GpPathGradient, matrix : *mut Matrix) -> Status);
    GdipSetPathGradientTransform(brush, matrix)
}
#[inline]
pub unsafe fn GdipSetPathGradientWrapMode(brush: *mut GpPathGradient, wrapmode: WrapMode) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetPathGradientWrapMode(brush : *mut GpPathGradient, wrapmode : WrapMode) -> Status);
    GdipSetPathGradientWrapMode(brush, wrapmode)
}
#[inline]
pub unsafe fn GdipSetPathMarker(path: *mut GpPath) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetPathMarker(path : *mut GpPath) -> Status);
    GdipSetPathMarker(path)
}
#[inline]
pub unsafe fn GdipSetPenBrushFill(pen: *mut GpPen, brush: *mut GpBrush) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetPenBrushFill(pen : *mut GpPen, brush : *mut GpBrush) -> Status);
    GdipSetPenBrushFill(pen, brush)
}
#[inline]
pub unsafe fn GdipSetPenColor(pen: *mut GpPen, argb: u32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetPenColor(pen : *mut GpPen, argb : u32) -> Status);
    GdipSetPenColor(pen, argb)
}
#[inline]
pub unsafe fn GdipSetPenCompoundArray(pen: *mut GpPen, dash: *const f32, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetPenCompoundArray(pen : *mut GpPen, dash : *const f32, count : i32) -> Status);
    GdipSetPenCompoundArray(pen, dash, count)
}
#[inline]
pub unsafe fn GdipSetPenCustomEndCap(pen: *mut GpPen, customcap: *mut GpCustomLineCap) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetPenCustomEndCap(pen : *mut GpPen, customcap : *mut GpCustomLineCap) -> Status);
    GdipSetPenCustomEndCap(pen, customcap)
}
#[inline]
pub unsafe fn GdipSetPenCustomStartCap(pen: *mut GpPen, customcap: *mut GpCustomLineCap) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetPenCustomStartCap(pen : *mut GpPen, customcap : *mut GpCustomLineCap) -> Status);
    GdipSetPenCustomStartCap(pen, customcap)
}
#[inline]
pub unsafe fn GdipSetPenDashArray(pen: *mut GpPen, dash: *const f32, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetPenDashArray(pen : *mut GpPen, dash : *const f32, count : i32) -> Status);
    GdipSetPenDashArray(pen, dash, count)
}
#[inline]
pub unsafe fn GdipSetPenDashCap197819(pen: *mut GpPen, dashcap: DashCap) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetPenDashCap197819(pen : *mut GpPen, dashcap : DashCap) -> Status);
    GdipSetPenDashCap197819(pen, dashcap)
}
#[inline]
pub unsafe fn GdipSetPenDashOffset(pen: *mut GpPen, offset: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetPenDashOffset(pen : *mut GpPen, offset : f32) -> Status);
    GdipSetPenDashOffset(pen, offset)
}
#[inline]
pub unsafe fn GdipSetPenDashStyle(pen: *mut GpPen, dashstyle: DashStyle) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetPenDashStyle(pen : *mut GpPen, dashstyle : DashStyle) -> Status);
    GdipSetPenDashStyle(pen, dashstyle)
}
#[inline]
pub unsafe fn GdipSetPenEndCap(pen: *mut GpPen, endcap: LineCap) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetPenEndCap(pen : *mut GpPen, endcap : LineCap) -> Status);
    GdipSetPenEndCap(pen, endcap)
}
#[inline]
pub unsafe fn GdipSetPenLineCap197819(pen: *mut GpPen, startcap: LineCap, endcap: LineCap, dashcap: DashCap) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetPenLineCap197819(pen : *mut GpPen, startcap : LineCap, endcap : LineCap, dashcap : DashCap) -> Status);
    GdipSetPenLineCap197819(pen, startcap, endcap, dashcap)
}
#[inline]
pub unsafe fn GdipSetPenLineJoin(pen: *mut GpPen, linejoin: LineJoin) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetPenLineJoin(pen : *mut GpPen, linejoin : LineJoin) -> Status);
    GdipSetPenLineJoin(pen, linejoin)
}
#[inline]
pub unsafe fn GdipSetPenMiterLimit(pen: *mut GpPen, miterlimit: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetPenMiterLimit(pen : *mut GpPen, miterlimit : f32) -> Status);
    GdipSetPenMiterLimit(pen, miterlimit)
}
#[inline]
pub unsafe fn GdipSetPenMode(pen: *mut GpPen, penmode: PenAlignment) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetPenMode(pen : *mut GpPen, penmode : PenAlignment) -> Status);
    GdipSetPenMode(pen, penmode)
}
#[inline]
pub unsafe fn GdipSetPenStartCap(pen: *mut GpPen, startcap: LineCap) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetPenStartCap(pen : *mut GpPen, startcap : LineCap) -> Status);
    GdipSetPenStartCap(pen, startcap)
}
#[inline]
pub unsafe fn GdipSetPenTransform(pen: *mut GpPen, matrix: *mut Matrix) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetPenTransform(pen : *mut GpPen, matrix : *mut Matrix) -> Status);
    GdipSetPenTransform(pen, matrix)
}
#[inline]
pub unsafe fn GdipSetPenUnit(pen: *mut GpPen, unit: Unit) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetPenUnit(pen : *mut GpPen, unit : Unit) -> Status);
    GdipSetPenUnit(pen, unit)
}
#[inline]
pub unsafe fn GdipSetPenWidth(pen: *mut GpPen, width: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetPenWidth(pen : *mut GpPen, width : f32) -> Status);
    GdipSetPenWidth(pen, width)
}
#[inline]
pub unsafe fn GdipSetPixelOffsetMode(graphics: *mut GpGraphics, pixeloffsetmode: PixelOffsetMode) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetPixelOffsetMode(graphics : *mut GpGraphics, pixeloffsetmode : PixelOffsetMode) -> Status);
    GdipSetPixelOffsetMode(graphics, pixeloffsetmode)
}
#[inline]
pub unsafe fn GdipSetPropertyItem(image: *mut GpImage, item: *const PropertyItem) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetPropertyItem(image : *mut GpImage, item : *const PropertyItem) -> Status);
    GdipSetPropertyItem(image, item)
}
#[inline]
pub unsafe fn GdipSetRenderingOrigin(graphics: *mut GpGraphics, x: i32, y: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetRenderingOrigin(graphics : *mut GpGraphics, x : i32, y : i32) -> Status);
    GdipSetRenderingOrigin(graphics, x, y)
}
#[inline]
pub unsafe fn GdipSetSmoothingMode(graphics: *mut GpGraphics, smoothingmode: SmoothingMode) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetSmoothingMode(graphics : *mut GpGraphics, smoothingmode : SmoothingMode) -> Status);
    GdipSetSmoothingMode(graphics, smoothingmode)
}
#[inline]
pub unsafe fn GdipSetSolidFillColor(brush: *mut GpSolidFill, color: u32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetSolidFillColor(brush : *mut GpSolidFill, color : u32) -> Status);
    GdipSetSolidFillColor(brush, color)
}
#[inline]
pub unsafe fn GdipSetStringFormatAlign(format: *mut GpStringFormat, align: StringAlignment) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetStringFormatAlign(format : *mut GpStringFormat, align : StringAlignment) -> Status);
    GdipSetStringFormatAlign(format, align)
}
#[inline]
pub unsafe fn GdipSetStringFormatDigitSubstitution(format: *mut GpStringFormat, language: u16, substitute: StringDigitSubstitute) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetStringFormatDigitSubstitution(format : *mut GpStringFormat, language : u16, substitute : StringDigitSubstitute) -> Status);
    GdipSetStringFormatDigitSubstitution(format, language, substitute)
}
#[inline]
pub unsafe fn GdipSetStringFormatFlags(format: *mut GpStringFormat, flags: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetStringFormatFlags(format : *mut GpStringFormat, flags : i32) -> Status);
    GdipSetStringFormatFlags(format, flags)
}
#[inline]
pub unsafe fn GdipSetStringFormatHotkeyPrefix(format: *mut GpStringFormat, hotkeyprefix: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetStringFormatHotkeyPrefix(format : *mut GpStringFormat, hotkeyprefix : i32) -> Status);
    GdipSetStringFormatHotkeyPrefix(format, hotkeyprefix)
}
#[inline]
pub unsafe fn GdipSetStringFormatLineAlign(format: *mut GpStringFormat, align: StringAlignment) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetStringFormatLineAlign(format : *mut GpStringFormat, align : StringAlignment) -> Status);
    GdipSetStringFormatLineAlign(format, align)
}
#[inline]
pub unsafe fn GdipSetStringFormatMeasurableCharacterRanges(format: *mut GpStringFormat, rangecount: i32, ranges: *const CharacterRange) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetStringFormatMeasurableCharacterRanges(format : *mut GpStringFormat, rangecount : i32, ranges : *const CharacterRange) -> Status);
    GdipSetStringFormatMeasurableCharacterRanges(format, rangecount, ranges)
}
#[inline]
pub unsafe fn GdipSetStringFormatTabStops(format: *mut GpStringFormat, firsttaboffset: f32, count: i32, tabstops: *const f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetStringFormatTabStops(format : *mut GpStringFormat, firsttaboffset : f32, count : i32, tabstops : *const f32) -> Status);
    GdipSetStringFormatTabStops(format, firsttaboffset, count, tabstops)
}
#[inline]
pub unsafe fn GdipSetStringFormatTrimming(format: *mut GpStringFormat, trimming: StringTrimming) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetStringFormatTrimming(format : *mut GpStringFormat, trimming : StringTrimming) -> Status);
    GdipSetStringFormatTrimming(format, trimming)
}
#[inline]
pub unsafe fn GdipSetTextContrast(graphics: *mut GpGraphics, contrast: u32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetTextContrast(graphics : *mut GpGraphics, contrast : u32) -> Status);
    GdipSetTextContrast(graphics, contrast)
}
#[inline]
pub unsafe fn GdipSetTextRenderingHint(graphics: *mut GpGraphics, mode: TextRenderingHint) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetTextRenderingHint(graphics : *mut GpGraphics, mode : TextRenderingHint) -> Status);
    GdipSetTextRenderingHint(graphics, mode)
}
#[inline]
pub unsafe fn GdipSetTextureTransform(brush: *mut GpTexture, matrix: *const Matrix) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetTextureTransform(brush : *mut GpTexture, matrix : *const Matrix) -> Status);
    GdipSetTextureTransform(brush, matrix)
}
#[inline]
pub unsafe fn GdipSetTextureWrapMode(brush: *mut GpTexture, wrapmode: WrapMode) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetTextureWrapMode(brush : *mut GpTexture, wrapmode : WrapMode) -> Status);
    GdipSetTextureWrapMode(brush, wrapmode)
}
#[inline]
pub unsafe fn GdipSetWorldTransform(graphics: *mut GpGraphics, matrix: *mut Matrix) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipSetWorldTransform(graphics : *mut GpGraphics, matrix : *mut Matrix) -> Status);
    GdipSetWorldTransform(graphics, matrix)
}
#[inline]
pub unsafe fn GdipShearMatrix(matrix: *mut Matrix, shearx: f32, sheary: f32, order: MatrixOrder) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipShearMatrix(matrix : *mut Matrix, shearx : f32, sheary : f32, order : MatrixOrder) -> Status);
    GdipShearMatrix(matrix, shearx, sheary, order)
}
#[inline]
pub unsafe fn GdipStartPathFigure(path: *mut GpPath) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipStartPathFigure(path : *mut GpPath) -> Status);
    GdipStartPathFigure(path)
}
#[inline]
pub unsafe fn GdipStringFormatGetGenericDefault(format: *mut *mut GpStringFormat) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipStringFormatGetGenericDefault(format : *mut *mut GpStringFormat) -> Status);
    GdipStringFormatGetGenericDefault(format)
}
#[inline]
pub unsafe fn GdipStringFormatGetGenericTypographic(format: *mut *mut GpStringFormat) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipStringFormatGetGenericTypographic(format : *mut *mut GpStringFormat) -> Status);
    GdipStringFormatGetGenericTypographic(format)
}
#[inline]
pub unsafe fn GdipTestControl(control: GpTestControlEnum, param1: *mut core::ffi::c_void) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipTestControl(control : GpTestControlEnum, param1 : *mut core::ffi::c_void) -> Status);
    GdipTestControl(control, param1)
}
#[inline]
pub unsafe fn GdipTransformMatrixPoints(matrix: *mut Matrix, pts: *mut PointF, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipTransformMatrixPoints(matrix : *mut Matrix, pts : *mut PointF, count : i32) -> Status);
    GdipTransformMatrixPoints(matrix, pts, count)
}
#[inline]
pub unsafe fn GdipTransformMatrixPointsI(matrix: *mut Matrix, pts: *mut Point, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipTransformMatrixPointsI(matrix : *mut Matrix, pts : *mut Point, count : i32) -> Status);
    GdipTransformMatrixPointsI(matrix, pts, count)
}
#[inline]
pub unsafe fn GdipTransformPath(path: *mut GpPath, matrix: *mut Matrix) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipTransformPath(path : *mut GpPath, matrix : *mut Matrix) -> Status);
    GdipTransformPath(path, matrix)
}
#[inline]
pub unsafe fn GdipTransformPoints(graphics: *mut GpGraphics, destspace: CoordinateSpace, srcspace: CoordinateSpace, points: *mut PointF, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipTransformPoints(graphics : *mut GpGraphics, destspace : CoordinateSpace, srcspace : CoordinateSpace, points : *mut PointF, count : i32) -> Status);
    GdipTransformPoints(graphics, destspace, srcspace, points, count)
}
#[inline]
pub unsafe fn GdipTransformPointsI(graphics: *mut GpGraphics, destspace: CoordinateSpace, srcspace: CoordinateSpace, points: *mut Point, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipTransformPointsI(graphics : *mut GpGraphics, destspace : CoordinateSpace, srcspace : CoordinateSpace, points : *mut Point, count : i32) -> Status);
    GdipTransformPointsI(graphics, destspace, srcspace, points, count)
}
#[inline]
pub unsafe fn GdipTransformRegion(region: *mut GpRegion, matrix: *mut Matrix) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipTransformRegion(region : *mut GpRegion, matrix : *mut Matrix) -> Status);
    GdipTransformRegion(region, matrix)
}
#[inline]
pub unsafe fn GdipTranslateClip(graphics: *mut GpGraphics, dx: f32, dy: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipTranslateClip(graphics : *mut GpGraphics, dx : f32, dy : f32) -> Status);
    GdipTranslateClip(graphics, dx, dy)
}
#[inline]
pub unsafe fn GdipTranslateClipI(graphics: *mut GpGraphics, dx: i32, dy: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipTranslateClipI(graphics : *mut GpGraphics, dx : i32, dy : i32) -> Status);
    GdipTranslateClipI(graphics, dx, dy)
}
#[inline]
pub unsafe fn GdipTranslateLineTransform(brush: *mut GpLineGradient, dx: f32, dy: f32, order: MatrixOrder) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipTranslateLineTransform(brush : *mut GpLineGradient, dx : f32, dy : f32, order : MatrixOrder) -> Status);
    GdipTranslateLineTransform(brush, dx, dy, order)
}
#[inline]
pub unsafe fn GdipTranslateMatrix(matrix: *mut Matrix, offsetx: f32, offsety: f32, order: MatrixOrder) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipTranslateMatrix(matrix : *mut Matrix, offsetx : f32, offsety : f32, order : MatrixOrder) -> Status);
    GdipTranslateMatrix(matrix, offsetx, offsety, order)
}
#[inline]
pub unsafe fn GdipTranslatePathGradientTransform(brush: *mut GpPathGradient, dx: f32, dy: f32, order: MatrixOrder) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipTranslatePathGradientTransform(brush : *mut GpPathGradient, dx : f32, dy : f32, order : MatrixOrder) -> Status);
    GdipTranslatePathGradientTransform(brush, dx, dy, order)
}
#[inline]
pub unsafe fn GdipTranslatePenTransform(pen: *mut GpPen, dx: f32, dy: f32, order: MatrixOrder) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipTranslatePenTransform(pen : *mut GpPen, dx : f32, dy : f32, order : MatrixOrder) -> Status);
    GdipTranslatePenTransform(pen, dx, dy, order)
}
#[inline]
pub unsafe fn GdipTranslateRegion(region: *mut GpRegion, dx: f32, dy: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipTranslateRegion(region : *mut GpRegion, dx : f32, dy : f32) -> Status);
    GdipTranslateRegion(region, dx, dy)
}
#[inline]
pub unsafe fn GdipTranslateRegionI(region: *mut GpRegion, dx: i32, dy: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipTranslateRegionI(region : *mut GpRegion, dx : i32, dy : i32) -> Status);
    GdipTranslateRegionI(region, dx, dy)
}
#[inline]
pub unsafe fn GdipTranslateTextureTransform(brush: *mut GpTexture, dx: f32, dy: f32, order: MatrixOrder) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipTranslateTextureTransform(brush : *mut GpTexture, dx : f32, dy : f32, order : MatrixOrder) -> Status);
    GdipTranslateTextureTransform(brush, dx, dy, order)
}
#[inline]
pub unsafe fn GdipTranslateWorldTransform(graphics: *mut GpGraphics, dx: f32, dy: f32, order: MatrixOrder) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipTranslateWorldTransform(graphics : *mut GpGraphics, dx : f32, dy : f32, order : MatrixOrder) -> Status);
    GdipTranslateWorldTransform(graphics, dx, dy, order)
}
#[inline]
pub unsafe fn GdipVectorTransformMatrixPoints(matrix: *mut Matrix, pts: *mut PointF, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipVectorTransformMatrixPoints(matrix : *mut Matrix, pts : *mut PointF, count : i32) -> Status);
    GdipVectorTransformMatrixPoints(matrix, pts, count)
}
#[inline]
pub unsafe fn GdipVectorTransformMatrixPointsI(matrix: *mut Matrix, pts: *mut Point, count: i32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipVectorTransformMatrixPointsI(matrix : *mut Matrix, pts : *mut Point, count : i32) -> Status);
    GdipVectorTransformMatrixPointsI(matrix, pts, count)
}
#[inline]
pub unsafe fn GdipWarpPath(path: *mut GpPath, matrix: *mut Matrix, points: *const PointF, count: i32, srcx: f32, srcy: f32, srcwidth: f32, srcheight: f32, warpmode: WarpMode, flatness: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipWarpPath(path : *mut GpPath, matrix : *mut Matrix, points : *const PointF, count : i32, srcx : f32, srcy : f32, srcwidth : f32, srcheight : f32, warpmode : WarpMode, flatness : f32) -> Status);
    GdipWarpPath(path, matrix, points, count, srcx, srcy, srcwidth, srcheight, warpmode, flatness)
}
#[inline]
pub unsafe fn GdipWidenPath(nativepath: *mut GpPath, pen: *mut GpPen, matrix: *mut Matrix, flatness: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipWidenPath(nativepath : *mut GpPath, pen : *mut GpPen, matrix : *mut Matrix, flatness : f32) -> Status);
    GdipWidenPath(nativepath, pen, matrix, flatness)
}
#[inline]
pub unsafe fn GdipWindingModeOutline(path: *mut GpPath, matrix: *mut Matrix, flatness: f32) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdipWindingModeOutline(path : *mut GpPath, matrix : *mut Matrix, flatness : f32) -> Status);
    GdipWindingModeOutline(path, matrix, flatness)
}
#[inline]
pub unsafe fn GdiplusNotificationHook(token: *mut usize) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdiplusNotificationHook(token : *mut usize) -> Status);
    GdiplusNotificationHook(token)
}
#[inline]
pub unsafe fn GdiplusNotificationUnhook(token: usize) {
    windows_targets::link!("gdiplus.dll" "system" fn GdiplusNotificationUnhook(token : usize));
    GdiplusNotificationUnhook(token)
}
#[inline]
pub unsafe fn GdiplusShutdown(token: usize) {
    windows_targets::link!("gdiplus.dll" "system" fn GdiplusShutdown(token : usize));
    GdiplusShutdown(token)
}
#[inline]
pub unsafe fn GdiplusStartup(token: *mut usize, input: *const GdiplusStartupInput, output: *mut GdiplusStartupOutput) -> Status {
    windows_targets::link!("gdiplus.dll" "system" fn GdiplusStartup(token : *mut usize, input : *const GdiplusStartupInput, output : *mut GdiplusStartupOutput) -> Status);
    GdiplusStartup(token, input, output)
}
windows_core::imp::define_interface!(GdiplusAbort, GdiplusAbort_Vtbl);
impl GdiplusAbort {
    pub unsafe fn Abort(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Abort)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct GdiplusAbort_Vtbl {
    pub Abort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IImageBytes, IImageBytes_Vtbl, 0x025d1823_6c7d_447b_bbdb_a3cbc3dfa2fc);
impl core::ops::Deref for IImageBytes {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IImageBytes, windows_core::IUnknown);
impl IImageBytes {
    pub unsafe fn CountBytes(&self, pcb: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).CountBytes)(windows_core::Interface::as_raw(self), pcb).ok()
    }
    pub unsafe fn LockBytes(&self, cb: u32, uloffset: u32, ppvbytes: *const *const core::ffi::c_void) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).LockBytes)(windows_core::Interface::as_raw(self), cb, uloffset, ppvbytes).ok()
    }
    pub unsafe fn UnlockBytes(&self, pvbytes: *const core::ffi::c_void, cb: u32, uloffset: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).UnlockBytes)(windows_core::Interface::as_raw(self), pvbytes, cb, uloffset).ok()
    }
}
#[repr(C)]
pub struct IImageBytes_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CountBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub LockBytes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *const core::ffi::c_void) -> windows_core::HRESULT,
    pub UnlockBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
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
pub const BlurEffectGuid: windows_core::GUID = windows_core::GUID::from_u128(0x633c80a4_1843_482b_9ef2_be2834c5fdd4);
pub const BrightnessContrastEffectGuid: windows_core::GUID = windows_core::GUID::from_u128(0xd3a1dbe1_8ec4_4c17_9f4c_ea97ad1c343d);
pub const BrushTypeHatchFill: BrushType = BrushType(1i32);
pub const BrushTypeLinearGradient: BrushType = BrushType(4i32);
pub const BrushTypePathGradient: BrushType = BrushType(3i32);
pub const BrushTypeSolidColor: BrushType = BrushType(0i32);
pub const BrushTypeTextureFill: BrushType = BrushType(2i32);
pub const CodecIImageBytes: windows_core::GUID = windows_core::GUID::from_u128(0x025d1823_6c7d_447b_bbdb_a3cbc3dfa2fc);
pub const ColorAdjustTypeAny: ColorAdjustType = ColorAdjustType(6i32);
pub const ColorAdjustTypeBitmap: ColorAdjustType = ColorAdjustType(1i32);
pub const ColorAdjustTypeBrush: ColorAdjustType = ColorAdjustType(2i32);
pub const ColorAdjustTypeCount: ColorAdjustType = ColorAdjustType(5i32);
pub const ColorAdjustTypeDefault: ColorAdjustType = ColorAdjustType(0i32);
pub const ColorAdjustTypePen: ColorAdjustType = ColorAdjustType(3i32);
pub const ColorAdjustTypeText: ColorAdjustType = ColorAdjustType(4i32);
pub const ColorBalanceEffectGuid: windows_core::GUID = windows_core::GUID::from_u128(0x537e597d_251e_48da_9664_29ca496b70f8);
pub const ColorChannelFlagsC: ColorChannelFlags = ColorChannelFlags(0i32);
pub const ColorChannelFlagsK: ColorChannelFlags = ColorChannelFlags(3i32);
pub const ColorChannelFlagsLast: ColorChannelFlags = ColorChannelFlags(4i32);
pub const ColorChannelFlagsM: ColorChannelFlags = ColorChannelFlags(1i32);
pub const ColorChannelFlagsY: ColorChannelFlags = ColorChannelFlags(2i32);
pub const ColorCurveEffectGuid: windows_core::GUID = windows_core::GUID::from_u128(0xdd6a0022_58e4_4a67_9d9b_d48eb881a53d);
pub const ColorLUTEffectGuid: windows_core::GUID = windows_core::GUID::from_u128(0xa7ce72a9_0f7f_40d7_b3cc_d0c02d5c3212);
pub const ColorMatrixEffectGuid: windows_core::GUID = windows_core::GUID::from_u128(0x718f2615_7933_40e3_a511_5f68fe14dd74);
pub const ColorMatrixFlagsAltGray: ColorMatrixFlags = ColorMatrixFlags(2i32);
pub const ColorMatrixFlagsDefault: ColorMatrixFlags = ColorMatrixFlags(0i32);
pub const ColorMatrixFlagsSkipGrays: ColorMatrixFlags = ColorMatrixFlags(1i32);
pub const ColorModeARGB32: ColorMode = ColorMode(0i32);
pub const ColorModeARGB64: ColorMode = ColorMode(1i32);
pub const CombineModeComplement: CombineMode = CombineMode(5i32);
pub const CombineModeExclude: CombineMode = CombineMode(4i32);
pub const CombineModeIntersect: CombineMode = CombineMode(1i32);
pub const CombineModeReplace: CombineMode = CombineMode(0i32);
pub const CombineModeUnion: CombineMode = CombineMode(2i32);
pub const CombineModeXor: CombineMode = CombineMode(3i32);
pub const CompositingModeSourceCopy: CompositingMode = CompositingMode(1i32);
pub const CompositingModeSourceOver: CompositingMode = CompositingMode(0i32);
pub const CompositingQualityAssumeLinear: CompositingQuality = CompositingQuality(4i32);
pub const CompositingQualityDefault: CompositingQuality = CompositingQuality(0i32);
pub const CompositingQualityGammaCorrected: CompositingQuality = CompositingQuality(3i32);
pub const CompositingQualityHighQuality: CompositingQuality = CompositingQuality(2i32);
pub const CompositingQualityHighSpeed: CompositingQuality = CompositingQuality(1i32);
pub const CompositingQualityInvalid: CompositingQuality = CompositingQuality(-1i32);
pub const ConvertToEmfPlusFlagsDefault: ConvertToEmfPlusFlags = ConvertToEmfPlusFlags(0i32);
pub const ConvertToEmfPlusFlagsInvalidRecord: ConvertToEmfPlusFlags = ConvertToEmfPlusFlags(4i32);
pub const ConvertToEmfPlusFlagsRopUsed: ConvertToEmfPlusFlags = ConvertToEmfPlusFlags(1i32);
pub const ConvertToEmfPlusFlagsText: ConvertToEmfPlusFlags = ConvertToEmfPlusFlags(2i32);
pub const CoordinateSpaceDevice: CoordinateSpace = CoordinateSpace(2i32);
pub const CoordinateSpacePage: CoordinateSpace = CoordinateSpace(1i32);
pub const CoordinateSpaceWorld: CoordinateSpace = CoordinateSpace(0i32);
pub const CurveChannelAll: CurveChannel = CurveChannel(0i32);
pub const CurveChannelBlue: CurveChannel = CurveChannel(3i32);
pub const CurveChannelGreen: CurveChannel = CurveChannel(2i32);
pub const CurveChannelRed: CurveChannel = CurveChannel(1i32);
pub const CustomLineCapTypeAdjustableArrow: CustomLineCapType = CustomLineCapType(1i32);
pub const CustomLineCapTypeDefault: CustomLineCapType = CustomLineCapType(0i32);
pub const DashCapFlat: DashCap = DashCap(0i32);
pub const DashCapRound: DashCap = DashCap(2i32);
pub const DashCapTriangle: DashCap = DashCap(3i32);
pub const DashStyleCustom: DashStyle = DashStyle(5i32);
pub const DashStyleDash: DashStyle = DashStyle(1i32);
pub const DashStyleDashDot: DashStyle = DashStyle(3i32);
pub const DashStyleDashDotDot: DashStyle = DashStyle(4i32);
pub const DashStyleDot: DashStyle = DashStyle(2i32);
pub const DashStyleSolid: DashStyle = DashStyle(0i32);
pub const DebugEventLevelFatal: DebugEventLevel = DebugEventLevel(0i32);
pub const DebugEventLevelWarning: DebugEventLevel = DebugEventLevel(1i32);
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
pub const DriverStringOptionsCmapLookup: DriverStringOptions = DriverStringOptions(1i32);
pub const DriverStringOptionsLimitSubpixel: DriverStringOptions = DriverStringOptions(8i32);
pub const DriverStringOptionsRealizedAdvance: DriverStringOptions = DriverStringOptions(4i32);
pub const DriverStringOptionsVertical: DriverStringOptions = DriverStringOptions(2i32);
pub const EmfPlusRecordTotal: EmfPlusRecordType = EmfPlusRecordType(16443i32);
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
pub const EmfToWmfBitsFlagsDefault: EmfToWmfBitsFlags = EmfToWmfBitsFlags(0i32);
pub const EmfToWmfBitsFlagsEmbedEmf: EmfToWmfBitsFlags = EmfToWmfBitsFlags(1i32);
pub const EmfToWmfBitsFlagsIncludePlaceable: EmfToWmfBitsFlags = EmfToWmfBitsFlags(2i32);
pub const EmfToWmfBitsFlagsNoXORClip: EmfToWmfBitsFlags = EmfToWmfBitsFlags(4i32);
pub const EmfTypeEmfOnly: EmfType = EmfType(3i32);
pub const EmfTypeEmfPlusDual: EmfType = EmfType(5i32);
pub const EmfTypeEmfPlusOnly: EmfType = EmfType(4i32);
pub const EncoderChrominanceTable: windows_core::GUID = windows_core::GUID::from_u128(0xf2e455dc_09b3_4316_8260_676ada32481c);
pub const EncoderColorDepth: windows_core::GUID = windows_core::GUID::from_u128(0x66087055_ad66_4c7c_9a18_38a2310b8337);
pub const EncoderColorSpace: windows_core::GUID = windows_core::GUID::from_u128(0xae7a62a0_ee2c_49d8_9d07_1ba8a927596e);
pub const EncoderCompression: windows_core::GUID = windows_core::GUID::from_u128(0xe09d739d_ccd4_44ee_8eba_3fbf8be4fc58);
pub const EncoderImageItems: windows_core::GUID = windows_core::GUID::from_u128(0x63875e13_1f1d_45ab_9195_a29b6066a650);
pub const EncoderLuminanceTable: windows_core::GUID = windows_core::GUID::from_u128(0xedb33bce_0266_4a77_b904_27216099e717);
pub const EncoderParameterValueTypeASCII: EncoderParameterValueType = EncoderParameterValueType(2i32);
pub const EncoderParameterValueTypeByte: EncoderParameterValueType = EncoderParameterValueType(1i32);
pub const EncoderParameterValueTypeLong: EncoderParameterValueType = EncoderParameterValueType(4i32);
pub const EncoderParameterValueTypeLongRange: EncoderParameterValueType = EncoderParameterValueType(6i32);
pub const EncoderParameterValueTypePointer: EncoderParameterValueType = EncoderParameterValueType(9i32);
pub const EncoderParameterValueTypeRational: EncoderParameterValueType = EncoderParameterValueType(5i32);
pub const EncoderParameterValueTypeRationalRange: EncoderParameterValueType = EncoderParameterValueType(8i32);
pub const EncoderParameterValueTypeShort: EncoderParameterValueType = EncoderParameterValueType(3i32);
pub const EncoderParameterValueTypeUndefined: EncoderParameterValueType = EncoderParameterValueType(7i32);
pub const EncoderQuality: windows_core::GUID = windows_core::GUID::from_u128(0x1d5be4b5_fa4a_452d_9cdd_5db35105e7eb);
pub const EncoderRenderMethod: windows_core::GUID = windows_core::GUID::from_u128(0x6d42c53a_229a_4825_8bb7_5c99e2b9a8b8);
pub const EncoderSaveAsCMYK: windows_core::GUID = windows_core::GUID::from_u128(0xa219bbc9_0a9d_4005_a3ee_3a421b8bb06c);
pub const EncoderSaveFlag: windows_core::GUID = windows_core::GUID::from_u128(0x292266fc_ac40_47bf_8cfc_a85b89a655de);
pub const EncoderScanMethod: windows_core::GUID = windows_core::GUID::from_u128(0x3a4e2661_3109_4e56_8536_42c156e7dcfa);
pub const EncoderTransformation: windows_core::GUID = windows_core::GUID::from_u128(0x8d0eb2d1_a58e_4ea8_aa14_108074b7b6f9);
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
pub const FileNotFound: Status = Status(10i32);
pub const FillModeAlternate: FillMode = FillMode(0i32);
pub const FillModeWinding: FillMode = FillMode(1i32);
pub const FlatnessDefault: f32 = 0.25f32;
pub const FlushIntentionFlush: FlushIntention = FlushIntention(0i32);
pub const FlushIntentionSync: FlushIntention = FlushIntention(1i32);
pub const FontFamilyNotFound: Status = Status(14i32);
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
pub const GdiplusNotInitialized: Status = Status(18i32);
pub const GdiplusStartupDefault: GdiplusStartupParams = GdiplusStartupParams(0i32);
pub const GdiplusStartupNoSetRound: GdiplusStartupParams = GdiplusStartupParams(1i32);
pub const GdiplusStartupSetPSValue: GdiplusStartupParams = GdiplusStartupParams(2i32);
pub const GdiplusStartupTransparencyMask: GdiplusStartupParams = GdiplusStartupParams(-16777216i32);
pub const GenericError: Status = Status(1i32);
pub const GenericFontFamilyMonospace: GenericFontFamily = GenericFontFamily(2i32);
pub const GenericFontFamilySansSerif: GenericFontFamily = GenericFontFamily(1i32);
pub const GenericFontFamilySerif: GenericFontFamily = GenericFontFamily(0i32);
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
pub const HistogramFormatA: HistogramFormat = HistogramFormat(7i32);
pub const HistogramFormatARGB: HistogramFormat = HistogramFormat(0i32);
pub const HistogramFormatB: HistogramFormat = HistogramFormat(4i32);
pub const HistogramFormatG: HistogramFormat = HistogramFormat(5i32);
pub const HistogramFormatGray: HistogramFormat = HistogramFormat(3i32);
pub const HistogramFormatPARGB: HistogramFormat = HistogramFormat(1i32);
pub const HistogramFormatR: HistogramFormat = HistogramFormat(6i32);
pub const HistogramFormatRGB: HistogramFormat = HistogramFormat(2i32);
pub const HotkeyPrefixHide: HotkeyPrefix = HotkeyPrefix(2i32);
pub const HotkeyPrefixNone: HotkeyPrefix = HotkeyPrefix(0i32);
pub const HotkeyPrefixShow: HotkeyPrefix = HotkeyPrefix(1i32);
pub const HueSaturationLightnessEffectGuid: windows_core::GUID = windows_core::GUID::from_u128(0x8b2dd6c3_eb07_4d87_a5f0_7108e26a9c5f);
pub const ImageCodecFlagsBlockingDecode: ImageCodecFlags = ImageCodecFlags(32i32);
pub const ImageCodecFlagsBuiltin: ImageCodecFlags = ImageCodecFlags(65536i32);
pub const ImageCodecFlagsDecoder: ImageCodecFlags = ImageCodecFlags(2i32);
pub const ImageCodecFlagsEncoder: ImageCodecFlags = ImageCodecFlags(1i32);
pub const ImageCodecFlagsSeekableEncode: ImageCodecFlags = ImageCodecFlags(16i32);
pub const ImageCodecFlagsSupportBitmap: ImageCodecFlags = ImageCodecFlags(4i32);
pub const ImageCodecFlagsSupportVector: ImageCodecFlags = ImageCodecFlags(8i32);
pub const ImageCodecFlagsSystem: ImageCodecFlags = ImageCodecFlags(131072i32);
pub const ImageCodecFlagsUser: ImageCodecFlags = ImageCodecFlags(262144i32);
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
pub const ImageLockModeRead: ImageLockMode = ImageLockMode(1i32);
pub const ImageLockModeUserInputBuf: ImageLockMode = ImageLockMode(4i32);
pub const ImageLockModeWrite: ImageLockMode = ImageLockMode(2i32);
pub const ImageTypeBitmap: ImageType = ImageType(1i32);
pub const ImageTypeMetafile: ImageType = ImageType(2i32);
pub const ImageTypeUnknown: ImageType = ImageType(0i32);
pub const InsufficientBuffer: Status = Status(5i32);
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
pub const ItemDataPositionAfterBits: ItemDataPosition = ItemDataPosition(2i32);
pub const ItemDataPositionAfterHeader: ItemDataPosition = ItemDataPosition(0i32);
pub const ItemDataPositionAfterPalette: ItemDataPosition = ItemDataPosition(1i32);
pub const LevelsEffectGuid: windows_core::GUID = windows_core::GUID::from_u128(0x99c354ec_2a31_4f3a_8c34_17a803b33a25);
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
pub const LineJoinBevel: LineJoin = LineJoin(1i32);
pub const LineJoinMiter: LineJoin = LineJoin(0i32);
pub const LineJoinMiterClipped: LineJoin = LineJoin(3i32);
pub const LineJoinRound: LineJoin = LineJoin(2i32);
pub const LinearGradientModeBackwardDiagonal: LinearGradientMode = LinearGradientMode(3i32);
pub const LinearGradientModeForwardDiagonal: LinearGradientMode = LinearGradientMode(2i32);
pub const LinearGradientModeHorizontal: LinearGradientMode = LinearGradientMode(0i32);
pub const LinearGradientModeVertical: LinearGradientMode = LinearGradientMode(1i32);
pub const MatrixOrderAppend: MatrixOrder = MatrixOrder(1i32);
pub const MatrixOrderPrepend: MatrixOrder = MatrixOrder(0i32);
pub const MetafileFrameUnitDocument: MetafileFrameUnit = MetafileFrameUnit(5i32);
pub const MetafileFrameUnitGdi: MetafileFrameUnit = MetafileFrameUnit(7i32);
pub const MetafileFrameUnitInch: MetafileFrameUnit = MetafileFrameUnit(4i32);
pub const MetafileFrameUnitMillimeter: MetafileFrameUnit = MetafileFrameUnit(6i32);
pub const MetafileFrameUnitPixel: MetafileFrameUnit = MetafileFrameUnit(2i32);
pub const MetafileFrameUnitPoint: MetafileFrameUnit = MetafileFrameUnit(3i32);
pub const MetafileTypeEmf: MetafileType = MetafileType(3i32);
pub const MetafileTypeEmfPlusDual: MetafileType = MetafileType(5i32);
pub const MetafileTypeEmfPlusOnly: MetafileType = MetafileType(4i32);
pub const MetafileTypeInvalid: MetafileType = MetafileType(0i32);
pub const MetafileTypeWmf: MetafileType = MetafileType(1i32);
pub const MetafileTypeWmfPlaceable: MetafileType = MetafileType(2i32);
pub const NotImplemented: Status = Status(6i32);
pub const NotTrueTypeFont: Status = Status(16i32);
pub const ObjectBusy: Status = Status(4i32);
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
pub const PaletteFlagsGrayScale: PaletteFlags = PaletteFlags(2i32);
pub const PaletteFlagsHalftone: PaletteFlags = PaletteFlags(4i32);
pub const PaletteFlagsHasAlpha: PaletteFlags = PaletteFlags(1i32);
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
pub const PathPointTypeBezier: PathPointType = PathPointType(3i32);
pub const PathPointTypeBezier3: PathPointType = PathPointType(3i32);
pub const PathPointTypeCloseSubpath: PathPointType = PathPointType(128i32);
pub const PathPointTypeDashMode: PathPointType = PathPointType(16i32);
pub const PathPointTypeLine: PathPointType = PathPointType(1i32);
pub const PathPointTypePathMarker: PathPointType = PathPointType(32i32);
pub const PathPointTypePathTypeMask: PathPointType = PathPointType(7i32);
pub const PathPointTypeStart: PathPointType = PathPointType(0i32);
pub const PenAlignmentCenter: PenAlignment = PenAlignment(0i32);
pub const PenAlignmentInset: PenAlignment = PenAlignment(1i32);
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
pub const PixelOffsetModeDefault: PixelOffsetMode = PixelOffsetMode(0i32);
pub const PixelOffsetModeHalf: PixelOffsetMode = PixelOffsetMode(4i32);
pub const PixelOffsetModeHighQuality: PixelOffsetMode = PixelOffsetMode(2i32);
pub const PixelOffsetModeHighSpeed: PixelOffsetMode = PixelOffsetMode(1i32);
pub const PixelOffsetModeInvalid: PixelOffsetMode = PixelOffsetMode(-1i32);
pub const PixelOffsetModeNone: PixelOffsetMode = PixelOffsetMode(3i32);
pub const ProfileNotFound: Status = Status(21i32);
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
pub const QualityModeDefault: QualityMode = QualityMode(0i32);
pub const QualityModeHigh: QualityMode = QualityMode(2i32);
pub const QualityModeInvalid: QualityMode = QualityMode(-1i32);
pub const QualityModeLow: QualityMode = QualityMode(1i32);
pub const RED_SHIFT: u32 = 16u32;
pub const RedEyeCorrectionEffectGuid: windows_core::GUID = windows_core::GUID::from_u128(0x74d29d05_69a4_4266_9549_3cc52836b632);
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
pub const RotateNoneFlipNone: RotateFlipType = RotateFlipType(0i32);
pub const RotateNoneFlipX: RotateFlipType = RotateFlipType(4i32);
pub const RotateNoneFlipXY: RotateFlipType = RotateFlipType(2i32);
pub const RotateNoneFlipY: RotateFlipType = RotateFlipType(6i32);
pub const SharpenEffectGuid: windows_core::GUID = windows_core::GUID::from_u128(0x63cbf3ee_c526_402c_8f71_62c540bf5142);
pub const SmoothingModeAntiAlias: SmoothingMode = SmoothingMode(4i32);
pub const SmoothingModeAntiAlias8x4: SmoothingMode = SmoothingMode(4i32);
pub const SmoothingModeAntiAlias8x8: SmoothingMode = SmoothingMode(5i32);
pub const SmoothingModeDefault: SmoothingMode = SmoothingMode(0i32);
pub const SmoothingModeHighQuality: SmoothingMode = SmoothingMode(2i32);
pub const SmoothingModeHighSpeed: SmoothingMode = SmoothingMode(1i32);
pub const SmoothingModeInvalid: SmoothingMode = SmoothingMode(-1i32);
pub const SmoothingModeNone: SmoothingMode = SmoothingMode(3i32);
pub const StringAlignmentCenter: StringAlignment = StringAlignment(1i32);
pub const StringAlignmentFar: StringAlignment = StringAlignment(2i32);
pub const StringAlignmentNear: StringAlignment = StringAlignment(0i32);
pub const StringDigitSubstituteNational: StringDigitSubstitute = StringDigitSubstitute(2i32);
pub const StringDigitSubstituteNone: StringDigitSubstitute = StringDigitSubstitute(1i32);
pub const StringDigitSubstituteTraditional: StringDigitSubstitute = StringDigitSubstitute(3i32);
pub const StringDigitSubstituteUser: StringDigitSubstitute = StringDigitSubstitute(0i32);
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
pub const StringTrimmingCharacter: StringTrimming = StringTrimming(1i32);
pub const StringTrimmingEllipsisCharacter: StringTrimming = StringTrimming(3i32);
pub const StringTrimmingEllipsisPath: StringTrimming = StringTrimming(5i32);
pub const StringTrimmingEllipsisWord: StringTrimming = StringTrimming(4i32);
pub const StringTrimmingNone: StringTrimming = StringTrimming(0i32);
pub const StringTrimmingWord: StringTrimming = StringTrimming(2i32);
pub const TestControlForceBilinear: GpTestControlEnum = GpTestControlEnum(0i32);
pub const TestControlGetBuildNumber: GpTestControlEnum = GpTestControlEnum(2i32);
pub const TestControlNoICM: GpTestControlEnum = GpTestControlEnum(1i32);
pub const TextRenderingHintAntiAlias: TextRenderingHint = TextRenderingHint(4i32);
pub const TextRenderingHintAntiAliasGridFit: TextRenderingHint = TextRenderingHint(3i32);
pub const TextRenderingHintClearTypeGridFit: TextRenderingHint = TextRenderingHint(5i32);
pub const TextRenderingHintSingleBitPerPixel: TextRenderingHint = TextRenderingHint(2i32);
pub const TextRenderingHintSingleBitPerPixelGridFit: TextRenderingHint = TextRenderingHint(1i32);
pub const TextRenderingHintSystemDefault: TextRenderingHint = TextRenderingHint(0i32);
pub const TintEffectGuid: windows_core::GUID = windows_core::GUID::from_u128(0x1077af00_2848_4441_9489_44ad4c2d7a2c);
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
pub const WarpModeBilinear: WarpMode = WarpMode(1i32);
pub const WarpModePerspective: WarpMode = WarpMode(0i32);
pub const Win32Error: Status = Status(7i32);
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
pub const WrapModeClamp: WrapMode = WrapMode(4i32);
pub const WrapModeTile: WrapMode = WrapMode(0i32);
pub const WrapModeTileFlipX: WrapMode = WrapMode(1i32);
pub const WrapModeTileFlipXY: WrapMode = WrapMode(3i32);
pub const WrapModeTileFlipY: WrapMode = WrapMode(2i32);
pub const WrongState: Status = Status(8i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct BrushType(pub i32);
impl windows_core::TypeKind for BrushType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for BrushType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("BrushType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ColorAdjustType(pub i32);
impl windows_core::TypeKind for ColorAdjustType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ColorAdjustType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ColorAdjustType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ColorChannelFlags(pub i32);
impl windows_core::TypeKind for ColorChannelFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ColorChannelFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ColorChannelFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ColorMatrixFlags(pub i32);
impl windows_core::TypeKind for ColorMatrixFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ColorMatrixFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ColorMatrixFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ColorMode(pub i32);
impl windows_core::TypeKind for ColorMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ColorMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ColorMode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CombineMode(pub i32);
impl windows_core::TypeKind for CombineMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CombineMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CombineMode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CompositingMode(pub i32);
impl windows_core::TypeKind for CompositingMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CompositingMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CompositingMode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CompositingQuality(pub i32);
impl windows_core::TypeKind for CompositingQuality {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CompositingQuality {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CompositingQuality").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ConvertToEmfPlusFlags(pub i32);
impl windows_core::TypeKind for ConvertToEmfPlusFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ConvertToEmfPlusFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ConvertToEmfPlusFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CoordinateSpace(pub i32);
impl windows_core::TypeKind for CoordinateSpace {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CoordinateSpace {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CoordinateSpace").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CurveAdjustments(pub i32);
impl windows_core::TypeKind for CurveAdjustments {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CurveAdjustments {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CurveAdjustments").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CurveChannel(pub i32);
impl windows_core::TypeKind for CurveChannel {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CurveChannel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CurveChannel").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct CustomLineCapType(pub i32);
impl windows_core::TypeKind for CustomLineCapType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for CustomLineCapType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("CustomLineCapType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DashCap(pub i32);
impl windows_core::TypeKind for DashCap {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DashCap {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DashCap").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DashStyle(pub i32);
impl windows_core::TypeKind for DashStyle {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DashStyle {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DashStyle").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DebugEventLevel(pub i32);
impl windows_core::TypeKind for DebugEventLevel {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DebugEventLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DebugEventLevel").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DitherType(pub i32);
impl windows_core::TypeKind for DitherType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DitherType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DitherType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct DriverStringOptions(pub i32);
impl windows_core::TypeKind for DriverStringOptions {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for DriverStringOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("DriverStringOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EmfPlusRecordType(pub i32);
impl windows_core::TypeKind for EmfPlusRecordType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for EmfPlusRecordType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EmfPlusRecordType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EmfToWmfBitsFlags(pub i32);
impl windows_core::TypeKind for EmfToWmfBitsFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for EmfToWmfBitsFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EmfToWmfBitsFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EmfType(pub i32);
impl windows_core::TypeKind for EmfType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for EmfType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EmfType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EncoderParameterValueType(pub i32);
impl windows_core::TypeKind for EncoderParameterValueType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for EncoderParameterValueType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EncoderParameterValueType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EncoderValue(pub i32);
impl windows_core::TypeKind for EncoderValue {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for EncoderValue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EncoderValue").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FillMode(pub i32);
impl windows_core::TypeKind for FillMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FillMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FillMode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FlushIntention(pub i32);
impl windows_core::TypeKind for FlushIntention {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FlushIntention {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FlushIntention").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct FontStyle(pub i32);
impl windows_core::TypeKind for FontStyle {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for FontStyle {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("FontStyle").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GdiplusStartupParams(pub i32);
impl windows_core::TypeKind for GdiplusStartupParams {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GdiplusStartupParams {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GdiplusStartupParams").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GenericFontFamily(pub i32);
impl windows_core::TypeKind for GenericFontFamily {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GenericFontFamily {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GenericFontFamily").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct GpTestControlEnum(pub i32);
impl windows_core::TypeKind for GpTestControlEnum {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for GpTestControlEnum {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("GpTestControlEnum").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HatchStyle(pub i32);
impl windows_core::TypeKind for HatchStyle {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HatchStyle {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HatchStyle").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HistogramFormat(pub i32);
impl windows_core::TypeKind for HistogramFormat {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HistogramFormat {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HistogramFormat").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct HotkeyPrefix(pub i32);
impl windows_core::TypeKind for HotkeyPrefix {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for HotkeyPrefix {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("HotkeyPrefix").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ImageCodecFlags(pub i32);
impl windows_core::TypeKind for ImageCodecFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ImageCodecFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ImageCodecFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ImageFlags(pub i32);
impl windows_core::TypeKind for ImageFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ImageFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ImageFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ImageLockMode(pub i32);
impl windows_core::TypeKind for ImageLockMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ImageLockMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ImageLockMode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ImageType(pub i32);
impl windows_core::TypeKind for ImageType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ImageType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ImageType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct InterpolationMode(pub i32);
impl windows_core::TypeKind for InterpolationMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for InterpolationMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("InterpolationMode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ItemDataPosition(pub i32);
impl windows_core::TypeKind for ItemDataPosition {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ItemDataPosition {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ItemDataPosition").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct LineCap(pub i32);
impl windows_core::TypeKind for LineCap {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for LineCap {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LineCap").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct LineJoin(pub i32);
impl windows_core::TypeKind for LineJoin {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for LineJoin {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LineJoin").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct LinearGradientMode(pub i32);
impl windows_core::TypeKind for LinearGradientMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for LinearGradientMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("LinearGradientMode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MatrixOrder(pub i32);
impl windows_core::TypeKind for MatrixOrder {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MatrixOrder {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MatrixOrder").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MetafileFrameUnit(pub i32);
impl windows_core::TypeKind for MetafileFrameUnit {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MetafileFrameUnit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MetafileFrameUnit").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct MetafileType(pub i32);
impl windows_core::TypeKind for MetafileType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for MetafileType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MetafileType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct ObjectType(pub i32);
impl windows_core::TypeKind for ObjectType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for ObjectType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ObjectType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PaletteFlags(pub i32);
impl windows_core::TypeKind for PaletteFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PaletteFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PaletteFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PaletteType(pub i32);
impl windows_core::TypeKind for PaletteType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PaletteType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PaletteType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PathPointType(pub i32);
impl windows_core::TypeKind for PathPointType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PathPointType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PathPointType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PenAlignment(pub i32);
impl windows_core::TypeKind for PenAlignment {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PenAlignment {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PenAlignment").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PenType(pub i32);
impl windows_core::TypeKind for PenType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PenType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PenType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct PixelOffsetMode(pub i32);
impl windows_core::TypeKind for PixelOffsetMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for PixelOffsetMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("PixelOffsetMode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct QualityMode(pub i32);
impl windows_core::TypeKind for QualityMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for QualityMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("QualityMode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct RotateFlipType(pub i32);
impl windows_core::TypeKind for RotateFlipType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for RotateFlipType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("RotateFlipType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct SmoothingMode(pub i32);
impl windows_core::TypeKind for SmoothingMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for SmoothingMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("SmoothingMode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct Status(pub i32);
impl windows_core::TypeKind for Status {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for Status {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Status").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct StringAlignment(pub i32);
impl windows_core::TypeKind for StringAlignment {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for StringAlignment {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("StringAlignment").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct StringDigitSubstitute(pub i32);
impl windows_core::TypeKind for StringDigitSubstitute {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for StringDigitSubstitute {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("StringDigitSubstitute").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct StringFormatFlags(pub i32);
impl windows_core::TypeKind for StringFormatFlags {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for StringFormatFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("StringFormatFlags").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct StringTrimming(pub i32);
impl windows_core::TypeKind for StringTrimming {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for StringTrimming {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("StringTrimming").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct TextRenderingHint(pub i32);
impl windows_core::TypeKind for TextRenderingHint {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for TextRenderingHint {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("TextRenderingHint").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct Unit(pub i32);
impl windows_core::TypeKind for Unit {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for Unit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Unit").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WarpMode(pub i32);
impl windows_core::TypeKind for WarpMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WarpMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WarpMode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WrapMode(pub i32);
impl windows_core::TypeKind for WrapMode {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WrapMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WrapMode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Bitmap(pub isize);
impl Default for Bitmap {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for Bitmap {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BitmapData {
    pub Width: u32,
    pub Height: u32,
    pub Stride: i32,
    pub PixelFormat: i32,
    pub Scan0: *mut core::ffi::c_void,
    pub Reserved: usize,
}
impl windows_core::TypeKind for BitmapData {
    type TypeKind = windows_core::CopyType;
}
impl Default for BitmapData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Blur {
    pub Base: Effect,
}
impl windows_core::TypeKind for Blur {
    type TypeKind = windows_core::CopyType;
}
impl Default for Blur {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BlurParams {
    pub radius: f32,
    pub expandEdge: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for BlurParams {
    type TypeKind = windows_core::CopyType;
}
impl Default for BlurParams {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BrightnessContrast {
    pub Base: Effect,
}
impl windows_core::TypeKind for BrightnessContrast {
    type TypeKind = windows_core::CopyType;
}
impl Default for BrightnessContrast {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BrightnessContrastParams {
    pub brightnessLevel: i32,
    pub contrastLevel: i32,
}
impl windows_core::TypeKind for BrightnessContrastParams {
    type TypeKind = windows_core::CopyType;
}
impl Default for BrightnessContrastParams {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CGpEffect(pub isize);
impl Default for CGpEffect {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for CGpEffect {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CachedBitmap(pub isize);
impl Default for CachedBitmap {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for CachedBitmap {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CharacterRange {
    pub First: i32,
    pub Length: i32,
}
impl windows_core::TypeKind for CharacterRange {
    type TypeKind = windows_core::CopyType;
}
impl Default for CharacterRange {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for Color {
    type TypeKind = windows_core::CopyType;
}
impl Default for Color {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ColorBalance {
    pub Base: Effect,
}
impl windows_core::TypeKind for ColorBalance {
    type TypeKind = windows_core::CopyType;
}
impl Default for ColorBalance {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ColorBalanceParams {
    pub cyanRed: i32,
    pub magentaGreen: i32,
    pub yellowBlue: i32,
}
impl windows_core::TypeKind for ColorBalanceParams {
    type TypeKind = windows_core::CopyType;
}
impl Default for ColorBalanceParams {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ColorCurve {
    pub Base: Effect,
}
impl windows_core::TypeKind for ColorCurve {
    type TypeKind = windows_core::CopyType;
}
impl Default for ColorCurve {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ColorCurveParams {
    pub adjustment: CurveAdjustments,
    pub channel: CurveChannel,
    pub adjustValue: i32,
}
impl windows_core::TypeKind for ColorCurveParams {
    type TypeKind = windows_core::CopyType;
}
impl Default for ColorCurveParams {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ColorLUT {
    pub Base: Effect,
}
impl windows_core::TypeKind for ColorLUT {
    type TypeKind = windows_core::CopyType;
}
impl Default for ColorLUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ColorLUTParams {
    pub lutB: [u8; 256],
    pub lutG: [u8; 256],
    pub lutR: [u8; 256],
    pub lutA: [u8; 256],
}
impl windows_core::TypeKind for ColorLUTParams {
    type TypeKind = windows_core::CopyType;
}
impl Default for ColorLUTParams {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ColorMap {
    pub oldColor: Color,
    pub newColor: Color,
}
impl windows_core::TypeKind for ColorMap {
    type TypeKind = windows_core::CopyType;
}
impl Default for ColorMap {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorMatrix {
    pub m: [f32; 25],
}
impl windows_core::TypeKind for ColorMatrix {
    type TypeKind = windows_core::CopyType;
}
impl Default for ColorMatrix {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ColorMatrixEffect {
    pub Base: Effect,
}
impl windows_core::TypeKind for ColorMatrixEffect {
    type TypeKind = windows_core::CopyType;
}
impl Default for ColorMatrixEffect {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ColorPalette {
    pub Flags: u32,
    pub Count: u32,
    pub Entries: [u32; 1],
}
impl windows_core::TypeKind for ColorPalette {
    type TypeKind = windows_core::CopyType;
}
impl Default for ColorPalette {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CustomLineCap(pub isize);
impl Default for CustomLineCap {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for CustomLineCap {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for ENHMETAHEADER3 {
    type TypeKind = windows_core::CopyType;
}
impl Default for ENHMETAHEADER3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Effect {
    pub lpVtbl: *mut *mut core::ffi::c_void,
    pub nativeEffect: *mut CGpEffect,
    pub auxDataSize: i32,
    pub auxData: *mut core::ffi::c_void,
    pub useAuxData: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for Effect {
    type TypeKind = windows_core::CopyType;
}
impl Default for Effect {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EncoderParameter {
    pub Guid: windows_core::GUID,
    pub NumberOfValues: u32,
    pub Type: u32,
    pub Value: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for EncoderParameter {
    type TypeKind = windows_core::CopyType;
}
impl Default for EncoderParameter {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EncoderParameters {
    pub Count: u32,
    pub Parameter: [EncoderParameter; 1],
}
impl windows_core::TypeKind for EncoderParameters {
    type TypeKind = windows_core::CopyType;
}
impl Default for EncoderParameters {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Font(pub isize);
impl Default for Font {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for Font {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FontCollection(pub isize);
impl Default for FontCollection {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for FontCollection {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FontFamily(pub isize);
impl Default for FontFamily {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for FontFamily {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GdiplusStartupInput {
    pub GdiplusVersion: u32,
    pub DebugEventCallback: isize,
    pub SuppressBackgroundThread: super::super::Foundation::BOOL,
    pub SuppressExternalCodecs: super::super::Foundation::BOOL,
}
impl windows_core::TypeKind for GdiplusStartupInput {
    type TypeKind = windows_core::CopyType;
}
impl Default for GdiplusStartupInput {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GdiplusStartupInputEx {
    pub Base: GdiplusStartupInput,
    pub StartupParameters: i32,
}
impl windows_core::TypeKind for GdiplusStartupInputEx {
    type TypeKind = windows_core::CopyType;
}
impl Default for GdiplusStartupInputEx {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GdiplusStartupOutput {
    pub NotificationHook: isize,
    pub NotificationUnhook: isize,
}
impl windows_core::TypeKind for GdiplusStartupOutput {
    type TypeKind = windows_core::CopyType;
}
impl Default for GdiplusStartupOutput {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GpAdjustableArrowCap(pub isize);
impl Default for GpAdjustableArrowCap {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GpAdjustableArrowCap {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GpBitmap(pub isize);
impl Default for GpBitmap {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GpBitmap {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GpBrush(pub isize);
impl Default for GpBrush {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GpBrush {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GpCachedBitmap(pub isize);
impl Default for GpCachedBitmap {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GpCachedBitmap {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GpCustomLineCap(pub isize);
impl Default for GpCustomLineCap {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GpCustomLineCap {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GpFont(pub isize);
impl Default for GpFont {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GpFont {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GpFontCollection(pub isize);
impl Default for GpFontCollection {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GpFontCollection {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GpFontFamily(pub isize);
impl Default for GpFontFamily {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GpFontFamily {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GpGraphics(pub isize);
impl Default for GpGraphics {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GpGraphics {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GpHatch(pub isize);
impl Default for GpHatch {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GpHatch {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GpImage(pub isize);
impl Default for GpImage {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GpImage {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GpImageAttributes(pub isize);
impl Default for GpImageAttributes {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GpImageAttributes {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GpInstalledFontCollection(pub isize);
impl Default for GpInstalledFontCollection {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GpInstalledFontCollection {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GpLineGradient(pub isize);
impl Default for GpLineGradient {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GpLineGradient {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GpMetafile(pub isize);
impl Default for GpMetafile {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GpMetafile {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GpPath(pub isize);
impl Default for GpPath {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GpPath {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GpPathGradient(pub isize);
impl Default for GpPathGradient {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GpPathGradient {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GpPathIterator(pub isize);
impl Default for GpPathIterator {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GpPathIterator {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GpPen(pub isize);
impl Default for GpPen {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GpPen {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GpPrivateFontCollection(pub isize);
impl Default for GpPrivateFontCollection {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GpPrivateFontCollection {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GpRegion(pub isize);
impl Default for GpRegion {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GpRegion {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GpSolidFill(pub isize);
impl Default for GpSolidFill {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GpSolidFill {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GpStringFormat(pub isize);
impl Default for GpStringFormat {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GpStringFormat {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GpTexture(pub isize);
impl Default for GpTexture {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GpTexture {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HueSaturationLightness {
    pub Base: Effect,
}
impl windows_core::TypeKind for HueSaturationLightness {
    type TypeKind = windows_core::CopyType;
}
impl Default for HueSaturationLightness {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HueSaturationLightnessParams {
    pub hueLevel: i32,
    pub saturationLevel: i32,
    pub lightnessLevel: i32,
}
impl windows_core::TypeKind for HueSaturationLightnessParams {
    type TypeKind = windows_core::CopyType;
}
impl Default for HueSaturationLightnessParams {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Image(pub isize);
impl Default for Image {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for Image {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
impl windows_core::TypeKind for ImageCodecInfo {
    type TypeKind = windows_core::CopyType;
}
impl Default for ImageCodecInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ImageItemData {
    pub Size: u32,
    pub Position: u32,
    pub Desc: *mut core::ffi::c_void,
    pub DescSize: u32,
    pub Data: *mut core::ffi::c_void,
    pub DataSize: u32,
    pub Cookie: u32,
}
impl windows_core::TypeKind for ImageItemData {
    type TypeKind = windows_core::CopyType;
}
impl Default for ImageItemData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InstalledFontCollection(pub isize);
impl Default for InstalledFontCollection {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for InstalledFontCollection {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Levels {
    pub Base: Effect,
}
impl windows_core::TypeKind for Levels {
    type TypeKind = windows_core::CopyType;
}
impl Default for Levels {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LevelsParams {
    pub highlight: i32,
    pub midtone: i32,
    pub shadow: i32,
}
impl windows_core::TypeKind for LevelsParams {
    type TypeKind = windows_core::CopyType;
}
impl Default for LevelsParams {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Matrix(pub isize);
impl Default for Matrix {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for Matrix {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Metafile(pub isize);
impl Default for Metafile {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for Metafile {
    type TypeKind = windows_core::CopyType;
}
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
impl windows_core::TypeKind for MetafileHeader {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for MetafileHeader_0 {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for MetafileHeader_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PWMFRect16 {
    pub Left: i16,
    pub Top: i16,
    pub Right: i16,
    pub Bottom: i16,
}
impl windows_core::TypeKind for PWMFRect16 {
    type TypeKind = windows_core::CopyType;
}
impl Default for PWMFRect16 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PathData(pub isize);
impl Default for PathData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PathData {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Point {
    pub X: i32,
    pub Y: i32,
}
impl windows_core::TypeKind for Point {
    type TypeKind = windows_core::CopyType;
}
impl Default for Point {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PointF {
    pub X: f32,
    pub Y: f32,
}
impl windows_core::TypeKind for PointF {
    type TypeKind = windows_core::CopyType;
}
impl Default for PointF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PrivateFontCollection(pub isize);
impl Default for PrivateFontCollection {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PrivateFontCollection {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PropertyItem {
    pub id: u32,
    pub length: u32,
    pub r#type: u16,
    pub value: *mut core::ffi::c_void,
}
impl windows_core::TypeKind for PropertyItem {
    type TypeKind = windows_core::CopyType;
}
impl Default for PropertyItem {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rect {
    pub X: i32,
    pub Y: i32,
    pub Width: i32,
    pub Height: i32,
}
impl windows_core::TypeKind for Rect {
    type TypeKind = windows_core::CopyType;
}
impl Default for Rect {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RectF {
    pub X: f32,
    pub Y: f32,
    pub Width: f32,
    pub Height: f32,
}
impl windows_core::TypeKind for RectF {
    type TypeKind = windows_core::CopyType;
}
impl Default for RectF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RedEyeCorrection {
    pub Base: Effect,
}
impl windows_core::TypeKind for RedEyeCorrection {
    type TypeKind = windows_core::CopyType;
}
impl Default for RedEyeCorrection {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RedEyeCorrectionParams {
    pub numberOfAreas: u32,
    pub areas: *mut super::super::Foundation::RECT,
}
impl windows_core::TypeKind for RedEyeCorrectionParams {
    type TypeKind = windows_core::CopyType;
}
impl Default for RedEyeCorrectionParams {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Region(pub isize);
impl Default for Region {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for Region {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Sharpen {
    pub Base: Effect,
}
impl windows_core::TypeKind for Sharpen {
    type TypeKind = windows_core::CopyType;
}
impl Default for Sharpen {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SharpenParams {
    pub radius: f32,
    pub amount: f32,
}
impl windows_core::TypeKind for SharpenParams {
    type TypeKind = windows_core::CopyType;
}
impl Default for SharpenParams {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Size {
    pub Width: i32,
    pub Height: i32,
}
impl windows_core::TypeKind for Size {
    type TypeKind = windows_core::CopyType;
}
impl Default for Size {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SizeF {
    pub Width: f32,
    pub Height: f32,
}
impl windows_core::TypeKind for SizeF {
    type TypeKind = windows_core::CopyType;
}
impl Default for SizeF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Tint {
    pub Base: Effect,
}
impl windows_core::TypeKind for Tint {
    type TypeKind = windows_core::CopyType;
}
impl Default for Tint {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TintParams {
    pub hue: i32,
    pub amount: i32,
}
impl windows_core::TypeKind for TintParams {
    type TypeKind = windows_core::CopyType;
}
impl Default for TintParams {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct WmfPlaceableFileHeader {
    pub Key: u32,
    pub Hmf: i16,
    pub BoundingBox: PWMFRect16,
    pub Inch: i16,
    pub Reserved: u32,
    pub Checksum: i16,
}
impl windows_core::TypeKind for WmfPlaceableFileHeader {
    type TypeKind = windows_core::CopyType;
}
impl Default for WmfPlaceableFileHeader {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DebugEventProc = Option<unsafe extern "system" fn(level: DebugEventLevel, message: windows_core::PCSTR)>;
pub type DrawImageAbort = Option<unsafe extern "system" fn() -> super::super::Foundation::BOOL>;
pub type EnumerateMetafileProc = Option<unsafe extern "system" fn(param0: EmfPlusRecordType, param1: u32, param2: u32, param3: *const u8, param4: *mut core::ffi::c_void) -> super::super::Foundation::BOOL>;
pub type GetThumbnailImageAbort = Option<unsafe extern "system" fn() -> super::super::Foundation::BOOL>;
pub type ImageAbort = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void) -> super::super::Foundation::BOOL>;
pub type NotificationHookProc = Option<unsafe extern "system" fn(token: *mut usize) -> Status>;
pub type NotificationUnhookProc = Option<unsafe extern "system" fn(token: usize)>;
#[cfg(feature = "implement")]
core::include!("impl.rs");
