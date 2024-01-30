/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

/*
 * drawline.c: Functions for drawing window lines on the screen.
 * This is the middle level, drawscreen.c is the higher level and screen.c the
 * lower level.
 */

#include "vim.h"

#ifdef FEAT_SYN_HL
/*
 * Advance **color_cols and return TRUE when there are columns to draw.
 */
    static int
advance_color_col(int vcol, int **color_cols)
{
    while (**color_cols >= 0 && vcol > **color_cols)
	++*color_cols;
    return (**color_cols >= 0);
}
#endif

#ifdef FEAT_SYN_HL
/*
 * Used when 'cursorlineopt' contains "screenline": compute the margins between
 * which the highlighting is used.
 */
    static void
margin_columns_win(win_T *wp, int *left_col, int *right_col)
{
    // cache previous calculations depending on w_virtcol
    static int saved_w_virtcol;
    static win_T *prev_wp;
    static int prev_left_col;
    static int prev_right_col;
    static int prev_col_off;

    int cur_col_off = win_col_off(wp);
    int	width1;
    int	width2;

    if (saved_w_virtcol == wp->w_virtcol
	    && prev_wp == wp && prev_col_off == cur_col_off)
    {
	*right_col = prev_right_col;
	*left_col = prev_left_col;
	return;
    }

    width1 = wp->w_width - cur_col_off;
    width2 = width1 + win_col_off2(wp);

    *left_col = 0;
    *right_col = width1;

    if (wp->w_virtcol >= (colnr_T)width1)
	*right_col = width1 + ((wp->w_virtcol - width1) / width2 + 1) * width2;
    if (wp->w_virtcol >= (colnr_T)width1 && width2 > 0)
	*left_col = (wp->w_virtcol - width1) / width2 * width2 + width1;

    // cache values
    prev_left_col = *left_col;
    prev_right_col = *right_col;
    prev_wp = wp;
    saved_w_virtcol = wp->w_virtcol;
    prev_col_off = cur_col_off;
}
#endif

#if defined(FEAT_SIGNS) || defined(FEAT_QUICKFIX) \
	|| defined(FEAT_SYN_HL) || defined(FEAT_DIFF)
// using an attribute for the whole line
# define LINE_ATTR
#endif

// structure with variables passed between win_line() and other functions
typedef struct {
    int		draw_state;	// what to draw next

    linenr_T	lnum;		// line number to be drawn

    int		startrow;	// first row in the window to be drawn
    int		row;		// row in the window, excl w_winrow
    int		screen_row;	// row on the screen, incl w_winrow

    long	vcol;		// virtual column, before wrapping
    int		col;		// visual column on screen, after wrapping
#ifdef FEAT_CONCEAL
    int		boguscols;	// nonexistent columns added to "col" to force
				// wrapping
    int		vcol_off_co;	// offset for concealed characters
#endif
    int		vcol_off_tp;	// offset for virtual text
#ifdef FEAT_SYN_HL
    int		draw_color_col;	// highlight colorcolumn
    int		*color_cols;	// pointer to according columns array
#endif
    int		eol_hl_off;	// 1 if highlighted char after EOL

    unsigned	off;		// offset in ScreenLines/ScreenAttrs

    int		win_attr;	// background for the whole window, except
				// margins and "~" lines.
    int		wcr_attr;	// attributes from 'wincolor'
#ifdef FEAT_SYN_HL
    int		cul_attr;	// set when 'cursorline' active
#endif
#ifdef LINE_ATTR
    int		line_attr;	// for the whole line, includes 'cursorline'
#endif

    int		screen_line_flags;  // flags for screen_line()

    int		fromcol;	// start of inverting
    int		tocol;		// end of inverting

#ifdef FEAT_LINEBREAK
    long	vcol_sbr;	    // virtual column after showbreak
    int		need_showbreak;	    // overlong line, skipping first x chars
    int		dont_use_showbreak; // do not use 'showbreak'
#endif
#ifdef FEAT_PROP_POPUP
    int		text_prop_above_count;
#endif

    // TRUE when 'cursorlineopt' has "screenline" and cursor is in this line
    int		cul_screenline;

    int		char_attr;	// attributes for the next character

    int		n_extra;	// number of extra bytes
    char_u	*p_extra;	// string of extra chars, plus NUL, only used
				// when c_extra and c_final are NUL
    char_u	*p_extra_free;  // p_extra buffer that needs to be freed
    int		extra_attr;	// attributes for p_extra, should be combined
				// with win_attr if needed
    int		n_attr_skip;    // chars to skip before using extra_attr
    int		c_extra;	// extra chars, all the same
    int		c_final;	// final char, mandatory if set
    int		extra_for_textprop; // n_extra set for textprop
    int		start_extra_for_textprop; // extra_for_textprop was just set

    // saved "extra" items for when draw_state becomes WL_LINE (again)
    int		saved_n_extra;
    char_u	*saved_p_extra;
    char_u	*saved_p_extra_free;
    int		saved_extra_attr;
    int		saved_n_attr_skip;
    int		saved_extra_for_textprop;
    int		saved_c_extra;
    int		saved_c_final;
    int		saved_char_attr;

    char_u	extra[NUMBUFLEN + MB_MAXBYTES];
				// "%ld " and 'fdc' must fit in here, as well
				// any text sign

#ifdef FEAT_DIFF
    hlf_T	diff_hlf;	// type of diff highlighting
#endif
    int		filler_lines;	// nr of filler lines to be drawn
    int		filler_todo;	// nr of filler lines still to do + 1
#ifdef FEAT_SIGNS
    sign_attrs_T sattr;
#endif
#ifdef FEAT_LINEBREAK
     // do consider wrapping in linebreak mode only after encountering
     // a non whitespace char
    int		need_lbr;
#endif
} winlinevars_T;

// draw_state values for items that are drawn in sequence:
#define WL_START	0		// nothing done yet, must be zero
#define WL_CMDLINE	(WL_START + 1)	// cmdline window column
#ifdef FEAT_FOLDING
# define WL_FOLD	(WL_CMDLINE + 1)	// 'foldcolumn'
#else
# define WL_FOLD	WL_CMDLINE
#endif
#ifdef FEAT_SIGNS
# define WL_SIGN	(WL_FOLD + 1)	// column for signs
#else
# define WL_SIGN	WL_FOLD		// column for signs
#endif
#define WL_NR		(WL_SIGN + 1)	// line number
#ifdef FEAT_LINEBREAK
# define WL_BRI		(WL_NR + 1)	// 'breakindent'
#else
# define WL_BRI		WL_NR
#endif
#if defined(FEAT_LINEBREAK) || defined(FEAT_DIFF)
# define WL_SBR		(WL_BRI + 1)	// 'showbreak' or 'diff'
#else
# define WL_SBR		WL_BRI
#endif
#define WL_LINE		(WL_SBR + 1)	// text in the line

#if defined(FEAT_SIGNS) || defined(FEAT_FOLDING)
/*
 * Return TRUE if CursorLineSign highlight is to be used.
 */
    static int
use_cursor_line_highlight(win_T *wp, linenr_T lnum)
{
    return wp->w_p_cul
	    && lnum == wp->w_cursor.lnum
	    && (wp->w_p_culopt_flags & CULOPT_NBR);
}
#endif


#ifdef FEAT_FOLDING
/*
 * Setup for drawing the 'foldcolumn', if there is one.
 */
    static void
handle_foldcolumn(win_T *wp, winlinevars_T *wlv)
{
    int fdc = compute_foldcolumn(wp, 0);

    if (fdc <= 0)
	return;

    // Allocate a buffer, "wlv->extra[]" may already be in use.
    vim_free(wlv->p_extra_free);
    wlv->p_extra_free = alloc(MAX_MCO * fdc + 1);
    if (wlv->p_extra_free == NULL)
	return;

    wlv->n_extra = (int)fill_foldcolumn(wlv->p_extra_free,
							 wp, FALSE, wlv->lnum);
    wlv->p_extra_free[wlv->n_extra] = NUL;
    wlv->p_extra = wlv->p_extra_free;
    wlv->c_extra = NUL;
    wlv->c_final = NUL;
    if (use_cursor_line_highlight(wp, wlv->lnum))
	wlv->char_attr = hl_combine_attr(wlv->wcr_attr, HL_ATTR(HLF_CLF));
    else
	wlv->char_attr = hl_combine_attr(wlv->wcr_attr, HL_ATTR(HLF_FC));
}
#endif

#ifdef FEAT_SIGNS
/*
 * Get information needed to display the sign in line "wlv->lnum" in window
 * "wp".
 * If "nrcol" is TRUE, the sign is going to be displayed in the number column.
 * Otherwise the sign is going to be displayed in the sign column.
 */
    static void
get_sign_display_info(
	int		nrcol,
	win_T		*wp,
	winlinevars_T	*wlv)
{
    int	text_sign;
# ifdef FEAT_SIGN_ICONS
    int	icon_sign;
# endif

    // Draw two cells with the sign value or blank.
    wlv->c_extra = ' ';
    wlv->c_final = NUL;
    if (nrcol)
	wlv->n_extra = number_width(wp) + 1;
    else
    {
	if (use_cursor_line_highlight(wp, wlv->lnum))
	    wlv->char_attr = hl_combine_attr(wlv->wcr_attr, HL_ATTR(HLF_CLS));
	else
	    wlv->char_attr = hl_combine_attr(wlv->wcr_attr, HL_ATTR(HLF_SC));
	wlv->n_extra = 2;
    }

    if (wlv->row == wlv->startrow
#ifdef FEAT_DIFF
	    + wlv->filler_lines && wlv->filler_todo <= 0
#endif
       )
    {
	text_sign = (wlv->sattr.sat_text != NULL) ? wlv->sattr.sat_typenr : 0;
# ifdef FEAT_SIGN_ICONS
	icon_sign = (wlv->sattr.sat_icon != NULL) ? wlv->sattr.sat_typenr : 0;
	if (gui.in_use && icon_sign != 0)
	{
	    // Use the image in this position.
	    if (nrcol)
	    {
		wlv->c_extra = NUL;
		sprintf((char *)wlv->extra, "%-*c ",
						  number_width(wp), SIGN_BYTE);
		wlv->p_extra = wlv->extra;
		wlv->n_extra = (int)STRLEN(wlv->p_extra);
	    }
	    else
		wlv->c_extra = SIGN_BYTE;
#  ifdef FEAT_NETBEANS_INTG
	    if (netbeans_active() && (buf_signcount(wp->w_buffer, wlv->lnum)
									  > 1))
	    {
		if (nrcol)
		{
		    wlv->c_extra = NUL;
		    sprintf((char *)wlv->extra, "%-*c ", number_width(wp),
							MULTISIGN_BYTE);
		    wlv->p_extra = wlv->extra;
		    wlv->n_extra = (int)STRLEN(wlv->p_extra);
		}
		else
		    wlv->c_extra = MULTISIGN_BYTE;
	    }
#  endif
	    wlv->c_final = NUL;
	    wlv->char_attr = icon_sign;
	}
	else
# endif
	    if (text_sign != 0)
	    {
		wlv->p_extra = wlv->sattr.sat_text;
		if (wlv->p_extra != NULL)
		{
		    if (nrcol)
		    {
			int width = number_width(wp) - 2;
			int n;

			for (n = 0; n < width; n++)
			    wlv->extra[n] = ' ';
			vim_snprintf((char *)wlv->extra + n,
				  sizeof(wlv->extra) - n, "%s ", wlv->p_extra);
			wlv->p_extra = wlv->extra;
		    }
		    wlv->c_extra = NUL;
		    wlv->c_final = NUL;
		    wlv->n_extra = (int)STRLEN(wlv->p_extra);
		}

		if (use_cursor_line_highlight(wp, wlv->lnum)
						  && wlv->sattr.sat_culhl > 0)
		    wlv->char_attr = wlv->sattr.sat_culhl;
		else
		    wlv->char_attr = wlv->sattr.sat_texthl;
	    }
    }
}
#endif

/*
 * Display the absolute or relative line number.  After the first row fill with
 * blanks when the 'n' flag isn't in 'cpo'.
 */
    static void
handle_lnum_col(
	win_T		*wp,
	winlinevars_T	*wlv,
	int		sign_present UNUSED,
	int		num_attr UNUSED)
{
    int has_cpo_n = vim_strchr(p_cpo, CPO_NUMCOL) != NULL;
    int lnum_row = wlv->startrow + wlv->filler_lines
#ifdef FEAT_PROP_POPUP
		      + wlv->text_prop_above_count
#endif
		      ;

    if ((wp->w_p_nu || wp->w_p_rnu)
	     && (wlv->row <= lnum_row || !has_cpo_n)
	     // there is no line number in a wrapped line when "n" is in
	     // 'cpoptions', but 'breakindent' assumes it anyway.
	     && !((has_cpo_n
#ifdef FEAT_LINEBREAK
		     && !wp->w_p_bri
#endif
		  ) && wp->w_skipcol > 0 && wlv->lnum == wp->w_topline))
    {
#ifdef FEAT_SIGNS
	// If 'signcolumn' is set to 'number' and a sign is present
	// in 'lnum', then display the sign instead of the line
	// number.
	if ((*wp->w_p_scl == 'n' && *(wp->w_p_scl + 1) == 'u') && sign_present)
	    get_sign_display_info(TRUE, wp, wlv);
	else
#endif
	{
	  // Draw the line number (empty space after wrapping).
	  // When there are text properties above the line put the line number
	  // below them.
	  if (wlv->row == lnum_row
		    && (wp->w_skipcol == 0 || wlv->row > 0
					       || (wp->w_p_nu && wp->w_p_rnu)))
	  {
	      long num;
	      char *fmt = "%*ld ";

	      if (wp->w_p_nu && !wp->w_p_rnu)
		  // 'number' + 'norelativenumber'
		  num = (long)wlv->lnum;
	      else
	      {
		  // 'relativenumber', don't use negative numbers
		  num = labs((long)get_cursor_rel_lnum(wp, wlv->lnum));
		  if (num == 0 && wp->w_p_nu && wp->w_p_rnu)
		  {
		      // 'number' + 'relativenumber'
		      num = wlv->lnum;
		      fmt = "%-*ld ";
		  }
	      }

	      sprintf((char *)wlv->extra, fmt, number_width(wp), num);
	      if (wp->w_skipcol > 0 && wlv->startrow == 0)
		  for (wlv->p_extra = wlv->extra; *wlv->p_extra == ' ';
			  ++wlv->p_extra)
		      *wlv->p_extra = '-';
#ifdef FEAT_RIGHTLEFT
	      if (wp->w_p_rl)		    // reverse line numbers
	      {
		  char_u    *p1, *p2;
		  int	    t;

		  // like rl_mirror(), but keep the space at the end
		  p2 = skipwhite(wlv->extra);
		  p2 = skiptowhite(p2) - 1;
		  for (p1 = skipwhite(wlv->extra); p1 < p2; ++p1, --p2)
		  {
		      t = *p1;
		      *p1 = *p2;
		      *p2 = t;
		  }
	      }
#endif
	      wlv->p_extra = wlv->extra;
	      wlv->c_extra = NUL;
	      wlv->c_final = NUL;
	  }
	  else
	  {
	      wlv->c_extra = ' ';
	      wlv->c_final = NUL;
	  }
	  wlv->n_extra = number_width(wp) + 1;
	  wlv->char_attr = hl_combine_attr(wlv->wcr_attr, HL_ATTR(HLF_N));
#ifdef FEAT_SYN_HL
	  // When 'cursorline' is set highlight the line number of
	  // the current line differently.
	  // When 'cursorlineopt' does not have "line" only
	  // highlight the line number itself.
	  // TODO: Can we use CursorLine instead of CursorLineNr
	  // when CursorLineNr isn't set?
	  if (wp->w_p_cul
		  && wlv->lnum == wp->w_cursor.lnum
		  && (wp->w_p_culopt_flags & CULOPT_NBR)
		  && (wlv->row == wlv->startrow + wlv->filler_lines
		      || (wlv->row > wlv->startrow + wlv->filler_lines
			 && (wp->w_p_culopt_flags & CULOPT_LINE))))
	    wlv->char_attr = hl_combine_attr(wlv->wcr_attr, HL_ATTR(HLF_CLN));
#endif
	  if (wp->w_p_rnu && wlv->lnum < wp->w_cursor.lnum
						      && HL_ATTR(HLF_LNA) != 0)
	      // Use LineNrAbove
	      wlv->char_attr = hl_combine_attr(wlv->wcr_attr, HL_ATTR(HLF_LNA));
	  if (wp->w_p_rnu && wlv->lnum > wp->w_cursor.lnum
						      && HL_ATTR(HLF_LNB) != 0)
	      // Use LineNrBelow
	      wlv->char_attr = hl_combine_attr(wlv->wcr_attr, HL_ATTR(HLF_LNB));
	}
#ifdef FEAT_SIGNS
	if (num_attr)
	    wlv->char_attr = num_attr;
#endif
    }
}

#ifdef FEAT_LINEBREAK
    static void
handle_breakindent(win_T *wp, winlinevars_T *wlv)
{
    if (wp->w_briopt_sbr && wlv->draw_state == WL_BRI - 1
					    && *get_showbreak_value(wp) != NUL)
	// draw indent after showbreak value
	wlv->draw_state = WL_BRI;
    else if (wp->w_briopt_sbr && wlv->draw_state == WL_SBR)
	// After the showbreak, draw the breakindent
	wlv->draw_state = WL_BRI - 1;

    // draw 'breakindent': indent wrapped text accordingly
    if (wlv->draw_state == WL_BRI - 1)
    {
	wlv->draw_state = WL_BRI;
	// if wlv->need_showbreak is set, breakindent also applies
	if (wp->w_p_bri && (wlv->row > wlv->startrow
# ifdef FEAT_DIFF
		    + wlv->filler_lines
# endif
		    || wlv->need_showbreak)
# ifdef FEAT_PROP_POPUP
		&& !wlv->dont_use_showbreak
# endif
	   )
	{
	    wlv->char_attr = 0;
# ifdef FEAT_DIFF
	    if (wlv->diff_hlf != (hlf_T)0)
		wlv->char_attr = HL_ATTR(wlv->diff_hlf);
# endif
	    wlv->p_extra = NULL;
	    wlv->c_extra = ' ';
	    wlv->c_final = NUL;
	    wlv->n_extra = get_breakindent_win(wp,
				   ml_get_buf(wp->w_buffer, wlv->lnum, FALSE));
	    if (wlv->row == wlv->startrow)
	    {
		wlv->n_extra -= win_col_off2(wp);
		if (wlv->n_extra < 0)
		    wlv->n_extra = 0;
	    }

	    // Correct start of highlighted area for 'breakindent',
	    if (wlv->fromcol >= wlv->vcol
				    && wlv->fromcol < wlv->vcol + wlv->n_extra)
		wlv->fromcol = wlv->vcol + wlv->n_extra;

	    // Correct end of highlighted area for 'breakindent',
	    // required when 'linebreak' is also set.
	    if (wlv->tocol == wlv->vcol)
		wlv->tocol += wlv->n_extra;
	}

	if (wp->w_skipcol > 0 && wlv->startrow == 0 && wp->w_p_wrap
							   && wp->w_briopt_sbr)
	    wlv->need_showbreak = FALSE;
    }
}
#endif

