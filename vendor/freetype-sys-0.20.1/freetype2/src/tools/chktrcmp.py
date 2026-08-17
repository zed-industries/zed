#!/usr/bin/env python3
#
# Check trace components in FreeType 2 source.
# Author: suzuki toshiya, 2009, 2013, 2020
#
# This code is explicitly into the public domain.

import sys
import os
import re

SRC_FILE_LIST = []
USED_COMPONENT = {}
KNOWN_COMPONENT = {}

SRC_FILE_DIRS = ["src"]
TRACE_DEF_FILES = ["include/freetype/internal/fttrace.h"]


def usage():
    print("Usage: %s [option]" % sys.argv[0])
    print("Search used-but-defined and defined-but-not-used trace_XXX macros")
    print("")
    print("  --help:")
    print("        Show this help")
    print("")
    print("  --src-dirs=dir1:dir2:...")
    print("        Specify the directories of C source files to be checked")
    print("        Default is %s" % ":".join(SRC_FILE_DIRS))
    print("")
    print("  --def-files=file1:file2:...")
    print("        Specify the header files including FT_TRACE_DEF()")
    print("        Default is %s" % ":".join(TRACE_DEF_FILES))
    print("")


# --------------------------------------------------------------
# Parse command line options
#
for i in range(1, len(sys.argv)):
    if sys.argv[i].startswith("--help"):
        usage()
        exit(0)
    if sys.argv[i].startswith("--src-dirs="):
        SRC_FILE_DIRS = sys.argv[i].replace("--src-dirs=", "", 1).split(":")
    elif sys.argv[i].startswith("--def-files="):
        TRACE_DEF_FILES = sys.argv[i].replace("--def-files=", "", 1).split(":")

# --------------------------------------------------------------
# Scan C source and header files using trace macros.
#

c_pathname_pat = re.compile('^.*\.[ch]$', re.IGNORECASE)
trace_use_pat = re.compile('^[ \t]*#define[ \t]+FT_COMPONENT[ \t]+')

for d in SRC_FILE_DIRS:
    for (p, dlst, flst) in os.walk(d):
        for f in flst:
            if c_pathname_pat.match(f) is not None:
                src_pathname = os.path.join(p, f)

                line_num = 0
                for src_line in open(src_pathname, 'r'):
                    line_num = line_num + 1
                    src_line = src_line.strip()
                    if trace_use_pat.match(src_line) is not None:
                        component_name = trace_use_pat.sub('', src_line)
                        if component_name in USED_COMPONENT:
                            USED_COMPONENT[component_name]\
                                .append("%s:%d" % (src_pathname, line_num))
                        else:
                            USED_COMPONENT[component_name] =\
                                ["%s:%d" % (src_pathname, line_num)]

# --------------------------------------------------------------
# Scan header file(s) defining trace macros.
#

trace_def_pat_opn = re.compile('^.*FT_TRACE_DEF[ \t]*\([ \t]*')
trace_def_pat_cls = re.compile('[ \t\)].*$')

for f in TRACE_DEF_FILES:
    line_num = 0
    for hdr_line in open(f, 'r'):
        line_num = line_num + 1
        hdr_line = hdr_line.strip()
        if trace_def_pat_opn.match(hdr_line) is not None:
            component_name = trace_def_pat_opn.sub('', hdr_line)
            component_name = trace_def_pat_cls.sub('', component_name)
            if component_name in KNOWN_COMPONENT:
                print("trace component %s is defined twice,"
                      " see %s and fttrace.h:%d" %
                      (component_name, KNOWN_COMPONENT[component_name],
                       line_num))
            else:
                KNOWN_COMPONENT[component_name] =\
                    "%s:%d" % (os.path.basename(f), line_num)

# --------------------------------------------------------------
# Compare the used and defined trace macros.
#

print("# Trace component used in the implementations but not defined in "
      "fttrace.h.")
cmpnt = list(USED_COMPONENT.keys())
cmpnt.sort()
for c in cmpnt:
    if c not in KNOWN_COMPONENT:
        print("Trace component %s (used in %s) is not defined." %
              (c, ", ".join(USED_COMPONENT[c])))

print("# Trace component is defined but not used in the implementations.")
cmpnt = list(KNOWN_COMPONENT.keys())
cmpnt.sort()
for c in cmpnt:
    if c not in USED_COMPONENT:
        if c != "any":
            print("Trace component %s (defined in %s) is not used." %
                  (c, KNOWN_COMPONENT[c]))
