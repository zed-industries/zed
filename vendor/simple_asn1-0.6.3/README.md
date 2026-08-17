# Overview

This is a simple library for parsing DER-encoded data.

In particular, this library automates the process of understanding the DER
encoded objects in an ASN.1 data stream. These tokens can then be parsed by your
library, based on the ASN.1 description in your format.

For convenience, we create the traits `ToASN1` and `FromASN` to abstract the
ability to decode a type from an ASN.1 token stream. If your type implements one
of these traits, your program or library can then use the convenience functions
`der_encode` and `der_decode` to do all the parsing work in one action.

Patches welcome!

