# Clang 12 (or Apple clang 13) and later makes a warning '-Wcompound-token-split-by-macro' enable by default.
/^PERL_CFLAGS_EXTRA[[:blank:]]*=/s/$/ -Wno-error=compound-token-split-by-macro -Wno-compound-token-split-by-macro/
/^RUBY_CFLAGS_EXTRA[[:blank:]]*=/s/$/ -Wno-error=compound-token-split-by-macro/
