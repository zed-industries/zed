#![allow(unused_macros)]

/// Builds one single FFI binding function.
macro_rules! one_func {
	($func:ident( $( $parm:ty ),* ) -> $ret:ty) => {
		pub(crate) fn $func( $( _: $parm, )* ) -> $ret;
	};

	($func:ident( $( $parm:ty ),* )) => {
		pub(crate) fn $func( $( _: $parm, )* );
	};
}

/// Builds a block of FFI bindings.
macro_rules! extern_sys {
	(
		$dll:expr;
		$(
			$func:ident( $( $parm:ty ),* ) $( -> $ret:ty )?
		)*
	) => {
		#[link(name = $dll)]
		extern "system" {
			$(
				one_func!( $func( $( $parm ),* ) $(-> $ret)? );
			)*
		}
	};
}
