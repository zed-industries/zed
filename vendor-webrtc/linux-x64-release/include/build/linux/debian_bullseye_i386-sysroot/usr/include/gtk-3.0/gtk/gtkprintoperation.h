/* GTK - The GIMP Toolkit
 * gtkprintoperation.h: Print Operation
 * Copyright (C) 2006, Red Hat, Inc.
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

#ifndef __GTK_PRINT_OPERATION_H__
#define __GTK_PRINT_OPERATION_H__


#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <cairo.h>
#include <gtk/gtkmain.h>
#include <gtk/gtkwindow.h>
#include <gtk/gtkpagesetup.h>
#include <gtk/gtkprintsettings.h>
#include <gtk/gtkprintcontext.h>
#include <gtk/gtkprintoperationpreview.h>


G_BEGIN_DECLS

#define GTK_TYPE_PRINT_OPERATION                (gtk_print_operation_get_type ())
#define GTK_PRINT_OPERATION(obj)                (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_PRINT_OPERATION, GtkPrintOperation))
#define GTK_PRINT_OPERATION_CLASS(klass)        (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_PRINT_OPERATION, GtkPrintOperationClass))
#define GTK_IS_PRINT_OPERATION(obj)             (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_PRINT_OPERATION))
#define GTK_IS_PRINT_OPERATION_CLASS(klass)     (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_PRINT_OPERATION))
#define GTK_PRINT_OPERATION_GET_CLASS(obj)      (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_PRINT_OPERATION, GtkPrintOperationClass))

typedef struct _GtkPrintOperationClass   GtkPrintOperationClass;
typedef struct _GtkPrintOperationPrivate GtkPrintOperationPrivate;
typedef struct _GtkPrintOperation        GtkPrintOperation;

/**
 * GtkPrintStatus:
 * @GTK_PRINT_STATUS_INITIAL: The printing has not started yet; this
 *     status is set initially, and while the print dialog is shown.
 * @GTK_PRINT_STATUS_PREPARING: This status is set while the begin-print
 *     signal is emitted and during pagination.
 * @GTK_PRINT_STATUS_GENERATING_DATA: This status is set while the
 *     pages are being rendered.
 * @GTK_PRINT_STATUS_SENDING_DATA: The print job is being sent off to the
 *     printer.
 * @GTK_PRINT_STATUS_PENDING: The print job has been sent to the printer,
 *     but is not printed for some reason, e.g. the printer may be stopped.
 * @GTK_PRINT_STATUS_PENDING_ISSUE: Some problem has occurred during
 *     printing, e.g. a paper jam.
 * @GTK_PRINT_STATUS_PRINTING: The printer is processing the print job.
 * @GTK_PRINT_STATUS_FINISHED: The printing has been completed successfully.
 * @GTK_PRINT_STATUS_FINISHED_ABORTED: The printing has been aborted.
 *
 * The status gives a rough indication of the completion of a running
 * print operation.
 */
typedef enum {
  GTK_PRINT_STATUS_INITIAL,
  GTK_PRINT_STATUS_PREPARING,
  GTK_PRINT_STATUS_GENERATING_DATA,
  GTK_PRINT_STATUS_SENDING_DATA,
  GTK_PRINT_STATUS_PENDING,
  GTK_PRINT_STATUS_PENDING_ISSUE,
  GTK_PRINT_STATUS_PRINTING,
  GTK_PRINT_STATUS_FINISHED,
  GTK_PRINT_STATUS_FINISHED_ABORTED
} GtkPrintStatus;

/**
 * GtkPrintOperationResult:
 * @GTK_PRINT_OPERATION_RESULT_ERROR: An error has occurred.
 * @GTK_PRINT_OPERATION_RESULT_APPLY: The print settings should be stored.
 * @GTK_PRINT_OPERATION_RESULT_CANCEL: The print operation has been canceled,
 *     the print settings should not be stored.
 * @GTK_PRINT_OPERATION_RESULT_IN_PROGRESS: The print operation is not complete
 *     yet. This value will only be returned when running asynchronously.
 *
 * A value of this type is returned by gtk_print_operation_run().
 */
typedef enum {
  GTK_PRINT_OPERATION_RESULT_ERROR,
  GTK_PRINT_OPERATION_RESULT_APPLY,
  GTK_PRINT_OPERATION_RESULT_CANCEL,
  GTK_PRINT_OPERATION_RESULT_IN_PROGRESS
} GtkPrintOperationResult;

/**
 * GtkPrintOperationAction:
 * @GTK_PRINT_OPERATION_ACTION_PRINT_DIALOG: Show the print dialog.
 * @GTK_PRINT_OPERATION_ACTION_PRINT: Start to print without showing
 *     the print dialog, based on the current print settings.
 * @GTK_PRINT_OPERATION_ACTION_PREVIEW: Show the print preview.
 * @GTK_PRINT_OPERATION_ACTION_EXPORT: Export to a file. This requires
 *     the export-filename property to be set.
 *
 * The @action parameter to gtk_print_operation_run()
 * determines what action the print operation should perform.
 */
