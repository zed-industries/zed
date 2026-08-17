Help:
```console
$ typed-derive fn-parser --help
Usage: typed-derive fn-parser [OPTIONS]

Options:
  -D <KEY=VALUE>      Hand-written parser for tuples
  -h, --help          Print help

```

Defines (key-value pairs)
```console
$ typed-derive fn-parser -D Foo=10 -D Alice=30
FnParser(FnParser { defines: [("Foo", 10), ("Alice", 30)] })

$ typed-derive fn-parser -D Foo
? failed
error: invalid value 'Foo' for '-D <KEY=VALUE>': invalid KEY=value: no `=` found in `Foo`

For more information, try '--help'.

$ typed-derive fn-parser -D Foo=Bar
? failed
error: invalid value 'Foo=Bar' for '-D <KEY=VALUE>': invalid digit found in string

For more information, try '--help'.

```
