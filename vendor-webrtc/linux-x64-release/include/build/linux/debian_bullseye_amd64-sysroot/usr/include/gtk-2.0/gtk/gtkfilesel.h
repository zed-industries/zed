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

#ifndef GTK_DISABLE_DEPRECATED

#ifndef __GTK_FILESEL_H__
#define __GTK_FILESEL_H__

#include <gtk/gtk.h>


G_BEGIN_DECLS


#define GTK_TYPE_FILE_SELECTION            (gtk_file_selection_get_type ())
#define GTK_FILE_SELECTION(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_FILE_SELECTION, GtkFileSelection))
#define GTK_FILE_SELECTION_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_FILE_SELECTION, GtkFileSelectionClass))
#define GTK_IS_FILE_SELECTION(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_FILE_SELECTION))
#define GTK_IS_FILE_SELECTION_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_FILE_SELECTION))
#define GTK_FILE_SELECTION_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_FILE_SELECTION, GtkFileSelectionClass))


typedef struct _GtkFileSelection       GtkFileSelection;
typedef struct _GtkFileSelectionClass  GtkFileSelectionClass;

struct _GtkFileSelection
{
  /*< private >*/
  GtkDialog parent_instance;

  /*< public >*/
  GtkWidget *dir_list;
  GtkWidget *file_list;
  GtkWidget *selection_entry;
  GtkWidget *selection_text;
  GtkWidget *main_vbox;
  GtkWidget *ok_button;
  GtkWidget *cancel_button;
  GtkWidget *help_button;
  GtkWidget *history_pulldown;
  GtkWidget *history_menu;
  GList     *history_list;
  GtkWidget *fileop_dialog;
  GtkWidget *fileop_entry;
  gchar     *fileop_file;
  gpointer   cmpl_state;
  
  GtkWidget *fileop_c_dir;
  GtkWidget *fileop_del_file;
  GtkWidget *fileop_ren_file;
  
  GtkWidget *button_area;
  GtkWidget *action_area;

  /*< private >*/
  GPtrArray *selected_names;
  gchar     *last_selected;
};

struct _GtkFileSelectionClass
{
  GtkDialogClass parent_class;

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};


#ifdef G_OS_WIN32
/* Reserve old names for DLL ABI backward compatibility */
#define gtk_file_selection_get_filename gtk_file_selection_get_filename_utf8
#define gtk_file_selection_set_filename gtk_file_selection_set_filename_utf8
#define gtk_file_selection_get_selections gtk_file_selection_get_selections_utf8
#endif

GType      gtk_file_selection_get_type            (void) G_GNUC_CONST;
GtkWidget* gtk_file_selection_new                 (const gchar      *title);
void       gtk_file_selection_set_filename        (GtkFileSelection *filesel,
						   const gchar      *filename);
const gchar* gtk_file_selection_get_filename      (GtkFileSelection *filesel);

void	   gtk_file_selection_complete		  (GtkFileSelection *filesel,
						   const gchar	    *pattern);
void       gtk_file_selection_show_fileop_buttons (GtkFileSelection *filesel);
void       gtk_file_selection_hide_fileop_buttons (GtkFileSelection *filesel);

gchar**    gtk_file_selection_get_selections      (GtkFileSelection *filesel);

void       gtk_file_selection_set_select_multiple (GtkFileSelection *filesel,
						   gboolean          select_multiple);
gboolean   gtk_file_selection_get_select_multiple (GtkFileSelection *filesel);


G_END_DECLS


#endif /* __GTK_FILESEL_H__ */

#endif /* GTK_DISABLE_DEPRECATED */
