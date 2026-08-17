/* GMODULE - GLIB wrapper code for dynamic module loading
 * Copyright (C) 1998 Tim Janik
 *
 * SPDX-License-Identifier: LGPL-2.1-or-later
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, see <http://www.gnu.org/licenses/>.
 */
#ifndef __G_MODULE_CONF_H__
#define __G_MODULE_CONF_H__

#ifdef __cplusplus
extern "C" {
#endif /* __cplusplus */

#define G_MODULE_IMPL_NONE 0
#define G_MODULE_IMPL_DL 1
#define G_MODULE_IMPL_WIN32 3
#define G_MODULE_IMPL_AR 7

#define G_MODULE_IMPL G_MODULE_IMPL_DL
#undef G_MODULE_HAVE_DLERROR
#if (1)
#define G_MODULE_HAVE_DLERROR
#endif
#if (0)
#define G_MODULE_NEED_USCORE
#endif
#if (0)
#define G_MODULE_BROKEN_RTLD_GLOBAL
#endif

#ifdef __cplusplus
}
#endif /* __cplusplus */

#endif /* __G_MODULE_CONF_H__ */
