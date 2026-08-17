/* GtkPrintJob
 * Copyright (C) 2006 Red Hat,Inc.
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
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 */

#ifndef __GTK_PRINT_JOB_H__
#define __GTK_PRINT_JOB_H__

#if !defined (__GTK_UNIX_PRINT_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtkunixprint.h> can be included directly."
#endif

#include <cairo.h>

#include <gtk/gtk.h>
#include <gtk/gtkprinter.h>

G_BEGIN_DECLS

#define GTK_TYPE_PRINT_JOB                  (gtk_print_job_get_type ())
#define GTK_PRINT_JOB(obj)                  (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_PRINT_JOB, GtkPrintJob))
#define GTK_PRINT_JOB_CLASS(klass)          (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_PRINT_JOB, GtkPrintJobClass))
#define GTK_IS_PRINT_JOB(obj)               (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_PRINT_JOB))
#define GTK_IS_PRINT_JOB_CLASS(klass)       (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_PRINT_JOB))
#define GTK_PRINT_JOB_GET_CLASS(obj)        (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_PRINT_JOB, GtkPrintJobClass))

typedef struct _GtkPrintJob          GtkPrintJob;
typedef struct _GtkPrintJobClass     GtkPrintJobClass;
typedef struct _GtkPrintJobPrivate   GtkPrintJobPrivate;

/**
 * GtkPrintJobCompleteFunc:
 * @print_job: the #GtkPrintJob
 * @user_data: user data that has been passed to gtk_print_job_send()
 * @error: a #GError that contains error information if the sending
 *     of the print job failed, otherwise %NULL
 *
 * The type of callback that is passed to gtk_print_job_send().
 * It is called when the print job has been completely sent.
 */
typedef void (*GtkPrintJobCompleteFunc) (GtkPrintJob  *print_job,
                                         gpointer      user_data,
                                         const GError *error);

struct _GtkPrinter;

struct _GtkPrintJob
{
  GObject parent_instance;

  GtkPrintJobPrivate *priv;
};

struct _GtkPrintJobClass
{
  GObjectClass parent_class;

  void (*status_changed) (GtkPrintJob *job);

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};

GDK_AVAILABLE_IN_ALL
GType                    gtk_print_job_get_type               (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkPrintJob             *gtk_print_job_new                    (const gchar              *title,
							       GtkPrinter               *printer,
							       GtkPrintSettings         *settings,
							       GtkPageSetup             *page_setup);
GDK_AVAILABLE_IN_ALL
GtkPrintSettings        *gtk_print_job_get_settings           (GtkPrintJob              *job);
GDK_AVAILABLE_IN_ALL
GtkPrinter              *gtk_print_job_get_printer            (GtkPrintJob              *job);
GDK_AVAILABLE_IN_ALL
const gchar *            gtk_print_job_get_title              (GtkPrintJob              *job);
GDK_AVAILABLE_IN_ALL
GtkPrintStatus           gtk_print_job_get_status             (GtkPrintJob              *job);
GDK_AVAILABLE_IN_ALL
gboolean                 gtk_print_job_set_source_file        (GtkPrintJob              *job,
							       const gchar              *filename,
							       GError                  **error);
GDK_AVAILABLE_IN_3_22
gboolean                 gtk_print_job_set_source_fd          (GtkPrintJob              *job,
							       int                       fd,
							       GError                  **error);
GDK_AVAILABLE_IN_ALL
cairo_surface_t         *gtk_print_job_get_surface            (GtkPrintJob              *job,
							       GError                  **error);
GDK_AVAILABLE_IN_ALL
void                     gtk_print_job_set_track_print_status (GtkPrintJob              *job,
							       gboolean                  track_status);
GDK_AVAILABLE_IN_ALL
gboolean                 gtk_print_job_get_track_print_status (GtkPrintJob              *job);
GDK_AVAILABLE_IN_ALL
void                     gtk_print_job_send                   (GtkPrintJob              *job,
							       GtkPrintJobCompleteFunc   callback,
							       gpointer                  user_data,
							       GDestroyNotify            dnotify);

GDK_AVAILABLE_IN_ALL
GtkPrintPages     gtk_print_job_get_pages       (GtkPrintJob       *job);
GDK_AVAILABLE_IN_ALL
void              gtk_print_job_set_pages       (GtkPrintJob       *job,
                                                 GtkPrintPages      pages);
GDK_AVAILABLE_IN_ALL
GtkPageRange *    gtk_print_job_get_page_ranges (GtkPrintJob       *job,
                                                 gint              *n_ranges);
GDK_AVAILABLE_IN_ALL
void              gtk_print_job_set_page_ranges (GtkPrintJob       *job,
                                                 GtkPageRange      *ranges,
                                                 gint               n_ranges);
GDK_AVAILABLE_IN_ALL
GtkPageSet        gtk_print_job_get_page_set    (GtkPrintJob       *job);
GDK_AVAILABLE_IN_ALL
void              gtk_print_job_set_page_set    (GtkPrintJob       *job,
                                                 GtkPageSet         page_set);
GDK_AVAILABLE_IN_ALL
gint              gtk_print_job_get_num_copies  (GtkPrintJob       *job);
GDK_AVAILABLE_IN_ALL
void              gtk_print_job_set_num_copies  (GtkPrintJob       *job,
                                                 gint               num_copies);
GDK_AVAILABLE_IN_ALL
gdouble           gtk_print_job_get_scale       (GtkPrintJob       *job);
GDK_AVAILABLE_IN_ALL
void              gtk_print_job_set_scale       (GtkPrintJob       *job,
                                                 gdouble            scale);
GDK_AVAILABLE_IN_ALL
guint             gtk_print_job_get_n_up        (GtkPrintJob       *job);
GDK_AVAILABLE_IN_ALL
void              gtk_print_job_set_n_up        (GtkPrintJob       *job,
                                                 guint              n_up);
GDK_AVAILABLE_IN_ALL
GtkNumberUpLayout gtk_print_job_get_n_up_layout (GtkPrintJob       *job);
GDK_AVAILABLE_IN_ALL
void              gtk_print_job_set_n_up_layout (GtkPrintJob       *job,
                                                 GtkNumberUpLayout  layout);
GDK_AVAILABLE_IN_ALL
gboolean          gtk_print_job_get_rotate      (GtkPrintJob       *job);
GDK_AVAILABLE_IN_ALL
void              gtk_print_job_set_rotate      (GtkPrintJob       *job,
                                                 gboolean           rotate);
GDK_AVAILABLE_IN_ALL
gboolean          gtk_print_job_get_collate     (GtkPrintJob       *job);
GDK_AVAILABLE_IN_ALL
void              gtk_print_job_set_collate     (GtkPrintJob       *job,
                                                 gboolean           collate);
GDK_AVAILABLE_IN_ALL
gboolean          gtk_print_job_get_reverse     (GtkPrintJob       *job);
GDK_AVAILABLE_IN_ALL
void              gtk_print_job_set_reverse     (GtkPrintJob       *job,
                                                 gboolean           reverse);

G_END_DECLS

#endif /* __GTK_PRINT_JOB_H__ */
