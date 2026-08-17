/* -*- Mode: C; c-file-style: "gnu"; tab-width: 8 -*- */
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

#ifndef __GTK_NOTEBOOK_H__
#define __GTK_NOTEBOOK_H__


#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkcontainer.h>


G_BEGIN_DECLS

#define GTK_TYPE_NOTEBOOK                  (gtk_notebook_get_type ())
#define GTK_NOTEBOOK(obj)                  (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_NOTEBOOK, GtkNotebook))
#define GTK_NOTEBOOK_CLASS(klass)          (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_NOTEBOOK, GtkNotebookClass))
#define GTK_IS_NOTEBOOK(obj)               (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_NOTEBOOK))
#define GTK_IS_NOTEBOOK_CLASS(klass)       (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_NOTEBOOK))
#define GTK_NOTEBOOK_GET_CLASS(obj)        (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_NOTEBOOK, GtkNotebookClass))


typedef enum
{
  GTK_NOTEBOOK_TAB_FIRST,
  GTK_NOTEBOOK_TAB_LAST
} GtkNotebookTab;

typedef struct _GtkNotebook              GtkNotebook;
typedef struct _GtkNotebookPrivate       GtkNotebookPrivate;
typedef struct _GtkNotebookClass         GtkNotebookClass;

struct _GtkNotebook
{
  /*< private >*/
  GtkContainer container;

  GtkNotebookPrivate *priv;
};

struct _GtkNotebookClass
{
  GtkContainerClass parent_class;

  void (* switch_page)       (GtkNotebook     *notebook,
                              GtkWidget       *page,
			      guint            page_num);

  /* Action signals for keybindings */
  gboolean (* select_page)     (GtkNotebook       *notebook,
                                gboolean           move_focus);
  gboolean (* focus_tab)       (GtkNotebook       *notebook,
                                GtkNotebookTab     type);
  gboolean (* change_current_page) (GtkNotebook   *notebook,
                                gint               offset);
  void (* move_focus_out)      (GtkNotebook       *notebook,
				GtkDirectionType   direction);
  gboolean (* reorder_tab)     (GtkNotebook       *notebook,
				GtkDirectionType   direction,
				gboolean           move_to_last);

  /* More vfuncs */
  gint (* insert_page)         (GtkNotebook       *notebook,
			        GtkWidget         *child,
				GtkWidget         *tab_label,
				GtkWidget         *menu_label,
				gint               position);

  GtkNotebook * (* create_window) (GtkNotebook       *notebook,
                                   GtkWidget         *page,
                                   gint               x,
                                   gint               y);

  void (* page_reordered)      (GtkNotebook     *notebook,
                                GtkWidget       *child,
                                guint            page_num);

  void (* page_removed)        (GtkNotebook     *notebook,
                                GtkWidget       *child,
                                guint            page_num);

  void (* page_added)          (GtkNotebook     *notebook,
                                GtkWidget       *child,
                                guint            page_num);

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
  void (*_gtk_reserved5) (void);
  void (*_gtk_reserved6) (void);
  void (*_gtk_reserved7) (void);
  void (*_gtk_reserved8) (void);
};

/***********************************************************
 *           Creation, insertion, deletion                 *
 ***********************************************************/

