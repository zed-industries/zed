/* GTK - The GIMP Toolkit
 * Copyright (C) 2000 Red Hat Software
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 */

#ifndef __GTK_IM_MODULE_H__
#define __GTK_IM_MODULE_H__

#include <gtk/gtk.h>

/* The following entry points are exported by each input method module
 */

/*
void          im_module_list   (const GtkIMContextInfo ***contexts,
				guint                    *n_contexts);
void          im_module_init   (GtkModule                *module);
void          im_module_exit   (void);
GtkIMContext *im_module_create (const gchar              *context_id);
*/

#endif /* __GTK_IM_MODULE_H__ */
