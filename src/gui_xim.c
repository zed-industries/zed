/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

/*
 * gui_xim.c: functions for the X Input Method
 */

#include "vim.h"

#if !defined(GTK_CHECK_VERSION)
# define GTK_CHECK_VERSION(a, b, c) 0
#endif
#if !defined(FEAT_GUI_GTK) && defined(PROTO)
typedef int GtkWidget;
typedef int GtkIMContext;
typedef int gchar;
typedef int gpointer;
typedef int PangoAttrIterator;
typedef int GdkEventKey;
#endif

#if defined(FEAT_GUI_GTK) && defined(FEAT_XIM)
# if GTK_CHECK_VERSION(3,0,0)
#  include <gdk/gdkkeysyms-compat.h>
# else
#  include <gdk/gdkkeysyms.h>
# endif
# ifdef MSWIN
#  include <gdk/gdkwin32.h>
# else
#  include <gdk/gdkx.h>
# endif
#endif

/*
 * XIM often causes trouble.  Define XIM_DEBUG to get a log of XIM callbacks
 * in the "xim.log" file.
 */
// #define XIM_DEBUG
#if defined(XIM_DEBUG) && defined(FEAT_GUI_GTK)
static void xim_log(char *s, ...) ATTRIBUTE_FORMAT_PRINTF(1, 2);

    static void
xim_log(char *s, ...)
{
    va_list arglist;
    static FILE *fd = NULL;

    if (fd == (FILE *)-1)
	return;
    if (fd == NULL)
    {
	fd = mch_fopen("xim.log", "w");
	if (fd == NULL)
	{
	    emsg("Cannot open xim.log");
	    fd = (FILE *)-1;
	    return;
	}
    }

    va_start(arglist, s);
    vfprintf(fd, s, arglist);
    va_end(arglist);
}
#endif

#if defined(FEAT_GUI_MSWIN)
# define USE_IMACTIVATEFUNC (!gui.in_use && *p_imaf != NUL)
# define USE_IMSTATUSFUNC (!gui.in_use && *p_imsf != NUL)
#else
# define USE_IMACTIVATEFUNC (*p_imaf != NUL)
# define USE_IMSTATUSFUNC (*p_imsf != NUL)
#endif

#if (defined(FEAT_EVAL) && \
     (defined(FEAT_XIM) || defined(IME_WITHOUT_XIM) || defined(VIMDLL))) || \
    defined(PROTO)
static callback_T imaf_cb;	    // 'imactivatefunc' callback function
static callback_T imsf_cb;	    // 'imstatusfunc' callback function

    char *
did_set_imactivatefunc(optset_T *args UNUSED)
{
    if (option_set_callback_func(p_imaf, &imaf_cb) == FAIL)
	return e_invalid_argument;

    return NULL;
}

    char *
did_set_imstatusfunc(optset_T *args UNUSED)
{
    if (option_set_callback_func(p_imsf, &imsf_cb) == FAIL)
	return e_invalid_argument;

    return NULL;
}

    static void
call_imactivatefunc(int active)
{
    typval_T argv[2];
    int save_KeyTyped = KeyTyped;

    argv[0].v_type = VAR_NUMBER;
    argv[0].vval.v_number = active ? 1 : 0;
    argv[1].v_type = VAR_UNKNOWN;
    (void)call_callback_retnr(&imaf_cb, 1, argv);

    KeyTyped = save_KeyTyped;
}

    static int
call_imstatusfunc(void)
{
    int is_active;
    int save_KeyTyped = KeyTyped;

    // FIXME: Don't execute user function in unsafe situation.
    if (exiting || is_autocmd_blocked())
	return FALSE;
    // FIXME: :py print 'xxx' is shown duplicate result.
    // Use silent to avoid it.
    ++msg_silent;
    is_active = call_callback_retnr(&imsf_cb, 0, NULL);
    --msg_silent;

    KeyTyped = save_KeyTyped;
    return (is_active > 0);
}
#endif

#if defined(EXITFREE) || defined(PROTO)
    void
free_xim_stuff(void)
{
# if defined(FEAT_EVAL) && \
    (defined(FEAT_XIM) || defined(IME_WITHOUT_XIM) || defined(VIMDLL))
    free_callback(&imaf_cb);
    free_callback(&imsf_cb);
# endif
}
#endif

#if defined(FEAT_EVAL) || defined(PROTO)
/*
 * Mark the global 'imactivatefunc' and 'imstatusfunc' callbacks with "copyID"
 * so that they are not garbage collected.
 */
    int
set_ref_in_im_funcs(int copyID UNUSED)
{
    int abort = FALSE;

# if defined(FEAT_XIM) || defined(IME_WITHOUT_XIM) || defined(VIMDLL)
    abort = set_ref_in_callback(&imaf_cb, copyID);
    abort = abort || set_ref_in_callback(&imsf_cb, copyID);
# endif

    return abort;
}
#endif


#if defined(FEAT_XIM) || defined(PROTO)

# if defined(FEAT_GUI_GTK) || defined(PROTO)
static int xim_has_preediting = FALSE;  // IM current status

/*
 * Set preedit_start_col to the current cursor position.
 */
    static void
init_preedit_start_col(void)
{
    if (State & MODE_CMDLINE)
	preedit_start_col = cmdline_getvcol_cursor();
    else if (curwin != NULL && curwin->w_buffer != NULL)
	getvcol(curwin, &curwin->w_cursor, &preedit_start_col, NULL, NULL);
    // Prevent that preediting marks the buffer as changed.
    xim_changed_while_preediting = curbuf->b_changed;
}

static int im_is_active	       = FALSE;	// IM is enabled for current mode
static int preedit_is_active   = FALSE;
static int im_preedit_cursor   = 0;	// cursor offset in characters
static int im_preedit_trailing = 0;	// number of characters after cursor

static unsigned long im_commit_handler_id  = 0;
static unsigned int  im_activatekey_keyval = GDK_VoidSymbol;
static unsigned int  im_activatekey_state  = 0;

static GtkWidget *preedit_window = NULL;
static GtkWidget *preedit_label = NULL;

static void im_preedit_window_set_position(void);

    void
im_set_active(int active)
{
    int was_active;

    was_active = !!im_get_status();
    im_is_active = (active && !p_imdisable);

    if (im_is_active != was_active)
	xim_reset();
}

    void
xim_set_focus(int focus)
{
    if (xic == NULL)
	return;

    if (focus)
	gtk_im_context_focus_in(xic);
    else
	gtk_im_context_focus_out(xic);
}

    void
im_set_position(int row, int col)
{
    if (xic == NULL)
	return;

    GdkRectangle area;

    area.x = FILL_X(col);
    area.y = FILL_Y(row);
    area.width  = gui.char_width * (mb_lefthalve(row, col) ? 2 : 1);
    area.height = gui.char_height;

    gtk_im_context_set_cursor_location(xic, &area);

    if (p_imst == IM_OVER_THE_SPOT)
	im_preedit_window_set_position();
}

#  if 0 || defined(PROTO) // apparently only used in gui_x11.c
    void
xim_set_preedit(void)
{
    im_set_position(gui.row, gui.col);
}
#  endif

    static void