typedef enum {
  GTK_PRINT_OPERATION_ACTION_PRINT_DIALOG,
  GTK_PRINT_OPERATION_ACTION_PRINT,
  GTK_PRINT_OPERATION_ACTION_PREVIEW,
  GTK_PRINT_OPERATION_ACTION_EXPORT
} GtkPrintOperationAction;


struct _GtkPrintOperation
{
  GObject parent_instance;

  /*< private >*/
  GtkPrintOperationPrivate *priv;
};

/**
 * GtkPrintOperationClass:
 * @parent_class: The parent class.
 * @done: Signal emitted when the print operation run has finished
 *    doing everything required for printing.
 * @begin_print: Signal emitted after the user has finished changing
 *    print settings in the dialog, before the actual rendering starts.
 * @paginate: Signal emitted after the “begin-print” signal, but
 *    before the actual rendering starts.
 * @request_page_setup: Emitted once for every page that is printed,
 *    to give the application a chance to modify the page setup.
 * @draw_page: Signal emitted for every page that is printed.
 * @end_print: Signal emitted after all pages have been rendered.
 * @status_changed: Emitted at between the various phases of the print
 *    operation.
 * @create_custom_widget: Signal emitted when displaying the print dialog.
 * @custom_widget_apply: Signal emitted right before “begin-print” if
 *    you added a custom widget in the “create-custom-widget” handler.
 * @preview: Signal emitted when a preview is requested from the
 *    native dialog.
 * @update_custom_widget: Emitted after change of selected printer.
 */
struct _GtkPrintOperationClass
{
  GObjectClass parent_class;

  /*< public >*/

  void     (*done)               (GtkPrintOperation *operation,
                                  GtkPrintOperationResult result);
  void     (*begin_print)        (GtkPrintOperation *operation,
                                  GtkPrintContext   *context);
  gboolean (*paginate)           (GtkPrintOperation *operation,
                                  GtkPrintContext   *context);
  void     (*request_page_setup) (GtkPrintOperation *operation,
                                  GtkPrintContext   *context,
                                  gint               page_nr,
                                  GtkPageSetup      *setup);
  void     (*draw_page)          (GtkPrintOperation *operation,
                                  GtkPrintContext   *context,
                                  gint               page_nr);
  void     (*end_print)          (GtkPrintOperation *operation,
                                  GtkPrintContext   *context);
  void     (*status_changed)     (GtkPrintOperation *operation);

  GtkWidget *(*create_custom_widget) (GtkPrintOperation *operation);
  void       (*custom_widget_apply)  (GtkPrintOperation *operation,
                                      GtkWidget         *widget);

  gboolean (*preview)        (GtkPrintOperation        *operation,
                              GtkPrintOperationPreview *preview,
                              GtkPrintContext          *context,
                              GtkWindow                *parent);

  void     (*update_custom_widget) (GtkPrintOperation *operation,
                                    GtkWidget         *widget,
                                    GtkPageSetup      *setup,
                                    GtkPrintSettings  *settings);

