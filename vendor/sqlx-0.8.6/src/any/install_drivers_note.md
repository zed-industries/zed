  
The underlying database drivers are chosen at runtime from the list set via
[`install_drivers`][crate::any::install_drivers]. Any use of [`AnyConnection`] or [`AnyPool`]
without this will panic.

It is recommended to use [`install_default_drivers`][crate::any::install_default_drivers] to activate all currently compiled-in drivers.  

[`AnyConnection`]: sqlx_core::any::AnyConnection
[`AnyPool`]: sqlx_core::any::AnyPool
