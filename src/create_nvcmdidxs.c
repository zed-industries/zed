/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar et al.
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

/*
 * create_nvcmdidxs.c: helper program for `make nvcmdidxs`
 *
 * This outputs the list of command characters from the nv_cmds table in
 * decimal form, one per line.
 */

#include "vim.h"

// Declare nv_cmds[].
#include "nv_cmds.h"

#include <stdio.h>

int main(void)
{
    size_t i;

    for (i = 0; i < NV_CMDS_SIZE; i++)
    {
	int cmdchar = nv_cmds[i];

	// Special keys are negative, use the negated value for sorting.
	if (cmdchar < 0)
	    cmdchar = -cmdchar;
	printf("%d\n", cmdchar);
    }
    return 0;
}
