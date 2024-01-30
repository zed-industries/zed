/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved		by Bram Moolenaar
 * VMS port			by Henk Elbers
 * VMS deport			by Zoltan Arpadffy
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

#include	"vim.h"

// define _generic_64 for use in time functions
#if !defined(VAX) && !defined(PROTO)
#   include <gen64def.h>
#else
// based on Alpha's gen64def.h; the file is absent on VAX
typedef struct _generic_64 {
#   pragma __nomember_alignment
    __union  {				// You can treat me as...
	// long long is not available on VAXen
	// unsigned __int64 gen64$q_quadword; ...a single 64-bit value, or

	unsigned int gen64$l_longword [2]; // ...two 32-bit values, or
	unsigned short int gen64$w_word [4]; // ...four 16-bit values
    } gen64$r_quad_overlay;
} GENERIC_64;
#endif

typedef struct
{
    char	class;
    char	type;
    short	width;
    union
    {
	struct
	{
	    char	_basic[3];
	    char	length;
	}	y;
	int	basic;
    }	x;
    int		extended;
}	TT_MODE;

typedef struct
{
    short	buflen;
    short	itemcode;
    char	*bufadrs;
    int		*retlen;
}	ITEM;

typedef struct
{
    ITEM	equ;
    int		nul;
}	ITMLST1;

typedef struct
{
    ITEM	index;
    ITEM	string;
    int	nul;
}	ITMLST2;

static TT_MODE	orgmode;
static short	iochan;			// TTY I/O channel
static short	iosb[4];		// IO status block

static int vms_match_num = 0;
static int vms_match_free = 0;
static char_u **vms_fmatch = NULL;
static char *Fspec_Rms;		       // rms file spec, passed implicitly between routines



static TT_MODE	get_tty(void);
static void	set_tty(int row, int col);

#define EXPL_ALLOC_INC 64

#define EQN(S1,S2,LN) (strncmp(S1,S2,LN) == 0)
#define SKIP_FOLLOWING_SLASHES(Str) do { while (Str[1] == '/') ++Str; } while (0)


/*
 *	vul_desc	vult een descriptor met een string en de lengte
 *			hier van.
 */
    static void
vul_desc(DESC *des, char *str)
{
    des->dsc$b_dtype = DSC$K_DTYPE_T;
    des->dsc$b_class = DSC$K_CLASS_S;
    des->dsc$a_pointer = str;
    des->dsc$w_length = str ? strlen(str) : 0;
}

/*
 *	vul_item	vult een item met een aantal waarden
 */
    static void
vul_item(ITEM *itm, short len, short cod, char *adr, int *ret)
{
    itm->buflen   = len;
    itm->itemcode = cod;
    itm->bufadrs  = adr;
    itm->retlen   = ret;
}

    void
mch_settmode(tmode_T tmode)
{
    int	status;

    if ( tmode == TMODE_RAW )
	set_tty(0, 0);
    else
    {
	switch (orgmode.width)
	{
	    case 132:	OUT_STR_NF((char_u *)"\033[?3h\033>");	break;
	    case 80:	OUT_STR_NF((char_u *)"\033[?3l\033>");	break;
	    default:	break;
	}
	out_flush();
	status = sys$qiow(0, iochan, IO$_SETMODE, iosb, 0, 0,
					  &orgmode, sizeof(TT_MODE), 0,0,0,0);
	if (status!=SS$_NORMAL || (iosb[0]&0xFFFF)!=SS$_NORMAL)
	    return;
	(void)sys$dassgn(iochan);
	iochan = 0;
    }
}

    static void
set_tty(int row, int col)
{
    int		    status;
    TT_MODE	    newmode;		// New TTY mode bits
    static short    first_time = TRUE;

    if (first_time)
    {
	orgmode = get_tty();
	first_time = FALSE;
    }
    newmode = get_tty();
    if (col)
	newmode.width		 = col;
    if (row)
	newmode.x.y.length       = row;
    newmode.x.basic		|= (TT$M_NOECHO | TT$M_HOSTSYNC);
    newmode.x.basic		&= ~TT$M_TTSYNC;
    newmode.extended		|= TT2$M_PASTHRU;
    status = sys$qiow(0, iochan, IO$_SETMODE, iosb, 0, 0,
			  &newmode, sizeof(newmode), 0, 0, 0, 0);
    if (status!=SS$_NORMAL || (iosb[0]&0xFFFF)!=SS$_NORMAL)
	return;
}

    static TT_MODE
