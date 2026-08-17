/*!

## Rationale for defaults

### Why is AES-256 preferred over AES-128?

This is a trade-off between:

1. classical security level: searching a 2^128 key space is as implausible as 2^256.
2. post-quantum security level: the difference is more meaningful, and AES-256 seems like the conservative choice.
3. performance: AES-256 is around 40% slower than AES-128, though hardware acceleration typically narrows this gap.

The choice is frankly quite marginal.

### Why is AES-GCM preferred over chacha20-poly1305?

Hardware support for accelerating AES-GCM is widespread, and hardware-accelerated AES-GCM
is quicker than un-accelerated chacha20-poly1305.

However, if you know your application will run on a platform without that, you should
_definitely_ change the default order to prefer chacha20-poly1305: both the performance and
the implementation security will be improved.  We think this is an uncommon case.

### Why is x25519 preferred for key exchange over nistp256?

Both provide roughly the same classical security level, but x25519 has better performance and
it's _much_ more likely that both peers will have good quality implementations.

*/
