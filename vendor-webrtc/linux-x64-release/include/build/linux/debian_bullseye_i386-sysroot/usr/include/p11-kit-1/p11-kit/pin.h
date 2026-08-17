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

#ifndef P11_KIT_PIN_H
#define P11_KIT_PIN_H

#include <p11-kit/uri.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct p11_kit_pin P11KitPin;

typedef enum {
	P11_KIT_PIN_FLAGS_USER_LOGIN = 1<<0,
	P11_KIT_PIN_FLAGS_SO_LOGIN = 1<<1,
	P11_KIT_PIN_FLAGS_CONTEXT_LOGIN = 1<<2,
	P11_KIT_PIN_FLAGS_RETRY = 1<<3,
	P11_KIT_PIN_FLAGS_MANY_TRIES = 1<<4,
	P11_KIT_PIN_FLAGS_FINAL_TRY = 1<<5
} P11KitPinFlags;

#define P11_KIT_PIN_FALLBACK ""

typedef void        (*p11_kit_pin_destroy_func)             (void *data);

P11KitPin*            p11_kit_pin_new                       (const unsigned char *value,
                                                             size_t length);

P11KitPin*            p11_kit_pin_new_for_string            (const char *value);

P11KitPin*            p11_kit_pin_new_for_buffer            (unsigned char *buffer,
                                                             size_t length,
                                                             p11_kit_pin_destroy_func destroy);

P11KitPin*            p11_kit_pin_ref                       (P11KitPin *pin);

void                  p11_kit_pin_unref                     (P11KitPin *pin);

const unsigned char * p11_kit_pin_get_value                 (P11KitPin *pin,
                                                             size_t *length);

size_t                p11_kit_pin_get_length                (P11KitPin *pin);

typedef P11KitPin*  (*p11_kit_pin_callback)                 (const char *pin_source,
                                                             P11KitUri *pin_uri,
                                                             const char *pin_description,
                                                             P11KitPinFlags pin_flags,
                                                             void *callback_data);

int                   p11_kit_pin_register_callback         (const char *pin_source,
                                                             p11_kit_pin_callback callback,
                                                             void *callback_data,
                                                             p11_kit_pin_destroy_func callback_destroy);

void                  p11_kit_pin_unregister_callback       (const char *pin_source,
                                                             p11_kit_pin_callback callback,
                                                             void *callback_data);

P11KitPin*            p11_kit_pin_request                   (const char *pin_source,
                                                             P11KitUri *pin_uri,
                                                             const char *pin_description,
                                                             P11KitPinFlags pin_flags);

P11KitPin*            p11_kit_pin_file_callback             (const char *pin_source,
                                                             P11KitUri *pin_uri,
                                                             const char *pin_description,
                                                             P11KitPinFlags pin_flags,
                                                             void *callback_data);

#ifdef __cplusplus
} /* extern "C" */
#endif

#endif /* P11_KIT_URI_H */