get_tty(void)
{

    static $DESCRIPTOR(odsc,"SYS$OUTPUT");   // output descriptor

    int		status;
    TT_MODE	tt_mode;

    if (!iochan)
	status = sys$assign(&odsc,&iochan,0,0);

    status = sys$qiow(0, iochan, IO$_SENSEMODE, iosb, 0, 0,
		      &tt_mode, sizeof(tt_mode), 0, 0, 0, 0);
    if (status != SS$_NORMAL || (iosb[0] & 0xFFFF) != SS$_NORMAL)
    {
	tt_mode.width		= 0;
	tt_mode.type		= 0;
	tt_mode.class		= 0;
	tt_mode.x.basic		= 0;
	tt_mode.x.y.length	= 0;
	tt_mode.extended	= 0;
    }
    return(tt_mode);
}

/*
 * Get the current window size in Rows and Columns.
 */
    int
mch_get_shellsize(void)
{
    TT_MODE	tmode;

    tmode = get_tty();			// get size from VMS
    Columns = tmode.width;
    Rows = tmode.x.y.length;
    return OK;
}

/*
 * Try to set the window size to Rows and new_Columns.
 */
    void
mch_set_shellsize(void)
{
    set_tty(Rows, Columns);
    switch (Columns)
    {
	case 132:	OUT_STR_NF((char_u *)"\033[?3h\033>");	break;
	case 80:	OUT_STR_NF((char_u *)"\033[?3l\033>");	break;
	default:	break;
    }
    out_flush();
    screen_start();
}

    char_u *
mch_getenv(char_u *lognam)
{
    DESC		d_file_dev, d_lognam  ;
    static char		buffer[LNM$C_NAMLENGTH+1];
    char_u		*cp = NULL;
    unsigned long	attrib;
    int			lengte = 0, dum = 0, idx = 0;
    ITMLST2		itmlst;
    char		*sbuf = NULL;

    vul_desc(&d_lognam, (char *)lognam);
    vul_desc(&d_file_dev, "LNM$FILE_DEV");
    attrib = LNM$M_CASE_BLIND;
    vul_item(&itmlst.index, sizeof(int), LNM$_INDEX, (char *)&idx, &dum);
    vul_item(&itmlst.string, LNM$C_NAMLENGTH, LNM$_STRING, buffer, &lengte);
    itmlst.nul	= 0;
    if (sys$trnlnm(&attrib, &d_file_dev, &d_lognam, NULL,&itmlst) == SS$_NORMAL)
    {
	buffer[lengte] = '\0';
	if (cp = alloc(lengte + 1))
	    strcpy((char *)cp, buffer);
	return(cp);
    }
    else if ((sbuf = getenv((char *)lognam)))
    {
	lengte = strlen(sbuf) + 1;
	cp = alloc(lengte);
	if (cp)
	    strcpy((char *)cp, sbuf);
	return cp;
    }
    else
	return(NULL);
}

/*
 *	mch_setenv	VMS version of setenv()
 */
    int
mch_setenv(char *var, char *value, int x)
{
    int		res, dum;
    long	attrib = 0L;
    char	acmode = PSL$C_SUPER;	// needs SYSNAM privilege
    DESC	tabnam, lognam;
    ITMLST1	itmlst;

    vul_desc(&tabnam, "LNM$JOB");
    vul_desc(&lognam, var);
    vul_item(&itmlst.equ, value ? strlen(value) : 0, value ? LNM$_STRING : 0,
	    value, &dum);
    itmlst.nul	= 0;
    res = sys$crelnm(&attrib, &tabnam, &lognam, &acmode, &itmlst);
    return((res == 1) ? 0 : -1);
}

    int
