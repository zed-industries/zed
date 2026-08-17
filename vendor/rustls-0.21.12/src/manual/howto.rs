/*! # Customising private key usage

By default rustls supports PKCS#8-format[^1] RSA or ECDSA keys, plus PKCS#1-format RSA keys.

However, if your private key resides in a HSM, or in another process, or perhaps
another machine, rustls has some extension points to support this:

The main trait you must implement is [`sign::SigningKey`][signing_key]. The primary method here
is [`choose_scheme`][choose_scheme] where you are given a set of [`SignatureScheme`s][sig_scheme] the client says
it supports: you must choose one (or return `None` -- this aborts the handshake). Having
done that, you return an implementation of the [`sign::Signer`][signer] trait.
The [`sign()`][sign_method] performs the signature and returns it.

(Unfortunately this is currently designed for keys with low latency access, like in a
PKCS#11 provider, Microsoft CryptoAPI, etc. so is blocking rather than asynchronous.
It's a TODO to make these and other extension points async.)

Once you have these two pieces, configuring a server to use them involves, briefly:

- packaging your `sign::SigningKey` with the matching certificate chain into a [`sign::CertifiedKey`][certified_key]
- making a [`ResolvesServerCertUsingSni`][cert_using_sni] and feeding in your `sign::CertifiedKey` for all SNI hostnames you want to use it for,
- setting that as your `ServerConfig`'s [`cert_resolver`][cert_resolver]

[signing_key]: ../../sign/trait.SigningKey.html
[choose_scheme]: ../../sign/trait.SigningKey.html#tymethod.choose_scheme
[sig_scheme]: ../../enum.SignatureScheme.html
[signer]: ../../sign/trait.Signer.html
[sign_method]: ../../sign/trait.Signer.html#tymethod.sign
[certified_key]: ../../sign/struct.CertifiedKey.html
[cert_using_sni]: ../../struct.ResolvesServerCertUsingSni.html
[cert_resolver]: ../../struct.ServerConfig.html#structfield.cert_resolver

[^1]: For PKCS#8 it does not support password encryption -- there's not a meaningful threat
      model addressed by this, and the encryption supported is typically extremely poor.

*/
