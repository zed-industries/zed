/* SPDX-License-Identifier: LGPL-2.1-or-later */
#ifndef foosdid128hfoo
#define foosdid128hfoo

/***
  systemd is free software; you can redistribute it and/or modify it
  under the terms of the GNU Lesser General Public License as published by
  the Free Software Foundation; either version 2.1 of the License, or
  (at your option) any later version.

  systemd is distributed in the hope that it will be useful, but
  WITHOUT ANY WARRANTY; without even the implied warranty of
  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
  Lesser General Public License for more details.

  You should have received a copy of the GNU Lesser General Public License
  along with systemd; If not, see <https://www.gnu.org/licenses/>.
***/

#include <inttypes.h>
#include <stdarg.h>
#include <string.h>

#include "_sd-common.h"

_SD_BEGIN_DECLARATIONS;

/* 128-bit ID APIs. See sd-id128(3) for more information. */

typedef union sd_id128 sd_id128_t;

union sd_id128 {
        uint8_t bytes[16];
        uint64_t qwords[2];
};

#define SD_ID128_STRING_MAX 33U
#define SD_ID128_UUID_STRING_MAX 37U

char *sd_id128_to_string(sd_id128_t id, char s[_SD_ARRAY_STATIC SD_ID128_STRING_MAX]);
char *sd_id128_to_uuid_string(sd_id128_t id, char s[_SD_ARRAY_STATIC SD_ID128_UUID_STRING_MAX]);
int sd_id128_from_string(const char *s, sd_id128_t *ret);

#define SD_ID128_TO_STRING(id) sd_id128_to_string((id), (char[SD_ID128_STRING_MAX]) {})
#define SD_ID128_TO_UUID_STRING(id) sd_id128_to_uuid_string((id), (char[SD_ID128_UUID_STRING_MAX]) {})

int sd_id128_randomize(sd_id128_t *ret);

int sd_id128_get_machine(sd_id128_t *ret);
int sd_id128_get_boot(sd_id128_t *ret);
int sd_id128_get_invocation(sd_id128_t *ret);

int sd_id128_get_machine_app_specific(sd_id128_t app_id, sd_id128_t *ret);
int sd_id128_get_boot_app_specific(sd_id128_t app_id, sd_id128_t *ret);

