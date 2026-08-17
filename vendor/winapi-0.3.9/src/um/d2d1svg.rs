// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
//! Mappings for the contents of d2d1svg.h
use ctypes::c_void;
use shared::basetsd::UINT32;
use shared::guiddef::REFIID;
use shared::minwindef::{BOOL, FLOAT};
use shared::ntdef::{PCWSTR, PWSTR, WCHAR};
use shared::winerror::HRESULT;
use um::d2d1::{
    D2D1_CAP_STYLE_FLAT, D2D1_CAP_STYLE_ROUND, D2D1_CAP_STYLE_SQUARE, D2D1_COLOR_F, D2D1_FILL_MODE,
    D2D1_LINE_JOIN_BEVEL, D2D1_LINE_JOIN_MITER_OR_BEVEL, D2D1_LINE_JOIN_ROUND, D2D1_POINT_2F,
    D2D1_SIZE_F, ID2D1Resource, ID2D1ResourceVtbl
};
use um::d2d1_1::ID2D1PathGeometry1;
use um::objidlbase::IStream;
ENUM!{enum D2D1_SVG_PAINT_TYPE {
    D2D1_SVG_PAINT_TYPE_NONE = 0,
    D2D1_SVG_PAINT_TYPE_COLOR = 1,
    D2D1_SVG_PAINT_TYPE_CURRENT_COLOR = 2,
    D2D1_SVG_PAINT_TYPE_URI = 3,
    D2D1_SVG_PAINT_TYPE_URI_NONE = 4,
    D2D1_SVG_PAINT_TYPE_URI_COLOR = 5,
    D2D1_SVG_PAINT_TYPE_URI_CURRENT_COLOR = 6,
}}
ENUM!{enum D2D1_SVG_LENGTH_UNITS {
    D2D1_SVG_LENGTH_UNITS_NUMBER = 0,
    D2D1_SVG_LENGTH_UNITS_PERCENTAGE = 1,
}}
ENUM!{enum D2D1_SVG_DISPLAY {
    D2D1_SVG_DISPLAY_INLINE = 0,
    D2D1_SVG_DISPLAY_NONE = 1,
}}
ENUM!{enum D2D1_SVG_VISIBILITY {
    D2D1_SVG_VISIBILITY_VISIBLE = 0,
    D2D1_SVG_VISIBILITY_HIDDEN = 1,
}}
ENUM!{enum D2D1_SVG_OVERFLOW {
    D2D1_SVG_OVERFLOW_VISIBLE = 0,
    D2D1_SVG_OVERFLOW_HIDDEN = 1,
}}
ENUM!{enum D2D1_SVG_LINE_CAP {
    D2D1_SVG_LINE_CAP_BUTT = D2D1_CAP_STYLE_FLAT,
    D2D1_SVG_LINE_CAP_SQUARE = D2D1_CAP_STYLE_SQUARE,
    D2D1_SVG_LINE_CAP_ROUND = D2D1_CAP_STYLE_ROUND,
}}
ENUM!{enum D2D1_SVG_LINE_JOIN {
    D2D1_SVG_LINE_JOIN_BEVEL = D2D1_LINE_JOIN_BEVEL,
    D2D1_SVG_LINE_JOIN_MITER = D2D1_LINE_JOIN_MITER_OR_BEVEL,
    D2D1_SVG_LINE_JOIN_ROUND = D2D1_LINE_JOIN_ROUND,
}}
ENUM!{enum D2D1_SVG_ASPECT_ALIGN {
    D2D1_SVG_ASPECT_ALIGN_NONE = 0,
    D2D1_SVG_ASPECT_ALIGN_X_MIN_Y_MIN = 1,
    D2D1_SVG_ASPECT_ALIGN_X_MID_Y_MIN = 2,
    D2D1_SVG_ASPECT_ALIGN_X_MAX_Y_MIN = 3,
    D2D1_SVG_ASPECT_ALIGN_X_MIN_Y_MID = 4,
    D2D1_SVG_ASPECT_ALIGN_X_MID_Y_MID = 5,
    D2D1_SVG_ASPECT_ALIGN_X_MAX_Y_MID = 6,
    D2D1_SVG_ASPECT_ALIGN_X_MIN_Y_MAX = 7,
    D2D1_SVG_ASPECT_ALIGN_X_MID_Y_MAX = 8,
    D2D1_SVG_ASPECT_ALIGN_X_MAX_Y_MAX = 9,
}}
ENUM!{enum D2D1_SVG_ASPECT_SCALING {
    D2D1_SVG_ASPECT_SCALING_MEET = 0,
    D2D1_SVG_ASPECT_SCALING_SLICE = 1,
}}
ENUM!{enum D2D1_SVG_PATH_COMMAND {
    D2D1_SVG_PATH_COMMAND_CLOSE_PATH = 0,
    D2D1_SVG_PATH_COMMAND_MOVE_ABSOLUTE = 1,
    D2D1_SVG_PATH_COMMAND_MOVE_RELATIVE = 2,
    D2D1_SVG_PATH_COMMAND_LINE_ABSOLUTE = 3,
    D2D1_SVG_PATH_COMMAND_LINE_RELATIVE = 4,
    D2D1_SVG_PATH_COMMAND_CUBIC_ABSOLUTE = 5,
    D2D1_SVG_PATH_COMMAND_CUBIC_RELATIVE = 6,
    D2D1_SVG_PATH_COMMAND_QUADRADIC_ABSOLUTE = 7,
    D2D1_SVG_PATH_COMMAND_QUADRADIC_RELATIVE = 8,
    D2D1_SVG_PATH_COMMAND_ARC_ABSOLUTE = 9,
    D2D1_SVG_PATH_COMMAND_ARC_RELATIVE = 10,
    D2D1_SVG_PATH_COMMAND_HORIZONTAL_ABSOLUTE = 11,
    D2D1_SVG_PATH_COMMAND_HORIZONTAL_RELATIVE = 12,
    D2D1_SVG_PATH_COMMAND_VERTICAL_ABSOLUTE = 13,
    D2D1_SVG_PATH_COMMAND_VERTICAL_RELATIVE = 14,
    D2D1_SVG_PATH_COMMAND_CUBIC_SMOOTH_ABSOLUTE = 15,
    D2D1_SVG_PATH_COMMAND_CUBIC_SMOOTH_RELATIVE = 16,
    D2D1_SVG_PATH_COMMAND_QUADRADIC_SMOOTH_ABSOLUTE = 17,
    D2D1_SVG_PATH_COMMAND_QUADRADIC_SMOOTH_RELATIVE = 18,
}}
ENUM!{enum D2D1_SVG_UNIT_TYPE {
    D2D1_SVG_UNIT_TYPE_USER_SPACE_ON_USE = 0,
    D2D1_SVG_UNIT_TYPE_OBJECT_BOUNDING_BOX = 1,
}}
ENUM!{enum D2D1_SVG_ATTRIBUTE_STRING_TYPE {
    D2D1_SVG_ATTRIBUTE_STRING_TYPE_SVG = 0,
    D2D1_SVG_ATTRIBUTE_STRING_TYPE_ID = 1,
}}
ENUM!{enum D2D1_SVG_ATTRIBUTE_POD_TYPE {
    D2D1_SVG_ATTRIBUTE_POD_TYPE_FLOAT = 0,
    D2D1_SVG_ATTRIBUTE_POD_TYPE_COLOR = 1,
    D2D1_SVG_ATTRIBUTE_POD_TYPE_FILL_MODE = 2,
    D2D1_SVG_ATTRIBUTE_POD_TYPE_DISPLAY = 3,
    D2D1_SVG_ATTRIBUTE_POD_TYPE_OVERFLOW = 4,
    D2D1_SVG_ATTRIBUTE_POD_TYPE_LINE_CAP = 5,
    D2D1_SVG_ATTRIBUTE_POD_TYPE_LINE_JOIN = 6,
    D2D1_SVG_ATTRIBUTE_POD_TYPE_VISIBILITY = 7,
    D2D1_SVG_ATTRIBUTE_POD_TYPE_MATRIX = 8,
    D2D1_SVG_ATTRIBUTE_POD_TYPE_UNIT_TYPE = 9,
    D2D1_SVG_ATTRIBUTE_POD_TYPE_EXTEND_MODE = 10,
    D2D1_SVG_ATTRIBUTE_POD_TYPE_PRESERVE_ASPECT_RATIO = 11,
    D2D1_SVG_ATTRIBUTE_POD_TYPE_VIEWBOX = 12,
    D2D1_SVG_ATTRIBUTE_POD_TYPE_LENGTH = 13,
}}
STRUCT!{struct D2D1_SVG_LENGTH {
    value: FLOAT,
    units: D2D1_SVG_LENGTH_UNITS,
}}
STRUCT!{struct D2D1_SVG_PRESERVE_ASPECT_RATIO {
    defer: BOOL,
    align: D2D1_SVG_ASPECT_ALIGN,
    meetOrSlice: D2D1_SVG_ASPECT_SCALING,
}}
STRUCT!{struct D2D1_SVG_VIEWBOX {
    x: FLOAT,
    y: FLOAT,
    width: FLOAT,
    height: FLOAT,
}}
DEFINE_GUID!{IID_ID2D1SvgAttribute,
    0xc9cdb0dd, 0xf8c9, 0x4e70, 0xb7, 0xc2, 0x30, 0x1c, 0x80, 0x29, 0x2c, 0x5e}
