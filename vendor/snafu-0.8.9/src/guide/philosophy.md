# SNAFU's design philosophy

SNAFU believes in several points that are reflected in its design,
development, and maintenance. Knowing them may help users more
effectively use SNAFU.

## Categorize underlying errors by their context

It should be easy to bin one underlying error type (such as
[`io::Error`][Error]) into multiple domain-specific errors while
optionally adding contextual information.

[Error]: std::io::Error

## Library vs. application

SNAFU is designed to be used equally well in libraries and end-user applications.

## Many error types

Each module should have one (or more!) error types that are scoped
to that module, reducing the need to deal with unrelated errors
when matching and increasing cohesiveness of a single error type.
