/*
 * Copyright (c) 2021, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_AV1_ENCODER_SPARSE_LINEAR_SOLVER_H_
#define AOM_AV1_ENCODER_SPARSE_LINEAR_SOLVER_H_

#ifdef __cplusplus
extern "C" {
#endif

#include "config/aom_config.h"

#if CONFIG_OPTICAL_FLOW_API

// Number of iterations for solving linear equations.
#define MAX_CG_SP_ITER 100

typedef struct {
  int n_elem;  // number of non-zero elements
  int n_rows;
  int n_cols;
  // using arrays to represent non-zero elements.
  int *col_pos;
  int *row_pos;  // starts with 0
  double *value;
} SPARSE_MTX;

int av1_init_sparse_mtx(const int *rows, const int *cols, const double *values,
                        int num_elem, int num_rows, int num_cols,
                        SPARSE_MTX *sm);
int av1_init_combine_sparse_mtx(const SPARSE_MTX *sm1, const SPARSE_MTX *sm2,
                                SPARSE_MTX *sm, int row_offset1,
                                int col_offset1, int row_offset2,
                                int col_offset2, int new_n_rows,
                                int new_n_cols);
void av1_free_sparse_mtx_elems(SPARSE_MTX *sm);

void av1_mtx_vect_multi_right(const SPARSE_MTX *sm, const double *srcv,
                              double *dstv, int dstl);
void av1_mtx_vect_multi_left(const SPARSE_MTX *sm, const double *srcv,
                             double *dstv, int dstl);
double av1_vect_vect_multi(const double *src1, int src1l, const double *src2);
void av1_constant_multiply_sparse_matrix(SPARSE_MTX *sm, double c);

int av1_conjugate_gradient_sparse(const SPARSE_MTX *A, const double *b, int bl,
                                  double *x);
int av1_bi_conjugate_gradient_sparse(const SPARSE_MTX *A, const double *b,
                                     int bl, double *x);
int av1_jacobi_sparse(const SPARSE_MTX *A, const double *b, int bl, double *x);
int av1_steepest_descent_sparse(const SPARSE_MTX *A, const double *b, int bl,
                                double *x);

#endif  // CONFIG_OPTICAL_FLOW_API

#ifdef __cplusplus
}  // extern "C"
#endif

#endif /* AOM_AV1_ENCODER_SPARSE_LINEAR_SOLVER_H_ */
