/*!
Module responsible for calculating the offset and span for types.

There exists two types of layouts std140 and std430 (there's technically
two more layouts, shared and packed. Shared is not supported by spirv. Packed is
implementation dependent and for now it's just implemented as an alias to
std140).

The OpenGl spec (the layout rules are defined by the OpenGl spec in section
7.6.2.2 as opposed to the GLSL spec) uses the term basic machine units which are
equivalent to bytes.
*/

use alloc::vec::Vec;

use super::{
    ast::StructLayout,
    error::{Error, ErrorKind},
    Span,
};
use crate::{proc::Alignment, Handle, Scalar, Type, TypeInner, UniqueArena};

/// Struct with information needed for defining a struct member.
///
/// Returned by [`calculate_offset`].
#[derive(Debug)]
pub struct TypeAlignSpan {
    /// The handle to the type, this might be the same handle passed to
    /// [`calculate_offset`] or a new such a new array type with a different
    /// stride set.
    pub ty: Handle<Type>,
    /// The alignment required by the type.
    pub align: Alignment,
    /// The size of the type.
    pub span: u32,
}

/// Returns the type, alignment and span of a struct member according to a [`StructLayout`].
///
/// The functions returns a [`TypeAlignSpan`] which has a `ty` member this
/// should be used as the struct member type because for example arrays may have
/// to change the stride and as such need to have a different type.
pub fn calculate_offset(
    mut ty: Handle<Type>,
    meta: Span,
    layout: StructLayout,
    types: &mut UniqueArena<Type>,
    errors: &mut Vec<Error>,
) -> TypeAlignSpan {
    // When using the std430 storage layout, shader storage blocks will be laid out in buffer storage
    // identically to uniform and shader storage blocks using the std140 layout, except
    // that the base alignment and stride of arrays of scalars and vectors in rule 4 and of
    // structures in rule 9 are not rounded up a multiple of the base alignment of a vec4.

    let (align, span) = match types[ty].inner {
        // 1. If the member is a scalar consuming N basic machine units,
        // the base alignment is N.
        TypeInner::Scalar(Scalar { width, .. }) => (Alignment::from_width(width), width as u32),
        // 2. If the member is a two- or four-component vector with components
        // consuming N basic machine units, the base alignment is 2N or 4N, respectively.
        // 3. If the member is a three-component vector with components consuming N
        // basic machine units, the base alignment is 4N.
        TypeInner::Vector {
            size,
            scalar: Scalar { width, .. },
        } => (
            Alignment::from(size) * Alignment::from_width(width),
            size as u32 * width as u32,
        ),
        // 4. If the member is an array of scalars or vectors, the base alignment and array
        // stride are set to match the base alignment of a single array element, according
        // to rules (1), (2), and (3), and rounded up to the base alignment of a vec4.
        // TODO: Matrices array
        TypeInner::Array { base, size, .. } => {
            let info = calculate_offset(base, meta, layout, types, errors);

            let name = types[ty].name.clone();

            // See comment at the beginning of the function
            let (align, stride) = if StructLayout::Std430 == layout {
                (info.align, info.align.round_up(info.span))
            } else {
                let align = info.align.max(Alignment::MIN_UNIFORM);
                (align, align.round_up(info.span))
            };

            let span = match size {
                crate::ArraySize::Constant(size) => size.get() * stride,
                crate::ArraySize::Pending(_) => unreachable!(),
                crate::ArraySize::Dynamic => stride,
            };

            let ty_span = types.get_span(ty);
            ty = types.insert(
                Type {
                    name,
                    inner: TypeInner::Array {
                        base: info.ty,
                        size,
                        stride,
                    },
                },
                ty_span,
            );

            (align, span)
        }
        // 5. If the member is a column-major matrix with C columns and R rows, the
        // matrix is stored identically to an array of C column vectors with R
        // components each, according to rule (4)
        // TODO: Row major matrices
        TypeInner::Matrix {
            columns,
            rows,
            scalar,
        } => {
            let mut align = Alignment::from(rows) * Alignment::from_width(scalar.width);

            // See comment at the beginning of the function
            if StructLayout::Std430 != layout {
                align = align.max(Alignment::MIN_UNIFORM);
            }

            // See comment on the error kind
            if StructLayout::Std140 == layout {
                // Do the f16 test first, as it's more specific
                if scalar == Scalar::F16 {
                    errors.push(Error {
                        kind: ErrorKind::UnsupportedF16MatrixInStd140 {
                            columns: columns as u8,
                            rows: rows as u8,
                        },
                        meta,
                    });
                }
                if rows == crate::VectorSize::Bi {
                    errors.push(Error {
                        kind: ErrorKind::UnsupportedMatrixWithTwoRowsInStd140 {
                            columns: columns as u8,
                        },
                        meta,
                    });
                }
            }

            (align, align * columns as u32)
        }
        TypeInner::Struct { ref members, .. } => {
            let mut span = 0;
            let mut align = Alignment::ONE;
            let mut members = members.clone();
            let name = types[ty].name.clone();

            for member in members.iter_mut() {
                let info = calculate_offset(member.ty, meta, layout, types, errors);

                let member_alignment = info.align;
                span = member_alignment.round_up(span);
                align = member_alignment.max(align);

                member.ty = info.ty;
                member.offset = span;

                span += info.span;
            }

            span = align.round_up(span);

            let ty_span = types.get_span(ty);
            ty = types.insert(
                Type {
                    name,
                    inner: TypeInner::Struct { members, span },
                },
                ty_span,
            );

            (align, span)
        }
        _ => {
            errors.push(Error {
                kind: ErrorKind::SemanticError("Invalid struct member type".into()),
                meta,
            });
            (Alignment::ONE, 0)
        }
    };

    TypeAlignSpan { ty, align, span }
}
