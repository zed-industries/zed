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

#ifndef __GTK_IM_CONTEXT_H__
#define __GTK_IM_CONTEXT_H__


#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gdk/gdk.h>
#include <gtk/gtkobject.h>


G_BEGIN_DECLS

#define GTK_TYPE_IM_CONTEXT              (gtk_im_context_get_type ())
#define GTK_IM_CONTEXT(obj)              (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_IM_CONTEXT, GtkIMContext))
#define GTK_IM_CONTEXT_CLASS(klass)      (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_IM_CONTEXT, GtkIMContextClass))
#define GTK_IS_IM_CONTEXT(obj)           (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_IM_CONTEXT))
#define GTK_IS_IM_CONTEXT_CLASS(klass)   (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_IM_CONTEXT))
#define GTK_IM_CONTEXT_GET_CLASS(obj)    (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_IM_CONTEXT, GtkIMContextClass))


typedef struct _GtkIMContext       GtkIMContext;
typedef struct _GtkIMContextClass  GtkIMContextClass;

struct _GtkIMContext
{
  GObject parent_instance;
};

struct _GtkIMContextClass
{
  /*< private >*/
  /* Yes, this should be GObjectClass, be we can't fix it without breaking
   * binary compatibility - see bug #90935
   */
  GtkObjectClass parent_class;

  /*< public >*/
  /* Signals */
  void     (*preedit_start)        (GtkIMContext *context);
  void     (*preedit_end)          (GtkIMContext *context);
  void     (*preedit_changed)      (GtkIMContext *context);
  void     (*commit)               (GtkIMContext *context, const gchar *str);
  gboolean (*retrieve_surrounding) (GtkIMContext *context);
  gboolean (*delete_surrounding)   (GtkIMContext *context,
				    gint          offset,
				    gint          n_chars);

  /* Virtual functions */
  void     (*set_client_window)   (GtkIMContext   *context,
				   GdkWindow      *window);
  void     (*get_preedit_string)  (GtkIMContext   *context,
				   gchar         **str,
				   PangoAttrList **attrs,
				   gint           *cursor_pos);
  gboolean (*filter_keypress)     (GtkIMContext   *context,
			           GdkEventKey    *event);
  void     (*focus_in)            (GtkIMContext   *context);
  void     (*focus_out)           (GtkIMContext   *context);
  void     (*reset)               (GtkIMContext   *context);
  void     (*set_cursor_location) (GtkIMContext   *context,
				   GdkRectangle   *area);
  void     (*set_use_preedit)     (GtkIMContext   *context,
				   gboolean        use_preedit);
  void     (*set_surrounding)     (GtkIMContext   *context,
				   const gchar    *text,
				   gint            len,
				   gint            cursor_index);
  gboolean (*get_surrounding)     (GtkIMContext   *context,
				   gchar         **text,
				   gint           *cursor_index);
  /*< private >*/
  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
  void (*_gtk_reserved5) (void);
  void (*_gtk_reserved6) (void);
};

GType    gtk_im_context_get_type            (void) G_GNUC_CONST;

void     gtk_im_context_set_client_window   (GtkIMContext       *context,
					     GdkWindow          *window);
void     gtk_im_context_get_preedit_string  (GtkIMContext       *context,
					     gchar             **str,
					     PangoAttrList     **attrs,
					     gint               *cursor_pos);
gboolean gtk_im_context_filter_keypress     (GtkIMContext       *context,
					     GdkEventKey        *event);
void     gtk_im_context_focus_in            (GtkIMContext       *context);
void     gtk_im_context_focus_out           (GtkIMContext       *context);
void     gtk_im_context_reset               (GtkIMContext       *context);
void     gtk_im_context_set_cursor_location (GtkIMContext       *context,
					     const GdkRectangle *area);
void     gtk_im_context_set_use_preedit     (GtkIMContext       *context,
					     gboolean            use_preedit);
void     gtk_im_context_set_surrounding     (GtkIMContext       *context,
					     const gchar        *text,
					     gint                len,
					     gint                cursor_index);
gboolean gtk_im_context_get_surrounding     (GtkIMContext       *context,
					     gchar             **text,
					     gint               *cursor_index);
gboolean gtk_im_context_delete_surrounding  (GtkIMContext       *context,
					     gint                offset,
					     gint                n_chars);

G_END_DECLS

#endif /* __GTK_IM_CONTEXT_H__ */
