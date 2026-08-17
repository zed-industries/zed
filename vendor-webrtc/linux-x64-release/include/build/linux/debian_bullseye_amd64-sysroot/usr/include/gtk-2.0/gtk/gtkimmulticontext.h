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

#ifndef __GTK_IM_MULTICONTEXT_H__
#define __GTK_IM_MULTICONTEXT_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkimcontext.h>
#include <gtk/gtkmenushell.h>

G_BEGIN_DECLS

#define GTK_TYPE_IM_MULTICONTEXT              (gtk_im_multicontext_get_type ())
#define GTK_IM_MULTICONTEXT(obj)              (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_IM_MULTICONTEXT, GtkIMMulticontext))
#define GTK_IM_MULTICONTEXT_CLASS(klass)      (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_IM_MULTICONTEXT, GtkIMMulticontextClass))
#define GTK_IS_IM_MULTICONTEXT(obj)           (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_IM_MULTICONTEXT))
#define GTK_IS_IM_MULTICONTEXT_CLASS(klass)   (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_IM_MULTICONTEXT))
#define GTK_IM_MULTICONTEXT_GET_CLASS(obj)    (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_IM_MULTICONTEXT, GtkIMMulticontextClass))


typedef struct _GtkIMMulticontext        GtkIMMulticontext;
typedef struct _GtkIMMulticontextClass   GtkIMMulticontextClass;
typedef struct _GtkIMMulticontextPrivate GtkIMMulticontextPrivate;

struct _GtkIMMulticontext
{
  GtkIMContext object;

  GtkIMContext *GSEAL (slave);

  GtkIMMulticontextPrivate *GSEAL (priv);

  gchar *GSEAL (context_id);
};

struct _GtkIMMulticontextClass
{
  GtkIMContextClass parent_class;

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};

GType         gtk_im_multicontext_get_type (void) G_GNUC_CONST;
GtkIMContext *gtk_im_multicontext_new      (void);

void          gtk_im_multicontext_append_menuitems (GtkIMMulticontext *context,
						    GtkMenuShell      *menushell);
const char  * gtk_im_multicontext_get_context_id   (GtkIMMulticontext *context);

void          gtk_im_multicontext_set_context_id   (GtkIMMulticontext *context,
                                                    const char        *context_id);
 
G_END_DECLS

#endif /* __GTK_IM_MULTICONTEXT_H__ */