vms_sys(char *cmd, char *out, char *inp)
{
    DESC	cdsc, odsc, idsc;
    long	status;

    if (cmd)
	vul_desc(&cdsc, cmd);
    if (out)
	vul_desc(&odsc, out);
    if (inp)
	vul_desc(&idsc, inp);

    lib$spawn(cmd ? &cdsc : NULL,		// command string
	      inp ? &idsc : NULL,		// input file
	      out ? &odsc : NULL,		// output file
	      0, 0, 0, &status, 0, 0, 0, 0, 0, 0);
    return status;
}

/*
 * Convert string to lowercase - most often filename
 */
    char *
vms_tolower( char *name )
{
    int i,nlen = strlen(name);
    for (i = 0; i < nlen; i++)
	name[i] = TOLOWER_ASC(name[i]);
    return name;
}

/*
 * Convert VMS system() or lib$spawn() return code to Unix-like exit value.
 */
    int
vms_sys_status(int status)
{
    if (status != SS$_NORMAL && (status & STS$M_SUCCESS) == 0)
	return status;		// Command failed.
    return 0;
}

/*
 * vms_read()
 * function for low level char input
 *
 * Returns: input length
 */
    int
vms_read(char *inbuf, size_t nbytes)
{
    int		status, function, len;
    TT_MODE	tt_mode;
    ITEM	itmlst[2];     // terminates on everything
    static long trm_mask[8] = {-1, -1, -1, -1, -1, -1, -1, -1};

    // whatever happened earlier we need an iochan here
    if (!iochan)
	tt_mode = get_tty();

    // important: clean the inbuf
    memset(inbuf, 0, nbytes);

    // set up the itemlist for the first read
    vul_item(&itmlst[0], 0, TRM$_MODIFIERS,
	 (char *)( TRM$M_TM_NOECHO  | TRM$M_TM_NOEDIT	 |
		   TRM$M_TM_NOFILTR | TRM$M_TM_TRMNOECHO |
		   TRM$M_TM_NORECALL) , 0);
    vul_item(&itmlst[1], sizeof(trm_mask), TRM$_TERM, (char *)&trm_mask, 0);

    // wait forever for a char
    function = (IO$_READLBLK | IO$M_EXTEND);
    status = sys$qiow(0, iochan, function, &iosb, 0, 0,
			 inbuf, nbytes-1, 0, 0, &itmlst, sizeof(itmlst));
    len = strlen(inbuf); // how many chars we got?

    // read immediately the rest in the IO queue
    function = (IO$_READLBLK | IO$M_TIMED | IO$M_ESCAPE | IO$M_NOECHO | IO$M_NOFILTR);
    status = sys$qiow(0, iochan, function, &iosb, 0, 0,
			 inbuf+len, nbytes-1-len, 0, 0, 0, 0);

    len = strlen(inbuf); // return the total length

    return len;
}

/*
 * vms_wproc() is called for each matching filename by decc$to_vms().
 * We want to save each match for later retrieval.
 *
 * Returns:  1 - continue finding matches
 *	     0 - stop trying to find any further matches
 */
    static int
vms_wproc(char *name, int val)
{
    int i;
    static int vms_match_alloced = 0;

    if (val == DECC$K_FOREIGN ) // foreign non VMS files are not counting
	return 1;

    // accept all DECC$K_FILE and DECC$K_DIRECTORY
    if (vms_match_num == 0)
    {
	// first time through, setup some things
	if (NULL == vms_fmatch)
	{
	    vms_fmatch = ALLOC_MULT(char_u *, EXPL_ALLOC_INC);
	    if (!vms_fmatch)
		return 0;
	    vms_match_alloced = EXPL_ALLOC_INC;
	    vms_match_free = EXPL_ALLOC_INC;
	}
	else
	{
	    // re-use existing space
	    vms_match_free = vms_match_alloced;
	}
    }

    // make matches look uniform
    vms_remove_version(name);
    name=vms_tolower(name);

    // if name already exists, don't add it
    for (i = 0; i<vms_match_num; i++)
    {
	if (0 == STRCMP((char_u *)name,vms_fmatch[i]))
	    return 1;
    }
    if (--vms_match_free == 0)
    {
	char_u **old_vms_fmatch = vms_fmatch;

	// add more space to store matches
	vms_match_alloced += EXPL_ALLOC_INC;
	vms_fmatch = vim_realloc(old_vms_fmatch,
		sizeof(char **) * vms_match_alloced);
	if (!vms_fmatch)
	{
	    vim_free(old_vms_fmatch);
	    return 0;
	}
	vms_match_free = EXPL_ALLOC_INC;
    }
    vms_fmatch[vms_match_num] = vim_strsave((char_u *)name);

    ++vms_match_num;
    return 1;
}

