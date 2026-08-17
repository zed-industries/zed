/*
 * Copyright (C) 2024 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_ETM_OPENCSD_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_ETM_OPENCSD_H_

// This file should be used instead of the librarys <opencsd.h>. This limits the
// number of includes and in particular avoids <opencsd/ptm/ptm_decoder.h> that
// has some inline functions that throw exceptions. This messes up the build
// quite a bit on GCC and since the file is not needed we just not include it.

#include <opencsd/ocsd_if_types.h>       // IWYU pragma: export
#include <opencsd/trc_gen_elem_types.h>  // IWYU pragma: export
#include <opencsd/trc_pkt_types.h>       // IWYU pragma: export

#include <interfaces/trc_data_raw_in_i.h>       // IWYU pragma: export
#include <interfaces/trc_data_rawframe_in_i.h>  // IWYU pragma: export
#include <interfaces/trc_error_log_i.h>         // IWYU pragma: export
#include <interfaces/trc_gen_elem_in_i.h>       // IWYU pragma: export
#include <interfaces/trc_instr_decode_i.h>      // IWYU pragma: export
#include <interfaces/trc_pkt_in_i.h>            // IWYU pragma: export
#include <interfaces/trc_pkt_raw_in_i.h>        // IWYU pragma: export
#include <interfaces/trc_tgt_mem_access_i.h>    // IWYU pragma: export

#include <common/ocsd_error.h>             // IWYU pragma: export
#include <common/ocsd_version.h>           // IWYU pragma: export
#include <common/trc_core_arch_map.h>      // IWYU pragma: export
#include <common/trc_frame_deformatter.h>  // IWYU pragma: export
#include <common/trc_gen_elem.h>           // IWYU pragma: export

#include <opencsd/ete/ete_decoder.h>      // IWYU pragma: export
#include <opencsd/etmv4/etmv4_decoder.h>  // IWYU pragma: export

#include <common/ocsd_error_logger.h>  // IWYU pragma: export
#include <common/ocsd_msg_logger.h>    // IWYU pragma: export
#include <i_dec/trc_i_decode.h>        // IWYU pragma: export
#include <mem_acc/trc_mem_acc.h>       // IWYU pragma: export

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_ETM_OPENCSD_H_
