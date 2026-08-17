use crate::{ColumnTrait, EntityTrait, IdenStatic};
use sea_query::{Alias, DynIden, Iden, IntoIden, SeaRc};
use std::fmt;

/// List of column identifier
#[derive(Debug, Clone)]
pub enum Identity {
    /// Column identifier consists of 1 column
    Unary(DynIden),
    /// Column identifier consists of 2 columns
    Binary(DynIden, DynIden),
    /// Column identifier consists of 3 columns
    Ternary(DynIden, DynIden, DynIden),
    /// Column identifier consists of more than 3 columns
    Many(Vec<DynIden>),
}

impl IntoIterator for Identity {
    type Item = DynIden;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Identity::Unary(ident1) => vec![ident1].into_iter(),
            Identity::Binary(ident1, ident2) => vec![ident1, ident2].into_iter(),
            Identity::Ternary(ident1, ident2, ident3) => vec![ident1, ident2, ident3].into_iter(),
            Identity::Many(vec) => vec.into_iter(),
        }
    }
}

impl Iden for Identity {
    fn unquoted(&self, s: &mut dyn fmt::Write) {
        match self {
            Identity::Unary(iden) => {
                write!(s, "{}", iden.to_string()).unwrap();
            }
            Identity::Binary(iden1, iden2) => {
                write!(s, "{}", iden1.to_string()).unwrap();
                write!(s, "{}", iden2.to_string()).unwrap();
            }
            Identity::Ternary(iden1, iden2, iden3) => {
                write!(s, "{}", iden1.to_string()).unwrap();
                write!(s, "{}", iden2.to_string()).unwrap();
                write!(s, "{}", iden3.to_string()).unwrap();
            }
            Identity::Many(vec) => {
                for iden in vec.iter() {
                    write!(s, "{}", iden.to_string()).unwrap();
                }
            }
        }
    }
}

/// Performs a conversion into an [Identity]
pub trait IntoIdentity {
    /// Method to perform the conversion
    fn into_identity(self) -> Identity;
}

/// Check the [Identity] of an Entity
pub trait IdentityOf<E>
where
    E: EntityTrait,
{
    /// Method to call to perform this check
    fn identity_of(self) -> Identity;
}

impl IntoIdentity for Identity {
    fn into_identity(self) -> Identity {
        self
    }
}

impl IntoIdentity for String {
    fn into_identity(self) -> Identity {
        self.as_str().into_identity()
    }
}

impl IntoIdentity for &str {
    fn into_identity(self) -> Identity {
        Identity::Unary(SeaRc::new(Alias::new(self)))
    }
}

impl<T> IntoIdentity for T
where
    T: IdenStatic,
{
    fn into_identity(self) -> Identity {
        Identity::Unary(self.into_iden())
    }
}

impl<T, C> IntoIdentity for (T, C)
where
    T: IdenStatic,
    C: IdenStatic,
{
    fn into_identity(self) -> Identity {
        Identity::Binary(self.0.into_iden(), self.1.into_iden())
    }
}

impl<T, C, R> IntoIdentity for (T, C, R)
where
    T: IdenStatic,
    C: IdenStatic,
    R: IdenStatic,
{
    fn into_identity(self) -> Identity {
        Identity::Ternary(self.0.into_iden(), self.1.into_iden(), self.2.into_iden())
    }
}

macro_rules! impl_into_identity {
    ( $($T:ident : $N:tt),+ $(,)? ) => {
        impl< $($T),+ > IntoIdentity for ( $($T),+ )
        where
            $($T: IdenStatic),+
        {
            fn into_identity(self) -> Identity {
                Identity::Many(vec![
                    $(self.$N.into_iden()),+
                ])
            }
        }
    };
}

#[rustfmt::skip]
mod impl_into_identity {
    use super::*;

    impl_into_identity!(T0:0, T1:1, T2:2, T3:3);
    impl_into_identity!(T0:0, T1:1, T2:2, T3:3, T4:4);
    impl_into_identity!(T0:0, T1:1, T2:2, T3:3, T4:4, T5:5);
    impl_into_identity!(T0:0, T1:1, T2:2, T3:3, T4:4, T5:5, T6:6);
    impl_into_identity!(T0:0, T1:1, T2:2, T3:3, T4:4, T5:5, T6:6, T7:7);
    impl_into_identity!(T0:0, T1:1, T2:2, T3:3, T4:4, T5:5, T6:6, T7:7, T8:8);
    impl_into_identity!(T0:0, T1:1, T2:2, T3:3, T4:4, T5:5, T6:6, T7:7, T8:8, T9:9);
    impl_into_identity!(T0:0, T1:1, T2:2, T3:3, T4:4, T5:5, T6:6, T7:7, T8:8, T9:9, T10:10);
    impl_into_identity!(T0:0, T1:1, T2:2, T3:3, T4:4, T5:5, T6:6, T7:7, T8:8, T9:9, T10:10, T11:11);
}

impl<E, C> IdentityOf<E> for C
where
    E: EntityTrait<Column = C>,
    C: ColumnTrait,
{
    fn identity_of(self) -> Identity {
        self.into_identity()
    }
}

macro_rules! impl_identity_of {
    ( $($T:ident),+ $(,)? ) => {
        impl<E, C> IdentityOf<E> for ( $($T),+ )
        where
            E: EntityTrait<Column = C>,
            C: ColumnTrait,
        {
            fn identity_of(self) -> Identity {
                self.into_identity()
            }
        }
    };
}

#[rustfmt::skip]
mod impl_identity_of {
    use super::*;

    impl_identity_of!(C, C);
    impl_identity_of!(C, C, C);
    impl_identity_of!(C, C, C, C);
    impl_identity_of!(C, C, C, C, C);
    impl_identity_of!(C, C, C, C, C, C);
    impl_identity_of!(C, C, C, C, C, C, C);
    impl_identity_of!(C, C, C, C, C, C, C, C);
    impl_identity_of!(C, C, C, C, C, C, C, C, C);
    impl_identity_of!(C, C, C, C, C, C, C, C, C, C);
    impl_identity_of!(C, C, C, C, C, C, C, C, C, C, C);
    impl_identity_of!(C, C, C, C, C, C, C, C, C, C, C, C);
}
