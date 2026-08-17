use crate::base::dimension::{Const, Dim, DimName, Dyn};
use crate::base::matrix_view::{ViewStorage, ViewStorageMut};
use crate::base::{MatrixView, MatrixViewMut, Scalar};

use num_rational::Ratio;

/// # Creating matrix views from `&[T]`
impl<'a, T: Scalar, R: Dim, C: Dim, RStride: Dim, CStride: Dim>
    MatrixView<'a, T, R, C, RStride, CStride>
{
    /// Creates, without bounds checking, a matrix view from an array and with dimensions and strides specified by generic types instances.
    ///
    /// # Safety
    /// This method is unsafe because the input data array is not checked to contain enough elements.
    /// The generic types `R`, `C`, `RStride`, `CStride` can either be type-level integers or integers wrapped with `Dyn()`.
    #[inline]
    pub const unsafe fn from_slice_with_strides_generic_unchecked(
        data: &'a [T],
        start: usize,
        nrows: R,
        ncols: C,
        rstride: RStride,
        cstride: CStride,
    ) -> Self {
        unsafe {
            let data = ViewStorage::from_raw_parts(
                data.as_ptr().add(start),
                (nrows, ncols),
                (rstride, cstride),
            );
            Self::from_data(data)
        }
    }

    /// Creates a matrix view from an array and with dimensions and strides specified by generic types instances.
    ///
    /// Panics if the input data array dose not contain enough elements.
    /// The generic types `R`, `C`, `RStride`, `CStride` can either be type-level integers or integers wrapped with `Dyn()`.
    #[inline]
    pub fn from_slice_with_strides_generic(
        data: &'a [T],
        nrows: R,
        ncols: C,
        rstride: RStride,
        cstride: CStride,
    ) -> Self {
        // NOTE: The assertion implements the following formula, but without subtractions to avoid
        // underflow panics:
        //      len >= (ncols - 1) * cstride + (nrows - 1) * rstride + 1
        assert!(
            data.len() + cstride.value() + rstride.value()
                >= ncols.value() * cstride.value() + nrows.value() * rstride.value() + 1,
            "Matrix view: input data buffer too small."
        );

        unsafe {
            Self::from_slice_with_strides_generic_unchecked(data, 0, nrows, ncols, rstride, cstride)
        }
    }
}

impl<'a, T: Scalar, R: Dim, C: Dim> MatrixView<'a, T, R, C> {
    /// Creates, without bound-checking, a matrix view from an array and with dimensions specified by generic types instances.
    ///
    /// # Safety
    /// This method is unsafe because the input data array is not checked to contain enough elements.
    /// The generic types `R` and `C` can either be type-level integers or integers wrapped with `Dyn()`.
    #[inline]
    pub unsafe fn from_slice_generic_unchecked(
        data: &'a [T],
        start: usize,
        nrows: R,
        ncols: C,
    ) -> Self {
        unsafe {
            Self::from_slice_with_strides_generic_unchecked(
                data, start, nrows, ncols, Const::<1>, nrows,
            )
        }
    }

    /// Creates a matrix view from an array and with dimensions and strides specified by generic types instances.
    ///
    /// Panics if the input data array dose not contain enough elements.
    /// The generic types `R` and `C` can either be type-level integers or integers wrapped with `Dyn()`.
    #[inline]
    pub fn from_slice_generic(data: &'a [T], nrows: R, ncols: C) -> Self {
        Self::from_slice_with_strides_generic(data, nrows, ncols, Const::<1>, nrows)
    }
}

