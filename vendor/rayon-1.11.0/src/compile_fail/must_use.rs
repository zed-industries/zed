// Check that we are flagged for ignoring `must_use` parallel adaptors.
// (unfortunately there's no error code for `unused_must_use`)

macro_rules! must_use {
    ($( $name:ident #[$expr:meta] )*) => {$(
        /// First sanity check that the expression is OK.
        ///
        /// ```
        /// #![deny(unused_must_use)]
        ///
        /// use rayon::prelude::*;
        ///
        /// let v: Vec<_> = (0..100).map(Some).collect();
        /// let _ =
        #[$expr]
        /// ```
        ///
        /// Now trigger the `must_use`.
        ///
        /// ```compile_fail
        /// #![deny(unused_must_use)]
        ///
        /// use rayon::prelude::*;
        ///
        /// let v: Vec<_> = (0..100).map(Some).collect();
        #[$expr]
        /// ```
        mod $name {}
    )*}
}

must_use! {
    by_exponential_blocks  /** v.par_iter().by_exponential_blocks(); */
    by_uniform_blocks   /** v.par_iter().by_uniform_blocks(2); */
    step_by             /** v.par_iter().step_by(2); */
    chain               /** v.par_iter().chain(&v); */
    chunks              /** v.par_iter().chunks(2); */
    fold_chunks         /** v.par_iter().fold_chunks(2, || 0, |x, _| x); */
    fold_chunks_with    /** v.par_iter().fold_chunks_with(2, 0, |x, _| x); */
    cloned              /** v.par_iter().cloned(); */
    copied              /** v.par_iter().copied(); */
    enumerate           /** v.par_iter().enumerate(); */
    filter              /** v.par_iter().filter(|_| true); */
    filter_map          /** v.par_iter().filter_map(|x| *x); */
    flat_map            /** v.par_iter().flat_map(|x| *x); */
    flat_map_iter       /** v.par_iter().flat_map_iter(|x| *x); */
    flatten             /** v.par_iter().flatten(); */
    flatten_iter        /** v.par_iter().flatten_iter(); */
    fold                /** v.par_iter().fold(|| 0, |x, _| x); */
    fold_with           /** v.par_iter().fold_with(0, |x, _| x); */
    try_fold            /** v.par_iter().try_fold(|| 0, |x, _| Some(x)); */
    try_fold_with       /** v.par_iter().try_fold_with(0, |x, _| Some(x)); */
    inspect             /** v.par_iter().inspect(|_| {}); */
    interleave          /** v.par_iter().interleave(&v); */
    interleave_shortest /** v.par_iter().interleave_shortest(&v); */
    intersperse         /** v.par_iter().intersperse(&None); */
    map                 /** v.par_iter().map(|x| x); */
    map_with            /** v.par_iter().map_with(0, |_, x| x); */
    map_init            /** v.par_iter().map_init(|| 0, |_, x| x); */
    panic_fuse          /** v.par_iter().panic_fuse(); */
    positions           /** v.par_iter().positions(|_| true); */
    rev                 /** v.par_iter().rev(); */
    skip                /** v.par_iter().skip(1); */
    take                /** v.par_iter().take(1); */
    update              /** v.par_iter().update(|_| {}); */
    while_some          /** v.par_iter().cloned().while_some(); */
    with_max_len        /** v.par_iter().with_max_len(1); */
    with_min_len        /** v.par_iter().with_min_len(1); */
    zip                 /** v.par_iter().zip(&v); */
    zip_eq              /** v.par_iter().zip_eq(&v); */
}
