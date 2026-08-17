use super::plumbing::*;
use super::*;

/// `MultiZip` is an iterator that zips up a tuple of parallel iterators to
/// produce tuples of their items.
///
/// It is created by calling `into_par_iter()` on a tuple of types that
/// implement `IntoParallelIterator`, or `par_iter()`/`par_iter_mut()` with
/// types that are iterable by reference.
///
/// The implementation currently support tuples up to length 12.
///
/// # Examples
///
/// ```
/// use rayon::prelude::*;
///
/// // This will iterate `r` by mutable reference, like `par_iter_mut()`, while
/// // ranges are all iterated by value like `into_par_iter()`.
/// // Note that the zipped iterator is only as long as the shortest input.
/// let mut r = vec![0; 3];
/// (&mut r, 1..10, 10..100, 100..1000).into_par_iter()
///     .for_each(|(r, x, y, z)| *r = x * y + z);
///
/// assert_eq!(&r, &[1 * 10 + 100, 2 * 11 + 101, 3 * 12 + 102]);
/// ```
///
/// For a group that should all be iterated by reference, you can use a tuple reference.
///
/// ```
/// use rayon::prelude::*;
///
/// let xs: Vec<_> = (1..10).collect();
/// let ys: Vec<_> = (10..100).collect();
/// let zs: Vec<_> = (100..1000).collect();
///
/// // Reference each input separately with `IntoParallelIterator`:
/// let r1: Vec<_> = (&xs, &ys, &zs).into_par_iter()
///     .map(|(x, y, z)| x * y + z)
///     .collect();
///
/// // Reference them all together with `IntoParallelRefIterator`:
/// let r2: Vec<_> = (xs, ys, zs).par_iter()
///     .map(|(x, y, z)| x * y + z)
///     .collect();
///
/// assert_eq!(r1, r2);
/// ```
///
/// Mutable references to a tuple will work similarly.
///
/// ```
/// use rayon::prelude::*;
///
/// let mut xs: Vec<_> = (1..4).collect();
/// let mut ys: Vec<_> = (-4..-1).collect();
/// let mut zs = vec![0; 3];
///
/// // Mutably reference each input separately with `IntoParallelIterator`:
/// (&mut xs, &mut ys, &mut zs).into_par_iter().for_each(|(x, y, z)| {
///     *z += *x + *y;
///     std::mem::swap(x, y);
/// });
///
/// assert_eq!(xs, (vec![-4, -3, -2]));
/// assert_eq!(ys, (vec![1, 2, 3]));
/// assert_eq!(zs, (vec![-3, -1, 1]));
///
/// // Mutably reference them all together with `IntoParallelRefMutIterator`:
/// let mut tuple = (xs, ys, zs);
/// tuple.par_iter_mut().for_each(|(x, y, z)| {
///     *z += *x + *y;
///     std::mem::swap(x, y);
/// });
///
/// assert_eq!(tuple, (vec![1, 2, 3], vec![-4, -3, -2], vec![-6, -2, 2]));
/// ```
#[derive(Debug, Clone)]
pub struct MultiZip<T> {
    tuple: T,
}

// These macros greedily consume 4 or 2 items first to achieve log2 nesting depth.
// For example, 5 => 4,1 => (2,2),1.
//
// The tuples go up to 12, so we might want to greedily consume 8 too, but
// the depth works out the same if we let that expand on the right:
//      9 => 4,5 => (2,2),(4,1) => (2,2),((2,2),1)
//     12 => 4,8 => (2,2),(4,4) => (2,2),((2,2),(2,2))
//
// But if we ever increase to 13, we would want to split 8,5 rather than 4,9.

macro_rules! reduce {
    ($a:expr, $b:expr, $c:expr, $d:expr, $( $x:expr ),+ => $fn:path) => {
        reduce!(reduce!($a, $b, $c, $d => $fn),
                reduce!($( $x ),+ => $fn)
                => $fn)
    };
    ($a:expr, $b:expr, $( $x:expr ),+ => $fn:path) => {
        reduce!(reduce!($a, $b => $fn),
                reduce!($( $x ),+ => $fn)
                => $fn)
    };
    ($a:expr, $b:expr => $fn:path) => { $fn($a, $b) };
    ($a:expr => $fn:path) => { $a };
}

