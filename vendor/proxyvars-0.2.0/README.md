## proxyvars

proxyvars is a parser for the `HTTP_PROXY`, `HTTPS_PROXY` and `NO_PROXY` environment variables, with `NO_PROXY` matcher based on [Go's implementation](https://github.com/golang/net/blob/master/http/httpproxy/proxy.go).

This crate can:

- Read `HTTPS_PROXY` and `HTTP_PROXY` and its lowercase variants
- Read, parse and evaluate the `NO_PROXY` environment variables against a given URL
    - Supports CIDR
    - Supports IPv4/IPv6 address and optinal ports
    - Supports Hostnames and optinal ports
    - Supports Wildcard

### Usage

```rust
if let Some(no_proxy) = proxyvars::no_proxy() {
    // This environment has NO_PROXY defined
    if no_proxy.matches("https://company.com") {
        // We should not use a proxy for this URL
    } else {
        // We should use a proxy for this URL, which are available at:
        let https_proxy = proxyvars::https_proxy();
        let http_proxy = proxyvars::http_proxy();
    }
}
```

### Notes

1. The implementation of the `NO_PROXY` matcher is heavily inspired by Go's implementation located at [http/httpproxy/proxy.go](https://github.com/golang/net/blob/master/http/httpproxy/proxy.go).
2. It's outside the scope of this crate to actually perform the proxying.