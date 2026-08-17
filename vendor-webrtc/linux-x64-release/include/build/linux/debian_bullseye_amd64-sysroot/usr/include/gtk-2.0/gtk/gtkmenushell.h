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
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.	 See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, write to the
 * Free Software Foundation, Inc., 59 Temple Place - Suite 330,
 * Boston, MA 02111-1307, USA.
 */

/*
 * Modified by the GTK+ Team and others 1997-2000.  See the AUTHORS
 * file for a list of people on the GTK+ Team.  See the ChangeLog
 * files for a list of changes.  These files are distributed with
 * GTK+ at ftp://ftp.gtk.org/pub/gtk/.
 */

#ifndef __GTK_MENU_SHELL_H__
#define __GTK_MENU_SHELL_H__


#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkcontainer.h>


G_BEGIN_DECLS

#define	GTK_TYPE_MENU_SHELL		(gtk_menu_shell_get_type ())
#define GTK_MENU_SHELL(obj)		(G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_MENU_SHELL, GtkMenuShell))
#define GTK_MENU_SHELL_CLASS(klass)	(G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_MENU_SHELL, GtkMenuShellClass))
#define GTK_IS_MENU_SHELL(obj)		(G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_MENU_SHELL))
#define GTK_IS_MENU_SHELL_CLASS(klass)	(G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_MENU_SHELL))
#define GTK_MENU_SHELL_GET_CLASS(obj)   (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_MENU_SHELL, GtkMenuShellClass))


typedef struct _GtkMenuShell	   GtkMenuShell;
typedef struct _GtkMenuShellClass  GtkMenuShellClass;

struct _GtkMenuShell
{
  GtkContainer container;

  GList *GSEAL (children);
  GtkWidget *GSEAL (active_menu_item);
  GtkWidget *GSEAL (parent_menu_shell);

  guint GSEAL (button);
  guint32 GSEAL (activate_time);

  guint GSEAL (active) : 1;
  guint GSEAL (have_grab) : 1;
  guint GSEAL (have_xgrab) : 1;
  guint GSEAL (ignore_leave) : 1; /* unused */
  guint GSEAL (menu_flag) : 1;    /* unused */
  guint GSEAL (ignore_enter) : 1;
  guint GSEAL (keyboard_mode) : 1;
};

struct _GtkMenuShellClass
{
  GtkContainerClass parent_class;
  
  guint submenu_placement : 1;
  
  void (*deactivate)     (GtkMenuShell *menu_shell);
  void (*selection_done) (GtkMenuShell *menu_shell);

  void (*move_current)     (GtkMenuShell        *menu_shell,
			    GtkMenuDirectionType direction);
  void (*activate_current) (GtkMenuShell *menu_shell,
			    gboolean      force_hide);
  void (*cancel)           (GtkMenuShell *menu_shell);
  void (*select_item)      (GtkMenuShell *menu_shell,
			    GtkWidget    *menu_item);
  void (*insert)           (GtkMenuShell *menu_shell,
			    GtkWidget    *child,
			    gint          position);
  gint (*get_popup_delay)  (GtkMenuShell *menu_shell);
  gboolean (*move_selected) (GtkMenuShell *menu_shell,
			     gint          distance);

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
};


GType gtk_menu_shell_get_type          (void) G_GNUC_CONST;
void  gtk_menu_shell_append            (GtkMenuShell *menu_shell,
					GtkWidget    *child);
void  gtk_menu_shell_prepend           (GtkMenuShell *menu_shell,
					GtkWidget    *child);
void  gtk_menu_shell_insert            (GtkMenuShell *menu_shell,
					GtkWidget    *child,
					gint          position);
void  gtk_menu_shell_deactivate        (GtkMenuShell *menu_shell);
void  gtk_menu_shell_select_item       (GtkMenuShell *menu_shell,
					GtkWidget    *menu_item);
void  gtk_menu_shell_deselect          (GtkMenuShell *menu_shell);
void  gtk_menu_shell_activate_item     (GtkMenuShell *menu_shell,
					GtkWidget    *menu_item,
					gboolean      force_deactivate);
void  gtk_menu_shell_select_first      (GtkMenuShell *menu_shell,
					gboolean      search_sensitive);
void _gtk_menu_shell_select_last       (GtkMenuShell *menu_shell,
					gboolean      search_sensitive);
gint  _gtk_menu_shell_get_popup_delay  (GtkMenuShell *menu_shell);
void  gtk_menu_shell_cancel            (GtkMenuShell *menu_shell);

void  _gtk_menu_shell_add_mnemonic     (GtkMenuShell *menu_shell,
                                        guint         keyval,
                                        GtkWidget    *target);
void  _gtk_menu_shell_remove_mnemonic  (GtkMenuShell *menu_shell,
                                        guint         keyval,
                                        GtkWidget    *target);

gboolean gtk_menu_shell_get_take_focus (GtkMenuShell *menu_shell);
void     gtk_menu_shell_set_take_focus (GtkMenuShell *menu_shell,
                                        gboolean      take_focus);

void     _gtk_menu_shell_update_mnemonics  (GtkMenuShell *menu_shell);
void     _gtk_menu_shell_set_keyboard_mode (GtkMenuShell *menu_shell,
                                            gboolean      keyboard_mode);
gboolean _gtk_menu_shell_get_keyboard_mode (GtkMenuShell *menu_shell);

G_END_DECLS

#endif /* __GTK_MENU_SHELL_H__ */
