#![allow(dead_code)]
#![deny(clippy::allow_attributes)]

use bytemuck::{
  checked::CheckedCastError, AnyBitPattern, CheckedBitPattern, Contiguous,
  NoUninit, Pod, TransparentWrapper, Zeroable,
};
use std::marker::{PhantomData, PhantomPinned};

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
struct Test {
  a: u16,
  b: u16,
}

#[derive(Pod, Zeroable)]
#[repr(C, packed)]
struct GenericPackedStruct<T: Pod> {
  a: u32,
  b: T,
  c: u32,
}

impl<T: Pod> Clone for GenericPackedStruct<T> {
  fn clone(&self) -> Self {
    *self
  }
}

impl<T: Pod> Copy for GenericPackedStruct<T> {}

#[derive(Pod, Zeroable)]
#[repr(C, packed(1))]
struct GenericPackedStructExplicitPackedAlignment<T: Pod> {
  a: u32,
  b: T,
  c: u32,
}

impl<T: Pod> Clone for GenericPackedStructExplicitPackedAlignment<T> {
  fn clone(&self) -> Self {
    *self
  }
}

impl<T: Pod> Copy for GenericPackedStructExplicitPackedAlignment<T> {}

#[derive(Zeroable)]
struct ZeroGeneric<T: bytemuck::Zeroable> {
  a: T,
}

#[derive(Zeroable)]
#[repr(u8)]
enum ZeroEnum {
  A = 0,
  B = 1,
  C = 2,
}

#[derive(Zeroable)]
#[repr(u8)]
enum BasicFieldfulZeroEnum {
  A(u8) = 0,
  B = 1,
  C(String) = 2,
}

#[derive(Zeroable)]
#[repr(C)]
enum ReprCFieldfulZeroEnum {
  A(u8),
  B(Box<[u8]>),
  C,
}

#[derive(Zeroable)]
#[repr(C, i32)]
enum ReprCIntFieldfulZeroEnum {
  B(String) = 1,
  A(u8, bool, char) = 0,
  C = 2,
}

#[derive(Zeroable)]
#[repr(i32)]
enum GenericFieldfulZeroEnum<T> {
  A(Box<T>) = 1,
  B(T, T) = 0,
}

#[derive(Zeroable)]
#[repr(i32)]
#[zeroable(bound = "")]
enum GenericCustomBoundFieldfulZeroEnum<T> {
  A(Option<Box<T>>),
  B(String),
}

#[derive(TransparentWrapper)]
#[repr(transparent)]
struct TransparentSingle {
  a: u16,
}

#[derive(TransparentWrapper)]
#[repr(transparent)]
#[transparent(u16)]
struct TransparentWithZeroSized<T> {
  a: u16,
  b: PhantomData<T>,
}

struct MyZst<T>(PhantomData<T>, [u8; 0], PhantomPinned);
unsafe impl<T> Zeroable for MyZst<T> {}

#[derive(TransparentWrapper)]
#[repr(transparent)]
#[transparent(u16)]
struct TransparentTupleWithCustomZeroSized<T>(u16, MyZst<T>);

#[repr(u8)]
#[derive(Clone, Copy, Contiguous)]
enum ContiguousWithValues {
  A = 0,
  B = 1,
  C = 2,
  D = 3,
  E = 4,
}

#[repr(i8)]
#[derive(Clone, Copy, Contiguous)]
enum ContiguousWithImplicitValues {
  A = -10,
  B,
  C,
  D,
  E,
}

#[derive(Copy, Clone, NoUninit)]
#[repr(C)]
struct NoUninitTest {
  a: u16,
  b: u16,
}

#[derive(Copy, Clone, AnyBitPattern)]
#[repr(C)]
union UnionTestAnyBitPattern {
  a: u8,
  b: u16,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, NoUninit, CheckedBitPattern, PartialEq, Eq)]
enum CheckedBitPatternEnumWithValues {
  A = 0,
  B = 1,
  C = 2,
  D = 3,
  E = 4,
}

#[repr(i8)]
#[derive(Clone, Copy, NoUninit, CheckedBitPattern)]
enum CheckedBitPatternEnumWithImplicitValues {
  A = -10,
  B,
  C,
  D,
  E,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, NoUninit, CheckedBitPattern, PartialEq, Eq)]
enum CheckedBitPatternEnumNonContiguous {
  A = 1,
  B = 8,
  C = 2,
  D = 3,
  E = 56,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, NoUninit, CheckedBitPattern, PartialEq, Eq)]
enum CheckedBitPatternEnumByteLit {
  A = b'A',
  B = b'B',
  C = b'C',
  D = b'D',
  E = b'E',
}

