/* GTK - The GIMP Toolkit
 * Copyright (C) 1995-1997 Peter Mattis, Spencer Kimball and Josh MacDonald
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

/*
 * Modified by the GTK+ Team and others 1997-2000.  See the AUTHORS
 * file for a list of people on the GTK+ Team.  See the ChangeLog
 * files for a list of changes.  These files are distributed with
 * GTK+ at ftp://ftp.gtk.org/pub/gtk/.
 */

#ifndef __GTK_EDITABLE_H__
#define __GTK_EDITABLE_H__


#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkwidget.h>


G_BEGIN_DECLS

#define GTK_TYPE_EDITABLE             (gtk_editable_get_type ())
#define GTK_EDITABLE(obj)             (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_EDITABLE, GtkEditable))
#define GTK_IS_EDITABLE(obj)          (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_EDITABLE))
#define GTK_EDITABLE_GET_IFACE(inst)  (G_TYPE_INSTANCE_GET_INTERFACE ((inst), GTK_TYPE_EDITABLE, GtkEditableInterface))

typedef struct _GtkEditable          GtkEditable;         /* Dummy typedef */
typedef struct _GtkEditableInterface GtkEditableInterface;

struct _GtkEditableInterface
{
  GTypeInterface                   base_iface;

  /* signals */
  void (* insert_text)              (GtkEditable    *editable,
                                     const char     *text,
                                     int             length,
                                     int            *position);
  void (* delete_text)              (GtkEditable    *editable,
                                     int             start_pos,
                                     int             end_pos);
  void (* changed)                  (GtkEditable    *editable);

  /* vtable */
  const char * (* get_text)         (GtkEditable    *editable);
  void     (* do_insert_text)       (GtkEditable    *editable,
                                     const char     *text,
                                     int             length,
                                     int            *position);
  void     (* do_delete_text)       (GtkEditable    *editable,
                                     int             start_pos,
                                     int             end_pos);

  gboolean (* get_selection_bounds) (GtkEditable    *editable,
                                     int            *start_pos,
                                     int            *end_pos);
  void     (* set_selection_bounds) (GtkEditable    *editable,
                                     int             start_pos,
                                     int             end_pos);
  GtkEditable * (* get_delegate)    (GtkEditable    *editable);
};

