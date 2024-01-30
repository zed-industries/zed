# Makefile for xxd on the Amiga, using Aztec/Manx C 5.0 or later
#

#>>>>> choose between debugging (-bs) or optimizing (-so)
OPTIONS = -so
#OPTIONS = -bs

#>>>>>> choose -g for debugging
LN_DEBUG =
#LN_DEBUG = -g

CFLAGS = $(OPTIONS) -wapruq -ps -qf -DAMIGA -Dconst=

Xxd: xxd.o
	ln +q -m $(LN_DEBUG) -o Xxd xxd.o -lc16

xxd.o: xxd.c
	cc $(CFLAGS) xxd.c -o xxd.o
