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

#ifndef __GTK_H__
#define __GTK_H__

#define __GTK_H_INSIDE__

#include <gdk/gdk.h>
#include <gtk/gtkaboutdialog.h>
#include <gtk/gtkaccelgroup.h>
#include <gtk/gtkaccellabel.h>
#include <gtk/gtkaccelmap.h>
#include <gtk/gtkaccessible.h>
#include <gtk/gtkactionable.h>
#include <gtk/gtkactionbar.h>
#include <gtk/gtkadjustment.h>
#include <gtk/gtkappchooser.h>
#include <gtk/gtkappchooserdialog.h>
#include <gtk/gtkappchooserwidget.h>
#include <gtk/gtkappchooserbutton.h>
#include <gtk/gtkapplication.h>
#include <gtk/gtkapplicationwindow.h>
#include <gtk/gtkaspectframe.h>
#include <gtk/gtkassistant.h>
#include <gtk/gtkbbox.h>
#include <gtk/gtkbin.h>
#include <gtk/gtkbindings.h>
#include <gtk/gtkborder.h>
#include <gtk/gtkbox.h>
#include <gtk/gtkbuildable.h>
#include <gtk/gtkbuilder.h>
#include <gtk/gtkbutton.h>
#include <gtk/gtkcalendar.h>
#include <gtk/gtkcellarea.h>
#include <gtk/gtkcellareabox.h>
#include <gtk/gtkcellareacontext.h>
#include <gtk/gtkcelleditable.h>
#include <gtk/gtkcelllayout.h>
#include <gtk/gtkcellrenderer.h>
#include <gtk/gtkcellrendereraccel.h>
#include <gtk/gtkcellrenderercombo.h>
#include <gtk/gtkcellrendererpixbuf.h>
#include <gtk/gtkcellrendererprogress.h>
#include <gtk/gtkcellrendererspin.h>
#include <gtk/gtkcellrendererspinner.h>
#include <gtk/gtkcellrenderertext.h>
#include <gtk/gtkcellrenderertoggle.h>
#include <gtk/gtkcellview.h>
#include <gtk/gtkcheckbutton.h>
#include <gtk/gtkcheckmenuitem.h>
#include <gtk/gtkclipboard.h>
#include <gtk/gtkcolorbutton.h>
#include <gtk/gtkcolorchooser.h>
#include <gtk/gtkcolorchooserdialog.h>
#include <gtk/gtkcolorchooserwidget.h>
#include <gtk/gtkcolorutils.h>
#include <gtk/gtkcombobox.h>
#include <gtk/gtkcomboboxtext.h>
#include <gtk/gtkcontainer.h>
#include <gtk/gtkcssprovider.h>
#include <gtk/gtkcsssection.h>
#include <gtk/gtkdebug.h>
#include <gtk/gtkdialog.h>
#include <gtk/gtkdnd.h>
#include <gtk/gtkdragdest.h>
#include <gtk/gtkdragsource.h>
#include <gtk/gtkdrawingarea.h>
#include <gtk/gtkeditable.h>
#include <gtk/gtkentry.h>
#include <gtk/gtkentrybuffer.h>
#include <gtk/gtkentrycompletion.h>
#include <gtk/gtkenums.h>
#include <gtk/gtkeventbox.h>
#include <gtk/gtkeventcontroller.h>
#include <gtk/gtkeventcontrollerkey.h>
#include <gtk/gtkeventcontrollermotion.h>
#include <gtk/gtkeventcontrollerscroll.h>
#include <gtk/gtkexpander.h>
#include <gtk/gtkfixed.h>
#include <gtk/gtkfilechooser.h>
#include <gtk/gtkfilechooserbutton.h>
#include <gtk/gtkfilechooserdialog.h>
#include <gtk/gtkfilechoosernative.h>
#include <gtk/gtkfilechooserwidget.h>
#include <gtk/gtkfilefilter.h>
#include <gtk/gtkflowbox.h>
#include <gtk/gtkfontbutton.h>
#include <gtk/gtkfontchooser.h>
#include <gtk/gtkfontchooserdialog.h>
#include <gtk/gtkfontchooserwidget.h>
#include <gtk/gtkframe.h>
#include <gtk/gtkgesture.h>
#include <gtk/gtkgesturedrag.h>
#include <gtk/gtkgesturelongpress.h>
#include <gtk/gtkgesturemultipress.h>
#include <gtk/gtkgesturepan.h>
#include <gtk/gtkgesturerotate.h>
#include <gtk/gtkgesturesingle.h>
#include <gtk/gtkgesturestylus.h>
#include <gtk/gtkgestureswipe.h>
#include <gtk/gtkgesturezoom.h>
#include <gtk/gtkglarea.h>
#include <gtk/gtkgrid.h>
#include <gtk/gtkheaderbar.h>
#include <gtk/gtkicontheme.h>
#include <gtk/gtkiconview.h>
#include <gtk/gtkimage.h>
#include <gtk/gtkimcontext.h>
#include <gtk/gtkimcontextinfo.h>
#include <gtk/gtkimcontextsimple.h>
#include <gtk/gtkimmulticontext.h>
#include <gtk/gtkinfobar.h>
#include <gtk/gtkinvisible.h>
#include <gtk/gtklabel.h>
#include <gtk/gtklayout.h>
#include <gtk/gtklevelbar.h>
#include <gtk/gtklinkbutton.h>
#include <gtk/gtklistbox.h>
#include <gtk/gtkliststore.h>
#include <gtk/gtklockbutton.h>
#include <gtk/gtkmain.h>
#include <gtk/gtkmenu.h>
#include <gtk/gtkmenubar.h>
#include <gtk/gtkmenubutton.h>
#include <gtk/gtkmenuitem.h>
#include <gtk/gtkmenushell.h>
#include <gtk/gtkmenutoolbutton.h>
#include <gtk/gtkmessagedialog.h>
#include <gtk/gtkmodelbutton.h>
#include <gtk/gtkmodules.h>
#include <gtk/gtkmountoperation.h>
#include <gtk/gtknativedialog.h>
#include <gtk/gtknotebook.h>
#include <gtk/gtkoffscreenwindow.h>
#include <gtk/gtkorientable.h>
#include <gtk/gtkoverlay.h>
#include <gtk/gtkpadcontroller.h>
#include <gtk/gtkpagesetup.h>
#include <gtk/gtkpapersize.h>
#include <gtk/gtkpaned.h>
#include <gtk/gtkplacessidebar.h>
#include <gtk/gtkpopover.h>
#include <gtk/gtkpopovermenu.h>
#include <gtk/gtkprintcontext.h>
#include <gtk/gtkprintoperation.h>
#include <gtk/gtkprintoperationpreview.h>
#include <gtk/gtkprintsettings.h>
#include <gtk/gtkprogressbar.h>
#include <gtk/gtkradiobutton.h>
#include <gtk/gtkradiomenuitem.h>
#include <gtk/gtkradiotoolbutton.h>
#include <gtk/gtkrange.h>
#include <gtk/gtkrecentchooser.h>
#include <gtk/gtkrecentchooserdialog.h>
#include <gtk/gtkrecentchoosermenu.h>
#include <gtk/gtkrecentchooserwidget.h>
#include <gtk/gtkrecentfilter.h>
#include <gtk/gtkrecentmanager.h>
#include <gtk/gtkrender.h>
#include <gtk/gtkrevealer.h>
#include <gtk/gtkscale.h>
#include <gtk/gtkscalebutton.h>
#include <gtk/gtkscrollable.h>
#include <gtk/gtkscrollbar.h>
#include <gtk/gtkscrolledwindow.h>
#include <gtk/gtksearchbar.h>
#include <gtk/gtksearchentry.h>
#include <gtk/gtkselection.h>
#include <gtk/gtkseparator.h>
#include <gtk/gtkseparatormenuitem.h>
#include <gtk/gtkseparatortoolitem.h>
#include <gtk/gtksettings.h>
#include <gtk/gtkshortcutlabel.h>
#include <gtk/gtkshortcutsgroup.h>
#include <gtk/gtkshortcutssection.h>
#include <gtk/gtkshortcutsshortcut.h>
#include <gtk/gtkshortcutswindow.h>
#include <gtk/gtkshow.h>
#include <gtk/gtkstacksidebar.h>
#include <gtk/gtksizegroup.h>
#include <gtk/gtksizerequest.h>
#include <gtk/gtkspinbutton.h>
#include <gtk/gtkspinner.h>
#include <gtk/gtkstack.h>
#include <gtk/gtkstackswitcher.h>
#include <gtk/gtkstatusbar.h>
#include <gtk/gtkstylecontext.h>
#include <gtk/gtkstyleprovider.h>
#include <gtk/gtkswitch.h>
#include <gtk/gtktextattributes.h>
#include <gtk/gtktextbuffer.h>
#include <gtk/gtktextbufferrichtext.h>
#include <gtk/gtktextchild.h>
#include <gtk/gtktextiter.h>
#include <gtk/gtktextmark.h>
#include <gtk/gtktexttag.h>
#include <gtk/gtktexttagtable.h>
#include <gtk/gtktextview.h>
#include <gtk/gtktogglebutton.h>
#include <gtk/gtktoggletoolbutton.h>
#include <gtk/gtktoolbar.h>
#include <gtk/gtktoolbutton.h>
#include <gtk/gtktoolitem.h>
#include <gtk/gtktoolitemgroup.h>
#include <gtk/gtktoolpalette.h>
#include <gtk/gtktoolshell.h>
#include <gtk/gtktooltip.h>
#include <gtk/gtktestutils.h>
#include <gtk/gtktreednd.h>
#include <gtk/gtktreemodel.h>
#include <gtk/gtktreemodelfilter.h>
#include <gtk/gtktreemodelsort.h>
#include <gtk/gtktreeselection.h>
#include <gtk/gtktreesortable.h>
#include <gtk/gtktreestore.h>
#include <gtk/gtktreeview.h>
#include <gtk/gtktreeviewcolumn.h>
#include <gtk/gtktypebuiltins.h>
#include <gtk/gtktypes.h>
#include <gtk/gtkversion.h>
#include <gtk/gtkviewport.h>
#include <gtk/gtkvolumebutton.h>
#include <gtk/gtkwidget.h>
#include <gtk/gtkwidgetpath.h>
#include <gtk/gtkwindow.h>
#include <gtk/gtkwindowgroup.h>

