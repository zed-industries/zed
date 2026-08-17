/*
 * wrappers.h - wrappers to modify output of MPFR/MPC test functions
 *
 * Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
 * See https://llvm.org/LICENSE.txt for license information.
 * SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
 */

typedef struct {
    /* Structure type should be considered opaque outside wrappers.c,
     * though we have to define it here so its size is known. */
    int nops;
    int nresults;
    mpfr_srcptr mpfr_ops[2];
    mpfr_ptr mpfr_result;
    mpc_srcptr mpc_ops[2];
    mpc_ptr mpc_result;
    const uint32 *ieee_ops[2];
    uint32 *ieee_result;
    int size_ops[2];
    int size_result;
    int need_regen;
} wrapperctx;

typedef void (*wrapperfunc)(wrapperctx *ctx);
#define MAXWRAPPERS 3

/*
 * Functions for the test harness to call.
 *
 * When the test harness executes a test function, it should
 * initialise a wrapperctx with wrapper_init, then provide all the
 * operands and results in both mpfr/mpc and IEEE (+ extrabits)
 * formats via wrapper_op_* and wrapper_result_*. Then it should run
 * the function's wrappers using wrapper_run(), and if that returns
 * true then the primary result has been rewritten in mpfr/mpc format
 * and it should therefore retranslate into IEEE.
 *
 * 'size' in all prototypes below represents an FP type by giving the
 * number of 32-bit words it requires, so 1=float and 2=double. Input
 * operands will be that many words (or that many for both their real
 * and imag parts); outputs will have one extra word for 'extrabits'.
 *
 * This system only applies at all to reference functions using
 * mpfr/mpc. The seminumerical functions we implement in pure IEEE
 * form are expected to handle all their own special cases correctly.
 */

void wrapper_init(wrapperctx *ctx);

/* Real operand. */
void wrapper_op_real(wrapperctx *ctx, const mpfr_t r,
                     int size, const uint32 *ieee);

/* Complex operand. Real part starts at ieee[0], the imag part at ieee[2]. */
void wrapper_op_complex(wrapperctx *ctx, const mpc_t c,
                        int size, const uint32 *ieee);

/* Real result. ieee contains size+1 words, as discussed above. */
void wrapper_result_real(wrapperctx *ctx, mpfr_t r,
                         int size, uint32 *ieee);

/* Complex result. ieee contains size+1 words of real part starting at
 * ieee[0], and another size+1 of imag part starting at ieee[4]. */
void wrapper_result_complex(wrapperctx *ctx, mpc_t c,
                            int size, uint32 *ieee);

int wrapper_run(wrapperctx *ctx, wrapperfunc wrappers[MAXWRAPPERS]);

/*
 * Functions for wrappers to call. 'op' indicates which operand is
 * being requested: 0,1 means first and second, and -1 means the
 * result.
 */

mpfr_srcptr wrapper_get_mpfr(wrapperctx *ctx, int op);
const uint32 *wrapper_get_ieee(wrapperctx *ctx, int op);

mpc_srcptr wrapper_get_mpc(wrapperctx *ctx, int op);
mpfr_srcptr wrapper_get_mpfr_r(wrapperctx *ctx, int op);
mpfr_srcptr wrapper_get_mpfr_i(wrapperctx *ctx, int op);
const uint32 *wrapper_get_ieee_r(wrapperctx *ctx, int op);
const uint32 *wrapper_get_ieee_i(wrapperctx *ctx, int op);

/* Query operand count + types */
int wrapper_get_nops(wrapperctx *ctx);
int wrapper_get_size(wrapperctx *ctx, int op);
int wrapper_is_complex(wrapperctx *ctx, int op);

/* Change just the sign of the result. Only the top bit of 'sign' is used. */
void wrapper_set_sign(wrapperctx *ctx, uint32 sign);
void wrapper_set_sign_r(wrapperctx *ctx, uint32 sign);
void wrapper_set_sign_i(wrapperctx *ctx, uint32 sign);

/* Set a result to NaN. */
void wrapper_set_nan(wrapperctx *ctx);
void wrapper_set_nan_r(wrapperctx *ctx);
void wrapper_set_nan_i(wrapperctx *ctx);

/* Set a result to an integer value (converted to the appropriate
 * float format). */
void wrapper_set_int(wrapperctx *ctx, int val);
void wrapper_set_int_r(wrapperctx *ctx, int val);
void wrapper_set_int_i(wrapperctx *ctx, int val);

/* Set a result to a new MPFR float. */
void wrapper_set_mpfr(wrapperctx *ctx, const mpfr_t val);
void wrapper_set_mpfr_r(wrapperctx *ctx, const mpfr_t val);
void wrapper_set_mpfr_i(wrapperctx *ctx, const mpfr_t val);

/*
 * A universal wrapper called for _all_ functions, that doesn't have
 * to be specified individually everywhere.
 */
void universal_wrapper(wrapperctx *ctx);
