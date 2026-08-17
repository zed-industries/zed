* Check what we're doing with outgoing request parameters, there are 2 allocations going on in call_args! and rpc_args!, and we're just reading it in the end.

* Can we use the non-generic `split` methods from tokio for unixstream, tcpstream? Supposedly better performance, but introduces lifetimes...

* Propogate errors from `model::encode()` in `handler_loop()`

* Don't return an error on channel close, because the the regular way to shut down a plugin
  --> maybe? For a plugin, sure, what about GUIs?

* Don't build neovim ourselves, download a binary