#if defined(FEAT_LINEBREAK) || defined(FEAT_DIFF)
    static void
handle_showbreak_and_filler(win_T *wp, winlinevars_T *wlv)
{
# ifdef FEAT_DIFF
    if (wlv->filler_todo > 0)
    {
	// Draw "deleted" diff line(s).
	if (char2cells(wp->w_fill_chars.diff) > 1)
	{
	    wlv->c_extra = '-';
	    wlv->c_final = NUL;
	}
	else
	{
	    wlv->c_extra = wp->w_fill_chars.diff;
	    wlv->c_final = NUL;
	}
#  ifdef FEAT_RIGHTLEFT
	if (wp->w_p_rl)
	    wlv->n_extra = wlv->col + 1;
	else
#  endif
	    wlv->n_extra = wp->w_width - wlv->col;
	wlv->char_attr = HL_ATTR(HLF_DED);
    }
# endif

# ifdef FEAT_LINEBREAK
    char_u *sbr = get_showbreak_value(wp);
    if (*sbr != NUL && wlv->need_showbreak)
    {
	// Draw 'showbreak' at the start of each broken line.
	wlv->p_extra = sbr;
	wlv->c_extra = NUL;
	wlv->c_final = NUL;
	wlv->n_extra = (int)STRLEN(sbr);
	wlv->vcol_sbr = wlv->vcol + MB_CHARLEN(sbr);

	// Correct start of highlighted area for 'showbreak'.
	if (wlv->fromcol >= wlv->vcol && wlv->fromcol < wlv->vcol_sbr)
	    wlv->fromcol = wlv->vcol_sbr;

	// Correct end of highlighted area for 'showbreak',
	// required when 'linebreak' is also set.
	if (wlv->tocol == wlv->vcol)
	    wlv->tocol += wlv->n_extra;
	// combine 'showbreak' with 'wincolor'
	wlv->char_attr = hl_combine_attr(wlv->win_attr, HL_ATTR(HLF_AT));
#  ifdef FEAT_SYN_HL
	// combine 'showbreak' with 'cursorline'
	if (wlv->cul_attr != 0)
	    wlv->char_attr = hl_combine_attr(wlv->char_attr, wlv->cul_attr);
#  endif
    }

    if (wp->w_skipcol == 0 || wlv->startrow > 0 || !wp->w_p_wrap
							  || !wp->w_briopt_sbr)
	wlv->need_showbreak = FALSE;
# endif
}
#endif

#if defined(FEAT_PROP_POPUP) || defined(PROTO)
/*
 * Return the cell size of virtual text after truncation.
 */
    static int
textprop_size_after_trunc(
	win_T	*wp,
	int	flags,	    // TP_FLAG_ALIGN_*
	int	added,
	int	padding,
	char_u	*text,
	int	*n_used_ptr)
{
    int	space = (flags & (TP_FLAG_ALIGN_BELOW | TP_FLAG_ALIGN_ABOVE))
				       ? wp->w_width - win_col_off(wp) : added;
    int len = (int)STRLEN(text);
    int strsize = 0;
    int n_used;

    // if the remaining size is to small and 'wrap' is set we wrap anyway and
    // use the next line
    if (space < PROP_TEXT_MIN_CELLS && wp->w_p_wrap)
	space += wp->w_width;
    if (flags & (TP_FLAG_ALIGN_BELOW | TP_FLAG_ALIGN_ABOVE))
	space -= padding;
    for (n_used = 0; n_used < len; n_used += (*mb_ptr2len)(text + n_used))
    {
	int clen = ptr2cells(text + n_used);

	if (strsize + clen > space)
	    break;
	strsize += clen;
    }
    *n_used_ptr = n_used;
    return strsize;
}

/*
 * Take care of padding, right-align and truncation of virtual text after a
 * line.
 * if "n_attr" is not NULL then "n_extra" and "p_extra" are adjusted for any
 * padding, right-align and truncation.  Otherwise only the size is computed.
 * When "n_attr" is NULL returns the number of screen cells used.
 * Otherwise returns TRUE when drawing continues on the next line.
 */
    int
text_prop_position(
	win_T	    *wp,
	textprop_T  *tp,
	int	    vcol,	    // current text column
	int	    scr_col,	    // current screen column
	int	    *n_extra,	    // nr of bytes for virtual text
	char_u	    **p_extra,	    // virtual text
	int	    *n_attr,	    // attribute cells, NULL if not used
	int	    *n_attr_skip,   // cells to skip attr, NULL if not used
	int	    do_skip)	    // skip_cells is not zero
{
    int	    right = (tp->tp_flags & TP_FLAG_ALIGN_RIGHT);
    int	    above = (tp->tp_flags & TP_FLAG_ALIGN_ABOVE);
    int	    below = (tp->tp_flags & TP_FLAG_ALIGN_BELOW);
    int	    wrap = tp->tp_col < MAXCOL || (tp->tp_flags & TP_FLAG_WRAP);
    int	    padding = tp->tp_col == MAXCOL && tp->tp_len > 1
							  ? tp->tp_len - 1 : 0;
    int	    col_with_padding = scr_col + (below ? 0 : padding);
    int	    room = wp->w_width - col_with_padding;
    int	    before = room;	// spaces before the text
    int	    after = 0;		// spaces after the text
    int	    n_used = *n_extra;
    char_u  *l = NULL;
    int	    strsize = vim_strsize(*p_extra);
    int	    cells = wrap ? strsize : textprop_size_after_trunc(wp,
			     tp->tp_flags, before, padding, *p_extra, &n_used);

    if (wrap || right || above || below || padding > 0 || n_used < *n_extra)
    {
	int	    col_off = win_col_off(wp) - win_col_off2(wp);

	if (above)
	{
	    before = 0;
	    after = wp->w_width - cells - win_col_off(wp) - padding;
	    if (after < 0)
	    {
		// text "above" has too much padding to fit
		padding += after;
		after = 0;
	    }
	}
	else
	{
	    // Right-align: fill with before
	    if (right)
		before -= cells;

	    // Below-align: empty line add one character
	    if (below && vcol == 0 && col_with_padding == col_off
					    && wp->w_width - col_off == before)
		col_with_padding += 1;

	    if (before < 0
		    || !(right || below)
		    || (below ? (col_with_padding <= col_off || !wp->w_p_wrap)
			      : (n_used < *n_extra)))
	    {
		if (right && (wrap
			      || (room < PROP_TEXT_MIN_CELLS && wp->w_p_wrap)))
		{
		    // right-align on next line instead of wrapping if possible
		    before = wp->w_width - col_off - strsize + room;
		    if (before < 0)
			before = 0;
		    else
			n_used = *n_extra;
		}
		else if (below && before > vcol && do_skip)
		    before -= vcol;
		else
		    before = 0;
	    }
	}

	// With 'nowrap' add one to show the "extends" character if needed (it
	// doesn't show if the text just fits).
	if (!wp->w_p_wrap
		&& n_used < *n_extra
		&& wp->w_lcs_chars.ext != NUL
		&& wp->w_p_list)
	    ++n_used;

	// add 1 for NUL, 2 for when '…' is used
	if (n_attr != NULL)
	    l = alloc(n_used + before + after + (padding > 0 ? padding : 0) + 3);
	if (n_attr == NULL || l != NULL)
	{
	    int off = 0;

	    if (n_attr != NULL)
	    {
		vim_memset(l, ' ', before);
		off += before;
		if (padding > 0)
		{
		    vim_memset(l + off, ' ', padding);
		    off += padding;
		}
		vim_strncpy(l + off, *p_extra, n_used);
		off += n_used;
	    }
	    else
	    {
		off = before + after + padding + n_used;
		cells += before + after + padding;
	    }
	    if (n_attr != NULL)
	    {
		if (n_used < *n_extra && wp->w_p_wrap)
		{
		    char_u *lp = l + off - 1;

		    if (has_mbyte)
		    {
			char_u	buf[MB_MAXBYTES + 1];
			char_u	*cp = buf;

			// change the last character to '…', converted to the
			// current 'encoding'
			STRCPY(buf, "…");
			if (!enc_utf8)
			{
			    vimconv_T	vc;

			    vc.vc_type = CONV_NONE;
			    convert_setup(&vc, (char_u *)"utf-8", p_enc);
			    if (vc.vc_type != CONV_NONE)
			    {
				cp = string_convert(&vc, buf, NULL);
				if (cp == NULL)
				{
				    // when conversion fails use '>'
				    cp = buf;
				    STRCPY(buf, ">");
				}
				convert_setup(&vc, NULL, NULL);
			    }
			}

			lp -= (*mb_ptr2cells)(cp) - 1;
			lp -= (*mb_head_off)(l, lp);
			STRCPY(lp, cp);
			n_used = lp - l + 3 - before - padding;
			if (cp != buf)
			    vim_free(cp);
		    }
		    else
			// change last character to '>'
			*lp = '>';
		}
		else if (after > 0)
		{
		    vim_memset(l + off, ' ', after);
		    l[off + after] = NUL;
		}

		*p_extra = l;
		*n_extra = n_used + before + after + padding;
		*n_attr = mb_charlen(*p_extra);
		if (above)
		    *n_attr -= padding + after;

		// n_attr_skip will not be decremented before draw_state is
		// WL_LINE
		*n_attr_skip = before + (padding > 0 ? padding : 0);
	    }
	}
    }

    if (n_attr == NULL)
	return cells;
    return (below && col_with_padding > win_col_off(wp) && !wp->w_p_wrap);
}
#endif

/*
 * Call screen_line() using values from "wlv".
 * Also takes care of putting "<<<" on the first line for 'smoothscroll'
 * when 'showbreak' is not set.
 */
    static void
wlv_screen_line(win_T *wp, winlinevars_T *wlv, int negative_width)
{
    if (wlv->row == 0 && wp->w_skipcol > 0
#if defined(FEAT_LINEBREAK)
	    // do not overwrite the 'showbreak' text with "<<<"
	    && *get_showbreak_value(wp) == NUL
#endif
	    // do not overwrite the 'listchars' "precedes" text with "<<<"
	    && !(wp->w_p_list && wp->w_lcs_chars.prec != 0))
    {
	int off = (int)(current_ScreenLine - ScreenLines);
	int max_off = off + screen_Columns;
	int skip = 0;

	if (wp->w_p_nu && wp->w_p_rnu)
	    // Do not overwrite the line number, change "123 text" to
	    // "123<<<xt".
	    while (skip < wp->w_width && VIM_ISDIGIT(ScreenLines[off]))
	    {
		++off;
		++skip;
	    }

	for (int i = 0; i < 3 && i + skip < wp->w_width; ++i)
	{
	    if ((*mb_off2cells)(off, max_off) > 1)
		// When the first half of a double-width character is
		// overwritten, change the second half to a space.
		ScreenLines[off + 1] = ' ';
	    ScreenLines[off] = '<';
	    if (enc_utf8)
		ScreenLinesUC[off] = 0;
	    ScreenAttrs[off] = HL_ATTR(HLF_AT);
	    ++off;
	}
    }

    screen_line(wp, wlv->screen_row, wp->w_wincol, wlv->col,
		    negative_width ? -wp->w_width : wp->w_width,
		    wlv->screen_line_flags);
}

/*
 * Called when finished with the line: draw the screen line and handle any
 * highlighting until the right of the window.
 */
    static void
draw_screen_line(win_T *wp, winlinevars_T *wlv)
{
#ifdef FEAT_SYN_HL
    long	v;

    // Highlight 'cursorcolumn' & 'colorcolumn' past end of the line.
    if (wp->w_p_wrap)
	v = wlv->startrow == 0 ? wp->w_skipcol : 0;
    else
	v = wp->w_leftcol;

    // check if line ends before left margin
    if (wlv->vcol < v + wlv->col - win_col_off(wp))
	wlv->vcol = v + wlv->col - win_col_off(wp);
# ifdef FEAT_CONCEAL
    // Get rid of the boguscols now, we want to draw until the right
    // edge for 'cursorcolumn'.
    wlv->col -= wlv->boguscols;
    wlv->boguscols = 0;
#  define VCOL_HLC (wlv->vcol - wlv->vcol_off_co - wlv->vcol_off_tp)
# else
#  define VCOL_HLC (wlv->vcol - wlv->vcol_off_tp)
# endif

    if (wlv->draw_color_col)
	wlv->draw_color_col = advance_color_col(VCOL_HLC, &wlv->color_cols);

    if (((wp->w_p_cuc
		    && (int)wp->w_virtcol >= VCOL_HLC - wlv->eol_hl_off
		    && (int)wp->w_virtcol <
			 (long)wp->w_width * (wlv->row - wlv->startrow + 1) + v
			 && wlv->lnum != wp->w_cursor.lnum)
		|| wlv->draw_color_col
# ifdef LINE_ATTR
		|| wlv->line_attr != 0
# endif
		|| wlv->win_attr != 0)
# ifdef FEAT_RIGHTLEFT
	    && !wp->w_p_rl
# endif
	    )
    {
	int	rightmost_vcol = 0;
	int	i;

	if (wp->w_p_cuc)
	    rightmost_vcol = wp->w_virtcol;
	if (wlv->draw_color_col)
	    // determine rightmost colorcolumn to possibly draw
	    for (i = 0; wlv->color_cols[i] >= 0; ++i)
		if (rightmost_vcol < wlv->color_cols[i])
		    rightmost_vcol = wlv->color_cols[i];

	while (wlv->col < wp->w_width)
	{
	    ScreenLines[wlv->off] = ' ';
	    if (enc_utf8)
		ScreenLinesUC[wlv->off] = 0;
	    ScreenCols[wlv->off] = MAXCOL;
	    ++wlv->col;
	    if (wlv->draw_color_col)
		wlv->draw_color_col = advance_color_col(
						   VCOL_HLC, &wlv->color_cols);

	    int attr = wlv->win_attr;
	    if (wp->w_p_cuc && VCOL_HLC == (long)wp->w_virtcol)
		attr = HL_ATTR(HLF_CUC);
	    else if (wlv->draw_color_col && VCOL_HLC == *wlv->color_cols)
		attr = HL_ATTR(HLF_MC);
# ifdef LINE_ATTR
	    else if (wlv->line_attr != 0)
		attr = wlv->line_attr;
# endif
	    ScreenAttrs[wlv->off++] = attr;

	    if (VCOL_HLC >= rightmost_vcol
# ifdef LINE_ATTR
		    && wlv->line_attr == 0
# endif
		    && wlv->win_attr == 0)
		break;

	    ++wlv->vcol;
	}
    }
#endif

    wlv_screen_line(wp, wlv, FALSE);
    ++wlv->row;
    ++wlv->screen_row;
}
#undef VCOL_HLC

/*
 * Start a screen line at column zero.
 * When "save_extra" is TRUE save and reset n_extra, p_extra, etc.
 */
    static void
win_line_start(win_T *wp UNUSED, winlinevars_T *wlv, int save_extra)
{
    wlv->col = 0;
    wlv->off = (unsigned)(current_ScreenLine - ScreenLines);
#ifdef FEAT_LINEBREAK
    wlv->need_lbr = FALSE;
#endif

#ifdef FEAT_RIGHTLEFT
    if (wp->w_p_rl)
    {
	// Rightleft window: process the text in the normal direction, but put
	// it in current_ScreenLine[] from right to left.  Start at the
	// rightmost column of the window.
	wlv->col = wp->w_width - 1;
	wlv->off += wlv->col;
	wlv->screen_line_flags |= SLF_RIGHTLEFT;
    }
#endif
    if (save_extra)
    {
	// reset the drawing state for the start of a wrapped line
	wlv->draw_state = WL_START;
	wlv->saved_n_extra = wlv->n_extra;
	wlv->saved_p_extra = wlv->p_extra;
	vim_free(wlv->saved_p_extra_free);
	wlv->saved_p_extra_free = wlv->p_extra_free;
	wlv->p_extra_free = NULL;
	wlv->saved_extra_attr = wlv->extra_attr;
	wlv->saved_n_attr_skip = wlv->n_attr_skip;
	wlv->saved_extra_for_textprop = wlv->extra_for_textprop;
	wlv->saved_c_extra = wlv->c_extra;
	wlv->saved_c_final = wlv->c_final;
#ifdef FEAT_LINEBREAK
	wlv->need_lbr = TRUE;
#endif
#ifdef FEAT_SYN_HL
	if (!(wlv->cul_screenline
# ifdef FEAT_DIFF
		    && wlv->diff_hlf == (hlf_T)0
# endif
	     ))
	    wlv->saved_char_attr = wlv->char_attr;
	else
#endif
	    wlv->saved_char_attr = 0;

	// these are not used until restored in win_line_continue()
	wlv->n_extra = 0;
	wlv->n_attr_skip = 0;
    }
}

/*
 * Called when wlv->draw_state is set to WL_LINE.
 */
    static void
win_line_continue(winlinevars_T *wlv)
{
    if (wlv->saved_n_extra > 0)
    {
	// Continue item from end of wrapped line.
	wlv->n_extra = wlv->saved_n_extra;
	wlv->saved_n_extra = 0;
	wlv->c_extra = wlv->saved_c_extra;
	wlv->c_final = wlv->saved_c_final;
	wlv->p_extra = wlv->saved_p_extra;
	vim_free(wlv->p_extra_free);
	wlv->p_extra_free = wlv->saved_p_extra_free;
	wlv->saved_p_extra_free = NULL;
	wlv->extra_attr = wlv->saved_extra_attr;
	wlv->n_attr_skip = wlv->saved_n_attr_skip;
	wlv->extra_for_textprop = wlv->saved_extra_for_textprop;
	wlv->char_attr = wlv->saved_char_attr;
    }
    else
	wlv->char_attr = wlv->win_attr;
}

