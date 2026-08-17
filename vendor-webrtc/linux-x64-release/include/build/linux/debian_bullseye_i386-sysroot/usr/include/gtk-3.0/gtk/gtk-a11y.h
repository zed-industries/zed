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

#ifndef __GTK_A11Y_H__
#define __GTK_A11Y_H__

#define __GTK_A11Y_H_INSIDE__

#include <gtk/gtk.h>
#include <gtk/a11y/gtkarrowaccessible.h>
#include <gtk/a11y/gtkbooleancellaccessible.h>
#include <gtk/a11y/gtkbuttonaccessible.h>
#include <gtk/a11y/gtkcellaccessible.h>
#include <gtk/a11y/gtkcellaccessibleparent.h>
#include <gtk/a11y/gtkcheckmenuitemaccessible.h>
#include <gtk/a11y/gtkcomboboxaccessible.h>
#include <gtk/a11y/gtkcontaineraccessible.h>
#include <gtk/a11y/gtkcontainercellaccessible.h>
#include <gtk/a11y/gtkentryaccessible.h>
#include <gtk/a11y/gtkexpanderaccessible.h>
#include <gtk/a11y/gtkflowboxaccessible.h>
#include <gtk/a11y/gtkflowboxchildaccessible.h>
#include <gtk/a11y/gtkframeaccessible.h>
#include <gtk/a11y/gtkiconviewaccessible.h>
#include <gtk/a11y/gtkimageaccessible.h>
#include <gtk/a11y/gtkimagecellaccessible.h>
#include <gtk/a11y/gtklabelaccessible.h>
#include <gtk/a11y/gtklevelbaraccessible.h>
#include <gtk/a11y/gtklinkbuttonaccessible.h>
#include <gtk/a11y/gtklistboxaccessible.h>
#include <gtk/a11y/gtklistboxrowaccessible.h>
#include <gtk/a11y/gtklockbuttonaccessible.h>
#include <gtk/a11y/gtkmenuaccessible.h>
#include <gtk/a11y/gtkmenubuttonaccessible.h>
#include <gtk/a11y/gtkmenuitemaccessible.h>
#include <gtk/a11y/gtkmenushellaccessible.h>
#include <gtk/a11y/gtknotebookaccessible.h>
#include <gtk/a11y/gtknotebookpageaccessible.h>
#include <gtk/a11y/gtkplugaccessible.h>
#include <gtk/a11y/gtkpopoveraccessible.h>
#include <gtk/a11y/gtkpanedaccessible.h>
#include <gtk/a11y/gtkprogressbaraccessible.h>
#include <gtk/a11y/gtkradiobuttonaccessible.h>
#include <gtk/a11y/gtkradiomenuitemaccessible.h>
#include <gtk/a11y/gtkrangeaccessible.h>
#include <gtk/a11y/gtkrenderercellaccessible.h>
#include <gtk/a11y/gtkscaleaccessible.h>
#include <gtk/a11y/gtkscalebuttonaccessible.h>
#include <gtk/a11y/gtkscrolledwindowaccessible.h>
#include <gtk/a11y/gtksocketaccessible.h>
#include <gtk/a11y/gtkspinbuttonaccessible.h>
#include <gtk/a11y/gtkspinneraccessible.h>
#include <gtk/a11y/gtkstackaccessible.h>
#include <gtk/a11y/gtkstatusbaraccessible.h>
#include <gtk/a11y/gtkswitchaccessible.h>
#include <gtk/a11y/gtktextcellaccessible.h>
#include <gtk/a11y/gtktextviewaccessible.h>
#include <gtk/a11y/gtktogglebuttonaccessible.h>
#include <gtk/a11y/gtktoplevelaccessible.h>
#include <gtk/a11y/gtktreeviewaccessible.h>
#include <gtk/a11y/gtkwidgetaccessible.h>
#include <gtk/a11y/gtkwindowaccessible.h>

#include <gtk/a11y/gtk-a11y-autocleanups.h>

#undef __GTK_A11Y_H_INSIDE__

#endif /* __GTK_A11Y_H__ */
