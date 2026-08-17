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

#ifndef __GTK_IM_CONTEXT_SIMPLE_H__
#define __GTK_IM_CONTEXT_SIMPLE_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkimcontext.h>


G_BEGIN_DECLS


#define GTK_TYPE_IM_CONTEXT_SIMPLE              (gtk_im_context_simple_get_type ())
#define GTK_IM_CONTEXT_SIMPLE(obj)              (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_IM_CONTEXT_SIMPLE, GtkIMContextSimple))
#define GTK_IM_CONTEXT_SIMPLE_CLASS(klass)      (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_IM_CONTEXT_SIMPLE, GtkIMContextSimpleClass))
#define GTK_IS_IM_CONTEXT_SIMPLE(obj)           (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_IM_CONTEXT_SIMPLE))
#define GTK_IS_IM_CONTEXT_SIMPLE_CLASS(klass)   (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_IM_CONTEXT_SIMPLE))
#define GTK_IM_CONTEXT_SIMPLE_GET_CLASS(obj)    (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_IM_CONTEXT_SIMPLE, GtkIMContextSimpleClass))


typedef struct _GtkIMContextSimple       GtkIMContextSimple;
typedef struct _GtkIMContextSimpleClass  GtkIMContextSimpleClass;

#define GTK_MAX_COMPOSE_LEN 7

struct _GtkIMContextSimple
{
  GtkIMContext object;

  GSList *GSEAL (tables);

  guint GSEAL (compose_buffer[GTK_MAX_COMPOSE_LEN + 1]);
  gunichar GSEAL (tentative_match);
  gint GSEAL (tentative_match_len);

  guint GSEAL (in_hex_sequence) : 1;
  guint GSEAL (modifiers_dropped) : 1;
};

struct _GtkIMContextSimpleClass
{
  GtkIMContextClass parent_class;
};

GType         gtk_im_context_simple_get_type  (void) G_GNUC_CONST;
GtkIMContext *gtk_im_context_simple_new       (void);

void          gtk_im_context_simple_add_table (GtkIMContextSimple *context_simple,
					       guint16            *data,
					       gint                max_seq_len,
					       gint                n_seqs);


G_END_DECLS


#endif /* __GTK_IM_CONTEXT_SIMPLE_H__ */