#[derive(Debug, Copy, Clone, NoUninit, CheckedBitPattern, PartialEq, Eq)]
#[repr(C)]
struct CheckedBitPatternStruct {
  a: u8,
  b: CheckedBitPatternEnumNonContiguous,
}

#[derive(Debug, Copy, Clone, NoUninit)]
#[repr(C)]
enum NoUninitEnum {
  A,
  B,
}

#[derive(Debug, Copy, Clone, NoUninit)]
#[repr(C)]
enum NoUninitEnumWithFields {
  A(u32, u32),
  B(u16, u16, u16, u16),
}

#[derive(Debug, Copy, Clone, NoUninit)]
#[repr(C, u16)]
enum NoUninitEnumWithFieldsAndCAndDiscriminant {
  A(u16, u16),
  B(u8, u8, u8, u8),
}

#[derive(Debug, Clone, Copy, NoUninit)]
#[repr(u16)]
enum NoUninitEnumWithFieldsAndDiscriminant {
  A(u16, u16),
  B(u8, u8, u8, u8),
}

#[derive(Debug, Copy, Clone, AnyBitPattern, PartialEq, Eq)]
#[repr(C)]
struct AnyBitPatternTest<A: AnyBitPattern, B: AnyBitPattern> {
  a: A,
  b: B,
}

#[derive(Clone, Copy, CheckedBitPattern)]
#[repr(C, align(8))]
struct CheckedBitPatternAlignedStruct {
  a: u16,
}

#[derive(Clone, Copy, CheckedBitPattern)]
#[repr(C, packed)]
struct CheckedBitPatternPackedStruct {
  a: u8,
  b: u16,
}

#[derive(Debug, Clone, Copy, CheckedBitPattern, PartialEq, Eq)]
#[repr(C)]
enum CheckedBitPatternCDefaultDiscriminantEnum {
  A,
  B,
}

#[derive(Debug, Clone, Copy, CheckedBitPattern, PartialEq, Eq)]
#[repr(C)]
enum CheckedBitPatternCDefaultDiscriminantEnumWithFields {
  A(u64),
  B { c: u64 },
}

#[derive(Debug, Clone, Copy, CheckedBitPattern, PartialEq, Eq)]
#[repr(C, u8)]
enum CheckedBitPatternCEnumWithFields {
  A(u32),
  B { c: u32 },
}

#[derive(Debug, Clone, Copy, CheckedBitPattern, PartialEq, Eq)]
#[repr(u8)]
enum CheckedBitPatternIntEnumWithFields {
  A(u8),
  B { c: u32 },
}

#[derive(Debug, Clone, Copy, CheckedBitPattern, PartialEq, Eq)]
#[repr(transparent)]
enum CheckedBitPatternTransparentEnumWithFields {
  A { b: u32 },
}

// size 24, align 8.
// first byte always the u8 discriminant, then 7 bytes of padding until the
// payload union since the align of the payload is the greatest of the align of
// all the variants, which is 8 (from
// CheckedBitPatternCDefaultDiscriminantEnumWithFields)
#[derive(Debug, Clone, Copy, CheckedBitPattern, PartialEq, Eq)]
#[repr(C, u8)]
enum CheckedBitPatternEnumNested {
  A(CheckedBitPatternCEnumWithFields),
  B(CheckedBitPatternCDefaultDiscriminantEnumWithFields),
}

/// ```compile_fail
/// use bytemuck::{Pod, Zeroable};
///
/// #[derive(Pod, Zeroable)]
/// #[repr(transparent)]
/// struct TransparentSingle<T>(T);
///
/// struct NotPod(u32);
///
/// let _: u32 = bytemuck::cast(TransparentSingle(NotPod(0u32)));
/// ```
#[derive(
  Debug, Copy, Clone, PartialEq, Eq, Pod, Zeroable, TransparentWrapper,
)]
#[repr(transparent)]
struct NewtypeWrapperTest<T>(T);

#[derive(Debug, Clone, PartialEq, Eq, TransparentWrapper)]
#[repr(transparent)]
struct AlgebraicNewtypeWrapperTest<T>(Vec<T>);

#[test]
fn algebraic_newtype_corect() {
  let x: Vec<u32> = vec![1, 2, 3, 4];
  let y: AlgebraicNewtypeWrapperTest<u32> =
    AlgebraicNewtypeWrapperTest::wrap(x.clone());
  assert_eq!(y.0, x);
}

#[derive(Debug, Clone, PartialEq, Eq, TransparentWrapper)]
#[repr(transparent)]
#[transparent(Vec<T>)]
struct AlgebraicNewtypeWrapperTestWithFields<T, U>(Vec<T>, PhantomData<U>);