im_add_to_input(char_u *str, int len)
{
    // Convert from 'termencoding' (always "utf-8") to 'encoding'
    if (input_conv.vc_type != CONV_NONE)
    {
	str = string_convert(&input_conv, str, &len);
	g_return_if_fail(str != NULL);
    }

    add_to_input_buf_csi(str, len);

    if (input_conv.vc_type != CONV_NONE)
	vim_free(str);

    if (p_mh) // blank out the pointer if necessary
	gui_mch_mousehide(TRUE);
}

     static void
im_preedit_window_set_position(void)
{
    int x, y, width, height;
    int screen_x, screen_y, screen_width, screen_height;

    if (preedit_window == NULL)
	return;

    gui_gtk_get_screen_geom_of_win(gui.drawarea, 0, 0,
			  &screen_x, &screen_y, &screen_width, &screen_height);
    gdk_window_get_origin(gtk_widget_get_window(gui.drawarea), &x, &y);
    gtk_window_get_size(GTK_WINDOW(preedit_window), &width, &height);
    x = x + FILL_X(gui.col);
    y = y + FILL_Y(gui.row);
    if (x + width > screen_x + screen_width)
	x = screen_x + screen_width - width;
    if (y + height > screen_y + screen_height)
	y = screen_y + screen_height - height;
    gtk_window_move(GTK_WINDOW(preedit_window), x, y);
}

    static void
im_preedit_window_open(void)
{
    char *preedit_string;
#if !GTK_CHECK_VERSION(3,16,0)
    char buf[8];
#endif
    PangoAttrList *attr_list;
    PangoLayout *layout;
#if GTK_CHECK_VERSION(3,0,0)
# if !GTK_CHECK_VERSION(3,16,0)
    GdkRGBA color;
# endif
#else
    GdkColor color;
#endif
    gint w, h;

    if (preedit_window == NULL)
    {
	preedit_window = gtk_window_new(GTK_WINDOW_POPUP);
	gtk_window_set_transient_for(GTK_WINDOW(preedit_window),
						     GTK_WINDOW(gui.mainwin));
	preedit_label = gtk_label_new("");
	gtk_widget_set_name(preedit_label, "vim-gui-preedit-area");
	gtk_container_add(GTK_CONTAINER(preedit_window), preedit_label);
    }

#if GTK_CHECK_VERSION(3,16,0)
    {
	GtkStyleContext * const context
				  = gtk_widget_get_style_context(gui.drawarea);
	GtkCssProvider * const provider = gtk_css_provider_new();
	gchar		   *css = NULL;
	const char * const fontname
			   = pango_font_description_get_family(gui.norm_font);
	gint	fontsize
		= pango_font_description_get_size(gui.norm_font) / PANGO_SCALE;
	gchar	*fontsize_propval = NULL;

	if (!pango_font_description_get_size_is_absolute(gui.norm_font))
	{
	    // fontsize was given in points.  Convert it into that in pixels
	    // to use with CSS.
	    GdkScreen * const screen
		  = gdk_window_get_screen(gtk_widget_get_window(gui.mainwin));
	    const gdouble dpi = gdk_screen_get_resolution(screen);
	    fontsize = dpi * fontsize / 72;
	}
	if (fontsize > 0)
	    fontsize_propval = g_strdup_printf("%dpx", fontsize);
	else
	    fontsize_propval = g_strdup_printf("inherit");

	css = g_strdup_printf(
		"widget#vim-gui-preedit-area {\n"
		"  font-family: %s,monospace;\n"
		"  font-size: %s;\n"
		"  color: #%.2lx%.2lx%.2lx;\n"
		"  background-color: #%.2lx%.2lx%.2lx;\n"
		"}\n",
		fontname != NULL ? fontname : "inherit",
		fontsize_propval,
		(gui.norm_pixel >> 16) & 0xff,
		(gui.norm_pixel >> 8) & 0xff,
		gui.norm_pixel & 0xff,
		(gui.back_pixel >> 16) & 0xff,
		(gui.back_pixel >> 8) & 0xff,
		gui.back_pixel & 0xff);

	gtk_css_provider_load_from_data(provider, css, -1, NULL);
	gtk_style_context_add_provider(context,
				     GTK_STYLE_PROVIDER(provider), G_MAXUINT);

	g_free(css);
	g_free(fontsize_propval);
	g_object_unref(provider);
    }
#elif GTK_CHECK_VERSION(3,0,0)
    gtk_widget_override_font(preedit_label, gui.norm_font);

    vim_snprintf(buf, sizeof(buf), "#%06X", gui.norm_pixel);
    gdk_rgba_parse(&color, buf);
    gtk_widget_override_color(preedit_label, GTK_STATE_FLAG_NORMAL, &color);

    vim_snprintf(buf, sizeof(buf), "#%06X", gui.back_pixel);
    gdk_rgba_parse(&color, buf);
    gtk_widget_override_background_color(preedit_label, GTK_STATE_FLAG_NORMAL,
								      &color);
#else
    gtk_widget_modify_font(preedit_label, gui.norm_font);

    vim_snprintf(buf, sizeof(buf), "#%06X", (unsigned)gui.norm_pixel);
    gdk_color_parse(buf, &color);
    gtk_widget_modify_fg(preedit_label, GTK_STATE_NORMAL, &color);

    vim_snprintf(buf, sizeof(buf), "#%06X", (unsigned)gui.back_pixel);
    gdk_color_parse(buf, &color);
    gtk_widget_modify_bg(preedit_window, GTK_STATE_NORMAL, &color);
#endif

    gtk_im_context_get_preedit_string(xic, &preedit_string, &attr_list, NULL);

    if (preedit_string[0] != NUL)
    {
	gtk_label_set_text(GTK_LABEL(preedit_label), preedit_string);
	gtk_label_set_attributes(GTK_LABEL(preedit_label), attr_list);

	layout = gtk_label_get_layout(GTK_LABEL(preedit_label));
	pango_layout_get_pixel_size(layout, &w, &h);
	h = MAX(h, gui.char_height);
	gtk_window_resize(GTK_WINDOW(preedit_window), w, h);

	gtk_widget_show_all(preedit_window);

	im_preedit_window_set_position();
    }

    g_free(preedit_string);
    pango_attr_list_unref(attr_list);
}

    static void
im_preedit_window_close(void)
{
    if (preedit_window != NULL)
	gtk_widget_hide(preedit_window);
}

    static void
im_show_preedit(void)
{
    im_preedit_window_open();

    if (p_mh) // blank out the pointer if necessary
	gui_mch_mousehide(TRUE);
}

    static void
im_delete_preedit(void)
{
    char_u bskey[]  = {CSI, 'k', 'b'};
    char_u delkey[] = {CSI, 'k', 'D'};

    if (p_imst == IM_OVER_THE_SPOT)
    {
	im_preedit_window_close();
	return;
    }

    if (State & MODE_NORMAL
#ifdef FEAT_TERMINAL
	    && !term_use_loop()
#endif
       )
    {
	im_preedit_cursor = 0;
	return;
    }
    for (; im_preedit_cursor > 0; --im_preedit_cursor)
	add_to_input_buf(bskey, (int)sizeof(bskey));

    for (; im_preedit_trailing > 0; --im_preedit_trailing)
	add_to_input_buf(delkey, (int)sizeof(delkey));
}

