/* gtktoggletoolbutton.h
 *
 * Copyright (C) 2002 Anders Carlsson <andersca@gnome.org>
 * Copyright (C) 2002 James Henstridge <james@daa.com.au>
 * Copyright (C) 2003 Soeren Sandmann <sandmann@daimi.au.dk>
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

#ifndef __GTK_TOGGLE_TOOL_BUTTON_H__
#define __GTK_TOGGLE_TOOL_BUTTON_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtktoolbutton.h>

G_BEGIN_DECLS

#define GTK_TYPE_TOGGLE_TOOL_BUTTON             (gtk_toggle_tool_button_get_type ())
#define GTK_TOGGLE_TOOL_BUTTON(obj)             (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_TOGGLE_TOOL_BUTTON, GtkToggleToolButton))
#define GTK_TOGGLE_TOOL_BUTTON_CLASS(klass)     (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_TOGGLE_TOOL_BUTTON, GtkToggleToolButtonClass))
#define GTK_IS_TOGGLE_TOOL_BUTTON(obj)          (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_TOGGLE_TOOL_BUTTON))
#define GTK_IS_TOGGLE_TOOL_BUTTON_CLASS(klass)  (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_TOGGLE_TOOL_BUTTON))
#define GTK_TOGGLE_TOOL_BUTTON_GET_CLASS(obj)   (G_TYPE_INSTANCE_GET_CLASS((obj), GTK_TYPE_TOGGLE_TOOL_BUTTON, GtkToggleToolButtonClass))

typedef struct _GtkToggleToolButton        GtkToggleToolButton;
typedef struct _GtkToggleToolButtonClass   GtkToggleToolButtonClass;
typedef struct _GtkToggleToolButtonPrivate GtkToggleToolButtonPrivate;

struct _GtkToggleToolButton
{
  GtkToolButton parent;

  /*< private >*/
  GtkToggleToolButtonPrivate *priv;
};

/**
 * GtkToggleToolButtonClass:
 * @parent_class: The parent class.
 * @toggled: Signal emitted whenever the toggle tool button changes state.
 */
struct _GtkToggleToolButtonClass
{
  GtkToolButtonClass parent_class;

  /*< public >*/

  /* signal */
  void (* toggled) (GtkToggleToolButton *button);

  /*< private >*/

  /* Padding for future expansion */
  void (* _gtk_reserved1) (void);
  void (* _gtk_reserved2) (void);
  void (* _gtk_reserved3) (void);
  void (* _gtk_reserved4) (void);
};

GDK_AVAILABLE_IN_ALL
GType        gtk_toggle_tool_button_get_type       (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkToolItem *gtk_toggle_tool_button_new            (void);
GDK_DEPRECATED_IN_3_10_FOR(gtk_toggle_tool_button_new)
GtkToolItem *gtk_toggle_tool_button_new_from_stock (const gchar *stock_id);

GDK_AVAILABLE_IN_ALL
void         gtk_toggle_tool_button_set_active     (GtkToggleToolButton *button,
						    gboolean             is_active);
GDK_AVAILABLE_IN_ALL
gboolean     gtk_toggle_tool_button_get_active     (GtkToggleToolButton *button);

G_END_DECLS

#endif /* __GTK_TOGGLE_TOOL_BUTTON_H__ */