GDK_AVAILABLE_IN_ALL
GType    gtk_editable_get_type             (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
const char * gtk_editable_get_text         (GtkEditable *editable);
GDK_AVAILABLE_IN_ALL
void     gtk_editable_set_text             (GtkEditable *editable,
                                            const char  *text);
GDK_AVAILABLE_IN_ALL
char *   gtk_editable_get_chars            (GtkEditable *editable,
                                            int          start_pos,
                                            int          end_pos) G_GNUC_MALLOC;
GDK_AVAILABLE_IN_ALL
void     gtk_editable_insert_text          (GtkEditable *editable,
                                            const char  *text,
                                            int          length,
                                            int         *position);
GDK_AVAILABLE_IN_ALL
void     gtk_editable_delete_text          (GtkEditable *editable,
                                            int          start_pos,
                                            int          end_pos);

GDK_AVAILABLE_IN_ALL
gboolean gtk_editable_get_selection_bounds (GtkEditable *editable,
                                            int         *start_pos,
                                            int         *end_pos);
GDK_AVAILABLE_IN_ALL
void     gtk_editable_delete_selection     (GtkEditable *editable);
GDK_AVAILABLE_IN_ALL
void     gtk_editable_select_region        (GtkEditable *editable,
                                            int          start_pos,
                                            int          end_pos);

GDK_AVAILABLE_IN_ALL
void     gtk_editable_set_position         (GtkEditable *editable,
                                            int          position);
GDK_AVAILABLE_IN_ALL
int      gtk_editable_get_position         (GtkEditable *editable);

GDK_AVAILABLE_IN_ALL
gboolean gtk_editable_get_editable         (GtkEditable *editable);
GDK_AVAILABLE_IN_ALL
void     gtk_editable_set_editable         (GtkEditable *editable,
                                            gboolean     is_editable);

GDK_AVAILABLE_IN_ALL
float    gtk_editable_get_alignment        (GtkEditable *editable);
GDK_AVAILABLE_IN_ALL
void     gtk_editable_set_alignment        (GtkEditable *editable,
                                            float        xalign);

GDK_AVAILABLE_IN_ALL
int      gtk_editable_get_width_chars      (GtkEditable *editable);
GDK_AVAILABLE_IN_ALL
void     gtk_editable_set_width_chars      (GtkEditable *editable,
                                            int          n_chars);

GDK_AVAILABLE_IN_ALL
int      gtk_editable_get_max_width_chars  (GtkEditable *editable);
GDK_AVAILABLE_IN_ALL
void     gtk_editable_set_max_width_chars  (GtkEditable *editable,
                                            int          n_chars);
GDK_AVAILABLE_IN_ALL
gboolean gtk_editable_get_enable_undo      (GtkEditable *editable);
GDK_AVAILABLE_IN_ALL
void     gtk_editable_set_enable_undo      (GtkEditable *editable,
                                            gboolean     enable_undo);

/* api for implementations */

/**
 * GtkEditableProperties:
 * @GTK_EDITABLE_PROP_TEXT: the property id for [property@Gtk.Editable:text]
 * @GTK_EDITABLE_PROP_CURSOR_POSITION: the property id for [property@Gtk.Editable:cursor-position]
 * @GTK_EDITABLE_PROP_SELECTION_BOUND: the property id for [property@Gtk.Editable:selection-bound]
 * @GTK_EDITABLE_PROP_EDITABLE: the property id for [property@Gtk.Editable:editable]
 * @GTK_EDITABLE_PROP_WIDTH_CHARS: the property id for [property@Gtk.Editable:width-chars]
 * @GTK_EDITABLE_PROP_MAX_WIDTH_CHARS: the property id for [property@Gtk.Editable:max-width-chars]
 * @GTK_EDITABLE_PROP_XALIGN: the property id for [property@Gtk.Editable:xalign]
 * @GTK_EDITABLE_PROP_ENABLE_UNDO: the property id for [property@Gtk.Editable:enable-undo]
 * @GTK_EDITABLE_NUM_PROPERTIES: the number of properties
 *
 * The identifiers for [iface@Gtk.Editable] properties.
 *
 * See [func@Gtk.Editable.install_properties] for details on how to
 * implement the `GtkEditable` interface.
 */
typedef enum {
  GTK_EDITABLE_PROP_TEXT,
  GTK_EDITABLE_PROP_CURSOR_POSITION,
  GTK_EDITABLE_PROP_SELECTION_BOUND,
  GTK_EDITABLE_PROP_EDITABLE,
  GTK_EDITABLE_PROP_WIDTH_CHARS,
  GTK_EDITABLE_PROP_MAX_WIDTH_CHARS,
  GTK_EDITABLE_PROP_XALIGN,
  GTK_EDITABLE_PROP_ENABLE_UNDO,
  GTK_EDITABLE_NUM_PROPERTIES
} GtkEditableProperties;

GDK_AVAILABLE_IN_ALL
guint        gtk_editable_install_properties    (GObjectClass *object_class,
                                                 guint         first_prop);
GDK_AVAILABLE_IN_ALL
GtkEditable *gtk_editable_get_delegate          (GtkEditable *editable);
GDK_AVAILABLE_IN_ALL
void         gtk_editable_init_delegate         (GtkEditable  *editable);
GDK_AVAILABLE_IN_ALL
void         gtk_editable_finish_delegate       (GtkEditable  *editable);
GDK_AVAILABLE_IN_ALL
gboolean     gtk_editable_delegate_set_property (GObject      *object,
                                                 guint         prop_id,
                                                 const GValue *value,
                                                 GParamSpec   *pspec);
GDK_AVAILABLE_IN_ALL
gboolean     gtk_editable_delegate_get_property (GObject      *object,
                                                 guint         prop_id,
                                                 GValue       *value,
                                                 GParamSpec   *pspec);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkEditable, g_object_unref)

G_END_DECLS

#endif /* __GTK_EDITABLE_H__ */