/*
 *	mch_expand_wildcards	this code does wild-card pattern
 *				matching NOT using the shell
 *
 *	return OK for success, FAIL for error (you may lose some
 *	memory) and put an error message in *file.
 *
 *	num_pat	   number of input patterns
 *	pat	   array of pointers to input patterns
 *	num_file   pointer to number of matched file names
 *	file	   pointer to array of pointers to matched file names
 *
 */
    int
mch_expand_wildcards(int num_pat, char_u **pat, int *num_file, char_u ***file, int flags)
{
    int		i, cnt = 0;
    char_u	buf[MAXPATHL];
    char       *result;
    int		dir;
    int files_alloced, files_free;

    *num_file = 0;			// default: no files found
    files_alloced = EXPL_ALLOC_INC;
    files_free = EXPL_ALLOC_INC;
    *file = ALLOC_MULT(char_u *, files_alloced);
    if (*file == NULL)
    {
	*num_file = 0;
	return FAIL;
    }
    for (i = 0; i < num_pat; i++)
    {
	// expand environment var or home dir
	if (vim_strchr(pat[i],'$') || vim_strchr(pat[i],'~'))
	    expand_env(pat[i],buf,MAXPATHL);
	else
	    STRCPY(buf,pat[i]);

	vms_match_num = 0; // reset collection counter
	result = decc$translate_vms(vms_fixfilename(buf));
	if ( (int) result == 0 || (int) result == -1  )
	{
	    cnt = 0;
	}
	else
	{
	    cnt = decc$to_vms(result, vms_wproc, 1 /*allow wild*/ , (flags & EW_DIR ? 0:1 ) /*allow directory*/) ;
	}
	if (cnt > 0)
	    cnt = vms_match_num;

	if (cnt < 1)
	    continue;

	for (i = 0; i < cnt; i++)
	{
	    // files should exist if expanding interactively
	    if (!(flags & EW_NOTFOUND) && mch_getperm(vms_fmatch[i]) < 0)
		continue;

	    // do not include directories
	    dir = (mch_isdir(vms_fmatch[i]));
	    if (( dir && !(flags & EW_DIR)) || (!dir && !(flags & EW_FILE)))
		continue;

	    // Skip files that are not executable if we check for that.
	    if (!dir && (flags & EW_EXEC)
		 && !mch_can_exe(vms_fmatch[i], NULL, !(flags & EW_SHELLCMD)))
		continue;

	    // allocate memory for pointers
	    if (--files_free < 1)
	    {
		char_u **old_file = *file;

		files_alloced += EXPL_ALLOC_INC;
		*file = vim_realloc(old_file, sizeof(char_u **) * files_alloced);
		if (*file == NULL)
		{
		    vim_free(old_file);
		    *file = (char_u **)"";
		    *num_file = 0;
		    return(FAIL);
		}
		files_free = EXPL_ALLOC_INC;
	    }

	    (*file)[*num_file++] = vms_fmatch[i];
	}
    }
    return OK;
}

    int
mch_expandpath(garray_T *gap, char_u *path, int flags)
{
    int		i,cnt = 0;
    char       *result;

    vms_match_num = 0;
    // the result from the decc$translate_vms needs to be handled
    // otherwise it might create ACCVIO error in decc$to_vms
    result = decc$translate_vms(vms_fixfilename(path));
    if ( (int) result == 0 || (int) result == -1  )
    {
	cnt = 0;
    }
    else
    {
	cnt = decc$to_vms(result, vms_wproc, 1 /*allow_wild*/, (flags & EW_DIR ? 0:1 ) /*allow directory*/);
    }
    if (cnt > 0)
	cnt = vms_match_num;
    for (i = 0; i < cnt; i++)
    {
	if (mch_getperm(vms_fmatch[i]) >= 0) // add existing file
	    addfile(gap, vms_fmatch[i], flags);
    }
    return cnt;
}