/*
 * Move the cursor left by "num_move_back" characters.
 * Note that ins_left() checks im_is_preediting() to avoid breaking undo for
 * these K_LEFT keys.
 */
    static void
im_correct_cursor(int num_move_back)
{
    char_u backkey[] = {CSI, 'k', 'l'};

    if (State & MODE_NORMAL)
	return;
#  ifdef FEAT_RIGHTLEFT
    if ((State & MODE_CMDLINE) == 0 && curwin != NULL && curwin->w_p_rl)
	backkey[2] = 'r';
#  endif
    for (; num_move_back > 0; --num_move_back)
	add_to_input_buf(backkey, (int)sizeof(backkey));
}

static int xim_expected_char = NUL;
static int xim_ignored_char = FALSE;

/*
 * Update the mode and cursor while in an IM callback.
 */
    static void
im_show_info(void)
{
    int	    old_vgetc_busy;

    old_vgetc_busy = vgetc_busy;
    vgetc_busy = TRUE;
    showmode();
    vgetc_busy = old_vgetc_busy;
    if ((State & MODE_NORMAL) || (State & MODE_INSERT))
	setcursor();
    out_flush();
}

/*
 * Callback invoked when the user finished preediting.
 * Put the final string into the input buffer.
 */
    static void
im_commit_cb(GtkIMContext *context UNUSED,
	     const gchar *str,
	     gpointer data UNUSED)
{
    int		slen = (int)STRLEN(str);
    int		add_to_input = TRUE;
    int		clen;
    int		len = slen;
    int		commit_with_preedit = TRUE;
    char_u	*im_str;

#ifdef XIM_DEBUG
    xim_log("im_commit_cb(): %s\n", str);
#endif

    if (p_imst == IM_ON_THE_SPOT)
    {
	// The imhangul module doesn't reset the preedit string before
	// committing.  Call im_delete_preedit() to work around that.
	im_delete_preedit();

	// Indicate that preediting has finished.
	if (preedit_start_col == MAXCOL)
	{
	    init_preedit_start_col();
	    commit_with_preedit = FALSE;
	}

	// The thing which setting "preedit_start_col" to MAXCOL means that
	// "preedit_start_col" will be set forcedly when calling
	// preedit_changed_cb() next time.
	// "preedit_start_col" should not reset with MAXCOL on this part. Vim
	// is simulating the preediting by using add_to_input_str(). when
	// preedit begin immediately before committed, the typebuf is not
	// flushed to screen, then it can't get correct "preedit_start_col".
	// Thus, it should calculate the cells by adding cells of the committed
	// string.
	if (input_conv.vc_type != CONV_NONE)
	{
	    im_str = string_convert(&input_conv, (char_u *)str, &len);
	    g_return_if_fail(im_str != NULL);
	}
	else
	    im_str = (char_u *)str;

	clen = mb_string2cells(im_str, len);

	if (input_conv.vc_type != CONV_NONE)
	    vim_free(im_str);
	preedit_start_col += clen;
    }

    // Is this a single character that matches a keypad key that's just
    // been pressed?  If so, we don't want it to be entered as such - let
    // us carry on processing the raw keycode so that it may be used in
    // mappings as <kSomething>.
    if (xim_expected_char != NUL)
    {
	// We're currently processing a keypad or other special key
	if (slen == 1 && str[0] == xim_expected_char)
	{
	    // It's a match - don't do it here
	    xim_ignored_char = TRUE;
	    add_to_input = FALSE;
	}
	else
	{
	    // Not a match
	    xim_ignored_char = FALSE;
	}
    }

    if (add_to_input)
	im_add_to_input((char_u *)str, slen);

    if (p_imst == IM_ON_THE_SPOT)
    {
	// Inserting chars while "im_is_active" is set does not cause a
	// change of buffer.  When the chars are committed the buffer must be
	// marked as changed.
	if (!commit_with_preedit)
	    preedit_start_col = MAXCOL;

	// This flag is used in changed() at next call.
	xim_changed_while_preediting = TRUE;
    }

    if (gtk_main_level() > 0)
	gtk_main_quit();
}

/*
 * Callback invoked after start to the preedit.
 */
    static void
im_preedit_start_cb(GtkIMContext *context UNUSED, gpointer data UNUSED)
{
#ifdef XIM_DEBUG
    xim_log("im_preedit_start_cb()\n");
#endif

    im_is_active = TRUE;
    preedit_is_active = TRUE;
    gui_update_cursor(TRUE, FALSE);
    im_show_info();
}

/*
 * Callback invoked after end to the preedit.
 */
    static void
im_preedit_end_cb(GtkIMContext *context UNUSED, gpointer data UNUSED)
{
#ifdef XIM_DEBUG
    xim_log("im_preedit_end_cb()\n");
#endif
    im_delete_preedit();

    // Indicate that preediting has finished
    if (p_imst == IM_ON_THE_SPOT)
	preedit_start_col = MAXCOL;
    xim_has_preediting = FALSE;

#if 0
    // Removal of this line suggested by Takuhiro Nishioka.  Fixes that IM was
    // switched off unintentionally.  We now use preedit_is_active (added by
    // SungHyun Nam).
    im_is_active = FALSE;
#endif
    preedit_is_active = FALSE;
    gui_update_cursor(TRUE, FALSE);
    im_show_info();
}

/*
 * Callback invoked after changes to the preedit string.  If the preedit
 * string was empty before, remember the preedit start column so we know
 * where to apply feedback attributes.  Delete the previous preedit string
 * if there was one, save the new preedit cursor offset, and put the new
 * string into the input buffer.
 *
 * TODO: The pragmatic "put into input buffer" approach used here has
 *       several fundamental problems:
 *
 * - The characters in the preedit string are subject to remapping.
 *   That's broken, only the finally committed string should be remapped.
 *
 * - There is a race condition involved:  The retrieved value for the
 *   current cursor position will be wrong if any unprocessed characters
 *   are still queued in the input buffer.
 *
 * - Due to the lack of synchronization between the file buffer in memory
 *   and any typed characters, it's practically impossible to implement the
 *   "retrieve_surrounding" and "delete_surrounding" signals reliably.  IM
 *   modules for languages such as Thai are likely to rely on this feature
 *   for proper operation.
 *
 * Conclusions:  I think support for preediting needs to be moved to the
 * core parts of Vim.  Ideally, until it has been committed, the preediting
 * string should only be displayed and not affect the buffer content at all.
 * The question how to deal with the synchronization issue still remains.
 * Circumventing the input buffer is probably not desirable.  Anyway, I think
 * implementing "retrieve_surrounding" is the only hard problem.
 *
 * One way to solve all of this in a clean manner would be to queue all key
 * press/release events "as is" in the input buffer, and apply the IM filtering
 * at the receiving end of the queue.  This, however, would have a rather large
 * impact on the code base.  If there is an easy way to force processing of all
 * remaining input from within the "retrieve_surrounding" signal handler, this
 * might not be necessary.  Gotta ask on vim-dev for opinions.
 */
    static void
