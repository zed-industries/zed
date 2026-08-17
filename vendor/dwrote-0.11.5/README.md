# dwrote
A thin wrapper around Windows DirectWrite for Rust

Dwrote provides access to pieces of DirectWrite needed by WebRender
and Servo.  It can be easily extended to other parts of DirectWrite,
but full mapping of the DirectWrite API is not a goal (pull requests
accepted, of course).

There are a few additional helper functions on individual types that
don't exist in DirectWrite, and a few have had their signatures changed,
but for the most part this library attempts to replicate the DirectWrite
API.
