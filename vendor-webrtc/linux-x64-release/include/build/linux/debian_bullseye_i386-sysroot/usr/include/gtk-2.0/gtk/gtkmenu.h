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

#ifndef __GTK_MENU_H__
#define __GTK_MENU_H__


#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkaccelgroup.h>
#include <gtk/gtkmenushell.h>


G_BEGIN_DECLS

#define GTK_TYPE_MENU			(gtk_menu_get_type ())
#define GTK_MENU(obj)			(G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_MENU, GtkMenu))
#define GTK_MENU_CLASS(klass)		(G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_MENU, GtkMenuClass))
#define GTK_IS_MENU(obj)		(G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_MENU))
#define GTK_IS_MENU_CLASS(klass)	(G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_MENU))
#define GTK_MENU_GET_CLASS(obj)         (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_MENU, GtkMenuClass))


typedef struct _GtkMenu	      GtkMenu;
typedef struct _GtkMenuClass  GtkMenuClass;

typedef void (*GtkMenuPositionFunc) (GtkMenu   *menu,
				     gint      *x,
				     gint      *y,
				     gboolean  *push_in,
				     gpointer	user_data);
typedef void (*GtkMenuDetachFunc)   (GtkWidget *attach_widget,
				     GtkMenu   *menu);

struct _GtkMenu
{
  GtkMenuShell GSEAL (menu_shell);
  
  GtkWidget *GSEAL (parent_menu_item);
  GtkWidget *GSEAL (old_active_menu_item);

  GtkAccelGroup *GSEAL (accel_group);
  gchar         *GSEAL (accel_path);
  GtkMenuPositionFunc GSEAL (position_func);
  gpointer GSEAL (position_func_data);

  guint GSEAL (toggle_size);
  /* Do _not_ touch these widgets directly. We hide the reference
   * count from the toplevel to the menu, so it must be restored
   * before operating on these widgets
   */
  GtkWidget *GSEAL (toplevel);
  
  GtkWidget *GSEAL (tearoff_window);
  GtkWidget *GSEAL (tearoff_hbox);
  GtkWidget *GSEAL (tearoff_scrollbar);
  GtkAdjustment *GSEAL (tearoff_adjustment);

  GdkWindow *GSEAL (view_window);
  GdkWindow *GSEAL (bin_window);

  gint GSEAL (scroll_offset);
  gint GSEAL (saved_scroll_offset);
  gint GSEAL (scroll_step);
  guint GSEAL (timeout_id);
  
  /* When a submenu of this menu is popped up, motion in this
   * region is ignored
   */
  GdkRegion *GSEAL (navigation_region); /* unused */
  guint GSEAL (navigation_timeout);

  guint GSEAL (needs_destruction_ref_count) : 1;
  guint GSEAL (torn_off) : 1;
  /* The tearoff is active when it is torn off and the not-torn-off
   * menu is not popped up.
   */
  guint GSEAL (tearoff_active) : 1;

  guint GSEAL (scroll_fast) : 1;

  guint GSEAL (upper_arrow_visible) : 1;
  guint GSEAL (lower_arrow_visible) : 1;
  guint GSEAL (upper_arrow_prelight) : 1;
  guint GSEAL (lower_arrow_prelight) : 1;
};

struct _GtkMenuClass
{
  GtkMenuShellClass parent_class;

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};


GType	   gtk_menu_get_type		  (void) G_GNUC_CONST;
GtkWidget* gtk_menu_new			  (void);

/* Display the menu onscreen */
void	   gtk_menu_popup		  (GtkMenu	       *menu,
					   GtkWidget	       *parent_menu_shell,
					   GtkWidget	       *parent_menu_item,
					   GtkMenuPositionFunc	func,
					   gpointer		data,
					   guint		button,
					   guint32		activate_time);

/* Position the menu according to its position function. Called
 * from gtkmenuitem.c when a menu-item changes its allocation
 */
void	   gtk_menu_reposition		  (GtkMenu	       *menu);

void	   gtk_menu_popdown		  (GtkMenu	       *menu);

/* Keep track of the last menu item selected. (For the purposes
 * of the option menu
 */
GtkWidget* gtk_menu_get_active		  (GtkMenu	       *menu);
void	   gtk_menu_set_active		  (GtkMenu	       *menu,
					   guint		index_);

/* set/get the accelerator group that holds global accelerators (should
 * be added to the corresponding toplevel with gtk_window_add_accel_group().
 */
void	       gtk_menu_set_accel_group	  (GtkMenu	       *menu,
					   GtkAccelGroup       *accel_group);
GtkAccelGroup* gtk_menu_get_accel_group	  (GtkMenu	       *menu);
void           gtk_menu_set_accel_path    (GtkMenu             *menu,
					   const gchar         *accel_path);
const gchar*   gtk_menu_get_accel_path    (GtkMenu             *menu);

/* A reference count is kept for a widget when it is attached to
 * a particular widget. This is typically a menu item; it may also
 * be a widget with a popup menu - for instance, the Notebook widget.
 */
void	   gtk_menu_attach_to_widget	  (GtkMenu	       *menu,
					   GtkWidget	       *attach_widget,
					   GtkMenuDetachFunc	detacher);
void	   gtk_menu_detach		  (GtkMenu	       *menu);

/* This should be dumped in favor of data set when the menu is popped
 * up - that is currently in the ItemFactory code, but should be
 * in the Menu code.
 */
GtkWidget* gtk_menu_get_attach_widget	  (GtkMenu	       *menu);

void       gtk_menu_set_tearoff_state     (GtkMenu             *menu,
					   gboolean             torn_off);
gboolean   gtk_menu_get_tearoff_state     (GtkMenu             *menu);

/* This sets the window manager title for the window that
 * appears when a menu is torn off
 */
void       gtk_menu_set_title             (GtkMenu             *menu,
					   const gchar         *title);
const gchar *gtk_menu_get_title           (GtkMenu             *menu);

void       gtk_menu_reorder_child         (GtkMenu             *menu,
                                           GtkWidget           *child,
                                           gint                position);

void	   gtk_menu_set_screen		  (GtkMenu	       *menu,
					   GdkScreen	       *screen);

void       gtk_menu_attach                (GtkMenu             *menu,
                                           GtkWidget           *child,
                                           guint                left_attach,
                                           guint                right_attach,
                                           guint                top_attach,
                                           guint                bottom_attach);

void       gtk_menu_set_monitor           (GtkMenu             *menu,
                                           gint                 monitor_num);
gint       gtk_menu_get_monitor           (GtkMenu             *menu);
GList*     gtk_menu_get_for_attach_widget (GtkWidget           *widget); 

#ifndef GTK_DISABLE_DEPRECATED
#define gtk_menu_append(menu,child)	gtk_menu_shell_append  ((GtkMenuShell *)(menu),(child))
#define gtk_menu_prepend(menu,child)    gtk_menu_shell_prepend ((GtkMenuShell *)(menu),(child))
#define gtk_menu_insert(menu,child,pos)	gtk_menu_shell_insert ((GtkMenuShell *)(menu),(child),(pos))
#endif /* GTK_DISABLE_DEPRECATED */

void     gtk_menu_set_reserve_toggle_size (GtkMenu  *menu,
                                          gboolean   reserve_toggle_size);
gboolean gtk_menu_get_reserve_toggle_size (GtkMenu  *menu);


G_END_DECLS

#endif /* __GTK_MENU_H__ */
