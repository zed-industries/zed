#![macro_use]

// TODO: merge with `md_impl`.
/// Macro for the implementation of multiplication and division.
macro_rules! md_impl(
    (
    // Operator, operator method, and scalar bounds.
     $Op: ident, $op: ident $(where T: $($ScalarBounds: ident),*)*;
     // Storage dimensions, and dimension bounds.
     ($R1: ty, $C1: ty),($R2: ty, $C2: ty)
     // Const type declaration
     const $($D: ident),*;
     // Other generic type declarations.
     for $($DimsDecl: ident),*;
     // Where clause.
     where $($ConstraintType: ty: $ConstraintBound: ident$(<$($ConstraintBoundParams: ty $( = $EqBound: ty )*),*>)*),*;
     // Argument identifiers and types + output.
     $lhs: ident: $Lhs: ty, $rhs: ident: $Rhs: ty, Output = $Result: ty;
     // Operator actual implementation.
     $action: expr_2021;
     // Lifetime.
     $($lives: tt),*) => {
        impl<$($lives ,)* T $(, $DimsDecl)* $(, const $D: usize)*> $Op<$Rhs> for $Lhs
            where T: Scalar + Zero + One + ClosedAddAssign + ClosedMulAssign $($(+ $ScalarBounds)*)*,
                  $( $ConstraintType: $ConstraintBound$(<$( $ConstraintBoundParams $( = $EqBound )*),*>)* ),*
                   {
            type Output = $Result;

            #[inline]
            fn $op($lhs, $rhs: $Rhs) -> Self::Output {
                $action
            }
        }
    }
);

