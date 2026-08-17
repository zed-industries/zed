/* GtkPrinter
 * Copyright (C) 2006 John (J5) Palmieri <johnp@redhat.com>
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

#ifndef __GTK_PRINTER_H__
#define __GTK_PRINTER_H__

#if !defined (__GTK_UNIX_PRINT_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtkunixprint.h> can be included directly."
#endif

#include <cairo.h>
#include <gtk/gtk.h>

G_BEGIN_DECLS

#define GTK_TYPE_PRINT_CAPABILITIES (gtk_print_capabilities_get_type ())

/* Note, this type is manually registered with GObject in gtkprinter.c
 * If you add any flags, update the registration as well!
 */
/**
 * GtkPrintCapabilities:
 * @GTK_PRINT_CAPABILITY_PAGE_SET: Print dialog will offer printing even/odd pages.
 * @GTK_PRINT_CAPABILITY_COPIES: Print dialog will allow to print multiple copies.
 * @GTK_PRINT_CAPABILITY_COLLATE: Print dialog will allow to collate multiple copies.
 * @GTK_PRINT_CAPABILITY_REVERSE: Print dialog will allow to print pages in reverse order.
 * @GTK_PRINT_CAPABILITY_SCALE: Print dialog will allow to scale the output.
 * @GTK_PRINT_CAPABILITY_GENERATE_PDF: The program will send the document to
 *   the printer in PDF format
 * @GTK_PRINT_CAPABILITY_GENERATE_PS: The program will send the document to
 *   the printer in Postscript format
 * @GTK_PRINT_CAPABILITY_PREVIEW: Print dialog will offer a preview
 * @GTK_PRINT_CAPABILITY_NUMBER_UP: Print dialog will offer printing multiple
 *   pages per sheet. Since 2.12
 * @GTK_PRINT_CAPABILITY_NUMBER_UP_LAYOUT: Print dialog will allow to rearrange
 *   pages when printing multiple pages per sheet. Since 2.14
 *
 * An enum for specifying which features the print dialog should offer.
 * If neither %GTK_PRINT_CAPABILITY_GENERATE_PDF nor
 * %GTK_PRINT_CAPABILITY_GENERATE_PS is specified, GTK+ assumes that all
 * formats are supported.
 */
typedef enum
{
  GTK_PRINT_CAPABILITY_PAGE_SET         = 1 << 0,
  GTK_PRINT_CAPABILITY_COPIES           = 1 << 1,
  GTK_PRINT_CAPABILITY_COLLATE          = 1 << 2,
  GTK_PRINT_CAPABILITY_REVERSE          = 1 << 3,
  GTK_PRINT_CAPABILITY_SCALE            = 1 << 4,
  GTK_PRINT_CAPABILITY_GENERATE_PDF     = 1 << 5,
  GTK_PRINT_CAPABILITY_GENERATE_PS      = 1 << 6,
  GTK_PRINT_CAPABILITY_PREVIEW          = 1 << 7,
  GTK_PRINT_CAPABILITY_NUMBER_UP        = 1 << 8,
  GTK_PRINT_CAPABILITY_NUMBER_UP_LAYOUT = 1 << 9
} GtkPrintCapabilities;

GDK_AVAILABLE_IN_ALL
GType gtk_print_capabilities_get_type (void) G_GNUC_CONST;

#define GTK_TYPE_PRINTER                  (gtk_printer_get_type ())
#define GTK_PRINTER(obj)                  (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_PRINTER, GtkPrinter))
#define GTK_PRINTER_CLASS(klass)          (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_PRINTER, GtkPrinterClass))
#define GTK_IS_PRINTER(obj)               (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_PRINTER))
#define GTK_IS_PRINTER_CLASS(klass)       (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_PRINTER))
#define GTK_PRINTER_GET_CLASS(obj)        (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_PRINTER, GtkPrinterClass))

typedef struct _GtkPrinter          GtkPrinter;
typedef struct _GtkPrinterClass     GtkPrinterClass;
typedef struct _GtkPrinterPrivate   GtkPrinterPrivate;
typedef struct _GtkPrintBackend     GtkPrintBackend;

struct _GtkPrintBackend;

struct _GtkPrinter
{
  GObject parent_instance;