im_preedit_changed_cb(GtkIMContext *context, gpointer data UNUSED)
{
    char    *preedit_string = NULL;
    int	    cursor_index    = 0;
    int	    num_move_back   = 0;
    char_u  *str;
    char_u  *p;
    int	    i;

    if (p_imst == IM_ON_THE_SPOT)
	gtk_im_context_get_preedit_string(context,
					  &preedit_string, NULL,
					  &cursor_index);
    else
	gtk_im_context_get_preedit_string(context,
					  &preedit_string, NULL,
					  NULL);

#ifdef XIM_DEBUG
    xim_log("im_preedit_changed_cb(): %s\n", preedit_string);
#endif

    g_return_if_fail(preedit_string != NULL); // just in case

    if (p_imst == IM_OVER_THE_SPOT)
    {
	if (preedit_string[0] == NUL)
	{
	    xim_has_preediting = FALSE;
	    im_delete_preedit();
	}
	else
	{
	    xim_has_preediting = TRUE;
	    im_show_preedit();
	}
    }
    else
    {
	// If preedit_start_col is MAXCOL set it to the current cursor position.
	if (preedit_start_col == MAXCOL && preedit_string[0] != '\0')
	{
	    xim_has_preediting = TRUE;

	    // Urgh, this breaks if the input buffer isn't empty now
	    init_preedit_start_col();
	}
	else if (cursor_index == 0 && preedit_string[0] == '\0')
	{
	    xim_has_preediting = FALSE;

	    // If at the start position (after typing backspace)
	    // preedit_start_col must be reset.
	    preedit_start_col = MAXCOL;
	}

	im_delete_preedit();

	// Compute the end of the preediting area: "preedit_end_col".
	// According to the documentation of
	// gtk_im_context_get_preedit_string(), the cursor_pos output argument
	// returns the offset in bytes.  This is unfortunately not true -- real
	// life shows the offset is in characters, and the GTK+ source code
	// agrees with me.  Will file a bug later.
	if (preedit_start_col != MAXCOL)
	    preedit_end_col = preedit_start_col;
	str = (char_u *)preedit_string;
	for (p = str, i = 0; *p != NUL; p += utf_byte2len(*p), ++i)
	{
	    int is_composing;

	    is_composing = ((*p & 0x80) != 0
					  && utf_iscomposing(utf_ptr2char(p)));
	    // These offsets are used as counters when generating <BS> and
	    // <Del> to delete the preedit string.  So don't count composing
	    // characters unless 'delcombine' is enabled.
	    if (!is_composing || p_deco)
	    {
		if (i < cursor_index)
		    ++im_preedit_cursor;
		else
		    ++im_preedit_trailing;
	    }
	    if (!is_composing && i >= cursor_index)
	    {
		// This is essentially the same as im_preedit_trailing, except
		// composing characters are not counted even if p_deco is set.
		++num_move_back;
	    }
	    if (preedit_start_col != MAXCOL)
		preedit_end_col += utf_ptr2cells(p);
	}

	if (p > str)
	{
	    im_add_to_input(str, (int)(p - str));
	    im_correct_cursor(num_move_back);
	}
    }

    g_free(preedit_string);

    if (gtk_main_level() > 0)
	gtk_main_quit();
}

/*
 * Translate the Pango attributes at iter to Vim highlighting attributes.
 * Ignore attributes not supported by Vim highlighting.  This shouldn't have
 * too much impact -- right now we handle even more attributes than necessary
 * for the IM modules I tested with.
 */
    static int
translate_pango_attributes(PangoAttrIterator *iter)
{
    PangoAttribute  *attr;
    int		    char_attr = HL_NORMAL;

    attr = pango_attr_iterator_get(iter, PANGO_ATTR_UNDERLINE);
    if (attr != NULL && ((PangoAttrInt *)attr)->value
						 != (int)PANGO_UNDERLINE_NONE)
	char_attr |= HL_UNDERLINE;

    attr = pango_attr_iterator_get(iter, PANGO_ATTR_WEIGHT);
    if (attr != NULL && ((PangoAttrInt *)attr)->value >= (int)PANGO_WEIGHT_BOLD)
	char_attr |= HL_BOLD;

    attr = pango_attr_iterator_get(iter, PANGO_ATTR_STYLE);
    if (attr != NULL && ((PangoAttrInt *)attr)->value
						   != (int)PANGO_STYLE_NORMAL)
	char_attr |= HL_ITALIC;

    attr = pango_attr_iterator_get(iter, PANGO_ATTR_BACKGROUND);
    if (attr != NULL)
    {
	const PangoColor *color = &((PangoAttrColor *)attr)->color;

	// Assume inverse if black background is requested
	if ((color->red | color->green | color->blue) == 0)
	    char_attr |= HL_INVERSE;
    }

    return char_attr;
}

/*
 * Retrieve the highlighting attributes at column col in the preedit string.
 * Return -1 if not in preediting mode or if col is out of range.
 */
    int
im_get_feedback_attr(int col)
{
    char	    *preedit_string = NULL;
    PangoAttrList   *attr_list	    = NULL;
    int		    char_attr	    = -1;

    if (xic == NULL)
	return char_attr;

    gtk_im_context_get_preedit_string(xic, &preedit_string, &attr_list, NULL);

    if (preedit_string != NULL && attr_list != NULL)
    {
	int idx;

	// Get the byte index as used by PangoAttrIterator
	for (idx = 0; col > 0 && preedit_string[idx] != '\0'; --col)
	    idx += utfc_ptr2len((char_u *)preedit_string + idx);

	if (preedit_string[idx] != '\0')
	{
	    PangoAttrIterator	*iter;
	    int			start, end;

	    char_attr = HL_NORMAL;
	    iter = pango_attr_list_get_iterator(attr_list);

	    // Extract all relevant attributes from the list.
	    do
	    {
		pango_attr_iterator_range(iter, &start, &end);

		if (idx >= start && idx < end)
		    char_attr |= translate_pango_attributes(iter);
	    }
	    while (pango_attr_iterator_next(iter));

	    pango_attr_iterator_destroy(iter);
	}
    }

    if (attr_list != NULL)
	pango_attr_list_unref(attr_list);
    g_free(preedit_string);

    return char_attr;
}

    void
xim_init(void)
{
#ifdef XIM_DEBUG
    xim_log("xim_init()\n");
#endif

    g_return_if_fail(gui.drawarea != NULL);
    g_return_if_fail(gtk_widget_get_window(gui.drawarea) != NULL);

    xic = gtk_im_multicontext_new();
    g_object_ref(xic);

    im_commit_handler_id = g_signal_connect(G_OBJECT(xic), "commit",
					    G_CALLBACK(&im_commit_cb), NULL);
    g_signal_connect(G_OBJECT(xic), "preedit_changed",
		     G_CALLBACK(&im_preedit_changed_cb), NULL);
    g_signal_connect(G_OBJECT(xic), "preedit_start",
		     G_CALLBACK(&im_preedit_start_cb), NULL);
    g_signal_connect(G_OBJECT(xic), "preedit_end",
		     G_CALLBACK(&im_preedit_end_cb), NULL);

    gtk_im_context_set_client_window(xic, gtk_widget_get_window(gui.drawarea));
}

    void
