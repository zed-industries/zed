dnl configure.ac: autoconf script for Vim

dnl Process this file with autoconf 2.71 to produce "configure".

AC_INIT
AC_CONFIG_SRCDIR([vim.h])
AC_CONFIG_HEADERS(auto/config.h:config.h.in)

dnl Being able to run configure means the system is Unix (compatible).
AC_DEFINE(UNIX)
AC_PROG_MAKE_SET

dnl Checks for programs.
AC_PROG_CC		dnl required by almost everything
AC_PROG_CPP		dnl required by header file checks
AC_PROG_EGREP		dnl required by AC_EGREP_CPP
AC_PROG_FGREP		dnl finds working grep -F
AC_PROG_AWK		dnl required for "make html" in ../doc

dnl Don't strip if we don't have it
AC_CHECK_PROG(STRIP, strip, strip, :)

dnl Check for extension of executables
AC_EXEEXT

AC_HEADER_SYS_WAIT

dnl Check that the C99 features that Vim uses are supported:
if test x"$ac_cv_prog_cc_c99" != xno; then
  dnl If the compiler doesn't explicitly support C99, then check
  dnl for the specific features Vim uses

  AC_TYPE_LONG_LONG_INT
  if test "$ac_cv_type_long_long_int" = no; then
    AC_MSG_FAILURE([Compiler does not support long long int])
  fi

  AC_MSG_CHECKING([if the compiler supports trailing commas])
  trailing_commas=no
  AC_COMPILE_IFELSE([AC_LANG_PROGRAM([], [
    enum {
      one,
    };])],
    [AC_MSG_RESULT(yes); trailing_commas=yes],
    [AC_MSG_RESULT(no)])
  if test "$trailing_commas" = no; then
    AC_MSG_FAILURE([Compiler does not support trailing comma in enum])
  fi

  AC_MSG_CHECKING([if the compiler supports C++ comments])
  slash_comments=no
  AC_COMPILE_IFELSE([AC_LANG_PROGRAM([],
    [// C++ comments?])],
    [AC_MSG_RESULT(yes); slash_comments=yes],
    [AC_MSG_RESULT(no)])
  if test "$slash_comments" = no; then
    AC_MSG_FAILURE([Compiler does not support C++ comments])
  fi
fi

dnl If $SOURCE_DATE_EPOCH is present in the environment, use that as the
dnl "compiled" timestamp in :version's output.  Attempt to get the formatted
dnl date using GNU date syntax, BSD date syntax, and finally falling back to
dnl just using the current time.
if test -n "$SOURCE_DATE_EPOCH"; then
  DATE_FMT="%b %d %Y %H:%M:%S"
  BUILD_DATE=$(LC_ALL=C date -u -d "@$SOURCE_DATE_EPOCH" "+$DATE_FMT" 2>/dev/null || LC_ALL=C date -u -r "$SOURCE_DATE_EPOCH" "+$DATE_FMT" 2>/dev/null || LC_ALL=C date -u "+$DATE_FMT")
  AC_DEFINE_UNQUOTED(BUILD_DATE, ["$BUILD_DATE"])
  BUILD_DATE_MSG=-"echo -e '=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=\nNOTE: build date/time is fixed: $BUILD_DATE\n=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-='"
  AC_SUBST(BUILD_DATE_MSG)
fi

dnl Check for the flag that fails if stuff are missing.

AC_MSG_CHECKING(--enable-fail-if-missing argument)
AC_ARG_ENABLE(fail_if_missing,
	[  --enable-fail-if-missing    Fail if dependencies on additional features
     specified on the command line are missing.],
	[fail_if_missing="yes"],
	[fail_if_missing="no"])
AC_MSG_RESULT($fail_if_missing)

dnl Keep original value to check later.
with_x_arg="$with_x"

dnl Set default value for CFLAGS if none is defined or it's empty
if test -z "$CFLAGS"; then
  CFLAGS="-O"
  test "$GCC" = yes && CFLAGS="-O2 -fno-strength-reduce -Wall -Wno-deprecated-declarations"
fi

if test "$GCC" = yes; then
  dnl Method that should work for nearly all gcc versions.
  gccversion=`$CC -dumpversion`
  if test "x$gccversion" = "x"; then
    dnl old method; fall-back for when -dumpversion doesn't work
    gccversion=`$CC --version | sed -e '2,$d' -e 's/darwin.//' -e 's/^[[^0-9]]*\([[0-9]]\.[[0-9.]]*\).*$/\1/g'`
  fi

  dnl Gcc version 4.0.1 was reported to cause trouble on Macintosh by Marcin
  dnl Dalecki.  For Mac with Intel the symptom reported is: malloc() reports
  dnl double free.  This happens in expand_filename(), because the optimizer
  dnl swaps two blocks of code, both using "repl", that can't be swapped.
  if test "$gccversion" = "3.0.1" -o "$gccversion" = "3.0.2" -o "$gccversion" = "4.0.1"; then
    echo 'GCC [[34]].0.[[12]] has a bug in the optimizer, disabling "-O#"'
    CFLAGS=`echo "$CFLAGS" | sed 's/-O[[23456789]]/-O/'`
  else
    if test "$gccversion" = "3.1" -o "$gccversion" = "3.2" -o "$gccversion" = "3.2.1" && `echo "$CFLAGS" | grep -v fno-strength-reduce >/dev/null`; then
      echo 'GCC 3.1 and 3.2 have a bug in the optimizer, adding "-fno-strength-reduce"'
      CFLAGS="$CFLAGS -fno-strength-reduce"
    fi
  fi
fi

dnl clang-500.2.75 or around has abandoned -f[no-]strength-reduce and issues a
dnl warning when that flag is passed to.  Accordingly, adjust CFLAGS based on
dnl the version number of the clang in use.
dnl Note that this does not work to get the version of clang 3.1 or 3.2.
AC_MSG_CHECKING(for clang version)
CLANG_VERSION_STRING=`$CC --version 2>/dev/null | sed  -n -e 's/^.*clang[[^0-9]]*\([[0-9]][[0-9]]*\.[[0-9]][[0-9]]*\.[[0-9]][[0-9]]*\).*$/\1/p'`
if test x"$CLANG_VERSION_STRING" != x"" ; then
  CLANG_MAJOR=`echo "$CLANG_VERSION_STRING" | sed -n -e 's/\([[0-9]][[0-9]]*\)\.[[0-9]][[0-9]]*\.[[0-9]][[0-9]]*/\1/p'`
  CLANG_MINOR=`echo "$CLANG_VERSION_STRING" | sed -n -e 's/[[0-9]][[0-9]]*\.\([[0-9]][[0-9]]*\)\.[[0-9]][[0-9]]*/\1/p'`
  CLANG_REVISION=`echo "$CLANG_VERSION_STRING" | sed -n -e 's/[[0-9]][[0-9]]*\.[[0-9]][[0-9]]*\.\([[0-9]][[0-9]]*\)/\1/p'`
  CLANG_VERSION=`expr $CLANG_MAJOR '*' 1000000 '+' $CLANG_MINOR '*' 1000 '+' $CLANG_REVISION`
  AC_MSG_RESULT($CLANG_VERSION)
  dnl If you find the same issue with versions earlier than 500.2.75,
  dnl change the constant 500002075 below appropriately.  To get the
  dnl integer corresponding to a version number, refer to the
  dnl definition of CLANG_VERSION above.
  dnl Clang 11 reports "11", assume Clang 10 and later work like this.
  AC_MSG_CHECKING(if clang supports -fno-strength-reduce)
  if test "$CLANG_MAJOR" -ge 10 -o "$CLANG_VERSION" -ge 500002075 ; then
    AC_MSG_RESULT(no)
    CFLAGS=`echo "$CFLAGS" | sed -e 's/-fno-strength-reduce/ /'`
  else
    AC_MSG_RESULT(yes)
  fi
else
  AC_MSG_RESULT(N/A)
fi

dnl If configure thinks we are cross compiling, there might be something
dnl wrong with the CC or CFLAGS settings, give a useful warning message
CROSS_COMPILING=
if test "$cross_compiling" = yes; then
  AC_MSG_RESULT([cannot compile a simple program; if not cross compiling check CC and CFLAGS])
  CROSS_COMPILING=1
fi
AC_SUBST(CROSS_COMPILING)

dnl gcc-cpp has the wonderful -MM option to produce nicer dependencies.
dnl But gcc 3.1 changed the meaning!  See near the end.
test "$GCC" = yes && CPP_MM=M; AC_SUBST(CPP_MM)

if test -f ./toolcheck; then
  AC_MSG_CHECKING(for buggy tools)
  sh ./toolcheck 1>&AS_MESSAGE_FD
fi

OS_EXTRA_SRC=""; OS_EXTRA_OBJ=""

dnl When cross-compiling set $vim_cv_uname_output, $vim_cv_uname_r_output and
dnl $vim_cv_uname_m_output to the desired value for the target system
AC_MSG_CHECKING(uname)
if test "x$vim_cv_uname_output" = "x" ; then
  vim_cv_uname_output=`(uname) 2>/dev/null`
  AC_MSG_RESULT($vim_cv_uname_output)
else
  AC_MSG_RESULT([$vim_cv_uname_output (cached)])
fi

AC_MSG_CHECKING(uname -r)
if test "x$vim_cv_uname_r_output" = "x" ; then
  vim_cv_uname_r_output=`(uname -r) 2>/dev/null`
  AC_MSG_RESULT($vim_cv_uname_r_output)
else
  AC_MSG_RESULT([$vim_cv_uname_r_output (cached)])
fi

AC_MSG_CHECKING(uname -m)
if test "x$vim_cv_uname_m_output" = "x" ; then
  vim_cv_uname_m_output=`(uname -m) 2>/dev/null`
  AC_MSG_RESULT($vim_cv_uname_m_output)
else
  AC_MSG_RESULT([$vim_cv_uname_m_output (cached)])
fi

AC_MSG_CHECKING(for Haiku)
case $vim_cv_uname_output in
    Haiku) HAIKU=yes; AC_MSG_RESULT(yes);;
    *)     HAIKU=no; AC_MSG_RESULT(no);;
esac

dnl If QNX is found, assume we don't want to use Xphoton
dnl unless it was specifically asked for (--with-x)
AC_MSG_CHECKING(for QNX)
case $vim_cv_uname_output in
    QNX)	OS_EXTRA_SRC=os_qnx.c; OS_EXTRA_OBJ=objects/os_qnx.o
		test -z "$with_x" && with_x=no
		QNX=yes; AC_MSG_RESULT(yes);;
    *)		QNX=no; AC_MSG_RESULT(no);;
esac

dnl Check for Darwin and MacOS X
dnl We do a check for MacOS X in the very beginning because there
dnl are a lot of other things we need to change besides GUI stuff
AC_MSG_CHECKING([for Darwin (Mac OS X)])
if test "$vim_cv_uname_output" = Darwin; then
  AC_MSG_RESULT(yes)
  MACOS_X=yes
  CPPFLAGS="$CPPFLAGS -DMACOS_X"

  AC_MSG_CHECKING(--disable-darwin argument)
  AC_ARG_ENABLE(darwin,
	  [  --disable-darwin        Disable Darwin (Mac OS X) support.],
	  , [enable_darwin="yes"])
  if test "$enable_darwin" = "yes"; then
    AC_MSG_RESULT(no)
    AC_MSG_CHECKING(if Darwin files are there)
    if test -f os_macosx.m; then
      AC_MSG_RESULT(yes)
    else
      AC_MSG_RESULT([no, Darwin support disabled])
      enable_darwin=no
    fi
  else
    AC_MSG_RESULT([yes, Darwin support excluded])
  fi

  AC_MSG_CHECKING(--with-mac-arch argument)
  AC_ARG_WITH(mac-arch, [  --with-mac-arch=ARCH    current, intel, ppc or both],
	MACARCH="$withval"; AC_MSG_RESULT($MACARCH),
	MACARCH="current"; AC_MSG_RESULT(defaulting to $MACARCH))

  AC_MSG_CHECKING(--with-developer-dir argument)
  AC_ARG_WITH(developer-dir, [  --with-developer-dir=PATH    use PATH as location for Xcode developer tools],
	DEVELOPER_DIR="$withval"; AC_MSG_RESULT($DEVELOPER_DIR),
        AC_MSG_RESULT(not present))

  if test "x$DEVELOPER_DIR" = "x"; then
    AC_PATH_PROG(XCODE_SELECT, xcode-select)
    if test "x$XCODE_SELECT" != "x"; then
      AC_MSG_CHECKING(for developer dir using xcode-select)
      DEVELOPER_DIR=`$XCODE_SELECT -print-path`
      AC_MSG_RESULT([$DEVELOPER_DIR])
    else
      DEVELOPER_DIR=/Developer
    fi
  fi

  if test "x$MACARCH" = "xboth"; then
    AC_MSG_CHECKING(for 10.4 universal SDK)
    dnl There is a terrible inconsistency (but we appear to get away with it):
    dnl $CFLAGS uses the 10.4u SDK library for the headers, while $CPPFLAGS
    dnl doesn't, because "gcc -E" doesn't grok it.  That means the configure
    dnl tests using the preprocessor are actually done with the wrong header
    dnl files. $LDFLAGS is set at the end, because configure uses it together
    dnl with $CFLAGS and we can only have one -sysroot argument.
    save_cppflags="$CPPFLAGS"
    save_cflags="$CFLAGS"
    save_ldflags="$LDFLAGS"
    CFLAGS="$CFLAGS -isysroot $DEVELOPER_DIR/SDKs/MacOSX10.4u.sdk -arch i386 -arch ppc"
    AC_LINK_IFELSE([AC_LANG_PROGRAM([ ], [ ])],
	AC_MSG_RESULT(found, will make universal binary),

	AC_MSG_RESULT(not found)
	CFLAGS="$save_cflags"
	AC_MSG_CHECKING(if Intel architecture is supported)
	CPPFLAGS="$CPPFLAGS -arch i386"
	LDFLAGS="$save_ldflags -arch i386"
	AC_LINK_IFELSE([AC_LANG_PROGRAM([ ], [ ])],
	    AC_MSG_RESULT(yes); MACARCH="intel",
	    AC_MSG_RESULT(no, using PowerPC)
		MACARCH="ppc"
		CPPFLAGS="$save_cppflags -arch ppc"
		LDFLAGS="$save_ldflags -arch ppc"))
  elif test "x$MACARCH" = "xintel"; then
    CPPFLAGS="$CPPFLAGS -arch intel"
    LDFLAGS="$LDFLAGS -arch intel"
  elif test "x$MACARCH" = "xppc"; then
    CPPFLAGS="$CPPFLAGS -arch ppc"
    LDFLAGS="$LDFLAGS -arch ppc"
  fi

  if test "$enable_darwin" = "yes"; then
    MACOS_X_DARWIN=yes
    OS_EXTRA_SRC="os_macosx.m os_mac_conv.c";
    OS_EXTRA_OBJ="objects/os_macosx.o objects/os_mac_conv.o"
    dnl os_macosx.m implements timer_create() and friends
    AC_DEFINE(HAVE_TIMER_CREATE)
    dnl TODO: use -arch i386 on Intel machines
    dnl Removed -no-cpp-precomp, only for very old compilers.
    CPPFLAGS="$CPPFLAGS -DMACOS_X_DARWIN"

     dnl Assume we don't want X11 unless it was specifically asked for
     dnl (--with-x) or Motif or GTK GUI is used.
     if test -z "$with_x" -a "X$enable_gui" != Xmotif -a "X$enable_gui" != Xgtk2 -a "X$enable_gui" != Xgtk3; then
      with_x=no
     fi
  fi
else
  AC_MSG_RESULT(no)
fi

dnl Mac OS X 10.9+ no longer include AvailabilityMacros.h in Carbon
dnl so we need to include it to have access to version macros.
AC_CHECK_HEADERS(AvailabilityMacros.h)
# 10.5 and earlier lack dispatch
AC_CHECK_HEADERS(dispatch/dispatch.h)

AC_SUBST(OS_EXTRA_SRC)
AC_SUBST(OS_EXTRA_OBJ)

dnl Add /usr/local/lib to $LDFLAGS and /usr/local/include to CFLAGS.
dnl Only when the directory exists and it wasn't there yet.
dnl For gcc don't do this when it is already in the default search path.
dnl Skip all of this when cross-compiling.
if test "$cross_compiling" = no; then
  AC_MSG_CHECKING(--with-local-dir argument)
  have_local_include=''
  have_local_lib=''
  AC_ARG_WITH([local-dir], [  --with-local-dir=PATH   search PATH instead of /usr/local for local libraries.
  --without-local-dir     do not search /usr/local for local libraries.], [
    local_dir="$withval"
    case "$withval" in
    */*) ;;
    no)
      # avoid adding local dir to LDFLAGS and CPPFLAGS
      have_local_include=yes
      have_local_lib=yes
      ;;
    *) AC_MSG_ERROR(must pass path argument to --with-local-dir) ;;
    esac
	AC_MSG_RESULT($local_dir)
  ], [
    local_dir=/usr/local
    AC_MSG_RESULT(Defaulting to $local_dir)
  ])
  if test "$GCC" = yes -a "$local_dir" != no; then
    echo 'void f(){}' > conftest.c
    dnl Removed -no-cpp-precomp, only needed for OS X 10.2 (Ben Fowler)
    have_local_include=`${CC-cc} -c -v conftest.c 2>&1 | grep "${local_dir}/include"`
    have_local_lib=`${CC-cc} -c -v conftest.c 2>&1 | grep "${local_dir}/lib"`
    rm -f conftest.c conftest.o
  fi
  if test -z "$have_local_lib" -a -d "${local_dir}/lib"; then
    tt=`echo "$LDFLAGS" | sed -e "s+-L${local_dir}/lib ++g" -e "s+-L${local_dir}/lib$++g"`
    if test "$tt" = "$LDFLAGS"; then
      LDFLAGS="$LDFLAGS -L${local_dir}/lib"
    fi
  fi
  if test -z "$have_local_include" -a -d "${local_dir}/include"; then
    tt=`echo "$CPPFLAGS" | sed -e "s+-I${local_dir}/include ++g" -e "s+-I${local_dir}/include$++g"`
    if test "$tt" = "$CPPFLAGS"; then
      CPPFLAGS="$CPPFLAGS -I${local_dir}/include"
    fi
  fi
fi

AC_MSG_CHECKING(--with-vim-name argument)
AC_ARG_WITH(vim-name, [  --with-vim-name=NAME    what to call the Vim executable],
	VIMNAME="$withval"; AC_MSG_RESULT($VIMNAME),
	VIMNAME="vim"; AC_MSG_RESULT(Defaulting to $VIMNAME))
AC_SUBST(VIMNAME)
AC_MSG_CHECKING(--with-ex-name argument)
AC_ARG_WITH(ex-name, [  --with-ex-name=NAME     what to call the Ex executable],
	EXNAME="$withval"; AC_MSG_RESULT($EXNAME),
	EXNAME="ex"; AC_MSG_RESULT(Defaulting to ex))
AC_SUBST(EXNAME)
AC_MSG_CHECKING(--with-view-name argument)
AC_ARG_WITH(view-name, [  --with-view-name=NAME   what to call the View executable],
	VIEWNAME="$withval"; AC_MSG_RESULT($VIEWNAME),
	VIEWNAME="view"; AC_MSG_RESULT(Defaulting to view))
AC_SUBST(VIEWNAME)

AC_MSG_CHECKING(--with-global-runtime argument)
AC_ARG_WITH(global-runtime, [  --with-global-runtime=DIR    global runtime directory in 'runtimepath', comma-separated for multiple directories],
	RUNTIME_GLOBAL="$withval"; AC_MSG_RESULT($withval),
	AC_MSG_RESULT(no))

if test "X$RUNTIME_GLOBAL" != "X"; then
  RUNTIME_GLOBAL_AFTER=$(printf -- "$RUNTIME_GLOBAL\\n" | $AWK -F, 'BEGIN { comma=0 } { for (i = NF; i > 0; i--) { if (comma) { printf ",%s/after", $i } else { printf "%s/after", $i; comma=1 } } } END { printf "\n" }')
  AC_DEFINE_UNQUOTED(RUNTIME_GLOBAL, "$RUNTIME_GLOBAL")
  AC_DEFINE_UNQUOTED(RUNTIME_GLOBAL_AFTER, "$RUNTIME_GLOBAL_AFTER")
fi

AC_MSG_CHECKING(--with-modified-by argument)
AC_ARG_WITH(modified-by, [  --with-modified-by=NAME       name of who modified a release version],
	AC_MSG_RESULT($withval); AC_DEFINE_UNQUOTED(MODIFIED_BY, "$withval"),
	AC_MSG_RESULT(no))

dnl Check for EBCDIC stolen from the LYNX port to z/OS Unix
AC_MSG_CHECKING(if character set is EBCDIC)
AC_COMPILE_IFELSE([AC_LANG_PROGRAM([ ],
[ /* TryCompile function for CharSet.
   Treat any failure as ASCII for compatibility with existing art.
   Use compile-time rather than run-time tests for cross-compiler
   tolerance.  */
#if '0'!=240
make an error "Character set is not EBCDIC"
#endif ])],
[ # TryCompile action if true
cf_cv_ebcdic=yes ],
[ # TryCompile action if false
cf_cv_ebcdic=no])
# end of TryCompile ])
# end of CacheVal CvEbcdic
AC_MSG_RESULT($cf_cv_ebcdic)
case "$cf_cv_ebcdic" in  #(vi
    yes)	AC_DEFINE(EBCDIC)
		line_break='"\\n"'
		;;
    *)		line_break='"\\012"';;
esac
AC_SUBST(line_break)

if test "$cf_cv_ebcdic" = "yes"; then
dnl If we have EBCDIC we most likely have z/OS Unix, let's test it!
AC_MSG_CHECKING(for z/OS Unix)
case $vim_cv_uname_output in
    OS/390)	zOSUnix="yes";
		dnl If using cc the environment variable _CC_CCMODE must be
		dnl set to "1", so that some compiler extensions are enabled.
		dnl If using c89 the environment variable is named _CC_C89MODE.
		dnl Note: compile with c89 never tested.
		if test "$CC" = "cc"; then
		  ccm="$_CC_CCMODE"
		  ccn="CC"
		else
		  if test "$CC" = "c89"; then
		    ccm="$_CC_C89MODE"
		    ccn="C89"
		  else
		    ccm=1
		  fi
		fi
		if test "$ccm" != "1"; then
		  echo ""
		  echo "------------------------------------------"
		  echo " On z/OS Unix, the environment variable"
		  echo " _CC_${ccn}MODE must be set to \"1\"!"
		  echo " Do:"
		  echo "    export _CC_${ccn}MODE=1"
		  echo " and then call configure again."
		  echo "------------------------------------------"
		  exit 1
		fi
		# Set CFLAGS for configure process.
		# This will be reset later for config.mk.
		# Use haltonmsg to force error for missing H files.
		CFLAGS="$CFLAGS -D_ALL_SOURCE -Wc,float(ieee),haltonmsg(3296)";
		LDFLAGS="$LDFLAGS -Wl,EDIT=NO"
		AC_MSG_RESULT(yes)
		;;
    *)		zOSUnix="no";
		AC_MSG_RESULT(no)
		;;
esac
fi

dnl Set QUOTESED. Needs additional backslashes on zOS
if test "$zOSUnix" = "yes"; then
    QUOTESED="sed -e 's/[[\\\\\"]]/\\\\\\\\&/g' -e 's/\\\\\\\\\"/\"/' -e 's/\\\\\\\\\";\$\$/\";/' -e 's/  */ /g'"
else
    QUOTESED="sed -e 's/[[\\\\\"]]/\\\\&/g' -e 's/\\\\\"/\"/' -e 's/\\\\\";\$\$/\";/' -e 's/  */ /g'"
fi
AC_SUBST(QUOTESED)


dnl Link with -lsmack for Smack stuff; if not found
AC_MSG_CHECKING(--disable-smack argument)
AC_ARG_ENABLE(smack,
	[  --disable-smack	  Do not check for Smack support.],
	, enable_smack="yes")
if test "$enable_smack" = "yes"; then
  AC_MSG_RESULT(no)
  AC_CHECK_HEADER([linux/xattr.h], true, enable_smack="no")
else
  AC_MSG_RESULT(yes)
fi
if test "$enable_smack" = "yes"; then
  AC_CHECK_HEADER([sys/xattr.h], true, enable_smack="no")
fi
if test "$enable_smack" = "yes"; then
  AC_MSG_CHECKING(for XATTR_NAME_SMACKEXEC in linux/xattr.h)
  AC_EGREP_CPP(XATTR_NAME_SMACKEXEC, [#include <linux/xattr.h>],
	       AC_MSG_RESULT(yes),
	       AC_MSG_RESULT(no); enable_smack="no")
fi
if test "$enable_smack" = "yes"; then
  AC_CHECK_LIB(attr, setxattr,
	  [LIBS="$LIBS -lattr"
	   found_smack="yes"
	   AC_DEFINE(HAVE_SMACK)])
fi

dnl When smack was found don't search for SELinux
if test "x$found_smack" = "x"; then
  dnl Link with -lselinux for SELinux stuff; if not found
  AC_MSG_CHECKING(--disable-selinux argument)
  AC_ARG_ENABLE(selinux,
	  [  --disable-selinux	  Do not check for SELinux support.],
	  , enable_selinux="yes")
  if test "$enable_selinux" = "yes"; then
    AC_MSG_RESULT(no)
    AC_CHECK_LIB(selinux, is_selinux_enabled,
	[AC_CHECK_HEADER(selinux/selinux.h,
	    [LIBS="$LIBS -lselinux"
	    AC_DEFINE(HAVE_SELINUX)])])
  else
     AC_MSG_RESULT(yes)
  fi
fi

dnl enable xattr support
AC_MSG_CHECKING(--enable-xattr argument)
AC_ARG_ENABLE(xattr,
	[  --disable-xattr	  Do not check for XATTR support.],
	, enable_xattr="yes")
if test "$enable_xattr" = "yes"; then
  AC_MSG_RESULT(yes)
  AC_CHECK_HEADER([sys/xattr.h], [AC_DEFINE(HAVE_XATTR)])
else
  AC_MSG_RESULT(no)
fi

dnl Check user requested features.

AC_MSG_CHECKING(--with-features argument)
AC_ARG_WITH(features, [  --with-features=TYPE    tiny, normal or huge (default: huge)],
	features="$withval"; AC_MSG_RESULT($features),
	features="huge"; AC_MSG_RESULT(Defaulting to huge))

dnl "small" is supported for backwards compatibility, now an alias for "tiny"
dnl "big" is supported for backwards compatibility, now an alias for "normal"
case "$features" in
  small) features="tiny"   ;;
  big)   features="normal" ;;
esac

dovimdiff=""
dogvimdiff=""
case "$features" in
  tiny)		AC_DEFINE(FEAT_TINY) ;;
  normal)	AC_DEFINE(FEAT_NORMAL) dovimdiff="installvimdiff";
			dogvimdiff="installgvimdiff" ;;
  huge)		AC_DEFINE(FEAT_HUGE) dovimdiff="installvimdiff";
			dogvimdiff="installgvimdiff" ;;
  *)		AC_MSG_RESULT([Sorry, $features is not supported]) ;;
esac

AC_SUBST(dovimdiff)
AC_SUBST(dogvimdiff)

if test "x$features" = "xtiny"; then
  has_eval=no
else
  has_eval=yes
fi

AC_MSG_CHECKING(--with-compiledby argument)
AC_ARG_WITH(compiledby, [  --with-compiledby=NAME  name to show in :version message],
	compiledby="$withval"; AC_MSG_RESULT($withval),
	compiledby=""; AC_MSG_RESULT(no))
AC_SUBST(compiledby)

AC_MSG_CHECKING(--disable-xsmp argument)
AC_ARG_ENABLE(xsmp,
	[  --disable-xsmp          Disable XSMP session management],
	, enable_xsmp="yes")

