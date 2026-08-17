/* gtkshortcutswindow.h
 *
 * Copyright (C) 2015 Christian Hergert <christian@hergert.me>
 *
 *  This library is free software; you can redistribute it and/or
 *  modify it under the terms of the GNU Library General Public License as
 *  published by the Free Software Foundation; either version 2 of the
 *  License, or (at your option) any later version.
 *
 *  This library is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 *  Library General Public License for more details.
 *
 *  You should have received a copy of the GNU Library General Public
 *  License along with this library. If not, see <http://www.gnu.org/licenses/>.
 */

#ifndef __GTK_SHORTCUTS_WINDOW_H__
#define __GTK_SHORTCUTS_WINDOW_H__

#include <gtk/gtkwindow.h>

G_BEGIN_DECLS

#define GTK_TYPE_SHORTCUTS_WINDOW            (gtk_shortcuts_window_get_type ())
#define GTK_SHORTCUTS_WINDOW(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_SHORTCUTS_WINDOW, GtkShortcutsWindow))
#define GTK_SHORTCUTS_WINDOW_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_SHORTCUTS_WINDOW, GtkShortcutsWindowClass))
#define GTK_IS_SHORTCUTS_WINDOW(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_SHORTCUTS_WINDOW))
#define GTK_IS_SHORTCUTS_WINDOW_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_SHORTCUTS_WINDOW))
#define GTK_SHORTCUTS_WINDOW_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_SHORTCUTS_WINDOW, GtkShortcutsWindowClass))


typedef struct _GtkShortcutsWindow         GtkShortcutsWindow;
typedef struct _GtkShortcutsWindowClass    GtkShortcutsWindowClass;


struct _GtkShortcutsWindow
{
  GtkWindow window;
};

struct _GtkShortcutsWindowClass
{
  GtkWindowClass parent_class;

  void (*close)  (GtkShortcutsWindow *self);
  void (*search) (GtkShortcutsWindow *self);
};

GDK_AVAILABLE_IN_3_20
GType gtk_shortcuts_window_get_type (void) G_GNUC_CONST;

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkShortcutsWindow, g_object_unref)

G_END_DECLS

#endif /* GTK_SHORTCUTS_WINDOW _H */
