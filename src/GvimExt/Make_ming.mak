# Project: gvimext
# Generates gvimext.dll with gcc.
# To be used with MingW and Cygwin.
#
# Originally, the DLL base address was fixed: -Wl,--image-base=0x1C000000
# Now it is allocated dynamically by the linker by evaluating all DLLs
# already loaded in memory. The binary image contains as well information
# for automatic pseudo-rebasing, if needed by the system. ALV 2004-02-29

# If cross-compiling set this to yes, else set it to no
CROSS = no
#CROSS = yes
# For the old MinGW 2.95 (the one you get e.g. with debian woody)
# set the following variable to yes and check if the executables are
# really named that way.
# If you have a newer MinGW or you are using cygwin set it to no and
# check also the executables
MINGWOLD = no

# Link against the shared versions of libgcc/libstdc++ by default.  Set
# STATIC_STDCPLUS to "yes" to link against static versions instead.
STATIC_STDCPLUS=no
#STATIC_STDCPLUS=yes

# Note: -static-libstdc++ is not available until gcc 4.5.x.
LDFLAGS += -shared
ifeq (yes, $(STATIC_STDCPLUS))
LDFLAGS += -static-libgcc -static-libstdc++
endif

ifeq ($(CROSS),yes)
DEL = rm
ifeq ($(MINGWOLD),yes)
CXXFLAGS := -O2 -fvtable-thunks
else
CXXFLAGS := -O2
endif
else
CXXFLAGS := -O2
ifneq (sh.exe, $(SHELL))
DEL = rm
else
DEL = del
endif
endif
# Set the default $(WINVER) to make it work with Windows 7.
ifndef WINVER
WINVER = 0x0601
endif
CXX := $(CROSS_COMPILE)g++
WINDRES := $(CROSS_COMPILE)windres
# this used to have --preprocessor, but it's no longer supported
WINDRES_FLAGS =
LIBS :=  -luuid -lgdi32
RES  := gvimext.res
ifeq ($(findstring clang++,$(CXX)),)
DEFFILE = gvimext_ming.def
endif
OBJ  := gvimext.o

DLL  := gvimext.dll

.PHONY: all all-before all-after clean clean-custom

all: all-before $(DLL) all-after

$(DLL): $(OBJ) $(RES) $(DEFFILE)
	$(CXX) $(LDFLAGS) $(CXXFLAGS) -s -o $@ \
		-Wl,--enable-auto-image-base \
		-Wl,--enable-auto-import \
		-Wl,--whole-archive \
			$^ \
		-Wl,--no-whole-archive \
			$(LIBS)

gvimext.o: gvimext.cpp
	$(CXX) $(CXXFLAGS) -DFEAT_GETTEXT -DWINVER=$(WINVER) -D_WIN32_WINNT=$(WINVER) -c $? -o $@

$(RES): gvimext_ming.rc
	$(WINDRES) $(WINDRES_FLAGS) --input-format=rc --output-format=coff -DMING $? -o $@

clean: clean-custom
	-$(DEL)  $(OBJ) $(RES) $(DLL)
