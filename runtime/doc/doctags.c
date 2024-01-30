/* vim:set ts=4 sw=4:
 *
 * This program makes a tags file for help text.
 *
 * Usage: doctags *.txt ... >tags
 *
 * A tag in this context is an identifier between stars, e.g. *c_files*
 */

#include <stdio.h>
#include <string.h>
#include <ctype.h>
#include <stdlib.h>

#define LINELEN 200

	int
main(int argc, char **argv)
{
	char	line[LINELEN];
	char	*p1, *p2;
	char	*p;
	FILE	*fd;
	int		len;
	int		in_example;

	if (argc <= 1)
	{
		fprintf(stderr, "Usage: doctags docfile ... >tags\n");
		exit(1);
	}
	printf("help-tags\ttags\t1\n");
	while (--argc > 0)
	{
		++argv;
		fd = fopen(argv[0], "r");
		if (fd == NULL)
		{
			fprintf(stderr, "Unable to open %s for reading\n", argv[0]);
			continue;
		}
		in_example = 0;
		while (fgets(line, LINELEN, fd) != NULL)
		{
			if (in_example)
			{
				// skip over example; non-blank in first column ends example
				if (strchr(" \t\n\r", line[0]) != NULL)
					continue;
				in_example = 0;
			}
			p1 = strchr(line, '*');				// find first '*'
			while (p1 != NULL)
			{
				p2 = strchr(p1 + 1, '*');		// find second '*'
				if (p2 != NULL && p2 > p1 + 1)	// skip "*" and "**"
				{
					for (p = p1 + 1; p < p2; ++p)
						if (*p == ' ' || *p == '\t' || *p == '|')
							break;
					// Only accept a *tag* when it consists of valid
					// characters, there is white space before it and is
					// followed by a white character or end-of-line.
					if (p == p2
							&& (p1 == line || p1[-1] == ' ' || p1[-1] == '\t')
								&& (strchr(" \t\n\r", p[1]) != NULL
									|| p[1] == '\0'))
					{
						*p2 = '\0';
						++p1;
						printf("%s\t%s\t/*", p1, argv[0]);
						while (*p1)
						{
							// insert backslash before '\\' and '/'
							if (*p1 == '\\' || *p1 == '/')
								putchar('\\');
							putchar(*p1);
							++p1;
						}
						printf("*\n");
						p2 = strchr(p2 + 1, '*');		// find next '*'
					}
				}
				p1 = p2;
			}
			len = strlen(line);
			if ((len == 2 && strcmp(&line[len - 2], ">\n") == 0)
					|| (len >= 3 && strcmp(&line[len - 3], " >\n") == 0))
				in_example = 1;
		}
		fclose(fd);
	}
	return 0;
}
