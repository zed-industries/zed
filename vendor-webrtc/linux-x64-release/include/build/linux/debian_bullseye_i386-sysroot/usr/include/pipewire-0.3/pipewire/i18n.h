/* PipeWire
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

#ifndef PIPEWIRE_I18N_H
#define PIPEWIRE_I18N_H

#ifdef __cplusplus
extern "C" {
#endif

/** \defgroup pw_gettext Internationalization
 * Gettext interface
 */

/**
 * \addtogroup pw_gettext
 * \{
 */
#include <spa/support/i18n.h>

SPA_FORMAT_ARG_FUNC(1) const char *pw_gettext(const char *msgid);
SPA_FORMAT_ARG_FUNC(1) const char *pw_ngettext(const char *msgid, const char *msgid_plural, unsigned long int n);

#define _(String)	(pw_gettext(String))
#define N_(String)	(String)

/**
 * \}
 */

#ifdef __cplusplus
}
#endif

#endif /* PIPEWIRE_I18N_H */