#define SD_ID128_ARRAY(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15) \
        { .bytes = { 0x##v0, 0x##v1, 0x##v2, 0x##v3, 0x##v4, 0x##v5, 0x##v6, 0x##v7, \
                     0x##v8, 0x##v9, 0x##v10, 0x##v11, 0x##v12, 0x##v13, 0x##v14, 0x##v15 }}

#define SD_ID128_MAKE(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15) \
        ((const sd_id128_t) SD_ID128_ARRAY(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15))

/* Note that SD_ID128_FORMAT_VAL will evaluate the passed argument 16
 * times. It is hence not a good idea to call this macro with an
 * expensive function as parameter or an expression with side
 * effects */

#define SD_ID128_FORMAT_STR "%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x"
#define SD_ID128_FORMAT_VAL(x) (x).bytes[0], (x).bytes[1], (x).bytes[2], (x).bytes[3], (x).bytes[4], (x).bytes[5], (x).bytes[6], (x).bytes[7], (x).bytes[8], (x).bytes[9], (x).bytes[10], (x).bytes[11], (x).bytes[12], (x).bytes[13], (x).bytes[14], (x).bytes[15]

/* Like SD_ID128_FORMAT_STR, but formats as UUID, not in plain format (Strictly Big Endian byte order,
 * i.e. treats everything as RFC4122 Variant 1 UUIDs, even if variant says otherwise, but matching other
 * Linux userspace behaviour.) */
#define SD_ID128_UUID_FORMAT_STR "%02x%02x%02x%02x-%02x%02x-%02x%02x-%02x%02x-%02x%02x%02x%02x%02x%02x"

#define SD_ID128_CONST_STR(x)                                           \
        ((const char[SD_ID128_STRING_MAX]) {                            \
                ((x).bytes[0] >> 4) >= 10 ? 'a' + ((x).bytes[0] >> 4) - 10 : '0' + ((x).bytes[0] >> 4), \
                ((x).bytes[0] & 15) >= 10 ? 'a' + ((x).bytes[0] & 15) - 10 : '0' + ((x).bytes[0] & 15), \
                ((x).bytes[1] >> 4) >= 10 ? 'a' + ((x).bytes[1] >> 4) - 10 : '0' + ((x).bytes[1] >> 4), \
                ((x).bytes[1] & 15) >= 10 ? 'a' + ((x).bytes[1] & 15) - 10 : '0' + ((x).bytes[1] & 15), \
                ((x).bytes[2] >> 4) >= 10 ? 'a' + ((x).bytes[2] >> 4) - 10 : '0' + ((x).bytes[2] >> 4), \
                ((x).bytes[2] & 15) >= 10 ? 'a' + ((x).bytes[2] & 15) - 10 : '0' + ((x).bytes[2] & 15), \
                ((x).bytes[3] >> 4) >= 10 ? 'a' + ((x).bytes[3] >> 4) - 10 : '0' + ((x).bytes[3] >> 4), \
                ((x).bytes[3] & 15) >= 10 ? 'a' + ((x).bytes[3] & 15) - 10 : '0' + ((x).bytes[3] & 15), \
                ((x).bytes[4] >> 4) >= 10 ? 'a' + ((x).bytes[4] >> 4) - 10 : '0' + ((x).bytes[4] >> 4), \
                ((x).bytes[4] & 15) >= 10 ? 'a' + ((x).bytes[4] & 15) - 10 : '0' + ((x).bytes[4] & 15), \
                ((x).bytes[5] >> 4) >= 10 ? 'a' + ((x).bytes[5] >> 4) - 10 : '0' + ((x).bytes[5] >> 4), \
                ((x).bytes[5] & 15) >= 10 ? 'a' + ((x).bytes[5] & 15) - 10 : '0' + ((x).bytes[5] & 15), \
                ((x).bytes[6] >> 4) >= 10 ? 'a' + ((x).bytes[6] >> 4) - 10 : '0' + ((x).bytes[6] >> 4), \
                ((x).bytes[6] & 15) >= 10 ? 'a' + ((x).bytes[6] & 15) - 10 : '0' + ((x).bytes[6] & 15), \
                ((x).bytes[7] >> 4) >= 10 ? 'a' + ((x).bytes[7] >> 4) - 10 : '0' + ((x).bytes[7] >> 4), \
                ((x).bytes[7] & 15) >= 10 ? 'a' + ((x).bytes[7] & 15) - 10 : '0' + ((x).bytes[7] & 15), \
                ((x).bytes[8] >> 4) >= 10 ? 'a' + ((x).bytes[8] >> 4) - 10 : '0' + ((x).bytes[8] >> 4), \
                ((x).bytes[8] & 15) >= 10 ? 'a' + ((x).bytes[8] & 15) - 10 : '0' + ((x).bytes[8] & 15), \
                ((x).bytes[9] >> 4) >= 10 ? 'a' + ((x).bytes[9] >> 4) - 10 : '0' + ((x).bytes[9] >> 4), \
                ((x).bytes[9] & 15) >= 10 ? 'a' + ((x).bytes[9] & 15) - 10 : '0' + ((x).bytes[9] & 15), \
                ((x).bytes[10] >> 4) >= 10 ? 'a' + ((x).bytes[10] >> 4) - 10 : '0' + ((x).bytes[10] >> 4), \
                ((x).bytes[10] & 15) >= 10 ? 'a' + ((x).bytes[10] & 15) - 10 : '0' + ((x).bytes[10] & 15), \
                ((x).bytes[11] >> 4) >= 10 ? 'a' + ((x).bytes[11] >> 4) - 10 : '0' + ((x).bytes[11] >> 4), \
                ((x).bytes[11] & 15) >= 10 ? 'a' + ((x).bytes[11] & 15) - 10 : '0' + ((x).bytes[11] & 15), \
                ((x).bytes[12] >> 4) >= 10 ? 'a' + ((x).bytes[12] >> 4) - 10 : '0' + ((x).bytes[12] >> 4), \
                ((x).bytes[12] & 15) >= 10 ? 'a' + ((x).bytes[12] & 15) - 10 : '0' + ((x).bytes[12] & 15), \
                ((x).bytes[13] >> 4) >= 10 ? 'a' + ((x).bytes[13] >> 4) - 10 : '0' + ((x).bytes[13] >> 4), \
                ((x).bytes[13] & 15) >= 10 ? 'a' + ((x).bytes[13] & 15) - 10 : '0' + ((x).bytes[13] & 15), \
                ((x).bytes[14] >> 4) >= 10 ? 'a' + ((x).bytes[14] >> 4) - 10 : '0' + ((x).bytes[14] >> 4), \
                ((x).bytes[14] & 15) >= 10 ? 'a' + ((x).bytes[14] & 15) - 10 : '0' + ((x).bytes[14] & 15), \
                ((x).bytes[15] >> 4) >= 10 ? 'a' + ((x).bytes[15] >> 4) - 10 : '0' + ((x).bytes[15] >> 4), \
                ((x).bytes[15] & 15) >= 10 ? 'a' + ((x).bytes[15] & 15) - 10 : '0' + ((x).bytes[15] & 15), \
                0 })

#define SD_ID128_MAKE_STR(a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p) \
        #a #b #c #d #e #f #g #h #i #j #k #l #m #n #o #p

#define SD_ID128_MAKE_UUID_STR(a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p) \
        #a #b #c #d "-" #e #f "-" #g #h "-" #i #j "-" #k #l #m #n #o #p

_sd_pure_ static __inline__ int sd_id128_equal(sd_id128_t a, sd_id128_t b) {
        return a.qwords[0] == b.qwords[0] && a.qwords[1] == b.qwords[1];
}

int sd_id128_string_equal(const char *s, sd_id128_t id);

_sd_pure_ static __inline__ int sd_id128_is_null(sd_id128_t a) {
        return a.qwords[0] == 0 && a.qwords[1] == 0;
}

_sd_pure_ static __inline__ int sd_id128_is_allf(sd_id128_t a) {
        return a.qwords[0] == UINT64_C(0xFFFFFFFFFFFFFFFF) && a.qwords[1] == UINT64_C(0xFFFFFFFFFFFFFFFF);
}

#define SD_ID128_NULL ((const sd_id128_t) { .qwords = { 0, 0 }})
#define SD_ID128_ALLF ((const sd_id128_t) { .qwords = { UINT64_C(0xFFFFFFFFFFFFFFFF), UINT64_C(0xFFFFFFFFFFFFFFFF) }})

_sd_pure_ static __inline__ int sd_id128_in_setv(sd_id128_t a, va_list ap) {
        for (;;) {
                sd_id128_t b = va_arg(ap, sd_id128_t);

                if (sd_id128_is_null(b))
                        return 0;

                if (sd_id128_equal(a, b))
                        return 1;
        }
}

_sd_pure_ static __inline__ int sd_id128_in_set_sentinel(sd_id128_t a, ...) {
        va_list ap;
        int r;

        va_start(ap, a);
        r = sd_id128_in_setv(a, ap);
        va_end(ap);

        return r;
}

#define sd_id128_in_set(a, ...) \
        sd_id128_in_set_sentinel(a, ##__VA_ARGS__, SD_ID128_NULL)

_SD_END_DECLARATIONS;

#endif