macro_rules! impl_constructors(
    ($($Dims: ty),*; $(=> $DimIdent: ident: $DimBound: ident),*; $($gargs: expr_2021),*; $($args: ident),*) => {
        impl<'a, T: Scalar, $($DimIdent: $DimBound),*> MatrixView<'a, T, $($Dims),*> {
            /// Creates a new matrix view from the given data array.
            ///
            /// Panics if `data` does not contain enough elements.
            #[inline]
            pub fn from_slice(data: &'a [T], $($args: usize),*) -> Self {
                Self::from_slice_generic(data, $($gargs),*)
            }

            /// Creates, without bound checking, a new matrix view from the given data array.
            /// # Safety
            /// `data[start..start+rstride * cstride]` must be within bounds.
            #[inline]
            pub unsafe fn from_slice_unchecked(data: &'a [T], start: usize, $($args: usize),*) -> Self { unsafe {
                Self::from_slice_generic_unchecked(data, start, $($gargs),*)
            }}
        }

        impl<'a, T: Scalar, $($DimIdent: $DimBound, )*> MatrixView<'a, T, $($Dims,)* Dyn, Dyn> {
            /// Creates a new matrix view with the specified strides from the given data array.
            ///
            /// Panics if `data` does not contain enough elements.
            #[inline]
            pub fn from_slice_with_strides(data: &'a [T], $($args: usize,)* rstride: usize, cstride: usize) -> Self {
                Self::from_slice_with_strides_generic(data, $($gargs,)* Dyn(rstride), Dyn(cstride))
            }

            /// Creates, without bound checking, a new matrix view with the specified strides from the given data array.
            ///
            /// # Safety
            ///
            /// `start`, `rstride`, and `cstride`, with the given matrix size will not index
            /// outside of `data`.
            #[inline]
            pub unsafe fn from_slice_with_strides_unchecked(data: &'a [T], start: usize, $($args: usize,)* rstride: usize, cstride: usize) -> Self { unsafe {
                Self::from_slice_with_strides_generic_unchecked(data, start, $($gargs,)* Dyn(rstride), Dyn(cstride))
            }}
        }
    }
);

// TODO: this is not very pretty. We could find a better call syntax.
impl_constructors!(R, C;                         // Arguments for Matrix<T, ..., S>
=> R: DimName, => C: DimName; // Type parameters for impl<T, ..., S>
R::name(), C::name();         // Arguments for `_generic` constructors.
); // Arguments for non-generic constructors.

impl_constructors!(R, Dyn;
                   => R: DimName;
                   R::name(), Dyn(ncols);
                   ncols);

impl_constructors!(Dyn, C;
                   => C: DimName;
                   Dyn(nrows), C::name();
                   nrows);

impl_constructors!(Dyn, Dyn;
                   ;
                   Dyn(nrows), Dyn(ncols);
                   nrows, ncols);

/// # Creating mutable matrix views from `&mut [T]`
impl<'a, T: Scalar, R: Dim, C: Dim, RStride: Dim, CStride: Dim>
    MatrixViewMut<'a, T, R, C, RStride, CStride>
{
    /// Creates, without bound-checking, a mutable matrix view from an array and with dimensions and strides specified by generic types instances.
    ///
    /// # Safety
    /// This method is unsafe because the input data array is not checked to contain enough elements.
    /// The generic types `R`, `C`, `RStride`, `CStride` can either be type-level integers or integers wrapped with `Dyn()`.
    #[inline]
    pub const unsafe fn from_slice_with_strides_generic_unchecked(
        data: &'a mut [T],
        start: usize,
        nrows: R,
        ncols: C,
        rstride: RStride,
        cstride: CStride,
    ) -> Self {
        unsafe {
            let data = ViewStorageMut::from_raw_parts(
                data.as_mut_ptr().add(start),
                (nrows, ncols),
                (rstride, cstride),
            );
            Self::from_data(data)
        }
    }

    /// Creates a mutable matrix view from an array and with dimensions and strides specified by generic types instances.
    ///
    /// Panics if the input data array dose not contain enough elements.
    /// The generic types `R`, `C`, `RStride`, `CStride` can either be type-level integers or integers wrapped with `Dyn()`.
    #[inline]
    pub fn from_slice_with_strides_generic(
        data: &'a mut [T],
        nrows: R,
        ncols: C,
        rstride: RStride,
        cstride: CStride,
    ) -> Self {
        // NOTE: The assertion implements the following formula, but without subtractions to avoid
        // underflow panics:
        //      len >= (ncols - 1) * cstride + (nrows - 1) * rstride + 1
        assert!(
            data.len() + cstride.value() + rstride.value()
                >= ncols.value() * cstride.value() + nrows.value() * rstride.value() + 1,
            "Matrix view: input data buffer too small."
        );

        assert!(
            {
                let nrows = nrows.value();
                let ncols = ncols.value();
                let rstride = rstride.value();
                let cstride = cstride.value();

                nrows * ncols <= 1
                    || match (rstride, cstride) {
                        (0, 0) => false,      // otherwise: matrix[(0, 0)] == index[(nrows - 1, ncols - 1)],
                        (0, _) => nrows <= 1, // otherwise: matrix[(0, 0)] == index[(nrows - 1, 0)],
                        (_, 0) => ncols <= 1, // otherwise: matrix[(0, 0)] == index[(0, ncols - 1)],
                        (_, _) => {
                            // otherwise: matrix[(0, numer)] == index[(denom, 0)]
                            let ratio = Ratio::new(rstride, cstride);
                            nrows <= *ratio.denom() || ncols <= *ratio.numer()
                        }
                    }
            },
            "Matrix view: dimensions and strides result in aliased indices."
        );

        unsafe {
            Self::from_slice_with_strides_generic_unchecked(data, 0, nrows, ncols, rstride, cstride)
        }
    }
}

