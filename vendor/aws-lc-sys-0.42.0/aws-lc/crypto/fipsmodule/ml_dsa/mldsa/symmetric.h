/*
 * Copyright (c) The mldsa-native project authors
 * SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT
 */
#ifndef MLD_SYMMETRIC_H
#define MLD_SYMMETRIC_H

#include "cbmc.h"
#include "common.h"

#include MLD_FIPS202_HEADER_FILE
#if !defined(MLD_CONFIG_SERIAL_FIPS202_ONLY)
#include MLD_FIPS202X4_HEADER_FILE
#endif

#define MLD_STREAM128_BLOCKBYTES SHAKE128_RATE
#define MLD_STREAM256_BLOCKBYTES SHAKE256_RATE

#define mld_xof256_ctx mld_shake256ctx
#define mld_xof256_init(CTX) mld_shake256_init(CTX)

#define mld_xof256_absorb_once(CTX, IN, INBYTES) \
  do                                             \
  {                                              \
    mld_shake256_absorb(CTX, IN, INBYTES);       \
    mld_shake256_finalize(CTX);                  \
  } while (0)


#define mld_xof256_release(CTX) mld_shake256_release(CTX)
#define mld_xof256_squeezeblocks(OUT, OUTBLOCKS, STATE) \
  mld_shake256_squeeze(OUT, (OUTBLOCKS) * SHAKE256_RATE, STATE)

#define mld_xof128_ctx mld_shake128ctx
#define mld_xof128_init(CTX) mld_shake128_init(CTX)

#define mld_xof128_absorb_once(CTX, IN, INBYTES) \
  do                                             \
  {                                              \
    mld_shake128_absorb(CTX, IN, INBYTES);       \
    mld_shake128_finalize(CTX);                  \
  } while (0)

#define mld_xof128_release(CTX) mld_shake128_release(CTX)
#define mld_xof128_squeezeblocks(OUT, OUTBLOCKS, STATE) \
  mld_shake128_squeeze(OUT, (OUTBLOCKS) * SHAKE128_RATE, STATE)

#define mld_xof256_x4_ctx mld_shake256x4ctx
#define mld_xof256_x4_init(CTX) mld_shake256x4_init((CTX))
#define mld_xof256_x4_absorb(CTX, IN, INBYTES)                          \
  mld_shake256x4_absorb_once((CTX), (IN)[0], (IN)[1], (IN)[2], (IN)[3], \
                             (INBYTES))
#define mld_xof256_x4_squeezeblocks(BUF, NBLOCKS, CTX)                 \
  mld_shake256x4_squeezeblocks((BUF)[0], (BUF)[1], (BUF)[2], (BUF)[3], \
                               (NBLOCKS), (CTX))
#define mld_xof256_x4_release(CTX) mld_shake256x4_release((CTX))

#define mld_xof128_x4_ctx mld_shake128x4ctx
#define mld_xof128_x4_init(CTX) mld_shake128x4_init((CTX))
#define mld_xof128_x4_absorb(CTX, IN, INBYTES)                          \
  mld_shake128x4_absorb_once((CTX), (IN)[0], (IN)[1], (IN)[2], (IN)[3], \
                             (INBYTES))
#define mld_xof128_x4_squeezeblocks(BUF, NBLOCKS, CTX)                 \
  mld_shake128x4_squeezeblocks((BUF)[0], (BUF)[1], (BUF)[2], (BUF)[3], \
                               (NBLOCKS), (CTX))
#define mld_xof128_x4_release(CTX) mld_shake128x4_release((CTX))

#endif /* !MLD_SYMMETRIC_H */