/*
 * attempt to translate a mixed unix-vms file specification to pure vms
 */
    static void
vms_unix_mixed_filespec(char *in, char *out)
{
    char *lastcolon;
    char *end_of_dir;
    char ch;
    int len;
    char *out_str=out;

    // copy vms filename portion up to last colon
    // (node and/or disk)
    lastcolon = strrchr(in, ':');   // find last colon
    if (lastcolon != NULL)
    {
	len = lastcolon - in + 1;
	strncpy(out, in, len);
	out += len;
	in += len;
    }

    end_of_dir = NULL;	// default: no directory

    // start of directory portion
    ch = *in;
    if ((ch == '[') || (ch == '/') || (ch == '<')) // start of directory(s) ?
    {
	ch = '[';
	SKIP_FOLLOWING_SLASHES(in);
    }
    else if (EQN(in, "../", 3)) // Unix parent directory?
    {
	*out++ = '[';
	*out++ = '-';
	end_of_dir = out;
	ch = '.';
	in += 2;
	SKIP_FOLLOWING_SLASHES(in);
    }
    else
    {		    // not a special character
	while (EQN(in, "./", 2))	// Ignore Unix "current dir"
	{
	    in += 2;
	    SKIP_FOLLOWING_SLASHES(in);
    }
    if (strchr(in, '/') == NULL)  // any more Unix directories ?
    {
	strcpy(out, in);	// No - get rest of the spec
	return;
    }
    else
    {
	*out++ = '[';	    // Yes, denote a Vms subdirectory
	ch = '.';
	--in;
	}
    }

    // if we get here, there is a directory part of the filename

    // initialize output file spec
    *out++ = ch;
    ++in;

    while (*in != '\0')
    {
	ch = *in;
	if ((ch == ']') || (ch == '/') || (ch == '>') )	// end of (sub)directory ?
	{
	    end_of_dir = out;
	    ch = '.';
	    SKIP_FOLLOWING_SLASHES(in);
	    }
	else if (EQN(in, "../", 3))	// Unix parent directory?
	{
	    *out++ = '-';
	    end_of_dir = out;
	    ch = '.';
	    in += 2;
	    SKIP_FOLLOWING_SLASHES(in);
	    }
	else
	{
	    while (EQN(in, "./", 2))  // Ignore Unix "current dir"
	    {
		end_of_dir = out;
		in += 2;
		SKIP_FOLLOWING_SLASHES(in);
		ch = *in;
	    }
	}

    // Place next character into output file spec
	*out++ = ch;
	++in;
    }

    *out = '\0';    // Terminate output file spec

    if (end_of_dir != NULL) // Terminate directory portion
	*end_of_dir = ']';
}

/*
 * for decc$to_vms in vms_fixfilename
 */
    static int
vms_fspec_proc(char *fil, int val)
{
    strcpy(Fspec_Rms,fil);
    return(1);
}

/*
 * change unix and mixed filenames to VMS
 */
    void *
vms_fixfilename(void *instring)
{
    static char		*buf = NULL;
    static size_t	buflen = 0;
    size_t		len;

    // get a big-enough buffer
    len = strlen(instring) + 1;
    if (len > buflen)
    {
	buflen = len + 128;
	buf = vim_realloc(buf, buflen * sizeof(char));
    }

#ifdef DEBUG
     char		 *tmpbuf = NULL;
     tmpbuf = ALLOC_MULT(char, buflen);
     strcpy(tmpbuf, instring);
#endif

    Fspec_Rms = buf;				// for decc$to_vms

    if (strchr(instring,'/') == NULL)
	// It is already a VMS file spec
	strcpy(buf, instring);
    else if (strchr(instring,'"') == NULL)	// password in the path?
    {
	// Seems it is a regular file, let guess that it is pure Unix fspec
	if ( (strchr(instring,'[') == NULL) && (strchr(instring,'<') == NULL) &&
	     (strchr(instring,']') == NULL) && (strchr(instring,'>') == NULL) &&
	     (strchr(instring,':') == NULL) )
	{
	    // It must be a truly unix fspec
	    decc$to_vms(instring, vms_fspec_proc, 0, 0);
	}
	else
	{
	    // It is a mixed fspec
	    vms_unix_mixed_filespec(instring, buf);
	}
    }
    else
	// we have a password in the path
	// decc$ functions can not handle
	// this is our only hope to resolv
	vms_unix_mixed_filespec(instring, buf);

    return buf;
}

