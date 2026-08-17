/*
 * Copyright (c) 2017, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_TOOLS_OBU_PARSER_H_
#define AOM_TOOLS_OBU_PARSER_H_

#include <cstdint>

namespace aom_tools {

// Print information obtained from OBU(s) in data until data is exhausted or an
// error occurs. Returns true when all data is consumed successfully, and
// optionally reports OBU storage overhead via obu_overhead_bytes when the
// pointer is non-null.
bool DumpObu(const uint8_t *data, int length, int *obu_overhead_bytes);

}  // namespace aom_tools

#endif  // AOM_TOOLS_OBU_PARSER_H_
