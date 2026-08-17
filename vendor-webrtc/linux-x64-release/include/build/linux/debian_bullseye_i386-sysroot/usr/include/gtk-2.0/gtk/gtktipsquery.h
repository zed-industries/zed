/* GTK - The GIMP Toolkit
 * Copyright (C) 1995-1997 Peter Mattis, Spencer Kimball and Josh MacDonald
 *
 * GtkQueryTips: Query onscreen widgets for their tooltips
 * Copyright (C) 1998 Tim Janik
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

#ifndef __GTK_TIPS_QUERY_H__
#define __GTK_TIPS_QUERY_H__


#include <gtk/gtk.h>


G_BEGIN_DECLS

/* --- type macros --- */
#define	GTK_TYPE_TIPS_QUERY		(gtk_tips_query_get_type ())
#define GTK_TIPS_QUERY(obj)		(G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_TIPS_QUERY, GtkTipsQuery))
#define GTK_TIPS_QUERY_CLASS(klass)	(G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_TIPS_QUERY, GtkTipsQueryClass))
#define GTK_IS_TIPS_QUERY(obj)		(G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_TIPS_QUERY))
#define GTK_IS_TIPS_QUERY_CLASS(klass)	(G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_TIPS_QUERY))
#define GTK_TIPS_QUERY_GET_CLASS(obj)   (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_TIPS_QUERY, GtkTipsQueryClass))


/* --- typedefs --- */
typedef	struct	_GtkTipsQuery		GtkTipsQuery;
typedef	struct	_GtkTipsQueryClass	GtkTipsQueryClass;


/* --- structures --- */
struct	_GtkTipsQuery
{
  GtkLabel	label;

  guint		emit_always : 1;
  guint		in_query : 1;
  gchar		*label_inactive;
  gchar		*label_no_tip;

  GtkWidget	*caller;
  GtkWidget	*last_crossed;

  GdkCursor	*query_cursor;
};

struct	_GtkTipsQueryClass
{
  GtkLabelClass			parent_class;

  void	(*start_query)		(GtkTipsQuery	*tips_query);
  void	(*stop_query)		(GtkTipsQuery	*tips_query);
  void	(*widget_entered)	(GtkTipsQuery	*tips_query,
				 GtkWidget	*widget,
				 const gchar	*tip_text,
				 const gchar	*tip_private);
  gint	(*widget_selected)	(GtkTipsQuery	*tips_query,
				 GtkWidget	*widget,
				 const gchar	*tip_text,
				 const gchar	*tip_private,
				 GdkEventButton	*event);

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};


/* --- prototypes --- */
GType		gtk_tips_query_get_type		(void) G_GNUC_CONST;
GtkWidget*	gtk_tips_query_new		(void);
void		gtk_tips_query_start_query	(GtkTipsQuery	*tips_query);
void		gtk_tips_query_stop_query	(GtkTipsQuery	*tips_query);
void		gtk_tips_query_set_caller	(GtkTipsQuery	*tips_query,
						 GtkWidget	*caller);
void		gtk_tips_query_set_labels 	(GtkTipsQuery   *tips_query,
						 const gchar    *label_inactive,
						 const gchar    *label_no_tip);

G_END_DECLS

#endif	/* __GTK_TIPS_QUERY_H__ */

#endif /* GTK_DISABLE_DEPRECATED */
