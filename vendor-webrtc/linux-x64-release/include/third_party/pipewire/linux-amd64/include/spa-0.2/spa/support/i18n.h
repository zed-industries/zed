/* Simple Plugin API
 *
 * Copyright © 2021 Wim Taymans
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

#ifndef SPA_I18N_H
#define SPA_I18N_H

#ifdef __cplusplus
extern "C" {
#endif

#include <spa/utils/hook.h>
#include <spa/utils/defs.h>

/** \defgroup spa_i18n I18N
 * Gettext interface
 */

/**
 * \addtogroup spa_i18n
 * \{
 */

#define SPA_TYPE_INTERFACE_I18N		SPA_TYPE_INFO_INTERFACE_BASE "I18N"

#define SPA_VERSION_I18N		0
struct spa_i18n { struct spa_interface iface; };

struct spa_i18n_methods {
#define SPA_VERSION_I18N_METHODS	0
        uint32_t version;

	/**
	 * Translate a message
         *
         * \param object the i18n interface
         * \param msgid the message
         * \return a translated message
	 */
	const char *(*text) (void *object, const char *msgid);

	/**
	 * Translate a message for a number
         *
         * \param object the i18n interface
         * \param msgid the message to translate
         * \param msgid_plural the plural form of \a msgid
         * \param n a number
         * \return a translated message for the number \a n
	 */
	const char *(*ntext) (void *object, const char *msgid,
			const char *msgid_plural, unsigned long int n);
};

SPA_FORMAT_ARG_FUNC(2)
static inline const char *
spa_i18n_text(struct spa_i18n *i18n, const char *msgid)
{
	const char *res = msgid;
	if (SPA_LIKELY(i18n != NULL))
		spa_interface_call_res(&i18n->iface,
	                        struct spa_i18n_methods, res,
				text, 0, msgid);
	return res;
}

static inline const char *
spa_i18n_ntext(struct spa_i18n *i18n, const char *msgid,
		const char *msgid_plural, unsigned long int n)
{
	const char *res = n == 1 ? msgid : msgid_plural;
	if (SPA_LIKELY(i18n != NULL))
		spa_interface_call_res(&i18n->iface,
	                        struct spa_i18n_methods, res,
				ntext, 0, msgid, msgid_plural, n);
	return res;
}

/**
 * \}
 */

#ifdef __cplusplus
}  /* extern "C" */
#endif

#endif /* SPA_I18N_H */
