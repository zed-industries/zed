use std::iter;
use std::mem;

use crate::allocator::Allocator;
use crate::sparse::{CsMatrix, CsStorage, CsStorageIter, CsStorageIterMut, CsVecStorage};
use crate::{Const, DefaultAllocator, Dim, Matrix, OVector, RealField};

/// The cholesky decomposition of a column compressed sparse matrix.
pub struct CsCholesky<T: RealField, D: Dim>
where
    DefaultAllocator: Allocator<D>,
{
    // Non-zero pattern of the original matrix upper-triangular part.
    // Unlike the original matrix, the `original_p` array does contain the last sentinel value
    // equal to `original_i.len()` at the end.
    original_p: Vec<usize>,
    original_i: Vec<usize>,
    // Decomposition result.
    l: CsMatrix<T, D, D>,
    // Used only for the pattern.
    // TODO: store only the nonzero pattern instead.
    u: CsMatrix<T, D, D>,
    ok: bool,
    // Workspaces.
    work_x: OVector<T, D>,
    work_c: OVector<usize, D>,
}

impl<T: RealField, D: Dim> CsCholesky<T, D>
where
    DefaultAllocator: Allocator<D> + Allocator<D>,
{
    /// Computes the cholesky decomposition of the sparse matrix `m`.
    pub fn new(m: &CsMatrix<T, D, D>) -> Self {
        let mut me = Self::new_symbolic(m);
        let _ = me.decompose_left_looking(&m.data.vals);
        me
    }
    /// Perform symbolic analysis for the given matrix.
    ///
    /// This does not access the numerical values of `m`.
    pub fn new_symbolic(m: &CsMatrix<T, D, D>) -> Self {
        assert!(
            m.is_square(),
            "The matrix `m` must be square to compute its elimination tree."
        );

        let (l, u) = Self::nonzero_pattern(m);

        // Workspaces.
        let work_x = Matrix::zeros_generic(m.data.shape().0, Const::<1>);
        let work_c = Matrix::zeros_generic(m.data.shape().1, Const::<1>);
        let mut original_p = m.data.p.as_slice().to_vec();
        original_p.push(m.data.i.len());

        CsCholesky {
            original_p,
            original_i: m.data.i.clone(),
            l,
            u,
            ok: false,
            work_x,
            work_c,
        }
    }

    /// The lower-triangular matrix of the cholesky decomposition.
    #[must_use]
    pub fn l(&self) -> Option<&CsMatrix<T, D, D>> {
        if self.ok { Some(&self.l) } else { None }
    }

    /// Extracts the lower-triangular matrix of the cholesky decomposition.
    pub fn unwrap_l(self) -> Option<CsMatrix<T, D, D>> {
        if self.ok { Some(self.l) } else { None }
    }

    /// Perform a numerical left-looking cholesky decomposition of a matrix with the same structure as the
    /// one used to initialize `self`, but with different non-zero values provided by `values`.
    pub fn decompose_left_looking(&mut self, values: &[T]) -> bool {
        assert!(
            values.len() >= self.original_i.len(),
            "The set of values is too small."
        );

        let n = self.l.nrows();

        // Reset `work_c` to the column pointers of `l`.
        self.work_c.copy_from(&self.l.data.p);

        unsafe {
            for k in 0..n {
                // Scatter the k-th column of the original matrix with the values provided.
                let range_k =
                    *self.original_p.get_unchecked(k)..*self.original_p.get_unchecked(k + 1);

                *self.work_x.vget_unchecked_mut(k) = T::zero();
                for p in range_k.clone() {
                    let irow = *self.original_i.get_unchecked(p);

                    if irow >= k {
                        *self.work_x.vget_unchecked_mut(irow) = values.get_unchecked(p).clone();
                    }
                }

                for j in self.u.data.column_row_indices(k) {
                    let factor = -self
                        .l
                        .data
                        .vals
                        .get_unchecked(*self.work_c.vget_unchecked(j))
                        .clone();
                    *self.work_c.vget_unchecked_mut(j) += 1;

                    if j < k {
                        for (z, val) in self.l.data.column_entries(j) {
                            if z >= k {
                                *self.work_x.vget_unchecked_mut(z) += val * factor.clone();
                            }
                        }
                    }
                }

                let diag = self.work_x.vget_unchecked(k).clone();

                if diag > T::zero() {
                    let denom = diag.sqrt();
                    *self
                        .l
                        .data
                        .vals
                        .get_unchecked_mut(*self.l.data.p.vget_unchecked(k)) = denom.clone();

                    for (p, val) in self.l.data.column_entries_mut(k) {
                        *val = self.work_x.vget_unchecked(p).clone() / denom.clone();
                        *self.work_x.vget_unchecked_mut(p) = T::zero();
                    }
                } else {
                    self.ok = false;
                    return false;
                }
            }
        }

        self.ok = true;
        true
    }

    /// Perform a numerical up-looking cholesky decomposition of a matrix with the same structure as the
    /// one used to initialize `self`, but with different non-zero values provided by `values`.
    pub fn decompose_up_looking(&mut self, values: &[T]) -> bool {
        assert!(
            values.len() >= self.original_i.len(),
            "The set of values is too small."
        );

        // Reset `work_c` to the column pointers of `l`.
        self.work_c.copy_from(&self.l.data.p);

        // Perform the decomposition.
        for k in 0..self.l.nrows() {
            unsafe {
                // Scatter the k-th column of the original matrix with the values provided.
                let column_range =
                    *self.original_p.get_unchecked(k)..*self.original_p.get_unchecked(k + 1);

                *self.work_x.vget_unchecked_mut(k) = T::zero();
                for p in column_range.clone() {
                    let irow = *self.original_i.get_unchecked(p);

                    if irow <= k {
                        *self.work_x.vget_unchecked_mut(irow) = values.get_unchecked(p).clone();
                    }
                }

                let mut diag = self.work_x.vget_unchecked(k).clone();
                *self.work_x.vget_unchecked_mut(k) = T::zero();

                // Triangular solve.
                for irow in self.u.data.column_row_indices(k) {
                    if irow >= k {
                        continue;
                    }

                    let lki = self.work_x.vget_unchecked(irow).clone()
                        / self
                            .l
                            .data
                            .vals
                            .get_unchecked(*self.l.data.p.vget_unchecked(irow))
                            .clone();
                    *self.work_x.vget_unchecked_mut(irow) = T::zero();

                    for p in
                        *self.l.data.p.vget_unchecked(irow) + 1..*self.work_c.vget_unchecked(irow)
                    {
                        *self
                            .work_x
                            .vget_unchecked_mut(*self.l.data.i.get_unchecked(p)) -=
                            self.l.data.vals.get_unchecked(p).clone() * lki.clone();
                    }

                    diag -= lki.clone() * lki.clone();
                    let p = *self.work_c.vget_unchecked(irow);
                    *self.work_c.vget_unchecked_mut(irow) += 1;
                    *self.l.data.i.get_unchecked_mut(p) = k;
                    *self.l.data.vals.get_unchecked_mut(p) = lki;
                }

                if diag <= T::zero() {
                    self.ok = false;
                    return false;
                }

                // Deal with the diagonal element.
                let p = *self.work_c.vget_unchecked(k);
                *self.work_c.vget_unchecked_mut(k) += 1;
                *self.l.data.i.get_unchecked_mut(p) = k;
                *self.l.data.vals.get_unchecked_mut(p) = diag.sqrt();
            }
        }

        self.ok = true;
        true
    }

    fn elimination_tree<S: CsStorage<T, D, D>>(m: &CsMatrix<T, D, D, S>) -> Vec<usize> {
        let nrows = m.nrows();
        let mut forest: Vec<_> = iter::repeat(usize::MAX).take(nrows).collect();
        let mut ancestor: Vec<_> = iter::repeat(usize::MAX).take(nrows).collect();

        for k in 0..nrows {
            for irow in m.data.column_row_indices(k) {
                let mut i = irow;

                while i < k {
                    let i_ancestor = ancestor[i];
                    ancestor[i] = k;

                    if i_ancestor == usize::MAX {
                        forest[i] = k;
                        break;
                    }

                    i = i_ancestor;
                }
            }
        }

        forest
    }

    fn reach<S: CsStorage<T, D, D>>(
        m: &CsMatrix<T, D, D, S>,
        j: usize,
        max_j: usize,
        tree: &[usize],
        marks: &mut Vec<bool>,
        out: &mut Vec<usize>,
    ) {
        marks.clear();
        marks.resize(tree.len(), false);

        // TODO: avoid all those allocations.
        let mut tmp = Vec::new();
        let mut res = Vec::new();

        for irow in m.data.column_row_indices(j) {
            let mut curr = irow;
            while curr != usize::MAX && curr <= max_j && !marks[curr] {
                marks[curr] = true;
                tmp.push(curr);
                curr = tree[curr];
            }

            tmp.append(&mut res);
            mem::swap(&mut tmp, &mut res);
        }

        out.append(&mut res);
    }

    fn nonzero_pattern<S: CsStorage<T, D, D>>(
        m: &CsMatrix<T, D, D, S>,
    ) -> (CsMatrix<T, D, D>, CsMatrix<T, D, D>) {
        let etree = Self::elimination_tree(m);
        let (nrows, ncols) = m.data.shape();
        let mut rows = Vec::with_capacity(m.len());
        let mut cols = Matrix::zeros_generic(m.data.shape().0, Const::<1>);
        let mut marks = Vec::new();

        // NOTE: the following will actually compute the non-zero pattern of
        // the transpose of l.
        for i in 0..nrows.value() {
            cols[i] = rows.len();
            Self::reach(m, i, i, &etree, &mut marks, &mut rows);
        }

        let mut vals = Vec::with_capacity(rows.len());
        unsafe {
            vals.set_len(rows.len());
        }
        vals.shrink_to_fit();

        let data = CsVecStorage {
            shape: (nrows, ncols),
            p: cols,
            i: rows,
            vals,
        };

        let u = CsMatrix::from_data(data);
        // XXX: avoid this transpose.
        let l = u.transpose();

        (l, u)
    }

    /*
     *
     * NOTE: All the following methods are untested and currently unused.
     *
     *
    fn column_counts<S: CsStorage<T, D, D>>(
        m: &CsMatrix<T, D, D, S>,
        tree: &[usize],
    ) -> Vec<usize> {
        let len = m.data.shape().0.value();
        let mut counts: Vec<_> = iter::repeat(0).take(len).collect();
        let mut reach = Vec::new();
        let mut marks = Vec::new();

        for i in 0..len {
            Self::reach(m, i, i, tree, &mut marks, &mut reach);

            for j in reach.drain(..) {
                counts[j] += 1;
            }
        }

        counts
    }

    fn tree_postorder(tree: &[usize]) -> Vec<usize> {
        // TODO: avoid all those allocations?
        let mut first_child: Vec<_> = iter::repeat(usize::MAX).take(tree.len()).collect();
        let mut other_children: Vec<_> =
            iter::repeat(usize::MAX).take(tree.len()).collect();

        // Build the children list from the parent list.
        // The set of children of the node `i` is given by the linked list
        // starting at `first_child[i]`. The nodes of this list are then:
        // { first_child[i], other_children[first_child[i]], other_children[other_children[first_child[i]], ... }
        for (i, parent) in tree.iter().enumerate() {
            if *parent != usize::MAX {
                let brother = first_child[*parent];
                first_child[*parent] = i;
                other_children[i] = brother;
            }
        }

        let mut stack = Vec::with_capacity(tree.len());
        let mut postorder = Vec::with_capacity(tree.len());

        for (i, node) in tree.iter().enumerate() {
            if *node == usize::MAX {
                Self::dfs(
                    i,
                    &mut first_child,
                    &other_children,
                    &mut stack,
                    &mut postorder,
                )
            }
        }

        postorder
    }

    fn dfs(
        i: usize,
        first_child: &mut [usize],
        other_children: &[usize],
        stack: &mut Vec<usize>,
        result: &mut Vec<usize>,
    ) {
        stack.clear();
        stack.push(i);

        while let Some(n) = stack.pop() {
            let child = first_child[n];

            if child == usize::MAX {
                // No children left.
                result.push(n);
            } else {
                stack.push(n);
                stack.push(child);
                first_child[n] = other_children[child];
            }
        }
    }
    */
}
