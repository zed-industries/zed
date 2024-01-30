/^CFLAGS[[:blank:]]*=/s/$/ -Wall -Wextra -Wshadow -Wstrict-prototypes -Wmissing-prototypes -Werror -Wno-deprecated-declarations/
/^PERL_CFLAGS_EXTRA[[:blank:]]*=/s/$/ -Wno-error=unused-function -Wno-shadow/
/^RUBY_CFLAGS_EXTRA[[:blank:]]*=/s/$/ -Wno-error=unused-parameter -Wno-strict-prototypes/
