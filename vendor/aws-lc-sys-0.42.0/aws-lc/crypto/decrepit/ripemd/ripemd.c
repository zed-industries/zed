// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#include <openssl/ripemd.h>

#include <string.h>

#include "../../internal.h"
#include "../../fipsmodule/digest/md32_common.h"


#define RIPEMD160_A 0x67452301L
#define RIPEMD160_B 0xEFCDAB89L
#define RIPEMD160_C 0x98BADCFEL
#define RIPEMD160_D 0x10325476L
#define RIPEMD160_E 0xC3D2E1F0L

int RIPEMD160_Init(RIPEMD160_CTX *ctx) {
  OPENSSL_memset(ctx, 0, sizeof(*ctx));
  ctx->h[0] = RIPEMD160_A;
  ctx->h[1] = RIPEMD160_B;
  ctx->h[2] = RIPEMD160_C;
  ctx->h[3] = RIPEMD160_D;
  ctx->h[4] = RIPEMD160_E;
  return 1;
}

static void ripemd160_block_data_order(uint32_t h[5], const uint8_t *data,
                                       size_t num);

int RIPEMD160_Update(RIPEMD160_CTX *c, const void *data, size_t len) {
  crypto_md32_update(&ripemd160_block_data_order, c->h, c->data,
                     RIPEMD160_CBLOCK, &c->num, &c->Nh, &c->Nl, data, len);
  return 1;
}

int RIPEMD160_Final(uint8_t out[RIPEMD160_DIGEST_LENGTH], RIPEMD160_CTX *c) {
  crypto_md32_final(&ripemd160_block_data_order, c->h, c->data,
                    RIPEMD160_CBLOCK, &c->num, c->Nh, c->Nl,
                    /*is_big_endian=*/0);

  CRYPTO_store_u32_le(out, c->h[0]);
  CRYPTO_store_u32_le(out + 4, c->h[1]);
  CRYPTO_store_u32_le(out + 8, c->h[2]);
  CRYPTO_store_u32_le(out + 12, c->h[3]);
  CRYPTO_store_u32_le(out + 16, c->h[4]);
  return 1;
}

// Transformed F2 and F4 are courtesy of Wei Dai <weidai@eskimo.com>
#define F1(x, y, z) ((x) ^ (y) ^ (z))
#define F2(x, y, z) ((((y) ^ (z)) & (x)) ^ (z))
#define F3(x, y, z) (((~(y)) | (x)) ^ (z))
#define F4(x, y, z) ((((x) ^ (y)) & (z)) ^ (y))
#define F5(x, y, z) (((~(z)) | (y)) ^ (x))

#define RIP1(a, b, c, d, e, w, s)  \
  {                                \
    a += F1(b, c, d) + X(w);       \
    a = CRYPTO_rotl_u32(a, s) + e; \
    c = CRYPTO_rotl_u32(c, 10);    \
  }

#define RIP2(a, b, c, d, e, w, s, K) \
  {                                  \
    a += F2(b, c, d) + X(w) + K;     \
    a = CRYPTO_rotl_u32(a, s) + e;   \
    c = CRYPTO_rotl_u32(c, 10);      \
  }

#define RIP3(a, b, c, d, e, w, s, K) \
  {                                  \
    a += F3(b, c, d) + X(w) + K;     \
    a = CRYPTO_rotl_u32(a, s) + e;   \
    c = CRYPTO_rotl_u32(c, 10);      \
  }

#define RIP4(a, b, c, d, e, w, s, K) \
  {                                  \
    a += F4(b, c, d) + X(w) + K;     \
    a = CRYPTO_rotl_u32(a, s) + e;   \
    c = CRYPTO_rotl_u32(c, 10);      \
  }

#define RIP5(a, b, c, d, e, w, s, K) \
  {                                  \
    a += F5(b, c, d) + X(w) + K;     \
    a = CRYPTO_rotl_u32(a, s) + e;   \
    c = CRYPTO_rotl_u32(c, 10);      \
  }

#define KL0 0x00000000L
#define KL1 0x5A827999L
#define KL2 0x6ED9EBA1L
#define KL3 0x8F1BBCDCL
#define KL4 0xA953FD4EL

