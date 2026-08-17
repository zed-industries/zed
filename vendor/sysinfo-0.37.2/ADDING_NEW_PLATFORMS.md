To get the `target_os` etc:
```
rustc --print cfg
```

In `src/lib.rs` add the matching based on the `cfg` data.

Create `src/<new_platform>`. 
