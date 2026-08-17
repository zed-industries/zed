Validate the base client configuration.

This gets called upon client construction. The full config may not be available at
this time (hence why it has [`RuntimeComponentsBuilder`] as an argument rather
than [`RuntimeComponents`]). Any error returned here will become a panic
in the client constructor.

[`RuntimeComponentsBuilder`]: crate::client::runtime_components::RuntimeComponentsBuilder
[`RuntimeComponents`]: crate::client::runtime_components::RuntimeComponents