im_shutdown(void)
{
#ifdef XIM_DEBUG
    xim_log("im_shutdown()\n");
#endif

    if (xic != NULL)
    {
	gtk_im_context_focus_out(xic);
	g_object_unref(xic);
	xic = NULL;
    }
    im_is_active = FALSE;
    im_commit_handler_id = 0;
    if (p_imst == IM_ON_THE_SPOT)
	preedit_start_col = MAXCOL;
    xim_has_preediting = FALSE;
}

/*
 * Convert the string argument to keyval and state for GdkEventKey.
 * If str is valid return TRUE, otherwise FALSE.
 *
 * See 'imactivatekey' for documentation of the format.
 */
    static int
im_string_to_keyval(const char *str, unsigned int *keyval, unsigned int *state)
{
    const char	    *mods_end;
    unsigned	    tmp_keyval;
    unsigned	    tmp_state = 0;

    mods_end = strrchr(str, '-');
    mods_end = (mods_end != NULL) ? mods_end + 1 : str;

    // Parse modifier keys
    while (str < mods_end)
	switch (*str++)
	{
	    case '-':							break;
	    case 'S': case 's': tmp_state |= (unsigned)GDK_SHIFT_MASK;	break;
	    case 'L': case 'l': tmp_state |= (unsigned)GDK_LOCK_MASK;	break;
	    case 'C': case 'c': tmp_state |= (unsigned)GDK_CONTROL_MASK;break;
	    case '1':		tmp_state |= (unsigned)GDK_MOD1_MASK;	break;
	    case '2':		tmp_state |= (unsigned)GDK_MOD2_MASK;	break;
	    case '3':		tmp_state |= (unsigned)GDK_MOD3_MASK;	break;
	    case '4':		tmp_state |= (unsigned)GDK_MOD4_MASK;	break;
	    case '5':		tmp_state |= (unsigned)GDK_MOD5_MASK;	break;
	    default:
		return FALSE;
	}

    tmp_keyval = gdk_keyval_from_name(str);

    if (tmp_keyval == 0 || tmp_keyval == GDK_VoidSymbol)
	return FALSE;

    if (keyval != NULL)
	*keyval = tmp_keyval;
    if (state != NULL)
	*state = tmp_state;

    return TRUE;
}

/*
 * Return TRUE if p_imak is valid, otherwise FALSE.  As a special case, an
 * empty string is also regarded as valid.
 *
 * Note: The numerical key value of p_imak is cached if it was valid; thus
 * boldly assuming im_xim_isvalid_imactivate() will always be called whenever
 * 'imak' changes.  This is currently the case but not obvious -- should
 * probably rename the function for clarity.
 */
    int
im_xim_isvalid_imactivate(void)
{
    if (p_imak[0] == NUL)
    {
	im_activatekey_keyval = GDK_VoidSymbol;
	im_activatekey_state  = 0;
	return TRUE;
    }

    return im_string_to_keyval((const char *)p_imak,
			       &im_activatekey_keyval,
			       &im_activatekey_state);
}

    static void
im_synthesize_keypress(unsigned int keyval, unsigned int state)
{
    GdkEventKey *event;

    event = (GdkEventKey *)gdk_event_new(GDK_KEY_PRESS);
    g_object_ref(gtk_widget_get_window(gui.drawarea));
					// unreffed by gdk_event_free()
    event->window = gtk_widget_get_window(gui.drawarea);
    event->send_event = TRUE;
    event->time = GDK_CURRENT_TIME;
    event->state  = state;
    event->keyval = keyval;
    event->hardware_keycode = // needed for XIM
	XKeysymToKeycode(GDK_WINDOW_XDISPLAY(event->window), (KeySym)keyval);
    event->length = 0;
    event->string = NULL;

    gtk_im_context_filter_keypress(xic, event);

    // For consistency, also send the corresponding release event.
    event->type = GDK_KEY_RELEASE;
    event->send_event = FALSE;
    gtk_im_context_filter_keypress(xic, event);

    gdk_event_free((GdkEvent *)event);
}

    void
xim_reset(void)
{
# ifdef FEAT_EVAL
    if (USE_IMACTIVATEFUNC)
	call_imactivatefunc(im_is_active);
    else
# endif
    if (xic != NULL)
    {
	gtk_im_context_reset(xic);

	if (p_imdisable)
	    im_shutdown();
	else
	{
	    xim_set_focus(gui.in_focus);

	    if (im_activatekey_keyval != GDK_VoidSymbol)
	    {
		if (im_is_active)
		{
		    g_signal_handler_block(xic, im_commit_handler_id);
		    im_synthesize_keypress(im_activatekey_keyval,
						    im_activatekey_state);
		    g_signal_handler_unblock(xic, im_commit_handler_id);
		}
	    }
	    else
	    {
		im_shutdown();
		xim_init();
		xim_set_focus(gui.in_focus);
	    }
	}
    }

    if (p_imst == IM_ON_THE_SPOT)
	preedit_start_col = MAXCOL;
    xim_has_preediting = FALSE;
}

    int
