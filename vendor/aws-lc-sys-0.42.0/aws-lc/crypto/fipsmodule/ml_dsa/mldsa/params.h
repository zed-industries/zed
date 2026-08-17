/*
 * Copyright (c) The mldsa-native project authors
 * SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT
 */
#ifndef MLD_PARAMS_H
#define MLD_PARAMS_H

#define MLDSA_SEEDBYTES 32
#define MLDSA_CRHBYTES 64
#define MLDSA_TRBYTES 64
#define MLDSA_RNDBYTES 32
#define MLDSA_N 256
#define MLDSA_Q 8380417
#define MLDSA_Q_HALF ((MLDSA_Q + 1) / 2)
#define MLDSA_D 13

#define MLDSA_GAMMA2_88 ((MLDSA_Q - 1) / 88)
#define MLDSA_GAMMA2_32 ((MLDSA_Q - 1) / 32)
#define MLDSA_POLYW1_PACKEDBYTES_88 192
#define MLDSA_POLYW1_PACKEDBYTES_32 128

#if MLD_CONFIG_PARAMETER_SET == 44

#define MLDSA_K 4
#define MLDSA_L 4
#define MLDSA_ETA 2
#define MLDSA_TAU 39
#define MLDSA_BETA 78
#define MLDSA_GAMMA1 (1 << 17)
#define MLDSA_GAMMA2 MLDSA_GAMMA2_88
#define MLDSA_OMEGA 80
#define MLDSA_CTILDEBYTES 32
#define MLDSA_POLYZ_PACKEDBYTES 576
#define MLDSA_POLYW1_PACKEDBYTES MLDSA_POLYW1_PACKEDBYTES_88
#define MLDSA_POLYETA_PACKEDBYTES 96

#elif MLD_CONFIG_PARAMETER_SET == 65

#define MLDSA_K 6
#define MLDSA_L 5
#define MLDSA_ETA 4
#define MLDSA_TAU 49
#define MLDSA_BETA 196
#define MLDSA_GAMMA1 (1 << 19)
#define MLDSA_GAMMA2 MLDSA_GAMMA2_32
#define MLDSA_OMEGA 55
#define MLDSA_CTILDEBYTES 48
#define MLDSA_POLYZ_PACKEDBYTES 640
#define MLDSA_POLYW1_PACKEDBYTES MLDSA_POLYW1_PACKEDBYTES_32
#define MLDSA_POLYETA_PACKEDBYTES 128

#elif MLD_CONFIG_PARAMETER_SET == 87

#define MLDSA_K 8
#define MLDSA_L 7
#define MLDSA_ETA 2
#define MLDSA_TAU 60
#define MLDSA_BETA 120
#define MLDSA_GAMMA1 (1 << 19)
#define MLDSA_GAMMA2 MLDSA_GAMMA2_32
#define MLDSA_OMEGA 75
#define MLDSA_CTILDEBYTES 64
#define MLDSA_POLYZ_PACKEDBYTES 640
#define MLDSA_POLYW1_PACKEDBYTES MLDSA_POLYW1_PACKEDBYTES_32
#define MLDSA_POLYETA_PACKEDBYTES 96

#endif /* MLD_CONFIG_PARAMETER_SET == 87 */

#define MLDSA_POLYT1_PACKEDBYTES 320
#define MLDSA_POLYT0_PACKEDBYTES 416
#define MLDSA_POLYVECH_PACKEDBYTES (MLDSA_OMEGA + MLDSA_K)

/* Layout of the packed public key pk[MLDSA_CRYPTO_PUBLICKEYBYTES] = (rho, t1):
 *
 *   +-------------+--------------------------+
 *   | rho         | t1                       |
 *   +-------------+--------------------------+
 *   | SEEDBYTES   | K * POLYT1_PACKEDBYTES   |
 *   +-------------+--------------------------+
 */
#define MLDSA_PK_RHO_OFFSET 0
#define MLDSA_PK_RHO_BYTES MLDSA_SEEDBYTES

#define MLDSA_PK_T1_OFFSET (MLDSA_PK_RHO_OFFSET + MLDSA_PK_RHO_BYTES)
#define MLDSA_PK_T1_BYTES (MLDSA_K * MLDSA_POLYT1_PACKEDBYTES)

