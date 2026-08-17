/// Generates sequential `u16` constants starting from the given value.
///
/// This macro is useful to generate constants for loaded resources, like menus
/// or dialog windows.
///
/// Each constant also supports individual documentation.
///
/// # Examples
///
/// ```no_run
/// use winsafe::seq_ids;
///
/// seq_ids! {
///     MNU_FILE = 3000;
///     MNU_FILE_OPEN
///     MNU_FILE_SAVE
///     /// This menu closes the application.
///     MNU_FILE_CLOSE
/// }
/// ```
///
/// The code above will generate the following:
///
/// ```no_run
/// pub const MNU_FILE: u16 = 3000;
/// pub const MNU_FILE_OPEN: u16 = 3001;
/// pub const MNU_FILE_SAVE: u16 = 3002;
/// /// This menu closes the application.
/// pub const MNU_FILE_CLOSE: u16 = 3003;
/// ```
#[macro_export]
macro_rules! seq_ids {
	() => {};

	($val:expr,) => {};

	(
		$( #[$comment:meta] )*
		$name:ident = $val:expr;
		$( $others:tt )*
	) => {
		$( #[$comment] )*
		pub const $name: u16 = $val;
		seq_ids!($val + 1, $( $others )*);
	};

	(
		$next_val:expr,
		$( #[$comment:meta] )*
		$name:ident = $val:expr;
		$( $others:tt )*
	) => {
		$( #[$comment] )*
		pub const $name: u16 = $val;
		seq_ids!($val + 1, $( $others )*);
	};

	(
		$next_val:expr,
		$( #[$comment:meta] )*
		$name:ident
		$( $others:tt )*
	) => {
		$( #[$comment] )*
		pub const $name: u16 = $next_val;
		seq_ids!($next_val + 1, $( $others )*);
	};
}
