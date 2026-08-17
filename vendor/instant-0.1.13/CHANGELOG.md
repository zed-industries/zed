# v0.1.12
## Added 
- Add `SystemTime` which works in both native and WASM environments.

## Modified
- The `now` function is always available now: there is no need to enable the `now` feature any more. The `now` feature
  still exists (but doesn’t do anything) for backwards compatibility.