# TOP_DIR and OBJ_DIR should be set by the user to the right directories,
# if necessary.

TOP_DIR ?= ../../..
OBJ_DIR ?= $(TOP_DIR)/objs


# The setup below is for gcc on a Unix-like platform,
# where FreeType has been set up to create a static library
# (which is the default).

VPATH = $(OBJ_DIR) \
        $(OBJ_DIR)/.libs

SRC_DIR = $(TOP_DIR)/src/tools/ftrandom

CC = gcc
WFLAGS = -Wmissing-prototypes \
         -Wunused \
         -Wimplicit \
         -Wreturn-type \
         -Wparentheses \
         -pedantic \
         -Wformat \
         -Wchar-subscripts \
         -Wsequence-point
CFLAGS = $(WFLAGS) \
         -g
INCLUDES = -I $(TOP_DIR)/include
LDFLAGS =
LIBS = -lm \
       -lz \
       -lpng \
       -lbz2 \
       -lharfbuzz

all: $(OBJ_DIR)/ftrandom

$(OBJ_DIR)/ftrandom.o: $(SRC_DIR)/ftrandom.c
	$(CC) $(CFLAGS) $(INCLUDES) -c -o $@ $<

$(OBJ_DIR)/ftrandom: $(OBJ_DIR)/ftrandom.o libfreetype.a
	$(CC) $(LDFLAGS) -o $@ $^ $(LIBS)

# EOF