DEFINE_GUID!{IID_ID2D1SvgPaint,
    0xd59bab0a, 0x68a2, 0x455b, 0xa5, 0xdc, 0x9e, 0xb2, 0x85, 0x4e, 0x24, 0x90}
DEFINE_GUID!{IID_ID2D1SvgStrokeDashArray,
    0xf1c0ca52, 0x92a3, 0x4f00, 0xb4, 0xce, 0xf3, 0x56, 0x91, 0xef, 0xd9, 0xd9}
DEFINE_GUID!{IID_ID2D1SvgPointCollection,
    0x9dbe4c0d, 0x3572, 0x4dd9, 0x98, 0x25, 0x55, 0x30, 0x81, 0x3b, 0xb7, 0x12}
DEFINE_GUID!{IID_ID2D1SvgPathData,
    0xc095e4f4, 0xbb98, 0x43d6, 0x97, 0x45, 0x4d, 0x1b, 0x84, 0xec, 0x98, 0x88}
DEFINE_GUID!{IID_ID2D1SvgElement,
    0xac7b67a6, 0x183e, 0x49c1, 0xa8, 0x23, 0x0e, 0xbe, 0x40, 0xb0, 0xdb, 0x29}
DEFINE_GUID!{IID_ID2D1SvgDocument,
    0x86b88e4d, 0xafa4, 0x4d7b, 0x88, 0xe4, 0x68, 0xa5, 0x1c, 0x4a, 0x0a, 0xec}