#define MLDSA_PK_END (MLDSA_PK_T1_OFFSET + MLDSA_PK_T1_BYTES)

#define MLDSA_CRYPTO_PUBLICKEYBYTES MLDSA_PK_END

/* Layout of the packed secret key
 *  sk[MLDSA_CRYPTO_SECRETKEYBYTES] = (rho, key, tr, s1, s2, t0):
 *
 *   +-----------+-----------+-----------+-----------+-----------+-----------+
 *   | rho       | key       | tr        | s1        | s2        | t0        |
 *   +-----------+-----------+-----------+-----------+-----------+-----------+
 *   | SEEDBYTES | SEEDBYTES | TRBYTES   | L *       | K *       | K *       |
 *   |           |           |           | POLYETA_  | POLYETA_  | POLYT0_   |
 *   |           |           |           | PACKED-   | PACKED-   | PACKED-   |
 *   |           |           |           | BYTES     | BYTES     | BYTES     |
 *   +-----------+-----------+-----------+-----------+-----------+-----------+
 */
#define MLDSA_SK_RHO_OFFSET 0
#define MLDSA_SK_RHO_BYTES MLDSA_SEEDBYTES

#define MLDSA_SK_KEY_OFFSET (MLDSA_SK_RHO_OFFSET + MLDSA_SK_RHO_BYTES)
#define MLDSA_SK_KEY_BYTES MLDSA_SEEDBYTES

#define MLDSA_SK_TR_OFFSET (MLDSA_SK_KEY_OFFSET + MLDSA_SK_KEY_BYTES)
#define MLDSA_SK_TR_BYTES MLDSA_TRBYTES

#define MLDSA_SK_S1_OFFSET (MLDSA_SK_TR_OFFSET + MLDSA_SK_TR_BYTES)
#define MLDSA_SK_S1_BYTES (MLDSA_L * MLDSA_POLYETA_PACKEDBYTES)

#define MLDSA_SK_S2_OFFSET (MLDSA_SK_S1_OFFSET + MLDSA_SK_S1_BYTES)
#define MLDSA_SK_S2_BYTES (MLDSA_K * MLDSA_POLYETA_PACKEDBYTES)

#define MLDSA_SK_T0_OFFSET (MLDSA_SK_S2_OFFSET + MLDSA_SK_S2_BYTES)
#define MLDSA_SK_T0_BYTES (MLDSA_K * MLDSA_POLYT0_PACKEDBYTES)

#define MLDSA_SK_END (MLDSA_SK_T0_OFFSET + MLDSA_SK_T0_BYTES)

#define MLDSA_CRYPTO_SECRETKEYBYTES MLDSA_SK_END

/* Layout of the packed signature sig[MLDSA_CRYPTO_BYTES] = (c, z, h):
 *
 *   +----------------+-------------------+----------------------+
 *   | c (challenge)  | z                 | h (hints)            |
 *   +----------------+-------------------+----------------------+
 *   | CTILDEBYTES    | L *               | POLYVECH_PACKEDBYTES |
 *   |                | POLYZ_PACKEDBYTES | (= OMEGA + K)        |
 *   +----------------+-------------------+----------------------+
 */
#define MLDSA_SIG_C_OFFSET 0
#define MLDSA_SIG_C_BYTES MLDSA_CTILDEBYTES

#define MLDSA_SIG_Z_OFFSET (MLDSA_SIG_C_OFFSET + MLDSA_SIG_C_BYTES)
#define MLDSA_SIG_Z_BYTES (MLDSA_L * MLDSA_POLYZ_PACKEDBYTES)

#define MLDSA_SIG_H_OFFSET (MLDSA_SIG_Z_OFFSET + MLDSA_SIG_Z_BYTES)
#define MLDSA_SIG_H_BYTES MLDSA_POLYVECH_PACKEDBYTES

#define MLDSA_SIG_END (MLDSA_SIG_H_OFFSET + MLDSA_SIG_H_BYTES)

#define MLDSA_CRYPTO_BYTES MLDSA_SIG_END

#endif /* !MLD_PARAMS_H */