#ifdef FEAT_SYN_HL
    static void
apply_cursorline_highlight(
	winlinevars_T *wlv,
	int sign_present UNUSED)
{
    wlv->cul_attr = HL_ATTR(HLF_CUL);
# ifdef FEAT_SIGNS
    // Combine the 'cursorline' and sign highlighting, depending on
    // the sign priority.
    if (sign_present && wlv->sattr.sat_linehl > 0)
    {
	if (wlv->sattr.sat_priority >= 100)
	    wlv->line_attr = hl_combine_attr(wlv->cul_attr, wlv->line_attr);
	else
	    wlv->line_attr = hl_combine_attr(wlv->line_attr, wlv->cul_attr);
    }
    else
# endif
# if defined(FEAT_QUICKFIX)
	// let the line attribute overrule 'cursorline', otherwise
	// it disappears when both have background set;
	// 'cursorline' can use underline or bold to make it show
	wlv->line_attr = hl_combine_attr(wlv->cul_attr, wlv->line_attr);
# else
	wlv->line_attr = wlv->cul_attr;
# endif
}
#endif

/*
 * Display line "lnum" of window "wp" on the screen.
 * Start at row "startrow", stop when "endrow" is reached.
 * When "number_only" is TRUE only update the number column.
 * "spv" is used to store information for spell checking, kept between
 * sequential calls for the same window.
 * wp->w_virtcol needs to be valid.
 *
 * Return the number of last row the line occupies.
 */
    int
