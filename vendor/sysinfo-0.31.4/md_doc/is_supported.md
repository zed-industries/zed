Returns `true` if this OS is supported. Please refer to the
[crate-level documentation](index.html) to get the list of supported OSes.

```
if sysinfo::IS_SUPPORTED_SYSTEM {
    println!("This OS is supported!");
} else {
    println!("This OS isn't supported (yet?).");
}
```
