/*
 * HTTP authentication
 * Copyright (c) 2010 Martin Storsjo
 *
 * This file is part of FFmpeg.
 *
 * FFmpeg is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * FFmpeg is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with FFmpeg; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
 */

#ifndef AVFORMAT_HTTPAUTH_H
#define AVFORMAT_HTTPAUTH_H

/**
 * Authentication types, ordered from weakest to strongest.
 */
typedef enum HTTPAuthType {
    HTTP_AUTH_NONE = 0,    /**< No authentication specified */
    HTTP_AUTH_BASIC,       /**< HTTP 1.0 Basic auth from RFC 1945
                             *  (also in RFC 2617) */
    HTTP_AUTH_DIGEST,      /**< HTTP 1.1 Digest auth from RFC 2617 */
} HTTPAuthType;

typedef struct DigestParams {
    char nonce[300];       /**< Server specified nonce */
    char algorithm[10];    /**< Server specified digest algorithm */
    char qop[30];          /**< Quality of protection, containing the one
                             *  that we've chosen to use, from the
                             *  alternatives that the server offered. */
    char opaque[300];      /**< A server-specified string that should be
                             *  included in authentication responses, not
                             *  included in the actual digest calculation. */
    char stale[10];        /**< The server indicated that the auth was ok,
                             * but needs to be redone with a new, non-stale
                             * nonce. */
    int nc;                /**< Nonce count, the number of earlier replies
                             *  where this particular nonce has been used. */
} DigestParams;

/**
 * HTTP Authentication state structure. Must be zero-initialized
 * before used with the functions below.
 */
typedef struct HTTPAuthState {
    /**
     * The currently chosen auth type.
     */
    int auth_type;
    /**
     * Authentication realm
     */
    char realm[200];
    /**
     * The parameters specific to digest authentication.
     */
    DigestParams digest_params;
    /**
     * Auth ok, but needs to be resent with a new nonce.
     */
    int stale;
} HTTPAuthState;

void ff_http_auth_handle_header(HTTPAuthState *state, const char *key,
                                const char *value);
char *ff_http_auth_create_response(HTTPAuthState *state, const char *auth,
                                   const char *path, const char *method);

#endif /* AVFORMAT_HTTPAUTH_H */