  /*< private >*/
  GtkPrinterPrivate *priv;
};

struct _GtkPrinterClass
{
  GObjectClass parent_class;

  void (*details_acquired) (GtkPrinter *printer,
                            gboolean    success);

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

GDK_AVAILABLE_IN_ALL
GType                    gtk_printer_get_type              (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkPrinter              *gtk_printer_new                   (const gchar     *name,
							    GtkPrintBackend *backend,
							    gboolean         virtual_);
GDK_AVAILABLE_IN_ALL
GtkPrintBackend         *gtk_printer_get_backend           (GtkPrinter      *printer);
GDK_AVAILABLE_IN_ALL
const gchar *            gtk_printer_get_name              (GtkPrinter      *printer);
GDK_AVAILABLE_IN_ALL
const gchar *            gtk_printer_get_state_message     (GtkPrinter      *printer);
GDK_AVAILABLE_IN_ALL
const gchar *            gtk_printer_get_description       (GtkPrinter      *printer);
GDK_AVAILABLE_IN_ALL
const gchar *            gtk_printer_get_location          (GtkPrinter      *printer);
GDK_AVAILABLE_IN_ALL
const gchar *            gtk_printer_get_icon_name         (GtkPrinter      *printer);
GDK_AVAILABLE_IN_ALL
gint                     gtk_printer_get_job_count         (GtkPrinter      *printer);
GDK_AVAILABLE_IN_ALL
gboolean                 gtk_printer_is_active             (GtkPrinter      *printer);
GDK_AVAILABLE_IN_ALL
gboolean                 gtk_printer_is_paused             (GtkPrinter      *printer);
GDK_AVAILABLE_IN_ALL
gboolean                 gtk_printer_is_accepting_jobs     (GtkPrinter      *printer);
GDK_AVAILABLE_IN_ALL
gboolean                 gtk_printer_is_virtual            (GtkPrinter      *printer);
GDK_AVAILABLE_IN_ALL
gboolean                 gtk_printer_is_default            (GtkPrinter      *printer);
GDK_AVAILABLE_IN_ALL
gboolean                 gtk_printer_accepts_pdf           (GtkPrinter      *printer);
GDK_AVAILABLE_IN_ALL
gboolean                 gtk_printer_accepts_ps            (GtkPrinter      *printer);
GDK_AVAILABLE_IN_ALL
GList                   *gtk_printer_list_papers           (GtkPrinter      *printer);
GDK_AVAILABLE_IN_ALL
GtkPageSetup            *gtk_printer_get_default_page_size (GtkPrinter      *printer);
GDK_AVAILABLE_IN_ALL
gint                     gtk_printer_compare               (GtkPrinter *a,
						    	    GtkPrinter *b);
GDK_AVAILABLE_IN_ALL
gboolean                 gtk_printer_has_details           (GtkPrinter       *printer);
GDK_AVAILABLE_IN_ALL
void                     gtk_printer_request_details       (GtkPrinter       *printer);
GDK_AVAILABLE_IN_ALL
GtkPrintCapabilities     gtk_printer_get_capabilities      (GtkPrinter       *printer);
GDK_AVAILABLE_IN_ALL
gboolean                 gtk_printer_get_hard_margins      (GtkPrinter       *printer,
                                                            gdouble          *top,
                                                            gdouble          *bottom,
                                                            gdouble          *left,
                                                            gdouble          *right);

/**
 * GtkPrinterFunc:
 * @printer: a #GtkPrinter
 * @data: (closure): user data passed to gtk_enumerate_printers()
 *
 * The type of function passed to gtk_enumerate_printers().
 * Note that you need to ref @printer, if you want to keep
 * a reference to it after the function has returned.
 *
 * Returns: %TRUE to stop the enumeration, %FALSE to continue
 *
 * Since: 2.10
 */
typedef gboolean (*GtkPrinterFunc) (GtkPrinter *printer,
				    gpointer    data);

GDK_AVAILABLE_IN_ALL
void                     gtk_enumerate_printers        (GtkPrinterFunc   func,
							gpointer         data,
							GDestroyNotify   destroy,
							gboolean         wait);

G_END_DECLS

#endif /* __GTK_PRINTER_H__ */
