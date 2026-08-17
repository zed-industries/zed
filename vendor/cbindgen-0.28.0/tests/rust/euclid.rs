struct UnknownUnit;
struct LayoutUnit;

#[repr(C)]
struct TypedLength<T, Unit>(T, PhantomData<Unit>);

#[repr(C)]
struct TypedSideOffsets2D<T, U> {
    top: T,
    right: T,
    bottom: T,
    left: T,
    _phantom: PhantomData<U>,
}

#[repr(C)]
struct TypedSize2D<T, U> {
    width: T,
    height: T,
    _phantom: PhantomData<U>,
}

#[repr(C)]
struct TypedPoint2D<T, U> {
    x: T,
    y: T,
    _phantom: PhantomData<U>,
}

#[repr(C)]
struct TypedRect<T, U> {
    origin: TypedPoint2D<T, U>,
    size: TypedSize2D<T, U>,
    _phantom: PhantomData<U>,
}

#[repr(C)]
struct TypedTransform2D<T, Src, Dst> {
    m11: T, m12: T,
    m21: T, m22: T,
    m31: T, m32: T,
    _phantom: PhantomData<U>,
}

type Length<T> = TypedLength<T, UnknownUnit>;
type SideOffsets2D<T> = TypedSideOffsets2D<T, UnknownUnit>;
type Size2D<T> = TypedSize2D<T, UnknownUnit>;
type Point2D<T> = TypedPoint2D<T, UnknownUnit>;
type Rect<T> = TypedRect<T, UnknownUnit>;

type LayoutLength = TypedLength<f32, LayoutUnit>;
type LayoutSideOffsets2D = TypedSideOffsets2D<f32, LayoutUnit>;
type LayoutSize2D = TypedSize2D<f32, LayoutUnit>;
type LayoutPoint2D = TypedPoint2D<f32, LayoutUnit>;
type LayoutRect = TypedRect<f32, LayoutUnit>;

#[no_mangle]
pub extern "C" fn root(
    length_a: TypedLength<f32, UnknownUnit>,
    length_b: TypedLength<f32, LayoutUnit>,
    length_c: Length<f32>,
    length_d: LayoutLength,
    side_offsets_a: TypedSideOffsets2D<f32, UnknownUnit>,
    side_offsets_b: TypedSideOffsets2D<f32, LayoutUnit>,
    side_offsets_c: SideOffsets2D<f32>,
    side_offsets_d: LayoutSideOffsets2D,
    size_a: TypedSize2D<f32, UnknownUnit>,
    size_b: TypedSize2D<f32, LayoutUnit>,
    size_c: Size2D<f32>,
    size_d: LayoutSize2D,
    point_a: TypedPoint2D<f32, UnknownUnit>,
    point_b: TypedPoint2D<f32, LayoutUnit>,
    point_c: Point2D<f32>,
    point_d: LayoutPoint2D,
    rect_a: TypedRect<f32, UnknownUnit>,
    rect_b: TypedRect<f32, LayoutUnit>,
    rect_c: Rect<f32>,
    rect_d: LayoutRect,
    transform_a: TypedTransform2D<f32, UnknownUnit, LayoutUnit>,
    transform_b: TypedTransform2D<f32, LayoutUnit, UnknownUnit>
) { }