RIDL!{#[uuid(0xc9cdb0dd, 0xf8c9, 0x4e70, 0xb7, 0xc2, 0x30, 0x1c, 0x80, 0x29, 0x2c, 0x5e)]
interface ID2D1SvgAttribute(ID2D1SvgAttributeVtbl): ID2D1Resource(ID2D1ResourceVtbl) {
    fn GetElement(
        element: *mut *mut ID2D1SvgElement,
    ) -> (),
    fn Clone(
        attribute: *mut *mut ID2D1SvgAttribute,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xd59bab0a, 0x68a2, 0x455b, 0xa5, 0xdc, 0x9e, 0xb2, 0x85, 0x4e, 0x24, 0x90)]
interface ID2D1SvgPaint(ID2D1SvgPaintVtbl): ID2D1SvgAttribute(ID2D1SvgAttributeVtbl) {
    fn SetPaintType(
        paintType: D2D1_SVG_PAINT_TYPE,
    ) -> HRESULT,
    fn GetPaintType() -> D2D1_SVG_PAINT_TYPE,
    fn SetColor(
        color: D2D1_COLOR_F,
    ) -> HRESULT,
    fn GetColor(
        color: *mut D2D1_COLOR_F,
    ) -> (),
    fn SetId(
        id: PCWSTR,
    ) -> HRESULT,
    fn GetId(
        id: PWSTR,
        idCount: UINT32,
    ) -> HRESULT,
    fn GetIdLength() -> UINT32,
}}
RIDL!{#[uuid(0xf1c0ca52, 0x92a3, 0x4f00, 0xb4, 0xce, 0xf3, 0x56, 0x91, 0xef, 0xd9, 0xd9)]
interface ID2D1SvgStrokeDashArray(ID2D1SvgStrokeDashArrayVtbl):
    ID2D1SvgAttribute(ID2D1SvgAttributeVtbl) {
    fn RemoveDashesAtEnd(
        dashesCount: UINT32,
    ) -> HRESULT,
    fn UpdateDashes_1(
        dashes: *const D2D1_SVG_LENGTH,
        dashesCount: UINT32,
        startIndex: UINT32,
    ) -> HRESULT,
    fn UpdateDashes_2(
        dashes: *const FLOAT,
        dashesCount: UINT32,
        startIndex: UINT32,
    ) -> HRESULT,
    fn GetDashes_1(
        dashes: *mut D2D1_SVG_LENGTH,
        dashesCount: UINT32,
        startIndex: UINT32,
    ) -> HRESULT,
    fn GetDashes_2(
        dashes: *mut FLOAT,
        dashesCount: UINT32,
        startIndex: UINT32,
    ) -> HRESULT,
    fn GetDashesCount() -> UINT32,
}}
RIDL!{#[uuid(0x9dbe4c0d, 0x3572, 0x4dd9, 0x98, 0x25, 0x55, 0x30, 0x81, 0x3b, 0xb7, 0x12)]
interface ID2D1SvgPointCollection(ID2D1SvgPointCollectionVtbl):
    ID2D1SvgAttribute(ID2D1SvgAttributeVtbl) {
    fn RemovePointsAtEnd(
        pointsCount: UINT32,
    ) -> HRESULT,
    fn UpdatePoints(
        points: *const D2D1_POINT_2F,
        pointsCount: UINT32,
        startIndex: UINT32,
    ) -> HRESULT,
    fn GetPoints(
        points: *mut D2D1_POINT_2F,
        pointsCount: UINT32,
        startIndex: UINT32,
    ) -> HRESULT,
    fn GetPointsCount() -> UINT32,
}}
RIDL!{#[uuid(0xc095e4f4, 0xbb98, 0x43d6, 0x97, 0x45, 0x4d, 0x1b, 0x84, 0xec, 0x98, 0x88)]
interface ID2D1SvgPathData(ID2D1SvgPathDataVtbl): ID2D1SvgAttribute(ID2D1SvgAttributeVtbl) {
    fn RemoveSegmentDataAtEnd(
        dataCount: UINT32,
    ) -> HRESULT,
    fn UpdateSegmentData(
        data: *const FLOAT,
        dataCount: UINT32,
        startIndex: UINT32,
    ) -> HRESULT,
    fn GetSegmentData(
        data: *mut FLOAT,
        dataCount: UINT32,
        startIndex: UINT32,
    ) -> HRESULT,
    fn GetSegmentDataCount() -> UINT32,
    fn RemoveCommandsAtEnd(
        commandsCount: UINT32,
    ) -> HRESULT,
    fn UpdateCommands(
        commands: *const D2D1_SVG_PATH_COMMAND,
        commandsCount: UINT32,
        startIndex: UINT32,
    ) -> HRESULT,
    fn GetCommands(
        commands: *mut D2D1_SVG_PATH_COMMAND,
        commandsCount: UINT32,
        startIndex: UINT32,
    ) -> HRESULT,
    fn GetCommandsCount() -> UINT32,
    fn CreatePathGeometry(
        fillMode: D2D1_FILL_MODE,
        pathGeometry: *mut *mut ID2D1PathGeometry1,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0xac7b67a6, 0x183e, 0x49c1, 0xa8, 0x23, 0x0e, 0xbe, 0x40, 0xb0, 0xdb, 0x29)]
interface ID2D1SvgElement(ID2D1SvgElementVtbl): ID2D1Resource(ID2D1ResourceVtbl) {
    fn GetDocument(
        document: *mut *mut ID2D1SvgDocument,
    ) -> (),
    fn GetTagName(
        name: PWSTR,
        nameCount: UINT32,
    ) -> HRESULT,
    fn GetTagNameLength() -> UINT32,
    fn IsTextContent() -> BOOL,
    fn GetParent(
        parent: *mut *mut ID2D1SvgElement,
    ) -> (),
    fn HasChildren() -> BOOL,
    fn GetFirstChild(
        child: *mut *mut ID2D1SvgElement,
    ) -> (),
    fn GetLastChild(
        child: *mut *mut ID2D1SvgElement,
    ) -> (),
    fn GetPreviousChild(
        referenceChild: *mut ID2D1SvgElement,
        previousChild: *mut *mut ID2D1SvgElement,
    ) -> HRESULT,
    fn GetNextChild(
        referenceChild: *mut ID2D1SvgElement,
        nextChild: *mut *mut ID2D1SvgElement,
    ) -> HRESULT,
    fn InsertChildBefore(
        newChild: *mut ID2D1SvgElement,
        referenceChild: *mut ID2D1SvgElement,
    ) -> HRESULT,
    fn AppendChild(
        newChild: *mut ID2D1SvgElement,
    ) -> HRESULT,
    fn ReplaceChild(
        newChild: *mut ID2D1SvgElement,
        oldChild: *mut ID2D1SvgElement,
    ) -> HRESULT,
    fn RemoveChild(
        oldChild: *mut ID2D1SvgElement,
    ) -> HRESULT,
    fn IsAttributeSpecified(
        name: PCWSTR, inherited: *mut BOOL,
    ) -> BOOL,
    fn GetSpecifiedAttributeCount() -> UINT32,
    fn GetSpecifiedAttributeName(
        index: UINT32,
        name: PWSTR,
        nameCount: UINT32,
        inherited: *mut BOOL,
    ) -> HRESULT,
    fn GetSpecifiedAttributeNameLength(
        index: UINT32,
        nameLength: *mut UINT32,
        inherited: *mut BOOL,
    ) -> HRESULT,
    fn RemoveAttribute(
        name: PCWSTR,
    ) -> HRESULT,
    fn SetTextValue(
        name: *const WCHAR,
        nameCount: UINT32,
    ) -> HRESULT,
    fn GetTextValue(
        name: PWSTR,
        nameCount: UINT32,
    ) -> HRESULT,
    fn GetTextValueLength() -> UINT32,
    fn SetAttributeValue_1(
        name: PCWSTR,
        value: *mut ID2D1SvgAttribute,
    ) -> HRESULT,
    fn SetAttributeValue_2(
        name: PCWSTR,
        type_: D2D1_SVG_ATTRIBUTE_POD_TYPE,
        value: *const c_void,
        valueSizeInBytes: UINT32,
    ) -> HRESULT,
    fn SetAttributeValue_3(
        name: PCWSTR,
        type_: D2D1_SVG_ATTRIBUTE_STRING_TYPE,
        value: PCWSTR,
    ) -> HRESULT,
    fn GetAttributeValue_1(
        name: PCWSTR,
        riid: REFIID,
        value: *mut *mut c_void,
    ) -> HRESULT,
    fn GetAttributeValue_2(
        name: PCWSTR,
        type_: D2D1_SVG_ATTRIBUTE_POD_TYPE,
        value: *mut c_void,
        valueSizeInBytes: UINT32,
    ) -> HRESULT,
    fn GetAttributeValue_3(
        name: PCWSTR,
        type_: D2D1_SVG_ATTRIBUTE_STRING_TYPE,
        value: PWSTR,
        valueCount: UINT32,
    ) -> HRESULT,
    fn GetAttributeValueLength(
        name: PCWSTR,
        type_: D2D1_SVG_ATTRIBUTE_STRING_TYPE,
        valueLength: *mut UINT32,
    ) -> HRESULT,
}}
RIDL!{#[uuid(0x86b88e4d, 0xafa4, 0x4d7b, 0x88, 0xe4, 0x68, 0xa5, 0x1c, 0x4a, 0x0a, 0xec)]
interface ID2D1SvgDocument(ID2D1SvgDocumentVtbl): ID2D1Resource(ID2D1ResourceVtbl) {
    fn SetViewportSize(
        viewportSize: D2D1_SIZE_F,
    ) -> HRESULT,
    fn GetViewportSize() -> D2D1_SIZE_F,
    fn SetRoot(
        root: *mut ID2D1SvgElement,
    ) -> HRESULT,
    fn GetRoot(
        root: *mut *mut ID2D1SvgElement,
    ) -> (),
    fn FindElementById(
        id: PCWSTR,
        svgElement: *mut *mut ID2D1SvgElement,
    ) -> HRESULT,
    fn Serialize(
        outputXmlStream: *mut IStream,
        subtree: *mut ID2D1SvgElement,
    ) -> HRESULT,
    fn Deserialize(
        inputXmlStream: *mut IStream,
        subtree: *mut *mut ID2D1SvgElement,
    ) -> HRESULT,
    fn CreatePaint(
        paintType: D2D1_SVG_PAINT_TYPE,
        color: *const D2D1_COLOR_F,
        id: PCWSTR,
        paint: *mut *mut ID2D1SvgPaint,
    ) -> HRESULT,
    fn CreateStrokeDashArray(
        dashes: *const D2D1_SVG_LENGTH,
        dashesCount: UINT32,
        strokeDashArray: *mut *mut ID2D1SvgStrokeDashArray,
    ) -> HRESULT,
    fn CreatePointCollection(
        points: *const D2D1_POINT_2F,
        pountsCount: UINT32,
        pointCollection: *mut ID2D1SvgPointCollection,
    ) -> HRESULT,
    fn CreatePathData(
        segmentData: *const FLOAT,
        segmentDataCount: UINT32,
        commands: *const D2D1_SVG_PATH_COMMAND,
        commandsCount: UINT32,
        pathData: *mut *mut ID2D1SvgPathData,
    ) -> HRESULT,
}}