macro_rules! nest {
    ($A:tt, $B:tt, $C:tt, $D:tt, $( $X:tt ),+) => {
        (nest!($A, $B, $C, $D), nest!($( $X ),+))
    };
    ($A:tt, $B:tt, $( $X:tt ),+) => {
        (($A, $B), nest!($( $X ),+))
    };
    ($A:tt, $B:tt) => { ($A, $B) };
    ($A:tt) => { $A };
}

macro_rules! flatten {
    ($( $T:ident ),+) => {{
        #[allow(non_snake_case)]
        fn flatten<$( $T ),+>(nest!($( $T ),+) : nest!($( $T ),+)) -> ($( $T, )+) {
            ($( $T, )+)
        }
        flatten
    }};
}

macro_rules! multizip_impls {
    ($(
        $Tuple:ident {
            $(($idx:tt) -> $T:ident)+
        }
    )+) => {
        $(
            impl<$( $T, )+> IntoParallelIterator for ($( $T, )+)
            where
                $(
                    $T: IntoParallelIterator<Iter: IndexedParallelIterator>,
                )+
            {
                type Item = ($( $T::Item, )+);
                type Iter = MultiZip<($( $T::Iter, )+)>;

                fn into_par_iter(self) -> Self::Iter {
                    MultiZip {
                        tuple: ( $( self.$idx.into_par_iter(), )+ ),
                    }
                }
            }

            impl<'a, $( $T, )+> IntoParallelIterator for &'a ($( $T, )+)
            where
                $(
                    $T: IntoParallelRefIterator<'a, Iter: IndexedParallelIterator>,
                )+
            {
                type Item = ($( $T::Item, )+);
                type Iter = MultiZip<($( $T::Iter, )+)>;

                fn into_par_iter(self) -> Self::Iter {
                    MultiZip {
                        tuple: ( $( self.$idx.par_iter(), )+ ),
                    }
                }
            }

            impl<'a, $( $T, )+> IntoParallelIterator for &'a mut ($( $T, )+)
            where
                $(
                    $T: IntoParallelRefMutIterator<'a, Iter: IndexedParallelIterator>,
                )+
            {
                type Item = ($( $T::Item, )+);
                type Iter = MultiZip<($( $T::Iter, )+)>;

                fn into_par_iter(self) -> Self::Iter {
                    MultiZip {
                        tuple: ( $( self.$idx.par_iter_mut(), )+ ),
                    }
                }
            }

            impl<$( $T, )+> ParallelIterator for MultiZip<($( $T, )+)>
            where
                $( $T: IndexedParallelIterator, )+
            {
                type Item = ($( $T::Item, )+);

                fn drive_unindexed<CONSUMER>(self, consumer: CONSUMER) -> CONSUMER::Result
                where
                    CONSUMER: UnindexedConsumer<Self::Item>,
                {
                    self.drive(consumer)
                }

                fn opt_len(&self) -> Option<usize> {
                    Some(self.len())
                }
            }

            impl<$( $T, )+> IndexedParallelIterator for MultiZip<($( $T, )+)>
            where
                $( $T: IndexedParallelIterator, )+
            {
                fn drive<CONSUMER>(self, consumer: CONSUMER) -> CONSUMER::Result
                where
                    CONSUMER: Consumer<Self::Item>,
                {
                    reduce!($( self.tuple.$idx ),+ => IndexedParallelIterator::zip)
                        .map(flatten!($( $T ),+))
                        .drive(consumer)
                }

                fn len(&self) -> usize {
                    reduce!($( self.tuple.$idx.len() ),+ => Ord::min)
                }

                fn with_producer<CB>(self, callback: CB) -> CB::Output
                where
                    CB: ProducerCallback<Self::Item>,
                {
                    reduce!($( self.tuple.$idx ),+ => IndexedParallelIterator::zip)
                        .map(flatten!($( $T ),+))
                        .with_producer(callback)
                }
            }
        )+
    }
}

multizip_impls! {
    Tuple1 {
        (0) -> A
    }
    Tuple2 {
        (0) -> A
        (1) -> B
    }
    Tuple3 {
        (0) -> A
        (1) -> B
        (2) -> C
    }
    Tuple4 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
    }
    Tuple5 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
    }
    Tuple6 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
    }
    Tuple7 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
    }
    Tuple8 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
    }
    Tuple9 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
    }
    Tuple10 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
        (9) -> J
    }
    Tuple11 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
        (9) -> J
        (10) -> K
    }
    Tuple12 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
        (6) -> G
        (7) -> H
        (8) -> I
        (9) -> J
        (10) -> K
        (11) -> L
    }
}