#define KR0 0x50A28BE6L
#define KR1 0x5C4DD124L
#define KR2 0x6D703EF3L
#define KR3 0x7A6D76E9L
#define KR4 0x00000000L

#define WL00  0
#define SL00 11
#define WL01  1
#define SL01 14
#define WL02  2
#define SL02 15
#define WL03  3
#define SL03 12
#define WL04  4
#define SL04  5
#define WL05  5
#define SL05  8
#define WL06  6
#define SL06  7
#define WL07  7
#define SL07  9
#define WL08  8
#define SL08 11
#define WL09  9
#define SL09 13
#define WL10 10
#define SL10 14
#define WL11 11
#define SL11 15
#define WL12 12
#define SL12  6
#define WL13 13
#define SL13  7
#define WL14 14
#define SL14  9
#define WL15 15
#define SL15  8

#define WL16  7
#define SL16  7
#define WL17  4
#define SL17  6
#define WL18 13
#define SL18  8
#define WL19  1
#define SL19 13
#define WL20 10
#define SL20 11
#define WL21  6
#define SL21  9
#define WL22 15
#define SL22  7
#define WL23  3
#define SL23 15
#define WL24 12
#define SL24  7
#define WL25  0
#define SL25 12
#define WL26  9
#define SL26 15
#define WL27  5
#define SL27  9
#define WL28  2
#define SL28 11
#define WL29 14
#define SL29  7
#define WL30 11
#define SL30 13
#define WL31  8
#define SL31 12

#define WL32  3
#define SL32 11
#define WL33 10
#define SL33 13
#define WL34 14
#define SL34  6
#define WL35  4
#define SL35  7
#define WL36  9
#define SL36 14
#define WL37 15
#define SL37  9
#define WL38  8
#define SL38 13
#define WL39  1
#define SL39 15
#define WL40  2
#define SL40 14
#define WL41  7
#define SL41  8
#define WL42  0
#define SL42 13
#define WL43  6
#define SL43  6
#define WL44 13
#define SL44  5
#define WL45 11
#define SL45 12
#define WL46  5
#define SL46  7
#define WL47 12
#define SL47  5

#define WL48  1
#define SL48 11
#define WL49  9
#define SL49 12
#define WL50 11
#define SL50 14
#define WL51 10
#define SL51 15
#define WL52  0
#define SL52 14
#define WL53  8
#define SL53 15
#define WL54 12
#define SL54  9
#define WL55  4
#define SL55  8
#define WL56 13
#define SL56  9
#define WL57  3
#define SL57 14
#define WL58  7
#define SL58  5
#define WL59 15
#define SL59  6
#define WL60 14
#define SL60  8
#define WL61  5
#define SL61  6
#define WL62  6
#define SL62  5
#define WL63  2
#define SL63 12

#define WL64  4
#define SL64  9
#define WL65  0
#define SL65 15
#define WL66  5
#define SL66  5
#define WL67  9
#define SL67 11
#define WL68  7
#define SL68  6
#define WL69 12
#define SL69  8
#define WL70  2
#define SL70 13
#define WL71 10
#define SL71 12
#define WL72 14
#define SL72  5
#define WL73  1
#define SL73 12
#define WL74  3
#define SL74 13
#define WL75  8
#define SL75 14
#define WL76 11
#define SL76 11
#define WL77  6
#define SL77  8
#define WL78 15
#define SL78  5
#define WL79 13
#define SL79  6

#define WR00  5
#define SR00  8
#define WR01 14
#define SR01  9
#define WR02  7
#define SR02  9
#define WR03  0
#define SR03 11
#define WR04  9
#define SR04 13
#define WR05  2
#define SR05 15
#define WR06 11
#define SR06 15
#define WR07  4
#define SR07  5
#define WR08 13
#define SR08  7
#define WR09  6
#define SR09  7
#define WR10 15
#define SR10  8
#define WR11  8
#define SR11 11
#define WR12  1
#define SR12 14
#define WR13 10
#define SR13 14
#define WR14  3
#define SR14 12
#define WR15 12
#define SR15  6

