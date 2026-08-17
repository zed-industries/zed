Help:
```console
$ typed-derive builtin --help
Usage: typed-derive builtin [OPTIONS]

Options:
      --port <PORT>            Support for discrete numbers [default: 22] [possible values: 22, 80]
      --log-level <LOG_LEVEL>  Support enums from a foreign crate that don't implement `ValueEnum` [default: info] [possible values: trace, debug, info, warn, error]
  -h, --help                   Print help

```

Discrete numbers
```console
$ typed-derive builtin --port 22
Builtin(BuiltInParsers { port: 22, log_level: Info })

$ typed-derive builtin --port 80
Builtin(BuiltInParsers { port: 80, log_level: Info })

$ typed-derive builtin --port
? failed
error: a value is required for '--port <PORT>' but none was supplied
  [possible values: 22, 80]

For more information, try '--help'.

$ typed-derive builtin --port 3000
? failed
error: invalid value '3000' for '--port <PORT>'
  [possible values: 22, 80]

For more information, try '--help'.

```

Enums from crates that can't implement `ValueEnum`
```console
$ typed-derive builtin --log-level debug
Builtin(BuiltInParsers { port: 22, log_level: Debug })

$ typed-derive builtin --log-level error
Builtin(BuiltInParsers { port: 22, log_level: Error })

$ typed-derive builtin --log-level
? failed
error: a value is required for '--log-level <LOG_LEVEL>' but none was supplied
  [possible values: trace, debug, info, warn, error]

For more information, try '--help'.

$ typed-derive builtin --log-level critical
? failed
error: invalid value 'critical' for '--log-level <LOG_LEVEL>'
  [possible values: trace, debug, info, warn, error]

For more information, try '--help'.

```