#[test]
fn algebraic_newtype_fields_corect() {
  let x: Vec<u32> = vec![1, 2, 3, 4];
  let y: AlgebraicNewtypeWrapperTestWithFields<u32, f32> =
    AlgebraicNewtypeWrapperTestWithFields::wrap(x.clone());
  assert_eq!(y.0, x);
}

#[test]
fn fails_cast_contiguous() {
  let can_cast = CheckedBitPatternEnumWithValues::is_valid_bit_pattern(&5);
  assert!(!can_cast);
}

#[test]
fn passes_cast_contiguous() {
  let res =
    bytemuck::checked::from_bytes::<CheckedBitPatternEnumWithValues>(&[2u8]);
  assert_eq!(*res, CheckedBitPatternEnumWithValues::C);
}

#[test]
fn fails_cast_noncontiguous() {
  let can_cast = CheckedBitPatternEnumNonContiguous::is_valid_bit_pattern(&4);
  assert!(!can_cast);
}

#[test]
fn passes_cast_noncontiguous() {
  let res =
    bytemuck::checked::from_bytes::<CheckedBitPatternEnumNonContiguous>(&[
      56u8,
    ]);
  assert_eq!(*res, CheckedBitPatternEnumNonContiguous::E);
}

#[test]
fn fails_cast_bytelit() {
  let can_cast = CheckedBitPatternEnumByteLit::is_valid_bit_pattern(&b'a');
  assert!(!can_cast);
}

#[test]
fn passes_cast_bytelit() {
  let res =
    bytemuck::checked::cast_slice::<u8, CheckedBitPatternEnumByteLit>(b"CAB");
  assert_eq!(
    res,
    [
      CheckedBitPatternEnumByteLit::C,
      CheckedBitPatternEnumByteLit::A,
      CheckedBitPatternEnumByteLit::B
    ]
  );
}

#[test]
fn fails_cast_struct() {
  let pod = [0u8, 24u8];
  let res = bytemuck::checked::try_from_bytes::<CheckedBitPatternStruct>(&pod);
  assert!(res.is_err());
}

#[test]
fn passes_cast_struct() {
  let pod = [0u8, 8u8];
  let res = bytemuck::checked::from_bytes::<CheckedBitPatternStruct>(&pod);
  assert_eq!(
    *res,
    CheckedBitPatternStruct { a: 0, b: CheckedBitPatternEnumNonContiguous::B }
  );
}

#[test]
fn anybitpattern_implies_zeroable() {
  let test = AnyBitPatternTest::<isize, usize>::zeroed();
  assert_eq!(test, AnyBitPatternTest { a: 0isize, b: 0usize });
}

#[test]
fn checkedbitpattern_try_pod_read_unaligned() {
  let pod = [0u8];
  let res = bytemuck::checked::try_pod_read_unaligned::<
    CheckedBitPatternEnumWithValues,
  >(&pod);
  assert!(res.is_ok());

  let pod = [5u8];
  let res = bytemuck::checked::try_pod_read_unaligned::<
    CheckedBitPatternEnumWithValues,
  >(&pod);
  assert!(res.is_err());
}

#[test]
fn checkedbitpattern_aligned_struct() {
  let pod = [0u8; 8];
  bytemuck::checked::pod_read_unaligned::<CheckedBitPatternAlignedStruct>(&pod);
}

#[test]
fn checkedbitpattern_c_default_discriminant_enum_with_fields() {
  let pod = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xcc, 0x55, 0x55, 0x55,
    0x55, 0x55, 0x55, 0xcc,
  ];
  let value = bytemuck::checked::pod_read_unaligned::<
    CheckedBitPatternCDefaultDiscriminantEnumWithFields,
  >(&pod);
  assert_eq!(
    value,
    CheckedBitPatternCDefaultDiscriminantEnumWithFields::A(0xcc555555555555cc)
  );

  let pod = [
    0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xcc, 0x55, 0x55, 0x55,
    0x55, 0x55, 0x55, 0xcc,
  ];
  let value = bytemuck::checked::pod_read_unaligned::<
    CheckedBitPatternCDefaultDiscriminantEnumWithFields,
  >(&pod);
  assert_eq!(
    value,
    CheckedBitPatternCDefaultDiscriminantEnumWithFields::B {
      c: 0xcc555555555555cc
    }
  );
}

#[test]
fn checkedbitpattern_c_enum_with_fields() {
  let pod = [0x00, 0x00, 0x00, 0x00, 0xcc, 0x55, 0x55, 0xcc];
  let value = bytemuck::checked::pod_read_unaligned::<
    CheckedBitPatternCEnumWithFields,
  >(&pod);
  assert_eq!(value, CheckedBitPatternCEnumWithFields::A(0xcc5555cc));

  let pod = [0x01, 0x00, 0x00, 0x00, 0xcc, 0x55, 0x55, 0xcc];
  let value = bytemuck::checked::pod_read_unaligned::<
    CheckedBitPatternCEnumWithFields,
  >(&pod);
  assert_eq!(value, CheckedBitPatternCEnumWithFields::B { c: 0xcc5555cc });
}

