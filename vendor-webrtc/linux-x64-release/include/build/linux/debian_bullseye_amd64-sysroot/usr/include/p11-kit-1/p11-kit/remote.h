/*
 * Copyright (c) 2014 Red Hat Inc.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 *     * Redistributions of source code must retain the above
 *       copyright notice, this list of conditions and the
 *       following disclaimer.
 *     * Redistributions in binary form must reproduce the
 *       above copyright notice, this list of conditions and
 *       the following disclaimer in the documentation and/or
 *       other materials provided with the distribution.
 *     * The names of contributors to this software may not be
 *       used to endorse or promote products derived from this
 *       software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
 * FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
 * COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT,
 * INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING,
 * BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS
 * OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED
 * AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
 * OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF
 * THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH
 * DAMAGE.
 *
 * Author: Stef Walter <stefw@redhat.com>
 */

#ifndef __P11_KIT_REMOTE_H__
#define __P11_KIT_REMOTE_H__

#include "p11-kit/p11-kit.h"

#ifdef __cplusplus
extern "C" {
#endif

#ifdef P11_KIT_FUTURE_UNSTABLE_API

int                    p11_kit_remote_serve_module          (CK_FUNCTION_LIST *module,
							     int in_fd,
							     int out_fd);

#ifndef P11_KIT_DISABLE_DEPRECATED

int		       p11_kit_remote_serve_token	    (CK_FUNCTION_LIST *module,
							     CK_TOKEN_INFO *token,
							     int in_fd,
							     int out_fd);

#endif /* P11_KIT_DISABLE_DEPRECATED */

int                    p11_kit_remote_serve_tokens          (const char **tokens,
							     size_t n_tokens,
							     CK_FUNCTION_LIST *provider,
							     int in_fd,
							     int out_fd);

#endif

#ifdef __cplusplus
} /* extern "C" */
#endif

#endif /* __P11_KIT_REMOTE_H__ */
