Returns the tree-sitter outline of a source code file.

Markdown headings indicate the structure of the outline; just like
with markdown headings, the more # symbols there are at the beginning of a line,
the deeper it is in the outline hierarchy.

Each heading ends with a line number or range, which tells you what portion of the
underlying source code file corresponds to that part of the outline. You can use
that line information with other tools, to strategically read portions of the source code.

For example, you can use this tool to get the outline of a large source code file, then use
the line number information from that outline to read different sections of that file,
without having to read the entire file all at once (which can be slow, or use a lot of tokens).

<example>
# class Foo [L123-136]
## method do_something(arg1, arg2) [L124-126]
## method process_data(data) [L128-135]
# class Bar [L145-161]
## method initialize() [L146-149]
## method update_state(new_state) [L160]
## private method _validate_state(state) [L161-162]
</example>

This example shows how tree-sitter outlines the structure of source code:

1. `class Foo` is defined on lines 123-136
   - It contains a method `do_something` spanning lines 124-126
   - It also has a method `process_data` spanning lines 128-135

2. `class Bar` is defined on lines 145-161
   - It has an `initialize` method spanning lines 146-149
   - It has an `update_state` method on line 160
   - It has a private method `_validate_state` spanning lines 161-162
