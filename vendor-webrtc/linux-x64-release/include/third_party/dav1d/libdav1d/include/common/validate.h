/*
 * Copyright © 2018, VideoLAN and dav1d authors
 * Copyright © 2018, Two Orioles, LLC
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice, this
 *    list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR
 * ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef DAV1D_COMMON_VALIDATE_H
#define DAV1D_COMMON_VALIDATE_H

#include <stdio.h>
#include <stdlib.h>

#if defined(NDEBUG)
#define debug_print(...) do {} while (0)
#define debug_abort() do {} while (0)
#else
#define debug_print(...) fprintf(stderr, __VA_ARGS__)
#define debug_abort abort
#endif

#define validate_input_or_ret_with_msg(x, r, ...) \
    if (!(x)) { \
        debug_print("Input validation check \'%s\' failed in %s!\n", \
                    #x, __func__); \
        debug_print(__VA_ARGS__); \
        debug_abort(); \
        return r; \
    }

#define validate_input_or_ret(x, r) \
    if (!(x)) { \
        debug_print("Input validation check \'%s\' failed in %s!\n", \
                    #x, __func__); \
        debug_abort(); \
        return r; \
    }

#define validate_input(x) validate_input_or_ret(x, )

#endif /* DAV1D_COMMON_VALIDATE_H */