#define WR16  6
#define SR16  9
#define WR17 11
#define SR17 13
#define WR18  3
#define SR18 15
#define WR19  7
#define SR19  7
#define WR20  0
#define SR20 12
#define WR21 13
#define SR21  8
#define WR22  5
#define SR22  9
#define WR23 10
#define SR23 11
#define WR24 14
#define SR24  7
#define WR25 15
#define SR25  7
#define WR26  8
#define SR26 12
#define WR27 12
#define SR27  7
#define WR28  4
#define SR28  6
#define WR29  9
#define SR29 15
#define WR30  1
#define SR30 13
#define WR31  2
#define SR31 11

#define WR32 15
#define SR32  9
#define WR33  5
#define SR33  7
#define WR34  1
#define SR34 15
#define WR35  3
#define SR35 11
#define WR36  7
#define SR36  8
#define WR37 14
#define SR37  6
#define WR38  6
#define SR38  6
#define WR39  9
#define SR39 14
#define WR40 11
#define SR40 12
#define WR41  8
#define SR41 13
#define WR42 12
#define SR42  5
#define WR43  2
#define SR43 14
#define WR44 10
#define SR44 13
#define WR45  0
#define SR45 13
#define WR46  4
#define SR46  7
#define WR47 13
#define SR47  5

#define WR48  8
#define SR48 15
#define WR49  6
#define SR49  5
#define WR50  4
#define SR50  8
#define WR51  1
#define SR51 11
#define WR52  3
#define SR52 14
#define WR53 11
#define SR53 14
#define WR54 15
#define SR54  6
#define WR55  0
#define SR55 14
#define WR56  5
#define SR56  6
#define WR57 12
#define SR57  9
#define WR58  2
#define SR58 12
#define WR59 13
#define SR59  9
#define WR60  9
#define SR60 12
#define WR61  7
#define SR61  5
#define WR62 10
#define SR62 15
#define WR63 14
#define SR63  8

#define WR64 12
#define SR64  8
#define WR65 15
#define SR65  5
#define WR66 10
#define SR66 12
#define WR67  4
#define SR67  9
#define WR68  1
#define SR68 12
#define WR69  5
#define SR69  5
#define WR70  8
#define SR70 14
#define WR71  7
#define SR71  6
#define WR72  6
#define SR72  8
#define WR73  2
#define SR73 13
#define WR74 13
#define SR74  6
#define WR75 14
#define SR75  5
#define WR76  0
#define SR76 15
#define WR77  3
#define SR77 13
#define WR78  9
#define SR78 11
#define WR79 11
#define SR79 11

