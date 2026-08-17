/* GTK - The GIMP Toolkit
 * Copyright (C) 1995-1997 Peter Mattis, Spencer Kimball and Josh MacDonald
 *
 * GTK Calendar Widget
 * Copyright (C) 1998 Cesar Miquel and Shawn T. Amundson
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
 * License along with this library; if not, write to the Free
 * Software Foundation, Inc., 675 Mass Ave, Cambridge, MA 02139, USA.
 */

/*
 * Modified by the GTK+ Team and others 1997-2000.  See the AUTHORS
 * file for a list of people on the GTK+ Team.  See the ChangeLog
 * files for a list of changes.  These files are distributed with
 * GTK+ at ftp://ftp.gtk.org/pub/gtk/.
 */

#ifndef __GTK_CALENDAR_H__
#define __GTK_CALENDAR_H__


#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkwidget.h>

/* Not needed, retained for compatibility -Yosh */
#include <gtk/gtksignal.h>


G_BEGIN_DECLS

#define GTK_TYPE_CALENDAR                  (gtk_calendar_get_type ())
#define GTK_CALENDAR(obj)                  (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_CALENDAR, GtkCalendar))
#define GTK_CALENDAR_CLASS(klass)          (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_CALENDAR, GtkCalendarClass))
#define GTK_IS_CALENDAR(obj)               (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_CALENDAR))
#define GTK_IS_CALENDAR_CLASS(klass)       (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_CALENDAR))
#define GTK_CALENDAR_GET_CLASS(obj)        (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_CALENDAR, GtkCalendarClass))


typedef struct _GtkCalendar	       GtkCalendar;
typedef struct _GtkCalendarClass       GtkCalendarClass;

typedef struct _GtkCalendarPrivate     GtkCalendarPrivate;

/**
 * GtkCalendarDisplayOptions:
 * @GTK_CALENDAR_SHOW_HEADING: Specifies that the month and year should be displayed.
 * @GTK_CALENDAR_SHOW_DAY_NAMES: Specifies that three letter day descriptions should be present.
 * @GTK_CALENDAR_NO_MONTH_CHANGE: Prevents the user from switching months with the calendar.
 * @GTK_CALENDAR_SHOW_WEEK_NUMBERS: Displays each week numbers of the current year, down the
 * left side of the calendar.
 * @GTK_CALENDAR_WEEK_START_MONDAY: Since GTK+ 2.4, this option is deprecated and ignored by GTK+.
 * The information on which day the calendar week starts is derived from the locale.
 * @GTK_CALENDAR_SHOW_DETAILS: Just show an indicator, not the full details
 * text when details are provided. See gtk_calendar_set_detail_func().
 *
 * These options can be used to influence the display and behaviour of a #GtkCalendar.
 */
typedef enum
{
  GTK_CALENDAR_SHOW_HEADING		= 1 << 0,
  GTK_CALENDAR_SHOW_DAY_NAMES		= 1 << 1,
  GTK_CALENDAR_NO_MONTH_CHANGE		= 1 << 2,
  GTK_CALENDAR_SHOW_WEEK_NUMBERS	= 1 << 3,
  GTK_CALENDAR_WEEK_START_MONDAY	= 1 << 4,
  GTK_CALENDAR_SHOW_DETAILS		= 1 << 5
} GtkCalendarDisplayOptions;

/**
 * GtkCalendarDetailFunc:
 * @calendar: a #GtkCalendar.
 * @year: the year for which details are needed.
 * @month: the month for which details are needed.
 * @day: the day of @month for which details are needed.
 * @user_data: the data passed with gtk_calendar_set_detail_func().
 *
 * This kind of functions provide Pango markup with detail information for the
 * specified day. Examples for such details are holidays or appointments. The
 * function returns %NULL when no information is available.
 *
 * Since: 2.14
 *
 * Return value: Newly allocated string with Pango markup with details
 * for the specified day, or %NULL.
 */