GDK_AVAILABLE_IN_ALL
GType   gtk_notebook_get_type       (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkWidget * gtk_notebook_new        (void);
GDK_AVAILABLE_IN_ALL
gint gtk_notebook_append_page       (GtkNotebook *notebook,
				     GtkWidget   *child,
				     GtkWidget   *tab_label);
GDK_AVAILABLE_IN_ALL
gint gtk_notebook_append_page_menu  (GtkNotebook *notebook,
				     GtkWidget   *child,
				     GtkWidget   *tab_label,
				     GtkWidget   *menu_label);
GDK_AVAILABLE_IN_ALL
gint gtk_notebook_prepend_page      (GtkNotebook *notebook,
				     GtkWidget   *child,
				     GtkWidget   *tab_label);
GDK_AVAILABLE_IN_ALL
gint gtk_notebook_prepend_page_menu (GtkNotebook *notebook,
				     GtkWidget   *child,
				     GtkWidget   *tab_label,
				     GtkWidget   *menu_label);
GDK_AVAILABLE_IN_ALL
gint gtk_notebook_insert_page       (GtkNotebook *notebook,
				     GtkWidget   *child,
				     GtkWidget   *tab_label,
				     gint         position);
GDK_AVAILABLE_IN_ALL
gint gtk_notebook_insert_page_menu  (GtkNotebook *notebook,
				     GtkWidget   *child,
				     GtkWidget   *tab_label,
				     GtkWidget   *menu_label,
				     gint         position);
GDK_AVAILABLE_IN_ALL
void gtk_notebook_remove_page       (GtkNotebook *notebook,
				     gint         page_num);

/***********************************************************
 *           Tabs drag and drop                            *
 ***********************************************************/

GDK_AVAILABLE_IN_ALL
void         gtk_notebook_set_group_name (GtkNotebook *notebook,
                                          const gchar *group_name);
GDK_AVAILABLE_IN_ALL
const gchar *gtk_notebook_get_group_name (GtkNotebook *notebook);



/***********************************************************
 *            query, set current NotebookPage              *
 ***********************************************************/

GDK_AVAILABLE_IN_ALL
gint       gtk_notebook_get_current_page (GtkNotebook *notebook);
GDK_AVAILABLE_IN_ALL
GtkWidget* gtk_notebook_get_nth_page     (GtkNotebook *notebook,
					  gint         page_num);
GDK_AVAILABLE_IN_ALL
gint       gtk_notebook_get_n_pages      (GtkNotebook *notebook);
GDK_AVAILABLE_IN_ALL
gint       gtk_notebook_page_num         (GtkNotebook *notebook,
					  GtkWidget   *child);
GDK_AVAILABLE_IN_ALL
void       gtk_notebook_set_current_page (GtkNotebook *notebook,
					  gint         page_num);
GDK_AVAILABLE_IN_ALL
void       gtk_notebook_next_page        (GtkNotebook *notebook);
GDK_AVAILABLE_IN_ALL
void       gtk_notebook_prev_page        (GtkNotebook *notebook);

/***********************************************************
 *            set Notebook, NotebookTab style              *
 ***********************************************************/

GDK_AVAILABLE_IN_ALL
void     gtk_notebook_set_show_border      (GtkNotebook     *notebook,
					    gboolean         show_border);
GDK_AVAILABLE_IN_ALL
gboolean gtk_notebook_get_show_border      (GtkNotebook     *notebook);
GDK_AVAILABLE_IN_ALL
void     gtk_notebook_set_show_tabs        (GtkNotebook     *notebook,
					    gboolean         show_tabs);
GDK_AVAILABLE_IN_ALL
gboolean gtk_notebook_get_show_tabs        (GtkNotebook     *notebook);
GDK_AVAILABLE_IN_ALL
void     gtk_notebook_set_tab_pos          (GtkNotebook     *notebook,
				            GtkPositionType  pos);
GDK_AVAILABLE_IN_ALL
GtkPositionType gtk_notebook_get_tab_pos   (GtkNotebook     *notebook);
GDK_AVAILABLE_IN_ALL
void     gtk_notebook_set_scrollable       (GtkNotebook     *notebook,
					    gboolean         scrollable);
GDK_AVAILABLE_IN_ALL
gboolean gtk_notebook_get_scrollable       (GtkNotebook     *notebook);
GDK_DEPRECATED_IN_3_4
guint16  gtk_notebook_get_tab_hborder      (GtkNotebook     *notebook);
GDK_DEPRECATED_IN_3_4
guint16  gtk_notebook_get_tab_vborder      (GtkNotebook     *notebook);

/***********************************************************
 *               enable/disable PopupMenu                  *
 ***********************************************************/

GDK_AVAILABLE_IN_ALL
void gtk_notebook_popup_enable  (GtkNotebook *notebook);
GDK_AVAILABLE_IN_ALL
void gtk_notebook_popup_disable (GtkNotebook *notebook);

/***********************************************************
 *             query/set NotebookPage Properties           *
 ***********************************************************/

GDK_AVAILABLE_IN_ALL
GtkWidget * gtk_notebook_get_tab_label    (GtkNotebook *notebook,
					   GtkWidget   *child);
GDK_AVAILABLE_IN_ALL
void gtk_notebook_set_tab_label           (GtkNotebook *notebook,
					   GtkWidget   *child,
					   GtkWidget   *tab_label);
GDK_AVAILABLE_IN_ALL
void          gtk_notebook_set_tab_label_text (GtkNotebook *notebook,
                                               GtkWidget   *child,
                                               const gchar *tab_text);
GDK_AVAILABLE_IN_ALL
const gchar * gtk_notebook_get_tab_label_text (GtkNotebook *notebook,
                                               GtkWidget   *child);
GDK_AVAILABLE_IN_ALL
GtkWidget * gtk_notebook_get_menu_label   (GtkNotebook *notebook,
					   GtkWidget   *child);
GDK_AVAILABLE_IN_ALL
void gtk_notebook_set_menu_label          (GtkNotebook *notebook,
					   GtkWidget   *child,
					   GtkWidget   *menu_label);
GDK_AVAILABLE_IN_ALL
void          gtk_notebook_set_menu_label_text (GtkNotebook *notebook,
                                                GtkWidget   *child,
                                                const gchar *menu_text);
GDK_AVAILABLE_IN_ALL
const gchar * gtk_notebook_get_menu_label_text (GtkNotebook *notebook,
							GtkWidget   *child);
GDK_AVAILABLE_IN_ALL
void gtk_notebook_reorder_child           (GtkNotebook *notebook,
					   GtkWidget   *child,
					   gint         position);
GDK_AVAILABLE_IN_ALL
gboolean gtk_notebook_get_tab_reorderable (GtkNotebook *notebook,
					   GtkWidget   *child);
GDK_AVAILABLE_IN_ALL
void gtk_notebook_set_tab_reorderable     (GtkNotebook *notebook,
					   GtkWidget   *child,
					   gboolean     reorderable);
GDK_AVAILABLE_IN_ALL
gboolean gtk_notebook_get_tab_detachable  (GtkNotebook *notebook,
					   GtkWidget   *child);
GDK_AVAILABLE_IN_ALL
void gtk_notebook_set_tab_detachable      (GtkNotebook *notebook,
					   GtkWidget   *child,
					   gboolean     detachable);
GDK_AVAILABLE_IN_3_16
void gtk_notebook_detach_tab              (GtkNotebook *notebook,
                                           GtkWidget   *child);

GDK_AVAILABLE_IN_ALL
GtkWidget* gtk_notebook_get_action_widget (GtkNotebook *notebook,
                                           GtkPackType  pack_type);
GDK_AVAILABLE_IN_ALL
void       gtk_notebook_set_action_widget (GtkNotebook *notebook,
                                           GtkWidget   *widget,
                                           GtkPackType  pack_type);

G_END_DECLS

#endif /* __GTK_NOTEBOOK_H__ */
