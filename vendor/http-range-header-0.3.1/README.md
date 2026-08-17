# Range header parsing

[![Latest workflow](https://github.com/MarcusGrass/parse-range-headers/workflows/check_commit/badge.svg)](https://github.com/MarcusGrass/parse-range-headers/actions)
[![CratesIo](https://shields.io/crates/v/http-range-header)](https://crates.io/crates/http-range-header)

The main goals of this parser is:
* Follow specification [RFC-2616](https://www.ietf.org/rfc/rfc2616.txt)
* Behave as expected [MDN](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Range)
* Accuracy - parses headers strictly
* Security - Never panics, ensured by fuzzing
* Stability
* No dependecies

Secondary goals are:
* Speed
* Information on why the header was rejected (feature 'with_error_cause')

The parser is strict. Any range where all parts are not syntactically correct and makes sense in the context of the underlying 
resource will be rejected.