typedef gchar* (*GtkCalendarDetailFunc) (GtkCalendar *calendar,
                                         guint        year,
                                         guint        month,
                                         guint        day,
                                         gpointer     user_data);

struct _GtkCalendar
{
  GtkWidget widget;
  
  GtkStyle  *GSEAL (header_style);
  GtkStyle  *GSEAL (label_style);
  
  gint GSEAL (month);
  gint GSEAL (year);
  gint GSEAL (selected_day);
  
  gint GSEAL (day_month[6][7]);
  gint GSEAL (day[6][7]);
  
  gint GSEAL (num_marked_dates);
  gint GSEAL (marked_date[31]);
  GtkCalendarDisplayOptions  GSEAL (display_flags);
  GdkColor GSEAL (marked_date_color[31]);
  
  GdkGC *GSEAL (gc);			/* unused */
  GdkGC *GSEAL (xor_gc);		/* unused */

  gint GSEAL (focus_row);
  gint GSEAL (focus_col);

  gint GSEAL (highlight_row);
  gint GSEAL (highlight_col);
  
  GtkCalendarPrivate *GSEAL (priv);
  gchar GSEAL (grow_space [32]);

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};

struct _GtkCalendarClass
{
  GtkWidgetClass parent_class;
  
  /* Signal handlers */
  void (* month_changed)		(GtkCalendar *calendar);
  void (* day_selected)			(GtkCalendar *calendar);
  void (* day_selected_double_click)	(GtkCalendar *calendar);
  void (* prev_month)			(GtkCalendar *calendar);
  void (* next_month)			(GtkCalendar *calendar);
  void (* prev_year)			(GtkCalendar *calendar);
  void (* next_year)			(GtkCalendar *calendar);
  
};


GType	   gtk_calendar_get_type	(void) G_GNUC_CONST;
GtkWidget* gtk_calendar_new		(void);

gboolean   gtk_calendar_select_month	(GtkCalendar *calendar,
					 guint	      month,
					 guint	      year);
void	   gtk_calendar_select_day	(GtkCalendar *calendar,
					 guint	      day);

gboolean   gtk_calendar_mark_day	(GtkCalendar *calendar,
					 guint	      day);
gboolean   gtk_calendar_unmark_day	(GtkCalendar *calendar,
					 guint	      day);
void	   gtk_calendar_clear_marks	(GtkCalendar *calendar);


void	   gtk_calendar_set_display_options (GtkCalendar    	      *calendar,
					     GtkCalendarDisplayOptions flags);
GtkCalendarDisplayOptions
           gtk_calendar_get_display_options (GtkCalendar   	      *calendar);
#ifndef GTK_DISABLE_DEPRECATED
void	   gtk_calendar_display_options (GtkCalendar		  *calendar,
					 GtkCalendarDisplayOptions flags);
#endif

void	   gtk_calendar_get_date	(GtkCalendar *calendar, 
					 guint	     *year,
					 guint	     *month,
					 guint	     *day);

void       gtk_calendar_set_detail_func (GtkCalendar           *calendar,
                                         GtkCalendarDetailFunc  func,
                                         gpointer               data,
                                         GDestroyNotify         destroy);

void       gtk_calendar_set_detail_width_chars (GtkCalendar    *calendar,
                                                gint            chars);
void       gtk_calendar_set_detail_height_rows (GtkCalendar    *calendar,
                                                gint            rows);

gint       gtk_calendar_get_detail_width_chars (GtkCalendar    *calendar);
gint       gtk_calendar_get_detail_height_rows (GtkCalendar    *calendar);

#ifndef GTK_DISABLE_DEPRECATED
void	   gtk_calendar_freeze		(GtkCalendar *calendar);
void	   gtk_calendar_thaw		(GtkCalendar *calendar);
#endif

G_END_DECLS

#endif /* __GTK_CALENDAR_H__ */