if test "$enable_xsmp" = "yes"; then
  AC_MSG_RESULT(no)
  AC_MSG_CHECKING(--disable-xsmp-interact argument)
  AC_ARG_ENABLE(xsmp-interact,
	  [  --disable-xsmp-interact Disable XSMP interaction],
	  , enable_xsmp_interact="yes")
  if test "$enable_xsmp_interact" = "yes"; then
    AC_MSG_RESULT(no)
    AC_DEFINE(USE_XSMP_INTERACT)
  else
    AC_MSG_RESULT(yes)
  fi
else
  AC_MSG_RESULT(yes)
fi

AC_MSG_CHECKING([diff feature])
if test "x$features" = "xtiny"; then
  AC_MSG_RESULT([disabled in $features version])
else
  AC_MSG_RESULT(enabled)
  AC_DEFINE(FEAT_DIFF)
  XDIFF_OBJS_USED="\$(XDIFF_OBJS)"
  AC_SUBST(XDIFF_OBJS_USED)
fi

dnl Check for Lua feature.
AC_MSG_CHECKING(--enable-luainterp argument)
AC_ARG_ENABLE(luainterp,
	[  --enable-luainterp[=OPTS]      Include Lua interpreter.  [default=no] [OPTS=no/yes/dynamic]], ,
	[enable_luainterp="no"])
AC_MSG_RESULT($enable_luainterp)

if test "$enable_luainterp" = "yes" -o "$enable_luainterp" = "dynamic"; then
  if test "$has_eval" = "no"; then
    AC_MSG_ERROR([cannot use Lua with tiny features])
  fi

  dnl -- find the lua executable
  AC_SUBST(vi_cv_path_lua)

  AC_MSG_CHECKING(--with-lua-prefix argument)
  AC_ARG_WITH(lua_prefix,
	[  --with-lua-prefix=PFX   Prefix where Lua is installed.],
	with_lua_prefix="$withval"; AC_MSG_RESULT($with_lua_prefix),
	with_lua_prefix="";AC_MSG_RESULT(no))

  if test "X$with_lua_prefix" != "X"; then
       vi_cv_path_lua_pfx="$with_lua_prefix"
  else
    AC_MSG_CHECKING(LUA_PREFIX environment var)
    if test "X$LUA_PREFIX" != "X"; then
	AC_MSG_RESULT("$LUA_PREFIX")
	vi_cv_path_lua_pfx="$LUA_PREFIX"
    else
	AC_MSG_RESULT([not set, default to /usr])
	vi_cv_path_lua_pfx="/usr"
    fi
  fi

  AC_MSG_CHECKING(--with-luajit)
  AC_ARG_WITH(luajit,
	[  --with-luajit           Link with LuaJIT instead of Lua.],
	[vi_cv_with_luajit="$withval"],
	[vi_cv_with_luajit="no"])
  AC_MSG_RESULT($vi_cv_with_luajit)

  LUA_INC=
  if test "X$vi_cv_path_lua_pfx" != "X"; then
    if test "x$vi_cv_with_luajit" != "xno"; then
      dnl -- try to find LuaJIT executable
      AC_PATH_PROG(vi_cv_path_luajit, luajit)
      if test "X$vi_cv_path_luajit" != "X"; then
	dnl -- find LuaJIT version
	AC_CACHE_CHECK(LuaJIT version, vi_cv_version_luajit,
       [ vi_cv_version_luajit=`${vi_cv_path_luajit} -v 2>&1 | sed 's/LuaJIT \([[0-9.]]*\)\.[[0-9]]\(-*[[a-z0-9]]*\)* .*/\1/'` ])
	AC_CACHE_CHECK(Lua version of LuaJIT, vi_cv_version_lua_luajit,
	[ vi_cv_version_lua_luajit=`${vi_cv_path_luajit} -e "print(_VERSION)" | sed 's/.* //'` ])
	vi_cv_path_lua="$vi_cv_path_luajit"
	vi_cv_version_lua="$vi_cv_version_lua_luajit"
      fi
    else
      dnl -- try to find Lua executable
      AC_PATH_PROG(vi_cv_path_plain_lua, lua)
      if test "X$vi_cv_path_plain_lua" != "X"; then
	dnl -- find Lua version
	AC_CACHE_CHECK(Lua version, vi_cv_version_plain_lua,
	[ vi_cv_version_plain_lua=`${vi_cv_path_plain_lua} -e "print(_VERSION)" | sed 's/.* //'` ])
      fi
      vi_cv_path_lua="$vi_cv_path_plain_lua"
      vi_cv_version_lua="$vi_cv_version_plain_lua"
    fi
    if test "x$vi_cv_with_luajit" != "xno" && test "X$vi_cv_version_luajit" != "X"; then
      AC_MSG_CHECKING(if lua.h can be found in $vi_cv_path_lua_pfx/include/luajit-$vi_cv_version_luajit)
      if test -f "$vi_cv_path_lua_pfx/include/luajit-$vi_cv_version_luajit/lua.h"; then
	AC_MSG_RESULT(yes)
	LUA_INC=/luajit-$vi_cv_version_luajit
      fi
    fi
    if test "X$LUA_INC" = "X"; then
      AC_MSG_CHECKING(if lua.h can be found in $vi_cv_path_lua_pfx/include)
      if test -f "$vi_cv_path_lua_pfx/include/lua.h"; then
	AC_MSG_RESULT(yes)
      else
	AC_MSG_RESULT(no)
	AC_MSG_CHECKING(if lua.h can be found in $vi_cv_path_lua_pfx/include/lua$vi_cv_version_lua)
	if test -f "$vi_cv_path_lua_pfx/include/lua$vi_cv_version_lua/lua.h"; then
	  AC_MSG_RESULT(yes)
	  LUA_INC=/lua$vi_cv_version_lua
	else
	  AC_MSG_RESULT(no)

	  # Detect moonjit:
	  # https://groups.google.com/forum/#!topic/vim_use/O0vek60WuTk
	  lua_suf=/moonjit-2.3
	  inc_path="$vi_cv_path_lua_pfx/include"
	  for dir in "$inc_path"/moonjit-[[0-9]]* ; do
	    if test -d "$dir" ; then
	      lua_suf=`basename "$dir"`
	      lua_suf="/$lua_suf"
	      break
	    fi
	  done
	  AC_MSG_CHECKING(if lua.h can be found in $inc_path$lua_suf)
	  if test -f "$inc_path$lua_suf/lua.h"; then
	    AC_MSG_RESULT(yes)
	    LUA_INC=$lua_suf
	  else
	    AC_MSG_RESULT(no)
	    vi_cv_path_lua_pfx=
	  fi
	fi
      fi
    fi
  fi

  if test "X$vi_cv_path_lua_pfx" != "X"; then
    if test "x$vi_cv_with_luajit" != "xno"; then
      multiarch=`dpkg-architecture -qDEB_HOST_MULTIARCH 2> /dev/null`
      if test "X$multiarch" != "X"; then
	lib_multiarch="lib/${multiarch}"
      else
	lib_multiarch="lib"
      fi
      if test "X$vi_cv_version_lua" = "X"; then
	LUA_LIBS="-L${vi_cv_path_lua_pfx}/${lib_multiarch} -lluajit"
      else
	LUA_LIBS="-L${vi_cv_path_lua_pfx}/${lib_multiarch} -lluajit-$vi_cv_version_lua"
      fi
    else
      dnl Test alternate lib location including version
      if test -d "${vi_cv_path_lua_pfx}/lib/lua$vi_cv_version_lua"; then
	LUALIBDIR="lib/lua$vi_cv_version_lua"
      else
	LUALIBDIR=lib
      fi
      if test "X$LUA_INC" != "X"; then
	dnl Test alternate name including version
	LUA_LIBS="-L${vi_cv_path_lua_pfx}/${LUALIBDIR} -llua$vi_cv_version_lua"
      else
	LUA_LIBS="-L${vi_cv_path_lua_pfx}/${LUALIBDIR} -llua"
      fi
    fi
    if test "$enable_luainterp" = "dynamic"; then
      lua_ok="yes"
    else
      AC_MSG_CHECKING([if link with ${LUA_LIBS} is sane])
      libs_save=$LIBS
      LIBS="$LIBS $LUA_LIBS"
      AC_LINK_IFELSE([AC_LANG_PROGRAM(,[ ])],
	AC_MSG_RESULT(yes); lua_ok="yes",
	AC_MSG_RESULT(no); lua_ok="no"; LUA_LIBS="")
      LIBS=$libs_save
    fi
    if test "x$lua_ok" = "xyes"; then
      LUA_CFLAGS="-I${vi_cv_path_lua_pfx}/include${LUA_INC}"
      LUA_SRC="if_lua.c"
      LUA_OBJ="objects/if_lua.o"
      LUA_PRO="if_lua.pro"
      AC_DEFINE(FEAT_LUA)
    fi
    if test "$enable_luainterp" = "dynamic"; then
      if test "x$vi_cv_with_luajit" != "xno"; then
	luajit="jit"
      fi
      if test -f "${vi_cv_path_lua_pfx}/bin/cyglua-${vi_cv_version_lua}.dll"; then
	vi_cv_dll_name_lua="cyglua-${vi_cv_version_lua}.dll"
      else
	if test "x$MACOS_X" = "xyes"; then
	  ext="dylib"
	  indexes=""
	else
	  ext="so"
	  indexes=".0 .1 .2 .3 .4 .5 .6 .7 .8 .9"
	  multiarch=`dpkg-architecture -qDEB_HOST_MULTIARCH 2> /dev/null`
	  if test "X$multiarch" != "X"; then
	    lib_multiarch="lib/${multiarch}"
	  fi
	fi
	dnl Determine the sover for the current version, but fallback to
	dnl liblua${vi_cv_version_lua}.so if no sover-versioned file is found.
	AC_MSG_CHECKING(if liblua${luajit}*.${ext}* can be found in $vi_cv_path_lua_pfx)
	for subdir in "${lib_multiarch}" lib64 lib; do
	  if test -z "$subdir"; then
	    continue
	  fi
	  for sover in "${vi_cv_version_lua}.${ext}" "-${vi_cv_version_lua}.${ext}" \
	    ".${vi_cv_version_lua}.${ext}" ".${ext}.${vi_cv_version_lua}"; do
	    for i in $indexes ""; do
	      if test -f "${vi_cv_path_lua_pfx}/${subdir}/liblua${luajit}${sover}$i"; then
		sover2="$i"
		break 3
	      fi
	    done
	  done
	  sover=""
	done
	if test "X$sover" = "X"; then
	  AC_MSG_RESULT(no)
	  lua_ok="no"
	  vi_cv_dll_name_lua="liblua${luajit}.${ext}"
	else
	  AC_MSG_RESULT(yes)
	  lua_ok="yes"
	  vi_cv_dll_name_lua="liblua${luajit}${sover}$sover2"
	fi
      fi
      AC_DEFINE(DYNAMIC_LUA)
      LUA_LIBS=""
      LUA_CFLAGS="-DDYNAMIC_LUA_DLL=\\\"${vi_cv_dll_name_lua}\\\" $LUA_CFLAGS"
    fi
    if test "X$LUA_CFLAGS$LUA_LIBS" != "X" && \
       test "x$MACOS_X" = "xyes" && test "x$vi_cv_with_luajit" != "xno" && \
       test "$vim_cv_uname_m_output" = "x86_64"; then
      dnl OSX/x64 requires these flags. See http://luajit.org/install.html
      LUA_LIBS="-pagezero_size 10000 -image_base 100000000 $LUA_LIBS"
    fi
  fi
  if test "$fail_if_missing" = "yes" -a "$lua_ok" != "yes"; then
    AC_MSG_ERROR([could not configure lua])
  fi
  AC_SUBST(LUA_SRC)
  AC_SUBST(LUA_OBJ)
  AC_SUBST(LUA_PRO)
  AC_SUBST(LUA_LIBS)
  AC_SUBST(LUA_CFLAGS)
  AC_SUBST(LUA_CFLAGS_EXTRA)
fi


dnl Check for MzScheme feature.
AC_MSG_CHECKING(--enable-mzschemeinterp argument)
AC_ARG_ENABLE(mzschemeinterp,
	[  --enable-mzschemeinterp      Include MzScheme interpreter.], ,
	[enable_mzschemeinterp="no"])
AC_MSG_RESULT($enable_mzschemeinterp)

if test "$enable_mzschemeinterp" = "yes"; then
  dnl -- find the mzscheme executable
  AC_SUBST(vi_cv_path_mzscheme)

  AC_MSG_CHECKING(--with-plthome argument)
  AC_ARG_WITH(plthome,
	[  --with-plthome=PLTHOME   Use PLTHOME.],
	with_plthome="$withval"; AC_MSG_RESULT($with_plthome),
	with_plthome="";AC_MSG_RESULT("no"))

  if test "X$with_plthome" != "X"; then
       vi_cv_path_mzscheme_pfx="$with_plthome"
       vi_cv_path_mzscheme="${vi_cv_path_mzscheme_pfx}/bin/mzscheme"
  else
    AC_MSG_CHECKING(PLTHOME environment var)
    if test "X$PLTHOME" != "X"; then
	AC_MSG_RESULT("$PLTHOME")
	vi_cv_path_mzscheme_pfx="$PLTHOME"
	vi_cv_path_mzscheme="${vi_cv_path_mzscheme_pfx}/bin/mzscheme"
    else
	AC_MSG_RESULT(not set)
	dnl -- try to find MzScheme executable
	AC_PATH_PROG(vi_cv_path_mzscheme, mzscheme)

	dnl resolve symbolic link, the executable is often elsewhere and there
	dnl are no links for the include files.
	if test "X$vi_cv_path_mzscheme" != "X"; then
	  lsout=`ls -l $vi_cv_path_mzscheme`
	  if echo "$lsout" | grep -e '->' >/dev/null 2>/dev/null; then
	    vi_cv_path_mzscheme=`echo "$lsout" | sed 's/.*-> \(.*\)/\1/'`
	  fi
	fi

	if test "X$vi_cv_path_mzscheme" != "X"; then
	    dnl -- find where MzScheme thinks it was installed
	    AC_CACHE_CHECK(MzScheme install prefix,vi_cv_path_mzscheme_pfx,
	    dnl different versions of MzScheme differ in command line processing
	    dnl use universal approach
	    echo "(display (simplify-path		\
	       (build-path (call-with-values	\
		(lambda () (split-path (find-system-path (quote exec-file)))) \
		(lambda (base name must-be-dir?) base)) (quote up))))" > mzdirs.scm
	    dnl Remove a trailing slash
	    [ vi_cv_path_mzscheme_pfx=`${vi_cv_path_mzscheme} -r mzdirs.scm | \
		sed -e 's+/$++'` ])
	    rm -f mzdirs.scm
	fi
    fi
  fi

  if test "X$vi_cv_path_mzscheme_pfx" != "X"; then
    AC_MSG_CHECKING(for racket include directory)
    SCHEME_INC=`${vi_cv_path_mzscheme} -e '(require setup/dirs)(let ((p (find-include-dir))) (when (path? p) (display p)))'`
    if test "X$SCHEME_INC" != "X"; then
      AC_MSG_RESULT(${SCHEME_INC})
    else
      AC_MSG_RESULT(not found)
      AC_MSG_CHECKING(if scheme.h can be found in $vi_cv_path_mzscheme_pfx/include)
      if test -f "$vi_cv_path_mzscheme_pfx/include/scheme.h"; then
	SCHEME_INC=${vi_cv_path_mzscheme_pfx}/include
	AC_MSG_RESULT(yes)
      else
	AC_MSG_RESULT(no)
	AC_MSG_CHECKING(if scheme.h can be found in $vi_cv_path_mzscheme_pfx/include/plt)
	if test -f "$vi_cv_path_mzscheme_pfx/include/plt/scheme.h"; then
	  AC_MSG_RESULT(yes)
	  SCHEME_INC=${vi_cv_path_mzscheme_pfx}/include/plt
	else
	  AC_MSG_RESULT(no)
	  AC_MSG_CHECKING(if scheme.h can be found in $vi_cv_path_mzscheme_pfx/include/racket)
	  if test -f "$vi_cv_path_mzscheme_pfx/include/racket/scheme.h"; then
	    AC_MSG_RESULT(yes)
	    SCHEME_INC=${vi_cv_path_mzscheme_pfx}/include/racket
	  else
	    AC_MSG_RESULT(no)
	    AC_MSG_CHECKING(if scheme.h can be found in /usr/include/plt/)
	    if test -f /usr/include/plt/scheme.h; then
	      AC_MSG_RESULT(yes)
	      SCHEME_INC=/usr/include/plt
	    else
	      AC_MSG_RESULT(no)
	      AC_MSG_CHECKING(if scheme.h can be found in /usr/include/racket/)
	      if test -f /usr/include/racket/scheme.h; then
		AC_MSG_RESULT(yes)
		SCHEME_INC=/usr/include/racket
	      else
		AC_MSG_RESULT(no)
		vi_cv_path_mzscheme_pfx=
	      fi
	    fi
	  fi
	fi
      fi
    fi
  fi

  if test "X$vi_cv_path_mzscheme_pfx" != "X"; then

    AC_MSG_CHECKING(for racket lib directory)
    SCHEME_LIB=`${vi_cv_path_mzscheme} -e '(require setup/dirs)(let ((p (find-lib-dir))) (when (path? p) (display p)))'`
    if test "X$SCHEME_LIB" != "X"; then
      AC_MSG_RESULT(${SCHEME_LIB})
    else
      AC_MSG_RESULT(not found)
    fi

    for path in "${vi_cv_path_mzscheme_pfx}/lib" "${SCHEME_LIB}"; do
      if test "X$path" != "X"; then
	if test "x$MACOS_X" = "xyes"; then
	  MZSCHEME_LIBS="-framework Racket"
	  MZSCHEME_CFLAGS="-DMZ_PRECISE_GC"
	elif test -f "${path}/libmzscheme3m.a"; then
	  MZSCHEME_LIBS="${path}/libmzscheme3m.a"
	  MZSCHEME_CFLAGS="-DMZ_PRECISE_GC"
	elif test -f "${path}/libracket3m.a"; then
	  MZSCHEME_LIBS="${path}/libracket3m.a"
	  if test -f "${path}/librktio.a"; then
	    MZSCHEME_LIBS="${MZSCHEME_LIBS} ${path}/librktio.a"
	  fi
	  MZSCHEME_CFLAGS="-DMZ_PRECISE_GC"
	elif test -f "${path}/libracket.a"; then
	  MZSCHEME_LIBS="${path}/libracket.a ${path}/libmzgc.a"
	elif test -f "${path}/libmzscheme.a"; then
	  MZSCHEME_LIBS="${path}/libmzscheme.a ${path}/libmzgc.a"
	else
	  dnl Using shared objects
	  if test -f "${path}/libmzscheme3m.so"; then
	    MZSCHEME_LIBS="-L${path} -lmzscheme3m"
	    MZSCHEME_CFLAGS="-DMZ_PRECISE_GC"
	  elif test -f "${path}/libracket3m.so"; then
	    MZSCHEME_LIBS="-L${path} -lracket3m"
	    MZSCHEME_CFLAGS="-DMZ_PRECISE_GC"
	  elif test -f "${path}/libracket.so"; then
	    MZSCHEME_LIBS="-L${path} -lracket -lmzgc"
	  else
	    dnl try next until last
	    if test "$path" != "$SCHEME_LIB"; then
	      continue
	    fi
	    MZSCHEME_LIBS="-L${path} -lmzscheme -lmzgc"
	  fi
	  if test "$GCC" = yes; then
	    dnl Make Vim remember the path to the library.  For when it's not in
	    dnl $LD_LIBRARY_PATH.
	    MZSCHEME_LIBS="${MZSCHEME_LIBS} -Wl,-rpath -Wl,${path}"
	  elif test "$vim_cv_uname_output" = SunOS &&
			   echo $vim_cv_uname_r_output | grep '^5' >/dev/null; then
	    MZSCHEME_LIBS="${MZSCHEME_LIBS} -R ${path}"
	  fi
	fi
      fi
      if test "X$MZSCHEME_LIBS" != "X"; then
	break
      fi
    done

    AC_MSG_CHECKING([if racket requires -pthread])
    if test "X$SCHEME_LIB" != "X" && $FGREP -e -pthread "$SCHEME_LIB/buildinfo" >/dev/null ; then
      AC_MSG_RESULT(yes)
      MZSCHEME_LIBS="${MZSCHEME_LIBS} -pthread"
      MZSCHEME_CFLAGS="${MZSCHEME_CFLAGS} -pthread"
    else
      AC_MSG_RESULT(no)
    fi

    AC_MSG_CHECKING(for racket config directory)
    SCHEME_CONFIGDIR=`${vi_cv_path_mzscheme} -e '(require setup/dirs)(let ((p (find-config-dir))) (when (path? p) (display p)))'`
    if test "X$SCHEME_CONFIGDIR" != "X"; then
      MZSCHEME_CFLAGS="${MZSCHEME_CFLAGS} -DMZSCHEME_CONFIGDIR='\"${SCHEME_CONFIGDIR}\"'"
      AC_MSG_RESULT(${SCHEME_CONFIGDIR})
    else
      AC_MSG_RESULT(not found)
    fi

    AC_MSG_CHECKING(for racket collects directory)
    SCHEME_COLLECTS=`${vi_cv_path_mzscheme} -e '(require setup/dirs)(let ((p (find-collects-dir))) (when (path? p) (let-values (((base _1 _2) (split-path p))) (display base))))'`
    if test "X$SCHEME_COLLECTS" = "X"; then
      if test -d "$vi_cv_path_mzscheme_pfx/lib/plt/collects"; then
	SCHEME_COLLECTS=$vi_cv_path_mzscheme_pfx/lib/plt/
      else
	if test -d "$vi_cv_path_mzscheme_pfx/lib/racket/collects"; then
	  SCHEME_COLLECTS=$vi_cv_path_mzscheme_pfx/lib/racket/
	else
	  if test -d "$vi_cv_path_mzscheme_pfx/share/racket/collects"; then
	    SCHEME_COLLECTS=$vi_cv_path_mzscheme_pfx/share/racket/
	  else
	    if test -d "$vi_cv_path_mzscheme_pfx/collects"; then
	      SCHEME_COLLECTS=$vi_cv_path_mzscheme_pfx/
	    fi
	  fi
	fi
      fi
    fi
    if test "X$SCHEME_COLLECTS" != "X" ; then
      AC_MSG_RESULT(${SCHEME_COLLECTS})
    else
      AC_MSG_RESULT(not found)
    fi

    AC_MSG_CHECKING(for mzscheme_base.c)
    if test -f "${SCHEME_COLLECTS}collects/scheme/base.ss" ; then
      MZSCHEME_EXTRA="mzscheme_base.c"
      MZSCHEME_MZC="${vi_cv_path_mzscheme_pfx}/bin/mzc"
      MZSCHEME_MOD="++lib scheme/base"
    else
      if test -f "${SCHEME_COLLECTS}collects/scheme/base.rkt" ; then
	MZSCHEME_EXTRA="mzscheme_base.c"
	MZSCHEME_MZC="${vi_cv_path_mzscheme_pfx}/bin/mzc"
	MZSCHEME_MOD="++lib scheme/base"
      else
	if test -f "${SCHEME_COLLECTS}collects/racket/base.rkt" ; then
	  MZSCHEME_EXTRA="mzscheme_base.c"
	  MZSCHEME_MZC="${vi_cv_path_mzscheme_pfx}/bin/raco ctool"
	  MZSCHEME_MOD=""
	fi
      fi
    fi
    if test "X$MZSCHEME_EXTRA" != "X" ; then
      dnl need to generate bytecode for MzScheme base
      MZSCHEME_CFLAGS="${MZSCHEME_CFLAGS} -DINCLUDE_MZSCHEME_BASE"
      AC_MSG_RESULT(needed)
    else
      AC_MSG_RESULT(not needed)
    fi

    dnl On Ubuntu this fixes "undefined reference to symbol 'ffi_type_void'".
    AC_CHECK_LIB(ffi, ffi_type_void, [MZSCHEME_LIBS="$MZSCHEME_LIBS -lffi"])

    MZSCHEME_CFLAGS="${MZSCHEME_CFLAGS} -I${SCHEME_INC} \
      -DMZSCHEME_COLLECTS='\"${SCHEME_COLLECTS}collects\"'"

    dnl Test that we can compile a simple program with these CFLAGS and LIBS.
    AC_MSG_CHECKING([if compile and link flags for MzScheme are sane])
    cflags_save=$CFLAGS
    libs_save=$LIBS
    CFLAGS="$CFLAGS $MZSCHEME_CFLAGS"
    LIBS="$LIBS $MZSCHEME_LIBS"
    AC_LINK_IFELSE([AC_LANG_PROGRAM(,[ ])],
	   AC_MSG_RESULT(yes); mzs_ok=yes,
	   AC_MSG_RESULT(no: MZSCHEME DISABLED); mzs_ok=no)
    CFLAGS=$cflags_save
    LIBS=$libs_save
    if test $mzs_ok = yes; then
      MZSCHEME_SRC="if_mzsch.c"
      MZSCHEME_OBJ="objects/if_mzsch.o"
      MZSCHEME_PRO="if_mzsch.pro"
      AC_DEFINE(FEAT_MZSCHEME)
    else
      MZSCHEME_CFLAGS=
      MZSCHEME_LIBS=
      MZSCHEME_EXTRA=
      MZSCHEME_MZC=
    fi
  fi
  AC_SUBST(MZSCHEME_SRC)
  AC_SUBST(MZSCHEME_OBJ)
  AC_SUBST(MZSCHEME_PRO)
  AC_SUBST(MZSCHEME_LIBS)
  AC_SUBST(MZSCHEME_CFLAGS)
  AC_SUBST(MZSCHEME_EXTRA)
  AC_SUBST(MZSCHEME_MZC)
fi


AC_MSG_CHECKING(--enable-perlinterp argument)
AC_ARG_ENABLE(perlinterp,
	[  --enable-perlinterp[=OPTS]     Include Perl interpreter.  [default=no] [OPTS=no/yes/dynamic]], ,
	[enable_perlinterp="no"])
