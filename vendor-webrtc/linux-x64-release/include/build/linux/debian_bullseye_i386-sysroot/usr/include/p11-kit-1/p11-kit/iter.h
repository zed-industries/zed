/*
 * Copyright (c) 2013,2016 Red Hat, Inc
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

#ifndef P11_KIT_ITER_H
#define P11_KIT_ITER_H

#include "p11-kit/p11-kit.h"
#include "p11-kit/pkcs11.h"
#include "p11-kit/uri.h"

#ifdef __cplusplus
extern "C" {
#endif

#ifdef P11_KIT_FUTURE_UNSTABLE_API

/*
 * If the caller is using the PKCS#11 GNU calling convention, then we cater
 * to that here.
 */
#ifdef CRYPTOKI_GNU
typedef unsigned char CK_BBOOL;
typedef ck_object_handle_t CK_OBJECT_HANDLE;
typedef ck_session_handle_t CK_SESSION_HANDLE;
#endif

typedef struct p11_kit_iter P11KitIter;
typedef P11KitIter p11_kit_iter;

typedef enum {
	P11_KIT_ITER_KIND_MODULE,
	P11_KIT_ITER_KIND_SLOT,
	P11_KIT_ITER_KIND_TOKEN,
	P11_KIT_ITER_KIND_OBJECT,
	P11_KIT_ITER_KIND_UNKNOWN = -1,
} P11KitIterKind;

typedef enum {
	P11_KIT_ITER_BUSY_SESSIONS = 1 << 1,
	P11_KIT_ITER_WANT_WRITABLE = 1 << 2,
	P11_KIT_ITER_WITH_MODULES = 1 << 3,
	P11_KIT_ITER_WITH_SLOTS = 1 << 4,
	P11_KIT_ITER_WITH_TOKENS = 1 << 5,
	P11_KIT_ITER_WITHOUT_OBJECTS = 1 << 6,
} P11KitIterBehavior;

typedef CK_RV      (* p11_kit_iter_callback)                (P11KitIter *iter,
                                                             CK_BBOOL *matches,
                                                             void *data);

P11KitIter *          p11_kit_iter_new                      (P11KitUri *uri,
                                                             P11KitIterBehavior behavior);

void                  p11_kit_iter_free                     (P11KitIter *iter);

void                  p11_kit_iter_add_callback             (P11KitIter *iter,
                                                             p11_kit_iter_callback callback,
                                                             void *callback_data,
                                                             p11_kit_destroyer callback_destroy);

void                  p11_kit_iter_add_filter               (P11KitIter *iter,
                                                             CK_ATTRIBUTE *matching,
                                                             CK_ULONG count);

void                  p11_kit_iter_set_uri                  (P11KitIter *iter,
                                                             P11KitUri *uri);

void                  p11_kit_iter_begin                    (P11KitIter *iter,
                                                             CK_FUNCTION_LIST_PTR *modules);

void                  p11_kit_iter_begin_with               (P11KitIter *iter,
                                                             CK_FUNCTION_LIST_PTR module,
                                                             CK_SLOT_ID slot,
                                                             CK_SESSION_HANDLE session);

CK_RV                 p11_kit_iter_next                     (P11KitIter *iter);

P11KitIterKind        p11_kit_iter_get_kind                 (P11KitIter *iter);

CK_FUNCTION_LIST_PTR  p11_kit_iter_get_module               (P11KitIter *iter);

CK_SLOT_ID            p11_kit_iter_get_slot                 (P11KitIter *iter);

CK_SLOT_INFO *        p11_kit_iter_get_slot_info            (P11KitIter *iter);

CK_TOKEN_INFO *       p11_kit_iter_get_token                (P11KitIter *iter);

CK_SESSION_HANDLE     p11_kit_iter_get_session              (P11KitIter *iter);

CK_OBJECT_HANDLE      p11_kit_iter_get_object               (P11KitIter *iter);

CK_RV                 p11_kit_iter_get_attributes           (P11KitIter *iter,
                                                             CK_ATTRIBUTE *template,
                                                             CK_ULONG count);

CK_RV                 p11_kit_iter_load_attributes          (P11KitIter *iter,
                                                             CK_ATTRIBUTE *template,
                                                             CK_ULONG count);

CK_SESSION_HANDLE     p11_kit_iter_keep_session             (P11KitIter *iter);

CK_RV                 p11_kit_iter_destroy_object           (P11KitIter *iter);

#endif /* P11_KIT_FUTURE_UNSTABLE_API */

#ifdef __cplusplus
} /* extern "C" */
#endif

#endif /* P11_KIT_ITER_H */
