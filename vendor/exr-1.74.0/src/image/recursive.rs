//! A generic wrapper which can be used to represent recursive types.
//! Supports conversion from and to tuples of the same size.

/// No more recursion. Can be used within any `Recursive<NoneMore, YourValue>` type.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct NoneMore;

/// A recursive type-level linked list of `Value` entries.
/// Mainly used to represent an arbitrary number of channels.
/// The recursive architecture removes the need to implement traits for many different tuples.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Recursive<Inner, Value> {
    /// The remaining values of this linked list,
    /// probably either `NoneMore` or another instance of the same `Recursive<Inner - 1, Value>`.
    pub inner: Inner,

    /// The next item in this linked list.
    pub value: Value,
}

impl<Inner, Value> Recursive<Inner, Value> {
    /// Create a new recursive type. Equivalent to the manual constructor, but less verbose.
    pub fn new(inner: Inner, value: Value) -> Self { Self { inner, value } }
}

/// Convert this recursive type into a tuple.
/// This is nice as it will require less typing for the same type.
/// A type might or might not be convertible to the specified `Tuple` type.
pub trait IntoTuple<Tuple> {
    /// Convert this recursive type to a nice tuple.
    fn into_tuple(self) -> Tuple;
}

/// Convert this recursive type into a tuple.
/// This is nice as it will require less typing for the same type.
/// A type will be converted to the specified `Self::NonRecursive` type.
pub trait IntoNonRecursive {
    /// The resulting tuple type.
    type NonRecursive;

    /// Convert this recursive type to a nice tuple.
    fn into_non_recursive(self) -> Self::NonRecursive;
}

/// Create a recursive type from this tuple.
pub trait IntoRecursive {
    /// The recursive type resulting from this tuple.
    type Recursive;

    /// Create a recursive type from this tuple.
    fn into_recursive(self) -> Self::Recursive;
}

impl IntoRecursive for NoneMore {
    type Recursive = Self;
    fn into_recursive(self) -> Self::Recursive { self }
}

impl<Inner: IntoRecursive, Value> IntoRecursive for Recursive<Inner, Value> {
    type Recursive = Recursive<Inner::Recursive, Value>;
    fn into_recursive(self) -> Self::Recursive { Recursive::new(self.inner.into_recursive(), self.value) }
}

// Automatically implement IntoTuple so we have to generate less code in the macros
impl<I: IntoNonRecursive> IntoTuple<I::NonRecursive> for I {
    fn into_tuple(self) -> <I as IntoNonRecursive>::NonRecursive {
        self.into_non_recursive()
    }
}

//Implement traits for the empty tuple, the macro doesn't handle that
impl IntoRecursive for () {
    type Recursive = NoneMore;
    fn into_recursive(self) -> Self::Recursive { NoneMore }
}

impl IntoNonRecursive for NoneMore {
    type NonRecursive = ();

    fn into_non_recursive(self) -> Self::NonRecursive {
        ()
    }
}

/// Generates the recursive type corresponding to this tuple:
/// ```nocheck
/// gen_recursive_type!(A, B, C)
/// => Recursive<Recursive<Recursive<NoneMore, A>, B>, C>
/// ```
macro_rules! gen_recursive_type {
    () => { NoneMore };
    ($last:ident $(,$not_last:ident)*) => {
        Recursive<gen_recursive_type!($($not_last),*), $last>
    };
}

/// Generates the recursive value corresponding to the given indices:
/// ```nocheck
/// gen_recursive_value(self; 1, 0)
/// => Recursive { inner: Recursive {  inner: NoneMore, value: self.0 }, value: self.1 }
/// ```
macro_rules! gen_recursive_value {
    ($self:ident;) => { NoneMore };
    ($self:ident; $last:tt $(,$not_last:tt)*) => {
        Recursive { inner: gen_recursive_value!($self; $($not_last),*), value: $self.$last }
    };
}

