Validate the final client configuration.

This gets called immediately after the [`Intercept::read_before_execution`] trait hook
when the final configuration has been resolved. Any error returned here will
cause the operation to return that error.

[`Intercept::read_before_execution`]: crate::client::interceptors::Intercept::read_before_execution
