# error-graph

Allows non-fatal errors in a tree of subfunctions to easily be collected by a caller

Provides the `error_graph::ErrorList<E>` type to hold a list of non-fatal errors
that occurred while a function was running.

It has a `subwriter()` method that can be passed as a parameter to
a subfunction and allows that subfunction to record all the non-fatal errors it encounters.
When the subfunction is done running, its error list will be mapped to the caller's error type
and added to the caller's `ErrorList` automatically.

Since subfunctions may in-turn also use the `subwriter()`
function on the writter given to them by their caller, this creates a tree of non-fatal errors
that occurred during the execution of an entire call graph.

# Usage

```
# use error_graph::{ErrorList, WriteErrorList, strategy::{DontCare, ErrorOccurred}};
enum UpperError {
    Upper,
    Middle(ErrorList<MiddleError>),
}
enum MiddleError {
    Middle,
    Lower(ErrorList<LowerError>),
}
enum LowerError {
    Lower,
}
fn upper() {
    let mut errors = ErrorList::default();
    errors.push(UpperError::Upper);
    // Map the ErrorList<MiddleError> to our UpperError::Middle variant
    middle(errors.subwriter(UpperError::Middle));
    errors.push(UpperError::Upper);

    // Some callers just don't want to know if things went wrong or not
    middle(DontCare);

    // Some callers are only interested in whether an error occurred or not
    let mut error_occurred = ErrorOccurred::default();
    middle(&mut error_occurred);
    if error_occurred.as_bool() {
        errors.push(UpperError::Upper);
    }
}
fn middle(mut errors: impl WriteErrorList<MiddleError>) {
    // We can pass a sublist by mutable reference if we need to manipulate it before and after
    let mut sublist = errors.sublist(MiddleError::Lower);
    lower(&mut sublist);
    let num_errors = sublist.len();
    sublist.finish();
    if num_errors > 10 {
        errors.push(MiddleError::Middle);
    }
    // We can pass a reference directly to our error list for peer functions
    middle_2(&mut errors);
}
fn middle_2(mut errors: impl WriteErrorList<MiddleError>) {
    errors.push(MiddleError::Middle);
}
fn lower(mut errors: impl WriteErrorList<LowerError>) {
    errors.push(LowerError::Lower);
}
```

# Motivation

In most call graphs, a function that encounters an error will early-return and pass an
error type to its caller. The caller will often respond by passing that error further up the
call stack up to its own caller (possibly after wrapping it in its own error type). That
continues so-on-and-so-forth until some caller finally handles the error, returns from `main`,
or panics. Ultimately, the result is that some interested caller will receive a linear chain of
errors that led to the failure.

But, not all errors are fatal -- Sometimes, a function might be able to continue working after
it encounters an error and still be able to at-least-partially achieve its goals. Calling it
again - or calling other functions in the same API - is still permissible and may also result
in full or partial functionality.

In that case, the function may still choose to return `Result::Ok`; however, that leaves the
function with a dilemma -- How can it report the non-fatal errors to the caller?

1.  **Return a tuple in its `Result::Ok` type**: that wouldn't capture the non-fatal errors in
    the case that a fatal error occurs, so it would also have to be added to the `Result::Err`
    type as well.

    That adds a bunch of boilerplate, as the function needs to allocate the list and map it
    into the return type for every error return and good return. It also makes the function
    signature much more noisy.

2.  **Take a list as a mutable reference?**: Better, but now the caller has to allocate the
    list, and there's no way for it to opt out if it doesn't care about the non-fatal errors.

3.  **Maybe add an `Option` to it?** Okay, so a parameter like `errors: Option<&mut Vec<E>>`?
    Getting warmer, but now the child has to do a bunch of
    `if let Some(v) = errors { v.push(error); }` all over the place.

And what about the caller side of it? For a simple caller, the last point isn't too bad: The
caller just has to allocate the list, pass `Some(&mut errors)` to the child, and check it upon
return.

But often, the caller itself is keeping its own list of non-fatal errors and may also be a
subfunction to some other caller, and so-on-and-so-forth. In this case, we no longer have
a simple chain of errors, but instead we have a tree of errors -- Each level in the tree
contains all the non-fatal errors that occurred during execution of a function and all
subfunctions in its call graph.

# Solution

The main behavior we want is captured by the `WriteErrorList` trait in this crate. It can be
passed as a parameter to any function that wants to be able to report non-fatal errors to its
caller, and it gives the caller flexibility to decide what it wants to do with that
information.

The main concrete type in this crate is `ErrorList`, which stores a list of a single type of
error. Any time a list of errors needs to be stored in memory, this is the type to use. It will
usually be created by the top-level caller using `ErrorList::default`, and any subfunction will
give an `ErrorList` of its own error type to the `map_fn` that was passed in by its caller upon
return.

However, `ErrorList` should rarely be passed as a parameter to a function, as that wouldn't
provide the caller with the flexiblity to decide what strategy it actually wants
to use when collecting its subfunction's non-fatal errors. The caller may want to pass direct
reference to its own error list, it may want to pass a `Sublist` type that automatically
pushes the subfunction's error list to its own error list after mapping, or it may want to
pass the `DontCare` type if it doesn't want to know anything about the
subfunction's non-fatal errors.

Instead, subfunctions should take `impl WriteErrorList<E>` as a parameter.
This allows any of those types above, as well as mutable references to those types, to be
passed in by the caller. This also allows future caller strategies to be implemented, like
a caller that only cares how many non-fatal errors occurred but doesn't care about the details.

# Serde

(This section only applies if the `serde` feature is enabled)

`ErrorList` implements the `Serialize` trait if the errors it contains do, and
likewise with the `Deserialize` trait. This means that if every error type in the tree
implements these traits then the entire tree can be sent over the wire and recreated elsewhere.
Very useful if the errors are to be examined remotely!