xim_queue_key_press_event(GdkEventKey *event, int down)
{
#ifdef FEAT_GUI_GTK
    if (event->state & GDK_SUPER_MASK) return FALSE;
#endif
    if (down)
    {
	// Workaround GTK2 XIM 'feature' that always converts keypad keys to
	// chars., even when not part of an IM sequence (ref. feature of
	// gdk/gdkkeyuni.c).
	// Flag any keypad keys that might represent a single char.
	// If this (on its own - i.e., not part of an IM sequence) is
	// committed while we're processing one of these keys, we can ignore
	// that commit and go ahead & process it ourselves.  That way we can
	// still distinguish keypad keys for use in mappings.
	// Also add GDK_space to make <S-Space> work.
	switch (event->keyval)
	{
	    case GDK_KP_Add:      xim_expected_char = '+';  break;
	    case GDK_KP_Subtract: xim_expected_char = '-';  break;
	    case GDK_KP_Divide:   xim_expected_char = '/';  break;
	    case GDK_KP_Multiply: xim_expected_char = '*';  break;
	    case GDK_KP_Decimal:  xim_expected_char = '.';  break;
	    case GDK_KP_Equal:    xim_expected_char = '=';  break;
	    case GDK_KP_0:	  xim_expected_char = '0';  break;
	    case GDK_KP_1:	  xim_expected_char = '1';  break;
	    case GDK_KP_2:	  xim_expected_char = '2';  break;
	    case GDK_KP_3:	  xim_expected_char = '3';  break;
	    case GDK_KP_4:	  xim_expected_char = '4';  break;
	    case GDK_KP_5:	  xim_expected_char = '5';  break;
	    case GDK_KP_6:	  xim_expected_char = '6';  break;
	    case GDK_KP_7:	  xim_expected_char = '7';  break;
	    case GDK_KP_8:	  xim_expected_char = '8';  break;
	    case GDK_KP_9:	  xim_expected_char = '9';  break;
	    case GDK_space:	  xim_expected_char = ' ';  break;
	    default:		  xim_expected_char = NUL;
	}
	xim_ignored_char = FALSE;
    }

    // When typing fFtT, XIM may be activated. Thus it must pass
    // gtk_im_context_filter_keypress() in Normal mode.
    // And while doing :sh too.
    if (xic != NULL && !p_imdisable
		    && (State & (MODE_INSERT | MODE_CMDLINE
					      | MODE_NORMAL | MODE_EXTERNCMD)))
    {
	// Filter 'imactivatekey' and map it to CTRL-^.  This way, Vim is
	// always aware of the current status of IM, and can even emulate
	// the activation key for modules that don't support one.
	if (event->keyval == im_activatekey_keyval
	     && (event->state & im_activatekey_state) == im_activatekey_state)
	{
	    unsigned int state_mask;

	    // Require the state of the 3 most used modifiers to match exactly.
	    // Otherwise e.g. <S-C-space> would be unusable for other purposes
	    // if the IM activate key is <S-space>.
	    state_mask  = im_activatekey_state;
	    state_mask |= ((int)GDK_SHIFT_MASK | (int)GDK_CONTROL_MASK
							| (int)GDK_MOD1_MASK);

	    if ((event->state & state_mask) != im_activatekey_state)
		return FALSE;

	    // Don't send it a second time on GDK_KEY_RELEASE.
	    if (event->type != GDK_KEY_PRESS)
		return TRUE;

	    if (map_to_exists_mode((char_u *)"", MODE_LANGMAP, FALSE))
	    {
		im_set_active(FALSE);

		// ":lmap" mappings exists, toggle use of mappings.
		State ^= MODE_LANGMAP;
		if (State & MODE_LANGMAP)
		{
		    curbuf->b_p_iminsert = B_IMODE_NONE;
		    State &= ~MODE_LANGMAP;
		}
		else
		{
		    curbuf->b_p_iminsert = B_IMODE_LMAP;
		    State |= MODE_LANGMAP;
		}
		return TRUE;
	    }

	    return gtk_im_context_filter_keypress(xic, event);
	}

	// Don't filter events through the IM context if IM isn't active
	// right now.  Unlike with GTK+ 1.2 we cannot rely on the IM module
	// not doing anything before the activation key was sent.
	if (im_activatekey_keyval == GDK_VoidSymbol || im_is_active)
	{
	    int imresult = gtk_im_context_filter_keypress(xic, event);

	    if (p_imst == IM_ON_THE_SPOT)
	    {
		// Some XIM send following sequence:
		// 1. preedited string.
		// 2. committed string.
		// 3. line changed key.
		// 4. preedited string.
		// 5. remove preedited string.
		// if 3, Vim can't move back the above line for 5.
		// thus, this part should not parse the key.
		if (!imresult && preedit_start_col != MAXCOL
					    && event->keyval == GDK_Return)
		{
		    im_synthesize_keypress(GDK_Return, 0U);
		    return FALSE;
		}
	    }

	    // If XIM tried to commit a keypad key as a single char.,
	    // ignore it so we can use the keypad key 'raw', for mappings.
	    if (xim_expected_char != NUL && xim_ignored_char)
		// We had a keypad key, and XIM tried to thieve it
		return FALSE;

	    // This is supposed to fix a problem with iBus, that space
	    // characters don't work in input mode.
	    xim_expected_char = NUL;

	    // Normal processing
	    return imresult;
	}
    }

    return FALSE;
}

    int
im_get_status(void)
{
#  ifdef FEAT_EVAL
    if (USE_IMSTATUSFUNC)
	return call_imstatusfunc();
#  endif
    return im_is_active;
}

    int
preedit_get_status(void)
{
    return preedit_is_active;
}

    int
im_is_preediting(void)
{
    return xim_has_preediting;
}

# else // !FEAT_GUI_GTK

static int	xim_is_active = FALSE;  // XIM should be active in the current
					// mode
static int	xim_has_focus = FALSE;	// XIM is really being used for Vim
#  ifdef FEAT_GUI_X11
static XIMStyle	input_style;
static int	status_area_enabled = TRUE;
#  endif

/*
 * Switch using XIM on/off.  This is used by the code that changes "State".
 * When 'imactivatefunc' is defined use that function instead.
 */
    void
im_set_active(int active_arg)
{
    int active = active_arg;

    // If 'imdisable' is set, XIM is never active.
    if (p_imdisable)
	active = FALSE;
    else if (input_style & XIMPreeditPosition)
	// There is a problem in switching XIM off when preediting is used,
	// and it is not clear how this can be solved.  For now, keep XIM on
	// all the time, like it was done in Vim 5.8.
	active = TRUE;

#  if defined(FEAT_EVAL)
    if (USE_IMACTIVATEFUNC)
    {
	if (active != im_get_status())
	{
	    call_imactivatefunc(active);
	    xim_has_focus = active;
	}
	return;
    }
#  endif

    if (xic == NULL)
	return;

    // Remember the active state, it is needed when Vim gets keyboard focus.
    xim_is_active = active;
    xim_set_preedit();
}

/*
 * Adjust using XIM for gaining or losing keyboard focus.  Also called when
 * "xim_is_active" changes.
 */
    void
xim_set_focus(int focus)
{
    if (xic == NULL)
	return;

    // XIM only gets focus when the Vim window has keyboard focus and XIM has
    // been set active for the current mode.
    if (focus && xim_is_active)
    {
	if (!xim_has_focus)
	{
	    xim_has_focus = TRUE;
	    XSetICFocus(xic);
	}
    }
    else
    {
	if (xim_has_focus)
	{
	    xim_has_focus = FALSE;
	    XUnsetICFocus(xic);
	}
    }
}

    void
im_set_position(int row UNUSED, int col UNUSED)
{
    xim_set_preedit();
}

/*
 * Set the XIM to the current cursor position.
 */
    void
xim_set_preedit(void)
{
    XVaNestedList attr_list;
    XRectangle spot_area;
    XPoint over_spot;
    int line_space;

    if (xic == NULL)
	return;

    xim_set_focus(TRUE);

    if (!xim_has_focus)
    {
	// hide XIM cursor
	over_spot.x = 0;
	over_spot.y = -100; // arbitrary invisible position
	attr_list = (XVaNestedList) XVaCreateNestedList(0,
							XNSpotLocation,
							&over_spot,
							NULL);
	XSetICValues(xic, XNPreeditAttributes, attr_list, NULL);
	XFree(attr_list);
	return;
    }

    if (input_style & XIMPreeditPosition)
    {
	if (xim_fg_color == INVALCOLOR)
	{
	    xim_fg_color = gui.def_norm_pixel;
	    xim_bg_color = gui.def_back_pixel;
	}
	over_spot.x = TEXT_X(gui.col);
	over_spot.y = TEXT_Y(gui.row);
	spot_area.x = 0;
	spot_area.y = 0;
	spot_area.height = gui.char_height * Rows;
	spot_area.width  = gui.char_width * Columns;
	line_space = gui.char_height;
	attr_list = (XVaNestedList) XVaCreateNestedList(0,
					XNSpotLocation, &over_spot,
					XNForeground, (Pixel) xim_fg_color,
					XNBackground, (Pixel) xim_bg_color,
					XNArea, &spot_area,
					XNLineSpace, line_space,
					NULL);
	if (XSetICValues(xic, XNPreeditAttributes, attr_list, NULL))
	    emsg(_(e_cannot_set_ic_values));
	XFree(attr_list);
    }
}