win_line(
    win_T	*wp,
    linenr_T	lnum,
    int		startrow,
    int		endrow,
    int		number_only,
    spellvars_T	*spv UNUSED)
{
    winlinevars_T	wlv;		// variables passed between functions

    int		c = 0;			// init for GCC
    long	vcol_prev = -1;		// "wlv.vcol" of previous character
    char_u	*line;			// current line
    char_u	*ptr;			// current position in "line"

#ifdef FEAT_PROP_POPUP
    char_u	*p_extra_free2 = NULL;   // another p_extra to be freed
#endif
#if defined(FEAT_LINEBREAK) && defined(FEAT_PROP_POPUP)
    int		in_linebreak = FALSE;	// n_extra set for showing linebreak
#endif
    static char_u *at_end_str = (char_u *)""; // used for p_extra when
					// displaying eol at end-of-line
    int		lcs_eol_one = wp->w_lcs_chars.eol; // eol until it's been used
    int		lcs_prec_todo = wp->w_lcs_chars.prec;
					// prec until it's been used

    int		n_attr = 0;	    // chars with special attr
    int		saved_attr2 = 0;    // char_attr saved for n_attr
    int		n_attr3 = 0;	    // chars with overruling special attr
    int		saved_attr3 = 0;    // char_attr saved for n_attr3

    int		skip_cells = 0;		// nr of cells to skip for w_leftcol or
					// w_skipcol or concealing
    int		skipped_cells = 0;	// nr of skipped cells for virtual text
					// to be added to wlv.vcol later
    int		fromcol_prev = -2;	// start of inverting after cursor
    int		noinvcur = FALSE;	// don't invert the cursor
    int		lnum_in_visual_area = FALSE;
    pos_T	pos;
    long	v;

    int		attr_pri = FALSE;	// char_attr has priority
    int		area_highlighting = FALSE; // Visual or incsearch highlighting
					   // in this line
    int		vi_attr = 0;		// attributes for Visual and incsearch
					// highlighting
    int		area_attr = 0;		// attributes desired by highlighting
    int		search_attr = 0;	// attributes desired by 'hlsearch'
#ifdef FEAT_SYN_HL
    int		vcol_save_attr = 0;	// saved attr for 'cursorcolumn'
    int		syntax_attr = 0;	// attributes desired by syntax
    int		prev_syntax_col = -1;	// column of prev_syntax_attr
    int		prev_syntax_attr = 0;	// syntax_attr at prev_syntax_col
    int		has_syntax = FALSE;	// this buffer has syntax highl.
    int		save_did_emsg;
#endif
#ifdef FEAT_PROP_POPUP
    int		did_line = FALSE;	// set to TRUE when line text done
    int		text_prop_count;
    int		last_textprop_text_idx = -1;
    int		text_prop_next = 0;	// next text property to use
    textprop_T	*text_props = NULL;
    int		*text_prop_idxs = NULL;
    int		text_props_active = 0;
    proptype_T  *text_prop_type = NULL;
    int		text_prop_attr = 0;
    int		text_prop_attr_comb = 0;  // text_prop_attr combined with
					  // syntax_attr
    int		text_prop_id = 0;	// active property ID
    int		text_prop_flags = 0;
    int		text_prop_above = FALSE;  // first doing virtual text above
    int		text_prop_follows = FALSE;  // another text prop to display
    int		saved_search_attr = 0;	// search_attr to be used when n_extra
					// goes to zero
    int		saved_area_attr = 0;	// idem for area_attr
    int		reset_extra_attr = FALSE;
#endif
#ifdef FEAT_SPELL
    int		can_spell = FALSE;
# define SPWORDLEN 150
    char_u	nextline[SPWORDLEN * 2];// text with start of the next line
    int		nextlinecol = 0;	// column where nextline[] starts
    int		nextline_idx = 0;	// index in nextline[] where next line
					// starts
    int		spell_attr = 0;		// attributes desired by spelling
    int		word_end = 0;		// last byte with same spell_attr
    int		cur_checked_col = 0;	// checked column for current line
#endif
    int		extra_check = 0;	// has extra highlighting
    int		multi_attr = 0;		// attributes desired by multibyte
    int		mb_l = 1;		// multi-byte byte length
    int		mb_c = 0;		// decoded multi-byte character
    int		mb_utf8 = FALSE;	// screen char is UTF-8 char
    int		u8cc[MAX_MCO];		// composing UTF-8 chars
#ifdef FEAT_DIFF
    int		change_start = MAXCOL;	// first col of changed area
    int		change_end = -1;	// last col of changed area
#endif
    colnr_T	trailcol = MAXCOL;	// start of trailing spaces
    colnr_T	leadcol = 0;		// start of leading spaces
    int		in_multispace = FALSE;	// in multiple consecutive spaces
    int		multispace_pos = 0;	// position in lcs-multispace string
#ifdef LINE_ATTR
    int		line_attr_save = 0;
#endif
    int		sign_present = FALSE;
    int		num_attr = 0;		// attribute for the number column
#ifdef FEAT_ARABIC
    int		prev_c = 0;		// previous Arabic character
    int		prev_c1 = 0;		// first composing char for prev_c
#endif
#if defined(LINE_ATTR)
    int		did_line_attr = 0;
#endif
#ifdef FEAT_TERMINAL
    int		get_term_attr = FALSE;
#endif

#if defined(FEAT_SYN_HL) || defined(FEAT_DIFF)
    // margin columns for the screen line, needed for when 'cursorlineopt'
    // contains "screenline"
    int		left_curline_col = 0;
    int		right_curline_col = 0;
#endif

#if defined(FEAT_XIM) && defined(FEAT_GUI_GTK)
    int		feedback_col = 0;
    int		feedback_old_attr = -1;
#endif

#if defined(FEAT_CONCEAL) || defined(FEAT_SEARCH_EXTRA)
    int		match_conc	= 0;	// cchar for match functions
#endif
#if defined(FEAT_CONCEAL) || defined(FEAT_SEARCH_EXTRA) || defined(FEAT_LINEBREAK)
    int		on_last_col     = FALSE;
#endif
#ifdef FEAT_CONCEAL
    int		syntax_flags	= 0;
    int		syntax_seqnr	= 0;
    int		prev_syntax_id	= 0;
    int		conceal_attr	= HL_ATTR(HLF_CONCEAL);
    int		is_concealing	= FALSE;
    int		did_wcol	= FALSE;
    int		old_boguscols   = 0;
# define VCOL_HLC (wlv.vcol - wlv.vcol_off_co - wlv.vcol_off_tp)
# define FIX_FOR_BOGUSCOLS \
    { \
	wlv.n_extra += wlv.vcol_off_co; \
	wlv.vcol -= wlv.vcol_off_co; \
	wlv.vcol_off_co = 0; \
	wlv.col -= wlv.boguscols; \
	old_boguscols = wlv.boguscols; \
	wlv.boguscols = 0; \
    }
#else
# define VCOL_HLC (wlv.vcol - wlv.vcol_off_tp)
#endif

    if (startrow > endrow)		// past the end already!
	return startrow;

    CLEAR_FIELD(wlv);

    wlv.lnum = lnum;
    wlv.startrow = startrow;
    wlv.row = startrow;
    wlv.screen_row = wlv.row + W_WINROW(wp);
    wlv.fromcol = -10;
    wlv.tocol = MAXCOL;
#ifdef FEAT_LINEBREAK
    wlv.vcol_sbr = -1;
#endif

    if (!number_only)
    {
	// To speed up the loop below, set extra_check when there is linebreak,
	// trailing white space and/or syntax processing to be done.
#ifdef FEAT_LINEBREAK
	extra_check = wp->w_p_lbr;
#endif
#ifdef FEAT_SYN_HL
	if (syntax_present(wp) && !wp->w_s->b_syn_error
# ifdef SYN_TIME_LIMIT
		&& !wp->w_s->b_syn_slow
# endif
	   )
	{
	    // Prepare for syntax highlighting in this line.  When there is an
	    // error, stop syntax highlighting.
	    save_did_emsg = did_emsg;
	    did_emsg = FALSE;
	    syntax_start(wp, lnum);
	    if (did_emsg)
		wp->w_s->b_syn_error = TRUE;
	    else
	    {
		did_emsg = save_did_emsg;
#ifdef SYN_TIME_LIMIT
		if (!wp->w_s->b_syn_slow)
#endif
		{
		    has_syntax = TRUE;
		    extra_check = TRUE;
		}
	    }
	}

	// Check for columns to display for 'colorcolumn'.
	wlv.color_cols = wp->w_p_cc_cols;
	if (wlv.color_cols != NULL)
	    wlv.draw_color_col = advance_color_col(VCOL_HLC, &wlv.color_cols);
#endif

#ifdef FEAT_TERMINAL
	if (term_show_buffer(wp->w_buffer))
	{
	    extra_check = TRUE;
	    get_term_attr = TRUE;
	    wlv.win_attr = term_get_attr(wp, lnum, -1);
	}
#endif

	// handle Visual active in this window
	if (VIsual_active && wp->w_buffer == curwin->w_buffer)
	{
	    pos_T	*top, *bot;

	    if (LTOREQ_POS(curwin->w_cursor, VIsual))
	    {
		// Visual is after curwin->w_cursor
		top = &curwin->w_cursor;
		bot = &VIsual;
	    }
	    else
	    {
		// Visual is before curwin->w_cursor
		top = &VIsual;
		bot = &curwin->w_cursor;
	    }
	    lnum_in_visual_area = (lnum >= top->lnum && lnum <= bot->lnum);
	    if (VIsual_mode == Ctrl_V)
	    {
		// block mode
		if (lnum_in_visual_area)
		{
		    wlv.fromcol = wp->w_old_cursor_fcol;
		    wlv.tocol = wp->w_old_cursor_lcol;
		}
	    }
	    else
	    {
		// non-block mode
		if (lnum > top->lnum && lnum <= bot->lnum)
		    wlv.fromcol = 0;
		else if (lnum == top->lnum)
		{
		    if (VIsual_mode == 'V')	// linewise
			wlv.fromcol = 0;
		    else
		    {
			getvvcol(wp, top, (colnr_T *)&wlv.fromcol, NULL, NULL);
			if (gchar_pos(top) == NUL)
			    wlv.tocol = wlv.fromcol + 1;
		    }
		}
		if (VIsual_mode != 'V' && lnum == bot->lnum)
		{
		    if (*p_sel == 'e' && bot->col == 0 && bot->coladd == 0)
		    {
			wlv.fromcol = -10;
			wlv.tocol = MAXCOL;
		    }
		    else if (bot->col == MAXCOL)
			wlv.tocol = MAXCOL;
		    else
		    {
			pos = *bot;
			if (*p_sel == 'e')
			    getvvcol(wp, &pos, (colnr_T *)&wlv.tocol,
								   NULL, NULL);
			else
			{
			    getvvcol(wp, &pos, NULL, NULL,
							(colnr_T *)&wlv.tocol);
			    ++wlv.tocol;
			}
		    }
		}
	    }

	    // Check if the character under the cursor should not be inverted
	    if (!highlight_match && lnum == curwin->w_cursor.lnum
								&& wp == curwin
#ifdef FEAT_GUI
		    && !gui.in_use
#endif
		    )
		noinvcur = TRUE;

	    // if inverting in this line set area_highlighting
	    if (wlv.fromcol >= 0)
	    {
		area_highlighting = TRUE;
		vi_attr = HL_ATTR(HLF_V);
#if defined(FEAT_CLIPBOARD) && defined(FEAT_X11)
		if ((clip_star.available && !clip_star.owned
						      && clip_isautosel_star())
			|| (clip_plus.available && !clip_plus.owned
						     && clip_isautosel_plus()))
		    vi_attr = HL_ATTR(HLF_VNC);
#endif
	    }
	}

	// handle 'incsearch' and ":s///c" highlighting
	else if (highlight_match
		&& wp == curwin
		&& lnum >= curwin->w_cursor.lnum
		&& lnum <= curwin->w_cursor.lnum + search_match_lines)
	{
	    if (lnum == curwin->w_cursor.lnum)
		getvcol(curwin, &(curwin->w_cursor),
					  (colnr_T *)&wlv.fromcol, NULL, NULL);
	    else
		wlv.fromcol = 0;
	    if (lnum == curwin->w_cursor.lnum + search_match_lines)
	    {
		pos.lnum = lnum;
		pos.col = search_match_endcol;
		getvcol(curwin, &pos, (colnr_T *)&wlv.tocol, NULL, NULL);
	    }
	    else
		wlv.tocol = MAXCOL;
	    // do at least one character; happens when past end of line
	    if (wlv.fromcol == wlv.tocol && search_match_endcol)
		wlv.tocol = wlv.fromcol + 1;
	    area_highlighting = TRUE;
	    vi_attr = HL_ATTR(HLF_I);
	}
    }

#ifdef FEAT_DIFF
    wlv.filler_lines = diff_check(wp, lnum);
    if (wlv.filler_lines < 0)
    {
	if (wlv.filler_lines == -1)
	{
	    if (diff_find_change(wp, lnum, &change_start, &change_end))
		wlv.diff_hlf = HLF_ADD;	// added line
	    else if (change_start == 0)
		wlv.diff_hlf = HLF_TXD;	// changed text
	    else
		wlv.diff_hlf = HLF_CHD;	// changed line
	}
	else
	    wlv.diff_hlf = HLF_ADD;		// added line
	wlv.filler_lines = 0;
	area_highlighting = TRUE;
    }
    if (lnum == wp->w_topline)
	wlv.filler_lines = wp->w_topfill;
    wlv.filler_todo = wlv.filler_lines;
#endif

#ifdef FEAT_SIGNS
    sign_present = buf_get_signattrs(wp, lnum, &wlv.sattr);
    if (sign_present)
	num_attr = wlv.sattr.sat_numhl;
#endif

#ifdef LINE_ATTR
# ifdef FEAT_SIGNS
    // If this line has a sign with line highlighting set wlv.line_attr.
    if (sign_present)
	wlv.line_attr = wlv.sattr.sat_linehl;
# endif
# if defined(FEAT_QUICKFIX)
    // Highlight the current line in the quickfix window.
    if (bt_quickfix(wp->w_buffer) && qf_current_entry(wp) == lnum)
	wlv.line_attr = HL_ATTR(HLF_QFL);
# endif
    if (wlv.line_attr != 0)
	area_highlighting = TRUE;
#endif

#ifdef FEAT_SPELL
    if (spv->spv_has_spell && !number_only)
    {
	// Prepare for spell checking.
	extra_check = TRUE;

	// When a word wrapped from the previous line the start of the
	// current line is valid.
	if (lnum == spv->spv_checked_lnum)
	    cur_checked_col = spv->spv_checked_col;
	// Previous line was not spell checked, check for capital. This happens
	// for the first line in an updated region or after a closed fold.
	if (spv->spv_capcol_lnum == 0 && check_need_cap(wp, lnum, 0))
	    spv->spv_cap_col = 0;
	else if (lnum != spv->spv_capcol_lnum)
	    spv->spv_cap_col = -1;
	spv->spv_checked_lnum = 0;

	// Get the start of the next line, so that words that wrap to the
	// next line are found too: "et<line-break>al.".
	// Trick: skip a few chars for C/shell/Vim comments
	nextline[SPWORDLEN] = NUL;
	if (lnum < wp->w_buffer->b_ml.ml_line_count)
	{
	    line = ml_get_buf(wp->w_buffer, lnum + 1, FALSE);
	    spell_cat_line(nextline + SPWORDLEN, line, SPWORDLEN);
	}
	line = ml_get_buf(wp->w_buffer, lnum, FALSE);

	// If current line is empty, check first word in next line for capital.
	ptr = skipwhite(line);
	if (*ptr == NUL)
	{
	    spv->spv_cap_col = 0;
	    spv->spv_capcol_lnum = lnum + 1;
	}
	// For checking first word with a capital skip white space.
	else if (spv->spv_cap_col == 0)
	    spv->spv_cap_col = ptr - line;

	// Copy the end of the current line into nextline[].
	if (nextline[SPWORDLEN] == NUL)
	{
	    // No next line or it is empty.
	    nextlinecol = MAXCOL;
	    nextline_idx = 0;
	}
	else
	{
	    v = (long)STRLEN(line);
	    if (v < SPWORDLEN)
	    {
		// Short line, use it completely and append the start of the
		// next line.
		nextlinecol = 0;
		mch_memmove(nextline, line, (size_t)v);
		STRMOVE(nextline + v, nextline + SPWORDLEN);
		nextline_idx = v + 1;
	    }
	    else
	    {
		// Long line, use only the last SPWORDLEN bytes.
		nextlinecol = v - SPWORDLEN;
		mch_memmove(nextline, line + nextlinecol, SPWORDLEN);
		nextline_idx = SPWORDLEN + 1;
	    }
	}
    }
#endif

    line = ml_get_buf(wp->w_buffer, lnum, FALSE);
    ptr = line;

    if (wp->w_p_list)
    {
	if (wp->w_lcs_chars.space
		|| wp->w_lcs_chars.multispace != NULL
		|| wp->w_lcs_chars.leadmultispace != NULL
		|| wp->w_lcs_chars.trail
		|| wp->w_lcs_chars.lead
		|| wp->w_lcs_chars.nbsp)
	    extra_check = TRUE;

	// find start of trailing whitespace
	if (wp->w_lcs_chars.trail)
	{
	    trailcol = (colnr_T)STRLEN(ptr);
	    while (trailcol > (colnr_T)0 && VIM_ISWHITE(ptr[trailcol - 1]))
		--trailcol;
	    trailcol += (colnr_T)(ptr - line);
	}
	// find end of leading whitespace
	if (wp->w_lcs_chars.lead || wp->w_lcs_chars.leadmultispace != NULL)
	{
	    leadcol = 0;
	    while (VIM_ISWHITE(ptr[leadcol]))
		++leadcol;
	    if (ptr[leadcol] == NUL)
		// in a line full of spaces all of them are treated as trailing
		leadcol = (colnr_T)0;
	    else
		// keep track of the first column not filled with spaces
		leadcol += (colnr_T)(ptr - line) + 1;
	}
    }

    wlv.wcr_attr = get_wcr_attr(wp);
    if (wlv.wcr_attr != 0)
    {
	wlv.win_attr = wlv.wcr_attr;
	area_highlighting = TRUE;
    }

    // When w_skipcol is non-zero and there is virtual text above the actual
    // text, then this much of the virtual text is skipped.
    int skipcol_in_text_prop_above = 0;

#ifdef FEAT_PROP_POPUP
    if (WIN_IS_POPUP(wp))
	wlv.screen_line_flags |= SLF_POPUP;

    char_u *prop_start;
    text_prop_count = get_text_props(wp->w_buffer, lnum, &prop_start, FALSE);
    if (text_prop_count > 0)
    {
	// Make a copy of the properties, so that they are properly
	// aligned.
	text_props = ALLOC_MULT(textprop_T, text_prop_count);
	if (text_props != NULL)
	    mch_memmove(text_props, prop_start,
				     text_prop_count * sizeof(textprop_T));

	// Allocate an array for the indexes.
	text_prop_idxs = ALLOC_MULT(int, text_prop_count);
	if (text_prop_idxs == NULL)
	    VIM_CLEAR(text_props);

	if (text_props != NULL)
	{
	    area_highlighting = TRUE;
	    extra_check = TRUE;

	    /* Find the last text property that inserts text. */
	    for (int i = 0; i < text_prop_count; ++i)
		if (text_props[i].tp_id < 0)
		    last_textprop_text_idx = i;

	    // When skipping virtual text the props need to be sorted.  The
	    // order is reversed!
	    if (lnum == wp->w_topline && wp->w_skipcol > 0)
	    {
		for (int i = 0; i < text_prop_count; ++i)
		    text_prop_idxs[i] = i;
		sort_text_props(wp->w_buffer, text_props,
					      text_prop_idxs, text_prop_count);
	    }

	    // Text props "above" move the line number down to where the text
	    // is.  Only count the ones that are visible, not those that are
	    // skipped because of w_skipcol.
	    int text_width = wp->w_width - win_col_off(wp);
	    for (int i = text_prop_count - 1; i >= 0; --i)
		if (text_props[i].tp_flags & TP_FLAG_ALIGN_ABOVE)
		{
		    if (lnum == wp->w_topline
			    && wp->w_skipcol - skipcol_in_text_prop_above
								 >= text_width)
		    {
			// This virtual text above is skipped, remove it from
			// the array.
			skipcol_in_text_prop_above += text_width;
			for (int j = i + 1; j < text_prop_count; ++j)
			    text_props[j - 1] = text_props[j];
			++i;
			--text_prop_count;
		    }
		    else
			++wlv.text_prop_above_count;
		}
	}
    }

    if (number_only)
    {
	// skip over rows only used for virtual text above
	wlv.row += wlv.text_prop_above_count;
	if (wlv.row > endrow)
	    return wlv.row;
	wlv.screen_row += wlv.text_prop_above_count;
    }
#endif

#if defined(FEAT_LINEBREAK) || defined(FEAT_PROP_POPUP)
    colnr_T vcol_first_char = 0;
    if (wp->w_p_lbr && !number_only)
    {
	chartabsize_T cts;
	init_chartabsize_arg(&cts, wp, lnum, 0, line, line);
	(void)win_lbr_chartabsize(&cts, NULL);
	vcol_first_char = cts.cts_first_char;
	clear_chartabsize_arg(&cts);
    }
#endif

    // 'nowrap' or 'wrap' and a single line that doesn't fit: Advance to the
    // first character to be displayed.
    if (wp->w_p_wrap)
	v = startrow == 0 ? wp->w_skipcol - skipcol_in_text_prop_above : 0;
    else
	v = wp->w_leftcol;
    if (v > 0 && !number_only)
    {
	char_u		*prev_ptr = ptr;
	chartabsize_T	cts;
	int		charsize = 0;
	int		head = 0;

	init_chartabsize_arg(&cts, wp, lnum, wlv.vcol, line, ptr);
	cts.cts_max_head_vcol = v;
	while (cts.cts_vcol < v && *cts.cts_ptr != NUL)
	{
	    head = 0;
	    charsize = win_lbr_chartabsize(&cts, &head);
	    cts.cts_vcol += charsize;
	    prev_ptr = cts.cts_ptr;
	    MB_PTR_ADV(cts.cts_ptr);
	    if (wp->w_p_list)
	    {
		in_multispace = *prev_ptr == ' ' && (*cts.cts_ptr == ' '
				  || (prev_ptr > line && prev_ptr[-1] == ' '));
		if (!in_multispace)
		    multispace_pos = 0;
		else if (cts.cts_ptr >= line + leadcol
					 && wp->w_lcs_chars.multispace != NULL)
		{
		    ++multispace_pos;
		    if (wp->w_lcs_chars.multispace[multispace_pos] == NUL)
			multispace_pos = 0;
		}
		else if (cts.cts_ptr < line + leadcol
				     && wp->w_lcs_chars.leadmultispace != NULL)
		{
		    ++multispace_pos;
		    if (wp->w_lcs_chars.leadmultispace[multispace_pos] == NUL)
			multispace_pos = 0;
		}
	    }
	}
	wlv.vcol = cts.cts_vcol;
	ptr = cts.cts_ptr;
	clear_chartabsize_arg(&cts);

	// When:
	// - 'cuc' is set, or
	// - 'colorcolumn' is set, or
	// - 'virtualedit' is set, or
	// - the visual mode is active,
	// the end of the line may be before the start of the displayed part.
	if (wlv.vcol < v && (
#ifdef FEAT_SYN_HL
	     wp->w_p_cuc || wlv.draw_color_col ||
#endif
	     virtual_active() ||
	     (VIsual_active && wp->w_buffer == curwin->w_buffer)))
	    wlv.vcol = v;

	// Handle a character that's not completely on the screen: Put ptr at
	// that character but skip the first few screen characters.
	if (wlv.vcol > v)
	{
	    wlv.vcol -= charsize;
	    ptr = prev_ptr;
	}
	if (v > wlv.vcol)
	    skip_cells = v - wlv.vcol - head;

	// Adjust for when the inverted text is before the screen,
	// and when the start of the inverted text is before the screen.
	if (wlv.tocol <= wlv.vcol)
	    wlv.fromcol = 0;
	else if (wlv.fromcol >= 0 && wlv.fromcol < wlv.vcol)
	    wlv.fromcol = wlv.vcol;

#ifdef FEAT_LINEBREAK
	// When w_skipcol is non-zero, first line needs 'showbreak'
	if (wp->w_p_wrap)
	    wlv.need_showbreak = TRUE;
#endif
#ifdef FEAT_SPELL
	// When spell checking a word we need to figure out the start of the
	// word and if it's badly spelled or not.
	if (spv->spv_has_spell)
	{
	    int		len;
	    colnr_T	linecol = (colnr_T)(ptr - line);
	    hlf_T	spell_hlf = HLF_COUNT;

	    pos = wp->w_cursor;
	    wp->w_cursor.lnum = lnum;
	    wp->w_cursor.col = linecol;
	    len = spell_move_to(wp, FORWARD, TRUE, TRUE, &spell_hlf);

	    // spell_move_to() may call ml_get() and make "line" invalid
	    line = ml_get_buf(wp->w_buffer, lnum, FALSE);
	    ptr = line + linecol;

	    if (len == 0 || (int)wp->w_cursor.col > ptr - line)
	    {
		// no bad word found at line start, don't check until end of a
		// word
		spell_hlf = HLF_COUNT;
		word_end = (int)(spell_to_word_end(ptr, wp) - line + 1);
	    }
	    else
	    {
		// bad word found, use attributes until end of word
		word_end = wp->w_cursor.col + len + 1;

		// Turn index into actual attributes.
		if (spell_hlf != HLF_COUNT)
		    spell_attr = highlight_attr[spell_hlf];
	    }
	    wp->w_cursor = pos;

# ifdef FEAT_SYN_HL
	    // Need to restart syntax highlighting for this line.
	    if (has_syntax)
		syntax_start(wp, lnum);
# endif
	}
#endif
    }

    // Correct highlighting for cursor that can't be disabled.
    // Avoids having to check this for each character.
    if (wlv.fromcol >= 0)
    {
	if (noinvcur)
	{
	    if ((colnr_T)wlv.fromcol == wp->w_virtcol)
	    {
		// highlighting starts at cursor, let it start just after the
		// cursor
		fromcol_prev = wlv.fromcol;
		wlv.fromcol = -1;
	    }
	    else if ((colnr_T)wlv.fromcol < wp->w_virtcol)
		// restart highlighting after the cursor
		fromcol_prev = wp->w_virtcol;
	}
	if (wlv.fromcol >= wlv.tocol)
	    wlv.fromcol = -1;
    }

#ifdef FEAT_SEARCH_EXTRA
    if (!number_only)
    {
	v = (long)(ptr - line);
	area_highlighting |= prepare_search_hl_line(wp, lnum, (colnr_T)v,
					      &line, &screen_search_hl,
					      &search_attr);
	ptr = line + v; // "line" may have been updated
    }
#endif

#ifdef FEAT_SYN_HL
    // Cursor line highlighting for 'cursorline' in the current window.
    if (wp->w_p_cul && lnum == wp->w_cursor.lnum)
    {
	// Do not show the cursor line in the text when Visual mode is active,
	// because it's not clear what is selected then.
	if (!(wp == curwin && VIsual_active)
					 && wp->w_p_culopt_flags != CULOPT_NBR)
	{
	    wlv.cul_screenline = (wp->w_p_wrap
				   && (wp->w_p_culopt_flags & CULOPT_SCRLINE));

	    // Only apply CursorLine highlight here when "screenline" is not
	    // present in 'cursorlineopt'.  Otherwise it's done later.
	    if (!wlv.cul_screenline)
		apply_cursorline_highlight(&wlv, sign_present);
	    else
	    {
		line_attr_save = wlv.line_attr;
		margin_columns_win(wp, &left_curline_col, &right_curline_col);
	    }
	    area_highlighting = TRUE;
	}
    }
#endif

    win_line_start(wp, &wlv, FALSE);

    // Repeat for the whole displayed line.
    for (;;)
    {
#if defined(FEAT_CONCEAL) || defined(FEAT_SEARCH_EXTRA)
	int	has_match_conc = 0;	// match wants to conceal
#endif
#ifdef FEAT_CONCEAL
	int	did_decrement_ptr = FALSE;
#endif

	// Skip this quickly when working on the text.
	if (wlv.draw_state != WL_LINE)
	{
#ifdef FEAT_SYN_HL
	    if (wlv.cul_screenline)
	    {
		wlv.cul_attr = 0;
		wlv.line_attr = line_attr_save;
	    }
#endif
	    if (wlv.draw_state == WL_CMDLINE - 1 && wlv.n_extra == 0)
	    {
		wlv.draw_state = WL_CMDLINE;
		if (wp == cmdwin_win)
		{
		    // Draw the cmdline character.
		    wlv.n_extra = 1;
		    wlv.c_extra = cmdwin_type;
		    wlv.c_final = NUL;
		    wlv.char_attr =
				hl_combine_attr(wlv.wcr_attr, HL_ATTR(HLF_AT));
		}
	    }
#ifdef FEAT_FOLDING
	    if (wlv.draw_state == WL_FOLD - 1 && wlv.n_extra == 0)
	    {
		wlv.draw_state = WL_FOLD;
		handle_foldcolumn(wp, &wlv);
	    }
#endif
#ifdef FEAT_SIGNS
	    if (wlv.draw_state == WL_SIGN - 1 && wlv.n_extra == 0)
	    {
		// Show the sign column when desired or when using Netbeans.
		wlv.draw_state = WL_SIGN;
		if (signcolumn_on(wp))
		    get_sign_display_info(FALSE, wp, &wlv);
	    }
#endif
	    if (wlv.draw_state == WL_NR - 1 && wlv.n_extra == 0)
	    {
		// Show the line number, if desired.
		wlv.draw_state = WL_NR;
		handle_lnum_col(wp, &wlv, sign_present, num_attr);
	    }
#ifdef FEAT_LINEBREAK
	    // Check if 'breakindent' applies and show it.
	    // May change wlv.draw_state to WL_BRI or WL_BRI - 1.
	    if (wlv.n_extra == 0)
		handle_breakindent(wp, &wlv);
#endif
#if defined(FEAT_LINEBREAK) || defined(FEAT_DIFF)
	    if (wlv.draw_state == WL_SBR - 1 && wlv.n_extra == 0)
	    {
		wlv.draw_state = WL_SBR;
		handle_showbreak_and_filler(wp, &wlv);
	    }
#endif
	    if (wlv.draw_state == WL_LINE - 1 && wlv.n_extra == 0)
	    {
		wlv.draw_state = WL_LINE;
		win_line_continue(&wlv);  // use wlv.saved_ values
	    }
	}

#ifdef FEAT_SYN_HL
	if (wlv.cul_screenline && wlv.draw_state == WL_LINE
		&& wlv.vcol >= left_curline_col
		&& wlv.vcol < right_curline_col)
	{
	    apply_cursorline_highlight(&wlv, sign_present);
	}
#endif

	// When still displaying '$' of change command, stop at cursor.
	// When only displaying the (relative) line number and that's done,
	// stop here.
	if (((dollar_vcol >= 0 && wp == curwin
			       && lnum == wp->w_cursor.lnum
			       && wlv.vcol >= (long)wp->w_virtcol)
		|| (number_only && wlv.draw_state > WL_NR))
#ifdef FEAT_DIFF
				   && wlv.filler_todo <= 0
#endif
		)
	{
	    wlv_screen_line(wp, &wlv, TRUE);
	    // Pretend we have finished updating the window.  Except when
	    // 'cursorcolumn' is set.
#ifdef FEAT_SYN_HL
	    if (wp->w_p_cuc)
		wlv.row = wp->w_cline_row + wp->w_cline_height;
	    else
#endif
		wlv.row = wp->w_height;
	    break;
	}

	if (wlv.draw_state == WL_LINE && (area_highlighting || extra_check))
	{
#ifdef FEAT_PROP_POPUP
	    if (text_props != NULL)
	    {
		int pi;
		int bcol = (int)(ptr - line);

		if (wlv.n_extra > 0
# ifdef FEAT_LINEBREAK
			&& !in_linebreak
# endif
			)
		    --bcol;  // still working on the previous char, e.g. Tab

		// Check if any active property ends.
		for (pi = 0; pi < text_props_active; ++pi)
		{
		    int		tpi = text_prop_idxs[pi];
		    textprop_T  *tp = &text_props[tpi];

		    // An inline property ends when after the start column plus
		    // length. An "above" property ends when used and n_extra
		    // is zero.
		    if ((tp->tp_col != MAXCOL
				       && bcol >= tp->tp_col - 1 + tp->tp_len))
		    {
			if (pi + 1 < text_props_active)
			    mch_memmove(text_prop_idxs + pi,
					text_prop_idxs + pi + 1,
					sizeof(int)
					     * (text_props_active - (pi + 1)));
			--text_props_active;
			--pi;
# ifdef FEAT_LINEBREAK
			// not exactly right but should work in most cases
			if (in_linebreak && syntax_attr == text_prop_attr_comb)
			    syntax_attr = 0;
# endif
		    }
		}

# ifdef FEAT_LINEBREAK
		if (wlv.n_extra > 0 && in_linebreak)
		    // not on the next char yet, don't start another prop
		    --bcol;
# endif
		int display_text_first = FALSE;

		// Add any text property that starts in this column.
		// With 'nowrap' and not in the first screen line only "below"
		// text prop can show.
		while (text_prop_next < text_prop_count
			   && (text_props[text_prop_next].tp_col == MAXCOL
			      ? ((*ptr == NUL
				  && (wp->w_p_wrap
				      || wlv.row == startrow
				      || (text_props[text_prop_next].tp_flags
						       & TP_FLAG_ALIGN_BELOW)))
			       || (bcol == 0
					&& (text_props[text_prop_next].tp_flags
						       & TP_FLAG_ALIGN_ABOVE)))
			      : bcol >= text_props[text_prop_next].tp_col - 1))
		{
		    if (text_props[text_prop_next].tp_col == MAXCOL
			    || bcol <= text_props[text_prop_next].tp_col - 1
					   + text_props[text_prop_next].tp_len)
			text_prop_idxs[text_props_active++] = text_prop_next;
		    ++text_prop_next;
		}

		if (wlv.n_extra == 0 ||
			(!wlv.extra_for_textprop
			 && !(text_prop_type != NULL &&
			     text_prop_flags & PT_FLAG_OVERRIDE)
		    ))
		{
		    text_prop_attr = 0;
		    text_prop_attr_comb = 0;
		    text_prop_flags = 0;
		    text_prop_type = NULL;
		    text_prop_id = 0;
		    reset_extra_attr = FALSE;
		}
		if (text_props_active > 0 && wlv.n_extra == 0
							&& !display_text_first)
		{
		    int used_tpi = -1;
		    int used_attr = 0;
		    int other_tpi = -1;

		    text_prop_above = FALSE;
		    text_prop_follows = FALSE;

		    // Sort the properties on priority and/or starting last.
		    // Then combine the attributes, highest priority last.
		    sort_text_props(wp->w_buffer, text_props,
					    text_prop_idxs, text_props_active);

		    for (pi = 0; pi < text_props_active; ++pi)
		    {
			int	    tpi = text_prop_idxs[pi];
			textprop_T  *tp = &text_props[tpi];
			proptype_T  *pt = text_prop_type_by_id(
						    wp->w_buffer, tp->tp_type);

			// Only use a text property that can be displayed.
			// Skip "after" properties when wrap is off and at the
			// end of the window.
			if (pt != NULL
				&& (pt->pt_hl_id > 0 || tp->tp_id < 0)
				&& tp->tp_id != -MAXCOL
				&& !(tp->tp_id < 0
				    && !wp->w_p_wrap
				    && (tp->tp_flags & (TP_FLAG_ALIGN_RIGHT
						| TP_FLAG_ALIGN_ABOVE
						| TP_FLAG_ALIGN_BELOW)) == 0
				    && wlv.col >= wp->w_width))
			{
			    if (tp->tp_col == MAXCOL
				     && *ptr == NUL
				     && ((wp->w_p_list && lcs_eol_one > 0
					     && (tp->tp_flags
						   & TP_FLAG_ALIGN_ABOVE) == 0)
					 || (ptr == line
						&& !did_line
						&& (tp->tp_flags
						      & TP_FLAG_ALIGN_BELOW))))
			    {
				// skip this prop, first display the '$' after
				// the line or display an empty line
				text_prop_follows = TRUE;
				if (used_tpi < 0)
				    display_text_first = TRUE;
				continue;
			    }

			    if (pt->pt_hl_id > 0)
				used_attr = syn_id2attr(pt->pt_hl_id);
			    text_prop_type = pt;
			    text_prop_attr =
				   hl_combine_attr(text_prop_attr, used_attr);
			    if (used_tpi >= 0 && text_props[used_tpi].tp_id < 0)
				other_tpi = used_tpi;
			    text_prop_flags = pt->pt_flags;
			    text_prop_id = tp->tp_id;
			    used_tpi = tpi;
			    display_text_first = FALSE;
			}
		    }
		    if (text_prop_id < 0 && used_tpi >= 0
			    && -text_prop_id
				      <= wp->w_buffer->b_textprop_text.ga_len)
		    {
			textprop_T  *tp = &text_props[used_tpi];
			char_u	    *p = ((char_u **)wp->w_buffer
						   ->b_textprop_text.ga_data)[
							   -text_prop_id - 1];
			int	    above = (tp->tp_flags
							& TP_FLAG_ALIGN_ABOVE);
			int	    bail_out = FALSE;

			// reset the ID in the copy to avoid it being used
			// again
			tp->tp_id = -MAXCOL;

			if (p != NULL)
			{
			    int	    right = (tp->tp_flags
							& TP_FLAG_ALIGN_RIGHT);
			    int	    below = (tp->tp_flags
							& TP_FLAG_ALIGN_BELOW);
			    int	    wrap = tp->tp_col < MAXCOL
					      || (tp->tp_flags & TP_FLAG_WRAP);
			    int	    padding = tp->tp_col == MAXCOL
						 && tp->tp_len > 1
							  ? tp->tp_len - 1 : 0;

			    // Insert virtual text before the current
			    // character, or add after the end of the line.
			    wlv.p_extra = p;
			    wlv.c_extra = NUL;
			    wlv.c_final = NUL;
			    wlv.n_extra = (int)STRLEN(p);
			    wlv.extra_for_textprop = TRUE;
			    wlv.start_extra_for_textprop = TRUE;
			    wlv.extra_attr = hl_combine_attr(wlv.win_attr,
								    used_attr);
			    n_attr = mb_charlen(p);
			    text_prop_attr = 0;
			    text_prop_attr_comb = 0;
			    if (*ptr == NUL)
				// don't combine char attr after EOL
				text_prop_flags &= ~PT_FLAG_COMBINE;
# ifdef FEAT_LINEBREAK
			    if (above || below || right || !wrap)
			    {
				// no 'showbreak' before "below" text property
				// or after "above" or "right" text property
				wlv.need_showbreak = FALSE;
				wlv.dont_use_showbreak = TRUE;
			    }
# endif
			    if ((right || above || below || !wrap
					    || padding > 0) && wp->w_width > 2)
			    {
				char_u	*prev_p_extra = wlv.p_extra;
				int	start_line;

				// Take care of padding, right-align and
				// truncation.
				// Shared with win_lbr_chartabsize(), must do
				// exactly the same.
				start_line = text_prop_position(wp, tp,
						    wlv.vcol,
# ifdef FEAT_RIGHTLEFT
						    wp->w_p_rl
						    ? wp->w_width - wlv.col - 1
						    :
# endif
						    wlv.col,
						    &wlv.n_extra, &wlv.p_extra,
						    &n_attr, &wlv.n_attr_skip,
						    skip_cells > 0);
				if (wlv.p_extra != prev_p_extra)
				{
				    // wlv.p_extra was allocated
				    vim_free(p_extra_free2);
				    p_extra_free2 = wlv.p_extra;
				}

				if (above)
				    wlv.vcol_off_tp = wlv.n_extra;

				if (lcs_eol_one < 0
					&& wp->w_p_wrap
					&& wlv.col
					       + wlv.n_extra - 2 > wp->w_width)
				    // don't bail out at end of line
				    text_prop_follows = TRUE;

				// When 'wrap' is off then for "below" we need
				// to start a new line explicitly.
				if (start_line)
				{
				    draw_screen_line(wp, &wlv);

				    // When line got too long for screen break
				    // here.
				    if (wlv.row == endrow)
				    {
					++wlv.row;
					break;
				    }
				    win_line_start(wp, &wlv, TRUE);
				    bail_out = TRUE;
				}
			    }
			}

			// If the text didn't reach until the first window
			// column we need to skip cells.
			if (skip_cells > 0)
			{
			    if (wlv.n_extra > skip_cells)
			    {
				wlv.n_extra -= skip_cells;
				wlv.p_extra += skip_cells;
				wlv.n_attr_skip -= skip_cells;
				if (wlv.n_attr_skip < 0)
				    wlv.n_attr_skip = 0;
				skipped_cells += skip_cells;
				skip_cells = 0;
			    }
			    else
			    {
				// the whole text is left of the window, drop
				// it and advance to the next one
				skip_cells -= wlv.n_extra;
				skipped_cells += wlv.n_extra;
				wlv.n_extra = 0;
				wlv.n_attr_skip = 0;
				bail_out = TRUE;
			    }
			}

			// If another text prop follows the condition below at
			// the last window column must know.
			// If this is an "above" text prop and 'nowrap' the we
			// must wrap anyway.
			text_prop_above = above;
			text_prop_follows |= other_tpi != -1
					&& (wp->w_p_wrap
					     || (text_props[other_tpi].tp_flags
			       & (TP_FLAG_ALIGN_BELOW | TP_FLAG_ALIGN_RIGHT)));

			if (bail_out)
			    // starting a new line for "below"
			    continue;
		    }
		}
		else if (text_prop_next < text_prop_count
			   && text_props[text_prop_next].tp_col == MAXCOL
			   && ((*ptr != NUL && ptr[mb_ptr2len(ptr)] == NUL)
			       || (!wp->w_p_wrap
				       && wlv.col == wp->w_width - 1
				       && (text_props[text_prop_next].tp_flags
						      & TP_FLAG_ALIGN_BELOW))))
		    // When at last-but-one character and a text property
		    // follows after it, we may need to flush the line after
		    // displaying that character.
		    // Or when not wrapping and at the rightmost column.
		    text_prop_follows = TRUE;
	    }

	    if (wlv.start_extra_for_textprop)
	    {
		wlv.start_extra_for_textprop = FALSE;
		// restore search_attr and area_attr when n_extra
		// is down to zero
		saved_search_attr = search_attr;
		saved_area_attr = area_attr;
		search_attr = 0;
		area_attr = 0;
	    }
#endif

	    int *area_attr_p =
#ifdef FEAT_PROP_POPUP
		wlv.extra_for_textprop ? &saved_area_attr :
#endif
							    &area_attr;

	    // handle Visual or match highlighting in this line
	    if (wlv.vcol == wlv.fromcol
		    || (has_mbyte && wlv.vcol + 1 == wlv.fromcol
			&& ((wlv.n_extra == 0 && (*mb_ptr2cells)(ptr) > 1)
			    || (wlv.n_extra > 0 && wlv.p_extra != NULL
				&& (*mb_ptr2cells)(wlv.p_extra) > 1)))
		    || ((int)vcol_prev == fromcol_prev
			&& vcol_prev < wlv.vcol	// not at margin
			&& wlv.vcol < wlv.tocol))
		*area_attr_p = vi_attr;		// start highlighting
	    else if (*area_attr_p != 0
		    && (wlv.vcol == wlv.tocol
			|| (noinvcur && (colnr_T)wlv.vcol == wp->w_virtcol)))
		*area_attr_p = 0;		// stop highlighting

#ifdef FEAT_SEARCH_EXTRA
	    if (wlv.n_extra == 0)
	    {
		// Check for start/end of 'hlsearch' and other matches.
		// After end, check for start/end of next match.
		// When another match, have to check for start again.
		v = (long)(ptr - line);
		search_attr = update_search_hl(wp, lnum, (colnr_T)v, &line,
				      &screen_search_hl, &has_match_conc,
				      &match_conc, did_line_attr, lcs_eol_one,
				      &on_last_col);
		ptr = line + v;  // "line" may have been changed

		// Do not allow a conceal over EOL otherwise EOL will be missed
		// and bad things happen.
		if (*ptr == NUL)
		    has_match_conc = 0;
	    }
#endif

#ifdef FEAT_DIFF
	    if (wlv.diff_hlf != (hlf_T)0)
	    {
		// When there is extra text (e.g. virtual text) it gets the
		// diff highlighting for the line, but not for changed text.
		if (wlv.diff_hlf == HLF_CHD && ptr - line >= change_start
							   && wlv.n_extra == 0)
		    wlv.diff_hlf = HLF_TXD;		// changed text
		if (wlv.diff_hlf == HLF_TXD
			&& ((ptr - line > change_end && wlv.n_extra == 0)
			       || (wlv.n_extra > 0 && wlv.extra_for_textprop)))
		    wlv.diff_hlf = HLF_CHD;		// changed line
		wlv.line_attr = HL_ATTR(wlv.diff_hlf);
		if (wp->w_p_cul && lnum == wp->w_cursor.lnum
			&& wp->w_p_culopt_flags != CULOPT_NBR
			&& (!wlv.cul_screenline || (wlv.vcol >= left_curline_col
					    && wlv.vcol <= right_curline_col)))
		    wlv.line_attr = hl_combine_attr(
					  wlv.line_attr, HL_ATTR(HLF_CUL));
	    }
#endif

#ifdef FEAT_SYN_HL
	    if (extra_check && wlv.n_extra == 0)
	    {
		syntax_attr = 0;
# ifdef FEAT_TERMINAL
		if (get_term_attr)
		    syntax_attr = term_get_attr(wp, lnum, wlv.vcol);
# endif
		// Get syntax attribute.
		if (has_syntax)
		{
		    // Get the syntax attribute for the character.  If there
		    // is an error, disable syntax highlighting.
		    save_did_emsg = did_emsg;
		    did_emsg = FALSE;

		    v = (long)(ptr - line);
		    if (v == prev_syntax_col)
			// at same column again
			syntax_attr = prev_syntax_attr;
		    else
		    {
# ifdef FEAT_SPELL
			can_spell = TRUE;
# endif
			syntax_attr = get_syntax_attr((colnr_T)v,
# ifdef FEAT_SPELL
					    spv->spv_has_spell ? &can_spell :
# endif
					    NULL, FALSE);
			prev_syntax_col = v;
			prev_syntax_attr = syntax_attr;
		    }

		    if (did_emsg)
		    {
			wp->w_s->b_syn_error = TRUE;
			has_syntax = FALSE;
			syntax_attr = 0;
		    }
		    else
			did_emsg = save_did_emsg;
# ifdef SYN_TIME_LIMIT
		    if (wp->w_s->b_syn_slow)
			has_syntax = FALSE;
# endif

		    // Need to get the line again, a multi-line regexp may
		    // have made it invalid.
		    line = ml_get_buf(wp->w_buffer, lnum, FALSE);
		    ptr = line + v;
# ifdef FEAT_CONCEAL
		    // no concealing past the end of the line, it interferes
		    // with line highlighting
		    if (*ptr == NUL)
			syntax_flags = 0;
		    else
			syntax_flags = get_syntax_info(&syntax_seqnr);
# endif
		}
	    }
# ifdef FEAT_PROP_POPUP
	    // Combine text property highlight into syntax highlight.
	    if (text_prop_type != NULL)
	    {
		if (text_prop_flags & PT_FLAG_COMBINE)
		    syntax_attr = hl_combine_attr(syntax_attr, text_prop_attr);
		else
		    syntax_attr = text_prop_attr;
		text_prop_attr_comb = syntax_attr;
	    }
# endif
#endif

	    // Decide which of the highlight attributes to use.
	    attr_pri = TRUE;
#ifdef LINE_ATTR
	    if (area_attr != 0)
	    {
		wlv.char_attr = hl_combine_attr(wlv.line_attr, area_attr);
		if (!highlight_match)
		    // let search highlight show in Visual area if possible
		    wlv.char_attr = hl_combine_attr(search_attr, wlv.char_attr);
# ifdef FEAT_SYN_HL
		wlv.char_attr = hl_combine_attr(syntax_attr, wlv.char_attr);
# endif
	    }
	    else if (search_attr != 0)
	    {
		wlv.char_attr = hl_combine_attr(wlv.line_attr, search_attr);
# ifdef FEAT_SYN_HL
		wlv.char_attr = hl_combine_attr(syntax_attr, wlv.char_attr);
# endif
	    }
	    else if (wlv.line_attr != 0
		    && ((wlv.fromcol == -10 && wlv.tocol == MAXCOL)
			      || wlv.vcol < wlv.fromcol
			      || vcol_prev < fromcol_prev
			      || wlv.vcol >= wlv.tocol))
	    {
		// Use wlv.line_attr when not in the Visual or 'incsearch' area
		// (area_attr may be 0 when "noinvcur" is set).
# ifdef FEAT_SYN_HL
		wlv.char_attr = hl_combine_attr(syntax_attr, wlv.line_attr);
# else
		wlv.char_attr = wlv.line_attr;
# endif
		attr_pri = FALSE;
	    }
#else
	    if (area_attr != 0)
		wlv.char_attr = area_attr;
	    else if (search_attr != 0)
		wlv.char_attr = search_attr;
#endif
	    else
	    {
		attr_pri = FALSE;
#ifdef FEAT_SYN_HL
		wlv.char_attr = syntax_attr;
#else
		wlv.char_attr = 0;
#endif
	    }
#ifdef FEAT_PROP_POPUP
	    // override with text property highlight when "override" is TRUE
	    if (text_prop_type != NULL && (text_prop_flags & PT_FLAG_OVERRIDE))
		wlv.char_attr = hl_combine_attr(wlv.char_attr, text_prop_attr);
#endif
	}

	// combine attribute with 'wincolor'
	if (wlv.win_attr != 0)
	{
	    if (wlv.char_attr == 0)
		wlv.char_attr = wlv.win_attr;
	    else
		wlv.char_attr = hl_combine_attr(wlv.win_attr, wlv.char_attr);
	}

	// Get the next character to put on the screen.

	// The "p_extra" points to the extra stuff that is inserted to
	// represent special characters (non-printable stuff) and other
	// things.  When all characters are the same, c_extra is used.
	// If wlv.c_final is set, it will compulsorily be used at the end.
	// "p_extra" must end in a NUL to avoid mb_ptr2len() reads past
	// "p_extra[n_extra]".
	// For the '$' of the 'list' option, n_extra == 1, p_extra == "".
	if (wlv.n_extra > 0)
	{
	    if (wlv.c_extra != NUL || (wlv.n_extra == 1 && wlv.c_final != NUL))
	    {
		c = (wlv.n_extra == 1 && wlv.c_final != NUL)
						   ? wlv.c_final : wlv.c_extra;
		mb_c = c;	// doesn't handle non-utf-8 multi-byte!
		if (enc_utf8 && utf_char2len(c) > 1)
		{
		    mb_utf8 = TRUE;
		    u8cc[0] = 0;
		    c = 0xc0;
		}
		else
		    mb_utf8 = FALSE;
	    }
	    else
	    {
		c = *wlv.p_extra;
		if (has_mbyte)
		{
		    mb_c = c;
		    if (enc_utf8)
		    {
			// If the UTF-8 character is more than one byte:
			// Decode it into "mb_c".
			mb_l = utfc_ptr2len(wlv.p_extra);
			mb_utf8 = FALSE;
			if (mb_l > wlv.n_extra)
			    mb_l = 1;
			else if (mb_l > 1)
			{
			    mb_c = utfc_ptr2char(wlv.p_extra, u8cc);
			    mb_utf8 = TRUE;
			    c = 0xc0;
			}
		    }
		    else
		    {
			// if this is a DBCS character, put it in "mb_c"
			mb_l = MB_BYTE2LEN(c);
			if (mb_l >= wlv.n_extra)
			    mb_l = 1;
			else if (mb_l > 1)
			    mb_c = (c << 8) + wlv.p_extra[1];
		    }
		    if (mb_l == 0)  // at the NUL at end-of-line
			mb_l = 1;

		    // If a double-width char doesn't fit display a '>' in the
		    // last column.
		    if ((
# ifdef FEAT_RIGHTLEFT
			    wp->w_p_rl ? (wlv.col <= 0) :
# endif
				    (wlv.col >= wp->w_width - 1))
			    && (*mb_char2cells)(mb_c) == 2)
		    {
			c = '>';
			mb_c = c;
			mb_l = 1;
			mb_utf8 = FALSE;
			multi_attr = HL_ATTR(HLF_AT);
#ifdef FEAT_SYN_HL
			if (wlv.cul_attr)
			    multi_attr = hl_combine_attr(
						     multi_attr, wlv.cul_attr);
#endif
			multi_attr = hl_combine_attr(wlv.win_attr, multi_attr);

			// put the pointer back to output the double-width
			// character at the start of the next line.
			++wlv.n_extra;
			--wlv.p_extra;
		    }
		    else
		    {
			wlv.n_extra -= mb_l - 1;
			wlv.p_extra += mb_l - 1;
		    }
		}
		++wlv.p_extra;
	    }
	    --wlv.n_extra;
#if defined(FEAT_PROP_POPUP)
	    if (wlv.n_extra <= 0)
	    {
		// Only restore search_attr and area_attr after "n_extra" in
		// the next screen line is also done.
		if (wlv.saved_n_extra <= 0)
		{
		    if (search_attr == 0)
			search_attr = saved_search_attr;
		    if (area_attr == 0 && *ptr != NUL)
			area_attr = saved_area_attr;

		    if (wlv.extra_for_textprop)
			// wlv.extra_attr should be used at this position but
			// not any further.
			reset_extra_attr = TRUE;
		}

		wlv.extra_for_textprop = FALSE;
		in_linebreak = FALSE;
	    }
#endif
	}
	else
	{
#ifdef FEAT_LINEBREAK
	    int		c0;
#endif
	    char_u	*prev_ptr = ptr;

	    // Get a character from the line itself.
	    c = *ptr;
#ifdef FEAT_LINEBREAK
	    c0 = *ptr;
#endif
	    if (c == NUL)
	    {
#ifdef FEAT_PROP_POPUP
		// text is finished, may display a "below" virtual text
		did_line = TRUE;
#endif
		// no more cells to skip
		skip_cells = 0;
	    }

	    if (has_mbyte)
	    {
		mb_c = c;
		if (enc_utf8)
		{
		    // If the UTF-8 character is more than one byte: Decode it
		    // into "mb_c".
		    mb_l = utfc_ptr2len(ptr);
		    mb_utf8 = FALSE;
		    if (mb_l > 1)
		    {
			mb_c = utfc_ptr2char(ptr, u8cc);
			// Overlong encoded ASCII or ASCII with composing char
			// is displayed normally, except a NUL.
			if (mb_c < 0x80)
			{
			    c = mb_c;
#ifdef FEAT_LINEBREAK
			    c0 = mb_c;
#endif
			}
			mb_utf8 = TRUE;

			// At start of the line we can have a composing char.
			// Draw it as a space with a composing char.
			if (utf_iscomposing(mb_c))
			{
			    int i;

			    for (i = Screen_mco - 1; i > 0; --i)
				u8cc[i] = u8cc[i - 1];
			    u8cc[0] = mb_c;
			    mb_c = ' ';
			}
		    }

		    if ((mb_l == 1 && c >= 0x80)
			    || (mb_l >= 1 && mb_c == 0)
			    || (mb_l > 1 && (!vim_isprintc(mb_c))))
		    {
			// Illegal UTF-8 byte: display as <xx>.
			// Non-BMP character : display as ? or fullwidth ?.
			transchar_hex(wlv.extra, mb_c);
# ifdef FEAT_RIGHTLEFT
			if (wp->w_p_rl)		// reverse
			    rl_mirror(wlv.extra);
# endif
			wlv.p_extra = wlv.extra;
			c = *wlv.p_extra;
			mb_c = mb_ptr2char_adv(&wlv.p_extra);
			mb_utf8 = (c >= 0x80);
			wlv.n_extra = (int)STRLEN(wlv.p_extra);
			wlv.c_extra = NUL;
			wlv.c_final = NUL;
			if (area_attr == 0 && search_attr == 0)
			{
			    n_attr = wlv.n_extra + 1;
			    wlv.extra_attr = hl_combine_attr(
						 wlv.win_attr, HL_ATTR(HLF_8));
			    saved_attr2 = wlv.char_attr; // save current attr
			}
		    }
		    else if (mb_l == 0)  // at the NUL at end-of-line
			mb_l = 1;
#ifdef FEAT_ARABIC
		    else if (p_arshape && !p_tbidi && ARABIC_CHAR(mb_c))
		    {
			// Do Arabic shaping.
			int	pc, pc1, nc;
			int	pcc[MAX_MCO];

			// The idea of what is the previous and next
			// character depends on 'rightleft'.
			if (wp->w_p_rl)
			{
			    pc = prev_c;
			    pc1 = prev_c1;
			    nc = utf_ptr2char(ptr + mb_l);
			    prev_c1 = u8cc[0];
			}
			else
			{
			    pc = utfc_ptr2char(ptr + mb_l, pcc);
			    nc = prev_c;
			    pc1 = pcc[0];
			}
			prev_c = mb_c;

			mb_c = arabic_shape(mb_c, &c, &u8cc[0], pc, pc1, nc);
		    }
		    else
			prev_c = mb_c;
#endif
		}
		else	// enc_dbcs
		{
		    mb_l = MB_BYTE2LEN(c);
		    if (mb_l == 0)  // at the NUL at end-of-line
			mb_l = 1;
		    else if (mb_l > 1)
		    {
			// We assume a second byte below 32 is illegal.
			// Hopefully this is OK for all double-byte encodings!
			if (ptr[1] >= 32)
			    mb_c = (c << 8) + ptr[1];
			else
			{
			    if (ptr[1] == NUL)
			    {
				// head byte at end of line
				mb_l = 1;
				transchar_nonprint(wp->w_buffer, wlv.extra, c);
			    }
			    else
			    {
				// illegal tail byte
				mb_l = 2;
				STRCPY(wlv.extra, "XX");
			    }
			    wlv.p_extra = wlv.extra;
			    wlv.n_extra = (int)STRLEN(wlv.extra) - 1;
			    wlv.c_extra = NUL;
			    wlv.c_final = NUL;
			    c = *wlv.p_extra++;
			    if (area_attr == 0 && search_attr == 0)
			    {
				n_attr = wlv.n_extra + 1;
				wlv.extra_attr = hl_combine_attr(
						 wlv.win_attr, HL_ATTR(HLF_8));
				// save current attr
				saved_attr2 = wlv.char_attr;
			    }
			    mb_c = c;
			}
		    }
		}
		// If a double-width char doesn't fit display a '>' in the
		// last column; the character is displayed at the start of the
		// next line.
		if ((
# ifdef FEAT_RIGHTLEFT
			    wp->w_p_rl ? (wlv.col <= 0) :
# endif
				(wlv.col >= wp->w_width - 1))
			&& (*mb_char2cells)(mb_c) == 2)
		{
		    c = '>';
		    mb_c = c;
		    mb_utf8 = FALSE;
		    mb_l = 1;
		    multi_attr = hl_combine_attr(wlv.win_attr, HL_ATTR(HLF_AT));
		    // Put pointer back so that the character will be
		    // displayed at the start of the next line.
		    --ptr;
#ifdef FEAT_CONCEAL
		    did_decrement_ptr = TRUE;
#endif
		}
		else if (*ptr != NUL)
		    ptr += mb_l - 1;

		// If a double-width char doesn't fit at the left side display
		// a '<' in the first column.  Don't do this for unprintable
		// characters.
		if (skip_cells > 0 && mb_l > 1 && wlv.n_extra == 0)
		{
		    wlv.n_extra = 1;
		    wlv.c_extra = MB_FILLER_CHAR;
		    wlv.c_final = NUL;
		    c = ' ';
		    if (area_attr == 0 && search_attr == 0)
		    {
			n_attr = wlv.n_extra + 1;
			wlv.extra_attr = hl_combine_attr(
						wlv.win_attr, HL_ATTR(HLF_AT));
			saved_attr2 = wlv.char_attr; // save current attr
		    }
		    mb_c = c;
		    mb_utf8 = FALSE;
		    mb_l = 1;
		}

	    }
	    ++ptr;

	    if (extra_check)
	    {
#ifdef FEAT_SPELL
		// Check spelling (unless at the end of the line).
		// Only do this when there is no syntax highlighting, the
		// @Spell cluster is not used or the current syntax item
		// contains the @Spell cluster.
		v = (long)(ptr - line);
		if (spv->spv_has_spell && v >= word_end && v > cur_checked_col)
		{
		    spell_attr = 0;
		    // do not calculate cap_col at the end of the line or when
		    // only white space is following
		    if (c != 0 && (*skipwhite(prev_ptr) != NUL) && (
# ifdef FEAT_SYN_HL
				!has_syntax ||
# endif
				can_spell))
		    {
			char_u	*p;
			int	len;
			hlf_T	spell_hlf = HLF_COUNT;

			if (has_mbyte)
			    v -= mb_l - 1;

			// Use nextline[] if possible, it has the start of the
			// next line concatenated.
			if ((prev_ptr - line) - nextlinecol >= 0)
			    p = nextline + (prev_ptr - line) - nextlinecol;
			else
			    p = prev_ptr;
			spv->spv_cap_col -= (int)(prev_ptr - line);
			len = spell_check(wp, p, &spell_hlf, &spv->spv_cap_col,
							    spv->spv_unchanged);
			word_end = v + len;

			// In Insert mode only highlight a word that
			// doesn't touch the cursor.
			if (spell_hlf != HLF_COUNT
				&& (State & MODE_INSERT)
				&& wp->w_cursor.lnum == lnum
				&& wp->w_cursor.col >=
						    (colnr_T)(prev_ptr - line)
				&& wp->w_cursor.col < (colnr_T)word_end)
			{
			    spell_hlf = HLF_COUNT;
			    spell_redraw_lnum = lnum;
			}

			if (spell_hlf == HLF_COUNT && p != prev_ptr
				       && (p - nextline) + len > nextline_idx)
			{
			    // Remember that the good word continues at the
			    // start of the next line.
			    spv->spv_checked_lnum = lnum + 1;
			    spv->spv_checked_col = (p - nextline) + len
								- nextline_idx;
			}

			// Turn index into actual attributes.
			if (spell_hlf != HLF_COUNT)
			    spell_attr = highlight_attr[spell_hlf];

			if (spv->spv_cap_col > 0)
			{
			    if (p != prev_ptr
				    && (p - nextline) + spv->spv_cap_col
								>= nextline_idx)
			    {
				// Remember that the word in the next line
				// must start with a capital.
				spv->spv_capcol_lnum = lnum + 1;
				spv->spv_cap_col = ((p - nextline)
					    + spv->spv_cap_col - nextline_idx);
			    }
			    else
				// Compute the actual column.
				spv->spv_cap_col += (prev_ptr - line);
			}
		    }
		}
		if (spell_attr != 0)
		{
		    if (!attr_pri)
			wlv.char_attr = hl_combine_attr(wlv.char_attr,
								   spell_attr);
		    else
			wlv.char_attr = hl_combine_attr(spell_attr,
								wlv.char_attr);
		}
#endif
#ifdef FEAT_LINEBREAK
		// we don't want linebreak to apply for lines that start with
		// leading spaces, followed by long letters (since it would add
		// a break at the beginning of a line and this might be unexpected)
		//
		// So only allow to linebreak, once we have found chars not in
		// 'breakat' in the line.
		if ( wp->w_p_lbr && !wlv.need_lbr && c != NUL &&
			!VIM_ISBREAK((int)*ptr))
		    wlv.need_lbr = TRUE;
#endif
#ifdef FEAT_LINEBREAK
		// Found last space before word: check for line break.
		if (wp->w_p_lbr && c0 == c && wlv.need_lbr
				  && VIM_ISBREAK(c) && !VIM_ISBREAK((int)*ptr))
		{
		    int	    mb_off = has_mbyte ? (*mb_head_off)(line, ptr - 1)
									   : 0;
		    char_u  *p = ptr - (mb_off + 1);
		    chartabsize_T cts;

		    init_chartabsize_arg(&cts, wp, lnum, wlv.vcol
# ifdef FEAT_PROP_POPUP
							     - vcol_first_char,
# endif
								      line, p);
# ifdef FEAT_PROP_POPUP
		    // do not want virtual text counted here
		    cts.cts_has_prop_with_text = FALSE;
# endif
		    wlv.n_extra = win_lbr_chartabsize(&cts, NULL) - 1;
		    clear_chartabsize_arg(&cts);

		    if (on_last_col && c != TAB)
			// Do not continue search/match highlighting over the
			// line break, but for TABs the highlighting should
			// include the complete width of the character
			search_attr = 0;

		    if (c == TAB && wlv.n_extra + wlv.col > wp->w_width)
# ifdef FEAT_VARTABS
			wlv.n_extra = tabstop_padding(wlv.vcol,
					      wp->w_buffer->b_p_ts,
					      wp->w_buffer->b_p_vts_array) - 1;
# else
			wlv.n_extra = (int)wp->w_buffer->b_p_ts
				    - wlv.vcol % (int)wp->w_buffer->b_p_ts - 1;
# endif

		    wlv.c_extra = mb_off > 0 ? MB_FILLER_CHAR : ' ';
		    wlv.c_final = NUL;
# ifdef FEAT_PROP_POPUP
		    if (wlv.n_extra > 0 && c != TAB)
			in_linebreak = TRUE;
# endif
		    if (VIM_ISWHITE(c))
		    {
# ifdef FEAT_CONCEAL
			if (c == TAB)
			    // See "Tab alignment" below.
			    FIX_FOR_BOGUSCOLS;
# endif
			if (!wp->w_p_list)
			    c = ' ';
		    }
		}
#endif
		if (wp->w_p_list)
		{
		    in_multispace = c == ' ' && (*ptr == ' '
				  || (prev_ptr > line && prev_ptr[-1] == ' '));
		    if (!in_multispace)
			multispace_pos = 0;
		}

		// 'list': Change char 160 to 'nbsp' and space to 'space'
		// setting in 'listchars'.  But not when the character is
		// followed by a composing character (use mb_l to check that).
		if (wp->w_p_list
			&& ((((c == 160 && mb_l == 1)
			      || (mb_utf8
				  && ((mb_c == 160 && mb_l == 2)
				      || (mb_c == 0x202f && mb_l == 3))))
			     && wp->w_lcs_chars.nbsp)
			    || (c == ' '
				&& mb_l == 1
				&& (wp->w_lcs_chars.space
				    || (in_multispace
					&& wp->w_lcs_chars.multispace != NULL))
				&& ptr - line >= leadcol
				&& ptr - line <= trailcol)))
		{
		    if (in_multispace && wp->w_lcs_chars.multispace != NULL)
		    {
			c = wp->w_lcs_chars.multispace[multispace_pos++];
			if (wp->w_lcs_chars.multispace[multispace_pos] == NUL)
			    multispace_pos = 0;
		    }
		    else
			c = (c == ' ') ? wp->w_lcs_chars.space
					: wp->w_lcs_chars.nbsp;
		    if (area_attr == 0 && search_attr == 0)
		    {
			n_attr = 1;
			wlv.extra_attr = hl_combine_attr(wlv.win_attr,
							       HL_ATTR(HLF_8));
			saved_attr2 = wlv.char_attr; // save current attr
		    }
		    mb_c = c;
		    if (enc_utf8 && utf_char2len(c) > 1)
		    {
			mb_utf8 = TRUE;
			u8cc[0] = 0;
			c = 0xc0;
		    }
		    else
			mb_utf8 = FALSE;
		}

		if (c == ' ' && ((trailcol != MAXCOL && ptr > line + trailcol)
				    || (leadcol != 0 && ptr < line + leadcol)))
		{
		    if (leadcol != 0 && in_multispace && ptr < line + leadcol
			    && wp->w_lcs_chars.leadmultispace != NULL)
		    {
			c = wp->w_lcs_chars.leadmultispace[multispace_pos++];
			if (wp->w_lcs_chars.leadmultispace[multispace_pos]
									== NUL)
			    multispace_pos = 0;
		    }

		    else if (ptr > line + trailcol && wp->w_lcs_chars.trail)
			c = wp->w_lcs_chars.trail;

		    else if (ptr < line + leadcol && wp->w_lcs_chars.lead)
			c = wp->w_lcs_chars.lead;

		    else if (leadcol != 0 && wp->w_lcs_chars.space)
			c = wp->w_lcs_chars.space;


		    if (!attr_pri)
		    {
			n_attr = 1;
			wlv.extra_attr = hl_combine_attr(wlv.win_attr,
							       HL_ATTR(HLF_8));
			saved_attr2 = wlv.char_attr; // save current attr
		    }
		    mb_c = c;
		    if (enc_utf8 && utf_char2len(c) > 1)
		    {
			mb_utf8 = TRUE;
			u8cc[0] = 0;
			c = 0xc0;
		    }
		    else
			mb_utf8 = FALSE;
		}
	    }

	    // Handling of non-printable characters.
	    if (!vim_isprintc(c))
	    {
		// when getting a character from the file, we may have to
		// turn it into something else on the way to putting it
		// into "ScreenLines".
		if (c == TAB && (!wp->w_p_list || wp->w_lcs_chars.tab1))
		{
		    int	    tab_len = 0;
		    long    vcol_adjusted = wlv.vcol; // removed showbreak len
#ifdef FEAT_LINEBREAK
		    char_u  *sbr = get_showbreak_value(wp);

		    // only adjust the tab_len, when at the first column
		    // after the showbreak value was drawn
		    if (*sbr != NUL && wlv.vcol == wlv.vcol_sbr && wp->w_p_wrap)
			vcol_adjusted = wlv.vcol - MB_CHARLEN(sbr);
#endif
		    // tab amount depends on current column
#ifdef FEAT_VARTABS
		    tab_len = tabstop_padding(vcol_adjusted,
					      wp->w_buffer->b_p_ts,
					      wp->w_buffer->b_p_vts_array) - 1;
#else
		    tab_len = (int)wp->w_buffer->b_p_ts
			       - vcol_adjusted % (int)wp->w_buffer->b_p_ts - 1;
#endif

#ifdef FEAT_LINEBREAK
		    if (!wp->w_p_lbr || !wp->w_p_list)
#endif
		    {
			// tab amount depends on current column
			wlv.n_extra = tab_len;
		    }
#ifdef FEAT_LINEBREAK
		    else
		    {
			char_u	*p;
			int	len;
			int	i;
			int	saved_nextra = wlv.n_extra;

# ifdef FEAT_CONCEAL
			if (wlv.vcol_off_co > 0)
			    // there are characters to conceal
			    tab_len += wlv.vcol_off_co;

			// boguscols before FIX_FOR_BOGUSCOLS macro from above
			if (wp->w_p_list && wp->w_lcs_chars.tab1
						      && old_boguscols > 0
						      && wlv.n_extra > tab_len)
			    tab_len += wlv.n_extra - tab_len;
# endif
			if (tab_len > 0)
			{
			    // If wlv.n_extra > 0, it gives the number of chars
			    // to use for a tab, else we need to calculate the
			    // width for a tab.
			    int tab2_len = mb_char2len(wp->w_lcs_chars.tab2);
			    len = tab_len * tab2_len;
			    if (wp->w_lcs_chars.tab3)
				len += mb_char2len(wp->w_lcs_chars.tab3)
								    - tab2_len;
			    if (wlv.n_extra > 0)
				len += wlv.n_extra - tab_len;
			    c = wp->w_lcs_chars.tab1;
			    p = alloc(len + 1);
			    if (p == NULL)
				wlv.n_extra = 0;
			    else
			    {
				vim_memset(p, ' ', len);
				p[len] = NUL;
				vim_free(wlv.p_extra_free);
				wlv.p_extra_free = p;
				for (i = 0; i < tab_len; i++)
				{
				    int lcs = wp->w_lcs_chars.tab2;

				    if (*p == NUL)
				    {
					tab_len = i;
					break;
				    }

				    // if tab3 is given, use it for the last
				    // char
				    if (wp->w_lcs_chars.tab3
							   && i == tab_len - 1)
					lcs = wp->w_lcs_chars.tab3;
				    p += mb_char2bytes(lcs, p);
				    wlv.n_extra += mb_char2len(lcs)
						  - (saved_nextra > 0 ? 1 : 0);
				}
				wlv.p_extra = wlv.p_extra_free;
# ifdef FEAT_CONCEAL
				// n_extra will be increased by
				// FIX_FOX_BOGUSCOLS macro below, so need to
				// adjust for that here
				if (wlv.vcol_off_co > 0)
				    wlv.n_extra -= wlv.vcol_off_co;
# endif
			    }
			}
		    }
#endif
#ifdef FEAT_CONCEAL
		    {
			int vc_saved = wlv.vcol_off_co;

			// Tab alignment should be identical regardless of
			// 'conceallevel' value. So tab compensates of all
			// previous concealed characters, and thus resets
			// vcol_off_co and boguscols accumulated so far in the
			// line. Note that the tab can be longer than
			// 'tabstop' when there are concealed characters.
			FIX_FOR_BOGUSCOLS;

			// Make sure, the highlighting for the tab char will be
			// correctly set further below (effectively reverts the
			// FIX_FOR_BOGSUCOLS macro).
			if (wlv.n_extra == tab_len + vc_saved && wp->w_p_list
						&& wp->w_lcs_chars.tab1)
			    tab_len += vc_saved;
		    }
#endif
		    mb_utf8 = FALSE;	// don't draw as UTF-8
		    if (wp->w_p_list)
		    {
			c = (wlv.n_extra == 0 && wp->w_lcs_chars.tab3)
							? wp->w_lcs_chars.tab3
							: wp->w_lcs_chars.tab1;
#ifdef FEAT_LINEBREAK
			if (wp->w_p_lbr && wlv.p_extra != NULL
							&& *wlv.p_extra != NUL)
			    wlv.c_extra = NUL; // using p_extra from above
			else
#endif
			    wlv.c_extra = wp->w_lcs_chars.tab2;
			wlv.c_final = wp->w_lcs_chars.tab3;
			n_attr = tab_len + 1;
			wlv.extra_attr = hl_combine_attr(wlv.win_attr,
							       HL_ATTR(HLF_8));
			saved_attr2 = wlv.char_attr; // save current attr
			mb_c = c;
			if (enc_utf8 && utf_char2len(c) > 1)
			{
			    mb_utf8 = TRUE;
			    u8cc[0] = 0;
			    c = 0xc0;
			}
		    }
		    else
		    {
			wlv.c_final = NUL;
			wlv.c_extra = ' ';
			c = ' ';
		    }
		}
		else if (c == NUL
			&& wlv.n_extra == 0
			&& (wp->w_p_list
			    || ((wlv.fromcol >= 0 || fromcol_prev >= 0)
				&& wlv.tocol > wlv.vcol
				&& VIsual_mode != Ctrl_V
				&& (
# ifdef FEAT_RIGHTLEFT
				    wp->w_p_rl ? (wlv.col >= 0) :
# endif
				    (wlv.col < wp->w_width))
				&& !(noinvcur
				    && lnum == wp->w_cursor.lnum
				    && (colnr_T)wlv.vcol == wp->w_virtcol)))
			&& lcs_eol_one > 0)
		{
		    // Display a '$' after the line or highlight an extra
		    // character if the line break is included.
#if defined(FEAT_DIFF) || defined(LINE_ATTR)
		    // For a diff line the highlighting continues after the
		    // "$".
		    if (
# ifdef FEAT_DIFF
			    wlv.diff_hlf == (hlf_T)0
#  ifdef LINE_ATTR
			    &&
#  endif
# endif
# ifdef LINE_ATTR
			    wlv.line_attr == 0
# endif
		       )
#endif
		    {
			// In virtualedit, visual selections may extend
			// beyond end of line.
			if (!(area_highlighting && virtual_active()
				       && wlv.tocol != MAXCOL
				       && wlv.vcol < wlv.tocol))
			    wlv.p_extra = at_end_str;
			wlv.n_extra = 0;
		    }
		    if (wp->w_p_list && wp->w_lcs_chars.eol > 0)
			c = wp->w_lcs_chars.eol;
		    else
			c = ' ';
		    lcs_eol_one = -1;
		    --ptr;	    // put it back at the NUL
		    if (!attr_pri)
		    {
			wlv.extra_attr = hl_combine_attr(wlv.win_attr,
							      HL_ATTR(HLF_AT));
			n_attr = 1;
		    }
		    mb_c = c;
		    if (enc_utf8 && utf_char2len(c) > 1)
		    {
			mb_utf8 = TRUE;
			u8cc[0] = 0;
			c = 0xc0;
		    }
		    else
			mb_utf8 = FALSE;	// don't draw as UTF-8
		}
		else if (c != NUL)
		{
		    wlv.p_extra = transchar_buf(wp->w_buffer, c);
		    if (wlv.n_extra == 0)
			wlv.n_extra = byte2cells(c) - 1;
#ifdef FEAT_RIGHTLEFT
		    if ((dy_flags & DY_UHEX) && wp->w_p_rl)
			rl_mirror(wlv.p_extra);	// reverse "<12>"
#endif
		    wlv.c_extra = NUL;
		    wlv.c_final = NUL;
#ifdef FEAT_LINEBREAK
		    if (wp->w_p_lbr)
		    {
			char_u *p;

			c = *wlv.p_extra;
			p = alloc(wlv.n_extra + 1);
			vim_memset(p, ' ', wlv.n_extra);
			STRNCPY(p, wlv.p_extra + 1, STRLEN(wlv.p_extra) - 1);
			p[wlv.n_extra] = NUL;
			vim_free(wlv.p_extra_free);
			wlv.p_extra_free = wlv.p_extra = p;
		    }
		    else
#endif
		    {
			wlv.n_extra = byte2cells(c) - 1;
			c = *wlv.p_extra++;
		    }
		    if (!attr_pri)
		    {
			n_attr = wlv.n_extra + 1;
			wlv.extra_attr = hl_combine_attr(wlv.win_attr,
							       HL_ATTR(HLF_8));
#ifdef FEAT_PROP_POPUP
		    if (text_prop_type != NULL &&
			     text_prop_flags & PT_FLAG_OVERRIDE)
			wlv.extra_attr = hl_combine_attr(text_prop_attr, wlv.extra_attr);
#endif

			saved_attr2 = wlv.char_attr; // save current attr
		    }
		    mb_utf8 = FALSE;	// don't draw as UTF-8
		}
		else if (VIsual_active
			 && (VIsual_mode == Ctrl_V
			     || VIsual_mode == 'v')
			 && virtual_active()
			 && wlv.tocol != MAXCOL
			 && wlv.vcol < wlv.tocol
			 && (
#ifdef FEAT_RIGHTLEFT
			    wp->w_p_rl ? (wlv.col >= 0) :
#endif
			    (wlv.col < wp->w_width)))
		{
		    c = ' ';
		    --ptr;	    // put it back at the NUL
		}
#if defined(LINE_ATTR)
		else if ((
# ifdef FEAT_DIFF
			    wlv.diff_hlf != (hlf_T)0 ||
# endif
# ifdef FEAT_TERMINAL
			    wlv.win_attr != 0 ||
# endif
			    wlv.line_attr != 0
			) && (
# ifdef FEAT_RIGHTLEFT
			    wp->w_p_rl ? (wlv.col >= 0) :
# endif
			    (wlv.col
# ifdef FEAT_CONCEAL
				- wlv.boguscols
# endif
					    < wp->w_width)))
		{
		    // Highlight until the right side of the window
		    c = ' ';
		    --ptr;	    // put it back at the NUL

		    // Remember we do the char for line highlighting.
		    ++did_line_attr;

		    // don't do search HL for the rest of the line
		    if (wlv.line_attr != 0 && wlv.char_attr == search_attr
					&& (did_line_attr > 1
					    || (wp->w_p_list &&
						wp->w_lcs_chars.eol > 0)))
			wlv.char_attr = wlv.line_attr;
#ifdef FEAT_SIGNS
		    // At end of line: if Sign is present with line highlight, reset char_attr
		    // but not when cursorline is active
		    if (sign_present && wlv.sattr.sat_linehl > 0 && wlv.draw_state == WL_LINE
			 && !(wp->w_p_cul && lnum == wp->w_cursor.lnum))
			wlv.char_attr = wlv.sattr.sat_linehl;
#endif
# ifdef FEAT_DIFF
		    if (wlv.diff_hlf == HLF_TXD)
		    {
			wlv.diff_hlf = HLF_CHD;
			if (vi_attr == 0 || wlv.char_attr != vi_attr)
			{
			    wlv.char_attr = HL_ATTR(wlv.diff_hlf);
			    if (wp->w_p_cul && lnum == wp->w_cursor.lnum
				    && wp->w_p_culopt_flags != CULOPT_NBR
				    && (!wlv.cul_screenline
					|| (wlv.vcol >= left_curline_col
					    && wlv.vcol <= right_curline_col)))
				wlv.char_attr = hl_combine_attr(
					  wlv.char_attr, HL_ATTR(HLF_CUL));
			}
		    }
# endif
# ifdef FEAT_TERMINAL
		    if (wlv.win_attr != 0)
		    {
			wlv.char_attr = wlv.win_attr;
			if (wp->w_p_cul && lnum == wp->w_cursor.lnum
				    && wp->w_p_culopt_flags != CULOPT_NBR)
			{
			    if (!wlv.cul_screenline
				    || (wlv.vcol >= left_curline_col
					     && wlv.vcol <= right_curline_col))
				wlv.char_attr = hl_combine_attr(
					      wlv.char_attr, HL_ATTR(HLF_CUL));
			}
			else if (wlv.line_attr)
			    wlv.char_attr = hl_combine_attr(
						 wlv.char_attr, wlv.line_attr);
		    }
# endif
		}
#endif
	    }

#ifdef FEAT_CONCEAL
	    if (   wp->w_p_cole > 0
		&& (wp != curwin || lnum != wp->w_cursor.lnum
						    || conceal_cursor_line(wp))
		&& ((syntax_flags & HL_CONCEAL) != 0 || has_match_conc > 0)
		&& !(lnum_in_visual_area
				    && vim_strchr(wp->w_p_cocu, 'v') == NULL))
	    {
		wlv.char_attr = conceal_attr;
		if (((prev_syntax_id != syntax_seqnr
					   && (syntax_flags & HL_CONCEAL) != 0)
			    || has_match_conc > 1)
			&& (syn_get_sub_char() != NUL
				|| (has_match_conc && match_conc)
				|| wp->w_p_cole == 1)
			&& wp->w_p_cole != 3)
		{
		    // First time at this concealed item: display one
		    // character.
		    if (has_match_conc && match_conc)
			c = match_conc;
		    else if (syn_get_sub_char() != NUL)
			c = syn_get_sub_char();
		    else if (wp->w_lcs_chars.conceal != NUL)
			c = wp->w_lcs_chars.conceal;
		    else
			c = ' ';

		    prev_syntax_id = syntax_seqnr;

		    if (wlv.n_extra > 0)
			wlv.vcol_off_co += wlv.n_extra;
		    wlv.vcol += wlv.n_extra;
		    if (wp->w_p_wrap && wlv.n_extra > 0)
		    {
# ifdef FEAT_RIGHTLEFT
			if (wp->w_p_rl)
			{
			    wlv.col -= wlv.n_extra;
			    wlv.boguscols -= wlv.n_extra;
			}
			else
# endif
			{
			    wlv.boguscols += wlv.n_extra;
			    wlv.col += wlv.n_extra;
			}
		    }
		    wlv.n_extra = 0;
		    n_attr = 0;
		}
		else if (skip_cells == 0)
		{
		    is_concealing = TRUE;
		    skip_cells = 1;
		}
		mb_c = c;
		if (enc_utf8 && utf_char2len(c) > 1)
		{
		    mb_utf8 = TRUE;
		    u8cc[0] = 0;
		    c = 0xc0;
		}
		else
		    mb_utf8 = FALSE;	// don't draw as UTF-8
	    }
	    else
	    {
		prev_syntax_id = 0;
		is_concealing = FALSE;
	    }

	    if (skip_cells > 0 && did_decrement_ptr)
		// not showing the '>', put pointer back to avoid getting stuck
		++ptr;

#endif // FEAT_CONCEAL
	}

#ifdef FEAT_CONCEAL
	// In the cursor line and we may be concealing characters: correct
	// the cursor column when we reach its position.
	if (!did_wcol && wlv.draw_state == WL_LINE
		&& wp == curwin && lnum == wp->w_cursor.lnum
		&& conceal_cursor_line(wp)
		&& (int)wp->w_virtcol <= wlv.vcol + skip_cells)
	{
# ifdef FEAT_RIGHTLEFT
	    if (wp->w_p_rl)
		wp->w_wcol = wp->w_width - wlv.col + wlv.boguscols - 1;
	    else
# endif
		wp->w_wcol = wlv.col - wlv.boguscols;
	    wp->w_wrow = wlv.row;
	    did_wcol = TRUE;
	    curwin->w_valid |= VALID_WCOL|VALID_WROW|VALID_VIRTCOL;
# ifdef FEAT_PROP_POPUP
	    curwin->w_flags &= ~(WFLAG_WCOL_OFF_ADDED | WFLAG_WROW_OFF_ADDED);
# endif
	}
#endif

	// Use "wlv.extra_attr", but don't override visual selection
	// highlighting, unless text property overrides.
	// Don't use "wlv.extra_attr" until wlv.n_attr_skip is zero.
	if (wlv.n_attr_skip == 0 && n_attr > 0
		&& wlv.draw_state == WL_LINE
		&& (!attr_pri
#ifdef FEAT_PROP_POPUP
		    || (text_prop_flags & PT_FLAG_OVERRIDE)
#endif
		   ))
	{
#ifdef LINE_ATTR
	    if (wlv.line_attr)
		wlv.char_attr = hl_combine_attr(wlv.line_attr, wlv.extra_attr);
	    else
#endif
		wlv.char_attr = wlv.extra_attr;
#ifdef FEAT_PROP_POPUP
	    if (reset_extra_attr)
	    {
		reset_extra_attr = FALSE;
		wlv.extra_attr = 0;
	    }
#endif
	}

#if defined(FEAT_XIM) && defined(FEAT_GUI_GTK)
	// XIM don't send preedit_start and preedit_end, but they send
	// preedit_changed and commit.  Thus Vim can't set "im_is_active", use
	// im_is_preediting() here.
	if (p_imst == IM_ON_THE_SPOT
		&& xic != NULL
		&& lnum == wp->w_cursor.lnum
		&& (State & MODE_INSERT)
		&& !p_imdisable
		&& im_is_preediting()
		&& wlv.draw_state == WL_LINE)
	{
	    colnr_T tcol;

	    if (preedit_end_col == MAXCOL)
		getvcol(curwin, &(wp->w_cursor), &tcol, NULL, NULL);
	    else
		tcol = preedit_end_col;
	    if ((long)preedit_start_col <= wlv.vcol && wlv.vcol < (long)tcol)
	    {
		if (feedback_old_attr < 0)
		{
		    feedback_col = 0;
		    feedback_old_attr = wlv.char_attr;
		}
		wlv.char_attr = im_get_feedback_attr(feedback_col);
		if (wlv.char_attr < 0)
		    wlv.char_attr = feedback_old_attr;
		feedback_col++;
	    }
	    else if (feedback_old_attr >= 0)
	    {
		wlv.char_attr = feedback_old_attr;
		feedback_old_attr = -1;
		feedback_col = 0;
	    }
	}
#endif
	// Handle the case where we are in column 0 but not on the first
	// character of the line and the user wants us to show us a
	// special character (via 'listchars' option "precedes:<char>".
	if (lcs_prec_todo != NUL
		&& wp->w_p_list
		&& (wp->w_p_wrap ? (wp->w_skipcol > 0 && wlv.row == 0)
				 : wp->w_leftcol > 0)
#ifdef FEAT_DIFF
		&& wlv.filler_todo <= 0
#endif
		&& wlv.draw_state > WL_NR
		&& c != NUL)
	{
	    c = wp->w_lcs_chars.prec;
	    lcs_prec_todo = NUL;
	    if (has_mbyte && (*mb_char2cells)(mb_c) > 1)
	    {
		// Double-width character being overwritten by the "precedes"
		// character, need to fill up half the character.
		wlv.c_extra = MB_FILLER_CHAR;
		wlv.c_final = NUL;
		wlv.n_extra = 1;
		n_attr = 2;
		wlv.extra_attr =
				hl_combine_attr(wlv.win_attr, HL_ATTR(HLF_AT));
	    }
	    mb_c = c;
	    if (enc_utf8 && utf_char2len(c) > 1)
	    {
		mb_utf8 = TRUE;
		u8cc[0] = 0;
		c = 0xc0;
	    }
	    else
		mb_utf8 = FALSE;	// don't draw as UTF-8
	    if (!attr_pri)
	    {
		saved_attr3 = wlv.char_attr; // save current attr
		wlv.char_attr = hl_combine_attr(wlv.win_attr, HL_ATTR(HLF_AT));
		n_attr3 = 1;
	    }
	}

	// At end of the text line or just after the last character.
	if ((c == NUL
#if defined(LINE_ATTR)
		|| did_line_attr == 1
#endif
		) && wlv.eol_hl_off == 0)
	{
#ifdef FEAT_SEARCH_EXTRA
	    // flag to indicate whether prevcol equals startcol of search_hl or
	    // one of the matches
	    int prevcol_hl_flag = get_prevcol_hl_flag(wp, &screen_search_hl,
					      (long)(ptr - line) - (c == NUL));
#endif
	    // Invert at least one char, used for Visual and empty line or
	    // highlight match at end of line. If it's beyond the last
	    // char on the screen, just overwrite that one (tricky!)  Not
	    // needed when a '$' was displayed for 'list'.
	    if (wp->w_lcs_chars.eol == lcs_eol_one
		    && ((area_attr != 0 && wlv.vcol == wlv.fromcol
			    && (VIsual_mode != Ctrl_V
				|| lnum == VIsual.lnum
				|| lnum == curwin->w_cursor.lnum)
			    && c == NUL)
#ifdef FEAT_SEARCH_EXTRA
			// highlight 'hlsearch' match at end of line
			|| (prevcol_hl_flag
# ifdef FEAT_SYN_HL
			    && !(wp->w_p_cul && lnum == wp->w_cursor.lnum
				    && !(wp == curwin && VIsual_active))
# endif
# ifdef FEAT_DIFF
			    && wlv.diff_hlf == (hlf_T)0
# endif
# if defined(LINE_ATTR)
			    && did_line_attr <= 1
# endif
			   )
#endif
		       ))
	    {
		int n = 0;

#ifdef FEAT_RIGHTLEFT
		if (wp->w_p_rl)
		{
		    if (wlv.col < 0)
			n = 1;
		}
		else
#endif
		{
		    if (wlv.col >= wp->w_width)
			n = -1;
		}
		if (n != 0)
		{
		    // At the window boundary, highlight the last character
		    // instead (better than nothing).
		    wlv.off += n;
		    wlv.col += n;
		}
		else
		{
		    // Add a blank character to highlight.
		    ScreenLines[wlv.off] = ' ';
		    if (enc_utf8)
			ScreenLinesUC[wlv.off] = 0;
		}
#ifdef FEAT_SEARCH_EXTRA
		if (area_attr == 0)
		{
		    // Use attributes from match with highest priority among
		    // 'search_hl' and the match list.
		    get_search_match_hl(wp, &screen_search_hl,
					   (long)(ptr - line), &wlv.char_attr);
		}
#endif
		ScreenAttrs[wlv.off] = wlv.char_attr;
		ScreenCols[wlv.off] = MAXCOL;
#ifdef FEAT_RIGHTLEFT
		if (wp->w_p_rl)
		{
		    --wlv.col;
		    --wlv.off;
		}
		else
#endif
		{
		    ++wlv.col;
		    ++wlv.off;
		}
		++wlv.vcol;
		wlv.eol_hl_off = 1;
	    }
	}

	// At end of the text line.
	if (c == NUL)
	{
#ifdef FEAT_PROP_POPUP
	    if (text_prop_follows)
	    {
		// Put the pointer back to the NUL.
		--ptr;
		c = ' ';
	    }
	    else
#endif
	    {
		draw_screen_line(wp, &wlv);

		// Update w_cline_height and w_cline_folded if the cursor line
		// was updated (saves a call to plines() later).
		if (wp == curwin && lnum == curwin->w_cursor.lnum)
		{
		    curwin->w_cline_row = startrow;
		    curwin->w_cline_height = wlv.row - startrow;
#ifdef FEAT_FOLDING
		    curwin->w_cline_folded = FALSE;
#endif
		    curwin->w_valid |= (VALID_CHEIGHT|VALID_CROW);
		}
		break;
	    }
	}

	// Show "extends" character from 'listchars' if beyond the line end and
	// 'list' is set.
	if (wp->w_lcs_chars.ext != NUL
		&& wlv.draw_state == WL_LINE
		&& wp->w_p_list
		&& !wp->w_p_wrap
#ifdef FEAT_DIFF
		&& wlv.filler_todo <= 0
#endif
		&& (
#ifdef FEAT_RIGHTLEFT
		    wp->w_p_rl ? wlv.col == 0 :
#endif
		    wlv.col == wp->w_width - 1)
		&& (*ptr != NUL
		    || lcs_eol_one > 0
		    || (wlv.n_extra > 0 && (wlv.c_extra != NUL
						     || *wlv.p_extra != NUL))
#ifdef FEAT_PROP_POPUP
		    || text_prop_next <= last_textprop_text_idx
#endif
		   ))
	{
	    c = wp->w_lcs_chars.ext;
	    wlv.char_attr = hl_combine_attr(wlv.win_attr, HL_ATTR(HLF_AT));
	    mb_c = c;
	    if (enc_utf8 && utf_char2len(c) > 1)
	    {
		mb_utf8 = TRUE;
		u8cc[0] = 0;
		c = 0xc0;
	    }
	    else
		mb_utf8 = FALSE;
	}

#ifdef FEAT_SYN_HL
	// advance to the next 'colorcolumn'
	if (wlv.draw_color_col)
	    wlv.draw_color_col = advance_color_col(VCOL_HLC, &wlv.color_cols);

	// Highlight the cursor column if 'cursorcolumn' is set.  But don't
	// highlight the cursor position itself.
	// Also highlight the 'colorcolumn' if it is different than
	// 'cursorcolumn'
	// Also highlight the 'colorcolumn' if 'breakindent' and/or 'showbreak'
	// options are set
	vcol_save_attr = -1;
	if (((wlv.draw_state == WL_LINE
		    || wlv.draw_state == WL_BRI
		    || wlv.draw_state == WL_SBR)
		&& !lnum_in_visual_area
		&& search_attr == 0
		&& area_attr == 0)
# ifdef FEAT_DIFF
			&& wlv.filler_todo <= 0
# endif
		)
	{
	    if (wp->w_p_cuc && VCOL_HLC == (long)wp->w_virtcol
						 && lnum != wp->w_cursor.lnum)
	    {
		vcol_save_attr = wlv.char_attr;
		wlv.char_attr = hl_combine_attr(wlv.char_attr,
							     HL_ATTR(HLF_CUC));
	    }
	    else if (wlv.draw_color_col && VCOL_HLC == *wlv.color_cols)
	    {
		vcol_save_attr = wlv.char_attr;
		wlv.char_attr = hl_combine_attr(wlv.char_attr, HL_ATTR(HLF_MC));
	    }
	}
#endif

	if (wlv.draw_state == WL_LINE)
	    vcol_prev = wlv.vcol;

	// Store character to be displayed.
	// Skip characters that are left of the screen for 'nowrap'.
	if (wlv.draw_state < WL_LINE || skip_cells <= 0)
	{
	    // Store the character.
#if defined(FEAT_RIGHTLEFT)
	    if (has_mbyte && wp->w_p_rl && (*mb_char2cells)(mb_c) > 1)
	    {
		// A double-wide character is: put first half in left cell.
		--wlv.off;
		--wlv.col;
	    }
#endif
	    ScreenLines[wlv.off] = c;
	    if (enc_dbcs == DBCS_JPNU)
	    {
		if ((mb_c & 0xff00) == 0x8e00)
		    ScreenLines[wlv.off] = 0x8e;
		ScreenLines2[wlv.off] = mb_c & 0xff;
	    }
	    else if (enc_utf8)
	    {
		if (mb_utf8)
		{
		    int i;

		    ScreenLinesUC[wlv.off] = mb_c;
		    if ((c & 0xff) == 0)
			ScreenLines[wlv.off] = 0x80;   // avoid storing zero
		    for (i = 0; i < Screen_mco; ++i)
		    {
			ScreenLinesC[i][wlv.off] = u8cc[i];
			if (u8cc[i] == 0)
			    break;
		    }
		}
		else
		    ScreenLinesUC[wlv.off] = 0;
	    }
	    if (multi_attr)
	    {
		ScreenAttrs[wlv.off] = multi_attr;
		multi_attr = 0;
	    }
	    else
		ScreenAttrs[wlv.off] = wlv.char_attr;

	    if (wlv.draw_state > WL_NR
#ifdef FEAT_DIFF
		    && wlv.filler_todo <= 0
#endif
		    )
		ScreenCols[wlv.off] = wlv.vcol;
	    else
		ScreenCols[wlv.off] = -1;

	    if (has_mbyte && (*mb_char2cells)(mb_c) > 1)
	    {
		// Need to fill two screen columns.
		++wlv.off;
		++wlv.col;
		if (enc_utf8)
		    // UTF-8: Put a 0 in the second screen char.
		    ScreenLines[wlv.off] = 0;
		else
		    // DBCS: Put second byte in the second screen char.
		    ScreenLines[wlv.off] = mb_c & 0xff;

		if (wlv.draw_state > WL_NR
#ifdef FEAT_DIFF
			&& wlv.filler_todo <= 0
#endif
			)
		    ScreenCols[wlv.off] = ++wlv.vcol;
		else
		    ScreenCols[wlv.off] = -1;

		// When "wlv.tocol" is halfway a character, set it to the end
		// of the character, otherwise highlighting won't stop.
		if (wlv.tocol == wlv.vcol)
		    ++wlv.tocol;

#ifdef FEAT_RIGHTLEFT
		if (wp->w_p_rl)
		{
		    // now it's time to backup one cell
		    --wlv.off;
		    --wlv.col;
		}
#endif
	    }
#ifdef FEAT_RIGHTLEFT
	    if (wp->w_p_rl)
	    {
		--wlv.off;
		--wlv.col;
	    }
	    else
#endif
	    {
		++wlv.off;
		++wlv.col;
	    }
	}
#ifdef FEAT_CONCEAL
	else if (wp->w_p_cole > 0 && is_concealing)
	{
	    --skip_cells;
	    ++wlv.vcol_off_co;
	    if (wlv.n_extra > 0)
		wlv.vcol_off_co += wlv.n_extra;
	    if (wp->w_p_wrap)
	    {
		// Special voodoo required if 'wrap' is on.
		//
		// Advance the column indicator to force the line
		// drawing to wrap early. This will make the line
		// take up the same screen space when parts are concealed,
		// so that cursor line computations aren't messed up.
		//
		// To avoid the fictitious advance of 'wlv.col' causing
		// trailing junk to be written out of the screen line
		// we are building, 'boguscols' keeps track of the number
		// of bad columns we have advanced.
		if (wlv.n_extra > 0)
		{
		    wlv.vcol += wlv.n_extra;
# ifdef FEAT_RIGHTLEFT
		    if (wp->w_p_rl)
		    {
			wlv.col -= wlv.n_extra;
			wlv.boguscols -= wlv.n_extra;
		    }
		    else
# endif
		    {
			wlv.col += wlv.n_extra;
			wlv.boguscols += wlv.n_extra;
		    }
		    wlv.n_extra = 0;
		    n_attr = 0;
		}


		if (has_mbyte && (*mb_char2cells)(mb_c) > 1)
		{
		    // Need to fill two screen columns.
# ifdef FEAT_RIGHTLEFT
		    if (wp->w_p_rl)
		    {
			--wlv.boguscols;
			--wlv.col;
		    }
		    else
# endif
		    {
			++wlv.boguscols;
			++wlv.col;
		    }
		}

# ifdef FEAT_RIGHTLEFT
		if (wp->w_p_rl)
		{
		    --wlv.boguscols;
		    --wlv.col;
		}
		else
# endif
		{
		    ++wlv.boguscols;
		    ++wlv.col;
		}
	    }
	    else
	    {
		if (wlv.n_extra > 0)
		{
		    wlv.vcol += wlv.n_extra;
		    wlv.n_extra = 0;
		    n_attr = 0;
		}
	    }

	}
#endif // FEAT_CONCEAL
	else
	    --skip_cells;

	if (wlv.draw_state > WL_NR && skipped_cells > 0)
	{
	    wlv.vcol += skipped_cells;
	    skipped_cells = 0;
	}

	// Only advance the "wlv.vcol" when after the 'number' or
	// 'relativenumber' column.
	if (wlv.draw_state > WL_NR
#ifdef FEAT_DIFF
		&& wlv.filler_todo <= 0
#endif
		)
	    ++wlv.vcol;

#ifdef FEAT_SYN_HL
	if (vcol_save_attr >= 0)
	    wlv.char_attr = vcol_save_attr;
#endif

	// restore attributes after "precedes" in 'listchars'
	if (wlv.draw_state > WL_NR && n_attr3 > 0 && --n_attr3 == 0)
	    wlv.char_attr = saved_attr3;

	// restore attributes after last 'listchars' or 'number' char
	if (n_attr > 0 && wlv.draw_state == WL_LINE
				      && wlv.n_attr_skip == 0 && --n_attr == 0)
	    wlv.char_attr = saved_attr2;
	if (wlv.n_attr_skip > 0)
	    --wlv.n_attr_skip;

	// At end of screen line and there is more to come: Display the line
	// so far.  If there is no more to display it is caught above.
	if ((
#ifdef FEAT_RIGHTLEFT
	    wp->w_p_rl ? (wlv.col < 0) :
#endif
				    (wlv.col >= wp->w_width))
		&& (wlv.draw_state != WL_LINE
		    || *ptr != NUL
#ifdef FEAT_DIFF
		    || wlv.filler_todo > 0
#endif
#ifdef FEAT_PROP_POPUP
		    || text_prop_above || text_prop_follows
		    || text_prop_next <= last_textprop_text_idx
#endif
		    || (wp->w_p_list && wp->w_lcs_chars.eol != NUL
						&& wlv.p_extra != at_end_str)
		    || (wlv.n_extra != 0 && (wlv.c_extra != NUL
						      || *wlv.p_extra != NUL)))
		)
	{
#ifdef FEAT_CONCEAL
	    wlv.col -= wlv.boguscols;
	    wlv_screen_line(wp, &wlv, FALSE);
	    wlv.col += wlv.boguscols;
	    wlv.boguscols = 0;
	    wlv.vcol_off_co = 0;
#else
	    wlv_screen_line(wp, &wlv, FALSE);
#endif
	    ++wlv.row;
	    ++wlv.screen_row;

	    // When not wrapping and finished diff lines, or when displayed
	    // '$' and highlighting until last column, break here.
	    if (((!wp->w_p_wrap
#ifdef FEAT_DIFF
			&& wlv.filler_todo <= 0
#endif
#ifdef FEAT_PROP_POPUP
			&& !text_prop_above
#endif
		 ) || lcs_eol_one == -1)
#ifdef FEAT_PROP_POPUP
		    && !text_prop_follows
#endif
		       )
		break;
#ifdef FEAT_PROP_POPUP
	    if (!wp->w_p_wrap && text_prop_follows && !text_prop_above)
	    {
		// do not output more of the line, only the "below" prop
		ptr += STRLEN(ptr);
# ifdef FEAT_LINEBREAK
		wlv.dont_use_showbreak = TRUE;
# endif
	    }
#endif

	    // When the window is too narrow draw all "@" lines.
	    if (wlv.draw_state != WL_LINE
#ifdef FEAT_DIFF
		    && wlv.filler_todo <= 0
#endif
		    )
	    {
		win_draw_end(wp, '@', ' ', TRUE, wlv.row, wp->w_height, HLF_AT);
		draw_vsep_win(wp, wlv.row);
		wlv.row = endrow;
	    }

	    // When line got too long for screen break here.
	    if (wlv.row == endrow)
	    {
		++wlv.row;
		break;
	    }

	    if (screen_cur_row == wlv.screen_row - 1
#ifdef FEAT_DIFF
		     && wlv.filler_todo <= 0
#endif
#ifdef FEAT_PROP_POPUP
		     && !text_prop_above && !text_prop_follows
#endif
		     && wp->w_width == Columns)
	    {
		// Remember that the line wraps, used for modeless copy.
		LineWraps[wlv.screen_row - 1] = TRUE;

		// Special trick to make copy/paste of wrapped lines work with
		// xterm/screen: write an extra character beyond the end of
		// the line. This will work with all terminal types
		// (regardless of the xn,am settings).
		// Only do this on a fast tty.
		// Only do this if the cursor is on the current line
		// (something has been written in it).
		// Don't do this for the GUI.
		// Don't do this for double-width characters.
		// Don't do this for a window not at the right screen border.
		if (p_tf
#ifdef FEAT_GUI
			 && !gui.in_use
#endif
			 && !(has_mbyte
			     && ((*mb_off2cells)(LineOffset[wlv.screen_row],
				   LineOffset[wlv.screen_row] + screen_Columns)
									  == 2
				 || (*mb_off2cells)(
				     LineOffset[wlv.screen_row - 1]
							    + (int)Columns - 2,
				     LineOffset[wlv.screen_row]
						      + screen_Columns) == 2)))
		{
		    // First make sure we are at the end of the screen line,
		    // then output the same character again to let the
		    // terminal know about the wrap.  If the terminal doesn't
		    // auto-wrap, we overwrite the character.
		    if (screen_cur_col != wp->w_width)
			screen_char(LineOffset[wlv.screen_row - 1]
						       + (unsigned)Columns - 1,
				       wlv.screen_row - 1, (int)(Columns - 1));

		    // When there is a multi-byte character, just output a
		    // space to keep it simple.
		    if (has_mbyte && MB_BYTE2LEN(ScreenLines[LineOffset[
				     wlv.screen_row - 1] + (Columns - 1)]) > 1)
			out_char(' ');
		    else
			out_char(ScreenLines[LineOffset[wlv.screen_row - 1]
							    + (Columns - 1)]);
		    // force a redraw of the first char on the next line
		    ScreenAttrs[LineOffset[wlv.screen_row]] = (sattr_T)-1;
		    screen_start();	// don't know where cursor is now
		}
	    }

	    win_line_start(wp, &wlv, TRUE);

	    lcs_prec_todo = wp->w_lcs_chars.prec;
#ifdef FEAT_LINEBREAK
	    if (!wlv.dont_use_showbreak
# ifdef FEAT_DIFF
		    && wlv.filler_todo <= 0
# endif
	       )
		wlv.need_showbreak = TRUE;
#endif
#ifdef FEAT_DIFF
	    --wlv.filler_todo;
	    // When the filler lines are actually below the last line of the
	    // file, don't draw the line itself, break here.
	    if (wlv.filler_todo == 0 && wp->w_botfill)
		break;
#endif
	}

    }	// for every character in the line
#ifdef FEAT_PROP_POPUP
    vim_free(text_props);
    vim_free(text_prop_idxs);
    vim_free(p_extra_free2);
#endif

    vim_free(wlv.p_extra_free);
    vim_free(wlv.saved_p_extra_free);
    return wlv.row;
}
