/* GTK - The GIMP Toolkit
 * Copyright (C) 2000 Red Hat, Inc.
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
 * License along with this library; if not, write to the Free
 * Software Foundation, Inc., 675 Mass Ave, Cambridge, MA 02139, USA.
 */

#ifndef __GTK_SETTINGS_H__
#define __GTK_SETTINGS_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkrc.h>

G_BEGIN_DECLS


/* -- type macros --- */
#define GTK_TYPE_SETTINGS             (gtk_settings_get_type ())
#define GTK_SETTINGS(obj)             (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_SETTINGS, GtkSettings))
#define GTK_SETTINGS_CLASS(klass)     (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_SETTINGS, GtkSettingsClass))
#define GTK_IS_SETTINGS(obj)          (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_SETTINGS))
#define GTK_IS_SETTINGS_CLASS(klass)  (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_SETTINGS))
#define GTK_SETTINGS_GET_CLASS(obj)   (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_SETTINGS, GtkSettingsClass))


/* --- typedefs --- */
typedef struct    _GtkSettingsClass GtkSettingsClass;
typedef struct    _GtkSettingsValue GtkSettingsValue;
typedef struct    _GtkSettingsPropertyValue GtkSettingsPropertyValue; /* Internal */


/* --- structures --- */
struct _GtkSettings
{
  GObject parent_instance;

  GData  *GSEAL (queued_settings);	/* of type GtkSettingsValue* */
  GtkSettingsPropertyValue *GSEAL (property_values);

  GtkRcContext *GSEAL (rc_context);
  GdkScreen    *GSEAL (screen);
};

struct _GtkSettingsClass
{
  GObjectClass parent_class;
};

struct _GtkSettingsValue
{
  /* origin should be something like "filename:linenumber" for rc files,
   * or e.g. "XProperty" for other sources
   */
  gchar *origin;

  /* valid types are LONG, DOUBLE and STRING corresponding to the token parsed,
   * or a GSTRING holding an unparsed statement
   */
  GValue value;
};


/* --- functions --- */
GType		gtk_settings_get_type		     (void) G_GNUC_CONST;
#ifndef GDK_MULTIHEAD_SAFE
GtkSettings*	gtk_settings_get_default	     (void);
#endif
GtkSettings*	gtk_settings_get_for_screen	     (GdkScreen *screen);

void		gtk_settings_install_property	     (GParamSpec         *pspec);
void		gtk_settings_install_property_parser (GParamSpec         *pspec,
						      GtkRcPropertyParser parser);

/* --- precoded parsing functions --- */
gboolean gtk_rc_property_parse_color       (const GParamSpec *pspec,
					    const GString    *gstring,
					    GValue           *property_value);
gboolean gtk_rc_property_parse_enum        (const GParamSpec *pspec,
					    const GString    *gstring,
					    GValue           *property_value);
gboolean gtk_rc_property_parse_flags       (const GParamSpec *pspec,
					    const GString    *gstring,
					    GValue           *property_value);
gboolean gtk_rc_property_parse_requisition (const GParamSpec *pspec,
					    const GString    *gstring,
					    GValue           *property_value);
gboolean gtk_rc_property_parse_border      (const GParamSpec *pspec,
					    const GString    *gstring,
					    GValue           *property_value);

/*< private >*/
void		gtk_settings_set_property_value	 (GtkSettings	*settings,
						  const gchar	*name,
						  const GtkSettingsValue *svalue);
void		gtk_settings_set_string_property (GtkSettings	*settings,
						  const gchar	*name,
						  const gchar	*v_string,
						  const gchar   *origin);
void		gtk_settings_set_long_property	 (GtkSettings	*settings,
						  const gchar	*name,
						  glong		 v_long,
						  const gchar   *origin);
void		gtk_settings_set_double_property (GtkSettings	*settings,
						  const gchar	*name,
						  gdouble	 v_double,
						  const gchar   *origin);


/* implementation details */
void _gtk_settings_set_property_value_from_rc (GtkSettings            *settings,
					       const gchar            *name,
					       const GtkSettingsValue *svalue);
void _gtk_settings_reset_rc_values            (GtkSettings            *settings);

void                _gtk_settings_handle_event        (GdkEventSetting    *event);
GtkRcPropertyParser _gtk_rc_property_parser_from_type (GType               type);
gboolean	    _gtk_settings_parse_convert       (GtkRcPropertyParser parser,
						       const GValue       *src_value,
						       GParamSpec         *pspec,
						       GValue	          *dest_value);


G_END_DECLS

#endif /* __GTK_SETTINGS_H__ */