  /*< private >*/

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

/**
 * GTK_PRINT_ERROR:
 *
 * The error domain for #GtkPrintError errors.
 */
#define GTK_PRINT_ERROR gtk_print_error_quark ()

/**
 * GtkPrintError:
 * @GTK_PRINT_ERROR_GENERAL: An unspecified error occurred.
 * @GTK_PRINT_ERROR_INTERNAL_ERROR: An internal error occurred.
 * @GTK_PRINT_ERROR_NOMEM: A memory allocation failed.
 * @GTK_PRINT_ERROR_INVALID_FILE: An error occurred while loading a page setup
 *     or paper size from a key file.
 *
 * Error codes that identify various errors that can occur while
 * using the GTK+ printing support.
 */
typedef enum
{
  GTK_PRINT_ERROR_GENERAL,
  GTK_PRINT_ERROR_INTERNAL_ERROR,
  GTK_PRINT_ERROR_NOMEM,
  GTK_PRINT_ERROR_INVALID_FILE
} GtkPrintError;

GDK_AVAILABLE_IN_ALL
GQuark gtk_print_error_quark (void);

GDK_AVAILABLE_IN_ALL
GType                   gtk_print_operation_get_type               (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkPrintOperation *     gtk_print_operation_new                    (void);
GDK_AVAILABLE_IN_ALL
void                    gtk_print_operation_set_default_page_setup (GtkPrintOperation  *op,
                                                                    GtkPageSetup       *default_page_setup);
GDK_AVAILABLE_IN_ALL
GtkPageSetup *          gtk_print_operation_get_default_page_setup (GtkPrintOperation  *op);
GDK_AVAILABLE_IN_ALL
void                    gtk_print_operation_set_print_settings     (GtkPrintOperation  *op,
                                                                    GtkPrintSettings   *print_settings);
GDK_AVAILABLE_IN_ALL
GtkPrintSettings *      gtk_print_operation_get_print_settings     (GtkPrintOperation  *op);
GDK_AVAILABLE_IN_ALL
void                    gtk_print_operation_set_job_name           (GtkPrintOperation  *op,
                                                                    const gchar        *job_name);
GDK_AVAILABLE_IN_ALL
void                    gtk_print_operation_set_n_pages            (GtkPrintOperation  *op,
                                                                    gint                n_pages);
GDK_AVAILABLE_IN_ALL
void                    gtk_print_operation_set_current_page       (GtkPrintOperation  *op,
                                                                    gint                current_page);
GDK_AVAILABLE_IN_ALL
void                    gtk_print_operation_set_use_full_page      (GtkPrintOperation  *op,
                                                                    gboolean            full_page);
GDK_AVAILABLE_IN_ALL
void                    gtk_print_operation_set_unit               (GtkPrintOperation  *op,
                                                                    GtkUnit             unit);
GDK_AVAILABLE_IN_ALL
void                    gtk_print_operation_set_export_filename    (GtkPrintOperation  *op,
                                                                    const gchar        *filename);
GDK_AVAILABLE_IN_ALL
void                    gtk_print_operation_set_track_print_status (GtkPrintOperation  *op,
                                                                    gboolean            track_status);
GDK_AVAILABLE_IN_ALL
void                    gtk_print_operation_set_show_progress      (GtkPrintOperation  *op,
                                                                    gboolean            show_progress);
GDK_AVAILABLE_IN_ALL
void                    gtk_print_operation_set_allow_async        (GtkPrintOperation  *op,
                                                                    gboolean            allow_async);
GDK_AVAILABLE_IN_ALL
void                    gtk_print_operation_set_custom_tab_label   (GtkPrintOperation  *op,
                                                                    const gchar        *label);
GDK_AVAILABLE_IN_ALL
GtkPrintOperationResult gtk_print_operation_run                    (GtkPrintOperation  *op,
                                                                    GtkPrintOperationAction action,
                                                                    GtkWindow          *parent,
                                                                    GError            **error);
GDK_AVAILABLE_IN_ALL
void                    gtk_print_operation_get_error              (GtkPrintOperation  *op,
                                                                    GError            **error);
GDK_AVAILABLE_IN_ALL
GtkPrintStatus          gtk_print_operation_get_status             (GtkPrintOperation  *op);
GDK_AVAILABLE_IN_ALL
const gchar *           gtk_print_operation_get_status_string      (GtkPrintOperation  *op);
GDK_AVAILABLE_IN_ALL
gboolean                gtk_print_operation_is_finished            (GtkPrintOperation  *op);
GDK_AVAILABLE_IN_ALL
void                    gtk_print_operation_cancel                 (GtkPrintOperation  *op);
GDK_AVAILABLE_IN_ALL
void                    gtk_print_operation_draw_page_finish       (GtkPrintOperation  *op);
GDK_AVAILABLE_IN_ALL
void                    gtk_print_operation_set_defer_drawing      (GtkPrintOperation  *op);
GDK_AVAILABLE_IN_ALL
void                    gtk_print_operation_set_support_selection  (GtkPrintOperation  *op,
                                                                    gboolean            support_selection);
GDK_AVAILABLE_IN_ALL
gboolean                gtk_print_operation_get_support_selection  (GtkPrintOperation  *op);
GDK_AVAILABLE_IN_ALL
void                    gtk_print_operation_set_has_selection      (GtkPrintOperation  *op,
                                                                    gboolean            has_selection);
GDK_AVAILABLE_IN_ALL
gboolean                gtk_print_operation_get_has_selection      (GtkPrintOperation  *op);
GDK_AVAILABLE_IN_ALL
void                    gtk_print_operation_set_embed_page_setup   (GtkPrintOperation  *op,
                                                                    gboolean            embed);
GDK_AVAILABLE_IN_ALL
gboolean                gtk_print_operation_get_embed_page_setup   (GtkPrintOperation  *op);
GDK_AVAILABLE_IN_ALL
gint                    gtk_print_operation_get_n_pages_to_print   (GtkPrintOperation  *op);

GDK_AVAILABLE_IN_ALL
GtkPageSetup           *gtk_print_run_page_setup_dialog            (GtkWindow          *parent,
                                                                    GtkPageSetup       *page_setup,
                                                                    GtkPrintSettings   *settings);

/**
 * GtkPageSetupDoneFunc:
 * @page_setup: the #GtkPageSetup that has been
 * @data: (closure): user data that has been passed to
 *     gtk_print_run_page_setup_dialog_async()
 *
 * The type of function that is passed to
 * gtk_print_run_page_setup_dialog_async().
 *
 * This function will be called when the page setup dialog
 * is dismissed, and also serves as destroy notify for @data.
 */
typedef void  (* GtkPageSetupDoneFunc) (GtkPageSetup *page_setup,
                                        gpointer      data);

GDK_AVAILABLE_IN_ALL
void                    gtk_print_run_page_setup_dialog_async      (GtkWindow            *parent,
                                                                    GtkPageSetup         *page_setup,
                                                                    GtkPrintSettings     *settings,
                                                                    GtkPageSetupDoneFunc  done_cb,
                                                                    gpointer              data);

G_END_DECLS

#endif /* __GTK_PRINT_OPERATION_H__ */