/*
 * Remove version number from file name
 * we need it in some special cases as:
 * creating swap file name and writing new file
 */
    void
vms_remove_version(void * fname)
{
    char_u	*cp;
    char_u	*fp;

    if ((cp = vim_strchr( fname, ';')) != NULL) // remove version
	*cp = '\0';
    else if ((cp = vim_strrchr( fname, '.')) != NULL )
    {
	if      ((fp = vim_strrchr( fname, ']')) != NULL )
	    {;}
	else if ((fp = vim_strrchr( fname, '>')) != NULL )
	    {;}
	else
	    fp = fname;

	while ( *fp != '\0' && fp < cp )
	    if ( *fp++ == '.' )
		*cp = '\0';
    }
    return ;
}

struct typeahead_st {
    unsigned short numchars;
    unsigned char  firstchar;
    unsigned char  reserved0;
    unsigned long  reserved1;
} typeahead;

/*
 * Wait "msec" msec until a character is available from file descriptor "fd".
 * "msec" == 0 will check for characters once.
 * "msec" == -1 will block until a character is available.
 */
    int
RealWaitForChar(
    int		fd UNUSED, // always read from iochan
    long	msec,
    int		*check_for_gpm UNUSED,
    int		*interrupted)
{
    int status;
    struct _generic_64 time_curr;
    struct _generic_64 time_diff;
    struct _generic_64 time_out;
    unsigned int convert_operation = LIB$K_DELTA_SECONDS_F;
    float sec =(float) msec/1000;

    // make sure the iochan is set
    if (!iochan)
	get_tty();

    if (sec > 0)
    {
	// time-out specified; convert it to absolute time
	// sec>0 requirement of lib$cvtf_to_internal_time()

	// get current time (number of 100ns ticks since the VMS Epoch)
	status = sys$gettim(&time_curr);
	if (status != SS$_NORMAL)
	    return 0; // error
	// construct the delta time
#if __G_FLOAT==0
# ifndef VAX
	// IEEE is default on IA64, but can be used on Alpha too - but not on VAX
	status = lib$cvts_to_internal_time(
		&convert_operation, &sec, &time_diff);
# endif
#else   // default on Alpha and VAX
	status = lib$cvtf_to_internal_time(
		&convert_operation, &sec, &time_diff);
#endif
	if (status != LIB$_NORMAL)
	    return 0; // error
	// add them up
	status = lib$add_times(
		&time_curr,
		&time_diff,
		&time_out);
	if (status != LIB$_NORMAL)
	    return 0; // error
    }

    while (TRUE)
    {
	// select()
	status = sys$qiow(0, iochan, IO$_SENSEMODE | IO$M_TYPEAHDCNT, iosb,
		0, 0, &typeahead, 8, 0, 0, 0, 0);
	if (status != SS$_NORMAL || (iosb[0] & 0xFFFF) != SS$_NORMAL)
	    return 0; // error

	if (typeahead.numchars)
	    return 1; // ready to read

	// there's nothing to read; what now?
	if (msec == 0)
	{
	    // immediate time-out; return impatiently
	    return 0;
	}
	else if (msec < 0)
	{
	    // no time-out; wait on indefinitely
	    return 1; // fakeout to force a wait in vms_read()
	}
	else
	{
	    // time-out needs to be checked
	    status = sys$gettim(&time_curr);
	    if (status != SS$_NORMAL)
		return 0; // error

	    status = lib$sub_times(
		    &time_out,
		    &time_curr,
		    &time_diff);
	    if (status != LIB$_NORMAL)
		return 0; // error, incl. time_diff < 0 (i.e. time-out)

	    // otherwise wait some more
	}
    }
}
