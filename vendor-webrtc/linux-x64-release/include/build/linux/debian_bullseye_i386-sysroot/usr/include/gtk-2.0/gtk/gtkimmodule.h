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
 * License along with this library; if not, write to the
 * Free Software Foundation, Inc., 59 Temple Place - Suite 330,
 * Boston, MA 02111-1307, USA.
 */

#ifndef __GTK_IM_MODULE_H__
#define __GTK_IM_MODULE_H__

#include <gtk/gtk.h>

G_BEGIN_DECLS

typedef struct _GtkIMContextInfo GtkIMContextInfo;

struct _GtkIMContextInfo
{
  const gchar *context_id;
  const gchar *context_name;
  const gchar *domain;
  const gchar *domain_dirname;
  const gchar *default_locales;
};

/* Functions for use within GTK+
 */
void           _gtk_im_module_list                   (const GtkIMContextInfo ***contexts,
						      guint                    *n_contexts);
GtkIMContext * _gtk_im_module_create                 (const gchar              *context_id);
const gchar  * _gtk_im_module_get_default_context_id (GdkWindow                *client_window);

/* The following entry points are exported by each input method module
 */

/*
void          im_module_list   (const GtkIMContextInfo ***contexts,
				guint                    *n_contexts);
void          im_module_init   (GtkModule                *module);
void          im_module_exit   (void);
GtkIMContext *im_module_create (const gchar              *context_id);
*/

G_END_DECLS

#endif /* __GTK_IM_MODULE_H__ */
