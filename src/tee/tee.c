/*  vim:set ts=4 sw=4:
 *
 *	Copyright (c) 1996, Paul Slootman
 *
 *	Author: Paul Slootman
 *			(paul@wurtel.hobby.nl, paul@murphy.nl, paulS@toecompst.nl)
 *	Modifications for MSVC: Yasuhiro Matsumoto
 *
 *	This source code is released into the public domain. It is provided on an
 *	as-is basis and no responsibility is accepted for its failure to perform
 *	as expected. It is worth at least as much as you paid for it!
 *
 * tee.c - pipe fitting
 *
 * tee reads stdin, and writes what it reads to each of the specified
 * files. The primary reason of existence for this version is a quick
 * and dirty implementation to distribute with Vim, to make one of the
 * most useful features of Vim possible on OS/2: quickfix.
 *
 * Of course, not using tee but instead redirecting make's output directly
 * into a temp file and then processing that is possible, but if we have a
 * system capable of correctly piping (unlike DOS, for example), why not
 * use it as well as possible? This tee should also work on other systems,
 * but it's not been tested there, only on OS/2.
 *
 * tee is also available in the GNU shellutils package, which is available
 * precompiled for OS/2. That one probably works better.
 */

#ifndef _MSC_VER
# include <unistd.h>
#endif
#include <malloc.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <fcntl.h>

#ifdef _WIN32
# define sysconf(x) -1
#endif

void usage(void)
{
	fprintf(stderr,
"tee usage:\n\
\ttee [-a] file ... file_n\n\
\n\
\t-a\tappend to files instead of truncating\n\
\nTee reads its input, and writes to each of the specified files,\n\
as well as to the standard output.\n\
\n\
This version supplied with Vim 4.2 to make ':make' possible.\n\
For a more complete and stable version, consider getting\n\
[a port of] the GNU shellutils package.\n\
");
}

/*
 * fread only returns when count is read or at EOF.
 * We could use fgets, but I want to be able to handle binary blubber.
 */

int
myfread(char *buf, int elsize /*ignored*/, int max, FILE *fp)
{
	int	c;
	int	n = 0;

	while ((n < max) && ((c = getchar()) != EOF))
	{
		*(buf++) = c;
		n++;
		if (c == '\n' || c == '\r')
			break;
	}
	return n;
}


int
main(int argc, char *argv[])
{
	int	append = 0;
	size_t	numfiles;
	int	maxfiles;
	FILE	**filepointers;
	int	i;
	char	buf[BUFSIZ];
	int	n;
	int	optind = 1;

	for (i = 1; i < argc; i++)
	{
		if (argv[i][0] != '-')
			break;
		if (!strcmp(argv[i], "-a"))
			append++;
		else
			usage();
		optind++;
	}

	numfiles = argc - optind;

	if (numfiles == 0)
	{
		fprintf(stderr, "doesn't make much sense using tee without any file name arguments...\n");
		usage();
		exit(2);
	}

	maxfiles = sysconf(_SC_OPEN_MAX);	/* or fill in 10 or so */
	if (maxfiles < 0)
		maxfiles = 10;
	if (numfiles + 3 > maxfiles)	/* +3 accounts for stdin, out, err */
	{
		fprintf(stderr, "Sorry, there is a limit of max %d files.\n", maxfiles - 3);
		exit(1);
	}
	filepointers = calloc(numfiles, sizeof(FILE *));
	if (filepointers == NULL)
	{
		fprintf(stderr, "Error allocating memory for %ld files\n",
															   (long)numfiles);
		exit(1);
	}
	for (i = 0; i < numfiles; i++)
	{
		filepointers[i] = fopen(argv[i+optind], append ? "ab" : "wb");
		if (filepointers[i] == NULL)
		{
			fprintf(stderr, "Can't open \"%s\"\n", argv[i+optind]);
			exit(1);
		}
	}
#ifdef _WIN32
	setmode(fileno(stdin),  O_BINARY);
	fflush(stdout);	/* needed for _fsetmode(stdout) */
	setmode(fileno(stdout),  O_BINARY);
#endif

	while ((n = myfread(buf, sizeof(char), sizeof(buf), stdin)) > 0)
	{
		fwrite(buf, sizeof(char), n, stdout);
		fflush(stdout);
		for (i = 0; i < numfiles; i++)
		{
			if (filepointers[i] &&
			     fwrite(buf, sizeof(char), n, filepointers[i]) != n)
			{
				fprintf(stderr, "Error writing to file \"%s\"\n", argv[i+optind]);
				fclose(filepointers[i]);
				filepointers[i] = NULL;
			}
		}
	}
	for (i = 0; i < numfiles; i++)
	{
		if (filepointers[i])
			fclose(filepointers[i]);
	}

	exit(0);
}