AC_MSG_RESULT($enable_perlinterp)
if test "$enable_perlinterp" = "yes" -o "$enable_perlinterp" = "dynamic"; then
  if test "$has_eval" = "no"; then
    AC_MSG_ERROR([cannot use Perl with tiny features])
  fi
  AC_SUBST(vi_cv_path_perl)
  AC_PATH_PROG(vi_cv_path_perl, perl)
  if test "X$vi_cv_path_perl" != "X"; then
    AC_MSG_CHECKING(Perl version)
    if $vi_cv_path_perl -e 'require 5.003_01' >/dev/null 2>/dev/null; then
     eval `$vi_cv_path_perl -V:usethreads`
     eval `$vi_cv_path_perl -V:libperl`
     if test "X$usethreads" = "XUNKNOWN" -o "X$usethreads" = "Xundef"; then
       badthreads=no
     else
       if $vi_cv_path_perl -e 'require 5.6.0' >/dev/null 2>/dev/null; then
	 eval `$vi_cv_path_perl -V:use5005threads`
	 if test "X$use5005threads" = "XUNKNOWN" -o "X$use5005threads" = "Xundef"; then
	   badthreads=no
	 else
	   badthreads=yes
	   AC_MSG_RESULT(>>> Perl > 5.6 with 5.5 threads cannot be used <<<)
	 fi
       else
	 badthreads=yes
	 AC_MSG_RESULT(>>> Perl 5.5 with threads cannot be used <<<)
       fi
     fi
     if test $badthreads = no; then
      AC_MSG_RESULT(OK)
      eval `$vi_cv_path_perl -V:shrpenv`
      if test "X$shrpenv" = "XUNKNOWN"; then # pre 5.003_04
	shrpenv=""
      fi
      vi_cv_perllib=`$vi_cv_path_perl -MConfig -e 'print $Config{privlibexp}'`
      AC_SUBST(vi_cv_perllib)
      vi_cv_perl_extutils=unknown_perl_extutils_path

      AC_MSG_CHECKING(--with-xsubpp path)
      vi_cv_perl_xsubpp=
      AC_ARG_WITH(xsubpp, [  --with-xsubpp=PATH  path to the xsubpp command], [
            if test -f "$withval"
            then
	      vi_cv_perl_xsubpp="$withval"
            fi
      ])

      if test "x$vi_cv_perl_xsubpp" = "x"
      then
	for extutils_rel_path in ExtUtils vendor_perl/ExtUtils; do
	  xsubpp_path="$vi_cv_perllib/$extutils_rel_path/xsubpp"
	  if test -f "$xsubpp_path"; then
	    vi_cv_perl_xsubpp="$xsubpp_path"
	  fi
	done
      fi

      if test "x$vi_cv_perl_xsubpp" = "x"
      then
        AC_MSG_RESULT(empty)
      else
        AC_MSG_RESULT($vi_cv_perl_xsubpp)
      fi
      AC_SUBST(vi_cv_perl_xsubpp)

      dnl Remove "-fno-something", it breaks using cproto.
      dnl Remove "-fdebug-prefix-map", it isn't supported by clang.
      dnl Remove "FORTIFY_SOURCE", it will be defined twice.
      dnl remove -pipe and -Wxxx, it confuses cproto
      perlcppflags=`$vi_cv_path_perl -Mlib=$srcdir -MExtUtils::Embed \
		-e 'ccflags;perl_inc;print"\n"' | sed -e 's/-fno[[^ ]]*//' \
		-e 's/-fdebug-prefix-map[[^ ]]*//g' \
		-e 's/-pipe //' \
		-e 's/-W[[^ ]]*//g' \
		-e 's/-D_FORTIFY_SOURCE=.//g'`
      dnl Remove "-lc", it breaks on FreeBSD when using "-pthread".
      perllibs=`cd $srcdir; $vi_cv_path_perl -MExtUtils::Embed -e 'ldopts' | \
		sed -e '/Warning/d' -e '/Note (probably harmless)/d' \
			-e 's/-bE:perl.exp//' -e 's/-lc //'`
      dnl Don't add perl lib to $LIBS: if it's not in LD_LIBRARY_PATH
      dnl a test in configure may fail because of that.
      perlldflags=`cd $srcdir; $vi_cv_path_perl -MExtUtils::Embed \
		-e 'ccdlflags' | sed -e 's/-bE:perl.exp//'`

      dnl check that compiling a simple program still works with the flags
      dnl added for Perl.
      AC_MSG_CHECKING([if compile and link flags for Perl are sane])
      cflags_save=$CFLAGS
      libs_save=$LIBS
      ldflags_save=$LDFLAGS
      CFLAGS="$CFLAGS $perlcppflags"
      LIBS="$LIBS $perllibs"
      perlldflags=`echo "$perlldflags" | sed -e 's/^ *//g'`
      LDFLAGS="$perlldflags $LDFLAGS"
      AC_LINK_IFELSE([AC_LANG_PROGRAM(,[ ])],
	     AC_MSG_RESULT(yes); perl_ok=yes,
	     AC_MSG_RESULT(no: PERL DISABLED); perl_ok=no)
      CFLAGS=$cflags_save
      LIBS=$libs_save
      LDFLAGS=$ldflags_save
      if test $perl_ok = yes; then
	if test "X$perlcppflags" != "X"; then
	  PERL_CFLAGS=$perlcppflags
	fi
	if test "X$perlldflags" != "X"; then
	  if test "X`echo \"$LDFLAGS\" | $FGREP -e \"$perlldflags\"`" = "X"; then
	    LDFLAGS="$perlldflags $LDFLAGS"
	  fi
	fi
	PERL_LIBS=$perllibs
	PERL_SRC="auto/if_perl.c if_perlsfio.c"
	PERL_OBJ="objects/if_perl.o objects/if_perlsfio.o"
	PERL_PRO="if_perl.pro if_perlsfio.pro"
	AC_DEFINE(FEAT_PERL)
      fi
     fi
    else
      AC_MSG_RESULT(>>> too old; need Perl version 5.003_01 or later <<<)
    fi
  fi

  if test "x$MACOS_X" = "xyes"; then
    dnl Mac OS X 10.2 or later
    dir=/System/Library/Perl
    darwindir=$dir/darwin
    if test -d $darwindir; then
      PERL=/usr/bin/perl
    else
      dnl Mac OS X 10.3
      dir=/System/Library/Perl/5.8.1
      darwindir=$dir/darwin-thread-multi-2level
      if test -d $darwindir; then
	PERL=/usr/bin/perl
      fi
    fi
    if test -n "$PERL"; then
      PERL_DIR="$dir"
      PERL_CFLAGS="-DFEAT_PERL -I$darwindir/CORE"
      PERL_OBJ="objects/if_perl.o objects/if_perlsfio.o $darwindir/auto/DynaLoader/DynaLoader.a"
      PERL_LIBS="-L$darwindir/CORE -lperl"
    fi
    dnl Perl on Mac OS X 10.5 adds "-arch" flags but these should only
    dnl be included if requested by passing --with-mac-arch to
    dnl configure, so strip these flags first (if present)
    PERL_LIBS=`echo "$PERL_LIBS" | sed -e 's/-arch\ ppc//' -e 's/-arch\ i386//' -e 's/-arch\ x86_64//'`
    PERL_CFLAGS=`echo "$PERL_CFLAGS" | sed -e 's/-arch\ ppc//' -e 's/-arch\ i386//' -e 's/-arch\ x86_64//'`
  fi
  if test "$enable_perlinterp" = "dynamic"; then
    if test "$perl_ok" = "yes" -a "X$libperl" != "X"; then
      AC_DEFINE(DYNAMIC_PERL)
      PERL_CFLAGS="-DDYNAMIC_PERL_DLL=\\\"$libperl\\\" $PERL_CFLAGS"
      PERL_LIBS=""
    fi
  fi

  if test "$fail_if_missing" = "yes" -a "$perl_ok" != "yes"; then
    AC_MSG_ERROR([could not configure perl])
  fi
fi
AC_SUBST(shrpenv)
AC_SUBST(PERL_SRC)
AC_SUBST(PERL_OBJ)
AC_SUBST(PERL_PRO)
AC_SUBST(PERL_CFLAGS)
AC_SUBST(PERL_CFLAGS_EXTRA)
AC_SUBST(PERL_LIBS)

AC_MSG_CHECKING(--enable-pythoninterp argument)
AC_ARG_ENABLE(pythoninterp,
	[  --enable-pythoninterp[=OPTS]   Include Python interpreter. [default=no] [OPTS=no/yes/dynamic]], ,
	[enable_pythoninterp="no"])
