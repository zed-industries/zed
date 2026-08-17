/* PipeWire
 *
 * Copyright © 2018 Wim Taymans
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice (including the next
 * paragraph) shall be included in all copies or substantial portions of the
 * Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 */

#ifndef PIPEWIRE_PROPERTIES_H
#define PIPEWIRE_PROPERTIES_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdarg.h>

#include <spa/utils/dict.h>
#include <spa/utils/string.h>

/** \defgroup pw_properties Properties
 *
 * Properties are used to pass around arbitrary key/value pairs.
 * Both keys and values are strings which keeps things simple.
 * Encoding of arbitrary values should be done by using a string
 * serialization such as base64 for binary blobs.
 */

/**
 * \addtogroup pw_properties
 * \{
 */
struct pw_properties {
	struct spa_dict dict;	/**< dictionary of key/values */
	uint32_t flags;		/**< extra flags */
};

struct pw_properties *
pw_properties_new(const char *key, ...) SPA_SENTINEL;

struct pw_properties *
pw_properties_new_dict(const struct spa_dict *dict);

struct pw_properties *
pw_properties_new_string(const char *args);

struct pw_properties *
pw_properties_copy(const struct pw_properties *properties);

int pw_properties_update_keys(struct pw_properties *props,
		     const struct spa_dict *dict, const char * const keys[]);
int pw_properties_update_ignore(struct pw_properties *props,
		const struct spa_dict *dict, const char * const ignore[]);

/* Update props with all key/value pairs from dict */
int pw_properties_update(struct pw_properties *props,
		     const struct spa_dict *dict);
/* Update props with all key/value pairs from str */
int pw_properties_update_string(struct pw_properties *props,
		const char *str, size_t size);

int pw_properties_add(struct pw_properties *oldprops,
		     const struct spa_dict *dict);
int pw_properties_add_keys(struct pw_properties *oldprops,
		     const struct spa_dict *dict, const char * const keys[]);

void pw_properties_clear(struct pw_properties *properties);

void
pw_properties_free(struct pw_properties *properties);

int
pw_properties_set(struct pw_properties *properties, const char *key, const char *value);

int
pw_properties_setf(struct pw_properties *properties,
		   const char *key, const char *format, ...) SPA_PRINTF_FUNC(3, 4);
int
pw_properties_setva(struct pw_properties *properties,
		   const char *key, const char *format, va_list args) SPA_PRINTF_FUNC(3,0);
const char *
pw_properties_get(const struct pw_properties *properties, const char *key);

int
pw_properties_fetch_uint32(const struct pw_properties *properties, const char *key, uint32_t *value);

int
pw_properties_fetch_int32(const struct pw_properties *properties, const char *key, int32_t *value);

int
pw_properties_fetch_uint64(const struct pw_properties *properties, const char *key, uint64_t *value);

int
pw_properties_fetch_int64(const struct pw_properties *properties, const char *key, int64_t *value);

int
pw_properties_fetch_bool(const struct pw_properties *properties, const char *key, bool *value);

static inline uint32_t
pw_properties_get_uint32(const struct pw_properties *properties, const char *key, uint32_t deflt)
{
	uint32_t val = deflt;
	pw_properties_fetch_uint32(properties, key, &val);
	return val;
}

static inline int32_t
pw_properties_get_int32(const struct pw_properties *properties, const char *key, int32_t deflt)
{
	int32_t val = deflt;
	pw_properties_fetch_int32(properties, key, &val);
	return val;
}

static inline uint64_t
pw_properties_get_uint64(const struct pw_properties *properties, const char *key, uint64_t deflt)
{
	uint64_t val = deflt;
	pw_properties_fetch_uint64(properties, key, &val);
	return val;
}

static inline int64_t
pw_properties_get_int64(const struct pw_properties *properties, const char *key, int64_t deflt)
{
	int64_t val = deflt;
	pw_properties_fetch_int64(properties, key, &val);
	return val;
}


static inline bool
pw_properties_get_bool(const struct pw_properties *properties, const char *key, bool deflt)
{
	bool val = deflt;
	pw_properties_fetch_bool(properties, key, &val);
	return val;
}

const char *
pw_properties_iterate(const struct pw_properties *properties, void **state);

#define PW_PROPERTIES_FLAG_NL	(1<<0)
int pw_properties_serialize_dict(FILE *f, const struct spa_dict *dict, uint32_t flags);

static inline bool pw_properties_parse_bool(const char *value) {
	return spa_atob(value);
}

static inline int pw_properties_parse_int(const char *value) {
	int v;
	return spa_atoi32(value, &v, 0) ? v: 0;
}

static inline int64_t pw_properties_parse_int64(const char *value) {
	int64_t v;
	return spa_atoi64(value, &v, 0) ? v : 0;
}

static inline uint64_t pw_properties_parse_uint64(const char *value) {
	uint64_t v;
	return spa_atou64(value, &v, 0) ? v : 0;
}

static inline float pw_properties_parse_float(const char *value) {
	float v;
	return spa_atof(value, &v) ? v : 0.0f;
}

static inline double pw_properties_parse_double(const char *value) {
	double v;
	return spa_atod(value, &v) ? v : 0.0;
}

/**
 * \}
 */

#ifdef __cplusplus
}
#endif

#endif /* PIPEWIRE_PROPERTIES_H */
