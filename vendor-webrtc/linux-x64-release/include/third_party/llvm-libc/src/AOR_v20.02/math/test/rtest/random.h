/*
 * random.h - header for random.c
 *
 * Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
 * See https://llvm.org/LICENSE.txt for license information.
 * SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
 */

#include "types.h"

uint32 random32(void);
uint32 random_upto(uint32 limit);
uint32 random_upto_biased(uint32 limit, int bias);