AC_MSG_RESULT($enable_pythoninterp)
if test "$enable_pythoninterp" = "yes" -o "$enable_pythoninterp" = "dynamic"; then
  if test "$has_eval" = "no"; then
    AC_MSG_ERROR([cannot use Python with tiny features])
  fi

  dnl -- find the python executable
  AC_MSG_CHECKING(--with-python-command argument)
  AC_SUBST(vi_cv_path_python)
  AC_ARG_WITH(python-command, [  --with-python-command=NAME  name of the Python 2 command (default: python2 or python)],
	vi_cv_path_python="$withval"; AC_MSG_RESULT($vi_cv_path_python),
	AC_MSG_RESULT(no))

  if test "X$vi_cv_path_python" = "X"; then
    AC_PATH_PROGS(vi_cv_path_python, python2 python)
  fi
  if test "X$vi_cv_path_python" != "X"; then

    dnl -- get its version number
    AC_CACHE_CHECK(Python version,vi_cv_var_python_version,
    [[vi_cv_var_python_version=`
	    ${vi_cv_path_python} -c 'import sys; print sys.version[:3]'`
    ]])

    dnl -- it must be at least version 2.3
    AC_MSG_CHECKING(Python is 2.3 or better)
    if ${vi_cv_path_python} -c \
	"import sys; sys.exit(${vi_cv_var_python_version} < 2.3)"
    then
      AC_MSG_RESULT(yep)

      dnl -- find where python thinks it was installed
      AC_CACHE_CHECK(Python's install prefix,vi_cv_path_python_pfx,
      [ vi_cv_path_python_pfx=`
	    ${vi_cv_path_python} -c \
		"import sys; print sys.prefix"` ])

      dnl -- and where it thinks it runs
      AC_CACHE_CHECK(Python's execution prefix,vi_cv_path_python_epfx,
      [ vi_cv_path_python_epfx=`
	    ${vi_cv_path_python} -c \
		"import sys; print sys.exec_prefix"` ])

      dnl -- python's internal library path

      AC_CACHE_VAL(vi_cv_path_pythonpath,
      [ vi_cv_path_pythonpath=`
	    unset PYTHONPATH;
	    ${vi_cv_path_python} -c \
		"import sys, string; print string.join(sys.path,':')"` ])

      dnl -- where the Python implementation library archives are

      AC_ARG_WITH(python-config-dir,
	[  --with-python-config-dir=PATH  Python's config directory (deprecated)],
	[ vi_cv_path_python_conf="${withval}"; have_python_config_dir=1 ] )

      AC_CACHE_CHECK(Python's configuration directory,vi_cv_path_python_conf,
      [
	vi_cv_path_python_conf=
	d=`${vi_cv_path_python} -c "import distutils.sysconfig; print distutils.sysconfig.get_config_var('LIBPL')"`
	if test -d "$d" && test -f "$d/config.c"; then
	  vi_cv_path_python_conf="$d"
	else
	  for path in "${vi_cv_path_python_pfx}" "${vi_cv_path_python_epfx}"; do
	    for subdir in lib64 lib share; do
	      d="${path}/${subdir}/python${vi_cv_var_python_version}/config"
	      if test -d "$d" && test -f "$d/config.c"; then
		vi_cv_path_python_conf="$d"
	      fi
	    done
	  done
	fi
      ])

      PYTHON_CONFDIR="${vi_cv_path_python_conf}"

      if test "X$PYTHON_CONFDIR" = "X"; then
	AC_MSG_RESULT([can't find it!])
      else

	dnl -- we need to examine Python's config/Makefile too
	dnl    see what the interpreter is built from
	AC_CACHE_VAL(vi_cv_path_python_plibs,
	[
	    pwd=`pwd`
	    tmp_mkf="$pwd/config-PyMake$$"
	    cat -- "${PYTHON_CONFDIR}/Makefile" - <<'eof' >"${tmp_mkf}"
__:
	@echo "python_BASEMODLIBS='$(BASEMODLIBS)'"
	@echo "python_LIBS='$(LIBS)'"
	@echo "python_SYSLIBS='$(SYSLIBS)'"
	@echo "python_LINKFORSHARED='$(LINKFORSHARED)'"
	@echo "python_DLLLIBRARY='$(DLLLIBRARY)'"
	@echo "python_INSTSONAME='$(INSTSONAME)'"
	@echo "python_PYTHONFRAMEWORK='$(PYTHONFRAMEWORK)'"
	@echo "python_PYTHONFRAMEWORKPREFIX='$(PYTHONFRAMEWORKPREFIX)'"
	@echo "python_PYTHONFRAMEWORKINSTALLDIR='$(PYTHONFRAMEWORKINSTALLDIR)'"
eof
	    dnl -- delete the lines from make about Entering/Leaving directory
	    eval "`cd ${PYTHON_CONFDIR} && make -f "${tmp_mkf}" __ | sed '/ directory /d'`"
	    rm -f -- "${tmp_mkf}"
	    if test "x$MACOS_X" = "xyes" && test -n "${python_PYTHONFRAMEWORK}" && ${vi_cv_path_python} -c \
		"import sys; sys.exit(${vi_cv_var_python_version} < 2.3)"; then
	      vi_cv_path_python_plibs="-framework Python"
	      if test "x${vi_cv_path_python}" != "x/usr/bin/python" && test -n "${python_PYTHONFRAMEWORKPREFIX}"; then
		  vi_cv_path_python_plibs="-F${python_PYTHONFRAMEWORKPREFIX} -framework Python"
	      fi
	    else
	      vi_cv_path_python_plibs="-L${PYTHON_CONFDIR} -lpython${vi_cv_var_python_version}"
	      dnl -- Check if the path contained in python_LINKFORSHARED is
	      dnl    usable for vim build. If not, make and try other
	      dnl    candidates.
	      if test -n "${python_LINKFORSHARED}" && test -n "${python_PYTHONFRAMEWORKPREFIX}"; then
	        python_link_symbol=`echo ${python_LINKFORSHARED} | sed 's/\([[^ \t]][[^ \t]]*[[ \t]][[ \t]]*[[^ \t]][[^ \t]]*\)[[ \t]].*/\1/'`
		python_link_path=`echo ${python_LINKFORSHARED} |   sed 's/\([[^ \t]][[^ \t]]*[[ \t]][[ \t]]*[[^ \t]][[^ \t]]*\)[[ \t]][[ \t]]*\(.*\)/\2/'`
	        if test -n "${python_link_path}" && ! test -x "${python_link_path}"; then
	          dnl -- The path looks relative. Guess the absolute one using
		  dnl    the prefix and try that.
	          python_link_path="${python_PYTHONFRAMEWORKPREFIX}/${python_link_path}"
		  if test -n "${python_link_path}" && ! test -x "${python_link_path}"; then
		    dnl -- A last resort.
		    python_link_path="${python_PYTHONFRAMEWORKINSTALLDIR}/Versions/${vi_cv_var_python_version}/${python_PYTHONFRAMEWORK}"
	            dnl -- No check is done. The last word is left to the
	            dnl    "sanity" test on link flags that follows shortly.
		  fi
	          python_LINKFORSHARED="${python_link_symbol} ${python_link_path}"
	        fi
	      fi
	      vi_cv_path_python_plibs="${vi_cv_path_python_plibs} ${python_BASEMODLIBS} ${python_LIBS} ${python_SYSLIBS} ${python_LINKFORSHARED}"
	      dnl remove -ltermcap, it can conflict with an earlier -lncurses
	      vi_cv_path_python_plibs=`echo $vi_cv_path_python_plibs | sed s/-ltermcap//`
	    fi
	])
	AC_CACHE_CHECK(Python's dll name,vi_cv_dll_name_python,
	[
	  if test "X$python_DLLLIBRARY" != "X"; then
	    vi_cv_dll_name_python="$python_DLLLIBRARY"
	  else
	    vi_cv_dll_name_python="$python_INSTSONAME"
	  fi
	])

	PYTHON_LIBS="${vi_cv_path_python_plibs}"
	if test "${vi_cv_path_python_pfx}" = "${vi_cv_path_python_epfx}"; then
	  PYTHON_CFLAGS="-I${vi_cv_path_python_pfx}/include/python${vi_cv_var_python_version}"
	else
	  PYTHON_CFLAGS="-I${vi_cv_path_python_pfx}/include/python${vi_cv_var_python_version} -I${vi_cv_path_python_epfx}/include/python${vi_cv_var_python_version}"
	fi
	if test "X$have_python_config_dir" = "X1" -a "$enable_pythoninterp" = "dynamic"; then
	  dnl Define PYTHON_HOME if --with-python-config-dir was used
	  PYTHON_CFLAGS="${PYTHON_CFLAGS} -DPYTHON_HOME='\"${vi_cv_path_python_pfx}\"'"

	fi
	PYTHON_SRC="if_python.c"
	PYTHON_OBJ="objects/if_python.o"

	dnl On FreeBSD linking with "-pthread" is required to use threads.
	dnl _THREAD_SAFE must be used for compiling then.
	dnl The "-pthread" is added to $LIBS, so that the following check for
	dnl sigaltstack() will look in libc_r (it's there in libc!).
	dnl Otherwise, when using GCC, try adding -pthread to $CFLAGS.  GCC
	dnl will then define target-specific defines, e.g., -D_REENTRANT.
	dnl Don't do this for Mac OSX, -pthread will generate a warning.
	AC_MSG_CHECKING([if -pthread should be used])
	threadsafe_flag=
	thread_lib=
	dnl if test "x$MACOS_X" != "xyes"; then
        if test "$vim_cv_uname_output" != Darwin; then
	  test "$GCC" = yes && threadsafe_flag="-pthread"
	  if test "$vim_cv_uname_output" = FreeBSD; then
	    threadsafe_flag="-D_THREAD_SAFE"
	    thread_lib="-pthread"
	  fi
	  if test "$vim_cv_uname_output" = SunOS; then
	    threadsafe_flag="-pthreads"
	  fi
	fi
	libs_save_old=$LIBS
	if test -n "$threadsafe_flag"; then
	  cflags_save=$CFLAGS
	  CFLAGS="$CFLAGS $threadsafe_flag"
	  LIBS="$LIBS $thread_lib"
	  AC_LINK_IFELSE([AC_LANG_PROGRAM(,[ ])],
	     AC_MSG_RESULT(yes); PYTHON_CFLAGS="$PYTHON_CFLAGS $threadsafe_flag",
	     AC_MSG_RESULT(no); LIBS=$libs_save_old
	     )
	  CFLAGS=$cflags_save
	else
	  AC_MSG_RESULT(no)
	fi

	dnl Check that compiling a simple program still works with the flags
	dnl added for Python.
	AC_MSG_CHECKING([if compile and link flags for Python are sane])
	cflags_save=$CFLAGS
	libs_save=$LIBS
	CFLAGS="$CFLAGS $PYTHON_CFLAGS"
	LIBS="$LIBS $PYTHON_LIBS"
	AC_LINK_IFELSE([AC_LANG_PROGRAM(,[ ])],
	       AC_MSG_RESULT(yes); python_ok=yes,
	       AC_MSG_RESULT(no: PYTHON DISABLED); python_ok=no)
	CFLAGS=$cflags_save
	LIBS=$libs_save
	if test $python_ok = yes; then
	  AC_DEFINE(FEAT_PYTHON)
	else
	  LIBS=$libs_save_old
	  PYTHON_SRC=
	  PYTHON_OBJ=
	  PYTHON_LIBS=
	  PYTHON_CFLAGS=
	fi
      fi
    else
      AC_MSG_RESULT(too old)
    fi
  fi

  if test "$fail_if_missing" = "yes" -a "$python_ok" != "yes"; then
    AC_MSG_ERROR([could not configure python])
  fi
fi

AC_SUBST(PYTHON_LIBS)
AC_SUBST(PYTHON_CFLAGS)
AC_SUBST(PYTHON_CFLAGS_EXTRA)
AC_SUBST(PYTHON_SRC)
AC_SUBST(PYTHON_OBJ)


AC_MSG_CHECKING(--enable-python3interp argument)
AC_ARG_ENABLE(python3interp,
	[  --enable-python3interp[=OPTS]  Include Python3 interpreter. [default=no] [OPTS=no/yes/dynamic]], ,
	[enable_python3interp="no"])
AC_MSG_RESULT($enable_python3interp)
if test "$enable_python3interp" = "yes" -o "$enable_python3interp" = "dynamic"; then
  if test "$has_eval" = "no"; then
    AC_MSG_ERROR([cannot use Python with tiny features])
  fi

  dnl -- find the python3 executable
  AC_MSG_CHECKING(--with-python3-command argument)
  AC_SUBST(vi_cv_path_python3)
  AC_ARG_WITH(python3-command, [  --with-python3-command=NAME  name of the Python 3 command (default: python3 or python)],
	vi_cv_path_python3="$withval"; AC_MSG_RESULT($vi_cv_path_python3),
	AC_MSG_RESULT(no))

  if test "X$vi_cv_path_python3" = "X"; then
    AC_PATH_PROGS(vi_cv_path_python3, python3 python)
  fi
  if test "X$vi_cv_path_python3" != "X"; then

    dnl -- get its version number
    AC_CACHE_CHECK(Python version,vi_cv_var_python3_version,
    [[vi_cv_var_python3_version=`
          ${vi_cv_path_python3} -c 'import sys; print("{}.{}".format(sys.version_info.major, sys.version_info.minor))'`
    ]])

    dnl -- it must be at least version 3
    AC_MSG_CHECKING(Python is 3.0 or better)
    if ${vi_cv_path_python3} -c \
      "import sys; sys.exit(${vi_cv_var_python3_version} < 3.0)"
    then
      AC_MSG_RESULT(yep)

      dnl -- get the stable ABI version if passed in
      AC_MSG_CHECKING(--with-python3-stable-abi argument)
      AC_SUBST(vi_cv_var_python3_stable_abi)
      AC_ARG_WITH(python3-stable-abi, [  --with-python3-stable-abi=VERSION  stable ABI version to target (e.g. 3.8)],
        vi_cv_var_python3_stable_abi="$withval"; AC_MSG_RESULT($vi_cv_var_python3_stable_abi),
        AC_MSG_RESULT(no))
      if test "X$vi_cv_var_python3_stable_abi" != "X"; then
        AC_CACHE_VAL(vi_cv_var_python3_stable_abi_hex,
        [
         vi_cv_var_python3_stable_abi_hex=`
         ${vi_cv_path_python3} -c \
         "major_minor='${vi_cv_var_python3_stable_abi}'.split('.'); print('0x{0:X}'.format( (int(major_minor.__getitem__(0))<<24) + (int(major_minor.__getitem__(1))<<16) ))"` ])
        if test "X$vi_cv_var_python3_stable_abi_hex" = "X"; then
          AC_MSG_ERROR([can't parse Python 3 stable ABI version. It should be "<major>.<minor>"])
        fi
      fi

      dnl -- get abiflags for python 3.2 or higher (PEP 3149)
      AC_CACHE_CHECK(Python's abiflags,vi_cv_var_python3_abiflags,
      [
       vi_cv_var_python3_abiflags=
       if ${vi_cv_path_python3} -c \
           "import sys; sys.exit(${vi_cv_var_python3_version} < 3.2)"
       then
         vi_cv_var_python3_abiflags=`${vi_cv_path_python3} -c \
           "import sys; print(sys.abiflags)"`
       fi ])

      dnl -- find where python3 thinks it was installed
      AC_CACHE_CHECK(Python's install prefix,vi_cv_path_python3_pfx,
      [ vi_cv_path_python3_pfx=`
       ${vi_cv_path_python3} -c \
       "import sys; print(sys.prefix)"` ])

      dnl -- and where it thinks it runs
      AC_CACHE_CHECK(Python's execution prefix,vi_cv_path_python3_epfx,
      [ vi_cv_path_python3_epfx=`
       ${vi_cv_path_python3} -c \
       "import sys; print(sys.exec_prefix)"` ])

      dnl -- python3's include path
      AC_CACHE_CHECK(Python's include path,vi_cv_path_python3_include,
      [ vi_cv_path_python3_include=`
       ${vi_cv_path_python3} -c \
       "import sysconfig; print(sysconfig.get_path(\"include\"))"` ])

      dnl -- python3's platform include path
      AC_CACHE_CHECK(Python's include path,vi_cv_path_python3_platinclude,
      [ vi_cv_path_python3_platinclude=`
       ${vi_cv_path_python3} -c \
       "import sysconfig; print(sysconfig.get_path(\"platinclude\"))"` ])

      dnl -- python3's internal library path

      AC_CACHE_VAL(vi_cv_path_python3path,
      [ vi_cv_path_python3path=`
       unset PYTHONPATH;
       ${vi_cv_path_python3} -c \
       "import sys, string; print(':'.join(sys.path))"` ])

      dnl -- where the Python implementation library archives are

      AC_ARG_WITH(python3-config-dir,
       [  --with-python3-config-dir=PATH  Python's config directory (deprecated)],
       [ vi_cv_path_python3_conf="${withval}"; have_python3_config_dir=1 ] )

      AC_CACHE_CHECK(Python's configuration directory,vi_cv_path_python3_conf,
      [
       vi_cv_path_python3_conf=
       config_dir="config-${vi_cv_var_python3_version}${vi_cv_var_python3_abiflags}"
       d=`${vi_cv_path_python3} -c "import sysconfig; print(sysconfig.get_config_var('LIBPL'))" 2> /dev/null`
       if test "x$d" = "x"; then
         d=`${vi_cv_path_python3} -c "import distutils.sysconfig; print(distutils.sysconfig.get_config_var('LIBPL'))"`
       fi
       if test -d "$d" && test -f "$d/config.c"; then
         vi_cv_path_python3_conf="$d"
       else
         for path in "${vi_cv_path_python3_pfx}" "${vi_cv_path_python3_epfx}"; do
	   for subdir in lib64 lib share; do
	     d="${path}/${subdir}/python${vi_cv_var_python3_version}/${config_dir}"
	     if test -d "$d" && test -f "$d/config.c"; then
	       vi_cv_path_python3_conf="$d"
	     fi
	   done
         done
       fi
      ])

      PYTHON3_CONFDIR="${vi_cv_path_python3_conf}"

      if test "X$PYTHON3_CONFDIR" = "X"; then
        AC_MSG_RESULT([can't find it!])
      else

        dnl -- we need to examine Python's config/Makefile too
        dnl    see what the interpreter is built from
        AC_CACHE_VAL(vi_cv_path_python3_plibs,
        [
            pwd=`pwd`
            tmp_mkf="$pwd/config-PyMake$$"
            cat -- "${PYTHON3_CONFDIR}/Makefile" - <<'eof' >"${tmp_mkf}"
__:
	@echo "python3_BASEMODLIBS='$(BASEMODLIBS)'"
	@echo "python3_LIBS='$(LIBS)'"
	@echo "python3_SYSLIBS='$(SYSLIBS)'"
	@echo "python3_DLLLIBRARY='$(DLLLIBRARY)'"
	@echo "python3_INSTSONAME='$(INSTSONAME)'"
	@echo "python3_PYTHONFRAMEWORKPREFIX='$(PYTHONFRAMEWORKPREFIX)'"
eof
	    dnl -- delete the lines from make about Entering/Leaving directory
	    eval "`cd ${PYTHON3_CONFDIR} && make -f "${tmp_mkf}" __ | sed '/ directory /d'`"
	    rm -f -- "${tmp_mkf}"
	    vi_cv_path_python3_plibs="-L${PYTHON3_CONFDIR} -lpython${vi_cv_var_python3_version}${vi_cv_var_python3_abiflags}"
	    vi_cv_path_python3_plibs="${vi_cv_path_python3_plibs} ${python3_BASEMODLIBS} ${python3_LIBS} ${python3_SYSLIBS}"
	    dnl remove -ltermcap, it can conflict with an earlier -lncurses
	    vi_cv_path_python3_plibs=`echo $vi_cv_path_python3_plibs | sed s/-ltermcap//`
	    vi_cv_path_python3_plibs=`echo $vi_cv_path_python3_plibs | sed s/-lffi//`
	])
	AC_CACHE_CHECK(Python3's dll name,vi_cv_dll_name_python3,
	[
	  if test "X$python3_DLLLIBRARY" != "X"; then
	    vi_cv_dll_name_python3="$python3_DLLLIBRARY"
	  elif test "X$python3_PYTHONFRAMEWORKPREFIX" != "X"; then
	    vi_cv_dll_name_python3="${python3_PYTHONFRAMEWORKPREFIX}/${python3_INSTSONAME}"
	  else
	    vi_cv_dll_name_python3="$python3_INSTSONAME"
	  fi
	])

        PYTHON3_LIBS="${vi_cv_path_python3_plibs}"
        if test "${vi_cv_path_python3_include}" = "${vi_cv_path_python3_platinclude}"; then
          PYTHON3_CFLAGS="-I${vi_cv_path_python3_include}"
        else
          PYTHON3_CFLAGS="-I${vi_cv_path_python3_include} -I${vi_cv_path_python3_platinclude}"
        fi
        if test "X$have_python3_config_dir" = "X1" -a "$enable_python3interp" = "dynamic"; then
          dnl Define PYTHON3_HOME if --with-python-config-dir was used
          PYTHON3_CFLAGS="${PYTHON3_CFLAGS}  -DPYTHON3_HOME='L\"${vi_cv_path_python3_pfx}\"'"
        fi
        if test "X$vi_cv_var_python3_stable_abi_hex" != "X"; then
          PYTHON3_CFLAGS="${PYTHON3_CFLAGS} -DPy_LIMITED_API=${vi_cv_var_python3_stable_abi_hex}"
        fi
        PYTHON3_SRC="if_python3.c"
        PYTHON3_OBJ="objects/if_python3.o"

        dnl On FreeBSD linking with "-pthread" is required to use threads.
        dnl _THREAD_SAFE must be used for compiling then.
        dnl The "-pthread" is added to $LIBS, so that the following check for
        dnl sigaltstack() will look in libc_r (it's there in libc!).
        dnl Otherwise, when using GCC, try adding -pthread to $CFLAGS.  GCC
        dnl will then define target-specific defines, e.g., -D_REENTRANT.
        dnl Don't do this for Mac OSX, -pthread will generate a warning.
        AC_MSG_CHECKING([if -pthread should be used])
        threadsafe_flag=
        thread_lib=
        dnl if test "x$MACOS_X" != "xyes"; then
        if test "$vim_cv_uname_output" != Darwin; then
          test "$GCC" = yes && threadsafe_flag="-pthread"
          if test "$vim_cv_uname_output" = FreeBSD; then
            threadsafe_flag="-D_THREAD_SAFE"
            thread_lib="-pthread"
          fi
          if test "$vim_cv_uname_output" = SunOS; then
            threadsafe_flag="-pthreads"
          fi
        fi
        libs_save_old=$LIBS
        if test -n "$threadsafe_flag"; then
          cflags_save=$CFLAGS
          CFLAGS="$CFLAGS $threadsafe_flag"
          LIBS="$LIBS $thread_lib"
          AC_LINK_IFELSE([AC_LANG_PROGRAM(,[ ])],
             AC_MSG_RESULT(yes); PYTHON3_CFLAGS="$PYTHON3_CFLAGS $threadsafe_flag",
             AC_MSG_RESULT(no); LIBS=$libs_save_old
             )
          CFLAGS=$cflags_save
        else
          AC_MSG_RESULT(no)
        fi

        dnl check that compiling a simple program still works with the flags
        dnl added for Python.
        AC_MSG_CHECKING([if compile and link flags for Python 3 are sane])
        cflags_save=$CFLAGS
        libs_save=$LIBS
        CFLAGS="$CFLAGS $PYTHON3_CFLAGS"
        LIBS="$LIBS $PYTHON3_LIBS"
        AC_LINK_IFELSE([AC_LANG_PROGRAM(,[ ])],
               AC_MSG_RESULT(yes); python3_ok=yes,
               AC_MSG_RESULT(no: PYTHON3 DISABLED); python3_ok=no)
        CFLAGS=$cflags_save
        LIBS=$libs_save
        if test "$python3_ok" = yes; then
          AC_DEFINE(FEAT_PYTHON3)
        else
          LIBS=$libs_save_old
          PYTHON3_SRC=
          PYTHON3_OBJ=
          PYTHON3_LIBS=
          PYTHON3_CFLAGS=
        fi
      fi
    else
      AC_MSG_RESULT(too old)
    fi
  fi
  if test "$fail_if_missing" = "yes" -a "$python3_ok" != "yes"; then
    AC_MSG_ERROR([could not configure python3])
  fi
fi

AC_SUBST(PYTHON3_LIBS)
AC_SUBST(PYTHON3_CFLAGS)
AC_SUBST(PYTHON3_CFLAGS_EXTRA)
AC_SUBST(PYTHON3_SRC)
AC_SUBST(PYTHON3_OBJ)

dnl if python2.x and python3.x are enabled one can only link in code
dnl with dlopen(), dlsym(), dlclose()
if test "$python_ok" = yes && test "$python3_ok" = yes; then
  AC_DEFINE(DYNAMIC_PYTHON)
  AC_DEFINE(DYNAMIC_PYTHON3)
  if test "X$vi_cv_var_python3_stable_abi_hex" != "X"; then
    AC_DEFINE(DYNAMIC_PYTHON3_STABLE_ABI)
  fi
  AC_MSG_CHECKING(whether we can do without RTLD_GLOBAL for Python)
  cflags_save=$CFLAGS
  CFLAGS="$CFLAGS $PYTHON_CFLAGS"
  libs_save=$LIBS
  dnl -ldl must go first to make this work on Archlinux (Roland Puntaier)
  LIBS="-ldl $LIBS"
  AC_RUN_IFELSE([AC_LANG_SOURCE([
    #include <dlfcn.h>
    /* If this program fails, then RTLD_GLOBAL is needed.
     * RTLD_GLOBAL will be used and then it is not possible to
     * have both python versions enabled in the same vim instance.
     * Only the first python version used will be switched on.
     */

    static int no_rtl_global_needed_for(char *python_instsoname, char *prefix)
    {
      int needed = 0;
      void* pylib = dlopen(python_instsoname, RTLD_LAZY|RTLD_LOCAL);
      if (pylib != 0)
      {
          void (*pfx)(char *home) = dlsym(pylib, "Py_SetPythonHome");
          void (*init)(void) = dlsym(pylib, "Py_Initialize");
          int (*simple)(char*) = dlsym(pylib, "PyRun_SimpleString");
          void (*final)(void) = dlsym(pylib, "Py_Finalize");
          (*pfx)(prefix);
          (*init)();
          needed = (*simple)("import termios") == -1;
          (*final)();
          dlclose(pylib);
      }
      return !needed;
    }

    int main()
    {
      int not_needed = 0;
      if (no_rtl_global_needed_for("${vi_cv_dll_name_python}", "${vi_cv_path_python_pfx}"))
            not_needed = 1;
      return !not_needed;
    }])],
    [AC_MSG_RESULT(yes);AC_DEFINE(PY_NO_RTLD_GLOBAL)], [AC_MSG_RESULT(no)])

  CFLAGS=$cflags_save
  LIBS=$libs_save

  AC_MSG_CHECKING(whether we can do without RTLD_GLOBAL for Python3)
  cflags_save=$CFLAGS
  CFLAGS="$CFLAGS $PYTHON3_CFLAGS"
  libs_save=$LIBS
  dnl -ldl must go first to make this work on Archlinux (Roland Puntaier)
  LIBS="-ldl $LIBS"
  AC_RUN_IFELSE([AC_LANG_SOURCE([
    #include <dlfcn.h>
    #include <wchar.h>
    /* If this program fails, then RTLD_GLOBAL is needed.
     * RTLD_GLOBAL will be used and then it is not possible to
     * have both python versions enabled in the same vim instance.
     * Only the first python version used will be switched on.
     */

    static int no_rtl_global_needed_for(char *python_instsoname, wchar_t *prefix)
    {
      int needed = 0;
      void* pylib = dlopen(python_instsoname, RTLD_LAZY|RTLD_LOCAL);
      if (pylib != 0)
      {
          void (*pfx)(wchar_t *home) = dlsym(pylib, "Py_SetPythonHome");
          void (*init)(void) = dlsym(pylib, "Py_Initialize");
          int (*simple)(char*) = dlsym(pylib, "PyRun_SimpleString");
          void (*final)(void) = dlsym(pylib, "Py_Finalize");
          (*pfx)(prefix);
          (*init)();
          needed = (*simple)("import termios") == -1;
          (*final)();
          dlclose(pylib);
      }
      return !needed;
    }

    int main()
    {
      int not_needed = 0;
      if (no_rtl_global_needed_for("${vi_cv_dll_name_python3}", L"${vi_cv_path_python3_pfx}"))
            not_needed = 1;
      return !not_needed;
    }])],
    [AC_MSG_RESULT(yes);AC_DEFINE(PY3_NO_RTLD_GLOBAL)], [AC_MSG_RESULT(no)])

  CFLAGS=$cflags_save
  LIBS=$libs_save

  PYTHON_SRC="if_python.c"
  PYTHON_OBJ="objects/if_python.o"
  PYTHON_CFLAGS="$PYTHON_CFLAGS -DDYNAMIC_PYTHON_DLL=\\\"${vi_cv_dll_name_python}\\\""
  PYTHON_LIBS=
  PYTHON3_SRC="if_python3.c"
  PYTHON3_OBJ="objects/if_python3.o"
  PYTHON3_CFLAGS="$PYTHON3_CFLAGS -DDYNAMIC_PYTHON3_DLL=\\\"${vi_cv_dll_name_python3}\\\""
  PYTHON3_LIBS=
elif test "$python_ok" = yes && test "$enable_pythoninterp" = "dynamic"; then
  AC_DEFINE(DYNAMIC_PYTHON)
  PYTHON_SRC="if_python.c"
  PYTHON_OBJ="objects/if_python.o"
  PYTHON_CFLAGS="$PYTHON_CFLAGS -DDYNAMIC_PYTHON_DLL=\\\"${vi_cv_dll_name_python}\\\""
  PYTHON_LIBS=
elif test "$python_ok" = yes; then
  dnl Check that adding -fPIE works.  It may be needed when using a static
  dnl Python library.
  AC_MSG_CHECKING([if -fPIE can be added for Python])
  cflags_save=$CFLAGS
  libs_save=$LIBS
  CFLAGS="$CFLAGS $PYTHON_CFLAGS -fPIE"
  LIBS="$LIBS $PYTHON_LIBS"
  AC_LINK_IFELSE([AC_LANG_PROGRAM(,[ ])],
	 AC_MSG_RESULT(yes); fpie_ok=yes,
	 AC_MSG_RESULT(no); fpie_ok=no)
  CFLAGS=$cflags_save
  LIBS=$libs_save
  if test $fpie_ok = yes; then
    PYTHON_CFLAGS="$PYTHON_CFLAGS -fPIE"
  fi
elif test "$python3_ok" = yes && test "$enable_python3interp" = "dynamic"; then
  AC_DEFINE(DYNAMIC_PYTHON3)
  if test "X$vi_cv_var_python3_stable_abi_hex" != "X"; then
    AC_DEFINE(DYNAMIC_PYTHON3_STABLE_ABI)
  fi
  PYTHON3_SRC="if_python3.c"
  PYTHON3_OBJ="objects/if_python3.o"
  PYTHON3_CFLAGS="$PYTHON3_CFLAGS -DDYNAMIC_PYTHON3_DLL=\\\"${vi_cv_dll_name_python3}\\\""
  PYTHON3_LIBS=
elif test "$python3_ok" = yes; then
  dnl Check that adding -fPIE works.  It may be needed when using a static
  dnl Python library.
  AC_MSG_CHECKING([if -fPIE can be added for Python3])
  cflags_save=$CFLAGS
  libs_save=$LIBS
  CFLAGS="$CFLAGS $PYTHON3_CFLAGS -fPIE"
  LIBS="$LIBS $PYTHON3_LIBS"
  AC_LINK_IFELSE([AC_LANG_PROGRAM(,[ ])],
	 AC_MSG_RESULT(yes); fpie_ok=yes,
	 AC_MSG_RESULT(no); fpie_ok=no)
  CFLAGS=$cflags_save
  LIBS=$libs_save
  if test $fpie_ok = yes; then
    PYTHON3_CFLAGS="$PYTHON3_CFLAGS -fPIE"
  fi
fi

AC_MSG_CHECKING(--enable-tclinterp argument)
AC_ARG_ENABLE(tclinterp,
	[  --enable-tclinterp[=OPTS]      Include Tcl interpreter. [default=no] [OPTS=no/yes/dynamic]], ,
	[enable_tclinterp="no"])
AC_MSG_RESULT($enable_tclinterp)

if test "$enable_tclinterp" = "yes" -o "$enable_tclinterp" = "dynamic"; then

  dnl on FreeBSD tclsh is a silly script, look for tclsh8.[5420]
  AC_MSG_CHECKING(--with-tclsh argument)
  AC_ARG_WITH(tclsh, [  --with-tclsh=PATH       which tclsh to use (default: tclsh8.0)],
	tclsh_name="$withval"; AC_MSG_RESULT($tclsh_name),
	tclsh_name="tclsh8.5"; AC_MSG_RESULT(no))
  AC_PATH_PROG(vi_cv_path_tcl, $tclsh_name)
  AC_SUBST(vi_cv_path_tcl)

  dnl when no specific version specified, also try 8.4, 8.2 and 8.0
  if test "X$vi_cv_path_tcl" = "X" -a $tclsh_name = "tclsh8.5"; then
    tclsh_name="tclsh8.4"
    AC_PATH_PROG(vi_cv_path_tcl, $tclsh_name)
  fi
  if test "X$vi_cv_path_tcl" = "X" -a $tclsh_name = "tclsh8.4"; then
    tclsh_name="tclsh8.2"
    AC_PATH_PROG(vi_cv_path_tcl, $tclsh_name)
  fi
  if test "X$vi_cv_path_tcl" = "X" -a $tclsh_name = "tclsh8.2"; then
    tclsh_name="tclsh8.0"
    AC_PATH_PROG(vi_cv_path_tcl, $tclsh_name)
  fi
  dnl still didn't find it, try without version number
  if test "X$vi_cv_path_tcl" = "X"; then
    tclsh_name="tclsh"
    AC_PATH_PROG(vi_cv_path_tcl, $tclsh_name)
  fi
  if test "X$vi_cv_path_tcl" != "X"; then
    AC_MSG_CHECKING(Tcl version)
    if echo 'exit [[expr [info tclversion] < 8.0]]' | "$vi_cv_path_tcl" - ; then
      tclver=`echo 'puts [[info tclversion]]' | $vi_cv_path_tcl -`
      AC_MSG_RESULT($tclver - OK);
      tclloc=`echo 'set l [[info library]];set i [[string last lib $l]];incr i -2;puts [[string range $l 0 $i]]' | $vi_cv_path_tcl -`
      tcldll=`echo 'puts libtcl[[info tclversion]][[info sharedlibextension]]' | $vi_cv_path_tcl -`

      AC_MSG_CHECKING(for location of Tcl include)
      if test "x$MACOS_X" != "xyes"; then
	tclinc="$tclloc/include $tclloc/include/tcl $tclloc/include/tcl$tclver /usr/local/include /usr/local/include/tcl$tclver /usr/include /usr/include/tcl$tclver"
      else
	dnl For all macOS, use the value from TCL in case use of, say, homebrew
	dnl For Mac OS X 10.3, use the OS-provided framework location
	dnl For Mac OS X 10.14, the OS-provided framework location doesn't contain the headers, so also check the Xcode SDK
	tclinc="$tclloc/include $tclloc/include/tcl $tclloc/include/tcl$tclver /System/Library/Frameworks/Tcl.framework/Headers `xcrun --show-sdk-path`/System/Library/Frameworks/Tcl.framework/Versions/Current/Headers"
      fi
      TCL_INC=
      for try in $tclinc; do
	if test -f "$try/tcl.h"; then
	  AC_MSG_RESULT($try/tcl.h)
	  TCL_INC=$try
	  break
	fi
      done
      if test -z "$TCL_INC"; then
	AC_MSG_RESULT(<not found>)
	SKIP_TCL=YES
      fi
      if test -z "$SKIP_TCL"; then
	AC_MSG_CHECKING(for location of tclConfig.sh script)
	if test "x$MACOS_X" != "xyes"; then
	  tclcnf=`echo $tclinc | sed s/include/lib/g`
	  tclcnf="$tclcnf `echo $tclinc | sed s/include/lib64/g`"
	else
	  dnl For all macOS, use the value from TCL in case use of, say, homebrew
	  dnl For Mac OS X 10.3, use the OS-provided framework location
	  dnl For Mac OS X 10.14, the OS-provided framework location doesn't contain the headers, so also check the Xcode SDK
	  tclcnf=`echo $tclinc | sed s/include/lib/g`
	  tclcnf="$tclcnf /System/Library/Frameworks/Tcl.framework `xcrun --show-sdk-path`/System/Library/Frameworks/Tcl.framework"
	fi
	for try in $tclcnf; do
	  if test -f "$try/tclConfig.sh"; then
	    AC_MSG_RESULT($try/tclConfig.sh)
	    . "$try/tclConfig.sh"
	    dnl use eval, because tcl 8.2 includes ${TCL_DBGX}
	    if test "$enable_tclinterp" = "dynamic"; then
	      TCL_LIBS=`eval echo "$TCL_STUB_LIB_SPEC $TCL_LIBS"`
	    else
	      TCL_LIBS=`eval echo "$TCL_LIB_SPEC $TCL_LIBS"`
	    fi
	    dnl Use $TCL_DEFS for -D_THREAD_SAFE et al.  But only use the
	    dnl "-D_ABC" items.  Watch out for -DFOO=long\ long.
	    TCL_DEFS=`echo $TCL_DEFS | sed -e 's/\\\\ /\\\\X/g' | tr ' ' '\012' | sed -e '/^[[^-]]/d' -e '/^-[[^D]]/d' -e '/-D[[^_]]/d' -e 's/-D_/ -D_/' | tr '\012' ' ' | sed -e 's/\\\\X/\\\\ /g'`
	    break
	  fi
	done
	if test -z "$TCL_LIBS"; then
	  AC_MSG_RESULT(<not found>)
	  AC_MSG_CHECKING(for Tcl library by myself)
	  tcllib=`echo $tclinc | sed s/include/lib/g`
	  tcllib="$tcllib `echo $tclinc | sed s/include/lib64/g`"
	  for ext in .so .a ; do
	    for ver in "" $tclver ; do
	      for try in $tcllib ; do
		trylib=tcl$ver$ext
		if test -f "$try/lib$trylib" ; then
		  AC_MSG_RESULT($try/lib$trylib)
		  TCL_LIBS="-L\"$try\" -ltcl$ver -ldl -lm"
		  if test "$vim_cv_uname_output" = SunOS &&
					 echo $vim_cv_uname_r_output | grep '^5' >/dev/null; then
		    TCL_LIBS="$TCL_LIBS -R $try"
		  fi
		  break 3
		fi
	      done
	    done
	  done
	  if test -z "$TCL_LIBS"; then
	    AC_MSG_RESULT(<not found>)
	    SKIP_TCL=YES
	  fi
	fi
	if test -z "$SKIP_TCL"; then
	  AC_DEFINE(FEAT_TCL)
	  TCL_SRC=if_tcl.c
	  TCL_OBJ=objects/if_tcl.o
	  TCL_PRO=if_tcl.pro
	  TCL_CFLAGS="-I$TCL_INC $TCL_DEFS"
	fi
      fi
    else
      AC_MSG_RESULT(too old; need Tcl version 8.0 or later)
    fi
  fi
  if test "$enable_tclinterp" = "dynamic"; then
    if test "X$TCL_SRC" != "X" -a "X$tcldll" != "X"; then
      AC_DEFINE(DYNAMIC_TCL)
      TCL_CFLAGS="-DDYNAMIC_TCL_DLL=\\\"$tcldll\\\" -DDYNAMIC_TCL_VER=\\\"$tclver\\\" $TCL_CFLAGS"
    fi
  fi
  if test "$fail_if_missing" = "yes" -a -z "$TCL_SRC"; then
    AC_MSG_ERROR([could not configure Tcl])
  fi
fi
AC_SUBST(TCL_SRC)
AC_SUBST(TCL_OBJ)
AC_SUBST(TCL_PRO)
AC_SUBST(TCL_CFLAGS)
AC_SUBST(TCL_CFLAGS_EXTRA)
AC_SUBST(TCL_LIBS)

AC_MSG_CHECKING(--enable-rubyinterp argument)
AC_ARG_ENABLE(rubyinterp,
	[  --enable-rubyinterp[=OPTS]     Include Ruby interpreter.  [default=no] [OPTS=no/yes/dynamic]], ,
	[enable_rubyinterp="no"])
AC_MSG_RESULT($enable_rubyinterp)
if test "$enable_rubyinterp" = "yes" -o "$enable_rubyinterp" = "dynamic"; then
  if test "$has_eval" = "no"; then
    AC_MSG_ERROR([cannot use Ruby with tiny features])
  fi

  AC_MSG_CHECKING(--with-ruby-command argument)
  AC_SUBST(vi_cv_path_ruby)
  AC_ARG_WITH(ruby-command, [  --with-ruby-command=RUBY  name of the Ruby command (default: ruby)],
	RUBY_CMD="$withval"; vi_cv_path_ruby="$withval"; AC_MSG_RESULT($RUBY_CMD),
	RUBY_CMD="ruby"; AC_MSG_RESULT(defaulting to $RUBY_CMD))
  AC_PATH_PROG(vi_cv_path_ruby, $RUBY_CMD)
  if test "X$vi_cv_path_ruby" != "X"; then
    AC_MSG_CHECKING(Ruby version)
    if $vi_cv_path_ruby -e 'RUBY_VERSION >= "1.9.1" or exit 1' >/dev/null 2>/dev/null; then
      AC_MSG_RESULT(OK)
      AC_MSG_CHECKING(Ruby rbconfig)
      ruby_rbconfig="RbConfig"
      if ! $vi_cv_path_ruby -r rbconfig -e 'RbConfig' >/dev/null 2>/dev/null; then
	ruby_rbconfig="Config"
      fi
      AC_MSG_RESULT($ruby_rbconfig)
      AC_MSG_CHECKING(Ruby header files)
      rubyhdrdir=`$vi_cv_path_ruby -r mkmf -e "print $ruby_rbconfig::CONFIG[['rubyhdrdir']] || $ruby_rbconfig::CONFIG[['archdir']] || \\$hdrdir" 2>/dev/null`
      if test "X$rubyhdrdir" != "X"; then
	AC_MSG_RESULT($rubyhdrdir)
	RUBY_CFLAGS="-I$rubyhdrdir"
        rubyarchdir=`$vi_cv_path_ruby -r rbconfig -e "print ($ruby_rbconfig::CONFIG.has_key? 'rubyarchhdrdir') ? $ruby_rbconfig::CONFIG[['rubyarchhdrdir']] : '$rubyhdrdir/'+$ruby_rbconfig::CONFIG[['arch']]"`
        if test -d "$rubyarchdir"; then
          RUBY_CFLAGS="$RUBY_CFLAGS -I$rubyarchdir"
        fi
        rubyversion=`$vi_cv_path_ruby -r rbconfig -e "print $ruby_rbconfig::CONFIG[['ruby_version']].gsub(/\./, '')[[0,2]]"`
	if test "X$rubyversion" = "X"; then
	  rubyversion=`$vi_cv_path_ruby -e "print RUBY_VERSION.gsub(/\./, '')[[0,2]]"`
	fi
        RUBY_CFLAGS="$RUBY_CFLAGS -DRUBY_VERSION=$rubyversion"
	rubylibs=`$vi_cv_path_ruby -r rbconfig -e "print $ruby_rbconfig::CONFIG[['LIBS']]"`
	if test "X$rubylibs" != "X"; then
	  RUBY_LIBS="$rubylibs"
	fi
	librubyarg=`$vi_cv_path_ruby -r rbconfig -e "print $ruby_rbconfig.expand($ruby_rbconfig::CONFIG[['LIBRUBYARG']])"`
	librubya=`$vi_cv_path_ruby -r rbconfig -e "print $ruby_rbconfig.expand($ruby_rbconfig::CONFIG[['LIBRUBY_A']])"`
	rubylibdir=`$vi_cv_path_ruby -r rbconfig -e "print $ruby_rbconfig.expand($ruby_rbconfig::CONFIG[['libdir']])"`
	if test -f "$rubylibdir/$librubya" || expr "$librubyarg" : "-lruby" >/dev/null; then
	  RUBY_LIBS="$RUBY_LIBS -L$rubylibdir"
	elif test "$librubyarg" = "libruby.a"; then
	  dnl required on Mac OS 10.3 where libruby.a doesn't exist
	  librubyarg="-lruby"
	  RUBY_LIBS="$RUBY_LIBS -L$rubylibdir"
	fi

	if test "X$librubyarg" != "X"; then
	  RUBY_LIBS="$librubyarg $RUBY_LIBS"
	fi

	dnl Here the Ruby LDFLAGS used to be added to LDFLAGS, but that turns
	dnl out to cause trouble and was removed.

	RUBY_SRC="if_ruby.c"
	RUBY_OBJ="objects/if_ruby.o"
	RUBY_PRO="if_ruby.pro"
	AC_DEFINE(FEAT_RUBY)
	if test "$enable_rubyinterp" = "dynamic"; then
	  libruby_soname=`$vi_cv_path_ruby -r rbconfig -e "puts $ruby_rbconfig::CONFIG[['LIBRUBY_ALIASES']].split[[0]]"`
	  if test -z "$libruby_soname"; then
	    libruby_soname=`$vi_cv_path_ruby -r rbconfig -e "puts $ruby_rbconfig::CONFIG[['LIBRUBY_SO']]"`
	  fi
	  AC_DEFINE(DYNAMIC_RUBY)
	  RUBY_CFLAGS="-DDYNAMIC_RUBY_DLL=\\\"$libruby_soname\\\" $RUBY_CFLAGS"
	  RUBY_LIBS=
	fi
	if test "X$CLANG_VERSION" != "X" -a "$rubyversion" -ge 30; then
	  RUBY_CFLAGS="$RUBY_CFLAGS -fdeclspec"
	fi
      else
	AC_MSG_RESULT(not found; disabling Ruby)
      fi
    else
      AC_MSG_RESULT(too old; need Ruby version 1.9.1 or later)
    fi
  fi

  if test "$fail_if_missing" = "yes" -a -z "$RUBY_OBJ"; then
    AC_MSG_ERROR([could not configure Ruby])
  fi
fi
AC_SUBST(RUBY_SRC)
AC_SUBST(RUBY_OBJ)
AC_SUBST(RUBY_PRO)
AC_SUBST(RUBY_CFLAGS)
AC_SUBST(RUBY_CFLAGS_EXTRA)
AC_SUBST(RUBY_LIBS)

AC_MSG_CHECKING(--enable-cscope argument)
AC_ARG_ENABLE(cscope,
	[  --enable-cscope         Include cscope interface.], ,
	[enable_cscope="no"])
AC_MSG_RESULT($enable_cscope)
if test "$enable_cscope" = "yes"; then
  AC_DEFINE(FEAT_CSCOPE)
fi

AC_MSG_CHECKING(--disable-netbeans argument)
AC_ARG_ENABLE(netbeans,
	[  --disable-netbeans      Disable NetBeans integration support.],
	, [enable_netbeans="yes"])
if test "$enable_netbeans" = "yes"; then
  if test "$has_eval" = "no"; then
    AC_MSG_RESULT([cannot use NetBeans with tiny features])
    enable_netbeans="no"
  else
    AC_MSG_RESULT(no)
  fi
else
  AC_MSG_RESULT(yes)
fi

AC_MSG_CHECKING(--disable-channel argument)
AC_ARG_ENABLE(channel,
	[  --disable-channel       Disable process communication support.],
	, [enable_channel="yes"])
if test "$enable_channel" = "yes"; then
  if test "$has_eval" = "no"; then
    AC_MSG_RESULT([cannot use channels with tiny features])
    enable_channel="no"
  else
    AC_MSG_RESULT(no)
  fi
else
  if test "$enable_netbeans" = "yes"; then
    AC_MSG_RESULT([yes, netbeans also disabled])
    enable_netbeans="no"
  else
    AC_MSG_RESULT(yes)
  fi
fi

if test "$enable_channel" = "yes"; then
  dnl On Solaris we need the socket library, or on Haiku the network library.
  if test "x$HAIKU" = "xyes"; then
	AC_CHECK_LIB(network, socket)
  else
	AC_CHECK_LIB(socket, socket)
  fi

  AC_CACHE_CHECK([whether compiling with IPv6 networking is possible], [vim_cv_ipv6_networking],
    [AC_LINK_IFELSE([AC_LANG_PROGRAM([
#include <stdio.h>
#include <stdlib.h>
#include <stdarg.h>
#include <fcntl.h>
#include <netdb.h>
#include <netinet/in.h>
#include <errno.h>
#include <sys/types.h>
#include <sys/socket.h>
	/* Check bitfields */
	struct nbbuf {
	unsigned int  initDone:1;
	unsigned short signmaplen;
	};
	    ], [
		/* Check creating a socket. */
		struct sockaddr_in server;
		struct addrinfo *res;
		(void)socket(AF_INET, SOCK_STREAM, 0);
		(void)htons(100);
		(void)getaddrinfo("microsoft.com", NULL, NULL, &res);
		if (errno == ECONNREFUSED)
		  (void)connect(1, (struct sockaddr *)&server, sizeof(server));
		(void)freeaddrinfo(res);
	    ])],
	[vim_cv_ipv6_networking="yes"],
	[vim_cv_ipv6_networking="no"])])

  if test "x$vim_cv_ipv6_networking" = "xyes"; then
    AC_DEFINE(FEAT_IPV6)
    AC_CHECK_FUNCS(inet_ntop)
  else
    dnl On Solaris we need the nsl library.
    AC_CHECK_LIB(nsl, gethostbyname)
    AC_CACHE_CHECK([whether compiling with IPv4 networking is possible], [vim_cv_ipv4_networking],
      [AC_LINK_IFELSE([AC_LANG_PROGRAM([
#include <stdio.h>
#include <stdlib.h>
#include <stdarg.h>
#include <fcntl.h>
#include <netdb.h>
#include <netinet/in.h>
#include <errno.h>
#include <sys/types.h>
#include <sys/socket.h>
	/* Check bitfields */
	struct nbbuf {
	unsigned int  initDone:1;
	unsigned short signmaplen;
	};
	    ], [
		/* Check creating a socket. */
		struct sockaddr_in server;
		(void)socket(AF_INET, SOCK_STREAM, 0);
		(void)htons(100);
		(void)gethostbyname("microsoft.com");
		if (errno == ECONNREFUSED)
		  (void)connect(1, (struct sockaddr *)&server, sizeof(server));
	    ])],
	[vim_cv_ipv4_networking="yes"],
	[vim_cv_ipv4_networking="no"; enable_netbeans="no"; enable_channel="no"])])
  fi
fi
if test "$enable_netbeans" = "yes"; then
  AC_DEFINE(FEAT_NETBEANS_INTG)
  NETBEANS_SRC="netbeans.c"
  AC_SUBST(NETBEANS_SRC)
  NETBEANS_OBJ="objects/netbeans.o"
  AC_SUBST(NETBEANS_OBJ)
fi
if test "$enable_channel" = "yes"; then
  AC_DEFINE(FEAT_JOB_CHANNEL)
  CHANNEL_SRC="job.c channel.c"
  AC_SUBST(CHANNEL_SRC)
  CHANNEL_OBJ="objects/job.o objects/channel.o"
  AC_SUBST(CHANNEL_OBJ)
fi

AC_MSG_CHECKING(--enable-terminal argument)
AC_ARG_ENABLE(terminal,
	[  --enable-terminal       Enable terminal emulation support.],
	, [enable_terminal="auto"])
if test "$enable_terminal" = "yes" || test "$enable_terminal" = "auto" -a "x$features" = "xhuge" ; then
  if test "$has_eval" = "no"; then
    AC_MSG_RESULT([cannot use terminal emulator with tiny features])
    enable_terminal="no"
  else
    if test "$enable_terminal" = "auto"; then
      enable_terminal="yes"
      AC_MSG_RESULT(defaulting to yes)
    else
      AC_MSG_RESULT(yes)
    fi
  fi
else
  if test "$enable_terminal" = "auto"; then
    enable_terminal="no"
    AC_MSG_RESULT(defaulting to no)
  else
    AC_MSG_RESULT(no)
  fi
fi
if test "$enable_terminal" = "yes" -a "$enable_channel" = "yes"; then
  AC_DEFINE(FEAT_TERMINAL)
  TERM_SRC="libvterm/src/encoding.c libvterm/src/keyboard.c libvterm/src/mouse.c libvterm/src/parser.c libvterm/src/pen.c libvterm/src/creen.c libvterm/src/state.c libvterm/src/unicode.c libvterm/src/vterm.c"
  AC_SUBST(TERM_SRC)
  TERM_OBJ="objects/vterm_encoding.o objects/vterm_keyboard.o objects/vterm_mouse.o objects/vterm_parser.o objects/vterm_pen.o objects/vterm_screen.o objects/vterm_state.o objects/vterm_unicode.o objects/vterm_vterm.o"
  AC_SUBST(TERM_OBJ)
  TERM_TEST="test_libvterm"
  AC_SUBST(TERM_TEST)
fi

AC_MSG_CHECKING(--enable-autoservername argument)
AC_ARG_ENABLE(autoservername,
	[  --enable-autoservername Automatically define servername at vim startup.], ,
	[enable_autoservername="no"])
AC_MSG_RESULT($enable_autoservername)
if test "$enable_autoservername" = "yes"; then
  AC_DEFINE(FEAT_AUTOSERVERNAME)
fi

AC_MSG_CHECKING(--enable-multibyte argument)
AC_ARG_ENABLE(multibyte,
	[  --enable-multibyte      Include multibyte editing support.], ,
	[enable_multibyte="yes"])
AC_MSG_RESULT($enable_multibyte)
if test "$enable_multibyte" != "yes"; then
  AC_MSG_ERROR([The multi-byte feature can no longer be disabled. If you have
		a problem with this, discuss on the Vim mailing list.])
fi

dnl Right-to-Left language support for Vim will be included with huge features,
dnl unless ENABLE_RIGHTLEFT is undefined.
AC_MSG_CHECKING(--disable-rightleft argument)
AC_ARG_ENABLE(rightleft,
	[  --disable-rightleft     Do not include Right-to-Left language support.],
	, [enable_rightleft="yes"])
if test "$enable_rightleft" = "yes"; then
	AC_MSG_RESULT(no)
else
	AC_MSG_RESULT(yes)
	AC_DEFINE(DISABLE_RIGHTLEFT)
fi

dnl Arabic language support for Vim will be included with huge features,
dnl unless ENABLE_ARABIC is undefined.
AC_MSG_CHECKING(--disable-arabic argument)
AC_ARG_ENABLE(arabic,
	[  --disable-arabic        Do not include Arabic language support.],
	, [enable_arabic="yes"])
if test "$enable_arabic" = "yes"; then
	AC_MSG_RESULT(no)
else
	AC_MSG_RESULT(yes)
	AC_DEFINE(DISABLE_ARABIC)
fi

dnl Farsi language support has been removed, ignore --disable-farsi
AC_ARG_ENABLE(farsi,
	[  --disable-farsi         Deprecated.],,)

AC_MSG_CHECKING(--enable-xim argument)
AC_ARG_ENABLE(xim,
	[  --enable-xim            Include XIM input support.],
	AC_MSG_RESULT($enable_xim),
	[enable_xim="auto"; AC_MSG_RESULT(defaulting to auto)])

AC_MSG_CHECKING(--enable-fontset argument)
AC_ARG_ENABLE(fontset,
	[  --enable-fontset        Include X fontset output support.], ,
	[enable_fontset="no"])
AC_MSG_RESULT($enable_fontset)
dnl defining FEAT_XFONTSET is delayed, so that it can be disabled for no GUI

test -z "$with_x" && with_x=yes
test "${enable_gui-yes}" != no -a "x$MACOS_X" != "xyes" -a "x$QNX" != "xyes" && with_x=yes
if test "$with_x" = no; then
  AC_MSG_RESULT(defaulting to: don't HAVE_X11)
else
  dnl Do this check early, so that its failure can override user requests.

  AC_PATH_PROG(xmkmfpath, xmkmf)

  AC_PATH_XTRA

  dnl On z/OS Unix the X libraries are DLLs. To use them the code must
  dnl be compiled with a special option.
  dnl Also add SM, ICE and Xmu to X_EXTRA_LIBS.
  if test "$zOSUnix" = "yes"; then
    CFLAGS="$CFLAGS -W c,dll"
    LDFLAGS="$LDFLAGS -W l,dll"
    X_EXTRA_LIBS="$X_EXTRA_LIBS -lSM -lICE -lXmu"
  fi

  dnl On my HPUX system the X include dir is found, but the lib dir not.
  dnl This is a desperate try to fix this.

  if test -d "$x_includes" && test ! -d "$x_libraries"; then
    x_libraries=`echo "$x_includes" | sed s/include/lib/`
    AC_MSG_RESULT(Corrected X libraries to $x_libraries)
    X_LIBS="$X_LIBS -L$x_libraries"
    if test "$vim_cv_uname_output" = SunOS &&
					 echo $vim_cv_uname_r_output | grep '^5' >/dev/null; then
      X_LIBS="$X_LIBS -R $x_libraries"
    fi
  fi

  if test -d "$x_libraries" && test ! -d "$x_includes"; then
    x_includes=`echo "$x_libraries" | sed s/lib/include/`
    AC_MSG_RESULT(Corrected X includes to $x_includes)
    X_CFLAGS="$X_CFLAGS -I$x_includes"
  fi

  dnl Remove "-I/usr/include " from X_CFLAGS, should not be needed.
  X_CFLAGS="`echo $X_CFLAGS\  | sed 's%-I/usr/include %%'`"
  dnl Remove "-L/usr/lib " from X_LIBS, should not be needed.
  X_LIBS="`echo $X_LIBS\  | sed 's%-L/usr/lib %%'`"
  dnl Same for "-R/usr/lib ".
  X_LIBS="`echo $X_LIBS\  | sed -e 's%-R/usr/lib %%' -e 's%-R /usr/lib %%'`"


  dnl Check if the X11 header files are correctly installed. On some systems
  dnl Xlib.h includes files that don't exist.  On some systems X11/Intrinsic.h
  dnl is missing.
  AC_MSG_CHECKING(if X11 header files can be found)
  cflags_save=$CFLAGS
  CFLAGS="$CFLAGS $X_CFLAGS"
  AC_COMPILE_IFELSE([AC_LANG_PROGRAM([#include <X11/Xlib.h>
#include <X11/Intrinsic.h>], )],
	AC_MSG_RESULT(yes),
	AC_MSG_RESULT(no); no_x=yes)
  CFLAGS=$cflags_save

  if test "${no_x-no}" = yes; then
    with_x=no
  else
    AC_DEFINE(HAVE_X11)
    X_LIB="-lXt -lX11";
    AC_SUBST(X_LIB)

    ac_save_LDFLAGS="$LDFLAGS"
    LDFLAGS="-L$x_libraries $LDFLAGS"

    dnl Check for -lXdmcp (needed on SunOS 4.1.4)
    dnl For HP-UX 10.20 it must be before -lSM -lICE
    AC_CHECK_LIB(Xdmcp, _XdmcpAuthDoIt, [X_EXTRA_LIBS="$X_EXTRA_LIBS -lXdmcp"],,
		[-lXt $X_PRE_LIBS -lX11 $X_EXTRA_LIBS -lXdmcp])

    dnl Some systems need -lnsl -lsocket when testing for ICE.
    dnl The check above doesn't do this, try here (again).  Also needed to get
    dnl them after Xdmcp.  link.sh will remove them when not needed.
    dnl Check for other function than above to avoid the cached value
    AC_CHECK_LIB(ICE, IceOpenConnection,
		  [X_EXTRA_LIBS="$X_EXTRA_LIBS -lSM -lICE"],, [$X_EXTRA_LIBS])

    dnl Check for -lXpm (needed for some versions of Motif)
    LDFLAGS="$X_LIBS $ac_save_LDFLAGS"
    AC_CHECK_LIB(Xpm, XpmCreatePixmapFromData, [X_PRE_LIBS="$X_PRE_LIBS -lXpm"],,
		[-lXt $X_PRE_LIBS -lXpm -lX11 $X_EXTRA_LIBS])

    dnl Check that the X11 header files don't use implicit declarations
    AC_MSG_CHECKING(if X11 header files implicitly declare return values)
    cflags_save=$CFLAGS
    dnl -Werror is GCC only, others like Solaris Studio might not like it
    if test "$GCC" = yes; then
      CFLAGS="$CFLAGS $X_CFLAGS -Werror"
    else
      CFLAGS="$CFLAGS $X_CFLAGS"
    fi
    AC_COMPILE_IFELSE([AC_LANG_PROGRAM([#include <X11/Xlib.h>], )],
	AC_MSG_RESULT(no),
	CFLAGS="$CFLAGS -Wno-implicit-int"
	AC_COMPILE_IFELSE([AC_LANG_PROGRAM([#include <X11/Xlib.h>], )],
	    AC_MSG_RESULT(yes); cflags_save="$cflags_save -Wno-implicit-int",
	    AC_MSG_RESULT(test failed)
	)
    )
    CFLAGS=$cflags_save

    LDFLAGS="$ac_save_LDFLAGS"

    AC_CHECK_SIZEOF([wchar_t])
    if test "$ac_cv_sizeof_wchar_t" -le 2; then
      AC_DEFINE(SMALL_WCHAR_T)
    fi

  fi
fi

dnl Check if --with-x was given but it doesn't work.
if test "x$with_x" = xno -a "x$with_x_arg" = xyes; then
    AC_MSG_ERROR([could not configure X])
fi

test "x$with_x" = xno -a "x$HAIKU" != "xyes" -a "x$MACOS_X" != "xyes" -a "x$QNX" != "xyes" && enable_gui=no

AC_MSG_CHECKING(--enable-gui argument)
AC_ARG_ENABLE(gui,
 [  --enable-gui[=OPTS]       X11 GUI. [default=auto] [OPTS=auto/no/gtk2/gnome2/gtk3/motif/haiku/photon/carbon]], , enable_gui="auto")

dnl Canonicalize the --enable-gui= argument so that it can be easily compared.
dnl Do not use character classes for portability with old tools.
enable_gui_canon=`echo "_$enable_gui" | \
	sed 's/[[ _+-]]//g;y/ABCDEFGHIJKLMNOPQRSTUVWXYZ/abcdefghijklmnopqrstuvwxyz/'`

dnl Skip everything by default.
SKIP_GTK2=YES
SKIP_GTK3=YES
SKIP_GNOME=YES
SKIP_MOTIF=YES
SKIP_PHOTON=YES
SKIP_HAIKU=YES
GUITYPE=NONE

if test "x$HAIKU" = "xyes"; then
  SKIP_HAIKU=
  case "$enable_gui_canon" in
    no)     AC_MSG_RESULT(no GUI support)
            SKIP_HAIKU=YES ;;
    yes|"") AC_MSG_RESULT(yes - automatic GUI support) ;;
    auto)   AC_MSG_RESULT(auto - automatic GUI support) ;;
    haiku)  AC_MSG_RESULT(Haiku GUI support) ;;
    *)      AC_MSG_RESULT([Sorry, $enable_gui GUI is not supported])
            SKIP_HAIKU=YES ;;
    esac
elif test "x$QNX" = "xyes" -a "x$with_x" = "xno" ; then
  SKIP_PHOTON=
  case "$enable_gui_canon" in
    no)		AC_MSG_RESULT(no GUI support)
		SKIP_PHOTON=YES ;;
    yes|""|auto) AC_MSG_RESULT(automatic GUI support)
    		gui_auto=yes ;;
    photon)	AC_MSG_RESULT(Photon GUI support) ;;
    *)		AC_MSG_RESULT([Sorry, $enable_gui GUI is not supported])
		SKIP_PHOTON=YES ;;
  esac

elif test "x$MACOS_X" = "xyes" -a "x$with_x" = "xno" ; then
  case "$enable_gui_canon" in
    no)		AC_MSG_RESULT(no GUI support) ;;
    yes|"")	AC_MSG_RESULT(yes - automatic GUI support)
    		gui_auto=yes ;;
    auto)	AC_MSG_RESULT(auto - disable GUI support for Mac OS) ;;
    *)		AC_MSG_RESULT([Sorry, $enable_gui GUI is not supported]) ;;
  esac
else

  case "$enable_gui_canon" in
    no|none)	AC_MSG_RESULT(no GUI support) ;;
    yes|""|auto)	AC_MSG_RESULT(yes/auto - automatic GUI support)
		gui_auto=yes
		SKIP_GTK2=
		SKIP_GTK3=
		SKIP_GNOME=
		SKIP_MOTIF=;;
    gtk2)	AC_MSG_RESULT(GTK+ 2.x GUI support)
		SKIP_GTK2=;;
    gnome2)	AC_MSG_RESULT(GNOME 2.x GUI support)
		SKIP_GNOME=
		SKIP_GTK2=;;
    gtk3)	AC_MSG_RESULT(GTK+ 3.x GUI support)
		SKIP_GTK3=;;
    motif)	AC_MSG_RESULT(Motif GUI support)
		SKIP_MOTIF=;;
    *)		AC_MSG_RESULT([Sorry, $enable_gui GUI is not supported]) ;;
  esac

