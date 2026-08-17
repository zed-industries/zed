use crate::Const;
use crate::base::ArrayStorage;
use crate::geometry::Reflection;

/// A 1-dimensional reflection.
pub type Reflection1<T> = Reflection<T, Const<1>, ArrayStorage<T, 1, 1>>;

/// A 2-dimensional reflection.
pub type Reflection2<T> = Reflection<T, Const<2>, ArrayStorage<T, 2, 1>>;

/// A 3-dimensional reflection.
pub type Reflection3<T> = Reflection<T, Const<3>, ArrayStorage<T, 3, 1>>;

/// A 4-dimensional reflection.
pub type Reflection4<T> = Reflection<T, Const<4>, ArrayStorage<T, 4, 1>>;

/// A 5-dimensional reflection.
pub type Reflection5<T> = Reflection<T, Const<5>, ArrayStorage<T, 5, 1>>;

/// A 6-dimensional reflection.
pub type Reflection6<T> = Reflection<T, Const<6>, ArrayStorage<T, 6, 1>>;