/// Macro for the implementation of multiplication and division.
/// Implements all the argument reference combinations.
macro_rules! md_impl_all(
    (
     // Operator, operator method, and scalar bounds.
     $Op: ident, $op: ident $(where T: $($ScalarBounds: ident),*)*;
     // Storage dimensions, and dimension bounds.
     ($R1: ty, $C1: ty),($R2: ty, $C2: ty)
     // Const type declaration
     const $($D: ident),*;
     // Other generic type declarations.
     for $($DimsDecl: ident),*;
     // Where clause.
     where $($ConstraintType: ty: $ConstraintBound: ident$(<$($ConstraintBoundParams: ty $( = $EqBound: ty )*),*>)*),*;
     // Argument identifiers and types + output.
     $lhs: ident: $Lhs: ty, $rhs: ident: $Rhs: ty, Output = $Result: ty;
     // Operators actual implementations.
     [val val] => $action_val_val: expr_2021;
     [ref val] => $action_ref_val: expr_2021;
     [val ref] => $action_val_ref: expr_2021;
     [ref ref] => $action_ref_ref: expr_2021;) => {

        md_impl!(
            $Op, $op $(where T: $($ScalarBounds),*)*;
            ($R1, $C1),($R2, $C2)
            const $($D),*;
            for $($DimsDecl),*;
            where $($ConstraintType: $ConstraintBound$(<$($ConstraintBoundParams $( = $EqBound )*),*>)*),*;
            $lhs: $Lhs, $rhs: $Rhs, Output = $Result;
            $action_val_val; );

        md_impl!(
            $Op, $op $(where T: $($ScalarBounds),*)*;
            ($R1, $C1),($R2, $C2)
            const $($D),*;
            for $($DimsDecl),*;
            where $($ConstraintType: $ConstraintBound$(<$($ConstraintBoundParams $( = $EqBound )*),*>)*),*;
            $lhs: &'a $Lhs, $rhs: $Rhs, Output = $Result;
            $action_ref_val; 'a);

        md_impl!(
            $Op, $op $(where T: $($ScalarBounds),*)*;
            ($R1, $C1),($R2, $C2)
            const $($D),*;
            for $($DimsDecl),*;
            where $($ConstraintType: $ConstraintBound$(<$($ConstraintBoundParams $( = $EqBound )*),*>)*),*;
            $lhs: $Lhs, $rhs: &'b $Rhs, Output = $Result;
            $action_val_ref; 'b);

        md_impl!(
            $Op, $op $(where T: $($ScalarBounds),*)*;
            ($R1, $C1),($R2, $C2)
            const $($D),*;
            for $($DimsDecl),*;
            where $($ConstraintType: $ConstraintBound$(<$($ConstraintBoundParams $( = $EqBound )*),*>)*),*;
            $lhs: &'a $Lhs, $rhs: &'b $Rhs, Output = $Result;
            $action_ref_ref; 'a, 'b);
    }
);

/// Macro for the implementation of assignment-multiplication and assignment-division.
macro_rules! md_assign_impl(
    (
     // Operator, operator method, and scalar bounds.
     $Op: ident, $op: ident $(where T: $($ScalarBounds: ident),*)* $(for T::Element: $($ElementBounds: ident),*)*;
     // Storage dimensions, and dimension bounds.
     ($R1: ty, $C1: ty),($R2: ty, $C2: ty)
     // Const type declaration
     const $($D: ident),*;
     // Other generic type declarations.
     for $($DimsDecl: ident),*;
     // Where clause.
     where $($ConstraintType: ty: $ConstraintBound: ident$(<$($ConstraintBoundParams: ty $( = $EqBound: ty )*),*>)*),*;
     // Argument identifiers and types.
     $lhs: ident: $Lhs: ty, $rhs: ident: $Rhs: ty;
     // Actual implementation and lifetimes.
     $action: expr_2021; $($lives: tt),*) => {
        impl<$($lives ,)* T $(, $DimsDecl)* $(, const $D: usize)*> $Op<$Rhs> for $Lhs
            where T: Scalar + Zero + One + ClosedAddAssign + ClosedMulAssign $($(+ $ScalarBounds)*)*,
                  $($(T::Element: $ElementBounds,)*)*
                  $( $ConstraintType: $ConstraintBound $(<$( $ConstraintBoundParams $( = $EqBound )*),*>)* ),*
        {
            #[inline]
            fn $op(&mut $lhs, $rhs: $Rhs) {
                $action
            }
        }
    }
);

/// Macro for the implementation of assignment-multiplication and assignment-division with and
/// without reference to the right-hand-side.
macro_rules! md_assign_impl_all(
    (
     // Operator, operator method, and scalar bounds.
     $Op: ident, $op: ident $(where T: $($ScalarBounds: ident),*)* $(for T::Element: $($ElementBounds: ident),*)*;
     // Storage dimensions, and dimension bounds.
     ($R1: ty, $C1: ty),($R2: ty, $C2: ty)
     // Const type declaration
     const $($D: ident),*;
     // Other generic type declarations.
     for $($DimsDecl: ident),*;
     // Where clause.
     where $($ConstraintType: ty: $ConstraintBound: ident$(<$($ConstraintBoundParams: ty $( = $EqBound: ty )*),*>)*),*;
     // Argument identifiers and types.
     $lhs: ident: $Lhs: ty, $rhs: ident: $Rhs: ty;
     // Actual implementation and lifetimes.
     [val] => $action_val: expr_2021;
     [ref] => $action_ref: expr_2021;) => {
        md_assign_impl!(
            $Op, $op $(where T: $($ScalarBounds),*)* $(for T::Element: $($ElementBounds),*)*;
            ($R1, $C1),($R2, $C2)
            const $($D),*;
            for $($DimsDecl),*;
            where $($ConstraintType: $ConstraintBound$(<$($ConstraintBoundParams $( = $EqBound )*),*>)*),*;
            $lhs: $Lhs, $rhs: $Rhs;
            $action_val; );

        md_assign_impl!(
            $Op, $op $(where T: $($ScalarBounds),*)* $(for T::Element: $($ElementBounds),*)*;
            ($R1, $C1),($R2, $C2)
            const $($D),*;
            for $($DimsDecl),*;
            where $($ConstraintType: $ConstraintBound$(<$($ConstraintBoundParams $( = $EqBound )*),*>)*),*;
            $lhs: $Lhs, $rhs: &'b $Rhs;
            $action_ref; 'b);
    }
);

// TODO: merge with `as_impl`.
/// Macro for the implementation of addition and subtraction.
macro_rules! add_sub_impl(
    ($Op: ident, $op: ident, $bound: ident;
     ($R1: ty, $C1: ty),($R2: ty, $C2: ty) $(-> ($RRes: ty, $CRes: ty))*
     // Const type declaration
     const $($D: ident),*;
     // Other generic type declarations.
     for $($DimsDecl: ident),*;
     // Where clause.
     where $($ConstraintType: ty: $ConstraintBound: ident$(<$($ConstraintBoundParams: ty $( = $EqBound: ty )*),*>)*),*;
     $lhs: ident: $Lhs: ty, $rhs: ident: $Rhs: ty, Output = $Result: ty;
     $action: expr_2021; $($lives: tt),*) => {
        impl<$($lives ,)* T $(, $DimsDecl)* $(, const $D: usize)*> $Op<$Rhs> for $Lhs
            where T: Scalar + $bound,
                  ShapeConstraint: SameNumberOfRows<$R1, $R2 $(, Representative = $RRes)*> +
                                   SameNumberOfColumns<$C1, $C2 $(, Representative = $CRes)*>,
                  $( $ConstraintType: $ConstraintBound$(<$( $ConstraintBoundParams $( = $EqBound )*),*>)* ),* {
            type Output = $Result;

            #[inline]
            fn $op($lhs, $rhs: $Rhs) -> Self::Output {
                $action
            }
        }
    }
);

// TODO: merge with `md_assign_impl`.
/// Macro for the implementation of assignment-addition and assignment-subtraction.
macro_rules! add_sub_assign_impl(
    ($Op: ident, $op: ident, $bound: ident;
    $(const $D: ident),*;
     $lhs: ident: $Lhs: ty, $rhs: ident: $Rhs: ty;
     $action: expr_2021; $($lives: tt),*) => {
        impl<$($lives ,)* T $(, const $D: usize),*> $Op<$Rhs> for $Lhs
            where T: Scalar + $bound {
            #[inline]
            fn $op(&mut $lhs, $rhs: $Rhs) {
                $action
            }
        }
    }
);