fi

if test "x$SKIP_GTK2" != "xYES" -a "$enable_gui_canon" != "gtk2" \
				-a "$enable_gui_canon" != "gnome2"; then
  AC_MSG_CHECKING(whether or not to look for GTK+ 2)
  AC_ARG_ENABLE(gtk2-check,
	[  --enable-gtk2-check     If auto-select GUI, check for GTK+ 2 [default=yes]],
	, enable_gtk2_check="yes")
  AC_MSG_RESULT($enable_gtk2_check)
  if test "x$enable_gtk2_check" = "xno"; then
    SKIP_GTK2=YES
    SKIP_GNOME=YES
  fi
fi

if test "x$SKIP_GNOME" != "xYES" -a "$enable_gui_canon" != "gnome2"; then
  AC_MSG_CHECKING(whether or not to look for GNOME)
  AC_ARG_ENABLE(gnome-check,
	[  --enable-gnome-check    If GTK GUI, check for GNOME [default=no]],
	, enable_gnome_check="no")
  AC_MSG_RESULT($enable_gnome_check)
  if test "x$enable_gnome_check" = "xno"; then
    SKIP_GNOME=YES
  fi
fi

if test "x$SKIP_GTK3" != "xYES" -a "$enable_gui_canon" != "gtk3"; then
  AC_MSG_CHECKING(whether or not to look for GTK+ 3)
  AC_ARG_ENABLE(gtk3-check,
	[  --enable-gtk3-check     If auto-select GUI, check for GTK+ 3 [default=yes]],
	, enable_gtk3_check="yes")
  AC_MSG_RESULT($enable_gtk3_check)
  if test "x$enable_gtk3_check" = "xno"; then
    SKIP_GTK3=YES
  fi
fi

if test "x$SKIP_MOTIF" != "xYES" -a "$enable_gui_canon" != "motif"; then
  AC_MSG_CHECKING(whether or not to look for Motif)
  AC_ARG_ENABLE(motif-check,
	[  --enable-motif-check    If auto-select GUI, check for Motif [default=yes]],
	, enable_motif_check="yes")
  AC_MSG_RESULT($enable_motif_check)
  if test "x$enable_motif_check" = "xno"; then
    SKIP_MOTIF=YES
  fi
fi

dnl define an autoconf function to check for a specified version of GTK, and
dnl try to compile/link a GTK program.
dnl
dnl AM_PATH_GTK([MINIMUM-VERSION, [ACTION-IF-FOUND [, ACTION-IF-NOT-FOUND]]])
dnl Test for GTK, and define GTK_CFLAGS, GTK_LIBDIR and GTK_LIBS
dnl
AC_DEFUN(AM_PATH_GTK,
[
  if test "X$GTK_CONFIG" != "Xno" -o "X$PKG_CONFIG" != "Xno"; then
  {
    no_gtk=""
    if (test "X$SKIP_GTK2" != "XYES" -a "X$PKG_CONFIG" != "Xno") \
	  && $PKG_CONFIG --exists gtk+-2.0; then
    {
      min_gtk_version=ifelse([$1], ,2.2.0,$1)
      AC_MSG_CHECKING(for GTK - version >= $min_gtk_version)
      dnl We should be using PKG_CHECK_MODULES() instead of this hack.
      dnl But I guess the dependency on pkgconfig.m4 is not wanted or
      dnl something like that.
      GTK_CFLAGS=`$PKG_CONFIG --cflags gtk+-2.0`
      GTK_LIBDIR=`$PKG_CONFIG --libs-only-L gtk+-2.0`
      GTK_LIBS=`$PKG_CONFIG --libs gtk+-2.0`
      gtk_major_version=`$PKG_CONFIG --modversion gtk+-2.0 | \
	     sed 's/\([[0-9]]*\)\.\([[0-9]]*\)\.\([[0-9]]*\)/\1/'`
      gtk_minor_version=`$PKG_CONFIG --modversion gtk+-2.0 | \
	     sed 's/\([[0-9]]*\)\.\([[0-9]]*\)\.\([[0-9]]*\)/\2/'`
      gtk_micro_version=`$PKG_CONFIG --modversion gtk+-2.0 | \
	     sed 's/\([[0-9]]*\)\.\([[0-9]]*\)\.\([[0-9]]*\)/\3/'`
    }
    elif (test "X$SKIP_GTK3" != "XYES" -a "X$PKG_CONFIG" != "Xno") \
	  && $PKG_CONFIG --exists gtk+-3.0; then
    {
      min_gtk_version=ifelse([$1], ,3.0.0,$1)
      AC_MSG_CHECKING(for GTK - version >= $min_gtk_version)

      GTK_CFLAGS=`$PKG_CONFIG --cflags gtk+-3.0`
      GTK_LIBDIR=`$PKG_CONFIG --libs-only-L gtk+-3.0`
      GTK_LIBS=`$PKG_CONFIG --libs gtk+-3.0`
      gtk_major_version=`$PKG_CONFIG --modversion gtk+-3.0 | \
	     sed 's/\([[0-9]]*\)\.\([[0-9]]*\)\.\([[0-9]]*\)/\1/'`
      gtk_minor_version=`$PKG_CONFIG --modversion gtk+-3.0 | \
	     sed 's/\([[0-9]]*\)\.\([[0-9]]*\)\.\([[0-9]]*\)/\2/'`
      gtk_micro_version=`$PKG_CONFIG --modversion gtk+-3.0 | \
	     sed 's/\([[0-9]]*\)\.\([[0-9]]*\)\.\([[0-9]]*\)/\3/'`
    }
    else
      dnl Put some text before the "no" to hint at installing the gtk-dev
      dnl packages.
      AC_MSG_CHECKING(for GTK -dev package)
      no_gtk=yes
    fi

    if test "x$enable_gtktest" = "xyes" -a "x$no_gtk" = "x"; then
    {
      ac_save_CFLAGS="$CFLAGS"
      ac_save_LIBS="$LIBS"
      CFLAGS="$CFLAGS $GTK_CFLAGS"
      LIBS="$LIBS $GTK_LIBS"

      dnl
      dnl Now check if the installed GTK is sufficiently new.
      dnl
      rm -f conf.gtktest
      AC_RUN_IFELSE([AC_LANG_SOURCE([
#include <gtk/gtk.h>
#include <stdio.h>
#if STDC_HEADERS
# include <stdlib.h>
# include <stddef.h>
#endif

int
main ()
{
int major, minor, micro;
char *tmp_version;

system ("touch conf.gtktest");

/* HP/UX 9 (%@#!) writes to sscanf strings */
tmp_version = g_strdup("$min_gtk_version");
if (sscanf(tmp_version, "%d.%d.%d", &major, &minor, &micro) != 3) {
   printf("%s, bad version string\n", "$min_gtk_version");
   g_free(tmp_version);
   exit(1);
 }

g_free(tmp_version);

if ((gtk_major_version > major) ||
    ((gtk_major_version == major) && (gtk_minor_version > minor)) ||
    ((gtk_major_version == major) && (gtk_minor_version == minor) &&
				     (gtk_micro_version >= micro)))
{
    return 0;
}
return 1;
}
])],, no_gtk=yes,[echo $ac_n "cross compiling; assumed OK... $ac_c"])
      CFLAGS="$ac_save_CFLAGS"
      LIBS="$ac_save_LIBS"
    }
    fi
    if test "x$no_gtk" = x ; then
      if test "x$enable_gtktest" = "xyes"; then
	AC_MSG_RESULT(yes; found version $gtk_major_version.$gtk_minor_version.$gtk_micro_version)
      else
	AC_MSG_RESULT(found version $gtk_major_version.$gtk_minor_version.$gtk_micro_version)
      fi
      ifelse([$2], , :, [$2])
    else
    {
      AC_MSG_RESULT(no)
      GTK_CFLAGS=""
      GTK_LIBS=""
      ifelse([$3], , :, [$3])
      if test "$fail_if_missing" = "yes" -a "X$gui_auto" != "Xyes"; then
	AC_MSG_ERROR([could not configure GTK])
      fi
    }
    fi
  }
  else
    GTK_CFLAGS=""
    GTK_LIBS=""
    ifelse([$3], , :, [$3])
  fi
  AC_SUBST(GTK_CFLAGS)
  AC_SUBST(GTK_LIBS)
  rm -f conf.gtktest
])

dnl ---------------------------------------------------------------------------
dnl gnome
dnl ---------------------------------------------------------------------------
AC_DEFUN([GNOME_INIT_HOOK],
[
  AC_SUBST(GNOME_LIBS)
  AC_SUBST(GNOME_LIBDIR)
  AC_SUBST(GNOME_INCLUDEDIR)

  AC_ARG_WITH(gnome-includes,
    [  --with-gnome-includes=DIR Specify location of GNOME headers],
    [CFLAGS="$CFLAGS -I$withval"]
  )

  AC_ARG_WITH(gnome-libs,
    [  --with-gnome-libs=DIR   Specify location of GNOME libs],
    [LDFLAGS="$LDFLAGS -L$withval" gnome_prefix=$withval]
  )

  AC_ARG_WITH(gnome,
    [  --with-gnome            Specify prefix for GNOME files],
    if test x$withval = xyes; then
      want_gnome=yes
      ifelse([$1], [], :, [$1])
    else
      if test "x$withval" = xno; then
	want_gnome=no
      else
	want_gnome=yes
	LDFLAGS="$LDFLAGS -L$withval/lib"
	CFLAGS="$CFLAGS -I$withval/include"
	gnome_prefix=$withval/lib
      fi
    fi,
    want_gnome=yes)

  if test "x$want_gnome" = xyes; then
  {
    AC_MSG_CHECKING(for libgnomeui-2.0)
    if $PKG_CONFIG --exists libgnomeui-2.0; then
      AC_MSG_RESULT(yes)
      GNOME_LIBS=`$PKG_CONFIG --libs-only-l libgnomeui-2.0`
      GNOME_LIBDIR=`$PKG_CONFIG --libs-only-L libgnomeui-2.0`
      GNOME_INCLUDEDIR=`$PKG_CONFIG --cflags libgnomeui-2.0`

      dnl On FreeBSD we need -pthread but pkg-config doesn't include it.
      dnl This might not be the right way but it works for me...
      AC_MSG_CHECKING(for FreeBSD)
      if test "$vim_cv_uname_output" = FreeBSD; then
	AC_MSG_RESULT(yes, adding -pthread)
	GNOME_INCLUDEDIR="$GNOME_INCLUDEDIR -D_THREAD_SAFE"
	GNOME_LIBS="$GNOME_LIBS -pthread"
      else
	AC_MSG_RESULT(no)
      fi
      $1
    else
      AC_MSG_RESULT(not found)
      if test "x$2" = xfail; then
	AC_MSG_ERROR(Could not find libgnomeui-2.0 via pkg-config)
      fi
    fi
  }
  fi
])

AC_DEFUN([GNOME_INIT],[
	GNOME_INIT_HOOK([],fail)
])

if test "X$PKG_CONFIG" = "X"; then
  AC_PATH_TOOL(PKG_CONFIG, pkg-config, no)
fi