static void ripemd160_block_data_order(uint32_t h[5], const uint8_t *data,
                                       size_t num) {
  uint32_t A, B, C, D, E;
  uint32_t a, b, c, d, e;
  uint32_t XX0, XX1, XX2, XX3, XX4, XX5, XX6, XX7, XX8, XX9, XX10, XX11, XX12,
      XX13, XX14, XX15;
#define X(i) XX##i

  for (; num--;) {
    A = h[0];
    B = h[1];
    C = h[2];
    D = h[3];
    E = h[4];

    X(0) = CRYPTO_load_u32_le(data);
    data += 4;
    X(1) = CRYPTO_load_u32_le(data);
    data += 4;
    RIP1(A, B, C, D, E, WL00, SL00);
    X(2) = CRYPTO_load_u32_le(data);
    data += 4;
    RIP1(E, A, B, C, D, WL01, SL01);
    X(3) = CRYPTO_load_u32_le(data);
    data += 4;
    RIP1(D, E, A, B, C, WL02, SL02);
    X(4) = CRYPTO_load_u32_le(data);
    data += 4;
    RIP1(C, D, E, A, B, WL03, SL03);
    X(5) = CRYPTO_load_u32_le(data);
    data += 4;
    RIP1(B, C, D, E, A, WL04, SL04);
    X(6) = CRYPTO_load_u32_le(data);
    data += 4;
    RIP1(A, B, C, D, E, WL05, SL05);
    X(7) = CRYPTO_load_u32_le(data);
    data += 4;
    RIP1(E, A, B, C, D, WL06, SL06);
    X(8) = CRYPTO_load_u32_le(data);
    data += 4;
    RIP1(D, E, A, B, C, WL07, SL07);
    X(9) = CRYPTO_load_u32_le(data);
    data += 4;
    RIP1(C, D, E, A, B, WL08, SL08);
    X(10) = CRYPTO_load_u32_le(data);
    data += 4;
    RIP1(B, C, D, E, A, WL09, SL09);
    X(11) = CRYPTO_load_u32_le(data);
    data += 4;
    RIP1(A, B, C, D, E, WL10, SL10);
    X(12) = CRYPTO_load_u32_le(data);
    data += 4;
    RIP1(E, A, B, C, D, WL11, SL11);
    X(13) = CRYPTO_load_u32_le(data);
    data += 4;
    RIP1(D, E, A, B, C, WL12, SL12);
    X(14) = CRYPTO_load_u32_le(data);
    data += 4;
    RIP1(C, D, E, A, B, WL13, SL13);
    X(15) = CRYPTO_load_u32_le(data);
    data += 4;
    RIP1(B, C, D, E, A, WL14, SL14);
    RIP1(A, B, C, D, E, WL15, SL15);

    RIP2(E, A, B, C, D, WL16, SL16, KL1);
    RIP2(D, E, A, B, C, WL17, SL17, KL1);
    RIP2(C, D, E, A, B, WL18, SL18, KL1);
    RIP2(B, C, D, E, A, WL19, SL19, KL1);
    RIP2(A, B, C, D, E, WL20, SL20, KL1);
    RIP2(E, A, B, C, D, WL21, SL21, KL1);
    RIP2(D, E, A, B, C, WL22, SL22, KL1);
    RIP2(C, D, E, A, B, WL23, SL23, KL1);
    RIP2(B, C, D, E, A, WL24, SL24, KL1);
    RIP2(A, B, C, D, E, WL25, SL25, KL1);
    RIP2(E, A, B, C, D, WL26, SL26, KL1);
    RIP2(D, E, A, B, C, WL27, SL27, KL1);
    RIP2(C, D, E, A, B, WL28, SL28, KL1);
    RIP2(B, C, D, E, A, WL29, SL29, KL1);
    RIP2(A, B, C, D, E, WL30, SL30, KL1);
    RIP2(E, A, B, C, D, WL31, SL31, KL1);

    RIP3(D, E, A, B, C, WL32, SL32, KL2);
    RIP3(C, D, E, A, B, WL33, SL33, KL2);
    RIP3(B, C, D, E, A, WL34, SL34, KL2);
    RIP3(A, B, C, D, E, WL35, SL35, KL2);
    RIP3(E, A, B, C, D, WL36, SL36, KL2);
    RIP3(D, E, A, B, C, WL37, SL37, KL2);
    RIP3(C, D, E, A, B, WL38, SL38, KL2);
    RIP3(B, C, D, E, A, WL39, SL39, KL2);
    RIP3(A, B, C, D, E, WL40, SL40, KL2);
    RIP3(E, A, B, C, D, WL41, SL41, KL2);
    RIP3(D, E, A, B, C, WL42, SL42, KL2);
    RIP3(C, D, E, A, B, WL43, SL43, KL2);
    RIP3(B, C, D, E, A, WL44, SL44, KL2);
    RIP3(A, B, C, D, E, WL45, SL45, KL2);
    RIP3(E, A, B, C, D, WL46, SL46, KL2);
    RIP3(D, E, A, B, C, WL47, SL47, KL2);

    RIP4(C, D, E, A, B, WL48, SL48, KL3);
    RIP4(B, C, D, E, A, WL49, SL49, KL3);
    RIP4(A, B, C, D, E, WL50, SL50, KL3);
    RIP4(E, A, B, C, D, WL51, SL51, KL3);
    RIP4(D, E, A, B, C, WL52, SL52, KL3);
    RIP4(C, D, E, A, B, WL53, SL53, KL3);
    RIP4(B, C, D, E, A, WL54, SL54, KL3);
    RIP4(A, B, C, D, E, WL55, SL55, KL3);
    RIP4(E, A, B, C, D, WL56, SL56, KL3);
    RIP4(D, E, A, B, C, WL57, SL57, KL3);
    RIP4(C, D, E, A, B, WL58, SL58, KL3);
    RIP4(B, C, D, E, A, WL59, SL59, KL3);
    RIP4(A, B, C, D, E, WL60, SL60, KL3);
    RIP4(E, A, B, C, D, WL61, SL61, KL3);
    RIP4(D, E, A, B, C, WL62, SL62, KL3);
    RIP4(C, D, E, A, B, WL63, SL63, KL3);

    RIP5(B, C, D, E, A, WL64, SL64, KL4);
    RIP5(A, B, C, D, E, WL65, SL65, KL4);
    RIP5(E, A, B, C, D, WL66, SL66, KL4);
    RIP5(D, E, A, B, C, WL67, SL67, KL4);
    RIP5(C, D, E, A, B, WL68, SL68, KL4);
    RIP5(B, C, D, E, A, WL69, SL69, KL4);
    RIP5(A, B, C, D, E, WL70, SL70, KL4);
    RIP5(E, A, B, C, D, WL71, SL71, KL4);
    RIP5(D, E, A, B, C, WL72, SL72, KL4);
    RIP5(C, D, E, A, B, WL73, SL73, KL4);
    RIP5(B, C, D, E, A, WL74, SL74, KL4);
    RIP5(A, B, C, D, E, WL75, SL75, KL4);
    RIP5(E, A, B, C, D, WL76, SL76, KL4);
    RIP5(D, E, A, B, C, WL77, SL77, KL4);
    RIP5(C, D, E, A, B, WL78, SL78, KL4);
    RIP5(B, C, D, E, A, WL79, SL79, KL4);

    a = A;
    b = B;
    c = C;
    d = D;
    e = E;
    // Do other half
    A = h[0];
    B = h[1];
    C = h[2];
    D = h[3];
    E = h[4];

    RIP5(A, B, C, D, E, WR00, SR00, KR0);
    RIP5(E, A, B, C, D, WR01, SR01, KR0);
    RIP5(D, E, A, B, C, WR02, SR02, KR0);
    RIP5(C, D, E, A, B, WR03, SR03, KR0);
    RIP5(B, C, D, E, A, WR04, SR04, KR0);
    RIP5(A, B, C, D, E, WR05, SR05, KR0);
    RIP5(E, A, B, C, D, WR06, SR06, KR0);
    RIP5(D, E, A, B, C, WR07, SR07, KR0);
    RIP5(C, D, E, A, B, WR08, SR08, KR0);
    RIP5(B, C, D, E, A, WR09, SR09, KR0);
    RIP5(A, B, C, D, E, WR10, SR10, KR0);
    RIP5(E, A, B, C, D, WR11, SR11, KR0);
    RIP5(D, E, A, B, C, WR12, SR12, KR0);
    RIP5(C, D, E, A, B, WR13, SR13, KR0);
    RIP5(B, C, D, E, A, WR14, SR14, KR0);
    RIP5(A, B, C, D, E, WR15, SR15, KR0);

    RIP4(E, A, B, C, D, WR16, SR16, KR1);
    RIP4(D, E, A, B, C, WR17, SR17, KR1);
    RIP4(C, D, E, A, B, WR18, SR18, KR1);
    RIP4(B, C, D, E, A, WR19, SR19, KR1);
    RIP4(A, B, C, D, E, WR20, SR20, KR1);
    RIP4(E, A, B, C, D, WR21, SR21, KR1);
    RIP4(D, E, A, B, C, WR22, SR22, KR1);
    RIP4(C, D, E, A, B, WR23, SR23, KR1);
    RIP4(B, C, D, E, A, WR24, SR24, KR1);
    RIP4(A, B, C, D, E, WR25, SR25, KR1);
    RIP4(E, A, B, C, D, WR26, SR26, KR1);
    RIP4(D, E, A, B, C, WR27, SR27, KR1);
    RIP4(C, D, E, A, B, WR28, SR28, KR1);
    RIP4(B, C, D, E, A, WR29, SR29, KR1);
    RIP4(A, B, C, D, E, WR30, SR30, KR1);
    RIP4(E, A, B, C, D, WR31, SR31, KR1);

    RIP3(D, E, A, B, C, WR32, SR32, KR2);
    RIP3(C, D, E, A, B, WR33, SR33, KR2);
    RIP3(B, C, D, E, A, WR34, SR34, KR2);
    RIP3(A, B, C, D, E, WR35, SR35, KR2);
    RIP3(E, A, B, C, D, WR36, SR36, KR2);
    RIP3(D, E, A, B, C, WR37, SR37, KR2);
    RIP3(C, D, E, A, B, WR38, SR38, KR2);
    RIP3(B, C, D, E, A, WR39, SR39, KR2);
    RIP3(A, B, C, D, E, WR40, SR40, KR2);
    RIP3(E, A, B, C, D, WR41, SR41, KR2);
    RIP3(D, E, A, B, C, WR42, SR42, KR2);
    RIP3(C, D, E, A, B, WR43, SR43, KR2);
    RIP3(B, C, D, E, A, WR44, SR44, KR2);
    RIP3(A, B, C, D, E, WR45, SR45, KR2);
    RIP3(E, A, B, C, D, WR46, SR46, KR2);
    RIP3(D, E, A, B, C, WR47, SR47, KR2);

    RIP2(C, D, E, A, B, WR48, SR48, KR3);
    RIP2(B, C, D, E, A, WR49, SR49, KR3);
    RIP2(A, B, C, D, E, WR50, SR50, KR3);
    RIP2(E, A, B, C, D, WR51, SR51, KR3);
    RIP2(D, E, A, B, C, WR52, SR52, KR3);
    RIP2(C, D, E, A, B, WR53, SR53, KR3);
    RIP2(B, C, D, E, A, WR54, SR54, KR3);
    RIP2(A, B, C, D, E, WR55, SR55, KR3);
    RIP2(E, A, B, C, D, WR56, SR56, KR3);
    RIP2(D, E, A, B, C, WR57, SR57, KR3);
    RIP2(C, D, E, A, B, WR58, SR58, KR3);
    RIP2(B, C, D, E, A, WR59, SR59, KR3);
    RIP2(A, B, C, D, E, WR60, SR60, KR3);
    RIP2(E, A, B, C, D, WR61, SR61, KR3);
    RIP2(D, E, A, B, C, WR62, SR62, KR3);
    RIP2(C, D, E, A, B, WR63, SR63, KR3);

    RIP1(B, C, D, E, A, WR64, SR64);
    RIP1(A, B, C, D, E, WR65, SR65);
    RIP1(E, A, B, C, D, WR66, SR66);
    RIP1(D, E, A, B, C, WR67, SR67);
    RIP1(C, D, E, A, B, WR68, SR68);
    RIP1(B, C, D, E, A, WR69, SR69);
    RIP1(A, B, C, D, E, WR70, SR70);
    RIP1(E, A, B, C, D, WR71, SR71);
    RIP1(D, E, A, B, C, WR72, SR72);
    RIP1(C, D, E, A, B, WR73, SR73);
    RIP1(B, C, D, E, A, WR74, SR74);
    RIP1(A, B, C, D, E, WR75, SR75);
    RIP1(E, A, B, C, D, WR76, SR76);
    RIP1(D, E, A, B, C, WR77, SR77);
    RIP1(C, D, E, A, B, WR78, SR78);
    RIP1(B, C, D, E, A, WR79, SR79);

    D = h[1] + c + D;
    h[1] = h[2] + d + E;
    h[2] = h[3] + e + A;
    h[3] = h[4] + a + B;
    h[4] = h[0] + b + C;
    h[0] = D;
  }

#undef X
}

uint8_t *RIPEMD160(const uint8_t *data, size_t len,
                   uint8_t out[RIPEMD160_DIGEST_LENGTH]) {
  RIPEMD160_CTX ctx;

  if (!RIPEMD160_Init(&ctx)) {
    return NULL;
  }

  RIPEMD160_Update(&ctx, data, len);
  RIPEMD160_Final(out, &ctx);
  return out;
}
