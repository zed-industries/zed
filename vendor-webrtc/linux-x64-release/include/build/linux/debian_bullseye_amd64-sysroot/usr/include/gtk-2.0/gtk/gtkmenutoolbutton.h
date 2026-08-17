/* GTK - The GIMP Toolkit
 *
 * Copyright (C) 2003 Ricardo Fernandez Pascual
 * Copyright (C) 2004 Paolo Borelli
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

#ifndef __GTK_MENU_TOOL_BUTTON_H__
#define __GTK_MENU_TOOL_BUTTON_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkmenu.h>
#include <gtk/gtktoolbutton.h>

G_BEGIN_DECLS

#define GTK_TYPE_MENU_TOOL_BUTTON         (gtk_menu_tool_button_get_type ())
#define GTK_MENU_TOOL_BUTTON(o)           (G_TYPE_CHECK_INSTANCE_CAST ((o), GTK_TYPE_MENU_TOOL_BUTTON, GtkMenuToolButton))
#define GTK_MENU_TOOL_BUTTON_CLASS(k)     (G_TYPE_CHECK_CLASS_CAST((k), GTK_TYPE_MENU_TOOL_BUTTON, GtkMenuToolButtonClass))
#define GTK_IS_MENU_TOOL_BUTTON(o)        (G_TYPE_CHECK_INSTANCE_TYPE ((o), GTK_TYPE_MENU_TOOL_BUTTON))
#define GTK_IS_MENU_TOOL_BUTTON_CLASS(k)  (G_TYPE_CHECK_CLASS_TYPE ((k), GTK_TYPE_MENU_TOOL_BUTTON))
#define GTK_MENU_TOOL_BUTTON_GET_CLASS(o) (G_TYPE_INSTANCE_GET_CLASS ((o), GTK_TYPE_MENU_TOOL_BUTTON, GtkMenuToolButtonClass))

typedef struct _GtkMenuToolButtonClass   GtkMenuToolButtonClass;
typedef struct _GtkMenuToolButton        GtkMenuToolButton;
typedef struct _GtkMenuToolButtonPrivate GtkMenuToolButtonPrivate;

struct _GtkMenuToolButton
{
  GtkToolButton parent;

  /*< private >*/
  GtkMenuToolButtonPrivate *GSEAL (priv);
};

struct _GtkMenuToolButtonClass
{
  GtkToolButtonClass parent_class;

  void (*show_menu) (GtkMenuToolButton *button);

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};

GType         gtk_menu_tool_button_get_type       (void) G_GNUC_CONST;
GtkToolItem  *gtk_menu_tool_button_new            (GtkWidget   *icon_widget,
                                                   const gchar *label);
GtkToolItem  *gtk_menu_tool_button_new_from_stock (const gchar *stock_id);

void          gtk_menu_tool_button_set_menu       (GtkMenuToolButton *button,
                                                   GtkWidget         *menu);
GtkWidget    *gtk_menu_tool_button_get_menu       (GtkMenuToolButton *button);

#ifndef GTK_DISABLE_DEPRECATED
void          gtk_menu_tool_button_set_arrow_tooltip (GtkMenuToolButton *button,
                                                      GtkTooltips       *tooltips,
                                                      const gchar       *tip_text,
                                                      const gchar       *tip_private);
#endif /* GTK_DISABLE_DEPRECATED */

void          gtk_menu_tool_button_set_arrow_tooltip_text   (GtkMenuToolButton *button,
							     const gchar       *text);
void          gtk_menu_tool_button_set_arrow_tooltip_markup (GtkMenuToolButton *button,
							     const gchar       *markup);

G_END_DECLS

#endif /* __GTK_MENU_TOOL_BUTTON_H__ */
