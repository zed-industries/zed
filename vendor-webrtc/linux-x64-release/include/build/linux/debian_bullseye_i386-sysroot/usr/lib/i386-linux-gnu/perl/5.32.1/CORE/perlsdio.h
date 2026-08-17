/*    perlsdio.h
 *
 *    Copyright (C) 1996, 1997, 1998, 1999, 2000, 2001,
 *    2002, 2003, 2006, 2007, 2008 by Larry Wall and others
 *
 *    You may distribute under the terms of either the GNU General Public
 *    License or the Artistic License, as specified in the README file.
 *
 */

/* Shouldn't be possible to get here, but if we did ... */

#ifdef PERLIO_IS_STDIO

#  error "stdio is no longer supported as the default base layer -- use perlio."

#endif /* PERLIO_IS_STDIO */

/*
 * ex: set ts=8 sts=4 sw=4 et:
 */
