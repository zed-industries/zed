[
 "("
 ")"
 "{"
 "}"
] @punctuation.bracket

[
 ":"
 "&:"
 "::"
 "|"
 ";"
 "\""
 "'"
 ","
] @punctuation.delimiter

[
 "$"
 "$$"
] @punctuation.special

(automatic_variable
 [ "@" "%" "<" "?" "^" "+" "/" "*" "D" "F"] @punctuation.special)

(automatic_variable
 "/" @error . ["D" "F"])

[
 "="
 ":="
 "::="
 "?="
 "+="
 "!="
 "@"
 "-"
 "+"
] @operator

[
 (text)
 (string)
 (raw_text)
] @string

(variable_assignment (word) @string)

[
 "ifeq"
 "ifneq"
 "ifdef"
 "ifndef"
 "else"
 "endif"
 "if"
 "or"  ; boolean functions are conditional in make grammar
 "and"
] @conditional

"foreach" @repeat

[
 "define"
 "endef"
 "vpath"
 "undefine"
 "export"
 "unexport"
 "override"
 "private"
; "load"
] @keyword

[
 "include"
 "sinclude"
 "-include"
] @include

[
 "subst"
 "patsubst"
 "strip"
 "findstring"
 "filter"
 "filter-out"
 "sort"
 "word"
 "words"
 "wordlist"
 "firstword"
 "lastword"
 "dir"
 "notdir"
 "suffix"
 "basename"
 "addsuffix"
 "addprefix"
 "join"
 "wildcard"
 "realpath"
 "abspath"
 "call"
 "eval"
 "file"
 "value"
 "shell"
] @keyword.function

[
 "error"
 "warning"
 "info"
] @exception

;; Variable
(variable_assignment
  name: (word) @constant)

(variable_reference
  (word) @constant)

(comment) @comment

((word) @clean @string.regex
 (#match? @clean "[%\*\?]"))

(function_call
  function: "error"
  (arguments (text) @text.danger))

(function_call
  function: "warning"
  (arguments (text) @text.warning))

(function_call
  function: "info"
  (arguments (text) @text.note))

;; Install Command Categories
;; Others special variables
;; Variables Used by Implicit Rules
[
 "VPATH"
 ".RECIPEPREFIX"
] @constant.builtin

(variable_assignment
  name: (word) @clean @constant.builtin
        (#match? @clean "^(AR|AS|CC|CXX|CPP|FC|M2C|PC|CO|GET|LEX|YACC|LINT|MAKEINFO|TEX|TEXI2DVI|WEAVE|CWEAVE|TANGLE|CTANGLE|RM|ARFLAGS|ASFLAGS|CFLAGS|CXXFLAGS|COFLAGS|CPPFLAGS|FFLAGS|GFLAGS|LDFLAGS|LDLIBS|LFLAGS|YFLAGS|PFLAGS|RFLAGS|LINTFLAGS|PRE_INSTALL|POST_INSTALL|NORMAL_INSTALL|PRE_UNINSTALL|POST_UNINSTALL|NORMAL_UNINSTALL|MAKEFILE_LIST|MAKE_RESTARTS|MAKE_TERMOUT|MAKE_TERMERR|\.DEFAULT_GOAL|\.RECIPEPREFIX|\.EXTRA_PREREQS)$"))

(variable_reference
  (word) @clean @constant.builtin
  (#match? @clean "^(AR|AS|CC|CXX|CPP|FC|M2C|PC|CO|GET|LEX|YACC|LINT|MAKEINFO|TEX|TEXI2DVI|WEAVE|CWEAVE|TANGLE|CTANGLE|RM|ARFLAGS|ASFLAGS|CFLAGS|CXXFLAGS|COFLAGS|CPPFLAGS|FFLAGS|GFLAGS|LDFLAGS|LDLIBS|LFLAGS|YFLAGS|PFLAGS|RFLAGS|LINTFLAGS|PRE_INSTALL|POST_INSTALL|NORMAL_INSTALL|PRE_UNINSTALL|POST_UNINSTALL|NORMAL_UNINSTALL|MAKEFILE_LIST|MAKE_RESTARTS|MAKE_TERMOUT|MAKE_TERMERR|\.DEFAULT_GOAL|\.RECIPEPREFIX|\.EXTRA_PREREQS\.VARIABLES|\.FEATURES|\.INCLUDE_DIRS|\.LOADED)$"))

;; Standart targets
(targets
  (word) @constant.macro
  (#match? @constant.macro "^(all|install|install-html|install-dvi|install-pdf|install-ps|uninstall|install-strip|clean|distclean|mostlyclean|maintainer-clean|TAGS|info|dvi|html|pdf|ps|dist|check|installcheck|installdirs)$"))

(targets
  (word) @constant.macro
  (#match? @constant.macro "^(all|install|install-html|install-dvi|install-pdf|install-ps|uninstall|install-strip|clean|distclean|mostlyclean|maintainer-clean|TAGS|info|dvi|html|pdf|ps|dist|check|installcheck|installdirs)$"))

;; Builtin targets
(targets
  (word) @constant.macro
  (#match? @constant.macro "^\.(PHONY|SUFFIXES|DEFAULT|PRECIOUS|INTERMEDIATE|SECONDARY|SECONDEXPANSION|DELETE_ON_ERROR|IGNORE|LOW_RESOLUTION_TIME|SILENT|EXPORT_ALL_VARIABLES|NOTPARALLEL|ONESHELL|POSIX)$"))

