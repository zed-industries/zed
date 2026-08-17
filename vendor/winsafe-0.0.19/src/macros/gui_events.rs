#![allow(unused_macros)]

/// Ordinary window message, no parameters, no meaningful return.
macro_rules! fn_wm_noparm_noret {
	(
		$name:ident, $wmconst:expr;
		$( #[$doc:meta] )*
	) => {
		$( #[$doc] )*
		fn $name<F>(&self, func: F)
			where F: Fn() -> AnyResult<()> + 'static,
		{
			self.wm($wmconst, move |_| {
				func()?;
				Ok(None)
			});
		}
	};
}

/// Ordinary window message, no parameters, returns bool.
macro_rules! fn_wm_noparm_boolret {
	(
		$name:ident, $wmconst:expr;
		$( #[$doc:meta] )*
	) => {
		$( #[$doc] )*
		fn $name<F>(&self, func: F)
			where F: Fn() -> AnyResult<bool> + 'static,
		{
			self.wm($wmconst,
				move |_| Ok(Some(func()? as _)));
		}
	};
}

/// Ordinary window message, with parameters, no meaningful return.
macro_rules! fn_wm_withparm_noret {
	(
		$name:ident, $wmconst:expr, $parm:ty;
		$( #[$doc:meta] )*
	) => {
		$( #[$doc] )*
		fn $name<F>(&self, func: F)
			where F: Fn($parm) -> AnyResult<()> + 'static,
		{
			self.wm($wmconst, move |p| {
				func(<$parm>::from_generic_wm(p))?;
				Ok(None)
			});
		}
	};
}

/// Ordinary window message, with parameters, returns bool.
macro_rules! fn_wm_withparm_boolret {
	(
		$name:ident, $wmconst:expr, $parm:ty;
		$( #[$doc:meta] )*
	) => {
		$( #[$doc] )*
		fn $name<F>(&self, func: F)
			where F: Fn($parm) -> AnyResult<bool> + 'static,
		{
			self.wm($wmconst,
				move |p| Ok(Some(func(<$parm>::from_generic_wm(p))? as _)));
		}
	};
}

/// Ordinary window message, with parameters, returns constant.
macro_rules! fn_wm_withparm_coret {
	(
		$name:ident, $wmconst:expr, $parm:ty, $coret:ty;
		$( #[$doc:meta] )*
	) => {
		$( #[$doc] )*
		fn $name<F>(&self, func: F)
			where F: Fn($parm) -> AnyResult<$coret> + 'static,
		{
			self.wm($wmconst,
				move |p| Ok(Some(func(<$parm>::from_generic_wm(p))?.raw() as _)));
		}
	};
}

/// WM_CTLCOLOR* message.
macro_rules! fn_wm_ctlcolor {
	(
		$name:ident, $wmconst:expr, $parm:ty;
		$( #[$doc:meta] )*
	) => {
		$( #[$doc] )*
		fn $name<F>(&self, func: F)
			where F: Fn($parm) -> AnyResult<crate::user::decl::HBRUSH> + 'static,
		{
			self.wm($wmconst,
				move |p| Ok(Some(func(<$parm>::from_generic_wm(p))?.ptr() as _)));
		}
	};
}

//------------------------------------------------------------------------------

/// WM_COMMAND message, no parameters, no meaningful return.
macro_rules! pub_fn_cmd_noparm_noret {
	(
		$name:ident, $cmd:expr;
		$( #[$doc:meta] )*
	) => {
		$( #[$doc] )*
		pub fn $name<F>(&self, func: F)
			where F: Fn() -> AnyResult<()> + 'static,
		{
			self.0.wm_command($cmd,
				move || func());
		}
	};
}

/// WM_NOTIFY message, no parameters, no meaningful return.
macro_rules! pub_fn_nfy_noparm_noret {
	(
		$name:ident, $nfy:expr;
		$( #[$doc:meta] )*
	) => {
		$( #[$doc] )*
		pub fn $name<F>(&self, func: F)
			where F: Fn() -> AnyResult<()> + 'static,
		{
			self.0.wm_notify($nfy, move |_| {
				func()?;
				Ok(None)
			});
		}
	};
}

/// WM_NOTIFY message, with parameters, no meaningful return.
macro_rules! pub_fn_nfy_withparm_noret {
	(
		$name:ident, $nfy:expr, $param:ty;
		$( #[$doc:meta] )*
	) => {
		$( #[$doc] )*
		pub fn $name<F>(&self, func: F)
			where F: Fn(&$param) -> AnyResult<()> + 'static,
		{
			self.0.wm_notify($nfy, move |p| {
				func(unsafe { p.cast_nmhdr::<$param>() })?;
				Ok(None)
			});
		}
	};
}

/// WM_NOTIFY message, with mutable parameters, no meaningful return.
macro_rules! pub_fn_nfy_withmutparm_noret {
	(
		$name:ident, $nfy:expr, $param:ty;
		$( #[$doc:meta] )*
	) => {
		$( #[$doc] )*
		pub fn $name<F>(&self, func: F)
			where F: Fn(&mut $param) -> AnyResult<()> + 'static,
		{
			self.0.wm_notify($nfy, move |p| {
				func(unsafe { p.cast_nmhdr_mut::<$param>() })?;
				Ok(None)
			});
		}
	};
}

/// WM_NOTIFY message, no parameters, returns bool.
macro_rules! pub_fn_nfy_noparm_boolret {
	(
		$name:ident, $nfy:expr;
		$( #[$doc:meta] )*
	) => {
		$( #[$doc] )*
		pub fn $name<F>(&self, func: F)
			where F: Fn() -> AnyResult<bool> + 'static,
		{
			self.0.wm_notify($nfy,
				move |_| Ok(Some(func()? as _)));
		}
	};
}

/// WM_NOTIFY message, with parameters, returns bool.
macro_rules! pub_fn_nfy_withparm_boolret {
	(
		$name:ident, $nfy:expr, $param:ty;
		$( #[$doc:meta] )*
	) => {
		$( #[$doc] )*
		pub fn $name<F>(&self, func: F)
			where F: Fn(&$param) -> AnyResult<bool> + 'static,
		{
			self.0.wm_notify($nfy,
				move |p| Ok(Some(func(unsafe { p.cast_nmhdr::<$param>() })? as _)));
		}
	};
}

/// WM_NOTIFY message, no parameters, returns i32.
macro_rules! pub_fn_nfy_noparm_i32ret {
	(
		$name:ident, $nfy:expr;
		$( #[$doc:meta] )*
	) => {
		$( #[$doc] )*
		pub fn $name<F>(&self, func: F)
			where F: Fn() -> AnyResult<i32> + 'static,
		{
			self.0.wm_notify($nfy,
				move |_| Ok(Some(func()? as _)));
		}
	};
}

/// WM_NOTIFY message, with parameters, returns i32.
macro_rules! pub_fn_nfy_withparm_i32ret {
	(
		$name:ident, $nfy:expr, $param:ty;
		$( #[$doc:meta] )*
	) => {
		$( #[$doc] )*
		pub fn $name<F>(&self, func: F)
			where F: Fn(&$param) -> AnyResult<i32> + 'static,
		{
			self.0.wm_notify($nfy,
				move |p| Ok(Some(func(unsafe { p.cast_nmhdr::<$param>() })? as _)));
		}
	};
}