/// Generates the into_tuple value corresponding to the given type names:
/// ```nocheck
/// gen_tuple_value(self; A, B, C)
/// => (self.inner.inner.value, self.inner.value, self.value)
/// ```
macro_rules! gen_tuple_value {
    ($self:ident; $($all:ident),* ) => {
        gen_tuple_value!(@ $self; (); $($all),*  )
    };

    (@ $self:ident; ($($state:expr),*);) => { ($($state .value,)*) };
    (@ $self:ident; ($($state:expr),*); $last:ident $(,$not_last:ident)* ) => {
        gen_tuple_value!(@ $self; ($($state .inner,)* $self); $($not_last),*  )
    };
}

/// Generate the trait implementations given a sequence of type names in both directions and the indices backwards:
/// ```nocheck
/// generate_single(A, B, C; C, B, A; 2, 1, 0)
/// ```
macro_rules! generate_single {
    ( $($name_fwd:ident),* ; $($name_back:ident),* ; $($index_back:tt),*) => {
        impl<$($name_fwd),*> IntoNonRecursive for gen_recursive_type!($($name_back),*) {
            type NonRecursive = ($($name_fwd,)*);
            fn into_non_recursive(self) -> Self::NonRecursive {
                gen_tuple_value!(self; $($name_fwd),*)
            }
        }

        impl<$($name_fwd),*> IntoRecursive for ($($name_fwd,)*) {
            type Recursive = gen_recursive_type!($($name_back),*);
            fn into_recursive(self) -> Self::Recursive {
                gen_recursive_value!(self; $($index_back),*)
            }
        }
    };
}