#ifndef GTK_DISABLE_DEPRECATED
#include <gtk/deprecated/gtkarrow.h>
#include <gtk/deprecated/gtkactivatable.h>
#include <gtk/deprecated/gtkaction.h>
#include <gtk/deprecated/gtkactiongroup.h>
#include <gtk/deprecated/gtkalignment.h>
#include <gtk/deprecated/gtkcolorsel.h>
#include <gtk/deprecated/gtkcolorseldialog.h>
#include <gtk/deprecated/gtkfontsel.h>
#include <gtk/deprecated/gtkgradient.h>
#include <gtk/deprecated/gtkhandlebox.h>
#include <gtk/deprecated/gtkhbbox.h>
#include <gtk/deprecated/gtkhbox.h>
#include <gtk/deprecated/gtkhpaned.h>
#include <gtk/deprecated/gtkhsv.h>
#include <gtk/deprecated/gtkhscale.h>
#include <gtk/deprecated/gtkhscrollbar.h>
#include <gtk/deprecated/gtkhseparator.h>
#include <gtk/deprecated/gtkiconfactory.h>
#include <gtk/deprecated/gtkimagemenuitem.h>
#include <gtk/deprecated/gtkmisc.h>
#include <gtk/deprecated/gtknumerableicon.h>
#include <gtk/deprecated/gtkradioaction.h>
#include <gtk/deprecated/gtkrc.h>
#include <gtk/deprecated/gtkrecentaction.h>
#include <gtk/deprecated/gtkstatusicon.h>
#include <gtk/deprecated/gtkstock.h>
#include <gtk/deprecated/gtkstyle.h>
#include <gtk/deprecated/gtkstyleproperties.h>
#include <gtk/deprecated/gtksymboliccolor.h>
#include <gtk/deprecated/gtktable.h>
#include <gtk/deprecated/gtktearoffmenuitem.h>
#include <gtk/deprecated/gtkthemingengine.h>
#include <gtk/deprecated/gtktoggleaction.h>
#include <gtk/deprecated/gtkuimanager.h>
#include <gtk/deprecated/gtkvbbox.h>
#include <gtk/deprecated/gtkvbox.h>
#include <gtk/deprecated/gtkvpaned.h>
#include <gtk/deprecated/gtkvscale.h>
#include <gtk/deprecated/gtkvscrollbar.h>
#include <gtk/deprecated/gtkvseparator.h>
#endif /* GTK_DISABLE_DEPRECATED */

#include <gtk/gtk-autocleanups.h>

#undef __GTK_H_INSIDE__

#endif /* __GTK_H__ */
