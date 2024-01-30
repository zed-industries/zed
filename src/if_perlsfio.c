/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */
/*
 * if_perlsfio.c: Special I/O functions for Perl interface.
 */

#define _memory_h	// avoid memset redeclaration
#define IN_PERL_FILE	// don't include if_perl.pro from prot.h

#include "vim.h"

#if defined(USE_SFIO) || defined(PROTO)

#ifndef USE_SFIO	// just generating prototypes
# define Sfio_t int
# define Sfdisc_t int
#endif

#define NIL(type)	((type)0)

    static int
sfvimwrite(
    Sfio_t	    *f,		// stream involved
    char	    *buf,	// buffer to read from
    int		    n,		// number of bytes to write
    Sfdisc_t	    *disc)	// discipline
{
    char_u *str;

    str = vim_strnsave((char_u *)buf, n);
    if (str == NULL)
	return 0;
    msg_split((char *)str);
    vim_free(str);

    return n;
}

/*
 * sfdcnewnvi --
 *  Create Vim discipline
 */
    Sfdisc_t *
sfdcnewvim(void)
{
    Sfdisc_t	*disc;

    disc = ALLOC_ONE(Sfdisc_t);
    if (disc == NULL)
	return NULL;

    disc->readf = (Sfread_f)NULL;
    disc->writef = sfvimwrite;
    disc->seekf = (Sfseek_f)NULL;
    disc->exceptf = (Sfexcept_f)NULL;

    return disc;
}

#endif // USE_SFIO