#[test]
fn checkedbitpattern_int_enum_with_fields() {
  let pod = [0x00, 0x55, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
  let value = bytemuck::checked::pod_read_unaligned::<
    CheckedBitPatternIntEnumWithFields,
  >(&pod);
  assert_eq!(value, CheckedBitPatternIntEnumWithFields::A(0x55));

  let pod = [0x01, 0x00, 0x00, 0x00, 0xcc, 0x55, 0x55, 0xcc];
  let value = bytemuck::checked::pod_read_unaligned::<
    CheckedBitPatternIntEnumWithFields,
  >(&pod);
  assert_eq!(value, CheckedBitPatternIntEnumWithFields::B { c: 0xcc5555cc });
}

#[test]
fn checkedbitpattern_nested_enum_with_fields() {
  // total size 24 bytes. first byte always the u8 discriminant.

  #[repr(C, align(8))]
  struct Align8Bytes([u8; 24]);

  // first we'll check variantA, nested variant A
  let pod = Align8Bytes([
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, // byte 0 discriminant = 0 = variant A, bytes 1-7 irrelevant padding.
    0x00, 0x00, 0x00, 0x00, 0xcc, 0x55, 0x55,
    0xcc, // bytes 8-15 are the nested CheckedBitPatternCEnumWithFields,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // bytes 16-23 padding
  ]);
  let value =
    bytemuck::checked::from_bytes::<CheckedBitPatternEnumNested>(&pod.0);
  assert_eq!(
    value,
    &CheckedBitPatternEnumNested::A(CheckedBitPatternCEnumWithFields::A(
      0xcc5555cc
    ))
  );

  // next we'll check invalid first discriminant fails
  let pod = Align8Bytes([
    0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, // byte 0 discriminant = 2 = invalid, bytes 1-7 padding
    0x00, 0x00, 0x00, 0x00, 0xcc, 0x55, 0x55,
    0xcc, // bytes 8-15 are the nested CheckedBitPatternCEnumWithFields = A,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // bytes 16-23 padding
  ]);
  let result =
    bytemuck::checked::try_from_bytes::<CheckedBitPatternEnumNested>(&pod.0);
  assert_eq!(result, Err(CheckedCastError::InvalidBitPattern));

  // next we'll check variant B, nested variant B
  let pod = Align8Bytes([
    0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, // byte 0 discriminant = 1 = variant B, bytes 1-7 padding
    0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, /* bytes 8-15 is C int size discriminant of
           * CheckedBitPatternCDefaultDiscrimimantEnumWithFields, 1 (LE byte
           * order) = variant B */
    0xcc, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55,
    0xcc, // bytes 16-13 is the data contained in nested variant B
  ]);
  let value =
    bytemuck::checked::from_bytes::<CheckedBitPatternEnumNested>(&pod.0);
  assert_eq!(
    value,
    &CheckedBitPatternEnumNested::B(
      CheckedBitPatternCDefaultDiscriminantEnumWithFields::B {
        c: 0xcc555555555555cc
      }
    )
  );

  // finally we'll check variant B, nested invalid discriminant
  let pod = Align8Bytes([
    0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, // 1 discriminant = variant B, bytes 1-7 padding
    0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, /* bytes 8-15 is C int size discriminant of
           * CheckedBitPatternCDefaultDiscrimimantEnumWithFields, 0x08 is
           * invalid */
    0xcc, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55,
    0xcc, // bytes 16-13 is the data contained in nested variant B
  ]);
  let result =
    bytemuck::checked::try_from_bytes::<CheckedBitPatternEnumNested>(&pod.0);
  assert_eq!(result, Err(CheckedCastError::InvalidBitPattern));
}
#[test]
fn checkedbitpattern_transparent_enum_with_fields() {
  let pod = [0xcc, 0x55, 0x55, 0xcc];
  let value = bytemuck::checked::pod_read_unaligned::<
    CheckedBitPatternTransparentEnumWithFields,
  >(&pod);
  assert_eq!(
    value,
    CheckedBitPatternTransparentEnumWithFields::A { b: 0xcc5555cc }
  );
}

#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C, align(16))]
struct Issue127 {}

use bytemuck as reexport_name;
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, bytemuck::ByteEq)]
#[bytemuck(crate = "reexport_name")]
#[repr(C)]
struct Issue93 {}
