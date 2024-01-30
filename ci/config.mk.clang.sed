/^CFLAGS[[:blank:]]*=/s/$/ -Wno-error=missing-field-initializers/
/^RUBY_CFLAGS_EXTRA[[:blank:]]*=/s/$/ -Wno-error=unknown-attributes -Wno-error=ignored-attributes/
