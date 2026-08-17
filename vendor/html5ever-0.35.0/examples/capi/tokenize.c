// Copyright 2014-2017 The html5ever Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#include <stdio.h>

#include "html5ever.h"

void put_str(const char *x) {
    fputs(x, stdout);
}

void put_buf(struct h5e_buf text) {
    fwrite(text.data, text.len, 1, stdout);
}

void do_chars(void *user, struct h5e_buf text) {
    put_str("CHARS : ");
    put_buf(text);
    put_str("\n");
}

void do_start_tag(void *user, struct h5e_buf name, int self_closing, size_t num_attrs) {
    put_str("TAG   : <");
    put_buf(name);
    if (self_closing) {
        putchar('/');
    }
    put_str(">\n");
}

void do_tag_attr(void *user, struct h5e_buf name, struct h5e_buf value) {
    put_str("  ATTR: ");
    put_buf(name);
    put_str("=\"");
    put_buf(value);
    put_str("\"\n");
}

void do_end_tag(void *user, struct h5e_buf name) {
    put_str("TAG   : </");
    put_buf(name);
    put_str(">\n");
}

struct h5e_token_ops ops = {
    .do_chars = do_chars,
    .do_start_tag = do_start_tag,
    .do_tag_attr = do_tag_attr,
    .do_end_tag = do_end_tag,
};

struct h5e_token_sink sink = {
    .ops = &ops,
    .user = NULL,
};

int main(int argc, char *argv[]) {
    if (argc < 2) {
        printf("Usage: %s 'HTML fragment'\n", argv[0]);
        return 1;
    }

    struct h5e_tokenizer *tok = h5e_tokenizer_new(&sink);
    h5e_tokenizer_feed(tok, h5e_buf_from_cstr(argv[1]));
    h5e_tokenizer_end(tok);
    h5e_tokenizer_free(tok);
    return 0;
}