#  if defined(FEAT_GUI_X11) || defined(PROTO)
#   if defined(XtSpecificationRelease) && XtSpecificationRelease >= 6 && !defined(SUN_SYSTEM)
#    define USE_X11R6_XIM
#   endif

static int xim_real_init(Window x11_window, Display *x11_display);


#  ifdef USE_X11R6_XIM
    static void
xim_instantiate_cb(
    Display	*display,
    XPointer	client_data UNUSED,
    XPointer	call_data UNUSED)
{
    Window	x11_window;
    Display	*x11_display;

#   ifdef XIM_DEBUG
    xim_log("xim_instantiate_cb()\n");
#   endif

    gui_get_x11_windis(&x11_window, &x11_display);
    if (display != x11_display)
	return;

    xim_real_init(x11_window, x11_display);
    gui_set_shellsize(FALSE, FALSE, RESIZE_BOTH);
    if (xic != NULL)
	XUnregisterIMInstantiateCallback(x11_display, NULL, NULL, NULL,
					 xim_instantiate_cb, NULL);
}

    static void
xim_destroy_cb(
    XIM		im UNUSED,
    XPointer	client_data UNUSED,
    XPointer	call_data UNUSED)
{
    Window	x11_window;
    Display	*x11_display;

#   ifdef XIM_DEBUG
    xim_log("xim_destroy_cb()\n");
   #endif
    gui_get_x11_windis(&x11_window, &x11_display);

    xic = NULL;
    status_area_enabled = FALSE;

    gui_set_shellsize(FALSE, FALSE, RESIZE_BOTH);

    XRegisterIMInstantiateCallback(x11_display, NULL, NULL, NULL,
				   xim_instantiate_cb, NULL);
}
#  endif

    void
xim_init(void)
{
    Window	x11_window;
    Display	*x11_display;

#  ifdef XIM_DEBUG
    xim_log("xim_init()\n");
#  endif

    gui_get_x11_windis(&x11_window, &x11_display);

    xic = NULL;

    if (xim_real_init(x11_window, x11_display))
	return;

    gui_set_shellsize(FALSE, FALSE, RESIZE_BOTH);

#  ifdef USE_X11R6_XIM
    XRegisterIMInstantiateCallback(x11_display, NULL, NULL, NULL,
				   xim_instantiate_cb, NULL);
#  endif
}

    static int
xim_real_init(Window x11_window, Display *x11_display)
{
    int		i;
    char	*p,
		*s,
		*ns,
		*end,
		tmp[1024];
#  define IMLEN_MAX 40
    char	buf[IMLEN_MAX + 7];
    XIM		xim = NULL;
    XIMStyles	*xim_styles;
    XIMStyle	this_input_style = 0;
    Boolean	found;
    XPoint	over_spot;
    XVaNestedList preedit_list, status_list;

    input_style = 0;
    status_area_enabled = FALSE;

    if (xic != NULL)
	return FALSE;

    if (gui.rsrc_input_method != NULL && *gui.rsrc_input_method != NUL)
    {
	strcpy(tmp, gui.rsrc_input_method);
	for (ns = s = tmp; ns != NULL && *s != NUL;)
	{
	    s = (char *)skipwhite((char_u *)s);
	    if (*s == NUL)
		break;
	    if ((ns = end = strchr(s, ',')) == NULL)
		end = s + strlen(s);
	    while (SAFE_isspace(end[-1]))
		end--;
	    *end = NUL;

	    if (strlen(s) <= IMLEN_MAX)
	    {
		strcpy(buf, "@im=");
		strcat(buf, s);
		if ((p = XSetLocaleModifiers(buf)) != NULL && *p != NUL
			&& (xim = XOpenIM(x11_display, NULL, NULL, NULL))
								      != NULL)
		    break;
	    }

	    s = ns + 1;
	}
    }

    if (xim == NULL && (p = XSetLocaleModifiers("")) != NULL && *p != NUL)
	xim = XOpenIM(x11_display, NULL, NULL, NULL);

    // This is supposed to be useful to obtain characters through
    // XmbLookupString() without really using a XIM.
    if (xim == NULL && (p = XSetLocaleModifiers("@im=none")) != NULL
								 && *p != NUL)
	xim = XOpenIM(x11_display, NULL, NULL, NULL);

    if (xim == NULL)
    {
	// Only give this message when verbose is set, because too many people
	// got this message when they didn't want to use a XIM.
	if (p_verbose > 0)
	{
	    verbose_enter();
	    emsg(_(e_failed_to_open_input_method));
	    verbose_leave();
	}
	return FALSE;
    }

#  ifdef USE_X11R6_XIM
    {
	XIMCallback destroy_cb;

	destroy_cb.callback = xim_destroy_cb;
	destroy_cb.client_data = NULL;
	if (XSetIMValues(xim, XNDestroyCallback, &destroy_cb, NULL))
	    emsg(_(e_warning_could_not_set_destroy_callback_to_im));
    }
#  endif

    if (XGetIMValues(xim, XNQueryInputStyle, &xim_styles, NULL) || !xim_styles)
    {
	emsg(_(e_input_method_doesnt_support_any_style));
	XCloseIM(xim);
	return FALSE;
    }

    found = False;
    strcpy(tmp, gui.rsrc_preedit_type_name);
    for (s = tmp; s && !found; )
    {
	while (*s && SAFE_isspace(*s))
	    s++;
	if (!*s)
	    break;
	if ((ns = end = strchr(s, ',')) != 0)
	    ns++;
	else
	    end = s + strlen(s);
	while (SAFE_isspace(*end))
	    end--;
	*end = '\0';

	if (!strcmp(s, "OverTheSpot"))
	    this_input_style = (XIMPreeditPosition | XIMStatusArea);
	else if (!strcmp(s, "OffTheSpot"))
	    this_input_style = (XIMPreeditArea | XIMStatusArea);
	else if (!strcmp(s, "Root"))
	    this_input_style = (XIMPreeditNothing | XIMStatusNothing);

	for (i = 0; (unsigned short)i < xim_styles->count_styles; i++)
	{
	    if (this_input_style == xim_styles->supported_styles[i])
	    {
		found = True;
		break;
	    }
	}
	if (!found)
	    for (i = 0; (unsigned short)i < xim_styles->count_styles; i++)
	    {
		if ((xim_styles->supported_styles[i] & this_input_style)
			== (this_input_style & ~XIMStatusArea))
		{
		    this_input_style &= ~XIMStatusArea;
		    found = True;
		    break;
		}
	    }

	s = ns;
    }
    XFree(xim_styles);

    if (!found)
    {
	// Only give this message when verbose is set, because too many people
	// got this message when they didn't want to use a XIM.
	if (p_verbose > 0)
	{
	    verbose_enter();
	    emsg(_(e_input_method_doesnt_support_my_preedit_type));
	    verbose_leave();
	}
	XCloseIM(xim);
	return FALSE;
    }

    over_spot.x = TEXT_X(gui.col);
    over_spot.y = TEXT_Y(gui.row);
    input_style = this_input_style;

    // A crash was reported when trying to pass gui.norm_font as XNFontSet,
    // thus that has been removed.  Hopefully the default works...
#  ifdef FEAT_XFONTSET
    if (gui.fontset != NOFONTSET)
    {
	preedit_list = XVaCreateNestedList(0,
				XNSpotLocation, &over_spot,
				XNForeground, (Pixel)gui.def_norm_pixel,
				XNBackground, (Pixel)gui.def_back_pixel,
				XNFontSet, (XFontSet)gui.fontset,
				NULL);
	status_list = XVaCreateNestedList(0,
				XNForeground, (Pixel)gui.def_norm_pixel,
				XNBackground, (Pixel)gui.def_back_pixel,
				XNFontSet, (XFontSet)gui.fontset,
				NULL);
    }
    else
#  endif
    {
	preedit_list = XVaCreateNestedList(0,
				XNSpotLocation, &over_spot,
				XNForeground, (Pixel)gui.def_norm_pixel,
				XNBackground, (Pixel)gui.def_back_pixel,
				NULL);
	status_list = XVaCreateNestedList(0,
				XNForeground, (Pixel)gui.def_norm_pixel,
				XNBackground, (Pixel)gui.def_back_pixel,
				NULL);
    }

    xic = XCreateIC(xim,
		    XNInputStyle, input_style,
		    XNClientWindow, x11_window,
		    XNFocusWindow, gui.wid,
		    XNPreeditAttributes, preedit_list,
		    XNStatusAttributes, status_list,
		    NULL);
    XFree(status_list);
    XFree(preedit_list);
    if (xic != NULL)
    {
	if (input_style & XIMStatusArea)
	{
	    xim_set_status_area();
	    status_area_enabled = TRUE;
	}
	else
	    gui_set_shellsize(FALSE, FALSE, RESIZE_BOTH);
    }
    else
    {
	if (!is_not_a_term())
	    emsg(_(e_failed_to_create_input_context));
	XCloseIM(xim);
	return FALSE;
    }

    return TRUE;
}

