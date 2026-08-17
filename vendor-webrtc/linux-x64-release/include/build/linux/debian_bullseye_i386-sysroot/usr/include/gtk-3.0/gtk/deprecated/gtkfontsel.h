/* GTK - The GIMP Toolkit
 * Copyright (C) 1995-1997 Peter Mattis, Spencer Kimball and Josh MacDonald
 *
 * GtkFontSelection widget for Gtk+, by Damon Chaplin, May 1998.
 * Based on the GnomeFontSelector widget, by Elliot Lee, but major changes.
 * The GnomeFontSelector was derived from app/text_tool.c in the GIMP.
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

#ifndef __GTK_FONTSEL_H__
#define __GTK_FONTSEL_H__


#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkdialog.h>
#include <gtk/gtkbox.h>


G_BEGIN_DECLS

#define GTK_TYPE_FONT_SELECTION              (gtk_font_selection_get_type ())
#define GTK_FONT_SELECTION(obj)              (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_FONT_SELECTION, GtkFontSelection))
#define GTK_FONT_SELECTION_CLASS(klass)      (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_FONT_SELECTION, GtkFontSelectionClass))
#define GTK_IS_FONT_SELECTION(obj)           (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_FONT_SELECTION))
#define GTK_IS_FONT_SELECTION_CLASS(klass)   (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_FONT_SELECTION))
#define GTK_FONT_SELECTION_GET_CLASS(obj)    (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_FONT_SELECTION, GtkFontSelectionClass))


#define GTK_TYPE_FONT_SELECTION_DIALOG              (gtk_font_selection_dialog_get_type ())
#define GTK_FONT_SELECTION_DIALOG(obj)              (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_FONT_SELECTION_DIALOG, GtkFontSelectionDialog))
#define GTK_FONT_SELECTION_DIALOG_CLASS(klass)      (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_FONT_SELECTION_DIALOG, GtkFontSelectionDialogClass))
#define GTK_IS_FONT_SELECTION_DIALOG(obj)           (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_FONT_SELECTION_DIALOG))
#define GTK_IS_FONT_SELECTION_DIALOG_CLASS(klass)   (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_FONT_SELECTION_DIALOG))
#define GTK_FONT_SELECTION_DIALOG_GET_CLASS(obj)    (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_FONT_SELECTION_DIALOG, GtkFontSelectionDialogClass))


typedef struct _GtkFontSelection              GtkFontSelection;
typedef struct _GtkFontSelectionPrivate       GtkFontSelectionPrivate;
typedef struct _GtkFontSelectionClass         GtkFontSelectionClass;

typedef struct _GtkFontSelectionDialog              GtkFontSelectionDialog;
typedef struct _GtkFontSelectionDialogPrivate       GtkFontSelectionDialogPrivate;
typedef struct _GtkFontSelectionDialogClass         GtkFontSelectionDialogClass;

struct _GtkFontSelection
{
  GtkBox parent_instance;

  /*< private >*/
  GtkFontSelectionPrivate *priv;
};

struct _GtkFontSelectionClass
{
  GtkBoxClass parent_class;

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};


struct _GtkFontSelectionDialog
{
  GtkDialog parent_instance;

  /*< private >*/
  GtkFontSelectionDialogPrivate *priv;
};

struct _GtkFontSelectionDialogClass
{
  GtkDialogClass parent_class;

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};

GDK_DEPRECATED_IN_3_2
GType        gtk_font_selection_get_type          (void) G_GNUC_CONST;
GDK_DEPRECATED_IN_3_2_FOR(GtkFontChooser)
GtkWidget *  gtk_font_selection_new               (void);
GDK_DEPRECATED_IN_3_2_FOR(GtkFontChooser)
GtkWidget *  gtk_font_selection_get_family_list   (GtkFontSelection *fontsel);
GDK_DEPRECATED_IN_3_2_FOR(GtkFontChooser)
GtkWidget *  gtk_font_selection_get_face_list     (GtkFontSelection *fontsel);
GDK_DEPRECATED_IN_3_2_FOR(GtkFontChooser)
GtkWidget *  gtk_font_selection_get_size_entry    (GtkFontSelection *fontsel);
GDK_DEPRECATED_IN_3_2_FOR(GtkFontChooser)
GtkWidget *  gtk_font_selection_get_size_list     (GtkFontSelection *fontsel);
GDK_DEPRECATED_IN_3_2_FOR(GtkFontChooser)
GtkWidget *  gtk_font_selection_get_preview_entry (GtkFontSelection *fontsel);
GDK_DEPRECATED_IN_3_2_FOR(GtkFontChooser)
PangoFontFamily *
             gtk_font_selection_get_family        (GtkFontSelection *fontsel);
GDK_DEPRECATED_IN_3_2_FOR(GtkFontChooser)
PangoFontFace *
             gtk_font_selection_get_face          (GtkFontSelection *fontsel);
GDK_DEPRECATED_IN_3_2_FOR(GtkFontChooser)
gint         gtk_font_selection_get_size          (GtkFontSelection *fontsel);
GDK_DEPRECATED_IN_3_2_FOR(GtkFontChooser)
gchar*       gtk_font_selection_get_font_name     (GtkFontSelection *fontsel);

GDK_DEPRECATED_IN_3_2_FOR(GtkFontChooser)
gboolean     gtk_font_selection_set_font_name     (GtkFontSelection *fontsel,
                                                   const gchar      *fontname);
GDK_DEPRECATED_IN_3_2_FOR(GtkFontChooser)
const gchar* gtk_font_selection_get_preview_text  (GtkFontSelection *fontsel);
GDK_DEPRECATED_IN_3_2_FOR(GtkFontChooser)
void         gtk_font_selection_set_preview_text  (GtkFontSelection *fontsel,
                                                   const gchar      *text);

GDK_DEPRECATED_IN_3_2
GType      gtk_font_selection_dialog_get_type          (void) G_GNUC_CONST;
GDK_DEPRECATED_IN_3_2_FOR(GtkFontChooser)
GtkWidget *gtk_font_selection_dialog_new               (const gchar            *title);

GDK_DEPRECATED_IN_3_2_FOR(GtkFontChooser)
GtkWidget *gtk_font_selection_dialog_get_ok_button     (GtkFontSelectionDialog *fsd);
GDK_DEPRECATED_IN_3_2_FOR(GtkFontChooser)
GtkWidget *gtk_font_selection_dialog_get_cancel_button (GtkFontSelectionDialog *fsd);
GDK_DEPRECATED_IN_3_2_FOR(GtkFontChooser)
GtkWidget *gtk_font_selection_dialog_get_font_selection (GtkFontSelectionDialog *fsd);
GDK_DEPRECATED_IN_3_2_FOR(GtkFontChooser)
gchar*     gtk_font_selection_dialog_get_font_name     (GtkFontSelectionDialog *fsd);
GDK_DEPRECATED_IN_3_2_FOR(GtkFontChooser)
gboolean   gtk_font_selection_dialog_set_font_name     (GtkFontSelectionDialog *fsd,
                                                        const gchar            *fontname);
GDK_DEPRECATED_IN_3_2_FOR(GtkFontChooser)
const gchar*
          gtk_font_selection_dialog_get_preview_text   (GtkFontSelectionDialog *fsd);
GDK_DEPRECATED_IN_3_2_FOR(GtkFontChooser)
void      gtk_font_selection_dialog_set_preview_text   (GtkFontSelectionDialog *fsd,
                                                        const gchar            *text);

G_END_DECLS


#endif /* __GTK_FONTSEL_H__ */
