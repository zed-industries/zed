#
# Sysinfo
#
# Copyright (c) 2017 Guillaume Gomez
#

#
# Please note that this Makefile only generates the c example.
#

IDIR = ./src
CC = gcc
CFLAGS = -I$(IDIR)

ODIR = examples/
LDIR = ./target/debug/
LDIR-RELEASE = ./target/release/

LIBS = -lsysinfo -lpthread

_DEPS = sysinfo.h
DEPS = $(patsubst %,$(IDIR)/%,$(_DEPS))

_OBJ = simple.o
OBJ = $(patsubst %,$(ODIR)/%,$(_OBJ))


simple: $(OBJ)
	@echo "Compiling in debug mode"
	cargo rustc --features=c-interface --crate-type cdylib
	gcc -o $@ $^ $(CFLAGS) -L$(LDIR) $(LIBS)

release: $(OBJ)
	@echo "Compiling in release mode"
	cargo rustc --features=c-interface --release --crate-type cdylib
	gcc -o simple $^ $(CFLAGS) -L$(LDIR-RELEASE) $(LIBS)

$(ODIR)/%.o: %.c $(DEPS)
	$(CC) -c -o $@ $< $(CFLAGS)

.PHONY: simple

clean:
	@echo "Cleaning mess"
	rm -f $(ODIR)/*.o *~ core $(INCDIR)/*~
