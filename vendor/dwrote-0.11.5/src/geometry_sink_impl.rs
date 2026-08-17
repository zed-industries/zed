#![allow(non_snake_case, non_upper_case_globals)]

use std::mem;
use std::slice;
use std::sync::atomic::AtomicUsize;
use winapi::shared::guiddef::REFIID;
use winapi::shared::minwindef::{UINT, ULONG};
use winapi::shared::winerror::S_OK;
use winapi::um::d2d1::{ID2D1SimplifiedGeometrySink, ID2D1SimplifiedGeometrySinkVtbl};
use winapi::um::d2d1::{D2D1_BEZIER_SEGMENT, D2D1_FIGURE_BEGIN, D2D1_FIGURE_END};
use winapi::um::d2d1::{D2D1_FIGURE_END_CLOSED, D2D1_FILL_MODE, D2D1_PATH_SEGMENT, D2D1_POINT_2F};
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};
use winapi::um::winnt::HRESULT;

use crate::com_helpers::Com;
use crate::outline_builder::OutlineBuilder;

static GEOMETRY_SINK_VTBL: ID2D1SimplifiedGeometrySinkVtbl = ID2D1SimplifiedGeometrySinkVtbl {
    parent: implement_iunknown!(static ID2D1SimplifiedGeometrySink, GeometrySinkImpl),
    BeginFigure: GeometrySinkImpl_BeginFigure,
    EndFigure: GeometrySinkImpl_EndFigure,
    AddLines: GeometrySinkImpl_AddLines,
    AddBeziers: GeometrySinkImpl_AddBeziers,
    Close: GeometrySinkImpl_Close,
    SetFillMode: GeometrySinkImpl_SetFillMode,
    SetSegmentFlags: GeometrySinkImpl_SetSegmentFlags,
};

#[repr(C)]
pub struct GeometrySinkImpl {
    // NB: This must be the first field.
    _refcount: AtomicUsize,
    outline_builder: Box<dyn OutlineBuilder>,
}

impl Com<ID2D1SimplifiedGeometrySink> for GeometrySinkImpl {
    type Vtbl = ID2D1SimplifiedGeometrySinkVtbl;
    #[inline]
    fn vtbl() -> &'static ID2D1SimplifiedGeometrySinkVtbl {
        &GEOMETRY_SINK_VTBL
    }
}

impl Com<IUnknown> for GeometrySinkImpl {
    type Vtbl = IUnknownVtbl;
    #[inline]
    fn vtbl() -> &'static IUnknownVtbl {
        &GEOMETRY_SINK_VTBL.parent
    }
}

impl GeometrySinkImpl {
    pub fn new(outline_builder: Box<dyn OutlineBuilder>) -> GeometrySinkImpl {
        GeometrySinkImpl {
            _refcount: AtomicUsize::new(1),
            outline_builder,
        }
    }
}

unsafe extern "system" fn GeometrySinkImpl_BeginFigure(
    this: *mut ID2D1SimplifiedGeometrySink,
    start_point: D2D1_POINT_2F,
    _: D2D1_FIGURE_BEGIN,
) {
    let this = GeometrySinkImpl::from_interface(this);
    this
        .outline_builder
        .move_to(start_point.x, start_point.y)
}

unsafe extern "system" fn GeometrySinkImpl_EndFigure(
    this: *mut ID2D1SimplifiedGeometrySink,
    figure_end: D2D1_FIGURE_END,
) {
    let this = GeometrySinkImpl::from_interface(this);
    if figure_end == D2D1_FIGURE_END_CLOSED {
        this.outline_builder.close()
    }
}

unsafe extern "system" fn GeometrySinkImpl_AddLines(
    this: *mut ID2D1SimplifiedGeometrySink,
    points: *const D2D1_POINT_2F,
    points_count: UINT,
) {
    let this = GeometrySinkImpl::from_interface(this);
    let points = slice::from_raw_parts(points, points_count as usize);
    for point in points {
        this.outline_builder.line_to(point.x, point.y)
    }
}

unsafe extern "system" fn GeometrySinkImpl_AddBeziers(
    this: *mut ID2D1SimplifiedGeometrySink,
    beziers: *const D2D1_BEZIER_SEGMENT,
    beziers_count: UINT,
) {
    let this = GeometrySinkImpl::from_interface(this);
    let beziers = slice::from_raw_parts(beziers, beziers_count as usize);
    for bezier in beziers {
        this.outline_builder.curve_to(
            bezier.point1.x,
            bezier.point1.y,
            bezier.point2.x,
            bezier.point2.y,
            bezier.point3.x,
            bezier.point3.y,
        )
    }
}

unsafe extern "system" fn GeometrySinkImpl_Close(_: *mut ID2D1SimplifiedGeometrySink) -> HRESULT {
    S_OK
}

unsafe extern "system" fn GeometrySinkImpl_SetFillMode(
    _: *mut ID2D1SimplifiedGeometrySink,
    _: D2D1_FILL_MODE,
) {
}

unsafe extern "system" fn GeometrySinkImpl_SetSegmentFlags(
    _: *mut ID2D1SimplifiedGeometrySink,
    _: D2D1_PATH_SEGMENT,
) {
}
