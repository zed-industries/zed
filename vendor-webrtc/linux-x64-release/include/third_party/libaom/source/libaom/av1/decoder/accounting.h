/*
 * Copyright (c) 2016, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */
#ifndef AOM_AV1_DECODER_ACCOUNTING_H_
#define AOM_AV1_DECODER_ACCOUNTING_H_
#include <stdlib.h>
#include "aom/aomdx.h"

#ifdef __cplusplus
extern "C" {
#endif  // __cplusplus

#define AOM_ACCOUNTING_HASH_SIZE (1021)

/* Max number of entries for symbol types in the dictionary (increase as
   necessary). */
#define MAX_SYMBOL_TYPES (256)

/*The resolution of fractional-precision bit usage measurements, i.e.,
   3 => 1/8th bits.*/
#define AOM_ACCT_BITRES (3)

typedef struct {
  int16_t x;
  int16_t y;
} AccountingSymbolContext;

typedef struct {
  AccountingSymbolContext context;
  uint32_t id;
  /** Number of bits in units of 1/8 bit. */
  uint32_t bits;
  uint32_t samples;
} AccountingSymbol;

/** Dictionary for translating strings into id. */
typedef struct {
  char *strs[MAX_SYMBOL_TYPES];
  int num_strs;
} AccountingDictionary;

typedef struct {
  /** All recorded symbols decoded. */
  AccountingSymbol *syms;
  /** Number of syntax actually recorded. */
  int num_syms;
  /** Raw symbol decoding calls for non-binary values. */
  int num_multi_syms;
  /** Raw binary symbol decoding calls. */
  int num_binary_syms;
  /** Dictionary for translating strings into id. */
  AccountingDictionary dictionary;
} AccountingSymbols;

struct Accounting {
  AccountingSymbols syms;
  /** Size allocated for symbols (not all may be used). */
  int num_syms_allocated;
  int16_t hash_dictionary[AOM_ACCOUNTING_HASH_SIZE];
  AccountingSymbolContext context;
  uint32_t last_tell_frac;
};

void aom_accounting_init(Accounting *accounting);
void aom_accounting_reset(Accounting *accounting);
void aom_accounting_clear(Accounting *accounting);
void aom_accounting_set_context(Accounting *accounting, int16_t x, int16_t y);
int aom_accounting_dictionary_lookup(Accounting *accounting, const char *str);
void aom_accounting_record(Accounting *accounting, const char *str,
                           uint32_t bits);
void aom_accounting_dump(Accounting *accounting);
#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
#endif  // AOM_AV1_DECODER_ACCOUNTING_H_