#  endif // FEAT_GUI_X11

/*
 * Get IM status.  When IM is on, return TRUE.  Else return FALSE.
 * FIXME: This doesn't work correctly: Having focus doesn't always mean XIM is
 * active, when not having focus XIM may still be active (e.g., when using a
 * tear-off menu item).
 */
    int
im_get_status(void)
{
#  ifdef FEAT_EVAL
    if (USE_IMSTATUSFUNC)
	return call_imstatusfunc();
#  endif
    return xim_has_focus;
}

# endif // !FEAT_GUI_GTK

# if !defined(FEAT_GUI_GTK) || defined(PROTO)
/*
 * Set up the status area.
 *
 * This should use a separate Widget, but that seems not possible, because
 * preedit_area and status_area should be set to the same window as for the
 * text input.  Unfortunately this means the status area pollutes the text
 * window...
 */
    void
xim_set_status_area(void)
{
    XVaNestedList preedit_list = 0, status_list = 0, list = 0;
    XRectangle pre_area, status_area;

    if (xic == NULL)
	return;

    if (input_style & XIMStatusArea)
    {
	if (input_style & XIMPreeditArea)
	{
	    XRectangle *needed_rect;

	    // to get status_area width
	    status_list = XVaCreateNestedList(0, XNAreaNeeded,
					      &needed_rect, NULL);
	    XGetICValues(xic, XNStatusAttributes, status_list, NULL);
	    XFree(status_list);

	    status_area.width = needed_rect->width;
	}
	else
	    status_area.width = gui.char_width * Columns;

	status_area.x = 0;
	status_area.y = gui.char_height * Rows + gui.border_offset;
	if (gui.which_scrollbars[SBAR_BOTTOM])
	    status_area.y += gui.scrollbar_height;
#ifdef FEAT_MENU
	if (gui.menu_is_active)
	    status_area.y += gui.menu_height;
#endif
	status_area.height = gui.char_height;
	status_list = XVaCreateNestedList(0, XNArea, &status_area, NULL);
    }
    else
    {
	status_area.x = 0;
	status_area.y = gui.char_height * Rows + gui.border_offset;
	if (gui.which_scrollbars[SBAR_BOTTOM])
	    status_area.y += gui.scrollbar_height;
#ifdef FEAT_MENU
	if (gui.menu_is_active)
	    status_area.y += gui.menu_height;
#endif
	status_area.width = 0;
	status_area.height = gui.char_height;
    }

    if (input_style & XIMPreeditArea)   // off-the-spot
    {
	pre_area.x = status_area.x + status_area.width;
	pre_area.y = gui.char_height * Rows + gui.border_offset;
	pre_area.width = gui.char_width * Columns - pre_area.x;
	if (gui.which_scrollbars[SBAR_BOTTOM])
	    pre_area.y += gui.scrollbar_height;
#ifdef FEAT_MENU
	if (gui.menu_is_active)
	    pre_area.y += gui.menu_height;
#endif
	pre_area.height = gui.char_height;
	preedit_list = XVaCreateNestedList(0, XNArea, &pre_area, NULL);
    }
    else if (input_style & XIMPreeditPosition)   // over-the-spot
    {
	pre_area.x = 0;
	pre_area.y = 0;
	pre_area.height = gui.char_height * Rows;
	pre_area.width = gui.char_width * Columns;
	preedit_list = XVaCreateNestedList(0, XNArea, &pre_area, NULL);
    }

    if (preedit_list && status_list)
	list = XVaCreateNestedList(0, XNPreeditAttributes, preedit_list,
				   XNStatusAttributes, status_list, NULL);
    else if (preedit_list)
	list = XVaCreateNestedList(0, XNPreeditAttributes, preedit_list,
				   NULL);
    else if (status_list)
	list = XVaCreateNestedList(0, XNStatusAttributes, status_list,
				   NULL);
    else
	list = NULL;

    if (list)
    {
	XSetICValues(xic, XNVaNestedList, list, NULL);
	XFree(list);
    }
    if (status_list)
	XFree(status_list);
    if (preedit_list)
	XFree(preedit_list);
}

    int
xim_get_status_area_height(void)
{
    if (status_area_enabled)
	return gui.char_height;
    return 0;
}
# endif

#else // !defined(FEAT_XIM)

# if defined(IME_WITHOUT_XIM) || defined(VIMDLL) || defined(PROTO)
static int im_was_set_active = FALSE;

    int
#  ifdef VIMDLL
mbyte_im_get_status(void)
#  else
im_get_status(void)
#  endif
{
#  if defined(FEAT_EVAL)
    if (USE_IMSTATUSFUNC)
	return call_imstatusfunc();
#  endif
    return im_was_set_active;
}

    void
#  ifdef VIMDLL
mbyte_im_set_active(int active_arg)
#  else
im_set_active(int active_arg)
#  endif
{
#  if defined(FEAT_EVAL)
    int	    active = !p_imdisable && active_arg;

    if (USE_IMACTIVATEFUNC && active != im_get_status())
    {
	call_imactivatefunc(active);
	im_was_set_active = active;
    }
#  endif
}

#  if defined(FEAT_GUI) && !defined(FEAT_GUI_HAIKU) && !defined(VIMDLL)
    void
im_set_position(int row UNUSED, int col UNUSED)
{
}
#  endif
# endif

#endif // FEAT_XIM
