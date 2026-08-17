# 3.0.6
 - switch from serde to serde_core

# 3.0.5
 - updated base version in the docs

# 3.0.4
 - updated base64 to 0.22.0

# 3.0.3
 - allow general whitespace separators instead of just newlines

# 3.0.2
 - allow EncodeConfig to be built in a const context

# 3.0.1
 - reduce allocations in `pem::encode`

# 3.0.0
 - trim `proptest` features to prevent an MSRV break for testing
 - make EncodeConfig struct extendable and add a line_wrap config option

# 2.0.1

 - Fix serde support on no\_std
 - Drop MSRV to 1.60

# 2.0

 - Add no\_std support
 - Bump MSRV to 1.67
 - Refactor API to prevent direct modification and access of elements and to
   allow access to the optional rfc1421-described headers.

# 1.1.1
 - Allow PEM files to be parsed with the optional rfc1421-described headers
   (although you cannot retrieve the headers)

# 1.1.0
 - Add optional serde support

# 1.0.2
 - Remove dependency on Regex in favor of a hand-rolled parser

# 1.0.1

 - hide the ASCII\_ARMOR symbol to work around a linking issue with 32-bit windows builds

# 1.0

 - `pem::parse_many` now returns a `Result<Vec<Pem>>` instead of a `Vec<Pem>` that silently discarded invalid sections.