generate_single!(A; A; 0);
generate_single!(A,B; B,A; 1,0);
generate_single!(A,B,C; C,B,A; 2,1,0);
generate_single!(A,B,C,D; D,C,B,A; 3,2,1,0);
generate_single!(A,B,C,D,E; E,D,C,B,A; 4,3,2,1,0);
generate_single!(A,B,C,D,E,F; F,E,D,C,B,A; 5,4,3,2,1,0);
generate_single!(A,B,C,D,E,F,G; G,F,E,D,C,B,A; 6,5,4,3,2,1,0);
generate_single!(A,B,C,D,E,F,G,H; H,G,F,E,D,C,B,A; 7,6,5,4,3,2,1,0);
generate_single!(A,B,C,D,E,F,G,H,I; I,H,G,F,E,D,C,B,A; 8,7,6,5,4,3,2,1,0);
generate_single!(A,B,C,D,E,F,G,H,I,J; J,I,H,G,F,E,D,C,B,A; 9,8,7,6,5,4,3,2,1,0);
generate_single!(A,B,C,D,E,F,G,H,I,J,K; K,J,I,H,G,F,E,D,C,B,A; 10,9,8,7,6,5,4,3,2,1,0);
generate_single!(A,B,C,D,E,F,G,H,I,J,K,L; L,K,J,I,H,G,F,E,D,C,B,A; 11,10,9,8,7,6,5,4,3,2,1,0);
generate_single!(A,B,C,D,E,F,G,H,I,J,K,L,M; M,L,K,J,I,H,G,F,E,D,C,B,A; 12,11,10,9,8,7,6,5,4,3,2,1,0);
generate_single!(A,B,C,D,E,F,G,H,I,J,K,L,M,N; N,M,L,K,J,I,H,G,F,E,D,C,B,A; 13,12,11,10,9,8,7,6,5,4,3,2,1,0);
generate_single!(A,B,C,D,E,F,G,H,I,J,K,L,M,N,O; O,N,M,L,K,J,I,H,G,F,E,D,C,B,A; 14,13,12,11,10,9,8,7,6,5,4,3,2,1,0);
generate_single!(A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P; P,O,N,M,L,K,J,I,H,G,F,E,D,C,B,A; 15,14,13,12,11,10,9,8,7,6,5,4,3,2,1,0);
generate_single!(A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q; Q,P,O,N,M,L,K,J,I,H,G,F,E,D,C,B,A; 16,15,14,13,12,11,10,9,8,7,6,5,4,3,2,1,0);
generate_single!(A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R; R,Q,P,O,N,M,L,K,J,I,H,G,F,E,D,C,B,A; 17,16,15,14,13,12,11,10,9,8,7,6,5,4,3,2,1,0);
generate_single!(A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S; S,R,Q,P,O,N,M,L,K,J,I,H,G,F,E,D,C,B,A; 18,17,16,15,14,13,12,11,10,9,8,7,6,5,4,3,2,1,0);
generate_single!(A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T; T,S,R,Q,P,O,N,M,L,K,J,I,H,G,F,E,D,C,B,A; 19,18,17,16,15,14,13,12,11,10,9,8,7,6,5,4,3,2,1,0);
generate_single!(A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U; U,T,S,R,Q,P,O,N,M,L,K,J,I,H,G,F,E,D,C,B,A; 20,19,18,17,16,15,14,13,12,11,10,9,8,7,6,5,4,3,2,1,0);
generate_single!(A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V; V,U,T,S,R,Q,P,O,N,M,L,K,J,I,H,G,F,E,D,C,B,A; 21,20,19,18,17,16,15,14,13,12,11,10,9,8,7,6,5,4,3,2,1,0);
generate_single!(A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W; W,V,U,T,S,R,Q,P,O,N,M,L,K,J,I,H,G,F,E,D,C,B,A; 22,21,20,19,18,17,16,15,14,13,12,11,10,9,8,7,6,5,4,3,2,1,0);
generate_single!(A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X; X,W,V,U,T,S,R,Q,P,O,N,M,L,K,J,I,H,G,F,E,D,C,B,A; 23,22,21,20,19,18,17,16,15,14,13,12,11,10,9,8,7,6,5,4,3,2,1,0);
generate_single!(A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X,Y; Y,X,W,V,U,T,S,R,Q,P,O,N,M,L,K,J,I,H,G,F,E,D,C,B,A; 24,23,22,21,20,19,18,17,16,15,14,13,12,11,10,9,8,7,6,5,4,3,2,1,0);
generate_single!(A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X,Y,Z; Z,Y,X,W,V,U,T,S,R,Q,P,O,N,M,L,K,J,I,H,G,F,E,D,C,B,A; 25,24,23,22,21,20,19,18,17,16,15,14,13,12,11,10,9,8,7,6,5,4,3,2,1,0);
generate_single!(A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X,Y,Z,A1; A1,Z,Y,X,W,V,U,T,S,R,Q,P,O,N,M,L,K,J,I,H,G,F,E,D,C,B,A; 26,25,24,23,22,21,20,19,18,17,16,15,14,13,12,11,10,9,8,7,6,5,4,3,2,1,0);
generate_single!(A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X,Y,Z,A1,B1; B1,A1,Z,Y,X,W,V,U,T,S,R,Q,P,O,N,M,L,K,J,I,H,G,F,E,D,C,B,A; 27,26,25,24,23,22,21,20,19,18,17,16,15,14,13,12,11,10,9,8,7,6,5,4,3,2,1,0);
generate_single!(A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X,Y,Z,A1,B1,C1; C1,B1,A1,Z,Y,X,W,V,U,T,S,R,Q,P,O,N,M,L,K,J,I,H,G,F,E,D,C,B,A; 28,27,26,25,24,23,22,21,20,19,18,17,16,15,14,13,12,11,10,9,8,7,6,5,4,3,2,1,0);
generate_single!(A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X,Y,Z,A1,B1,C1,D1; D1,C1,B1,A1,Z,Y,X,W,V,U,T,S,R,Q,P,O,N,M,L,K,J,I,H,G,F,E,D,C,B,A; 29,28,27,26,25,24,23,22,21,20,19,18,17,16,15,14,13,12,11,10,9,8,7,6,5,4,3,2,1,0);
generate_single!(A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X,Y,Z,A1,B1,C1,D1,E1; E1,D1,C1,B1,A1,Z,Y,X,W,V,U,T,S,R,Q,P,O,N,M,L,K,J,I,H,G,F,E,D,C,B,A; 30,29,28,27,26,25,24,23,22,21,20,19,18,17,16,15,14,13,12,11,10,9,8,7,6,5,4,3,2,1,0);
generate_single!(A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X,Y,Z,A1,B1,C1,D1,E1,F1; F1,E1,D1,C1,B1,A1,Z,Y,X,W,V,U,T,S,R,Q,P,O,N,M,L,K,J,I,H,G,F,E,D,C,B,A; 31,30,29,28,27,26,25,24,23,22,21,20,19,18,17,16,15,14,13,12,11,10,9,8,7,6,5,4,3,2,1,0);