impl<'a, T: Scalar, R: Dim, C: Dim> MatrixViewMut<'a, T, R, C> {
    /// Creates, without bound-checking, a mutable matrix view from an array and with dimensions specified by generic types instances.
    ///
    /// # Safety
    /// This method is unsafe because the input data array is not checked to contain enough elements.
    /// The generic types `R` and `C` can either be type-level integers or integers wrapped with `Dyn()`.
    #[inline]
    pub unsafe fn from_slice_generic_unchecked(
        data: &'a mut [T],
        start: usize,
        nrows: R,
        ncols: C,
    ) -> Self {
        unsafe {
            Self::from_slice_with_strides_generic_unchecked(
                data, start, nrows, ncols, Const::<1>, nrows,
            )
        }
    }

    /// Creates a mutable matrix view from an array and with dimensions and strides specified by generic types instances.
    ///
    /// Panics if the input data array dose not contain enough elements.
    /// The generic types `R` and `C` can either be type-level integers or integers wrapped with `Dyn()`.
    #[inline]
    pub fn from_slice_generic(data: &'a mut [T], nrows: R, ncols: C) -> Self {
        Self::from_slice_with_strides_generic(data, nrows, ncols, Const::<1>, nrows)
    }
}

macro_rules! impl_constructors_mut(
    ($($Dims: ty),*; $(=> $DimIdent: ident: $DimBound: ident),*; $($gargs: expr_2021),*; $($args: ident),*) => {
        impl<'a, T: Scalar, $($DimIdent: $DimBound),*> MatrixViewMut<'a, T, $($Dims),*> {
            /// Creates a new mutable matrix view from the given data array.
            ///
            /// Panics if `data` does not contain enough elements.
            #[inline]
            pub fn from_slice(data: &'a mut [T], $($args: usize),*) -> Self {
                Self::from_slice_generic(data, $($gargs),*)
            }

            /// Creates, without bound checking, a new mutable matrix view from the given data array.
            ///
            /// # Safety
            ///
            /// `data[start..start+(R * C)]` must be within bounds.
            #[inline]
            pub unsafe fn from_slice_unchecked(data: &'a mut [T], start: usize, $($args: usize),*) -> Self { unsafe {
                Self::from_slice_generic_unchecked(data, start, $($gargs),*)
            }}
        }

        impl<'a, T: Scalar, $($DimIdent: $DimBound, )*> MatrixViewMut<'a, T, $($Dims,)* Dyn, Dyn> {
            /// Creates a new mutable matrix view with the specified strides from the given data array.
            ///
            /// Panics if `data` does not contain enough elements.
            #[inline]
            pub fn from_slice_with_strides_mut(data: &'a mut [T], $($args: usize,)* rstride: usize, cstride: usize) -> Self {
                Self::from_slice_with_strides_generic(
                    data, $($gargs,)* Dyn(rstride), Dyn(cstride))
            }

            /// Creates, without bound checking, a new mutable matrix view with the specified strides from the given data array.
            /// # Safety
            /// `data[start..start+rstride * cstride]` must be within bounds.
            #[inline]
            pub unsafe fn from_slice_with_strides_unchecked(data: &'a mut [T], start: usize, $($args: usize,)* rstride: usize, cstride: usize) -> Self { unsafe {
                Self::from_slice_with_strides_generic_unchecked(
                    data, start, $($gargs,)* Dyn(rstride), Dyn(cstride))
            }}
        }
    }
);

// TODO: this is not very pretty. We could find a better call syntax.
impl_constructors_mut!(R, C;                         // Arguments for Matrix<T, ..., S>
=> R: DimName, => C: DimName; // Type parameters for impl<T, ..., S>
R::name(), C::name();         // Arguments for `_generic` constructors.
); // Arguments for non-generic constructors.

impl_constructors_mut!(R, Dyn;
                       => R: DimName;
                       R::name(), Dyn(ncols);
                       ncols);

impl_constructors_mut!(Dyn, C;
                       => C: DimName;
                       Dyn(nrows), C::name();
                       nrows);

impl_constructors_mut!(Dyn, Dyn;
                       ;
                       Dyn(nrows), Dyn(ncols);
                       nrows, ncols);
