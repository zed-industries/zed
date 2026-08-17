/*
 * Copyright (c) 2011 Collabora Ltd.
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
 * Author: Stef Walter <stefw@collabora.co.uk>
 */

#ifndef P11_KIT_URI_H
#define P11_KIT_URI_H

#include "p11-kit/pkcs11.h"

#ifdef __cplusplus
extern "C" {
#endif

#define P11_KIT_URI_SCHEME "pkcs11"
#define P11_KIT_URI_SCHEME_LEN 6

typedef enum {
	P11_KIT_URI_OK = 0,
	P11_KIT_URI_UNEXPECTED = -1,
	P11_KIT_URI_BAD_SCHEME = -2,
	P11_KIT_URI_BAD_ENCODING = -3,
	P11_KIT_URI_BAD_SYNTAX = -4,
	P11_KIT_URI_BAD_VERSION = -5,
	P11_KIT_URI_NOT_FOUND = -6,
} P11KitUriResult;

#define P11_KIT_URI_NO_MEMORY P11_KIT_URI_UNEXPECTED

typedef enum {
	P11_KIT_URI_FOR_OBJECT =  (1 << 1),
	P11_KIT_URI_FOR_TOKEN =   (1 << 2),
	P11_KIT_URI_FOR_SLOT =    (1 << 5),
	P11_KIT_URI_FOR_MODULE =  (1 << 3),

	P11_KIT_URI_FOR_MODULE_WITH_VERSION =
		(1 << 4) | P11_KIT_URI_FOR_MODULE,

	P11_KIT_URI_FOR_OBJECT_ON_TOKEN =
		P11_KIT_URI_FOR_OBJECT | P11_KIT_URI_FOR_TOKEN,

	P11_KIT_URI_FOR_OBJECT_ON_TOKEN_AND_MODULE =
		P11_KIT_URI_FOR_OBJECT_ON_TOKEN | P11_KIT_URI_FOR_MODULE,

	P11_KIT_URI_FOR_ANY =     0x0000FFFF,
} P11KitUriType;

/*
 * If the caller is using the PKCS#11 GNU calling convention, then we cater
 * to that here.
 */
#ifdef CRYPTOKI_GNU
typedef struct ck_info CK_INFO;
typedef struct ck_info *CK_INFO_PTR;
typedef struct ck_token_info CK_TOKEN_INFO;
typedef struct ck_token_info *CK_TOKEN_INFO_PTR;
typedef ck_attribute_type_t CK_ATTRIBUTE_TYPE;
typedef struct ck_attribute CK_ATTRIBUTE;
typedef struct ck_attribute *CK_ATTRIBUTE_PTR;
typedef unsigned long int CK_ULONG;
typedef struct ck_slot_info CK_SLOT_INFO;
typedef struct ck_slot_info *CK_SLOT_INFO_PTR;
typedef ck_slot_id_t CK_SLOT_ID;
typedef P11KitUriType p11_kit_uri_type_t;
typedef P11KitUriResult p11_kit_uri_result_t;
#endif

typedef struct p11_kit_uri P11KitUri;
typedef struct p11_kit_uri p11_kit_uri;

CK_INFO_PTR         p11_kit_uri_get_module_info             (P11KitUri *uri);

int                 p11_kit_uri_match_module_info           (const P11KitUri *uri,
                                                             const CK_INFO *info);

CK_SLOT_INFO_PTR    p11_kit_uri_get_slot_info               (P11KitUri *uri);

int                 p11_kit_uri_match_slot_info             (const P11KitUri *uri,
                                                             const CK_SLOT_INFO *slot_info);

CK_SLOT_ID          p11_kit_uri_get_slot_id                 (P11KitUri *uri);
void                p11_kit_uri_set_slot_id                 (P11KitUri *uri,
                                                             CK_SLOT_ID slot_id);

CK_TOKEN_INFO_PTR   p11_kit_uri_get_token_info              (P11KitUri *uri);

int                 p11_kit_uri_match_token_info            (const P11KitUri *uri,
                                                             const CK_TOKEN_INFO *token_info);

CK_ATTRIBUTE_PTR    p11_kit_uri_get_attribute               (P11KitUri *uri,
                                                             CK_ATTRIBUTE_TYPE attr_type);

int                 p11_kit_uri_set_attribute               (P11KitUri *uri,
                                                             CK_ATTRIBUTE_PTR attr);

int                 p11_kit_uri_clear_attribute             (P11KitUri *uri,
                                                             CK_ATTRIBUTE_TYPE attr_type);

CK_ATTRIBUTE_PTR    p11_kit_uri_get_attributes              (P11KitUri *uri,
                                                             CK_ULONG *n_attrs);

int                 p11_kit_uri_set_attributes              (P11KitUri *uri,
                                                             CK_ATTRIBUTE_PTR attrs,
                                                             CK_ULONG n_attrs);

void                p11_kit_uri_clear_attributes            (P11KitUri *uri);

int                 p11_kit_uri_match_attributes            (const P11KitUri *uri,
                                                             const CK_ATTRIBUTE *attrs,
                                                             CK_ULONG n_attrs);

const char*         p11_kit_uri_get_pin_value               (const P11KitUri *uri);

void                p11_kit_uri_set_pin_value               (P11KitUri *uri,
                                                             const char *pin);

const char*         p11_kit_uri_get_pin_source              (const P11KitUri *uri);

void                p11_kit_uri_set_pin_source              (P11KitUri *uri,
                                                             const char *pin_source);

#ifndef P11_KIT_DISABLE_DEPRECATED

const char*         p11_kit_uri_get_pinfile                 (const P11KitUri *uri);

void                p11_kit_uri_set_pinfile                 (P11KitUri *uri,
                                                             const char *pinfile);

#endif /* P11_KIT_DISABLE_DEPRECATED */

const char*         p11_kit_uri_get_module_name             (const P11KitUri *uri);

void                p11_kit_uri_set_module_name             (P11KitUri *uri,
                                                             const char *name);

const char*         p11_kit_uri_get_module_path             (const P11KitUri *uri);

void                p11_kit_uri_set_module_path             (P11KitUri *uri,
                                                             const char *path);

const char*         p11_kit_uri_get_vendor_query            (const P11KitUri *uri,
                                                             const char *name);
int                 p11_kit_uri_set_vendor_query            (P11KitUri *uri,
                                                             const char *name,
                                                             const char *value);

void                p11_kit_uri_set_unrecognized            (P11KitUri *uri,
                                                             int unrecognized);

int                 p11_kit_uri_any_unrecognized            (P11KitUri *uri);

P11KitUri*          p11_kit_uri_new                         (void);

int                 p11_kit_uri_format                      (P11KitUri *uri,
                                                             P11KitUriType uri_type,
                                                             char **string);

int                 p11_kit_uri_parse                       (const char *string,
                                                             P11KitUriType uri_type,
                                                             P11KitUri *uri);

void                p11_kit_uri_free                        (P11KitUri *uri);

const char*         p11_kit_uri_message                     (int code);

#ifdef __cplusplus
} /* extern "C" */
#endif

#endif /* P11_KIT_URI_H */