dnl ---------------------------------------------------------------------------
dnl Check for GTK2.  If it fails, then continue on for Motif as before...
dnl ---------------------------------------------------------------------------
if test -z "$SKIP_GTK2"; then

  AC_MSG_CHECKING(--disable-gtktest argument)
  AC_ARG_ENABLE(gtktest, [  --disable-gtktest       Do not try to compile and run a test GTK program],
	, enable_gtktest=yes)
  if test "x$enable_gtktest" = "xyes" ; then
    AC_MSG_RESULT(gtk test enabled)
  else
    AC_MSG_RESULT(gtk test disabled)
  fi

  if test "x$PKG_CONFIG" != "xno"; then
    dnl First try finding version 2.2.0 or later.  The 2.0.x series has
    dnl problems (bold fonts, --remote doesn't work).
    dnl Disable checking for GTK3 here, otherwise it's found when GTK2 is not
    dnl found.
    save_skip_gtk3=$SKIP_GTK3
    SKIP_GTK3=YES
    AM_PATH_GTK(2.2.0,
		[GUI_LIB_LOC="$GTK_LIBDIR"
		 GTK_LIBNAME="$GTK_LIBS"
		GUI_INC_LOC="$GTK_CFLAGS"], )
    if test "x$GTK_CFLAGS" != "x"; then
      SKIP_GTK3=YES
      SKIP_MOTIF=YES
      GUITYPE=GTK
      AC_SUBST(GTK_LIBNAME)
    else
      SKIP_GTK3=$save_skip_gtk3
    fi
  fi
  if test "x$GUITYPE" = "xGTK"; then
    dnl
    dnl if GTK exists, then check for GNOME.
    dnl
    if test -z "$SKIP_GNOME"; then
    {
      GNOME_INIT_HOOK([have_gnome=yes])
      if test "x$have_gnome" = xyes ; then
	AC_DEFINE(FEAT_GUI_GNOME)
	GUI_INC_LOC="$GUI_INC_LOC $GNOME_INCLUDEDIR"
	GTK_LIBNAME="$GTK_LIBNAME $GNOME_LIBDIR $GNOME_LIBS"
      fi
    }
    fi
  fi
fi


dnl ---------------------------------------------------------------------------
dnl Check for GTK3.
dnl ---------------------------------------------------------------------------
if test -z "$SKIP_GTK3"; then

  AC_MSG_CHECKING(--disable-gtktest argument)
  AC_ARG_ENABLE(gtktest, [  --disable-gtktest       Do not try to compile and run a test GTK program],
	, enable_gtktest=yes)
  if test "x$enable_gtktest" = "xyes" ; then
    AC_MSG_RESULT(gtk test enabled)
  else
    AC_MSG_RESULT(gtk test disabled)
  fi

  if test "x$PKG_CONFIG" != "xno"; then
    save_skip_gtk2=$SKIP_GTK2
    SKIP_GTK2=YES
    AM_PATH_GTK(3.0.0,
		[GUI_LIB_LOC="$GTK_LIBDIR"
		 GTK_LIBNAME="$GTK_LIBS"
		GUI_INC_LOC="$GTK_CFLAGS"], )
    if test "x$GTK_CFLAGS" != "x"; then
      SKIP_GTK2=YES
      SKIP_GNOME=YES
      SKIP_MOTIF=YES
      GUITYPE=GTK
      AC_SUBST(GTK_LIBNAME)
      AC_DEFINE(USE_GTK3)
    else
      SKIP_GTK2=$save_skip_gtk2
    fi
  fi
fi

dnl Check the version of Gdk-Pixbuf.  If the version is 2.31 or later and
dnl glib-compile-resources is found in PATH, use GResource.
if test "x$GUITYPE" = "xGTK"; then
  AC_MSG_CHECKING([version of Gdk-Pixbuf])
  gdk_pixbuf_version=`$PKG_CONFIG --modversion gdk-pixbuf-2.0`
  if test "x$gdk_pixbuf_version" != x ; then
    gdk_pixbuf_version_minor=`echo $gdk_pixbuf_version | \
      sed -e 's/[[0-9]][[0-9]]*\.\([[0-9]][[0-9]]*\)\.[[0-9]][[0-9]]*/\1/'`
    if test "x$gdk_pixbuf_version_minor" != x -a \
	$gdk_pixbuf_version_minor -ge 31 ; then
      AC_MSG_RESULT([OK.])
      AC_PATH_PROG(GLIB_COMPILE_RESOURCES,[glib-compile-resources],no)
      AC_MSG_CHECKING([glib-compile-resources])
      if test "x$GLIB_COMPILE_RESOURCES" = xno ; then
	GLIB_COMPILE_RESOURCES=""
	AC_MSG_RESULT([cannot be found in PATH.])
      else
	AC_MSG_RESULT([usable.])
	AC_DEFINE(USE_GRESOURCE)
	GRESOURCE_SRC="auto/gui_gtk_gresources.c"
	GRESOURCE_OBJ="objects/gui_gtk_gresources.o"
      fi
    else
      AC_MSG_RESULT([not usable.])
    fi
  else
    AC_MSG_RESULT([cannot obtain from pkg_config.])
  fi

  AC_MSG_CHECKING([--disable-icon-cache-update argument])
  AC_ARG_ENABLE(icon_cache_update,
          [  --disable-icon-cache-update        update disabled],
          [],
          [enable_icon_cache_update="yes"])
  if test "$enable_icon_cache_update" = "yes"; then
    AC_MSG_RESULT([not set])
    AC_PATH_PROG(GTK_UPDATE_ICON_CACHE,[gtk-update-icon-cache],no)
    if test "x$GTK_UPDATE_ICON_CACHE" = "xno" ; then
      AC_MSG_RESULT([not found in PATH.])
    fi
  else
    AC_MSG_RESULT([update disabled])
  fi

  AC_MSG_CHECKING([--disable-desktop-database-update argument])
  AC_ARG_ENABLE(desktop_database_update,
          [  --disable-desktop-database-update  update disabled],
          [],
          [enable_desktop_database_update="yes"])
  if test "$enable_desktop_database_update" = "yes"; then
    AC_MSG_RESULT([not set])
    AC_PATH_PROG(UPDATE_DESKTOP_DATABASE,[update-desktop-database],no)
    if test "x$UPDATE_DESKTOP_DATABASE" = "xno" ; then
      AC_MSG_RESULT([not found in PATH.])
    fi
  else
    AC_MSG_RESULT([update disabled])
  fi
fi
AC_SUBST(GLIB_COMPILE_RESOURCES)
AC_SUBST(GRESOURCE_SRC)
AC_SUBST(GRESOURCE_OBJ)
AC_SUBST(GTK_UPDATE_ICON_CACHE)
AC_SUBST(UPDATE_DESKTOP_DATABASE)

dnl Check for Motif include files location.
dnl The LAST one found is used, this makes the highest version to be used,
dnl e.g. when Motif1.2 and Motif2.0 are both present.

if test -z "$SKIP_MOTIF"; then
  gui_XXX="/usr/XXX/Motif* /usr/Motif*/XXX /usr/XXX /usr/shlib /usr/X11*/XXX /usr/XXX/X11* /usr/dt/XXX /local/Motif*/XXX /local/XXX/Motif* /usr/local/Motif*/XXX /usr/local/XXX/Motif* /usr/local/XXX /usr/local/X11*/XXX /usr/local/LessTif/Motif*/XXX $MOTIFHOME/XXX"
  dnl Remove "-I" from before $GUI_INC_LOC if it's there
  GUI_INC_LOC="`echo $GUI_INC_LOC|sed 's%-I%%g'`"

  AC_MSG_CHECKING(for location of Motif GUI includes)
  gui_includes="`echo $x_includes|sed 's%/[^/][^/]*$%%'` `echo "$gui_XXX" | sed s/XXX/include/g` $GUI_INC_LOC"
  GUI_INC_LOC=
  for try in $gui_includes; do
    if test -f "$try/Xm/Xm.h"; then
      GUI_INC_LOC=$try
    fi
  done
  if test -n "$GUI_INC_LOC"; then
    if test "$GUI_INC_LOC" = /usr/include; then
      GUI_INC_LOC=
      AC_MSG_RESULT(in default path)
    else
      AC_MSG_RESULT($GUI_INC_LOC)
    fi
  else
    AC_MSG_RESULT(<not found>)
    SKIP_MOTIF=YES
  fi
fi

dnl Check for Motif library files location.  In the same order as the include
dnl files, to avoid a mixup if several versions are present

if test -z "$SKIP_MOTIF"; then
  AC_MSG_CHECKING(--with-motif-lib argument)
  AC_ARG_WITH(motif-lib,
  [  --with-motif-lib=STRING Library for Motif ],
  [ MOTIF_LIBNAME="${withval}" ] )

  if test -n "$MOTIF_LIBNAME"; then
    AC_MSG_RESULT($MOTIF_LIBNAME)
    GUI_LIB_LOC=
  else
    AC_MSG_RESULT(no)

    dnl Remove "-L" from before $GUI_LIB_LOC if it's there
    GUI_LIB_LOC="`echo $GUI_LIB_LOC|sed 's%-L%%g'`"

    dnl Ubuntu has libXm.so in /usr/lib/i386-linux-gnu and elsewhere.  The
    dnl linker will figure out which one to use, we only check if one exists.
    dnl Cygwin uses the .dll.a extension.
    dnl OpenSUSE appears to use /usr/lib64.
    AC_MSG_CHECKING(for location of Motif GUI libs)
    gui_libs="`echo $x_libraries|sed 's%/[^/][^/]*$%%'` `echo "$gui_XXX" | sed s/XXX/lib/g` /usr/lib64 /usr/lib/i386-linux-gnu /usr/lib/x86_64-linux-gnu `echo "$GUI_INC_LOC" | sed s/include/lib/` $GUI_LIB_LOC"
    GUI_LIB_LOC=
    for try in $gui_libs; do
      for libtry in "$try"/libXm.a "$try"/libXm.dll.a "$try"/libXm.so* "$try"/libXm.sl "$try"/libXm.dylib; do
	if test -f "$libtry"; then
	  GUI_LIB_LOC=$try
	fi
      done
    done
    if test -n "$GUI_LIB_LOC"; then
      dnl Remove /usr/lib, it causes trouble on some systems.
      dnl For consistency also remove other standard lib directories.
      if test "$GUI_LIB_LOC" = /usr/lib \
	   -o "$GUI_LIB_LOC" = /usr/lib64 \
	   -o "$GUI_LIB_LOC" = /usr/lib/i386-linux-gnu \
	   -o "$GUI_LIB_LOC" = /usr/lib/x86_64-linux-gnu; then
	GUI_LIB_LOC=
	AC_MSG_RESULT(in default path)
      else
	if test -n "$GUI_LIB_LOC"; then
	  AC_MSG_RESULT($GUI_LIB_LOC)
	  if test "$vim_cv_uname_output" = SunOS &&
					 echo $vim_cv_uname_r_output | grep '^5' >/dev/null; then
	    GUI_LIB_LOC="$GUI_LIB_LOC -R $GUI_LIB_LOC"
	  fi
	fi
      fi
      MOTIF_LIBNAME=-lXm
    else
      AC_MSG_RESULT(<not found>)
      SKIP_MOTIF=YES
    fi
  fi
fi

if test -z "$SKIP_MOTIF"; then
  GUITYPE=MOTIF
  AC_SUBST(MOTIF_LIBNAME)
fi

if test -z "$SKIP_MOTIF"; then
  dnl Prepend -I and -L to $GUI_INC_LOC and $GUI_LIB_LOC if not empty
  dnl Avoid adding it when it twice
  if test -n "$GUI_INC_LOC"; then
    GUI_INC_LOC=-I"`echo $GUI_INC_LOC|sed 's%-I%%'`"
  fi
  if test -n "$GUI_LIB_LOC"; then
    GUI_LIB_LOC=-L"`echo $GUI_LIB_LOC|sed 's%-L%%'`"
  fi

  dnl Check for -lXext and then for -lXmu
  ldflags_save=$LDFLAGS
  LDFLAGS="$X_LIBS $LDFLAGS"
  AC_CHECK_LIB(Xext, XShapeQueryExtension, [GUI_X_LIBS="-lXext"],,
		[-lXt $X_PRE_LIBS -lX11 $X_EXTRA_LIBS])
  dnl For Solaris we need -lw and -ldl before linking with -lXmu works.
  AC_CHECK_LIB(w, wslen, [X_EXTRA_LIBS="$X_EXTRA_LIBS -lw"],,
		[$GUI_X_LIBS -lXt $X_PRE_LIBS -lX11 $X_EXTRA_LIBS])
  AC_CHECK_LIB(dl, dlsym, [X_EXTRA_LIBS="$X_EXTRA_LIBS -ldl"],,
		[$GUI_X_LIBS -lXt $X_PRE_LIBS -lX11 $X_EXTRA_LIBS])
  AC_CHECK_LIB(Xmu, XmuCreateStippledPixmap, [GUI_X_LIBS="-lXmu $GUI_X_LIBS"],,
		[$GUI_X_LIBS -lXt $X_PRE_LIBS -lX11 $X_EXTRA_LIBS])
  if test -z "$SKIP_MOTIF"; then
    AC_CHECK_LIB(Xp, XpEndJob, [GUI_X_LIBS="-lXp $GUI_X_LIBS"],,
		[$GUI_X_LIBS -lXm -lXt $X_PRE_LIBS -lX11 $X_EXTRA_LIBS])
  fi
  LDFLAGS=$ldflags_save

  dnl Execute xmkmf to figure out if -DNARROWPROTO is needed.
  AC_MSG_CHECKING(for extra X11 defines)
  NARROW_PROTO=
  rm -fr conftestdir
  if mkdir conftestdir; then
    cd conftestdir
    cat > Imakefile <<'EOF'
acfindx:
	@echo 'NARROW_PROTO="${PROTO_DEFINES}"'
EOF
    if (xmkmf) >/dev/null 2>/dev/null && test -f Makefile; then
      eval `${MAKE-make} acfindx 2>/dev/null | grep -v make`
    fi
    cd ..
    rm -fr conftestdir
  fi
  if test -z "$NARROW_PROTO"; then
    AC_MSG_RESULT(no)
  else
    AC_MSG_RESULT($NARROW_PROTO)
  fi
  AC_SUBST(NARROW_PROTO)
fi

dnl Look for XSMP support - but don't necessarily restrict it to X11 GUIs
dnl use the X11 include path
if test "$enable_xsmp" = "yes"; then
  cppflags_save=$CPPFLAGS
  CPPFLAGS="$CPPFLAGS $X_CFLAGS"
  AC_CHECK_HEADERS(X11/SM/SMlib.h)
  CPPFLAGS=$cppflags_save
fi


if test -z "$SKIP_MOTIF" -o -z "$SKIP_GTK2" -o -z "$SKIP_GTK3"; then
  dnl Check for X11/xpm.h and X11/Sunkeysym.h with the GUI include path
  cppflags_save=$CPPFLAGS
  CPPFLAGS="$CPPFLAGS $X_CFLAGS"
  AC_CHECK_HEADERS(X11/xpm.h X11/Sunkeysym.h)

  dnl automatically disable XIM when XIMtext isn't in X11/Xlib.h
  if test ! "$enable_xim" = "no"; then
    AC_MSG_CHECKING(for XIMText in X11/Xlib.h)
    AC_EGREP_CPP(XIMText, [#include <X11/Xlib.h>],
		  AC_MSG_RESULT(yes),
		  AC_MSG_RESULT(no; xim has been disabled); enable_xim="no")
  fi
  CPPFLAGS=$cppflags_save

  dnl automatically enable XIM in the GUI
  if test "$enable_xim" = "auto" -a "x$GUITYPE" != "xNONE" ; then
    AC_MSG_RESULT(X GUI selected; xim has been enabled)
    enable_xim="yes"
  fi
fi

if test -z "$SKIP_MOTIF"; then
  cppflags_save=$CPPFLAGS
  CPPFLAGS="$CPPFLAGS $X_CFLAGS"
dnl Xmu/Editres.h may exist but can only be used after including Intrinsic.h
  AC_MSG_CHECKING([for X11/Xmu/Editres.h])
  AC_COMPILE_IFELSE([AC_LANG_PROGRAM([
#include <X11/Intrinsic.h>
#include <X11/Xmu/Editres.h>],
		      [int i; i = 0;])],
	      AC_MSG_RESULT(yes)
		      AC_DEFINE(HAVE_X11_XMU_EDITRES_H),
	      AC_MSG_RESULT(no))
  CPPFLAGS=$cppflags_save
fi

dnl Only use the Xm directory when compiling Motif.
if test -z "$SKIP_MOTIF"; then
  cppflags_save=$CPPFLAGS
  CPPFLAGS="$CPPFLAGS $X_CFLAGS"
  if test "$zOSUnix" = "yes"; then
	AC_CHECK_HEADERS(Xm/Xm.h)
  else
	AC_CHECK_HEADERS(Xm/Xm.h Xm/XpmP.h Xm/JoinSideT.h Xm/TraitP.h Xm/Manager.h Xm/UnhighlightT.h Xm/Notebook.h)
  fi

  if test "x$ac_cv_header_Xm_XpmP_h" = "xyes"; then
    dnl Solaris uses XpmAttributes_21, very annoying.
    AC_MSG_CHECKING([for XpmAttributes_21 in Xm/XpmP.h])
    AC_COMPILE_IFELSE([AC_LANG_PROGRAM([#include <Xm/XpmP.h>], [XpmAttributes_21 attr;])],
	AC_MSG_RESULT(yes); AC_DEFINE(XPMATTRIBUTES_TYPE, XpmAttributes_21),
	AC_MSG_RESULT(no); AC_DEFINE(XPMATTRIBUTES_TYPE, XpmAttributes)
	)
  else
    AC_DEFINE(XPMATTRIBUTES_TYPE, XpmAttributes)
  fi
  CPPFLAGS=$cppflags_save
fi

if test "x$GUITYPE" = "xNONE" -a "$enable_xim" = "yes"; then
  AC_MSG_RESULT(no GUI selected; xim has been disabled)
  enable_xim="no"
fi
if test "x$GUITYPE" = "xNONE" -a "$enable_fontset" = "yes"; then
  AC_MSG_RESULT(no GUI selected; fontset has been disabled)
  enable_fontset="no"
fi
if test "x$GUITYPE:$enable_fontset" = "xGTK:yes"; then
  AC_MSG_RESULT(GTK+ 2 GUI selected; fontset has been disabled)
  enable_fontset="no"
fi

dnl There is no test for the Haiku GUI, if it's selected it's used
if test -z "$SKIP_HAIKU"; then
  GUITYPE=HAIKUGUI
fi

if test -z "$SKIP_PHOTON"; then
  GUITYPE=PHOTONGUI
fi

AC_SUBST(GUI_INC_LOC)
AC_SUBST(GUI_LIB_LOC)
AC_SUBST(GUITYPE)
AC_SUBST(GUI_X_LIBS)

if test "$enable_workshop" = "yes" -a -n "$SKIP_MOTIF"; then
  AC_MSG_ERROR([cannot use workshop without Motif])
fi

dnl defining FEAT_XIM and FEAT_XFONTSET is delayed, so that they can be disabled
if test "$enable_xim" = "yes"; then
  AC_DEFINE(FEAT_XIM)
fi
if test "$enable_fontset" = "yes"; then
  AC_DEFINE(FEAT_XFONTSET)
fi


dnl ---------------------------------------------------------------------------
dnl end of GUI-checking
dnl ---------------------------------------------------------------------------

AC_MSG_CHECKING([for /proc link to executable])
if test -L "/proc/self/exe"; then
  dnl Linux
  AC_MSG_RESULT([/proc/self/exe])
  AC_DEFINE(PROC_EXE_LINK, "/proc/self/exe")
elif test -L "/proc/self/path/a.out"; then
  dnl Solaris
  AC_MSG_RESULT([/proc/self/path/a.out])
  AC_DEFINE(PROC_EXE_LINK, "/proc/self/path/a.out")
elif test -L "/proc/curproc/file"; then
  dnl FreeBSD
  AC_MSG_RESULT([/proc/curproc/file])
  AC_DEFINE(PROC_EXE_LINK, "/proc/curproc/file")
else
  AC_MSG_RESULT(no)
fi

dnl Check for Cygwin, which needs an extra source file if not using X11
AC_MSG_CHECKING(for CYGWIN or MSYS environment)
case $vim_cv_uname_output in
    CYGWIN*|MSYS*)    CYGWIN=yes; AC_MSG_RESULT(yes)
                AC_MSG_CHECKING(for CYGWIN clipboard support)
                if test "x$with_x" = "xno" ; then
                  OS_EXTRA_SRC=winclip.c; OS_EXTRA_OBJ=objects/winclip.o
                  AC_MSG_RESULT(yes)
                  AC_DEFINE(FEAT_CYGWIN_WIN32_CLIPBOARD)
                else
                  AC_MSG_RESULT(no - using X11)
                fi ;;

    *)          CYGWIN=no; AC_MSG_RESULT(no);;
esac

dnl Checks for libraries and include files.

AC_CACHE_CHECK([whether toupper is broken], [vim_cv_toupper_broken],
  [
    AC_RUN_IFELSE([AC_LANG_SOURCE([[
#include "confdefs.h"
#include <ctype.h>
#if STDC_HEADERS
# include <stdlib.h>
# include <stddef.h>
#endif
int main() { exit(toupper('A') == 'A' && tolower('z') == 'z'); }
  ]])],[
    vim_cv_toupper_broken=yes
  ],[
    vim_cv_toupper_broken=no
  ],[
    AC_MSG_ERROR(cross-compiling: please set 'vim_cv_toupper_broken')
  ])])

if test "x$vim_cv_toupper_broken" = "xyes" ; then
  AC_DEFINE(BROKEN_TOUPPER)
fi

AC_MSG_CHECKING(whether __DATE__ and __TIME__ work)
AC_COMPILE_IFELSE([AC_LANG_PROGRAM([#include <stdio.h>], [printf("(" __DATE__ " " __TIME__ ")");])],
	AC_MSG_RESULT(yes); AC_DEFINE(HAVE_DATE_TIME),
	AC_MSG_RESULT(no))

AC_MSG_CHECKING(whether __attribute__((unused)) is allowed)
AC_COMPILE_IFELSE([AC_LANG_PROGRAM([#include <stdio.h>], [int x __attribute__((unused));])],
	AC_MSG_RESULT(yes); AC_DEFINE(HAVE_ATTRIBUTE_UNUSED),
	AC_MSG_RESULT(no))

dnl Checks for header files.
AC_CHECK_HEADER(elf.h, HAS_ELF=1)
dnl AC_CHECK_HEADER(dwarf.h, SVR4=1)
if test "$HAS_ELF" = 1; then
  AC_CHECK_LIB(elf, main)
fi

AC_HEADER_DIRENT

dnl If sys/wait.h is not found it might still exist but not be POSIX
dnl compliant. In that case we define HAVE_UNION_WAIT (for NeXT)
if test $ac_cv_header_sys_wait_h = no; then
  AC_MSG_CHECKING([for sys/wait.h that defines union wait])
  AC_COMPILE_IFELSE([AC_LANG_PROGRAM([#include <sys/wait.h>],
			[union wait xx, yy; xx = yy])],
		AC_MSG_RESULT(yes)
			AC_DEFINE(HAVE_SYS_WAIT_H)
			AC_DEFINE(HAVE_UNION_WAIT),
		AC_MSG_RESULT(no))
fi

AC_CHECK_HEADERS(stdint.h stdlib.h string.h \
	sys/select.h sys/utsname.h termcap.h fcntl.h \
	sgtty.h sys/ioctl.h sys/time.h sys/types.h \
	termio.h iconv.h inttypes.h langinfo.h math.h \
	unistd.h stropts.h errno.h sys/resource.h \
	sys/systeminfo.h locale.h sys/stream.h termios.h \
	libc.h sys/statfs.h poll.h sys/poll.h pwd.h \
	utime.h sys/param.h sys/ptms.h libintl.h libgen.h \
	util/debug.h util/msg18n.h frame.h sys/acl.h \
	sys/access.h sys/sysinfo.h wchar.h wctype.h)

dnl sys/ptem.h depends on sys/stream.h on Solaris
AC_CHECK_HEADERS(sys/ptem.h, [], [],
[#if defined HAVE_SYS_STREAM_H
#  include <sys/stream.h>
#endif])

dnl sys/sysctl.h depends on sys/param.h on OpenBSD
AC_CHECK_HEADERS(sys/sysctl.h, [], [],
[#if defined HAVE_SYS_PARAM_H
#  include <sys/param.h>
#endif])


dnl pthread_np.h may exist but can only be used after including pthread.h
AC_MSG_CHECKING([for pthread_np.h])
AC_COMPILE_IFELSE([AC_LANG_PROGRAM([
#include <pthread.h>
#include <pthread_np.h>],
		      [int i; i = 0;])],
	      AC_MSG_RESULT(yes)
		      AC_DEFINE(HAVE_PTHREAD_NP_H),
	      AC_MSG_RESULT(no))

AC_CHECK_HEADERS(strings.h)
if test "x$MACOS_X" = "xyes"; then
  dnl The strings.h file on OS/X contains a warning and nothing useful.
  AC_DEFINE(NO_STRINGS_WITH_STRING_H)
else

dnl Check if strings.h and string.h can both be included when defined.
AC_MSG_CHECKING([if strings.h can be included after string.h])
cppflags_save=$CPPFLAGS
CPPFLAGS="$CPPFLAGS $X_CFLAGS"
AC_COMPILE_IFELSE([AC_LANG_PROGRAM([
#if defined(_AIX) && !defined(_AIX51) && !defined(_NO_PROTO)
# define _NO_PROTO	/* like in os_unix.h, causes conflict for AIX (Winn) */
			/* but don't do it on AIX 5.1 (Uribarri) */
#endif
#ifdef HAVE_XM_XM_H
# include <Xm/Xm.h>	/* This breaks it for HP-UX 11 (Squassabia) */
#endif
#ifdef HAVE_STRING_H
# include <string.h>
#endif
#if defined(HAVE_STRINGS_H)
# include <strings.h>
#endif
		], [int i; i = 0;])],
		AC_MSG_RESULT(yes),
		AC_DEFINE(NO_STRINGS_WITH_STRING_H)
		AC_MSG_RESULT(no))
CPPFLAGS=$cppflags_save
fi

dnl Checks for typedefs, structures, and compiler characteristics.
AC_PROG_GCC_TRADITIONAL
AC_C_CONST
AC_C_VOLATILE
AC_TYPE_MODE_T
AC_TYPE_OFF_T
AC_TYPE_PID_T
AC_TYPE_SIZE_T
AC_TYPE_UID_T
AC_TYPE_UINT32_T

AC_CHECK_HEADERS_ONCE([sys/time.h])

AC_CHECK_TYPE(ino_t, long)
AC_CHECK_TYPE(dev_t, unsigned)
AC_C_BIGENDIAN(,,,)
AC_C_INLINE

AC_MSG_CHECKING(for rlim_t)
if eval "test \"`echo '$''{'ac_cv_type_rlim_t'+set}'`\" = set"; then
  AC_MSG_RESULT([(cached) $ac_cv_type_rlim_t])
else
  AC_EGREP_CPP(dnl
changequote(<<,>>)dnl
<<(^|[^a-zA-Z_0-9])rlim_t[^a-zA-Z_0-9]>>dnl
changequote([,]),
  [
#include <sys/types.h>
#if STDC_HEADERS
# include <stdlib.h>
# include <stddef.h>
#endif
#ifdef HAVE_SYS_RESOURCE_H
# include <sys/resource.h>
#endif
	  ], ac_cv_type_rlim_t=yes, ac_cv_type_rlim_t=no)
	  AC_MSG_RESULT($ac_cv_type_rlim_t)
fi
if test $ac_cv_type_rlim_t = no; then
  cat >> confdefs.h <<\EOF
#define rlim_t unsigned long
EOF
fi

AC_MSG_CHECKING(for stack_t)
if eval "test \"`echo '$''{'ac_cv_type_stack_t'+set}'`\" = set"; then
  AC_MSG_RESULT([(cached) $ac_cv_type_stack_t])
else
  AC_EGREP_CPP(stack_t,
  [
#include <sys/types.h>
#if STDC_HEADERS
# include <stdlib.h>
# include <stddef.h>
#endif
#include <signal.h>
	  ], ac_cv_type_stack_t=yes, ac_cv_type_stack_t=no)
	  AC_MSG_RESULT($ac_cv_type_stack_t)
fi
if test $ac_cv_type_stack_t = no; then
  cat >> confdefs.h <<\EOF
#define stack_t struct sigaltstack
EOF
fi

dnl BSDI uses ss_base while others use ss_sp for the stack pointer.
AC_MSG_CHECKING(whether stack_t has an ss_base field)
AC_COMPILE_IFELSE([AC_LANG_PROGRAM([
#include <sys/types.h>
#if STDC_HEADERS
# include <stdlib.h>
# include <stddef.h>
#endif
#include <signal.h>
#include "confdefs.h"
			], [stack_t sigstk; sigstk.ss_base = 0; ])],
	AC_MSG_RESULT(yes); AC_DEFINE(HAVE_SS_BASE),
	AC_MSG_RESULT(no))

olibs="$LIBS"
AC_MSG_CHECKING(--with-tlib argument)
AC_ARG_WITH(tlib, [  --with-tlib=library     terminal library to be used ],)
if test -n "$with_tlib"; then
  AC_MSG_RESULT($with_tlib)
  LIBS="$LIBS -l$with_tlib"
  AC_MSG_CHECKING(for linking with $with_tlib library)
  AC_LINK_IFELSE([AC_LANG_PROGRAM([], [])], AC_MSG_RESULT(OK), AC_MSG_ERROR(FAILED))
  dnl Need to check for tgetent() below.
  olibs="$LIBS"
else
  AC_MSG_RESULT([empty: automatic terminal library selection])
  dnl  On HP-UX 10.10 termcap or termlib should be used instead of
  dnl  curses, because curses is much slower.
  dnl  Newer versions of ncurses are preferred over anything, except
  dnl  when tinfo has been split off, it contains all we need.
  dnl  Older versions of ncurses have bugs, get a new one!
  dnl  Digital Unix (OSF1) should use curses (Ronald Schild).
  dnl  On SCO Openserver should prefer termlib (Roger Cornelius).
  case "$vim_cv_uname_output" in
	OSF1|SCO_SV)	tlibs="tinfo ncurses curses termlib termcap";;
	*)	tlibs="tinfo ncurses termlib termcap curses";;
  esac
  for libname in $tlibs; do
    AC_CHECK_LIB(${libname}, tgetent,,)
    if test "x$olibs" != "x$LIBS"; then
      dnl It's possible that a library is found but it doesn't work
      dnl e.g., shared library that cannot be found
      dnl compile and run a test program to be sure
      AC_RUN_IFELSE([AC_LANG_SOURCE([
#ifdef HAVE_TERMCAP_H
# include <termcap.h>
#endif
#if STDC_HEADERS
# include <stdlib.h>
# include <stddef.h>
#endif
int main() {char *s; s=(char *)tgoto("%p1%d", 0, 1); exit(0); }])],
			  res="OK", res="FAIL", res="FAIL")
      if test "$res" = "OK"; then
	break
      fi
      AC_MSG_RESULT($libname library is not usable)
      LIBS="$olibs"
    fi
  done
  if test "x$olibs" = "x$LIBS"; then
    AC_MSG_RESULT(no terminal library found)
  fi
fi

if test "x$olibs" = "x$LIBS"; then
  AC_MSG_CHECKING([for tgetent()])
  AC_LINK_IFELSE([AC_LANG_PROGRAM([int tgetent(char *, const char *);],
      [[char s[10000]; int res = tgetent(s, "thisterminaldoesnotexist");]])],
	AC_MSG_RESULT(yes),
	AC_MSG_ERROR([NOT FOUND!
      You need to install a terminal library; for example ncurses.
      On Linux that would be the libncurses-dev package.
      Or specify the name of the library with --with-tlib.]))
fi

AC_CACHE_CHECK([whether we talk terminfo], [vim_cv_terminfo],
  [
    AC_RUN_IFELSE([AC_LANG_SOURCE([[
#include "confdefs.h"
#ifdef HAVE_TERMCAP_H
# include <termcap.h>
#endif
#ifdef HAVE_STRING_H
# include <string.h>
#endif
#if STDC_HEADERS
# include <stdlib.h>
# include <stddef.h>
#endif
int main()
{char *s; s=(char *)tgoto("%p1%d", 0, 1); exit(!strcmp(s==0 ? "" : s, "1")); }
    ]])],[
      vim_cv_terminfo=no
    ],[
      vim_cv_terminfo=yes
    ],[
      AC_MSG_ERROR(cross-compiling: please set 'vim_cv_terminfo')
    ])
  ])

if test "x$vim_cv_terminfo" = "xyes" ; then
  AC_DEFINE(TERMINFO)
fi

AC_CACHE_CHECK([what tgetent() returns for an unknown terminal], [vim_cv_tgetent],
  [
    AC_RUN_IFELSE([AC_LANG_SOURCE([[
#include "confdefs.h"
#ifdef HAVE_TERMCAP_H
# include <termcap.h>
#endif
#if STDC_HEADERS
# include <stdlib.h>
# include <stddef.h>
#endif
int main()
{char s[10000]; int res = tgetent(s, "thisterminaldoesnotexist"); exit(res != 0); }
    ]])],[
      vim_cv_tgetent=zero
    ],[
      vim_cv_tgetent=non-zero
    ],[
      AC_MSG_ERROR(failed to compile test program.)
    ])
  ])

if test "x$vim_cv_tgetent" = "xzero" ; then
  AC_DEFINE(TGETENT_ZERO_ERR, 0)
fi

AC_MSG_CHECKING(whether termcap.h contains ospeed)
AC_LINK_IFELSE([AC_LANG_PROGRAM([
#ifdef HAVE_TERMCAP_H
# include <termcap.h>
#endif
			], [ospeed = 20000])],
	AC_MSG_RESULT(yes); AC_DEFINE(HAVE_OSPEED),
	[AC_MSG_RESULT(no)
	AC_MSG_CHECKING(whether ospeed can be extern)
	AC_LINK_IFELSE([AC_LANG_PROGRAM([
#ifdef HAVE_TERMCAP_H
# include <termcap.h>
#endif
extern short ospeed;
			], [ospeed = 20000])],
		AC_MSG_RESULT(yes); AC_DEFINE(OSPEED_EXTERN),
		AC_MSG_RESULT(no))]
	)

AC_MSG_CHECKING([whether termcap.h contains UP, BC and PC])
AC_LINK_IFELSE([AC_LANG_PROGRAM([
#ifdef HAVE_TERMCAP_H
# include <termcap.h>
#endif
			], [if (UP == 0 && BC == 0) PC = 1])],
	AC_MSG_RESULT(yes); AC_DEFINE(HAVE_UP_BC_PC),
	[AC_MSG_RESULT(no)
	AC_MSG_CHECKING([whether UP, BC and PC can be extern])
	AC_LINK_IFELSE([AC_LANG_PROGRAM([
#ifdef HAVE_TERMCAP_H
# include <termcap.h>
#endif
extern char *UP, *BC, PC;
			], [if (UP == 0 && BC == 0) PC = 1])],
		AC_MSG_RESULT(yes); AC_DEFINE(UP_BC_PC_EXTERN),
		AC_MSG_RESULT(no))]
	)

AC_MSG_CHECKING(whether tputs() uses outfuntype)
AC_COMPILE_IFELSE([AC_LANG_PROGRAM([
#ifdef HAVE_TERMCAP_H
# include <termcap.h>
#endif
			], [extern int xx(); tputs("test", 1, (outfuntype)xx)])],
	AC_MSG_RESULT(yes); AC_DEFINE(HAVE_OUTFUNTYPE),
	AC_MSG_RESULT(no))

AC_MSG_CHECKING([whether del_curterm() can be used])
AC_LINK_IFELSE([AC_LANG_PROGRAM([
#ifdef HAVE_TERMCAP_H
# include <termcap.h>
#endif
#include <term.h>
			], [if (cur_term) del_curterm(cur_term);])],
	AC_MSG_RESULT(yes); AC_DEFINE(HAVE_DEL_CURTERM),
	AC_MSG_RESULT(no))

dnl On some SCO machines sys/select redefines struct timeval
AC_MSG_CHECKING([whether sys/select.h and sys/time.h may both be included])
AC_COMPILE_IFELSE([AC_LANG_PROGRAM([
#include <sys/types.h>
#include <sys/time.h>
#include <sys/select.h>], )],
	  AC_MSG_RESULT(yes)
			AC_DEFINE(SYS_SELECT_WITH_SYS_TIME),
	  AC_MSG_RESULT(no))

dnl Checks for pty.c (copied from screen) ==========================
AC_MSG_CHECKING(for /dev/ptc)
if test -r /dev/ptc; then
  AC_DEFINE(HAVE_DEV_PTC)
  AC_MSG_RESULT(yes)
else
  AC_MSG_RESULT(no)
fi

AC_MSG_CHECKING(for SVR4 ptys)
if test -c /dev/ptmx ; then
  AC_LINK_IFELSE([AC_LANG_PROGRAM([
// These should be in stdlib.h, but it depends on _XOPEN_SOURCE.
char *ptsname(int);
int unlockpt(int);
int grantpt(int);
	       ], [
		ptsname(0);
		grantpt(0);
		unlockpt(0);])],
	AC_MSG_RESULT(yes); AC_DEFINE(HAVE_SVR4_PTYS),
	AC_MSG_RESULT(no))
else
  AC_MSG_RESULT(no)
fi

AC_MSG_CHECKING(for ptyranges)
if test -d /dev/ptym ; then
  pdir='/dev/ptym'
else
  pdir='/dev'
fi
dnl SCO uses ptyp%d
AC_EGREP_CPP(yes,
[#ifdef M_UNIX
   yes;
#endif
	], ptys=`echo /dev/ptyp??`, ptys=`echo $pdir/pty??`)
dnl if test -c /dev/ptyp19; then
dnl ptys=`echo /dev/ptyp??`
dnl else
dnl ptys=`echo $pdir/pty??`
dnl fi
if test "$ptys" != "$pdir/pty??" ; then
  p0=`echo $ptys | tr ' ' '\012' | sed -e 's/^.*\(.\).$/\1/g' | sort -u | tr -d '\012'`
  p1=`echo $ptys | tr ' ' '\012' | sed -e 's/^.*\(.\)$/\1/g'  | sort -u | tr -d '\012'`
  AC_DEFINE_UNQUOTED(PTYRANGE0,"$p0")
  AC_DEFINE_UNQUOTED(PTYRANGE1,"$p1")
  AC_MSG_RESULT([$p0 / $p1])
else
  AC_MSG_RESULT([don't know])
fi

dnl Checks for library functions. ===================================

dnl check if struct sigcontext is defined (used for SGI only)
AC_MSG_CHECKING(for struct sigcontext)
AC_COMPILE_IFELSE([AC_LANG_PROGRAM([
#include <signal.h>
int test_sig()
{
    struct sigcontext *scont;
    scont = (struct sigcontext *)0;
    return 1;
} ], )],
	  AC_MSG_RESULT(yes)
		AC_DEFINE(HAVE_SIGCONTEXT),
	  AC_MSG_RESULT(no))

dnl tricky stuff: try to find out if getcwd() is implemented with
dnl system("sh -c pwd")
AC_CACHE_CHECK([getcwd implementation is broken], [vim_cv_getcwd_broken],
  [
    AC_RUN_IFELSE([AC_LANG_SOURCE([[
#include "confdefs.h"
#ifdef HAVE_UNISTD_H
#include <unistd.h>
#endif
char *dagger[] = { "IFS=pwd", 0 };
int main()
{
  char buffer[500];
  extern char **environ;
  environ = dagger;
  return getcwd(buffer, 500) ? 0 : 1;
}
    ]])],[
      vim_cv_getcwd_broken=no
    ],[
      vim_cv_getcwd_broken=yes
    ],[
      AC_MSG_ERROR(cross-compiling: please set 'vim_cv_getcwd_broken')
    ])
  ])

if test "x$vim_cv_getcwd_broken" = "xyes" ; then
  AC_DEFINE(BAD_GETCWD)
  AC_CHECK_FUNCS(getwd)
fi

dnl Check for functions in one big call, to reduce the size of configure.
dnl Can only be used for functions that do not require any include.
AC_CHECK_FUNCS(fchdir fchown fchmod fsync getcwd getpseudotty \
	getpwent getpwnam getpwuid getrlimit gettimeofday localtime_r lstat \
	memset mkdtemp nanosleep opendir putenv qsort readlink select setenv \
	getpgid setpgid setsid sigaltstack sigstack sigset sigsetjmp sigaction \
	sigprocmask sigvec strcasecmp strcoll strerror strftime stricmp strncasecmp \
	strnicmp strpbrk strptime strtol tgetent towlower towupper iswupper \
	tzset usleep utime utimes mblen ftruncate unsetenv posix_openpt \
	clock_gettime)
AC_FUNC_SELECT_ARGTYPES
AC_FUNC_FSEEKO

dnl define _LARGE_FILES, _FILE_OFFSET_BITS and _LARGEFILE_SOURCE when
dnl appropriate, so that off_t is 64 bits when needed.
AC_SYS_LARGEFILE

AC_MSG_CHECKING(--enable-canberra argument)
AC_ARG_ENABLE(canberra,
	[  --disable-canberra      Do not use libcanberra.],
	, [enable_canberra="maybe"])

if test "$enable_canberra" = "maybe"; then
  if test "$features" = "huge"; then
    AC_MSG_RESULT(Defaulting to yes)
    enable_canberra="yes"
  else
    AC_MSG_RESULT(Defaulting to no)
    enable_canberra="no"
  fi
else
  if test "$enable_canberra" = "yes" -a "$has_eval" = "no"; then
    AC_MSG_RESULT([cannot use sound with tiny features])
    enable_canberra="no"
  else
    AC_MSG_RESULT($enable_canberra)
  fi
fi
if test "$enable_canberra" = "yes"; then
  if test "x$PKG_CONFIG" != "xno"; then
    canberra_lib=`$PKG_CONFIG --libs libcanberra 2>/dev/null`
    canberra_cflags=`$PKG_CONFIG --cflags libcanberra 2>/dev/null`
  fi
  if test "x$canberra_lib" = "x"; then
    canberra_lib=-lcanberra
    canberra_cflags=-D_REENTRANT
  fi
  AC_MSG_CHECKING(for libcanberra)
  ac_save_CFLAGS="$CFLAGS"
  ac_save_LIBS="$LIBS"
  if `echo "$CFLAGS" | grep -v "$canberra_cflags" 2>/dev/null`; then
    CFLAGS="$CFLAGS $canberra_cflags"
  fi
  LIBS="$LIBS $canberra_lib"
  AC_LINK_IFELSE([AC_LANG_PROGRAM([
  # include <canberra.h>
      ], [
     ca_context *hello;
     ca_context_create(&hello);])],
       AC_MSG_RESULT(yes); AC_DEFINE(HAVE_CANBERRA),
       AC_MSG_RESULT(no; try installing libcanberra-dev); CFLAGS="$ac_save_CFLAGS"; LIBS="$ac_save_LIBS")
fi

AC_MSG_CHECKING(--enable-libsodium argument)
AC_ARG_ENABLE(libsodium,
	[  --disable-libsodium      Do not use libsodium.],
	, [enable_libsodium="maybe"])

if test "$enable_libsodium" = "maybe"; then
  if test "$features" = "huge"; then
    AC_MSG_RESULT(Defaulting to yes)
    enable_libsodium="yes"
  else
    AC_MSG_RESULT(Defaulting to no)
    enable_libsodium="no"
  fi
else
  AC_MSG_RESULT($enable_libsodium)
fi
if test "$enable_libsodium" = "yes"; then
  if test "x$PKG_CONFIG" != "xno"; then
    libsodium_lib=`$PKG_CONFIG --libs libsodium 2>/dev/null`
    libsodium_cflags=`$PKG_CONFIG --cflags libsodium 2>/dev/null`
  fi
  if test "x$libsodium_lib" = "x"; then
    libsodium_lib=-lsodium
    libsodium_cflags=
  fi
  AC_MSG_CHECKING(for libsodium)
  ac_save_CFLAGS="$CFLAGS"
  ac_save_LIBS="$LIBS"
  CFLAGS="$CFLAGS $libsodium_cflags"
  LIBS="$LIBS $libsodium_lib"
  AC_LINK_IFELSE([AC_LANG_PROGRAM([
  # include <sodium.h>
      ], [
     printf("%d", sodium_init()); ])],
       AC_MSG_RESULT(yes); AC_DEFINE(HAVE_SODIUM),
       AC_MSG_RESULT(no; try installing libsodium-dev); CFLAGS="$ac_save_CFLAGS"; LIBS="$ac_save_LIBS")
fi

dnl fstatfs() can take 2 to 4 arguments, try to use st_blksize if possible
AC_MSG_CHECKING(for st_blksize)
AC_COMPILE_IFELSE([AC_LANG_PROGRAM(
[#include <sys/types.h>
#include <sys/stat.h>],
[	struct stat st;
	int n;

	stat("/", &st);
	n = (int)st.st_blksize;])],
	AC_MSG_RESULT(yes); AC_DEFINE(HAVE_ST_BLKSIZE),
	AC_MSG_RESULT(no))

dnl Check for timer_create. It probably requires the 'rt' library.
dnl Run the program to find out if timer_create(CLOCK_MONOTONIC) actually
dnl works, on Solaris timer_create() exists but fails at runtime.
AC_CACHE_CHECK([for timer_create without -lrt], [vim_cv_timer_create], [
AC_RUN_IFELSE([AC_LANG_PROGRAM([
#if STDC_HEADERS
# include <stdlib.h>
# include <stddef.h>
#endif
#include <signal.h>
#include <time.h>
static void set_flag(union sigval sv) {}
], [
  struct timespec ts;
  struct sigevent action = {0};
  timer_t timer_id;

  action.sigev_notify = SIGEV_THREAD;
  action.sigev_notify_function = set_flag;
  if (timer_create(CLOCK_MONOTONIC, &action, &timer_id) < 0)
    exit(1);  // cannot create a monotonic timer
  ])],
  vim_cv_timer_create=yes,
  vim_cv_timer_create=no,
  AC_MSG_WARN([failed to build test program; if cross-compiling please set 'vim_cv_timer_create'])
  )])

dnl If the previous failed, check for timer_create() and linking with -lrt.
if test "x$vim_cv_timer_create" = "xno" ; then
  save_LIBS="$LIBS"
  LIBS="$LIBS -lrt"
  AC_CACHE_CHECK([for timer_create with -lrt], [vim_cv_timer_create_with_lrt], [
    AC_RUN_IFELSE([AC_LANG_PROGRAM([
    #if STDC_HEADERS
    # include <stdlib.h>
    # include <stddef.h>
    #endif
    #include <signal.h>
    #include <time.h>
    static void set_flag(union sigval sv) {}
    ], [
      struct timespec ts;
      struct sigevent action = {0};
      timer_t timer_id;

      action.sigev_notify = SIGEV_THREAD;
      action.sigev_notify_function = set_flag;
      if (timer_create(CLOCK_MONOTONIC, &action, &timer_id) < 0)
	exit(1);  // cannot create a monotonic timer
      ])],
      vim_cv_timer_create_with_lrt=yes,
      vim_cv_timer_create_with_lrt=no,
      AC_MSG_WARN([failed to build test program; if cross-compiling please set 'vim_cv_timer_create_with_lrt'])
    )])
  LIBS="$save_LIBS"
else
  vim_cv_timer_create_with_lrt=no
fi

if test "x$vim_cv_timer_create" = "xyes" ; then
  AC_DEFINE(HAVE_TIMER_CREATE)
fi
if test "x$vim_cv_timer_create_with_lrt" = "xyes" ; then
  AC_DEFINE(HAVE_TIMER_CREATE)
  LIBS="$LIBS -lrt"
fi

AC_CACHE_CHECK([whether stat() ignores a trailing slash], [vim_cv_stat_ignores_slash],
  [
    AC_RUN_IFELSE([AC_LANG_SOURCE([[
#include "confdefs.h"
#if STDC_HEADERS
# include <stdlib.h>
# include <stddef.h>
#endif
#include <sys/types.h>
#include <sys/stat.h>
int main() {struct stat st;  exit(stat("configure/", &st) != 0); }
    ]])],[
      vim_cv_stat_ignores_slash=yes
    ],[
      vim_cv_stat_ignores_slash=no
    ],[
      AC_MSG_ERROR(cross-compiling: please set 'vim_cv_stat_ignores_slash')
    ])
  ])

if test "x$vim_cv_stat_ignores_slash" = "xyes" ; then
  AC_DEFINE(STAT_IGNORES_SLASH)
fi

dnl nanoseconds field of struct stat
AC_CACHE_CHECK([for nanoseconds field of struct stat],
  ac_cv_struct_st_mtim_nsec,
  [ac_save_CPPFLAGS="$CPPFLAGS"
   ac_cv_struct_st_mtim_nsec=no
   # st_mtim.tv_nsec -- the usual case
   # st_mtim._tv_nsec -- Solaris 2.6, if
   #	(defined _XOPEN_SOURCE && _XOPEN_SOURCE_EXTENDED == 1
   #	 && !defined __EXTENSIONS__)
   # st_mtim.st__tim.tv_nsec -- UnixWare 2.1.2
   # st_mtime_n -- AIX 5.2 and above
   # st_mtimespec.tv_nsec -- Darwin (Mac OSX)
   for ac_val in st_mtim.tv_nsec st_mtim._tv_nsec st_mtim.st__tim.tv_nsec st_mtime_n st_mtimespec.tv_nsec; do
     CPPFLAGS="$ac_save_CPPFLAGS -DST_MTIM_NSEC=$ac_val"
     AC_COMPILE_IFELSE([AC_LANG_PROGRAM([#include <sys/types.h>
#include <sys/stat.h>], [struct stat s; s.ST_MTIM_NSEC;])],
       [ac_cv_struct_st_mtim_nsec=$ac_val; break])
   done
   CPPFLAGS="$ac_save_CPPFLAGS"
])
if test $ac_cv_struct_st_mtim_nsec != no; then
  AC_DEFINE_UNQUOTED([ST_MTIM_NSEC], [$ac_cv_struct_st_mtim_nsec],
  [Define if struct stat contains a nanoseconds field])
fi

dnl Link with iconv for charset translation, if not found without library.
dnl check for iconv() requires including iconv.h
dnl Add "-liconv" when possible; Solaris has iconv but use GNU iconv when it
dnl has been installed.
AC_MSG_CHECKING(for iconv_open())
save_LIBS="$LIBS"
LIBS="$LIBS -liconv"
AC_LINK_IFELSE([AC_LANG_PROGRAM([
#ifdef HAVE_ICONV_H
# include <iconv.h>
#endif
    ], [iconv_open("fr", "to");])],
    AC_MSG_RESULT(yes; with -liconv); AC_DEFINE(HAVE_ICONV),
    LIBS="$save_LIBS"
    AC_LINK_IFELSE([AC_LANG_PROGRAM([
#ifdef HAVE_ICONV_H
# include <iconv.h>
#endif
	], [iconv_open("fr", "to");])],
	AC_MSG_RESULT(yes); AC_DEFINE(HAVE_ICONV),
	AC_MSG_RESULT(no)))


AC_MSG_CHECKING(for nl_langinfo(CODESET))
AC_LINK_IFELSE([AC_LANG_PROGRAM([
#ifdef HAVE_LANGINFO_H
# include <langinfo.h>
#endif
], [char *cs = nl_langinfo(CODESET);])],
	AC_MSG_RESULT(yes); AC_DEFINE(HAVE_NL_LANGINFO_CODESET),
	AC_MSG_RESULT(no))

dnl Floating point support may require the "m" library
AC_CHECK_LIB(m, strtod)

dnl isinf() and isnan() need to include header files and may need -lm.
AC_MSG_CHECKING([for isinf()])
AC_LINK_IFELSE([AC_LANG_PROGRAM([
#ifdef HAVE_MATH_H
# include <math.h>
#endif
#if STDC_HEADERS
# include <stdlib.h>
# include <stddef.h>
#endif
], [int r = isinf(1.11); ])],
	AC_MSG_RESULT(yes); AC_DEFINE(HAVE_ISINF),
	AC_MSG_RESULT(no))

AC_MSG_CHECKING([for isnan()])
AC_LINK_IFELSE([AC_LANG_PROGRAM([
#ifdef HAVE_MATH_H
# include <math.h>
#endif
#if STDC_HEADERS
# include <stdlib.h>
# include <stddef.h>
#endif
], [int r = isnan(1.11); ])],
	AC_MSG_RESULT(yes); AC_DEFINE(HAVE_ISNAN),
	AC_MSG_RESULT(no))

dnl Link with -lposix1e for ACL stuff; if not found, try -lacl for SGI
dnl when -lacl works, also try to use -lattr (required for Debian).
dnl On Solaris, use the acl_get/set functions in libsec, if present.
AC_MSG_CHECKING(--disable-acl argument)
AC_ARG_ENABLE(acl,
	[  --disable-acl           No check for ACL support.],
	, [enable_acl="yes"])
if test "$enable_acl" = "yes"; then
  AC_MSG_RESULT(no)
  AC_CHECK_LIB(posix1e, acl_get_file, [LIBS="$LIBS -lposix1e"],
	AC_CHECK_LIB(acl, acl_get_file, [LIBS="$LIBS -lacl"
		  AC_CHECK_LIB(attr, fgetxattr, LIBS="$LIBS -lattr",,)],,),)

  AC_MSG_CHECKING(for POSIX ACL support)
  AC_LINK_IFELSE([AC_LANG_PROGRAM([
#include <sys/types.h>
#ifdef HAVE_SYS_ACL_H
# include <sys/acl.h>
#endif
acl_t acl;], [acl = acl_get_file("foo", ACL_TYPE_ACCESS);
	acl_set_file("foo", ACL_TYPE_ACCESS, acl);
	acl_free(acl);])],
	AC_MSG_RESULT(yes); AC_DEFINE(HAVE_POSIX_ACL),
	AC_MSG_RESULT(no))

  AC_CHECK_LIB(sec, acl_get, [LIBS="$LIBS -lsec"; AC_DEFINE(HAVE_SOLARIS_ZFS_ACL)],
  AC_MSG_CHECKING(for Solaris ACL support)
  AC_LINK_IFELSE([AC_LANG_PROGRAM([
#ifdef HAVE_SYS_ACL_H
# include <sys/acl.h>
#endif], [acl("foo", GETACLCNT, 0, NULL);
	])],
	AC_MSG_RESULT(yes); AC_DEFINE(HAVE_SOLARIS_ACL),
	AC_MSG_RESULT(no)))

  AC_MSG_CHECKING(for AIX ACL support)
  AC_LINK_IFELSE([AC_LANG_PROGRAM([
#if STDC_HEADERS
# include <stdlib.h>
# include <stddef.h>
#endif
#ifdef HAVE_SYS_ACL_H
# include <sys/acl.h>
#endif
#ifdef HAVE_SYS_ACCESS_H
# include <sys/access.h>
#endif
#define _ALL_SOURCE

#include <sys/stat.h>

int aclsize;
struct acl *aclent;], [aclsize = sizeof(struct acl);
	aclent = (void *)malloc(aclsize);
	statacl("foo", STX_NORMAL, aclent, aclsize);
	])],
	AC_MSG_RESULT(yes); AC_DEFINE(HAVE_AIX_ACL),
	AC_MSG_RESULT(no))
else
  AC_MSG_RESULT(yes)
fi

if test "x$GTK_CFLAGS" != "x"; then
  dnl pango_shape_full() is new, fall back to pango_shape().
  AC_MSG_CHECKING(for pango_shape_full)
  ac_save_CFLAGS="$CFLAGS"
  ac_save_LIBS="$LIBS"
  CFLAGS="$CFLAGS $GTK_CFLAGS"
  LIBS="$LIBS $GTK_LIBS"
  AC_LINK_IFELSE([AC_LANG_PROGRAM(
  [#include <gtk/gtk.h>],
  [ pango_shape_full(NULL, 0, NULL, 0, NULL, NULL); ])],
	  AC_MSG_RESULT(yes); AC_DEFINE(HAVE_PANGO_SHAPE_FULL),
	  AC_MSG_RESULT(no))
  CFLAGS="$ac_save_CFLAGS"
  LIBS="$ac_save_LIBS"
fi

AC_MSG_CHECKING(--enable-gpm argument)
AC_ARG_ENABLE(gpm,
	[  --enable-gpm=OPTS       Use gpm (Linux mouse daemon). default=yes OPTS=yes/no/dynamic], ,
	[enable_gpm="yes"])

if test "$enable_gpm" = "yes" -o "$enable_gpm" = "dynamic"; then
  AC_MSG_RESULT($enable_gpm)
  dnl Checking if gpm support can be compiled
  AC_CACHE_CHECK([for gpm], vi_cv_have_gpm,
	[olibs="$LIBS" ; LIBS="-lgpm"]
	AC_LINK_IFELSE([AC_LANG_PROGRAM(
	    [#include <gpm.h>
	    #include <linux/keyboard.h>],
	    [Gpm_GetLibVersion(NULL);])],
	    dnl Configure defines HAVE_GPM, if it is defined feature.h defines
	    dnl FEAT_MOUSE_GPM if mouse support is included
	    [vi_cv_have_gpm=yes],
	    [vi_cv_have_gpm=no])
	[LIBS="$olibs"]
    )
  if test $vi_cv_have_gpm = yes; then
    if test "$enable_gpm" = "yes"; then
      LIBS="$LIBS -lgpm"
    else
      AC_DEFINE(DYNAMIC_GPM)
    fi
    AC_DEFINE(HAVE_GPM)
  fi
else
  AC_MSG_RESULT(no)
fi

AC_MSG_CHECKING(--disable-sysmouse argument)
AC_ARG_ENABLE(sysmouse,
	[  --disable-sysmouse      Don't use sysmouse (mouse in *BSD console).], ,
	[enable_sysmouse="yes"])

if test "$enable_sysmouse" = "yes"; then
  AC_MSG_RESULT(no)
  dnl Checking if sysmouse support can be compiled
  dnl Configure defines HAVE_SYSMOUSE, if it is defined feature.h
  dnl defines FEAT_SYSMOUSE if mouse support is included
  AC_CACHE_CHECK([for sysmouse], vi_cv_have_sysmouse,
	AC_LINK_IFELSE([AC_LANG_PROGRAM(
	    [#include <sys/consio.h>
	     #include <signal.h>
	     #include <sys/fbio.h>],
	    [struct mouse_info   mouse;
	     mouse.operation = MOUSE_MODE;
	     mouse.operation = MOUSE_SHOW;
	     mouse.u.mode.mode = 0;
	     mouse.u.mode.signal = SIGUSR2;])],
	    [vi_cv_have_sysmouse=yes],
	    [vi_cv_have_sysmouse=no])
    )
  if test $vi_cv_have_sysmouse = yes; then
    AC_DEFINE(HAVE_SYSMOUSE)
  fi
else
  AC_MSG_RESULT(yes)
fi

dnl make sure the FD_CLOEXEC flag for fcntl()'s F_SETFD command is known
AC_MSG_CHECKING(for FD_CLOEXEC)
AC_COMPILE_IFELSE([AC_LANG_PROGRAM(
[#if HAVE_FCNTL_H
# include <fcntl.h>
#endif],
[	int flag = FD_CLOEXEC;])],
	AC_MSG_RESULT(yes); AC_DEFINE(HAVE_FD_CLOEXEC),
	AC_MSG_RESULT(not usable))

dnl rename needs to be checked separately to work on Nextstep with cc
AC_MSG_CHECKING(for rename)
AC_LINK_IFELSE([AC_LANG_PROGRAM([#include <stdio.h>], [rename("this", "that")])],
	AC_MSG_RESULT(yes); AC_DEFINE(HAVE_RENAME),
	AC_MSG_RESULT(no))

dnl check for dirfd()
AC_MSG_CHECKING(for dirfd)
AC_LINK_IFELSE([AC_LANG_PROGRAM(
[#include <sys/types.h>
#include <dirent.h>],
[DIR * dir=opendir("dirname"); dirfd(dir);])],
AC_MSG_RESULT(yes); AC_DEFINE(HAVE_DIRFD), AC_MSG_RESULT(not usable))

dnl check for flock()
AC_MSG_CHECKING(for flock)
AC_LINK_IFELSE([AC_LANG_PROGRAM(
[#include <sys/file.h>],
[flock(10, LOCK_SH);])],
AC_MSG_RESULT(yes); AC_DEFINE(HAVE_FLOCK), AC_MSG_RESULT(not usable))

dnl sysctl() may exist but not the arguments we use
AC_MSG_CHECKING(for sysctl)
AC_COMPILE_IFELSE([AC_LANG_PROGRAM(
[#include <sys/types.h>
#include <sys/sysctl.h>],
[[	int mib[2], r;
	size_t len;

	mib[0] = CTL_HW;
	mib[1] = HW_USERMEM;
	len = sizeof(r);
	(void)sysctl(mib, 2, &r, &len, (void *)0, (size_t)0);
	]])],
	AC_MSG_RESULT(yes); AC_DEFINE(HAVE_SYSCTL),
	AC_MSG_RESULT(not usable))

dnl sysinfo() may exist but not be Linux compatible.
dnl On some FreeBSD systems it may depend on libsysinfo, try to link
AC_MSG_CHECKING(for sysinfo)
AC_LINK_IFELSE([AC_LANG_PROGRAM(
[#include <sys/types.h>
#include <sys/sysinfo.h>],
[	struct sysinfo sinfo;
	int t;

	(void)sysinfo(&sinfo);
	t = sinfo.totalram;
	])],
	AC_MSG_RESULT(yes); AC_DEFINE(HAVE_SYSINFO),
	AC_MSG_RESULT(not usable))

dnl struct sysinfo may have the mem_unit field or not
AC_MSG_CHECKING(for sysinfo.mem_unit)
AC_COMPILE_IFELSE([AC_LANG_PROGRAM(
[#include <sys/types.h>
#include <sys/sysinfo.h>],
[	struct sysinfo sinfo;
	sinfo.mem_unit = 1;
	])],
	AC_MSG_RESULT(yes); AC_DEFINE(HAVE_SYSINFO_MEM_UNIT),
	AC_MSG_RESULT(no))

dnl struct sysinfo may have the uptime field or not
AC_MSG_CHECKING(for sysinfo.uptime)
AC_COMPILE_IFELSE([AC_LANG_PROGRAM(
[#include <sys/types.h>
#include <sys/sysinfo.h>],
[	struct sysinfo sinfo;
        long ut;

	(void)sysinfo(&sinfo);
	ut = sinfo.uptime;
	])],
	AC_MSG_RESULT(yes); AC_DEFINE(HAVE_SYSINFO_UPTIME),
	AC_MSG_RESULT(no))

dnl sysconf() may exist but not support what we want to use
AC_MSG_CHECKING(for sysconf)
AC_COMPILE_IFELSE([AC_LANG_PROGRAM(
[#include <unistd.h>],
[	(void)sysconf(_SC_PAGESIZE);
	(void)sysconf(_SC_PHYS_PAGES);
	])],
	AC_MSG_RESULT(yes); AC_DEFINE(HAVE_SYSCONF),
	AC_MSG_RESULT(not usable))

dnl check if we have _SC_SIGSTKSZ via sysconf()
AC_MSG_CHECKING(for _SC_SIGSTKSZ via sysconf())
AC_COMPILE_IFELSE([AC_LANG_PROGRAM(
[#include <unistd.h>],
[	(void)sysconf(_SC_SIGSTKSZ);
	])],
	AC_MSG_RESULT(yes); AC_DEFINE(HAVE_SYSCONF_SIGSTKSZ),
	AC_MSG_RESULT(not usable))

AC_CHECK_SIZEOF([int])
AC_CHECK_SIZEOF([long])
AC_CHECK_SIZEOF([time_t])
AC_CHECK_SIZEOF([off_t])

dnl Use different names to avoid clashing with other header files.
AC_DEFINE_UNQUOTED(VIM_SIZEOF_INT, [$ac_cv_sizeof_int])
AC_DEFINE_UNQUOTED(VIM_SIZEOF_LONG, [$ac_cv_sizeof_long])

dnl Make sure that uint32_t is really 32 bits unsigned.
AC_MSG_CHECKING([uint32_t is 32 bits])
AC_RUN_IFELSE([AC_LANG_SOURCE([
#ifdef HAVE_STDINT_H
# include <stdint.h>
#endif
#ifdef HAVE_INTTYPES_H
# include <inttypes.h>
#endif
int main() {
  uint32_t nr1 = (uint32_t)-1;
  uint32_t nr2 = (uint32_t)0xffffffffUL;
  if (sizeof(uint32_t) != 4 || nr1 != 0xffffffffUL || nr2 + 1 != 0) return 1;
  return 0;
}])],
AC_MSG_RESULT(ok),
AC_MSG_ERROR([WRONG!  uint32_t not defined correctly.]),
AC_MSG_WARN([cannot check uint32_t when cross-compiling.]))

dnl Check for memmove() before bcopy(), makes memmove() be used when both are
dnl present, fixes problem with incompatibility between Solaris 2.4 and 2.5.

[bcopy_test_prog='
#include "confdefs.h"
#ifdef HAVE_STRING_H
# include <string.h>
#endif
#if STDC_HEADERS
# include <stdlib.h>
# include <stddef.h>
#endif
int main() {
  char buf[10];
  strcpy(buf, "abcdefghi");
  mch_memmove(buf, buf + 2, 3);
  if (strncmp(buf, "ababcf", 6))
    exit(1);
  strcpy(buf, "abcdefghi");
  mch_memmove(buf + 2, buf, 3);
  if (strncmp(buf, "cdedef", 6))
    exit(1);
  exit(0); /* libc version works properly.  */
}']

AC_CACHE_CHECK([whether memmove handles overlaps],[vim_cv_memmove_handles_overlap],
  [
    AC_RUN_IFELSE([AC_LANG_SOURCE([[#define mch_memmove(s,d,l) memmove(d,s,l) $bcopy_test_prog]])],
      [
	vim_cv_memmove_handles_overlap=yes
      ],[
	vim_cv_memmove_handles_overlap=no
      ],[
	AC_MSG_ERROR(cross-compiling: please set 'vim_cv_memmove_handles_overlap')
      ])
  ])

if test "x$vim_cv_memmove_handles_overlap" = "xyes" ; then
  AC_DEFINE(USEMEMMOVE)
else
  AC_CACHE_CHECK([whether bcopy handles overlaps],[vim_cv_bcopy_handles_overlap],
    [
      AC_RUN_IFELSE([AC_LANG_SOURCE([[#define mch_bcopy(s,d,l) bcopy(d,s,l) $bcopy_test_prog]])],
      [
	vim_cv_bcopy_handles_overlap=yes
      ],[
	vim_cv_bcopy_handles_overlap=no
      ],[
	AC_MSG_ERROR(cross-compiling: please set 'vim_cv_bcopy_handles_overlap')
      ])
    ])

  if test "x$vim_cv_bcopy_handles_overlap" = "xyes" ; then
    AC_DEFINE(USEBCOPY)
  else
    AC_CACHE_CHECK([whether memcpy handles overlaps],[vim_cv_memcpy_handles_overlap],
      [
	AC_RUN_IFELSE([AC_LANG_SOURCE([[#define mch_memcpy(s,d,l) memcpy(d,s,l) $bcopy_test_prog]])],
	  [
	    vim_cv_memcpy_handles_overlap=yes
	  ],[
	    vim_cv_memcpy_handles_overlap=no
	  ],[
	    AC_MSG_ERROR(cross-compiling: please set 'vim_cv_memcpy_handles_overlap')
	  ])
      ])

    if test "x$vim_cv_memcpy_handles_overlap" = "xyes" ; then
      AC_DEFINE(USEMEMCPY)
    fi
  fi
fi


dnl Check for multibyte locale functions
dnl Find out if _Xsetlocale() is supported by libX11.
dnl Check if X_LOCALE should be defined.
if test "x$with_x" = "xyes"; then
  cflags_save=$CFLAGS
  libs_save=$LIBS
  LIBS="$LIBS $X_LIBS $GUI_LIB_LOC $GUI_X_LIBS $X_PRE_LIBS $X_LIB $X_EXTRA_LIBS"
  CFLAGS="$CFLAGS $X_CFLAGS"

  AC_MSG_CHECKING(whether X_LOCALE needed)
  AC_COMPILE_IFELSE([AC_LANG_PROGRAM([#include <X11/Xlocale.h>],)],
      AC_LINK_IFELSE([AC_LANG_CALL([],[_Xsetlocale])], [AC_MSG_RESULT(yes)
	      AC_DEFINE(X_LOCALE)], AC_MSG_RESULT(no)),
      AC_MSG_RESULT(no))

  AC_MSG_CHECKING(whether Xutf8SetWMProperties() can be used)
  AC_LINK_IFELSE([AC_LANG_CALL([],[Xutf8SetWMProperties])], [AC_MSG_RESULT(yes)
	      AC_DEFINE(HAVE_XUTF8SETWMPROPERTIES)], AC_MSG_RESULT(no))

  CFLAGS=$cflags_save
  LIBS=$libs_save
fi

dnl Link with xpg4, it is said to make Korean locale working
AC_CHECK_LIB(xpg4, _xpg4_setrunelocale, [LIBS="$LIBS -lxpg4"],,)

dnl Check how we can run ctags.  Default to "ctags" when nothing works.
dnl Use --version to detect Exuberant ctags (preferred)
dnl       Add --fields=+S to get function signatures for omni completion.
dnl -t for typedefs (many ctags have this)
dnl -s for static functions (Elvis ctags only?)
dnl -v for variables. Dangerous, most ctags take this for 'vgrind style'.
dnl -i+m to test for older Exuberant ctags
AC_MSG_CHECKING(how to create tags)
test -f tags && mv tags tags.save
if (eval ctags --version /dev/null | grep Exuberant) < /dev/null 1>&AS_MESSAGE_LOG_FD 2>&1; then
  TAGPRG="ctags -I INIT+,INIT2+,INIT3+,INIT4+,INIT5+ --fields=+S"
elif (eval exctags --version /dev/null | grep Exuberant) < /dev/null 1>&AS_MESSAGE_LOG_FD 2>&1; then
  TAGPRG="exctags -I INIT+,INIT2+,INIT3+,INIT4+,INIT5+ --fields=+S"
elif (eval exuberant-ctags --version /dev/null | grep Exuberant) < /dev/null 1>&AS_MESSAGE_LOG_FD 2>&1; then
  TAGPRG="exuberant-ctags -I INIT+,INIT2+,INIT3+,INIT4+,INIT5+ --fields=+S"
else
  TAGPRG="ctags"
  (eval etags	   /dev/null) < /dev/null 1>&AS_MESSAGE_LOG_FD 2>&1 && TAGPRG="etags"
  (eval etags -c   /dev/null) < /dev/null 1>&AS_MESSAGE_LOG_FD 2>&1 && TAGPRG="etags -c"
  (eval ctags	   /dev/null) < /dev/null 1>&AS_MESSAGE_LOG_FD 2>&1 && TAGPRG="ctags"
  (eval ctags -t   /dev/null) < /dev/null 1>&AS_MESSAGE_LOG_FD 2>&1 && TAGPRG="ctags -t"
  (eval ctags -ts  /dev/null) < /dev/null 1>&AS_MESSAGE_LOG_FD 2>&1 && TAGPRG="ctags -ts"
  (eval ctags -tvs /dev/null) < /dev/null 1>&AS_MESSAGE_LOG_FD 2>&1 && TAGPRG="ctags -tvs"
  (eval ctags -i+m /dev/null) < /dev/null 1>&AS_MESSAGE_LOG_FD 2>&1 && TAGPRG="ctags -i+m"
fi
test -f tags.save && mv tags.save tags
AC_MSG_RESULT($TAGPRG) AC_SUBST(TAGPRG)

dnl Check how we can run man with a section number
AC_MSG_CHECKING(how to run man with a section nr)
MANDEF="man"
(eval MANPAGER=cat PAGER=cat man -s 2 read) < /dev/null > /dev/null 2>&AS_MESSAGE_LOG_FD && MANDEF="man -s"
AC_MSG_RESULT($MANDEF)
if test "$MANDEF" = "man -s"; then
  AC_DEFINE(USEMAN_S)
fi

dnl Check if gettext() is working and if it needs -lintl
dnl We take care to base this on an empty LIBS: on some systems libelf would be
dnl in LIBS and implicitly take along libintl. The final LIBS would then not
dnl contain libintl, and the link step would fail due to -Wl,--as-needed.
AC_MSG_CHECKING(--disable-nls argument)
AC_ARG_ENABLE(nls,
	[  --disable-nls           Don't support NLS (gettext()).], ,
	[enable_nls="yes"])

if test "$enable_nls" = "yes"; then
  AC_MSG_RESULT(no)

  INSTALL_LANGS=install-languages
  AC_SUBST(INSTALL_LANGS)
  INSTALL_TOOL_LANGS=install-tool-languages
  AC_SUBST(INSTALL_TOOL_LANGS)

  AC_CHECK_PROG(MSGFMT, msgfmt, msgfmt, )
  AC_MSG_CHECKING([for NLS])
  if test -f po/Makefile; then
    have_gettext="no"
    if test -n "$MSGFMT"; then
      olibs=$LIBS
      LIBS=""
      AC_LINK_IFELSE([AC_LANG_PROGRAM(
	[#include <libintl.h>],
	[gettext("Test");])],
	AC_MSG_RESULT([gettext() works]); have_gettext="yes"; LIBS=$olibs,
	  LIBS="-lintl"
	  AC_LINK_IFELSE([AC_LANG_PROGRAM(
	      [#include <libintl.h>],
	      [gettext("Test");])],
	      AC_MSG_RESULT([gettext() works with -lintl]); have_gettext="yes";
	      LIBS="$olibs -lintl",
	      AC_MSG_RESULT([gettext() doesn't work]);
	      LIBS=$olibs))
    else
      AC_MSG_RESULT([msgfmt not found - disabled]);
    fi
    if test $have_gettext = "yes" -a "x$features" != "xtiny"; then
      AC_DEFINE(HAVE_GETTEXT)
      MAKEMO=yes
      AC_SUBST(MAKEMO)
      dnl this was added in GNU gettext 0.10.36
      AC_CHECK_FUNCS(bind_textdomain_codeset)
      dnl _nl_msg_cat_cntr is required for GNU gettext
      AC_MSG_CHECKING([for _nl_msg_cat_cntr])
      AC_LINK_IFELSE([AC_LANG_PROGRAM(
		[#include <libintl.h>
		extern int _nl_msg_cat_cntr;],
		[++_nl_msg_cat_cntr;])],
		AC_MSG_RESULT([yes]); AC_DEFINE(HAVE_NL_MSG_CAT_CNTR),
		AC_MSG_RESULT([no]))
      AC_MSG_CHECKING([if msgfmt supports --desktop])
      MSGFMT_DESKTOP=
      if "$MSGFMT" --help | grep -e '--desktop' >/dev/null; then
	if "$MSGFMT" --version | grep '0.19.[[3-7]]$' >/dev/null; then
	  dnl GNU gettext 0.19.7's --desktop is broken.  We assume back to
	  dnl 0.19.3 is also broken.
	  AC_MSG_RESULT([broken])
	else
	  AC_MSG_RESULT([yes])
	  MSGFMT_DESKTOP="gvim.desktop vim.desktop"
	fi
      else
	AC_MSG_RESULT([no])
      fi
      AC_SUBST(MSGFMT_DESKTOP)
    fi
  else
    AC_MSG_RESULT([no "po/Makefile" - disabled]);
  fi
else
  AC_MSG_RESULT(yes)
fi

dnl Check for dynamic linking loader
AC_CHECK_HEADER(dlfcn.h, DLL=dlfcn.h, [AC_CHECK_HEADER(dl.h, DLL=dl.h)])
if test x${DLL} = xdlfcn.h; then
  AC_DEFINE(HAVE_DLFCN_H, 1, [ Define if we have dlfcn.h. ])
  AC_MSG_CHECKING([for dlopen()])
  AC_LINK_IFELSE([AC_LANG_PROGRAM(,[
		extern void* dlopen();
		dlopen();
      ])],
      AC_MSG_RESULT(yes);
	      AC_DEFINE(HAVE_DLOPEN, 1, [ Define if we have dlopen() ]),
      AC_MSG_RESULT(no);
	      AC_MSG_CHECKING([for dlopen() in -ldl])
	      olibs=$LIBS
	      LIBS="$LIBS -ldl"
	      AC_LINK_IFELSE([AC_LANG_PROGRAM(,[
				extern void* dlopen();
				dlopen();
		 ])],
		 AC_MSG_RESULT(yes);
			  AC_DEFINE(HAVE_DLOPEN, 1, [ Define if we have dlopen() ]),
		 AC_MSG_RESULT(no);
			  LIBS=$olibs))
  dnl ReliantUNIX has dlopen() in libc but everything else in libdl
  dnl ick :-)
  AC_MSG_CHECKING([for dlsym()])
  AC_LINK_IFELSE([AC_LANG_PROGRAM(,[
		extern void* dlsym();
		dlsym();
      ])],
      AC_MSG_RESULT(yes);
	      AC_DEFINE(HAVE_DLSYM, 1, [ Define if we have dlsym() ]),
      AC_MSG_RESULT(no);
	      AC_MSG_CHECKING([for dlsym() in -ldl])
	      olibs=$LIBS
	      LIBS="$LIBS -ldl"
	      AC_LINK_IFELSE([AC_LANG_PROGRAM(,[
				extern void* dlsym();
				dlsym();
		 ])],
		 AC_MSG_RESULT(yes);
			  AC_DEFINE(HAVE_DLSYM, 1, [ Define if we have dlsym() ]),
		 AC_MSG_RESULT(no);
			  LIBS=$olibs))
elif test x${DLL} = xdl.h; then
  AC_DEFINE(HAVE_DL_H, 1, [ Define if we have dl.h. ])
  AC_MSG_CHECKING([for shl_load()])
  AC_LINK_IFELSE([AC_LANG_PROGRAM(,[
		extern void* shl_load();
		shl_load();
     ])],
     AC_MSG_RESULT(yes);
	  AC_DEFINE(HAVE_SHL_LOAD, 1, [ Define if we have shl_load() ]),
     AC_MSG_RESULT(no);
	  AC_MSG_CHECKING([for shl_load() in -ldld])
	  olibs=$LIBS
	  LIBS="$LIBS -ldld"
	  AC_LINK_IFELSE([AC_LANG_PROGRAM(,[
			extern void* shl_load();
			shl_load();
	     ])],
	     AC_MSG_RESULT(yes);
		  AC_DEFINE(HAVE_SHL_LOAD, 1, [ Define if we have shl_load() ]),
	     AC_MSG_RESULT(no);
		  LIBS=$olibs))
fi
AC_CHECK_HEADERS(setjmp.h)

if test "x$MACOS_X" = "xyes" -a -n "$PERL"; then
  dnl -ldl must come after DynaLoader.a
  if echo $LIBS | grep -e '-ldl' >/dev/null; then
    LIBS=`echo $LIBS | sed s/-ldl//`
    PERL_LIBS="$PERL_LIBS -ldl"
  fi
fi

if test "$MACOS_X" = "yes"; then
  AC_MSG_CHECKING([whether we need macOS frameworks])
  if test "$MACOS_X_DARWIN" = "yes"; then
    if test "$features" = "tiny"; then
      dnl Since no FEAT_CLIPBOARD or FEAT_SOUND, no need for os_macosx.m.
      OS_EXTRA_SRC=`echo "$OS_EXTRA_SRC" | sed -e 's+os_macosx.m++'`
      OS_EXTRA_OBJ=`echo "$OS_EXTRA_OBJ" | sed -e 's+objects/os_macosx.o++'`
      AC_MSG_RESULT([yes, we need CoreServices])
      LIBS="$LIBS -framework CoreServices"
    else
      AC_MSG_RESULT([yes, we need AppKit])
      LIBS="$LIBS -framework AppKit"
    fi
  else
    AC_MSG_RESULT([no])
  fi
fi

dnl On some systems REENTRANT needs to be defined.  It should not hurt to use
dnl it everywhere.
if `echo "$CFLAGS" | grep -v D_REENTRANT >/dev/null`; then
  CFLAGS="$CFLAGS -D_REENTRANT"
fi

dnl gcc 3.1 changed the meaning of -MM.  The only solution appears to be to
dnl use "-isystem" instead of "-I" for all non-Vim include dirs.
dnl But only when making dependencies, cproto and lint don't take "-isystem".
dnl Mac gcc returns "powerpc-apple-darwin8-gcc-4.0.1 (GCC)...", need to allow
dnl the number before the version number.
DEPEND_CFLAGS_FILTER=
if test "$GCC" = yes; then
  AC_MSG_CHECKING(for GCC 3 or later)
  gccmajor=`echo "$gccversion" | sed -e 's/^\([[1-9]][[0-9]]*\)\..*$/\1/g'`
  if test "$gccmajor" -gt "2"; then
    DEPEND_CFLAGS_FILTER="| sed 's+-I */+-isystem /+g'"
    AC_MSG_RESULT(yes)
  else
    AC_MSG_RESULT(no)
  fi
  dnl -D_FORTIFY_SOURCE=2 crashes Vim on strcpy(buf, "000") when buf is
  dnl declared as char x[1] but actually longer.  Introduced in gcc 4.0.
  dnl Also remove duplicate _FORTIFY_SOURCE arguments.
  dnl And undefine it first to avoid a warning.
  AC_MSG_CHECKING(whether we need -D_FORTIFY_SOURCE=1)
  if test "$gccmajor" -gt "3"; then
    CFLAGS=`echo "$CFLAGS" | sed -e 's/-D_FORTIFY_SOURCE=.,//g' -e 's/ *-Wp,-D_FORTIFY_SOURCE=. / /g' -e 's/,-D_FORTIFY_SOURCE=. //g' -e 's/ *-D_FORTIFY_SOURCE=.//g' -e 's/ *-Wp,-U_FORTIFY_SOURCE/ /g' -e 's/ *-U_FORTIFY_SOURCE//g' -e 's/$/ -U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=1/'`
    CPPFLAGS=`echo "$CPPFLAGS" | sed -e 's/-D_FORTIFY_SOURCE=.,//g' -e 's/ *-Wp,-D_FORTIFY_SOURCE=. / /g' -e 's/,-D_FORTIFY_SOURCE=. //g' -e 's/ *-D_FORTIFY_SOURCE=.//g' -e 's/ *-Wp,-U_FORTIFY_SOURCE/ /g' -e 's/ *-U_FORTIFY_SOURCE//g'`
    AC_MSG_RESULT(yes)
  else
    AC_MSG_RESULT(no)
  fi
fi
AC_SUBST(DEPEND_CFLAGS_FILTER)

dnl On some systems AC_SYS_LARGEFILE determines that -D_FILE_OFFSET_BITS=64
dnl isn't required, but the CFLAGS for some of the libraries we're using
dnl include the define.  Since the define changes the size of some datatypes
dnl (e.g. ino_t and off_t), all of Vim's modules must be compiled with a
dnl consistent value.  It's therefore safest to force the use of the define
dnl if it's present in any of the *_CFLAGS variables.
AC_MSG_CHECKING(whether we need to force -D_FILE_OFFSET_BITS=64)
if echo "$CFLAGS $LUA_CFLAGS $MZSCHEME_CFLAGS $PERL_CFLAGS $PYTHON_CFLAGS $PYTHON3_CFLAGS $TCL_CFLAGS $RUBY_CFLAGS $GTK_CFLAGS" | grep -q D_FILE_OFFSET_BITS 2>/dev/null; then
  AC_MSG_RESULT(yes)
  AC_DEFINE(_FILE_OFFSET_BITS, 64)
else
  AC_MSG_RESULT(no)
fi

dnl $LDFLAGS is passed to glibtool in libvterm, it doesn't like a space
dnl between "-L" and the path that follows.
LDFLAGS=`echo "$LDFLAGS" | sed -e 's/-L /-L/g'`

dnl link.sh tries to avoid overlinking in a hackish way.
dnl At least GNU ld supports --as-needed which provides the same functionality
dnl at linker level. Let's use it.
AC_MSG_CHECKING(linker --as-needed support)
LINK_AS_NEEDED=
# Check if linker supports --as-needed and --no-as-needed options
if $CC -Wl,--help 2>/dev/null | grep as-needed > /dev/null; then
  if ! echo "$LDFLAGS" | grep -q -- '-Wl,[[^[:space:]]]*--as-needed'; then
    LDFLAGS="$LDFLAGS -Wl,--as-needed"
  fi
  LINK_AS_NEEDED=yes
fi
if test "$LINK_AS_NEEDED" = yes; then
  AC_MSG_RESULT(yes)
else
  AC_MSG_RESULT(no)
fi
AC_SUBST(LINK_AS_NEEDED)

# IBM z/OS reset CFLAGS for config.mk
if test "$zOSUnix" = "yes"; then
	CFLAGS="-D_ALL_SOURCE -Wc,float\(ieee\),dll"
fi

dnl write output files
AC_CONFIG_FILES(auto/config.mk:config.mk.in)
AC_OUTPUT

dnl vim: set sw=2 tw=78 fo+=l:
